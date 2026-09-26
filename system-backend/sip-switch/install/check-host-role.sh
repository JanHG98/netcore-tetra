#!/usr/bin/env bash
# Sourceable, read-only preflight for mutually exclusive Asterisk host roles.

netcore_sip_has_include() {
  local file="$1" wanted="$2" line target
  [[ -f "$file" ]] || return 1
  while IFS= read -r line || [[ -n "$line" ]]; do
    # Asterisk permits quoted/absolute include paths and trailing comments.
    if [[ "$line" =~ ^[[:space:]]*#(try)?include[[:space:]]+([^\;]+) ]]; then
      target="${BASH_REMATCH[2]}"
      target="${target//\"/}"
      target="${target//\</}"
      target="${target//\>/}"
      target="${target//[[:space:]]/}"
      [[ "${target##*/}" == "$wanted" ]] && return 0
    fi
  done <"$file"
  return 1
}

netcore_check_sip_host_role() {
  # The optional root is for isolated tests; installers always use the real host.
  local role="$1" root="${2:-}" other config unit pair file include evidence=""
  local -a includes
  case "$role" in
    central)
      other="TBS-Fallback"
      config="tbs-sip-fallback.toml"
      unit="netcore-tbs-sip-failover.service"
      includes=("pjsip.conf:netcore-tbs-fallback-pjsip.conf" "pjsip.conf:netcore-active-registration.conf" "extensions.conf:netcore-tbs-fallback-extensions.conf" "rtp.conf:netcore-tbs-fallback-rtp.conf")
      ;;
    tbs)
      other="zentraler SIP-Switch"
      config="sip-switch.toml"
      unit="netcore-sip-switch.service"
      includes=("pjsip.conf:netcore-pjsip.conf" "extensions.conf:netcore-extensions.conf" "rtp.conf:netcore-rtp.conf")
      ;;
    *) printf 'Unbekannte SIP-Hostrolle.\n' >&2; return 2 ;;
  esac
  if [[ -e "${root}/etc/netcore/${config}" ]]; then
    evidence="/etc/netcore/${config}"
  else
    for pair in "${includes[@]}"; do
      file="${pair%%:*}"; include="${pair#*:}"
      if netcore_sip_has_include "${root}/etc/asterisk/${file}" "$include"; then
        evidence="/etc/asterisk/${file}: ${include}"
        break
      fi
    done
    if [[ -z "$evidence" ]] && command -v systemctl >/dev/null 2>&1; then
      if systemctl is-active --quiet "$unit" 2>/dev/null || systemctl is-enabled --quiet "$unit" 2>/dev/null; then
        evidence="$unit (aktiv oder aktiviert)"
      fi
    fi
  fi
  if [[ -n "$evidence" ]]; then
    printf 'SIP-Installation/Update abgebrochen: Auf diesem Host ist bereits die Rolle "%s" eingerichtet (%s).\n' "$other" "$evidence" >&2
    printf 'TBS-Fallback und zentralen SIP-Switch auf getrennten Hosts betreiben. Eine Fehlinstallation zuerst gezielt zurücknehmen; keine Dateien oder Dienste wurden geändert.\n' >&2
    return 2
  fi
}

netcore_check_sip_install_args() {
  local argument index=0
  for argument in "$@"; do
    index=$((index + 1))
    if [[ "$argument" =~ \<[^\<\>]+\> ]]; then
      printf 'Installation abgebrochen: Argument %s enthält noch einen Platzhalter. Tatsächlichen Wert einsetzen; keine Dateien oder Dienste wurden geändert.\n' "$index" >&2
      return 2
    fi
  done
}
