#!/usr/bin/env bash
# NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für gespeicherte Aufzeichnungen, TTS- und Mediendateien.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
PREFIX="${PREFIX:-/opt/netcore-media-library}"
CONFIG="${CONFIG:-/etc/netcore/media-library.toml}"
NETCORE_TBS_ID="${NETCORE_TBS_ID:-srv-m-tbs-01}"
NETCORE_TBS_NAME="${NETCORE_TBS_NAME:-SRV-M-TBS-01}"
NETCORE_TBS_URL="${NETCORE_TBS_URL:-http://10.0.1.22:8080}"
NETCORE_TBS_USERNAME="${NETCORE_TBS_USERNAME:-}"
NETCORE_TBS_PASSWORD="${NETCORE_TBS_PASSWORD:-}"
SERVICE="${SERVICE:-/etc/systemd/system/netcore-media-library.service}"
[[ ${EUID} -eq 0 ]] || { echo "install.sh must run as root" >&2; exit 1; }
id -u netcore-media-library >/dev/null 2>&1 || useradd --system --home /var/lib/netcore-media-library --shell /usr/sbin/nologin netcore-media-library
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -d -o netcore-media-library -g netcore-media-library -m 0750 /var/lib/netcore-media-library /var/lib/netcore-media-library/assets /var/lib/netcore-media-library/tmp /var/lib/netcore-media-library/backups
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -d -o root -g root -m 0755 "${PREFIX}/bin" "$(dirname "${CONFIG}")"
# Was: Prüft die folgende Voraussetzung und führt den passenden Zweig aus.
# Warum: Fehlende Rechte, Dateien oder Einstellungen sollen früh und verständlich behandelt werden.
if [[ ! -e "${CONFIG}" ]]; then
  # Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
  # Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
  install -o root -g netcore-media-library -m 0640 "${ROOT}/system-backend/media-library/config/media-library.example.toml" "${CONFIG}"
fi
PLAYOUT_MIGRATION=(
  --config "${CONFIG}"
  --station-id "${NETCORE_TBS_ID}"
  --station-name "${NETCORE_TBS_NAME}"
  --station-url "${NETCORE_TBS_URL}"
)
if [[ -n "${NETCORE_TBS_USERNAME}" || -n "${NETCORE_TBS_PASSWORD}" ]]; then
  [[ -n "${NETCORE_TBS_USERNAME}" && -n "${NETCORE_TBS_PASSWORD}" ]] || {
    echo "NETCORE_TBS_USERNAME and NETCORE_TBS_PASSWORD must be supplied together" >&2
    exit 1
  }
  PLAYOUT_MIGRATION+=(
    --username "${NETCORE_TBS_USERNAME}"
    --password "${NETCORE_TBS_PASSWORD}"
  )
fi
python3 "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/migrate-playout-config.py" \
  "${PLAYOUT_MIGRATION[@]}"
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/shared-storage.sh"
netcore_prepare_media_local_storage "${CONFIG}"
netcore_prepare_media_shared_storage "${CONFIG}"
bash "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/ensure-piper.sh"
# Was: Baut oder prüft die Rust-Komponenten.
# Warum: So wird vor Installation oder Start sichergestellt, dass der Quellcode technisch verwendbar ist.
bash "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/cargo-build.sh" "${ROOT}" netcore-media-library
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -o root -g root -m 0755 "${ROOT}/target/release/netcore-media-library" "${PREFIX}/bin/netcore-media-library"
install -o root -g root -m 0755 "${ROOT}/system-backend/media-library/install/migrate-archive-layout.py" "${PREFIX}/bin/migrate-archive-layout.py"
install -o root -g root -m 0755 "${ROOT}/system-backend/media-library/install/migrate-playout-config.py" "${PREFIX}/bin/migrate-playout-config.py"
install -o root -g root -m 0755 "${ROOT}/system-backend/media-library/install/diagnose-basisstation-playout.py" "${PREFIX}/bin/diagnose-basisstation-playout.py"
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -o root -g root -m 0644 "${ROOT}/system-backend/media-library/README.md" "${PREFIX}/README.md"
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -o root -g root -m 0644 "${ROOT}/system-backend/media-library/systemd/netcore-media-library.service" "${SERVICE}"
# Bestehende UUID-Archive werden einmalig in YYYY/MM/DD mit verständlichen
# Dateinamen migriert. Bei einer Neuinstallation ist der Lauf einfach leer.
python3 "${PREFIX}/bin/migrate-archive-layout.py" --config "${CONFIG}"
netcore_prepare_media_local_storage "${CONFIG}"
netcore_prepare_media_shared_storage "${CONFIG}"
source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../../shared/install" && pwd)/lxc-network.sh"
netcore_configure_lxc_endpoint "${CONFIG}" "media-library" "8230"
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl daemon-reload
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl enable --now netcore-media-library.service
command -v ffmpeg >/dev/null || echo "WARNING: ffmpeg is missing; non-canonical WAV and MP3 preview processing will fail." >&2
echo "OPEN LAB: no login, no tokens and no TLS. Isolated management network only."
echo "Run playout diagnostic: /opt/netcore-media-library/bin/diagnose-basisstation-playout.py"
