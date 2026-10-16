"""Fixed service installers, commit-pinned Git deployments and TBS provisioning."""
import copy
import codecs
import base64
from contextlib import contextmanager
import fcntl
import json
import os
from pathlib import Path
import re
import shlex
import signal
import subprocess
import tempfile
import threading
import time
import tomllib
import uuid

from bindings import resolve_config
from common import CATALOG, ROOT, SHA, atomic_write, identifier, json_file, request_json, toml_dump

HEALTH_TIMEOUT_SECONDS = 60
RUNTIME_SUBSET_VERSION = '2'
RUNTIME_DOCS = ('Docs/basisstation.config.sanitized.example.toml',)


class CommandFailed(RuntimeError):
    """Keep bounded command output for transport classification, never argv secrets."""
    def __init__(self, command, code, output):
        self.command = tuple(map(str, command))
        self.returncode = code
        self.output = output[-8192:]
        super().__init__(f'Command exited with {code}: {command[0]}')


def transient_git_failure(exc):
    if isinstance(exc, TimeoutError):
        return True
    if not isinstance(exc, CommandFailed) or Path(exc.command[0]).name not in ('git', 'git.exe'):
        return False
    output = exc.output.lower()
    if any(text in output for text in ('no space left', 'permission denied', 'authentication failed',
            'repository not found', 'could not read username', 'dubious ownership', 'index.lock',
            'cannot lock ref', 'not our ref', "couldn\x27t find remote ref", 'certificate verification failed',
            'ssl certificate problem', 'would be overwritten')):
        return False
    return any(text in output for text in ('rpc failed; curl 56', 'rpc failed; curl 18',
        'rpc failed; curl 92', 'connection reset by peer', 'early eof', 'unexpected disconnect',
        'remote end hung up unexpectedly', 'could not resolve host', 'failed to connect',
        'tls connection was non-properly terminated'))


def run(command, log, cwd=None, timeout=3600, env=None):
    log('$ ' + shlex.join(map(str, command)))
    with tempfile.TemporaryFile() as output:
        process = subprocess.Popen(command, cwd=cwd, env=env, stdin=subprocess.DEVNULL,
                                   stdout=output, stderr=subprocess.STDOUT, start_new_session=True)
        deadline = time.monotonic() + timeout
        position = 0
        decoder = codecs.getincrementaldecoder('utf-8')(errors='replace')
        pending = ''
        tail = ''
        try:
            while True:
                chunk = os.pread(output.fileno(), 65536, position)
                position += len(chunk)
                if chunk:
                    decoded = decoder.decode(chunk)
                    pending += decoded
                    tail = (tail + decoded)[-8192:]
                    if '\n' in pending:
                        complete, pending = pending.rsplit('\n', 1)
                        log(complete)
                code = process.poll()
                if code is not None and not chunk:
                    pending += decoder.decode(b'', final=True)
                    if pending:
                        log(pending)
                    if code:
                        raise CommandFailed(command, code, tail)
                    return
                if time.monotonic() >= deadline:
                    raise TimeoutError('Deployment command timed out')
                time.sleep(0.1)
        finally:
            if process.poll() is None:
                os.killpg(process.pid, signal.SIGTERM)
                try:
                    process.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    os.killpg(process.pid, signal.SIGKILL)
                    process.wait()


