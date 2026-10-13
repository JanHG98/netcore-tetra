#!/usr/bin/env python3
"""Disposable Linux proof of real systemd/Python/HTTP runtime replacement.

Run as root on an isolated CI runner. This creates one uniquely named fixture
unit and temporary source/runtime trees, never a production NetCore service.
No systemctl or /proc operations are mocked. No RF or external service is used.
"""
from __future__ import annotations

import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import socket
import subprocess
import sys
import tempfile
import time
import urllib.request
import uuid


ROOT = Path(__file__).resolve().parents[1]
COMMIT = 'a' * 40


def run(args, **kwargs):
    return subprocess.run(args, check=True, **kwargs)


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def fetch(port, path='/version'):
    opener = urllib.request.build_opener(urllib.request.ProxyHandler({}))
    with opener.open(f'http://127.0.0.1:{port}{path}', timeout=3) as response:
        return json.loads(response.read())


def wait_version(port, expected):
    error = None
    for _ in range(30):
        try:
            actual = fetch(port)
            if actual.get('version') == expected:
                return actual
        except (OSError, ValueError) as exc:
            error = exc
        time.sleep(0.2)
    raise RuntimeError(f'Fixture HTTP version {expected!r} not reached: {error}')


RUNTIME = r'''#!/usr/bin/env python3
import argparse
import json
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
import tomllib
__VERSION_LINE__
p = argparse.ArgumentParser()
p.add_argument('--config', required=True)
a = p.parse_args()
cfg = tomllib.loads(Path(a.config).read_text())
host, port = cfg['server']['bind'].rsplit(':', 1)
class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path in ('/health/live', '/health/ready'):
            body = {'ok': True, 'ready': True, 'version': VERSION}
        else:
            body = {'version': VERSION, 'pid': __import__('os').getpid(),
                    'page': Path(__file__).with_name('page.html').read_text()}
        raw = json.dumps(body).encode()
        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.send_header('Content-Length', str(len(raw)))
        self.end_headers()
        self.wfile.write(raw)
    def log_message(self, *args):
        pass
ThreadingHTTPServer((host, int(port)), Handler).serve_forever()
'''


RUST_RUNTIME = r'''
use std::io::{Read, Write};
use std::net::TcpListener;
fn main() {
    let config = std::env::args().nth(2).expect("--config PATH");
    let text = std::fs::read_to_string(config).unwrap();
    let bind = text.lines().find(|line| line.starts_with("bind")).unwrap()
        .split_once('=').unwrap().1.trim().trim_matches('"');
    let listener = TcpListener::bind(bind).unwrap();
    for connection in listener.incoming() {
        let mut stream = connection.unwrap();
        let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(3)));
        let mut request = [0u8; 8192];
        if stream.read(&mut request).unwrap_or(0) == 0 { continue; }
        let body = format!(r#"{{"ok":true,"ready":true,"version":"__VERSION__","pid":{}}}"#,
                           std::process::id());
        let response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                               body.len(), body);
        let _ = stream.write_all(response.as_bytes());
    }
}
'''


