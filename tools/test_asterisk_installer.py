#!/usr/bin/env python3
"""Test installer decisions without apt, compilation, or host service changes."""
import os
from pathlib import Path
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[1]
HELPER = ROOT / "system-backend/sip-switch/install/ensure-asterisk.sh"

HARNESS = r'''
set -euo pipefail
source "$1"
netcore_asterisk_source_incomplete() {
    [[ "${TEST_SOURCE_INCOMPLETE:-0}" == 1 ]]
}
netcore_asterisk_existing_binary() {
    printf '%s\n' discover >> "$TEST_LOG"
    [[ -n "${TEST_EXISTING_BINARY:-}" ]] || return 1
    printf '%s\n' "$TEST_EXISTING_BINARY"
}
netcore_asterisk_package_candidate() {
    printf '%s\n' candidate >> "$TEST_LOG"
    [[ -n "${TEST_PACKAGE_CANDIDATE:-}" ]] || return 1
    printf '%s\n' "$TEST_PACKAGE_CANDIDATE"
}
netcore_asterisk_install_package() {
    printf '%s\n' package >> "$TEST_LOG"
    [[ "${TEST_PACKAGE_FAILURE:-0}" == 0 ]] || return 42
    NETCORE_ASTERISK_BINARY=/test/package/asterisk
    TEST_EXISTING_BINARY=$NETCORE_ASTERISK_BINARY
}
netcore_asterisk_install_source() {
    printf '%s\n' source >> "$TEST_LOG"
    [[ "${TEST_SOURCE_FAILURE:-0}" == 0 ]] || return 43
    NETCORE_ASTERISK_BINARY=/test/source/asterisk
    TEST_EXISTING_BINARY=$NETCORE_ASTERISK_BINARY
}
netcore_asterisk_validate_install() {
    printf '%s\n' validate >> "$TEST_LOG"
    [[ "${TEST_VALIDATION_FAILURE:-0}" == 0 ]] || return 44
}
netcore_ensure_asterisk
printf 'selected=%s\n' "$NETCORE_ASTERISK_BINARY"
'''


