"""Profile removal persists only metadata and refuses active or uncertain use."""
from concurrent.futures import ThreadPoolExecutor
from copy import deepcopy
import json
import os
from pathlib import Path
import stat
import sys
import threading
import time
import unittest
from unittest.mock import patch
from urllib.error import HTTPError
import uuid

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from common import atomic_write, request_json
from image_worker import Worker
from main import App
import test_http
from test_images import image_request


class ImageFixture:
    def __init__(self):
        self.active = []
        self.requests = []

    def status(self):
        return {'available': True, 'active_profiles': list(self.active), 'jobs': [], 'artifacts': []}

    def request(self, path, request):
        self.requests.append((path, deepcopy(request)))
        self.active = [request['profile']['name']]
        return {'id': 'd' * 32, 'status': 'queued'}


def persisted_job(jobs, profile, status='running', result=None, created=1):
    key = uuid.uuid4().hex
    with jobs.connect() as db:
        db.execute('INSERT INTO jobs VALUES (?,?,?,?,?,?,?)',
                   (key, created, created, status, json.dumps({'profile': profile}), '', json.dumps(result or {})))
    return key


class ProfileLifecycleTests(unittest.TestCase):
    def setUp(self):
        self.fixture = test_http.HTTPTests()
        self.fixture.setUp()
        self.addCleanup(self.fixture.tearDown)
        self.app, self.url = self.fixture.app('profiles', 'controller')
        self.images = ImageFixture()
        self.app.images = self.images
        self.req = image_request()
        request_json(self.url + '/api/v1/template', {'toml': self.req['config']})
        request_json(self.url + '/api/v1/profiles', self.req['profile'])
        self.other = dict(self.req['profile'], name='TBS-03', issi=4010003)
        request_json(self.url + '/api/v1/profiles', self.other)

    def remove(self, name='TBS-02', expected=None):
        if expected is None:
            return request_json(self.url + '/api/v1/profiles/remove', {'name': name})
        with self.assertRaises(HTTPError) as error:
            request_json(self.url + '/api/v1/profiles/remove', {'name': name})
        self.assertEqual(error.exception.code, expected)
        return json.loads(error.exception.read())

    def test_remove_persists_across_reload_and_preserves_other_state(self):
        job = persisted_job(self.app.jobs, 'TBS-02', 'succeeded')
        template = (self.app.state / 'tbs-site-template.toml').read_bytes()
        artifact = self.app.state / 'completed-image.fixture'
        artifact.write_bytes(b'previous image stays')
        self.assertEqual(self.remove(), {'deleted': 'TBS-02'})
        self.assertEqual(request_json(self.url + '/api/v1/profiles'), [self.other])
        self.assertEqual(json.loads((self.app.state / 'profiles.json').read_text()), {'TBS-03': self.other})
        restored = App(dict(self.app.cfg))
        self.addCleanup(lambda: (restored.stop.set(), restored.discovery.close()))
        self.assertEqual(restored.profiles, {'TBS-03': self.other})
        self.assertEqual(restored.jobs.get(job)['status'], 'succeeded')
        self.assertEqual((self.app.state / 'tbs-site-template.toml').read_bytes(), template)
        self.assertEqual(artifact.read_bytes(), b'previous image stays')

    def test_missing_and_invalid_profile_names_do_not_change_metadata(self):
        before = (self.app.state / 'profiles.json').read_bytes()
        self.remove('missing-profile', 404)
        for name in ('../TBS-02', '', None, 2):
            with self.subTest(name=name):
                self.remove(name, 400)
        self.assertEqual((self.app.state / 'profiles.json').read_bytes(), before)

    def test_persistence_error_leaves_memory_and_disk_unchanged(self):
        before = deepcopy(self.app.profiles)
        content = (self.app.state / 'profiles.json').read_bytes()
        with patch('main.atomic_write', side_effect=OSError('read-only filesystem')):
            self.remove(expected=400)
            with self.assertRaises(HTTPError):
                request_json(self.url + '/api/v1/profiles', dict(self.other, cc=3))
        self.assertEqual(self.app.profiles, before)
        self.assertEqual((self.app.state / 'profiles.json').read_bytes(), content)

    def test_post_replace_directory_fsync_failure_reports_applied_change_and_reconciles_memory(self):
        real_fsync = os.fsync
        def fsync(fd):
            if stat.S_ISDIR(os.fstat(fd).st_mode):
                raise OSError('directory fsync failed after replacement')
            return real_fsync(fd)
        with patch('common.os.fsync', side_effect=fsync):
            result = self.remove(expected=503)
        self.assertTrue(result['profile_change_applied'])
        self.assertTrue(result['durability_uncertain'])
        self.assertEqual(self.app.profiles, {'TBS-03': self.other})
        self.assertEqual(json.loads((self.app.state / 'profiles.json').read_text()), self.app.profiles)
        with patch('common.os.fsync', side_effect=fsync), self.assertRaises(HTTPError) as error:
            request_json(self.url + '/api/v1/profiles', dict(self.other, cc=3))
        self.assertEqual(error.exception.code, 503)
        self.assertTrue(json.loads(error.exception.read())['profile_change_applied'])
        self.assertEqual(self.app.profiles['TBS-03']['cc'], 3)
        self.assertEqual(json.loads((self.app.state / 'profiles.json').read_text()), self.app.profiles)

    def test_active_and_uncertain_deployments_are_not_hidden_by_ui_limit(self):
        for i in range(105):
            persisted_job(self.app.jobs, 'unrelated', 'succeeded', created=100 + i)
        for status, result in [('queued', {}), ('running', {}),
                               ('failed', {'remote_uncertain': True}),
                               ('interrupted', {'remote_uncertain': True})]:
            with self.subTest(status=status):
                key = persisted_job(self.app.jobs, 'TBS-02', status, result, created=1)
                self.assertNotIn(key, [job['id'] for job in self.app.jobs.list()])
                self.remove(expected=409)
                with self.app.jobs.connect() as db:
                    db.execute('DELETE FROM jobs WHERE id=?', (key,))
        self.assertIn('TBS-02', self.app.profiles)

    def test_active_image_names_cover_all_database_rows_and_block_matching_profile(self):
        worker = Worker(self.app.state / 'image-worker')
        self.addCleanup(worker.lock_file.close)
        for i in range(105):
            persisted_job(worker.jobs, {'name': 'other'}, 'succeeded', created=100 + i)
        persisted_job(worker.jobs, self.req['profile'], 'queued', created=1)
        status = worker.status()
        self.assertFalse(any(job['status'] == 'queued' for job in status['jobs']))
        self.assertEqual(status['active_profiles'], ['TBS-02'])
        self.app.images.status = lambda: status
        self.remove(expected=409)
        self.remove('TBS-03')

    def test_old_or_unavailable_image_worker_fails_closed(self):
        for status in ({'available': False}, {'available': True, 'jobs': []},
                       {'available': True, 'active_profiles': None},
                       {'available': True, 'active_profiles': [3]}):
            with self.subTest(status=status):
                self.app.images.status = lambda: status
                self.remove(expected=409)
                self.assertIn('TBS-02', self.app.profiles)

    def test_malformed_persisted_use_and_worker_transport_errors_fail_closed(self):
        for profile, result in [({'not-a-name': 'TBS-02'}, {}),
                                ('TBS-02', {'remote_uncertain': 'true'})]:
            with self.subTest(profile=profile, result=result):
                key = persisted_job(self.app.jobs, profile, 'running', result)
                self.remove(expected=409)
                with self.app.jobs.connect() as db:
                    db.execute('DELETE FROM jobs WHERE id=?', (key,))
        with patch.object(self.images, 'status', side_effect=ValueError('malformed worker reply')):
            self.remove(expected=409)
        self.assertIn('TBS-02', self.app.profiles)

    def test_build_profile_snapshot_is_immutable_during_concurrent_update_and_delete(self):
        entered, release = threading.Event(), threading.Event()
        def latest(log):
            entered.set()
            if not release.wait(3):
                raise RuntimeError('test synchronization timeout')
            return 'b' * 40
        self.app.repo.resolve_latest_main = latest
        request = dict(self.req, profile='TBS-02')
        with ThreadPoolExecutor(max_workers=3) as pool:
            build = pool.submit(request_json, self.url + '/api/v1/images/build', request)
            self.assertTrue(entered.wait(2))
            update = pool.submit(request_json, self.url + '/api/v1/profiles', dict(self.req['profile'], cc=3))
            delete = pool.submit(self.remove, 'TBS-02', 409)
            time.sleep(.05)
            self.assertFalse(update.done())
            self.assertFalse(delete.done())
            release.set()
            self.assertEqual(build.result(timeout=3)['status'], 'queued')
            self.assertEqual(update.result(timeout=3)['cc'], 3)
            delete.result(timeout=3)
        submitted = self.images.requests[0][1]
        self.assertEqual(submitted['profile']['cc'], 2)
        self.assertIn('colour_code', submitted['config'])
        self.assertEqual(self.app.profiles['TBS-02']['cc'], 3)

    def test_queued_deployment_persists_and_executes_original_profile_snapshot(self):
        agent, agent_url = self.fixture.app('target-agent')
        self.app.discovery.accept(agent_url, agent.manifest())
        entered, release = threading.Event(), threading.Event()
        self.addCleanup(release.set)
        def resolve(ref, log):
            entered.set()
            if not release.wait(3):
                raise RuntimeError('test synchronization timeout')
            return 'b' * 40
        self.app.repo.resolve = resolve
        received = []
        agent.jobs.execute = lambda request, log: received.append(deepcopy(request)) or {'ready': True}
        job = request_json(self.url + '/api/v1/deploy', {
            'node_id': 'target-agent', 'service': 'tbs', 'action': 'install',
            'profile': 'TBS-02', 'ref': 'main', 'confirm_restart': True})
        try:
            self.assertTrue(entered.wait(2))
            request_json(self.url + '/api/v1/profiles', dict(self.req['profile'], cc=3))
            self.remove(expected=409)
            queued = self.app.jobs.get(job['id'])
            self.assertEqual(queued['request']['profile_snapshot']['cc'], 2)
        finally:
            release.set()
        self.fixture.wait_for(lambda: self.app.jobs.get(job['id'])['status'] == 'succeeded')
        self.assertEqual(received[0]['profile']['cc'], 2)
        self.assertEqual(self.app.profiles['TBS-02']['cc'], 3)


if __name__ == '__main__':
    unittest.main()
