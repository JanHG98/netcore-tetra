#!/usr/bin/env python3
"""VM119: replace only image_build.py using the reviewed idle/rollback guards."""
import importlib.util
from pathlib import Path

SPEC = importlib.util.spec_from_file_location('vm119_guard', Path(__file__).with_name('vm119-jobs-update.py'))
UPDATER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(UPDATER)
UPDATER.TARGET = Path('/usr/local/lib/netcore-deployment/image_build.py')
UPDATER.SOURCE = Path(__file__).resolve().parents[3] / 'system-backend/deployment-core/image_build.py'
UPDATER.OLD_SHA = '5a2db0699d6bb8ebc7e7915e7eca911ee0c810f24b9f1035c871a2b3d4a2f294'
UPDATER.NEW_SHA = '13aab8659a09bfddbe6aab3dce2bd193e8d5f62f9afcad3cfdce054a6dd61e61'


if __name__ == '__main__':
    UPDATER.main()