def elf_fixture(update, work):
    """Actual compiled Rust std fixture; this is not a NetCore backend build."""
    # sudo's secure PATH excludes the runner's rustup shims. CI supplies the
    # actual compiler from `rustup which rustc`, so no root toolchain is guessed.
    rustc = os.environ.get('NETCORE_NATIVE_RUSTC') or shutil.which('rustc')
    if not rustc:
        raise RuntimeError('Native Rust/ELF fixture requires rustc on the disposable CI runner.')
    source, installed = work / 'elf-source', work / 'elf-runtime'
    (source / 'fixtures').mkdir(parents=True)
    (source / 'tools/operator').mkdir(parents=True)
    installed.mkdir()
    target, new_binary = installed / 'runtime', source / 'fixtures/runtime'
    rust_source = source / 'fixtures/runtime.rs'
    rust_source.write_text(RUST_RUNTIME.replace('__VERSION__', 'e1'), encoding='utf-8')
    run([rustc, '--edition=2021', '-O', str(rust_source), '-o', str(target)])
    rust_source.write_text(RUST_RUNTIME.replace('__VERSION__', 'e2'), encoding='utf-8')
    run([rustc, '--edition=2021', '-O', str(rust_source), '-o', str(new_binary)])
    assert target.read_bytes()[:4] == b'\x7fELF' and new_binary.read_bytes()[:4] == b'\x7fELF'
    unit_name = 'netcore-runtime-elf-ci-' + uuid.uuid4().hex + '.service'
    unit_path = Path('/etc/systemd/system') / unit_name
    config = work / 'elf-config.toml'
    with socket.socket() as sock:
        sock.bind(('127.0.0.1', 0))
        port = sock.getsockname()[1]
    config.write_text(f'[server]\nbind = "127.0.0.1:{port}"\n', encoding='utf-8')
    (source / 'tools/operator/service-update-assets.json').write_text(json.dumps({
        'version': 1, 'roles': {'elf-fixture': {'files': [
            {'source': 'fixtures/runtime', 'target': str(target), 'mode': '0755'}]}}}), encoding='utf-8')
    spec = dict(name='elf-fixture', unit=unit_name, command=f'{target} --config {config}',
                config_target=str(config), port=port, health='/health/ready', depends_on=[])
    unit_path.write_text('[Unit]\nDescription=Disposable compiled Rust ELF update fixture\n'
                        '[Service]\nType=simple\n'
                        f'ExecStart={target} --config {config}\nRestart=no\nTimeoutStopSec=5\n', encoding='utf-8')
    config_before, unit_before = sha(config), sha(unit_path)
    try:
        run(['systemctl', 'daemon-reload'])
        run(['systemctl', 'start', unit_name])
        old = wait_version(port, 'e1')
        props, matched = update.validate_install(spec)
        assert matched == target
        plan = update.prepare_plan(spec, target, props, 'c' * 40, root=source)
        assert plan['runtime_kind'] == 'elf'
        assert wait_version(port, 'e1')['pid'] == old['pid']
        receipt = update.apply_plan(spec, plan, props, root=source,
                                    backup_root=work / 'elf-backups', receipt_root=work / 'elf-receipts')
        new = wait_version(port, 'e2')
        assert new['pid'] != old['pid']
        assert receipt['process']['kind'] == 'elf'
        assert receipt['process']['MainPID'] == new['pid']
        assert receipt['process']['InvocationID'] != receipt['process']['previous_invocation']
        assert sha(Path('/proc') / str(new['pid']) / 'exe') == sha(target) == receipt['process']['exe_sha256']
        assert receipt['files'][0]['sha256'] == sha(new_binary) == sha(target)
        assert receipt['liveness'] is True and receipt['readiness'] is True
        assert sha(config) == config_before and sha(unit_path) == unit_before
        return dict(native_compiled_rust_fixture=True, actual_netcore_backend_build=False,
                    rustc=run([rustc, '--version'], capture_output=True, text=True).stdout.strip(),
                    actual_proc_exe_sha256=receipt['process']['exe_sha256'],
                    old_pid=old['pid'], new_pid=new['pid'], receipt=receipt)
    finally:
        subprocess.run(['systemctl', 'stop', unit_name], check=False)
        subprocess.run(['systemctl', 'reset-failed', unit_name], check=False)
        unit_path.unlink(missing_ok=True)
        run(['systemctl', 'daemon-reload'])


