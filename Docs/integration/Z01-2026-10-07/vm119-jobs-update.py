#!/usr/bin/env python3
"""VM119: update only the reviewed SQLite Jobs implementation, without installers."""
import contextlib
import hashlib
import json
import os
from pathlib import Path
import socket
import sqlite3
import stat
import subprocess
import sys
import tempfile
import time
import tomllib
import urllib.request

OLD_SHA = '30ce8b0f936383f9c20ddb5ffda244c4065b1de1a567ddf3fbda360058d17e64'
NEW_SHA = '905e5fc4d257e5d1ad56542cbaef38d2f42a00af357240c0e5c92ea7facc2c21'
TARGET = Path('/usr/local/lib/netcore-deployment/jobs.py')
SOURCE = Path(__file__).resolve().parents[3] / 'system-backend/deployment-core/jobs.py'
CONFIG = Path('/etc/netcore/deployment.toml')
BUILD_STATE = Path('/var/lib/netcore-image-builder')
CONTROLLER, BUILDER = 'netcore-deployment.service', 'netcore-image-builder.service'
HTTP = urllib.request.build_opener(urllib.request.ProxyHandler({}))


def require(condition, message):
    if not condition:
        raise RuntimeError(message)


def command(*args):
    subprocess.run(args, check=True, timeout=40)


def api(path):
    with HTTP.open('http://127.0.0.1:8320' + path, timeout=10) as response:
        return json.load(response)


def builder_status():
    sys.path.insert(0, str(TARGET.parent))
    from image_client import ImageClient
    return ImageClient().request('/status')


def require_jobs_idle(jobs, allow_checks=False):
    for job in jobs:
        require(not job.get('result', {}).get('remote_uncertain'), 'Ungeklärter Remote-Auftrag: ' + job['id'])
        active = job['status'] in ('queued', 'running')
        require(not active or allow_checks and job.get('request', {}).get('kind') == 'check',
                'Aktiver Auftrag: ' + job['id'])


def database_idle(state, allow_checks=False):
    path = (state / 'jobs.sqlite3').resolve()
    with contextlib.closing(sqlite3.connect(path.as_uri() + '?mode=ro', uri=True, timeout=5)) as db:
        jobs = [{'id': key, 'status': status, 'request': json.loads(request), 'result': json.loads(result)}
                for key, status, request, result in db.execute('SELECT id,status,request,result FROM jobs')]
    require_jobs_idle(jobs, allow_checks)


def snapshot(paths):
    result = {}
    for path in paths:
        require(not path.is_symlink(), 'Geschützter Pfad ist ein Symlink: ' + str(path))
        result[str(path)] = hashlib.sha256(path.read_bytes()).hexdigest() if path.exists() else None
    return result


def replace(body, metadata):
    fd, name = tempfile.mkstemp(prefix='.jobs-update-', dir=TARGET.parent)
    try:
        with os.fdopen(fd, 'wb') as output:
            os.fchown(output.fileno(), metadata.st_uid, metadata.st_gid)
            os.fchmod(output.fileno(), stat.S_IMODE(metadata.st_mode))
            output.write(body)
            output.flush()
            os.fsync(output.fileno())
        os.replace(name, TARGET)
        directory = os.open(TARGET.parent, os.O_RDONLY | os.O_DIRECTORY)
        try:
            os.fsync(directory)
        finally:
            os.close(directory)
    finally:
        if os.path.exists(name):
            os.unlink(name)


def ready():
    command('systemctl', 'is-active', '--quiet', CONTROLLER, BUILDER)
    require(api('/health/ready').get('ready') is True, 'Controller nicht bereit.')
    require(builder_status().get('available') is True, 'Builder nicht verfügbar.')


