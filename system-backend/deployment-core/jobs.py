"""Durable, serialized jobs. Interrupted jobs never restart themselves."""
from contextlib import contextmanager
import json
import os
from pathlib import Path
import queue
import sqlite3
import threading
import time
import uuid


class RemoteJobUncertain(RuntimeError):
    """The agent may still be working; never turn a lost reply into a retry POST."""
    def __init__(self, message, remote_url, remote_job=''):
        super().__init__(message)
        self.result = dict(error=message, remote_uncertain=True,
                           remote_url=remote_url, remote_job=remote_job)


class Jobs:
    def __init__(self, state, execute):
        self.path = Path(state) / 'jobs.sqlite3'
        self.path.parent.mkdir(parents=True, exist_ok=True)
        self.execute = execute
        self.lock = threading.Lock()
        self.db_lock = threading.Lock()
        self.queue = queue.Queue(maxsize=64)
        with self.connect() as db:
            db.execute('CREATE TABLE IF NOT EXISTS jobs (id TEXT PRIMARY KEY, created REAL, updated REAL, '
                       'status TEXT, request TEXT, log TEXT, result TEXT)')
            db.execute("UPDATE jobs SET status='interrupted', updated=? WHERE status IN ('queued','running')", (time.time(),))
        os.chmod(self.path, 0o600)
        threading.Thread(target=self._work, daemon=True).start()

    @contextmanager
    def connect(self):
        # Serialize only short database operations, never installers or polling.
        # Connection.__exit__ commits/rolls back but does not close the handle.
        with self.db_lock:
            db = sqlite3.connect(self.path, timeout=10)
            try:
                with db:
                    yield db
            finally:
                db.close()

    def submit(self, request):
        with self.lock:
            if self.queue.full():
                raise ValueError('Auftragswarteschlange ist voll')
            key = uuid.uuid4().hex
            now = time.time()
            with self.connect() as db:
                db.execute('INSERT INTO jobs VALUES (?,?,?,?,?,?,?)',
                           (key, now, now, 'queued', json.dumps(request), '', '{}'))
            self.queue.put_nowait(key)
            return self.get(key)

    def get(self, key):
        with self.connect() as db:
            row = db.execute('SELECT * FROM jobs WHERE id=?', (key,)).fetchone()
        if not row:
            raise KeyError(key)
        return self._decode(row)

    @staticmethod
    def _decode(row):
        return dict(id=row[0], created=row[1], updated=row[2], status=row[3],
                    request=json.loads(row[4]), log=row[5], result=json.loads(row[6]))

    def list(self):
        with self.connect() as db:
            rows = db.execute('SELECT * FROM jobs ORDER BY created DESC LIMIT 100').fetchall()
        return [self._decode(row) for row in rows]

    def log(self, key, line):
        with self.connect() as db:
            db.execute('UPDATE jobs SET log=substr(log || ?, -100000), updated=? WHERE id=?',
                       (str(line) + '\n', time.time(), key))

    def _work(self):
        while True:
            key = self.queue.get()
            with self.connect() as db:
                db.execute("UPDATE jobs SET status='running', updated=? WHERE id=?", (time.time(), key))
            try:
                result = self.execute(self.get(key)['request'], lambda line: self.log(key, line))
                status = 'succeeded'
            except Exception as exc:
                self.log(key, f'{type(exc).__name__}: {exc}')
                result = exc.result if isinstance(exc, RemoteJobUncertain) else {'error': str(exc)}
                status = 'failed'
            with self.connect() as db:
                db.execute('UPDATE jobs SET status=?, result=?, updated=? WHERE id=?',
                           (status, json.dumps(result or {}), time.time(), key))
            self.queue.task_done()
