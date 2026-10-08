"""Initramfs wrapper checks with the existing real atomic-swap operator fixture."""
import hashlib
import importlib.util
from pathlib import Path
import unittest
from unittest import mock

REPO = Path(__file__).resolve().parents[2]
WRAPPER = REPO / 'Docs/integration/Z01-2026-10-07/vm119-image-initramfs-update.py'


def load(path, name):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


FIXTURE = load(Path(__file__).with_name('test_vm119_image_apt_update.py'), 'vm119_guest_fixture')


def load_wrapper():
    return load(WRAPPER, 'vm119_image_initramfs')


class ImageInitramfsUpdateTests(unittest.TestCase):
    def reuse(self, method):
        with mock.patch.object(FIXTURE, 'load_wrapper', load_wrapper):
            getattr(FIXTURE.ImageAptUpdateTests, method)(self)

    def exercise(self, kind):
        self.reuse_atomic(kind)

    def reuse_atomic(self, kind):
        with mock.patch.object(FIXTURE, 'load_wrapper', load_wrapper):
            FIXTURE.ImageAptUpdateTests.exercise(self, kind)

    def test_target_and_source_are_only_the_pinned_guest_recipe(self):
        driver = load_wrapper().UPDATER
        self.assertEqual(driver.TARGET, Path('/usr/local/lib/netcore-deployment/image/build-guest.sh'))
        self.assertEqual(driver.SOURCE, REPO / 'system-backend/deployment-core/image/build-guest.sh')
        self.assertEqual(driver.OLD_SHA, '84b8d4070730b2fe94c4ddf4514411fadd08f19f72bfb3ce0adc86f14d1d8c3c')
        self.assertEqual(hashlib.sha256(driver.SOURCE.read_bytes()).hexdigest(), driver.NEW_SHA)
        driver.validate_source(driver.SOURCE.read_bytes())

    def test_valid_and_invalid_bash_without_executing_guest_commands(self):
        self.reuse('test_bash_validation_rejects_syntax_without_executing_guest_script')

    def test_atomic_source_swap_preserves_permissions_and_configuration(self):
        self.exercise('success')

    def test_failed_update_restores_actual_original_recipe_bytes(self):
        self.exercise('rollback')

    def test_busy_builder_stops_before_source_or_units_are_changed(self):
        self.exercise('busy')

    def test_wrong_machine_refused_without_commands(self):
        self.reuse('test_inherited_host_guard_refuses_wrong_machine')

    def test_nested_target_keeps_fixed_image_client_library_root(self):
        self.reuse('test_image_client_uses_library_root_for_nested_script_target')


if __name__ == '__main__':
    unittest.main()
