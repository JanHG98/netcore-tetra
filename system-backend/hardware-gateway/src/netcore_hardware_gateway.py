#!/usr/bin/env python3
from __future__ import annotations
import argparse, json, os, queue, signal, subprocess, threading, time, tomllib, uuid
from dataclasses import dataclass
from datetime import datetime, timezone
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import urlparse


def now(): return datetime.now(timezone.utc).isoformat().replace('+00:00','Z')
def safe_id(s): return ''.join(c for c in s if c.isalnum() or c in '-_.')[:128]

@dataclass
class Cfg:
    raw: dict
    @property
    def bind(self):
        host, port = self.raw['server']['bind'].rsplit(':',1); return host, int(port)
    @property
    def mqtt(self): return self.raw['mqtt']
    @property
    def storage(self): return self.raw['storage']
    @property
    def mon(self): return self.raw['monitoring']
    @property
    def thresholds(self): return self.raw.get('thresholds',[])

class App:
    def __init__(self,cfg):
        self.cfg=cfg; self.lock=threading.RLock(); self.devices={}; self.events=[]; self.started=now(); self.stop=False
        self.state_path=Path(cfg.storage['state_file']); self.event_path=Path(cfg.storage['event_log'])
        self.state_path.parent.mkdir(parents=True,exist_ok=True); self._load()
    def _load(self):
        try: self.devices=json.loads(self.state_path.read_text()).get('devices',{})
        except Exception: self.devices={}
    def persist(self):
        tmp=self.state_path.with_suffix('.tmp'); tmp.write_text(json.dumps({'devices':self.devices},indent=2)); tmp.replace(self.state_path)
    def publish(self,topic,payload,retain=False):
        m=self.cfg.mqtt; cmd=['mosquitto_pub','-h',m['host'],'-p',str(m['port']),'-t',topic,'-m',json.dumps(payload,separators=(',',':'))]
        if retain: cmd.append('-r')
        subprocess.run(cmd,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL,check=False)
    def add_event(self,kind,device_id,severity,detail):
        ev={'schema':'netcore-event-v1','event_id':str(uuid.uuid4()),'event_type':kind,'source':{'service':'netcore-hardware-gateway','instance':os.uname().nodename},'timestamp':now(),'severity':severity,'subject':{'type':'hardware_device','id':device_id},'payload':detail}
        with self.lock:
            self.events.append(ev); self.events=self.events[-500:]
            with self.event_path.open('a') as f: f.write(json.dumps(ev,separators=(',',':'))+'\n')
        self.publish(f"{self.cfg.mqtt['topic_prefix']}/events/{kind.replace('.','/')}",ev)
    def evaluate(self,device_id,metrics):
        alarms=[]
        for t in self.cfg.thresholds:
            metric=t.get('metric'); val=metrics.get(metric)
            if not isinstance(val,(int,float)): continue
            sev=None; reason=None
            if 'critical_above' in t and val>=t['critical_above']: sev='critical'; reason='critical_above'
            elif 'critical_below' in t and val<=t['critical_below']: sev='critical'; reason='critical_below'
            elif 'warning_above' in t and val>=t['warning_above']: sev='warning'; reason='warning_above'
            elif 'warning_below' in t and val<=t['warning_below']: sev='warning'; reason='warning_below'
            if sev: alarms.append({'metric':metric,'value':val,'severity':sev,'reason':reason})
        return alarms
    def ingest(self,payload,source='http'):
        did=safe_id(str(payload.get('device_id','')))
        if not did: raise ValueError('device_id fehlt')
        metrics=payload.get('metrics',{}); inputs=payload.get('inputs',{}); outputs=payload.get('outputs',{})
        if not isinstance(metrics,dict) or not isinstance(inputs,dict) or not isinstance(outputs,dict): raise ValueError('metrics/inputs/outputs müssen Objekte sein')
        ts=now(); alarms=self.evaluate(did,metrics)
        with self.lock:
            old=self.devices.get(did); self.devices[did]={'device_id':did,'name':payload.get('name',did),'kind':payload.get('kind','edge_io'),'location':payload.get('location'),'firmware':payload.get('firmware'),'last_seen':ts,'online':True,'source':source,'metrics':metrics,'inputs':inputs,'outputs':outputs,'alarms':alarms}
            self.persist()
        if old is None: self.add_event('hardware.device_registered',did,'info',{'kind':payload.get('kind','edge_io')})
        self.publish(f"{self.cfg.mqtt['topic_prefix']}/state/hardware/{did}",self.devices[did],True)
        for alarm in alarms: self.add_event('hardware.threshold_exceeded',did,alarm['severity'],alarm)
        return self.devices[did]
    def watchdog(self):
        while not self.stop:
            cutoff=time.time()-int(self.cfg.mon['heartbeat_timeout_secs'])
            changed=[]
            with self.lock:
                for did,d in self.devices.items():
                    try: epoch=datetime.fromisoformat(d['last_seen'].replace('Z','+00:00')).timestamp()
                    except Exception: epoch=0
                    online=epoch>=cutoff
                    if d.get('online') and not online:
                        d['online']=False; changed.append(did)
                if changed: self.persist()
            for did in changed:
                self.add_event('hardware.device_offline',did,'warning',{'heartbeat_timeout_secs':self.cfg.mon['heartbeat_timeout_secs']})
                self.publish(f"{self.cfg.mqtt['topic_prefix']}/state/hardware/{did}",self.devices[did],True)
            time.sleep(2)
    def prometheus(self):
        lines=['# HELP netcore_hardware_device_up Hardware device heartbeat state','# TYPE netcore_hardware_device_up gauge']
        with self.lock: devices=list(self.devices.values())
        for d in devices:
            did=str(d.get('device_id','unknown')).replace('"','')
            lines.append(f'netcore_hardware_device_up{{device_id="{did}"}} {1 if d.get("online") else 0}')
            for key,value in d.get('metrics',{}).items():
                if isinstance(value,(int,float)) and not isinstance(value,bool):
                    metric=''.join(c if c.isalnum() or c=='_' else '_' for c in str(key))
                    lines.append(f'netcore_hardware_{metric}{{device_id="{did}"}} {value}')
        return '\n'.join(lines)+'\n'
    def mqtt_loop(self):
        m=self.cfg.mqtt; topic=f"{m['topic_prefix']}/hardware/+/telemetry"
        while not self.stop:
            p=subprocess.Popen(['mosquitto_sub','-h',m['host'],'-p',str(m['port']),'-v','-t',topic],stdout=subprocess.PIPE,stderr=subprocess.DEVNULL,text=True)
            while not self.stop and p.poll() is None:
                line=p.stdout.readline()
                if not line: break
                try:
                    t,msg=line.rstrip().split(' ',1); self.ingest(json.loads(msg),f'mqtt:{t}')
                except Exception: pass
            p.terminate(); time.sleep(2)

