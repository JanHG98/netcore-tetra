"""Real SQLite/journal fixtures; the worker and ARM64 image build are isolated."""
import contextlib
import copy
import hashlib
import importlib.util
import io
import json
import lzma
from pathlib import Path
import sqlite3
import stat
import sys
import tempfile
import unittest
from unittest import mock

REPO = Path(__file__).resolve().parents[2]
DRIVER = REPO / 'Docs/integration/Z01-2026-10-07/vm119-cached-image-recovery.py'
RUNTIME = REPO / 'system-backend/deployment-core'
sys.path.insert(0, str(RUNTIME))
from image_spec import BASE, NETWORKS, public_job, validate_image

COMMIT = '1595259a2a76abfc9eff08842409156473b585a7'
RECIPE = '7699808215f76322a6ed0cceddf5804d24f707e088ceabd7bdae5e7ef07b6eb6'
SOURCE_JOB = 'f141c06d4b2f48699486e149c4887b16'
SOURCE_BUILD = '2e2603beacd942dfaddc7119b85a4af0'
CHILD_JOB = '1' * 32
CHILD_BUILD = '2' * 32


def load_driver():
    spec = importlib.util.spec_from_file_location('vm119_cached_image_recovery', DRIVER)
    driver = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(driver)
    return driver


class CachedRecoveryTests(unittest.TestCase):
    def setUp(self):
        self.driver = load_driver()
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.state = self.root / 'builder'
        self.library = self.root / 'installed'
        self.library.mkdir()
        for directory in ('cache', 'artifacts', 'requests', 'work'):
            (self.state / directory).mkdir(parents=True, exist_ok=True)
        self.driver.STATE = self.state
        self.driver.LIBRARY = self.library
        self.driver.CONFIG = self.root / 'deployment.toml'
        self.driver.CONFIG.write_text('node_id = "VM-H-DEPLOY-01"\nrole = "controller"\n')
        self.driver.RECEIPT = self.state / ('recovery-' + SOURCE_BUILD + '.json')
        self.database = self.state / 'jobs.sqlite3'
        with sqlite3.connect(self.database) as db:
            db.execute('CREATE TABLE jobs (id TEXT PRIMARY KEY, created REAL, updated REAL, '
                       'status TEXT, request TEXT, log TEXT, result TEXT)')
        self.request = validate_image({
            'profile': {'name': 'SRV-M-TBS-03', 'mcc': 901, 'mnc': 1510,
                        'issi': 4010003, 'la': 3, 'cc': 1},
            'commit': COMMIT, 'hostname': 'srv-m-tbs-03',
            'config': '# SITE-CONFIG-SECRET\n[net_info]\n[cell_info]\n[phy_io]\n',
            'username': 'jan', 'password_hash': '$6$fixture$' + 'A' * 86,
            'wifi_ssid': 'test-lab', 'wifi_password': 'WIFI-SECRET-NEVER-PRINT',
            'vpn_config': 'client\ndev tun\nremote vpn.example.org 1194\n'
                          '<auth-user-pass>\nVPN-SECRET-NEVER-PRINT\n'
                          'VPN-PASSWORD-NEVER-PRINT\n</auth-user-pass>\n',
            'controller_url': 'http://10.0.1.131:8320',
        }, NETWORKS)
        self.request['build_id'] = SOURCE_BUILD
        self.original_result = {
            'id': SOURCE_BUILD,
            'filename': 'netcore-srv-m-tbs-03-1595259a2a76-2e2603be.img.xz',
            'commit': COMMIT, 'recipe': RECIPE, 'profile': self.request['profile'],
            'hostname': self.request['hostname'], 'created': 1791500000.0,
            'size_bytes': 2043954184,
            'sha256': '4a6bc6549cace7559a3dfc9e1f8f6d6d0302e84df3128b63d31c4e6ac17d6c3b',
            'base': BASE, 'boot_tested': False,
        }
        self.insert(SOURCE_JOB, 'succeeded', self.request, self.original_result,
                    'Fertig: ' + self.original_result['filename'])
        self.cache_image = self.state / 'cache' / (RECIPE + '.img')
        self.cache_image.write_bytes(b'unpersonalized-software-fixture')
        self.driver.CACHE_BYTES = self.cache_image.stat().st_size
        self.cache_versions = self.cache_image.with_suffix('.json')
        self.cache_versions.write_text(json.dumps({'commit': COMMIT}))
        self.calls = []
        self.behavior = 'queued'
        self.after_receipt = None
        outer = self

        class Client:
            def status(self):
                return self.request('/status')

            def request(self, path, data=None):
                outer.calls.append((path, copy.deepcopy(data)))
                if path == '/status':
                    return {'available': True, 'error': None, 'jobs': [], 'artifacts': [],
                            'active_profiles': [], 'free_bytes': 80 * 1024 ** 3}
                outer.assertEqual(path, '/build')
                outer.assertIsInstance(data, dict)
                receipt = json.loads(outer.driver.RECEIPT.read_text())
                outer.assertEqual(receipt['phase'], 'submit_started')
                outer.assertEqual(stat.S_IMODE(outer.driver.RECEIPT.stat().st_mode), 0o600)
                outer.assertEqual(outer.rows()[SOURCE_JOB]['request'], outer.request)
                if outer.after_receipt:
                    outer.after_receipt(receipt)
                if outer.behavior == 'lost_before_acceptance':
                    raise TimeoutError('isolated POST reply loss')
                child_request = copy.deepcopy(data)
                child_request['build_id'] = CHILD_BUILD
                child_status = 'failed' if outer.behavior == 'failed' else 'queued'
                outer.insert(CHILD_JOB, child_status, child_request,
                             {'error': 'fixture failed'} if child_status == 'failed' else {})
                if outer.behavior == 'lost_after_acceptance':
                    raise TimeoutError('isolated POST reply loss')
                return public_job(outer.rows()[CHILD_JOB])

        self.client = Client()

    def insert(self, key, status, request=None, result=None, log='', created=1791500000.0):
        with sqlite3.connect(self.database) as db:
            db.execute('INSERT INTO jobs VALUES (?,?,?,?,?,?,?)',
                       (key, created, created, status, json.dumps(request or {}), log, json.dumps(result or {})))

    def rows(self):
        with sqlite3.connect(self.database) as db:
            rows = db.execute('SELECT * FROM jobs').fetchall()
        return {row[0]: {'id': row[0], 'created': row[1], 'updated': row[2],
                         'status': row[3], 'request': json.loads(row[4]),
                         'log': row[5], 'result': json.loads(row[6])} for row in rows}

    def run_recovery(self, recipe=RECIPE, validator=validate_image):
        output, error = io.StringIO(), io.StringIO()
        try:
            with contextlib.redirect_stdout(output), contextlib.redirect_stderr(error):
                result = self.driver.recover(self.client, lambda commit: recipe,
                                             validator, NETWORKS)
        except self.driver.RecoveryError as exc:
            self.assert_private(str(exc))
            raise
        finally:
            self.assert_private(output.getvalue() + error.getvalue())
            if self.driver.RECEIPT.exists():
                self.assert_private(self.driver.RECEIPT.read_text())
        self.assert_private(json.dumps(result))
        return result

    def assert_private(self, value):
        for secret in ('SITE-CONFIG-SECRET', 'WIFI-SECRET-NEVER-PRINT',
                       'VPN-SECRET-NEVER-PRINT', 'VPN-PASSWORD-NEVER-PRINT',
                       self.request['password_hash']):
            self.assertNotIn(secret, value)

    def build_posts(self):
        return [data for path, data in self.calls if path == '/build']

    def finish_child(self):
        payload = lzma.compress(b'fixture station image')
        directory = self.state / 'artifacts' / CHILD_BUILD
        directory.mkdir()
        checksum = hashlib.sha256(payload).hexdigest()
        manifest = {**self.original_result, 'id': CHILD_BUILD,
                    'filename': 'netcore-srv-m-tbs-03-1595259a2a76-22222222.img.xz',
                    'size_bytes': len(payload), 'sha256': checksum}
        (directory / 'image.img.xz').write_bytes(payload)
        (directory / 'manifest.json').write_text(json.dumps(manifest))
        (directory / 'image.sha256').write_text(checksum + '  ' + manifest['filename'] + '\n')
        with sqlite3.connect(self.database) as db:
            db.execute('UPDATE jobs SET status=?,result=? WHERE id=?',
                       ('succeeded', json.dumps(manifest), CHILD_JOB))
        return directory, manifest

    def expect_guard(self):
        with self.assertRaises(self.driver.RecoveryError):
            self.run_recovery()
        self.assertEqual(self.build_posts(), [])
        self.assertFalse(self.driver.RECEIPT.exists())

    def test_exact_cached_request_is_submitted_once_after_durable_private_receipt(self):
        original = copy.deepcopy(self.rows()[SOURCE_JOB])
        self.run_recovery()
        self.assertEqual(len(self.build_posts()), 1)
        self.assertEqual(self.build_posts()[0]['commit'], COMMIT)
        self.assertEqual(self.build_posts()[0]['profile'], self.request['profile'])
        self.assertEqual(self.build_posts()[0]['wifi_password'], self.request['wifi_password'])
        self.assertEqual(self.build_posts()[0]['vpn_config'], self.request['vpn_config'])
        self.assertEqual(self.rows()[SOURCE_JOB], original)
        self.run_recovery()
        self.assertEqual(len(self.build_posts()), 1)

    def test_hidden_queued_running_and_remote_uncertain_rows_stop_before_post(self):
        with sqlite3.connect(self.database) as db:
            for number in range(150):
                db.execute('INSERT INTO jobs VALUES (?,?,?,?,?,?,?)',
                           (format(number + 1000, '032x'), 2000000000 + number,
                            2000000000 + number, 'succeeded', '{}', '', '{}'))
        for state in ('queued', 'running', 'uncertain'):
            with self.subTest(state=state):
                key = state.ljust(32, '0')
                self.insert(key, 'failed' if state == 'uncertain' else state,
                            {'profile': {'name': 'some-other-profile'}},
                            {'remote_uncertain': True} if state == 'uncertain' else {},
                            created=1)
                self.expect_guard()
                with sqlite3.connect(self.database) as db:
                    db.execute('DELETE FROM jobs WHERE id=?', (key,))

    def test_missing_cache_or_wrong_versions_never_falls_back_to_full_build(self):
        image_bytes = self.cache_image.read_bytes()
        self.cache_image.unlink()
        self.expect_guard()
        self.cache_image.write_bytes(image_bytes)
        self.cache_versions.unlink()
        self.expect_guard()
        self.cache_versions.write_text(json.dumps({'commit': 'f' * 40}))
        self.expect_guard()

    def test_recipe_mismatch_refuses_submission(self):
        with self.assertRaises(self.driver.RecoveryError):
            self.run_recovery(recipe='f' * 64)
        self.assertEqual(self.build_posts(), [])
        self.assertFalse(self.driver.RECEIPT.exists())

    def test_existing_old_artifact_refuses_duplicate_recovery_build(self):
        directory = self.state / 'artifacts' / SOURCE_BUILD
        directory.mkdir()
        (directory / 'image.img.xz').write_bytes(b'already-present')
        (directory / 'manifest.json').write_text(json.dumps(self.original_result))
        self.expect_guard()

    def test_lost_post_reply_adopts_one_existing_new_row_without_resubmission(self):
        self.behavior = 'lost_after_acceptance'
        try:
            self.run_recovery()
        except self.driver.RecoveryError:
            pass
        self.assertEqual(len(self.build_posts()), 1)
        self.assertIn(CHILD_JOB, self.rows())
        self.run_recovery()
        self.assertEqual(len(self.build_posts()), 1)

    def test_lost_reply_without_new_job_remains_uncertain_without_retry(self):
        self.behavior = 'lost_before_acceptance'
        with self.assertRaises(self.driver.RecoveryError):
            self.run_recovery()
        self.assertTrue(self.driver.RECEIPT.exists())
        with self.assertRaises(self.driver.RecoveryError):
            self.run_recovery()
        self.assertEqual(len(self.build_posts()), 1)

    def test_multiple_matching_added_jobs_remain_ambiguous_without_retry(self):
        self.behavior = 'lost_before_acceptance'
        with self.assertRaises(self.driver.RecoveryError):
            self.run_recovery()
        for key, build_id in (('3' * 32, '4' * 32), ('5' * 32, '6' * 32)):
            request = copy.deepcopy(self.request)
            request['build_id'] = build_id
            self.insert(key, 'queued', request)
        with self.assertRaises(self.driver.RecoveryError):
            self.run_recovery()
        self.assertEqual(len(self.build_posts()), 1)

    def test_failed_recovery_is_preserved_without_new_post(self):
        self.behavior = 'failed'
        try:
            self.run_recovery()
        except self.driver.RecoveryError:
            pass
        try:
            self.run_recovery()
        except self.driver.RecoveryError:
            pass
        self.assertEqual(len(self.build_posts()), 1)
        self.assertTrue(self.driver.RECEIPT.exists())

    def test_new_candidate_must_match_private_credentials_not_only_public_profile(self):
        self.behavior = 'lost_before_acceptance'
        with self.assertRaises(self.driver.RecoveryError):
            self.run_recovery()
        other_request = copy.deepcopy(self.request)
        other_request['build_id'] = CHILD_BUILD
        other_request['wifi_password'] = 'different-valid-wifi-password'
        self.insert(CHILD_JOB, 'queued', other_request)
        with self.assertRaises(self.driver.RecoveryError):
            self.run_recovery()
        self.assertEqual(len(self.build_posts()), 1)

    def test_finished_recovery_has_consistent_downloads_and_never_needs_original_cache_again(self):
        self.run_recovery()
        directory, manifest = self.finish_child()
        self.cache_image.unlink()
        self.cache_versions.unlink()
        result = self.run_recovery(recipe='f' * 64)
        self.assertEqual(result['status'], 'succeeded')
        self.assertEqual(result['job_id'], CHILD_JOB)
        self.assertEqual(result['build_id'], CHILD_BUILD)
        self.assertEqual(result['sha256'], manifest['sha256'])
        self.assertTrue(result['file_sha256_not_recomputed'])
        for kind in ('image', 'sha256', 'manifest'):
            self.assertTrue(result[kind + '_url'].endswith('/' + CHILD_BUILD + '/' + kind))
        self.assertEqual(len(self.build_posts()), 1)
        self.assertTrue((directory / 'image.img.xz').exists())

    def test_incomplete_or_inconsistent_success_artifact_never_triggers_another_post(self):
        self.run_recovery()
        directory, manifest = self.finish_child()
        mutations = ('missing_checksum', 'wrong_checksum', 'wrong_size', 'wrong_commit', 'wrong_id')
        for kind in mutations:
            with self.subTest(kind=kind):
                candidate = dict(manifest)
                checksum = directory / 'image.sha256'
                checksum.write_text(manifest['sha256'] + '  ' + manifest['filename'] + '\n')
                if kind == 'missing_checksum':
                    checksum.unlink()
                elif kind == 'wrong_checksum':
                    checksum.write_text('f' * 64 + '\n')
                elif kind == 'wrong_size':
                    candidate['size_bytes'] += 1
                elif kind == 'wrong_commit':
                    candidate['commit'] = 'f' * 40
                elif kind == 'wrong_id':
                    candidate['id'] = 'f' * 32
                (directory / 'manifest.json').write_text(json.dumps(candidate))
                with self.assertRaises(self.driver.RecoveryError):
                    self.run_recovery()
                self.assertEqual(len(self.build_posts()), 1)

    def test_journal_file_fsync_failure_prevents_the_only_post(self):
        with mock.patch.object(self.driver.os, 'fsync', side_effect=OSError('isolated fsync failure')):
            with self.assertRaises(OSError):
                self.run_recovery()
        self.assertEqual(self.build_posts(), [])
        self.assertFalse(self.driver.RECEIPT.exists())
        self.assertEqual(list(self.state.glob('.cached-recovery-*')), [])

    def test_journal_directory_fsync_failure_leaves_pending_receipt_and_forbids_retry(self):
        with mock.patch.object(self.driver.os, 'fsync', side_effect=[None, OSError('isolated directory fsync failure')]):
            with self.assertRaises(OSError):
                self.run_recovery()
        self.assertEqual(self.build_posts(), [])
        self.assertEqual(json.loads(self.driver.RECEIPT.read_text())['phase'], 'submit_started')
        with self.assertRaises(self.driver.RecoveryError):
            self.run_recovery()
        self.assertEqual(self.build_posts(), [])

    def test_dangling_receipt_symlink_is_preserved_and_never_overwritten_by_a_post_attempt(self):
        self.driver.RECEIPT.symlink_to(self.root / 'missing-receipt-target')
        self.expect_guard()
        self.assertTrue(self.driver.RECEIPT.is_symlink())
        self.assertFalse((self.root / 'missing-receipt-target').exists())

    def test_unknown_or_malformed_persisted_status_fails_closed_before_post(self):
        for status, uncertain in (('unknown', False), ('failed', 'false'),
                                  ('failed', 1), ('failed', None)):
            with self.subTest(status=status, uncertain=uncertain):
                self.insert('7' * 32, status, {}, {'remote_uncertain': uncertain})
                self.expect_guard()
                with sqlite3.connect(self.database) as db:
                    db.execute('DELETE FROM jobs WHERE id=?', ('7' * 32,))

    def test_abandoned_work_and_its_contents_are_preserved_without_worker_cleanup(self):
        directory = self.state / 'work' / ('7' * 32)
        directory.mkdir()
        image = directory / 'image.img'
        image.write_bytes(b'existing abandoned work; preserve for diagnosis')
        self.expect_guard()
        self.assertEqual(image.read_bytes(), b'existing abandoned work; preserve for diagnosis')

    def test_validation_that_changes_original_private_values_cannot_submit(self):
        def altered_validation(request, networks):
            result = validate_image(request, networks)
            result['wifi_password'] = 'different-valid-wifi-password'
            return result

        with self.assertRaises(self.driver.RecoveryError):
            self.run_recovery(validator=altered_validation)
        self.assertEqual(self.build_posts(), [])
        self.assertFalse(self.driver.RECEIPT.exists())
        self.assertEqual(self.rows()[SOURCE_JOB]['request'], self.request)

    def test_job_appearing_during_validation_prevents_recording_or_submitting_a_recovery(self):
        def concurrent_validation(request, networks):
            self.insert('7' * 32, 'queued', {'commit': 'f' * 40})
            return validate_image(request, networks)

        with self.assertRaises(self.driver.RecoveryError):
            self.run_recovery(validator=concurrent_validation)
        self.assertEqual(self.build_posts(), [])
        self.assertFalse(self.driver.RECEIPT.exists())
        self.assertEqual(self.rows()['7' * 32]['status'], 'queued')

    def test_receipt_write_failure_after_worker_acceptance_is_reconciled_without_another_post(self):
        with mock.patch.object(self.driver.os, 'fsync',
                               side_effect=[None, None, OSError('isolated accepted-receipt fsync failure')]):
            with self.assertRaises(OSError):
                self.run_recovery()
        self.assertEqual(len(self.build_posts()), 1)
        self.assertEqual(self.rows()[CHILD_JOB]['status'], 'queued')
        self.assertEqual(json.loads(self.driver.RECEIPT.read_text())['phase'], 'submit_started')
        result = self.run_recovery()
        self.assertEqual(result['job_id'], CHILD_JOB)
        self.assertEqual(len(self.build_posts()), 1)

    def test_wrong_host_is_rejected_before_systemd_sqlite_or_receipt_operations(self):
        before = self.database.read_bytes()
        with mock.patch.object(self.driver.sys, 'argv', [str(DRIVER)]), \
                mock.patch.object(self.driver.os, 'geteuid', return_value=0), \
                mock.patch.object(self.driver.socket, 'gethostname', return_value='another-vm'), \
                mock.patch.object(self.driver.subprocess, 'run') as command:
            with self.assertRaises(self.driver.RecoveryError):
                self.driver.main()
        command.assert_not_called()
        self.assertEqual(self.database.read_bytes(), before)
        self.assertFalse(self.driver.RECEIPT.exists())

    def test_missing_jobs_table_reports_sqlite_select_failure_without_a_post(self):
        with sqlite3.connect(self.database) as db:
            db.execute('ALTER TABLE jobs RENAME TO preserved_jobs')
        before = self.database.read_bytes()
        with self.assertRaises(self.driver.RecoveryError) as raised:
            self.run_recovery()
        self.assertIn('SELECT_JOBS', str(raised.exception))
        self.assertIn('SQLITE_ERROR', str(raised.exception))
        self.assertIn('no such table: jobs', str(raised.exception))
        self.assertEqual(self.build_posts(), [])
        self.assertFalse(self.driver.RECEIPT.exists())
        self.assertEqual(self.database.read_bytes(), before)

    def test_missing_jobs_column_reports_sqlite_select_failure_without_a_post(self):
        with sqlite3.connect(self.database) as db:
            db.execute('ALTER TABLE jobs RENAME TO preserved_jobs')
            db.execute('CREATE TABLE jobs (id TEXT,status TEXT,result TEXT)')
        before = self.database.read_bytes()
        with self.assertRaises(self.driver.RecoveryError) as raised:
            self.run_recovery()
        self.assertIn('SELECT_JOBS', str(raised.exception))
        self.assertIn('SQLITE_ERROR', str(raised.exception))
        self.assertIn('no such column: request', str(raised.exception))
        self.assertEqual(self.build_posts(), [])
        self.assertFalse(self.driver.RECEIPT.exists())
        self.assertEqual(self.database.read_bytes(), before)

    def actual_open_error(self):
        missing = self.root / 'not-created.sqlite3'
        try:
            sqlite3.connect(missing.as_uri() + '?mode=ro', uri=True)
        except sqlite3.Error as error:
            return error
        self.fail('Opening a nonexistent read-only database should fail.')

    def test_readonly_open_error_reports_native_sqlite_error_without_a_post(self):
        error = self.actual_open_error()
        with mock.patch.object(self.driver.sqlite3, 'connect', side_effect=error):
            with self.assertRaises(self.driver.RecoveryError) as raised:
                self.run_recovery()
        self.assertIn('OPEN_READONLY', str(raised.exception))
        self.assertIn('SQLITE_CANTOPEN', str(raised.exception))
        self.assertIn('unable to open database file', str(raised.exception))
        self.assertEqual(self.build_posts(), [])
        self.assertFalse(self.driver.RECEIPT.exists())

    def test_actual_sqlite_exclusive_lock_reports_busy_without_retrying_or_submitting(self):
        real_connect = sqlite3.connect

        def short_readonly_connect(*args, **kwargs):
            self.assertTrue(args[0].endswith('?mode=ro'))
            self.assertTrue(kwargs.get('uri'))
            kwargs['timeout'] = 0.02
            return real_connect(*args, **kwargs)

        with contextlib.closing(real_connect(self.database)) as writer:
            writer.execute('BEGIN EXCLUSIVE')
            with mock.patch.object(self.driver.sqlite3, 'connect', side_effect=short_readonly_connect):
                with self.assertRaises(self.driver.RecoveryError) as raised:
                    self.run_recovery()
            self.assertIn('SELECT_JOBS', str(raised.exception))
            self.assertIn('SQLITE_BUSY', str(raised.exception))
            self.assertIn('database is locked', str(raised.exception))
        self.assertEqual(self.build_posts(), [])
        self.assertFalse(self.driver.RECEIPT.exists())

    def test_database_read_failure_after_post_acceptance_preserves_receipt_and_resumes_existing_job(self):
        real_connect = sqlite3.connect
        error = self.actual_open_error()
        readonly_calls = 0

        def fail_accepted_resume(*args, **kwargs):
            nonlocal readonly_calls
            if isinstance(args[0], str) and args[0].endswith('?mode=ro'):
                readonly_calls += 1
                if readonly_calls == 3:
                    raise error
            return real_connect(*args, **kwargs)

        with mock.patch.object(self.driver.sqlite3, 'connect', side_effect=fail_accepted_resume):
            with self.assertRaises(self.driver.RecoveryError) as raised:
                self.run_recovery()
        self.assertIn('OPEN_READONLY', str(raised.exception))
        self.assertEqual(len(self.build_posts()), 1)
        self.assertEqual(self.rows()[CHILD_JOB]['status'], 'queued')
        self.assertEqual(json.loads(self.driver.RECEIPT.read_text())['phase'], 'submit_started')
        result = self.run_recovery()
        self.assertEqual(result['job_id'], CHILD_JOB)
        self.assertEqual(len(self.build_posts()), 1)


if __name__ == '__main__':
    unittest.main()
