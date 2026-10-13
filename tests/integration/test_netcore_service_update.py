"""Runtime-update regression checks with real file transactions and mocked systemd.

Run from repository root: python3 -m unittest discover -s tests/integration
Native ownership verification runs on POSIX; all transaction checks run on Windows too.
"""
from __future__ import annotations

from contextlib import ExitStack, redirect_stdout
import importlib.util
import io
import json
import os
from pathlib import Path
import stat
import subprocess
import tempfile
import unittest
from unittest.mock import MagicMock, patch


ROOT = Path(__file__).resolve().parents[2]
MODULE_PATH = ROOT / 'tools/operator/update-netcore-service.py'
MODULE_SPEC = importlib.util.spec_from_file_location('netcore_runtime_update', MODULE_PATH)
update = importlib.util.module_from_spec(MODULE_SPEC)
MODULE_SPEC.loader.exec_module(update)


def portable_metadata(stack):
    """Shim unsupported Windows metadata syscalls, leaving real file swaps intact."""
    for name in ('fchmod', 'fchown'):
        if not hasattr(update.os, name):
            stack.enter_context(patch.object(update.os, name, lambda *args: None, create=True))


class Fixture(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='netcore-update-test-')
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.source = self.root / 'source.py'
        self.source.write_bytes(b'print("new runtime")\n')
        self.target = self.root / 'installed.py'
        self.target.write_bytes(b'print("old runtime")\n')
        self.target.chmod(0o755)
        self.config = self.root / 'station.toml'
        self.config.write_bytes(b'# keep this exact comment\nbind = "127.0.0.1:8160"\n')
        self.unit = self.root / 'netcore-test.service'
        self.unit.write_bytes(b'[Service]\n# local operator changes\n')
        self.spec = dict(name='packet-core', unit='netcore-test.service',
                         command=repr(self.target.as_posix()),
                         config_target=self.config.as_posix(), port=8160,
                         health='/health/ready')
        self.backups = self.root / 'backups'
        self.props = dict(LoadState='loaded', FragmentPath=self.unit.as_posix(),
                          ActiveState='active',
                          ExecStart='{ path=' + self.target.as_posix() +
                          ' ; argv[]=' + self.target.as_posix() +
                          ' --config ' + self.config.as_posix() + ' ; }')

    def transaction_patches(self, active=True, state=None):
        stack = ExitStack()
        portable_metadata(stack)
        self.run_mock = stack.enter_context(patch.object(update, 'run'))
        props = dict(self.props, ActiveState=state or ('active' if active else 'inactive'))
        stack.enter_context(patch.object(update, 'properties', return_value=props))
        self.live_mock = stack.enter_context(patch.object(update, 'wait_live'))
        self.http_mock = stack.enter_context(patch.object(update, 'check_http', return_value=True))
        return stack

    def write_prepare_fixture(self, role='hardware-gateway', python_source='value = 1\n'):
        manifest = self.root / 'tools/operator/service-update-assets.json'
        manifest.parent.mkdir(parents=True)
        source = self.root / 'system-backend' / role / 'runtime.py'
        source.parent.mkdir(parents=True)
        source.write_text(python_source, encoding='utf-8')
        manifest.write_text(json.dumps({'roles': {role: {'files': [
            {'source': source.relative_to(self.root).as_posix(),
             'target': self.target.as_posix(), 'mode': '0755'}]}}}), encoding='utf-8')
        return source


