"""Pi 5 GPIO alias regressions without GPIO hardware or a real /dev mutation.

Temporary paths and symlinks are real; chip character-device metadata and ioctl
responses are simulated. Installation tests retain the real filesystem writes.
"""
from contextlib import contextmanager
import importlib.util
import os
from pathlib import Path
import stat
import struct
import sys
import tempfile
import unittest
from unittest.mock import patch


SOURCE = Path(__file__).resolve().parents[1] / 'image/gpio-compat.py'
SPEC = importlib.util.spec_from_file_location('netcore_gpio_compat_test', SOURCE)
gpio = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = gpio
SPEC.loader.exec_module(gpio)


class GpioCompatibilityTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name)
        self.dev = self.root / 'dev'
        self.dev.mkdir()

    def node(self, name):
        path = self.dev / name
        path.write_bytes(name.encode())
        return path

    @contextmanager
    def chips(self, metadata):
        """Keep real inodes for samefile, replacing only GPIO device type bits."""
        original_stat = Path.stat

        def simulated_stat(path, *args, **kwargs):
            result = original_stat(path, *args, **kwargs)
            if path in metadata and kwargs.get('follow_symlinks', True):
                values = list(result)
                values[0] = stat.S_IFCHR | stat.S_IMODE(result.st_mode)
                return os.stat_result(values)
            return result

        with patch.object(Path, 'stat', simulated_stat), \
                patch.object(gpio, 'chip_info', side_effect=lambda path: metadata[path]) as info:
            yield info

    def test_chip_info_reads_ioctl_and_always_closes_device(self):
        path = self.dev / 'gpiochip4'

        def chip_response(fd, request, buffer, *args):
            self.assertEqual(fd, 73)
            self.assertEqual(request, 0x8044B401)
            self.assertEqual(len(buffer), 68)
            return struct.pack('=32s32sI', b'gpiochip4', b'pinctrl-rp1', 54)

        with patch.object(gpio.os, 'open', return_value=73) as opening, \
                patch.object(gpio.os, 'close') as closing, \
                patch.object(gpio.fcntl, 'ioctl', side_effect=chip_response):
            self.assertEqual(gpio.chip_info(path), ('pinctrl-rp1', 54))
        self.assertEqual(Path(opening.call_args.args[0]), path)
        self.assertEqual(opening.call_args.args[1] & os.O_ACCMODE, os.O_RDONLY)
        closing.assert_called_once_with(73)
        with patch.object(gpio.os, 'open', return_value=73), \
                patch.object(gpio.os, 'close') as closing, \
                patch.object(gpio.fcntl, 'ioctl', side_effect=OSError('ioctl failed')):
            with self.assertRaises(OSError):
                gpio.chip_info(path)
        closing.assert_called_once_with(73)

    def test_discovery_requires_one_native_rp1_chip_with_54_lines(self):
        selected = self.node('gpiochip4')
        unrelated = self.node('gpiochip1')
        wrong_lines = self.node('gpiochip2')
        regular_file = self.node('gpiochip3')
        link = self.dev / 'gpiochip99'
        link.symlink_to(selected)
        metadata = {selected: ('pinctrl-rp1', 54), unrelated: ('soc-gpio', 32),
                    wrong_lines: ('pinctrl-rp1', 53)}
        with self.chips(metadata) as info:
            self.assertEqual(gpio.rp1_node(self.dev), selected)
        seen = {call.args[0] for call in info.call_args_list}
        self.assertNotIn(regular_file, seen)
        self.assertNotIn(link, seen)
        self.assertIn(selected, seen)

        metadata[selected] = ('pinctrl-rp1', 53)
        with self.chips(metadata):
            with self.assertRaises(RuntimeError):
                gpio.rp1_node(self.dev)
        metadata[selected] = ('pinctrl-rp1', 54)
        metadata[wrong_lines] = ('pinctrl-rp1', 54)
        with self.chips(metadata):
            with self.assertRaises(RuntimeError):
                gpio.rp1_node(self.dev)

    def test_alias_creation_and_existing_correct_paths(self):
        selected = self.node('gpiochip4')
        alias = self.dev / 'gpiochip0'
        with self.chips({selected: ('pinctrl-rp1', 54)}):
            self.assertEqual(gpio.ensure_alias(self.dev), selected)
            self.assertTrue(alias.is_symlink())
            self.assertEqual(os.readlink(alias), str(selected))
            self.assertTrue(Path(os.readlink(alias)).is_absolute())
            self.assertEqual(gpio.ensure_alias(self.dev), selected)
            self.assertTrue(os.path.samefile(alias, selected))

        alias.unlink()
        selected.rename(alias)
        with self.chips({alias: ('pinctrl-rp1', 54)}):
            self.assertEqual(gpio.ensure_alias(self.dev), alias)
        self.assertFalse(alias.is_symlink())
        self.assertEqual(alias.read_bytes(), b'gpiochip4')

    def test_conflicting_alias_is_never_overwritten(self):
        selected = self.node('gpiochip4')
        alias = self.node('gpiochip0')
        with self.chips({selected: ('pinctrl-rp1', 54), alias: ('other-gpio', 32)}):
            with self.assertRaises(RuntimeError):
                gpio.ensure_alias(self.dev)
        self.assertEqual(alias.read_bytes(), b'gpiochip0')
        self.assertFalse(alias.is_symlink())

        alias.unlink()
        for target in (self.dev / 'missing', self.node('other-device')):
            with self.subTest(target=target.name):
                alias.symlink_to(target)
                with self.chips({selected: ('pinctrl-rp1', 54)}):
                    with self.assertRaises(RuntimeError):
                        gpio.ensure_alias(self.dev)
                self.assertTrue(alias.is_symlink())
                self.assertEqual(os.readlink(alias), str(target))
                alias.unlink()

    def test_install_permissions_ordering_and_conflict_preflight(self):
        destinations = (
            Path('usr/local/lib/netcore-image/gpio-compat.py'),
            Path('etc/systemd/system/netcore-sx-gpio-compat.service'),
            Path('etc/systemd/system/tetra.service.d/10-gpio-compat.conf'),
        )
        previous_umask = os.umask(0o077)
        try:
            gpio.install(self.root, source=SOURCE)
        finally:
            os.umask(previous_umask)
        script, unit, dropin = [self.root / path for path in destinations]
        self.assertEqual(script.read_bytes(), SOURCE.read_bytes())
        self.assertEqual(stat.S_IMODE(script.stat().st_mode), 0o755)
        self.assertEqual(stat.S_IMODE(unit.stat().st_mode), 0o644)
        self.assertEqual(stat.S_IMODE(dropin.stat().st_mode), 0o644)
        self.assertIn('Before=tetra.service', unit.read_text())
        self.assertIn('Requires=netcore-sx-gpio-compat.service', dropin.read_text())
        self.assertIn('After=netcore-sx-gpio-compat.service', dropin.read_text())
        self.assertIn('/usr/local/lib/netcore-image/gpio-compat.py', unit.read_text())

        for conflict in destinations:
            with self.subTest(conflict=str(conflict)), \
                    tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                custom = root / conflict
                custom.parent.mkdir(parents=True)
                custom.write_bytes(b'custom site configuration\n')
                custom.chmod(0o600)
                with self.assertRaises(RuntimeError):
                    gpio.install(root, source=SOURCE)
                self.assertEqual(custom.read_bytes(), b'custom site configuration\n')
                self.assertEqual(stat.S_IMODE(custom.stat().st_mode), 0o600)
                for untouched in destinations:
                    if untouched != conflict:
                        self.assertFalse((root / untouched).exists())

    def test_main_guards_models_root_and_bounded_hardware_retry(self):
        model = self.root / 'model'
        with patch.object(gpio, 'MODEL_PATH', model), \
                patch.object(gpio.os, 'geteuid', return_value=0):
            for supported in ('Raspberry Pi 5 Model B Rev 1.0',
                              'Raspberry Pi Compute Module 5 Rev 1.0'):
                with self.subTest(model=supported):
                    model.write_bytes(supported.encode() + b'\0')
                    with patch.object(gpio, 'ensure_alias') as alias:
                        gpio.main([])
                    alias.assert_called_once_with()
            for legacy in ('Raspberry Pi 4 Model B Rev 1.5',
                           'Raspberry Pi Compute Module 4 Rev 1.1', ''):
                with self.subTest(model=legacy):
                    model.write_bytes(legacy.encode() + b'\0')
                    with patch.object(gpio, 'ensure_alias') as alias:
                        gpio.main([])
                    alias.assert_not_called()
            model.unlink()
            with patch.object(gpio, 'ensure_alias') as alias:
                gpio.main([])
            alias.assert_not_called()

            with patch.object(gpio, 'install') as installing, \
                    patch.object(gpio, 'ensure_alias') as alias:
                gpio.main(['--install'])
            installing.assert_called_once_with()
            alias.assert_not_called()

            model.write_bytes(b'Raspberry Pi 5 Model B Rev 1.0\0')
            with patch.object(gpio, 'ensure_alias',
                              side_effect=[gpio.RP1NotReady('udev pending'), None]) as alias, \
                    patch.object(gpio.time, 'monotonic', side_effect=[100, 101]), \
                    patch.object(gpio.time, 'sleep') as sleeping:
                gpio.main([])
            self.assertEqual(alias.call_count, 2)
            sleeping.assert_called_once_with(0.25)

            with patch.object(gpio, 'ensure_alias',
                              side_effect=gpio.RP1NotReady('RP1 absent')) as alias, \
                    patch.object(gpio.time, 'monotonic', side_effect=[100, 101, 116]), \
                    patch.object(gpio.time, 'sleep') as sleeping:
                with self.assertRaisesRegex(gpio.RP1NotReady, 'RP1 absent'):
                    gpio.main([])
            self.assertEqual(alias.call_count, 2)
            sleeping.assert_called_once_with(0.25)

            with patch.object(gpio, 'ensure_alias',
                              side_effect=RuntimeError('conflicting alias')) as alias, \
                    patch.object(gpio.time, 'sleep') as sleeping:
                with self.assertRaisesRegex(RuntimeError, 'conflicting alias'):
                    gpio.main([])
            alias.assert_called_once_with()
            sleeping.assert_not_called()

        with patch.object(gpio.os, 'geteuid', return_value=1000), \
                patch.object(gpio, 'install') as installing, \
                patch.object(gpio, 'ensure_alias') as alias:
            for args in ([], ['--install']):
                with self.subTest(args=args):
                    with self.assertRaisesRegex(RuntimeError, 'Root'):
                        gpio.main(args)
            installing.assert_not_called()
            alias.assert_not_called()


if __name__ == '__main__':
    unittest.main()
