#!/usr/bin/env python3
"""Append the basis-station playout block to an existing Media Library config.

The migration is intentionally conservative: an existing [playout] section is
never rewritten. File owner, group and mode are preserved across the atomic
replacement so the systemd service keeps reading the configuration correctly.
"""

from __future__ import annotations

import argparse
import os
import pathlib
import re
import stat
import tempfile


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--config", required=True)
    parser.add_argument("--station-id", default="srv-m-tbs-01")
    parser.add_argument("--station-name", default="SRV-M-TBS-01")
    parser.add_argument("--station-url", default="http://10.0.1.22:8080")
    parser.add_argument("--username")
    parser.add_argument("--password")
    args = parser.parse_args()
    if bool(args.username) != bool(args.password):
        parser.error("--username and --password must be supplied together")

    path = pathlib.Path(args.config)
    text = path.read_text(encoding="utf-8")
    if re.search(r"(?m)^\s*\[playout\]\s*(?:#.*)?$", text):
        print(f"[Media Library] [playout] already exists in {path}; migration skipped.")
        return 0

    station_id = toml_string(args.station_id.strip())
    station_name = toml_string(args.station_name.strip())
    station_url = toml_string(args.station_url.strip().rstrip("/"))
    auth_block = ""
    if args.username and args.password:
        auth_block = (
            f"\nusername = {toml_string(args.username)}"
            f"\npassword = {toml_string(args.password)}"
        )
    addition = f"""

# Added by NetCore Media Library playout migration.
# The TBS audio player downloads the complete preview WAV from this Media Library,
# performs native TETRA encoding locally, creates the call and releases it afterwards.
[playout]
mode = "basisstation"
default_station = {station_id}
request_timeout_secs = 15
completion_timeout_secs = 900
poll_interval_ms = 500

[[playout.stations]]
id = {station_id}
name = {station_name}
base_url = {station_url}
enabled = true{auth_block}
"""

    st = path.stat()
    parent = path.parent
    fd, temp_name = tempfile.mkstemp(prefix=f".{path.name}.", dir=parent)
    temp_path = pathlib.Path(temp_name)
    try:
        with os.fdopen(fd, "w", encoding="utf-8", newline="\n") as handle:
            handle.write(text.rstrip())
            handle.write(addition)
            handle.write("\n")
            handle.flush()
            os.fsync(handle.fileno())
        os.chown(temp_path, st.st_uid, st.st_gid)
        os.chmod(temp_path, stat.S_IMODE(st.st_mode))
        os.replace(temp_path, path)
        directory_fd = os.open(parent, os.O_DIRECTORY)
        try:
            os.fsync(directory_fd)
        finally:
            os.close(directory_fd)
    finally:
        if temp_path.exists():
            temp_path.unlink()

    print(
        f"[Media Library] Added basis-station playout target "
        f"{args.station_id} -> {args.station_url} to {path}."
    )
    return 0


def toml_string(value: str) -> str:
    return '"' + value.replace("\\", "\\\\").replace('"', '\\"') + '"'


if __name__ == "__main__":
    raise SystemExit(main())
