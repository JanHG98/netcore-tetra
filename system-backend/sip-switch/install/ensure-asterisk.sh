#!/usr/bin/env bash
# Shared prerequisite for the central SIP switch and the local TBS gateway.
# Sourcing this file does not install or start anything.
NETCORE_ASTERISK_INSTALLER_PATH="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/ensure-asterisk.sh"

netcore_asterisk_existing_binary() {
  local binary
  binary="$(command -v asterisk 2>/dev/null || true)"
  if [[ -z "$binary" && -x /usr/sbin/asterisk ]]; then binary=/usr/sbin/asterisk; fi
  [[ -n "$binary" ]] && "$binary" -V >/dev/null 2>&1 || return 1
  printf '%s\n' "$binary"
}

netcore_asterisk_source_incomplete() {
  [[ -e /var/lib/netcore-asterisk/source-install.pending ]]
}

netcore_asterisk_package_candidate() {
  local candidate
  candidate="$(LC_ALL=C apt-cache policy asterisk | awk '/Candidate:/ {print $2; exit}')"
  [[ -n "$candidate" && "$candidate" != '(none)' ]] || return 1
  printf '%s\n' "$candidate"
}

netcore_asterisk_validate_install() {
  NETCORE_ASTERISK_BINARY="$(netcore_asterisk_existing_binary)" || {
    echo 'Asterisk fehlt oder lässt sich nicht ausführen.' >&2; return 1;
  }
  export NETCORE_ASTERISK_BINARY
  "$NETCORE_ASTERISK_BINARY" -V
}

netcore_asterisk_install_package() {
  DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends asterisk
}

netcore_asterisk_install_source() {
  # A new shell keeps errexit effective even when our caller checks the exit status.
  bash "$NETCORE_ASTERISK_INSTALLER_PATH" --build-source
}

netcore_asterisk_build_source() (
  set -euo pipefail
  local version="${NETCORE_ASTERISK_VERSION:-22.11.0}"
  local jobs="${NETCORE_ASTERISK_BUILD_JOBS:-2}"
  local checksum="${NETCORE_ASTERISK_SHA256:-}"
  [[ "$version" =~ ^22\.[0-9]+\.[0-9]+$ ]] || {
    echo 'NETCORE_ASTERISK_VERSION muss eine stabile 22.x.y-Version sein.' >&2; exit 2;
  }
  [[ "$jobs" =~ ^[1-9][0-9]*$ ]] || {
    echo 'NETCORE_ASTERISK_BUILD_JOBS muss eine positive Ganzzahl sein.' >&2; exit 2;
  }
  if [[ -z "$checksum" && "$version" == 22.11.0 ]]; then
    checksum=3bd5ee040509a3d3cd9b1ba9520c18e6ec0a7e7981ca68c457dcd36ba3c54d94
  fi
  [[ "$checksum" =~ ^[[:xdigit:]]{64}$ ]] || {
    echo 'Für eine andere Version ist NETCORE_ASTERISK_SHA256 erforderlich.' >&2; exit 2;
  }
  echo "Kein Asterisk-Paket verfügbar: baue Asterisk ${version} mit ${jobs} Build-Job(s)."
  DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    build-essential pkg-config autoconf-archive ca-certificates curl bzip2 patch \
    libedit-dev libjansson-dev libxml2-dev libsqlite3-dev uuid-dev \
    libssl-dev zlib1g-dev libncurses-dev libcurl4-openssl-dev libnewt-dev libunbound-dev liburiparser-dev

  local build_dir archive
  build_dir="$(mktemp -d /var/tmp/netcore-asterisk.XXXXXXXX)"
  trap 'rm -rf -- "$build_dir"' EXIT
  cd "$build_dir"
  archive="asterisk-${version}.tar.gz"
  # Versioned HTTPS download, verified before extracting or executing build code.
  curl --fail --location --retry 3 --connect-timeout 20 --max-time 600 \
    "https://downloads.asterisk.org/pub/telephony/asterisk/releases/${archive}" -o "$archive"
  printf '%s  %s\n' "$checksum" "$archive" | sha256sum --check --strict
  tar -xzf "$archive" --no-same-owner
  cd "asterisk-${version}"
  ./configure --prefix=/usr --sysconfdir=/etc --localstatedir=/var \
    --libdir=/usr/lib --with-pjproject-bundled
  make menuselect.makeopts
  menuselect/menuselect --disable BUILD_NATIVE \
    --disable-category MENUSELECT_CORE_SOUNDS \
    --disable-category MENUSELECT_MOH \
    --disable-category MENUSELECT_EXTRA_SOUNDS menuselect.makeopts
  make -j"$jobs"
  # A binary can already work before modules, users and service setup finish.
  # Keep failed installs distinguishable from an existing complete installation.
  install -d /var/lib/netcore-asterisk
  printf 'version=%s\nsha256=%s\n' "$version" "$checksum" >/var/lib/netcore-asterisk/source-install.pending
  make install
  # Do not run `make samples`: that target overwrites existing configuration.
  local module
  for module in chan_pjsip res_pjsip res_pjsip_outbound_registration res_pjsip_registrar \
    res_pjsip_authenticator_digest res_pjsip_outbound_authenticator_digest \
    res_pjsip_endpoint_identifier_user res_pjsip_endpoint_identifier_ip \
    res_pjsip_session res_pjsip_sdp_rtp res_rtp_asterisk codec_ulaw pbx_config \
    app_dial app_stack func_callerid res_agi func_db func_pjsip_contact; do
    [[ -f "/usr/lib/asterisk/modules/${module}.so" ]] || {
      echo "Benötigtes Asterisk-Modul fehlt: ${module}" >&2; exit 1;
    }
  done
  ldconfig
  getent group asterisk >/dev/null || groupadd --system asterisk
  id -u asterisk >/dev/null 2>&1 || useradd --system --gid asterisk \
    --home-dir /var/lib/asterisk --no-create-home --shell /usr/sbin/nologin asterisk
  install -d -m 0755 /etc/asterisk /var/lib/asterisk/agi-bin /var/spool/asterisk /var/log/asterisk
  # The root failover controller writes 0640 files; setgid retains readable group ownership.
  chown root:asterisk /etc/asterisk
  chmod 2750 /etc/asterisk
  chown -R asterisk:asterisk /var/lib/asterisk /var/spool/asterisk /var/log/asterisk

  if [[ ! -e /etc/asterisk/asterisk.conf ]]; then
    cat >/etc/asterisk/asterisk.conf <<'CONF'
[directories]
astetcdir => /etc/asterisk
astmoddir => /usr/lib/asterisk/modules
astvarlibdir => /var/lib/asterisk
astdbdir => /var/lib/asterisk
astkeydir => /var/lib/asterisk
astdatadir => /var/lib/asterisk
astagidir => /var/lib/asterisk/agi-bin
astspooldir => /var/spool/asterisk
astrundir => /run/asterisk
astlogdir => /var/log/asterisk
astsbindir => /usr/sbin
CONF
  fi
  if [[ ! -e /etc/asterisk/modules.conf ]]; then
    printf '[modules]\nautoload=yes\n' >/etc/asterisk/modules.conf
  fi
  if [[ ! -e /etc/asterisk/logger.conf ]]; then
    printf '[logfiles]\nconsole => notice,warning,error\n' >/etc/asterisk/logger.conf
  fi
  # A native systemd unit avoids dependence on SysV compatibility in Trixie.
  if [[ ! -e /etc/systemd/system/asterisk.service && ! -e /usr/lib/systemd/system/asterisk.service \
        && ! -e /lib/systemd/system/asterisk.service && ! -e /etc/init.d/asterisk ]]; then
    install -d /etc/systemd/system
    cat >/etc/systemd/system/asterisk.service <<'UNIT'
[Unit]
Description=Asterisk SIP gateway (NetCore source installation)
Wants=network-online.target
After=network-online.target

[Service]
Type=simple
User=asterisk
Group=asterisk
RuntimeDirectory=asterisk
RuntimeDirectoryMode=0755
ExecStart=/usr/sbin/asterisk -f -C /etc/asterisk/asterisk.conf
ExecReload=/usr/sbin/asterisk -rx "core reload"
ExecStop=/usr/sbin/asterisk -rx "core stop gracefully"
Restart=on-failure
RestartSec=2

[Install]
WantedBy=multi-user.target
UNIT
  fi
  /usr/sbin/asterisk -V
  printf 'version=%s\nsha256=%s\n' "$version" "$checksum" >/var/lib/netcore-asterisk/source-install.txt
  rm -f /var/lib/netcore-asterisk/source-install.pending
  echo 'Asterisk aus Quellcode installiert. Der aufrufende Installer richtet die SIP-Konfiguration ein.'
)

