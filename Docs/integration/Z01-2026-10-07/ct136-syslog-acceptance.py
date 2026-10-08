#!/usr/bin/env python3
"""Guarded CT136 TCP receiver, local preview and real NAS archive acceptance."""
import datetime as dt
import gzip
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import socket
import subprocess
import tempfile
import time
import tomllib
import urllib.parse
import urllib.request
import uuid

OBS_CONFIG = Path('/etc/netcore/observability.toml')
LOG_CONFIG = Path('/etc/netcore/syslog.json')
LOG_STORE = Path('/opt/netcore-observability/logging/log_store.py')
BASES = ('http://127.0.0.1:8210', 'http://10.0.1.143:8210')
ARCHIVE_UNIT = 'netcore-syslog-archive.service'
CHECKPOINT_ROOT = '/var/tmp'
HTTP = urllib.request.build_opener(urllib.request.ProxyHandler({}))
PROPERTIES = ('LoadState', 'ActiveState', 'User', 'Group', 'Type', 'ExecStart',
              'InvocationID', 'ExecMainStartTimestampMonotonic',
              'ExecMainCode', 'ExecMainStatus', 'Result')


def require(condition, message):
    if not condition:
        raise RuntimeError(message)


def api(base, path):
    with HTTP.open(base + path, timeout=5) as response:
        return json.load(response)


def unit(name, timeout=10):
    args = ['systemctl', 'show', name, '--no-pager']
    args += ['--property=' + key for key in PROPERTIES]
    reply = subprocess.run(args, check=True, capture_output=True, text=True,
                           timeout=timeout)
    return dict(line.split('=', 1) for line in reply.stdout.splitlines() if '=' in line)


