#!/usr/bin/env python3
"""Control the same VPN profile shown in the desktop, independent of login."""
import ipaddress
import json
import os
from pathlib import Path
import subprocess
import uuid


def query(*args):
    return subprocess.check_output(['nmcli', '--escape', 'no', *args],
                                   text=True, timeout=10,
                                   env=dict(os.environ, LC_ALL='C')).strip()


def value(device, field):
    return query('-g', field, 'device', 'show', device)


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


def apply_policy(cfg, links, active, *, runner=subprocess.run):
    # A missing migration is an error, not a fallback which hides another tunnel
    # from KDE and can accidentally run two VPN clients at the same time.
    vpn_uuid = str(uuid.UUID(cfg['vpn_uuid']))
    local = trusted(links, cfg['trusted_ssids'], cfg['trusted_lan'])
    if local and vpn_uuid in active:
        runner(['nmcli', '--wait', '20', 'connection', 'down', 'uuid', vpn_uuid],
               check=True, timeout=25)
        return 'disconnected_home'
    if not local and links and vpn_uuid not in active:
        runner(['nmcli', '--wait', '20', 'connection', 'up', 'uuid', vpn_uuid],
               check=True, timeout=25)
        return 'connecting'
    return 'home' if local else 'active' if vpn_uuid in active else 'waiting_network'


def main(config_path=Path('/etc/netcore/image-network.json')):
    cfg = json.loads(config_path.read_text())
    devices = query('-t', '-f', 'DEVICE', 'device').splitlines()
    links = []
    for device in devices:
        if not value(device, 'GENERAL.STATE').startswith('100 '):
            continue
        kind = value(device, 'GENERAL.TYPE')
        # A VPN or bridge address must never identify a wired trusted subnet.
        if kind not in ('wifi', 'ethernet'):
            continue
        link = dict(type=kind, addresses=value(device, 'IP4.ADDRESS').splitlines())
        if kind == 'wifi':
            connection = value(device, 'GENERAL.CON-UUID')
            link['ssid'] = query('-g', '802-11-wireless.ssid', 'connection', 'show', 'uuid', connection)
        links.append(link)
    # Includes activating connections, avoiding a second activation while nmcli
    # is still waiting for the first handshake.
    active = set(query('-g', 'UUID', 'connection', 'show', '--active').splitlines())
    result = apply_policy(cfg, links, active)
    print('NetCore VPN policy: ' + result, flush=True)


if __name__ == '__main__':
    main()
