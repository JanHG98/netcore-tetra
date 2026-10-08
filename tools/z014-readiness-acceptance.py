#!/usr/bin/env python3
"""Run once inside CT150: fail a readiness check, then restore and verify it."""
import hashlib
import json
import os
from pathlib import Path
import socket
import stat
import subprocess
import sys
import time
import tomllib
from urllib.error import HTTPError
import urllib.request

sys.path.insert(0, '/usr/local/lib/netcore-deployment')
from common import atomic_write, load_config, toml_dump
from deploy import probe, service_spec

NODE = 'z014-test-hw'
SHA = 'c45a2ec1f5b7cdcfb766a2d81e8901b65f3dbacf'
LOCAL, CONTROLLER = 'http://127.0.0.1:8321', 'http://10.0.1.131:8320'
AGENT = Path('/etc/netcore/discovery.toml')
HARDWARE = Path('/etc/netcore/hardware-gateway.toml')
RUN = Path('/var/tmp/netcore-z014-readiness-c45a2ec1')
MISSING = '/z01-readiness-negative'
HTTP = urllib.request.build_opener(urllib.request.ProxyHandler({}))
report = {'node_id': NODE, 'hardware_commit': SHA, 'http_get_errors': 0,
          'job_poll_get_errors': 0, 'job_poll_http500': 0}
run_created = False

def save(phase=None, **fields):
    if phase:
        report['phase'] = phase
    report.update(fields)
    if run_created:
        atomic_write(RUN / 'result.json', json.dumps(report, indent=2))


def api(base, path, data=None):
    body = None if data is None else json.dumps(data).encode()
    request = urllib.request.Request(base + path, data=body,
                                    headers={'Content-Type': 'application/json'})
    try:
        with HTTP.open(request, timeout=20) as response:
            return json.load(response)
    except Exception as error:
        if data is None:
            save(http_get_errors=report['http_get_errors'] + 1)
            if base == LOCAL and path.startswith('/api/v1/jobs/'):
                save(job_poll_get_errors=report['job_poll_get_errors'] + 1)
                if isinstance(error, HTTPError) and error.code == 500:
                    save(job_poll_http500=report['job_poll_http500'] + 1)
        raise


def guard():
    manifest = api(LOCAL, '/api/v1/manifest')
    assert (manifest['node_id'], manifest['role'], manifest['services']) == (NODE, 'agent', [])
    local = api(LOCAL, '/api/v1/jobs')
    remote = [job for job in api(CONTROLLER, '/api/v1/jobs')
              if job.get('request', {}).get('node_id') == NODE]
    for job in local + remote:
        assert job['status'] not in ('queued', 'running'), f"Auftrag noch offen: {job['id']}"
        assert not job.get('result', {}).get('remote_uncertain'), f"Auftrag unklar: {job['id']}"
    return [job['id'] for job in local]


def restart_agent():
    subprocess.run(['systemctl', 'restart', 'netcore-discovery.service'], check=True, timeout=30)
    deadline = time.monotonic() + 30
    while time.monotonic() < deadline:
        try:
            if api(LOCAL, '/health/ready').get('ready') is True:
                guard()
                return
        except (OSError, ValueError):
            pass
        time.sleep(1)
    raise RuntimeError('Agent nach Neustart nicht erreichbar')


def update(phase):
    previous = guard()
    # Persist ambiguity before sending; neither lost replies nor reruns repeat POST.
    save(phase + '_post_pending', previous_job_ids=previous)
    job = api(LOCAL, '/api/v1/jobs', {'service': 'hardware-gateway', 'action': 'update',
                                     'commit': SHA, 'confirm_restart': True})
    key = job['id']
    assert isinstance(key, str) and len(key) == 32 and all(c in '0123456789abcdef' for c in key)
    save(phase + '_running', **{phase + '_job_id': key})
    print(f'{phase}: {LOCAL}/api/v1/jobs/{key}', flush=True)
    deadline = time.monotonic() + 600
    while time.monotonic() < deadline:
        try:
            job = api(LOCAL, '/api/v1/jobs/' + key)
        except (OSError, ValueError) as error:
            print(f'GET erneut: {error}', flush=True)
            time.sleep(2)
            continue
        assert job['id'] == key and job['status'] in ('queued', 'running', 'succeeded', 'failed', 'interrupted')
        if job['status'] not in ('queued', 'running'):
            assert not job.get('result', {}).get('remote_uncertain'), 'Auftragsausgang unklar'
            atomic_write(RUN / (phase + '-job.json'), json.dumps(job, indent=2))
            save(phase + '_terminal')
            return job
        time.sleep(2)
    raise RuntimeError(f'Auftrag weiterhin offen: {LOCAL}/api/v1/jobs/{key}; kein Neustart/zweiter POST')