def fingerprint(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def save(directory, result, phase=None):
    if phase:
        result['phase'] = phase
    result['updated_at'] = dt.datetime.now(dt.timezone.utc).isoformat()
    fd, temporary = tempfile.mkstemp(prefix='.result-', dir=directory)
    try:
        with os.fdopen(fd, 'w') as output:
            json.dump(result, output, indent=2, ensure_ascii=False)
            output.write('\n')
            output.flush()
            os.fsync(output.fileno())
        os.replace(temporary, directory / 'result.json')
        directory_fd = os.open(directory, os.O_RDONLY | os.O_DIRECTORY)
        try:
            os.fsync(directory_fd)
        finally:
            os.close(directory_fd)
    finally:
        if os.path.exists(temporary):
            os.unlink(temporary)


def run(directory, result):
    require(os.geteuid() == 0 and socket.gethostname() == 'Observability',
            'Nur root im CT136 Observability darf diesen Test ausführen.')
    cfg = json.loads(LOG_CONFIG.read_text())
    expected = {'state_dir': '/var/lib/netcore-observability/logs',
                'archive_mount': '/mnt/nfs-share', 'archive_directory': 'Logs/NetCore',
                'collector_id': 'observability-10.0.1.143', 'nms_url': BASES[0]}
    require(all(cfg.get(key) == value for key, value in expected.items()),
            'Syslog-Standortkonfiguration stimmt nicht mit CT136 überein.')
    require(set(cfg['allowed_networks']) == {'10.0.1.0/24', '127.0.0.0/8'},
            'Unerwartete Syslog-Allowlist.')
    require(tomllib.loads(OBS_CONFIG.read_text())['server']['bind'] == '10.0.1.143:8210',
            'Unerwarteter Observability-Bind.')
    hashes = {str(path): fingerprint(path) for path in (OBS_CONFIG, LOG_CONFIG)}
    result['config_sha256'] = hashes
    spec = importlib.util.spec_from_file_location('ct136_log_store', LOG_STORE)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    cfg = module.load_config(LOG_CONFIG)
    os.close(module.open_share(cfg))  # Require a real NFS/CIFS mount; no fallback.
    for name in ('netcore-observability.service', 'netcore-syslog.service',
                 'netcore-syslog-preview.service'):
        status = unit(name)
        require(status['LoadState'] == 'loaded' and status['ActiveState'] == 'active'
                and status['User'] == status['Group'] == 'netcore-observability',
                'Dienst nicht im erwarteten Zustand: ' + name)
    before = unit(ARCHIVE_UNIT)
    require(before['LoadState'] == 'loaded' and before['ActiveState'] in ('inactive', 'failed')
            and before['User'] == before['Group'] == 'netcore-observability'
            and before['Type'] == 'oneshot'
            and 'argv[]=/usr/bin/python3 ' + str(LOG_STORE) + ' archive ;' in before['ExecStart'],
            'Archiver läuft bereits oder seine Unit ist unerwartet.')
    for base in BASES:
        require(api(base, '/health/ready')['ready'] is True, base + ' ist nicht bereit.')
        require(api(base, '/api/v1/config')['server']['bind'] == '10.0.1.143:8210',
                'HTTP-Konfiguration passt nicht: ' + base)
    marker = 'Z014-SYSLOG-' + uuid.uuid4().hex
    sent_at = dt.datetime.now(dt.timezone.utc)
    result.update(marker=marker, marker_sent_at=sent_at.isoformat(),
                  archive_before=before, tcp_attempts=1)
    save(directory, result, 'tcp_send_started')
    print('Marker:', marker, flush=True)
    timestamp = sent_at.isoformat().replace('+00:00', 'Z')
    payload = f'<134>1 {timestamp} Observability z014-syslog - - - {marker}\n'.encode()
    with socket.create_connection(('127.0.0.1', 514), timeout=5) as connection:
        connection.sendall(payload)  # Exactly one attempt; never resend after ambiguity.
    save(directory, result, 'preview_pending')
    query = '/api/v1/logs?' + urllib.parse.urlencode({'contains': marker, 'limit': 20})
    deadline, notice = time.monotonic() + 300, time.monotonic() + 30
    while True:
        matches = [row for row in api(BASES[0], query) if marker in row.get('message', '')]
        if matches:
            break
        require(time.monotonic() < deadline, 'Marker nach 300s nicht in der Vorschau.')
        if time.monotonic() >= notice:
            print('Warte auf denselben Marker in der Vorschau …', flush=True)
            notice = time.monotonic() + 30
        time.sleep(1)
    require(len(matches) == 1, 'Marker ist in der Vorschau nicht eindeutig.')
    row = matches[0]
    fields = row.get('fields', {})
    record_id = fields.get('syslog_id', '')
    require(re.fullmatch('[0-9a-f]{32}', record_id) and fields.get('source_ip') == '127.0.0.1'
            and fields.get('transport') == 'imtcp', 'Keine bestätigte TCP-Empfangsvorschau.')
    remote = api(BASES[1], query)
    require(any(item.get('fields', {}).get('syslog_id') == record_id
                and marker in item.get('message', '') for item in remote),
            'Management-API zeigt nicht denselben Empfang.')
    result.update(syslog_id=record_id, preview_record=row)
    save(directory, result, 'preview_confirmed')
    raw = Path(cfg['state_dir']) / 'raw'
    found = []
    for path in raw.iterdir():
        if path.suffix not in ('.open', '.jsonl'):
            continue
        try:
            if path.stat().st_mtime < sent_at.timestamp() - 1:
                continue
            with path.open() as source:
                for line in source:
                    if not line.endswith('\n') and path.suffix == '.open':
                        continue
                    record = json.loads(line)
                    if record.get('id') == record_id and marker in record.get('message', ''):
                        found.append((path, record))
        except FileNotFoundError:
            continue
    require(len(found) == 1, 'Lokales Raw-Segment fehlt oder ist nicht eindeutig.')
    source, record = found[0]
    require(module.SEGMENT.fullmatch(source.with_suffix('.jsonl').name), 'Unerwarteter Segmentname.')
    day = dt.datetime.strptime(source.name[:8], '%Y%m%d').date().isoformat()
    archive = (Path(cfg['archive_mount']) / cfg['archive_directory'] / cfg['collector_id']
               / day / (source.with_suffix('.jsonl').name + '.gz'))
    result.update(raw_segment=str(source), raw_record=record, archive_file=str(archive))
    save(directory, result, 'preview_and_raw_confirmed')
    require(all(fingerprint(Path(path)) == digest for path, digest in hashes.items()),
            'Konfiguration während des Tests geändert.')
    current = unit(ARCHIVE_UNIT)
    require(current['ActiveState'] in ('inactive', 'failed')
            and all(current[key] == before[key] for key in
                    ('InvocationID', 'ExecMainStartTimestampMonotonic', 'ExecStart')),
            'Ein anderer Archivlauf hat begonnen; kein zusätzlicher Start.')
    os.close(module.open_share(cfg))
    result['archive_start_attempts'] = 1
    save(directory, result, 'archive_start_requested')
    subprocess.run(['systemctl', 'start', '--no-block', ARCHIVE_UNIT], check=True, timeout=10)
    deadline = time.monotonic() + 60
    while True:
        status = unit(ARCHIVE_UNIT, timeout=min(10, max(1, deadline - time.monotonic())))
        result['archive_unit'] = status
        fresh = (int(status['ExecMainStartTimestampMonotonic']) > int(before['ExecMainStartTimestampMonotonic'])
                 or bool(status['InvocationID']) and status['InvocationID'] != before['InvocationID'])
        if fresh and status['ActiveState'] in ('inactive', 'failed'):
            break
        require(time.monotonic() < deadline,
                'Archivlauf nach 60s noch offen/unbestätigt; nicht erneut starten.')
        time.sleep(1)
    require(status['ActiveState'] == 'inactive' and status['Result'] == 'success'
            and status['ExecMainStatus'] == '0' and status['ExecMainCode'] in ('1', 'exited'),
            'Der neue Archivlauf ist fehlgeschlagen.')
    archived = json.loads((Path(cfg['state_dir']) / 'archive-status.json').read_text())
    result['archive_status'] = archived
    save(directory, result, 'archive_completed')
    require(archived.get('error') is None and archived.get('archived_segments', 0) >= 1
            and dt.datetime.fromisoformat(archived['last_attempt']) >= sent_at,
            'Kein neuer erfolgreicher Archivlauf mit Segmenten.')
    seen, digest = 0, hashlib.sha256()
    with gzip.open(archive, 'rb') as saved:
        for line in saved:  # Read through EOF, including gzip CRC/footer checks.
            digest.update(line)
            if json.loads(line) == record:
                seen += 1
    require(seen == 1 and not source.exists() and not source.with_suffix('.jsonl').exists(),
            'Archivmarker oder verifizierte Entfernung des Raw-Segments fehlt.')
    require(all(fingerprint(Path(path)) == value for path, value in hashes.items()),
            'Konfiguration wurde während der Archivprüfung geändert.')
    result['archive_decompressed_sha256'] = digest.hexdigest()
    save(directory, result, 'passed')
    print('PASS: TCP-Empfang, beide Vorschau-APIs und NAS-gzip mit demselben Marker.', flush=True)


def main():
    directory = Path(tempfile.mkdtemp(prefix='netcore-ct136-syslog-', dir=CHECKPOINT_ROOT))
    os.chmod(directory, 0o700)
    result = {'phase': 'created', 'started_at': dt.datetime.now(dt.timezone.utc).isoformat()}
    print('Nachweis:', directory / 'result.json', flush=True)
    try:
        run(directory, result)
    except BaseException as error:
        result.update(failed_phase=result['phase'], error=f'{type(error).__name__}: {error}')
        save(directory, result, 'stopped')
        print('STOP:', result['error'], flush=True)
        return 1
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
