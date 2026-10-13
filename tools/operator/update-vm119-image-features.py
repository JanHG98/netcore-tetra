#!/usr/bin/env python3
"""Update the existing VM119 controller, builder, recipes and UI while idle.

Git main is resolved once by the calling block. This updates package assets,
preserving effective host units, configuration, profiles, artifacts and caches.
No package installation and no image build is started by this helper.
"""
from __future__ import annotations

import argparse
from contextlib import closing, contextmanager
import hashlib
import http.client
import json
import os
from pathlib import Path, PurePosixPath
import re
import shutil
import socket
import sqlite3
import stat
import subprocess
import tempfile
import time
import tomllib
import urllib.request

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / 'system-backend/deployment-core'
LIBRARY = Path('/usr/local/lib/netcore-deployment')
CONFIG = Path('/etc/netcore/deployment.toml')
STATE = Path('/var/lib/netcore-deployment')
BUILD_STATE = Path('/var/lib/netcore-image-builder')
BUILD_SOCKET = Path('/run/netcore-image-builder/api.sock')
BACKUP_ROOT = Path('/var/backups/netcore-vm-image-update')
PROC_ROOT = Path('/proc')
CONTROLLER = 'netcore-deployment.service'
BUILDER = 'netcore-image-builder.service'
SHA = re.compile(r'[0-9a-f]{40}')


def run(args, **kwargs):
    return subprocess.run(args, check=True, timeout=45, **kwargs)


def digest(path):
    with Path(path).open('rb') as stream:
        value = hashlib.sha256()
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            value.update(block)
    return value.hexdigest()


def safe_path(path, root=None):
    path = Path(path)
    if not path.is_absolute() or (root is not None and not path.is_relative_to(root)):
        raise RuntimeError('Pfad ausserhalb der erwarteten Bibliothek: ' + str(path))
    for candidate in (path, *path.parents):
        if candidate.is_symlink():
            raise RuntimeError('Symlink im Updatepfad: ' + str(candidate))
    if path.exists() and not path.is_file():
        raise RuntimeError('Updateziel ist keine regulaere Datei: ' + str(path))
    return path


def properties(unit):
    proc = run(['systemctl', 'show', unit, '-p', 'LoadState', '-p', 'FragmentPath',
                '-p', 'DropInPaths', '-p', 'ExecStart', '-p', 'ActiveState', '-p', 'InvocationID', '-p', 'MainPID'],
               capture_output=True, text=True)
    return dict(line.split('=', 1) for line in proc.stdout.splitlines() if '=' in line)


def installed_paths():
    protected = [CONFIG, STATE / 'settings.json', STATE / 'profiles.json',
                 STATE / 'tbs-site-template.toml']
    baseline_units = {}
    for unit, script in ((CONTROLLER, 'main.py'), (BUILDER, 'image_worker.py')):
        props = properties(unit)
        if (props.get('LoadState') != 'loaded' or not props.get('FragmentPath')
                or props.get('ActiveState') != 'active'):
            raise RuntimeError('VM-Dienst nicht installiert/aktiv: ' + unit)
        if str(LIBRARY / script) not in props.get('ExecStart', ''):
            raise RuntimeError('Abweichender VM-ExecStart: ' + unit)
        if unit == CONTROLLER and str(CONFIG) not in props.get('ExecStart', ''):
            raise RuntimeError('Abweichender Controller-Konfigurationspfad.')
        protected.extend(Path(p) for p in [props['FragmentPath'], *props.get('DropInPaths', '').split()])
        baseline_units[unit] = props
    return protected, baseline_units


def snapshot(paths):
    result = {}
    for path in paths:
        safe_path(path)
        result[str(path)] = digest(path) if path.exists() else None
    return result


def jobs_idle(state, allow_checks=False):
    path = safe_path(state / 'jobs.sqlite3')
    with closing(sqlite3.connect(path.resolve().as_uri() + '?mode=ro', uri=True, timeout=5)) as db:
        for key, status, request, result in db.execute('SELECT id,status,request,result FROM jobs'):
            request, result = json.loads(request), json.loads(result)
            if not isinstance(request, dict) or not isinstance(result, dict):
                raise RuntimeError('Ungueltiger persistenter Auftrag: ' + key)
            if result.get('remote_uncertain'):
                raise RuntimeError('Ungeklaerter Remote-Auftrag: ' + key)
            if status in ('queued', 'running') and not (allow_checks and request.get('kind') == 'check'):
                raise RuntimeError('Aktiver Auftrag: ' + key)


