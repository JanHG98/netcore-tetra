#!/usr/bin/env bash
# Run on the CENTRAL SIP switch (10.0.1.125), not on the Pi or deployment VM.
# This script uses existing central TBS credentials or creates that central entry.
# The direct PBX fallback account must already exist in FreePBX (10.0.1.21).
set -euo pipefail
sudo python3 - <<'PY'
import getpass
import json
import os
from pathlib import Path
import pwd
import secrets
import stat
import subprocess
import tomllib

node = 'SRV-M-TBS-01'
switch_file = Path('/etc/netcore/sip-switch.toml')
installer = Path('/opt/netcore-tetra/system-backend/sip-switch/install/add-tbs-openlab.sh')
filename = 'netcore-srv-m-tbs-01-sip-fallback.toml'

if os.geteuid() != 0 or not switch_file.is_file():
    raise SystemExit('Auf dem zentralen SIP-Switch mit sudo ausführen; dessen Konfiguration fehlt.')

# Resolve the real account home through the password database, never $HOME.
sudo_user = os.environ.get('SUDO_USER', '')
try:
    account = pwd.getpwnam(sudo_user) if sudo_user and sudo_user != 'root' else pwd.getpwuid(0)
except KeyError:
    raise SystemExit('SUDO_USER ist kein bekannter Betriebssystem-Benutzer.') from None
if account.pw_uid == 0:
    home = Path('/root')
else:
    sudo_uid = os.environ.get('SUDO_UID', '')
    if sudo_uid and sudo_uid != str(account.pw_uid):
        raise SystemExit('SUDO_USER und SUDO_UID passen nicht zusammen.')
    home = Path(account.pw_dir)
if not home.is_absolute():
    raise SystemExit('Das Benutzerverzeichnis muss ein absoluter Pfad sein.')
try:
    home = home.resolve(strict=True)
    metadata = home.stat()
except OSError:
    raise SystemExit('Das tatsächliche Benutzerverzeichnis ist nicht erreichbar.') from None
if not stat.S_ISDIR(metadata.st_mode) or metadata.st_uid != account.pw_uid:
    raise SystemExit('Das tatsächliche Benutzerverzeichnis gehört nicht zum ausführenden Benutzer.')
out = home / filename

