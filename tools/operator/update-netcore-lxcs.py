#!/usr/bin/env python3
"""Discover and update installed NetCore runtimes on this Proxmox node.

Default is inventory only. --apply pushes one pinned source archive with pct,
updates in dependency order and stops on the first build/install failure.
No guest OS upgrades, container starts or reboots are performed.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile
import time

ROOT = Path(__file__).resolve().parents[2]
GUEST_HELPER = 'tools/operator/update-netcore-service.py'
EXTERNAL_UNITS = ('brew-server.service', 'netcore-piper.service', 'asterisk.service',
                  'tetra.service', 'netcore-tbs.service', 'netcore-deployment.service')


def run(args, **kwargs):
    return subprocess.run(args, check=True, **kwargs)


def load_helper():
    spec = importlib.util.spec_from_file_location('netcore_service_update', ROOT / GUEST_HELPER)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def containers(output):
    result = []
    for line in output.splitlines():
        fields = line.split()
        if fields and fields[0].isdigit():
            if len(fields) < 2 or fields[1] not in ('running', 'stopped'):
                raise RuntimeError('Unerwartete pct-list-Ausgabe: ' + line)
            result.append(dict(ctid=int(fields[0]), status=fields[1], name=fields[-1]))
    return result


def ordered(roles):
    remaining, result = dict(roles), []
    while remaining:
        names = [n for n, s in remaining.items()
                 if not any(d in remaining for d in s.get('depends_on', []))]
        if not names:
            raise RuntimeError('Zyklische Dienstabhängigkeit im Katalog.')
        names.sort(key=lambda n: (n != 'discovery', n != 'directory', n))
        for name in names:
            result.append(name)
            remaining.pop(name)
    return result


SCAN = r'''
import json, pathlib, subprocess, sys
roles = json.loads(sys.argv[1])
managed = None
path = pathlib.Path('/etc/netcore/discovery.toml')
if path.is_file():
    import tomllib
    managed = tomllib.loads(path.read_text()).get('managed_services')
    if managed is not None and (not isinstance(managed, list) or
                              any(not isinstance(n, str) for n in managed)):
        raise RuntimeError('managed_services ist keine gültige Dienstliste')
result = {'installed': [], 'unmanaged': [], 'external': []}
for name, unit in roles.items():
    proc = subprocess.run(['systemctl', 'show', unit, '-p', 'LoadState',
                           '-p', 'FragmentPath', '-p', 'ActiveState'],
                          text=True, capture_output=True, check=True)
    props = dict(l.split('=', 1) for l in proc.stdout.splitlines() if '=' in l)
    if props.get('LoadState') == 'loaded' and props.get('FragmentPath'):
        if managed is not None and name not in ('directory', 'discovery') and name not in managed:
            result['unmanaged'].append(name)
        else:
            result['installed'].append(name)
for unit in ('brew-server.service', 'netcore-piper.service', 'asterisk.service',
             'tetra.service', 'netcore-tbs.service', 'netcore-deployment.service'):
    proc = subprocess.run(['systemctl', 'show', unit, '-p', 'LoadState', '-p', 'FragmentPath'],
                          text=True, capture_output=True, check=True)
    if 'LoadState=loaded' in proc.stdout and 'FragmentPath=/' in proc.stdout:
        result['external'].append(unit)
print(json.dumps(result))
'''


def scan(ctid, roles):
    units = json.dumps({name: s['unit'] for name, s in roles.items()})
    proc = run(['pct', 'exec', str(ctid), '--', 'python3', '-c', SCAN, units],
               capture_output=True, text=True)
    return json.loads(proc.stdout)


def inspect(ctid, roles):
    # First identify unit files using systemd itself. Unrelated systemd guests
    # need neither Python nor a NetCore agent for this read-only exclusion.
    proc = run(['pct', 'exec', str(ctid), '--', 'systemctl', 'list-unit-files',
                '--type=service', '--no-legend', '--no-pager'],
               capture_output=True, text=True)
    units = {line.split()[0] for line in proc.stdout.splitlines() if line.split()}
    if not units.intersection(spec['unit'] for spec in roles.values()):
        # Preserve information about separate legacy/external apps, without
        # treating them as an update target or requiring guest Python.
        return dict(installed=[], unmanaged=[],
                    external=[unit for unit in EXTERNAL_UNITS if unit in units])
    return scan(ctid, roles)


def stage(ctid, archive, sha, commit):
    proc = run(['pct', 'exec', str(ctid), '--', 'mktemp', '-d',
                '/var/tmp/netcore-source-update.XXXXXX'], capture_output=True, text=True)
    root = proc.stdout.strip()
    if not re.fullmatch(r'/var/tmp/netcore-source-update\.[A-Za-z0-9]+', root):
        raise RuntimeError('Unerwarteter Gast-Stagingpfad.')
    run(['pct', 'push', str(ctid), str(archive), root + '/source.tar', '--perms', '0600'])
    # Every argument is separate; no guest command string or shell interpolation.
    code = """import hashlib,pathlib,sys
