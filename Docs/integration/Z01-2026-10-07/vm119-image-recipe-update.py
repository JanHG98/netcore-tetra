#!/usr/bin/env python3
"""VM119: install the three verified image-recipe corrections as one update."""
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import socket
import stat
import subprocess
import sys
import tempfile
import time
import tomllib

GUARD_PATH = Path(__file__).with_name('vm119-jobs-update.py')
GUARD_SHA = '50dfc518d9de6cffc5ec3335b3b94e6716d3c22de6653868f895386e64ca39db'
if GUARD_PATH.is_symlink() or hashlib.sha256(GUARD_PATH.read_bytes()).hexdigest() != GUARD_SHA:
    raise RuntimeError('Ungeprüfter VM119-Guard.')
SPEC = importlib.util.spec_from_file_location('vm119_recipe_guard', GUARD_PATH)
GUARD = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(GUARD)

LIBRARY = Path('/usr/local/lib/netcore-deployment')
SOURCE = Path(__file__).resolve().parents[3] / 'system-backend/deployment-core'
WORKER_SHA = '343c84141cb5e747d9a3c6632ac40f17d82f981518d2b5108c7bd6e92d9c4900'
PINS = {
    'image_build.py': {
        'old': '13aab8659a09bfddbe6aab3dce2bd193e8d5f62f9afcad3cfdce054a6dd61e61',
        'new': 'ecb33c14e7e26c38d28e48ee3bdfae71e64836b368bed70a9daa24da2ce218cc',
    },
    'image/build-guest.sh': {
        'old': '4ecd8daeaff314d63da00407f49c994f3bff0569755dbd95640aef96289a8e68',
        'new': 'e6f992907060f7d20378ff1f1c1ca49618c4fa712c9c2dce3a64065bf62e1347',
    },
    'image/gpio-compat.py': {
        'old': None,
        'new': '133c4d6d433c67ff5aee0726d487282faf19ad370d38da710fc3b883153d33e0',
    },
}


def digest(body):
    return hashlib.sha256(body).hexdigest() if body is not None else None


def attributes(metadata):
    return (metadata.st_uid, metadata.st_gid, stat.S_IMODE(metadata.st_mode)) if metadata else (0, 0, 0o644)


def sync_directory(path):
    descriptor = os.open(path, os.O_RDONLY | getattr(os, 'O_DIRECTORY', 0))
    try:
        os.fsync(descriptor)
    finally:
        os.close(descriptor)


def replace(path, body, metadata):
    if body is None:
        if path.exists():
            path.unlink()
            sync_directory(path.parent)
        return
    descriptor, staging = tempfile.mkstemp(prefix='.image-recipe-', dir=path.parent)
    try:
        with os.fdopen(descriptor, 'wb') as output:
            uid, gid, mode = attributes(metadata)
            os.fchown(output.fileno(), uid, gid)
            os.fchmod(output.fileno(), mode)
            output.write(body)
            output.flush()
            os.fsync(output.fileno())
        os.replace(staging, path)
        sync_directory(path.parent)
    finally:
        if os.path.exists(staging):
            os.unlink(staging)


def idle(state, allow_checks=False):
    GUARD.database_idle(state, allow_checks)
    GUARD.database_idle(GUARD.BUILD_STATE)


def worker_current():
    worker = LIBRARY / 'image_worker.py'
    GUARD.require(not worker.is_symlink() and worker.is_file(), 'Ungültiger Imageworkerpfad.')
    GUARD.require(digest(worker.read_bytes()) == WORKER_SHA, 'Imageworker hat einen unerwarteten Stand.')


def verify_files(files, key):
    for name, record in files.items():
        path = LIBRARY / name
        body = record[key]
        GUARD.require(not path.is_symlink(), 'Rezeptdatei ist ein Symlink: ' + name)
        if body is None:
            GUARD.require(not path.exists(), 'Unerwartete neue Rezeptdatei: ' + name)
            continue
        GUARD.require(path.is_file() and path.read_bytes() == body, 'Rezeptbytes geändert: ' + name)
        metadata = path.stat()
        GUARD.require(attributes(metadata) == attributes(record['metadata']), 'Owner/Mode geändert: ' + name)


def wait_ready():
    deadline = time.monotonic() + 30
    while True:
        try:
            GUARD.ready()
            return
        except (OSError, RuntimeError, subprocess.SubprocessError):
            GUARD.require(time.monotonic() < deadline, 'Dienste nach Rezeptupdate nicht bereit.')
            time.sleep(1)