HTML='''<!doctype html><html><head><meta charset="utf-8"><title>NetCore Hardware Gateway</title><style>body{font-family:system-ui;background:#10151d;color:#eef;margin:2rem}table{border-collapse:collapse;width:100%}td,th{padding:.6rem;border-bottom:1px solid #344}code{background:#253040;padding:.15rem .3rem}.ok{color:#6fda8a}.bad{color:#ff7b7b}</style></head><body><h1>NetCore Hardware Gateway · OPEN LAB</h1><p>Hardware-I/O, Rack- und Umgebungsüberwachung. Ausgänge sind standardmäßig deaktiviert.</p><div id="s"></div><h2>Geräte</h2><table><thead><tr><th>ID</th><th>Name</th><th>Status</th><th>Letztes Signal</th><th>Messwerte</th><th>Alarme</th></tr></thead><tbody id="d"></tbody></table><script>async function r(){let s=await(await fetch('/api/v1/status')).json(),ds=await(await fetch('/api/v1/devices')).json();document.querySelector('#s').innerHTML=`Phase ${s.phase} · MQTT ${s.mqtt_host}:${s.mqtt_port} · ${s.devices_online}/${s.devices_total} online`;document.querySelector('#d').innerHTML=ds.map(x=>`<tr><td><code>${x.device_id}</code></td><td>${x.name}</td><td class="${x.online?'ok':'bad'}">${x.online?'online':'offline'}</td><td>${x.last_seen}</td><td><code>${JSON.stringify(x.metrics)}</code></td><td>${x.alarms.length}</td></tr>`).join('')}r();setInterval(r,3000)</script></body></html>'''

