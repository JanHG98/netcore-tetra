#!/usr/bin/env python3
"""Update installed NetCore runtime files; keep units, configuration and data.

Python 3.11+, standard library. Build/compile completes before the first stop.
This deliberately does not run installers, seeds, archive or Piper migrations.
"""
from __future__ import annotations

import argparse
import ast
import hashlib
import http.client
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import stat
import subprocess
import sys
import tempfile
import time
import tomllib
import urllib.error
import urllib.request

ROOT = Path(__file__).resolve().parents[2]


def run(args, **kwargs):
    return subprocess.run(args, check=True, **kwargs)


def properties(unit):
    result = run(['systemctl', 'show', unit, '-p', 'LoadState', '-p',
                  'FragmentPath', '-p', 'ActiveState', '-p', 'UnitFileState', '-p', 'ExecStart'],
                 capture_output=True, text=True)
    return dict(line.split('=', 1) for line in result.stdout.splitlines() if '=' in line)


def loaded(props):
    return props.get('LoadState') == 'loaded' and bool(props.get('FragmentPath'))


def cargo_manifest_path(role, root=ROOT):
    # Control Room's Rust server is distinct from its Python/UI source directory.
    if role == 'control-room':
        return root / 'bins/netcore-control-room/Cargo.toml'
    return root / 'system-backend' / role / 'Cargo.toml'


def specs(root=ROOT):
    catalog = json.loads((root / 'system-backend/deployment-core/catalog.json').read_text())
    result = {s['name']: s for s in catalog if s['name'] != 'tbs'}
    result['directory'] = dict(name='directory', unit='netcore-directory.service',
                              command='/opt/netcore-directory/netcore_directory-server.py',
                              port=8095, health='/api/devices', depends_on=[])
    result['discovery'] = dict(name='discovery', unit='netcore-discovery.service',
                              command='/usr/local/lib/netcore-deployment/main.py',
                              config_target='/etc/netcore/discovery.toml',
                              port=8321, health='/health/ready', depends_on=[])
    return result


def validate_install(spec):
    props = properties(spec['unit'])
    if not loaded(props):
        raise RuntimeError('Keine installierte Unit: ' + spec['unit'])
    if spec['name'] == 'directory':
        candidates = ['/opt/netcore-directory/netcore_directory-server.py',
                      '/opt/netcore-directory/netcore_directory_server.py']
    else:
        candidates = [shlex.split(spec['command'])[0]]
        if spec['name'] == 'alert-service':
            candidates = ['/usr/local/lib/netcore-alert-service/main.py']
    matched = [p for p in candidates if re.search(
        r'(?<![\w/.-])' + re.escape(p) + r'(?=$|[\s;"\'])', props.get('ExecStart', ''))]
    if len(matched) != 1:
        raise RuntimeError('Abweichender ExecStart; keine Pfade geraten: ' + spec['unit'])
    target = Path(matched[0])
    if target.is_symlink() or not target.is_file():
        raise RuntimeError('Runtime fehlt oder ist ein Symlink: ' + str(target))
    config = spec.get('config_target')
    if config:
        if not re.search(r'(?<![\w/.-])' + re.escape(config) + r'(?=$|[\s;"\'])',
                         props.get('ExecStart', '')):
            raise RuntimeError('Abweichender Konfigurationspfad: ' + spec['unit'])
        if Path(config).is_symlink():
            raise RuntimeError('Konfiguration ist ein Symlink: ' + config)
        tomllib.loads(Path(config).read_text())
    return props, target