# Pin the resolved directory and reserve a NEW private file before changing SIP.
# O_EXCL refuses any existing file or symlink, including a dangling symlink.
directory_fd = os.open(home, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
try:
    try:
        fd = os.open(filename, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW,
                     0o600, dir_fd=directory_fd)
    except FileExistsError:
        raise SystemExit('Ausgabedatei existiert bereits: ' + str(out) +
                         '. Bewusst sichern oder entfernen, dann erneut ausführen; SIP wurde nicht geändert.') from None
    file_identity = os.fstat(fd)
    complete = False
    try:
        try:
            switch = tomllib.loads(switch_file.read_text())
        except (OSError, tomllib.TOMLDecodeError):
            raise SystemExit('Die zentrale SIP-Konfiguration kann nicht gelesen werden.') from None
        matches = [tbs for tbs in switch.get('tbs', []) if tbs.get('node_id') == node]
        if len(matches) > 1:
            raise SystemExit('Doppelte Stationskennung im zentralen SIP-Switch; zuerst klären.')
        if matches:
            item = matches[0]
            if not item.get('enabled', True) or not item.get('username') or not item.get('password'):
                raise SystemExit('Vorhandener zentraler TBS-Zugang ist deaktiviert oder unvollständig; zuerst klären.')
            central_user, central_password = item['username'], item['password']
        else:
            if not installer.is_file():
                raise SystemExit('Der bestehende add-tbs-openlab.sh-Installer fehlt unter /opt/netcore-tetra.')
            central_user = 'tbs-srv-m-tbs-01'
            # This becomes the real credential only when add-tbs-openlab succeeds.
            central_password = secrets.token_urlsafe(32)

        def prompt(label):
            # Python reads its program from stdin; questions use the SSH TTY.
            with open('/dev/tty', 'r+') as tty:
                tty.write(label)
                tty.flush()
                value = tty.readline()
                if not value:
                    raise SystemExit('Keine Antwort am SSH-Terminal; Einrichtung abgebrochen.')
                return value.strip()

        pbx_user = prompt('Bereits in FreePBX eingerichtete Fallback-Kennung für TBS-01: ')
        if not pbx_user or any(c in pbx_user for c in '\r\n\x00'):
            raise SystemExit('Eine echte PBX-Fallback-Kennung angeben.')
        auth_mode = prompt('PBX-Anmeldung: Passwort oder IP-Vertrauen? [passwort/ip]: ').lower()
        if auth_mode == 'passwort':
            auth_user = prompt('PBX-Auth-Benutzer (leer = Fallback-Kennung): ') or pbx_user
            pbx_password = getpass.getpass('PBX-Passwort: ')
            if not pbx_password:
                raise SystemExit('PBX-Passwort fehlt.')
        elif auth_mode == 'ip':
            auth_user, pbx_password = '', ''
        else:
            raise SystemExit('Entweder passwort oder ip wählen.')

        config = {
            'service': {'node_id': node},
            'central': {'enabled': True, 'host': '10.0.1.125', 'port': 5060,
                        'username': central_user, 'password': central_password},
            'fallback_pbx': {'enabled': True, 'mode': 'registration', 'host': '10.0.1.21', 'port': 5060,
                             'username': pbx_user, 'auth_username': auth_user, 'password': pbx_password,
                             'from_user': pbx_user, 'contact_user': pbx_user, 'match': ['10.0.1.21']},
        }
        for section in config.values():
            if any(isinstance(v, str) and any(c in v for c in '\r\n\x00') for v in section.values()):
                raise SystemExit('SIP-Werte dürfen keine Zeilenumbrüche enthalten.')
        text = '\n\n'.join('[' + name + ']\n' + '\n'.join(
            key + ' = ' + json.dumps(value, ensure_ascii=True) for key, value in section.items())
            for name, section in config.items()) + '\n'
        tomllib.loads(text)

        # Complete the private file before invoking the central installer.
        with os.fdopen(os.dup(fd), 'w') as stream:
            os.fchmod(stream.fileno(), 0o600)
            stream.write(text)
            stream.flush()
            os.fsync(stream.fileno())
        if not matches:
            try:
                result = subprocess.run(['bash', str(installer), node, central_user, central_password],
                                        capture_output=True, text=True)
            except OSError:
                raise SystemExit('Zentraler TBS-Installer konnte nicht gestartet werden; keine Zugangsdaten ausgegeben.') from None
            if result.returncode:
                raise SystemExit('Zentraler TBS-Zugang konnte nicht vollständig eingerichtet werden; '
                                 'dessen Konfiguration und Dienste prüfen. Keine Zugangsdaten ausgegeben.')
        current = os.stat(filename, dir_fd=directory_fd, follow_symlinks=False)
        if (current.st_dev, current.st_ino) != (file_identity.st_dev, file_identity.st_ino):
            raise SystemExit('Ausgabepfad wurde während der Einrichtung verändert; Datei nicht veröffentlicht.')
        os.fchown(fd, account.pw_uid, account.pw_gid)
        os.fchmod(fd, 0o600)
        os.fsync(fd)
        os.fsync(directory_fd)
        complete = True
    finally:
        os.close(fd)
        if not complete:
            try:
                current = os.stat(filename, dir_fd=directory_fd, follow_symlinks=False)
                if (current.st_dev, current.st_ino) == (file_identity.st_dev, file_identity.st_ino):
                    os.unlink(filename, dir_fd=directory_fd)
            except FileNotFoundError:
                pass
finally:
    os.close(directory_fd)

print('Private Image-Konfiguration erstellt: ' + str(out))
print('Besitzer: ' + account.pw_name + '; Berechtigung: 0600.')
print('Als dieser Benutzer per SCP herunterladen und im privaten SIP-Fallback-Feld des Imagebuilders importieren.')
print('Zugangsdaten nicht im Chat posten.')
PY
