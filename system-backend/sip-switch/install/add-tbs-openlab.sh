#!/usr/bin/env bash
set -euo pipefail
if [[ $# -lt 3 ]]; then
  echo "Usage: $0 <NODE-ID> <SIP-USERNAME> <SIP-PASSWORD> [ENDPOINT-ID]" >&2
  exit 2
fi
NODE_ID="$1"
USERNAME="$2"
PASSWORD="$3"
ENDPOINT_ID="${4:-tbs-$(printf '%s' "$NODE_ID" | tr '[:upper:]' '[:lower:]' | sed 's/[^a-z0-9_.:-]/-/g')}"
CONFIG=/etc/netcore/sip-switch.toml
if grep -Eq "^node_id[[:space:]]*=[[:space:]]*\"${NODE_ID//./\.}\"" "$CONFIG"; then
  echo "TBS ${NODE_ID} ist bereits in ${CONFIG} vorhanden." >&2
  exit 1
fi
cat >>"$CONFIG" <<EOF

[[tbs]]
node_id = "${NODE_ID}"
endpoint_id = "${ENDPOINT_ID}"
username = "${USERNAME}"
password = "${PASSWORD}"
enabled = true
max_contacts = 1
aliases = []
EOF
/usr/local/bin/netcore-sip-switch --config "$CONFIG" --render-asterisk
systemctl restart asterisk.service
systemctl restart netcore-sip-switch.service
printf 'TBS %s als Endpoint %s angelegt.\n' "$NODE_ID" "$ENDPOINT_ID"
