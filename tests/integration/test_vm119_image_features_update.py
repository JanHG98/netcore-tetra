"""VM119 coherent-package/idle/rollback tests with real files and SQLite.

systemd and HTTP are transport fixtures. These tests never stop a host service
and do not claim a desktop/image build or an actual VM rollout.
"""
from __future__ import annotations

from contextlib import contextmanager, redirect_stdout
import importlib.util
import io
import hashlib
import json
import os
from pathlib import Path
import sqlite3
import stat
import subprocess
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import Mock, patch


ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location('vm119_update', ROOT / 'tools/operator/update-vm119-image-features.py')
UPDATE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(UPDATE)
COMMIT = 'a' * 40


@contextmanager
def database(path):
    connection = sqlite3.connect(path)
    try:
        with connection:
            yield connection
    finally:
        connection.close()


class VM119PackageTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.source, self.library = self.root / 'source', self.root / 'library'
        self.state, self.builder = self.root / 'state', self.root / 'builder'
        self.backup, self.config = self.root / 'backups', self.root / 'deployment.toml'
        for directory in (self.source, self.library, self.state, self.builder):
            directory.mkdir()
        self.source_files = {
            'main.py': b"VERSION = 'new'\n",
            'image_worker.py': b"VERSION = 'new'\n",
            'image_spec.py': b"DESKTOPS = ('none','kde','xfce','gnome','pios')\n",
            'catalog.json': b'[]\n',
            'static/index.html': b'<html><body>current</body></html>\n',
            'static/app.js': b'const version = "new";\n',
            'image/vpn-policy.py': b"VERSION = 'new'\n",
            'image/desktop-setup.sh': b'#!/bin/sh\nexit 0\n',
            'image/systemd/tetra.service': b'[Unit]\nDescription=Guest recipe\n',
            'install/prepare-host.sh': b'#!/bin/sh\nexit 0\n',
            'install/install-tbs.sh': b'#!/bin/sh\nexit 0\n',
            'image/new/sub/module.py': b"VPN_FIELDS = ('username','password')\n",
        }
        self.git_payload = dict(self.source_files)
        self.git_objects = {}
        for name, body in self.source_files.items():
            path = self.source / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(body)
            if name != 'image/new/sub/module.py':
                target = self.library / name
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_bytes(b"OLD = True\n" if path.suffix == '.py' else b'old bytes\n')
        self.config.write_text('role = "controller"\n# preserve comments\n', encoding='utf-8')
        self.protected = [self.config]
        for name in ('settings.json', 'profiles.json', 'tbs-site-template.toml'):
            path = self.state / name
            path.write_text('private original bytes\n', encoding='utf-8')
            self.protected.append(path)
        self.units = {}
        for unit in (UPDATE.CONTROLLER, UPDATE.BUILDER):
            path = self.root / unit
            path.write_text('[Service]\nEnvironment=LOCAL_OVERRIDE=1\n', encoding='utf-8')
            self.protected.append(path)
            script = 'main.py' if unit == UPDATE.CONTROLLER else 'image_worker.py'
            pid = '1001' if unit == UPDATE.CONTROLLER else '1002'
            command = f'/usr/bin/python3 {self.library / script}'
            if unit == UPDATE.CONTROLLER:
                command += ' --config ' + str(self.config)
            self.units[unit] = {'InvocationID': '0' * 32, 'LoadState': 'loaded', 'ActiveState': 'active',
                                'MainPID': pid, 'FragmentPath': str(path), 'DropInPaths': '', 'ExecStart': command}
            proc = self.root / 'proc' / pid
            proc.mkdir(parents=True)
            argv = [b'/usr/bin/python3', os.fsencode(self.library / script)]
            if unit == UPDATE.CONTROLLER:
                argv += [b'--config', os.fsencode(self.config)]
            (proc / 'cmdline').write_bytes(b'\0'.join(argv) + b'\0')
        for state in (self.state, self.builder):
            with database(state / 'jobs.sqlite3') as db:
                db.execute('CREATE TABLE jobs (id TEXT, status TEXT, request TEXT, result TEXT)')
        self.commands, self.events = [], []

        def run(args, **kwargs):
            self.commands.append(args)
            if 'ls-tree' in args:
                rows = []
                self.git_objects.clear()
                for name, body in sorted(self.git_payload.items()):
                    object_id = hashlib.sha1(b'blob ' + str(len(body)).encode() + b'\0' + body).hexdigest()
                    self.git_objects[object_id] = body
                    rows.append(f'100644 blob {object_id}\tsystem-backend/deployment-core/{name}'.encode() + b'\0')
                return SimpleNamespace(stdout=b''.join(rows))
            if 'cat-file' in args:
                return SimpleNamespace(stdout=self.git_objects[args[-1]])
            if args[:2] == ['bash', '-n']:
                return SimpleNamespace(stdout='')
            if args[:2] == ['systemctl', 'stop']:
                self.events.append(('stop', args[2]))
            elif args[:2] == ['systemctl', 'start']:
                self.events.append(('start', args[2]))
                self.units[args[2]]['InvocationID'] = '1' * 32
            else:
                raise AssertionError('Unexpected operation: ' + repr(args))
            return SimpleNamespace(stdout='')

        @contextmanager
        def exclusive(path):
            self.events.append(('lock', str(path)))
            yield
            self.events.append(('unlock', str(path)))

        self.run = Mock(side_effect=run)
        self.ready, self.wait_ready = Mock(), Mock()
        original_replace = UPDATE.replace

        def replace(path, body, metadata=None):
            self.events.append(('replace', str(path)))
            # Test actual atomic filesystem writes on Windows too, while
            # ownership/fchmod are verified by the POSIX CI environment.
            with patch.object(UPDATE.os, 'fchown', create=True), patch.object(UPDATE, 'fsync_directory'):
                if os.name == 'nt':
                    with patch.object(UPDATE.os, 'fchmod', create=True):
                        return original_replace(path, body, metadata)
                return original_replace(path, body, metadata)

        self.replace = Mock(side_effect=replace)
        patches = {
            'SOURCE': self.source, 'LIBRARY': self.library, 'STATE': self.state,
            'BUILD_STATE': self.builder, 'CONFIG': self.config, 'BACKUP_ROOT': self.backup,
            'PROC_ROOT': self.root / 'proc',
            'run': self.run, 'ready': self.ready, 'wait_ready': self.wait_ready,
            'installed_paths': Mock(return_value=(self.protected, {k: dict(v) for k, v in self.units.items()})),
            'properties': Mock(side_effect=lambda unit: dict(self.units[unit])),
            'exclusive': Mock(side_effect=exclusive), 'replace': self.replace,
            # These are unprivileged filesystem tests, not POSIX owner checks.
            # The disposable root VM smoke verifies actual owner/mode behavior.
            'attributes': lambda info: (0, 0, 0o644 if os.name == 'nt' else stat.S_IMODE(info.st_mode)),
        }
        for name, value in patches.items():
            patcher = patch.object(UPDATE, name, value)
            patcher.start()
            self.addCleanup(patcher.stop)

    def insert_job(self, state, key, status, kind='image', result=None):
        with database(state / 'jobs.sqlite3') as db:
            db.execute('INSERT INTO jobs VALUES (?,?,?,?)',
                       (key, status, json.dumps({'kind': kind}), json.dumps(result or {})))

    def update(self, apply=True):
        with redirect_stdout(io.StringIO()):
            return UPDATE.update(COMMIT, apply)

    def test_plan_is_read_only_and_covers_module_ui_recipe_and_install_helpers(self):
        before = UPDATE.snapshot(self.protected)
        result = self.update(apply=False)
        self.assertEqual(result['phase'], 'planned')
        self.assertEqual(result['package_files'], len(self.source_files))
        self.assertCountEqual(result['changed_files'], self.source_files)
        self.assertEqual(UPDATE.snapshot(self.protected), before)
        self.assertFalse(self.backup.exists())
        self.assertFalse(self.events)

    def test_full_package_published_only_after_both_services_stop_and_worker_lock(self):
        before = UPDATE.snapshot(self.protected)
        result = self.update()
        self.assertEqual(result['phase'], 'passed')
        stop_controller = self.events.index(('stop', UPDATE.CONTROLLER))
        stop_builder = self.events.index(('stop', UPDATE.BUILDER))
        worker_lock = self.events.index(('lock', str(self.builder / 'worker.lock')))
        first_write = next(i for i, event in enumerate(self.events) if event[0] == 'replace')
        self.assertLess(stop_controller, stop_builder)
        self.assertLess(stop_builder, worker_lock)
        self.assertLess(worker_lock, first_write)
        self.assertLess(self.events.index(('start', UPDATE.BUILDER)),
                        self.events.index(('start', UPDATE.CONTROLLER)))
        for name, body in self.source_files.items():
            self.assertEqual((self.library / name).read_bytes(), body)
        self.assertEqual(UPDATE.snapshot(self.protected), before)
        self.assertTrue(Path(result['backup']).is_dir())
        self.assertEqual(len(result['files']), len(self.source_files))

    def test_identical_package_returns_already_current_without_restart(self):
        for name, body in self.source_files.items():
            path = self.library / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(body)
        self.assertEqual(self.update()['phase'], 'already_current')
        self.assertFalse(self.events)
        self.assertFalse(self.backup.exists())

    def test_active_job_in_either_full_database_blocks_before_stop(self):
        for state in (self.state, self.builder):
            with self.subTest(state=state):
                self.insert_job(state, 'active-job', 'running')
                with self.assertRaisesRegex(RuntimeError, 'Aktiver Auftrag'):
                    self.update()
                self.assertFalse(self.events)
                with database(state / 'jobs.sqlite3') as db:
                    db.execute('DELETE FROM jobs')

    def test_remote_uncertain_terminal_job_blocks_even_outside_status_page(self):
        for i in range(45):
            self.insert_job(self.state, f'terminal-{i}', 'succeeded', result={'remote_uncertain': i == 0})
        with self.assertRaisesRegex(RuntimeError, 'Ungeklaerter Remote'):
            self.update()
        self.assertFalse(self.events)

    def test_read_only_startup_check_is_the_only_allowed_active_job(self):
        self.insert_job(self.state, 'check', 'running', kind='check')
        UPDATE.both_idle(allow_checks=True)
        with self.assertRaisesRegex(RuntimeError, 'Aktiver Auftrag'):
            UPDATE.both_idle()
        self.insert_job(self.builder, 'build', 'queued', kind='check')
        with self.assertRaisesRegex(RuntimeError, 'Aktiver Auftrag'):
            UPDATE.both_idle(allow_checks=True)

    def test_missing_or_malformed_database_cannot_be_treated_as_idle(self):
        (self.builder / 'jobs.sqlite3').unlink()
        with self.assertRaises(sqlite3.OperationalError):
            self.update()
        self.assertFalse(self.events)

    def test_bad_python_payload_refuses_before_any_stop(self):
        self.git_payload['main.py'] = b'def incomplete('
        with self.assertRaises(SyntaxError):
            self.update()
        self.assertFalse(self.events)

    def test_new_job_after_controller_stop_aborts_without_replacing_code(self):
        original_run = self.run.side_effect
        def queue_build(args, **kwargs):
            result = original_run(args, **kwargs)
            if args == ['systemctl', 'stop', UPDATE.CONTROLLER]:
                self.insert_job(self.builder, 'unexpected', 'queued')
            return result
        self.run.side_effect = queue_build
        with self.assertRaisesRegex(RuntimeError, 'Aktiver Auftrag'):
            self.update()
        self.replace.assert_not_called()

    def test_ready_failure_rolls_back_all_changed_code_and_new_helper(self):
        original = {name: (self.library / name).read_bytes() for name in self.source_files
                    if (self.library / name).exists()}
        protected = UPDATE.snapshot(self.protected)
        self.wait_ready.side_effect = [RuntimeError('new runtime failed'), None]
        with self.assertRaisesRegex(RuntimeError, 'new runtime failed'):
            self.update()
        for name, body in original.items():
            self.assertEqual((self.library / name).read_bytes(), body)
        self.assertFalse((self.library / 'image/new/sub/module.py').exists())
        self.assertEqual(UPDATE.snapshot(self.protected), protected)
        self.assertEqual(self.wait_ready.call_count, 2)

    def test_failure_after_rename_is_included_in_rollback(self):
        original_replace = self.replace.side_effect
        failed = False
        def fail_after_publish(path, body, metadata=None):
            nonlocal failed
            result = original_replace(path, body, metadata)
            if not failed:
                failed = True
                raise OSError('fsync failed after rename')
            return result
        self.replace.side_effect = fail_after_publish
        before = (self.library / 'catalog.json').read_bytes()
        with self.assertRaisesRegex(OSError, 'fsync failed'):
            self.update()
        self.assertEqual((self.library / 'catalog.json').read_bytes(), before)

    def test_no_new_invocation_is_not_a_successful_vm_update(self):
        UPDATE.properties.side_effect = lambda unit: dict(self.units[unit], InvocationID='0' * 32)
        with self.assertRaisesRegex(RuntimeError, 'nicht neu gestartet'):
            self.update()

    def test_protected_site_change_after_restart_refuses_success(self):
        def alter_site():
            self.config.write_text('externally changed\n', encoding='utf-8')
        self.wait_ready.side_effect = alter_site
        with self.assertRaisesRegex(RuntimeError, 'Lokale Konfiguration'):
            self.update()

    def test_untracked_payload_and_live_checkout_edits_cannot_change_pinned_bytes(self):
        (self.source / 'untracked.py').write_text('INJECTED = True\n', encoding='utf-8')
        (self.source / 'main.py').write_text('INJECTED = True\n', encoding='utf-8')
        result = self.update()
        self.assertEqual(result['phase'], 'passed')
        self.assertEqual((self.library / 'main.py').read_bytes(), self.git_payload['main.py'])
        self.assertFalse((self.library / 'untracked.py').exists())

    def test_tampered_git_object_refuses_before_service_stop(self):
        original_run = self.run.side_effect
        def corrupted_blob(args, **kwargs):
            result = original_run(args, **kwargs)
            if 'cat-file' in args:
                result.stdout += b'changed'
            return result
        self.run.side_effect = corrupted_blob
        with self.assertRaisesRegex(RuntimeError, 'Fingerprint'):
            self.update()
        self.assertFalse(self.events)

    def test_effective_unit_change_is_not_hidden_by_unchanged_unit_bytes(self):
        def alter_effective_unit():
            self.units[UPDATE.CONTROLLER]['ExecStart'] += ' --new-override'
        self.wait_ready.side_effect = alter_effective_unit
        with self.assertRaisesRegex(RuntimeError, 'Effektive VM-Unit'):
            self.update()

    def test_fake_active_without_main_pid_is_not_a_process_proof(self):
        def no_process():
            self.units[UPDATE.BUILDER]['MainPID'] = '0'
        self.wait_ready.side_effect = no_process
        with self.assertRaisesRegex(RuntimeError, 'MainPID'):
            self.update()

    def test_systemctl_execstart_runtime_pid_and_timestamps_do_not_change_configuration(self):
        before_units = {}
        for unit, current in self.units.items():
            command = current['ExecStart']
            stable = f'path=/usr/bin/python3 ; argv[]={command} ; ignore_errors=no'
            before_units[unit] = dict(current, ExecStart='{ ' + stable + ' ; start_time=old ; pid=10 ; code=0 ; }')
            current['ExecStart'] = '{ ' + stable + ' ; start_time=new ; pid=20 ; code=0 ; }'
        UPDATE.installed_paths.return_value = (self.protected, before_units)
        self.assertEqual(self.update()['phase'], 'passed')

    @unittest.skipIf(os.name == 'nt', 'POSIX permission bits are checked in Linux CI')
    def test_concurrent_mode_change_is_not_overwritten_from_stale_metadata(self):
        target = self.library / 'catalog.json'
        self.ready.side_effect = lambda: target.chmod(0o640)
        with self.assertRaisesRegex(RuntimeError, 'Owner/Mode'):
            self.update()
        self.assertFalse(self.events)
        self.assertEqual(stat.S_IMODE(target.stat().st_mode), 0o640)


if __name__ == '__main__':
    unittest.main()
