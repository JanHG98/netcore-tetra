#!/usr/bin/env bash
# Source from the installers after installing rsyslog. Never disable its profile.
if [[ -f /etc/apparmor.d/usr.sbin.rsyslogd ]]; then
  if ! rg -q 'include.*<rsyslog.d>' /etc/apparmor.d/usr.sbin.rsyslogd 2>/dev/null; then
    # rg is optional on target LXCs.
    if ! grep -q 'include.*<rsyslog.d>' /etc/apparmor.d/usr.sbin.rsyslogd; then
      echo "rsyslog AppArmor profile has no snippets include; add logging/apparmor-netcore to its local include before enabling syslog." >&2
      exit 1
    fi
  fi
  install -d -m 0755 /etc/apparmor.d/rsyslog.d
  install -m 0644 "${DIR}/logging/apparmor-netcore" /etc/apparmor.d/rsyslog.d/netcore
  if [[ -r /sys/kernel/security/apparmor/profiles ]] && grep -q '^rsyslogd ' /sys/kernel/security/apparmor/profiles; then
    apparmor_parser -r /etc/apparmor.d/usr.sbin.rsyslogd || {
      echo "Cannot reload rsyslog AppArmor rules in this LXC; apply the snippet in its AppArmor namespace and retry." >&2
      exit 1
    }
  fi
fi
