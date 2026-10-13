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


IMAGE_JOB_METADATA = ('build_id', 'profile', 'base', 'commit', 'hostname', 'desktop', 'sip_setup')
TERMINAL_STATUSES = ('succeeded', 'failed', 'interrupted')


def completed_image_request(request):
    """Image history needs metadata, never finished OS/WLAN/SIP/VPN credentials."""
    if not isinstance(request, dict):
        return request
    key = request.get('build_id')
    if not isinstance(key, str) or len(key) != 32 or any(c not in '0123456789abcdef' for c in key):
        return request
    return {name: request[name] for name in IMAGE_JOB_METADATA if name in request}


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
            scrubbed = self._scrub_finished_images(db)
        if scrubbed:
            self._checkpoint_image_cleanup(reclaim_legacy=True)
        os.chmod(self.path, 0o600)
        threading.Thread(target=self._work, daemon=True).start()

    @contextmanager
    def connect(self):
        # Serialize only short database operations, never installers or polling.
        # Connection.__exit__ commits/rolls back but does not close the handle.
        with self.db_lock:
            db = sqlite3.connect(self.path, timeout=10)
            try:
                db.execute('PRAGMA secure_delete=ON')
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

    def active_profile_names(self):
        """All persisted references, including uncertain remote jobs beyond UI limits."""
        names = set()
        with self.connect() as db:
            rows = db.execute('SELECT status, request, result FROM jobs')
            for status, request_json, result_json in rows:
                request, result = json.loads(request_json), json.loads(result_json)
                if not isinstance(request, dict) or not isinstance(result, dict):
                    raise ValueError('Ungültige persistierte Auftragsdaten; Profilbelegung unklar')
                uncertain = result.get('remote_uncertain', False)
                if not isinstance(uncertain, bool):
                    raise ValueError('Ungültiger Remote-Auftragsstatus; Profilbelegung unklar')
                if status not in ('queued', 'running') and not uncertain:
                    continue
                profile = request.get('profile')
                if isinstance(profile, dict):
                    profile = profile.get('name')
                    if not isinstance(profile, str) or not profile:
                        raise ValueError('Ungültige Profilreferenz; Profilbelegung unklar')
                if profile:
                    if not isinstance(profile, str):
                        raise ValueError('Ungültige Profilreferenz; Profilbelegung unklar')
                    names.add(profile)
        return sorted(names)

    def log(self, key, line):
        with self.connect() as db:
            db.execute('UPDATE jobs SET log=substr(log || ?, -100000), updated=? WHERE id=?',
                       (str(line) + '\n', time.time(), key))

    @staticmethod
    def _scrub_finished_images(db, key=None):
        scrubbed = False
        query = 'SELECT id, request FROM jobs WHERE status IN (?,?,?)'
        params = TERMINAL_STATUSES
        if key is not None:
            query += ' AND id=?'
            params += (key,)
        for job_id, blob in db.execute(query, params).fetchall():
            request = json.loads(blob)
            metadata = completed_image_request(request)
            if metadata != request:
                # Overwrite deleted cells/overflow pages, rather than retaining
                # credential text in the database's free space.
                db.execute('PRAGMA secure_delete=ON')
                db.execute('UPDATE jobs SET request=? WHERE id=?', (json.dumps(metadata), job_id))
                scrubbed = True
        return scrubbed

    def _checkpoint_image_cleanup(self, reclaim_legacy=False):
        # DELETE journals are the default. If a previous installation enabled
        # WAL, remove its old request frames after the scrub transaction commits.
        with self.connect() as db:
            if reclaim_legacy:
                # Older workers could leave private copies in freed pages while
                # growing job logs. Rebuild once when upgrading such history.
                db.execute('VACUUM')
            if db.execute('PRAGMA journal_mode').fetchone()[0].lower() == 'wal':
                db.execute('PRAGMA wal_checkpoint(TRUNCATE)')

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
                scrubbed = self._scrub_finished_images(db, key)
            if scrubbed:
                self._checkpoint_image_cleanup()
            self.queue.task_done()

