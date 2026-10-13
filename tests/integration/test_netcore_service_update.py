"""Preparation, file transactions, receipts and process-proof regression tests.

Systemd commands are mocked here; real systemd/HTTP smoke tests run separately.
These tests use real files, immutable snapshots, SQLite reads and atomic swaps.
"""
from contextlib import ExitStack, closing, nullcontext, redirect_stdout
import importlib.util
import io
import json
import os
from pathlib import Path
import sqlite3
import stat
import subprocess
import tempfile
import unittest
from unittest.mock import MagicMock, patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location('netcore_update', ROOT / 'tools/operator/update-netcore-service.py')
update = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(update)
COMMIT = 'a' * 40


class Fixture(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='netcore-update-')
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.target = self.root / 'installed.py'
        self.target.write_bytes(b'print("old runtime")\n')
        self.target.chmod(0o755)
        self.source = self.root / 'new-runtime.py'
        self.source.write_bytes(b'print("new runtime")\n')
        self.config = self.root / 'config.toml'
        self.config.write_bytes(b'# preserve this exact comment\nbind="127.0.0.1:8160"\n')
        self.unit = self.root / 'netcore-test.service'
        self.unit.write_bytes(b'[Service]\n# local changes are retained\n')
        self.spec = dict(name='hardware-gateway', unit='netcore-test.service',
                         command=repr(self.target.as_posix()), config_target=self.config.as_posix(),
                         port=8160, health='/health/ready')
        self.props = dict(LoadState='loaded', FragmentPath=self.unit.as_posix(),
                          ActiveState='active', UnitFileState='disabled', MainPID='241',
                          InvocationID='1' * 32,
                          ExecStart='{ argv[]=' + self.target.as_posix() + ' --config '
                          + self.config.as_posix() + ' ; }')
        self.receipts, self.backups = self.root / 'receipts', self.root / 'backups'
        self.agent_state = self.root / 'agent-state'
        self.agent_state.mkdir()
        self.proc = self.root / 'proc'
        self.manifest = self.root / 'tools/operator/service-update-assets.json'
        self.manifest.parent.mkdir(parents=True)
        self.write_assets([(self.source, self.target)])

    def write_assets(self, files):
        self.manifest.write_text(json.dumps({'roles': {self.spec['name']: {'files': [
            {'source': str(source.relative_to(self.root)), 'target': str(target), 'mode': '0644'}
            for source, target in files]}}}), encoding='utf-8')

    def plan(self, files=None):
        self.write_assets(files or [(self.source, self.target)])
        return update.prepare_plan(self.spec, self.target, self.props, COMMIT, root=self.root)

    def apply(self, plan):
        return update.apply_plan(self.spec, plan, self.props, root=self.root,
                                 backup_root=self.backups, receipt_root=self.receipts)

    def mocks(self, state='active', agent=True):
        stack = ExitStack()
        self.live_mock = stack.enter_context(patch.object(update, 'wait_live'))
        self.http_mock = stack.enter_context(patch.object(update, 'check_http', return_value=True))
        stack.enter_context(patch.object(update, 'agent_guard', return_value=nullcontext(
            self.agent_state if agent else None)))
        stack.enter_context(patch.object(update, 'PROC_ROOT', self.proc))
        runtime_state = dict(self.props, ActiveState=state)
        counter = 242

        def command(args, **kwargs):
            nonlocal counter
            self.assertEqual('systemctl', args[0])
            if args[1] == 'stop':
                runtime_state['ActiveState'] = 'inactive'
            elif args[1] == 'start':
                runtime_state.update(ActiveState='active', MainPID=str(counter),
                                     InvocationID=('%032x' % counter))
                folder = self.proc / str(counter)
                folder.mkdir(parents=True)
                folder.joinpath('cmdline').write_bytes(b'/usr/bin/python3\0' + os.fsencode(self.target) + b'\0')
                folder.joinpath('exe').write_bytes(self.target.read_bytes())
                counter += 1
            elif args[1] != 'reset-failed':
                raise AssertionError('Unexpected service operation: ' + repr(args))
            return subprocess.CompletedProcess(args, 0)

        self.run_mock = stack.enter_context(patch.object(update, 'run', side_effect=command))
        self.properties_mock = stack.enter_context(patch.object(update, 'properties',
                                      side_effect=lambda unit: dict(runtime_state)))
        return stack