class TransactionTests(Fixture):
    def test_update_changes_only_runtime_and_keeps_config_and_unit_bytes(self):
        before_config = self.config.read_bytes()
        before_unit = self.unit.read_bytes()
        old_runtime = self.target.read_bytes()
        with self.transaction_patches():
            result = update.transaction(self.spec, [(self.source, self.target)],
                                        self.props, self.backups)
        self.assertEqual('updated', result['phase'])
        self.assertTrue(result['readiness'])
        self.assertEqual(self.source.read_bytes(), self.target.read_bytes())
        self.assertEqual(before_config, self.config.read_bytes())
        self.assertEqual(before_unit, self.unit.read_bytes())
        stored = Path(result['backup']) / self.target.relative_to(self.target.anchor)
        self.assertEqual(old_runtime, stored.read_bytes())
        self.assertEqual([['systemctl', 'stop', self.spec['unit']],
                          ['systemctl', 'start', self.spec['unit']]],
                         [call.args[0] for call in self.run_mock.call_args_list])
        self.live_mock.assert_called_once_with(self.spec)

    def test_failed_liveness_restores_old_runtime_and_restarts_original_service(self):
        old_runtime = self.target.read_bytes()
        config = self.config.read_bytes()
        with self.transaction_patches(), redirect_stdout(io.StringIO()):
            self.live_mock.side_effect = RuntimeError('new process does not answer')
            with self.assertRaisesRegex(RuntimeError, 'does not answer'):
                update.transaction(self.spec, [(self.source, self.target)],
                                   self.props, self.backups)
        self.assertEqual(old_runtime, self.target.read_bytes())
        self.assertEqual(config, self.config.read_bytes())
        self.assertEqual([['systemctl', 'stop', self.spec['unit']],
                          ['systemctl', 'start', self.spec['unit']],
                          ['systemctl', 'stop', self.spec['unit']],
                          ['systemctl', 'start', self.spec['unit']]],
                         [call.args[0] for call in self.run_mock.call_args_list])

    def test_failure_after_first_replacement_restores_entire_file_set(self):
        second_old = self.root / 'installed-library.py'
        second_old.write_bytes(b'OLD_LIBRARY = True\n')
        second_new = self.root / 'source-library.py'
        second_new.write_bytes(b'OLD_LIBRARY = False\n')
        first_before, second_before = self.target.read_bytes(), second_old.read_bytes()
        original_replace = update.replace_file
        replacements = 0

        def fail_second(source, target):
            nonlocal replacements
            replacements += 1
            if replacements == 2:
                raise OSError('interrupted second file')
            return original_replace(source, target)

        with self.transaction_patches(), patch.object(update, 'replace_file', side_effect=fail_second), \
                redirect_stdout(io.StringIO()):
            with self.assertRaisesRegex(OSError, 'second file'):
                update.transaction(self.spec, [(self.source, self.target),
                                   (second_new, second_old)], self.props, self.backups)
        self.assertEqual(first_before, self.target.read_bytes())
        self.assertEqual(second_before, second_old.read_bytes())

    def test_inactive_service_is_updated_without_start_or_http_call(self):
        inactive = dict(self.props, ActiveState='inactive')
        with self.transaction_patches(active=False):
            result = update.transaction(self.spec, [(self.source, self.target)],
                                        inactive, self.backups)
        self.assertEqual('updated', result['phase'])
        self.assertIsNone(result['readiness'])
        self.assertEqual(self.source.read_bytes(), self.target.read_bytes())
        self.run_mock.assert_not_called()
        self.live_mock.assert_not_called()
        self.http_mock.assert_not_called()

    def test_already_current_does_not_restart_or_create_backup(self):
        self.source.write_bytes(self.target.read_bytes())
        with self.transaction_patches():
            result = update.transaction(self.spec, [(self.source, self.target)],
                                        self.props, self.backups)
        self.assertEqual('already_current', result['phase'])
        self.assertFalse(self.backups.exists())
        self.run_mock.assert_not_called()
        self.assertTrue(result['readiness'])
        self.assertEqual('active', result['active_before'])

    def test_already_current_but_unhealthy_is_reported_degraded_without_restart(self):
        self.source.write_bytes(self.target.read_bytes())
        with self.transaction_patches():
            self.http_mock.return_value = False
            result = update.transaction(self.spec, [(self.source, self.target)],
                                        self.props, self.backups)
        self.assertEqual('already_current_degraded', result['phase'])
        self.assertFalse(result['liveness'])
        self.assertFalse(result['readiness'])
        self.run_mock.assert_not_called()
        self.assertFalse(self.backups.exists())

    def test_current_inactive_runtime_never_starts_even_if_previous_snapshot_was_active(self):
        self.source.write_bytes(self.target.read_bytes())
        with self.transaction_patches(active=False):
            result = update.transaction(self.spec, [(self.source, self.target)],
                                        self.props, self.backups)
        self.assertEqual('already_current', result['phase'])
        self.assertIsNone(result['readiness'])
        self.assertEqual('inactive', result['active_before'])
        self.run_mock.assert_not_called()
        self.http_mock.assert_not_called()

    def test_failed_service_recovers_after_runtime_swap_without_enabling_it(self):
        with self.transaction_patches(state='failed'):
            result = update.transaction(self.spec, [(self.source, self.target)],
                                        dict(self.props, ActiveState='failed'), self.backups)
        self.assertEqual('updated', result['phase'])
        self.assertEqual('failed', result['active_before'])
        self.assertTrue(result['recovered_failed_service'])
        self.assertTrue(result['readiness'])
        self.assertEqual([['systemctl', 'reset-failed', self.spec['unit']],
                          ['systemctl', 'start', self.spec['unit']]],
                         [call.args[0] for call in self.run_mock.call_args_list])
        self.live_mock.assert_called_once_with(self.spec)

    def test_failed_already_current_service_recovers_without_backup_or_file_swap(self):
        self.source.write_bytes(self.target.read_bytes())
        with self.transaction_patches(state='failed'):
            result = update.transaction(self.spec, [(self.source, self.target)],
                                        dict(self.props, ActiveState='failed'), self.backups)
        self.assertEqual('recovered', result['phase'])
        self.assertEqual('failed', result['active_before'])
        self.assertTrue(result['recovered_failed_service'])
        self.assertFalse(self.backups.exists())
        self.assertEqual([['systemctl', 'reset-failed', self.spec['unit']],
                          ['systemctl', 'start', self.spec['unit']]],
                         [call.args[0] for call in self.run_mock.call_args_list])

    def test_failed_service_recovery_failure_restores_files_and_retries_old_service(self):
        old_runtime = self.target.read_bytes()
        with self.transaction_patches(state='failed'), redirect_stdout(io.StringIO()):
            self.live_mock.side_effect = RuntimeError('failed recovery')
            with self.assertRaisesRegex(RuntimeError, 'failed recovery'):
                update.transaction(self.spec, [(self.source, self.target)],
                                   dict(self.props, ActiveState='failed'), self.backups)
        self.assertEqual(old_runtime, self.target.read_bytes())
        self.assertEqual([['systemctl', 'reset-failed', self.spec['unit']],
                          ['systemctl', 'start', self.spec['unit']],
                          ['systemctl', 'stop', self.spec['unit']],
                          ['systemctl', 'reset-failed', self.spec['unit']],
                          ['systemctl', 'start', self.spec['unit']]],
                         [call.args[0] for call in self.run_mock.call_args_list])

    def test_transitional_service_state_is_rejected_without_file_change(self):
        old_runtime = self.target.read_bytes()
        with self.transaction_patches(state='deactivating'):
            with self.assertRaisesRegex(RuntimeError, 'Dienststatus'):
                update.transaction(self.spec, [(self.source, self.target)],
                                   self.props, self.backups)
        self.assertEqual(old_runtime, self.target.read_bytes())
        self.assertFalse(self.backups.exists())
        self.run_mock.assert_not_called()

    def test_readiness_failure_keeps_live_new_runtime_and_reports_degraded(self):
        with self.transaction_patches():
            self.http_mock.return_value = False
            result = update.transaction(self.spec, [(self.source, self.target)],
                                        self.props, self.backups)
        self.assertEqual('updated_degraded', result['phase'])
        self.assertFalse(result['readiness'])
        self.assertEqual(self.source.read_bytes(), self.target.read_bytes())
        self.assertEqual(2, self.run_mock.call_count)

    def test_external_config_change_is_detected_without_overwriting_operator_change(self):
        original_replace = update.replace_file
        mutated = False

        def change_config(source, target):
            nonlocal mutated
            original_replace(source, target)
            if not mutated:
                self.config.write_bytes(b'bind = "127.0.0.1:9999"\n')
                mutated = True

        before = self.target.read_bytes()
        with self.transaction_patches(), patch.object(update, 'replace_file', side_effect=change_config), \
                redirect_stdout(io.StringIO()):
            with self.assertRaisesRegex(RuntimeError, 'Konfiguration'):
                update.transaction(self.spec, [(self.source, self.target)],
                                   self.props, self.backups)
        self.assertEqual(before, self.target.read_bytes())
        self.assertEqual(b'bind = "127.0.0.1:9999"\n', self.config.read_bytes())

    @unittest.skipUnless(hasattr(os, 'fchmod') and hasattr(os, 'fchown'),
                         'Native Unix ownership and mode syscalls are POSIX-only')
    def test_atomic_replacement_preserves_posix_mode_owner_and_group(self):
        self.target.chmod(0o751)
        before = self.target.stat()
        update.replace_file(self.source, self.target)
        after = self.target.stat()
        self.assertEqual(0o751, stat.S_IMODE(after.st_mode))
        self.assertEqual((before.st_uid, before.st_gid), (after.st_uid, after.st_gid))
        self.assertEqual(self.source.read_bytes(), self.target.read_bytes())