p=pathlib.Path(sys.argv[1]);actual=hashlib.sha256(p.read_bytes()).hexdigest()
if actual != sys.argv[2]: raise RuntimeError('Quellpaket-SHA stimmt nicht')
"""
    run(['pct', 'exec', str(ctid), '--', 'python3', '-c', code, root + '/source.tar', sha])
    run(['pct', 'exec', str(ctid), '--', 'tar', '--extract', '--file', root + '/source.tar',
         '--directory', root, '--no-same-owner'])
    identity = """import json,pathlib,sys
pathlib.Path(sys.argv[1]).write_text(json.dumps({'commit':sys.argv[2],
    'archive_sha256':sys.argv[3]}) + '\\n')
"""
    run(['pct', 'exec', str(ctid), '--', 'python3', '-c', identity,
         root + '/.netcore-source.json', commit, sha])
    return root


def save_report(path, report):
    path.write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')


def guest_step(command, log):
    """Stream real prepare/apply progress and read the final JSON receipt."""
    receipt = None
    with log.open('w', encoding='utf-8') as stream:
        proc = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                                text=True, bufsize=1)
        for line in proc.stdout:
            print(line, end='', flush=True)
            stream.write(line)
            stream.flush()
            try:
                value = json.loads(line)
            except ValueError:
                continue
            if isinstance(value, dict) and 'phase' in value:
                receipt = value
        rc = proc.wait()
    return rc, receipt


def file_hashes(receipt):
    files = receipt.get('files')
    if not isinstance(files, list) or not files:
        raise RuntimeError('Updatebeleg hat keine Runtime-Dateipruefsummen.')
    result = {}
    for item in files:
        if not isinstance(item, dict):
            raise RuntimeError('Ungueltiger Dateibeleg.')
        target, sha = item.get('target'), item.get('sha256')
        if (not isinstance(target, str) or not target.startswith('/') or target in result
                or not isinstance(sha, str) or not re.fullmatch(r'[0-9a-f]{64}', sha)):
            raise RuntimeError('Ungueltiger oder doppelter Runtime-Dateibeleg.')
        result[target] = sha
    return result


def validate_receipt(receipt, role, commit, prepare=False, expected_files=None, rc=0):
    if not isinstance(receipt, dict):
        raise RuntimeError('Kein JSON-Updatebeleg; Exitcode allein belegt keine Aktualisierung.')
    if receipt.get('service') != role or receipt.get('source_commit') != commit:
        raise RuntimeError('Dienst oder Quellcommit im Updatebeleg stimmt nicht.')
    if receipt.get('assets_complete') is not True:
        raise RuntimeError('Runtime-Dateien wurden nicht vollstaendig geprueft.')
    files = file_hashes(receipt)
    phase = receipt.get('phase')
    if prepare:
        if rc != 0 or phase != 'prepared':
            raise RuntimeError('Vollstaendige Build-Vorbereitung fehlt.')
        return files
    if files != expected_files:
        raise RuntimeError('Installierte Pruefsummen weichen von den vorbereiteten Runtime-Dateien ab.')
    if receipt.get('verified') is not True:
        raise RuntimeError('Installierter Runtime-Zustand ist nicht als verifiziert markiert.')
    allowed = {'updated', 'updated_degraded', 'inactive_updated', 'already_current',
               'already_current_degraded', 'recovered', 'recovered_degraded'}
    if phase not in allowed or rc not in (0, 2):
        raise RuntimeError('Kein erfolgreicher Runtime-Austausch im Updatebeleg.')
    process = receipt.get('process')
    if not isinstance(process, dict):
        raise RuntimeError('Prozessnachweis fehlt im Updatebeleg.')
    if phase == 'inactive_updated':
        if (process.get('active') is not False or process.get('MainPID') != 0
                or receipt.get('liveness') is not None or rc != 0):
            raise RuntimeError('Inaktiver Dienst hat einen widerspruechlichen Updatebeleg.')
    else:
        pid, invocation = process.get('MainPID'), process.get('InvocationID')
        if (process.get('active') is not True or type(pid) is not int or pid <= 0
                or not isinstance(invocation, str) or not re.fullmatch(r'[0-9a-f]{32}', invocation)
                or receipt.get('liveness') is not True):
            raise RuntimeError('Aktualisierte Runtime ist nicht als laufender Prozess belegt.')
        if receipt.get('readiness') is not (rc == 0):
            raise RuntimeError('Readiness und Exitcode im Updatebeleg widersprechen sich.')
    return files


def complete_report(report):
    results = report.get('results', [])
    report['summary'] = dict(
        planned=len(report.get('plan', [])),
        prepared=sum('prepare_receipt' in item for item in results),
        updated=sum(item.get('phase') in ('updated', 'recovered') for item in results),
        already_current=sum(item.get('phase') == 'already_current' for item in results),
        inactive_updated=sum(item.get('phase') == 'inactive_updated' for item in results),
        degraded=sum(item.get('phase', '').endswith('_degraded') for item in results),
        failed=sum(item.get('phase', '').startswith('failed') for item in results),
        not_attempted=sum(item.get('phase') == 'not_attempted' for item in results))


def stop_remaining(report, reason):
    for item in report.get('results', []):
        if item.get('phase') in ('not_attempted', 'prepared'):
            item.update(phase='not_attempted', reason=reason)
    complete_report(report)


def rollout(args, roles, report, report_path):
    targets = containers(run(['pct', 'list'], capture_output=True, text=True).stdout)
    if args.ctid:
        known = {t['ctid'] for t in targets}
        missing = set(args.ctid) - known
        if missing:
            raise RuntimeError('CT nicht lokal vorhanden: ' + ', '.join(map(str, sorted(missing))))
        targets = [t for t in targets if t['ctid'] in args.ctid]
    tasks = []
    for ct in targets:
        entry = dict(ct)
        report['containers'].append(entry)
        if ct['status'] != 'running':
            entry['phase'] = 'stopped_not_inspected'
            print(f"CT {ct['ctid']}: gestoppt; nicht gestartet und nicht geprüft.", flush=True)
            continue
        try:
            found = inspect(ct['ctid'], roles)
        except Exception as exc:
            entry.update(phase='scan_failed', error=str(exc))
            print(f"CT {ct['ctid']}: Inventarisierung fehlgeschlagen; siehe Bericht.", flush=True)
            continue
        entry.update(found)
        entry['phase'] = 'planned' if found['installed'] else 'no_managed_runtime'
        print(f"CT {ct['ctid']} {ct['name']}: " + ', '.join(found['installed'] or ['kein NetCore-Runtime']), flush=True)
        for role in found['installed']:
            tasks.append((role, ct['ctid']))
    priority = {role: index for index, role in enumerate(ordered(roles))}
    tasks.sort(key=lambda t: (priority[t[0]], t[1]))
    report['plan'] = [dict(service=r, ctid=c) for r, c in tasks]
    if args.apply:
        report['results'] = [dict(service=r, ctid=c, phase='not_attempted') for r, c in tasks]
    complete_report(report)
    save_report(report_path, report)
    incomplete = any(c['phase'] == 'scan_failed' for c in report['containers'])
    if not args.apply:
        return 1 if incomplete else 0
    if incomplete:
        stop_remaining(report, 'Inventarisierung unvollstaendig; kein Runtime-Austausch begonnen.')
        save_report(report_path, report)
        raise RuntimeError('Inventarisierung unvollständig. Betroffene CTs prüfen oder --ctid gezielt wählen.')
    if not tasks:
        raise RuntimeError('Keine installierten und verwalteten NetCore-Dienste erkannt.')
    archive = report_path.parent / 'source.tar'
    run(['git', '-C', str(ROOT), 'archive', '--format=tar', '--output', str(archive), 'HEAD'])
    archive_sha = hashlib.sha256(archive.read_bytes()).hexdigest()
    stages, expected = {}, {}
    commit = report['commit']
    try:
        for ctid in sorted({c for _, c in tasks}):
            print(f'CT {ctid}: Quellpaket uebertragen und SHA-256 pruefen', flush=True)
            try:
                stages[ctid] = stage(ctid, archive, archive_sha, commit)
            except BaseException as exc:
                for item in report['results']:
                    if item['ctid'] == ctid:
                        item.update(phase='failed_stage', error=str(exc))
                raise
        # Build and freeze ALL assets before the first stop or runtime exchange.
        for index, ((role, ctid), item) in enumerate(zip(tasks, report['results']), 1):
            print(f'[{index}/{len(tasks)}] CT {ctid}: {role} vollstaendig bauen und vorbereiten', flush=True)
            item['phase'] = 'preparing'
            log = report_path.parent / f'{ctid}-{role}-prepare.log'
            item['prepare_log'] = str(log)
            save_report(report_path, report)
            command = ['pct', 'exec', str(ctid), '--', 'python3', '-B',
                       stages[ctid] + '/' + GUEST_HELPER, '--service', role,
                       '--commit', commit, '--prepare']
            if not getattr(args, 'no_prepare_build_deps', False):
                command.append('--prepare-build-deps')
            try:
                rc, receipt = guest_step(command, log)
                item['prepare_returncode'] = rc
                if rc != 0:
                    raise RuntimeError(f'Vorbereitung CT {ctid}/{role} fehlgeschlagen (Exit {rc}); siehe {log}.')
                expected[(role, ctid)] = validate_receipt(receipt, role, commit, prepare=True, rc=rc)
                item.update(phase='prepared', prepare_receipt=receipt)
            except BaseException as exc:
                item.update(phase='failed_prepare', error=str(exc))
                raise
            complete_report(report)
            save_report(report_path, report)
    except BaseException:
        stop_remaining(report, 'Vorbereitung abgebrochen; kein Runtime-Austausch begonnen.')
        save_report(report_path, report)
        raise
    print(f'Alle {len(tasks)} Ziele vollstaendig vorbereitet; Runtime-Austausch beginnt.', flush=True)
    degraded = False
    for index, ((role, ctid), item) in enumerate(zip(tasks, report['results']), 1):
        print(f'\n[{index}/{len(tasks)}] CT {ctid}: {role} vorbereitete Runtime installieren', flush=True)
        item['phase'] = 'applying'
        command = ['pct', 'exec', str(ctid), '--', 'python3', '-B',
                   stages[ctid] + '/' + GUEST_HELPER, '--service', role,
                   '--commit', commit, '--apply']
        log = report_path.parent / f'{ctid}-{role}.log'
        item['log'] = str(log)
        save_report(report_path, report)
        try:
            rc, receipt = guest_step(command, log)
            item['returncode'] = rc
            validate_receipt(receipt, role, commit, expected_files=expected[(role, ctid)], rc=rc)
            item.update(phase=receipt['phase'], receipt=receipt)
        except BaseException as exc:
            item.update(phase='failed_apply', error=str(exc))
            stop_remaining(report, f'Nach fehlgeschlagenem Update CT {ctid}/{role} nicht begonnen.')
            save_report(report_path, report)
            raise RuntimeError(f'CT {ctid} / {role} fehlgeschlagen. Weitere Updates gestoppt; siehe {log}.') from exc
        complete_report(report)
        save_report(report_path, report)
        print(f"CT {ctid} {role}: {receipt['phase']}; Commit {commit[:12]}; "
              f"{len(receipt['files'])} Dateipruefsummen bestaetigt", flush=True)
        degraded |= rc == 2
    return 2 if degraded else 0


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--apply', action='store_true')
    parser.add_argument('--ctid', type=int, action='append', help='Nur diese lokale CT-ID; wiederholbar')
    parser.add_argument('--no-prepare-build-deps', action='store_true',
                        help='Keine fehlenden minimalen Build-Voraussetzungen im Gast installieren')
    args = parser.parse_args(argv)
    if os.geteuid() != 0:
        raise RuntimeError('Auf dem Proxmox-Host als root ausführen.')
    import fcntl
    with open('/run/lock/netcore-lxc-update.lock', 'a') as lock:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        dirty = run(['git', '-C', str(ROOT), 'status', '--porcelain', '--untracked-files=no'],
                    capture_output=True, text=True).stdout.strip()
        if dirty:
            raise RuntimeError('Quellcheckout hat lokale Änderungen; keine nicht gepinnten Updates.')
        commit = run(['git', '-C', str(ROOT), 'rev-parse', 'HEAD'],
                     capture_output=True, text=True).stdout.strip()
        if not re.fullmatch(r'[0-9a-f]{40}', commit):
            raise RuntimeError('Kein vollständiger Quellcommit.')
        print(f"Modus: {'APPLY - echte Runtime-Aktualisierung' if args.apply else 'INVENTAR - nur lesen'}", flush=True)
        print('Quellcommit fuer alle Ziele: ' + commit, flush=True)
        work = Path(tempfile.mkdtemp(prefix='netcore-lxc-update-' + time.strftime('%Y%m%d-'), dir='/var/tmp'))
        report_path = work / 'report.json'
        report = dict(commit=commit, apply=args.apply, containers=[], plan=[], results=[], phase='running')
        try:
            code = rollout(args, load_helper().specs(), report, report_path)
            report['phase'] = ('incomplete' if code == 1 else
                               'completed_degraded' if code == 2 else
                               'completed' if args.apply else 'planned')
        except BaseException as exc:
            report.update(phase='failed', error=str(exc))
            raise
        finally:
            complete_report(report)
            save_report(report_path, report)
            print('\nZielbilanz: ' + ', '.join(f'{key}={value}' for key, value in report['summary'].items()), flush=True)
            for item in report.get('results', []):
                print(f"CT {item['ctid']} {item['service']}: {item['phase']}" +
                      ('; ' + item['reason'] if item.get('reason') else ''), flush=True)
            print('\nBericht: ' + str(report_path), flush=True)
        print('Ergebnis: ' + report['phase'], flush=True)
        return code


if __name__ == '__main__':
    try:
        raise SystemExit(main())
    except Exception as exc:
        print('Rollout abgebrochen: ' + str(exc), file=__import__('sys').stderr)
        raise SystemExit(1)
