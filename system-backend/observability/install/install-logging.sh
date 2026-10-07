#!/usr/bin/env bash
set -euo pipefail
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
[[ ${EUID} -eq 0 ]] || { echo "Run as root" >&2; exit 1; }
[[ "${PREFIX:-/opt/netcore-observability}" == /opt/netcore-observability ]] || {
  echo "Logging units require PREFIX=/opt/netcore-observability" >&2; exit 1;
}
export DEBIAN_FRONTEND=noninteractive
apt-get update
apt-get install -y python3 rsyslog rsyslog-relp
source "${DIR}/install/rsyslog-apparmor.sh"
python3 -c 'import tomllib' || { echo "Python 3.11+ required" >&2; exit 1; }
install -d -m 0755 /opt/netcore-observability/logging /etc/netcore
install -d -o netcore-observability -g netcore-observability -m 0750 \
  /var/lib/netcore-observability/logs /var/lib/netcore-observability/logs/queue
install -m 0644 "${DIR}/logging/log_store.py" /opt/netcore-observability/logging/log_store.py
install -m 0644 "${DIR}/logging/receiver.rsyslog.conf" /etc/netcore/receiver.rsyslog.conf
if [[ ! -e /etc/netcore/syslog.json ]]; then
  install -m 0640 -g netcore-observability "${DIR}/config/syslog.example.json" /etc/netcore/syslog.json
fi
if [[ ! -e /etc/netcore/openlab-hosts.json ]]; then
  install -m 0644 "${DIR}/config/openlab-hosts.json" /etc/netcore/openlab-hosts.json
fi
python3 /opt/netcore-observability/logging/log_store.py check
rsyslogd -N1 -f /etc/netcore/receiver.rsyslog.conf
for unit in netcore-syslog.service netcore-syslog-preview.service netcore-syslog-archive.service netcore-syslog-archive.timer; do
  install -m 0644 "${DIR}/systemd/${unit}" "/etc/systemd/system/${unit}"
done
systemctl daemon-reload
systemctl enable netcore-syslog.service netcore-syslog-preview.service netcore-syslog-archive.timer
systemctl restart netcore-syslog.service netcore-syslog-preview.service netcore-syslog-archive.timer
echo "Syslog ready: TCP/UDP 514, RELP 20514. Configure share permissions in docs/syslog-update.md."
