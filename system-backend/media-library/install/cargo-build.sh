#!/usr/bin/env bash
set -Eeuo pipefail

# Build helper for the Media-Library LXC.
# The installer itself runs as root, but Rust installed through rustup often
# lives outside root's restricted PATH. This helper locates that toolchain,
# runs Cargo as the owning user, or installs a minimal stable toolchain when no
# usable Cargo installation exists yet.

[[ ${EUID} -eq 0 ]] || {
  echo "[Media Library Build] FEHLER: cargo-build.sh muss als root laufen." >&2
  exit 1
}

REPO_ROOT="${1:-}"
PACKAGE="${2:-netcore-media-library}"
[[ -n "${REPO_ROOT}" && -f "${REPO_ROOT}/Cargo.toml" ]] || {
  echo "[Media Library Build] FEHLER: Repository-Wurzel mit Cargo.toml fehlt: ${REPO_ROOT:-<leer>}" >&2
  exit 1
}

log() { printf '[Media Library Build] %s\n' "$*"; }
die() { printf '[Media Library Build] FEHLER: %s\n' "$*" >&2; exit 1; }

command -v getent >/dev/null 2>&1 || die "getent fehlt."
command -v runuser >/dev/null 2>&1 || die "runuser fehlt (Paket util-linux)."

requested_user="${BUILD_USER:-}"
if [[ -z "${requested_user}" && -n "${SUDO_USER:-}" && "${SUDO_USER}" != "root" ]]; then
  requested_user="${SUDO_USER}"
fi

cargo_bin="${CARGO_BIN:-}"
build_user="${requested_user}"

valid_cargo() {
  [[ -n "${1:-}" && -x "$1" ]]
}

# 1. Explicit path, 2. current PATH, 3. common rustup locations.
if ! valid_cargo "${cargo_bin}"; then
  cargo_bin="$(command -v cargo 2>/dev/null || true)"
fi
if ! valid_cargo "${cargo_bin}" && [[ -x /root/.cargo/bin/cargo ]]; then
  cargo_bin=/root/.cargo/bin/cargo
fi
if ! valid_cargo "${cargo_bin}" && [[ -x /usr/local/cargo/bin/cargo ]]; then
  cargo_bin=/usr/local/cargo/bin/cargo
fi
if ! valid_cargo "${cargo_bin}"; then
  for candidate in /home/*/.cargo/bin/cargo; do
    if [[ -x "${candidate}" ]]; then
      cargo_bin="${candidate}"
      break
    fi
  done
fi

# Infer the owner for a per-user rustup installation unless BUILD_USER was set.
if [[ -z "${build_user}" && -n "${cargo_bin}" ]]; then
  case "${cargo_bin}" in
    /home/*/.cargo/bin/cargo)
      inferred="${cargo_bin#/home/}"
      inferred="${inferred%%/*}"
      if id "${inferred}" >/dev/null 2>&1; then
        build_user="${inferred}"
      fi
      ;;
    *) build_user=root ;;
  esac
fi

# No Cargo found: install a minimal stable rustup toolchain. The source uses
# Rust edition 2024, so an arbitrarily old distro Cargo package is not enough.
if ! valid_cargo "${cargo_bin}"; then
  build_user="${build_user:-root}"
  id "${build_user}" >/dev/null 2>&1 || die "Build-Benutzer '${build_user}' existiert nicht."
  build_home="$(getent passwd "${build_user}" | awk -F: '{print $6}')"
  [[ -n "${build_home}" && -d "${build_home}" ]] || die "Home-Verzeichnis für '${build_user}' fehlt."

  log "Cargo wurde nicht gefunden; installiere eine minimale stabile Rust-Toolchain für ${build_user}."
  export DEBIAN_FRONTEND=noninteractive
  apt-get update
  apt-get install -y --no-install-recommends \
    ca-certificates curl build-essential pkg-config libssl-dev

  rustup_cmd='curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable'
  if [[ "${build_user}" == root ]]; then
    HOME="${build_home}" bash -lc "${rustup_cmd}"
  else
    runuser -u "${build_user}" -- env HOME="${build_home}" bash -lc "${rustup_cmd}"
  fi
  cargo_bin="${build_home}/.cargo/bin/cargo"
fi

valid_cargo "${cargo_bin}" || die "Cargo konnte nicht installiert oder gefunden werden."
build_user="${build_user:-root}"
id "${build_user}" >/dev/null 2>&1 || die "Build-Benutzer '${build_user}' existiert nicht."
build_home="$(getent passwd "${build_user}" | awk -F: '{print $6}')"
build_group="$(id -gn "${build_user}")"
[[ -n "${build_home}" && -d "${build_home}" ]] || die "Home-Verzeichnis für '${build_user}' fehlt."

cargo_dir="$(dirname "${cargo_bin}")"
build_path="${cargo_dir}:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
cargo_home="${CARGO_HOME_OVERRIDE:-${build_home}/.cargo}"
rustup_home="${RUSTUP_HOME_OVERRIDE:-${build_home}/.rustup}"

run_as_builder() {
  if [[ "${build_user}" == root ]]; then
    env \
      HOME="${build_home}" USER=root LOGNAME=root \
      PATH="${build_path}" \
      CARGO_HOME="${cargo_home}" RUSTUP_HOME="${rustup_home}" \
      CARGO_TARGET_DIR="${REPO_ROOT}/target" \
      "$@"
  else
    runuser -u "${build_user}" -- env \
      HOME="${build_home}" USER="${build_user}" LOGNAME="${build_user}" \
      PATH="${build_path}" \
      CARGO_HOME="${cargo_home}" RUSTUP_HOME="${rustup_home}" \
      CARGO_TARGET_DIR="${REPO_ROOT}/target" \
      "$@"
  fi
}

log "Build-Benutzer: ${build_user}"
log "Cargo: ${cargo_bin}"
run_as_builder "${cargo_bin}" --version || die "Cargo kann nicht ausgeführt werden."

install -d -o "${build_user}" -g "${build_group}" -m 0755 "${REPO_ROOT}/target"
chown -R "${build_user}:${build_group}" "${REPO_ROOT}/target"

log "Baue Paket ${PACKAGE} ..."
run_as_builder "${cargo_bin}" build \
  --release \
  --package "${PACKAGE}" \
  --manifest-path "${REPO_ROOT}/Cargo.toml"
