#!/usr/bin/env bash
# Sourced only inside the ARM64 guest. The optional root is for isolated tests.
netcore_install_desktop() {
  local desktop=${1:-none} root=${2:-/}
  case "$desktop" in
    none) return 0 ;;
    kde) ;;
    *) echo 'Desktop: none oder kde erforderlich' >&2; return 1 ;;
  esac
  local packages=(plasma-desktop plasma-workspace kwin-x11 sddm sddm-theme-breeze
                  xserver-xorg dbus-x11 konsole dolphin plasma-nm systemsettings
                  kde-config-screenlocker polkit-kde-agent-1 fonts-dejavu-core xdg-utils)
  printf 'sddm shared/default-x-display-manager select sddm\n' | debconf-set-selections
  "${NC_APT[@]}" install -y --no-install-recommends "${packages[@]}"
  local package
  for package in "${packages[@]}"; do
    [[ $(dpkg-query -W -f='${Status}' "$package") == 'install ok installed' ]] || {
      echo "KDE-Paket nicht installiert: $package" >&2; return 1;
    }
  done
  [[ -f "$root/usr/share/xsessions/plasma.desktop" ]] || {
    echo 'KDE-Anmeldesitzung fehlt' >&2; return 1;
  }
  install -d -m 0755 "$root/etc/sddm.conf.d"
  cat > "$root/etc/sddm.conf.d/90-netcore.conf" <<'EOF'
[General]
DisplayServer=x11

[Autologin]
User=
Session=
Relogin=false
EOF
  chmod 0644 "$root/etc/sddm.conf.d/90-netcore.conf"
  printf '/usr/bin/sddm\n' > "$root/etc/X11/default-display-manager"
  systemctl enable sddm.service
  systemctl set-default graphical.target
  echo 'KDE Plasma installiert; grafische Anmeldung mit dem persönlichen Pi-Benutzer.'
}
