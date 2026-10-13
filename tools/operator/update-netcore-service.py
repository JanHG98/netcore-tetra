#!/usr/bin/env python3
"""Prepare and verify complete installed NetCore runtime packages at one Git SHA.

Preparation builds and snapshots all code before any service stops. Application
uses those snapshots, preserves local configuration/units, and proves the new
process before publishing installation receipts. Python 3.11+, Linux/systemd.
"""
from __future__ import annotations

import argparse
import ast
from contextlib import closing, contextmanager
import hashlib
import http.client
import json
import os
from pathlib import Path, PurePosixPath
import re
import shlex
import shutil
import sqlite3
import stat
import subprocess
import sys
import tempfile
import time
import tomllib
import urllib.request

ROOT = Path(__file__).resolve().parents[2]
DISCOVERY_CONFIG = Path('/etc/netcore/discovery.toml')
RECEIPT_ROOT = Path('/var/lib/netcore-service-update')
BACKUP_ROOT = Path('/var/backups/netcore-service-update')
PROC_ROOT = Path('/proc')
SHA = re.compile(r'[0-9a-f]{40}')


def run(args, **kwargs):
    return subprocess.run(args, check=True, **kwargs)


def properties(unit):
    names = ('LoadState', 'FragmentPath', 'ActiveState', 'UnitFileState', 'ExecStart',
             'MainPID', 'InvocationID', 'ExecMainStartTimestampMonotonic')
    args = ['systemctl', 'show', unit]
    for name in names:
        args += ['-p', name]
    proc = run(args, capture_output=True, text=True)
    return dict(line.split('=', 1) for line in proc.stdout.splitlines() if '=' in line)


def loaded(props):
    return props.get('LoadState') == 'loaded' and bool(props.get('FragmentPath'))


def cargo_manifest_path(role, root=ROOT):
    if role == 'control-room':
        return root / 'bins/netcore-control-room/Cargo.toml'
    return root / 'system-backend' / role / 'Cargo.toml'


def specs(root=ROOT):
    catalog = json.loads((root / 'system-backend/deployment-core/catalog.json').read_text(encoding='utf-8'))
    result = {s['name']: s for s in catalog if s['name'] != 'tbs'}
    result['directory'] = dict(name='directory', unit='netcore-directory.service',
                              command='/opt/netcore-directory/netcore_directory-server.py',
                              port=8095, health='/api/devices', depends_on=[])
    result['discovery'] = dict(name='discovery', unit='netcore-discovery.service',
                              command='/usr/local/lib/netcore-deployment/main.py',
                              config_target='/etc/netcore/discovery.toml',
                              port=8321, health='/health/ready', depends_on=[])
    return result


def digest(path):
    with Path(path).open('rb') as stream:
        result = hashlib.sha256()
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            result.update(block)
    return result.hexdigest()


def safe_path(path):
    path = Path(path)
    if not path.is_absolute():
        raise RuntimeError('Installationspfad ist nicht absolut: ' + str(path))
    for candidate in (path, *path.parents):
        if candidate.is_symlink():
            raise RuntimeError('Installationspfad enthält einen Symlink: ' + str(candidate))
    if path.exists() and not path.is_file():
        raise RuntimeError('Installationsziel ist keine reguläre Datei: ' + str(path))
    return path


