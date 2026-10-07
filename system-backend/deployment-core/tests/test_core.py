import copy
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import time
import tomllib
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from common import CATALOG, PROTOCOL, ROOT, atomic_write, load_config, peer_url, toml_dump
from bindings import resolve_config
from deploy import Repository, install_dropin, run, tbs_config, validate_job, validate_profile
from discovery import Discovery
from jobs import Jobs
from launch import merge_runtime_edits

REPO = ROOT.parents[1]


def config(state, **extra):
    cfg=load_config(ROOT/'config/agent.example.toml')
    cfg.update(state_dir=str(state), node_id='test-agent', **extra)
    return cfg


def manifest(node='gateway-01', services=None, **extra):
    return dict(protocol=PROTOCOL, environment='netcore-openlab', security_mode='open_lab',
                node_id=node, role='agent', services=services if services is not None else [{'name':'node-gateway','port':8080,'ready':True}], **extra)


class CoreTests(unittest.TestCase):
    def test_all_repository_templates_roundtrip(self):
        for service in CATALOG.values():
            with self.subTest(service=service['name']):
                data=tomllib.loads((REPO/service['config_template']).read_text())
                self.assertEqual(tomllib.loads(toml_dump(data)), data)

    def test_semantic_binding_does_not_confuse_8080(self):
        original={'node_gateway':{'url':'ws://old:8080/ws/backend'},
                  'playout':{'stations':[{'base_url':'http://radio:8080'}]},
                  'server':{'public_base_url':'http://myself:8230'},
                  'connectors':[{'connector_id':'sds-router','endpoint':'http://old:8150/api/v1/messages'},
                                {'connector_id':'external','endpoint':'http://other:8080/x'}]}
        endpoints={'node-gateway':{'url':'http://10.1.2.3:8080'},'sds-router':{'url':'http://10.1.2.4:8150'}}
        result, changes=resolve_config('media-library',original,endpoints)
        self.assertEqual(result['node_gateway']['url'],'ws://10.1.2.3:8080/ws/backend')
        self.assertEqual(result['playout'],original['playout'])
        self.assertEqual(result['server'],original['server'])
        self.assertEqual(result['connectors'][0]['endpoint'],'http://10.1.2.4:8150/api/v1/messages')
        self.assertEqual(result['connectors'][1],original['connectors'][1])
        self.assertEqual(len(changes),2)

    def test_tbs_gateway_and_ui_edit_persistence(self):
        source={'control_room':{'host':'old','port':8080},'name':'original','values':[1]}
        runtime=copy.deepcopy(source);runtime['name']='edited';runtime['values']=[1,2]
        current=copy.deepcopy(source);current['values']=[5]
        merged=merge_runtime_edits(current,source,runtime)
        self.assertEqual(merged['name'],'edited')
        self.assertEqual(merged['values'],[5])
        changed,_=resolve_config('tbs',merged,{'node-gateway':{'url':'http://10.0.1.30:8080'}})
        self.assertEqual(changed['control_room']['host'],'10.0.1.30')

    def test_provisioning_top_level_dependencies_resolve_without_rewriting_other_urls(self):
        source = {'subscriber_core': 'http://old:8100', 'group_core': 'http://old:8110',
                  'public_base_url': 'http://local:8125', 'external': 'http://old:8100'}
        changed, fields = resolve_config('provisioning-core', source, {
            'subscriber-core': {'url': 'http://10.0.20.12:8100'},
            'group-core': {'url': 'http://10.0.20.13:8110'}})
        self.assertEqual(changed['subscriber_core'], 'http://10.0.20.12:8100')
        self.assertEqual(changed['group_core'], 'http://10.0.20.13:8110')
        self.assertEqual(changed['external'], source['external'])
        self.assertEqual(changed['public_base_url'], source['public_base_url'])
        self.assertEqual(set(fields), {'subscriber_core', 'group_core'})

    def test_peer_validation_and_conflicts_and_outage(self):
        with tempfile.TemporaryDirectory() as d:
            cfg=config(d);discovery=Discovery(cfg,lambda:manifest('test-agent',[]))
            try:
                discovery.accept('http://10.0.1.10:8321',manifest())
                first=discovery.snapshot()['endpoints']['node-gateway']
                self.assertEqual(first['url'],'http://10.0.1.10:8080')
                discovery.accept('http://10.0.1.11:8321',manifest('gateway-02'))
                self.assertIn('node-gateway',discovery.snapshot()['conflicts'])
                self.assertEqual(discovery.snapshot()['endpoints']['node-gateway'],first)
                cfg['bindings']={'node-gateway':'gateway-02'}
                discovery._resolve()
                self.assertEqual(discovery.endpoints['node-gateway']['node_id'],'gateway-02')
                for peer in discovery.peers.values(): peer['last_seen']=0
                discovery._resolve()
                self.assertEqual(discovery.endpoints['node-gateway']['node_id'],'gateway-02')
                bad=manifest();bad['environment']='other'
                with self.assertRaises(ValueError): discovery.accept('http://10.0.1.12:8321',bad)
                with self.assertRaises(ValueError): discovery.accept('http://8.8.8.8:8321',manifest())
                self.assertNotIn('password',json.dumps(discovery.snapshot()))
            finally: discovery.close()

    def test_url_validation(self):
        for url in ['https://10.0.0.1:8321','http://10.0.0.1:8321/evil','http://user@10.0.0.1:8321','http://127.0.0.1:8321/?x=1','file:///etc/passwd','http://hostname:8321']:
            with self.subTest(url=url), self.assertRaises(ValueError):peer_url(url,['10.0.0.0/8'])

    def test_live_but_degraded_service_is_discoverable_without_dependency_deadlock(self):
        with tempfile.TemporaryDirectory() as d:
            discovery=Discovery(config(d),lambda:manifest('test-agent',[]))
            try:
                discovery.accept('http://10.0.1.11:8321',manifest('control',[
                    {'name':'control-room','port':9010,'ready':False,'available':True}]))
                self.assertIn('control-room',discovery.endpoints)
                self.assertFalse(discovery.peers['control']['services'][0]['ready'])
            finally:discovery.close()

    def test_input_validation_and_openlab(self):
        profile=dict(name='TBS-02',mcc=901,mnc=1510,issi=4010002,la=2,cc=2)
        self.assertEqual(validate_profile(profile),profile)
        for key,value in [('cc',64),('mcc',True),('issi',0),('name','$(whoami)')]:
            with self.assertRaises(ValueError):validate_profile(dict(profile,**{key:value}))
        rendered=tomllib.loads(tbs_config((REPO/CATALOG['tbs']['config_template']).read_text(),profile))
        self.assertEqual(rendered['net_info']['mcc'],901)
        self.assertEqual(rendered['control_room']['node_id'],'TBS-02')
        self.assertEqual(rendered['brew']['username'],4010002)
        for request in [dict(service='bogus',action='restart',confirm_restart=True),
                        dict(service='node-gateway',action='update',commit='main',confirm_restart=True),
                        dict(service='tbs',action='restart')]:
            with self.assertRaises(ValueError):validate_job(request)
        with tempfile.TemporaryDirectory() as d:
            f=Path(d)/'bad.toml';f.write_text('mode="production"\n')
            with self.assertRaises(ValueError):load_config(f)

    def test_jobs_success_failure_and_restart_recovery(self):
        with tempfile.TemporaryDirectory() as d:
            def execute(request,log):
                log('started')
                if request.get('fail'): raise ValueError('simulated failure')
                return {'done':True}
            jobs=Jobs(d,execute)
            good=jobs.submit({})['id'];bad=jobs.submit({'fail':True})['id']
            jobs.queue.join()
            self.assertEqual(jobs.get(good)['status'],'succeeded')
            self.assertEqual(jobs.get(bad)['status'],'failed')
            with jobs.connect() as db: db.execute("UPDATE jobs SET status='running' WHERE id=?",(good,))
            recovered=Jobs(d,execute)
            self.assertEqual(recovered.get(good)['status'],'interrupted')

    def test_git_branch_resolves_to_pinned_commit(self):
        with tempfile.TemporaryDirectory() as d:
            origin=Path(d)/'origin';origin.mkdir()
            subprocess.run(['git','init','-b','main',str(origin)],check=True,capture_output=True)
            subprocess.run(['git','-C',str(origin),'-c','user.name=Test','-c','user.email=test@example.invalid','commit','--allow-empty','-m','initial'],check=True,capture_output=True)
            cfg=config(Path(d)/'agent');cfg['repository']=str(origin)
            repo=Repository(cfg);logs=[]
            sha=repo.resolve('main',logs.append);self.assertEqual(len(sha),40)
            source=repo.checkout(sha,logs.append)
            self.assertEqual(subprocess.check_output(['git','rev-parse','HEAD'],cwd=source,text=True).strip(),sha)
            with self.assertRaises(ValueError):repo.resolve('--upload-pack=evil',logs.append)

    def test_command_stream_no_lost_output_or_shell(self):
        lines=[]
        run([sys.executable,'-c','print("line\\n" * 20000)'],lines.append,timeout=5)
        self.assertEqual(('\n'.join(lines[1:])+'\n').count('line\n'),20000)
        with self.assertRaises(RuntimeError):run([sys.executable,'-c','raise SystemExit(3)'],lines.append)
        with self.assertRaises(TimeoutError):run([sys.executable,'-c','import time;time.sleep(3)'],lines.append,timeout=.2)

    def test_dropin_keeps_original_source_and_no_service_dependency(self):
        with tempfile.TemporaryDirectory() as d:
            install_dropin(CATALOG['node-gateway'],Path('/var/lib/netcore-discovery/endpoints.json'),Path(d))
            text=(Path(d)/'netcore-node-gateway.service.d/50-discovery.conf').read_text()
            self.assertIn('/etc/netcore/node-gateway.toml',text)
            self.assertIn('StateDirectory=netcore-discovery-node-gateway',text)
            self.assertNotIn('Requires=',text)
            self.assertNotIn('Wants=',text)


if __name__=='__main__':unittest.main()