class Repository:
    def __init__(self, cfg):
        self.url = cfg['repository']
        self.state = Path(cfg['state_dir'])
        self.path = self.state / 'source'
        self.lock = threading.RLock()

    @contextmanager
    def guard(self):
        # A thread lock alone does not protect another agent/process using the
        # same state directory. Keep this mutex outside replaceable checkouts.
        with self.lock:
            self.state.mkdir(parents=True, exist_ok=True)
            with (self.state / 'source.mutex').open('a') as mutex:
                try:
                    fcntl.flock(mutex, fcntl.LOCK_EX | fcntl.LOCK_NB)
                except BlockingIOError as exc:
                    raise RuntimeError('Ein anderer NetCore-Prozess verwendet bereits den Git-Cache.') from exc
                try:
                    selector = self.state / 'source-location.json'
                    directory = 'source'
                    if selector.exists():
                        data = json.loads(selector.read_text())
                        directory = data.get('directory') if isinstance(data, dict) else None
                        if not isinstance(directory, str) or not re.fullmatch(r'source-[0-9a-f]{32}', directory):
                            raise RuntimeError('Ungültiger verwalteter Git-Cache in source-location.json')
                    self.path = self.state / directory
                    yield
                finally:
                    fcntl.flock(mutex, fcntl.LOCK_UN)

    def _fetch(self, log):
        if not (self.path / '.git').exists():
            run(['git', 'clone', '--no-checkout', self.url, str(self.path)], log, timeout=300)
        run(['git', 'fetch', '--prune', 'origin', '+refs/heads/*:refs/remotes/origin/*',
             '+refs/tags/*:refs/tags/*'], log, self.path, timeout=300)

    def fetch(self, log):
        with self.guard():
            self._prepare(log)

    def _prepare(self, log):
        index_lock = self.path / '.git/index.lock'
        if not index_lock.exists() and not index_lock.is_symlink():
            self._fetch(log)
            return
        previous = self.path
        log(f'Git-Sperre gefunden: {index_lock}. Der bisherige Quellordner bleibt unverändert; '
            'ein separater verwalteter Checkout wird erstellt.')
        # Git may close its lock FD before committing the index. Neither age nor
        # fuser alone proves a lock abandoned. Never unlink/rename the lock or
        # the old tree: an external Git process may still be using either.
        with tempfile.TemporaryDirectory(prefix='source-new-', dir=self.state) as temporary:
            try:
                self.path = Path(temporary) / 'source'
                self._fetch(log)
                selected = self.state / ('source-' + uuid.uuid4().hex)
                self.path.rename(selected)
                # Persist only after the clone/fetch succeeded; failed recovery
                # leaves the previous selection and original checkout intact.
                atomic_write(self.state / 'source-location.json',
                             json.dumps({'directory': selected.name}), 0o600)
            finally:
                self.path = previous
        self.path = selected
        log(f'Neuer Git-Cache: {selected}. Alter Quellordner vollständig erhalten: {previous}')

    def resolve(self, ref, log):
        if not isinstance(ref, str) or not re.fullmatch(r'[A-Za-z0-9][A-Za-z0-9_./-]{0,127}', ref) or '..' in ref:
            raise ValueError('Ungültiger Git-Ref')
        with self.guard():
            self._prepare(log)
            options = [ref] if SHA.fullmatch(ref) else [f'refs/remotes/origin/{ref}', f'refs/tags/{ref}']
            for value in options:
                p = subprocess.run(['git', 'rev-parse', '--verify', value + '^{commit}'], cwd=self.path,
                                   capture_output=True, text=True, timeout=10)
                if p.returncode == 0 and SHA.fullmatch(p.stdout.strip()):
                    return p.stdout.strip()
        raise ValueError('Branch, Tag oder Commit nicht gefunden')

    def resolve_latest_main(self, log):
        """Fetch first, then pin exactly the remote main branch without tag fallback."""
        with self.guard():
            self._prepare(log)
            result = subprocess.run(
                ['git', 'rev-parse', '--verify', 'refs/remotes/origin/main^{commit}'],
                cwd=self.path, capture_output=True, text=True, timeout=10)
            if result.returncode == 0 and SHA.fullmatch(result.stdout.strip()):
                return result.stdout.strip()
        raise ValueError('Aktueller main-Branch im Repository nicht gefunden; kein Image-Auftrag angelegt')

    def checkout(self, sha, log):
        if not SHA.fullmatch(sha):
            raise ValueError('Deployment requires a full commit SHA')
        with self.guard():
            previous = self.path
            locked = (previous / '.git/index.lock').exists() or (previous / '.git/index.lock').is_symlink()
            if (previous / '.git/index').is_file() and not locked:
                dirty = subprocess.check_output(['git', '--no-optional-locks', 'status', '--porcelain',
                                                 '--untracked-files=no'], cwd=previous, text=True, timeout=30)
                if dirty:
                    raise RuntimeError('Managed checkout contains local changes; refusing to overwrite them')
                policy = subprocess.run(['git', 'config', '--get', 'netcore.runtimeSubset'], cwd=previous,
                                        capture_output=True, text=True, timeout=10)
                actual = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=previous,
                                                 text=True, timeout=10).strip()
                if policy.returncode == 0 and policy.stdout.strip() == RUNTIME_SUBSET_VERSION and actual == sha:
                    log('Geprüfter Runtime-Quellcommit bereits im Cache: ' + sha)
                    return previous
            # Build a complete replacement privately. A failed fetch or lazy
            # blob download during checkout never modifies the selected cache.
            for attempt in range(1, 4):
                log(f'Runtime-Quellabruf {attempt}/3: Commit {sha}; HTTP/1.1, flach, ohne große Dokumentationsdateien')
                try:
                    with tempfile.TemporaryDirectory(prefix='source-new-', dir=self.state) as temporary:
                        source = Path(temporary) / 'source'
                        run(['git', 'init', str(source)], log, timeout=30)
                        run(['git', 'remote', 'add', 'origin', self.url], log, source, timeout=30)
                        for name, value in (('http.version', 'HTTP/1.1'), ('pack.threads', '1'),
                                ('remote.origin.promisor', 'true'), ('remote.origin.partialclonefilter', 'blob:none')):
                            run(['git', 'config', name, value], log, source, timeout=30)
                        run(['git', 'sparse-checkout', 'set', '--no-cone', '/*', '!/Docs/*', '!/wiki/',
                             *('/' + name for name in RUNTIME_DOCS)],
                            log, source, timeout=30)
                        run(['git', 'fetch', '--depth=1', '--filter=blob:none', '--no-tags',
                             'origin', sha], log, source, timeout=300)
                        run(['git', 'checkout', '--detach', sha], log, source, timeout=300)
                        actual = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=source,
                                                         text=True, timeout=10).strip()
                        if actual != sha:
                            raise RuntimeError('Commit verification failed')
                        run(['git', 'config', 'netcore.runtimeSubset', RUNTIME_SUBSET_VERSION], log, source, timeout=30)
                        selected = self.state / ('source-' + uuid.uuid4().hex)
                        source.rename(selected)
                    break
                except (CommandFailed, TimeoutError) as exc:
                    if attempt == 3 or not transient_git_failure(exc):
                        raise
                    log(f'Git-Transport unterbrochen; erneuter Quellabruf in {attempt * 2}s. '
                        'Der Dienstinstaller wurde noch nicht gestartet.')
                    time.sleep(attempt * 2)
            atomic_write(self.state / 'source-location.json', json.dumps({'directory': selected.name}), 0o600)
            self.path = selected
            log('Runtime-Quelle vollständig geprüft: ' + sha)
        return self.path