def both_idle(allow_checks=False):
    jobs_idle(STATE, allow_checks)
    jobs_idle(BUILD_STATE)


class UnixHTTP(http.client.HTTPConnection):
    def connect(self):
        self.sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        self.sock.settimeout(self.timeout)
        self.sock.connect(str(BUILD_SOCKET))


def builder_status():
    connection = UnixHTTP('localhost', timeout=10)
    try:
        connection.request('GET', '/status')
        response = connection.getresponse()
        body = response.read(1024 * 1024 + 1)
        if response.status != 200 or len(body) > 1024 * 1024:
            raise RuntimeError('Builder-Status nicht verfuegbar.')
        return json.loads(body)
    finally:
        connection.close()


def ready():
    run(['systemctl', 'is-active', '--quiet', CONTROLLER, BUILDER])
    cfg = tomllib.loads(CONFIG.read_text(encoding='utf-8'))
    host, port = cfg.get('bind', '0.0.0.0'), cfg.get('port', 8320)
    if host in ('0.0.0.0', '::', '[::]'):
        host = '127.0.0.1'
    if ':' in host and not host.startswith('['):
        host = '[' + host + ']'
    opener = urllib.request.build_opener(urllib.request.ProxyHandler({}))
    with opener.open(f'http://{host}:{port}/health/ready', timeout=10) as response:
        body = response.read(1024 * 1024 + 1)
        if len(body) > 1024 * 1024 or json.loads(body).get('ready') is not True:
            raise RuntimeError('Controller nicht bereit.')
    if builder_status().get('available') is not True:
        raise RuntimeError('Builder nicht bereit.')


def wait_ready():
    deadline = time.monotonic() + 40
    while True:
        try:
            ready()
            return
        except (OSError, ValueError, RuntimeError, subprocess.SubprocessError, http.client.HTTPException):
            if time.monotonic() >= deadline:
                raise RuntimeError('Controller/Builder nach Update nicht bereit.')
            time.sleep(1)


@contextmanager
def exclusive(path):
    import fcntl
    safe_path(path)
    with path.open('a') as lock:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        yield


@contextmanager
def shared_update_mutex():
    """Keep the shared inode writable by the unprivileged controller."""
    import fcntl
    path = safe_path(STATE / 'runtime-update.mutex')
    owner = path.parent.stat()
    if not stat.S_ISDIR(owner.st_mode):
        raise RuntimeError('Controller-State ist kein Verzeichnis.')
    flags = os.O_RDWR | os.O_CREAT | getattr(os, 'O_CLOEXEC', 0) | getattr(os, 'O_NOFOLLOW', 0)
    descriptor = os.open(path, flags, 0o600)
    try:
        opened = os.fstat(descriptor)
        if not stat.S_ISREG(opened.st_mode) or opened.st_nlink != 1:
            raise RuntimeError('Shared-Update-Mutex ist keine einzelne regulaere Datei.')
        # Repair an older root-created mutex only after acquiring its existing
        # inode. Replacing/unlinking a lock would allow concurrent owners.
        fcntl.flock(descriptor, fcntl.LOCK_EX | fcntl.LOCK_NB)
        current = path.lstat()
        if (current.st_dev, current.st_ino) != (opened.st_dev, opened.st_ino):
            raise RuntimeError('Shared-Update-Mutex wurde beim Sperren ersetzt.')
        expected = (owner.st_uid, owner.st_gid, 0o600)
        if (opened.st_uid, opened.st_gid) != expected[:2]:
            os.fchown(descriptor, *expected[:2])
        if stat.S_IMODE(os.fstat(descriptor).st_mode) != 0o600:
            os.fchmod(descriptor, 0o600)
        if attributes(os.fstat(descriptor)) != expected:
            raise RuntimeError('Controller kann Shared-Update-Mutex nicht weiterverwenden.')
        yield
    finally:
        os.close(descriptor)