def validate_install(spec):
    props = properties(spec['unit'])
    if not loaded(props):
        raise RuntimeError('Keine installierte Unit: ' + spec['unit'])
    candidates = [shlex.split(spec['command'])[0]]
    if spec['name'] == 'directory':
        candidates = ['/opt/netcore-directory/netcore_directory-server.py',
                      '/opt/netcore-directory/netcore_directory_server.py']
    elif spec['name'] == 'alert-service':
        candidates = ['/usr/local/lib/netcore-alert-service/main.py']
    match = [p for p in candidates if re.search(r'(?<![\w/.-])' + re.escape(p)
              + r'(?=$|[\s;"\'])', props.get('ExecStart', ''))]
    if len(match) != 1:
        raise RuntimeError('Abweichender ExecStart; keine Pfade geraten: ' + spec['unit'])
    target = safe_path(match[0])
    if not target.is_file():
        raise RuntimeError('Installierte Runtime fehlt: ' + str(target))
    config = spec.get('config_target')
    if config:
        if not re.search(r'(?<![\w/.-])' + re.escape(config) + r'(?=$|[\s;"\'])',
                         props.get('ExecStart', '')):
            raise RuntimeError('Abweichender Konfigurationspfad: ' + spec['unit'])
        tomllib.loads(safe_path(config).read_text(encoding='utf-8'))
    return props, target


def owned_destination(role, primary, destination):
    """Prove package ownership without permitting arbitrary new system files."""
    destination = safe_path(destination)
    if destination.suffix in ('.toml', '.env', '.service', '.timer'):
        raise RuntimeError('Konfiguration/Unit gehört nicht zum Runtime-Dateisatz: ' + str(destination))
    if destination.is_relative_to(Path('/etc')):
        raise RuntimeError('Runtime-Update schreibt keine /etc-Dateien.')
    global_bins = (Path('/usr/local/bin'), Path('/usr/bin'))
    anchors = [] if primary.parent in global_bins else [primary.parent]
    if primary.parent.name == 'bin' and primary.parent.parent.name == 'netcore-' + role:
        anchors.append(primary.parent.parent)
    anchors += [Path('/usr/local/lib/netcore-' + role), Path('/usr/local/share/netcore-' + role)]
    exact = {primary}
    if role == 'sip-switch':
        exact |= {Path('/usr/local/bin/netcore_sip_runtime.py'),
                  Path('/var/lib/asterisk/agi-bin/netcore-sip-route.py')}
    if destination not in exact and not any(destination.is_relative_to(a) for a in anchors):
        raise RuntimeError('Neues Runtime-Ziel liegt außerhalb des nachgewiesenen Pakets: ' + str(destination))
    return destination


def runtime_assets(spec, target, root=ROOT):
    """Use complete owned source sets; do not omit newly introduced files."""
    role = spec['name']
    manifest = json.loads((root / 'tools/operator/service-update-assets.json').read_text(encoding='utf-8'))
    files = []
    if role in ('discovery', 'alert-service'):
        base = root / 'system-backend' / ('deployment-core' if role == 'discovery' else role)
        package = target.parent
        files += [(p, package / p.name) for p in sorted(base.glob('*.py'))]
        if role == 'discovery':
            files.append((base / 'catalog.json', package / 'catalog.json'))
            for name in ('prepare-host.sh', 'install-tbs.sh'):
                files.append((base / 'install' / name, package / 'install' / name))
        static_root = base / 'static'
        if not static_root.is_dir() or static_root.is_symlink():
            raise RuntimeError('Vollständige Webdateien fehlen: ' + str(static_root))
        files += [(p, package / 'static' / p.relative_to(static_root))
                  for p in sorted(static_root.rglob('*')) if p.is_file()]
    else:
        for asset in manifest['roles'].get(role, {}).get('files', []):
            source, dest = root / asset['source'], Path(asset['target'])
            if role == 'directory' and target.name == 'netcore_directory_server.py':
                source, dest = root / 'misc/ID-Server/netcore_directory_server.py', target
            files.append((source, dest))
        if role == 'media-library':
            # These are shipped operator programs; refresh them, never run migrations.
            for name in ('migrate-archive-layout.py', 'migrate-playout-config.py',
                         'diagnose-basisstation-playout.py'):
                files.append((root / 'system-backend/media-library/install' / name,
                              target.parent / name))
    seen = set()
    for source, dest in files:
        if dest in seen:
            raise RuntimeError('Doppeltes Runtime-Ziel: ' + str(dest))
        seen.add(dest)
        if not source.is_file() or source.is_symlink():
            raise RuntimeError('Runtime-Quelldatei fehlt: ' + str(source))
        owned_destination(role, target, dest)
        if source.suffix == '.py':
            ast.parse(source.read_text(encoding='utf-8'), filename=str(source))
    return files


