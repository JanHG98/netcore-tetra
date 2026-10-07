#!/usr/bin/env python3
import json
from pathlib import Path
import subprocess

country = json.loads(Path('/etc/netcore/image-network.json').read_text())['wifi_country']
subprocess.run(['rfkill', 'unblock', 'wifi'], check=False)
subprocess.run(['iw', 'reg', 'set', country], check=True)