class AsteriskInstallerTests(unittest.TestCase):
    def run_installer(self, **settings):
        with tempfile.TemporaryDirectory() as directory:
            event_log = Path(directory) / "events"
            env = {key: value for key, value in os.environ.items()
                   if not key.startswith(("NETCORE_ASTERISK_", "TEST_"))}
            env.update(TEST_LOG=str(event_log), **settings)
            result = subprocess.run(
                ["bash", "-c", HARNESS, "installer-test", str(HELPER)],
                env=env, text=True, capture_output=True, timeout=10,
            )
            events = event_log.read_text().splitlines() if event_log.exists() else []
            return result, events

    def assert_success(self, result):
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def assert_no_install(self, events):
        self.assertNotIn("package", events)
        self.assertNotIn("source", events)

    def test_existing_installation_is_preserved_in_every_mode(self):
        for mode in ("auto", "package", "source", "existing"):
            with self.subTest(mode=mode):
                result, events = self.run_installer(
                    NETCORE_ASTERISK_INSTALL_MODE=mode,
                    TEST_EXISTING_BINARY="/usr/local/sbin/asterisk",
                )
                self.assert_success(result)
                self.assert_no_install(events)
                self.assertNotIn("candidate", events)
                self.assertIn("selected=/usr/local/sbin/asterisk", result.stdout)

    def test_auto_prefers_available_distribution_package(self):
        result, events = self.run_installer(TEST_PACKAGE_CANDIDATE="1:22.0-1")
        self.assert_success(result)
        self.assertIn("package", events)
        self.assertNotIn("source", events)
        self.assertIn("validate", events)

    def test_auto_builds_source_only_when_package_has_no_candidate(self):
        result, events = self.run_installer()
        self.assert_success(result)
        self.assertIn("candidate", events)
        self.assertIn("source", events)
        self.assertNotIn("package", events)
        self.assertIn("validate", events)

    def test_explicit_source_mode_does_not_install_distribution_package(self):
        result, events = self.run_installer(
            NETCORE_ASTERISK_INSTALL_MODE="source", TEST_PACKAGE_CANDIDATE="1:22.0-1",
        )
        self.assert_success(result)
        self.assertIn("source", events)
        self.assertNotIn("package", events)

    def test_package_failure_never_silently_switches_to_source(self):
        result, events = self.run_installer(
            TEST_PACKAGE_CANDIDATE="1:22.0-1", TEST_PACKAGE_FAILURE="1",
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("package", events)
        self.assertNotIn("source", events)
        self.assertNotIn("validate", events)

    def test_explicit_package_mode_fails_when_no_candidate_exists(self):
        result, events = self.run_installer(NETCORE_ASTERISK_INSTALL_MODE="package")
        self.assertNotEqual(result.returncode, 0)
        self.assert_no_install(events)
        self.assertRegex(result.stderr, r"(?i)(candidate|kandidat|package|paket)")

    def test_existing_mode_requires_an_existing_binary(self):
        result, events = self.run_installer(NETCORE_ASTERISK_INSTALL_MODE="existing")
        self.assertNotEqual(result.returncode, 0)
        self.assert_no_install(events)
        self.assertNotIn("candidate", events)
        self.assertRegex(result.stderr, r"(?i)(existing|vorhanden|asterisk)")

    def test_invalid_mode_fails_before_any_installation(self):
        result, events = self.run_installer(NETCORE_ASTERISK_INSTALL_MODE="surprise")
        self.assertNotEqual(result.returncode, 0)
        self.assert_no_install(events)
        self.assertNotIn("candidate", events)

    def test_failed_source_build_propagates_failure(self):
        result, events = self.run_installer(TEST_SOURCE_FAILURE="1")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("source", events)
        self.assertNotIn("validate", events)

    def test_post_install_validation_failure_is_reported(self):
        result, events = self.run_installer(
            TEST_PACKAGE_CANDIDATE="1:22.0-1", TEST_VALIDATION_FAILURE="1",
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("package", events)
        self.assertNotIn("source", events)
        self.assertIn("validate", events)

    def test_incomplete_source_install_recovers_before_reusing_any_binary(self):
        for mode in ("auto", "source"):
            with self.subTest(mode=mode):
                result, events = self.run_installer(
                    NETCORE_ASTERISK_INSTALL_MODE=mode,
                    TEST_SOURCE_INCOMPLETE="1",
                    TEST_EXISTING_BINARY="/usr/sbin/asterisk",
                    TEST_PACKAGE_CANDIDATE="1:22.0-1",
                )
                self.assert_success(result)
                self.assertEqual(events, ["source", "validate"])
                self.assertIn("selected=/test/source/asterisk", result.stdout)

    def test_failed_source_recovery_does_not_reuse_partial_install(self):
        result, events = self.run_installer(
            TEST_SOURCE_INCOMPLETE="1", TEST_SOURCE_FAILURE="1",
            TEST_EXISTING_BINARY="/usr/sbin/asterisk", TEST_PACKAGE_CANDIDATE="1:22.0-1",
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(events, ["source"])

    def test_package_and_existing_modes_refuse_incomplete_source_install(self):
        for mode in ("package", "existing"):
            with self.subTest(mode=mode):
                result, events = self.run_installer(
                    NETCORE_ASTERISK_INSTALL_MODE=mode,
                    TEST_SOURCE_INCOMPLETE="1",
                    TEST_EXISTING_BINARY="/usr/sbin/asterisk",
                    TEST_PACKAGE_CANDIDATE="1:22.0-1",
                )
                self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
                self.assertEqual(events, [])

    def run_source_failure(self, **settings):
        # Call in an OR-list, as the real caller does. A subshell's `set -e`
        # alone must not let a failed dependency installation reach downloads.
        script = r'''
set -euo pipefail
source "$1"
apt-get() { printf 'apt\n' >> "$TEST_LOG"; return "${TEST_APT_RESULT:-42}"; }
mktemp() { printf '%s\n' "$TEST_BUILD_DIR"; }
curl() { printf 'download\n' >> "$TEST_LOG"; return 56; }
tar() { printf 'extract\n' >> "$TEST_LOG"; exit 90; }
export -f apt-get mktemp curl tar
netcore_asterisk_install_source || exit "$?"
'''
        with tempfile.TemporaryDirectory() as directory:
            event_log = Path(directory) / "events"
            build = Path(directory) / "build"
            build.mkdir()
            env = {key: value for key, value in os.environ.items()
                   if not key.startswith(("NETCORE_ASTERISK_", "TEST_"))}
            env.update(TEST_LOG=str(event_log), TEST_BUILD_DIR=str(build), **settings)
            result = subprocess.run(
                ["bash", "-c", script, "source-test", str(HELPER)],
                env=env, text=True, capture_output=True, timeout=10,
            )
            events = event_log.read_text().splitlines() if event_log.exists() else []
            return result, events

    def test_source_dependency_failure_stops_before_download_even_in_or_list(self):
        if os.geteuid() != 0:
            self.skipTest("source installer requires root; CI runs the mocked tests via sudo")
        result, events = self.run_source_failure()
        self.assertEqual(result.returncode, 42, result.stdout + result.stderr)
        self.assertEqual(events, ["apt"])

    def test_source_download_failure_stops_before_extraction(self):
        if os.geteuid() != 0:
            self.skipTest("source installer requires root; CI runs the mocked tests via sudo")
        result, events = self.run_source_failure(TEST_APT_RESULT="0")
        self.assertEqual(result.returncode, 56, result.stdout + result.stderr)
        self.assertEqual(events, ["apt", "download"])

    def test_invalid_build_parameters_fail_before_dependency_install(self):
        if os.geteuid() != 0:
            self.skipTest("source installer requires root; CI runs the mocked tests via sudo")
        for settings in (
            {"NETCORE_ASTERISK_BUILD_JOBS": "0"},
            {"NETCORE_ASTERISK_BUILD_JOBS": "-1"},
            {"NETCORE_ASTERISK_BUILD_JOBS": "all"},
            {"NETCORE_ASTERISK_VERSION": "../../unsafe"},
        ):
            with self.subTest(settings=settings):
                result, events = self.run_source_failure(**settings)
                self.assertNotEqual(result.returncode, 0)
                self.assertEqual(events, [])


if __name__ == "__main__":
    unittest.main(verbosity=2)
