#!/usr/bin/env python3
"""Isolated regression tests for removing an accidental SIP Switch role from a TBS."""
import contextlib
import importlib.util
import io
import json
from pathlib import Path
import stat
import subprocess
import sys
import tempfile
import tomllib
import unittest
from unittest.mock import patch


ROOT = Path(__file__).resolve().parents[1]
BASE = ROOT / 'system-backend/sip-switch'
SPEC = importlib.util.spec_from_file_location('sip_role_repair', BASE / 'install/repair-tbs-local-fallback.py')
repair = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(repair)


class RepairTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.config = self.path('/etc/netcore/tbs-sip-fallback.toml')
        original = (BASE / 'tbs-fallback/config/tbs-sip-fallback.example.toml').read_text()
        original = original.replace('"netcore-tbs-01"', '"<PBX-FALLBACK-ID>"')
        original = original.replace('password = "openlab-central"', 'password = ' + json.dumps('secret "quoted" \\ path # marker'))
        original = original.replace('auth_username = ""', 'auth_username = "104-auth"')
        original = original.replace('password = ""', 'password = ' + json.dumps('pbx "password" \\ # untouched'))
        self.write(self.config, original)
        self.config.chmod(0o640)
        for name, central in repair.CENTRAL_INCLUDES.items():
            includes = ''.join(f'#include {value}\n' for value in repair.FALLBACK_INCLUDES[name])
            text = f'; Existing site configuration\n[general]\n#include custom-{name}\n{includes}#include {central}\n'
            self.write(self.path('/etc/asterisk') / name, text)
        self.write(self.path('/etc/asterisk/custom-pjsip.conf'), '[custom]\ntype=endpoint\n')
        self.write(self.path('/etc/netcore/sip-switch.toml'), '[pbx]\npassword="central secret"\n')
        self.write(self.path('/etc/netcore/sip-switch-agi.env'), 'NETCORE_SIP_SWITCH=http://127.0.0.1:8300\n')
        self.write(self.path('/etc/netcore/lxc-network.env'), 'NETCORE_SERVICE=sip-switch\nNETCORE_IP=10.0.1.20\n')
        self.state = self.path('/var/lib/netcore-tbs-sip-fallback/state.json')
        self.write(self.state, json.dumps({'mode': 'pbx_direct', 'failures': 4, 'last_reason': 'central_unavailable'}))
        self.write(self.path('/etc/systemd/system/netcore-sip-switch.service'), '[Service]\nExecStart=/usr/local/bin/netcore-sip-switch\n')
        self.calls = []

    def path(self, absolute):
        return self.root / absolute.lstrip('/')

    @staticmethod
    def write(path, text):
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding='utf-8')

    def snapshot(self):
        return {str(path.relative_to(self.root)): path.read_bytes()
                for path in self.root.rglob('*') if path.is_file()}

    def fake_run(self, argv, **kwargs):
        self.assertTrue(kwargs.get('check'))
        self.calls.append(argv)
        return subprocess.CompletedProcess(argv, 0)

    def plan(self, **overrides):
        values = dict(root=self.root, node_id='SRV-M-TBS-01', pbx_user='104')
        values.update(overrides)
        return repair.prepare(**values)

    def apply(self, run=None):
        writes, quarantine = self.plan()
        with contextlib.redirect_stdout(io.StringIO()):
            return repair.apply_repair(writes, quarantine, run=run or self.fake_run, root=self.root)

    def test_prepare_is_read_only_and_removes_only_central_includes(self):
        before = self.snapshot()
        writes, _ = self.plan()
        self.assertEqual(self.snapshot(), before)
        for name, central in repair.CENTRAL_INCLUDES.items():
            repaired = writes[self.path('/etc/asterisk') / name]
            self.assertNotIn(f'#include {central}\n', repaired)
            self.assertIn(f'#include custom-{name}\n', repaired)
            self.assertIn('[general]', repaired)
            for fallback in repair.FALLBACK_INCLUDES[name]:
                self.assertEqual(repaired.count(f'#include {fallback}\n'), 1)

    def test_quoted_absolute_optional_includes_removed_without_touching_similar_names(self):
        source = (' #tryinclude "/etc/asterisk/netcore-pjsip.conf" ; generated\n'
                  '#include <netcore-pjsip.conf>\n'
                  '#include netcore-pjsip.conf.custom\n'
                  '; #include netcore-pjsip.conf\n'
                  '#include "/etc/asterisk/netcore-tbs-fallback-pjsip.conf"\n')
        result = repair.repair_includes(source, 'netcore-pjsip.conf', ['netcore-tbs-fallback-pjsip.conf'])
        self.assertNotIn('#tryinclude', result)
        self.assertNotIn('#include <netcore-pjsip.conf>', result)
        self.assertIn('#include netcore-pjsip.conf.custom', result)
        self.assertIn('; #include netcore-pjsip.conf', result)
        self.assertEqual(result.count('netcore-tbs-fallback-pjsip.conf'), 1)

    def test_missing_fallback_includes_are_restored(self):
        path = self.path('/etc/asterisk/pjsip.conf')
        self.write(path, '[custom]\n#include netcore-pjsip.conf')
        writes, _ = self.plan()
        result = writes[path]
        self.assertTrue(result.startswith('[custom]\n'))
        for name in repair.FALLBACK_INCLUDES['pjsip.conf']:
            self.assertIn(f'#include {name}\n', result)

    def test_pbx_identity_changes_without_password_or_auth_username_changes(self):
        before_text = self.config.read_text()
        before = tomllib.loads(before_text)
        writes, _ = self.plan()
        after = tomllib.loads(writes[self.config])
        for name in ('username', 'from_user', 'contact_user'):
            self.assertEqual(after['fallback_pbx'][name], '104')
            before['fallback_pbx'][name] = '104'
        self.assertEqual(after, before)
        before_secrets = [line for line in before_text.splitlines() if line.startswith('password =')]
        after_secrets = [line for line in writes[self.config].splitlines() if line.startswith('password =')]
        self.assertEqual(before_secrets, after_secrets)

    def test_valid_config_without_requested_pbx_change_is_byte_preserved(self):
        self.write(self.config, self.config.read_text().replace('<PBX-FALLBACK-ID>', '104'))
        writes, _ = self.plan(pbx_user=None)
        self.assertEqual(writes[self.config], self.config.read_text())

    def test_placeholder_requires_explicit_pbx_user(self):
        before = self.snapshot()
        with self.assertRaisesRegex(ValueError, 'PBX-Platzhalter'):
            self.plan(pbx_user=None)
        self.assertEqual(self.snapshot(), before)

    def test_invalid_pbx_user_and_incomplete_section_fail_without_changes(self):
        for username in ('<PBX-FALLBACK-ID>', '104@pbx', '104\ncontact_user=evil', ''):
            with self.subTest(username=username), self.assertRaises(ValueError):
                self.plan(pbx_user=username)
        text = self.config.read_text()
        self.write(self.config, '\n'.join(line for line in text.splitlines() if not line.startswith('contact_user =')))
        before = self.snapshot()
        with self.assertRaisesRegex(ValueError, 'unvollständig'):
            self.plan()
        self.assertEqual(self.snapshot(), before)

    def test_missing_config_or_wrong_node_refuses_any_change(self):
        before = self.snapshot()
        with self.assertRaisesRegex(ValueError, 'Node-ID'):
            self.plan(node_id='ANOTHER-TBS')
        self.assertEqual(self.snapshot(), before)
        self.config.unlink()
        before = self.snapshot()
        with self.assertRaises(FileNotFoundError):
            self.plan()
        self.assertEqual(self.snapshot(), before)

    def test_custom_asterisk_directory_refuses_automatic_repair(self):
        self.write(self.config, self.config.read_text().replace('config_dir = "/etc/asterisk"', 'config_dir = "/custom/asterisk"'))
        before = self.snapshot()
        with self.assertRaisesRegex(ValueError, 'Abweichendes'):
            self.plan()
        self.assertEqual(self.snapshot(), before)

    def test_network_metadata_is_quarantined_only_for_sip_switch(self):
        metadata = self.path('/etc/netcore/lxc-network.env')
        for service, expected in [('sip-switch', True), ('node-gateway', False)]:
            self.write(metadata, f'NETCORE_SERVICE={service}\nNETCORE_IP=10.0.1.20\n')
            _, quarantine = self.plan()
            self.assertEqual(metadata in quarantine, expected)

    def test_apply_backs_up_before_services_then_repairs_and_preserves_active_state(self):
        before = self.snapshot()
        def checked_run(argv, **kwargs):
            backups = list(self.path('/var/backups/netcore-sip-repair').glob('tbs-*'))
            self.assertEqual(len(backups), 1)
            self.assertEqual((backups[0] / self.config.relative_to(self.root)).read_bytes(), before[str(self.config.relative_to(self.root))])
            return self.fake_run(argv, **kwargs)
        backup = self.apply(run=checked_run)
        self.assertEqual(stat.S_IMODE(backup.stat().st_mode), 0o700)
        self.assertEqual(stat.S_IMODE(self.config.stat().st_mode), 0o640)
        for relative, original in before.items():
            saved = backup / relative
            self.assertTrue(saved.is_file(), relative)
            self.assertEqual(saved.read_bytes(), original, relative)
        for relative in ('/etc/netcore/sip-switch.toml', '/etc/netcore/sip-switch-agi.env', '/etc/netcore/lxc-network.env'):
            self.assertFalse(self.path(relative).exists())
        self.assertEqual(self.state.read_bytes(), before[str(self.state.relative_to(self.root))])
        self.assertEqual(self.calls, [
            ['systemctl', 'stop', 'netcore-tbs-sip-failover.service'],
            ['systemctl', 'disable', '--now', 'netcore-sip-switch.service'],
            ['/usr/local/bin/netcore-tbs-sip-fallback', '--config', '/etc/netcore/tbs-sip-fallback.toml', '--render'],
            ['systemctl', 'restart', 'asterisk.service'],
            ['systemctl', 'start', 'netcore-tbs-sip-failover.service'],
        ])

    def test_failed_render_stops_before_restart_and_retains_original_backup(self):
        original = self.config.read_bytes()
        def failing_run(argv, **kwargs):
            self.fake_run(argv, **kwargs)
            if '--render' in argv:
                raise subprocess.CalledProcessError(1, argv)
            return subprocess.CompletedProcess(argv, 0)
        with self.assertRaisesRegex(RuntimeError, 'Reparatur abgebrochen.*Sicherung'):
            self.apply(run=failing_run)
        self.assertFalse(any('restart' in argv for argv in self.calls))
        backup, = self.path('/var/backups/netcore-sip-repair').glob('tbs-*')
        self.assertEqual((backup / self.config.relative_to(self.root)).read_bytes(), original)
        self.assertTrue((backup / 'etc/netcore/sip-switch.toml').exists())

    def test_failure_stopping_service_does_not_modify_configuration(self):
        before = self.snapshot()
        def failing_run(argv, **kwargs):
            raise subprocess.CalledProcessError(1, argv)
        with self.assertRaises(RuntimeError):
            self.apply(run=failing_run)
        for relative, original in before.items():
            self.assertEqual((self.root / relative).read_bytes(), original)

    def test_repeating_repair_keeps_configuration_and_original_backup(self):
        first = self.apply()
        repaired = {path: path.read_bytes() for path in [self.config, *(self.path('/etc/asterisk') / name for name in repair.CENTRAL_INCLUDES)]}
        second = self.apply()
        self.assertNotEqual(first, second)
        for path, content in repaired.items():
            self.assertEqual(path.read_bytes(), content)
        self.assertIn(b'<PBX-FALLBACK-ID>', (first / self.config.relative_to(self.root)).read_bytes())

    def test_cli_defaults_to_dry_run(self):
        before = self.snapshot()
        prepared = self.plan()
        with patch.object(sys, 'argv', ['repair', '--node-id', 'SRV-M-TBS-01', '--pbx-user', '104']), \
                patch.object(repair, 'prepare', return_value=prepared), \
                patch.object(repair, 'apply_repair') as apply, \
                contextlib.redirect_stdout(io.StringIO()) as output:
            self.assertEqual(repair.main(), 0)
        apply.assert_not_called()
        self.assertIn('Prüflauf: keine Änderungen', output.getvalue())
        self.assertEqual(self.snapshot(), before)


if __name__ == '__main__':
    unittest.main()