def main():
    if sys.platform != 'linux' or os.geteuid() != 0:
        raise RuntimeError('Native fixture requires root on an isolated Linux systemd CI runner.')
    run(['systemctl', 'show', '--property=Version'], capture_output=True, text=True)
    module_spec = importlib.util.spec_from_file_location(
        'netcore_runtime_update', ROOT / 'tools/operator/update-netcore-service.py')
    update = importlib.util.module_from_spec(module_spec)
    module_spec.loader.exec_module(update)
    unit_name = 'netcore-runtime-ci-' + uuid.uuid4().hex + '.service'
    unit_path = Path('/etc/systemd/system') / unit_name
    dropin_dir = Path(str(unit_path) + '.d')
    work = Path(tempfile.mkdtemp(prefix='netcore-runtime-ci-', dir='/var/tmp'))
    source, installed = work / 'source', work / 'runtime'
    (source / 'fixtures').mkdir(parents=True)
    (source / 'tools/operator').mkdir(parents=True)
    installed.mkdir()
    target = installed / 'runtime.py'
    page, extra = installed / 'page.html', installed / 'new_helper.py'
    config = work / 'hardware.toml'
    with socket.socket() as sock:
        sock.bind(('127.0.0.1', 0))
        port = sock.getsockname()[1]
    config.write_text(f'[server]\nbind = "127.0.0.1:{port}"\n', encoding='utf-8')
    target.write_text(RUNTIME.replace('__VERSION_LINE__', "VERSION = 'v1'"), encoding='utf-8')
    target.chmod(0o755)
    page.write_text('old static page', encoding='utf-8')
    source_runtime = source / 'fixtures/runtime.py'
    source_runtime.write_text(RUNTIME.replace('__VERSION_LINE__', 'from new_helper import VERSION'), encoding='utf-8')
    (source / 'fixtures/new_helper.py').write_text("VERSION = 'v2'\n", encoding='utf-8')
    (source / 'fixtures/page.html').write_text('new static page', encoding='utf-8')
    assets = {'version': 1, 'roles': {'hardware-gateway': {'files': [
        {'source': 'fixtures/runtime.py', 'target': str(target), 'mode': '0755'},
        {'source': 'fixtures/new_helper.py', 'target': str(extra), 'mode': '0644'},
        {'source': 'fixtures/page.html', 'target': str(page), 'mode': '0644'},
    ]}}}
    (source / 'tools/operator/service-update-assets.json').write_text(json.dumps(assets), encoding='utf-8')
    spec = dict(name='hardware-gateway', unit=unit_name,
                command=f'{target} --config {config}', config_target=str(config),
                port=port, health='/health/ready', depends_on=[])
    unit_path.write_text('[Unit]\nDescription=Disposable NetCore runtime CI fixture\n'
                        '[Service]\nType=simple\n'
                        f'ExecStart=/usr/bin/python3 {target} --config {config}\n'
                        'Restart=no\nTimeoutStopSec=5\n', encoding='utf-8')
    dropin_dir.mkdir()
    dropin = dropin_dir / 'preserve.conf'
    dropin.write_text('[Service]\nEnvironment=NETCORE_TEST_SENTINEL=preserve\n', encoding='utf-8')
    config_before, unit_before, dropin_before = sha(config), sha(unit_path), sha(dropin)
    # Keep all agent metadata for this fixture in its temporary tree as well.
    agent_state = work / 'agent-state'
    update.DISCOVERY_CONFIG = work / 'discovery.toml'
    update.DISCOVERY_CONFIG.write_text(f'state_dir = "{agent_state}"\n', encoding='utf-8')
    if hasattr(update, 'RECEIPT_ROOT'):
        update.RECEIPT_ROOT = work / 'receipts'
    evidence = {'native_systemd': True, 'native_http': True, 'actual_netcore_rust_build': False,
                'fixture_unit': unit_name, 'source_commit': COMMIT}
    try:
        run(['systemctl', 'daemon-reload'])
        run(['systemctl', 'start', unit_name])
        old = wait_version(port, 'v1')
        props, matched = update.validate_install(spec)
        assert matched == target
        old_invocation = update.properties(unit_name).get('InvocationID')
        plan = update.prepare_plan(spec, target, props, COMMIT, root=source)
        assert wait_version(port, 'v1')['pid'] == old['pid'], 'Preparation restarted runtime'
        assert not extra.exists(), 'Preparation published new module before apply'
        # Apply consumes the immutable snapshot, not source files edited afterward.
        source_runtime.write_text("raise RuntimeError('source changed after prepare')\n", encoding='utf-8')
        receipt = update.apply_plan(spec, plan, props, root=source,
                                    backup_root=work / 'backups', receipt_root=work / 'receipts')
        new = wait_version(port, 'v2')
        assert new['pid'] != old['pid'], 'The old process remained after runtime update'
        assert new['page'] == 'new static page' and extra.is_file()
        assert update.properties(unit_name).get('InvocationID') != old_invocation
        assert receipt['source_commit'] == COMMIT and receipt['assets_complete'] is True
        assert receipt['liveness'] is True and receipt['readiness'] is True
        assert receipt['process']['MainPID'] == new['pid']
        assert all(sha(item['target']) == item['sha256'] for item in receipt['files'])
        assert sha(config) == config_before and sha(unit_path) == unit_before and sha(dropin) == dropin_before
        evidence['updated'] = receipt
        evidence['new_module_and_static'] = True
        evidence['immutable_prepared_snapshot'] = True
        evidence['config_unit_dropin_preserved'] = True
        marker_path = agent_state / 'deployed-hardware-gateway.json'
        marker = json.loads(marker_path.read_text(encoding='utf-8'))
        assert marker['commit'] == COMMIT and marker['state'] == 'installed'
        assert marker['process']['MainPID'] == new['pid']
        evidence['deployment_marker_matches_verified_runtime'] = True
        baseline = {path: sha(path) for path in (target, extra, page)}
        source_runtime.write_text("raise RuntimeError('intentional fixture startup failure')\n", encoding='utf-8')
        failed_props, _ = update.validate_install(spec)
        bad_plan = update.prepare_plan(spec, target, failed_props, 'b' * 40, root=source)
        try:
            update.apply_plan(spec, bad_plan, failed_props, root=source,
                              backup_root=work / 'backups', receipt_root=work / 'receipts')
        except (RuntimeError, subprocess.CalledProcessError):
            pass
        else:
            raise AssertionError('Invalid runtime startup was counted as a successful update')
        recovered = wait_version(port, 'v2')
        assert all(sha(path) == digest for path, digest in baseline.items())
        assert sha(config) == config_before and sha(unit_path) == unit_before and sha(dropin) == dropin_before
        evidence['actual_failed_start_rollback'] = True
        evidence['rollback_process'] = recovered
        failed_marker = json.loads(marker_path.read_text(encoding='utf-8'))
        assert failed_marker['commit'] == '' and failed_marker['state'] == 'rolled_back'
        evidence['failed_update_does_not_claim_installed_commit'] = True
        missing = run(['systemctl', 'show', unit_name + '-missing.service',
                       '-p', 'LoadState', '-p', 'FragmentPath'], capture_output=True, text=True)
        assert 'LoadState=not-found' in missing.stdout
        evidence['actual_missing_unit_show_exit'] = missing.returncode
        evidence['rust_elf_fixture'] = elf_fixture(update, work)
        evidence['phase'] = 'passed'
        output = ROOT / 'output/netcore-runtime-native.json'
        output.parent.mkdir(exist_ok=True)
        output.write_text(json.dumps(evidence, indent=2) + '\n', encoding='utf-8')
        print(json.dumps(evidence, indent=2), flush=True)
    finally:
        subprocess.run(['systemctl', 'stop', unit_name], check=False)
        subprocess.run(['systemctl', 'reset-failed', unit_name], check=False)
        unit_path.unlink(missing_ok=True)
        dropin.unlink(missing_ok=True)
        dropin_dir.rmdir()
        run(['systemctl', 'daemon-reload'])
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