class PreparationTests(Fixture):
    def rust_fixture(self):
        self.target = self.root / 'netcore-packet-core'
        self.spec['command'] = self.target.as_posix()
        old = bytearray(20)
        old[:6] = b'\x7fELF\x02\x01'
        old[18:20] = (62).to_bytes(2, 'little')
        self.target.write_bytes(old + b'old x86 runtime')
        manifest = self.root / 'tools/operator/service-update-assets.json'
        manifest.parent.mkdir(parents=True)
        manifest.write_text('{"roles":{"packet-core":{"files":[]}}}', encoding='utf-8')
        cargo_manifest = self.root / 'system-backend/packet-core/Cargo.toml'
        cargo_manifest.parent.mkdir(parents=True)
        cargo_manifest.write_text('[package]\nname="netcore-packet-core"\nversion="0.1.0"\n',
                                  encoding='utf-8')
        cache = self.root / 'cargo-cache'
        (cache / 'release').mkdir(parents=True)
        original_path = update.Path

        def fixture_path(value):
            if str(value) == '/var/cache/netcore-service-update/target':
                return cache
            return original_path(value)

        return old, cache, fixture_path

    def test_successful_build_selects_exact_binary_and_native_elf_before_stop(self):
        old_header, cache, fixture_path = self.rust_fixture()

        def compile_binary(args, **kwargs):
            (cache / 'release' / self.target.name).write_bytes(old_header + b'new x86 runtime')

        with patch.object(update.shutil, 'which', return_value='/root/.cargo/bin/cargo'), \
                patch.object(update, 'Path', side_effect=fixture_path), \
                patch.dict(update.os.environ, {'CARGO_BUILD_TARGET': 'foreign-linux'}), \
                patch.object(update, 'run', side_effect=compile_binary) as command:
            files = update.prepare(self.spec, self.target, root=self.root)
        self.assertEqual([(cache / 'release' / self.target.name, self.target)], files)
        arguments = command.call_args.args[0]
        self.assertEqual(self.target.name, arguments[arguments.index('--bin') + 1])
        self.assertNotIn('CARGO_BUILD_TARGET', command.call_args.kwargs['env'])
        self.assertEqual(old_header + b'old x86 runtime', self.target.read_bytes())

    def test_successful_cargo_with_wrong_architecture_is_rejected_before_stop(self):
        old_header, cache, fixture_path = self.rust_fixture()
        wrong_header = bytearray(old_header)
        wrong_header[18:20] = (183).to_bytes(2, 'little')

        def compile_wrong_binary(args, **kwargs):
            (cache / 'release' / self.target.name).write_bytes(wrong_header + b'wrong ARM runtime')

        with patch.object(update.shutil, 'which', return_value='/root/.cargo/bin/cargo'), \
                patch.object(update, 'Path', side_effect=fixture_path), \
                patch.object(update, 'run', side_effect=compile_wrong_binary) as command:
            with self.assertRaisesRegex(RuntimeError, 'Architektur'):
                update.prepare(self.spec, self.target, root=self.root)
        self.assertEqual(old_header + b'old x86 runtime', self.target.read_bytes())
        self.assertFalse(any(call.args[0][0] == 'systemctl' for call in command.call_args_list))

    def test_python_syntax_failure_happens_before_service_stop(self):
        self.write_prepare_fixture(python_source='def invalid(:\n')
        spec = dict(self.spec, name='hardware-gateway')
        before = self.target.read_bytes()
        with patch.object(update, 'run') as command:
            with self.assertRaises(SyntaxError):
                update.prepare(spec, self.target, root=self.root)
        self.assertEqual(before, self.target.read_bytes())
        command.assert_not_called()

    def test_missing_runtime_asset_is_rejected_before_service_stop(self):
        self.write_prepare_fixture()
        self.target.unlink()
        with patch.object(update, 'run') as command:
            with self.assertRaisesRegex(RuntimeError, 'Installation|Quelldatei'):
                update.prepare(dict(self.spec, name='hardware-gateway'), self.target,
                               root=self.root)
        command.assert_not_called()

    def test_failed_cargo_build_never_stops_service_or_replaces_runtime(self):
        self.write_prepare_fixture(role='packet-core')
        cargo_manifest = self.root / 'system-backend/packet-core/Cargo.toml'
        cargo_manifest.write_text('[package]\nname="netcore-packet-core"\nversion="0.1.0"\n',
                                  encoding='utf-8')
        old_runtime = self.target.read_bytes()
        old_config = self.config.read_bytes()
        with patch.object(update.shutil, 'which', return_value='/root/.cargo/bin/cargo'), \
                patch.object(update, 'run', side_effect=subprocess.CalledProcessError(101, ['cargo'])) as command:
            with self.assertRaises(subprocess.CalledProcessError):
                update.prepare(self.spec, self.target, root=self.root)
        self.assertEqual(old_runtime, self.target.read_bytes())
        self.assertEqual(old_config, self.config.read_bytes())
        commands = [call.args[0] for call in command.call_args_list]
        self.assertEqual(1, len(commands))
        self.assertEqual('/root/.cargo/bin/cargo', commands[0][0])
        self.assertIn('--locked', commands[0])
        self.assertFalse(any(cmd[0] == 'systemctl' for cmd in commands))

    def test_directory_legacy_runtime_uses_matching_legacy_source(self):
        self.write_prepare_fixture(role='directory')
        source = self.root / 'misc/ID-Server/netcore_directory_server.py'
        source.parent.mkdir(parents=True)
        source.write_text('LEGACY = True\n', encoding='utf-8')
        target = self.root / 'netcore_directory_server.py'
        target.write_text('LEGACY = False\n', encoding='utf-8')
        manifest = self.root / 'tools/operator/service-update-assets.json'
        data = json.loads(manifest.read_text())
        data['roles']['directory']['files'][0]['target'] = target.as_posix()
        manifest.write_text(json.dumps(data), encoding='utf-8')
        files = update.prepare(dict(self.spec, name='directory'), target, root=self.root)
        self.assertEqual([(source, target)], files)