def prepare(spec, target, root=ROOT, commit=None, prepare_build_deps=False):
    files = runtime_assets(spec, target, root)
    cargo_manifest = cargo_manifest_path(spec['name'], root)
    if cargo_manifest.is_file():
        if prepare_build_deps:
            run(['bash', str(root / 'tools/operator/prepare-netcore-build.sh'),
                 '--role', spec['name']], cwd=root)
        env = os.environ.copy()
        env['PATH'] = '/root/.cargo/bin:' + env.get('PATH', '/usr/local/bin:/usr/bin:/bin')
        if prepare_build_deps:
            env['RUSTUP_TOOLCHAIN'] = 'stable'
            env['CARGO_HOME'] = '/root/.cargo'
            env['RUSTUP_HOME'] = '/root/.rustup'
        cargo = shutil.which('cargo', path=env['PATH'])
        if not cargo:
            raise RuntimeError('Cargo fehlt; mit --prepare-build-deps vorbereiten. Dienst unverändert.')
        env['CARGO_BUILD_JOBS'] = env.get('CARGO_BUILD_JOBS', '1')
        env['CARGO_TARGET_DIR'] = '/var/cache/netcore-service-update/target'
        env['NETCORE_GIT_COMMIT'] = commit or ''
        env.pop('CARGO_BUILD_TARGET', None)
        package = tomllib.loads(cargo_manifest.read_text(encoding='utf-8'))['package']['name']
        print('Build vor Dienststopp: ' + package, flush=True)
        run([cargo, 'build', '--locked', '--release', '-p', package, '--bin', target.name],
            cwd=root, env=env)
        binary = Path(env['CARGO_TARGET_DIR']) / 'release' / target.name
        if not binary.is_file():
            raise RuntimeError('Build lieferte kein erwartetes Binary: ' + str(binary))
        with binary.open('rb') as stream:
            header = stream.read(20)
        with target.open('rb') as stream:
            old_header = stream.read(20)
        if (header[:4] != b'\x7fELF' or old_header[:4] != b'\x7fELF'
                or header[4:6] != old_header[4:6] or header[18:20] != old_header[18:20]):
            raise RuntimeError('Binaryformat/Architektur passt nicht zur vorhandenen Installation.')
        files.insert(0, (binary, target))
    if not files:
        raise RuntimeError('Kein geprüfter Runtime-Updateweg: ' + spec['name'])
    if target not in [dest for _, dest in files]:
        raise RuntimeError('Primäre Runtime fehlt im vollständigen Quellpaket: ' + str(target))
    return files


def fsync_directory(directory):
    if hasattr(os, 'O_DIRECTORY'):
        fd = os.open(directory, os.O_RDONLY | os.O_DIRECTORY)
        try:
            os.fsync(fd)
        finally:
            os.close(fd)


def atomic_json(path, data, mode=0o644):
    path = safe_path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    fd, name = tempfile.mkstemp(prefix='.netcore-json-', dir=path.parent)
    try:
        with os.fdopen(fd, 'w', encoding='utf-8') as stream:
            json.dump(data, stream, ensure_ascii=False, indent=2)
            stream.write('\n')
            stream.flush()
            os.fsync(stream.fileno())
        os.chmod(name, mode)
        os.replace(name, path)
        fsync_directory(path.parent)
    finally:
        if os.path.exists(name):
            os.unlink(name)


def prepared_path(spec, root=ROOT):
    return root / '.netcore-prepared' / (spec['name'] + '.json')


