"""Durable jobs and update batches. Agent/image queues remain serialized."""
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


class RemoteJobFailed(RuntimeError):
    """A remote terminal result was observed, unlike an ambiguous lost reply."""
    def __init__(self, message, remote_url, remote_job):
        super().__init__(message)
        self.result = dict(error=message, remote_uncertain=False,
                           remote_url=remote_url, remote_job=remote_job)


class JobConflict(ValueError):
    pass


class Jobs:
    def __init__(self, state, execute, *, workers=1, queue_size=64):
        self.path = Path(state) / 'jobs.sqlite3'
        self.path.parent.mkdir(parents=True, exist_ok=True)
        self.execute = execute
        self.lock = threading.Lock()
        self.db_lock = threading.Lock()
        self.queue = queue.Queue(maxsize=queue_size)
        with self.connect() as db:
            db.execute('CREATE TABLE IF NOT EXISTS jobs (id TEXT PRIMARY KEY, created REAL, updated REAL, '
                       'status TEXT, request TEXT, log TEXT, result TEXT)')
            db.execute('CREATE TABLE IF NOT EXISTS update_batches (id TEXT PRIMARY KEY, created REAL, status TEXT, request TEXT)')
            for key, request_json, result_json in db.execute("SELECT id,request,result FROM jobs WHERE status IN ('queued','running')").fetchall():
                request, result = json.loads(request_json), json.loads(result_json)
                rejected = result.get('dispatch_state') == 'rejected' and result.get('remote_uncertain') is False
                if result.get('remote_url') and not rejected:
                    result.update(remote_uncertain=True, error='Controller wurde unterbrochen; Remote-Auftrag separat prüfen. Keine automatische Wiederholung.')
                elif rejected or request.get('kind') == 'bulk_service':
                    result.update(not_attempted=True, remote_uncertain=False,
                                  error='Controller wurde unterbrochen; kein Remote-Update angelegt.')
                db.execute("UPDATE jobs SET status='interrupted',result=?,updated=? WHERE id=?",
                           (json.dumps(result), time.time(), key))
            scrubbed = self._scrub_finished_images(db)
        if scrubbed:
            self._checkpoint_image_cleanup(reclaim_legacy=True)
        os.chmod(self.path, 0o600)
        for index in range(workers):
            threading.Thread(target=self._work, daemon=True, name=f'jobs-{index}').start()

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
                self._assert_available(db, request)
                db.execute('INSERT INTO jobs VALUES (?,?,?,?,?,?,?)',
                           (key, now, now, 'queued', json.dumps(request), '', '{}'))
            self.queue.put_nowait(key)
            return self.get(key)

    @staticmethod
    def _assert_available(db, request, exclude=''):
        node, service = request.get('node_id'), request.get('service')
        if not node or not service:
            return
        for key, status, request_json, result_json in db.execute('SELECT id,status,request,result FROM jobs'):
            if key == exclude:
                continue
            other, result = json.loads(request_json), json.loads(result_json)
            if other.get('node_id') != node:
                continue
            if result.get('remote_uncertain') is True and status not in ('queued', 'running'):
                raise JobConflict(f'Ungeklärter Remote-Auftrag auf {node}; zuerst den vorhandenen Auftrag prüfen.')
            if status in ('queued', 'running') and other.get('service') == service:
                raise JobConflict(f'Für {service} auf {node} läuft bereits ein Auftrag.')

    def assert_available(self, request, exclude=''):
        with self.connect() as db:
            self._assert_available(db, request, exclude)

    def node_busy(self, node):
        with self.connect() as db:
            for status, request, result in db.execute('SELECT status,request,result FROM jobs'):
                if json.loads(request).get('node_id') == node and (status in ('queued', 'running') or json.loads(result).get('remote_uncertain') is True):
                    return True
        return False

    def has_active_jobs(self):
        with self.connect() as db:
            return db.execute("SELECT 1 FROM jobs WHERE status IN ('queued','running') LIMIT 1").fetchone() is not None

    def plan_batch(self, request):
        key, now = uuid.uuid4().hex, time.time()
        with self.connect() as db:
            db.execute('INSERT INTO update_batches VALUES (?,?,?,?)', (key, now, 'planned', json.dumps(request)))
        return self.batch(key)

    def start_batch(self, key, *, expiry=300):
        with self.lock:
            with self.connect() as db:
                row = db.execute('SELECT created,status,request FROM update_batches WHERE id=?', (key,)).fetchone()
                if row is None:
                    raise KeyError(key)
                created, status, blob = row
                if status != 'planned':
                    queued = []
                else:
                    if time.time() - created > expiry:
                        raise JobConflict('Die Update-Vorschau ist abgelaufen; neue Vorschau erstellen.')
                    request = json.loads(blob)
                    targets = request['targets']
                    if not targets:
                        raise ValueError('Keine installierten verwalteten Dienste erreichbar.')
                    if len(targets) > self.queue.maxsize - self.queue.qsize():
                        raise JobConflict('Auftragswarteschlange ist voll; später erneut starten.')
                    for target in targets:
                        self._assert_available(db, target)
                    # Round-robin hosts prevents one multi-service host from
                    # occupying every controller poller before other hosts start.
                    hosts = {}
                    for target in targets:
                        hosts.setdefault(target['node_id'], []).append(target)
                    ordered = []
                    while any(hosts.values()):
                        for rows in hosts.values():
                            if rows:
                                ordered.append(rows.pop(0))
                    queued = []
                    for target in ordered:
                        child = uuid.uuid4().hex
                        target['job_id'] = child
                        data = dict(kind='bulk_service', batch_id=key, node_id=target['node_id'],
                                    service=target['service'], agent_url=target['agent_url'],
                                    commit=request['commit'], action='update', confirm_restart=True)
                        db.execute('INSERT INTO jobs VALUES (?,?,?,?,?,?,?)',
                                   (child, time.time(), time.time(), 'queued', json.dumps(data), '', '{}'))
                        queued.append(child)
                    db.execute("UPDATE update_batches SET status='started',request=? WHERE id=?", (json.dumps(request), key))
            # All children and the consumed plan commit before any dispatch.
            for child in queued:
                self.queue.put_nowait(child)
        return self.batch(key)

    def batch(self, key):
        with self.connect() as db:
            row = db.execute('SELECT created,status,request FROM update_batches WHERE id=?', (key,)).fetchone()
            if row is None:
                raise KeyError(key)
            created, state, request = row
            request = json.loads(request)
            keys = [target['job_id'] for target in request['targets'] if target.get('job_id')]
            children = [self._decode(db.execute("SELECT id,created,updated,status,request,'' AS log,result FROM jobs WHERE id=?", (child,)).fetchone()) for child in keys]
        counts = {status: sum(job['status'] == status for job in children)
                  for status in ('queued', 'running', 'succeeded', 'failed', 'interrupted')}
        counts.update(total=len(children), targets=len(request['targets']), hosts=len({target['node_id'] for target in request['targets']}),
                      skipped=len(request.get('skipped', [])),
                      uncertain=sum(job['status'] not in ('queued', 'running') and job['result'].get('remote_uncertain') is True for job in children),
                      not_attempted=sum(job['result'].get('not_attempted') is True for job in children))
        status = ('planned' if state == 'planned' else 'running' if counts['running'] else 'queued' if counts['queued']
                  else 'succeeded' if counts['succeeded'] == counts['total'] else 'partial' if counts['succeeded']
                  else 'interrupted' if counts['interrupted'] else 'failed')
        compact = []
        for job in children:
            result = {name: job['result'][name] for name in
                      ('error', 'remote_job', 'remote_url', 'remote_uncertain', 'commit', 'service', 'ready', 'restarted', 'not_attempted', 'live', 'dispatch_state')
                      if name in job['result']}
            for name, value in result.items():
                if isinstance(value, str):
                    result[name] = value[:2000 if name == 'error' else 256]
            compact.append(dict(id=job['id'], created=job['created'], updated=job['updated'], status=job['status'],
                                request={name: job['request'][name] for name in ('node_id', 'service', 'commit', 'action', 'batch_id') if name in job['request']},
                                result=result))
        return dict(request, id=key, plan_id=key, created=created, status=status, counts=counts, jobs=compact)

    def batches(self):
        with self.connect() as db:
            keys = [row[0] for row in db.execute("SELECT id FROM update_batches WHERE status!='planned' ORDER BY created DESC LIMIT 30")]
        return [self.batch(key) for key in keys]

    def remote_progress(self, key, remote_url, remote_job='', *, uncertain=True, state='posted'):
        with self.connect() as db:
            row = db.execute('SELECT result FROM jobs WHERE id=?', (key,)).fetchone()
            result = json.loads(row[0])
            result.update(remote_url=remote_url, remote_job=remote_job, remote_uncertain=uncertain, dispatch_state=state)
            db.execute('UPDATE jobs SET result=?,updated=? WHERE id=?', (json.dumps(result), time.time(), key))

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
                def log(line):
                    self.log(key, line)
                log.remote = lambda url, remote_job='', **kwargs: self.remote_progress(key, url, remote_job, **kwargs)
                log.job_id = key
                request = self.get(key)['request']
                result = self.execute(request, log)
                status = 'succeeded'
            except Exception as exc:
                self.log(key, f'{type(exc).__name__}: {exc}')
                result = dict(self.get(key)['result'])
                result.update(exc.result if isinstance(exc, (RemoteJobUncertain, RemoteJobFailed)) else {'error': str(exc)})
                if (isinstance(exc, JobConflict) or result.get('dispatch_state') == 'rejected'
                        or (request.get('kind') == 'bulk_service' and not result.get('remote_url'))):
                    result.update(not_attempted=True, remote_uncertain=False)
                status = 'failed'
            with self.connect() as db:
                db.execute('UPDATE jobs SET status=?, result=?, updated=? WHERE id=?',
                           (status, json.dumps(result or {}), time.time(), key))
                scrubbed = self._scrub_finished_images(db, key)
            if scrubbed:
                self._checkpoint_image_cleanup()
            self.queue.task_done()

