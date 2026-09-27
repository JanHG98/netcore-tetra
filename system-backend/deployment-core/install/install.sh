#!/usr/bin/env bash
set -Eeuo pipefail
[[ $EUID -eq 0 ]] || { echo 'Bitte als root ausführen.' >&2; exit 1; }
SOURCE=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
ROLE=${1:-controller}
[[ $ROLE == controller || $ROLE == agent ]] || { echo 'Rolle: controller oder agent' >&2; exit 1; }
if [[ $# -gt 0 ]]; then shift; fi
apt-get update
DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends python3 git curl ca-certificates iproute2
python3 -c 'import sys; assert sys.version_info >= (3,11), "Python >= 3.11 erforderlich (Debian 12+)"'
install -d -m 0755 /usr/local/lib/netcore-deployment/{static,install} /etc/netcore
install -m 0644 "$SOURCE"/*.py "$SOURCE/catalog.json" /usr/local/lib/netcore-deployment/
install -m 0644 "$SOURCE"/static/* /usr/local/lib/netcore-deployment/static/
install -m 0755 "$SOURCE"/install/prepare-host.sh "$SOURCE"/install/install-tbs.sh /usr/local/lib/netcore-deployment/install/
if [[ $ROLE == controller ]]; then
  CONFIG=/etc/netcore/deployment.toml
  UNIT=netcore-deployment.service
  getent group netcore-deploy >/dev/null || groupadd --system netcore-deploy
  id netcore-deploy >/dev/null 2>&1 || useradd --system --gid netcore-deploy --home-dir /var/lib/netcore-deployment --shell /usr/sbin/nologin netcore-deploy
  install -d -m 0750 -o netcore-deploy -g netcore-deploy /var/lib/netcore-deployment
  [[ -f $CONFIG ]] || install -m 0644 "$SOURCE/config/deployment.example.toml" "$CONFIG"
else
  CONFIG=/etc/netcore/discovery.toml
  UNIT=netcore-discovery.service
  install -d -m 0755 /var/lib/netcore-discovery
  [[ -f $CONFIG ]] || install -m 0644 "$SOURCE/config/agent.example.toml" "$CONFIG"
fi
python3 - "$CONFIG" "$@" <<'PY'
import argparse, pathlib, sys, tomllib
sys.path.insert(0, '/usr/local/lib/netcore-deployment')
from common import atomic_write, identifier, peer_url, toml_dump
p=argparse.ArgumentParser()
p.add_argument('config');p.add_argument('--seed');p.add_argument('--node-id');p.add_argument('--tbs-template')
p.add_argument('--tbs-unit');p.add_argument('--tbs-config');p.add_argument('--tbs-command')
a=p.parse_args();file=pathlib.Path(a.config);cfg=tomllib.loads(file.read_text())
if a.node_id: cfg['node_id']=identifier(a.node_id)
if a.seed:
 seed=peer_url(a.seed,cfg['allowed_networks']);cfg['seeds']=list(dict.fromkeys(cfg.get('seeds',[])+[seed]))
if a.tbs_template: cfg['tbs_template']=a.tbs_template
if cfg['role']=='agent' and not any(s['name']=='tbs' for s in cfg.get('services',[])) and not a.tbs_unit:
 import re,subprocess
 for candidate in ['/etc/netcore/config.toml','/opt/tetra/config.toml','/etc/flowstation/config.toml']:
  path=pathlib.Path(candidate)
  if not path.is_file(): continue
  original=tomllib.loads(path.read_text())
  unit=original.get('service_name','tetra')
  if not unit.endswith('.service'):unit+='.service'
  identifier(unit)
  result=subprocess.run(['systemctl','cat',unit],capture_output=True,text=True)
  commands=[line.split('=',1)[1] for line in result.stdout.splitlines() if line.startswith('ExecStart=') and 'bluestation-bs' in line]
  if result.returncode==0 and commands:
   cfg.setdefault('services',[]).append(dict(name='tbs',unit=unit,config_target=candidate,command=commands[-1],port=original.get('dashboard',{}).get('port',8080)))
   print('Vorhandene TBS erkannt:',unit,candidate)
   break
if any((a.tbs_unit,a.tbs_config,a.tbs_command)):
 if not all((a.tbs_unit,a.tbs_config,a.tbs_command)): p.error('Alle drei --tbs-* Overrides zusammen angeben')
 identifier(a.tbs_unit)
 cfg['services']=[s for s in cfg.get('services',[]) if s['name']!='tbs']
 cfg['services'].append(dict(name='tbs',unit=a.tbs_unit,config_target=a.tbs_config,command=a.tbs_command))
atomic_write(file,toml_dump(cfg),0o644)
PY
install -m 0644 "$SOURCE/systemd/$UNIT" "/etc/systemd/system/$UNIT"
if [[ $ROLE == agent ]]; then
  # Prepare launch-time bindings without restarting any running radio service.
  python3 - "$CONFIG" <<'PY'
import pathlib,sys
sys.path.insert(0,'/usr/local/lib/netcore-deployment')
from common import CATALOG,load_config
from deploy import service_spec,install_dropin
cfg=load_config(sys.argv[1])
for name in CATALOG:
 s=service_spec(cfg,name)
 if pathlib.Path(s['config_target']).is_file(): install_dropin(s,pathlib.Path(cfg['state_dir'])/'endpoints.json')
PY
fi
systemctl daemon-reload
systemctl enable "$UNIT"
systemctl restart "$UNIT"
echo "NetCore $ROLE installiert. WebUI: HTTP-Port $([[ $ROLE == controller ]] && echo 8320 || echo 8321)."
