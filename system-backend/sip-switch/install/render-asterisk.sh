#!/usr/bin/env bash
set -euo pipefail
/usr/local/bin/netcore-sip-switch --config /etc/netcore/sip-switch.toml --render-asterisk
systemctl restart asterisk.service
systemctl restart netcore-sip-switch.service