def prepare_plan(spec, target, props, commit, root=ROOT, prepare_build_deps=False):
    if not SHA.fullmatch(commit):
        raise ValueError('Vollständiger Git-SHA erforderlich')
    files = prepare(spec, target, root, commit, prepare_build_deps)
    # New package files inherit the proven runtime's owner, including installations
    # with a dedicated service account. Existing destinations keep their own metadata.
    primary_info = target.stat()
    prepared_root = root / '.netcore-prepared'
    prepared_root.mkdir(mode=0o700, parents=True, exist_ok=True)
    stage = Path(tempfile.mkdtemp(prefix=spec['name'] + '-', dir=prepared_root))
    records = []
    try:
        for index, (source, dest) in enumerate(files):
            safe_path(dest)
            snapshot = stage / (str(index) + '-' + dest.name)
            shutil.copyfile(source, snapshot)
            snapshot.chmod(0o400)
            existing = dest.is_file()
            info = dest.stat() if existing else None
            with source.open('rb') as stream:
                executable_script = stream.read(2) == b'#!'
            mode = stat.S_IMODE(info.st_mode) if info else (0o755 if executable_script else 0o644)
            records.append(dict(target=str(dest), snapshot=str(snapshot.relative_to(root)),
                                sha256=digest(snapshot), previous_sha256=digest(dest) if existing else None,
                                created=not existing, mode=mode,
                                uid=info.st_uid if info else primary_info.st_uid,
                                gid=info.st_gid if info else primary_info.st_gid))
        primary = next(r for r in records if r['target'] == str(target))
        with (root / primary['snapshot']).open('rb') as stream:
            kind = 'elf' if stream.read(4) == b'\x7fELF' else 'python'
        plan = dict(version=2, service=spec['name'], unit=spec['unit'], source_commit=commit,
                    assets_complete=True, runtime_kind=kind, primary_runtime=str(target),
                    config_target=spec.get('config_target'),
                    config_sha256=digest(spec['config_target']) if spec.get('config_target') else None,
                    exec_start=props.get('ExecStart'), prepared_at=time.time(), files=records)
        atomic_json(prepared_path(spec, root), plan, mode=0o400)
        return plan
    except BaseException:
        shutil.rmtree(stage)
        raise


def validate_plan(spec, plan, commit, root=ROOT):
    if (plan.get('version') != 2 or plan.get('service') != spec['name']
            or plan.get('source_commit') != commit or plan.get('assets_complete') is not True):
        raise RuntimeError('Vorbereiteter Plan gehört nicht zu Dienst/Quellcommit.')
    prepared_root = (root / '.netcore-prepared').resolve()
    targets = set()
    for asset in plan['files']:
        snapshot = root / asset['snapshot']
        if not snapshot.resolve().is_relative_to(prepared_root) or snapshot.is_symlink():
            raise RuntimeError('Snapshot liegt außerhalb des vorbereiteten Pakets.')
        if digest(snapshot) != asset['sha256']:
            raise RuntimeError('Vorbereiteter Snapshot wurde verändert: ' + str(snapshot))
        target = safe_path(asset['target'])
        if str(target) in targets:
            raise RuntimeError('Doppeltes Ziel im vorbereiteten Plan.')
        targets.add(str(target))
        before = digest(target) if target.exists() else None
        if before != asset['previous_sha256']:
            raise RuntimeError('Installationsziel seit Vorbereitung verändert: ' + str(target))
    config = plan.get('config_target')
    if config and digest(safe_path(config)) != plan['config_sha256']:
        raise RuntimeError('Konfiguration seit Vorbereitung verändert; erneut vorbereiten.')


def state_path(spec):
    config = Path(spec['config_target']) if spec['name'] == 'discovery' else DISCOVERY_CONFIG
    if not config.is_file():
        return None
    data = tomllib.loads(config.read_text(encoding='utf-8'))
    # Match the established agent load_config default for older valid configs.
    value = data.get('state_dir', '/var/lib/netcore-discovery')
    if (not isinstance(value, str) or not value
            or not (Path(value).is_absolute() or PurePosixPath(value).is_absolute())):
        raise RuntimeError('Discovery state_dir fehlt oder ist ungültig; kein Pfad geraten.')
    return Path(value)


