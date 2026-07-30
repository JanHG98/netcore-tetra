#!/usr/bin/env python3
"""Remove obsolete basis-station-local TTS settings from config.toml safely."""
from __future__ import annotations

import argparse
import datetime as dt
from pathlib import Path
import os
import shutil


def remove_local_tts(source: str) -> str:
    lines = source.splitlines()
    output: list[str] = []
    skipping_tts = False

    for line in lines:
        stripped = line.strip()
        is_tts_header = stripped == "[tts]" or stripped.startswith("[[tts.")
        is_any_header = stripped.startswith("[") and stripped.endswith("]")

        if is_tts_header:
            skipping_tts = True
            continue
        if skipping_tts:
            if is_any_header and not is_tts_header:
                skipping_tts = False
            else:
                continue

        if stripped.startswith("tts_archive_enabled") and "=" in stripped:
            continue
        if stripped.startswith("tts_archive_directory") and "=" in stripped:
            continue
        output.append(line)

    # Collapse excessive blank lines left by the removed block.
    compact: list[str] = []
    blank_count = 0
    for line in output:
        if line.strip():
            blank_count = 0
            compact.append(line)
        else:
            blank_count += 1
            if blank_count <= 2:
                compact.append(line)
    return "\n".join(compact).rstrip() + "\n"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("config", nargs="?", default="/etc/netcore/config.toml")
    args = parser.parse_args()
    path = Path(args.config)
    if not path.is_file():
        print(f"[NetCore TTS migration] Config not found, skipped: {path}")
        return 0

    original_stat = path.stat()
    original = path.read_text(encoding="utf-8")
    updated = remove_local_tts(original)
    if updated == original:
        print("[NetCore TTS migration] No local [tts] configuration found.")
        return 0

    stamp = dt.datetime.now().strftime("%Y%m%d-%H%M%S")
    backup = path.with_name(f"{path.name}.pre-central-tts.{stamp}.bak")
    shutil.copy2(path, backup)
    temp = path.with_name(f".{path.name}.central-tts.tmp")
    temp.write_text(updated, encoding="utf-8")
    temp.chmod(original_stat.st_mode & 0o7777)
    # The migration runs as root. Preserve the original config owner/group before
    # the atomic replace, otherwise the file becomes root:root and a service
    # running as netcore can no longer pass ExecStartPre=/usr/bin/test -r.
    os.chown(temp, original_stat.st_uid, original_stat.st_gid)
    temp.replace(path)
    print(f"[NetCore TTS migration] Removed local TTS settings; backup: {backup}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
