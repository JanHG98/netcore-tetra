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
if [[ $ROLE == controller ]]; then
  install -d -m 0755 /usr/local/lib/netcore-deployment/image/systemd
  install -m 0644 "$SOURCE"/image/*.py "$SOURCE"/image/*.rules /usr/local/lib/netcore-deployment/image/
  install -m 0755 "$SOURCE"/image/*.sh /usr/local/lib/netcore-deployment/image/
  install -m 0644 "$SOURCE"/image/systemd/* /usr/local/lib/netcore-deployment/image/systemd/
fi
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
from common import CATALOG, atomic_write, identifier, peer_url, toml_dump
from deploy import service_spec, installed_service
p=argparse.ArgumentParser()
p.add_argument('config');p.add_argument('--seed');p.add_argument('--node-id');p.add_argument('--tbs-template')
p.add_argument('--tbs-unit');p.add_argument('--tbs-config');p.add_argument('--tbs-command')
p.add_argument('--managed-service',action='append',choices=[*CATALOG,'none'])
a=p.parse_args();file=pathlib.Path(a.config);cfg=tomllib.loads(file.read_text())
if a.node_id: cfg['node_id']=identifier(a.node_id)
if a.seed:
 seed=peer_url(a.seed,cfg['allowed_networks']);cfg['seeds']=list(dict.fromkeys(cfg.get('seeds',[])+[seed]))
if a.tbs_template: cfg['tbs_template']=a.tbs_template
if a.managed_service is not None:
 if 'none' in a.managed_service and len(a.managed_service)>1: p.error('none nicht mit Dienstnamen kombinieren')
 cfg['managed_services']=[] if a.managed_service==['none'] else list(dict.fromkeys(a.managed_service))
if cfg['role']=='agent' and not a.tbs_unit:
 spec=service_spec(dict(cfg,services=cfg.get('services',[])),'tbs')
 if installed_service(spec):
  print('Vorhandene TBS erkannt:',spec['unit'],spec['config_target'])
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
from deploy import service_spec,install_dropin,installed_service
cfg=load_config(sys.argv[1])
for name in cfg.get('managed_services',CATALOG):
 s=service_spec(cfg,name)
 # Existing radio units may use custom executables, flags and working dirs.
 # Preserve their ExecStart; fresh managed TBS installs get their own wrapper.
 if name!='tbs' and installed_service(s):
  install_dropin(s,pathlib.Path(cfg['state_dir'])/'endpoints.json')
PY
fi
systemctl daemon-reload
systemctl enable "$UNIT"
systemctl restart "$UNIT"
echo "NetCore $ROLE installiert. WebUI: HTTP-Port $([[ $ROLE == controller ]] && echo 8320 || echo 8321)."