def busy_jobs(state):
    if state is None or not (state / 'jobs.sqlite3').is_file():
        return []
    db = state / 'jobs.sqlite3'
    with closing(sqlite3.connect(db.resolve().as_uri() + '?mode=ro', uri=True, timeout=3)) as connection:
        return connection.execute("SELECT id,status FROM jobs WHERE status IN ('queued','running')").fetchall()


@contextmanager
def agent_guard(spec):
    state = state_path(spec)
    if state is None:
        yield None
        return
    import fcntl
    state.mkdir(parents=True, exist_ok=True)
    with (state / 'runtime-update.mutex').open('a') as lock:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        jobs = busy_jobs(state)
        if jobs:
            raise RuntimeError('Deployment-Aufträge aktiv: ' + ', '.join(str(row[0]) for row in jobs))
        yield state


def replace_file(source, dest, mode=0o644, uid=0, gid=0):
    dest = safe_path(dest)
    if dest.exists():
        info = dest.stat()
        mode, uid, gid = stat.S_IMODE(info.st_mode), info.st_uid, info.st_gid
    fd, name = tempfile.mkstemp(prefix='.netcore-update-', dir=dest.parent)
    try:
        with os.fdopen(fd, 'wb') as stream, Path(source).open('rb') as src:
            shutil.copyfileobj(src, stream)
            if hasattr(os, 'fchown'):
                os.fchown(stream.fileno(), uid, gid)
            os.chmod(name, mode)
            stream.flush()
            os.fsync(stream.fileno())
        os.replace(name, dest)
        fsync_directory(dest.parent)
    finally:
        if os.path.exists(name):
            os.unlink(name)


def clear_owned_bytecode(plan):
    for asset in plan['files']:
        source = Path(asset['target'])
        if source.suffix != '.py':
            continue
        cache = source.parent / '__pycache__'
        if cache.is_dir() and not cache.is_symlink():
            for bytecode in cache.glob(source.stem + '.*.pyc'):
                if bytecode.is_file() and not bytecode.is_symlink():
                    bytecode.unlink()
        legacy = source.with_suffix('.pyc')
        if legacy.is_file() and not legacy.is_symlink():
            legacy.unlink()


def check_http(spec, ready=False):
    cfg = tomllib.loads(Path(spec['config_target']).read_text(encoding='utf-8')) if spec.get('config_target') else {}
    bind = cfg.get('server', {}).get('bind') or cfg.get('bind', '127.0.0.1')
    host, port = '127.0.0.1', spec['port']
    if isinstance(bind, str) and ':' in bind:
        host, configured_port = bind.rsplit(':', 1)
        port = int(configured_port)
    elif isinstance(bind, str):
        host = bind
    port = cfg.get('server', {}).get('port', cfg.get('port', port))
    if host in ('0.0.0.0', '::', '[::]'):
        host = '127.0.0.1'
    if ':' in host and not host.startswith('['):
        host = '[' + host + ']'
    path = spec['health'] if ready or spec['name'] == 'directory' else '/health/live'
    opener = urllib.request.build_opener(urllib.request.ProxyHandler({}))
    with opener.open(f'http://{host}:{port}{path}', timeout=3) as response:
        body = response.read(1024 * 1024 + 1)
        if len(body) > 1024 * 1024:
            raise ValueError('Health-Antwort zu groß')
        data = json.loads(body)
    if spec['name'] == 'directory':
        return isinstance(data, (dict, list))
    return (isinstance(data, dict) and data.get('ready', data.get('ok', True)) is not False
            and data.get('status') not in ('degraded', 'failed', 'unavailable', 'not_ready', 'error'))


def wait_live(spec, attempts=15):
    for _ in range(attempts):
        if properties(spec['unit']).get('ActiveState') == 'active':
            try:
                if check_http(spec):
                    return
            except (OSError, ValueError, http.client.HTTPException):
                pass
        time.sleep(1)
    raise RuntimeError('Dienst-/HTTP-Liveness nach Austausch fehlgeschlagen: ' + spec['name'])


