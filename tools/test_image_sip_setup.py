#!/usr/bin/env python3
"""Offline SIP image setup uses explicit upstream credentials and no live daemons."""
import importlib.util
from contextlib import ExitStack
import os
from pathlib import Path
import shutil
import sys
import tempfile
import tomllib
import types
import unittest
from unittest.mock import Mock, patch

ROOT = Path(__file__).resolve().parents[1]
DEPLOY = ROOT / 'system-backend/deployment-core'
sys.path.insert(0, str(DEPLOY))
from common import toml_dump
spec = importlib.util.spec_from_file_location('image_sip_setup', DEPLOY / 'image/sip_setup.py')
sip = importlib.util.module_from_spec(spec)
spec.loader.exec_module(sip)


class ImageSipSetupTests(unittest.TestCase):
    def text(self, **sections):
        data = {
            'service': {'node_id': 'SRV-M-TBS-01'},
            'central': {'host': '10.0.1.125', 'username': 'tbs-srv-m-tbs-01', 'password': 'fixture-central'},
            'fallback_pbx': {'host': '10.0.1.21', 'username': 'operator-supplied'},
        }
        data.update(sections)
        return toml_dump(data)

    def validate(self, text):
        return sip.validate_sip_config(text, 'SRV-M-TBS-01')

    def test_no_configuration_performs_no_installation_or_unit_action(self):
        runner = Mock()
        self.assertIsNone(sip.configure_local_sip('', 'SRV-M-TBS-01', runner=runner))
        runner.assert_not_called()

    def test_missing_central_secret_has_actionable_secret_free_error(self):
        with self.assertRaisesRegex(ValueError, 'central.password fehlt'):
            self.validate(self.text(central={'host': '10.0.1.125', 'username': 'tbs'}))

    def test_pbx_identity_is_required_without_invented_default(self):
        with self.assertRaisesRegex(ValueError, 'fallback_pbx.username fehlt'):
            self.validate(self.text(fallback_pbx={'host': '10.0.1.21'}))

    def test_profile_mismatch_is_rejected(self):
        with self.assertRaisesRegex(ValueError, 'Stationskennung passt nicht'):
            self.validate(self.text(service={'node_id': 'SRV-M-TBS-03'}))

    def test_parser_error_does_not_include_credential_text(self):
        with self.assertRaises(ValueError) as error:
            self.validate('[central]\npassword = "sensitive fixture\n')
        self.assertEqual(str(error.exception), 'SIP-Fallback: ungültiges TOML')

    def test_upstream_auth_identity_without_password_is_rejected(self):
        with self.assertRaisesRegex(ValueError, 'PBX-Passwort'):
            self.validate(self.text(fallback_pbx={'host': '10.0.1.21', 'username': 'pbx', 'auth_username': 'pbx-auth'}))

    def test_loopback_upstream_and_conf_line_injection_are_rejected(self):
        for host in ('127.0.0.1', 'localhost', '10.0.1.125\n[transport]'):
            with self.subTest(host=host), self.assertRaises(ValueError):
                self.validate(self.text(central={'host': host, 'username': 'tbs', 'password': 'fixture'}))

    def test_invalid_native_and_rtp_ports_are_rejected(self):
        for section in ({'native_tbs': {'bind_port': 5060}}, {'asterisk': {'rtp_start': 30300, 'rtp_end': 30200}},
                        {'native_tbs': {'bind_port': True}}, {'native_tbs': {'bind_port': -1}}):
            with self.subTest(section=section), self.assertRaises(ValueError):
                self.validate(self.text(**section))

    def test_supplied_paths_and_starting_direct_mode_are_normalized(self):
        config = self.validate(self.text(asterisk={'binary': '/tmp/malicious', 'config_dir': '/tmp/elsewhere'},
                                         failover={'default_mode': 'pbx_direct', 'state_file': '/tmp/secret'}))
        self.assertEqual(config['asterisk']['binary'], '/usr/sbin/asterisk')
        self.assertEqual(config['asterisk']['config_dir'], '/etc/asterisk')
        self.assertEqual(config['failover']['default_mode'], 'central')
        self.assertEqual(config['failover']['state_file'], '/var/lib/netcore-tbs-sip-fallback/state.json')

    def test_rendered_image_has_exclusive_registration_private_secret_and_enable_only(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            executable = root / 'usr/local/bin/netcore-tbs-sip-fallback'
            executable.parent.mkdir(parents=True)
            shutil.copyfile(ROOT / 'system-backend/sip-switch/tbs-fallback/src/netcore_tbs_sip_fallback.py', executable)
            runner = Mock()
            with ExitStack() as stack:
                if os.name == 'nt':
                    stack.enter_context(patch.dict(sys.modules, {'fcntl': types.ModuleType('fcntl')}))
                    # Linux CI exercises common.atomic_write's directory fsync.
                    def windows_write(path, text, mode=0o600):
                        Path(path).write_text(text)
                        os.chmod(path, mode)
                    stack.enter_context(patch('common.atomic_write', side_effect=windows_write))
                native = sip.configure_local_sip(self.text(), 'SRV-M-TBS-01', root=root, runner=runner)
            self.assertEqual(native['remote_host'], '127.0.0.1')
            self.assertEqual(native['remote_port'], 5060)
            self.assertEqual(native['local_user'], 'netcore-native-srv-m-tbs-01')
            self.assertGreaterEqual(len(native['password']), 32)
            active = (root / 'etc/asterisk/netcore-active-registration.conf').read_text()
            self.assertIn('netcore-registration-central.conf', active)
            self.assertNotIn('netcore-registration-pbx-direct.conf', active)
            config_file = root / 'etc/netcore/tbs-sip-fallback.toml'
            config = tomllib.loads(config_file.read_text())
            self.assertEqual(config['central']['password'], 'fixture-central')
            self.assertEqual(config['native_tbs']['password'], native['password'])
            runner.assert_called_once_with(['systemctl', 'enable', 'asterisk.service', 'netcore-tbs-sip-failover.service'],
                                          check=True, capture_output=True, text=True)
            self.assertTrue((root / 'etc/systemd/system/tetra.service.d/30-local-sip.conf').is_file())
            if os.name == 'posix':
                self.assertEqual(config_file.stat().st_mode & 0o777, 0o600)


if __name__ == '__main__':
    unittest.main(verbosity=2)
