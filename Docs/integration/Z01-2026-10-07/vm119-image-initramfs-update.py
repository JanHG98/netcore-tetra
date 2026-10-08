#!/usr/bin/env python3
"""VM119: replace only the guest initramfs recipe, preserving host configuration."""
import importlib.util
from pathlib import Path
import subprocess

SPEC = importlib.util.spec_from_file_location('vm119_guard', Path(__file__).with_name('vm119-jobs-update.py'))
UPDATER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(UPDATER)
UPDATER.TARGET = Path('/usr/local/lib/netcore-deployment/image/build-guest.sh')
UPDATER.SOURCE = Path(__file__).resolve().parents[3] / 'system-backend/deployment-core/image/build-guest.sh'
UPDATER.OLD_SHA = '84b8d4070730b2fe94c4ddf4514411fadd08f19f72bfb3ce0adc86f14d1d8c3c'
UPDATER.NEW_SHA = '4ecd8daeaff314d63da00407f49c994f3bff0569755dbd95640aef96289a8e68'


def validate_source(body):
    subprocess.run(['/bin/bash', '-n'], input=body, check=True, timeout=10)


UPDATER.validate_source = validate_source


if __name__ == '__main__':
    UPDATER.main()
