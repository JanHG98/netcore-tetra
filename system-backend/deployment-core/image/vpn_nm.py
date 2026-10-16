#!/usr/bin/env python3
"""Provision an OpenVPN NetworkManager profile offline, without a bus or GUI."""
import os
from pathlib import Path, PurePosixPath
import uuid

from common import atomic_write

VPN_KEYFILE = PurePosixPath('/etc/NetworkManager/system-connections/netcore-vpn.nmconnection')
VPN_MATERIAL = PurePosixPath('/etc/openvpn/client/netcore.d')
VPN_NAME = 'NetCore VPN'
SERVICE_TYPE = 'org.freedesktop.NetworkManager.openvpn'
INLINE_FILES = {'ca', 'cert', 'key', 'tls-auth', 'tls-crypt', 'tls-crypt-v2'}


def networkmanager():
    import gi
    gi.require_version('NM', '1.0')
    from gi.repository import NM
    return NM


def private_directory(path):
    path.mkdir(parents=True, exist_ok=True, mode=0o700)
    if path.is_symlink() or not path.is_dir():
        raise RuntimeError('OpenVPN: ungültiges privates Materialverzeichnis')
    path.chmod(0o700)


def materialize_inline(profile, directory):
    """Only process previously validated inline blocks; never read uploaded paths.

    Supplying explicit private paths prevents the upstream importer from
    extracting inline certificates into the build user's home directory.
    """
    result, block, material = [], None, []
    for raw in profile.splitlines():
        line = raw.strip()
        if block is not None:
            if line == '</' + block + '>':
                path = directory / (block + '.pem')
                atomic_write(path, '\n'.join(material) + '\n', 0o600)
                # Generated absolute paths have no user-controlled quoting.
                result.append(block + ' "' + str(path) + '"')
                block, material = None, []
            else:
                material.append(raw)
        elif line.startswith('<') and line.endswith('>') and line[1:-1] in INLINE_FILES:
            block, material = line[1:-1], []
        else:
            result.append(raw)
    if block is not None:
        raise RuntimeError('OpenVPN: unvollständiger Inline-Block')
    return '\n'.join(result).rstrip() + '\n'


def render(req, *, root=Path('/'), nm=None):
    """Persist one system-wide profile with secrets available before login.

    The OpenVPN editor plugin performs the import; libnm writes and validates
    the keyfile, including correct escaping of credentials. No running
    NetworkManager, secret agent, desktop session or network is required.
    """
    nm = networkmanager() if nm is None else nm
    directory = root / str(VPN_MATERIAL).lstrip('/')
    private_directory(directory)
    source = directory / 'nm-import.ovpn'
    atomic_write(source, materialize_inline(req['vpn_config'], directory), 0o600)
    try:
        plugin = next((info.load_editor_plugin() for info in nm.VpnPluginInfo.list_load()
                       if info.get_service() == SERVICE_TYPE), None)
        if plugin is None:
            raise RuntimeError('OpenVPN-Importmodul fehlt')
        connection = plugin.import_(str(source))
        setting = connection.get_setting_connection()
        setting.set_property('id', VPN_NAME)
        setting.set_property('uuid', str(uuid.uuid4()))
        # VPN activation belongs to the trusted-network policy, not a user's
        # login/keyring and not a second openvpn-client systemd instance.
        setting.set_property('autoconnect', False)
        vpn = connection.get_setting_vpn()
        if req['vpn_username']:
            vpn.add_data_item('username', req['vpn_username'])
            vpn.add_data_item('password-flags', '0')
            vpn.add_secret('password', req['vpn_password'])
            vpn.set_secret_flags('password', nm.SettingSecretFlags.NONE)
        connection.normalize()
        connection.verify()
        data = nm.keyfile_write(connection, nm.KeyfileHandlerFlags.NONE, None, None)
        text, _ = data.to_data()
        destination = root / str(VPN_KEYFILE).lstrip('/')
        atomic_write(destination, text, 0o600)
        return connection.get_uuid()
    except Exception:
        # Plugin diagnostics can contain uploaded tokens/material. Keep them
        # out of the public build log; no unusable image may be published.
        raise RuntimeError('OpenVPN: NetworkManager-Profil konnte nicht vollständig importiert werden; '
                           'Importmodul und portable Client-Datei prüfen') from None
    finally:
        source.unlink(missing_ok=True)


if __name__ == '__main__':
    # Upgrade an existing NetCore image using its already private local files.
    # No credential needs to appear in argv, a terminal or a build log.
    import json
    import subprocess
    from image_spec import VPN_AUTH_PATH, validate_vpn_material

    if os.geteuid() != 0:
        raise SystemExit('root erforderlich')
    config = Path('/etc/openvpn/client/netcore.conf')
    credentials = Path(VPN_AUTH_PATH)
    user = password = ''
    if credentials.is_file():
        user, password = credentials.read_text().splitlines()
    profile, user, password = validate_vpn_material(config.read_text(), user, password)
    vpn_uuid = render(dict(vpn_config=profile, vpn_username=user, vpn_password=password))
    network = Path('/etc/netcore/image-network.json')
    state = json.loads(network.read_text())
    state['vpn_uuid'] = vpn_uuid
    atomic_write(network, json.dumps(state), 0o600)
    subprocess.run(['nmcli', 'connection', 'reload'], check=True, timeout=15)
    # Stop the old tunnel only once the replacement profile is persisted/loaded.
    subprocess.run(['systemctl', 'disable', '--now', 'openvpn-client@netcore.service'],
                   check=True, timeout=30)
    subprocess.run(['systemctl', 'enable', '--now', 'netcore-vpn-policy.timer'],
                   check=True, timeout=15)
    subprocess.run(['systemctl', 'start', 'netcore-vpn-policy.service'], check=True, timeout=60)
    print('NetCore VPN eingerichtet; gespeicherte Zugangsdaten werden nicht ausgegeben.')
