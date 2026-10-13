#!/usr/bin/env python3
"""Reconcile one uncertain controller receipt using fresh, exact agent GETs.

Diagnosis is read-only. --apply requires a stopped controller and an idle job
database. No installer, POST, ref resolution, inferred job search or flag-only
override is provided. Works independently of the installed controller version.
"""
from __future__ import annotations

import argparse
from contextlib import closing
import hashlib
import ipaddress
import json
import os
from pathlib import Path
import re
import sqlite3
import stat
import subprocess
import time
import tomllib
from urllib.error import HTTPError
import urllib.parse
import urllib.request
import uuid

JOB_ID = re.compile(r'[0-9a-f]{32}\Z')
SHA = re.compile(r'[0-9a-f]{40}\Z')
NAME = re.compile(r'[A-Za-z0-9][A-Za-z0-9_.-]{0,63}\Z')
TERMINAL = ('succeeded', 'failed', 'interrupted')
PROTOCOL = 'netcore.discovery.v1'
COLUMNS = 'id,created,updated,status,request,log,result'
PROFILE_KEYS = ('name', 'mcc', 'mnc', 'issi', 'la', 'cc')


class Pending(RuntimeError):
    """Evidence is insufficient; leave the complete local database unchanged."""


class NoRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        return None


def get_json(url):
    # Do not follow an agent redirect or send LAN requests through a proxy.
    opener = urllib.request.build_opener(urllib.request.ProxyHandler({}), NoRedirect())
    request = urllib.request.Request(url, headers={'Accept': 'application/json'}, method='GET')
    try:
        with opener.open(request, timeout=10) as response:
            body = response.read(1024 * 1024 + 1)
    except HTTPError as exc:
        exc.close()
        raise
    if len(body) > 1024 * 1024:
        raise Pending('remote_response_too_large')
    return json.loads(body)


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(',', ':'), ensure_ascii=False)


def fingerprint(value):
    return hashlib.sha256(canonical(value).encode()).hexdigest()


def safe_file(path, *, private=False, trusted_uid=None):
    path = Path(path).absolute()
    if any(candidate.is_symlink() for candidate in (path, *path.parents)):
        raise Pending('symlink_in_local_path')
    metadata = path.stat()
    if not stat.S_ISREG(metadata.st_mode):
        raise Pending('local_path_not_regular_file')
    if private and os.name == 'posix':
        if type(trusted_uid) is not int or trusted_uid < 0:
            raise Pending('controller_owner_uid_not_verified')
        owners = {0, trusted_uid}
        if metadata.st_uid not in owners or stat.S_IMODE(metadata.st_mode) != 0o600:
            raise Pending('controller_database_not_private_to_service_owner')
        for parent in path.parents:
            properties = parent.stat()
            if not stat.S_ISDIR(properties.st_mode) or properties.st_uid not in owners:
                raise Pending('controller_database_parent_owner_invalid')
            # A root-owned sticky temporary ancestor protects its owned child
            # from foreign renames. Ordinary writable ancestors never qualify.
            secure_temporary = properties.st_uid == 0 and properties.st_mode & stat.S_ISVTX
            if properties.st_mode & 0o022 and not secure_temporary:
                raise Pending('controller_database_parent_writable_by_others')
    return path


def read_row(path, key):
    with closing(sqlite3.connect(path.as_uri() + '?mode=ro', uri=True, timeout=10)) as db:
        row = db.execute(f'SELECT {COLUMNS} FROM jobs WHERE id=?', (key,)).fetchone()
    if row is None:
        raise Pending('local_job_missing')
    request, result = json.loads(row[4]), json.loads(row[6])
    if not isinstance(request, dict) or not isinstance(result, dict) or not isinstance(row[5], str):
        raise Pending('invalid_local_job')
    return row, request, result


def origin_and_id(value, networks):
    if not isinstance(value, str) or len(value) > 512:
        raise Pending('invalid_remote_receipt_url')
    parsed = urllib.parse.urlsplit(value)
    if (parsed.scheme != 'http' or parsed.username or parsed.password or parsed.query or parsed.fragment
            or not parsed.hostname or not parsed.port):
        raise Pending('invalid_remote_receipt_url')
    address = ipaddress.IPv4Address(parsed.hostname)
    if (address.is_multicast or address.is_unspecified
            or not any(address in ipaddress.IPv4Network(network) for network in networks)):
        raise Pending('remote_receipt_outside_allowed_networks')
    origin = f'http://{address}:{parsed.port}'
    if parsed.path == '/api/v1/jobs':
        return origin, ''
    match = re.fullmatch(r'/api/v1/jobs/([0-9a-f]{32})', parsed.path)
    if not match:
        raise Pending('invalid_remote_receipt_path')
    return origin, match[1]


