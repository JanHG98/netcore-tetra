"""Guest resolver permissions under the real worker umask, without real mounts.

The guest context and filesystem changes are real. Only mount commands and device
creation are simulated; no ARM64 execution or unprivileged DNS lookup is claimed.
"""
import os
from pathlib import Path
import stat
import sys
import tempfile
import unittest
from unittest.mock import call, patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from image_build import Disk, guest_dns


class GuestResolverTests(unittest.TestCase):
    def exercise_guest(self, original, fail=False):
        host_resolver = Path('/etc/resolv.conf').read_text()
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / 'etc').mkdir()
            (root / 'usr/sbin').mkdir(parents=True)
            resolver = root / 'etc/resolv.conf'
            saved = root / 'etc/resolv.conf.netcore-build'
            original_bytes = b'# Original guest resolver\nnameserver 192.0.2.53\n'
            link_target = '../run/NetworkManager/resolv.conf'
            if original == 'file':
                resolver.write_bytes(original_bytes)
                resolver.chmod(0o640)
            elif original == 'symlink':
                resolver.symlink_to(link_target)
                self.assertFalse(resolver.exists())

            policy = root / 'usr/sbin/policy-rc.d'
            original_policy = b'#!/bin/sh\nexit 0\n'
            policy.write_bytes(original_policy)
            policy.chmod(0o755)
            disk = Disk(root / 'unused.img', root)
            original_mounts = [root, root / 'boot/firmware']
            disk.mounts = original_mounts.copy()
            commands = []

            def simulated_command(args, **kwargs):
                commands.append(list(args))

            def simulated_mknod(path, mode, device):
                # Disk.guest still performs its real chmod on these placeholders.
                Path(path).touch()

            def enter_guest():
                with disk.guest():
                    self.assertFalse(resolver.is_symlink())
                    self.assertTrue(stat.S_ISREG(resolver.stat().st_mode))
                    self.assertEqual(stat.S_IMODE(resolver.stat().st_mode), 0o644)
                    self.assertEqual(resolver.read_text(), host_resolver)
                    self.assertEqual(policy.read_bytes(), b'#!/bin/sh\nexit 101\n')
                    self.assertEqual(stat.S_IMODE(policy.stat().st_mode), 0o755)
                    self.assertEqual(len(disk.mounts), len(original_mounts) + 6)
                    if fail:
                        raise RuntimeError('guest command failed')

            previous_umask = os.umask(0o077)
            try:
                with patch('image_build.command', side_effect=simulated_command), \
                        patch('image_build.os.mknod', side_effect=simulated_mknod):
                    if fail:
                        with self.assertRaisesRegex(RuntimeError, 'guest command failed'):
                            enter_guest()
                    else:
                        enter_guest()
            finally:
                os.umask(previous_umask)

            self.assertEqual(disk.mounts, original_mounts)
            mounted = [command[-1] for command in commands if command[0] == 'mount']
            unmounted = [command[1] for command in commands if command[0] == 'umount']
            self.assertEqual(unmounted, list(reversed([str(path) for path in mounted])))
            self.assertEqual(len(unmounted), 6)
            self.assertFalse(saved.exists())
            self.assertFalse(saved.is_symlink())
            self.assertEqual(policy.read_bytes(), original_policy)
            if original == 'file':
                self.assertFalse(resolver.is_symlink())
                self.assertEqual(resolver.read_bytes(), original_bytes)
                self.assertEqual(stat.S_IMODE(resolver.stat().st_mode), 0o640)
            elif original == 'symlink':
                self.assertTrue(resolver.is_symlink())
                self.assertEqual(os.readlink(resolver), link_target)
                self.assertFalse(resolver.exists())
            else:
                self.assertFalse(resolver.exists())
                self.assertFalse(resolver.is_symlink())

    def test_private_original_file_is_restored(self):
        self.exercise_guest('file')

    def test_dangling_original_symlink_is_restored(self):
        self.exercise_guest('symlink')

    def test_guest_error_restores_original_file_and_unmounts(self):
        self.exercise_guest('file', fail=True)

    def test_guest_error_restores_dangling_symlink_and_unmounts(self):
        self.exercise_guest('symlink', fail=True)

    def test_absent_original_resolver_stays_absent(self):
        for fail in (False, True):
            with self.subTest(guest_error=fail):
                self.exercise_guest('missing', fail=fail)


class GuestDnsTests(unittest.TestCase):
    def expected_calls(self, root):
        prefix = ['chroot', root, '/usr/sbin/runuser', '-u', '_apt', '--']
        return [
            call(prefix + ['/usr/bin/test', '-r', '/etc/resolv.conf'], timeout=10),
            *[call(prefix + ['/usr/bin/timeout', '--kill-after=2s', '15s',
                             '/usr/bin/getent', 'ahostsv4', host], timeout=20)
              for host in ('deb.debian.org', 'archive.raspberrypi.com')],
        ]

    def test_readability_and_both_hosts_use_apt_user_and_timeouts(self):
        root = Path('/fixture/guest')
        with patch('image_build.command') as command, patch('builtins.print'):
            guest_dns(root)
        self.assertEqual(command.call_args_list, self.expected_calls(root))

    def assert_failure_stops_queries(self, failed_index):
        root = Path('/fixture/guest')
        error = RuntimeError('guest DNS command failed')
        with patch('image_build.command', side_effect=[None] * failed_index + [error]) as command, \
                patch('builtins.print'):
            with self.assertRaises(RuntimeError) as raised:
                guest_dns(root)
        self.assertIs(raised.exception, error)
        self.assertEqual(command.call_args_list,
                         self.expected_calls(root)[:failed_index + 1])

    def test_unreadable_resolver_stops_before_network_queries(self):
        self.assert_failure_stops_queries(0)

    def test_first_host_failure_stops_before_second_host(self):
        self.assert_failure_stops_queries(1)

    def test_second_host_failure_propagates(self):
        self.assert_failure_stops_queries(2)


if __name__ == '__main__':
    unittest.main()