class H(BaseHTTPRequestHandler):
    app=None
    def sendj(self,obj,code=200):
        b=json.dumps(obj).encode(); self.send_response(code); self.send_header('Content-Type','application/json'); self.send_header('Content-Length',str(len(b))); self.end_headers(); self.wfile.write(b)
    def do_GET(self):
        p=urlparse(self.path).path
        if p=='/':
            b=HTML.encode(); self.send_response(200); self.send_header('Content-Type','text/html; charset=utf-8'); self.send_header('Content-Length',str(len(b))); self.end_headers(); self.wfile.write(b); return
        if p in ('/health/live','/health/ready'): return self.sendj({'status':'ok','mode':'open_lab'})
        if p=='/metrics':
            b=self.app.prometheus().encode(); self.send_response(200); self.send_header('Content-Type','text/plain; version=0.0.4; charset=utf-8'); self.send_header('Content-Length',str(len(b))); self.end_headers(); self.wfile.write(b); return
        if p=='/openapi.json': return self.sendj({'openapi':'3.0.3','info':{'title':'NetCore Hardware Gateway','version':'1.0.0-openlab'},'paths':{'/api/v1/status':{'get':{}},'/api/v1/devices':{'get':{}},'/api/v1/events':{'get':{}},'/api/v1/telemetry':{'post':{}},'/health/live':{'get':{}},'/health/ready':{'get':{}},'/metrics':{'get':{}}}})
        if p=='/api/v1/status':
            with self.app.lock:
                ds=list(self.app.devices.values())
            return self.sendj({'service':'netcore-hardware-gateway','phase':6,'security_mode':'open_lab','started_at':self.app.started,'mqtt_host':self.app.cfg.mqtt['host'],'mqtt_port':self.app.cfg.mqtt['port'],'devices_total':len(ds),'devices_online':sum(1 for d in ds if d.get('online')),'outputs_enabled':self.app.cfg.mon['outputs_enabled'],'warning':'OPEN LAB: keine Anmeldung, kein TLS'})
        if p=='/api/v1/devices':
            with self.app.lock: return self.sendj(list(self.app.devices.values()))
        if p=='/api/v1/events':
            with self.app.lock: return self.sendj(self.app.events[-100:])
        self.sendj({'error':'not_found'},404)
    def do_POST(self):
        p=urlparse(self.path).path
        if p!='/api/v1/telemetry': return self.sendj({'error':'not_found'},404)
        try:
            n=int(self.headers.get('Content-Length','0')); data=json.loads(self.rfile.read(n)); self.sendj(self.app.ingest(data,'http'),202)
        except Exception as e: self.sendj({'error':str(e)},400)
    def log_message(self,*a): pass

def main():
    ap=argparse.ArgumentParser(); ap.add_argument('--config',default='/etc/netcore/hardware-gateway.toml'); a=ap.parse_args()
    with open(a.config,'rb') as f: cfg=Cfg(tomllib.load(f))
    if cfg.raw['security']['mode']!='open_lab': raise SystemExit('Nur open_lab wird unterstützt')
    app=App(cfg); H.app=app
    threading.Thread(target=app.watchdog,daemon=True).start(); threading.Thread(target=app.mqtt_loop,daemon=True).start()
    host,port=cfg.bind; srv=ThreadingHTTPServer((host,port),H)
    signal.signal(signal.SIGTERM,lambda *_:(setattr(app,'stop',True),srv.shutdown()))
    srv.serve_forever()
if __name__=='__main__': main()