def prepare(spec, target, root=ROOT):
    manifest = json.loads((root / 'tools/operator/service-update-assets.json').read_text())
    role = spec['name']
    files = []
    if role == 'discovery':
        base = root / 'system-backend/deployment-core'
        files = [(p, Path('/usr/local/lib/netcore-deployment') / p.name)
                 for p in sorted(base.glob('*.py'))] + [(base / 'catalog.json',
                         Path('/usr/local/lib/netcore-deployment/catalog.json'))]
        for sub in ('static', 'install'):
            names = ('prepare-host.sh', 'install-tbs.sh') if sub == 'install' else None
            files.extend((p, Path('/usr/local/lib/netcore-deployment') / sub / p.name)
                         for p in sorted((base / sub).iterdir()) if p.is_file()
                         and (names is None or p.name in names)
                         and (Path('/usr/local/lib/netcore-deployment') / sub / p.name).is_file())
    else:
        for f in manifest['roles'].get(role, {}).get('files', []):
            source, dest = root / f['source'], Path(f['target'])
            if role == 'directory' and target.name == 'netcore_directory_server.py':
                source = root / 'misc/ID-Server/netcore_directory_server.py'
                dest = target
            if role == 'observability' and not dest.exists():
                continue  # A logging stack is an optional existing installation.
            files.append((source, dest))
    cargo_manifest = cargo_manifest_path(role, root)
    if cargo_manifest.is_file():
        env = os.environ.copy()
        env['PATH'] = '/root/.cargo/bin:' + env.get('PATH', '/usr/local/bin:/usr/bin:/bin')
        cargo = shutil.which('cargo', path=env['PATH'])
        if not cargo:
            raise RuntimeError('Cargo fehlt unter root; laufender Dienst unver�ndert.')
        env['CARGO_BUILD_JOBS'] = env.get('CARGO_BUILD_JOBS', '1')
        env['CARGO_TARGET_DIR'] = '/var/cache/netcore-service-update/target'
        package = tomllib.loads(cargo_manifest.read_text())['package']['name']
        print('Build vor Dienststopp: ' + package, flush=True)
        env.pop('CARGO_BUILD_TARGET', None)
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
        raise RuntimeError('Kein gepr�fter Runtime-Updateweg: ' + role)
    for source, dest in files:
        if not source.is_file() or dest.is_symlink() or not dest.is_file():
            raise RuntimeError('Abweichende Installation oder fehlende Quelldatei: ' + str(dest))
        if source.suffix == '.py':
            ast.parse(source.read_text(), filename=str(source))
    return files


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def replace_file(source, dest):
    """Replace a regular file on the same filesystem and preserve its metadata."""
    info = dest.stat()
    if dest.is_symlink() or not stat.S_ISREG(info.st_mode):
        raise RuntimeError('Kein regul�res Installationsziel: ' + str(dest))
    fd, name = tempfile.mkstemp(prefix='.netcore-update-', dir=dest.parent)
    try:
        with os.fdopen(fd, 'wb') as stream, source.open('rb') as src:
            shutil.copyfileobj(src, stream)
            os.fchmod(stream.fileno(), stat.S_IMODE(info.st_mode))
            os.fchown(stream.fileno(), info.st_uid, info.st_gid)
            stream.flush()
            os.fsync(stream.fileno())
        os.replace(name, dest)
    finally:
        if os.path.exists(name):
            os.unlink(name)


def check_http(spec, ready=False):
    cfg = tomllib.loads(Path(spec['config_target']).read_text()) if spec.get('config_target') else {}
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
            raise ValueError('Health-Antwort zu gro�')
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


