"""Regression coverage for job status reads during installer log writes."""
from concurrent.futures import ThreadPoolExecutor
from contextlib import closing
from pathlib import Path
import json
import sqlite3
import sys
import tempfile
import threading
import time
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from jobs import IMAGE_JOB_METADATA, Jobs


class JobsConcurrencyTests(unittest.TestCase):
    @staticmethod
    def image_request(key='a' * 32):
        return dict(build_id=key, profile={'name': 'TBS-02'}, base='fixture-base',
                    commit='b' * 40, hostname='tbs-02', desktop='kde', sip_setup='configured',
                    vpn_username='vpn-user-private-marker', vpn_password='vpn-password-private-marker',
                    vpn_config='<key>\nvpn-key-private-marker' + 'x' * 12000 + '\n</key>\n',
                    password_hash='os-hash-private-marker', wifi_password='wifi-password-private-marker',
                    ssh_key='ssh-key-private-marker', sip_fallback_config='sip-password-private-marker',
                    config='tbs-config-private-marker')

    def assert_private_payload_removed(self, jobs, request):
        markers = [value.split('x' * 32)[0] for name, value in request.items()
                   if name not in IMAGE_JOB_METADATA and isinstance(value, str)]
        for file in (jobs.path, Path(str(jobs.path) + '-wal')):
            if file.exists():
                raw = file.read_bytes()
                for marker in markers:
                    self.assertFalse(marker.encode() in raw, (file.name, marker))

    def test_completed_image_keeps_credentials_for_worker_then_scrubs_database(self):
        with tempfile.TemporaryDirectory() as state:
            started, release = threading.Event(), threading.Event()
            requests = []
            request = self.image_request()

            def execute(private_request, log):
                requests.append(private_request)
                started.set()
                if not release.wait(5):
                    raise RuntimeError('fixture release timed out')
                self.assertEqual(private_request, request)
                log('private image inputs consumed without logging them')
                return {'id': private_request['build_id'], 'ready': True}

            jobs = Jobs(state, execute)
            submitted = jobs.submit(request)
            self.assertTrue(started.wait(3))
            try:
                self.assertEqual(jobs.get(submitted['id'])['request'], request)
            finally:
                release.set()
            jobs.queue.join()
            completed = jobs.get(submitted['id'])
            self.assertEqual(completed['status'], 'succeeded')
            self.assertEqual(completed['request'], {name: request[name] for name in IMAGE_JOB_METADATA})
            self.assertEqual(requests, [request])
            self.assert_private_payload_removed(jobs, request)

    def test_failed_image_scrubs_credentials_and_preserves_nonsecret_error(self):
        with tempfile.TemporaryDirectory() as state:
            request = self.image_request()

            def execute(private_request, log):
                self.assertEqual(private_request['vpn_password'], request['vpn_password'])
                raise RuntimeError('fixture build failed')

            jobs = Jobs(state, execute)
            submitted = jobs.submit(request)
            jobs.queue.join()
            completed = jobs.get(submitted['id'])
            self.assertEqual(completed['status'], 'failed')
            self.assertEqual(completed['result'], {'error': 'fixture build failed'})
            self.assertNotIn('vpn_password', completed['request'])
            self.assert_private_payload_removed(jobs, request)

    def test_startup_scrubs_completed_and_interrupted_images_in_wal_database(self):
        with tempfile.TemporaryDirectory() as state:
            path = Path(state) / 'jobs.sqlite3'
            request = self.image_request()
            with closing(sqlite3.connect(path)) as anchor:
                self.assertEqual(anchor.execute('PRAGMA journal_mode=WAL').fetchone()[0], 'wal')
                anchor.execute('CREATE TABLE jobs (id TEXT PRIMARY KEY, created REAL, updated REAL, '
                               'status TEXT, request TEXT, log TEXT, result TEXT)')
                for index, status in enumerate(('succeeded', 'failed', 'interrupted', 'queued', 'running')):
                    anchor.execute('INSERT INTO jobs VALUES (?,?,?,?,?,?,?)',
                                   (str(index), 0, 0, status, json.dumps(request), '', '{}'))
                shared = {'service': 'tbs', 'build_id': 'not-an-image', 'vpn_password': 'shared-request-retained'}
                anchor.execute('INSERT INTO jobs VALUES (?,?,?,?,?,?,?)',
                               ('shared', 0, 0, 'failed', json.dumps(shared), '', '{}'))
                anchor.commit()
                # Old queue versions did not enable secure_delete. Growing and
                # shrinking a row can leave earlier private request copies in
                # freed database pages as well as in the WAL.
                anchor.execute('UPDATE jobs SET log=?', ('old build log ' * 2500,))
                anchor.commit()
                anchor.execute("UPDATE jobs SET log=''")
                anchor.commit()
                jobs = Jobs(state, lambda private_request, log: self.fail('interrupted job must not restart'))
                for index in range(5):
                    row = jobs.get(str(index))
                    self.assertNotIn(row['status'], ('queued', 'running'))
                    self.assertEqual(row['request'], {name: request[name] for name in IMAGE_JOB_METADATA})
                self.assertEqual(jobs.get('shared')['request'], shared)
                self.assert_private_payload_removed(jobs, request)
                self.assertTrue(jobs.queue.empty())

    def test_connections_close_after_commit_and_rollback(self):
        with tempfile.TemporaryDirectory() as state:
            jobs = Jobs(state, lambda request, log: {})
            with jobs.connect() as committed:
                committed.execute('CREATE TABLE probe (value TEXT)')
                committed.execute("INSERT INTO probe VALUES ('committed')")
            with self.assertRaises(sqlite3.ProgrammingError):
                committed.execute('SELECT * FROM probe')
            with self.assertRaisesRegex(RuntimeError, 'rollback'):
                with jobs.connect() as rolled_back:
                    rolled_back.execute("INSERT INTO probe VALUES ('not committed')")
                    raise RuntimeError('rollback')
            with self.assertRaises(sqlite3.ProgrammingError):
                rolled_back.execute('SELECT * FROM probe')
            with jobs.connect() as db:
                self.assertEqual(db.execute('SELECT * FROM probe').fetchall(), [('committed',)])

    def test_noisy_job_with_concurrent_status_reads_executes_once(self):
        with tempfile.TemporaryDirectory() as state:
            release = threading.Event()
            calls = []

            def execute(request, log):
                calls.append(request)
                if not release.wait(5):
                    raise RuntimeError('reader setup timed out')
                for index in range(150):
                    log(f'{index}: ' + 'x' * 1000)
                return {'ready': True, 'commit': 'a' * 40}

            jobs = Jobs(state, execute)
            submitted = jobs.submit({'service': 'hardware-gateway'})
            key = submitted['id']

            def read_status():
                deadline = time.monotonic() + 10
                while time.monotonic() < deadline:
                    current = jobs.get(key)
                    listed = jobs.list()
                    self.assertEqual(listed[0]['id'], key)
                    if current['status'] == 'succeeded':
                        return current
                    if current['status'] == 'failed':
                        self.fail(current['result'])
                self.fail('job status timed out')

            with ThreadPoolExecutor(max_workers=4) as readers:
                futures = [readers.submit(read_status) for _ in range(4)]
                release.set()
                for future in futures:
                    result = future.result(timeout=12)
                    self.assertTrue(result['result']['ready'])
                    self.assertEqual(result['result']['commit'], 'a' * 40)
                    self.assertLessEqual(len(result['log']), 100000)
                    self.assertIn('149: ', result['log'])
            jobs.queue.join()
            self.assertEqual(len(calls), 1)
            with closing(sqlite3.connect(jobs.path)) as db:
                self.assertEqual(db.execute('SELECT status FROM jobs WHERE id=?', (key,)).fetchone(),
                                 ('succeeded',))


if __name__ == '__main__':
    unittest.main()