def verify_installed(plan):
    for asset in plan['files']:
        if digest(asset['target']) != asset['sha256']:
            raise RuntimeError('Installierter Hash stimmt nicht: ' + asset['target'])
    config = plan.get('config_target')
    if config and digest(config) != plan['config_sha256']:
        raise RuntimeError('Konfiguration wurde während des Updates verändert.')


def verify_process(spec, plan, previous):
    props = properties(spec['unit'])
    pid, invocation = props.get('MainPID', ''), props.get('InvocationID', '')
    if (props.get('ActiveState') != 'active' or not pid.isdigit() or int(pid) <= 0
            or not re.fullmatch(r'[0-9a-f]{32}', invocation)):
        raise RuntimeError('Kein nachgewiesener laufender Dienstprozess: ' + spec['name'])
    if previous.get('InvocationID') and invocation == previous['InvocationID']:
        raise RuntimeError('Alter Dienstprozess wurde nicht ersetzt: ' + spec['name'])
    proc = PROC_ROOT / pid
    command = proc.joinpath('cmdline').read_bytes().split(b'\0')
    primary = plan['primary_runtime']
    proof = dict(active=True, MainPID=int(pid), InvocationID=invocation,
                 previous_invocation=previous.get('InvocationID', ''),
                 kind=plan['runtime_kind'], command=[v.decode('utf-8', 'replace') for v in command if v])
    if plan['runtime_kind'] == 'elf':
        expected = next(f['sha256'] for f in plan['files'] if f['target'] == primary)
        proof['exe_sha256'] = digest(proc / 'exe')
        if proof['exe_sha256'] != expected:
            raise RuntimeError('Laufendes ELF entspricht nicht dem vorbereiteten Binary.')
    elif os.fsencode(primary) not in command:
        raise RuntimeError('Laufender Python-Prozess verwendet nicht das geprüfte Hauptskript.')
    verify_installed(plan)
    again = properties(spec['unit'])
    if (again.get('MainPID'), again.get('InvocationID'), again.get('ActiveState')) != (pid, invocation, 'active'):
        raise RuntimeError('Dienstprozess wechselte während des Nachweises.')
    return proof


def verify_source_reference(root, commit):
    """CLI accepts a checked Git checkout or the host's verified transport receipt."""
    if (root / '.git').exists():
        actual = run(['git', '-C', str(root), 'rev-parse', 'HEAD'], capture_output=True, text=True).stdout.strip()
        dirty = run(['git', '-C', str(root), 'status', '--porcelain', '--untracked-files=no'],
                    capture_output=True, text=True).stdout.strip()
        if actual != commit or dirty:
            raise RuntimeError('Quellcheckout entspricht nicht dem unveränderten angegebenen Git-SHA.')
        return
    path = root / '.netcore-source.json'
    data = json.loads(path.read_text(encoding='utf-8')) if path.is_file() else {}
    if data.get('commit') != commit or not re.fullmatch(r'[0-9a-f]{64}', data.get('archive_sha256', '')):
        raise RuntimeError('Quellpaket besitzt keinen zum Git-SHA passenden Transportnachweis.')


def receipt_write(spec, receipt, receipt_root=None, state=None):
    path = (receipt_root or RECEIPT_ROOT) / (spec['name'] + '.json')
    atomic_json(path, receipt)
    if state is not None:
        marker = dict(service=spec['name'], commit=receipt.get('source_commit', '')
                      if receipt.get('verified') else '', requested_commit=receipt.get('requested_commit',
                      receipt.get('source_commit', '')), state='installed' if receipt.get('verified')
                      else receipt.get('phase', 'unknown'), deployed_at=time.time(),
                      receipt_path=str(path), files=receipt.get('files', []), process=receipt.get('process'))
        atomic_json(state / ('deployed-' + spec['name'] + '.json'), marker)
    return path