def validate_profile(profile):
    profile = copy.deepcopy(profile)
    identifier(profile.get('name'))
    for key, low, high in [('mcc', 0, 1023), ('mnc', 0, 16383), ('issi', 1, 16777214),
                          ('la', 0, 16383), ('cc', 0, 63)]:
        val = profile.get(key)
        if type(val) is not int or not low <= val <= high:
            raise ValueError(f'{key}: erwartet {low}…{high}')
    # RF values come from an explicit site template, never from stale personal config.
    return {k: profile[k] for k in ('name', 'mcc', 'mnc', 'issi', 'la', 'cc')}



# Same duplex-spacing rows (kHz) as tetra_core::freqs, bands 0 through 8.
TBS_DUPLEX_KHZ = (
    (None, 1600, 10000, 10000, 10000, 10000, 10000, None, None),
    (None, 4500, None, 36000, 7000, None, None, None, 45000),
    (0, 0, 0, 0, 0, 0, 0, 0, 0),
    (None, None, None, 8000, 8000, None, None, None, 18000),
    (None, None, None, 18000, 5000, None, 30000, 30000, None),
    (None, None, None, None, 9500, None, None, None, None),
    (None,) * 9,
    (None,) * 9,
)


def new_tbs_template(settings, endpoints=None):
    """Create a credential-free site template with matching SYSINFO/SDR frequencies."""
    defaults = dict(freq_band=4, main_carrier=720, secondary_carrier=721, freq_offset=0,
                    duplex_spacing=0, reverse_operation=False, custom_duplex_spacing=None,
                    sample_rate=600000, device='')
    if not isinstance(settings, dict) or set(settings) - defaults.keys():
        raise ValueError('Unbekannte Felder für das TBS-Template')
    values = {**defaults, **settings}

    def integer(key, low, high, optional=False):
        value = values[key]
        if optional and value is None:
            return None
        if type(value) is not int or not low <= value <= high:
            raise ValueError(f'{key}: erwartet {low}…{high}')
        return value

    band = integer('freq_band', 1, 8)
    main = integer('main_carrier', 0, 3999)
    secondary = integer('secondary_carrier', 0, 3999, optional=True)
    offset = integer('freq_offset', -6250, 12500)
    if offset not in (-6250, 0, 6250, 12500):
        raise ValueError('Frequenzoffset: -6250, 0, 6250 oder 12500 Hz wählen')
    spacing_id = integer('duplex_spacing', 0, 7)
    custom = integer('custom_duplex_spacing', 0, 100000000, optional=True)
    reverse = values['reverse_operation']
    if type(reverse) is not bool:
        raise ValueError('reverse_operation muss ein Wahrheitswert sein')
    rate = integer('sample_rate', 48000, 100000000)
    device = values['device']
    if not isinstance(device, str) or len(device) > 512 or any(ord(c) < 32 or ord(c) == 127 for c in device):
        raise ValueError('Ungültige SoapySDR-Geräteparameter')
    spacing = custom
    if spacing is None:
        khz = TBS_DUPLEX_KHZ[spacing_id][band]
        if khz is None:
            raise ValueError('Duplexabstand ist für dieses Frequenzband nicht definiert')
        spacing = khz * 1000
    if secondary == main:
        raise ValueError('Hauptträger und zweiter Träger müssen verschieden sein')
    if secondary is not None and (abs(secondary - main) + 1) * 25000 > rate:
        raise ValueError('Beide 25-kHz-Träger müssen in die gewählte SDR-Samplerate passen')
    carriers = [main] if secondary is None else [main, secondary]
    tx = [band * 100000000 + carrier * 25000 + offset for carrier in carriers]
    rx = [frequency + (spacing if reverse else -spacing) for frequency in tx]
    if min(tx + rx) <= 0 or max(tx + rx) > 4294967295:
        raise ValueError('Ungültige abgeleitete Sende-/Empfangsfrequenz')
    soapy = dict(tx_freq=tx[0], rx_freq=rx[0], sample_rate=rate,
                 tx_center_freq=(min(tx) + max(tx)) / 2,
                 rx_center_freq=(min(rx) + max(rx)) / 2)
    if device.strip():
        soapy['device'] = device.strip()
    cell = dict(freq_band=band, main_carrier=main, duplex_spacing=spacing_id,
                freq_offset=offset, reverse_operation=reverse, location_area=1, colour_code=1,
                timezone='Europe/Berlin', local_ssi_ranges=[[0, 90]],
                system_wide_services=True, no_minimum_mode=True, voice_service=True)
    if secondary is not None:
        cell['secondary_carrier'] = secondary
    if custom is not None:
        cell['custom_duplex_spacing'] = custom
    config = dict(config_version='0.6', stack_mode='Bs', service_name='tetra',
                  phy_io=dict(backend='SoapySdr', soapysdr=soapy),
                  net_info=dict(mcc=901, mnc=1510), cell_info=cell,
                  dashboard=dict(bind='0.0.0.0', port=8080),
                  control_room=dict(enabled=True, host='node-gateway', port=8080,
                                    use_tls=False, endpoint_path='/ws/node'))
    return resolve_config('tbs', config, endpoints or {})[0]


