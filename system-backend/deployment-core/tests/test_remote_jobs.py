"""Remote polling failures must not duplicate or misreport a running installer."""
from pathlib import Path
import sys
import time
import unittest
from unittest.mock import patch
from urllib.error import HTTPError

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import main
from common import request_json
import test_http


class RemoteJobTests(unittest.TestCase):
    def setUp(self):
        self.fixture = test_http.HTTPTests()
        self.fixture.setUp()
        self.controller, self.curl = self.fixture.app('controller', 'controller')
        self.agent, self.aurl = self.fixture.app('gateway')
        self.controller.discovery.accept(self.aurl, self.agent.manifest())
        self.controller.repo.resolve = lambda ref, log: 'b' * 40
        self.installs = []
        self.posts = 0
        self.polls = 0

        def install(data, log):
            self.installs.append(data)
            log('installer finished')
            return {'commit': data['commit']}
        self.agent.jobs.execute = install
        self.interval = patch.object(main, 'REMOTE_POLL_INTERVAL', .01, create=True)
        self.interval.start()

    def tearDown(self):
        self.interval.stop()
        self.fixture.tearDown()

    def submit(self):
        return self.controller.jobs.submit(dict(node_id='gateway', service='node-gateway',
            action='update', ref='main', confirm_restart=True))

    def result(self, job):
        self.fixture.wait_for(lambda: self.controller.jobs.get(job['id'])['status']
                              not in ('queued', 'running'))
        return self.controller.jobs.get(job['id'])

    def transport(self, failures=0, permanent=False, lost_post=False):
        def send(url, data=None, **kwargs):
            if data is not None:
                self.posts += 1
                response = request_json(url, data, **kwargs)
                if lost_post:
                    raise TimeoutError('POST reply lost after the agent accepted it')
                return response
            self.polls += 1
            if permanent:
                raise HTTPError(url, 404, 'job missing', {}, None)
            if self.polls <= failures:
                raise TimeoutError('agent status delayed under build load')
            return request_json(url, **kwargs)
        return send

    def test_transient_status_timeout_recovers_without_resubmitting(self):
        with patch.object(main, 'request_json', side_effect=self.transport(failures=2)):
            result = self.result(self.submit())
        self.assertEqual(result['status'], 'succeeded', result['log'])
        self.assertEqual(self.posts, 1)
        self.assertEqual(len(self.installs), 1)
        self.assertGreaterEqual(self.polls, 3)
        self.assertEqual(result['result']['commit'], 'b' * 40)

    def test_slow_real_http_reply_longer_than_old_limit_succeeds(self):
        server = self.fixture.servers[-1]
        original = server.RequestHandlerClass
        class SlowStatus(original):
            def do_GET(self):
                if self.path.startswith('/api/v1/jobs/'):
                    time.sleep(3.4)
                super().do_GET()
        server.RequestHandlerClass = SlowStatus
        with patch.object(main, 'request_json', side_effect=self.transport()):
            result = self.result(self.submit())
        self.assertEqual(result['status'], 'succeeded', result['log'])
        self.assertEqual(self.posts, 1)
        self.assertEqual(self.polls, 1)

    def test_outage_budget_preserves_remote_id_and_uncertainty(self):
        with patch.object(main, 'REMOTE_RETRY_SECONDS', .02, create=True), \
             patch.object(main, 'request_json', side_effect=self.transport(failures=100)):
            result = self.result(self.submit())
        self.assertEqual(result['status'], 'failed')
        self.assertTrue(result['result']['remote_uncertain'])
        self.assertEqual(result['result']['remote_job'], self.agent.jobs.list()[0]['id'])
        self.assertIn(self.aurl, result['result']['remote_url'])
        self.assertEqual(self.posts, 1)
        self.assertGreaterEqual(self.polls, 2)

    def test_lost_post_response_never_repeats_the_install(self):
        with patch.object(main, 'request_json', side_effect=self.transport(lost_post=True)):
            result = self.result(self.submit())
        self.assertTrue(result['result']['remote_uncertain'])
        self.assertEqual(self.posts, 1)
        self.assertEqual(len(self.agent.jobs.list()), 1)
        self.assertEqual(self.polls, 0)

    def test_missing_remote_job_is_uncertain_without_endless_retries(self):
        with patch.object(main, 'request_json', side_effect=self.transport(permanent=True)):
            result = self.result(self.submit())
        self.assertTrue(result['result']['remote_uncertain'])
        self.assertEqual(self.posts, 1)
        self.assertEqual(self.polls, 1)

    def test_real_installer_failure_remains_a_failure(self):
        def broken(data, log):
            raise RuntimeError('compiler failed')
        self.agent.jobs.execute = broken
        result = self.result(self.submit())
        self.assertEqual(result['status'], 'failed')
        self.assertIn('compiler failed', result['result']['error'])
        self.assertFalse(result['result'].get('remote_uncertain', False))


if __name__ == '__main__':
    unittest.main()