class SnapshotTests(Fixture):
    def test_snapshot_path_cannot_escape_prepared_package(self):
        plan = self.plan()
        plan['files'][0]['snapshot'] = str(self.source.relative_to(self.root))
        with self.assertRaisesRegex(RuntimeError, 'außerhalb'):
            update.validate_plan(self.spec, plan, COMMIT, self.root)

    def test_snapshots_are_immutable_inputs_even_if_original_source_changes(self):
        plan = self.plan()
        expected = self.source.read_bytes()
        self.source.write_bytes(b'CHANGED AFTER PREPARATION\n')
        with self.mocks():
            receipt = self.apply(plan)
        self.assertEqual(expected, self.target.read_bytes())
        self.assertEqual(COMMIT, receipt['source_commit'])
        self.assertEqual(plan['files'][0]['sha256'], receipt['files'][0]['sha256'])

    def test_tampered_snapshot_is_rejected_before_stop(self):
        plan = self.plan()
        snapshot = self.root / plan['files'][0]['snapshot']
        snapshot.chmod(0o600)
        snapshot.write_bytes(b'bad snapshot\n')
        with self.mocks():
            with self.assertRaisesRegex(RuntimeError, 'Snapshot'):
                self.apply(plan)
        self.run_mock.assert_not_called()

    def test_changed_destination_since_prepare_is_rejected_before_stop(self):
        plan = self.plan()
        self.target.write_bytes(b'operator change\n')
        with self.mocks():
            with self.assertRaisesRegex(RuntimeError, 'Vorbereitung'):
                self.apply(plan)
        self.run_mock.assert_not_called()

    def test_changed_config_since_prepare_is_rejected_before_stop(self):
        plan = self.plan()
        self.config.write_bytes(b'bind="127.0.0.1:9999"\n')
        with self.mocks():
            with self.assertRaisesRegex(RuntimeError, 'Konfiguration'):
                self.apply(plan)
        self.run_mock.assert_not_called()

    def test_prepared_plan_rejects_other_commit(self):
        plan = self.plan()
        with self.assertRaisesRegex(RuntimeError, 'Quellcommit'):
            update.validate_plan(self.spec, plan, 'b' * 40, self.root)

    def test_prepare_failure_never_stops_service(self):
        self.source.write_text('def invalid(:\n', encoding='utf-8')
        with patch.object(update, 'run') as command:
            with self.assertRaises(SyntaxError):
                self.plan()
        command.assert_not_called()

    def test_plan_primary_runtime_missing_is_rejected(self):
        secondary = self.root / 'module.py'
        self.write_assets([(self.source, secondary)])
        with self.assertRaisesRegex(RuntimeError, 'Primäre Runtime'):
            update.prepare_plan(self.spec, self.target, self.props, COMMIT, self.root)


