#!/usr/bin/env python3
"""CT136: prove the real archive mount guard retains an isolated raw record."""
import hashlib
import inspect
import json
import os
from pathlib import Path
import pwd
import socket
import subprocess
import sys
import tempfile
import uuid

MODULE = Path('/opt/netcore-observability/logging/log_store.py')
CONFIGS = (Path('/etc/netcore/observability.toml'), Path('/etc/netcore/syslog.json'))


def exercise(directory, module_file, identity=(999, 989)):
    """Pure fixture worker; identity/module overrides are for isolated tests only."""
    import datetime as dt
    import hashlib
    import importlib.util
    import json
    import os
    import uuid

    def check(condition, message):
        if not condition:
            raise RuntimeError(message)

    check((os.geteuid(), os.getegid()) == identity, 'Worker hat eine falsche Dienstidentität.')
    spec = importlib.util.spec_from_file_location('negative_log_store', module_file)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    cfg = module.load_config(directory / 'fixture.json')
    check(cfg['state_dir'] == str(directory / 'state')
          and cfg['archive_mount'] == str(directory / 'local-not-a-share'), 'Fixturepfade unerwartet.')
    target = directory / 'local-not-a-share'
    check(target.is_dir() and not list(target.iterdir()), 'Lokales Testziel ist nicht leer.')
    marker = 'Z014-ARCHIVE-NEGATIVE-' + uuid.uuid4().hex
    store = module.Store(cfg)
    try:
        store.append({'timestamp': dt.datetime.now(dt.timezone.utc).isoformat(),
                      'hostname': 'fixture-only', 'program': 'z014-archive-negative',
                      'severity': 'info', 'facility': 'local0', 'source_ip': '127.0.0.1',
                      'transport': 'isolated-fixture', 'message': marker})
        paths = list(store.raw.glob('*.open'))
        check(len(paths) == 1, 'Das eigene Raw-Segment fehlt.')
        before = paths[0].read_bytes()
        expected_error = 'Network share is not mounted at ' + str(target)
        try:
            module.archive(store)  # Genuine default open_share, never substituted.
        except OSError as error:
            check(str(error) == expected_error, 'Kein erwarteter Mountguard-Fehler: ' + str(error))
        else:
            raise RuntimeError('Archivieren auf ein lokales Nicht-Mount-Ziel wurde akzeptiert.')
        sealed = paths[0].with_suffix('.jsonl')
        after = sealed.read_bytes()
        record = json.loads(after)
        status = json.loads((directory / 'state/archive-status.json').read_text())
        check(before == after and record['message'] == marker, 'Raw-Inhalt wurde nicht erhalten.')
        check(not list(store.raw.glob('*.open')) and len(list(store.raw.glob('*.jsonl'))) == 1,
              'Versiegelter Raw-Zustand ist unerwartet.')
        check(status['error'] == expected_error and status['archived_segments'] == 0,
              'Archivstatus bestätigt den Mountfehler nicht.')
        check(store.status()['preview_pending'] == 1 and not list(target.iterdir()),
              'Fixture-Outbox oder lokales Archivziel wurde unerwartet verändert.')
        return {'phase': 'passed', 'identity': list(identity), 'marker': marker,
                'log_store_path': str(module_file),
                'log_store_sha256': hashlib.sha256(module_file.read_bytes()).hexdigest(),
                'raw_record_id': record['id'], 'raw_file': str(sealed),
                'raw_sha256_before': hashlib.sha256(before).hexdigest(),
                'raw_sha256_after': hashlib.sha256(after).hexdigest(),
                'archive_status': status, 'preview_pending': 1, 'local_target_empty': True}
    finally:
        store.db.close()


def fingerprints():
    return {str(path): hashlib.sha256(path.read_bytes()).hexdigest() for path in CONFIGS}


def save(directory, result):
    fd, name = tempfile.mkstemp(prefix='.result-', dir=directory)
    with os.fdopen(fd, 'w') as output:
        json.dump(result, output, indent=2, ensure_ascii=False)
        output.write('\n')
        output.flush()
        os.fsync(output.fileno())
    os.replace(name, directory / 'result.json')


def main():
    if len(sys.argv) != 1 or os.geteuid() != 0 or socket.gethostname() != 'Observability':
        raise RuntimeError('Nur root im CT136 Observability; keine Operator-Overrides.')
    account = pwd.getpwnam('netcore-observability')
    if (account.pw_uid, account.pw_gid) != (999, 989):
        raise RuntimeError('Dienstidentität stimmt nicht mit CT136 überein.')
    baseline = fingerprints()
    directory = Path(tempfile.mkdtemp(prefix='netcore-ct136-archive-negative-', dir='/var/tmp'))
    os.chown(directory, 999, 989)
    directory.chmod(0o700)
    result = {'phase': 'created', 'fixture': str(directory), 'production_sha256_before': baseline}
    print('Nachweis:', directory / 'result.json', flush=True)
    try:
        cfg = json.loads(CONFIGS[1].read_text())
        cfg.update(state_dir=str(directory / 'state'), archive_mount=str(directory / 'local-not-a-share'),
                   collector_id='z014-negative-' + uuid.uuid4().hex)
        target = directory / 'local-not-a-share'
        target.mkdir(mode=0o700)
        os.chown(target, 999, 989)
        fd = os.open(directory / 'fixture.json', os.O_CREAT | os.O_EXCL | os.O_WRONLY, 0o600)
        with os.fdopen(fd, 'w') as output:
            json.dump(cfg, output)
        os.chown(directory / 'fixture.json', 999, 989)
        save(directory, result)
        worker = inspect.getsource(exercise) + '\nfrom pathlib import Path\nimport json, sys\n'
        worker += 'print(json.dumps(exercise(Path(sys.argv[1]), Path(' + repr(str(MODULE)) + '))))\n'
        reply = subprocess.run(['runuser', '-u', 'netcore-observability', '--', 'python3', '-', str(directory)],
                               input=worker, capture_output=True, text=True, check=True, timeout=30)
        result.update(json.loads(reply.stdout))
        result['production_sha256_after'] = fingerprints()
        if result['production_sha256_after'] != baseline:
            raise RuntimeError('Produktionskonfiguration während der Prüfung geändert.')
        save(directory, result)
        print('PASS: Echter Mountguard verweigert lokales Ersatzarchiv; Raw unverändert erhalten.')
        print(json.dumps(result, indent=2, ensure_ascii=False))
        return 0
    except BaseException as error:
        result.update(phase='stopped', error=f'{type(error).__name__}: {error}')
        if isinstance(error, subprocess.CalledProcessError):
            result['worker_stderr'] = error.stderr
        save(directory, result)
        print('STOP:', result['error'])
        return 1


if __name__ == '__main__':
    raise SystemExit(main())