def transaction(spec, files, props, backup_root=Path('/var/backups/netcore-service-update')):
    changed = [(s, d) for s, d in files if digest(s) != digest(d)]
    units = [spec['unit']]
    if spec['name'] == 'observability' and any(d.name == 'log_store.py' for _, d in changed):
        units += [u for u in ('netcore-syslog-preview.service', 'netcore-syslog-archive.timer',
                             'netcore-syslog-archive.service') if loaded(properties(u))]
    # Re-read after the build: an operator may have stopped the service meanwhile.
    states = {u: properties(u).get('ActiveState') for u in units}
    before_state = states[spec['unit']]
    if any(state in ('activating', 'deactivating', 'reloading') for state in states.values()):
        raise RuntimeError('Dienststatus wechselt noch; Update erneut ausf�hren: ' + spec['name'])
    active = [u for u in units if states[u] == 'active']
    recover_failed = before_state == 'failed'
    start_units = list(active)
    if recover_failed:
        # Failed is a crashed service, unlike a deliberately inactive/disabled one.
        # Clear systemd's failure rate limit, without changing enable/disable state.
        start_units.insert(0, spec['unit'])
    if not changed:
        if recover_failed:
            run(['systemctl', 'reset-failed', spec['unit']])
            run(['systemctl', 'start', spec['unit']])
            wait_live(spec)
        live, ready = None, None
        if start_units:
            try:
                live = check_http(spec)
                ready = check_http(spec, ready=True) if live else False
            except (OSError, ValueError, http.client.HTTPException):
                live, ready = False, False
        phase = 'recovered' if recover_failed else 'already_current'
        if ready is False:
            phase += '_degraded'
        return dict(service=spec['name'], phase=phase, liveness=live,
                    readiness=ready, active_before=before_state,
                    recovered_failed_service=recover_failed)
    config = Path(spec['config_target']) if spec.get('config_target') else None
    before_config = digest(config) if config else None
    backup_root.mkdir(parents=True, exist_ok=True, mode=0o700)
    backup = Path(tempfile.mkdtemp(prefix=spec['name'] + '-', dir=backup_root))
    saved = []
    for _, dest in changed:
        stored = backup / dest.relative_to(dest.anchor)
        stored.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(dest, stored)
        saved.append((stored, dest))
    (backup / 'manifest.json').write_text(json.dumps({
        'service': spec['name'], 'active_units': active, 'active_before': before_state,
        'recovery_requested': recover_failed,
        'files': [{'target': str(d), 'sha256': digest(d)} for _, d in changed]}, indent=2))
    try:
        # Stop timers before their service to prevent a concurrent archive launch.
        for unit in sorted(active, key=lambda u: not u.endswith('.timer')):
            run(['systemctl', 'stop', unit])
        for source, dest in changed:
            replace_file(source, dest)
        if config and digest(config) != before_config:
            raise RuntimeError('Konfiguration wurde w�hrend des Updates extern ver�ndert.')
        if recover_failed:
            run(['systemctl', 'reset-failed', spec['unit']])
        for unit in start_units:
            run(['systemctl', 'start', unit])
        if spec['unit'] in start_units:
            wait_live(spec)
    except BaseException:
        rollback_errors = []
        for unit in sorted(start_units, key=lambda u: not u.endswith('.timer')):
            try:
                run(['systemctl', 'stop', unit])
            except Exception as exc:
                rollback_errors.append(str(exc))
        for source, dest in saved:
            try:
                replace_file(source, dest)
            except Exception as exc:
                rollback_errors.append(str(exc))
        if recover_failed:
            try:
                run(['systemctl', 'reset-failed', spec['unit']])
            except Exception as exc:
                rollback_errors.append(str(exc))
        for unit in start_units:
            try:
                run(['systemctl', 'start', unit])
            except Exception as exc:
                rollback_errors.append(str(exc))
        print('R�ckweg: ' + str(backup) + ('; FEHLER: ' + '; '.join(rollback_errors)
                                       if rollback_errors else '; alte Dateien wiederhergestellt'), flush=True)
        raise
    ready = None
    if spec['unit'] in start_units:
        try:
            ready = check_http(spec, ready=True)
        except (OSError, ValueError, http.client.HTTPException):
            ready = False
    return dict(service=spec['name'], phase='updated' if ready is not False else 'updated_degraded',
                readiness=ready, backup=str(backup), files=len(changed),
                liveness=True if spec['unit'] in start_units else None,
                active_before=before_state, recovered_failed_service=recover_failed)


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--service', required=True, choices=sorted(specs()))
    parser.add_argument('--apply', action='store_true', help='Runtime tats�chlich aktualisieren')
    args = parser.parse_args(argv)
    spec = specs()[args.service]
    if os.geteuid() != 0:
        raise RuntimeError('Auf dem Dienstserver als root/sudo ausf�hren.')
    import fcntl
    with open('/run/lock/netcore-service-update.lock', 'a') as lock:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        props, target = validate_install(spec)
        if not args.apply:
            print(json.dumps(dict(service=args.service, phase='planned', target=str(target),
                                  active=props.get('ActiveState')), ensure_ascii=False))
            return 0
        files = prepare(spec, target)
        result = transaction(spec, files, props)
        print(json.dumps(result, ensure_ascii=False), flush=True)
        return 2 if result.get('readiness') is False else 0


if __name__ == '__main__':
    try:
        raise SystemExit(main())
    except Exception as exc:
        print('Update abgebrochen: ' + str(exc), file=sys.stderr)
        raise SystemExit(1)