class FileTransactionTests(Fixture):
    def test_complete_update_preserves_config_unit_mode_and_disabled_state(self):
        before = self.config.read_bytes(), self.unit.read_bytes()
        plan = self.plan()
        with self.mocks():
            receipt = self.apply(plan)
        self.assertEqual('updated', receipt['phase'])
        self.assertEqual(before, (self.config.read_bytes(), self.unit.read_bytes()))
        self.assertEqual(self.source.read_bytes(), self.target.read_bytes())
        self.assertEqual(COMMIT, json.loads((self.agent_state / 'deployed-hardware-gateway.json').read_text())['commit'])
        self.assertEqual(COMMIT, json.loads((self.receipts / 'hardware-gateway.json').read_text())['source_commit'])
        self.assertTrue(receipt['verified'])
        self.assertTrue(receipt['assets_complete'])
        self.assertNotEqual(self.props['InvocationID'], receipt['process']['InvocationID'])
        operations = [call.args[0][1] for call in self.run_mock.call_args_list]
        self.assertEqual(['stop', 'start'], operations)

    def test_identical_disk_bytes_still_restart_active_process(self):
        self.source.write_bytes(self.target.read_bytes())
        plan = self.plan()
        with self.mocks():
            receipt = self.apply(plan)
        self.assertEqual('updated', receipt['phase'])
        self.assertEqual(['stop', 'start'], [call.args[0][1] for call in self.run_mock.call_args_list])
        self.assertNotEqual(self.props['InvocationID'], receipt['process']['InvocationID'])

    def test_inactive_service_is_installed_without_start_or_false_live_claim(self):
        plan = self.plan()
        with self.mocks(state='inactive'):
            receipt = self.apply(plan)
        self.run_mock.assert_not_called()
        self.http_mock.assert_not_called()
        self.assertEqual('inactive_updated', receipt['phase'])
        self.assertFalse(receipt['process']['active'])
        self.assertIsNone(receipt['readiness'])
        self.assertTrue(receipt['verified'])

    def test_failed_service_recovers_without_enabling_unit(self):
        plan = self.plan()
        with self.mocks(state='failed'):
            receipt = self.apply(plan)
        self.assertEqual(['reset-failed', 'start'], [call.args[0][1] for call in self.run_mock.call_args_list])
        self.assertEqual('failed', receipt['active_before'])
        self.assertTrue(receipt['liveness'])

    def test_new_owned_files_and_nested_directories_are_installed(self):
        module = self.root / 'newmodule.py'
        module.write_bytes(b'VERSION="new"\n')
        destination = self.root / 'package' / 'sub' / 'module.py'
        plan = self.plan([(self.source, self.target), (module, destination)])
        self.assertTrue(plan['files'][1]['created'])
        with self.mocks():
            receipt = self.apply(plan)
        self.assertEqual(module.read_bytes(), destination.read_bytes())
        self.assertTrue(receipt['files'][1]['created'])

    def test_liveness_failure_restores_old_files_removes_new_files_and_keeps_commit_unknown(self):
        old = self.target.read_bytes()
        module = self.root / 'newmodule.py'
        module.write_bytes(b'VALUE=1\n')
        destination = self.root / 'package' / 'sub' / 'module.py'
        plan = self.plan([(self.source, self.target), (module, destination)])
        with self.mocks(), redirect_stdout(io.StringIO()):
            self.live_mock.side_effect = RuntimeError('new listener failed')
            with self.assertRaisesRegex(RuntimeError, 'listener'):
                self.apply(plan)
        self.assertEqual(old, self.target.read_bytes())
        self.assertFalse(destination.exists())
        self.assertFalse(destination.parent.exists())
        marker = json.loads((self.agent_state / 'deployed-hardware-gateway.json').read_text())
        self.assertEqual('', marker['commit'])
        self.assertEqual('rolled_back', marker['state'])
        self.assertFalse(json.loads((self.receipts / 'hardware-gateway.json').read_text())['verified'])

    def test_partial_replacement_failure_restores_whole_owned_set(self):
        old = self.target.read_bytes()
        module = self.root / 'module.py'
        module.write_bytes(b'NEW=1\n')
        destination = self.root / 'existing-module.py'
        destination.write_bytes(b'OLD=1\n')
        original = destination.read_bytes()
        plan = self.plan([(self.source, self.target), (module, destination)])
        native_replace = update.replace_file
        count = 0

        def fail_second(source, dest, *args):
            nonlocal count
            count += 1
            if count == 2:
                raise OSError('write failed')
            return native_replace(source, dest, *args)

        with self.mocks(), patch.object(update, 'replace_file', side_effect=fail_second), redirect_stdout(io.StringIO()):
            with self.assertRaises(OSError):
                self.apply(plan)
        self.assertEqual(old, self.target.read_bytes())
        self.assertEqual(original, destination.read_bytes())

    def test_readiness_degraded_keeps_verified_new_process(self):
        plan = self.plan()
        with self.mocks():
            self.http_mock.return_value = False
            receipt = self.apply(plan)
        self.assertEqual('updated_degraded', receipt['phase'])
        self.assertFalse(receipt['readiness'])
        self.assertTrue(receipt['liveness'])
        self.assertTrue(receipt['verified'])
        self.assertEqual(COMMIT, receipt['source_commit'])

    def test_transitional_state_has_no_runtime_mutation(self):
        old = self.target.read_bytes()
        plan = self.plan()
        with self.mocks(state='deactivating'):
            with self.assertRaisesRegex(RuntimeError, 'Dienststatus'):
                self.apply(plan)
        self.run_mock.assert_not_called()
        self.assertEqual(old, self.target.read_bytes())

    @unittest.skipUnless(hasattr(os, 'fchown'), 'Native POSIX ownership test')
    def test_atomic_swap_preserves_posix_owner_and_mode(self):
        self.target.chmod(0o751)
        before = self.target.stat()
        update.replace_file(self.source, self.target)
        after = self.target.stat()
        self.assertEqual((before.st_uid, before.st_gid, stat.S_IMODE(before.st_mode)),
                         (after.st_uid, after.st_gid, stat.S_IMODE(after.st_mode)))


