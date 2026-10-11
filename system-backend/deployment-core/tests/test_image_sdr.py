"""SoapySX access regressions using real guest files and worker permissions.

The helper tests simulate only ARM64 command output. The cache integration test
also simulates mounts and guest commands. Filesystem changes and publication are
real; no ARM64 execution or hardware probe is claimed.
"""
from contextlib import contextmanager
import json
import os
from pathlib import Path
import shutil
import stat
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import image_build


SDR_DIRECTORY = Path('usr/local/lib/SoapySDR')
MODULE_DIRECTORY = SDR_DIRECTORY / 'modules0.8'
MODULE = MODULE_DIRECTORY / 'libSXSupport.so'
INFO = """Lib Version: v0.8.1-3
ABI Version: v0.8
Module found: /usr/local/lib/SoapySDR/modules0.8/libSXSupport.so
Available factories... sx
Available converters...
 - CF32 -> [CF32, CS16]
"""


class GuestSdrTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name)
        # The fixture has the normal OS ancestors. Only the SDR installation
        # inherits the image worker's restrictive umask.
        (self.root / 'usr/local/lib').mkdir(parents=True)

    def make_installation(self):
        previous_umask = os.umask(0o077)
        try:
            (self.root / MODULE_DIRECTORY).mkdir(parents=True)
            (self.root / MODULE).write_bytes(b'SoapySX fixture; not executable')
        finally:
            os.umask(previous_umask)

    def mode(self, path):
        return stat.S_IMODE(path.stat().st_mode)

    def expected_command(self):
        return ['chroot', self.root, '/usr/sbin/runuser', '-u', 'netcore', '--',
                '/usr/bin/timeout', '--kill-after=2s', '15s',
                '/usr/bin/SoapySDRUtil', '--info']

    def assert_rejected_before_probe(self):
        with patch('image_build.output') as output:
            with self.assertRaisesRegex(RuntimeError, 'SoapySX|netcore'):
                image_build.guest_sdr(self.root)
        output.assert_not_called()

    def test_repairs_cached_private_installation_without_broadening_other_files(self):
        self.make_installation()
        sdr = self.root / SDR_DIRECTORY
        modules = self.root / MODULE_DIRECTORY
        module = self.root / MODULE
        self.assertEqual(self.mode(sdr), 0o700)
        self.assertEqual(self.mode(modules), 0o700)
        self.assertEqual(self.mode(module), 0o600)
        original_module = module.read_bytes()

        private = sdr / 'private'
        private.mkdir(mode=0o700)
        secret = private / 'settings.json'
        secret.write_bytes(b'{"private": true}\n')
        secret.chmod(0o600)
        private_sibling = modules / 'unrelated-private.bin'
        private_sibling.write_bytes(b'private unrelated data')
        private_sibling.chmod(0o600)

        previous_umask = os.umask(0o077)
        try:
            with patch('image_build.output', return_value=INFO) as output:
                image_build.guest_sdr(self.root)
            self.assertEqual(os.umask(0o077), 0o077)
        finally:
            os.umask(previous_umask)

        output.assert_called_once_with(self.expected_command())
        self.assertEqual(self.mode(sdr), 0o755)
        self.assertEqual(self.mode(modules), 0o755)
        self.assertEqual(self.mode(module), 0o644)
        self.assertEqual(module.read_bytes(), original_module)
        self.assertEqual(self.mode(private), 0o700)
        self.assertEqual(self.mode(secret), 0o600)
        self.assertEqual(secret.read_bytes(), b'{"private": true}\n')
        self.assertEqual(self.mode(private_sibling), 0o600)
        self.assertEqual(private_sibling.read_bytes(), b'private unrelated data')

    def test_cache_hit_validates_sdr_before_compression_and_publication(self):
        for info, succeeds in ((INFO, True), ('Available factories... audio', False)):
            with self.subTest(sx_available=succeeds), \
                    tempfile.TemporaryDirectory() as directory:
                state = Path(directory)
                for name in ('requests', 'cache', 'work', 'artifacts'):
                    (state / name).mkdir()
                key, recipe = 'a' * 32, 'b' * 64
                request = dict(
                    profile=dict(name='test-tbs', mcc=901, mnc=1510,
                                 issi=4010003, la=3, cc=1),
                    commit='c' * 40, config='[net_info]\n[cell_info]\n[phy_io]\n',
                    password_hash='$6$fixture$' + 'A' * 86,
                    controller_url='http://10.0.1.131:8320')
                (state / 'requests' / (key + '.json')).write_text(json.dumps(request))
                cached_image = state / 'cache' / (recipe + '.img')
                cached_image.write_bytes(b'generalized cache image')
                (state / 'cache' / (recipe + '.json')).write_text(
                    json.dumps({'commit': request['commit']}))
                events = []

                class SimulatedDisk:
                    def __init__(disk, image, root):
                        disk.root = root
                        disk.loop = '/dev/fixture-loop'

                    def __enter__(disk):
                        return disk

                    def __exit__(disk, *exc):
                        return False

                    def rootfs(disk):
                        previous_umask = os.umask(0o077)
                        try:
                            (disk.root / MODULE_DIRECTORY).mkdir(parents=True)
                            (disk.root / MODULE).write_bytes(b'cached plugin')
                        finally:
                            os.umask(previous_umask)

                    @contextmanager
                    def guest(disk):
                        yield

                def simulated_command(args, **kwargs):
                    if args[0] == 'cp':
                        shutil.copyfile(args[-2], args[-1])
                    elif str(args[-1]).endswith('/personalize.py'):
                        events.append('personalize')
                    elif args[0] == 'xz':
                        self.assertEqual(events, ['personalize', 'sdr'])
                        self.assertFalse((state / 'artifacts' / key).exists())
                        events.append('compress')
                        Path(args[-1]).with_suffix('.img.xz').write_bytes(b'compressed fixture')
                    else:
                        self.fail('Unexpected guest command: ' + repr(args))

                def simulated_info(args):
                    root = state / 'work' / key / 'root'
                    self.assertEqual(args, ['chroot', root, '/usr/sbin/runuser',
                                            '-u', 'netcore', '--', '/usr/bin/timeout',
                                            '--kill-after=2s', '15s',
                                            '/usr/bin/SoapySDRUtil', '--info'])
                    self.assertEqual(events, ['personalize'])
                    self.assertEqual(self.mode(root / MODULE_DIRECTORY), 0o755)
                    self.assertEqual(self.mode(root / MODULE), 0o644)
                    events.append('sdr')
                    return info

                with patch('image_build.recover_work'), \
                        patch('image_build.preflight'), \
                        patch('image_build.recipe_key', return_value=recipe), \
                        patch('image_build.download_base') as download, \
                        patch('image_build.Disk', SimulatedDisk), \
                        patch('image_build.install_support'), \
                        patch('image_build.clean_identity'), \
                        patch('image_build.fsck'), \
                        patch('image_build.command', side_effect=simulated_command), \
                        patch('image_build.output', side_effect=simulated_info):
                    if succeeds:
                        image_build.build(state, key)
                    else:
                        with self.assertRaisesRegex(RuntimeError, 'SoapySX.*netcore'):
                            image_build.build(state, key)
                download.assert_not_called()
                self.assertEqual(cached_image.read_bytes(), b'generalized cache image')
                self.assertFalse((state / 'work' / key).exists())
                artifact = state / 'artifacts' / key
                if succeeds:
                    self.assertEqual(events, ['personalize', 'sdr', 'compress'])
                    self.assertEqual((artifact / 'image.img.xz').read_bytes(),
                                     b'compressed fixture')
                    manifest = json.loads((artifact / 'manifest.json').read_text())
                    self.assertEqual(manifest['id'], key)
                    self.assertEqual(manifest['commit'], request['commit'])
                else:
                    self.assertEqual(events, ['personalize', 'sdr'])
                    self.assertFalse(artifact.exists())

    def test_missing_installation_paths_fail_without_probe(self):
        # Progress through each incomplete installation, from missing root
        # directory to installed module directory with no plugin.
        self.assert_rejected_before_probe()
        (self.root / SDR_DIRECTORY).mkdir()
        self.assert_rejected_before_probe()
        (self.root / MODULE_DIRECTORY).mkdir()
        self.assert_rejected_before_probe()

    def test_wrong_filesystem_types_fail_without_probe(self):
        self.make_installation()
        for relative in (MODULE, MODULE_DIRECTORY, SDR_DIRECTORY):
            with self.subTest(path=str(relative)):
                if relative == MODULE:
                    (self.root / MODULE).unlink()
                    (self.root / MODULE).mkdir()
                    self.assert_rejected_before_probe()
                    (self.root / MODULE).rmdir()
                else:
                    (self.root / relative).rmdir()
                    (self.root / relative).write_bytes(b'not a directory')
                    self.assert_rejected_before_probe()
                    (self.root / relative).unlink()

    def test_symlinks_fail_without_changing_their_targets(self):
        # A plugin or its search directories must be the actual guest objects;
        # chmod must not follow an unexpected link to another location.
        for relative in (SDR_DIRECTORY, MODULE_DIRECTORY, MODULE):
            with self.subTest(path=str(relative)), \
                    tempfile.TemporaryDirectory() as fixture:
                self.root = Path(fixture)
                self.make_installation()
                original = self.root / relative
                target = original.with_name(original.name + '-target')
                original.rename(target)
                original.symlink_to(target, target_is_directory=target.is_dir())
                before = self.mode(target)
                self.assert_rejected_before_probe()
                self.assertTrue(original.is_symlink())
                self.assertEqual(self.mode(target), before)

    def test_exact_sx_factory_can_appear_among_other_factories(self):
        self.make_installation()
        info = INFO.replace('Available factories... sx',
                            'Available factories... audio, sx, rtlsdr')
        with patch('image_build.output', return_value=info) as output:
            image_build.guest_sdr(self.root)
        output.assert_called_once_with(self.expected_command())

    def test_missing_or_misleading_factory_output_fails(self):
        self.make_installation()
        invalid_outputs = (
            '',
            'Available factories... No factories found!',
            'Available factories... sx_other, xsx',
            'Module found: /usr/local/lib/SoapySDR/modules0.8/libSXSupport.so\n'
            'Available factories... audio',
            'Loader error: libSXSupport.so: cannot open shared object file\n'
            'Available factories... No factories found!',
            'Search path: /sx\nAvailable converters... sx',
        )
        for info in invalid_outputs:
            with self.subTest(info=info), \
                    patch('image_build.output', return_value=info):
                with self.assertRaisesRegex(RuntimeError, 'SoapySX.*netcore|netcore.*SoapySX'):
                    image_build.guest_sdr(self.root)

    def test_failed_guest_command_does_not_pass_validation(self):
        self.make_installation()
        error = subprocess.CalledProcessError(1, self.expected_command())
        with patch('image_build.output', side_effect=error):
            with self.assertRaises((RuntimeError, subprocess.CalledProcessError)):
                image_build.guest_sdr(self.root)


if __name__ == '__main__':
    unittest.main()
