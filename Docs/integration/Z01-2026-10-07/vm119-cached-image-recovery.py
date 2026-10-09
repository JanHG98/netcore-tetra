#!/usr/bin/env python3
"""Recreate the confirmed VM119 station image from its exact prepared cache."""
import contextlib
import fcntl
import hashlib
import json
import os
from pathlib import Path
import re
import socket
import sqlite3
import stat
import subprocess
import sys
import tempfile
import tomllib

LIBRARY = Path('/usr/local/lib/netcore-deployment')
STATE = Path('/var/lib/netcore-image-builder')
CONFIG = Path('/etc/netcore/deployment.toml')
SOURCE_JOB = 'f141c06d4b2f48699486e149c4887b16'
SOURCE_BUILD = '2e2603beacd942dfaddc7119b85a4af0'
COMMIT = '1595259a2a76abfc9eff08842409156473b585a7'
RECIPE = '7699808215f76322a6ed0cceddf5804d24f707e088ceabd7bdae5e7ef07b6eb6'
CACHE_BYTES = 5108662272
RECEIPT = STATE / ('recovery-' + SOURCE_BUILD + '.json')
LOCK = STATE / ('.recovery-' + SOURCE_BUILD + '.lock')
BASE_URL = 'http://10.0.1.131:8320'


class RecoveryError(RuntimeError):
    pass


def require(condition, message):
    if not condition:
        raise RecoveryError(message)


def request_digest(request):
    private = {key: value for key, value in request.items() if key != 'build_id'}
    return hashlib.sha256(json.dumps(private, sort_keys=True).encode()).hexdigest()


def read_jobs():
    path = STATE / 'jobs.sqlite3'
    require(path.is_file() and not path.is_symlink(), 'Auftragsdatenbank fehlt oder ist ein Symlink.')
    with contextlib.closing(sqlite3.connect(path.resolve().as_uri() + '?mode=ro', uri=True, timeout=5)) as db:
        rows = db.execute('SELECT id,status,request,result FROM jobs').fetchall()
    jobs = []
    for key, status, request, result in rows:
        try:
            request, result = json.loads(request), json.loads(result)
        except (ValueError, TypeError):
            raise RecoveryError('Ungültige persistierte Auftragsdaten.') from None
        require(isinstance(request, dict) and isinstance(result, dict), 'Ungültige persistierte Auftragsdaten.')
        require(isinstance(key, str) and status in ('queued', 'running', 'succeeded', 'failed', 'interrupted') and
                isinstance(result.get('remote_uncertain', False), bool), 'Ungültiger persistierter Auftragsstatus.')
        jobs.append(dict(id=key, status=status, request=request, result=result))
    return jobs


def save_receipt(receipt):
    descriptor, staging = tempfile.mkstemp(prefix='.cached-recovery-', dir=STATE)
    try:
        with os.fdopen(descriptor, 'w') as output:
            os.fchmod(output.fileno(), 0o600)
            json.dump(receipt, output, indent=2)
            output.write('\n')
            output.flush()
            os.fsync(output.fileno())
        os.replace(staging, RECEIPT)
        descriptor = os.open(STATE, os.O_RDONLY | os.O_DIRECTORY)
        try:
            os.fsync(descriptor)
        finally:
            os.close(descriptor)
    finally:
        if os.path.exists(staging):
            os.unlink(staging)


def summary(job):
    key = job['request'].get('build_id')
    require(isinstance(key, str) and re.fullmatch('[0-9a-f]{32}', key), 'Ungültige Artefakt-ID des Recoveryauftrags.')
    result = dict(status=job['status'], job_id=job['id'], build_id=key, commit=COMMIT,
                  receipt=str(RECEIPT), new_post=False)
    if job['status'] == 'succeeded':
        directory = STATE / 'artifacts' / key
        try:
            manifest = json.loads((directory / 'manifest.json').read_text())
            image = directory / 'image.img.xz'
            checksum = (directory / 'image.sha256').read_text().split()[0]
            require(not directory.is_symlink() and not image.is_symlink(), 'Artefaktsymlink nicht erlaubt.')
            require(manifest['id'] == key and manifest['commit'] == COMMIT and manifest['recipe'] == RECIPE,
                    'Recoverymanifest passt nicht zum gepinnten Cache.')
            require(image.is_file() and image.stat().st_size == manifest['size_bytes'], 'Recoveryimage fehlt oder hat falsche Größe.')
            require(checksum == manifest['sha256'] and re.fullmatch('[0-9a-f]{64}', checksum), 'Recoveryprüfsumme inkonsistent.')
        except (OSError, ValueError, KeyError, IndexError, TypeError):
            raise RecoveryError('Erfolgsjob ohne prüfbares Recoveryartefakt; kein neuer Auftrag.') from None
        result.update(filename=manifest['filename'], size_bytes=manifest['size_bytes'], sha256=checksum,
                      image_url=BASE_URL + '/api/v1/images/' + key + '/image',
                      sha256_url=BASE_URL + '/api/v1/images/' + key + '/sha256',
                      manifest_url=BASE_URL + '/api/v1/images/' + key + '/manifest',
                      file_sha256_not_recomputed=True)
    elif job['status'] not in ('queued', 'running'):
        result['followup'] = 'Auftrag beendet ohne Erfolg; vorhandenen Auftrag prüfen, kein automatischer Neuauftrag.'
    return result


