#!/usr/bin/env python3
"""Disposable Linux fixture: real systemd, service-owned SQLite and root CLI.

This creates a sleeping fixture unit, not an installed Deployment controller.
The remote agent is a loopback HTTP fixture. No deployment or installer runs.
Never run this fixture on a Deployment VM; it refuses an existing controller.
"""
from contextlib import closing
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import argparse
import json
import os
from pathlib import Path
import pwd
import shutil
import sqlite3
import stat
import subprocess
import sys
import threading

HELPER = Path(__file__).resolve().parent / 'operator/reconcile-remote-job.py'
UNIT = Path('/etc/systemd/system/netcore-deployment.service')
STATE = Path('/var/lib/netcore-reconcile-fixture')
CONFIG_DIR = Path('/etc/netcore-reconcile-fixture')
CONFIG = CONFIG_DIR / 'deployment.toml'
USER = 'netcore-deploy'
LOCAL_ID = '193b08c7b05b4b54951baab1eea95df3'
REMOTE_ID = '721b00fef0b143b28253123f7a168c7c'
UNBOUND_LOCAL_ID = '5' * 32
UNBOUND_REMOTE_ID = '6' * 32
COMMIT = 'c45a2ec1f5b7cdcfb766a2d81e8901b65f3dbacf'


def command(*args, input=None):
    return subprocess.run(args, input=input, check=True, text=True,
                          capture_output=True, timeout=20).stdout


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--disposable-runner', action='store_true', required=True)
    parser.parse_args()
    if sys.platform != 'linux' or os.geteuid() != 0:
        raise RuntimeError('This fixture requires root on a disposable Linux runner.')
    properties = dict(line.split('=', 1) for line in command(
        'systemctl', 'show', 'netcore-deployment.service', '-p', 'LoadState').splitlines())
    if properties.get('LoadState') != 'not-found' or any(
            path.exists() or path.is_symlink() for path in (UNIT, STATE, CONFIG_DIR)):
        raise RuntimeError('Refusing an existing controller or fixture path.')
    try:
        pwd.getpwnam(USER)
    except KeyError:
        pass
    else:
        raise RuntimeError('Refusing an existing service account.')

    remote = dict(id=REMOTE_ID, status='succeeded', request=dict(
        service='node-gateway', action='update', commit=COMMIT, confirm_restart=True),
        result=dict(service='node-gateway', commit=COMMIT, ready=True))
    manifest = dict(node_id='Node-Gateway', environment='netcore-openlab',
                    protocol='netcore.discovery.v1', role='agent', security_mode='open_lab')
    selected = dict(remote, id=UNBOUND_REMOTE_ID, created=150,
                    result=dict(service='node-gateway', commit=COMMIT, ready=False, live=True))
    calls = []

    class Handler(BaseHTTPRequestHandler):
        def do_GET(self):
            calls.append(('GET', self.path))
            if self.path == '/api/v1/manifest':
                value = manifest
            elif self.path == '/api/v1/jobs/' + REMOTE_ID:
                value = remote
            elif self.path == '/api/v1/jobs/' + UNBOUND_REMOTE_ID:
                value = selected
            else:
                self.send_error(404)
                return
            body = json.dumps(value).encode()
            self.send_response(200)
            self.send_header('Content-Type', 'application/json')
            self.send_header('Content-Length', str(len(body)))
            self.end_headers()
            self.wfile.write(body)

        def do_POST(self):
            calls.append(('POST', self.path))
            self.send_error(405)

        def log_message(self, *args):
            pass

    server = ThreadingHTTPServer(('127.0.0.1', 0), Handler)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    account_created = False
    unit_created = False
    try:
        command('useradd', '--system', '--user-group', '--no-create-home',
                '--home-dir', '/nonexistent', '--shell', '/usr/sbin/nologin', USER)
        account_created = True
        account = pwd.getpwnam(USER)
        CONFIG_DIR.mkdir(mode=0o755)
        CONFIG.write_text('role = "controller"\nenvironment = "netcore-openlab"\n'
                          'state_dir = "/var/lib/netcore-reconcile-fixture"\n'
                          'allowed_networks = ["127.0.0.0/8"]\n', encoding='utf-8')
        CONFIG.chmod(0o644)
        # A service account cannot assume access to the runner's home checkout.
        helper_copy = CONFIG_DIR / 'reconcile-remote-job.py'
        shutil.copyfile(HELPER, helper_copy)
        helper_copy.chmod(0o644)
        UNIT.write_text('[Unit]\nDescription=Disposable remote reconciliation CI fixture\n'
                        '[Service]\nType=simple\nUser=netcore-deploy\nGroup=netcore-deploy\n'
                        'ExecStart=/usr/bin/sleep infinity\n'
                        'StateDirectory=netcore-reconcile-fixture\nStateDirectoryMode=0750\n'
                        'UMask=0077\n', encoding='utf-8')
        unit_created = True
        command('systemctl', 'daemon-reload')
        command('systemctl', 'start', 'netcore-deployment.service')
        database = STATE / 'jobs.sqlite3'
        request = dict(node_id='Node-Gateway', service='node-gateway', action='update',
                       ref=COMMIT, commit=None, confirm_restart=True)
        result = dict(remote_uncertain=True, remote_job=REMOTE_ID,
                      remote_url=f'http://127.0.0.1:{server.server_port}/api/v1/jobs/{REMOTE_ID}')
        # SQLite really creates its file as the unit's service user, not as root.
        create = ('import json, os, sqlite3\n'
                  'from contextlib import closing\n'
                  f'with closing(sqlite3.connect({str(database)!r})) as db:\n'
                  ' assert db.execute("PRAGMA journal_mode=WAL").fetchone()[0] == "wal"\n'
                  ' db.execute("CREATE TABLE jobs (id TEXT PRIMARY KEY,created REAL,updated REAL,'
                  'status TEXT,request TEXT,log TEXT,result TEXT)")\n'
                  f' db.execute("INSERT INTO jobs VALUES (?,?,?,?,?,?,?)", '
                  f'({LOCAL_ID!r},100,200,"failed",{json.dumps(request)!r},"original fixture log",'
                  f'{json.dumps(result)!r}))\n'
                  ' db.commit()\n'
                  f'os.chmod({str(database)!r},0o600)\n')
        command('runuser', '-u', USER, '--', '/usr/bin/python3', '-B', '-', input=create)
        assert database.stat().st_uid == account.pw_uid
        assert stat.S_IMODE(database.stat().st_mode) == 0o600
        before = database.read_bytes()

        def cli(*options, as_user=False, job_id=LOCAL_ID):
            args = ['/usr/bin/python3', '-B', str(helper_copy), job_id, '--config', str(CONFIG), *options]
            if as_user:
                args = ['runuser', '-u', USER, '--', *args]
            response = subprocess.run(args, check=False, text=True, capture_output=True, timeout=30)
            value = json.loads(response.stdout)
            assert response.returncode in (0, 2), (response.returncode, response.stderr)
            return value

        diagnosis = cli(as_user=True)
        assert diagnosis['phase'] == 'verified', diagnosis
        assert diagnosis['commit'] == COMMIT
        assert diagnosis['commit_source'] == 'local_pinned_commit'
        assert database.read_bytes() == before
        print('PASS: unprivileged read-only diagnosis with the real service-owned 0600 database')

        running = cli('--apply')
        assert running['phase'] == 'pending' and running['reason'] == 'controller_must_be_stopped', running
        assert database.read_bytes() == before
        command('systemctl', 'stop', 'netcore-deployment.service')
        denied = cli('--apply', as_user=True)
        assert denied['phase'] == 'pending' and not denied['changed'], denied
        assert database.read_bytes() == before
        print('PASS: actual running systemd unit and unprivileged apply both refuse mutation')

        database.chmod(0o640)
        insecure = cli('--apply')
        assert insecure['phase'] == 'pending' and not insecure['changed'], insecure
        assert database.read_bytes() == before
        database.chmod(0o600)
        os.chown(database, pwd.getpwnam('nobody').pw_uid, account.pw_gid)
        outsider = cli('--apply')
        assert outsider['phase'] == 'pending' and not outsider['changed'], outsider
        assert database.read_bytes() == before
        os.chown(database, account.pw_uid, account.pw_gid)
        STATE.chmod(0o770)
        writable = cli('--apply')
        assert writable['phase'] == 'pending' and not writable['changed'], writable
        assert database.read_bytes() == before
        STATE.chmod(0o750)
        print('PASS: group-readable DB, unrelated owner and writable state directory refuse mutation')

        remote['status'] = 'running'
        pending = cli('--apply')
        assert pending['phase'] == 'pending' and pending['reason'] == 'remote_job_not_terminal', pending
        assert database.read_bytes() == before
        remote['status'] = 'succeeded'
        applied = cli('--apply')
        assert applied['phase'] == 'reconciled' and applied['changed'], applied
        assert database.stat().st_uid == account.pw_uid
        assert stat.S_IMODE(database.stat().st_mode) == 0o600
        with closing(sqlite3.connect(database)) as db:
            status, stored_request, log, stored_result = db.execute(
                'SELECT status,request,log,result FROM jobs WHERE id=?', (LOCAL_ID,)).fetchone()
            assert status == 'succeeded'
            assert json.loads(stored_request) == request and log == 'original fixture log'
            assert json.loads(stored_result)['remote_uncertain'] is False
            assert db.execute('SELECT count(*) FROM remote_job_reconciliations').fetchone()[0] == 1
        # A completed root SQLite connection must not leave root-owned WAL/SHM
        # files that prevent the controller account from writing after restart.
        assert not Path(str(database) + '-wal').exists()
        assert not Path(str(database) + '-shm').exists()
        write_as_service = ('import sqlite3\nfrom contextlib import closing\n'
                            f'with closing(sqlite3.connect({str(database)!r})) as db:\n'
                            ' assert db.execute("PRAGMA journal_mode").fetchone()[0] == "wal"\n'
                            ' assert db.execute("UPDATE jobs SET log=log WHERE id=?", '
                            f'({LOCAL_ID!r},)).rowcount == 1\n'
                            ' db.commit()\n')
        command('runuser', '-u', USER, '--', '/usr/bin/python3', '-B', '-', input=write_as_service)
        assert database.stat().st_uid == account.pw_uid
        assert stat.S_IMODE(database.stat().st_mode) == 0o600
        after = database.read_bytes()
        repeated = cli('--apply')
        assert repeated['phase'] == 'already_resolved' and not repeated['changed'], repeated
        assert database.read_bytes() == after
        assert all(method == 'GET' for method, _ in calls), calls
        print('PASS: root CLI reconciles terminal proof once, preserves ownership and issues only GETs')
        print('PASS: the service user can write the WAL database after root apply without blocked sidecars')

        unbound = dict(remote_uncertain=True, remote_job='', dispatch_state='posting',
                       remote_url=f'http://127.0.0.1:{server.server_port}/api/v1/jobs')
        insert_unbound = ('import sqlite3\nfrom contextlib import closing\n'
                          f'with closing(sqlite3.connect({str(database)!r})) as db:\n'
                          ' db.execute("INSERT INTO jobs VALUES (?,?,?,?,?,?,?)", '
                          f'({UNBOUND_LOCAL_ID!r},100,200,"failed",{json.dumps(request)!r},'
                          f'"lost POST reply",{json.dumps(unbound)!r}))\n'
                          ' db.commit()\n')
        command('runuser', '-u', USER, '--', '/usr/bin/python3', '-B', '-', input=insert_unbound)
        explicit_before = database.read_bytes()
        missing = cli(job_id=UNBOUND_LOCAL_ID, as_user=True)
        assert missing['reason'] == 'remote_job_receipt_missing_no_automatic_search', missing
        diagnosis = cli('--remote-job', UNBOUND_REMOTE_ID, job_id=UNBOUND_LOCAL_ID, as_user=True)
        assert diagnosis['phase'] == 'verified' and diagnosis['receipt_source'] == 'operator_selected', diagnosis
        assert database.read_bytes() == explicit_before
        selected['created'] = 205.01
        outside = cli('--remote-job', UNBOUND_REMOTE_ID, '--apply', job_id=UNBOUND_LOCAL_ID)
        assert outside['reason'] == 'explicit_remote_job_outside_local_attempt_window', outside
        assert database.read_bytes() == explicit_before
        selected['created'] = 150
        adopted = cli('--remote-job', UNBOUND_REMOTE_ID, '--apply', job_id=UNBOUND_LOCAL_ID)
        assert adopted['phase'] == 'reconciled' and adopted['status'] == 'failed', adopted
        with closing(sqlite3.connect(database)) as db:
            result = json.loads(db.execute('SELECT result FROM jobs WHERE id=?', (UNBOUND_LOCAL_ID,)).fetchone()[0])
            assert result['remote_uncertain'] is False
            assert result['reconciliation']['receipt_source'] == 'operator_selected'
            audit = json.loads(db.execute('SELECT proof FROM remote_job_reconciliations WHERE job_id=?',
                                         (UNBOUND_LOCAL_ID,)).fetchone()[0])
            assert audit['attempt_window']['remote_created'] == 150
            assert audit['receipt_source'] == 'operator_selected'
        assert database.stat().st_uid == account.pw_uid
        assert stat.S_IMODE(database.stat().st_mode) == 0o600
        assert not Path(str(database) + '-wal').exists()
        assert not Path(str(database) + '-shm').exists()
        command('runuser', '-u', USER, '--', '/usr/bin/python3', '-B', '-', input=write_as_service)
        assert all(method == 'GET' and path != '/api/v1/jobs' for method, path in calls), calls
        print('PASS: explicit lost-POST association verifies time, preserves degraded failure and service-owned WAL writes')
        print('Native reconciliation fixture passed; no installed controller or remote deployment was used.')
    finally:
        server.shutdown()
        server.server_close()
        thread.join(2)
        if unit_created:
            command('systemctl', 'stop', 'netcore-deployment.service')
            UNIT.unlink()
            command('systemctl', 'daemon-reload')
        # These are fixed fixture paths, refused at entry if already present.
        for directory in (STATE, CONFIG_DIR):
            if directory.exists():
                if directory.is_symlink() or directory.parent not in (Path('/var/lib'), Path('/etc')):
                    raise RuntimeError('Refusing unexpected fixture cleanup path.')
                shutil.rmtree(directory)
        if account_created:
            command('userdel', USER)
    return 0


if __name__ == '__main__':
    raise SystemExit(main())

