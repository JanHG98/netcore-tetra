#!/usr/bin/env bash
# NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Gruppen, Mitgliedschaften und Gruppenrichtlinien.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
[[ ${EUID} -eq 0 ]] || { echo "Bitte als root ausführen." >&2; exit 1; }
cd "${SCRIPT_DIR}/../../.."
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl stop netcore-group-core.service 2>/dev/null || true
# Was: Entfernt nicht mehr benötigte Dateien oder alte Zustände.
# Warum: Veraltete Reste könnten einen erneuten Start oder eine Neuinstallation verfälschen.
rm -f /usr/local/bin/netcore-group-core
# Was: Entfernt nicht mehr benötigte Dateien oder alte Zustände.
# Warum: Veraltete Reste könnten einen erneuten Start oder eine Neuinstallation verfälschen.
rm -rf target/release/netcore-group-core target/release/deps/netcore_group_core-* target/release/.fingerprint/netcore-group-core-*
# Was: Baut oder prüft die Rust-Komponenten.
# Warum: So wird vor Installation oder Start sichergestellt, dass der Quellcode technisch verwendbar ist.
cargo clean -p netcore-group-core 2>/dev/null || true
# Was: Baut oder prüft die Rust-Komponenten.
# Warum: So wird vor Installation oder Start sichergestellt, dass der Quellcode technisch verwendbar ist.
cargo build --release -p netcore-group-core
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -o root -g root -m 0755 target/release/netcore-group-core /usr/local/bin/netcore-group-core
source "${SCRIPT_DIR}/../../shared/install/lxc-network.sh"
netcore_configure_lxc_endpoint "/etc/netcore/group-core.toml" "group-core" "8110"
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl restart netcore-group-core.service
