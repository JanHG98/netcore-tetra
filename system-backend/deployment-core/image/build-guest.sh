#!/usr/bin/env bash
# Runs inside the ARM64 image, never on the VM root filesystem.
set -Eeuo pipefail
[[ $(dpkg --print-architecture) == arm64 ]] || { echo 'ARM64-Gast erforderlich'; exit 1; }
NETCORE_REPOSITORY=${1:?}
NETCORE_COMMIT=${2:?}
SOAPY_REPOSITORY=${3:?}
SOAPY_COMMIT=${4:?}
export DEBIAN_FRONTEND=noninteractive
export LC_ALL=C
# Build portable Pi initramfs images instead of detecting the VM's root device.
mkdir -p /etc/initramfs-tools/conf.d
printf '# NetCore portable Pi image\nMODULES=most\n' > /etc/initramfs-tools/conf.d/zz-netcore-image.conf
chmod 0644 /etc/initramfs-tools/conf.d/zz-netcore-image.conf
# dpkg conffile prompts are separate from debconf; preserve the guest's config.
NC_APT=(apt-get -o Dpkg::Options::=--force-confdef -o Dpkg::Options::=--force-confold)
"${NC_APT[@]}" -o APT::Update::Error-Mode=any update
"${NC_APT[@]}" -y dist-upgrade
"${NC_APT[@]}" install -y --no-install-recommends \
  git ca-certificates curl python3 python3-soapysdr build-essential pkg-config cmake \
  libssl-dev libsqlite3-dev libsoapysdr-dev soapysdr-tools libasound2-dev libgsm1-dev \
  alsa-utils device-tree-compiler ffmpeg jq sqlite3 nfs-common network-manager \
  openssh-server sudo tzdata openvpn iw rfkill cloud-guest-utils iproute2 nftables
install -d /opt /usr/share/netcore-image
git clone --no-checkout "$NETCORE_REPOSITORY" /opt/netcore-tetra
git -C /opt/netcore-tetra checkout --detach "$NETCORE_COMMIT"
[[ $(git -C /opt/netcore-tetra rev-parse HEAD) == "$NETCORE_COMMIT" ]]
git clone --no-checkout "$SOAPY_REPOSITORY" /opt/sxxcvr
git -C /opt/sxxcvr checkout --detach "$SOAPY_COMMIT"
[[ $(git -C /opt/sxxcvr rev-parse HEAD) == "$SOAPY_COMMIT" ]]
cmake -S /opt/sxxcvr/SoapySX -B /opt/sxxcvr/build -DCMAKE_BUILD_TYPE=Release
cmake --build /opt/sxxcvr/build --parallel 2
cmake --install /opt/sxxcvr/build
dtc -@ -I dts -O dtb /opt/sxxcvr/dts/sx1255_raspberrypi.dts -o /boot/firmware/overlays/netcore-sx1255.dtbo
cmake -S /opt/netcore-tetra/tetra-codec-master -B /opt/netcore-tetra/target/image-codec \
  -DCMAKE_INSTALL_PREFIX=/usr -DCMAKE_INSTALL_LIBDIR=lib
cmake --build /opt/netcore-tetra/target/image-codec --parallel 2
cmake --install /opt/netcore-tetra/target/image-codec
ldconfig
curl --fail --show-error --proto '=https' --tlsv1.2 https://sh.rustup.rs -o /tmp/netcore-rustup.sh
bash /tmp/netcore-rustup.sh -y --profile minimal --default-toolchain stable
export PATH="/root/.cargo/bin:$PATH"
export CARGO_BUILD_JOBS=2
cd /opt/netcore-tetra
cargo build --locked --release -p bluestation-bs
install -m 0755 target/release/bluestation-bs /usr/local/bin/bluestation-bs
# Validate dynamic dependencies without probing non-existent VM radio hardware.
ldd /usr/local/bin/bluestation-bs | tee /usr/share/netcore-image/ldd.txt
! grep -q 'not found' /usr/share/netcore-image/ldd.txt
SoapySDRUtil --info | tee /usr/share/netcore-image/soapysdr.txt
grep -q 'sx' /usr/share/netcore-image/soapysdr.txt
python3 - "$NETCORE_COMMIT" "$SOAPY_COMMIT" <<'PY'
import json, pathlib, subprocess, sys
data = dict(commit=sys.argv[1], soapy_commit=sys.argv[2],
            rustc=subprocess.check_output(['/root/.cargo/bin/rustc','--version'],text=True).strip(),
            packages=subprocess.check_output(['dpkg-query','-W','-f=${Package}\t${Version}\n'],text=True).splitlines())
pathlib.Path('/usr/share/netcore-image/versions.json').write_text(json.dumps(data,indent=2))
PY
SOURCE=/opt/netcore-tetra/system-backend/deployment-core
install -d /usr/local/lib/netcore-deployment/{static,install} /etc/netcore
install -m 0644 "$SOURCE"/*.py "$SOURCE/catalog.json" /usr/local/lib/netcore-deployment/
install -m 0644 "$SOURCE"/static/* /usr/local/lib/netcore-deployment/static/
install -m 0755 "$SOURCE"/install/prepare-host.sh "$SOURCE"/install/install-tbs.sh /usr/local/lib/netcore-deployment/install/
install -m 0644 "$SOURCE/systemd/netcore-discovery.service" /etc/systemd/system/
for group in netcore spi gpio; do getent group "$group" >/dev/null || groupadd --system "$group"; done
id netcore >/dev/null 2>&1 || useradd --system --gid netcore --groups audio,spi,gpio --home-dir /var/lib/netcore-tbs --shell /usr/sbin/nologin netcore
install -d -m 0750 -o netcore -g netcore /var/lib/netcore-tbs
install -d -m 0755 /var/lib/netcore-discovery /usr/local/libexec
install -m 0644 /usr/local/lib/netcore-image/systemd/* /etc/systemd/system/
install -m 0755 /opt/netcore-tetra/contrib/packet-data/netcore-tetra-packet-gateway-cleanup /usr/local/libexec/
install -d /etc/systemd/system/tetra.service.d
install -m 0644 /opt/netcore-tetra/contrib/systemd/tetra.service.d/20-packet-data-gateway.conf /etc/systemd/system/tetra.service.d/
install -m 0644 /usr/local/lib/netcore-image/99-netcore-sdr.rules /etc/udev/rules.d/
systemctl enable netcore-firstboot.service netcore-discovery.service tetra.service ssh.service NetworkManager.service
# Personalization supplies the user before the image is ever booted.
systemctl mask userconfig.service userconf-pi.service
systemctl disable regenerate_ssh_host_keys.service 2>/dev/null || true
# Don't distribute the compile tree or Rust download cache to every SD card.
rm -rf /opt/netcore-tetra/target /opt/sxxcvr/build /root/.cargo /root/.rustup
rm -f /tmp/netcore-rustup.sh
apt-get clean
rm -rf /var/lib/apt/lists/*
echo 'Softwarebasis vollständig gebaut. Noch kein Funk-/Boot-Test ausgeführt.'