def apply_plan(spec, plan, props, root=ROOT, backup_root=None, receipt_root=None):
    validate_plan(spec, plan, plan['source_commit'], root)
    with agent_guard(spec) as state:
        return _apply_plan(spec, plan, props, state, root,
                           backup_root or BACKUP_ROOT, receipt_root or RECEIPT_ROOT)


def _apply_plan(spec, plan, props, state, root, backup_root, receipt_root):
    current = properties(spec['unit'])
    before_state = current.get('ActiveState')
    if before_state in ('activating', 'deactivating', 'reloading'):
        raise RuntimeError('Dienststatus wechselt noch; Update erneut ausführen: ' + spec['name'])
    restart_main = before_state in ('active', 'failed')
    units = [spec['unit']]
    if spec['name'] == 'observability' and any(Path(a['target']).name == 'log_store.py' for a in plan['files']):
        units += [u for u in ('netcore-syslog-preview.service', 'netcore-syslog-archive.timer',
                             'netcore-syslog-archive.service') if loaded(properties(u))]
    active = [u for u in units if properties(u).get('ActiveState') == 'active']
    restart = list(active)
    if restart_main and spec['unit'] not in restart:
        restart.insert(0, spec['unit'])
    backup_root.mkdir(mode=0o700, parents=True, exist_ok=True)
    backup = Path(tempfile.mkdtemp(prefix=spec['name'] + '-', dir=backup_root))
    saved, new_files, created_dirs = [], [], []
    for asset in plan['files']:
        dest = Path(asset['target'])
        if not asset['created']:
            old = backup / str(len(saved))
            shutil.copy2(dest, old)
            saved.append((old, dest))
        else:
            new_files.append(dest)
    atomic_json(backup / 'manifest.json', dict(service=spec['name'], source_commit=plan['source_commit'],
                 files=plan['files'], active_before=before_state))
    invalid = dict(service=spec['name'], phase='installing', source_commit='',
                   requested_commit=plan['source_commit'], verified=False)
    receipt_write(spec, invalid, receipt_root, state)
    try:
        for unit in sorted(active, key=lambda u: not u.endswith('.timer')):
            run(['systemctl', 'stop', unit])
        for asset in plan['files']:
            dest = Path(asset['target'])
            missing = []
            parent = dest.parent
            while not parent.exists():
                missing.append(parent)
                parent = parent.parent
            for directory in reversed(missing):
                directory.mkdir(mode=0o755)
                created_dirs.append(directory)
            replace_file(root / asset['snapshot'], dest, asset['mode'], asset['uid'], asset['gid'])
        clear_owned_bytecode(plan)
        verify_installed(plan)
        if before_state == 'failed':
            run(['systemctl', 'reset-failed', spec['unit']])
        for unit in restart:
            run(['systemctl', 'start', unit])
        if restart_main:
            wait_live(spec)
            process = verify_process(spec, plan, current)
            live = True
            try:
                ready = check_http(spec, ready=True)
            except (OSError, ValueError, http.client.HTTPException):
                ready = False
        else:
            process = dict(active=False, MainPID=0, InvocationID='',
                           previous_invocation=current.get('InvocationID', ''), kind=plan['runtime_kind'])
            live, ready = None, None
        verify_installed(plan)
        files = [{k: a[k] for k in ('target', 'sha256', 'previous_sha256', 'created')} for a in plan['files']]
        phase = 'updated_degraded' if ready is False else 'updated' if restart_main else 'inactive_updated'
        receipt = dict(service=spec['name'], phase=phase, source_commit=plan['source_commit'],
                       assets_complete=True, verified=True, files=files, process=process,
                       liveness=live, readiness=ready, active_before=before_state,
                       backup=str(backup), verified_at=time.time())
        receipt['receipt_path'] = str(receipt_root / (spec['name'] + '.json'))
        receipt_write(spec, receipt, receipt_root, state)
        return receipt
    except BaseException as error:
        failures = []
        for unit in sorted(restart, key=lambda u: not u.endswith('.timer')):
            try:
                run(['systemctl', 'stop', unit])
            except Exception as exc:
                failures.append(str(exc))
        for source, dest in saved:
            try:
                replace_file(source, dest)
            except Exception as exc:
                failures.append(str(exc))
        for dest in new_files:
            try:
                dest.unlink(missing_ok=True)
            except Exception as exc:
                failures.append(str(exc))
        for directory in reversed(created_dirs):
            try:
                directory.rmdir()
            except OSError:
                pass
        if before_state == 'failed':
            try:
                run(['systemctl', 'reset-failed', spec['unit']])
            except Exception as exc:
                failures.append(str(exc))
        for unit in restart:
            try:
                run(['systemctl', 'start', unit])
            except Exception as exc:
                failures.append(str(exc))
        failed = dict(invalid, phase='rollback_failed' if failures else 'rolled_back',
                      error=str(error), rollback_errors=failures, backup=str(backup))
        try:
            receipt_write(spec, failed, receipt_root, state)
        except Exception as exc:
            print('Versionsmarker konnte nicht zurückgesetzt werden: ' + str(exc), file=sys.stderr)
        print('Rückweg: ' + str(backup) + ('; FEHLER: ' + '; '.join(failures)
                      if failures else '; alte Dateien wiederhergestellt, Versionsnachweis unbekannt'), flush=True)
        raise


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--service', required=True, choices=sorted(specs()))
    parser.add_argument('--commit', required=True, help='Einmal aufgelöster vollständiger Git-SHA')
    action = parser.add_mutually_exclusive_group()
    action.add_argument('--prepare', action='store_true', help='Bauen, prüfen und unveränderliche Snapshots vorbereiten')
    action.add_argument('--apply', action='store_true', help='Vorbereitetes vollständiges Runtime-Paket übernehmen')
    parser.add_argument('--prepare-build-deps', action='store_true',
                        help='Fehlende Root-Rust-/Buildvoraussetzungen vor dem Build ergänzen')
    args = parser.parse_args(argv)
    if not SHA.fullmatch(args.commit):
        raise ValueError('Vollständiger Git-SHA erforderlich')
    if os.geteuid() != 0:
        raise RuntimeError('Auf dem Dienstserver als root/sudo ausführen.')
    verify_source_reference(ROOT, args.commit)
    import fcntl
    spec = specs()[args.service]
    with open('/run/lock/netcore-service-update.lock', 'a') as lock:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        props, target = validate_install(spec)
        if not args.apply and not args.prepare:
            print(json.dumps(dict(service=args.service, phase='planned', source_commit=args.commit,
                                  target=str(target), active=props.get('ActiveState'))))
            return 0
        path = prepared_path(spec)
        if args.prepare or not path.is_file():
            plan = prepare_plan(spec, target, props, args.commit, prepare_build_deps=args.prepare_build_deps)
        else:
            plan = json.loads(path.read_text(encoding='utf-8'))
        validate_plan(spec, plan, args.commit)
        if args.prepare:
            print(json.dumps(dict(service=args.service, phase='prepared', source_commit=args.commit,
                 assets_complete=True, plan_path=str(path), files=[{k: a[k] for k in
                 ('target', 'sha256', 'previous_sha256', 'created')} for a in plan['files']]), ensure_ascii=False), flush=True)
            return 0
        if props.get('ExecStart') != plan.get('exec_start'):
            raise RuntimeError('Effektive Unit seit Vorbereitung verändert; erneut vorbereiten.')
        result = apply_plan(spec, plan, props)
        print(json.dumps(result, ensure_ascii=False), flush=True)
        return 2 if result.get('readiness') is False else 0


if __name__ == '__main__':
    try:
        raise SystemExit(main())
    except Exception as exc:
        print('Update abgebrochen: ' + str(exc), file=sys.stderr)
        raise SystemExit(1)