def update(state, protected):
    original, metadata, source = TARGET.read_bytes(), TARGET.stat(), SOURCE.read_bytes()
    require(not TARGET.is_symlink() and stat.S_ISREG(metadata.st_mode), 'jobs.py ist keine reguläre Datei.')
    old_sha = hashlib.sha256(original).hexdigest()
    require(hashlib.sha256(source).hexdigest() == NEW_SHA, 'Quellfingerprint ist nicht der geprüfte Fix.')
    require(old_sha in (OLD_SHA, NEW_SHA), 'Installierter Quellstand ist unerwartet.')
    compile(source, str(SOURCE), 'exec')
    baseline = snapshot(protected)
    ready()
    require_jobs_idle(api('/api/v1/jobs'))
    require_jobs_idle(builder_status()['jobs'])
    database_idle(state)
    database_idle(BUILD_STATE)
    if old_sha == NEW_SHA:
        print('PASS: Geprüfter Jobs-Fix bereits installiert; kein Neustart.')
        return
    changed, stopping = False, False
    try:
        require(snapshot(protected) == baseline and TARGET.read_bytes() == original,
                'Dateien während der Vorprüfung geändert.')
        database_idle(state)
        database_idle(BUILD_STATE)
        stopping = True
        command('systemctl', 'stop', CONTROLLER)
        require_jobs_idle(builder_status()['jobs'])
        database_idle(state)
        database_idle(BUILD_STATE)
        command('systemctl', 'stop', BUILDER)
        require(snapshot(protected) == baseline, 'Geschützte Dateien vor Austausch geändert.')
        changed = True
        replace(source, metadata)
        command('systemctl', 'start', BUILDER)
        command('systemctl', 'start', CONTROLLER)
        deadline = time.monotonic() + 30
        while True:
            try:
                ready()
                break
            except (OSError, RuntimeError, subprocess.CalledProcessError):
                require(time.monotonic() < deadline, 'Dienste nach Update nicht bereit.')
                time.sleep(1)
        require(snapshot(protected) == baseline and TARGET.read_bytes() == source,
                'Dateierhalt oder Jobs-Fingerprint nach Update falsch.')
        require_jobs_idle(api('/api/v1/jobs'), allow_checks=True)
        database_idle(state, allow_checks=True)
        require_jobs_idle(builder_status()['jobs'])
        database_idle(BUILD_STATE)
        print(json.dumps({'phase': 'passed', 'jobs_sha256_before': old_sha,
                          'jobs_sha256_after': NEW_SHA, 'protected_sha256': baseline}, indent=2))
    except BaseException:
        if stopping:
            try:
                if changed:
                    database_idle(state, allow_checks=True)  # Startup schedules a read-only Git check.
                    database_idle(BUILD_STATE)
                    command('systemctl', 'stop', CONTROLLER)
                    database_idle(BUILD_STATE)
                    command('systemctl', 'stop', BUILDER)
                    replace(original, metadata)
                command('systemctl', 'start', BUILDER)
                command('systemctl', 'start', CONTROLLER)
                print('Rückweg: ursprüngliche Jobs-Bytes wieder eingesetzt und Dienste gestartet.', file=sys.stderr)
            except BaseException as rollback_error:
                print('Rückweg nicht vollständig: ' + str(rollback_error), file=sys.stderr)
        raise


def main():
    require(len(sys.argv) == 1 and os.geteuid() == 0 and socket.gethostname() == 'VM-H-DEPLOY-01',
            'Nur root auf VM-H-DEPLOY-01; keine Operator-Overrides.')
    cfg = tomllib.loads(CONFIG.read_text())
    require(cfg['role'] == 'controller' and cfg['node_id'] == 'VM-H-DEPLOY-01', 'Falsche Controlleridentität.')
    state = Path(cfg.get('state_dir', '/var/lib/netcore-deployment'))
    require(state == Path('/var/lib/netcore-deployment'), 'Unerwartetes Controller-State-Verzeichnis.')
    update(state, [CONFIG, state / 'settings.json', state / 'profiles.json', state / 'tbs-site-template.toml'])


if __name__ == '__main__':
    main()
