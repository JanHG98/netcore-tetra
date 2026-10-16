#!/usr/bin/env python3
"""Reset controller/builder application state, retaining installed VM software.

No remote installer is cancelled by stopping the controller. Refuse to discard
its live/uncertain receipts. Keep private configuration/history for diagnosis;
delete reproducible images and caches only after stopping both whole cgroups.
"""
from __future__ import annotations

import argparse
from contextlib import closing
from datetime import datetime, timezone
import fcntl
import json
import os
from pathlib import Path
import pwd
import re
import shutil
import sqlite3
import subprocess
import tempfile
import time
import tomllib
from urllib.request import ProxyHandler, build_opener

CONFIG = Path('/etc/netcore/deployment.toml')
STATE = Path('/var/lib/netcore-deployment')
BUILDER = Path('/var/lib/netcore-image-builder')
BACKUPS = Path('/var/lib/netcore-deployment-resets')
PROC = Path('/proc')
CGROUP = Path('/sys/fs/cgroup')
UNITS = ('netcore-deployment.service', 'netcore-image-builder.service')
DEFAULTS = dict(mode='open_lab', role='controller', bind='0.0.0.0', port=8320,
                environment='netcore-openlab', state_dir=str(STATE),
                repository='https://github.com/JanHG98/netcore-tetra.git',
                ref='main', poll_seconds=3600, multicast_group='239.192.84.82',
                discovery_port=48320, interface='0.0.0.0', interval=15,
                lease_seconds=90, allowed_networks=['10.0.0.0/8', '172.16.0.0/12',
                '192.168.0.0/16', '127.0.0.0/8'], seeds=[])
KEEP = ('node_id', 'advertise_url', 'bind', 'port', 'environment', 'multicast_group',
        'discovery_port', 'interface', 'interval', 'lease_seconds', 'allowed_networks')


