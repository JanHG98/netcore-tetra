#!/usr/bin/env bash
# Sourced only inside the ARM64 guest. The optional root is for isolated tests.
netcore_install_desktop() {
  local desktop=${1:-none} root=${2:-/} manager session session_name greeter='' label daemon
  local packages=() programs=()
  case "$desktop" in
    none) return 0 ;;
    kde)
      label='KDE Plasma'; manager=sddm; session=usr/share/xsessions/plasma.desktop
      daemon=usr/bin/sddm; programs=(usr/bin/startplasma-x11)
      packages=(plasma-desktop plasma-workspace kwin-x11 sddm sddm-theme-breeze
                konsole dolphin plasma-nm systemsettings kde-config-screenlocker
                polkit-kde-agent-1 kwalletmanager libpam-kwallet5)
      ;;
    xfce)
      label='Xfce'; manager=lightdm; session=usr/share/xsessions/xfce.desktop
      daemon=usr/sbin/lightdm; programs=(usr/bin/startxfce4)
      greeter=lightdm-gtk-greeter
      packages=(xfce4 xfce4-session xfce4-terminal xfce4-notifyd xfce4-power-manager
                thunar lightdm lightdm-gtk-greeter policykit-1-gnome
                gnome-keyring libpam-gnome-keyring)
      ;;
    gnome)
      label='GNOME'; manager=gdm3; session=usr/share/wayland-sessions/gnome.desktop
      daemon=usr/sbin/gdm3; programs=(usr/bin/gnome-session usr/bin/gnome-shell)
      packages=(gnome-core gnome-session gnome-shell gnome-control-center
                gnome-terminal nautilus gdm3 gnome-keyring libpam-gnome-keyring)
      ;;
    pios)
      # The pinned Pi OS Bookworm desktop uses Raspberry Pi's Labwc session.
      # Its session and greeter definitions come from raspberrypi-ui-mods.
      label='Raspberry Pi OS Desktop'; manager=lightdm
      daemon=usr/sbin/lightdm; programs=(usr/bin/labwc-pi usr/bin/labwc usr/sbin/pi-greeter)
      session=usr/share/wayland-sessions/LXDE-pi-labwc.desktop
      greeter=pi-greeter-labwc
      packages=(raspberrypi-ui-mods labwc wf-panel-pi pi-greeter pcmanfm lxterminal
                lightdm mate-polkit-bin kanshi xwayland pixflat-theme pixflat-icons
                rpd-wallpaper fonts-piboto wfplug-netman rc-gui
                gnome-keyring libpam-gnome-keyring)
      ;;
    *) echo 'Desktop: none, kde, xfce, gnome oder pios erforderlich' >&2; return 1 ;;
  esac
  # --no-install-recommends is intentional: install the renderer, network editor
  # and both OpenVPN editors explicitly rather than relying on recommendations.
  packages+=(xserver-xorg libgl1-mesa-dri libegl-mesa0 dbus-x11 polkitd
             fonts-dejavu-core xdg-utils network-manager network-manager-gnome
             network-manager-openvpn network-manager-openvpn-gnome)
  printf '%s shared/default-x-display-manager select %s\n' "$manager" "$manager" | debconf-set-selections
  "${NC_APT[@]}" install -y --no-install-recommends "${packages[@]}"
  local package path
  for package in "${packages[@]}"; do
    [[ $(dpkg-query -W -f='${Status}' "$package") == 'install ok installed' ]] || {
      echo "Desktop-Paket nicht installiert: $package" >&2; return 1;
    }
  done
  [[ -f "$root/$session" ]] || {
    echo "$label-Anmeldesitzung fehlt" >&2; return 1;
  }
  if [[ -n $greeter ]]; then
    [[ -f "$root/usr/share/xgreeters/$greeter.desktop" ]] || {
      echo "$label-Anmeldeoberfläche fehlt" >&2; return 1;
    }
  fi
  for path in "${programs[@]}" "$daemon" usr/bin/nm-connection-editor \
              usr/lib/NetworkManager/nm-openvpn-service usr/lib/NetworkManager/nm-openvpn-auth-dialog; do
    [[ -x "$root/$path" ]] || { echo "Desktop-/VPN-Programm fehlt: $path" >&2; return 1; }
  done
  for path in usr/lib/NetworkManager/VPN/nm-openvpn-service.name \
              usr/lib/aarch64-linux-gnu/NetworkManager/libnm-vpn-plugin-openvpn.so \
              usr/lib/aarch64-linux-gnu/NetworkManager/libnm-vpn-plugin-openvpn-editor.so \
              usr/lib/aarch64-linux-gnu/NetworkManager/libnm-gtk4-vpn-plugin-openvpn-editor.so; do
    [[ -f "$root/$path" ]] || { echo "OpenVPN-Plugin fehlt: $path" >&2; return 1; }
  done
  if [[ $desktop == kde ]]; then
    path=usr/lib/aarch64-linux-gnu/qt5/plugins/plasma/network/vpn/plasmanetworkmanagement_openvpnui.so
    [[ -f "$root/$path" ]] || { echo 'KDE-OpenVPN-Editor fehlt' >&2; return 1; }
  fi
  [[ -f "$root/lib/systemd/system/$manager.service" ]] || {
    echo "Display-Manager-Unit fehlt: $manager.service" >&2; return 1;
  }
  install -d -m 0755 "$root/usr/share/applications"
  cat > "$root/usr/share/applications/netcore-vpn.desktop" <<'EOF'
[Desktop Entry]
Type=Application
Name=VPN Connections
Name[de]=VPN-Verbindungen
Comment=Manage network connections and import OpenVPN profiles
Comment[de]=Netzwerkverbindungen verwalten und OpenVPN-Profile importieren
Exec=nm-connection-editor
TryExec=nm-connection-editor
Icon=network-vpn
Terminal=false
Categories=Settings;Network;
EOF
  chmod 0644 "$root/usr/share/applications/netcore-vpn.desktop"
  case "$manager" in
    sddm)
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
      ;;
    lightdm)
      install -d -m 0755 "$root/etc/lightdm"
      # lightdm.conf is read after its drop-ins; select the intended session here
      # so a package's previous LXDE or autologin choice cannot override it.
      session_name=${session##*/}
      printf '[Seat:*]\nuser-session=%s\ngreeter-session=%s\nallow-guest=false\nautologin-user=\nautologin-session=\n' \
        "${session_name%.desktop}" "$greeter" > "$root/etc/lightdm/lightdm.conf"
      chmod 0644 "$root/etc/lightdm/lightdm.conf"
      printf '/usr/sbin/lightdm\n' > "$root/etc/X11/default-display-manager"
      ;;
    gdm3)
      install -d -m 0755 "$root/etc/gdm3"
      printf '[daemon]\nAutomaticLoginEnable=false\n' > "$root/etc/gdm3/daemon.conf"
      chmod 0644 "$root/etc/gdm3/daemon.conf"
      printf '/usr/sbin/gdm3\n' > "$root/etc/X11/default-display-manager"
      ;;
  esac
  if [[ $manager != gdm3 ]]; then
    systemctl enable "$manager.service"
  fi
  # Debian's GDM3 postinst manages this alias directly; gdm3.service can be an
  # alias/static unit and must not be enabled as if it were a normal unit.
  install -d -m 0755 "$root/etc/systemd/system"
  ln -sfn "/lib/systemd/system/$manager.service" "$root/etc/systemd/system/display-manager.service"
  systemctl set-default graphical.target
  echo "$label mit grafischer Anmeldung und OpenVPN-Verbindungsverwaltung installiert."
}
