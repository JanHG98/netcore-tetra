#!/usr/bin/env bash
# Backend build prerequisites only. No installer, daemon or OS-upgrade actions.
# The updater calls this before staging builds and before stopping any service.
set -euo pipefail

fail() { printf 'Build-Vorbereitung abgebrochen: %s\n' "$*" >&2; exit 1; }

usage() {
  printf '%s\n' \
    'Usage: prepare-netcore-build.sh --role ROLE [--role ROLE ...]' \
    '       prepare-netcore-build.sh --all-backends' \
    'Root, Debian/Ubuntu: fehlende Build-Pakete und root Rust stable vorbereiten.'
}

NC_ROLES=()
while (($#)); do
  case "$1" in
    --role)
      (($# >= 2)) || fail '--role braucht einen Wert.'
      case "$2" in
        node-gateway|mobility-core|subscriber-core|group-core|call-control|media-switch|recorder|sds-router|packet-core|ip-gateway|security-core|kmf|transit|application-gateway|media-library|control-room|observability|iot-gateway|provisioning-core)
          NC_ROLES+=("$2") ;;
        *) fail "Kein verwalteter Rust-Backend-Dienst: $2" ;;
      esac
      shift 2 ;;
    --all-backends) NC_ROLES+=('all-backends'); shift ;;
    --help|-h) usage; exit 0 ;;
    *) fail "Unbekanntes Argument: $1" ;;
  esac
done
((${#NC_ROLES[@]})) || { usage >&2; exit 1; }

[[ "$(uname -s)" == Linux ]] || fail 'Linux erforderlich.'
[[ "$(id -u)" == 0 ]] || fail 'Als root ausfuehren.'
[[ -r /etc/os-release ]] || fail '/etc/os-release fehlt.'
# This is the local OS metadata, not a fetched shell fragment.
. /etc/os-release
case "${ID:-}" in
  debian|ubuntu) ;;
  *) fail "Nur Debian/Ubuntu unterstuetzt (ID=${ID:-unbekannt})." ;;
esac
command -v dpkg-query >/dev/null || fail 'dpkg-query fehlt.'
command -v apt-get >/dev/null || fail 'apt-get fehlt.'

# Codec/Soapy/GPIO dependencies belong to the Pi image, not central backends.
NC_PACKAGES=(build-essential pkg-config libssl-dev libsqlite3-dev cmake git curl ca-certificates)
NC_MISSING=()
for NC_PACKAGE in "${NC_PACKAGES[@]}"; do
  NC_STATUS=$(dpkg-query -W -f='${Status}' "$NC_PACKAGE" 2>/dev/null || true)
  if [[ "$NC_STATUS" != 'install ok installed' ]]; then
    NC_MISSING+=("$NC_PACKAGE")
  fi
done
printf 'Backend-Rollen: %s\n' "${NC_ROLES[*]}"
printf 'Gepruefte Build-Pakete: %s\n' "${NC_PACKAGES[*]}"
if ((${#NC_MISSING[@]})); then
  printf 'Fehlende Build-Pakete installieren: %s\n' "${NC_MISSING[*]}"
  DEBIAN_FRONTEND=noninteractive apt-get update
  DEBIAN_FRONTEND=noninteractive apt-get install --no-install-recommends -y "${NC_MISSING[@]}"
else
  printf '%s\n' 'Build-Pakete bereits vorhanden; kein apt-get update/install.'
fi

# Dedicated root homes avoid using another operator's Rust installation.
# Neither installation nor updates alter the default toolchain or shell PATH.
export CARGO_HOME=/root/.cargo
export RUSTUP_HOME=/root/.rustup
NC_RUSTUP="$CARGO_HOME/bin/rustup"
if [[ ! -x "$NC_RUSTUP" ]]; then
  NC_RUSTUP_SCRIPT=$(mktemp /tmp/netcore-rustup.XXXXXX)
  trap 'rm -f -- "$NC_RUSTUP_SCRIPT"' EXIT
  printf '%s\n' 'Root rustup fehlt; offiziellen Installer ueber HTTPS herunterladen.'
  curl -q --fail --show-error --silent --location \
    --proto '=https' --proto-redir '=https' --tlsv1.2 \
    https://sh.rustup.rs -o "$NC_RUSTUP_SCRIPT"
  sh "$NC_RUSTUP_SCRIPT" -y --no-modify-path --default-toolchain none --profile minimal
fi
[[ -x "$NC_RUSTUP" ]] || fail "rustup wurde nicht installiert: $NC_RUSTUP"
"$NC_RUSTUP" toolchain install stable --profile minimal
"$NC_RUSTUP" run stable rustc --version
"$NC_RUSTUP" run stable cargo --version
printf '%s\n' 'Build-Vorbereitung abgeschlossen. Der Updater baut mit RUSTUP_TOOLCHAIN=stable.'