def receipt(row, result, networks):
    stored = result.get('remote_url')
    remote_id = result.get('remote_job', '')
    if remote_id and (not isinstance(remote_id, str) or not JOB_ID.fullmatch(remote_id)):
        raise Pending('invalid_remote_job_id')
    origin, url_id = origin_and_id(stored, networks) if stored else ('', '')
    if remote_id and url_id and remote_id != url_id:
        raise Pending('conflicting_remote_receipt_ids')
    remote_id = remote_id or url_id
    # Old controllers logged the actual POST receipt before polling. Only exact
    # receipt lines are evidence; never choose a matching service from /jobs.
    logged = set()
    for line in row[5].splitlines():
        match = re.fullmatch(r'Remote-Auftrag (http://[^\s]+)', line)
        if match:
            candidate_origin, candidate_id = origin_and_id(match[1], networks)
            if candidate_id:
                logged.add((candidate_origin, candidate_id))
    if logged:
        if len(logged) != 1:
            raise Pending('conflicting_logged_remote_receipts')
        logged_origin, logged_id = next(iter(logged))
        if (origin and origin != logged_origin) or (remote_id and remote_id != logged_id):
            raise Pending('conflicting_logged_remote_receipts')
        origin, remote_id = origin or logged_origin, remote_id or logged_id
    if not origin or not remote_id:
        raise Pending('remote_job_receipt_missing_no_automatic_search')
    return origin, remote_id, f'{origin}/api/v1/jobs/{remote_id}'


def accepted_profile(value):
    if not isinstance(value, dict) or any(name not in value for name in PROFILE_KEYS):
        raise Pending('invalid_remote_profile')
    # Only these six nonsecret values form the accepted TBS request.
    profile = {name: value[name] for name in PROFILE_KEYS}
    if not isinstance(profile['name'], str) or not NAME.fullmatch(profile['name']):
        raise Pending('invalid_remote_profile')
    for name, low, high in (('mcc', 0, 1023), ('mnc', 0, 16383), ('issi', 1, 16777214),
                            ('la', 0, 16383), ('cc', 0, 63)):
        if type(profile[name]) is not int or not low <= profile[name] <= high:
            raise Pending('invalid_remote_profile')
    return profile


def proof(cfg, row, local_request, result, fetch):
    node, service, action = (local_request.get(name) for name in ('node_id', 'service', 'action'))
    if (not isinstance(node, str) or not NAME.fullmatch(node) or not isinstance(service, str)
            or not NAME.fullmatch(service) or action not in ('install', 'update', 'restart')):
        raise Pending('local_target_or_action_missing')
    origin, remote_id, url = receipt(row, result, cfg['allowed_networks'])
    try:
        manifest = fetch(origin + '/api/v1/manifest')
        if (not isinstance(manifest, dict) or manifest.get('node_id') != node
                or manifest.get('environment') != cfg['environment'] or manifest.get('protocol') != PROTOCOL
                or manifest.get('role') != 'agent' or manifest.get('security_mode') != 'open_lab'):
            raise Pending('remote_agent_identity_mismatch')
        remote = fetch(url)
    except Pending:
        raise
    except Exception as exc:
        raise Pending('remote_get_unavailable_or_invalid') from exc
    if (not isinstance(remote, dict) or remote.get('id') != remote_id
            or not isinstance(remote.get('request'), dict) or not isinstance(remote.get('result'), dict)):
        raise Pending('remote_job_identity_or_schema_mismatch')
    if remote.get('status') not in TERMINAL:
        raise Pending('remote_job_not_terminal')
    accepted, observed = remote['request'], remote['result']
    if 'remote_uncertain' in observed and observed['remote_uncertain'] is not False:
        raise Pending('remote_job_itself_uncertain')
    if (accepted.get('service') != service or accepted.get('action') != action
            or accepted.get('confirm_restart') is not True):
        raise Pending('remote_job_request_mismatch')
    immutable = {name: accepted[name] for name in ('service', 'action', 'confirm_restart')}
    commit, source = '', 'restart_without_commit'
    if action != 'restart':
        commit = accepted.get('commit')
        if not isinstance(commit, str) or not SHA.fullmatch(commit):
            raise Pending('remote_accepted_commit_missing')
        recorded = local_request.get('commit')
        if recorded is None and isinstance(local_request.get('ref'), str) and SHA.fullmatch(local_request['ref']):
            recorded = local_request['ref']
        if recorded is not None and (not isinstance(recorded, str) or not SHA.fullmatch(recorded) or recorded != commit):
            raise Pending('remote_accepted_commit_mismatch')
        source = 'local_pinned_commit' if recorded is not None else 'observed_legacy_agent_request'
        immutable['commit'] = commit
    if accepted.get('profile'):
        profile = accepted_profile(accepted['profile'])
        local_profile = local_request.get('profile_snapshot') or local_request.get('profile')
        if isinstance(local_profile, str):
            if profile['name'] != local_profile:
                raise Pending('remote_profile_mismatch')
        elif isinstance(local_profile, dict):
            if accepted_profile(local_profile) != profile:
                raise Pending('remote_profile_mismatch')
        else:
            raise Pending('unexpected_remote_profile')
        immutable['profile'] = profile
    elif local_request.get('profile') or local_request.get('profile_snapshot'):
        raise Pending('remote_profile_missing')
    if remote['status'] == 'succeeded':
        if observed.get('service') != service or (action != 'restart' and observed.get('commit') != commit):
            raise Pending('remote_success_receipt_mismatch')
        if 'commit' in observed and (not isinstance(observed['commit'], str) or not SHA.fullmatch(observed['commit'])):
            raise Pending('remote_success_receipt_mismatch')
        if any(name in observed and type(observed[name]) is not bool for name in ('ready', 'live', 'restarted')):
            raise Pending('invalid_remote_success_flags')
    return dict(remote_job=remote_id, remote_url=url, remote_status=remote['status'],
                commit=commit, commit_source=source, accepted_request=immutable,
                request_sha256=fingerprint(immutable), result_sha256=fingerprint(observed),
                ready=observed.get('ready') if remote['status'] == 'succeeded' else None,
                result=({name: observed[name] for name in ('service', 'commit', 'ready', 'live', 'restarted')
                         if name in observed} if remote['status'] == 'succeeded' else {}))