def tbs_config(template, profile):
    profile = validate_profile(profile)
    data = tomllib.loads(template)
    # A newly provisioned OpenLab station inherits RF settings, not UI credentials.
    for key in ('username', 'password'):
        data.get('dashboard', {}).pop(key, None)
    data['net_info']['mcc'], data['net_info']['mnc'] = profile['mcc'], profile['mnc']
    data['cell_info']['location_area'], data['cell_info']['colour_code'] = profile['la'], profile['cc']
    data.setdefault('control_room', {}).update(node_id=profile['name'], station_name=profile['name'])
    if 'brew' in data:
        data['brew']['username'] = profile['issi']
    if 'audio_player' in data:
        data['audio_player']['source_issi'] = profile['issi']
    if 'media_library' in data:
        data['media_library']['station_id'] = profile['name']
    return toml_dump(data)


def unit_properties(unit):
    identifier(unit)
    try:
        result = subprocess.run(['systemctl', 'show', unit, '--property=LoadState,FragmentPath,ExecStart'],
                                capture_output=True, text=True, timeout=5)
    except (OSError, subprocess.TimeoutExpired):
        return {}
    return dict(line.split('=', 1) for line in result.stdout.splitlines() if '=' in line)


def installed_unit(unit):
    # A config or a leftover .service.d directory is not an installed service.
    properties = unit_properties(unit)
    return properties.get('LoadState') == 'loaded' and bool(properties.get('FragmentPath'))


