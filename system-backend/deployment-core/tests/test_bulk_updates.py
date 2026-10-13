"""Durable bulk dispatch with real HTTP agent queues; installers are isolated."""
from contextlib import ExitStack
from concurrent.futures import ThreadPoolExecutor
import json
import os
from pathlib import Path
import sqlite3
import sys
import tempfile
import threading
import time
import types
import unittest
from unittest.mock import Mock, patch
from urllib.error import HTTPError

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
if os.name == 'nt':
    for name in ('fcntl', 'grp', 'pwd'):
        sys.modules.setdefault(name, types.ModuleType(name))
import main
from common import ROOT, load_config, request_json
from jobs import Jobs, JobConflict
from main import App, Server, handler

SHA = 'b' * 40


class BulkUpdateTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.stack = ExitStack()
        self.apps, self.servers = [], []
        self.stack.enter_context(patch.object(main, 'REMOTE_POLL_INTERVAL', .01))
        if os.name == 'nt':
            def write(path, value, mode=0o600):
                path = Path(path)
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text(value, encoding='utf-8')
            for module in ('common', 'main', 'discovery', 'deploy'):
                self.stack.enter_context(patch(module + '.atomic_write', side_effect=write))
        self.controller, self.url = self.app('controller', 'controller')
        self.controller.repo.resolve_latest_main = Mock(return_value=SHA)
        self.controller.repo.resolve = Mock(side_effect=AssertionError('batch must not resolve ref again'))

    def tearDown(self):
        for app in self.apps:
            app.stop.set()
        for server in self.servers:
            server.shutdown()
            server.server_close()
        for app in self.apps:
            app.jobs.queue.join()
            app.discovery.close()
        self.stack.close()
        self.temp.cleanup()

    def app(self, name, role='agent', services=()):
        cfg = load_config(ROOT / 'config/agent.example.toml')
        cfg.update(node_id=name, role=role, state_dir=self.temp.name + '/' + name)
        app = App(cfg)
        app.local = [dict(name=service, port=8000 + index, ready=False, available=False,
                          commit='a' * 40, unit=service + '.service') for index, service in enumerate(services)]
        app.jobs.execute = app.execute if role == 'controller' else lambda data, log: dict(commit=data['commit'], service=data['service'], ready=True)
        server = Server(('127.0.0.1', 0), handler(app))
        cfg['port'] = server.server_address[1]
        threading.Thread(target=server.serve_forever, daemon=True).start()
        self.apps.append(app)
        self.servers.append(server)
        url = f'http://127.0.0.1:{cfg["port"]}'
        if role == 'agent':
            self.controller.discovery.accept(url, app.manifest())
        return app, url

    def plan(self):
        return request_json(self.url + '/api/v1/updates/plan', {})

    def execute_plan(self, plan):
        return request_json(self.url + '/api/v1/updates/run', dict(plan_id=plan['plan_id'], confirm_restart=True))

    def completed(self, key):
        deadline = time.monotonic() + 5
        while time.monotonic() < deadline:
            result = self.controller.jobs.batch(key)
            if result['status'] not in ('queued', 'running'):
                return result
            time.sleep(.01)
        self.fail('bulk fixture did not finish')

    def test_cross_host_parallelism_host_serialization_one_commit_and_installed_degraded_targets(self):
        agents = [self.app('alpha', services=('node-gateway', 'packet-core'))[0],
                  self.app('beta', services=('tbs',))[0]]
        started, release = [threading.Event(), threading.Event()], threading.Event()
        active, maximum, received = {}, {}, []
        lock = threading.Lock()
        for index, app in enumerate(agents):
            def install(data, log, index=index):
                with lock:
                    active[index] = active.get(index, 0) + 1
                    maximum[index] = max(maximum.get(index, 0), active[index])
                    received.append(dict(data))
                started[index].set()
                if not release.wait(4):
                    raise RuntimeError('cross-host dispatch did not start')
                with lock:
                    active[index] -= 1
                return dict(commit=data['commit'], service=data['service'], ready=True)
            app.jobs.execute = install
        plan = self.plan()
        self.assertEqual(len(plan['targets']), 3)
        self.assertEqual({row['service'] for row in plan['targets']}, {'node-gateway', 'packet-core', 'tbs'})
        batch = self.execute_plan(plan)
        try:
            self.assertTrue(all(event.wait(2) for event in started), 'another host must start before the first installer returns')
        finally:
            release.set()
        result = self.completed(batch['id'])
        self.assertEqual(result['status'], 'succeeded', result)
        self.assertEqual(maximum, {0: 1, 1: 1})
        self.assertEqual({row['commit'] for row in received}, {SHA})
        self.assertEqual({row['action'] for row in received}, {'update'})
        self.assertEqual(len(received), 3)
        self.controller.repo.resolve_latest_main.assert_called_once()
        self.controller.repo.resolve.assert_not_called()

    def test_offline_and_unknown_roles_are_skipped_without_install(self):
        agent, url = self.app('online', services=('call-control',))
        agent.local.append(dict(name='external-service', port=8099))
        offline, _ = self.app('offline', services=('media-switch',))
        with self.controller.discovery.lock:
            self.controller.discovery.peers['offline']['last_seen'] = 0
        plan = self.plan()
        self.assertEqual([(row['node_id'], row['service']) for row in plan['targets']], [('online', 'call-control')])
        self.assertGreaterEqual(len(plan['skipped']), 2)
        self.assertEqual(offline.jobs.list(), [])

    def test_all_offline_returns_empty_preview_with_reasons_and_cannot_execute(self):
        agent, _ = self.app('offline', services=('node-gateway',))
        with self.controller.discovery.lock:
            self.controller.discovery.peers['offline']['last_seen'] = 0
        plan = self.plan()
        self.assertEqual(plan['targets'], [])
        self.assertEqual(plan['counts']['targets'], 0)
        self.assertEqual(len(plan['skipped']), 1)
        with self.assertRaises(HTTPError) as error:
            self.execute_plan(plan)
        self.assertEqual(error.exception.code, 400)
        error.exception.close()
        self.assertEqual(agent.jobs.list(), [])

    def test_execute_twice_concurrently_consumes_plan_once(self):
        agent, _ = self.app('alpha', services=('node-gateway',))
        plan = self.plan()
        with ThreadPoolExecutor(max_workers=2) as pool:
            responses = list(pool.map(lambda _: self.execute_plan(plan), range(2)))
        self.assertEqual(responses[0]['id'], responses[1]['id'])
        self.assertEqual(responses[0]['targets'][0]['job_id'], responses[1]['targets'][0]['job_id'])
        self.completed(responses[0]['id'])
        self.assertEqual(len(agent.jobs.list()), 1)

    def test_terminal_failure_does_not_hide_other_host_success(self):
        failed, _ = self.app('alpha', services=('packet-core',))
        self.app('beta', services=('node-gateway',))
        def fail(data, log):
            raise RuntimeError('fixture compiler failed')
        failed.jobs.execute = fail
        result = self.completed(self.execute_plan(self.plan())['id'])
        self.assertEqual(result['status'], 'partial')
        self.assertEqual((result['counts']['succeeded'], result['counts']['failed']), (1, 1))
        self.assertEqual(result['counts']['uncertain'], 0)

    def test_controller_stop_before_work_or_after_manifest_never_posts_installer(self):
        agent, url = self.app('alpha', services=('node-gateway',))
        data = dict(kind='bulk_service', batch_id='f' * 32, node_id='alpha', service='node-gateway',
                    agent_url=url, commit=SHA, action='update', confirm_restart=True)
        self.controller.stop.set()
        with patch.object(main, 'request_json', wraps=request_json) as transport:
            early = self.controller.jobs.submit(data)
            self.controller.jobs.queue.join()
            transport.assert_not_called()
        result = self.controller.jobs.get(early['id'])
        self.assertTrue(result['result']['not_attempted'])
        self.assertFalse(result['result']['remote_uncertain'])
        self.controller.stop.clear()
        posts = []
        def request(target, payload=None, **kwargs):
            if payload is not None:
                posts.append(target)
            result = request_json(target, payload, **kwargs)
            if target.endswith('/api/v1/manifest'):
                self.controller.stop.set()
            return result
        with patch.object(main, 'request_json', side_effect=request):
            late = self.controller.jobs.submit(data)
            self.controller.jobs.queue.join()
        result = self.controller.jobs.get(late['id'])
        self.assertTrue(result['result']['not_attempted'])
        self.assertFalse(result['result']['remote_uncertain'])
        self.assertEqual(posts, [])
        self.assertEqual(agent.jobs.list(), [])

    def test_remote_success_requires_exact_batch_commit(self):
        agent, _ = self.app('alpha', services=('node-gateway',))
        agent.jobs.execute = lambda data, log: dict(commit='c' * 40, service=data['service'], ready=True)
        result = self.completed(self.execute_plan(self.plan())['id'])
        self.assertEqual(result['status'], 'failed')
        self.assertIn('Commit', result['jobs'][0]['result']['error'])
        self.assertFalse(result['jobs'][0]['result']['remote_uncertain'])

    def test_remote_success_requires_requested_service_even_with_correct_commit(self):
        agent, _ = self.app('alpha', services=('node-gateway',))
        agent.jobs.execute = lambda data, log: dict(commit=data['commit'], service='packet-core', ready=True)
        result = self.completed(self.execute_plan(self.plan())['id'])
        self.assertEqual(result['status'], 'failed')
        self.assertIn('Dienst', result['jobs'][0]['result']['error'])
        self.assertFalse(result['jobs'][0]['result']['remote_uncertain'])

    def test_plan_expiry_address_or_identity_change_rejects_without_dispatch(self):
        agent, _ = self.app('alpha', services=('node-gateway',))
        plan = self.plan()
        with self.controller.jobs.connect() as db:
            db.execute('UPDATE update_batches SET created=0 WHERE id=?', (plan['plan_id'],))
        with self.assertRaises(HTTPError) as error:
            self.execute_plan(plan)
        self.assertEqual(error.exception.code, 409)
        self.assertEqual(agent.jobs.list(), [])
        plan = self.plan()
        agent.cfg['node_id'] = 'replacement'
        with self.assertRaises(HTTPError) as error:
            self.execute_plan(plan)
        self.assertEqual(error.exception.code, 409)
        self.assertEqual(agent.jobs.list(), [])

    def test_pending_manual_job_conflicts_atomically_with_batch_admission(self):
        agent, _ = self.app('alpha', services=('node-gateway',))
        plan = self.plan()
        with self.controller.jobs.connect() as db:
            db.execute('INSERT INTO jobs VALUES (?,?,?,?,?,?,?)',
                       ('a' * 32, 0, 0, 'queued', json.dumps(dict(node_id='alpha', service='node-gateway')), '', '{}'))
        with self.assertRaises(HTTPError) as error:
            self.execute_plan(plan)
        self.assertEqual(error.exception.code, 409)
        self.assertEqual(self.controller.jobs.batch(plan['plan_id'])['status'], 'planned')
        self.assertEqual(agent.jobs.list(), [])

    def test_restart_preserves_remote_uncertainty_and_never_requeues_batch(self):
        state = self.temp.name + '/restarted'
        jobs = Jobs(state, lambda data, log: self.fail('must not dispatch'))
        target = dict(node_id='alpha', service='tbs', agent_url='http://127.0.0.1:8321')
        plan = jobs.plan_batch(dict(commit=SHA, ref='main', targets=[target], skipped=[]))
        child = 'a' * 32
        with jobs.connect() as db:
            request = dict(commit=SHA, targets=[dict(target, job_id=child)], skipped=[])
            db.execute("UPDATE update_batches SET status='started',request=? WHERE id=?", (json.dumps(request), plan['id']))
            db.execute('INSERT INTO jobs VALUES (?,?,?,?,?,?,?)',
                       (child, 0, 0, 'running', json.dumps(dict(node_id='alpha', service='tbs', batch_id=plan['id'])), '', '{}'))
        jobs.remote_progress(child, 'http://127.0.0.1:8321/api/v1/jobs/' + child, child)
        restarted = Jobs(state, lambda data, log: self.fail('must not dispatch'))
        result = restarted.batch(plan['id'])
        self.assertEqual(result['status'], 'interrupted')
        self.assertEqual(result['counts']['uncertain'], 1)
        self.assertEqual(result['jobs'][0]['result']['remote_job'], child)
        self.assertTrue(restarted.node_busy('alpha'))
        self.assertTrue(restarted.queue.empty())
        self.assertEqual(restarted.start_batch(plan['id'])['targets'][0]['job_id'], child)
        with self.assertRaises(JobConflict):
            restarted.submit(dict(node_id='alpha', service='node-gateway', action='update'))

    def test_restart_distinguishes_unsent_or_rejected_children_from_ambiguous_post(self):
        state = self.temp.name + '/recovery-cases'
        jobs = Jobs(state, lambda data, log: self.fail('must not dispatch'))
        cases = [('unsent', {}), ('rejected', dict(remote_url='http://127.0.0.1:8321/api/v1/jobs', dispatch_state='rejected', remote_uncertain=False)),
                 ('posting', dict(remote_url='http://127.0.0.1:8321/api/v1/jobs', dispatch_state='posting', remote_uncertain=True))]
        with jobs.connect() as db:
            for key, result in cases:
                db.execute('INSERT INTO jobs VALUES (?,?,?,?,?,?,?)',
                           (key, 0, 0, 'running', json.dumps(dict(kind='bulk_service', node_id=key, service='node-gateway')), '', json.dumps(result)))
        recovered = Jobs(state, lambda data, log: self.fail('must not dispatch'))
        for key in ('unsent', 'rejected'):
            row = recovered.get(key)
            self.assertEqual(row['status'], 'interrupted')
            self.assertTrue(row['result']['not_attempted'])
            self.assertFalse(row['result']['remote_uncertain'])
            self.assertFalse(recovered.node_busy(key))
        self.assertTrue(recovered.get('posting')['result']['remote_uncertain'])
        self.assertTrue(recovered.node_busy('posting'))
        self.assertTrue(recovered.queue.empty())

    def test_full_database_batch_status_and_busy_guard_exceed_listing_limit_without_bulk_logs(self):
        jobs = self.controller.jobs
        targets = [dict(node_id='alpha', service='node-gateway', agent_url='http://127.0.0.1:8321') for index in range(105)]
        plan = jobs.plan_batch(dict(commit=SHA, ref='main', targets=targets, skipped=[]))
        for index, target in enumerate(targets):
            target['job_id'] = f'{index:032x}'
        with jobs.connect() as db:
            db.execute("UPDATE update_batches SET status='started',request=? WHERE id=?",
                       (json.dumps(dict(commit=SHA, ref='main', targets=targets, skipped=[])), plan['id']))
            for index, target in enumerate(targets):
                result = dict(remote_uncertain=True, remote_job='old-uncertain') if index == 0 else dict(commit=SHA)
                db.execute('INSERT INTO jobs VALUES (?,?,?,?,?,?,?)',
                           (target['job_id'], index, index, 'failed' if index == 0 else 'succeeded',
                            json.dumps(dict(node_id='alpha', service='node-gateway', commit=SHA)), 'x' * 100000, json.dumps(result)))
        self.assertEqual(len(jobs.list()), 100)
        batch = jobs.batch(plan['id'])
        self.assertEqual(len(batch['jobs']), 105)
        self.assertEqual(batch['counts']['uncertain'], 1)
        self.assertTrue(jobs.node_busy('alpha'))
        self.assertLess(len(json.dumps(batch)), 100000)
        self.assertNotIn('log', batch['jobs'][0])
        self.assertEqual(len(jobs.get(targets[0]['job_id'])['log']), 100000)


if __name__ == '__main__':
    unittest.main(verbosity=2)