class ProcessEvidenceTests(Fixture):
    def process_fixture(self, plan, invocation='2' * 32):
        directory = self.proc / '242'
        directory.mkdir(parents=True)
        directory.joinpath('cmdline').write_bytes(b'/usr/bin/python3\0' + os.fsencode(self.target) + b'\0')
        directory.joinpath('exe').write_bytes(self.target.read_bytes())
        props = dict(self.props, MainPID='242', InvocationID=invocation)
        return directory, props

    def test_same_invocation_is_not_accepted_as_new_process(self):
        self.source.write_bytes(self.target.read_bytes())
        plan = self.plan()
        _, props = self.process_fixture(plan, self.props['InvocationID'])
        with patch.object(update, 'PROC_ROOT', self.proc), patch.object(update, 'properties', return_value=props):
            with self.assertRaisesRegex(RuntimeError, 'Alter Dienstprozess'):
                update.verify_process(self.spec, plan, self.props)

    def test_python_command_must_contain_exact_primary_script_token(self):
        self.source.write_bytes(self.target.read_bytes())
        plan = self.plan()
        directory, props = self.process_fixture(plan)
        directory.joinpath('cmdline').write_bytes(b'/usr/bin/python3\0' + os.fsencode(self.target) + b'.wrong\0')
        with patch.object(update, 'PROC_ROOT', self.proc), patch.object(update, 'properties', return_value=props):
            with self.assertRaisesRegex(RuntimeError, 'Hauptskript'):
                update.verify_process(self.spec, plan, self.props)

    def test_elf_process_hash_must_equal_prepared_artifact(self):
        self.target.write_bytes(b'\x7fELF old')
        self.source = self.root / 'runtime-elf'
        self.source.write_bytes(self.target.read_bytes())
        plan = self.plan()
        # This direct fixture is not a Rust build; mark its proof kind explicitly.
        plan['runtime_kind'] = 'elf'
        directory, props = self.process_fixture(plan)
        directory.joinpath('exe').write_bytes(b'\x7fELF OTHER BINARY')
        with patch.object(update, 'PROC_ROOT', self.proc), patch.object(update, 'properties', return_value=props):
            with self.assertRaisesRegex(RuntimeError, 'Laufendes ELF'):
                update.verify_process(self.spec, plan, self.props)

    def test_process_change_during_proof_is_rejected(self):
        self.source.write_bytes(self.target.read_bytes())
        plan = self.plan()
        _, props = self.process_fixture(plan)
        changed = dict(props, MainPID='999')
        with patch.object(update, 'PROC_ROOT', self.proc), patch.object(update, 'properties', side_effect=[props, changed]):
            with self.assertRaisesRegex(RuntimeError, 'wechselte'):
                update.verify_process(self.spec, plan, self.props)

    def test_owned_bytecode_only_is_removed(self):
        plan = self.plan()
        cache = self.target.parent / '__pycache__'
        cache.mkdir()
        owned = cache / (self.target.stem + '.cpython-311.pyc')
        unrelated = cache / 'operator-helper.cpython-311.pyc'
        owned.write_bytes(b'old cache')
        unrelated.write_bytes(b'operator cache')
        update.clear_owned_bytecode(plan)
        self.assertFalse(owned.exists())
        self.assertEqual(b'operator cache', unrelated.read_bytes())