def installed_service(spec):
    return (spec.get('unit_detected', True) and Path(spec['config_target']).is_file()
            and installed_unit(spec['unit']))


def existing_tbs(spec):
    """Resolve historical units without overwriting their ExecStart or binary path."""
    paths = dict.fromkeys([spec['config_target'], '/etc/netcore/config.toml',
                          '/opt/tetra/config.toml', '/etc/flowstation/config.toml'])
    for config in paths:
        try:
            data = tomllib.loads(Path(config).read_text())
        except (OSError, ValueError):
            continue
        configured = data.get('service_name', '')
        candidates = ([configured if configured.endswith('.service') else configured + '.service']
                      if isinstance(configured, str) and configured else [])
        candidates += ['netcore-tbs.service', 'tetra.service', 'bluestation.service',
                       'tetra-bluestation.service', 'bluestation-bs.service']
        for unit in dict.fromkeys(candidates):
            properties = unit_properties(unit)
            # A copied /etc config must not hide the config actually referenced
            # by a legacy unit under /opt. Also handles our launch.py wrapper.
            config_arg = re.search(r'(?<![\w/.-])' + re.escape(config) + r'(?=$|[\s;\"\'])',
                                   properties.get('ExecStart', ''))
            if (properties.get('LoadState') == 'loaded' and properties.get('FragmentPath')
                    and config_arg):
                return dict(spec, unit=unit, config_target=config, unit_detected=True)
    return dict(spec, unit_detected=False)


def service_spec(cfg, name):
    spec = dict(CATALOG[name])
    explicit_unit = False
    for override in cfg['services']:
        if override['name'] == name:
            explicit_unit = 'unit' in override
            for key in ('config_target', 'unit', 'command', 'port', 'health'):
                if key in override:
                    spec[key] = override[key]
    if name == 'tbs' and not explicit_unit:
        spec = existing_tbs(spec)
    try:
        actual = tomllib.loads(Path(spec['config_target']).read_text())
        if name == 'tbs':
            spec['port'] = actual.get('dashboard', {}).get('port', spec['port'])
        else:
            bind = actual.get('server', {}).get('bind', actual.get('bind', ''))
            if isinstance(bind, str) and ':' in bind:
                spec['port'] = int(bind.rsplit(':', 1)[1])
            elif 'port' in actual.get('server', {}):
                spec['port'] = int(actual['server']['port'])
    except (OSError, ValueError):
        pass
    return spec


def install_dropin(spec, cache, root=Path('/etc/systemd/system')):
    role = spec['name']
    # Trusted local catalog/config, not request-supplied executable paths.
    command = shlex.split(spec['command'])
    args = ['/usr/bin/python3', '/usr/local/lib/netcore-deployment/launch.py', '--service', role,
            '--config', spec['config_target'], '--cache', str(cache),
            '--runtime', f'/var/lib/netcore-discovery-{role}/config.toml', '--', *command]
    # systemd command quoting differs from shell quoting. Suppress specifier/env expansion.
    quote = lambda x: json.dumps(x.replace('%', '%%').replace('$', '$$'))
    body = ('[Service]\nExecStart=\nExecStart=' + ' '.join(quote(x) for x in args) + '\n'
            f'StateDirectory=netcore-discovery-{role}\nStateDirectoryMode=0700\n')
    atomic_write(root / (spec['unit'] + '.d') / '50-discovery.conf', body, 0o644)