def run(args, timeout=45):
    result = subprocess.run(list(map(str, args)), check=True, timeout=timeout,
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    return result.stdout.strip()


def safe_path(path):
    path = Path(path)
    if not path.is_absolute() or '..' in path.parts:
        raise RuntimeError('Ungueltiger Resetpfad: ' + str(path))
    for item in (path, *path.parents):
        if item.is_symlink():
            raise RuntimeError('Symlink im Resetpfad: ' + str(item))
    if path.exists() and not path.is_dir():
        raise RuntimeError('Resetpfad ist kein Verzeichnis: ' + str(path))
    return path


def mount_path(value):
    return Path(re.sub(r'\\([0-7]{3})', lambda m: chr(int(m[1], 8)), value))


def assert_no_mounts(roots):
    # Also inspect surviving private namespaces, not just the host's mounts.
    for info in PROC.glob('[0-9]*/mountinfo'):
        try:
            lines = info.read_text().splitlines()
        except (FileNotFoundError, ProcessLookupError):
            continue
        for line in lines:
            target = mount_path(line.split()[4])
            if any(target == root or root in target.parents for root in roots):
                raise RuntimeError('Mount noch aktiv; nichts loeschen: ' + str(target))


def remote_blockers(path):
    if not path.exists():
        return []
    if path.is_symlink() or not path.is_file():
        raise RuntimeError('Ungueltige Auftragsdatenbank: ' + str(path))
    blocked = []
    with closing(sqlite3.connect(path.as_uri() + '?mode=ro', uri=True, timeout=5)) as db:
        for key, status, request, result in db.execute('SELECT id,status,request,result FROM jobs'):
            q, r = json.loads(request), json.loads(result)
            if not isinstance(q, dict) or not isinstance(r, dict):
                raise RuntimeError('Ungueltige Auftragsdaten: ' + key)
            flag = r.get('remote_uncertain', False)
            if type(flag) is not bool:
                raise RuntimeError('Ungueltiger Remote-Status: ' + key)
            rejected = r.get('dispatch_state') == 'rejected' and flag is False
            active_remote = status == 'running' and q.get('kind') != 'check'
            if not rejected and (flag or active_remote):
                blocked.append(dict(id=key, status=status, node=q.get('node_id'),
                                    service=q.get('service'), remote_job=r.get('remote_job')))
    return blocked


def require_no_remote_jobs():
    blocked = remote_blockers(STATE / 'jobs.sqlite3')
    if blocked:
        raise RuntimeError('Laufende/ungeklaerte LXC-Auftraege bleiben geschuetzt. '
                           'Zuerst Endstatus abgleichen: ' + json.dumps(blocked, ensure_ascii=True))


def properties(unit):
    names = ('LoadState', 'ActiveState', 'MainPID', 'ControlGroup', 'ExecStart', 'KillMode')
    return dict(line.split('=', 1) for line in run(
        ['systemctl', 'show', unit, *['--property=' + name for name in names]]).splitlines())


def assert_stopped():
    for unit in UNITS:
        props = properties(unit)
        if props.get('ActiveState') not in ('inactive', 'failed') or props.get('MainPID') != '0':
            raise RuntimeError('Dienst nicht vollstaendig gestoppt: ' + unit)
        group = props.get('ControlGroup', '')
        if group:
            if not group.startswith('/') or '..' in Path(group).parts:
                raise RuntimeError('Ungueltige Cgroup: ' + unit)
            for file in (CGROUP / group.lstrip('/')).rglob('cgroup.procs'):
                try:
                    busy = file.read_text().strip()
                except FileNotFoundError:
                    continue
                if busy:
                    raise RuntimeError('Build-/Dienstprozesse noch aktiv: ' + unit)


def loops():
    return json.loads(run(['losetup', '--json', '--list', '--output', 'NAME,BACK-FILE']))['loopdevices']


def owned_loops(rows):
    result = []
    for row in rows:
        text = row.get('back-file') or ''
        back = Path(text.removesuffix(' (deleted)'))
        if back == BUILDER or BUILDER in back.parents:
            if (back.parent.parent != BUILDER / 'work' or back.name != 'image.img'
                    or not re.fullmatch('[0-9a-f]{32}', back.parent.name)
                    or not re.fullmatch('/dev/loop[0-9]+', row['name'])):
                raise RuntimeError('Unerwartetes Loopgeraet im Buildzustand: ' + row['name'])
            result.append(row['name'])
    return result


def fresh_config(old):
    cfg = {**DEFAULTS, **{key: old[key] for key in KEEP if key in old}}
    # No inherited seeds, manual bindings, template path, ref, profile or secrets.
    text = '\n'.join(key + ' = ' + json.dumps(value, ensure_ascii=False)
                     for key, value in cfg.items()) + '\n\n[bindings]\n'
    tomllib.loads(text)
    return text


def write_config(text):
    fd, name = tempfile.mkstemp(prefix='.deployment-reset-', dir=CONFIG.parent)
    try:
        with os.fdopen(fd, 'w', encoding='utf-8') as stream:
            stream.write(text)
            stream.flush()
            os.fsync(stream.fileno())
        os.chmod(name, 0o644)
        os.replace(name, CONFIG)
    finally:
        Path(name).unlink(missing_ok=True)


def prune(backup):
    targets = [backup / 'builder' / name for name in ('artifacts', 'cache', 'work')]
    targets.append(backup / 'controller' / 'source')
    assert_no_mounts(targets)
    for path in targets:
        # Only these exact children of the newly created private backup are removed.
        if path.is_symlink():
            path.unlink()
        elif path.exists():
            if not path.is_dir():
                raise RuntimeError('Unerwartetes Cacheziel: ' + str(path))
            print('Entfernen: ' + str(path), flush=True)
            shutil.rmtree(path)


def verify(cfg):
    host = cfg.get('bind', '0.0.0.0')
    if host == '0.0.0.0':
        host = '127.0.0.1'
    base = 'http://' + host + ':' + str(cfg.get('port', 8320))
    opener = build_opener(ProxyHandler({}))
    deadline = time.monotonic() + 25
    while True:
        try:
            def get(path):
                with opener.open(base + path, timeout=3) as reply:
                    return json.load(reply)
            ready, images, profiles, batches = [get(path) for path in
                ('/health/ready', '/api/v1/images', '/api/v1/profiles', '/api/v1/updates')]
            if (ready.get('ready') is not True or images.get('available') is not True
                    or images.get('jobs') != [] or images.get('artifacts') != []
                    or profiles != [] or batches != []):
                raise RuntimeError('Leerer Anwendungszustand noch nicht bestaetigt')
            return base
        except (OSError, ValueError, RuntimeError):
            if time.monotonic() >= deadline:
                raise
            time.sleep(1)


def reset(apply=False):
    for root in (STATE, BUILDER, BACKUPS):
        safe_path(root)
    if CONFIG.is_symlink() or CONFIG.resolve() != CONFIG:
        raise RuntimeError('Symlink in der Controller-Konfiguration')
    old_text = CONFIG.read_text(encoding='utf-8')
    cfg = tomllib.loads(old_text)
    if cfg.get('role') != 'controller' or Path(cfg.get('state_dir', STATE)) != STATE:
        raise RuntimeError('Nur die Standard-Deployment-VM wird unterstuetzt')
    if cfg.get('image_builder_socket', '/run/netcore-image-builder/api.sock') != '/run/netcore-image-builder/api.sock':
        raise RuntimeError('Individueller Buildersocket; Resetpfade zuerst pruefen')
    for unit, script in zip(UNITS, ('main.py --config /etc/netcore/deployment.toml', 'image_worker.py')):
        props = properties(unit)
        if (props.get('LoadState') != 'loaded' or props.get('KillMode') != 'control-group'
                or '/usr/local/lib/netcore-deployment/' + script not in props.get('ExecStart', '')
                or unit == UNITS[1] and any(arg in props['ExecStart'] for arg in ('--state', '--config', '--socket'))):
            raise RuntimeError('Abweichende Unit; keine Resetannahme: ' + unit)
    require_no_remote_jobs()
    fresh = fresh_config(cfg)
    print('Reset: Images, Cache, Builds, Jobs, Sammelverlauf, Profile, Template, Inventar und UI-Einstellungen.', flush=True)
    if not apply:
        return dict(phase='preview', changed=False, backup_root=str(BACKUPS))
    account = pwd.getpwnam('netcore-deploy')
    stopped, committed, moved, backup = False, False, [], None
    try:
        print('Controller und Builder samt Buildprozessen stoppen (bis zu 120s).', flush=True)
        run(['systemctl', 'stop', *UNITS], timeout=120)
        stopped = True
        assert_stopped()
        require_no_remote_jobs()
        assert_no_mounts((STATE, BUILDER))
        for root in (STATE, BUILDER):
            safe_path(root)
        attached = owned_loops(loops())
        for device in attached:
            run(['losetup', '--detach', device])
        if set(attached) & {row['name'] for row in loops()}:
            raise RuntimeError('Build-Loopgeraet ist noch belegt; keine Dateien loeschen')
        BACKUPS.mkdir(mode=0o700, exist_ok=True)
        os.chmod(BACKUPS, 0o700)
        backup = Path(tempfile.mkdtemp(prefix=datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%SZ-'), dir=BACKUPS))
        (backup / 'deployment.toml').write_text(old_text, encoding='utf-8')
        os.chmod(backup / 'deployment.toml', 0o600)
        (backup / 'dienstjournal.txt').write_text(run(['journalctl', '--no-pager', '-n', '200',
            '-u', UNITS[0], '-u', UNITS[1], '-o', 'short-iso']), encoding='utf-8')
        for root, name in ((STATE, 'controller'), (BUILDER, 'builder')):
            if root.exists():
                # rename refuses cross-filesystem moves: never copy gigabytes or follow mounts.
                root.rename(backup / name)
                moved.append((root, backup / name))
            root.mkdir(mode=0o750 if root == STATE else 0o700)
            os.chown(root, account.pw_uid if root == STATE else 0, account.pw_gid)
        write_config(fresh)
        committed = True
        print('Geschuetzte Sicherung von Konfiguration und Verlauf: ' + str(backup), flush=True)
        prune(backup)
        run(['systemctl', 'reset-failed', *UNITS])
        run(['systemctl', 'start', UNITS[1]])
        run(['systemctl', 'start', UNITS[0]])
        stopped = False
        url = verify(tomllib.loads(fresh))
        return dict(phase='completed', changed=True, backup=str(backup), webui=url,
                    note='Automatische Versionspruefung und Discovery starten wieder; LXC-Dienste bleiben installiert.')
    except BaseException:
        if stopped and not committed:
            for root, saved in reversed(moved):
                if root.exists():
                    root.rmdir()  # Refuse unexpected new files, do not recursively remove.
                saved.rename(root)
            if moved:
                write_config(old_text)
            run(['systemctl', 'start', UNITS[1]])
            run(['systemctl', 'start', UNITS[0]])
        if committed:
            print('Resetzustand bereits angelegt. Sicherung: ' + str(backup) +
                  '; Dienste bei Fehler erst nach Ursachenpruefung starten.', flush=True)
        raise


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--apply', action='store_true', help='Anwendungszustand wirklich zuruecksetzen')
    args = parser.parse_args()
    if os.geteuid() != 0:
        parser.error('Bitte mit sudo/root ausfuehren')
    with open('/run/lock/netcore-deployment-reset.lock', 'a') as lock:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        try:
            print(json.dumps(reset(args.apply), ensure_ascii=False, indent=2))
        except Exception as exc:
            raise SystemExit('RESET ABBRUCH: ' + str(exc)) from exc


if __name__ == '__main__':
    main()
