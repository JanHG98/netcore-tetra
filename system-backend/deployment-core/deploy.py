"""Fixed service installers, commit-pinned Git deployments and TBS provisioning."""
import copy
import codecs
import base64
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

from bindings import resolve_config
from common import CATALOG, ROOT, SHA, atomic_write, identifier, json_file, request_json, toml_dump


def run(command, log, cwd=None, timeout=3600, env=None):
    log('$ ' + shlex.join(map(str, command)))
    with tempfile.TemporaryFile() as output:
        process = subprocess.Popen(command, cwd=cwd, env=env, stdin=subprocess.DEVNULL,
                                   stdout=output, stderr=subprocess.STDOUT, start_new_session=True)
        deadline = time.monotonic() + timeout
        position = 0
        decoder = codecs.getincrementaldecoder('utf-8')(errors='replace')
        pending = ''
        try:
            while True:
                chunk = os.pread(output.fileno(), 65536, position)
                position += len(chunk)
                if chunk:
                    pending += decoder.decode(chunk)
                    if '\n' in pending:
                        complete, pending = pending.rsplit('\n', 1)
                        log(complete)
                code = process.poll()
                if code is not None and not chunk:
                    pending += decoder.decode(b'', final=True)
                    if pending:
                        log(pending)
                    if code:
                        raise RuntimeError(f'Command exited with {code}: {command[0]}')
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
        self.path = Path(cfg['state_dir']) / 'source'
        self.lock = threading.RLock()

    def fetch(self, log):
        if not (self.path / '.git').exists():
            run(['git', 'clone', '--no-checkout', self.url, str(self.path)], log, timeout=300)
        run(['git', 'fetch', '--prune', 'origin', '+refs/heads/*:refs/remotes/origin/*',
             '+refs/tags/*:refs/tags/*'], log, self.path, timeout=300)

    def resolve(self, ref, log):
        if not isinstance(ref, str) or not re.fullmatch(r'[A-Za-z0-9][A-Za-z0-9_./-]{0,127}', ref) or '..' in ref:
            raise ValueError('Ungültiger Git-Ref')
        with self.lock:
            self.fetch(log)
            options = [ref] if SHA.fullmatch(ref) else [f'refs/remotes/origin/{ref}', f'refs/tags/{ref}']
            for value in options:
                p = subprocess.run(['git', 'rev-parse', '--verify', value + '^{commit}'], cwd=self.path,
                                   capture_output=True, text=True, timeout=10)
                if p.returncode == 0 and SHA.fullmatch(p.stdout.strip()):
                    return p.stdout.strip()
        raise ValueError('Branch, Tag oder Commit nicht gefunden')

    def checkout(self, sha, log):
        if not SHA.fullmatch(sha):
            raise ValueError('Deployment requires a full commit SHA')
        with self.lock:
            self.fetch(log)
            # This is a dedicated managed clone, never the operator's working tree.
            dirty = subprocess.check_output(['git', 'status', '--porcelain', '--untracked-files=no'],
                                            cwd=self.path, text=True)
            # A first --no-checkout clone reports staged deletions. Its index does
            # not exist yet and is safe to populate. Later edits must not be lost.
            if (self.path / '.git/index').exists() and dirty:
                raise RuntimeError('Managed checkout contains local changes; refusing to overwrite them')
            run(['git', 'checkout', '--detach', sha], log, self.path, timeout=30)
            actual = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=self.path, text=True).strip()
            if actual != sha:
                raise RuntimeError('Commit verification failed')
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


def service_spec(cfg, name):
    spec = dict(CATALOG[name])
    for override in cfg['services']:
        if override['name'] == name:
            for key in ('config_target', 'unit', 'command', 'port', 'health'):
                if key in override:
                    spec[key] = override[key]
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
        name, action = request['service'], request['action']
        spec = service_spec(self.cfg, name)
        target = Path(spec['config_target'])
        if action == 'restart':
            if not target.is_file():
                raise ValueError('Dienst ist nicht installiert')
            install_dropin(spec, self.state / 'endpoints.json')
            run(['systemctl', 'daemon-reload'], log, timeout=30)
            run(['systemctl', 'restart', spec['unit']], log, timeout=90)
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
        if name == 'tbs':
            environment['BINARY_PATH'] = shlex.split(spec['command'])[0]
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
        install_dropin(spec, self.state / 'endpoints.json')
        run(['systemctl', 'daemon-reload'], log, timeout=30)
        run(['bash', str(installer)], log, source, env=environment)
        run(['systemctl', 'restart', spec['unit']], log, timeout=90)
        ready = self.health(spec, log)
        atomic_write(marker, json.dumps({
            'commit': sha, 'service': name, 'state': 'installed', 'deployed_at': time.time()}), 0o644)
        self.discovery.scan()
        return {'service': name, 'commit': sha, 'live': True, 'ready': ready}

    def health(self, spec, log):
        deadline = time.monotonic() + 60
        last = None
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
                    log('Dienst läuft; Readiness ist wegen seiner Abhängigkeiten noch eingeschränkt.')
                    return False
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