class ValidationTests(Fixture):
    def test_local_unit_and_matching_custom_config_path_are_accepted(self):
        with patch.object(update, 'properties', return_value=self.props):
            props, target = update.validate_install(self.spec)
        self.assertEqual(self.props, props)
        self.assertEqual(self.target, target)

    def test_unrecognized_execstart_fails_without_guessing(self):
        bad = dict(self.props, ExecStart='/different/private/runtime --config ' + self.config.as_posix())
        with patch.object(update, 'properties', return_value=bad):
            with self.assertRaisesRegex(RuntimeError, 'ExecStart'):
                update.validate_install(self.spec)

    def test_unit_using_other_config_path_is_rejected(self):
        bad = dict(self.props, ExecStart='{ argv[]=' + self.target.as_posix() +
                   ' --config /etc/operator/other.toml ; }')
        with patch.object(update, 'properties', return_value=bad):
            with self.assertRaisesRegex(RuntimeError, 'Konfigurationspfad'):
                update.validate_install(self.spec)

    def test_missing_unit_is_rejected(self):
        with patch.object(update, 'properties', return_value={'LoadState': 'not-found'}):
            with self.assertRaisesRegex(RuntimeError, 'Unit'):
                update.validate_install(self.spec)


class HttpContractTests(Fixture):
    def response(self, body):
        opener = MagicMock()
        response = opener.open.return_value.__enter__.return_value
        response.read.return_value = json.dumps(body).encode('utf-8')
        return opener

    def test_liveness_uses_configured_bind_and_not_readiness_endpoint(self):
        self.config.write_text('bind="10.0.1.123:9012"\n', encoding='utf-8')
        opener = self.response({'status': 'live'})
        with patch.object(update.urllib.request, 'build_opener', return_value=opener):
            self.assertTrue(update.check_http(self.spec))
        self.assertEqual('http://10.0.1.123:9012/health/live', opener.open.call_args.args[0])

    def test_nested_alert_bind_and_port_use_local_address_for_wildcard(self):
        self.config.write_text('[server]\nbind="0.0.0.0"\nport=8310\n', encoding='utf-8')
        opener = self.response({'status': 'live'})
        with patch.object(update.urllib.request, 'build_opener', return_value=opener):
            self.assertTrue(update.check_http(dict(self.spec, name='alert-service', port=8310)))
        self.assertEqual('http://127.0.0.1:8310/health/live', opener.open.call_args.args[0])

    def test_ready_false_is_not_reported_as_healthy(self):
        opener = self.response({'ready': False, 'status': 'degraded'})
        with patch.object(update.urllib.request, 'build_opener', return_value=opener):
            self.assertFalse(update.check_http(self.spec, ready=True))
        self.assertTrue(opener.open.call_args.args[0].endswith('/health/ready'))

    def test_directory_uses_api_contract_for_liveness(self):
        opener = self.response({'devices': []})
        directory = dict(name='directory', unit='netcore-directory.service',
                         port=8095, health='/api/devices')
        with patch.object(update.urllib.request, 'build_opener', return_value=opener):
            self.assertTrue(update.check_http(directory))
        self.assertEqual('http://127.0.0.1:8095/api/devices', opener.open.call_args.args[0])

    def test_invalid_json_health_response_is_rejected(self):
        opener = self.response({'status': 'live'})
        opener.open.return_value.__enter__.return_value.read.return_value = b'<html>wrong server</html>'
        with patch.object(update.urllib.request, 'build_opener', return_value=opener):
            with self.assertRaises(ValueError):
                update.check_http(self.spec)


