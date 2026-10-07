"""Validated image recipes and requests. No disk operations in the web process."""
import copy
import base64
import binascii
import ipaddress
import re
import subprocess
import tomllib
from zoneinfo import ZoneInfo, ZoneInfoNotFoundError

from common import SHA, peer_url
from deploy import validate_profile

BASE = {
    'id': 'raspios-bookworm-arm64',
    'label': 'Raspberry Pi OS Lite · Bookworm 64 Bit · Pi 4 / 5',
    'url': 'https://downloads.raspberrypi.com/raspios_lite_arm64/images/raspios_lite_arm64-2025-05-13/2025-05-13-raspios-bookworm-arm64-lite.img.xz',
    'sha256': '62d025b9bc7ca0e1facfec74ae56ac13978b6745c58177f081d39fbb8041ed45',
    'soapy_repository': 'https://github.com/tejeez/sxxcvr.git',
    'soapy_commit': '9705147dd8c189625071f3f163ea56119bda4a05',
}
NETWORKS = ['10.0.0.0/8', '172.16.0.0/12', '192.168.0.0/16', '127.0.0.0/8']
BUILD_ID = re.compile(r'[0-9a-f]{32}\Z')


def text(data, key, default='', maximum=4096):
    value = data.get(key, default)
    if not isinstance(value, str) or len(value) > maximum or '\x00' in value:
        raise ValueError(f'Ungültiges Feld: {key}')
    return value


def validate_vpn(value):
    """Accept a portable OpenVPN client with inline credentials, never hooks/files."""
    if not value:
        return ''
    allowed = {'client', 'dev', 'proto', 'remote', 'resolv-retry', 'nobind', 'persist-key',
               'persist-tun', 'remote-cert-tls', 'verify-x509-name', 'cipher', 'data-ciphers',
               'data-ciphers-fallback', 'auth', 'auth-nocache', 'verb', 'mute', 'float',
               'explicit-exit-notify', 'key-direction', 'tls-version-min', 'tls-cipher',
               'tls-ciphersuites', 'connect-retry', 'connect-timeout', 'reneg-sec',
               'sndbuf', 'rcvbuf', 'tun-mtu', 'mssfix', 'route', 'route-nopull',
               'route-metric', 'pull', 'pull-filter', 'redirect-gateway', 'dhcp-option',
               'remote-random', 'remote-random-hostname', 'server-poll-timeout',
               'ping', 'ping-restart', 'keepalive', 'compress', 'comp-lzo', 'allow-compression'}
    tags = {'ca', 'cert', 'key', 'tls-auth', 'tls-crypt', 'tls-crypt-v2', 'auth-user-pass'}
    block = None
    remote = client = False
    for raw in value.splitlines():
        line = raw.strip()
        if not line or line.startswith(('#', ';')):
            continue
        if block:
            if line == f'</{block}>':
                block = None
            continue
        if line.startswith('<'):
            block = line[1:-1] if line.endswith('>') else None
            if block not in tags:
                raise ValueError('OpenVPN: nur Inline-Zertifikate/-Schlüssel und Inline-Zugangsdaten')
            continue
        parts = line.split()
        if parts[0] not in allowed or '\\' in line:
            raise ValueError('OpenVPN: nicht unterstützte Direktive ' + parts[0])
        if parts[0] == 'dev' and parts[1:] != ['tun']:
            raise ValueError('OpenVPN: dev tun erforderlich')
        remote |= parts[0] == 'remote'
        client |= parts[0] == 'client'
    if block or not remote or not client:
        raise ValueError('Vollständige OpenVPN-Client-Datei mit remote und geschlossenen Inline-Blöcken erforderlich')
    return value.rstrip() + '\n'


def validate_ssh_key(value):
    if not value:
        return
    if len(value.splitlines()) != 1 or not re.fullmatch(
            r'(ssh-ed25519|ssh-rsa|ecdsa-sha2-nistp(?:256|384|521)) [A-Za-z0-9+/=]+(?: [^\r\n]*)?', value):
        raise ValueError('Einen SSH-Public-Key ohne authorized_keys-Optionen eingeben')
    kind, encoded = value.split(' ', 2)[:2]
    try:
        raw = base64.b64decode(encoded, validate=True)
        fields = []
        while raw:
            if len(raw) < 4:
                raise ValueError()
            length = int.from_bytes(raw[:4], 'big')
            if not 0 < length <= len(raw) - 4:
                raise ValueError()
            fields.append(raw[4:4 + length])
            raw = raw[4 + length:]
        if not fields or fields[0].decode() != kind:
            raise ValueError()
        if kind == 'ssh-ed25519' and not (len(fields) == 2 and len(fields[1]) == 32):
            raise ValueError()
        if kind == 'ssh-rsa' and not (len(fields) == 3 and 1 <= len(fields[1]) <= 8 and len(fields[2]) >= 128):
            raise ValueError()
        if kind.startswith('ecdsa') and not (len(fields) == 3 and fields[1].decode() == kind[11:] and
                len(fields[2]) == {'nistp256': 65, 'nistp384': 97, 'nistp521': 133}[kind[11:]]):
            raise ValueError()
    except (ValueError, UnicodeError, binascii.Error) as exc:
        raise ValueError('Der SSH-Public-Key ist unvollständig oder beschädigt') from exc