def hardware_unchanged(expected):
    assert hashlib.sha256(HARDWARE.read_bytes()).hexdigest() == expected, 'Hardware-TOML verändert'
    for path in (HARDWARE, Path('/var/lib/netcore-discovery-hardware-gateway/config.toml')):
        monitoring = tomllib.loads(path.read_text())['monitoring']
        assert monitoring['heartbeat_timeout_secs'] == 37 and monitoring['outputs_enabled'] is False


def main():
    global run_created
    assert os.geteuid() == 0 and socket.gethostname() == NODE, 'Nur als root innerhalb CT150 ausführen'
    assert not RUN.exists(), f'Test bereits begonnen: {RUN}/result.json prüfen; nicht erneut starten'
    guard()
    cfg = load_config(AGENT)
    assert cfg['node_id'] == NODE and cfg['role'] == 'agent' and cfg.get('managed_services') == []
    spec = service_spec(cfg, 'hardware-gateway')
    assert spec['health'] == '/health/ready' and spec['config_target'] == str(HARDWARE)
    marker = Path(cfg['state_dir']) / 'deployed-hardware-gateway.json'
    before = json.loads(marker.read_text())
    assert before['state'] == 'installed' and before['commit'] == SHA
    original = AGENT.read_bytes().decode('utf-8')
    mode = stat.S_IMODE(AGENT.stat().st_mode)
    expected = hashlib.sha256(HARDWARE.read_bytes()).hexdigest()
    hardware_unchanged(expected)
    assert probe(spec) and probe(spec, live=True), 'Hardware-Gateway vor Test nicht bereit'
    try:
        probe(dict(spec, health=MISSING))
    except HTTPError as error:
        assert error.code == 404, f'Negativer Pfad liefert unerwartet HTTP{error.code}'
    else:
        raise RuntimeError('Negativer Ready-Pfad liefert kein HTTP404')
    RUN.mkdir(mode=0o700)
    run_created = True
    atomic_write(RUN / 'agent.original.toml', original)
    save('prepared', original_mode=mode, hardware_sha256=expected,
         agent_sha256=hashlib.sha256(original.encode()).hexdigest())
    raw = tomllib.loads(original)
    raw.setdefault('services', []).append({'name': 'hardware-gateway', 'health': MISSING})
    atomic_write(AGENT, toml_dump(raw), mode)
    assert service_spec(load_config(AGENT), 'hardware-gateway')['health'] == MISSING
    save('negative_configured')
    guard()
    restart_agent()
    negative = update('negative')
    save(negative_status=negative['status'])
    accepted = False
    try:
        failed = json.loads(marker.read_text())
        save(negative_marker=failed, negative_live=probe(spec, live=True))
        hardware_unchanged(expected)
        save(negative_hardware_unchanged=True)
        accepted = (negative['status'] == 'failed'
                    and 'Healthcheck fehlgeschlagen' in negative['result'].get('error', '')
                    and failed.get('commit') == '' and failed.get('state') == 'installing'
                    and failed.get('previous_commit') == SHA and failed.get('requested_commit') == SHA
                    and report['negative_live'] is True)
    except Exception as error:
        save(negative_validation_error=str(error))
    # A known terminal result always restores the agent configuration first.
    guard()
    atomic_write(AGENT, original, mode)
    save('agent_config_restored', negative_accepted=accepted)
    restart_agent()
    assert AGENT.read_bytes().decode('utf-8') == original
    if not accepted:
        raise RuntimeError('Negativbefund unerwartet; Agent-TOML wiederhergestellt, Ergebnis prüfen')
    recovered = update('recovery')
    save(recovery_status=recovered['status'])
    final = json.loads(marker.read_text())
    save(recovery_marker=final)
    assert recovered['status'] == 'succeeded', recovered['result']
    assert recovered['result'].get('commit') == SHA and recovered['result'].get('ready') is True
    assert recovered['result'].get('live') is True and final.get('commit') == SHA and final.get('state') == 'installed'
    hardware_unchanged(expected)
    assert AGENT.read_bytes().decode('utf-8') == original
    if report['job_poll_http500']:
        raise RuntimeError('Readiness/Recovery bestanden, aber Jobstatus lieferte weiterhin HTTP500')
    save('passed')
    print('PASS: Negativprüfung, Wiederherstellung, Prüfwert 37.')
    print(json.dumps(report, indent=2))
    print(f'Nachweis: {RUN}/result.json')


if __name__ == '__main__':
    try:
        main()
    except Exception as error:
        save(error=str(error))
        print(f'STOP: {error}\nNachweis: {RUN}/result.json; laufende/unklare Aufträge zuerst prüfen.', file=sys.stderr)
        sys.exit(1)
