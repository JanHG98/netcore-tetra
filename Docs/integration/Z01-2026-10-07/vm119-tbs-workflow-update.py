#!/usr/bin/env python3
"""VM119: update seven reviewed workflow files while preserving site state."""
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import stat
import subprocess
import sys
import tempfile
import time
import tomllib

SPEC = importlib.util.spec_from_file_location('vm119_guard', Path(__file__).with_name('vm119-jobs-update.py'))
GUARD = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(GUARD)
LIBRARY = Path('/usr/local/lib/netcore-deployment')
SOURCE = Path(__file__).resolve().parents[3] / 'system-backend/deployment-core'
PINS = {
    'main.py': {'old': ('3655a11633318e7e6941862d368649959f5238b8bfcd015c9ebcf0efb4cf4e2e',), 'new': 'a73265fae52e97e0083f5cc6f0b34ae71313a7dfa8537026f1340007ca4a97ea'},
    'deploy.py': {'old': ('b17c23e3bd23147376120b68ad201ba884aa0d7b6945095dd396c55cb7127d65',), 'new': 'd2be318aff7baaeb3a73864741d5eb29f3b61f32ffd5bf0fa30c9377c54ba103'},
    'jobs.py': {'old': ('30ce8b0f936383f9c20ddb5ffda244c4065b1de1a567ddf3fbda360058d17e64',
                      '905e5fc4d257e5d1ad56542cbaef38d2f42a00af357240c0e5c92ea7facc2c21'), 'new': 'aa0c4084630977d838074b57ce85c50fe3b3522cc8e86fc4866b41df3bcbad0a'},
    'image_worker.py': {'old': ('d90fa18dec9a0976c558ccedd93027554418e24604c6769947b6a52b1b29cf32',), 'new': '343c84141cb5e747d9a3c6632ac40f17d82f981518d2b5108c7bd6e92d9c4900'},
    'static/index.html': {'old': ('02240108b526c94f74b127fa600f5f44f93199a3274b584740fcb2b613804811',), 'new': 'c2621415484332ec50b4d9254d500ad53f8e7895245ab3ac051b625ca13e8771'},
    'static/app.js': {'old': ('29ec4fc4e4c115cb9008a094fb7b469b6236b0d1b4bd6f5afd217de0de1ac017',), 'new': '8c9d0fdd37028d084e0f44d614f886446394c9baba210cd6e46f4d550fda1adf'},
    'static/style.css': {'old': ('eea4b2f89bf7dc6809f61af7ae251765e889ba83987d6410a0c8cba54590bd6f',), 'new': '032501b1e62cc843071a11155a8fc9966b9cb128f667d607a1db8405126abf33'},
}


def digest(body):
    return hashlib.sha256(body).hexdigest()


def idle(state, allow_checks=False):
    GUARD.database_idle(state, allow_checks)
    GUARD.database_idle(GUARD.BUILD_STATE)


def replace(path, body, metadata):
    descriptor, staging = tempfile.mkstemp(prefix='.tbs-workflow-', dir=path.parent)
    try:
        with os.fdopen(descriptor, 'wb') as output:
            os.fchown(output.fileno(), metadata.st_uid, metadata.st_gid)
            os.fchmod(output.fileno(), stat.S_IMODE(metadata.st_mode))
            output.write(body)
            output.flush()
            os.fsync(output.fileno())
        os.replace(staging, path)
        directory = os.open(path.parent, os.O_RDONLY | os.O_DIRECTORY)
        try:
            os.fsync(directory)
        finally:
            os.close(directory)
    finally:
        if os.path.exists(staging):
            os.unlink(staging)


def verify_files(files, key):
    for name, record in files.items():
        path = LIBRARY / name
        metadata = path.stat()
        expected = record['metadata']
        GUARD.require(not path.is_symlink() and path.read_bytes() == record[key], 'Dateierhalt falsch: ' + name)
        GUARD.require((metadata.st_uid, metadata.st_gid, stat.S_IMODE(metadata.st_mode)) ==
                      (expected.st_uid, expected.st_gid, stat.S_IMODE(expected.st_mode)),
                      'Owner/Mode geändert: ' + name)


def wait_ready():
    deadline = time.monotonic() + 30
    while True:
        try:
            GUARD.ready()
            return
        except (OSError, RuntimeError, subprocess.SubprocessError):
            GUARD.require(time.monotonic() < deadline, 'Dienste nach Update nicht bereit.')
            time.sleep(1)


