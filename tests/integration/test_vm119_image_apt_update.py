"""Bash wrapper regression, reusing the real atomic-swap operator fixture."""
import hashlib
import importlib.util
from pathlib import Path
import subprocess
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest import mock

REPO = Path(__file__).resolve().parents[2]
WRAPPER = REPO / 'Docs/integration/Z01-2026-10-07/vm119-image-apt-update.py'


def load(path, name):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


FIXTURE = load(Path(__file__).with_name('test_vm119_image_dns_update.py'), 'vm119_atomic_fixture')


def load_wrapper():
    return load(WRAPPER, 'vm119_image_apt')


class ImageAptUpdateTests(unittest.TestCase):
    def test_historical_wrapper_keeps_its_reviewed_target_and_pins(self):
        driver = load_wrapper().UPDATER
        self.assertEqual(driver.TARGET, Path('/usr/local/lib/netcore-deployment/image/build-guest.sh'))
        self.assertEqual(driver.SOURCE, REPO / 'system-backend/deployment-core/image/build-guest.sh')
        self.assertEqual(driver.OLD_SHA, 'de425150d67bb9e778ffdff9c10f1c548a1ca68a396b00955cc20f0ccf9fb40b')
        self.assertEqual(driver.NEW_SHA, '84b8d4070730b2fe94c4ddf4514411fadd08f19f72bfb3ce0adc86f14d1d8c3c')
        driver.validate_source(driver.SOURCE.read_bytes())

    def test_historical_source_mismatch_refused_before_units_or_file_change(self):
        driver = load_wrapper().UPDATER
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            driver.TARGET = directory / 'installed.sh'
            driver.SOURCE = directory / 'unreviewed.sh'
            original = b'#!/bin/bash\necho original\n'
            driver.TARGET.write_bytes(original)
            driver.SOURCE.write_bytes(b'#!/bin/bash\necho unreviewed\n')
            with mock.patch.object(driver, 'command') as command:
                with self.assertRaisesRegex(RuntimeError, 'Quellfingerprint'):
                    driver.update(directory, [])
                command.assert_not_called()
            self.assertEqual(driver.TARGET.read_bytes(), original)

    def test_image_client_uses_library_root_for_nested_script_target(self):
        driver = load_wrapper().UPDATER
        client = mock.Mock()
        client.request.return_value = {'available': True, 'jobs': []}
        module = SimpleNamespace(ImageClient=lambda: client)
        with mock.patch.dict(sys.modules, {'image_client': module}), mock.patch.object(sys, 'path', []):
            self.assertTrue(driver.builder_status()['available'])
            self.assertEqual(sys.path[0], '/usr/local/lib/netcore-deployment')
        client.request.assert_called_once_with('/status')

    def test_bash_validation_rejects_syntax_without_executing_guest_script(self):
        driver = load_wrapper().UPDATER
        with self.assertRaises(subprocess.CalledProcessError):
            driver.validate_source(b'if true; then\n')
        with tempfile.TemporaryDirectory() as temporary:
            marker = Path(temporary) / 'must-not-exist'
            driver.validate_source(('touch ' + str(marker) + '\nexit 99\n').encode())
            self.assertFalse(marker.exists())

    def exercise(self, kind):
        def fixture_wrapper():
            wrapper = load_wrapper()
            # Atomic-swap fixtures exercise the guard; historical production pins stay unchanged.
            wrapper.UPDATER.NEW_SHA = hashlib.sha256(wrapper.UPDATER.SOURCE.read_bytes()).hexdigest()
            return wrapper
        with mock.patch.object(FIXTURE, 'load_wrapper', fixture_wrapper):
            FIXTURE.ImageDnsUpdateTests.exercise(self, kind)

    def test_atomic_real_script_swap_preserves_permissions_and_configs(self):
        self.exercise('success')

    def test_atomic_real_script_rollback_restores_original_bytes(self):
        self.exercise('rollback')

    def test_busy_builder_refuses_before_file_or_units_change(self):
        self.exercise('busy')

    def test_inherited_host_guard_refuses_wrong_machine(self):
        driver = load_wrapper().UPDATER
        with mock.patch.object(driver.sys, 'argv', [str(WRAPPER)]), \
                mock.patch.object(driver.os, 'geteuid', return_value=0), \
                mock.patch.object(driver.socket, 'gethostname', return_value='wrong-host'), \
                mock.patch.object(driver, 'command') as command:
            with self.assertRaises(RuntimeError):
                driver.main()
            command.assert_not_called()


if __name__ == '__main__':
    unittest.main()
