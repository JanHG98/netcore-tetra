#!/usr/bin/env bash
# NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für die Verbindung zwischen Basisstationen und Backend-Diensten.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Was: Prüft die folgende Voraussetzung und führt den passenden Zweig aus.
# Warum: Fehlende Rechte, Dateien oder Einstellungen sollen früh und verständlich behandelt werden.
if [[ ${EUID} -ne 0 ]]; then
  echo "Bitte als root ausführen." >&2
  exit 1
fi

REPO_ROOT=${REPO_ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)}
cd "$REPO_ROOT"

# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl stop netcore-node-gateway.service 2>/dev/null || true
# Was: Entfernt nicht mehr benötigte Dateien oder alte Zustände.
# Warum: Veraltete Reste könnten einen erneuten Start oder eine Neuinstallation verfälschen.
rm -f /usr/local/bin/netcore-node-gateway
# Was: Entfernt nicht mehr benötigte Dateien oder alte Zustände.
# Warum: Veraltete Reste könnten einen erneuten Start oder eine Neuinstallation verfälschen.
rm -rf target/release/netcore-node-gateway target/release/deps/netcore_node_gateway-*
# Was: Baut oder prüft die Rust-Komponenten.
# Warum: So wird vor Installation oder Start sichergestellt, dass der Quellcode technisch verwendbar ist.
cargo build --release -p netcore-node-gateway

getent group netcore >/dev/null || groupadd --system netcore
id netcore >/dev/null 2>&1 || useradd --system --gid netcore --home-dir /var/lib/netcore-node-gateway --shell /usr/sbin/nologin netcore
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -d -o netcore -g netcore /var/lib/netcore-node-gateway
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -d -m 0755 /etc/netcore
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -m 0755 target/release/netcore-node-gateway /usr/local/bin/netcore-node-gateway
# Was: Prüft die folgende Voraussetzung und führt den passenden Zweig aus.
# Warum: Fehlende Rechte, Dateien oder Einstellungen sollen früh und verständlich behandelt werden.
if [[ ! -f /etc/netcore/node-gateway.toml ]]; then
  # Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
  # Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
  install -m 0644 system-backend/node-gateway/config/node-gateway.example.toml /etc/netcore/node-gateway.toml
fi
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -m 0644 system-backend/node-gateway/systemd/netcore-node-gateway.service /etc/systemd/system/netcore-node-gateway.service
source "${SCRIPT_DIR}/../../shared/install/lxc-network.sh"
netcore_configure_lxc_endpoint "/etc/netcore/node-gateway.toml" "node-gateway" "8080"
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl daemon-reload
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl enable --now netcore-node-gateway.service
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl --no-pager --full status netcore-node-gateway.service

echo "WARNUNG: offener Testmodus ohne Authentifizierung oder Tokens."
