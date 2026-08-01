#!/usr/bin/env bash
set -euo pipefail
if [[ $# -lt 4 ]]; then
  echo "Usage: $0 <SIP-SWITCH-IP> <TBS-USERNAME> <TBS-PASSWORD> <TBS-LOCAL-IP> [bind-port]" >&2
  exit 2
fi
SWITCH_IP="$1"
USER="$2"
PASSWORD="$3"
LOCAL_IP="$4"
BIND_PORT="${5:-5062}"
cat <<EOF
[asterisk]
enabled = true
outbound_prefix = "91*"
strip_outbound_prefix = true
inbound_prefix = ""
register = true
codec = "PCMU"
service_numbers = ["*"]
rtp_port_min = 30000
rtp_port_max = 30100
bind_addr = "0.0.0.0"
bind_port = ${BIND_PORT}
remote_host = "${SWITCH_IP}"
remote_port = 5060
contact_host = "${LOCAL_IP}"
from_domain = "${SWITCH_IP}"
local_user = "${USER}"
auth_user = "${USER}"
password = "${PASSWORD}"
realm = "asterisk"
options_interval_secs = 30
inbound_setup_timeout_secs = 30
EOF