def payload(commit):
    """Read inventory and bytes from immutable Git objects, never a live glob."""
    if not SHA.fullmatch(commit):
        raise RuntimeError('Vollstaendiger Git-SHA fuer VM-Paket erforderlich.')
    prefix = 'system-backend/deployment-core/'
    tree = run(['git', '-C', str(ROOT), 'ls-tree', '-r', '-z', commit, '--', prefix],
               capture_output=True).stdout
    result = []
    for record in tree.split(b'\0'):
        if not record:
            continue
        description, encoded_path = record.split(b'\t', 1)
        mode, kind, object_id = description.decode('ascii').split()
        name = encoded_path.decode('utf-8')
        if not name.startswith(prefix):
            raise RuntimeError('Git-Paketdatei ausserhalb des Deployment-Pakets.')
        relative = PurePosixPath(name[len(prefix):])
        if relative.is_absolute() or '..' in relative.parts:
            raise RuntimeError('Ungueltiger relativer Git-Paketpfad.')
        selected = (len(relative.parts) == 1 and (relative.suffix == '.py' or str(relative) == 'catalog.json')
                    or relative.parts[0] in ('static', 'image') and '__pycache__' not in relative.parts
                       and relative.suffix != '.pyc'
                    or str(relative) in ('install/prepare-host.sh', 'install/install-tbs.sh'))
        if not selected:
            continue
        if kind != 'blob' or mode not in ('100644', '100755') or not SHA.fullmatch(object_id):
            raise RuntimeError('Kein regulaeres Git-Quellobjekt: ' + name)
        body = run(['git', '-C', str(ROOT), 'cat-file', 'blob', object_id], capture_output=True).stdout
        object_body = b'blob ' + str(len(body)).encode('ascii') + b'\0' + body
        if hashlib.sha1(object_body).hexdigest() != object_id:
            raise RuntimeError('Git-Quellobjekt stimmt nicht mit seinem Fingerprint ueberein: ' + name)
        source = SOURCE / str(relative)
        dest = safe_path(LIBRARY / str(relative), LIBRARY)
        if relative.suffix == '.py':
            compile(body, name, 'exec')
        elif relative.suffix == '.sh':
            run(['bash', '-n'], input=body)
        elif relative.suffix == '.json':
            json.loads(body)
        metadata = dest.stat() if dest.exists() else None
        original = dest.read_bytes() if metadata else None
        result.append(dict(relative=relative.as_posix(), source=source, dest=dest, body=body,
                           git_blob=object_id, original=original, metadata=metadata))
    names = {r['relative'] for r in result}
    if not {'main.py', 'image_worker.py', 'image_spec.py', 'catalog.json',
            'install/prepare-host.sh', 'install/install-tbs.sh'} <= names:
        raise RuntimeError('Commit enthaelt kein vollstaendiges VM-Paket.')
    return result


def fsync_directory(path):
    fd = os.open(path, os.O_RDONLY | os.O_DIRECTORY)
    try:
        os.fsync(fd)
    finally:
        os.close(fd)


def replace(path, body, metadata=None):
    safe_path(path, LIBRARY)
    if body is None:
        path.unlink(missing_ok=True)
        fsync_directory(path.parent)
        return
    fd, name = tempfile.mkstemp(prefix='.netcore-vm-update-', dir=path.parent)
    try:
        with os.fdopen(fd, 'wb') as stream:
            mode = stat.S_IMODE(metadata.st_mode) if metadata else 0o755 if path.suffix == '.sh' else 0o644
            os.fchown(stream.fileno(), metadata.st_uid if metadata else 0, metadata.st_gid if metadata else 0)
            os.fchmod(stream.fileno(), mode)
            stream.write(body)
            stream.flush()
            os.fsync(stream.fileno())
        os.replace(name, path)
        fsync_directory(path.parent)
    finally:
        if os.path.exists(name):
            os.unlink(name)


def attributes(metadata):
    return metadata.st_uid, metadata.st_gid, stat.S_IMODE(metadata.st_mode)


def verify(files, key):
    for record in files:
        path = safe_path(record['dest'], LIBRARY)
        actual = path.read_bytes() if path.exists() else None
        if actual != record[key]:
            raise RuntimeError('VM-Paketdatei geaendert: ' + str(path))
        if actual is not None:
            metadata = path.stat()
            expected = record['metadata']
            ownership = (attributes(expected) if expected
                         else (0, 0, 0o755 if path.suffix == '.sh' else 0o644))
            if attributes(metadata) != ownership:
                raise RuntimeError('Owner/Mode der VM-Paketdatei geaendert: ' + str(path))


