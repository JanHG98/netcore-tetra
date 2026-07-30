#!/usr/bin/env bash
# NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Anwendungsdienste wie TTS und externe Integrationen.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
PREFIX="${PREFIX:-/opt/netcore-application-gateway}"
CONFIG="${CONFIG:-/etc/netcore/application-gateway.toml}"
SERVICE="${SERVICE:-/etc/systemd/system/netcore-application-gateway.service}"
[[ ${EUID} -eq 0 ]] || { echo "install.sh must run as root" >&2; exit 1; }
id -u netcore-app-gateway >/dev/null 2>&1 || useradd --system --home /var/lib/netcore-application-gateway --shell /usr/sbin/nologin netcore-app-gateway
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -d -o netcore-app-gateway -g netcore-app-gateway -m 0750 /var/lib/netcore-application-gateway /var/lib/netcore-application-gateway/spool /var/lib/netcore-application-gateway/backups
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -d -o root -g root -m 0755 "${PREFIX}/bin" "$(dirname "${CONFIG}")"
# Was: Baut oder prüft die Rust-Komponenten.
# Warum: So wird vor Installation oder Start sichergestellt, dass der Quellcode technisch verwendbar ist.
cargo build --release --package netcore-application-gateway --manifest-path "${ROOT}/Cargo.toml"
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -o root -g root -m 0755 "${ROOT}/target/release/netcore-application-gateway" "${PREFIX}/bin/netcore-application-gateway"
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -o root -g root -m 0644 "${ROOT}/system-backend/application-gateway/README.md" "${PREFIX}/README.md"
# Was: Prüft die folgende Voraussetzung und führt den passenden Zweig aus.
# Warum: Fehlende Rechte, Dateien oder Einstellungen sollen früh und verständlich behandelt werden.
if [[ ! -e "${CONFIG}" ]]; then
  # Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
  # Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
  install -o root -g netcore-app-gateway -m 0640 "${ROOT}/system-backend/application-gateway/config/application-gateway.example.toml" "${CONFIG}"
fi
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -o root -g root -m 0644 "${ROOT}/system-backend/application-gateway/systemd/netcore-application-gateway.service" "${SERVICE}"
source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../../shared/install" && pwd)/lxc-network.sh"
netcore_configure_lxc_endpoint "${CONFIG}" "application-gateway" "8220"
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl daemon-reload
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl enable --now netcore-application-gateway.service
echo "OPEN LAB: no login, no management tokens and no TLS. Isolated management network only."
