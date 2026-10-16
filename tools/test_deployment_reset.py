#!/usr/bin/env python3
"""Real disposable files/SQLite, with simulated service control and mount tables."""
from contextlib import ExitStack, closing
import importlib.util
import json
import os
from pathlib import Path
import sqlite3
import subprocess
import sys
import tempfile
import tomllib
import types
import unittest
from unittest.mock import patch

if os.name != 'posix':
    # Portable file/SQL checks still run; native permission/mount guards run in CI.
    sys.modules.setdefault('fcntl', types.SimpleNamespace())
    sys.modules.setdefault('pwd', types.SimpleNamespace(getpwnam=lambda _: None))
spec = importlib.util.spec_from_file_location('reset', Path(__file__).parent / 'operator/reset-deployment-vm.py')
reset = importlib.util.module_from_spec(spec)
spec.loader.exec_module(reset)


class ResetTest(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.base = Path(self.temp.name).resolve()
        self.stack = ExitStack()
        self.addCleanup(self.stack.close)
        for key, path in dict(STATE='controller', BUILDER='builder', CONFIG='etc/deployment.toml',
                              BACKUPS='backups', PROC='proc', CGROUP='cgroup').items():
            self.stack.enter_context(patch.object(reset, key, self.base / path))
        for root in (reset.STATE, reset.BUILDER, reset.CONFIG.parent, reset.PROC, reset.CGROUP):
            root.mkdir(parents=True)
        reset.CONFIG.write_text('role = "controller"\nstate_dir = ' + json.dumps(str(reset.STATE)) +
            '\nnode_id = "VM-H-DEPLOY-01"\nadvertise_url = "http://10.0.1.131:8320"\n' +
            'seeds = ["http://10.0.1.149:8321"]\nref = "old"\n[bindings]\nfoo = "old"\n', encoding='utf-8')
        self.old_config = reset.CONFIG.read_text(encoding='utf-8')
        self.db = reset.STATE / 'jobs.sqlite3'
        with closing(sqlite3.connect(self.db)) as db:
            db.execute('CREATE TABLE jobs(id TEXT PRIMARY KEY,status TEXT,request TEXT,result TEXT)')
            db.execute('CREATE TABLE update_batches(id TEXT)')
            db.commit()
        for path in ('artifacts/old/image.img.xz', 'cache/generalized.img', 'work/old/image.img', 'requests/old.json'):
            target = reset.BUILDER / path
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text('private-build-data', encoding='utf-8')
        for name in ('profiles.json', 'tbs-site-template.toml', 'settings.json', 'discovery-state.json'):
            (reset.STATE / name).write_text('old-' + name, encoding='utf-8')
        (reset.STATE / 'source').mkdir()
        (reset.STATE / 'source/old-file').write_text('reproducible', encoding='utf-8')
        self.calls = []
        self.stopped = False
        self.props = self.stack.enter_context(patch.object(reset, 'properties', side_effect=self.properties))
        self.runner = self.stack.enter_context(patch.object(reset, 'run', side_effect=self.fake_run))
        self.stack.enter_context(patch.object(reset.pwd, 'getpwnam', return_value=types.SimpleNamespace(
            pw_uid=os.getuid() if os.name == 'posix' else 0,
            pw_gid=os.getgid() if os.name == 'posix' else 0)))
        if os.name != 'posix':
            self.stack.enter_context(patch.object(reset.os, 'chown', create=True))
        self.stack.enter_context(patch.object(reset, 'verify', return_value='http://127.0.0.1:8320'))

    def properties(self, unit):
        script = 'main.py --config /etc/netcore/deployment.toml' if unit == reset.UNITS[0] else 'image_worker.py'
        return dict(LoadState='loaded', ActiveState='inactive' if self.stopped else 'active',
                    MainPID='0' if self.stopped else '42', ControlGroup='', KillMode='control-group',
                    ExecStart='/usr/bin/python3 /usr/local/lib/netcore-deployment/' + script)

    def fake_run(self, args, **kwargs):
        args = list(map(str, args))
        self.calls.append(args)
        if args[:2] == ['systemctl', 'stop']:
            self.stopped = True
        if args[:2] == ['losetup', '--json']:
            return '{"loopdevices": []}'
        return ''

    def job(self, key='job', status='failed', request=None, result=None):
        with closing(sqlite3.connect(self.db)) as db:
            db.execute('INSERT INTO jobs VALUES (?,?,?,?)', (key, status,
                       json.dumps(request or {'service': 'media-library', 'node_id': 'Media-Library'}),
                       json.dumps(result or {})))
            db.commit()

    def mount(self, target):
        directory = reset.PROC / '314'
        directory.mkdir(exist_ok=True)
        target = str(target).replace(' ', '\\040')
        (directory / 'mountinfo').write_text('30 29 7:2 / ' + target + ' rw - ext4 /dev/loop0p2 rw\n', encoding='utf-8')

    def test_preview_changes_no_application_files_or_services(self):
        result = reset.reset()
        self.assertEqual(result['phase'], 'preview')
        self.assertEqual(self.calls, [])
        self.assertEqual(reset.CONFIG.read_text(encoding='utf-8'), self.old_config)
        self.assertTrue((reset.BUILDER / 'artifacts/old/image.img.xz').exists())

    def test_full_reset_preserves_private_receipts_and_removes_images(self):
        self.job()
        result = reset.reset(True)
        self.assertEqual(result['phase'], 'completed')
        backup = Path(result['backup'])
        self.assertEqual(list(reset.STATE.iterdir()), [])
        self.assertEqual(list(reset.BUILDER.iterdir()), [])
        self.assertTrue((backup / 'controller/jobs.sqlite3').exists())
        self.assertTrue((backup / 'controller/profiles.json').exists())
        self.assertTrue((backup / 'builder/requests/old.json').exists())
        self.assertFalse((backup / 'builder/artifacts').exists())
        self.assertFalse((backup / 'builder/cache').exists())
        self.assertFalse((backup / 'builder/work').exists())
        self.assertFalse((backup / 'controller/source').exists())
        cfg = tomllib.loads(reset.CONFIG.read_text(encoding='utf-8'))
        self.assertEqual(cfg['node_id'], 'VM-H-DEPLOY-01')
        self.assertEqual(cfg['advertise_url'], 'http://10.0.1.131:8320')
        self.assertEqual((cfg['seeds'], cfg['bindings'], cfg['ref']), ([], {}, 'main'))
        self.assertEqual((backup / 'deployment.toml').read_text(encoding='utf-8'), self.old_config)
        self.assertEqual(self.calls[0], ['systemctl', 'stop', *reset.UNITS])
        self.assertEqual(self.calls[-2:], [['systemctl', 'start', reset.UNITS[1]], ['systemctl', 'start', reset.UNITS[0]]])
        if os.name == 'posix':
            self.assertEqual(backup.stat().st_mode & 0o777, 0o700)
            self.assertEqual((backup / 'deployment.toml').stat().st_mode & 0o777, 0o600)

    def test_uncertain_terminal_receipt_is_not_erased(self):
        self.job(result={'remote_uncertain': True, 'remote_job': 'unresolved'})
        with self.assertRaisesRegex(RuntimeError, 'LXC-Auftraege'):
            reset.reset(True)
        self.assertEqual(self.calls, [])
        self.assertTrue(self.db.exists())

    def test_running_remote_even_before_post_blocks_reset(self):
        self.job(status='running')
        with self.assertRaisesRegex(RuntimeError, 'LXC-Auftraege'):
            reset.reset(True)
        self.assertEqual(self.calls, [])

    def test_local_check_and_undispatched_queue_can_be_cancelled(self):
        self.job('check', 'running', {'kind': 'check'})
        self.job('queued', 'queued', {'kind': 'bulk_service', 'node_id': 'other'})
        self.assertEqual(reset.reset(True)['phase'], 'completed')

    def test_malformed_remote_flag_fails_closed(self):
        self.job(result={'remote_uncertain': 'false'})
        with self.assertRaisesRegex(RuntimeError, 'Remote-Status'):
            reset.reset(True)
        self.assertEqual(self.calls, [])

    def test_remote_dispatch_race_after_initial_check_preserves_history(self):
        original = self.fake_run
        def run(args, **kw):
            if list(args)[:2] == ['systemctl', 'stop']:
                self.job(result={'remote_uncertain': True, 'remote_job': 'raced'})
            return original(args, **kw)
        self.runner.side_effect = run
        with self.assertRaisesRegex(RuntimeError, 'LXC-Auftraege'):
            reset.reset(True)
        self.assertTrue(self.db.exists())
        self.assertTrue((reset.BUILDER / 'cache/generalized.img').exists())
        self.assertFalse(reset.BACKUPS.exists())

    def test_deferred_loop_detach_blocks_file_removal(self):
        row = dict(name='/dev/loop9', **{'back-file': str(reset.BUILDER / 'work' / ('a' * 32) / 'image.img')})
        original = self.fake_run
        self.runner.side_effect = lambda args, **kw: json.dumps({'loopdevices': [row]}) if list(args)[:2] == ['losetup', '--json'] else original(args, **kw)
        with patch.object(reset.time, 'monotonic', side_effect=[0, 11]):
            with self.assertRaisesRegex(RuntimeError, 'Loopgeraet ist noch belegt'):
                reset.reset(True)
        self.assertTrue(self.db.exists())
        self.assertTrue((reset.BUILDER / 'cache/generalized.img').exists())
        self.assertFalse(reset.BACKUPS.exists())

    def test_delayed_loop_detach_finishes_before_backing_files_move(self):
        own = dict(name='/dev/loop9', **{
            'back-file': str(reset.BUILDER / 'work' / ('a' * 32) / 'image.img')})
        foreign = dict(name='/dev/loop10', **{'back-file': str(self.base / 'unrelated.img')})
        snapshots = iter(([own, foreign], [own, foreign], [foreign]))
        original = self.fake_run
        def run(args, **kw):
            if list(args)[:2] == ['losetup', '--json']:
                # All observations occur before backing files may be moved.
                self.assertTrue(self.db.exists())
                self.assertTrue((reset.BUILDER / 'cache/generalized.img').exists())
                return json.dumps({'loopdevices': next(snapshots)})
            return original(args, **kw)
        self.runner.side_effect = run
        with patch.object(reset.time, 'sleep') as sleeping:
            result = reset.reset(True)
        self.assertEqual(result['phase'], 'completed')
        self.assertTrue(sleeping.called)
        detaches = [call for call in self.calls if call[:2] == ['losetup', '--detach']]
        self.assertEqual(detaches, [['losetup', '--detach', '/dev/loop9']])
        self.assertTrue((Path(result['backup']) / 'controller/jobs.sqlite3').exists())

    def test_private_mount_prevents_renames_and_deletion(self):
        self.mount(reset.BUILDER / 'work/old/root/boot/firmware')
        with self.assertRaisesRegex(RuntimeError, 'Mount noch aktiv'):
            reset.reset(True)
        self.assertTrue(self.db.exists())
        self.assertTrue((reset.BUILDER / 'cache/generalized.img').exists())
        self.assertFalse(reset.BACKUPS.exists())

    def test_mount_root_itself_also_blocks(self):
        self.mount(reset.STATE)
        with self.assertRaisesRegex(RuntimeError, 'Mount noch aktiv'):
            reset.reset(True)
        self.assertTrue(self.db.exists())

    @unittest.skipUnless(os.name == 'posix', 'native path/symlink guard')
    def test_state_symlink_is_rejected_without_touching_target(self):
        real = self.base / 'real-state'
        reset.STATE.rename(real)
        reset.STATE.symlink_to(real, target_is_directory=True)
        with self.assertRaisesRegex(RuntimeError, 'Symlink'):
            reset.reset(True)
        self.assertTrue((real / 'jobs.sqlite3').exists())
        self.assertEqual(self.calls, [])

    @unittest.skipUnless(os.name == 'posix', 'native path/symlink guard')
    def test_pruning_never_follows_external_cache_symlink(self):
        shutil = __import__('shutil')
        shutil.rmtree(reset.BUILDER / 'cache')
        outside = self.base / 'outside'
        outside.mkdir()
        sentinel = outside / 'keep'
        sentinel.write_text('keep', encoding='utf-8')
        (reset.BUILDER / 'cache').symlink_to(outside, target_is_directory=True)
        self.assertEqual(reset.reset(True)['phase'], 'completed')
        self.assertEqual(sentinel.read_text(encoding='utf-8'), 'keep')

    def test_known_loop_selection_never_detaches_unrelated_devices(self):
        rows = [dict(name='/dev/loop1', **{'back-file': '/snap/core/file'}),
                dict(name='/dev/loop3', **{'back-file': str(reset.BUILDER / 'work' / ('a' * 32) / 'image.img')})]
        self.assertEqual(reset.owned_loops(rows), ['/dev/loop3'])
        rows.append(dict(name='/dev/loop4', **{'back-file': str(reset.BUILDER / 'cache/unexpected.img')}))
        with self.assertRaisesRegex(RuntimeError, 'Unerwartetes Loopgeraet'):
            reset.owned_loops(rows)

    def test_lingering_cgroup_process_blocks_file_changes(self):
        file = reset.CGROUP / 'system.slice/fixture/cgroup.procs'
        file.parent.mkdir(parents=True)
        file.write_text('987\n', encoding='utf-8')
        original = self.properties
        def props(unit):
            return dict(original(unit), ControlGroup='/system.slice/fixture')
        self.props.side_effect = props
        with self.assertRaisesRegex(RuntimeError, 'prozesse noch aktiv'):
            reset.reset(True)
        self.assertTrue(self.db.exists())
        self.assertFalse(reset.BACKUPS.exists())

    def test_failure_after_first_state_move_restores_original(self):
        original = Path.rename
        def rename(source, target):
            if source == reset.BUILDER:
                raise OSError('simulated rename failure')
            return original(source, target)
        with patch.object(Path, 'rename', rename):
            with self.assertRaisesRegex(OSError, 'rename failure'):
                reset.reset(True)
        self.assertTrue(self.db.exists())
        self.assertTrue((reset.STATE / 'profiles.json').exists())
        self.assertTrue((reset.BUILDER / 'cache/generalized.img').exists())
        self.assertEqual(reset.CONFIG.read_text(encoding='utf-8'), self.old_config)

    def test_changed_custom_unit_is_rejected_before_stop(self):
        original = self.properties
        self.props.side_effect = lambda unit: dict(original(unit), KillMode='process')
        with self.assertRaisesRegex(RuntimeError, 'Abweichende Unit'):
            reset.reset(True)
        self.assertEqual(self.calls, [])

    def test_escaped_mount_name_is_decoded(self):
        self.mount(reset.BUILDER / 'work/a root')
        with self.assertRaisesRegex(RuntimeError, 'a root'):
            reset.assert_no_mounts((reset.BUILDER,))

    @unittest.skipUnless(os.name == 'posix' and os.geteuid() == 0, 'Linux root mount/loop integration')
    def test_actual_loop_detach_preserves_unrelated_loop(self):
        owned = reset.BUILDER / 'work' / ('b' * 32) / 'image.img'
        outside = self.base / 'unrelated.img'
        owned.parent.mkdir(parents=True)
        for path in (owned, outside):
            with path.open('wb') as file:
                file.truncate(1024 * 1024)
        attached = []
        def native(args):
            return subprocess.check_output(list(map(str, args)), text=True, timeout=20).strip()
        try:
            for path in (owned, outside):
                attached.append(native(['losetup', '--find', '--show', path]))
            original = self.fake_run
            self.runner.side_effect = lambda args, **kw: native(args) if str(args[0]) == 'losetup' else original(args, **kw)
            self.assertEqual(reset.reset(True)['phase'], 'completed')
            rows = json.loads(native(['losetup', '--json', '--list', '--output', 'NAME,BACK-FILE']))['loopdevices']
            names = {row['name'] for row in rows}
            self.assertNotIn(attached[0], names)
            self.assertIn(attached[1], names)
        finally:
            for device in attached:
                subprocess.run(['losetup', '--detach', device], check=False, capture_output=True)

    @unittest.skipUnless(os.name == 'posix' and os.geteuid() == 0, 'Linux root private namespace integration')
    def test_actual_private_namespace_mount_is_detected(self):
        import signal
        import time
        target = reset.BUILDER / 'work/private-root'
        target.mkdir(parents=True)
        ready = self.base / 'mounted'
        script = 'import pathlib,subprocess,sys,time;subprocess.run(["mount","-t","tmpfs","tmpfs",sys.argv[1]],check=True);pathlib.Path(sys.argv[2]).touch();time.sleep(60)'
        process = subprocess.Popen(['unshare', '--mount', '--fork', '--kill-child', '--propagation', 'private',
                                    sys.executable, '-c', script, str(target), str(ready)], start_new_session=True)
        try:
            deadline = time.monotonic() + 10
            while not ready.exists() and process.poll() is None and time.monotonic() < deadline:
                time.sleep(0.05)
            self.assertTrue(ready.exists(), 'Disposable namespace/mount must start on native CI')
            with patch.object(reset, 'PROC', Path('/proc')):
                with self.assertRaisesRegex(RuntimeError, 'Mount noch aktiv'):
                    reset.assert_no_mounts((reset.BUILDER,))
            self.assertTrue((reset.BUILDER / 'cache/generalized.img').exists())
        finally:
            if process.poll() is None:
                os.killpg(process.pid, signal.SIGKILL)
            process.wait(timeout=10)


if __name__ == '__main__':
    unittest.main(verbosity=2)
