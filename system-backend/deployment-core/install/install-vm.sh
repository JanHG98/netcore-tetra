#!/usr/bin/env bash
# Combined Ubuntu VM: controller + separate privileged image builder.
set -Eeuo pipefail
[[ $EUID -eq 0 ]] || { echo 'Bitte mit sudo/root ausführen.' >&2; exit 1; }
SOURCE=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
. /etc/os-release
case "${ID:-}:${VERSION_ID:-}" in
  ubuntu:24.04) QEMU_PACKAGES=(qemu-user-static binfmt-support) ;;
  # Since QEMU 9 the static interpreters live in qemu-user; registration uses systemd.
  ubuntu:26.04) QEMU_PACKAGES=(qemu-user qemu-user-binfmt) ;;
  *) echo "Ubuntu 24.04 oder 26.04 LTS erforderlich (Server oder Desktop); erkannt: ${PRETTY_NAME:-unbekannt}." >&2; exit 1 ;;
esac
if systemd-detect-virt --container --quiet; then
  echo 'Image-Builds benötigen eine vollständige VM; kein LXC/Container.' >&2; exit 1
fi
case $(uname -m) in x86_64|aarch64) ;; *) echo 'AMD64 oder ARM64 erforderlich.' >&2; exit 1 ;; esac
ADVERTISE_URL=
CONTROLLER_ARGS=()
while [[ $# -gt 0 ]]; do
  case "$1" in
    --advertise-url) ADVERTISE_URL=${2:?URL erforderlich}; shift 2 ;;
    --node-id) CONTROLLER_ARGS+=(--node-id "${2:?Node-ID erforderlich}"); shift 2 ;;
    *) echo "Unbekannte Option: $1" >&2; exit 1 ;;
  esac
done
if systemctl is-active --quiet netcore-image-builder.service; then
  python3 - <<'PY'
import sys
sys.path.insert(0,'/usr/local/lib/netcore-deployment')
from image_client import ImageClient
status=ImageClient().request('/status')
if any(j['status'] in ('running','queued') for j in status['jobs']):
    raise SystemExit('Image-Auftrag aktiv. VM-Update erst nach dessen Abschluss ausführen.')
PY
  systemctl stop netcore-image-builder.service
fi
apt-get update
DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
  python3 git curl ca-certificates openssl "${QEMU_PACKAGES[@]}" \
  xz-utils e2fsprogs fdisk util-linux udev qemu-guest-agent
if [[ $(uname -m) == x86_64 ]]; then
  if [[ $VERSION_ID == 24.04 ]]; then
    update-binfmts --enable qemu-aarch64
  else
    systemctl restart systemd-binfmt.service
  fi
fi
bash "$SOURCE/install/install.sh" controller "${CONTROLLER_ARGS[@]}"
if [[ -n $ADVERTISE_URL ]]; then
  python3 - "$ADVERTISE_URL" <<'PY'
import pathlib,sys
sys.path.insert(0,'/usr/local/lib/netcore-deployment')
from common import atomic_write,load_config,peer_url,toml_dump
path=pathlib.Path('/etc/netcore/deployment.toml');cfg=load_config(path)
cfg['advertise_url']=peer_url(sys.argv[1],cfg['allowed_networks'])
atomic_write(path,toml_dump(cfg),0o644)
PY
fi
install -d -m 0700 /var/lib/netcore-image-builder
install -m 0644 "$SOURCE/systemd/netcore-image-builder.service" /etc/systemd/system/
python3 - <<'PY'
import pathlib,sys
sys.path.insert(0,'/usr/local/lib/netcore-deployment')
from image_build import preflight
preflight(pathlib.Path('/var/lib/netcore-image-builder'))
PY
systemctl daemon-reload
systemctl enable --now netcore-image-builder.service
systemctl restart netcore-deployment.service
systemctl enable qemu-guest-agent.service 2>/dev/null || true
systemctl start qemu-guest-agent.service 2>/dev/null || true
echo 'NetCore Deployment-, Management- und Image-VM bereit.'
echo 'WebUI: http://<VM-IP>:8320 · OpenLab ohne TLS/Login.'
echo 'Im UI: Standort-TOML importieren, TBS-Profil anlegen, Pi-Image erstellen.'
