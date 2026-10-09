"""Image builds fetch and pin the current origin/main commit for the whole repo."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import Mock
from urllib.error import HTTPError

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from common import request_json
from deploy import Repository
import test_http
from test_images import image_request
from test_profile_lifecycle import ImageFixture


class LatestMainRepositoryTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.work, self.origin = self.root / 'work', self.root / 'origin.git'
        self.git('init', '-b', 'main', str(self.work))
        self.git('init', '--bare', str(self.origin))
        self.git('-C', str(self.work), 'remote', 'add', 'origin', str(self.origin))
        self.first = self.commit('first')
        self.git('-C', str(self.work), 'tag', 'main')
        self.git('-C', str(self.work), 'push', 'origin', 'refs/heads/main', '--tags')
        self.repo = Repository({'repository': str(self.origin), 'state_dir': str(self.root / 'cache')})

    def git(self, *args):
        return subprocess.check_output(['git', *args], stderr=subprocess.STDOUT, text=True).strip()

    def commit(self, message):
        self.git('-C', str(self.work), '-c', 'user.name=Test', '-c', 'user.email=test@example.invalid',
                 'commit', '--allow-empty', '-m', message)
        return self.git('-C', str(self.work), 'rev-parse', 'HEAD')

    def test_each_resolution_fetches_new_main_and_never_uses_same_named_tag(self):
        self.assertEqual(self.repo.resolve_latest_main(lambda _: None), self.first)
        latest = self.commit('new main')
        self.git('-C', str(self.work), 'push', 'origin', 'refs/heads/main')
        self.assertEqual(self.repo.resolve_latest_main(lambda _: None), latest)
        self.assertEqual(self.git('-C', str(self.repo.path), 'rev-parse', 'refs/tags/main'), self.first)

    def test_missing_main_rejects_even_when_a_main_tag_exists(self):
        self.assertEqual(self.repo.resolve_latest_main(lambda _: None), self.first)
        self.git('-C', str(self.origin), 'update-ref', '-d', 'refs/heads/main')
        with self.assertRaisesRegex(ValueError, 'main-Branch'):
            self.repo.resolve_latest_main(lambda _: None)
        self.assertEqual(self.git('-C', str(self.repo.path), 'rev-parse', 'refs/tags/main'), self.first)

    def test_fetch_failure_never_returns_a_cached_main(self):
        self.repo.resolve_latest_main(lambda _: None)
        self.git('-C', str(self.repo.path), 'remote', 'set-url', 'origin', str(self.root / 'missing.git'))
        with self.assertRaises(RuntimeError):
            self.repo.resolve_latest_main(lambda _: None)
        self.assertEqual(self.git('-C', str(self.repo.path), 'rev-parse', 'refs/remotes/origin/main'), self.first)


class LatestImageHTTPTests(unittest.TestCase):
    def setUp(self):
        self.fixture = test_http.HTTPTests()
        self.fixture.setUp()
        self.addCleanup(self.fixture.tearDown)
        self.app, self.url = self.fixture.app('images', 'controller', ref='stale-deployment-branch')
        self.req = image_request()
        request_json(self.url + '/api/v1/template', {'toml': self.req['config']})
        request_json(self.url + '/api/v1/profiles', self.req['profile'])
        self.images = ImageFixture()
        self.app.images = self.images
        self.app.repo.resolve = Mock(side_effect=AssertionError('legacy ref resolver must not be used'))
        self.app.repo.resolve_latest_main = Mock(return_value='b' * 40)

    def test_request_refs_and_old_desired_commit_cannot_override_fresh_main(self):
        self.app.desired = {'ref': 'old-tag', 'commit': 'a' * 40}
        response = request_json(self.url + '/api/v1/images/build', dict(self.req, profile='TBS-02',
                                                                      ref='main-tag-or-old-sha'))
        self.assertEqual(response['status'], 'queued')
        self.app.repo.resolve_latest_main.assert_called_once()
        self.assertEqual(self.images.requests[0][1]['commit'], 'b' * 40)
        self.assertNotIn('ref', self.images.requests[0][1])
        self.assertEqual(self.app.cfg['ref'], 'stale-deployment-branch')
        request_json(self.url + '/api/v1/status')
        request_json(self.url + '/api/v1/images')
        self.app.repo.resolve_latest_main.assert_called_once()

    def test_invalid_request_is_rejected_before_git_or_worker_submission(self):
        with self.assertRaises(HTTPError) as error:
            request_json(self.url + '/api/v1/images/build', dict(self.req, profile='TBS-02', username='root'))
        self.assertEqual(error.exception.code, 400)
        self.app.repo.resolve_latest_main.assert_not_called()
        self.assertEqual(self.images.requests, [])

    def test_fetch_or_missing_main_failure_never_submits_a_build(self):
        for failure in (RuntimeError('fetch failed'), ValueError('main branch missing')):
            with self.subTest(failure=failure):
                self.app.repo.resolve_latest_main.side_effect = failure
                with self.assertRaises(HTTPError):
                    request_json(self.url + '/api/v1/images/build', dict(self.req, profile='TBS-02'))
                self.assertEqual(self.images.requests, [])


if __name__ == '__main__':
    unittest.main()