def validate_image(data, networks=None, hash_password=True):
    if not isinstance(data, dict):
        raise ValueError('Image-Auftrag muss ein Objekt sein')
    profile = validate_profile(data.get('profile', {}))
    if data.get('base', BASE['id']) != BASE['id']:
        raise ValueError('Nicht unterstütztes Basisimage')
    commit = text(data, 'commit', maximum=40)
    if not SHA.fullmatch(commit):
        raise ValueError('Image benötigt einen geprüften vollständigen Git-Commit')
    template = text(data, 'config', maximum=196608)
    parsed = tomllib.loads(template)
    if not all(isinstance(parsed.get(k), dict) for k in ('net_info', 'cell_info', 'phy_io')):
        raise ValueError('Geprüfte TBS-Standortkonfiguration fehlt')
    hostname = text(data, 'hostname', profile['name'].lower(), 63).lower()
    if not re.fullmatch(r'[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?', hostname):
        raise ValueError('Hostname: 1–63 Zeichen, Buchstaben/Ziffern/Bindestrich')
    username = text(data, 'username', 'jan', 32)
    if not re.fullmatch(r'[a-z][a-z0-9_-]{0,31}', username) or username in ('root', 'netcore', 'nobody', 'daemon', 'pi'):
        raise ValueError('Ungültiger Benutzername; Standard ist jan')
    timezone = text(data, 'timezone', 'Europe/Berlin', 64)
    try:
        ZoneInfo(timezone)
    except (ValueError, ZoneInfoNotFoundError) as exc:
        raise ValueError('Ungültige Zeitzone') from exc
    public_key = text(data, 'ssh_key', maximum=8192).strip()
    validate_ssh_key(public_key)
    password = text(data, 'password', maximum=128)
    if '\n' in password or '\r' in password:
        raise ValueError('Passwort darf keine Zeilenumbrüche enthalten')
    password_hash = text(data, 'password_hash', maximum=200)
    if password_hash and not re.fullmatch(r'\$6\$[A-Za-z0-9./]{1,16}\$[A-Za-z0-9./]{86}', password_hash):
        raise ValueError('Ungültiger Passwort-Hash')
    if not (public_key or password or password_hash):
        raise ValueError('SSH-Public-Key oder Betriebssystem-Passwort für den Pi angeben')
    if password and hash_password:
        password_hash = subprocess.run(['openssl', 'passwd', '-6', '-stdin'], input=password + '\n',
                                       text=True, capture_output=True, check=True, timeout=10).stdout.strip()
    ssid = text(data, 'wifi_ssid', maximum=32)
    wifi_password = text(data, 'wifi_password', maximum=64)
    if len(ssid.encode()) > 32 or any(c in ssid + wifi_password for c in '\r\n'):
        raise ValueError('Ungültige WLAN-Daten')
    if ssid and not (8 <= len(wifi_password) <= 63 or re.fullmatch('[0-9a-fA-F]{64}', wifi_password)):
        raise ValueError('WLAN benötigt einen WPA2/WPA3-Schlüssel (8–63 Zeichen oder 64 Hex-Zeichen)')
    if wifi_password and not ssid:
        raise ValueError('WLAN-SSID fehlt')
    country = text(data, 'wifi_country', 'DE', 2).upper()
    if not re.fullmatch('[A-Z]{2}', country):
        raise ValueError('WLAN-Land als ISO-Code angeben')
    overlay = text(data, 'overlay', 'eeprom', 20)
    if overlay not in ('eeprom', 'sx1255'):
        raise ValueError('HAT-Konfiguration: eeprom oder sx1255')
    vpn = validate_vpn(text(data, 'vpn_config', maximum=65536))
    trusted_ssids = data.get('trusted_ssids', [])
    if not isinstance(trusted_ssids, list) or len(trusted_ssids) > 16 or any(
            not isinstance(s, str) or not 1 <= len(s.encode()) <= 32 or any(c in s for c in '\x00\r\n') for s in trusted_ssids):
        raise ValueError('Ungültige lokale WLAN-SSIDs')
    trusted_lan = text(data, 'trusted_lan', '10.0.1.0/24', 32)
    try:
        if ipaddress.ip_network(trusted_lan).version != 4:
            raise ValueError()
    except ValueError as exc:
        raise ValueError('Lokales LAN als IPv4-Netz angeben') from exc
    seed = peer_url(text(data, 'controller_url', maximum=128), networks or NETWORKS)
    if ipaddress.ip_address(seed.split('//')[1].split(':')[0]).is_loopback:
        raise ValueError('Für das Image die vom Pi erreichbare VM-IP angeben, keine Loopback-Adresse')
    environment = text(data, 'environment', 'netcore-openlab', 64)
    from common import identifier
    identifier(environment)
    return dict(profile=profile, base=BASE['id'], commit=commit, config=template, hostname=hostname,
                username=username, timezone=timezone, ssh_key=public_key, password_hash=password_hash,
                wifi_ssid=ssid, wifi_password=wifi_password, wifi_country=country, overlay=overlay,
                vpn_config=vpn, trusted_ssids=trusted_ssids, trusted_lan=trusted_lan,
                controller_url=seed, environment=environment, allowed_networks=networks or NETWORKS)


def public_job(job):
    """Never publish TBS, WLAN, VPN or OS credentials in the job API."""
    job = copy.deepcopy(job)
    req = job['request']
    job['request'] = {k: req[k] for k in ('build_id', 'profile', 'base', 'commit', 'hostname') if k in req}
    return job


def keyfile_escape(value):
    return value.replace('\\', '\\\\').replace('\n', '\\n').replace('\r', '\\r').replace('\t', '\\t').replace(' ', '\\s')


def wifi_keyfile(ssid, password):
    return ('[connection]\nid=netcore-wifi\ntype=wifi\nautoconnect=true\n\n'
            '[wifi]\nmode=infrastructure\nssid=' + keyfile_escape(ssid) + '\n\n'
            '[wifi-security]\nkey-mgmt=wpa-psk\npsk=' + keyfile_escape(password) + '\n\n'
            '[ipv4]\nmethod=auto\n\n[ipv6]\nmethod=auto\n')
