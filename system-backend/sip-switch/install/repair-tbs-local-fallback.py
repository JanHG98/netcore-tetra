#!/usr/bin/env python3
"""Remove an accidental central SIP Switch installation from a configured TBS.

Default: validate and print the proposed changes. --apply backs up configuration,
stops both controllers, removes central includes and restarts the local gateway.
The radio process must be stopped separately for the native SIP handoff.
"""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tempfile
import tomllib


FALLBACK = Path('/etc/netcore/tbs-sip-fallback.toml')
ASTERISK = Path('/etc/asterisk')
CENTRAL_INCLUDES = {
    'pjsip.conf': 'netcore-pjsip.conf',
    'extensions.conf': 'netcore-extensions.conf',
    'rtp.conf': 'netcore-rtp.conf',
}
FALLBACK_INCLUDES = {
    'pjsip.conf': ['netcore-tbs-fallback-pjsip.conf', 'netcore-active-registration.conf'],
    'extensions.conf': ['netcore-tbs-fallback-extensions.conf'],
    'rtp.conf': ['netcore-tbs-fallback-rtp.conf'],
}
INCLUDE = re.compile(r'^\s*#(?:try)?include\s+["<]?([^\s"<>;]+)[">]?\s*(?:;[^\n]*)?$')


def include_target(line: str) -> str | None:
    match = INCLUDE.fullmatch(line.rstrip('\r\n'))
    return match.group(1) if match else None


def repair_includes(text: str, central: str, fallback: list[str]) -> str:
    lines = [line for line in text.splitlines(keepends=True)
             if include_target(line) not in {central, str(ASTERISK / central)}]
    targets = {include_target(line) for line in lines}
    result = ''.join(lines)
    for name in fallback:
        if name not in targets and str(ASTERISK / name) not in targets:
            result = result.rstrip('\n') + f'\n#include {name}\n'
    return result


def repair_pbx_user(text: str, username: str | None) -> str:
    config = tomllib.loads(text)
    pbx = config.get('fallback_pbx', {})
    keys = ('username', 'from_user', 'contact_user')
    if username is None:
        if any(re.search(r'<[^>]+>', str(pbx.get(key, ''))) for key in keys):
            raise ValueError('PBX-Platzhalter vorhanden; tatsächliche SIP-Registrierungskennung mit --pbx-user angeben.')
        return text
    if not re.fullmatch(r'[A-Za-z0-9_.+!-]+', username):
        raise ValueError('--pbx-user muss eine SIP-Kennung ohne Platzhalter oder Leerzeichen sein.')
    active = False
    found: set[str] = set()
    lines = text.splitlines(keepends=True)
    for index, line in enumerate(lines):
        section = re.match(r'^\s*\[([^]]+)\]\s*(?:#.*)?$', line.rstrip())
        if section:
            active = section.group(1) == 'fallback_pbx'
        if active:
            key = re.match(r'^\s*(username|from_user|contact_user)\s*=', line)
            if key:
                name = key.group(1)
                lines[index] = f'{name} = {json.dumps(username)}\n'
                found.add(name)
    if found != set(keys):
        raise ValueError('PBX-Konfiguration unvollständig: username, from_user und contact_user müssen vorhanden sein.')
    result = ''.join(lines)
    tomllib.loads(result)
    return result


def prepare(root: Path, node_id: str, pbx_user: str | None):
    def local(path: Path) -> Path:
        return root / path.relative_to('/')

    cfg_path = local(FALLBACK)
    original = cfg_path.read_text(encoding='utf-8')
    cfg = tomllib.loads(original)
    if cfg.get('service', {}).get('node_id') != node_id:
        raise ValueError('Node-ID stimmt nicht mit der lokalen Fallback-Konfiguration überein.')
    if cfg.get('asterisk', {}).get('config_dir', '/etc/asterisk') != str(ASTERISK):
        raise ValueError('Abweichendes Asterisk-Konfigurationsverzeichnis: manuelle Reparatur erforderlich.')
    writes = {cfg_path: repair_pbx_user(original, pbx_user)}
    for name, central in CENTRAL_INCLUDES.items():
        path = local(ASTERISK / name)
        writes[path] = repair_includes(path.read_text(encoding='utf-8'), central, FALLBACK_INCLUDES[name])
    quarantine = [local(Path('/etc/netcore/sip-switch.toml')), local(Path('/etc/netcore/sip-switch-agi.env'))]
    metadata = local(Path('/etc/netcore/lxc-network.env'))
    if metadata.is_file() and 'NETCORE_SERVICE=sip-switch' in metadata.read_text().splitlines():
        quarantine.append(metadata)
    return writes, [path for path in quarantine if path.exists()]


