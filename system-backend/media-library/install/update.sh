#!/usr/bin/env bash
# NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für gespeicherte Aufzeichnungen, TTS- und Mediendateien.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
PREFIX="${PREFIX:-/opt/netcore-media-library}"
[[ ${EUID} -eq 0 ]] || { echo "update.sh must run as root" >&2; exit 1; }
CONFIG="${CONFIG:-/etc/netcore/media-library.toml}"
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -d -o root -g root -m 0755 "$(dirname "${CONFIG}")"
# Was: Prüft die folgende Voraussetzung und führt den passenden Zweig aus.
# Warum: Fehlende Rechte, Dateien oder Einstellungen sollen früh und verständlich behandelt werden.
if [[ ! -e "${CONFIG}" ]]; then
  # Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
  # Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
  install -o root -g netcore-media-library -m 0640 "${ROOT}/system-backend/media-library/config/media-library.example.toml" "${CONFIG}"
fi
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/shared-storage.sh"
netcore_prepare_media_local_storage "${CONFIG}"
netcore_prepare_media_shared_storage "${CONFIG}"
# Was: Baut oder prüft die Rust-Komponenten.
# Warum: So wird vor Installation oder Start sichergestellt, dass der Quellcode technisch verwendbar ist.
cargo build --release --package netcore-media-library --manifest-path "${ROOT}/Cargo.toml"
SERVICE_WAS_ACTIVE=0
if systemctl is-active --quiet netcore-media-library.service; then
  SERVICE_WAS_ACTIVE=1
  systemctl stop netcore-media-library.service
fi
restart_on_error() {
  if [[ ${SERVICE_WAS_ACTIVE} -eq 1 ]]; then
    systemctl start netcore-media-library.service || true
  fi
}
trap restart_on_error ERR
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -d -o root -g root -m 0755 "${PREFIX}/bin"
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -o root -g root -m 0755 "${ROOT}/target/release/netcore-media-library" "${PREFIX}/bin/netcore-media-library"
install -o root -g root -m 0755 "${ROOT}/system-backend/media-library/install/migrate-archive-layout.py" "${PREFIX}/bin/migrate-archive-layout.py"
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -o root -g root -m 0644 "${ROOT}/system-backend/media-library/README.md" "${PREFIX}/README.md"
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -o root -g root -m 0644 "${ROOT}/system-backend/media-library/systemd/netcore-media-library.service" /etc/systemd/system/netcore-media-library.service
python3 "${PREFIX}/bin/migrate-archive-layout.py" --config "${CONFIG}"
netcore_prepare_media_local_storage "${CONFIG}"
# Nach der Migration alle alten und neuen Archivdateien erneut für den
# parallelen SMB-Zugriff öffnen.
netcore_prepare_media_shared_storage "${CONFIG}"
source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../../shared/install" && pwd)/lxc-network.sh"
netcore_configure_lxc_endpoint "${CONFIG}" "media-library" "8230"
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl daemon-reload
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl restart netcore-media-library.service
trap - ERR
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl --no-pager --full status netcore-media-library.service
