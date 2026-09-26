#!/usr/bin/env bash
# NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Verbindungen zu anderen Netzen und Systemen.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
PREFIX="${PREFIX:-/opt/netcore-transit}"

# Was: Prüft die folgende Voraussetzung und führt den passenden Zweig aus.
# Warum: Fehlende Rechte, Dateien oder Einstellungen sollen früh und verständlich behandelt werden.
if [[ ${EUID} -ne 0 ]]; then
  echo "update.sh must run as root" >&2
  exit 1
fi

# Was: Baut oder prüft die Rust-Komponenten.
# Warum: So wird vor Installation oder Start sichergestellt, dass der Quellcode technisch verwendbar ist.
cargo build --release --package netcore-transit --manifest-path "${ROOT}/Cargo.toml"
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -o root -g root -m 0755 "${ROOT}/target/release/netcore-transit" "${PREFIX}/bin/netcore-transit"
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -o root -g root -m 0644 "${ROOT}/system-backend/transit/README.md" "${PREFIX}/README.md"
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -o root -g root -m 0644 "${ROOT}/system-backend/transit/systemd/netcore-transit.service" /etc/systemd/system/netcore-transit.service
source "${SCRIPT_DIR}/../../shared/install/lxc-network.sh"
netcore_configure_lxc_endpoint "/etc/netcore/transit.toml" "transit" "8200"
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl daemon-reload
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl restart netcore-transit.service
