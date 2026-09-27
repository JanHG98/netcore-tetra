#!/usr/bin/env bash
set -Eeuo pipefail
SERVICE=${1:?service required}
case "$SERVICE" in
  hardware-gateway|rf-monitor|alarm-workflow|task-workflow|asset-management|sip-switch|alert-service) exit 0 ;;
esac
apt-get update
DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
  build-essential pkg-config libssl-dev libsqlite3-dev cmake curl ca-certificates
if [[ ! -x /root/.cargo/bin/cargo ]]; then
  RUSTUP_SCRIPT=$(mktemp)
  trap 'rm -f "$RUSTUP_SCRIPT"' EXIT
  curl --fail --show-error --proto '=https' --tlsv1.2 https://sh.rustup.rs -o "$RUSTUP_SCRIPT"
  sh "$RUSTUP_SCRIPT" -y --profile minimal --default-toolchain stable
fi
if [[ $SERVICE == tbs ]]; then
  DEBIAN_FRONTEND=noninteractive apt-get install -y libsoapysdr-dev soapysdr-tools libgsm1-dev
  # Default TBS features include Asterisk and therefore the native speech codec.
  : "${REPO_ROOT:?}"
  cmake -S "$REPO_ROOT/tetra-codec-master" -B "$REPO_ROOT/target/deployment-tetra-codec" \
    -DCMAKE_INSTALL_PREFIX=/usr -DCMAKE_INSTALL_LIBDIR=lib
  cmake --build "$REPO_ROOT/target/deployment-tetra-codec" --parallel 2
  cmake --install "$REPO_ROOT/target/deployment-tetra-codec"
  ldconfig
fi