def configured_exec(value):
    """systemctl ExecStart also prints runtime PID and timestamps; ignore those."""
    fields = re.findall(r'(?:^|[;{])\s*(path|argv\[\]|ignore_errors)=([^;}]*)(?=[;}])', value)
    return tuple((name, body.strip()) for name, body in fields) if fields else value.strip()


def process_proof(unit, script, before):
    after = properties(unit)
    if (after.get('LoadState') != 'loaded' or after.get('ActiveState') != 'active'
            or not re.fullmatch(r'[0-9a-f]{32}', after.get('InvocationID', ''))
            or after['InvocationID'] == before.get('InvocationID')):
        raise RuntimeError('VM-Dienst nicht neu gestartet: ' + unit)
    for key in ('FragmentPath', 'DropInPaths'):
        if after.get(key) != before.get(key):
            raise RuntimeError('Effektive VM-Unit waehrend Update geaendert: ' + unit)
    if configured_exec(after.get('ExecStart', '')) != configured_exec(before.get('ExecStart', '')):
        raise RuntimeError('Effektive VM-Unit waehrend Update geaendert: ' + unit)
    pid = after.get('MainPID', '')
    if not pid.isdigit() or int(pid) <= 0:
        raise RuntimeError('VM-Dienst hat keinen nachgewiesenen MainPID: ' + unit)
    command = (PROC_ROOT / pid / 'cmdline').read_bytes().split(b'\0')
    if os.fsencode(LIBRARY / script) not in command:
        raise RuntimeError('Laufender Prozess verwendet nicht das aktualisierte VM-Modul: ' + unit)
    if unit == CONTROLLER and os.fsencode(CONFIG) not in command:
        raise RuntimeError('Laufender Controller verwendet einen anderen Konfigurationspfad.')
    return dict(MainPID=int(pid), InvocationID=after['InvocationID'],
                command=[part.decode('utf-8', 'replace') for part in command if part])


def clear_bytecode(files):
    for record in files:
        path = record['dest']
        cache = path.parent / '__pycache__'
        if path.suffix == '.py' and cache.is_dir() and not cache.is_symlink():
            for bytecode in cache.glob(path.stem + '.*.pyc'):
                if bytecode.is_file() and not bytecode.is_symlink():
                    bytecode.unlink()
        legacy = path.with_suffix('.pyc')
        if path.suffix == '.py' and legacy.is_file() and not legacy.is_symlink():
            legacy.unlink()