def backup_files(root: Path, backup: Path, paths: list[Path]) -> None:
    for path in paths:
        if not path.exists():
            continue
        target = backup / path.relative_to(root)
        target.parent.mkdir(parents=True, exist_ok=True)
        if path.is_dir():
            shutil.copytree(path, target, symlinks=True)
        else:
            shutil.copy2(path, target)


def write_preserving_metadata(path: Path, text: str) -> None:
    descriptor, temporary = tempfile.mkstemp(prefix=f'.{path.name}.repair-', dir=path.parent)
    os.close(descriptor)
    temp = Path(temporary)
    try:
        stat = path.stat()
        shutil.copy2(path, temp)
        if os.geteuid() == 0:
            os.chown(temp, stat.st_uid, stat.st_gid)
        temp.write_text(text, encoding='utf-8')
        temp.replace(path)
    finally:
        temp.unlink(missing_ok=True)


def apply_repair(writes, quarantine, run=subprocess.run, root: Path = Path('/')) -> Path:
    # Back up before any service or configuration mutation. Secrets stay under 0700.
    def local(path: Path) -> Path:
        return root / path.relative_to('/')

    parent = local(Path('/var/backups/netcore-sip-repair'))
    parent.mkdir(parents=True, exist_ok=True, mode=0o700)
    backup = Path(tempfile.mkdtemp(prefix='tbs-', dir=parent))
    backup_files(root, backup, [local(ASTERISK), local(FALLBACK), *quarantine,
        local(Path('/var/lib/netcore-tbs-sip-fallback/state.json')),
        local(Path('/etc/systemd/system/netcore-sip-switch.service'))])
    print(f'Sicherung: {backup}', flush=True)
    try:
        run(['systemctl', 'stop', 'netcore-tbs-sip-failover.service'], check=True)
        run(['systemctl', 'disable', '--now', 'netcore-sip-switch.service'], check=True)
        for path, text in writes.items():
            write_preserving_metadata(path, text)
        for path in quarantine:
            path.unlink()  # Complete original is retained in the backup above.
        run(['/usr/local/bin/netcore-tbs-sip-fallback', '--config', str(FALLBACK), '--render'], check=True)
        # Transport bindings cannot reliably be removed with a configuration reload.
        run(['systemctl', 'restart', 'asterisk.service'], check=True)
        run(['systemctl', 'start', 'netcore-tbs-sip-failover.service'], check=True)
    except (OSError, subprocess.CalledProcessError) as exc:
        raise RuntimeError(f'Reparatur abgebrochen. Sicherung: {backup}. Dienstzustände prüfen; '
                           'keine automatische Rückkehr zur kollidierenden Doppelinstallation.') from exc
    return backup


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--node-id', required=True, help='Erwartete lokale Node-ID')
    parser.add_argument('--pbx-user', help='PBX-SIP-Registrierungskennung, z. B. 104; unverändert ohne diese Option')
    parser.add_argument('--apply', action='store_true', help='Sichern, zentrale Rolle entfernen und lokalen Asterisk neu starten')
    args = parser.parse_args()
    try:
        writes, quarantine = prepare(Path('/'), args.node_id, args.pbx_user)
        print('Ziel: lokaler TBS-Fallback; zentrale Rolle auf diesem Host deaktivieren.')
        for path in writes:
            print(f'Konfiguration prüfen/aktualisieren: {path}')
        for path in quarantine:
            print(f'In Sicherung archivieren: {path}')
        if args.apply:
            if os.geteuid() != 0:
                parser.error('--apply erfordert root.')
            apply_repair(writes, quarantine)
            print('Lokaler Asterisk/Fallback wiederhergestellt. Native TBS-Konfiguration und Registrierung prüfen.')
        else:
            print('Prüflauf: keine Änderungen. Zum Ausführen --apply ergänzen.')
    except (OSError, ValueError, RuntimeError) as exc:
        parser.exit(1, f'Fehler: {exc}\n')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
