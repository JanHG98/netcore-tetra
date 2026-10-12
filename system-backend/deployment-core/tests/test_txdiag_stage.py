"""Exercise the operator helper's real atomic stage code on temporary files."""
import ast
import contextlib
import hashlib
import io
import json
import os
from pathlib import Path
import re
import stat
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch


HELPER = Path(__file__).resolve().parents[3] / "tools/operator/pi-txdiag-build-stage.sh"
PROGRAMS = re.findall(r"<<'PY'\n(.*?)\nPY", HELPER.read_text(), re.S)
STAGE_PROGRAM = PROGRAMS[-1]


class StageTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.binary = self.root / "bluestation-bs"
        self.source = self.root / "candidate"
        self.backups = self.root / "backups"
        self.old = b"original executable fixture"
        self.new = b"diagnostic executable fixture"
        self.binary.write_bytes(self.old)
        self.binary.chmod(0o755)
        self.source.write_bytes(self.new)
        self.old_sha = hashlib.sha256(self.old).hexdigest()
        self.new_sha = hashlib.sha256(self.new).hexdigest()

    def run_stage(self, *, active=False, reactivate=False, old_sha=None, new_sha=None):
        path_type = Path
        checks = 0

        def rooted_path(value):
            return self.backups if str(value) == "/var/backups/netcore-tetra" else path_type(value)

        def service_state(argv, **kwargs):
            nonlocal checks
            checks += 1
            key = argv[-2]
            if key == "ActiveState":
                return "active\n" if active or (reactivate and checks > 3) else "inactive\n"
            return {"MainPID": "0\n", "UnitFileState": "disabled\n"}[key]

        argv = ["stage", str(self.binary), str(self.source), old_sha or self.old_sha,
                new_sha or self.new_sha, "1" * 40, "2" * 40, "1.99.0"]
        # Service state and ownership require root on a real Pi. Hashing,
        # backup, fsync, modes, atomic replacement and cleanup use real files.
        with patch.object(sys, "argv", argv), patch("pathlib.Path", side_effect=rooted_path), \
                patch("subprocess.check_output", side_effect=service_state), \
                patch("os.fchown") as ownership, contextlib.redirect_stdout(io.StringIO()):
            exec(compile(STAGE_PROGRAM, str(HELPER) + ":stage", "exec"), {})
        ownership.assert_called_once()
        self.assertEqual(ownership.call_args.args[1:], (0, 0))

    def test_embedded_programs_parse(self):
        self.assertEqual(len(PROGRAMS), 3)
        for program in PROGRAMS:
            ast.parse(program)

    def test_unresolved_dynamic_dependency_cannot_pass_shell_guard(self):
        guard = re.search(r"if grep -q 'not found'[^\n]*\n.*?^fi$", HELPER.read_text(), re.S | re.M)
        self.assertIsNotNone(guard)
        for loader_output, allowed in [("libSoapySDR.so => not found\n", False),
                                       ("libSoapySDR.so => /usr/lib/libSoapySDR.so\n", True)]:
            with self.subTest(allowed=allowed):
                (self.root / "ldd.txt").write_text(loader_output)
                result = subprocess.run(
                    ["bash", "-c", "set -Eeuo pipefail\n" + guard.group(0) + "\nprintf allowed"],
                    env={**os.environ, "NC_WORK": str(self.root)}, text=True, capture_output=True,
                )
                self.assertEqual(result.returncode == 0, allowed)
                self.assertEqual(result.stdout, "allowed" if allowed else "")

    def test_verified_swap_keeps_backup_and_does_not_request_rf_start(self):
        self.run_stage()
        self.assertEqual(self.binary.read_bytes(), self.new)
        self.assertEqual(stat.S_IMODE(self.binary.stat().st_mode), 0o755)
        manifest_path, = self.backups.glob("*/manifest.json")
        manifest = json.loads(manifest_path.read_text())
        self.assertEqual(Path(manifest["backup_path"]).read_bytes(), self.old)
        self.assertEqual(manifest["old_binary_sha256"], self.old_sha)
        self.assertEqual(manifest["new_binary_sha256"], self.new_sha)
        self.assertFalse(manifest["rf_start_requested"])
        self.assertEqual(list(self.root.glob(".bluestation-bs-txdiag-*")), [])

    def test_changed_installed_binary_is_preserved(self):
        with self.assertRaises(SystemExit):
            self.run_stage(old_sha="0" * 64)
        self.assertEqual(self.binary.read_bytes(), self.old)
        self.assertFalse(self.backups.exists())

    def test_wrong_candidate_digest_is_rejected_before_backup(self):
        with self.assertRaises(SystemExit):
            self.run_stage(new_sha="0" * 64)
        self.assertEqual(self.binary.read_bytes(), self.old)
        self.assertFalse(self.backups.exists())

    def test_running_service_prevents_any_replacement(self):
        with self.assertRaises(SystemExit):
            self.run_stage(active=True)
        self.assertEqual(self.binary.read_bytes(), self.old)
        self.assertFalse(self.backups.exists())

    def test_service_reactivation_cleans_candidate_and_keeps_original(self):
        with self.assertRaises(SystemExit):
            self.run_stage(reactivate=True)
        self.assertEqual(self.binary.read_bytes(), self.old)
        self.assertEqual(list(self.root.glob(".bluestation-bs-txdiag-*")), [])
        backup, = self.backups.glob("*/bluestation-bs.before")
        self.assertEqual(backup.read_bytes(), self.old)

    def test_installed_symlink_is_left_unchanged(self):
        target = self.root / "original-target"
        self.binary.rename(target)
        self.binary.symlink_to(target)
        with self.assertRaises(SystemExit):
            self.run_stage()
        self.assertTrue(self.binary.is_symlink())
        self.assertEqual(target.read_bytes(), self.old)
        self.assertFalse(self.backups.exists())


if __name__ == "__main__":
    unittest.main()
