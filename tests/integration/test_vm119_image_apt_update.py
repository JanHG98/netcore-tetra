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
    def test_only_guest_script_is_selected_with_reviewed_pin(self):
        driver = load_wrapper().UPDATER
        self.assertEqual(driver.TARGET, Path('/usr/local/lib/netcore-deployment/image/build-guest.sh'))
        self.assertEqual(driver.SOURCE, REPO / 'system-backend/deployment-core/image/build-guest.sh')
        self.assertRegex(driver.OLD_SHA, r'^[0-9a-f]{64}$')
        self.assertEqual(hashlib.sha256(driver.SOURCE.read_bytes()).hexdigest(), driver.NEW_SHA)
        driver.validate_source(driver.SOURCE.read_bytes())

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
        with mock.patch.object(FIXTURE, 'load_wrapper', load_wrapper):
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
