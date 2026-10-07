#!/usr/bin/env python3
"""Check the actual installed services on a disposable Ubuntu CI VM."""
import json
from pathlib import Path
import subprocess
import time
import tomllib
from urllib.request import urlopen


def main():
    units = ['netcore-deployment.service', 'netcore-image-builder.service']
    for unit in units:
        for verb in ('is-enabled', 'is-active'):
            subprocess.run(['systemctl', verb, '--quiet', unit], check=True)
    config = tomllib.loads(Path('/etc/netcore/deployment.toml').read_text())
    assert config['node_id'] == 'ci-deploy', config
    assert config['advertise_url'] == 'http://127.0.0.1:8320', config
    base = 'http://127.0.0.1:8320'
    deadline = time.monotonic() + 20
    while True:
        try:
            with urlopen(base + '/health/live', timeout=2) as response:
                assert json.load(response)['ok']
            with urlopen(base + '/api/v1/images', timeout=2) as response:
                images = json.load(response)
            assert images['available'] is True, images
            assert images['jobs'] == [], images
            with urlopen(base + '/', timeout=2) as response:
                assert response.status == 200
                assert 'text/html' in response.headers['Content-Type']
            break
        except (OSError, AssertionError):
            if time.monotonic() >= deadline:
                raise
            time.sleep(0.25)
    print('PASS: installed systemd services, HTTP UI, controller-to-builder socket, preserved configuration.')


if __name__ == '__main__':
    main()
