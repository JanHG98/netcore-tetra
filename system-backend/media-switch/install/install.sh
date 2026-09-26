#!/usr/bin/env bash
# NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für die Audioverteilung zwischen Basisstationen und Rufen.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
CONFIG_SRC="$REPO_ROOT/system-backend/media-switch/config/media-switch.example.toml"
UNIT_SRC="$REPO_ROOT/system-backend/media-switch/systemd/netcore-media-switch.service"

# Was: Prüft die folgende Voraussetzung und führt den passenden Zweig aus.
# Warum: Fehlende Rechte, Dateien oder Einstellungen sollen früh und verständlich behandelt werden.
if [[ $EUID -ne 0 ]]; then
  echo "Bitte als root/sudo ausführen." >&2
  exit 1
fi

# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl stop netcore-media-switch.service 2>/dev/null || true
# Was: Entfernt nicht mehr benötigte Dateien oder alte Zustände.
# Warum: Veraltete Reste könnten einen erneuten Start oder eine Neuinstallation verfälschen.
rm -f /usr/local/bin/netcore-media-switch
# Was: Entfernt nicht mehr benötigte Dateien oder alte Zustände.
# Warum: Veraltete Reste könnten einen erneuten Start oder eine Neuinstallation verfälschen.
rm -rf "$REPO_ROOT/target"

getent group netcore >/dev/null || groupadd --system netcore
id -u netcore >/dev/null 2>&1 || useradd --system --gid netcore --home-dir /var/lib/netcore-media-switch --shell /usr/sbin/nologin netcore
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -d -o netcore -g netcore /var/lib/netcore-media-switch
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -d /etc/netcore
# Was: Prüft die folgende Voraussetzung und führt den passenden Zweig aus.
# Warum: Fehlende Rechte, Dateien oder Einstellungen sollen früh und verständlich behandelt werden.
if [[ ! -f /etc/netcore/media-switch.toml ]]; then
  # Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
  # Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
  install -m 0640 -o root -g netcore "$CONFIG_SRC" /etc/netcore/media-switch.toml
fi

cd "$REPO_ROOT"
# Was: Baut oder prüft die Rust-Komponenten.
# Warum: So wird vor Installation oder Start sichergestellt, dass der Quellcode technisch verwendbar ist.
cargo clean
# Was: Baut oder prüft die Rust-Komponenten.
# Warum: So wird vor Installation oder Start sichergestellt, dass der Quellcode technisch verwendbar ist.
cargo build --release -p netcore-media-switch
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -m 0755 target/release/netcore-media-switch /usr/local/bin/netcore-media-switch
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -m 0644 "$UNIT_SRC" /etc/systemd/system/netcore-media-switch.service
source "${SCRIPT_DIR}/../../shared/install/lxc-network.sh"
netcore_configure_lxc_endpoint "/etc/netcore/media-switch.toml" "media-switch" "8130"
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl daemon-reload
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl enable --now netcore-media-switch.service
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl --no-pager --full status netcore-media-switch.service
