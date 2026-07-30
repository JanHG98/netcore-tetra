#!/usr/bin/env bash
# NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Aufzeichnung und Ereignisprotokollierung.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
CONFIG_SRC="$REPO_ROOT/system-backend/recorder/config/recorder.example.toml"
UNIT_SRC="$REPO_ROOT/system-backend/recorder/systemd/netcore-recorder.service"

# Was: Prüft die folgende Voraussetzung und führt den passenden Zweig aus.
# Warum: Fehlende Rechte, Dateien oder Einstellungen sollen früh und verständlich behandelt werden.
if [[ $EUID -ne 0 ]]; then
  echo "Bitte als root/sudo ausführen." >&2
  exit 1
fi

# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl stop netcore-recorder.service 2>/dev/null || true
# Was: Entfernt nicht mehr benötigte Dateien oder alte Zustände.
# Warum: Veraltete Reste könnten einen erneuten Start oder eine Neuinstallation verfälschen.
rm -f /usr/local/bin/netcore-recorder
# Was: Entfernt nicht mehr benötigte Dateien oder alte Zustände.
# Warum: Veraltete Reste könnten einen erneuten Start oder eine Neuinstallation verfälschen.
rm -rf "$REPO_ROOT/target"

getent group netcore >/dev/null || groupadd --system netcore
id -u netcore >/dev/null 2>&1 || useradd --system --gid netcore --home-dir /var/lib/netcore-recorder --shell /usr/sbin/nologin netcore
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -d -o netcore -g netcore /var/lib/netcore-recorder
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -d -o netcore -g netcore /var/lib/netcore-recorder/recordings
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -d -o netcore -g netcore /var/lib/netcore-recorder/exports
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -d /etc/netcore
# Was: Prüft die folgende Voraussetzung und führt den passenden Zweig aus.
# Warum: Fehlende Rechte, Dateien oder Einstellungen sollen früh und verständlich behandelt werden.
if [[ ! -f /etc/netcore/recorder.toml ]]; then
  # Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
  # Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
  install -m 0640 -o root -g netcore "$CONFIG_SRC" /etc/netcore/recorder.toml
fi

cd "$REPO_ROOT"
# Was: Baut oder prüft die Rust-Komponenten.
# Warum: So wird vor Installation oder Start sichergestellt, dass der Quellcode technisch verwendbar ist.
cargo clean
# Was: Baut oder prüft die Rust-Komponenten.
# Warum: So wird vor Installation oder Start sichergestellt, dass der Quellcode technisch verwendbar ist.
cargo build --release -p netcore-recorder
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -m 0755 target/release/netcore-recorder /usr/local/bin/netcore-recorder
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -m 0644 "$UNIT_SRC" /etc/systemd/system/netcore-recorder.service
source "${SCRIPT_DIR}/../../shared/install/lxc-network.sh"
netcore_configure_lxc_endpoint "/etc/netcore/recorder.toml" "recorder" "8140"
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl daemon-reload
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl enable --now netcore-recorder.service
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl --no-pager --full status netcore-recorder.service
