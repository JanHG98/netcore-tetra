#!/usr/bin/env python3
"""VM119: update only the reviewed tetra.service image recipe."""
import configparser
import hashlib
import importlib.util
from pathlib import Path

GUARD = Path(__file__).with_name('vm119-jobs-update.py')
GUARD_SHA = '50dfc518d9de6cffc5ec3335b3b94e6716d3c22de6653868f895386e64ca39db'
if GUARD.is_symlink() or hashlib.sha256(GUARD.read_bytes()).hexdigest() != GUARD_SHA:
    raise RuntimeError('Unreviewed VM119 update guard.')
SPEC = importlib.util.spec_from_file_location('vm119_media_guard', GUARD)
UPDATER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(UPDATER)
UPDATER.TARGET = Path('/usr/local/lib/netcore-deployment/image/systemd/tetra.service')
UPDATER.SOURCE = Path(__file__).resolve().parents[3] / 'system-backend/deployment-core/image/systemd/tetra.service'
UPDATER.OLD_SHA = '97ba337cb6d1ea6db1f659b8451b3fb6e95f30e24701401e4a3830c6d8170319'
UPDATER.NEW_SHA = '54cc9b2e88899621f03e22a7c5506d188ffa4af149bfbe0600284b0c122ce7f0'


def validate_source(body):
    unit = configparser.ConfigParser(interpolation=None, strict=False)
    unit.read_string(body.decode('utf-8'))
    if unit['Service']['User'] != 'netcore' or unit['Service']['Group'] != 'netcore':
        raise RuntimeError('Unexpected image service identity.')


UPDATER.validate_source = validate_source


if __name__ == '__main__':
    UPDATER.main()
