#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
PREFIX="${PREFIX:-/opt/netcore-media-library}"
[[ ${EUID} -eq 0 ]] || { echo "update.sh must run as root" >&2; exit 1; }
CONFIG="${CONFIG:-/etc/netcore/media-library.toml}"
install -d -o root -g root -m 0755 "$(dirname "${CONFIG}")"
if [[ ! -e "${CONFIG}" ]]; then
  install -o root -g netcore-media-library -m 0640 "${ROOT}/system-backend/media-library/config/media-library.example.toml" "${CONFIG}"
fi
cargo build --release --package netcore-media-library --manifest-path "${ROOT}/Cargo.toml"
install -d -o root -g root -m 0755 "${PREFIX}/bin"
install -o root -g root -m 0755 "${ROOT}/target/release/netcore-media-library" "${PREFIX}/bin/netcore-media-library"
install -o root -g root -m 0644 "${ROOT}/system-backend/media-library/README.md" "${PREFIX}/README.md"
install -o root -g root -m 0644 "${ROOT}/system-backend/media-library/systemd/netcore-media-library.service" /etc/systemd/system/netcore-media-library.service
source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../../shared/install" && pwd)/lxc-network.sh"
netcore_configure_lxc_endpoint "${CONFIG}" "media-library" "8230"
systemctl daemon-reload
systemctl restart netcore-media-library.service
systemctl --no-pager --full status netcore-media-library.service