class AgentStateTests(Fixture):
    def test_missing_agent_state_key_uses_documented_default(self):
        config = self.root / 'discovery.toml'
        config.write_text('role="agent"\n', encoding='utf-8')
        with patch.object(update, 'DISCOVERY_CONFIG', config):
            self.assertEqual(Path('/var/lib/netcore-discovery'), update.state_path(self.spec))

    def test_absent_transport_identity_is_rejected(self):
        with self.assertRaisesRegex(RuntimeError, 'Transportnachweis'):
            update.verify_source_reference(self.root, COMMIT)

    def test_matching_archive_identity_is_accepted(self):
        (self.root / '.netcore-source.json').write_text(json.dumps({
            'commit': COMMIT, 'archive_sha256': 'b' * 64}), encoding='utf-8')
        update.verify_source_reference(self.root, COMMIT)

    def test_other_archive_source_sha_is_rejected(self):
        (self.root / '.netcore-source.json').write_text(json.dumps({
            'commit': 'c' * 40, 'archive_sha256': 'b' * 64}), encoding='utf-8')
        with self.assertRaisesRegex(RuntimeError, 'Transportnachweis'):
            update.verify_source_reference(self.root, COMMIT)

    def test_state_directory_is_resolved_from_existing_agent_config(self):
        config = self.root / 'discovery.toml'
        config.write_text('state_dir=' + json.dumps(self.agent_state.as_posix()) + '\n', encoding='utf-8')
        with patch.object(update, 'DISCOVERY_CONFIG', config):
            self.assertEqual(self.agent_state, update.state_path(self.spec))

    def test_busy_job_database_is_read_only_and_detects_queue_and_running(self):
        db = self.agent_state / 'jobs.sqlite3'
        with closing(sqlite3.connect(db)) as connection:
            connection.execute('CREATE TABLE jobs (id TEXT, status TEXT)')
            connection.executemany('INSERT INTO jobs VALUES (?,?)', [('done', 'succeeded'),
                                     ('queued-job', 'queued'), ('running-job', 'running')])
            connection.commit()
        before = db.read_bytes()
        self.assertEqual([('queued-job', 'queued'), ('running-job', 'running')], update.busy_jobs(self.agent_state))
        self.assertEqual(before, db.read_bytes())

    def test_unknown_jobs_schema_is_rejected_instead_of_assumed_idle(self):
        db = self.agent_state / 'jobs.sqlite3'
        with closing(sqlite3.connect(db)) as connection:
            connection.execute('CREATE TABLE old_jobs (name TEXT)')
            connection.commit()
        with self.assertRaises(sqlite3.DatabaseError):
            update.busy_jobs(self.agent_state)

    def test_marker_does_not_publish_unverified_requested_commit(self):
        invalid = dict(source_commit=COMMIT, requested_commit=COMMIT, verified=False, phase='installing')
        update.receipt_write(self.spec, invalid, self.receipts, self.agent_state)
        marker = json.loads((self.agent_state / 'deployed-hardware-gateway.json').read_text())
        self.assertEqual('', marker['commit'])
        self.assertEqual(COMMIT, marker['requested_commit'])


