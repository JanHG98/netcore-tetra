#!/usr/bin/env bash
set -Eeuo pipefail
[[ $EUID -eq 0 ]] || { echo 'Bitte als root ausführen.' >&2; exit 1; }
: "${REPO_ROOT:?}" "${CONFIG_PATH:?}" "${UNIT:?}"
[[ -f "$CONFIG_PATH" ]] || { echo 'TBS-Konfiguration fehlt' >&2; exit 1; }
cd "$REPO_ROOT"
# SDR drivers are hardware-specific and must already be installed on the Pi.
SDR_CHECK=$(mktemp)
trap 'rm -f "$SDR_CHECK"' EXIT
SoapySDRUtil --find 2>&1 | tee "$SDR_CHECK"
if grep -q 'No devices found' "$SDR_CHECK"; then
  echo 'Kein SoapySDR-Gerät gefunden. SDR/HAT-Treiber auf dem Pi installieren.' >&2
  exit 1
fi
cargo build --locked --release -p bluestation-bs
install -m 0755 target/release/bluestation-bs /usr/local/bin/bluestation-bs
[[ -f ${CONFIG_PATH}.fallback ]] || install -m 0600 "$CONFIG_PATH" "${CONFIG_PATH}.fallback"
if ! systemctl cat "$UNIT" >/dev/null 2>&1; then
  cat >"/etc/systemd/system/$UNIT" <<'UNITFILE'
[Unit]
Description=NetCore TETRA Base Station
After=network-online.target
Wants=network-online.target
[Service]
Type=simple
WorkingDirectory=/var/lib/netcore-tbs
ExecStart=/usr/local/bin/bluestation-bs /etc/netcore/config.toml
StateDirectory=netcore-tbs
Restart=on-failure
RestartSec=5
[Install]
WantedBy=multi-user.target
UNITFILE
fi
systemctl daemon-reload
systemctl enable --now "$UNIT"