class Deployer:
    def __init__(self, cfg, discovery, repository):
        self.cfg, self.discovery, self.repo = cfg, discovery, repository
        self.state = Path(cfg['state_dir'])

    def execute(self, request, log):
        # CLI runtime updates and UI jobs must not replace the same files at once.
        # Keep the mutex in the shared agent state, outside replaceable sources.
        self.state.mkdir(parents=True, exist_ok=True)
        with (self.state / 'runtime-update.mutex').open('a') as mutex:
            try:
                fcntl.flock(mutex, fcntl.LOCK_EX | fcntl.LOCK_NB)
            except BlockingIOError as exc:
                raise RuntimeError('Ein lokales Runtime-Update läuft; Deployment später erneut ausführen.') from exc
            try:
                return self._execute(request, log)
            finally:
                fcntl.flock(mutex, fcntl.LOCK_UN)

    def _execute(self, request, log):
        name, action = request['service'], request['action']
        spec = service_spec(self.cfg, name)
        target = Path(spec['config_target'])
        if action in ('update', 'restart') and not installed_service(spec):
            raise ValueError(f"Keine installierte Unit für {name}: {spec['unit']}. "
                             'Eine TOML-Datei allein genügt nicht für ein Update.')
        if action == 'restart':
            if not target.is_file():
                raise ValueError('Dienst ist nicht installiert')
            if name != 'tbs':
                install_dropin(spec, self.state / 'endpoints.json')
            run(['systemctl', 'daemon-reload'], log, timeout=30)
            run(['systemctl', 'restart', spec['unit']], log, timeout=240)
            ready = self.health(spec, log)
            return {'service': name, 'restarted': True, 'ready': ready}
        sha = request['commit']
        source = self.repo.checkout(sha, log)
        # Fail before installation if an existing source template cannot be parsed.
        if target.exists():
            tomllib.loads(target.read_text())
            backup = self.state / 'backups' / (name + '-' + str(time.time_ns()) + '.toml')
            atomic_write(backup, target.read_text())
            log('Konfigurationssicherung: ' + str(backup))
        elif action == 'update':
            raise ValueError('Update benötigt einen installierten Dienst; zuerst Installieren wählen')
        else:
            template = (source / spec['config_template']).read_text()
            if name == 'tbs':
                template_path = self.cfg.get('tbs_template')
                if not template_path:
                    raise ValueError('Für neue TBS zuerst ein geprüftes Standort-Template in tbs_template hinterlegen')
                template = tbs_config(Path(template_path).read_text(), request.get('profile', {}))
            data, _ = resolve_config(name, tomllib.loads(template), self.discovery.snapshot()['endpoints'])
            if name == 'alert-service':
                data['server']['allow_unauthenticated'] = True
                data['server']['admin_token'] = ''
            atomic_write(target, toml_dump(data), 0o644 if name != 'tbs' else 0o600)
            if name == 'alert-service' and not Path('/etc/netcore/alert-service.env').exists():
                atomic_write('/etc/netcore/alert-service.env', '# OpenLab: no management token\n')
        environment = dict(os.environ, DEBIAN_FRONTEND='noninteractive', REPO_ROOT=str(source),
                           NETCORE_GIT_COMMIT=sha, NETCORE_DISCOVERY_SKIP_INSTALL='1',
                           PATH='/root/.cargo/bin:' + os.environ.get('PATH', ''),
                           CONFIG_PATH=str(target), UNIT=spec['unit'],
                           NETCORE_TBS_COMMAND=spec['command'])
        if name == 'tbs' and action == 'install':
            environment['BINARY_PATH'] = shlex.split(spec['command'])[0]
        elif name == 'tbs':
            # The updater resolves /proc/MainPID/exe and the actual ExecStart.
            # The catalog path must never override an existing radio install.
            environment.pop('BINARY_PATH', None)
        if name == 'tbs' and action == 'update':
            installer = source / 'install/update-basisstation.sh'
        else:
            installer = source / spec['install']
        if not installer.is_file():
            raise ValueError('Installer fehlt in diesem Commit')
        run(['bash', str(ROOT / 'install/prepare-host.sh'), name], log, env=environment)
        marker = self.state / ('deployed-' + name + '.json')
        previous = json_file(marker, {}).get('commit', '')
        # Once an installer starts replacing files, the old binary version can no
        # longer be claimed. A failed/interrupted rollout remains explicitly unknown.
        atomic_write(marker, json.dumps({'service': name, 'commit': '',
            'previous_commit': previous, 'requested_commit': sha, 'state': 'installing'}), 0o644)
        if name != 'tbs' or action == 'install':
            install_dropin(spec, self.state / 'endpoints.json')
        run(['systemctl', 'daemon-reload'], log, timeout=30)
        run(['bash', str(installer)], log, source, env=environment)
        # An old process may need systemd's stop timeout (usually 90s) before
        # the fixed executable starts. Do not race that timeout at exactly 90s.
        if name != 'tbs' or action != 'update':
            run(['systemctl', 'restart', spec['unit']], log, timeout=240)
        ready = self.health(spec, log)
        atomic_write(marker, json.dumps({
            'commit': sha, 'service': name, 'state': 'installed', 'deployed_at': time.time()}), 0o644)
        self.discovery.scan()
        return {'service': name, 'commit': sha, 'live': True, 'ready': ready}

    def health(self, spec, log):
        deadline = time.monotonic() + HEALTH_TIMEOUT_SECONDS
        last = None
        live_reported = False
        while time.monotonic() < deadline:
            try:
                # The lxc installer may bind to a specific address instead of loopback.
                if probe(spec, timeout=2):
                    return True
                last = 'Dienst meldet nicht bereit'
            except (OSError, ValueError) as exc:
                last = str(exc)
            try:
                if probe(spec, timeout=2, live=True):
                    last = 'Dienst läuft; Readiness ist wegen seiner Abhängigkeiten noch eingeschränkt'
                    if not live_reported:
                        log(last + '. Warte weiter auf vollständige Readiness.')
                        live_reported = True
            except (OSError, ValueError):
                pass
            time.sleep(1)
        raise RuntimeError(f'Healthcheck fehlgeschlagen: {last}. Konfigurationssicherung liegt unter {self.state}/backups.')