class CompleteAssetsTests(Fixture):
    def test_manifest_cannot_write_unit_or_config_even_within_package_directory(self):
        self.write_assets([(self.source, self.target), (self.source, self.root / 'config.toml')])
        with self.assertRaisesRegex(RuntimeError, 'Konfiguration/Unit'):
            update.runtime_assets(self.spec, self.target, self.root)

    def test_new_file_outside_proven_package_is_rejected(self):
        outside = self.root.parent / 'other-service-runtime.py'
        self.write_assets([(self.source, self.target), (self.source, outside)])
        with self.assertRaisesRegex(RuntimeError, 'außerhalb'):
            update.runtime_assets(self.spec, self.target, self.root)

    def test_discovery_includes_absent_modules_static_and_install_helpers(self):
        self.spec['name'] = 'discovery'
        self.spec['config_target'] = self.config.as_posix()
        base = self.root / 'system-backend/deployment-core'
        (base / 'static' / 'nested').mkdir(parents=True)
        (base / 'install').mkdir()
        for name in ('main.py', 'newmodule.py'):
            (base / name).write_text('VALUE=1\n', encoding='utf-8')
        (base / 'catalog.json').write_text('[]', encoding='utf-8')
        (base / 'static' / 'nested' / 'new.js').write_text('let value=1;', encoding='utf-8')
        for name in ('prepare-host.sh', 'install-tbs.sh'):
            (base / 'install' / name).write_text('#!/bin/bash\ntrue\n', encoding='utf-8')
        self.target = self.root / 'installed-agent' / 'main.py'
        self.target.parent.mkdir()
        self.target.write_text('VALUE=0\n', encoding='utf-8')
        plan = update.prepare_plan(self.spec, self.target, self.props, COMMIT, self.root)
        names = {Path(a['target']).name for a in plan['files']}
        self.assertEqual({'main.py', 'newmodule.py', 'catalog.json', 'new.js',
                          'prepare-host.sh', 'install-tbs.sh'}, names)
        self.assertEqual(5, sum(a['created'] for a in plan['files']))

    def test_directory_legacy_name_uses_matching_source_without_seed_import(self):
        self.spec['name'] = 'directory'
        source = self.root / 'misc/ID-Server/netcore_directory_server.py'
        source.parent.mkdir(parents=True)
        source.write_text('LEGACY=True\n', encoding='utf-8')
        self.target = self.root / 'netcore_directory_server.py'
        self.target.write_text('LEGACY=False\n', encoding='utf-8')
        self.write_assets([(self.source, self.target)])
        files = update.runtime_assets(self.spec, self.target, self.root)
        self.assertEqual([(source, self.target)], files)

    def test_all_catalog_roles_have_runtime_route(self):
        catalog = update.specs(ROOT)
        manifest = json.loads((ROOT / 'tools/operator/service-update-assets.json').read_text())
        for role in catalog:
            with self.subTest(role=role):
                self.assertTrue(role in ('discovery', 'directory', 'alert-service')
                     or update.cargo_manifest_path(role, ROOT).is_file()
                     or bool(manifest['roles'].get(role, {}).get('files')))


