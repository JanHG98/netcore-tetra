#!/usr/bin/env bash
# Repairs Media Library ownership after root-run archive migrations and keeps
# the NFS/SMB archive traversable for OPEN LAB Windows clients.

set -euo pipefail
[[ ${EUID} -eq 0 ]] || { echo "Bitte als root ausführen: sudo $0" >&2; exit 1; }

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFIG="${CONFIG:-/etc/netcore/media-library.toml}"
SERVICE="${SERVICE:-netcore-media-library.service}"

source "${SCRIPT_DIR}/shared-storage.sh"

systemctl stop "${SERVICE}" 2>/dev/null || true
netcore_prepare_media_local_storage "${CONFIG}"
netcore_prepare_media_shared_storage "${CONFIG}"

systemctl daemon-reload
systemctl reset-failed "${SERVICE}" || true
systemctl restart "${SERVICE}"

printf '\nLokaler Zustand:\n'
stat -c '%A %a %U:%G %n' \
  /var/lib/netcore-media-library \
  /var/lib/netcore-media-library/state.json 2>/dev/null || true

printf '\nArchivwurzeln:\n'
stat -c '%A %a %U:%G %n' \
  /mnt/nfs-share/Media-Library \
  /mnt/nfs-share/Recordings \
  /mnt/nfs-share/TTS-Dateien 2>/dev/null || true

printf '\nDienststatus:\n'
systemctl --no-pager --full status "${SERVICE}"
