#!/usr/bin/env python3
"""Local PBX includes retain reader permissions during rendering and failover."""
import importlib.util
import os
from pathlib import Path
import stat
import sys
import tempfile
import types
import unittest
from unittest.mock import Mock, patch


ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / 'system-backend/sip-switch/tbs-fallback/src/netcore_tbs_sip_fallback.py'
spec = importlib.util.spec_from_file_location('tbs_sip_fallback_permissions', SOURCE)
sip = importlib.util.module_from_spec(spec)
if os.name == 'nt':
    # This suite does not exercise the Unix lock API.
    with patch.dict(sys.modules, {'fcntl': types.ModuleType('fcntl')}):
        spec.loader.exec_module(sip)
else:
    spec.loader.exec_module(sip)


class LocalIncludePermissionsTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name)
        self.path = self.root / 'asterisk/netcore-active-registration.conf'
        self.path.parent.mkdir()
        self.config = {
            'service': {'node_id': 'SRV-M-TBS-01'},
            'asterisk': {'config_dir': str(self.path.parent),
                         'native_snippet_file': str(self.root / 'netcore/native.toml')},
            'native_tbs': {'username': 'native', 'password': 'fixture-native'},
            'central': {'host': '192.0.2.1', 'username': 'central', 'password': 'fixture-central'},
            'fallback_pbx': {'host': '192.0.2.2', 'username': 'pbx'},
            'failover': {'state_file': str(self.root / 'state/state.json'),
                         'central_registration_file': str(self.path.parent / 'central.conf'),
                         'pbx_registration_file': str(self.path.parent / 'pbx.conf'),
                         'active_registration_file': str(self.path),
                         'default_mode': 'central'},
            'routing': {},
        }

    @unittest.skipUnless(os.name == 'posix', 'Unix permission semantics')
    def test_first_write_and_replacement_survive_restrictive_umask(self):
        previous = os.umask(0o077)
        self.addCleanup(os.umask, previous)
        for text in ('#include central.conf\n', '#include pbx.conf\n'):
            sip.write_asterisk_config(self.path, text)
            self.assertEqual(self.path.read_text(), text)
            self.assertEqual(stat.S_IMODE(self.path.stat().st_mode), 0o640)
            self.assertEqual(list(self.path.parent.glob('*.tmp')), [])

    @unittest.skipUnless(os.name == 'posix', 'Unix group semantics')
    def test_root_write_selects_asterisk_reader_group(self):
        with patch.object(sip.os, 'geteuid', return_value=0), \
                patch('grp.getgrnam', return_value=types.SimpleNamespace(gr_gid=4321)) as lookup, \
                patch.object(sip.os, 'fchown') as ownership:
            sip.write_asterisk_config(self.path, '#include central.conf\n')
        lookup.assert_called_once_with('asterisk')
        self.assertEqual(ownership.call_args.args[1:], (-1, 4321))

    @unittest.skipUnless(os.name == 'posix', 'Unix group semantics')
    def test_custom_render_target_uses_directory_group_without_asterisk_account(self):
        with patch.object(sip.os, 'geteuid', return_value=0), \
                patch('grp.getgrnam', side_effect=KeyError('asterisk')), \
                patch.object(sip.os, 'fchown') as ownership:
            sip.write_asterisk_config(self.path, '#include central.conf\n')
        self.assertEqual(ownership.call_args.args[1:], (-1, self.path.parent.stat().st_gid))

    @unittest.skipUnless(os.name == 'posix', 'Unix permission semantics')
    def test_permission_failure_preserves_previous_include(self):
        self.path.write_text('old include\n')
        with patch.object(sip.os, 'fchmod', side_effect=PermissionError('cannot set mode')):
            with self.assertRaises(PermissionError):
                sip.write_asterisk_config(self.path, 'new include\n')
        self.assertEqual(self.path.read_text(), 'old include\n')
        self.assertEqual(list(self.path.parent.glob('*.tmp')), [])

    @unittest.skipUnless(os.name == 'posix', 'Unix permission semantics')
    def test_owner_failure_preserves_previous_include(self):
        self.path.write_text('old include\n')
        with patch.object(sip.os, 'geteuid', return_value=0), \
                patch.object(sip.os, 'fchown', side_effect=PermissionError('cannot set group')):
            with self.assertRaises(PermissionError):
                sip.write_asterisk_config(self.path, 'new include\n')
        self.assertEqual(self.path.read_text(), 'old include\n')
        self.assertEqual(list(self.path.parent.glob('*.tmp')), [])

    def test_replace_failure_preserves_previous_include(self):
        self.path.write_text('old include\n')
        with patch.object(sip.os, 'replace', side_effect=OSError('cannot publish')):
            with self.assertRaises(OSError):
                sip.write_asterisk_config(self.path, 'new include\n')
        self.assertEqual(self.path.read_text(), 'old include\n')
        self.assertEqual(list(self.path.parent.glob('*.tmp')), [])

    def test_render_covers_all_six_pbx_includes_and_keeps_native_secret_separate(self):
        with patch.object(sip, 'load', return_value=self.config), \
                patch.object(sip, 'write_asterisk_config', wraps=sip.write_asterisk_config) as writer:
            result = sip.render(self.root / 'fixture.toml')
        includes = {Path(call.args[0]) for call in writer.call_args_list}
        self.assertEqual(len(includes), 6)
        self.assertTrue(all(path.is_file() for path in includes))
        snippet = Path(self.config['asterisk']['native_snippet_file'])
        self.assertNotIn(snippet, includes)
        self.assertIn('fixture-native', snippet.read_text())
        self.assertEqual(len(result['rendered']), 7)

    def controller(self):
        with patch.object(sip, 'load', return_value=self.config):
            return sip.FailoverController(self.root / 'fixture.toml', Mock(), sleeper=lambda _: None)

    def test_failover_include_uses_reader_permissions(self):
        controller = self.controller()
        with patch.object(sip, 'write_asterisk_config', wraps=sip.write_asterisk_config) as writer:
            controller.switch_mode('pbx_direct', 'fixture outage', now=1)
        self.assertEqual(controller.state['mode'], 'pbx_direct')
        self.assertEqual(self.path.read_text(), sip.render_active_registration(self.config, 'pbx_direct'))
        self.assertEqual(writer.call_args.args[0], self.path)
        if os.name == 'posix':
            self.assertEqual(stat.S_IMODE(self.path.stat().st_mode), 0o640)

    def test_failed_reload_restores_readable_central_include(self):
        controller = self.controller()
        controller.cli.reload_registrations.side_effect = [RuntimeError('reload failed'), None]
        with patch.object(sip, 'write_asterisk_config', wraps=sip.write_asterisk_config) as writer:
            with self.assertRaisesRegex(RuntimeError, 'reload failed'):
                controller.switch_mode('pbx_direct', 'fixture outage', now=1)
        self.assertEqual(controller.state['mode'], 'central')
        self.assertEqual(controller.state['phase'], 'TRANSITION_FAILED')
        self.assertEqual(self.path.read_text(), sip.render_active_registration(self.config, 'central'))
        self.assertEqual(writer.call_count, 2)
        if os.name == 'posix':
            self.assertEqual(stat.S_IMODE(self.path.stat().st_mode), 0o640)


if __name__ == '__main__':
    unittest.main(verbosity=2)