def update(state, protected):
    files = {}
    for name, pins in PINS.items():
        target, source = LIBRARY / name, SOURCE / name
        metadata = target.stat()
        GUARD.require(not target.is_symlink() and stat.S_ISREG(metadata.st_mode), 'Ungültige Zieldatei: ' + name)
        GUARD.require(not source.is_symlink(), 'Quellsymlink nicht erlaubt: ' + name)
        original, replacement = target.read_bytes(), source.read_bytes()
        GUARD.require(digest(original) in (*pins['old'], pins['new']), 'Unerwarteter installierter Stand: ' + name)
        GUARD.require(digest(replacement) == pins['new'], 'Ungeprüfter Quellfingerprint: ' + name)
        if name.endswith('.py'):
            compile(replacement, str(source), 'exec')
        files[name] = {'original': original, 'replacement': replacement, 'metadata': metadata}
    baseline = GUARD.snapshot(protected)
    GUARD.ready()
    GUARD.require_jobs_idle(GUARD.api('/api/v1/jobs'))
    GUARD.require_jobs_idle(GUARD.builder_status()['jobs'])
    idle(state)
    if all(record['original'] == record['replacement'] for record in files.values()):
        print('PASS: Alle sieben geprüften Workflow-Dateien bereits installiert; kein Neustart.')
        return
    stopping, changed = False, False
    try:
        verify_files(files, 'original')
        GUARD.require(GUARD.snapshot(protected) == baseline, 'Standortdateien vor Stopp geändert.')
        idle(state)
        stopping = True
        GUARD.command('systemctl', 'stop', GUARD.CONTROLLER)
        GUARD.require_jobs_idle(GUARD.builder_status()['jobs'])
        idle(state)
        GUARD.command('systemctl', 'stop', GUARD.BUILDER)
        verify_files(files, 'original')
        GUARD.require(GUARD.snapshot(protected) == baseline, 'Standortdateien vor Austausch geändert.')
        changed = True  # Also covers a successful rename followed by an fsync failure.
        for name, record in files.items():
            replace(LIBRARY / name, record['replacement'], record['metadata'])
        GUARD.command('systemctl', 'start', GUARD.BUILDER)
        GUARD.command('systemctl', 'start', GUARD.CONTROLLER)
        wait_ready()
        verify_files(files, 'replacement')
        GUARD.require(GUARD.snapshot(protected) == baseline, 'Standortdateien nach Update geändert.')
        GUARD.require_jobs_idle(GUARD.api('/api/v1/jobs'), allow_checks=True)
        GUARD.require_jobs_idle(GUARD.builder_status()['jobs'])
        idle(state, allow_checks=True)
        print(json.dumps({'phase': 'passed', 'files': [
            {'file': str(LIBRARY / name), 'sha256_before': digest(record['original']),
             'sha256_after': digest(record['replacement'])} for name, record in files.items()],
            'protected_sha256': baseline}, indent=2))
    except BaseException:
        if stopping:
            try:
                if changed:
                    idle(state, allow_checks=True)
                    GUARD.command('systemctl', 'stop', GUARD.CONTROLLER)
                    idle(state, allow_checks=True)
                    GUARD.command('systemctl', 'stop', GUARD.BUILDER)
                    for name, record in files.items():
                        replace(LIBRARY / name, record['original'], record['metadata'])
                    verify_files(files, 'original')
                GUARD.command('systemctl', 'start', GUARD.BUILDER)
                GUARD.command('systemctl', 'start', GUARD.CONTROLLER)
                print('Rückweg: ursprüngliche Workflow-Dateien erhalten/wieder eingesetzt; Dienste gestartet.', file=sys.stderr)
            except BaseException as rollback_error:
                print('Rückweg nicht vollständig: ' + str(rollback_error), file=sys.stderr)
        raise


def main():
    GUARD.require(len(sys.argv) == 1 and os.geteuid() == 0 and GUARD.socket.gethostname() == 'VM-H-DEPLOY-01',
                  'Nur root auf VM-H-DEPLOY-01; keine Operator-Overrides.')
    cfg = tomllib.loads(GUARD.CONFIG.read_text())
    GUARD.require(cfg['role'] == 'controller' and cfg['node_id'] == 'VM-H-DEPLOY-01', 'Falsche Controlleridentität.')
    state = Path(cfg.get('state_dir', '/var/lib/netcore-deployment'))
    GUARD.require(state == Path('/var/lib/netcore-deployment'), 'Unerwartetes Controller-State-Verzeichnis.')
    update(state, [GUARD.CONFIG, state / 'settings.json', state / 'profiles.json', state / 'tbs-site-template.toml'])


if __name__ == '__main__':
    main()
