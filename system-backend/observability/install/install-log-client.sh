#!/usr/bin/env bash
# Run separately on each emitting LXC, VM or Pi. Does not restart radio services.
set -euo pipefail
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
[[ ${EUID} -eq 0 ]] || { echo "Run as root" >&2; exit 1; }
export DEBIAN_FRONTEND=noninteractive
apt-get update
apt-get install -y python3 rsyslog rsyslog-relp
source "${DIR}/install/rsyslog-apparmor.sh"
install -d -m 0755 /etc/netcore /opt/netcore-log-client /etc/systemd/journald.conf.d
install -d -m 0700 /var/lib/netcore-log-client
install -m 0644 "${DIR}/logging/log_client.py" /opt/netcore-log-client/log_client.py
if [[ ! -e /etc/netcore/log-client.json ]]; then
  install -m 0600 "${DIR}/config/log-client.example.json" /etc/netcore/log-client.json
fi
for unit in netcore-log-client.service netcore-log-client-discovery.service netcore-log-client-discovery.timer; do
  install -m 0644 "${DIR}/systemd/${unit}" "/etc/systemd/system/${unit}"
done
cat > /etc/systemd/journald.conf.d/60-netcore-limits.conf <<'EOF'
# Limits for the source host. A custom smaller limit should use a later drop-in.
[Journal]
SystemMaxUse=256M
SystemKeepFree=512M
SystemMaxFileSize=32M
RuntimeMaxUse=64M
RuntimeKeepFree=64M
EOF
systemctl daemon-reload
python3 /opt/netcore-log-client/log_client.py
systemctl restart systemd-journald.service
systemctl enable netcore-log-client.service netcore-log-client-discovery.timer
systemctl restart netcore-log-client.service netcore-log-client-discovery.timer
echo "Journal forwarding enabled. Existing file-only logs require an explicit imfile input."
