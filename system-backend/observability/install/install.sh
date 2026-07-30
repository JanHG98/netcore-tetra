#!/usr/bin/env bash
# NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Metriken, Protokolle und Betriebsüberwachung.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
PREFIX="${PREFIX:-/opt/netcore-observability}"
CONFIG="${CONFIG:-/etc/netcore/observability.toml}"
SERVICE="${SERVICE:-/etc/systemd/system/netcore-observability.service}"
[[ ${EUID} -eq 0 ]] || { echo "install.sh must run as root" >&2; exit 1; }
id -u netcore-observability >/dev/null 2>&1 || useradd --system --home /var/lib/netcore-observability --shell /usr/sbin/nologin netcore-observability
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -d -o netcore-observability -g netcore-observability -m 0750 /var/lib/netcore-observability /var/lib/netcore-observability/diagnostics
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -d -o root -g root -m 0755 "${PREFIX}/bin" "${PREFIX}/stack" "$(dirname "${CONFIG}")"
# Was: Baut oder prüft die Rust-Komponenten.
# Warum: So wird vor Installation oder Start sichergestellt, dass der Quellcode technisch verwendbar ist.
cargo build --release --package netcore-observability --manifest-path "${ROOT}/Cargo.toml"
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -o root -g root -m 0755 "${ROOT}/target/release/netcore-observability" "${PREFIX}/bin/netcore-observability"
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -o root -g root -m 0644 "${ROOT}/system-backend/observability/README.md" "${PREFIX}/README.md"
# Was: Kopiert Dateien an ihren vorgesehenen Zielort.
# Warum: Dienstdateien und Konfigurationen müssen dort liegen, wo Betriebssystem oder Anwendung sie erwarten.
cp -a "${ROOT}/system-backend/observability/stack/." "${PREFIX}/stack/"
# Was: Prüft die folgende Voraussetzung und führt den passenden Zweig aus.
# Warum: Fehlende Rechte, Dateien oder Einstellungen sollen früh und verständlich behandelt werden.
if [[ ! -e "${CONFIG}" ]]; then install -o root -g netcore-observability -m 0640 "${ROOT}/system-backend/observability/config/observability.example.toml" "${CONFIG}"; fi
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -o root -g root -m 0644 "${ROOT}/system-backend/observability/systemd/netcore-observability.service" "${SERVICE}"
source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../../shared/install" && pwd)/lxc-network.sh"
netcore_configure_lxc_endpoint "${CONFIG}" "observability" "8210"
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl daemon-reload
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl enable --now netcore-observability.service
echo "OPEN LAB: no login, no tokens and no TLS. Isolated management network only."
