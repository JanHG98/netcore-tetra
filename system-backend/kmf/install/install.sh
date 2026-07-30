#!/usr/bin/env bash
# NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Schlüsselverwaltung und Schlüsselverteilung.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
PACKAGE="$ROOT/system-backend/kmf"

# Was: Prüft die folgende Voraussetzung und führt den passenden Zweig aus.
# Warum: Fehlende Rechte, Dateien oder Einstellungen sollen früh und verständlich behandelt werden.
if [[ ${EUID:-$(id -u)} -ne 0 ]]; then
  echo "Bitte als root ausführen." >&2
  exit 1
fi

getent group netcore-kmf >/dev/null || groupadd --system netcore-kmf
id -u netcore-kmf >/dev/null 2>&1 || useradd --system --gid netcore-kmf --home /var/lib/netcore-kmf --shell /usr/sbin/nologin netcore-kmf
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -d -m 0700 -o netcore-kmf -g netcore-kmf /var/lib/netcore-kmf /var/lib/netcore-kmf/backups /var/lib/netcore-kmf/bootstrap
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -d -m 0750 /etc/netcore

# Was: Baut oder prüft die Rust-Komponenten.
# Warum: So wird vor Installation oder Start sichergestellt, dass der Quellcode technisch verwendbar ist.
cargo build --release --package netcore-kmf
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -m 0755 "$ROOT/target/release/netcore-kmf" /usr/local/bin/netcore-kmf
# Was: Prüft die folgende Voraussetzung und führt den passenden Zweig aus.
# Warum: Fehlende Rechte, Dateien oder Einstellungen sollen früh und verständlich behandelt werden.
if [[ ! -e /etc/netcore/kmf.toml ]]; then
  # Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
  # Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
  install -m 0640 -o root -g netcore-kmf "$PACKAGE/config/kmf.example.toml" /etc/netcore/kmf.toml
fi
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -m 0644 "$PACKAGE/systemd/netcore-kmf.service" /etc/systemd/system/netcore-kmf.service
source "${SCRIPT_DIR}/../../shared/install/lxc-network.sh"
netcore_configure_lxc_endpoint "/etc/netcore/kmf.toml" "kmf" "8190"
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl daemon-reload
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl enable --now netcore-kmf.service

echo "KMF läuft im OPEN-LAB-Modus auf Port 8190. Managementnetz strikt isolieren."
