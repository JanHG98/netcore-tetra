#!/usr/bin/env python3
from __future__ import annotations

import argparse
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--config", default="/etc/netcore/tbs-sip-fallback.toml")
    args = parser.parse_args()
    path = Path(args.config)
    text = path.read_text(encoding="utf-8")
    original = text
    text = text.replace('phase = "11b"', 'phase = "11c"', 1)

    lines = text.splitlines()
    section = ""
    from_user = ""
    username_index = None
    mode_index = None
    for idx, line in enumerate(lines):
        stripped = line.strip()
        if stripped.startswith("[") and stripped.endswith("]"):
            section = stripped
            continue
        if section == "[fallback_pbx]":
            if stripped.startswith("from_user") and "=" in line:
                from_user = line.split("=", 1)[1].strip().strip('"')
            if stripped.startswith("username") and "=" in line:
                username_index = idx
            if stripped.startswith("mode") and "=" in line:
                mode_index = idx
    if mode_index is not None:
        lines[mode_index] = 'mode = "registration"'
    if username_index is not None and lines[username_index].split("=", 1)[1].strip().strip('"') == "":
        lines[username_index] = f'username = "{from_user or "netcore-tbs-fallback"}"'
    text = "\n".join(lines).rstrip() + "\n"

    if "[failover]" not in text:
        text += '''\n[failover]\nenabled = true\ncheck_interval_secs = 2\nstartup_grace_secs = 10\nfailure_threshold = 3\nrecovery_stable_secs = 30\ncentral_registration_grace_secs = 15\nunregister_grace_secs = 2\nstate_file = "/var/lib/netcore-tbs-sip-fallback/state.json"\nlock_file = "/run/netcore-tbs-sip-fallback.lock"\nactive_registration_file = "/etc/asterisk/netcore-active-registration.conf"\ncentral_registration_file = "/etc/asterisk/netcore-registration-central.conf"\npbx_registration_file = "/etc/asterisk/netcore-registration-pbx-direct.conf"\ndefault_mode = "central"\n'''
    if text != original:
        backup = path.with_name(path.name + ".pre-phase11c")
        backup.write_text(original, encoding="utf-8")
        path.write_text(text, encoding="utf-8")
        print(f"migrated {path}; backup {backup}")
    else:
        print(f"already phase11c: {path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