class CatalogCoverageTests(unittest.TestCase):
    def test_every_catalog_runtime_has_a_build_or_python_update_route(self):
        catalog = ROOT / 'system-backend/deployment-core/catalog.json'
        if not catalog.is_file():
            self.fail('Full checkout/catalog.json required for source coverage')
        manifest = json.loads((ROOT / 'tools/operator/service-update-assets.json').read_text())
        services = update.specs(ROOT)
        self.assertNotIn('tbs', services)
        self.assertIn('directory', services)
        self.assertIn('discovery', services)
        for role in services:
            with self.subTest(role=role):
                if role in ('discovery', 'deployment-core'):
                    continue  # Discovery special route; controller remains a VM update.
                rust = update.cargo_manifest_path(role, ROOT)
                self.assertTrue(rust.is_file() or bool(manifest['roles'].get(role, {}).get('files')),
                                role + ' lacks a runtime-only update route')

    def test_python_asset_manifest_does_not_deploy_configs_units_or_seeds(self):
        manifest = json.loads((ROOT / 'tools/operator/service-update-assets.json').read_text())
        expected = {'hardware-gateway', 'rf-monitor', 'alarm-workflow', 'task-workflow',
                    'asset-management', 'sip-switch', 'alert-service', 'directory'}
        self.assertTrue(expected.issubset(manifest['roles']))
        for role, record in manifest['roles'].items():
            for asset in record['files']:
                with self.subTest(role=role, source=asset['source']):
                    self.assertFalse(asset['target'].startswith('/etc/'))
                    self.assertFalse(asset['target'].endswith(('.service', '.timer', '.toml', '.env')))
                    self.assertNotEqual('seed.json', Path(asset['source']).name)


if __name__ == '__main__':
    unittest.main()
