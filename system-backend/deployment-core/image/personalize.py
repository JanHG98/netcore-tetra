#!/usr/bin/env python3
"""Station customization inside a fresh image. Reads secrets from a root-only file."""
import grp
import json
import os
from pathlib import Path
import pwd
import shutil
import subprocess
import tomllib

from common import atomic_write, toml_dump
from deploy import install_dropin
from image_spec import validate_image, wifi_keyfile


def call(*args, **kwargs):
    # No command logging: passwords/keys are never emitted to job logs.
    subprocess.run(args, check=True, **kwargs)


def personalize():
    data = json.loads(Path('/root/netcore-image-request.json').read_text())
    req = validate_image(data, data.get('allowed_networks'))
    name = req['username']
    try:
        pwd.getpwnam(name)
        raise ValueError('Betriebssystem-Benutzer existiert bereits im Basisimage')
    except KeyError:
        pass
    call('useradd', '--create-home', '--shell', '/bin/bash', '--groups', 'sudo,audio,video,spi,gpio', name)
    if req['password_hash']:
        call('chpasswd', '--encrypted', input=name + ':' + req['password_hash'] + '\n', text=True)
    user = pwd.getpwnam(name)
    ssh = Path(user.pw_dir) / '.ssh'
    if req['ssh_key']:
        ssh.mkdir(mode=0o700)
        atomic_write(ssh / 'authorized_keys', req['ssh_key'] + '\n')
        os.chown(ssh, user.pw_uid, user.pw_gid)
        os.chown(ssh / 'authorized_keys', user.pw_uid, user.pw_gid)
    atomic_write('/etc/sudoers.d/090-netcore-user', name + ' ALL=(ALL:ALL) NOPASSWD: ALL\n', 0o440)
    # Remove any stock autologin, never ship a shared/default login password.
    for directory in ('/etc/systemd/system/getty@tty1.service.d', '/etc/systemd/system/serial-getty@serial0.service.d'):
        if Path(directory).is_dir():
            shutil.rmtree(directory)
    try:
        pwd.getpwnam('pi')
        call('usermod', '--lock', '--shell', '/usr/sbin/nologin', 'pi')
    except KeyError:
        pass
    atomic_write('/etc/ssh/sshd_config.d/20-netcore.conf',
                 'PermitRootLogin no\nPasswordAuthentication ' + ('yes' if req['password_hash'] else 'no') + '\n', 0o644)
    Path('/etc/ssh/sshd_config.d/rename_user.conf').unlink(missing_ok=True)
    Path('/var/lib/userconf-pi/autologin').unlink(missing_ok=True)
    atomic_write('/etc/hostname', req['hostname'] + '\n', 0o644)
    hosts = Path('/etc/hosts').read_text().splitlines()
    hosts = [line for line in hosts if not line.startswith('127.0.1.1')]
    atomic_write('/etc/hosts', '\n'.join(hosts) + '\n127.0.1.1\t' + req['hostname'] + '\n', 0o644)
    atomic_write('/etc/timezone', req['timezone'] + '\n', 0o644)
    Path('/etc/localtime').unlink(missing_ok=True)
    Path('/etc/localtime').symlink_to('/usr/share/zoneinfo/' + req['timezone'])
    if req['wifi_ssid']:
        atomic_write('/etc/NetworkManager/system-connections/netcore-wifi.nmconnection',
                     wifi_keyfile(req['wifi_ssid'], req['wifi_password']))
    # Wired DHCP is explicit and does not assume the old station's reserved IP.
    atomic_write('/etc/NetworkManager/system-connections/netcore-lan.nmconnection',
                 '[connection]\nid=netcore-lan\ntype=ethernet\nautoconnect=true\n\n'
                 '[ethernet]\n\n[ipv4]\nmethod=auto\n\n[ipv6]\nmethod=auto\n')
    atomic_write('/etc/netcore/image-network.json', json.dumps({k: req[k] for k in
                 ('wifi_country', 'trusted_ssids', 'trusted_lan')}))
    call('systemctl', 'enable', 'netcore-wifi-country.service')
    if req['vpn_config']:
        atomic_write('/etc/openvpn/client/netcore.conf', req['vpn_config'])
        call('systemctl', 'enable', 'netcore-vpn-policy.timer')
    config = tomllib.loads(req['config'])
    config['service_name'] = 'tetra'
    config.setdefault('dashboard', {}).update(source_dir='/opt/netcore-tetra')
    for key in ('username', 'password'):
        config['dashboard'].pop(key, None)
    # Every image gets its own station identifiers; RF/SDR values remain from the template.
    from deploy import tbs_config
    rendered = tbs_config(toml_dump(config), req['profile'])
    for suffix in ('', '.fallback'):
        path = Path('/etc/netcore/config.toml' + suffix)
        atomic_write(path, rendered, 0o640)
        os.chown(path, 0, grp.getgrnam('netcore').gr_gid)
    agent = tomllib.loads(Path('/opt/netcore-tetra/system-backend/deployment-core/config/agent.example.toml').read_text())
    agent.update(node_id=req['profile']['name'], environment=req['environment'],
                 seeds=[req['controller_url']], allowed_networks=req['allowed_networks'],
                 tbs_template='/etc/netcore/config.toml',
                 services=[dict(name='tbs', unit='tetra.service', config_target='/etc/netcore/config.toml',
                                command='/usr/local/bin/bluestation-bs /etc/netcore/config.toml')])
    atomic_write('/etc/netcore/discovery.toml', toml_dump(agent), 0o644)
    spec = dict(name='tbs', unit='tetra.service', config_target='/etc/netcore/config.toml',
                command='/usr/local/bin/bluestation-bs /etc/netcore/config.toml')
    install_dropin(spec, Path('/var/lib/netcore-discovery/endpoints.json'))
    atomic_write('/var/lib/netcore-discovery/deployed-tbs.json',
                 json.dumps(dict(commit=req['commit'], image_preinstalled=True)))
    boot = Path('/boot/firmware/config.txt')
    if req['overlay'] == 'sx1255':
        # Opt-in only: don't load a second copy when the HAT EEPROM already supplies it.
        with boot.open('a') as file:
            file.write('\n[all]\n# NetCore: manual overlay for a HAT without a programmed EEPROM\n'
                       'dtparam=spi=on\ndtoverlay=netcore-sx1255\n')
    atomic_write('/usr/share/netcore-image/station.json',
                 json.dumps(dict(profile=req['profile'], commit=req['commit'], hostname=req['hostname'],
                                 overlay=req['overlay']), indent=2), 0o644)
    print('Station eingerichtet: ' + req['profile']['name'] + '. Netzwerk-/Zugangsdaten werden nicht protokolliert.')


if __name__ == '__main__':
    personalize()