def controller_properties():
    process = subprocess.run(['systemctl', 'show', 'netcore-deployment.service', '-p', 'LoadState',
                              '-p', 'ActiveState', '-p', 'SubState', '-p', 'MainPID', '-p', 'User', '-p', 'StateDirectory'],
                             check=True, capture_output=True, text=True, timeout=10)
    return dict(line.split('=', 1) for line in process.stdout.splitlines() if '=' in line)


def user_uid(user):
    import pwd
    return (pwd.getpwuid(int(user)) if user.isdecimal() else pwd.getpwnam(user)).pw_uid


def controller_owner(state):
    value = controller_properties()
    user = value.get('User') or 'root'
    directories = value.get('StateDirectory', '').split()
    if (value.get('LoadState') != 'loaded' or not NAME.fullmatch(user) or not directories
            or any(not NAME.fullmatch(name) or name in ('.', '..') for name in directories)
            or Path(state).absolute() not in {(Path('/var/lib') / name).absolute() for name in directories}):
        raise Pending('controller_user_or_state_directory_not_verified')
    try:
        return user_uid(user)
    except (KeyError, ValueError) as exc:
        raise Pending('controller_owner_uid_not_verified') from exc


def controller_stopped():
    value = controller_properties()
    return (value.get('LoadState') == 'loaded' and value.get('ActiveState') in ('inactive', 'failed')
            and value.get('SubState') in ('dead', 'failed') and value.get('MainPID') == '0')


