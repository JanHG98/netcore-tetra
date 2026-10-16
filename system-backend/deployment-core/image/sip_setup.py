#!/usr/bin/env python3
"""Provision the local Phase 11c PBX offline from explicit station credentials."""
import copy
import importlib.util
import re
import secrets
from pathlib import Path
import subprocess
import tomllib


FIXED_PATHS = {
    'asterisk': {
        'binary': '/usr/sbin/asterisk', 'config_dir': '/etc/asterisk',
        'native_snippet_file': '/etc/netcore/tbs-asterisk-local-snippet.toml',
    },
    'failover': {
        'state_file': '/var/lib/netcore-tbs-sip-fallback/state.json',
        'lock_file': '/run/netcore-tbs-sip-fallback.lock',
        'active_registration_file': '/etc/asterisk/netcore-active-registration.conf',
        'central_registration_file': '/etc/asterisk/netcore-registration-central.conf',
        'pbx_registration_file': '/etc/asterisk/netcore-registration-pbx-direct.conf',
    },
}


def validate_sip_config(text, profile_name):
    """Validate supplied upstream identities; never fill external credentials."""
    if not isinstance(text, str) or len(text) > 65536 or '\x00' in text:
        raise ValueError('SIP-Fallback: ungültige oder zu große TOML-Konfiguration')
    if not text.strip():
        return None
    try:
        data = tomllib.loads(text)
    except tomllib.TOMLDecodeError:
        # Parser exceptions may contain a credential-bearing source line.
        raise ValueError('SIP-Fallback: ungültiges TOML') from None
    if not re.fullmatch(r'[A-Za-z0-9_.:-]{1,120}', profile_name):
        raise ValueError('SIP-Fallback: ungültige Stationskennung')
    for section in ('service', 'asterisk', 'native_tbs', 'central', 'fallback_pbx', 'failover', 'routing'):
        if section in data and not isinstance(data[section], dict):
            raise ValueError('SIP-Fallback: ungültiger Abschnitt ' + section)
        data.setdefault(section, {})
    node = data['service'].get('node_id', profile_name)
    if node != profile_name:
        raise ValueError('SIP-Fallback: Stationskennung passt nicht zum ausgewählten TBS-Profil')

    def check_values(value):
        if isinstance(value, dict):
            for item in value.values():
                check_values(item)
        elif isinstance(value, list):
            for item in value:
                check_values(item)
        elif isinstance(value, str) and any(c in value for c in '\r\n\x00'):
            raise ValueError('SIP-Fallback: Werte dürfen keine Zeilenumbrüche enthalten')
    check_values(data)

    if data['central'].get('enabled', True) is not True:
        raise ValueError('SIP-Fallback: Zentralweg muss aktiviert sein')
    pbx_enabled = data['fallback_pbx'].get('enabled', bool(data['fallback_pbx']))
    if type(pbx_enabled) is not bool:
        raise ValueError('SIP-Fallback: ungültige FreePBX-Aktivierung')
    if not pbx_enabled:
        # Do not retain inactive external credentials or render a hidden PBX route.
        data['fallback_pbx'] = {'enabled': False}
    else:
        data['fallback_pbx']['enabled'] = True
    for section in ('central', 'fallback_pbx'):
        upstream = data[section]
        if section == 'fallback_pbx' and not pbx_enabled:
            continue
        for key in ('host', 'username'):
            value = upstream.get(key, '')
            if not isinstance(value, str) or not value.strip():
                raise ValueError('SIP-Fallback: ' + section + '.' + key + ' fehlt')
        for key in ('password', 'auth_username', 'from_user', 'from_domain', 'contact_user', 'realm', 'allow'):
            if key in upstream and not isinstance(upstream[key], str):
                raise ValueError('SIP-Fallback: ungültiges Feld ' + section + '.' + key)
        host = upstream['host']
        if not re.fullmatch(r'[A-Za-z0-9][A-Za-z0-9.-]{0,252}', host) or host in ('localhost', '127.0.0.1', '0.0.0.0'):
            raise ValueError('SIP-Fallback: ' + section + '.host muss eine erreichbare IPv4-Adresse oder DNS-Adresse sein')
        port = upstream.get('port', 5060)
        if type(port) is not int or not 1 <= port <= 65535:
            raise ValueError('SIP-Fallback: ungültiger Port für ' + section)
    password = data['central'].get('password')
    if not isinstance(password, str) or not password:
        raise ValueError('SIP-Fallback: central.password fehlt; Zugang am zentralen SIP-Switch einrichten')
    pbx = data['fallback_pbx']
    if pbx_enabled and pbx.get('mode', 'registration') != 'registration':
        raise ValueError('SIP-Fallback: fallback_pbx.mode muss registration sein')
    if pbx_enabled and pbx.get('auth_username') and not pbx.get('password'):
        raise ValueError('SIP-Fallback: PBX-Passwort für die angegebene Anmeldung fehlt')
    for section, keys in {
        'asterisk': ('rtp_start', 'rtp_end', 'central_dial_timeout_secs', 'fallback_dial_timeout_secs', 'tetra_dial_timeout_secs'),
        'native_tbs': ('bind_port',),
        'central': ('qualify_frequency_secs', 'qualify_timeout_secs', 'registration_expiration_secs'),
        'fallback_pbx': ('qualify_frequency_secs', 'qualify_timeout_secs', 'registration_expiration_secs'),
        'failover': ('check_interval_secs', 'startup_grace_secs', 'failure_threshold', 'recovery_stable_secs',
                     'central_registration_grace_secs', 'unregister_grace_secs'),
    }.items():
        for key in keys:
            if key in data[section] and (type(data[section][key]) is not int or not 0 <= data[section][key] <= 65535):
                raise ValueError('SIP-Fallback: ungültiger Zahlenwert ' + section + '.' + key)
    native_port = data['native_tbs'].get('bind_port', 5062)
    if native_port in (0, 5060):
        raise ValueError('SIP-Fallback: nativer SIP-Port muss vom lokalen PBX-Port 5060 abweichen')
    start, end = data['asterisk'].get('rtp_start', 30200), data['asterisk'].get('rtp_end', 30300)
    if not 1 <= start <= end <= 65535:
        raise ValueError('SIP-Fallback: ungültiger RTP-Portbereich')
    for key in ('username', 'password'):
        if key in data['native_tbs'] and not isinstance(data['native_tbs'][key], str):
            raise ValueError('SIP-Fallback: ungültiges natives Anmeldefeld')
    if 'username' in data['native_tbs'] and not re.fullmatch(r'[A-Za-z0-9_.:@+-]{1,128}', data['native_tbs']['username']):
        raise ValueError('SIP-Fallback: ungültige native SIP-Kennung')
    data['service'].update(node_id=profile_name, phase='11c')
    data['asterisk'].update(FIXED_PATHS['asterisk'], sip_bind='0.0.0.0:5060')
    data['failover'].update(FIXED_PATHS['failover'], default_mode='central')
    data['native_tbs'].update(contact_host='127.0.0.1', bind_port=native_port, allow='ulaw')
    data['native_tbs'].setdefault('username', 'netcore-native-' + profile_name.lower())
    if not data['native_tbs']['username']:
        raise ValueError('SIP-Fallback: native_tbs.username darf nicht leer sein')
    return data


