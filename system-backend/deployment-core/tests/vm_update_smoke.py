#!/usr/bin/env python3
"""Exercise the real idle VM updater and rollback on a disposable CI install."""
import argparse
import importlib.util
import json
import os
from pathlib import Path
import sqlite3
import subprocess
import tomllib
from unittest.mock import patch


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--disposable-runner', action='store_true', required=True)
    parser.parse_args()
    config = tomllib.loads(Path('/etc/netcore/deployment.toml').read_text())
    if os.geteuid() != 0 or config.get('node_id') != 'ci-deploy':
        raise RuntimeError('Only the disposable ci-deploy fixture is supported.')
    root = Path(__file__).resolve().parents[3]
    spec = importlib.util.spec_from_file_location('vm_update', root / 'tools/operator/update-vm119-image-features.py')
    updater = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(updater)
    commit = subprocess.check_output(['git', '-C', str(root), 'rev-parse', 'HEAD'], text=True).strip()
    asset = updater.LIBRARY / 'static/index.html'
    original = asset.read_bytes()
    try:
        # Actual existing package difference, real systemd restarts and live HTTP.
        asset.write_bytes(original + b'\n<!-- disposable previous package -->\n')
        result = updater.update(commit, apply=True)
        assert result['phase'] == 'passed', result
        assert asset.read_bytes() == original
        before = {u:updater.properties(u)['InvocationID'] for u in (updater.BUILDER, updater.CONTROLLER)}
        assert updater.update(commit, apply=True)['phase'] == 'already_current'
        assert before == {u:updater.properties(u)['InvocationID'] for u in before}
        # A queued image must reject before either service is stopped.
        with sqlite3.connect(updater.BUILD_STATE / 'jobs.sqlite3') as db:
            db.execute('INSERT INTO jobs VALUES (?,?,?,?,?,?,?)',
                       ('ci-busy-fixture', 0, 0, 'queued', json.dumps({'build_id':'f'*32}), '', '{}'))
        try:
            try:
                updater.update(commit, apply=True)
            except RuntimeError as exc:
                assert 'Aktiver Auftrag' in str(exc), str(exc)
            else:
                raise AssertionError('An active image job did not reject the update.')
            assert before == {u:updater.properties(u)['InvocationID'] for u in before}
        finally:
            with sqlite3.connect(updater.BUILD_STATE / 'jobs.sqlite3') as db:
                db.execute('DELETE FROM jobs WHERE id=?', ('ci-busy-fixture',))
        # Force a post-swap readiness failure; restore the actual old package and
        # bring back both services. This tests filesystem+systemd rollback together.
        previous = original + b'\n<!-- disposable rollback package -->\n'
        asset.write_bytes(previous)
        with patch.object(updater, 'wait_ready', side_effect=RuntimeError('ci-readiness-failure')):
            try:
                updater.update(commit, apply=True)
            except RuntimeError as exc:
                assert str(exc) == 'ci-readiness-failure'
            else:
                raise AssertionError('The readiness failure did not reject the update.')
        assert asset.read_bytes() == previous
        updater.wait_ready()
        assert updater.update(commit, apply=True)['phase'] == 'passed'
        assert asset.read_bytes() == original
        print('PASS: real VM files, service invocations, HTTP/socket readiness, config preservation, idle rejection, idempotence and rollback.')
    finally:
        asset.write_bytes(original)
        subprocess.run(['systemctl', 'start', updater.BUILDER, updater.CONTROLLER], check=True)


if __name__ == '__main__':
    main()
