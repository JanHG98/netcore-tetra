#!/usr/bin/env python3
"""Exact remote GET reconciliation with real HTTP and SQLite; no deployments."""
from contextlib import closing
import copy
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import importlib.util
import json
import os
from pathlib import Path
import sqlite3
import tempfile
import threading
import unittest
from unittest.mock import patch

SCRIPT = Path(__file__).resolve().parent / 'operator/reconcile-remote-job.py'
SPEC = importlib.util.spec_from_file_location('reconcile_remote_under_test', SCRIPT)
HELPER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(HELPER)
LOCAL_ID = '1' * 32
REMOTE_ID = '2' * 32
COMMIT = 'a' * 40


class RemoteReconciliationTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.path = Path(self.temporary.name) / 'jobs.sqlite3'
        self.remote = dict(id=REMOTE_ID, status='succeeded', request=dict(service='packet-core',
                           action='update', commit=COMMIT, confirm_restart=True),
                           result=dict(service='packet-core', commit=COMMIT, ready=True),
                           log='REMOTE_SECRET_MUST_NOT_BE_COPIED')
        self.manifest = dict(node_id='LXC-PACKET-01', environment='netcore-openlab',
                             protocol=HELPER.PROTOCOL, role='agent', security_mode='open_lab')
        self.calls = []
        self.before_response = None
        case = self
        class Handler(BaseHTTPRequestHandler):
            def do_GET(self):
                case.calls.append(('GET', self.path))
                if case.before_response:
                    case.before_response(self.path)
                if self.path == '/api/v1/manifest':
                    value = case.manifest
                elif self.path == '/api/v1/jobs/' + REMOTE_ID:
                    value = case.remote
                else:
                    self.send_error(404)
                    return
                if value is None:
                    self.send_error(404)
                    return
                body = json.dumps(value).encode()
                self.send_response(200)
                self.send_header('Content-Type', 'application/json')
                self.send_header('Content-Length', str(len(body)))
                self.end_headers()
                self.wfile.write(body)
            def do_POST(self):
                case.calls.append(('POST', self.path))
                self.send_error(405)
            def log_message(self, *args):
                pass
        self.server = ThreadingHTTPServer(('127.0.0.1', 0), Handler)
        self.thread = threading.Thread(target=self.server.serve_forever, daemon=True)
        self.thread.start()
        self.addCleanup(self.stop_server)
        self.origin = 'http://127.0.0.1:' + str(self.server.server_port)
        self.url = self.origin + '/api/v1/jobs/' + REMOTE_ID
        self.cfg = dict(role='controller', environment='netcore-openlab', allowed_networks=['127.0.0.0/8'])
        self.request = dict(node_id='LXC-PACKET-01', service='packet-core', action='update',
                            ref='main', confirm_restart=True)
        self.result = dict(remote_uncertain=True, remote_job=REMOTE_ID, remote_url=self.url,
                           error='Old polling timeout')
        with closing(sqlite3.connect(self.path)) as db:
            db.execute('CREATE TABLE jobs (id TEXT PRIMARY KEY,created REAL,updated REAL,status TEXT,'
                       'request TEXT,log TEXT,result TEXT)')
            db.execute('INSERT INTO jobs VALUES (?,?,?,?,?,?,?)',
                       (LOCAL_ID, 100, 200, 'failed', json.dumps(self.request), 'original local log', json.dumps(self.result)))
            db.execute('CREATE TABLE untouched (value TEXT)')
            db.execute("INSERT INTO untouched VALUES ('must remain')")
            db.commit()
        os.chmod(self.path, 0o600)

    def stop_server(self):
        self.server.shutdown()
        self.server.server_close()
        self.thread.join(2)

    def row(self):
        with closing(sqlite3.connect(self.path)) as db:
            return db.execute('SELECT * FROM jobs WHERE id=?', (LOCAL_ID,)).fetchone()

    def write_local(self, *, request=None, result=None, log=None, status=None):
        row = self.row()
        with closing(sqlite3.connect(self.path)) as db:
            db.execute('UPDATE jobs SET request=?,result=?,log=?,status=? WHERE id=?',
                       (json.dumps(request) if request is not None else row[4],
                        json.dumps(result) if result is not None else row[6],
                        log if log is not None else row[5], status or row[3], LOCAL_ID))
            db.commit()

    def run_helper(self, *, apply=False, stopped=lambda: True):
        # Trust only this fixture's actual UID; production resolves systemd User.
        return HELPER.reconcile(self.path, LOCAL_ID, self.cfg, apply=apply, stopped=stopped,
                                trusted_uid=os.geteuid() if os.name == 'posix' else 0)

    def assert_pending_unchanged(self, reason):
        before = self.path.read_bytes()
        value = self.run_helper(apply=True)
        self.assertEqual(value['phase'], 'pending', value)
        self.assertEqual(value['reason'], reason, value)
        self.assertEqual(self.path.read_bytes(), before)
        self.assertTrue(all(method == 'GET' for method, _ in self.calls))

    def test_diagnosis_does_not_write_and_legacy_main_uses_historical_accepted_sha(self):
        before = self.path.read_bytes()
        value = self.run_helper()
        self.assertEqual(value['phase'], 'verified', value)
        self.assertEqual(value['commit'], COMMIT)
        self.assertEqual(value['commit_source'], 'observed_legacy_agent_request')
        self.assertEqual(self.path.read_bytes(), before)
        self.assertEqual(self.calls, [('GET', '/api/v1/manifest'), ('GET', '/api/v1/jobs/' + REMOTE_ID)])
        self.assertNotIn('SECRET', json.dumps(value))

    def test_operator_legacy_node_gateway_full_sha_ref_with_null_commit_is_pinned(self):
        pinned = 'c45a2ec1f5b7cdcfb766a2d81e8901b65f3dbacf'
        self.write_local(request=dict(self.request, node_id='Node-Gateway', service='node-gateway',
                                      ref=pinned, commit=None))
        self.manifest['node_id'] = 'Node-Gateway'
        self.remote['request'].update(service='node-gateway', commit=pinned)
        self.remote['result'].update(service='node-gateway', commit=pinned)
        value = self.run_helper()
        self.assertEqual(value['phase'], 'verified', value)
        self.assertEqual(value['commit_source'], 'local_pinned_commit')
        self.assertEqual(value['commit'], pinned)
        self.remote['request']['commit'] = COMMIT
        self.assert_pending_unchanged('remote_accepted_commit_mismatch')

    def test_apply_exact_success_is_atomic_audited_and_idempotent(self):
        original = self.row()
        value = self.run_helper(apply=True)
        self.assertEqual(value['phase'], 'reconciled', value)
        row = self.row()
        self.assertEqual(row[3], 'succeeded')
        self.assertEqual(row[4:6], original[4:6])
        after = json.loads(row[6])
        self.assertIs(after['remote_uncertain'], False)
        self.assertEqual(after['commit'], COMMIT)
        self.assertNotIn('error', after)
        self.assertEqual(len(self.calls), 4)
        with closing(sqlite3.connect(self.path)) as db:
            audit = db.execute('SELECT before_result,proof FROM remote_job_reconciliations').fetchall()
            self.assertEqual(len(audit), 1)
            self.assertEqual(audit[0][0], original[6])
            self.assertEqual(json.loads(audit[0][1])['accepted_request']['commit'], COMMIT)
            self.assertNotIn('SECRET', audit[0][1])
            self.assertEqual(db.execute('SELECT * FROM untouched').fetchall(), [('must remain',)])
        value = self.run_helper(apply=True)
        self.assertEqual(value['phase'], 'already_resolved')
        self.assertEqual(self.row(), row)
        self.assertEqual(len(self.calls), 4)

    def test_terminal_failure_and_interruption_release_only_transport_uncertainty(self):
        for status in ('failed', 'interrupted'):
            with self.subTest(status=status):
                self.remote['status'] = status
                self.remote['result'] = {'error': 'PRIVATE_REMOTE_ERROR', 'ready': {'secret': 'PRIVATE_FLAG'},
                                        'commit': {'secret': 'PRIVATE_COMMIT'}, 'service': {'secret': 'PRIVATE_SERVICE'}}
                self.write_local(result=self.result, status='failed')
                value = self.run_helper(apply=True)
                self.assertEqual(value['phase'], 'reconciled', value)
                self.assertEqual(self.row()[3], status)
                result = json.loads(self.row()[6])
                self.assertIs(result['remote_uncertain'], False)
                self.assertNotIn('PRIVATE_REMOTE_ERROR', self.row()[6])
                self.assertNotIn('PRIVATE_FLAG', self.row()[6])
                with closing(sqlite3.connect(self.path)) as db:
                    audit = db.execute('SELECT proof FROM remote_job_reconciliations ORDER BY observed DESC LIMIT 1').fetchone()[0]
                    self.assertNotIn('PRIVATE_FLAG', audit)
                    self.assertNotIn('PRIVATE_COMMIT', audit)
                    self.assertNotIn('PRIVATE_SERVICE', audit)

    def test_succeeded_with_readiness_live_or_restart_false_becomes_failed(self):
        for name in ('ready', 'live', 'restarted'):
            with self.subTest(name=name):
                self.write_local(result=self.result, status='failed')
                self.remote['result'] = dict(service='packet-core', commit=COMMIT, ready=True)
                self.remote['result'][name] = False
                value = self.run_helper(apply=True)
                self.assertEqual(value['phase'], 'reconciled', value)
                self.assertEqual(self.row()[3], 'failed')
                self.assertIs(json.loads(self.row()[6])['remote_uncertain'], False)

    def test_remote_running_missing_malformed_or_nested_uncertain_stay_unchanged(self):
        original = copy.deepcopy(self.remote)
        for status in ('queued', 'running'):
            self.remote = dict(original, status=status)
            self.assert_pending_unchanged('remote_job_not_terminal')
        self.remote = None
        self.assert_pending_unchanged('remote_get_unavailable_or_invalid')
        self.remote = []
        self.assert_pending_unchanged('remote_job_identity_or_schema_mismatch')
        self.remote = original
        for invalid in (True, None, 0, 1, 'false', {}):
            self.remote['result']['remote_uncertain'] = invalid
            self.assert_pending_unchanged('remote_job_itself_uncertain')

    def test_wrong_agent_identity_never_releases(self):
        original = self.manifest.copy()
        for field in ('node_id', 'environment', 'protocol', 'role', 'security_mode'):
            with self.subTest(field=field):
                self.manifest = dict(original, **{field: 'wrong'})
                self.assert_pending_unchanged('remote_agent_identity_mismatch')

    def test_wrong_remote_id_service_action_confirm_commit_or_success_result_never_releases(self):
        original = copy.deepcopy(self.remote)
        self.remote['id'] = '3' * 32
        self.assert_pending_unchanged('remote_job_identity_or_schema_mismatch')
        for field, wrong in (('service', 'call-control'), ('action', 'install'), ('confirm_restart', False)):
            self.remote = copy.deepcopy(original)
            self.remote['request'][field] = wrong
            self.assert_pending_unchanged('remote_job_request_mismatch')
        self.remote = copy.deepcopy(original)
        self.remote['request']['commit'] = 'main'
        self.assert_pending_unchanged('remote_accepted_commit_missing')
        self.remote = copy.deepcopy(original)
        self.write_local(request=dict(self.request, commit='b' * 40))
        self.assert_pending_unchanged('remote_accepted_commit_mismatch')
        self.write_local(request=self.request)
        for field in ('service', 'commit'):
            self.remote = copy.deepcopy(original)
            self.remote['result'][field] = 'wrong'
            self.assert_pending_unchanged('remote_success_receipt_mismatch')

    def test_missing_id_never_searches_collection_and_known_log_receipt_can_recover(self):
        self.write_local(result=dict(self.result, remote_job='', remote_url=self.origin + '/api/v1/jobs'))
        self.assert_pending_unchanged('remote_job_receipt_missing_no_automatic_search')
        self.assertEqual(self.calls, [])
        self.write_local(log='Remote-Auftrag ' + self.url + '\n')
        value = self.run_helper(apply=True)
        self.assertEqual(value['phase'], 'reconciled', value)
        self.assertEqual(value['remote_job'], REMOTE_ID)
        self.assertTrue(all(path != '/api/v1/jobs' for _, path in self.calls))

    def test_conflicting_url_ids_origins_or_logged_receipts_never_releases(self):
        self.write_local(result=dict(self.result, remote_job='3' * 32))
        self.assert_pending_unchanged('conflicting_remote_receipt_ids')
        self.write_local(result=self.result, log='Remote-Auftrag ' + self.origin + '/api/v1/jobs/' + '3' * 32)
        self.assert_pending_unchanged('conflicting_logged_remote_receipts')
        self.write_local(result=self.result, log='Remote-Auftrag http://127.0.0.1:1/api/v1/jobs/' + REMOTE_ID)
        self.assert_pending_unchanged('conflicting_logged_remote_receipts')

    def test_controller_must_be_stopped_and_every_job_idle_including_checks(self):
        before = self.path.read_bytes()
        value = self.run_helper(apply=True, stopped=lambda: False)
        self.assertEqual(value['reason'], 'controller_must_be_stopped', value)
        self.assertEqual(before, self.path.read_bytes())
        with closing(sqlite3.connect(self.path)) as db:
            db.execute('INSERT INTO jobs VALUES (?,?,?,?,?,?,?)',
                       ('4' * 32, 100, 200, 'running', '{"kind":"check","ref":"main"}', '', '{}'))
            db.commit()
        self.assert_pending_unchanged('controller_jobs_not_idle')

    def test_local_row_race_rolls_back_without_audit_or_flag_clear(self):
        count = 0
        def change(path):
            nonlocal count
            if path.startswith('/api/v1/jobs/'):
                count += 1
                if count == 2:
                    self.write_local(log='concurrent log change')
        self.before_response = change
        value = self.run_helper(apply=True)
        self.assertEqual(value['reason'], 'local_job_changed_retry_diagnosis', value)
        self.assertIs(json.loads(self.row()[6])['remote_uncertain'], True)
        with closing(sqlite3.connect(self.path)) as db:
            self.assertIsNone(db.execute("SELECT name FROM sqlite_master WHERE name='remote_job_reconciliations'").fetchone())

    def test_changed_second_remote_proof_or_controller_restart_never_writes(self):
        count = 0
        def change(path):
            nonlocal count
            if path.startswith('/api/v1/jobs/'):
                count += 1
                if count == 2:
                    self.remote['result']['live'] = True
        self.before_response = change
        self.assert_pending_unchanged('remote_proof_changed_retry_diagnosis')
        self.before_response = None
        checks = iter((True, False))
        before = self.path.read_bytes()
        value = self.run_helper(apply=True, stopped=lambda: next(checks))
        self.assertEqual(value['reason'], 'controller_must_be_stopped', value)
        self.assertEqual(self.path.read_bytes(), before)

    def test_pinned_full_ref_and_profile_must_match_without_current_repository(self):
        self.write_local(request=dict(self.request, ref='b' * 40))
        self.assert_pending_unchanged('remote_accepted_commit_mismatch')
        profile = dict(name='TBS-02', mcc=901, mnc=1510, issi=4010002, la=2, cc=2)
        self.write_local(request=dict(self.request, profile_snapshot=profile, profile='TBS-02'))
        self.remote['request']['profile'] = dict(profile, la=3)
        self.assert_pending_unchanged('remote_profile_mismatch')
        self.remote['request']['profile'] = profile
        value = self.run_helper(apply=True)
        self.assertEqual(value['phase'], 'reconciled', value)

    def test_effective_systemd_service_user_and_state_directory_resolve_exact_uid(self):
        properties = dict(LoadState='loaded', User='netcore-deploy', StateDirectory='netcore-deployment')
        with patch.object(HELPER, 'controller_properties', return_value=properties), patch.object(HELPER, 'user_uid', return_value=23456) as lookup:
            self.assertEqual(HELPER.controller_owner(Path('/var/lib/netcore-deployment')), 23456)
            lookup.assert_called_once_with('netcore-deploy')
            with self.assertRaisesRegex(HELPER.Pending, 'controller_user_or_state_directory_not_verified'):
                HELPER.controller_owner(Path('/var/lib/unrelated-controller'))

    @unittest.skipUnless(os.name == 'posix', 'POSIX UID and permission regression; native Linux CI exercises it')
    def test_posix_legitimate_service_owner_private_database_and_750_parent_are_allowed(self):
        os.chmod(self.path.parent, 0o750)
        uid = os.geteuid()
        self.assertEqual(HELPER.safe_file(self.path, private=True, trusted_uid=uid), self.path)
        value = self.run_helper(apply=True)
        self.assertEqual(value['phase'], 'reconciled', value)

    @unittest.skipUnless(os.name == 'posix', 'POSIX UID and permission regression; native Linux CI exercises it')
    def test_posix_foreign_owner_and_writable_parent_are_rejected(self):
        uid = os.geteuid()
        if uid == 0:
            os.chown(self.path, 23457, -1)
            try:
                with self.assertRaisesRegex(HELPER.Pending, 'controller_database_not_private_to_service_owner'):
                    HELPER.safe_file(self.path, private=True, trusted_uid=23456)
            finally:
                os.chown(self.path, uid, -1)
        else:
            with self.assertRaisesRegex(HELPER.Pending, 'controller_database_not_private_to_service_owner'):
                HELPER.safe_file(self.path, private=True, trusted_uid=uid + 1)
        for mode in (0o777, 0o1777):
            os.chmod(self.path.parent, mode)
            try:
                with self.assertRaisesRegex(HELPER.Pending, 'controller_database_parent_writable_by_others'):
                    HELPER.safe_file(self.path, private=True, trusted_uid=uid)
            finally:
                os.chmod(self.path.parent, 0o700)
        for mode in (0o640, 0o400, 0o700):
            os.chmod(self.path, mode)
            with self.assertRaisesRegex(HELPER.Pending, 'controller_database_not_private_to_service_owner'):
                HELPER.safe_file(self.path, private=True, trusted_uid=uid)


if __name__ == '__main__':
    unittest.main()
