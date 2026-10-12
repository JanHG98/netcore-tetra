"""Behavioral tests for the VM119 image-features transaction.

Run with ``python -m unittest discover -s tests/integration -p
test_vm119_image_features_update.py``. Service control and HTTP are simulated;
the files, SQLite job databases, replacements and rollback are real. On
Windows only POSIX ownership/directory sync and unavailable Bash are isolated.
"""
from contextlib import ExitStack, closing
import hashlib
import importlib.util
import os
from pathlib import Path
import shutil
import sqlite3
import stat
import subprocess
import sys
import tempfile
import unittest
from unittest import mock


WORKSPACE = Path(__file__).resolve().parents[2]
SCRIPT = WORKSPACE / "Docs/integration/Z01-2026-10-07/vm119-image-features-update.py"


def digest(body):
    return hashlib.sha256(body).hexdigest()


def load_updater():
    name = "vm119_image_features_update_under_test"
    spec = importlib.util.spec_from_file_location(name, SCRIPT)
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


class ImageFeaturesUpdateTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.updater = load_updater()
        cls.source_manifest = cls.updater.PINS.copy()
        cls.source_root = cls.updater.SOURCE

    def setUp(self):
        self.stack = ExitStack()
        self.addCleanup(self.stack.close)
        self.root = Path(self.stack.enter_context(tempfile.TemporaryDirectory()))
        self.library = self.root / "installed"
        self.source = self.root / "reviewed"
        self.state = self.root / "controller-state"
        self.build_state = self.root / "builder-state"
        for directory in (self.library, self.source, self.state, self.build_state):
            directory.mkdir()
        (self.library / "image").mkdir()
        (self.source / "image").mkdir()
        self.original = {
            "image_build.py": b"IMAGE_VERSION = 'old'\n",
            "image_spec.py": b"DESKTOP_OPTIONS = ('none',)\n",
            "image/personalize.py": b"PERSONALIZE_VERSION = 'old'\n",
            "image/build-guest.sh": b"#!/bin/bash\nset -eu\nprintf 'old\\n'\n",
            "image/systemd/tetra.service": b"[Unit]\nDescription=old\n[Service]\nUser=netcore\nExecStart=/usr/local/bin/bluestation-bs /etc/netcore/config.toml\n[Install]\nWantedBy=multi-user.target\n",
            "image/desktop-setup.sh": None,
            "image/sip_setup.py": None,
            "image/49-netcore-networkmanager.rules": None,
            "static/index.html": b"<!doctype html><html><head></head><body><form id=\"image-form\"></form><script src=\"/app.js\"></script></body></html>",
            "static/app.js": b"'use strict'; const version = 'old';\n",
        }
        self.reviewed = {
            "image_build.py": b"IMAGE_VERSION = 'reviewed'\n",
            "image_spec.py": b"DESKTOP_OPTIONS = ('kde', 'none')\n",
            "image/personalize.py": b"PERSONALIZE_VERSION = 'reviewed'\n",
            "image/build-guest.sh": b"#!/bin/bash\nset -eu\nprintf 'reviewed\\n'\n",
            "image/systemd/tetra.service": b"[Unit]\nDescription=reviewed\n[Service]\nUser=netcore\nExecStart=/usr/local/bin/bluestation-bs /etc/netcore/config.toml\n[Install]\nWantedBy=multi-user.target\n",
            "image/desktop-setup.sh": b"#!/bin/bash\nset -eu\nprintf 'desktop\\n'\n",
            "image/sip_setup.py": b"SIP_PLATFORM = 'pi'\n",
            "image/49-netcore-networkmanager.rules": b"polkit.addRule(function(action, subject) { if (subject.user !== 'netcore') { return; } });\n",
            "static/index.html": b"<!doctype html><html><head></head><body><form id=\"image-form\"><select id=\"image-desktop\"><option value=\"kde\">KDE</option></select></form><script src=\"/app.js\"></script></body></html>",
            "static/app.js": b"'use strict'; const version = 'reviewed';\n",
        }
        self.pins = {}
        self.metadata = {}
        for name, before in self.original.items():
            target = self.library / name
            target.parent.mkdir(parents=True, exist_ok=True)
            (self.source / name).parent.mkdir(parents=True, exist_ok=True)
            if before is not None:
                target.write_bytes(before)
                target.chmod(0o750 if name.endswith(".sh") else 0o640)
                self.metadata[name] = target.stat()
            (self.source / name).write_bytes(self.reviewed[name])
            self.pins[name] = {"old": [digest(before) if before is not None else None],
                               "new": digest(self.reviewed[name])}
        self.worker = self.library / "image_worker.py"
        self.worker_bytes = b"WORKER_VERSION = 'unchanged'\n"
        self.worker.write_bytes(self.worker_bytes)
        self.gpio = self.library / "image/gpio-compat.py"
        self.gpio_bytes = b"GPIO_PLATFORM = 'unchanged'\n"
        self.gpio.write_bytes(self.gpio_bytes)
        self.protected = [
            self.root / "deployment.toml", self.state / "settings.json",
            self.state / "profiles.json", self.state / "tbs-site-template.toml",
        ]
        for index, path in enumerate(self.protected):
            path.write_bytes(f"protected fixture {index}\n".encode())
        for directory in (self.state, self.build_state):
            with closing(sqlite3.connect(directory / "jobs.sqlite3")) as database:
                with database:
                    database.execute("CREATE TABLE jobs (id TEXT, status TEXT, request TEXT, result TEXT)")
        self.artifacts = self.build_state / "artifacts"
        self.artifacts.mkdir()
        self.artifact = self.artifacts / "existing.img.xz"
        self.artifact.write_bytes(b"existing image payload")
        self.cache = self.build_state / "cache"
        self.cache.mkdir()
        (self.cache / "base-image.img").write_bytes(b"existing base image")
        self.events = []
        self.controller_jobs = []
        self.builder_jobs = []
        self.active = {self.updater.GUARD.CONTROLLER: True,
                       self.updater.GUARD.BUILDER: True}
        self.on_command = None
        for attr, value in (("LIBRARY", self.library), ("SOURCE", self.source),
                            ("PINS", self.pins), ("INVARIANTS", {"image_worker.py": digest(self.worker_bytes), "image/gpio-compat.py": digest(self.gpio_bytes)})):
            self.stack.enter_context(mock.patch.object(self.updater, attr, value))
        guard = self.updater.GUARD
        self.stack.enter_context(mock.patch.object(guard, "LIBRARY", self.library))
        self.stack.enter_context(mock.patch.object(guard, "BUILD_STATE", self.build_state))
        self.stack.enter_context(mock.patch.object(guard, "api", side_effect=self.api))
        self.stack.enter_context(mock.patch.object(guard, "builder_status", side_effect=self.builder_status))
        self.stack.enter_context(mock.patch.object(guard, "command", side_effect=self.command))
        # New GPIO files are root-owned in production. Unprivileged Linux CI
        # can use its own uid/gid without stubbing the atomic file operations.
        if os.name != "nt" and os.geteuid() != 0:
            attributes = self.updater.attributes

            def runner_attributes(metadata):
                return (os.getuid(), os.getgid(), 0o644) if metadata is None else attributes(metadata)

            self.stack.enter_context(mock.patch.object(self.updater, "attributes", side_effect=runner_attributes))
        if os.name == "nt":
            self.stack.enter_context(mock.patch.object(self.updater, "sync_directory"))
            self.stack.enter_context(mock.patch.object(os, "fchown", create=True))
            attributes = self.updater.attributes

            def windows_attributes(metadata):
                # Windows exposes a writable regular file as 0666 regardless
                # of POSIX group/other write bits passed to fchmod.
                return (0, 0, 0o666) if metadata is None else attributes(metadata)

            self.stack.enter_context(mock.patch.object(self.updater, "attributes", side_effect=windows_attributes))

    def api(self, path):
        if path == "/health/ready":
            return {"ready": True}
        if path == "/api/v1/jobs":
            return self.controller_jobs
        raise AssertionError(f"Unexpected API path: {path}")

    def builder_status(self):
        return {"available": True, "jobs": self.builder_jobs}

    def command(self, *args):
        self.events.append(args)
        if self.on_command is not None:
            self.on_command(args)
        if args[:2] == ("bash", "-n"):
            if os.name != "nt":
                subprocess.run(args, check=True, timeout=10)
            return
        if args[:2] == ("systemctl", "stop"):
            self.active[args[2]] = False
            return
        if args[:2] == ("systemctl", "start"):
            self.active[args[2]] = True
            return
        if args[:3] == ("systemctl", "is-active", "--quiet"):
            if not all(self.active[service] for service in args[3:]):
                raise subprocess.CalledProcessError(3, args)
            return
        raise AssertionError(f"Unexpected system command: {args}")

    def service_mutations(self):
        return [event for event in self.events if event[:2] in
                (("systemctl", "stop"), ("systemctl", "start"))]

    def assert_original_files(self):
        for name, before in self.original.items():
            target = self.library / name
            if before is None:
                self.assertFalse(target.exists(), name)
            else:
                self.assertEqual(target.read_bytes(), before, name)
                self.assertEqual(stat.S_IMODE(target.stat().st_mode),
                                 stat.S_IMODE(self.metadata[name].st_mode), name)

    def assert_untouched_payloads(self):
        self.assertEqual(self.worker.read_bytes(), self.worker_bytes)
        self.assertEqual(self.gpio.read_bytes(), self.gpio_bytes)
        self.assertEqual(self.artifact.read_bytes(), b"existing image payload")
        self.assertEqual((self.cache / "base-image.img").read_bytes(), b"existing base image")
        self.assertEqual(list(self.artifacts.iterdir()), [self.artifact])

    def put_running_builder_job(self):
        self.builder_jobs = [{"id": "image-racing", "status": "running",
                              "request": {"kind": "image_build"}, "result": {}}]
        with closing(sqlite3.connect(self.build_state / "jobs.sqlite3")) as database:
            with database:
                database.execute("INSERT INTO jobs VALUES (?, ?, ?, ?)",
                                 ("image-racing", "running", '{"kind":"image_build"}', '{}'))

    def test_success_adds_helpers_and_preserves_config_modes_cache_and_artifacts(self):
        protected_before = {path: path.read_bytes() for path in self.protected}
        result = self.updater.update(self.state, self.protected)
        self.assertEqual(result["phase"], "passed")
        self.assertEqual(len(result["changed_files"]), len(self.reviewed))
        for name, body in self.reviewed.items():
            target = self.library / name
            self.assertEqual(target.read_bytes(), body)
            if name in self.metadata:
                self.assertEqual(stat.S_IMODE(target.stat().st_mode),
                                 stat.S_IMODE(self.metadata[name].st_mode))
        if os.name != "nt":
            self.assertEqual(stat.S_IMODE((self.library / "image/sip_setup.py").stat().st_mode), 0o644)
        self.assertEqual(self.service_mutations(), [
            ("systemctl", "stop", self.updater.GUARD.CONTROLLER),
            ("systemctl", "stop", self.updater.GUARD.BUILDER),
            ("systemctl", "start", self.updater.GUARD.BUILDER),
            ("systemctl", "start", self.updater.GUARD.CONTROLLER),
        ])
        self.assertTrue(all(self.active.values()))
        self.assertEqual({path: path.read_bytes() for path in self.protected}, protected_before)
        self.assertEqual(result["protected_sha256"], self.updater.GUARD.snapshot(self.protected))
        self.assertIn(("bash", "-n", str(self.source / "image/build-guest.sh")), self.events)
        self.assert_untouched_payloads()

    def test_already_current_does_not_restart_services(self):
        for name, body in self.reviewed.items():
            (self.library / name).write_bytes(body)
        result = self.updater.update(self.state, self.protected)
        self.assertEqual(result["phase"], "already_current")
        self.assertEqual(result["changed_files"], [])
        self.assertEqual(self.service_mutations(), [])
        self.assert_untouched_payloads()

    def test_unexpected_installed_recipe_fails_before_stop(self):
        (self.library / "image_build.py").write_bytes(b"UNEXPECTED = True\n")
        with self.assertRaises(RuntimeError):
            self.updater.update(self.state, self.protected)
        self.assertEqual(self.service_mutations(), [])
        self.assertFalse((self.library / "image/sip_setup.py").exists())

    def test_unexpected_reviewed_source_fails_before_stop(self):
        (self.source / "image/sip_setup.py").write_bytes(b"UNREVIEWED = True\n")
        with self.assertRaises(RuntimeError):
            self.updater.update(self.state, self.protected)
        self.assertEqual(self.service_mutations(), [])
        self.assert_original_files()

    def test_changed_worker_fails_before_stop(self):
        self.worker.write_bytes(b"UNEXPECTED_WORKER = True\n")
        with self.assertRaises(RuntimeError):
            self.updater.update(self.state, self.protected)
        self.assertEqual(self.service_mutations(), [])
        self.assert_original_files()

    def test_invalid_python_source_fails_before_stop_even_with_matching_digest(self):
        invalid = b"def broken(:\n"
        (self.source / "image/sip_setup.py").write_bytes(invalid)
        self.pins["image/sip_setup.py"]["new"] = digest(invalid)
        with self.assertRaises((SyntaxError, RuntimeError)):
            self.updater.update(self.state, self.protected)
        self.assertEqual(self.service_mutations(), [])
        self.assert_original_files()

    @unittest.skipIf(os.name == "nt", "Bash is unavailable in the Windows execution sandbox")
    def test_invalid_bash_source_fails_before_stop_even_with_matching_digest(self):
        invalid = b"#!/bin/bash\nif then\n"
        (self.source / "image/build-guest.sh").write_bytes(invalid)
        self.pins["image/build-guest.sh"]["new"] = digest(invalid)
        with self.assertRaises((RuntimeError, subprocess.CalledProcessError)):
            self.updater.update(self.state, self.protected)
        self.assertEqual(self.service_mutations(), [])
        self.assert_original_files()

    def test_installed_symlink_is_rejected_before_stop(self):
        target = self.library / "image_build.py"
        original = target.read_bytes()
        external = self.root / "external.py"
        external.write_bytes(original)
        target.unlink()
        try:
            target.symlink_to(external)
        except OSError as error:
            self.skipTest(f"Symlinks unavailable for this runner: {error}")
        with self.assertRaises(RuntimeError):
            self.updater.update(self.state, self.protected)
        self.assertEqual(self.service_mutations(), [])
        self.assertEqual(external.read_bytes(), original)

    def test_active_builder_job_fails_before_stop(self):
        self.put_running_builder_job()
        with self.assertRaises(RuntimeError):
            self.updater.update(self.state, self.protected)
        self.assertEqual(self.service_mutations(), [])
        self.assert_original_files()

    def test_racing_builder_job_after_controller_stop_prevents_file_mutation(self):
        def race(args):
            if args == ("systemctl", "stop", self.updater.GUARD.CONTROLLER):
                self.put_running_builder_job()
        self.on_command = race
        with self.assertRaises(RuntimeError):
            self.updater.update(self.state, self.protected)
        self.assert_original_files()
        self.assertIn(("systemctl", "stop", self.updater.GUARD.CONTROLLER), self.events)
        self.assertNotIn(("systemctl", "stop", self.updater.GUARD.BUILDER), self.events)
        self.assertTrue(all(self.active.values()))

    def test_second_replacement_failure_rolls_back_prior_replacement(self):
        replace = self.updater.replace
        fail_once = [True]
        def injected(path, body, metadata):
            if path == self.library / "image/build-guest.sh" and body == self.reviewed["image/build-guest.sh"] and fail_once[0]:
                fail_once[0] = False
                raise OSError("injected replacement failure")
            return replace(path, body, metadata)
        with mock.patch.object(self.updater, "replace", side_effect=injected):
            with self.assertRaises(OSError):
                self.updater.update(self.state, self.protected)
        self.assert_original_files()
        self.assertTrue(all(self.active.values()))
        self.assert_untouched_payloads()

    def test_fsync_failure_after_new_helper_publication_removes_new_file_on_rollback(self):
        sync_directory = self.updater.sync_directory
        fail_once = [True]
        gpio = self.library / "image/sip_setup.py"
        def injected(path):
            sync_directory(path)
            if path == gpio.parent and gpio.exists() and gpio.read_bytes() == self.reviewed["image/sip_setup.py"] and fail_once[0]:
                fail_once[0] = False
                raise OSError("injected directory fsync failure after publication")
        with mock.patch.object(self.updater, "sync_directory", side_effect=injected):
            with self.assertRaises(OSError):
                self.updater.update(self.state, self.protected)
        self.assert_original_files()
        self.assertTrue(all(self.active.values()))
        self.assert_untouched_payloads()

    def test_controller_start_failure_restores_all_payload_files(self):
        fail_once = [True]
        def fail_start(args):
            if args == ("systemctl", "start", self.updater.GUARD.CONTROLLER) and fail_once[0]:
                fail_once[0] = False
                raise subprocess.CalledProcessError(1, args)
        self.on_command = fail_start
        with self.assertRaises(subprocess.CalledProcessError):
            self.updater.update(self.state, self.protected)
        self.assert_original_files()
        self.assertTrue(all(self.active.values()))
        self.assert_untouched_payloads()

    def test_protected_config_change_before_mutation_aborts_and_restarts_services(self):
        before = self.protected[0].read_bytes()
        def external_change(args):
            if args == ("systemctl", "stop", self.updater.GUARD.BUILDER):
                self.protected[0].write_bytes(before + b"external concurrent edit\n")
        self.on_command = external_change
        with self.assertRaises(RuntimeError):
            self.updater.update(self.state, self.protected)
        self.assert_original_files()
        self.assertTrue(all(self.active.values()))
        # The transaction must neither overwrite nor roll back another actor's
        # configuration edit when it detects the protected-file race.
        self.assertEqual(self.protected[0].read_bytes(), before + b"external concurrent edit\n")

    def alter_source(self, name, body):
        (self.source / name).write_bytes(body)
        self.pins[name]["new"] = digest(body)

    def test_unfinished_source_pin_fails_before_service_stop(self):
        self.pins["image_spec.py"]["new"] = "__UNFINISHED_PIN__"
        with self.assertRaisesRegex(RuntimeError, "nicht final"):
            self.updater.update(self.state, self.protected)
        self.assertEqual(self.service_mutations(), [])
        self.assert_original_files()

    def test_gpio_invariant_is_checked_independently_from_new_helpers(self):
        self.gpio.write_bytes(b"unexpected gpio content")
        with self.assertRaises(RuntimeError):
            self.updater.update(self.state, self.protected)
        self.assertEqual(self.service_mutations(), [])
        self.assert_original_files()

    def test_invalid_html_with_matching_pin_fails_before_stop(self):
        self.alter_source("static/index.html", b"<html><head></head><body><form></body></html>")
        with self.assertRaisesRegex(RuntimeError, "HTML"):
            self.updater.update(self.state, self.protected)
        self.assertEqual(self.service_mutations(), [])
        self.assert_original_files()

    def test_missing_desktop_control_with_matching_pin_fails_before_stop(self):
        self.alter_source("static/index.html", self.original["static/index.html"])
        with self.assertRaisesRegex(RuntimeError, "Desktopfeld"):
            self.updater.update(self.state, self.protected)
        self.assertEqual(self.service_mutations(), [])
        self.assert_original_files()

    def test_invalid_guest_unit_with_matching_pin_fails_before_stop(self):
        self.alter_source("image/systemd/tetra.service", b"[Unit]\nDescription=bad\n[Service]\nUser=root\n[Install]\nWantedBy=multi-user.target\n")
        with self.assertRaisesRegex(RuntimeError, "Gast-TBS"):
            self.updater.update(self.state, self.protected)
        self.assertEqual(self.service_mutations(), [])
        self.assert_original_files()

    @unittest.skipUnless(shutil.which("node"), "Native CI validates JavaScript when Node is unavailable locally")
    def test_invalid_javascript_and_policy_matching_pins_fail_before_stop(self):
        for name in ("static/app.js", "image/49-netcore-networkmanager.rules"):
            with self.subTest(file=name):
                self.alter_source(name, b"const invalid = ;\n")
                with self.assertRaises(subprocess.CalledProcessError):
                    self.updater.update(self.state, self.protected)
                self.assertEqual(self.service_mutations(), [])
                self.assert_original_files()
                self.alter_source(name, self.reviewed[name])

    def test_node_absence_requires_no_install_and_is_reported_explicitly(self):
        with mock.patch.object(self.updater.shutil, "which", return_value=None):
            result = self.updater.update(self.state, self.protected)
        self.assertEqual(result["phase"], "passed")
        for name in ("static/app.js", "image/49-netcore-networkmanager.rules"):
            self.assertIn("nativer CI", result["validation"][name])
        self.assertTrue(all(self.active.values()))
        self.assert_untouched_payloads()

    def test_all_new_helpers_are_removed_after_late_restart_failure(self):
        failed = [False]
        def fail_start(args):
            if args == ("systemctl", "start", self.updater.GUARD.CONTROLLER) and not failed[0]:
                failed[0] = True
                raise subprocess.CalledProcessError(1, args)
        self.on_command = fail_start
        with self.assertRaises(subprocess.CalledProcessError):
            self.updater.update(self.state, self.protected)
        for name, original in self.original.items():
            if original is None:
                self.assertFalse((self.library / name).exists(), name)
        self.assert_original_files()
        self.assertTrue(all(self.active.values()))
        self.assert_untouched_payloads()

    def test_failed_but_remote_uncertain_job_blocks_update(self):
        self.controller_jobs = [{"id":"uncertain", "status":"failed",
                                 "request":{"kind":"deploy"}, "result":{"remote_uncertain":True}}]
        with self.assertRaisesRegex(RuntimeError, "Remote"):
            self.updater.update(self.state, self.protected)
        self.assertEqual(self.service_mutations(), [])
        self.assert_original_files()

    def test_database_only_active_job_blocks_even_when_http_lists_are_idle(self):
        with closing(sqlite3.connect(self.build_state / "jobs.sqlite3")) as database:
            with database:
                database.execute("INSERT INTO jobs VALUES (?, ?, ?, ?)",
                                 ("db-only", "running", '{"kind":"image_build"}', '{}'))
        with self.assertRaisesRegex(RuntimeError, "Aktiver Auftrag"):
            self.updater.update(self.state, self.protected)
        self.assertEqual(self.service_mutations(), [])
        self.assert_original_files()

    def test_foreign_protected_edit_after_restart_is_preserved_during_rollback(self):
        original = self.protected[1].read_bytes()
        edited = original + b"external edit after restart\n"
        once = [False]
        def edit(args):
            if args == ("systemctl", "start", self.updater.GUARD.CONTROLLER) and not once[0]:
                once[0] = True
                self.protected[1].write_bytes(edited)
        self.on_command = edit
        with self.assertRaisesRegex(RuntimeError, "Standortdateien nach Update"):
            self.updater.update(self.state, self.protected)
        self.assertEqual(self.protected[1].read_bytes(), edited)
        self.assert_original_files()
        self.assert_untouched_payloads()

    def test_manifest_pins_all_ten_reviewed_repository_payload_blobs(self):
        self.assertEqual(len(type(self).source_manifest), 10)
        self.assertEqual(self.updater.SOURCE_COMMIT, "e842197c2ede8337267a913304816da11664f7a8")
        for name, pins in type(self).source_manifest.items():
            with self.subTest(file=name):
                self.assertEqual(digest((type(self).source_root / name).read_bytes()), pins["new"])


if __name__ == "__main__":
    unittest.main()
