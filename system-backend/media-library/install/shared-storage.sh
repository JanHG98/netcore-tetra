#!/usr/bin/env bash
# NETCORE-KOMMENTAR – Was: Bereitet das gemeinsam per NFS und SMB verwendete Medien-Share vor.
# NETCORE-KOMMENTAR – Warum: Media Library, Recorder, TTS und Windows-Clients sollen dieselben Ordner ohne 0700-Rechtefalle verwenden können.

# This file is sourced by install.sh and update.sh.

netcore_media_set_storage_path() {
  local config_path="$1"
  local key="$2"
  local value="$3"

  if grep -Eq "^[[:space:]]*${key}[[:space:]]*=" "${config_path}"; then
    sed -i -E \
      "s|^[[:space:]]*${key}[[:space:]]*=.*$|${key} = \"${value}\"|" \
      "${config_path}"
  elif grep -Eq '^\[storage\][[:space:]]*$' "${config_path}"; then
    sed -i \
      "/^\[storage\][[:space:]]*$/a ${key} = \"${value}\"" \
      "${config_path}"
  else
    printf '\n[storage]\n%s = "%s"\n' "${key}" "${value}" >> "${config_path}"
  fi
}


netcore_prepare_media_local_storage() {
  local service_user="${NETCORE_MEDIA_SERVICE_USER:-netcore-media-library}"
  local service_group="${NETCORE_MEDIA_SERVICE_GROUP:-netcore-media-library}"
  local local_root="${NETCORE_MEDIA_LOCAL_ROOT:-/var/lib/netcore-media-library}"
  local config_path="${1:-/etc/netcore/media-library.toml}"

  # Migration and update helpers run as root. Repair ownership afterwards so
  # an atomically replaced state.json or a root-created .part file cannot make
  # the service fail with PermissionDenied on startup.
  install -d -o "${service_user}" -g "${service_group}" -m 0750 \
    "${local_root}" \
    "${local_root}/assets" \
    "${local_root}/tmp" \
    "${local_root}/backups"

  chown -R "${service_user}:${service_group}" "${local_root}"
  find "${local_root}" -xdev -type d -exec chmod 0750 {} +
  find "${local_root}" -xdev -type f -exec chmod 0640 {} +

  if [[ -e "${config_path}" ]]; then
    chown root:"${service_group}" "${config_path}"
    chmod 0640 "${config_path}"
  fi
}

netcore_prepare_media_shared_storage() {
  local config_path="$1"
  local share_root="${NETCORE_MEDIA_SHARE_ROOT:-/mnt/nfs-share}"
  local shared_directory
  local -a shared_directories=(
    "Media-Library"
    "Recordings"
    "TTS-Dateien"
  )

  # Always make the standard mount point available. Subdirectories are created
  # only when the external share is actually mounted, so local files cannot be
  # written accidentally underneath a missing NFS mount.
  if [[ ! -d "${share_root}" ]]; then
    install -d -o root -g root -m 0755 "${share_root}"
  fi

  netcore_media_set_storage_path \
    "${config_path}" "archive_root" "${share_root%/}/Media-Library"
  netcore_media_set_storage_path \
    "${config_path}" "recording_archive_root" "${share_root%/}/Recordings"
  netcore_media_set_storage_path \
    "${config_path}" "tts_archive_root" "${share_root%/}/TTS-Dateien"

  if ! mountpoint -q "${share_root}"; then
    echo "WARNING: ${share_root} is not a mount point; shared media directories were not created." >&2
    echo "WARNING: Mount the NFS share there and rerun this installer or update script." >&2
    return 0
  fi

  for shared_directory in "${shared_directories[@]}"; do
    mkdir -p "${share_root%/}/${shared_directory}"

    # OPEN LAB / SMB interoperability: NFS-created directories must remain
    # traversable and writable by Windows clients using the parallel SMB share.
    # Repair both the root and every existing child. Earlier versions created
    # UUID directories with 0700/0600, which blocked the parallel SMB share.
    chmod 0777 "${share_root%/}/${shared_directory}"
    find "${share_root%/}/${shared_directory}" -xdev -type d -exec chmod 0777 {} +
    find "${share_root%/}/${shared_directory}" -xdev -type f -exec chmod 0666 {} +
  done

  echo "Shared media storage prepared at ${share_root}:"
  printf '  - %s\n' "${shared_directories[@]}"
}
