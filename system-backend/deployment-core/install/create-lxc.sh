#!/usr/bin/env bash
# Run on the Proxmox host. Never deletes or overwrites an existing CT.
set -Eeuo pipefail
CTID=${CTID:?Set CTID to an unused container ID}
TEMPLATE=${TEMPLATE:?Set TEMPLATE, e.g. local:vztmpl/debian-12-standard_...tar.zst}
STORAGE=${STORAGE:-local-lvm}
BRIDGE=${BRIDGE:-vmbr0}
IP=${IP:-dhcp}
REF=${REF:-main}
REPOSITORY=${REPOSITORY:-https://github.com/JanHG98/netcore-tetra.git}
[[ $EUID -eq 0 ]] || { echo 'Bitte auf dem Proxmox-Host als root ausführen.' >&2; exit 1; }
[[ $CTID =~ ^[1-9][0-9]+$ ]] || { echo 'Ungültige CTID' >&2; exit 1; }
command -v pct >/dev/null
if pct config "$CTID" >/dev/null 2>&1 || [[ -e /etc/pve/qemu-server/$CTID.conf ]]; then
  echo "ID $CTID ist bereits belegt. Abbruch ohne Änderung." >&2; exit 1
fi
NET="name=eth0,bridge=$BRIDGE,ip=$IP"
[[ -z ${GATEWAY:-} ]] || NET+=",gw=$GATEWAY"
[[ -z ${VLAN_TAG:-} ]] || NET+=",tag=$VLAN_TAG"
pct create "$CTID" "$TEMPLATE" --hostname netcore-deployment \
  --unprivileged 1 --cores 2 --memory 2048 --swap 512 --rootfs "$STORAGE:16" \
  --net0 "$NET" --onboot 1
pct start "$CTID"
for attempt in {1..30}; do
  if pct exec "$CTID" -- getent hosts github.com >/dev/null 2>&1; then break; fi
  sleep 2
done
pct exec "$CTID" -- bash -s -- "$REPOSITORY" "$REF" <<'INSTALL'
set -Eeuo pipefail
apt-get update
DEBIAN_FRONTEND=noninteractive apt-get install -y git ca-certificates python3
git clone "$1" /opt/netcore-tetra
cd /opt/netcore-tetra
git checkout "$2"
bash system-backend/deployment-core/install/install.sh controller
INSTALL
echo 'Deployment-LXC installiert. WebUI: http://<LXC-IP>:8320'
pct exec "$CTID" -- hostname -I