def resume(receipt):
    require(receipt.get('source_job') == SOURCE_JOB and receipt.get('commit') == COMMIT and
            receipt.get('recipe') == RECIPE and receipt.get('phase') in ('submit_started', 'submitted'),
            'Unerwarteter Recoverybeleg; kein neuer Auftrag.')
    baseline = receipt.get('baseline_ids')
    fingerprint = receipt.get('request_sha256')
    require(isinstance(baseline, list) and all(isinstance(key, str) for key in baseline) and
            isinstance(fingerprint, str) and re.fullmatch('[0-9a-f]{64}', fingerprint),
            'Ungültiger Recoverybeleg; kein neuer Auftrag.')
    candidates = [job for job in read_jobs() if job['id'] not in baseline and
                  request_digest(job['request']) == fingerprint]
    require(len(candidates) == 1, 'POST-Ausgang ungeklärt: kein eindeutig zugehöriger Auftrag; Beleg erhalten, kein neuer POST.')
    job = candidates[0]
    require(receipt.get('job_id', job['id']) == job['id'], 'Recoveryauftrags-ID inkonsistent; kein neuer Auftrag.')
    if receipt['phase'] != 'submitted':
        receipt.update(phase='submitted', job_id=job['id'])
        save_receipt(receipt)
    return summary(job)