def update(state, protected):
    worker_current()
    files = {}
    for name, pins in PINS.items():
        target, source = LIBRARY / name, SOURCE / name
        GUARD.require(target.parent.is_dir() and not target.parent.is_symlink(), 'Ungültiger Rezeptordner: ' + name)
        GUARD.require(not target.is_symlink(), 'Rezeptziel ist ein Symlink: ' + name)
        metadata = target.stat() if target.exists() else None
        if metadata is not None:
            GUARD.require(stat.S_ISREG(metadata.st_mode), 'Rezeptziel ist keine Datei: ' + name)
        original = target.read_bytes() if metadata is not None else None
        GUARD.require(digest(original) in (pins['old'], pins['new']), 'Unerwarteter installierter Rezeptstand: ' + name)
        GUARD.require(source.is_file() and not source.is_symlink(), 'Ungültige Rezeptquelle: ' + name)
        replacement = source.read_bytes()
        GUARD.require(digest(replacement) == pins['new'], 'Ungeprüfter Quellfingerprint: ' + name)
        if source.suffix == '.py':
            compile(replacement, str(source), 'exec')
        else:
            GUARD.command('bash', '-n', str(source))
        files[name] = {'original': original, 'replacement': replacement, 'metadata': metadata}
    baseline = GUARD.snapshot(protected)
    GUARD.ready()
    GUARD.require_jobs_idle(GUARD.api('/api/v1/jobs'))
    GUARD.require_jobs_idle(GUARD.builder_status()['jobs'])
    idle(state)
    changed_names = [name for name, record in files.items() if record['original'] != record['replacement']]
    if not changed_names:
        return {'phase': 'already_current', 'changed_files': []}

    stopping, attempted = False, []
    try:
        verify_files(files, 'original')
        worker_current()
        GUARD.require(GUARD.snapshot(protected) == baseline, 'Standortdateien vor Stopp geändert.')
        idle(state)
        stopping = True
        GUARD.command('systemctl', 'stop', GUARD.CONTROLLER)
        GUARD.require_jobs_idle(GUARD.builder_status()['jobs'])
        idle(state)
        GUARD.command('systemctl', 'stop', GUARD.BUILDER)
        verify_files(files, 'original')
        worker_current()
        GUARD.require(GUARD.snapshot(protected) == baseline, 'Standortdateien vor Austausch geändert.')
        for name in changed_names:
            attempted.append(name)  # A rename may succeed before a later fsync fails.
            record = files[name]
            replace(LIBRARY / name, record['replacement'], record['metadata'])
        GUARD.command('systemctl', 'start', GUARD.BUILDER)
        GUARD.command('systemctl', 'start', GUARD.CONTROLLER)
        wait_ready()
        verify_files(files, 'replacement')
        worker_current()
        GUARD.require(GUARD.snapshot(protected) == baseline, 'Standortdateien nach Update geändert.')
        GUARD.require_jobs_idle(GUARD.api('/api/v1/jobs'), allow_checks=True)
        GUARD.require_jobs_idle(GUARD.builder_status()['jobs'])
        idle(state, allow_checks=True)
        return {'phase': 'passed', 'changed_files': changed_names, 'files': [
            {'file': str(LIBRARY / name), 'sha256_before': digest(record['original']),
             'sha256_after': digest(record['replacement'])} for name, record in files.items()],
            'protected_sha256': baseline}
    except BaseException:
        if stopping:
            try:
                if attempted:
                    idle(state, allow_checks=True)
                    GUARD.command('systemctl', 'stop', GUARD.CONTROLLER)
                    idle(state, allow_checks=True)
                    GUARD.command('systemctl', 'stop', GUARD.BUILDER)
                    for name in reversed(attempted):
                        record, target = files[name], LIBRARY / name
                        GUARD.require(not target.is_symlink(), 'Rückwegziel ist ein Symlink: ' + name)
                        actual = target.read_bytes() if target.exists() else None
                        GUARD.require(actual in (record['original'], record['replacement']), 'Fremde Rezeptänderung vor Rückweg: ' + name)
                        replace(target, record['original'], record['metadata'])
                    verify_files(files, 'original')
                GUARD.command('systemctl', 'start', GUARD.BUILDER)
                GUARD.command('systemctl', 'start', GUARD.CONTROLLER)
                print('Rückweg: vorheriges Rezept wiederhergestellt; Dienste gestartet.', file=sys.stderr)
            except BaseException as rollback_error:
                print('Rückweg nicht vollständig: ' + str(rollback_error), file=sys.stderr)
        raise


def main():
    GUARD.require(len(sys.argv) == 1 and os.geteuid() == 0 and socket.gethostname() == 'VM-H-DEPLOY-01',
                  'Nur per sudo/root auf VM-H-DEPLOY-01; keine Operator-Overrides.')
    cfg = tomllib.loads(GUARD.CONFIG.read_text())
    GUARD.require(cfg['role'] == 'controller' and cfg['node_id'] == 'VM-H-DEPLOY-01', 'Falsche Controlleridentität.')
    state = Path(cfg.get('state_dir', '/var/lib/netcore-deployment'))
    GUARD.require(state == Path('/var/lib/netcore-deployment'), 'Unerwartetes Controller-State-Verzeichnis.')
    protected = [GUARD.CONFIG, state / 'settings.json', state / 'profiles.json', state / 'tbs-site-template.toml']
    print(json.dumps(update(state, protected), indent=2))


if __name__ == '__main__':
    main()
