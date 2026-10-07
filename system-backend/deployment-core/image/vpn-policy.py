#!/usr/bin/env python3
"""VPN off on trusted WLAN OR wired home subnet, on otherwise. No radio restarts."""
import ipaddress
import json
from pathlib import Path
import subprocess


def value(device, field):
    return subprocess.check_output(['nmcli', '--escape', 'no', '-g', field, 'device', 'show', device],
                                   text=True, timeout=10).strip()


def trusted(connections, ssids, network):
    network = ipaddress.ip_network(network)
    for link in connections:
        if link['type'] == 'wifi' and link.get('ssid') in ssids:
            return True
        if link['type'] == 'ethernet':
            for address in link.get('addresses', []):
                try:
                    if ipaddress.ip_interface(address).ip in network:
                        return True
                except ValueError:
                    pass
    return False


def main():
    cfg = json.loads(Path('/etc/netcore/image-network.json').read_text())
    devices = subprocess.check_output(['nmcli', '-t', '-f', 'DEVICE', 'device'], text=True, timeout=10).splitlines()
    links = []
    for device in devices:
        if not value(device, 'GENERAL.STATE').startswith('100 '):
            continue
        kind = value(device, 'GENERAL.TYPE')
        link = dict(type=kind, addresses=value(device, 'IP4.ADDRESS').splitlines())
        if kind == 'wifi':
            connection = value(device, 'GENERAL.CON-UUID')
            link['ssid'] = subprocess.check_output(['nmcli', '--escape', 'no', '-g', '802-11-wireless.ssid',
                                                    'connection', 'show', 'uuid', connection], text=True, timeout=10).strip()
        links.append(link)
    local = trusted(links, cfg['trusted_ssids'], cfg['trusted_lan'])
    subprocess.run(['systemctl', 'stop' if local else 'start', 'openvpn-client@netcore.service'], check=True, timeout=30)


if __name__ == '__main__':
    main()
