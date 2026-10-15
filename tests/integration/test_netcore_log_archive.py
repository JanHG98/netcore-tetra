"""Real files, open descriptors, rotation and interrupted NAS copies."""
from contextlib import contextmanager, redirect_stderr
import gzip
import hashlib
import importlib.util
import io
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import time
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location(
    "log_archive", ROOT / "tools/operator/archive-netcore-logs.py")
archive = importlib.util.module_from_spec(spec)
spec.loader.exec_module(archive)


class ArchiveTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.logs = self.root / "logs"
        self.nas = self.root / "nas"
        self.logs.mkdir()
        self.nas.mkdir()

    @contextmanager
    def mounted(self):
        with patch.object(archive, "network_mount", return_value={"fstype": "nfs4"}):
            with redirect_stderr(io.StringIO()):
                yield

    def source(self, name="syslog.1", data=b"important entry\n" * 300):
        path = self.logs / name
        path.write_bytes(data)
        # 31 Dec UTC becomes 1 Jan in Berlin; archive day follows configured zone.
        os.utime(path, (1767223800, 1767223800))
        return path

    def one(self, path, **kwargs):
        return archive.archive_one(path, path.relative_to(self.logs), self.nas,
                                   "Media-Library", "Europe/Berlin", **kwargs)

    def test_actual_compressed_copy_receipt_and_local_release(self):
        source = self.source()
        raw = source.read_bytes()
        with self.mounted():
            result = self.one(source)
        self.assertFalse(source.exists())
        stored = Path(result["archive"])
        self.assertIn("Media-Library/2026/01/01", stored.as_posix())
        self.assertEqual(gzip.decompress(stored.read_bytes()), raw)
        receipt = json.loads(stored.with_name(stored.name + ".json").read_text())
        self.assertEqual(receipt["sha256"], hashlib.sha256(raw).hexdigest())
        self.assertEqual(receipt["size_bytes"], len(raw))
        self.assertFalse(list(self.nas.rglob("*.part")))

    def test_existing_compressed_log_is_not_double_compressed(self):
        source = self.source("syslog.3.gz", gzip.compress(b"old entries\n" * 100))
        raw = source.read_bytes()
        with self.mounted():
            result = self.one(source)
        self.assertEqual(Path(result["archive"]).read_bytes(), raw)

    def test_binary_journal_stays_directly_readable(self):
        source = self.source("system@abcd-1234-5678.journal", b"LPKSHHRH" + bytes(100))
        raw = source.read_bytes()
        with self.mounted():
            result = self.one(source, is_journal=True)
        self.assertTrue(result["archive"].endswith(".journal"))
        self.assertEqual(Path(result["archive"]).read_bytes(), raw)

    def test_missing_nas_keeps_original_and_creates_no_shadow_archive(self):
        source = self.source()
        with patch.object(archive, "network_mount", side_effect=RuntimeError("NAS offline")):
            with self.assertRaisesRegex(RuntimeError, "NAS offline"):
                self.one(source)
        self.assertTrue(source.exists())
        self.assertEqual(list(self.nas.iterdir()), [])

    def test_corrupt_copy_keeps_original(self):
        source = self.source()
        with self.mounted(), patch.object(archive, "digest_file", return_value=("bad", 0)):
            with self.assertRaisesRegex(RuntimeError, "NAS-Kopie"):
                self.one(source)
        self.assertTrue(source.exists())
        self.assertFalse(list(self.nas.rglob("*.part")))

    def test_failed_receipt_never_releases_original_and_retry_reuses_copy(self):
        source = self.source()
        raw = source.read_bytes()
        with self.mounted(), patch.object(archive, "atomic_json", side_effect=OSError("NAS full")):
            with self.assertRaisesRegex(OSError, "NAS full"):
                self.one(source)
        self.assertTrue(source.exists())
        self.assertEqual(len(list(self.nas.rglob("*.gz"))), 1)
        with self.mounted():
            result = self.one(source)
        self.assertFalse(source.exists())
        self.assertEqual(len(list(self.nas.rglob("*.gz"))), 1)
        self.assertEqual(gzip.decompress(Path(result["archive"]).read_bytes()), raw)

    def test_append_during_copy_keeps_changed_original(self):
        source = self.source()
        original_digest = archive.digest_file
        def change_source(*args, **kwargs):
            value = original_digest(*args, **kwargs)
            with source.open("ab") as writer:
                writer.write(b"late writer\n")
            return value
        with self.mounted(), patch.object(archive, "digest_file", side_effect=change_source):
            with self.assertRaisesRegex(RuntimeError, "Quelle wurde"):
                self.one(source)
        self.assertTrue(source.exists())
        self.assertTrue(source.read_bytes().endswith(b"late writer\n"))

    def test_real_open_file_is_kept_until_reader_or_writer_closes(self):
        source = self.source()
        with source.open("ab") as writer, self.mounted():
            self.assertTrue(archive.file_is_open(source))
            result = self.one(source)
            self.assertEqual(result["status"], "open")
            writer.write(b"still active\n")
        with self.mounted():
            result = self.one(source)
        self.assertEqual(result["status"], "archived")
        self.assertFalse(source.exists())

    def test_source_symlink_is_refused(self):
        target = self.source("other")
        source = self.logs / "syslog.1"
        source.symlink_to(target)
        with self.mounted(), self.assertRaisesRegex(RuntimeError, "reguläre"):
            self.one(source)
        self.assertTrue(target.exists())

    def test_archive_directory_symlink_is_refused(self):
        source = self.source()
        elsewhere = self.root / "elsewhere"
        elsewhere.mkdir()
        (self.nas / "Media-Library").symlink_to(elsewhere, target_is_directory=True)
        with self.mounted(), self.assertRaisesRegex(RuntimeError, "Archivverzeichnis"):
            self.one(source)
        self.assertTrue(source.exists())
        self.assertEqual(list(elsewhere.iterdir()), [])

    def test_published_archive_replacement_does_not_release_source(self):
        source = self.source()
        atomic_json = archive.atomic_json
        def replace_after_receipt(path, value):
            atomic_json(path, value)
            final = Path(value["archive"])
            replacement = final.with_suffix(".replacement")
            replacement.write_bytes(b"tampered")
            replacement.replace(final)
        with self.mounted(), patch.object(archive, "atomic_json", side_effect=replace_after_receipt):
            with self.assertRaisesRegex(RuntimeError, "Archivdatei wurde"):
                self.one(source)
        self.assertTrue(source.exists())

    def test_selection_excludes_active_logs_and_symlinked_subtrees(self):
        for name in ("syslog", "syslog.1", "syslog.3.gz", "syslog-20261015-223000",
                     "auth.log", "auth.log.2", "wtmp", "wtmp.1"):
            self.source(name)
        journals = self.logs / "journal" / "machine"
        journals.mkdir(parents=True)
        (journals / "system.journal").write_bytes(b"active")
        (journals / "system@abcd-1234-5678.journal").write_bytes(b"closed")
        (self.logs / "linked").symlink_to(self.nas, target_is_directory=True)
        found = {relative.as_posix() for _, relative, _ in archive.candidates(self.logs)}
        self.assertEqual(found, {"syslog.1", "syslog.3.gz", "syslog-20261015-223000",
                                 "auth.log.2", "wtmp.1",
                                 "journal/machine/system@abcd-1234-5678.journal"})

    def test_mount_check_accepts_real_nfs_path_and_rejects_local_shadow(self):
        info = ("1 0 0:1 / / rw - ext4 /dev/root rw\n"
                f"2 1 0:2 / {self.nas} rw - nfs4 10.0.1.148:/archive rw\n")
        self.assertEqual(archive.network_mount(self.nas / "Logs", info)["fstype"], "nfs4")
        with self.assertRaisesRegex(RuntimeError, "nicht auf NFS"):
            archive.network_mount(self.logs, info)
        # A nested local overmount must win over an enclosing NAS mount.
        hidden = self.nas / "hidden"
        hidden.mkdir()
        info += f"3 2 0:3 / {hidden} rw - tmpfs tmpfs rw\n"
        with self.assertRaisesRegex(RuntimeError, "nicht auf NFS"):
            archive.network_mount(hidden, info)

    def test_second_archiver_cannot_acquire_same_process_lock(self):
        with archive.exclusive_lock(self.root / "archive.lock"):
            with self.assertRaises(BlockingIOError):
                with archive.exclusive_lock(self.root / "archive.lock"):
                    self.fail("second lock acquired")

    def test_dry_run_preserves_local_and_nas_contents(self):
        source = self.source()
        config = {"source_root": str(self.logs), "archive_root": str(self.nas),
                  "host": "Media-Library", "timezone": "Europe/Berlin"}
        with self.mounted():
            result = archive.archive_pass(config, False)
        self.assertTrue(source.exists())
        self.assertEqual(result["files"][0]["status"], "planned")
        self.assertEqual(list(self.nas.iterdir()), [])


