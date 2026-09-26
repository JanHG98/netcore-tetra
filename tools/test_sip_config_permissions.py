#!/usr/bin/env python3
"""Asterisk must read generated includes even after a root update with umask 077."""
import os
from pathlib import Path
import stat
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'system-backend/sip-switch/src'))
from netcore_sip_switch import write_asterisk_config


class IncludePermissionsTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.path = Path(self.directory.name) / 'netcore-pjsip.conf'

    @unittest.skipUnless(os.name == 'posix', 'Unix permission semantics')
    def test_first_write_and_replace_are_group_readable_under_umask_077(self):
        previous = os.umask(0o077)
        self.addCleanup(os.umask, previous)
        for text in ('[first]\ntype=endpoint\n', '[updated]\ntype=endpoint\n'):
            write_asterisk_config(self.path, text)
            self.assertEqual(self.path.read_text(), text)
            metadata = self.path.stat()
            self.assertEqual(stat.S_IMODE(metadata.st_mode), 0o640)
            if os.geteuid() == 0:
                import grp
                try:
                    expected = grp.getgrnam('asterisk').gr_gid
                except KeyError:
                    expected = self.path.parent.stat().st_gid
                self.assertEqual(metadata.st_gid, expected)
        self.assertEqual(list(self.path.parent.glob('*.tmp')), [])

    @unittest.skipUnless(os.name == 'posix', 'Unix permission semantics')
    def test_permission_failure_keeps_the_previous_include(self):
        self.path.write_text('existing configuration')
        with patch('netcore_sip_switch.os.fchmod', side_effect=PermissionError('cannot set mode')):
            with self.assertRaises(PermissionError):
                write_asterisk_config(self.path, 'new configuration')
        self.assertEqual(self.path.read_text(), 'existing configuration')
        self.assertEqual(list(self.path.parent.glob('*.tmp')), [])

    def test_replace_failure_keeps_the_previous_include(self):
        self.path.write_text('existing configuration')
        with patch('netcore_sip_switch.os.replace', side_effect=OSError('cannot replace')):
            with self.assertRaises(OSError):
                write_asterisk_config(self.path, 'new configuration')
        self.assertEqual(self.path.read_text(), 'existing configuration')
        self.assertEqual(list(self.path.parent.glob('*.tmp')), [])


if __name__ == '__main__':
    unittest.main(verbosity=2)
