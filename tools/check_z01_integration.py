#!/usr/bin/env python3
"""Run the shared, offline Z01 source/configuration gate; no SSH or radio access."""
import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]


def main():
    commands = [
        [sys.executable, "deploy/open-lab/netcore-deploy.py", "validate"],
        [sys.executable, "deploy/open-lab/netcore-deploy.py", "check-generated"],
        [sys.executable, "tools/check_shared_platform.py"],
        [sys.executable, "tools/check_e2e_integration.py"],
        [sys.executable, "tools/check_full_system_integration.py"],
        [sys.executable, "tools/check_observability.py"],
    ]
    env = dict(os.environ, PYTHONDONTWRITEBYTECODE="1", SOURCE_DATE_EPOCH="0")
    failures = []
    for command in commands:
        print("CHECK " + " ".join(command[1:]), flush=True)
        result = subprocess.run(command, cwd=ROOT, env=env)
        if result.returncode:
            failures.append(" ".join(command[1:]))
    if failures:
        print("Z01 source/configuration gate failed: " + "; ".join(failures), file=sys.stderr)
        return 1
    print("Z01 source/configuration gate: PASS; VM/Pi/LXC and on-air acceptance remain separate")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