RULE = """/var/log/syslog
/var/log/mail.log
/var/log/kern.log
/var/log/auth.log
/var/log/user.log
/var/log/cron.log
{
        rotate 4
        weekly
        missingok
        notifempty
        compress
        delaycompress
        sharedscripts
        postrotate
                /usr/lib/rsyslog/rsyslog-rotate
        endscript
}
"""


class RotationTests(unittest.TestCase):
    def test_distro_rule_retains_reopen_script_and_becomes_idempotent(self):
        rule = archive.adapt_rsyslog_rule(RULE)
        self.assertIn("/usr/lib/rsyslog/rsyslog-rotate", rule)
        self.assertIn("rotate -1\n", rule)
        self.assertIn("maxsize 64M\n", rule)
        self.assertIn("dateformat -%Y%m%d-%H%M%S\n", rule)
        self.assertNotIn("delaycompress", rule)
        self.assertEqual(archive.adapt_rsyslog_rule(rule), rule)

    def test_unknown_or_copytruncate_rules_fail_before_any_change(self):
        for rule in ("garbage", RULE + RULE, RULE.replace("weekly", "copytruncate"),
                     RULE.replace("postrotate", "# omitted postrotate")):
            with self.subTest(rule=rule), self.assertRaises(RuntimeError):
                archive.adapt_rsyslog_rule(rule)

    def test_maxage_cannot_delete_unarchived_files(self):
        rule = archive.adapt_rsyslog_rule(RULE.replace("weekly", "weekly\n maxage 2"))
        self.assertNotIn("maxage", rule)

    def test_real_logrotate_keeps_numeric_backlog_and_uses_stable_timestamp_names(self):
        self.assertIsNotNone(shutil.which("logrotate"), "native logrotate is required")
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            current = root / "syslog"
            old = root / "syslog.1"
            current.write_bytes(b"new log\n")
            old.write_bytes(b"huge old backlog\n")
            marker = root / "reopened"
            rule = archive.adapt_rsyslog_rule(
                str(current) + "\n{\n rotate 4\n weekly\n missingok\n notifempty\n"
                " compress\n delaycompress\n postrotate\n"
                f"  touch {marker}\n endscript\n}}\n")
            config = root / "rotation.conf"
            config.write_text(rule)
            result = subprocess.run(["logrotate", "--force", "--state", str(root / "state"),
                                     str(config)], capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertTrue(marker.exists())
            self.assertEqual(old.read_bytes(), b"huge old backlog\n")
            dated = list(root.glob("syslog-????????-??????"))
            self.assertEqual(len(dated), 1)
            self.assertEqual(dated[0].read_bytes(), b"new log\n")
            self.assertEqual(old.read_bytes(), b"huge old backlog\n")

    def test_native_systemd_units_pass_verification(self):
        self.assertIsNotNone(shutil.which("systemd-analyze"))
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            names = ("netcore-log-archive.service", "netcore-log-archive.timer",
                     "netcore-log-rotate.service", "netcore-log-rotate.timer")
            for name, content in zip(names, (archive.SERVICE, archive.TIMER,
                                            archive.ROTATION_SERVICE, archive.ROTATION_TIMER)):
                (root / name).write_text(content)
            checked = subprocess.run(["systemd-analyze", "verify",
                                      *(str(root / name) for name in names)],
                                     capture_output=True, text=True)
            self.assertEqual(checked.returncode, 0, checked.stderr)


class InstallTests(unittest.TestCase):
    def test_install_does_not_wait_for_huge_transfer_and_starts_both_timers(self):
        from argparse import Namespace
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            units = root / "systemd"
            units.mkdir()
            rule = root / "rsyslog"
            rule.write_text(RULE)
            nas = root / "nas"
            nas.mkdir()
            config = root / "config/log-archive.json"
            program = root / "opt/archive-netcore-logs.py"
            commands = []
            args = Namespace(archive_root=nas, apply=True)
            with patch.object(archive, "STATE", root / "state"), \
                 patch.object(archive, "PROGRAM", program), \
                 patch.object(archive, "DEFAULT_CONFIG", config), \
                 patch.object(archive, "SYSTEMD_UNITS", units), \
                 patch.object(archive, "RSYSLOG_RULE", rule), \
                 patch.object(archive.socket, "gethostname", return_value="Media-Library"), \
                 patch.object(archive, "network_mount", return_value={"fstype": "nfs4"}), \
                 patch.object(archive, "archive_pass", side_effect=AssertionError("blocking huge copy")), \
                 patch.object(archive, "run_command", side_effect=lambda cmd: commands.append(cmd) or ""):
                result = archive.install(args)
            self.assertEqual(result["phase"], "installed")
            self.assertEqual(result["archive_status"], "scheduled")
            self.assertEqual(json.loads(config.read_text())["host"], "Media-Library")
            self.assertEqual(program.read_bytes(), Path(archive.__file__).read_bytes())
            self.assertIn(["logrotate", "--force", str(rule)], commands)
            self.assertIn(["systemctl", "start", "--no-block",
                           "netcore-log-archive.service"], commands)
            self.assertIn(["systemctl", "enable", "--now", "netcore-log-rotate.timer",
                           "netcore-log-archive.timer"], commands)
            self.assertEqual(len(list(units.iterdir())), 4)
            self.assertTrue(list((root / "state").glob("rsyslog-logrotate-*.before")))

    def test_reopen_script_validation_failure_restores_old_rule(self):
        from argparse import Namespace
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            rule = root / "rsyslog"
            rule.write_text(RULE)
            def command(cmd):
                raise RuntimeError("bad logrotate syntax")
            with patch.object(archive, "STATE", root / "state"), \
                 patch.object(archive, "RSYSLOG_RULE", rule), \
                 patch.object(archive.socket, "gethostname", return_value="Media-Library"), \
                 patch.object(archive, "network_mount", return_value={"fstype": "nfs4"}), \
                 patch.object(archive, "run_command", side_effect=command):
                with self.assertRaisesRegex(RuntimeError, "bad logrotate"):
                    archive.install(Namespace(archive_root=root, apply=True))
            self.assertEqual(rule.read_text(), RULE)

if __name__ == "__main__":
    unittest.main()