def update(commit, apply=False):
    files = payload(commit)
    protected, before_units = installed_paths()
    baseline = snapshot(protected)
    ready()
    both_idle()
    changed = [r for r in files if r['body'] != r['original']]
    result = dict(phase='planned', source_commit=commit, package_files=len(files),
                  changed_files=[r['relative'] for r in changed], protected_sha256=baseline)
    if not apply:
        return result
    if not changed:
        verify(files, 'body')
        return dict(result, phase='already_current')
    BACKUP_ROOT.mkdir(parents=True, exist_ok=True, mode=0o700)
    backup = Path(tempfile.mkdtemp(prefix=commit[:12] + '-', dir=BACKUP_ROOT))
    for index, record in enumerate(changed):
        if record['original'] is not None:
            stored = backup / str(index)
            stored.write_bytes(record['original'])
            stored.chmod(0o600)
    (backup / 'manifest.json').write_text(json.dumps(dict(source_commit=commit,
        files=[dict(target=str(r['dest']), source_sha256=hashlib.sha256(r['body']).hexdigest(),
                    previous_sha256=hashlib.sha256(r['original']).hexdigest() if r['original'] else None)
               for r in changed]), indent=2), encoding='utf-8')
    attempted, created_dirs = [], []
    stopping = False
    try:
        verify(files, 'original')
        if snapshot(protected) != baseline:
            raise RuntimeError('Geschuetzte VM-Dateien vor Update geaendert.')
        both_idle()
        stopping = True
        run(['systemctl', 'stop', CONTROLLER])
        both_idle()
        # No API can submit a new image once the controller has stopped.
        run(['systemctl', 'stop', BUILDER])
        with exclusive(BUILD_STATE / 'worker.lock'):
            both_idle()
            verify(files, 'original')
            if snapshot(protected) != baseline:
                raise RuntimeError('Geschuetzte Dateien vor Austausch geaendert.')
            for record in changed:
                path = record['dest']
                missing, directory = [], path.parent
                while not directory.exists():
                    missing.append(directory)
                    directory = directory.parent
                for directory in reversed(missing):
                    if not directory.is_relative_to(LIBRARY):
                        raise RuntimeError('Neuer Ordner ausserhalb der Bibliothek.')
                    directory.mkdir(mode=0o755)
                    created_dirs.append(directory)
                attempted.append(record)
                replace(path, record['body'], record['metadata'])
            clear_bytecode(changed)
            verify(files, 'body')
        run(['systemctl', 'start', BUILDER])
        run(['systemctl', 'start', CONTROLLER])
        wait_ready()
        both_idle(allow_checks=True)
        verify(files, 'body')
        if snapshot(protected) != baseline:
            raise RuntimeError('Lokale Konfiguration/Profile/Units waehrend Update geaendert.')
        processes = {unit: process_proof(unit, script, before_units[unit]) for unit, script in
                     ((BUILDER, 'image_worker.py'), (CONTROLLER, 'main.py'))}
        return dict(result, phase='passed', backup=str(backup), processes=processes, files=[dict(
            file=str(r['dest']), sha256=hashlib.sha256(r['body']).hexdigest(), git_blob=r['git_blob']) for r in files])
    except BaseException:
        if stopping:
            errors = []
            try:
                run(['systemctl', 'stop', CONTROLLER])
                both_idle(allow_checks=True)
                run(['systemctl', 'stop', BUILDER])
                with exclusive(BUILD_STATE / 'worker.lock'):
                    both_idle(allow_checks=True)
                    for record in reversed(attempted):
                        actual = record['dest'].read_bytes() if record['dest'].exists() else None
                        if actual not in (record['original'], record['body']):
                            raise RuntimeError('Fremde Paketdatei vor Rueckweg geaendert.')
                        replace(record['dest'], record['original'], record['metadata'])
                    for directory in reversed(created_dirs):
                        try:
                            directory.rmdir()
                        except OSError:
                            pass
                    clear_bytecode(changed)
                    verify(files, 'original')
            except BaseException as exc:
                errors.append(str(exc))
            for unit in (BUILDER, CONTROLLER):
                try:
                    run(['systemctl', 'start', unit])
                except BaseException as exc:
                    errors.append(str(exc))
            if not errors:
                try:
                    wait_ready()
                    if snapshot(protected) != baseline:
                        raise RuntimeError('Geschuetzte Dateien nach Rueckweg geaendert.')
                except BaseException as exc:
                    errors.append(str(exc))
            print('Rueckweg: ' + str(backup) + ('; FEHLER: ' + '; '.join(errors)
                  if errors else '; vorheriges VM-Paket wiederhergestellt'), flush=True)
        raise


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--commit', required=True)
    parser.add_argument('--apply', action='store_true')
    args = parser.parse_args(argv)
    if not SHA.fullmatch(args.commit):
        raise RuntimeError('Vollstaendiger Git-SHA erforderlich.')
    if os.geteuid() != 0 or socket.gethostname() != 'VM-H-DEPLOY-01':
        raise RuntimeError('Nur mit sudo/root auf VM-H-DEPLOY-01 ausfuehren.')
    cfg = tomllib.loads(CONFIG.read_text(encoding='utf-8'))
    if cfg.get('role') != 'controller' or cfg.get('node_id') != 'VM-H-DEPLOY-01':
        raise RuntimeError('Falsche Deployment-VM-Identitaet.')
    if Path(cfg.get('state_dir', str(STATE))) != STATE:
        raise RuntimeError('Unerwartetes Controller-State-Verzeichnis.')
    actual = run(['git', '-C', str(ROOT), 'rev-parse', 'HEAD'], capture_output=True, text=True).stdout.strip()
    dirty = run(['git', '-C', str(ROOT), 'status', '--porcelain', '--untracked-files=no'],
                capture_output=True, text=True).stdout.strip()
    if actual != args.commit or dirty:
        raise RuntimeError('Quellcheckout stimmt nicht mit dem einmal aufgeloesten Git-SHA ueberein.')
    print('VM119 Controller/Builder/Rezept/UI: ' + args.commit, flush=True)
    with exclusive(Path('/run/lock/netcore-vm-image-update.lock')):
        with shared_update_mutex():
            result = update(args.commit, args.apply)
    print(json.dumps(result, ensure_ascii=False, indent=2), flush=True)
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
