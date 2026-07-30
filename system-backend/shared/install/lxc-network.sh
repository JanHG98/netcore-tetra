#!/usr/bin/env bash
# NETCORE-KOMMENTAR – Was: Enthält die Logik oder Einstellungen für lxc network.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

# Shared LXC network helper for NetCore-Tetra backend installers.
#
# The default address is the IPv4 source address selected by the kernel for the
# default route. This works with DHCP and DHCP static leases without embedding
# a site-specific subnet in the repository. Set NETCORE_LXC_IP to override the
# detected address for hosts with several management interfaces.

# Was: Führt den Arbeitsschritt `netcore_is_ipv4` für netcore is ipv4 aus.
# Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
netcore_is_ipv4() {
  local value=${1:-} octet
  local -a parts
  IFS=. read -r -a parts <<<"$value"
  [[ ${#parts[@]} -eq 4 ]] || return 1
  # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung gilt.
  # Warum: Gleichartige Installations- oder Prüfaufgaben werden dadurch vollständig abgearbeitet.
  for octet in "${parts[@]}"; do
    [[ $octet =~ ^[0-9]{1,3}$ ]] || return 1
    ((10#$octet >= 0 && 10#$octet <= 255)) || return 1
  done
  [[ $value != 127.* && $value != 169.254.* && $value != 0.* ]]
}

# Was: Führt den Arbeitsschritt `netcore_detect_lxc_ipv4` für netcore detect lxc ipv4 aus.
# Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
netcore_detect_lxc_ipv4() {
  local candidate=${NETCORE_LXC_IP:-}

  # Was: Prüft die folgende Voraussetzung und führt den passenden Zweig aus.
  # Warum: Fehlende Rechte, Dateien oder Einstellungen sollen früh und verständlich behandelt werden.
  if [[ -n $candidate ]]; then
    netcore_is_ipv4 "$candidate" || {
      echo "NETCORE_LXC_IP ist keine nutzbare IPv4-Adresse: $candidate" >&2
      return 1
    }
    printf '%s\n' "$candidate"
    return 0
  fi

  command -v ip >/dev/null 2>&1 || {
    echo "Das Werkzeug 'ip' fehlt. Bitte iproute2 installieren." >&2
    return 1
  }

  # A route lookup does not send traffic; it only asks the kernel which source
  # address it would use. Therefore it also works when Internet access itself
  # is blocked but a default route exists.
  candidate=$(ip -4 route get "${NETCORE_ROUTE_PROBE:-1.1.1.1}" 2>/dev/null \
    | awk '{for (i=1;i<=NF;i++) if ($i=="src") {print $(i+1); exit}}')

  # Was: Prüft die folgende Voraussetzung und führt den passenden Zweig aus.
  # Warum: Fehlende Rechte, Dateien oder Einstellungen sollen früh und verständlich behandelt werden.
  if ! netcore_is_ipv4 "$candidate"; then
    candidate=$(ip -o -4 addr show up scope global 2>/dev/null \
      | awk '{split($4,a,"/"); if (a[1] !~ /^127\./ && a[1] !~ /^169\.254\./) {print a[1]; exit}}')
  fi

  netcore_is_ipv4 "$candidate" || {
    echo "Keine nutzbare globale IPv4-Adresse im LXC gefunden." >&2
    echo "DHCP-Lease prüfen oder NETCORE_LXC_IP=x.x.x.x setzen." >&2
    return 1
  }

  printf '%s\n' "$candidate"
}

# Was: Führt den Arbeitsschritt `netcore_set_first_toml_string` für netcore set first toml string aus.
# Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
netcore_set_first_toml_string() {
  local file=$1 key=$2 value=$3
  [[ -f $file ]] || {
    echo "Konfigurationsdatei fehlt: $file" >&2
    return 1
  }

  # Was: Prüft die folgende Voraussetzung und führt den passenden Zweig aus.
  # Warum: Fehlende Rechte, Dateien oder Einstellungen sollen früh und verständlich behandelt werden.
  if grep -Eq "^[[:space:]]*${key}[[:space:]]*=" "$file"; then
    # Only the first matching key is changed. This matters for ip-gateway.toml,
    # which has additional listener keys in later sections.
    sed -i -E "0,/^[[:space:]]*${key}[[:space:]]*=/{s|^[[:space:]]*${key}[[:space:]]*=.*$|${key} = \"${value}\"|}" "$file"
  fi
}

# Was: Führt den Arbeitsschritt `netcore_configure_lxc_endpoint` für netcore configure lxc endpoint aus.
# Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
netcore_configure_lxc_endpoint() {
  local config=$1 service=$2 port=$3
  local ip webui state_dir=${NETCORE_STATE_DIR:-/etc/netcore}

  # Was: Richtet eine Netzwerkschnittstelle oder Route ein.
  # Warum: Paketdaten benötigen einen eindeutigen Weg zwischen TETRA-Seite und IP-Netz.
  ip=$(netcore_detect_lxc_ipv4)
  webui="http://${ip}:${port}/"

  netcore_set_first_toml_string "$config" bind "${ip}:${port}"

  # Services which publish absolute links need their externally reachable LXC
  # address rather than loopback or an example subnet.
  netcore_set_first_toml_string "$config" public_base_url "http://${ip}:${port}"
  netcore_set_first_toml_string "$config" advertised_endpoint "http://${ip}:${port}"

  # Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
  # Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
  install -d -m 0755 "$state_dir"
  cat >"${state_dir}/lxc-network.env" <<STATE
NETCORE_SERVICE=${service}
NETCORE_LXC_IP=${ip}
NETCORE_WEBUI_PORT=${port}
NETCORE_WEBUI_URL=${webui}
STATE
  chmod 0644 "${state_dir}/lxc-network.env"

  export NETCORE_DETECTED_LXC_IP=$ip
  export NETCORE_WEBUI_URL=$webui

  echo "LXC-Adresse erkannt: ${ip} (DHCP/static lease)"
  echo "WebUI: ${webui}"
}
