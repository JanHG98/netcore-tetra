"""Isolated browser-test fixture. Never executes host installers."""
from pathlib import Path
import signal
import sys
import tempfile
import threading

sys.path.insert(0,str(Path(__file__).resolve().parents[1]))
from common import ROOT,load_config
from main import App,Server,handler

with tempfile.TemporaryDirectory() as state:
    apps=[];servers=[]
    for name,role in [('test-controller','controller'),('test-gateway','agent')]:
        cfg=load_config(ROOT/'config/agent.example.toml')
        cfg.update(node_id=name,role=role,state_dir=state+'/'+name)
        app=App(cfg)
        server=Server(('127.0.0.1',0),handler(app))
        cfg['port']=server.server_address[1]
        threading.Thread(target=server.serve_forever,daemon=True).start()
        apps.append(app);servers.append(server)
    controller,agent=apps
    agent.local=[{'name':'node-gateway','port':8080,'ready':True,'commit':'a'*40}]
    agent.jobs.execute=lambda data,log:(log('Browser integration: simulated installer'),{'commit':data.get('commit','')})[1]
    controller.repo.resolve=lambda ref,log:'b'*40
    controller.desired={'ref':'feature/openlab-discovery-deployment','commit':'b'*40}
    controller.discovery.accept(f'http://127.0.0.1:{servers[1].server_address[1]}',agent.manifest())
    print(f'http://127.0.0.1:{servers[0].server_address[1]}',flush=True)
    signal.pause()