def reconcile(path, key, cfg, *, apply=False, fetch=get_json, stopped=controller_stopped, trusted_uid=None):
    summary = dict(job_id=key, phase='pending', changed=False, can_apply=False)
    try:
        if not isinstance(key, str) or not JOB_ID.fullmatch(key):
            raise Pending('invalid_local_job_id')
        if cfg.get('role') != 'controller' or not isinstance(cfg.get('environment'), str):
            raise Pending('controller_config_required')
        if apply and trusted_uid is None:
            trusted_uid = controller_owner(Path(path).parent)
        path = safe_file(path, private=apply, trusted_uid=trusted_uid)
        row, request, result = read_row(path, key)
        if row[3] not in TERMINAL:
            raise Pending('local_job_not_terminal')
        if result.get('remote_uncertain') is not True:
            summary.update(phase='already_resolved', reason='local_job_not_uncertain')
            return summary
        checked = proof(cfg, row, request, result, fetch)
        summary.update(node_id=request['node_id'], service=request['service'], action=request['action'],
                       **{name: checked[name] for name in ('remote_job', 'remote_url', 'remote_status', 'commit', 'commit_source')})
        summary.update(phase='verified', can_apply=True)
        if not apply:
            return summary
        if not stopped():
            raise Pending('controller_must_be_stopped')
        fresh = proof(cfg, row, request, result, fetch)
        if fresh != checked:
            raise Pending('remote_proof_changed_retry_diagnosis')
        safe_file(path, private=True, trusted_uid=trusted_uid)
        now = time.time()
        status = fresh['remote_status']
        if status == 'succeeded' and any(fresh['result'].get(name) is False for name in ('ready', 'live', 'restarted')):
            status = 'failed'
        after = dict(result, remote_job=fresh['remote_job'], remote_url=fresh['remote_url'],
                     remote_uncertain=False, dispatch_state='reconciled', **fresh['result'])
        after.pop('not_attempted', None)
        if status == 'succeeded':
            after.pop('error', None)
        else:
            after['error'] = ('Remote-Auftrag ist abgeschlossen, meldet aber keinen erfolgreichen Dienstzustand.' if
                              fresh['remote_status'] == 'succeeded' else 'Remote-Auftrag ist abgeschlossen: ' + status)
        audit_id = uuid.uuid4().hex
        evidence = dict(fresh, observed_at=now, node_id=request['node_id'],
                        local_request_sha256=fingerprint(request))
        after['reconciliation'] = dict(id=audit_id, observed_at=now,
                                       remote_status=fresh['remote_status'], request_sha256=fresh['request_sha256'])
        with closing(sqlite3.connect(path.as_uri() + '?mode=rw', uri=True, timeout=10)) as db:
            db.execute('PRAGMA synchronous=FULL')
            db.execute('BEGIN IMMEDIATE')
            try:
                current = db.execute(f'SELECT {COLUMNS} FROM jobs WHERE id=?', (key,)).fetchone()
                if current != row:
                    raise Pending('local_job_changed_retry_diagnosis')
                if db.execute("SELECT 1 FROM jobs WHERE status IN ('queued','running') LIMIT 1").fetchone():
                    raise Pending('controller_jobs_not_idle')
                if not stopped():
                    raise Pending('controller_must_be_stopped')
                safe_file(path, private=True, trusted_uid=trusted_uid)
                db.execute('CREATE TABLE IF NOT EXISTS remote_job_reconciliations '
                           '(id TEXT PRIMARY KEY, job_id TEXT, observed REAL, before_status TEXT, '
                           'before_result TEXT, after_status TEXT, after_result TEXT, proof TEXT)')
                db.execute('INSERT INTO remote_job_reconciliations VALUES (?,?,?,?,?,?,?,?)',
                           (audit_id, key, now, row[3], row[6], status, canonical(after), canonical(evidence)))
                changed = db.execute('UPDATE jobs SET status=?,result=?,updated=? WHERE id=? AND created=? '
                                     'AND updated=? AND status=? AND request=? AND log=? AND result=?',
                                     (status, canonical(after), now, *row)).rowcount
                if changed != 1:
                    raise Pending('local_job_changed_retry_diagnosis')
                db.commit()
            except Exception:
                db.rollback()
                raise
        summary.update(phase='reconciled', changed=True, status=status, audit_id=audit_id)
    except Pending as exc:
        summary.update(phase='pending', reason=str(exc), can_apply=False)
    except Exception:
        # Never print request, remote response, URL arguments or logs as errors.
        summary.update(phase='pending', reason='local_or_remote_evidence_unavailable', can_apply=False)
    return summary


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('job_id')
    parser.add_argument('--config', type=Path, default=Path('/etc/netcore/deployment.toml'))
    parser.add_argument('--apply', action='store_true', help='Persist verified terminal proof while controller is stopped')
    args = parser.parse_args()
    try:
        if args.apply and (os.name != 'posix' or os.geteuid() != 0):
            raise Pending('apply_requires_root_on_controller')
        config = safe_file(args.config)
        original = config.read_bytes()
        cfg = tomllib.loads(original.decode())
        cfg.setdefault('environment', 'netcore-openlab')
        cfg.setdefault('allowed_networks', ['10.0.0.0/8', '172.16.0.0/12', '192.168.0.0/16', '127.0.0.0/8'])
        path = Path(cfg.get('state_dir', '/var/lib/netcore-deployment')) / 'jobs.sqlite3'
        trusted_uid = controller_owner(path.parent) if args.apply else None
        def stable_stopped():
            return (config.read_bytes() == original and controller_stopped()
                    and controller_owner(path.parent) == trusted_uid)
        result = reconcile(path, args.job_id, cfg, apply=args.apply, stopped=stable_stopped, trusted_uid=trusted_uid)
    except Exception:
        result = dict(job_id=args.job_id, phase='pending', changed=False, can_apply=False,
                      reason='controller_config_or_permissions_invalid')
    print(json.dumps(result, ensure_ascii=False, indent=2))
    return 0 if result['phase'] in ('verified', 'reconciled', 'already_resolved') else 2


if __name__ == '__main__':
    raise SystemExit(main())