def recover(client, recipe_key, validate_image, networks):
    if RECEIPT.exists() or RECEIPT.is_symlink():
        require(not RECEIPT.is_symlink() and stat.S_ISREG(RECEIPT.stat().st_mode), 'Ungültiger Recoverybeleg.')
        try:
            receipt = json.loads(RECEIPT.read_text())
        except (ValueError, TypeError):
            raise RecoveryError('Recoverybeleg beschädigt; kein neuer POST.') from None
        require(isinstance(receipt, dict), 'Recoverybeleg beschädigt; kein neuer POST.')
        return resume(receipt)
    jobs = read_jobs()
    old = next((job for job in jobs if job['id'] == SOURCE_JOB), None)
    require(old is not None and old['status'] == 'succeeded', 'Der bestätigte ursprüngliche Erfolgsjob fehlt.')
    require(old['request'].get('build_id') == SOURCE_BUILD and old['request'].get('commit') == COMMIT and
            old['result'].get('id') == SOURCE_BUILD and old['result'].get('commit') == COMMIT and
            old['result'].get('recipe') == RECIPE, 'Ursprünglicher Erfolgsjob passt nicht zum geprüften Stand.')
    if (STATE / 'artifacts' / SOURCE_BUILD / 'image.img.xz').is_file():
        return summary(old)
    for job in jobs:
        require(job['status'] not in ('queued', 'running') and not job['result'].get('remote_uncertain'),
                'Aktiver oder ungeklärter Imageauftrag; zuerst vorhandenen Auftrag klären.')
    require(client.request('/status').get('available') is True, 'Imageworker ist nicht verfügbar.')
    work = STATE / 'work'
    require(not work.is_symlink() and (not work.exists() or work.is_dir()), 'Unerwarteter Workpfad.')
    if work.exists():
        require(not any(re.fullmatch('[0-9a-f]{32}', path.name) for path in work.iterdir()),
                'Verwaister Workpfad; vor Recovery ausschließlich bestehende Daten klären.')
    image, metadata = STATE / 'cache' / (RECIPE + '.img'), STATE / 'cache' / (RECIPE + '.json')
    require(image.is_file() and not image.is_symlink() and image.stat().st_size == CACHE_BYTES and
            metadata.is_file() and not metadata.is_symlink(), 'Geprüftes Cachepaar fehlt oder hat eine andere Größe.')
    require(recipe_key(COMMIT) == RECIPE, 'Installiertes Rezept passt nicht zum Cache; kein vollständiger Build gestartet.')
    try:
        versions = json.loads(metadata.read_text())
    except (ValueError, TypeError):
        raise RecoveryError('Cachemetadaten beschädigt.') from None
    require(isinstance(versions, dict) and versions.get('commit') == COMMIT, 'Cachecommit stimmt nicht überein.')
    # The persisted private request contains the original site/OS/WLAN/VPN values.
    # Its plaintext fields are never printed or written into the receipt.
    try:
        request = validate_image(old['request'], networks)
    except Exception:
        raise RecoveryError('Gespeicherter Stationsauftrag besteht die aktuelle Validierung nicht.') from None
    require(request.get('commit') == COMMIT, 'Stationsauftrag enthält einen anderen Commit.')
    require(all(value == old['request'].get(key) for key, value in request.items()),
            'Validierung würde gespeicherte Stationseinstellungen verändern.')
    # Catch new work or cache/recipe changes before recording the single POST attempt.
    current = read_jobs()
    require({job['id'] for job in current} == {job['id'] for job in jobs} and
            all(job['status'] not in ('queued', 'running') and not job['result'].get('remote_uncertain') for job in current),
            'Auftragslage während der Vorprüfung geändert.')
    require(recipe_key(COMMIT) == RECIPE and image.stat().st_size == CACHE_BYTES and
            json.loads(metadata.read_text()).get('commit') == COMMIT, 'Cache während der Vorprüfung geändert.')
    receipt = dict(phase='submit_started', source_job=SOURCE_JOB, source_build=SOURCE_BUILD,
                   commit=COMMIT, recipe=RECIPE, request_sha256=request_digest(request),
                   baseline_ids=[job['id'] for job in current])
    save_receipt(receipt)  # A durable pending attempt forbids every subsequent retry POST.
    try:
        client.request('/build', request)
    except Exception:
        # Acceptance can have happened before the HTTP response was lost.
        # Reconcile SQLite only; never submit a second time.
        pass
    result = resume(receipt)
    result['new_post'] = True
    return result


def main():
    require(len(sys.argv) == 1 and os.geteuid() == 0 and socket.gethostname() == 'VM-H-DEPLOY-01',
            'Nur root auf VM-H-DEPLOY-01; keine Operator-Overrides.')
    require(STATE.is_dir() and STATE.resolve() == STATE and CONFIG.is_file() and not CONFIG.is_symlink(),
            'Unerwarteter State- oder Konfigurationspfad.')
    cfg = tomllib.loads(CONFIG.read_text())
    require(cfg.get('role') == 'controller' and cfg.get('node_id') == 'VM-H-DEPLOY-01', 'Falsche Controlleridentität.')
    sys.path.insert(0, str(LIBRARY))
    from common import load_config
    from image_build import recipe_key
    from image_client import ImageClient
    from image_spec import validate_image
    require(subprocess.run(['systemctl', 'is-active', '--quiet', 'netcore-image-builder.service'], timeout=10).returncode == 0,
            'Imageworker ist nicht aktiv.')
    descriptor = os.open(LOCK, os.O_CREAT | os.O_RDWR | os.O_NOFOLLOW, 0o600)
    try:
        os.fchmod(descriptor, 0o600)
        fcntl.flock(descriptor, fcntl.LOCK_EX | fcntl.LOCK_NB)
        result = recover(ImageClient(), recipe_key, validate_image, load_config(CONFIG)['allowed_networks'])
        print(json.dumps(result, indent=2, ensure_ascii=False))
        if result['status'] in ('queued', 'running'):
            print('Recovery läuft: WebUI → Builds & Downloads. Derselbe Helfer fragt später ausschließlich diesen Auftrag ab.')
    finally:
        os.close(descriptor)


if __name__ == '__main__':
    try:
        main()
    except RecoveryError as error:
        print('STOP: ' + str(error), file=sys.stderr)
        sys.exit(1)
    except Exception as error:
        print('STOP: ' + type(error).__name__ + '; kein automatischer Neuauftrag. Vorhandenen Recoverybeleg prüfen.', file=sys.stderr)
        sys.exit(1)