def local_service_host(spec):
    try:
        config = tomllib.loads(Path(spec['config_target']).read_text())
        bind = config.get('server', {}).get('bind') or config.get('bind')
        if isinstance(bind, str) and ':' in bind:
            host = bind.rsplit(':', 1)[0]
            if host not in ('0.0.0.0', '::', '[::]'):
                return host
        elif isinstance(bind, str) and bind != '0.0.0.0':
            return bind
    except (OSError, ValueError):
        pass
    return '127.0.0.1'


def probe(spec, timeout=1, live=False):
    headers = {}
    if spec['name'] == 'tbs':
        dashboard = tomllib.loads(Path(spec['config_target']).read_text()).get('dashboard', {})
        # Existing station authentication is preserved, and only used locally.
        if dashboard.get('username') and dashboard.get('password'):
            credential = (dashboard['username'] + ':' + dashboard['password']).encode()
            headers['Authorization'] = 'Basic ' + base64.b64encode(credential).decode()
    health_path = '/health/live' if live and spec['name'] != 'tbs' else spec['health']
    result = request_json(f"http://{local_service_host(spec)}:{spec['port']}{health_path}",
                          timeout=timeout, headers=headers)
    if not isinstance(result, dict):
        return False
    if result.get('ready', result.get('ok', True)) is False:
        return False
    return result.get('status') not in ('degraded', 'failed', 'unavailable', 'not_ready', 'error')


def validate_job(data):
    if data.get('service') not in CATALOG:
        raise ValueError('Unbekannter Dienst')
    if data.get('action') not in ('install', 'update', 'restart'):
        raise ValueError('Unbekannte Aktion')
    if data.get('confirm_restart') is not True:
        raise ValueError('Die Aktion startet den ausgewählten Dienst neu; confirm_restart erforderlich')
    if data['action'] != 'restart' and not SHA.fullmatch(data.get('commit', '')):
        raise ValueError('Vollständiger Commit-SHA erforderlich')
    if data.get('profile'):
        data['profile'] = validate_profile(data['profile'])
    return {k: data[k] for k in ('service', 'action', 'commit', 'profile', 'confirm_restart') if k in data}


