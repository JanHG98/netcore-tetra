#!/usr/bin/env python3
from pathlib import Path
import json, os, subprocess, sys, tempfile, time, tomllib
from urllib.request import Request, urlopen

ROOT=Path(__file__).resolve().parents[1]
required=[
 'system-backend/asset-management/src/netcore_asset_management.py',
 'system-backend/asset-management/config/asset-management.example.toml',
 'system-backend/asset-management/systemd/netcore-asset-management.service',
 'system-backend/asset-management/install/install.sh',
 'system-backend/asset-management/install/update.sh',
 'system-backend/asset-management/install/uninstall.sh',
 'system-backend/asset-management/install/configure-openlab.sh',
 'system-backend/shared/contracts/schemas/netcore-asset-v1.schema.json',
 'system-backend/shared/contracts/schemas/netcore-person-v1.schema.json',
 'system-backend/shared/contracts/schemas/netcore-assignment-v1.schema.json',
 'system-backend/shared/contracts/ASSET_MANAGEMENT_V1.md',
 'Docs/PHASE_10_ASSET_DEVICE_USER_MANAGEMENT.md',
]
errors=[]
for rel in required:
    if not (ROOT/rel).is_file(): errors.append(f'missing {rel}')
with open(ROOT/'system-backend/asset-management/config/asset-management.example.toml','rb') as h: cfg=tomllib.load(h)
if not cfg['server']['bind'].endswith(':8290'): errors.append('wrong port')
if cfg['security']['mode']!='open_lab': errors.append('not open_lab')
for rel,const in [
 ('netcore-asset-v1.schema.json','netcore-asset-v1'),
 ('netcore-person-v1.schema.json','netcore-person-v1'),
 ('netcore-assignment-v1.schema.json','netcore-assignment-v1')]:
    schema=json.loads((ROOT/'system-backend/shared/contracts/schemas'/rel).read_text())
    if schema['properties']['schema']['const']!=const: errors.append(f'wrong schema {rel}')
event=(ROOT/'system-backend/shared/contracts/src/event.rs').read_text()
for name in ['asset.created','asset.assigned','asset.returned','person.created','assignment.created','assignment.returned','maintenance.created','maintenance.due','maintenance.completed']:
    if name not in event: errors.append(f'missing event {name}')
for script in (ROOT/'system-backend/asset-management/install').glob('*.sh'):
    if not os.access(script,os.X_OK): errors.append(f'not executable {script}')
if errors:
    print('\n'.join(errors),file=sys.stderr); raise SystemExit(1)

def request(url,method='GET',payload=None):
    data=None if payload is None else json.dumps(payload).encode()
    req=Request(url,data=data,method=method,headers={'Content-Type':'application/json'})
    return json.loads(urlopen(req,timeout=3).read())

with tempfile.TemporaryDirectory() as td:
    td=Path(td); port=18290
    config=f'''[service]\nname="netcore-asset-management"\nphase=10\nmode="open_lab"\n[server]\nbind="127.0.0.1:{port}"\n[security]\nmode="open_lab"\n[storage]\nstate_file="{td/'state.json'}"\nevent_log="{td/'events.ndjson'}"\naudit_log="{td/'audit.ndjson'}"\n[mqtt]\nenabled=false\nhost="127.0.0.1"\nport=1883\ntopic_prefix="netcore/v1"\nclient_id="test"\n[management]\nevent_history_limit=200\nupstream_sync_interval_secs=3600\n[upstreams.subscriber_core]\nenabled=false\nbase_url="http://127.0.0.1:1"\n[upstreams.mobility_core]\nenabled=false\nbase_url="http://127.0.0.1:1"\n[upstreams.task_workflow]\nenabled=false\nbase_url="http://127.0.0.1:1"\ndefault_gssi=15201\n'''
    cp=td/'config.toml'; cp.write_text(config)
    proc=subprocess.Popen([sys.executable,str(ROOT/'system-backend/asset-management/src/netcore_asset_management.py'),'--config',str(cp)],stdout=subprocess.PIPE,stderr=subprocess.PIPE)
    try:
        for _ in range(60):
            try: urlopen(f'http://127.0.0.1:{port}/health/live',timeout=.2); break
            except Exception: time.sleep(.1)
        else: raise RuntimeError(proc.stderr.read().decode())
        person=request(f'http://127.0.0.1:{port}/api/v1/persons','POST',{'person_id':'jan','username':'jan','display_name':'Jan','rui_username':'Jan','rui_issi':4010001})
        assert person['schema']=='netcore-person-v1' and person['pin_stored'] is False
        asset=request(f'http://127.0.0.1:{port}/api/v1/assets','POST',{'asset_id':'hrt-001','inventory_id':'HRT-001','kind':'tetra_radio','serial_number':'S1','issi':4010001})
        assert asset['schema']=='netcore-asset-v1'
        assignment=request(f'http://127.0.0.1:{port}/api/v1/assignments','POST',{'asset_id':'hrt-001','person_id':'jan'})
        assert assignment['schema']=='netcore-assignment-v1' and assignment['rui_context']['pin_stored'] is False
        returned=request(f"http://127.0.0.1:{port}/api/v1/assignments/{assignment['assignment_id']}/return",'POST',{'return_note':'ok'})
        assert returned['status']=='returned'
        maintenance=request(f'http://127.0.0.1:{port}/api/v1/maintenance','POST',{'asset_id':'hrt-001','title':'Prüfung','kind':'inspection'})
        completed=request(f"http://127.0.0.1:{port}/api/v1/maintenance/{maintenance['record_id']}",'PUT',{'status':'completed','result':'bestanden'})
        assert completed['status']=='completed'
        status=request(f'http://127.0.0.1:{port}/api/v1/status')
        assert status['assets_total']==1 and status['persons_total']==1 and status['active_assignments']==0
        assert b'NetCore Asset Management' in urlopen(f'http://127.0.0.1:{port}/').read()
    finally:
        proc.terminate(); proc.wait(timeout=5)
print('OK: Phase 10 Asset Management contracts, runtime, assignment and maintenance workflow')
