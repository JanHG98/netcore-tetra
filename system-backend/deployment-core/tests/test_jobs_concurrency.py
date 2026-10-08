"""Regression coverage for job status reads during installer log writes."""
from concurrent.futures import ThreadPoolExecutor
from contextlib import closing
from pathlib import Path
import sqlite3
import sys
import tempfile
import threading
import time
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from jobs import Jobs


class JobsConcurrencyTests(unittest.TestCase):
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