netcore_ensure_asterisk() {
  local mode="${NETCORE_ASTERISK_INSTALL_MODE:-auto}" candidate
  case "$mode" in auto|existing|package|source) ;; *)
    echo 'NETCORE_ASTERISK_INSTALL_MODE muss auto, existing, package oder source sein.' >&2; return 2;;
  esac
  if netcore_asterisk_source_incomplete; then
    if [[ "$mode" == existing || "$mode" == package ]]; then
      echo 'Unvollständige Asterisk-Quellinstallation; mit Installationsmodus auto oder source erneut ausführen.' >&2
      return 2
    fi
    echo 'Vervollständige die zuvor abgebrochene Asterisk-Quellinstallation.'
    netcore_asterisk_install_source || return $?
    netcore_asterisk_validate_install
    return $?
  fi
  if NETCORE_ASTERISK_BINARY="$(netcore_asterisk_existing_binary)"; then
    export NETCORE_ASTERISK_BINARY
    echo "Vorhandener Asterisk wird verwendet: ${NETCORE_ASTERISK_BINARY}"
    return 0
  fi
  if [[ "$mode" == existing ]]; then
    echo 'Kein vorhandener Asterisk gefunden; Installation erforderlich.' >&2; return 2
  fi
  if [[ "$mode" != source ]] && candidate="$(netcore_asterisk_package_candidate)"; then
    echo "Installiere Asterisk aus den konfigurierten Paketquellen: ${candidate}"
    netcore_asterisk_install_package || return $?
  elif [[ "$mode" == package ]]; then
    echo 'Kein APT-Kandidat für Asterisk verfügbar.' >&2; return 2
  else
    netcore_asterisk_install_source || return $?
  fi
  netcore_asterisk_validate_install
}

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  set -euo pipefail
  [[ ${EUID} -eq 0 ]] || { echo 'Als root ausführen.' >&2; exit 1; }
  case "${1:-}" in
    --build-source) netcore_asterisk_build_source ;;
    '') apt-get update; netcore_ensure_asterisk ;;
    *) echo 'Usage: ensure-asterisk.sh [--build-source]' >&2; exit 2 ;;
  esac
fi