class BuildTests(Fixture):
    def test_dependency_preparation_is_opt_in_and_root_toolchain_scoped(self):
        header, cache, mapped = self.rust()
        calls = []

        def command(args, **kwargs):
            calls.append((args, kwargs))
            if Path(args[0]).name == 'cargo':
                (cache / 'release' / self.target.name).write_bytes(header + b'new runtime')

        with patch.object(update, 'Path', side_effect=mapped), \
                patch.object(update.shutil, 'which', return_value='/root/.cargo/bin/cargo'), \
                patch.object(update, 'run', side_effect=command):
            update.prepare(self.spec, self.target, self.root, COMMIT, prepare_build_deps=True)
        self.assertEqual('bash', calls[0][0][0])
        self.assertEqual(['--role', 'packet-core'], calls[0][0][-2:])
        env = calls[1][1]['env']
        self.assertEqual('/root/.cargo', env['CARGO_HOME'])
        self.assertEqual('/root/.rustup', env['RUSTUP_HOME'])
        self.assertEqual('stable', env['RUSTUP_TOOLCHAIN'])
        self.assertEqual(COMMIT, env['NETCORE_GIT_COMMIT'])

    def rust(self):
        self.spec['name'] = 'packet-core'
        self.target = self.root / 'netcore-packet-core'
        header = bytearray(20)
        header[:6] = b'\x7fELF\x02\x01'
        header[18:20] = (62).to_bytes(2, 'little')
        self.target.write_bytes(header + b'old runtime')
        manifest = self.root / 'system-backend/packet-core/Cargo.toml'
        manifest.parent.mkdir(parents=True)
        manifest.write_text('[package]\nname="netcore-packet-core"\nversion="0.1.0"\n', encoding='utf-8')
        self.manifest.write_text('{"roles":{}}', encoding='utf-8')
        cache = self.root / 'cache'
        (cache / 'release').mkdir(parents=True)
        native_path = update.Path

        def mapped(value):
            return cache if str(value) == '/var/cache/netcore-service-update/target' else native_path(value)

        return header, cache, mapped

    def test_failed_build_preserves_running_runtime_before_any_stop(self):
        _, _, mapped = self.rust()
        before = self.target.read_bytes()
        with patch.object(update, 'Path', side_effect=mapped), \
                patch.object(update.shutil, 'which', return_value='/root/.cargo/bin/cargo'), \
                patch.object(update, 'run', side_effect=subprocess.CalledProcessError(101, ['cargo'])) as command:
            with self.assertRaises(subprocess.CalledProcessError):
                update.prepare(self.spec, self.target, self.root, COMMIT)
        self.assertEqual(before, self.target.read_bytes())
        self.assertEqual('cargo', Path(command.call_args.args[0][0]).name)
        self.assertIn('--locked', command.call_args.args[0])
        self.assertEqual(COMMIT, command.call_args.kwargs['env']['NETCORE_GIT_COMMIT'])

    def test_wrong_architecture_is_rejected_before_application(self):
        header, cache, mapped = self.rust()
        wrong = bytearray(header)
        wrong[18:20] = (183).to_bytes(2, 'little')

        def build(*args, **kwargs):
            (cache / 'release' / self.target.name).write_bytes(wrong + b'ARM binary')

        with patch.object(update, 'Path', side_effect=mapped), \
                patch.object(update.shutil, 'which', return_value='/root/.cargo/bin/cargo'), \
                patch.object(update, 'run', side_effect=build):
            with self.assertRaisesRegex(RuntimeError, 'Architektur'):
                update.prepare(self.spec, self.target, self.root, COMMIT)

    def test_prepared_apply_does_not_invoke_cargo_again(self):
        plan = self.plan()
        with self.mocks():
            self.apply(plan)
        self.assertTrue(all(call.args[0][0] == 'systemctl' for call in self.run_mock.call_args_list))


class HttpTests(Fixture):
    def response(self, body):
        opener = MagicMock()
        opener.open.return_value.__enter__.return_value.read.return_value = json.dumps(body).encode()
        return opener

    def test_configured_bind_and_ready_endpoint(self):
        self.config.write_text('[server]\nbind="10.0.1.123:9999"\n', encoding='utf-8')
        opener = self.response({'ready': True})
        with patch.object(update.urllib.request, 'build_opener', return_value=opener):
            self.assertTrue(update.check_http(self.spec, ready=True))
        self.assertEqual('http://10.0.1.123:9999/health/ready', opener.open.call_args.args[0])

    def test_directory_contract_uses_devices_api(self):
        spec = dict(name='directory', port=8095, health='/api/devices')
        opener = self.response({'devices': []})
        with patch.object(update.urllib.request, 'build_opener', return_value=opener):
            self.assertTrue(update.check_http(spec))
        self.assertEqual('http://127.0.0.1:8095/api/devices', opener.open.call_args.args[0])

    def test_degraded_json_is_not_reported_ready(self):
        opener = self.response({'ready': False})
        with patch.object(update.urllib.request, 'build_opener', return_value=opener):
            self.assertFalse(update.check_http(self.spec, ready=True))


if __name__ == '__main__':
    unittest.main()
