import json
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
import socket
import sqlite3
import sys
import tempfile
import threading
import time
import unittest
from unittest.mock import patch
from urllib.error import HTTPError
from urllib.request import Request, urlopen

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from common import ROOT, PROTOCOL, load_config, request_json
from main import App, Server, handler


class HTTPTests(unittest.TestCase):
    def setUp(self):
        self.temp=tempfile.TemporaryDirectory()
        self.apps=[];self.servers=[]

    def tearDown(self):
        for app in self.apps: app.stop.set();app.discovery.close()
        for server in self.servers: server.shutdown();server.server_close()
        self.temp.cleanup()

    def app(self,name,role='agent',**extra):
        cfg=load_config(ROOT/'config/agent.example.toml')
        cfg.update(node_id=name,role=role,state_dir=self.temp.name+'/'+name,**extra)
        app=App(cfg);server=Server(('127.0.0.1',0),handler(app))
        cfg['port']=server.server_address[1]
        threading.Thread(target=server.serve_forever,daemon=True).start()
        self.apps.append(app);self.servers.append(server)
        return app, f'http://127.0.0.1:{cfg["port"]}'

    def wait_for(self,predicate):
        deadline=time.monotonic()+5
        while not predicate():
            if time.monotonic()>deadline:self.fail('Timed out')
            time.sleep(.02)

    def test_unicast_real_http_discovery_and_cache(self):
        controller,curl=self.app('controller','controller')
        agent,aurl=self.app('gateway')
        agent.local=[{'name':'node-gateway','port':8080,'ready':True,'commit':'a'*40}]
        controller.discovery.schedule(aurl)
        self.wait_for(lambda: 'gateway' in controller.discovery.peers)
        status=request_json(curl+'/api/v1/status')
        self.assertEqual(status['endpoints']['node-gateway']['url'],'http://127.0.0.1:8080')
        self.assertTrue(status['peers'][0]['online'])
        # A second node can learn about the first through a routed controller seed.
        remote,rurl=self.app('remote',seeds=[curl])
        remote.discovery.schedule(curl)
        self.wait_for(lambda: 'gateway' in remote.discovery.peers)
        self.assertIn('node-gateway',remote.discovery.endpoints)
        self.assertTrue((controller.state/'endpoints.json').exists())

    def test_multicast_without_seed_uses_real_agent_manifest(self):
        with socket.socket(socket.AF_INET,socket.SOCK_DGRAM) as s:
            s.bind(('127.0.0.1',0));port=s.getsockname()[1]
        controller,curl=self.app('controller','controller',interface='127.0.0.1',discovery_port=port,interval=2)
        agent,aurl=self.app('gateway',interface='127.0.0.1',discovery_port=port,interval=2)
        agent.local=[{'name':'node-gateway','port':8080,'ready':True,'commit':'a'*40}]
        controller.discovery.start();agent.discovery.start()
        if not controller.discovery.sock or not agent.discovery.sock:
            self.skipTest('Multicast unavailable in test environment')
        self.wait_for(lambda:'gateway' in controller.discovery.peers)
        self.assertEqual(controller.discovery.peers['gateway']['agent_url'],aurl)

    def test_http_validation_errors_do_not_crash_server(self):
        app,url=self.app('agent')
        for payload in [dict(service='no',action='restart',confirm_restart=True),
                        dict(service='node-gateway',action='restart')]:
            with self.assertRaises(HTTPError) as exc:request_json(url+'/api/v1/jobs',payload)
            self.assertEqual(exc.exception.code,400)
        req=Request(url+'/api/v1/discovery/scan',data=b'{}',headers={'Content-Type':'application/json','Origin':'http://other.invalid'})
        with self.assertRaises(HTTPError) as exc:urlopen(req)
        self.assertEqual(exc.exception.code,400)
        self.assertTrue(request_json(url+'/health/ready')['ready'])
        self.assertTrue(request_json(url+'/api/v1/discovery/scan',{})['accepted'])
        self.assertIn(b'Alles an einem Ort',urlopen(url).read())

    def test_public_monitoring_contracts_over_real_http(self):
        app, url = self.app('controller', 'controller')
        app.cfg['unused_secret'] = 'never-publish-this-secret'
        spec = request_json(url + '/openapi.json')
        self.assertEqual(spec['openapi'], '3.0.3')
        self.assertIn('/api/v1/status', spec['paths'])
        self.assertIn('/metrics', spec['paths'])
        with urlopen(url + '/metrics') as response:
            self.assertIn('text/plain', response.headers['Content-Type'])
            metrics = response.read().decode()
        self.assertIn('netcore_deployment_up 1\n', metrics)
        self.assertIn('netcore_deployment_peers{state="online"} 0\n', metrics)
        self.assertIn('netcore_deployment_jobs{status="failed"} 0\n', metrics)
        self.assertNotIn('never-publish-this-secret', metrics + json.dumps(spec))

    def test_job_status_http_waits_for_concurrent_database_write(self):
        app, url = self.app('agent')
        calls = []
        app.jobs.execute = lambda request, log: calls.append(request) or {'ready': True}
        key = app.jobs.submit({'service': 'hardware-gateway'})['id']
        self.wait_for(lambda: app.jobs.get(key)['status'] == 'succeeded')
        locked, release = threading.Event(), threading.Event()
        entered_get, entered_list = threading.Event(), threading.Event()
        original_connect = sqlite3.connect
        original_get, original_list = app.jobs.get, app.jobs.list

        def short_timeout(*args, **kwargs):
            kwargs['timeout'] = .02
            return original_connect(*args, **kwargs)

        def write():
            with app.jobs.connect() as db:
                db.execute('BEGIN EXCLUSIVE')
                db.execute('UPDATE jobs SET log=? WHERE id=?', ('committed log', key))
                locked.set()
                if not release.wait(5):
                    raise RuntimeError('reader setup timed out')

        def get(key):
            entered_get.set()
            return original_get(key)

        def listing():
            entered_list.set()
            return original_list()

        # Real SQLite lock contention and real HTTP, with a short busy timeout
        # so the former HTTP 500 is reproduced without a ten-second test delay.
        with patch('jobs.sqlite3.connect', side_effect=short_timeout), \
                patch.object(app.jobs, 'get', side_effect=get), \
                patch.object(app.jobs, 'list', side_effect=listing), \
                ThreadPoolExecutor(max_workers=3) as pool:
            writer = pool.submit(write)
            try:
                self.assertTrue(locked.wait(3))
                detail = pool.submit(request_json, url + '/api/v1/jobs/' + key)
                overview = pool.submit(request_json, url + '/api/v1/jobs')
                self.assertTrue(entered_get.wait(3))
                self.assertTrue(entered_list.wait(3))
                time.sleep(.08)
                self.assertFalse(detail.done(), 'status read bypassed database serialization')
                self.assertFalse(overview.done(), 'listing bypassed database serialization')
            finally:
                release.set()
            writer.result(timeout=3)
            self.assertEqual(detail.result(timeout=3)['log'], 'committed log')
            self.assertEqual(overview.result(timeout=3)[0]['log'], 'committed log')
        self.assertEqual(len(calls), 1)

    def test_controller_plan_remote_job_and_commit_pinning(self):
        controller,curl=self.app('controller','controller')
        agent,aurl=self.app('gateway')
        agent.local=[{'name':'node-gateway','port':8080,'ready':True,'commit':'a'*40}]
        controller.discovery.accept(aurl,agent.manifest())
        sha='b'*40
        controller.repo.resolve=lambda ref,log:sha
        received=[]
        def execute(data,log):
            received.append(data);log('simulated target installer');return {'commit':data['commit']}
        agent.jobs.execute=execute
        plan=request_json(curl+'/api/v1/plan',dict(node_id='gateway',service='node-gateway',action='update',ref='main'))
        job=request_json(curl+'/api/v1/deploy',plan)
        self.wait_for(lambda:controller.jobs.get(job['id'])['status']=='succeeded')
        self.assertEqual(received[0]['commit'],sha)
        self.assertNotIn('ref',received[0])
        self.assertEqual(controller.jobs.get(job['id'])['result']['commit'],sha)
        self.assertIn('simulated target installer',controller.jobs.get(job['id'])['log'])

    def test_profile_bootstrap_has_exact_sha_and_no_secrets(self):
        app,url=self.app('controller','controller')
        app.desired={'commit':'a'*40}
        profile=dict(name='TBS-02',mcc=901,mnc=1510,issi=4010002,la=2,cc=2)
        template='[net_info]\nmcc=1\nmnc=1\n[cell_info]\nlocation_area=1\ncolour_code=1\n[phy_io]\nbackend="SoapySdr"\n[dashboard]\nusername="admin"\npassword="secret"\n'
        request_json(url+'/api/v1/template',{'toml':template})
        request_json(url+'/api/v1/profiles',profile)
        body=urlopen(url+'/bootstrap.sh?profile=TBS-02').read().decode()
        self.assertIn('git checkout --detach '+'a'*40,body)
        self.assertIn('--node-id TBS-02',body)
        self.assertNotIn('secret',body)
        config=urlopen(url+'/api/v1/profiles/TBS-02/config').read().decode()
        self.assertNotIn('secret',config)


if __name__=='__main__':unittest.main()