def configure_local_sip(text, profile_name, *, root=Path('/'), runner=subprocess.run):
    """Render private files and enable units; no SIP daemon runs in the build VM."""
    config = validate_sip_config(text, profile_name)
    if config is None:
        return None
    from common import atomic_write, toml_dump
    config = copy.deepcopy(config)
    # This credential is exclusively for the private, loopback TBS/PBX link.
    if not config['native_tbs'].get('password'):
        config['native_tbs']['password'] = secrets.token_urlsafe(32)

    def target(path):
        return root / path.lstrip('/')
    for section, values in FIXED_PATHS.items():
        for key, path in values.items():
            config[section][key] = str(target(path))
    config_file = target('/etc/netcore/tbs-sip-fallback.toml')
    config_file.parent.mkdir(parents=True, exist_ok=True)
    atomic_write(config_file, toml_dump(config), 0o600)
    executable = target('/usr/local/bin/netcore-tbs-sip-fallback')
    spec = importlib.util.spec_from_file_location('netcore_image_sip_fallback', executable)
    # Installed scripts have no .py suffix, so select the Python source loader.
    if spec is None:
        from importlib.machinery import SourceFileLoader
        spec = importlib.util.spec_from_loader('netcore_image_sip_fallback', SourceFileLoader('netcore_image_sip_fallback', str(executable)))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    for filename, includes in {
        'pjsip.conf': ('netcore-tbs-fallback-pjsip.conf', 'netcore-active-registration.conf'),
        'extensions.conf': ('netcore-tbs-fallback-extensions.conf',),
        'rtp.conf': ('netcore-tbs-fallback-rtp.conf',),
    }.items():
        path = target('/etc/asterisk/' + filename)
        previous = path.read_text() if path.exists() else ''
        lines = previous.splitlines()
        for include in includes:
            if '#include ' + include not in lines:
                lines.append('#include ' + include)
        module.write_asterisk_config(path, '\n'.join(lines) + '\n')
    module.render(config_file)
    runner(['systemctl', 'enable', 'asterisk.service', 'netcore-tbs-sip-failover.service'],
           check=True, capture_output=True, text=True)
    dropin = target('/etc/systemd/system/tetra.service.d/30-local-sip.conf')
    dropin.parent.mkdir(parents=True, exist_ok=True)
    atomic_write(dropin, '[Unit]\nAfter=asterisk.service netcore-tbs-sip-failover.service\n'
                 'Wants=asterisk.service netcore-tbs-sip-failover.service\n', 0o644)
    return tomllib.loads(Path(config['asterisk']['native_snippet_file']).read_text())['asterisk']
