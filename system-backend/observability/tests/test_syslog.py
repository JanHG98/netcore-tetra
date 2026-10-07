import copy
import gzip
import importlib.util
import json
import os
from pathlib import Path
import tempfile
import tomllib
import unittest
from unittest.mock import patch
import sys

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "logging"))
import log_store as logs
import log_client as client
spec = importlib.util.spec_from_file_location("seed", ROOT / "install/seed-openlab.py")
seed = importlib.util.module_from_spec(spec)
spec.loader.exec_module(seed)


class StoreTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name)
        self.cfg = json.loads((ROOT / "config/syslog.example.json").read_text())
        self.cfg.update(state_dir=str(self.root / "state"), inventory=str(ROOT / "config/openlab-hosts.json"),
                        archive_mount=str(self.root / "share"), local_min_free_bytes=1,
                        archive_min_free_bytes=1, archive_max_bytes=10 * 1024 * 1024)
        self.store = logs.Store(self.cfg)
        self.record = {"source_ip": "10.0.20.10", "hostname": "node-gateway",
                       "program": "test", "message": 'hello "quoted"\nnew line', "severity": "info"}

    def tearDown(self):
        self.store.db.close()
        self.tmp.cleanup()

    def mounted_test_share(self, cfg):
        share = Path(cfg["archive_mount"])
        share.mkdir(exist_ok=True)
        return os.open(share, os.O_RDONLY | os.O_DIRECTORY)

    def test_missing_mount_never_writes_to_local_mount_directory(self):
        self.store.append(self.record)
        Path(self.cfg["archive_mount"]).mkdir()
        with self.assertRaisesRegex(OSError, "not mounted"):
            logs.archive(self.store)
        self.assertEqual(len(list(self.store.raw.glob("*.jsonl"))), 1)
        self.assertEqual(list(Path(self.cfg["archive_mount"]).iterdir()), [])
        self.assertIn("not mounted", json.loads((self.store.root / "archive-status.json").read_text())["error"])

    def test_verified_archive_and_retry_after_rename_before_local_delete(self):
        self.store.append(self.record)
        self.store.recover()
        original = next(self.store.raw.glob("*.jsonl")).read_bytes()
        real_unlink = Path.unlink
        def fail_source_unlink(path, *args, **kwargs):
            if path.parent == self.store.raw:
                raise OSError("simulated crash after archive rename")
            return real_unlink(path, *args, **kwargs)
        with patch.object(Path, "unlink", fail_source_unlink):
            with self.assertRaises(OSError):
                logs.archive(self.store, self.mounted_test_share)
        self.assertEqual(len(list(self.store.raw.glob("*.jsonl"))), 1)
        logs.archive(self.store, self.mounted_test_share)
        archives = list(Path(self.cfg["archive_mount"]).rglob("*.gz"))
        self.assertEqual(len(archives), 1)
        self.assertEqual(gzip.decompress(archives[0].read_bytes()), original)
        self.assertEqual(list(self.store.raw.glob("*.jsonl")), [])

    def test_copy_failure_keeps_source_and_cleans_partial(self):
        self.store.append(self.record)
        with patch.object(logs.shutil, "copyfileobj", side_effect=OSError("share disconnected")):
            with self.assertRaisesRegex(OSError, "share disconnected"):
                logs.archive(self.store, self.mounted_test_share)
        self.assertEqual(len(list(self.store.raw.glob("*.jsonl"))), 1)
        self.assertEqual(list(Path(self.cfg["archive_mount"]).rglob("*.partial")), [])

    def test_checksum_failure_never_removes_source(self):
        self.store.append(self.record)
        with patch.object(logs, "digest", side_effect=[b"expected", b"corrupt"]):
            with self.assertRaisesRegex(OSError, "checksum"):
                logs.archive(self.store, self.mounted_test_share)
        self.assertEqual(len(list(self.store.raw.glob("*.jsonl"))), 1)
        self.assertEqual(list(Path(self.cfg["archive_mount"]).rglob("*.gz")), [])

    def test_share_symlink_is_rejected(self):
        self.store.append(self.record)
        share = Path(self.cfg["archive_mount"])
        share.mkdir()
        (share / "Logs").symlink_to(self.root / "state", target_is_directory=True)
        with self.assertRaises(OSError):
            logs.archive(self.store, self.mounted_test_share)
        self.assertFalse((self.store.root / "NetCore").exists())

    def test_local_limit_evicts_oldest_and_counts_loss(self):
        self.cfg.update(local_max_bytes=2200, segment_bytes=1000)
        for i in range(20):
            self.store.append(self.record | {"message": str(i) + "x" * 400})
        status = self.store.status()
        self.assertLessEqual(status["local_bytes"], 2200)
        self.assertGreater(status["counters"]["unarchived_segments_dropped"], 0)
        self.assertGreater(status["counters"]["unarchived_bytes_dropped"], 0)

    def test_free_disk_reserve_drops_new_input(self):
        self.cfg["local_min_free_bytes"] = 10 ** 18
        self.store.append(self.record)
        self.assertEqual(self.store.status()["local_bytes"], 0)
        self.assertEqual(self.store.status()["counters"]["incoming_records_dropped"], 1)

    def test_preview_is_bounded_without_deleting_archive_records(self):
        self.cfg["preview_records"] = 3
        for i in range(10):
            self.store.append(self.record | {"message": str(i)})
        self.assertEqual(self.store.status()["preview_pending"], 3)
        self.assertEqual(self.store.status()["counters"]["preview_records_expired"], 7)
        self.assertEqual(len(next(self.store.raw.glob("*.open")).read_text().splitlines()), 10)

    def test_restart_recovers_complete_lines_only(self):
        self.store.append(self.record)
        path = next(self.store.raw.glob("*.open"))
        good = path.read_bytes()
        with path.open("ab") as out:
            out.write(b'{"interrupted":')
        self.store.recover()
        self.assertEqual(next(self.store.raw.glob("*.jsonl")).read_bytes(), good)
        self.assertEqual(self.store.status()["counters"]["partial_bytes_recovered"], 15)

    def test_large_unicode_and_untrusted_hostname_do_not_escape_store(self):
        self.store.append(self.record | {"hostname": "../../escape", "message": "🛰\n" * 20000})
        raw = next(self.store.raw.glob("*.open")).read_bytes()
        self.assertLessEqual(len(raw), self.cfg["max_record_bytes"])
        self.assertEqual(json.loads(raw)["inventory_name"], "Node-Gateway")
        self.store.append(self.record | {"source_ip": "198.51.100.1"})
        self.assertEqual(self.store.status()["counters"]["rejected_sources"], 1)

    def test_archive_retention_leaves_recordings_and_other_collectors(self):
        share = Path(self.cfg["archive_mount"])
        recordings = share / "Recordings"
        recordings.mkdir(parents=True)
        (recordings / "call.wav").write_bytes(b"media")
        namespace = share / self.cfg["archive_directory"] / self.cfg["collector_id"]
        old = namespace / "2000-01-01"
        old.mkdir(parents=True)
        stale = old / ("20000101T000000-" + "a" * 32 + ".jsonl.gz")
        stale.write_bytes(b"old")
        other = namespace.parent / "other-collector"
        other.mkdir()
        (other / "keep.gz").write_bytes(b"keep")
        logs.archive(self.store, self.mounted_test_share)
        self.assertFalse(stale.exists())
        self.assertTrue((other / "keep.gz").exists())
        self.assertEqual((recordings / "call.wav").read_bytes(), b"media")


class ClientTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.target = Path(self.tmp.name) / "client.conf"
        self.cfg = json.loads((ROOT / "config/log-client.example.json").read_text())
        self.cfg["rsyslog_config"] = str(self.target)
        self.data = {"protocol": "netcore.discovery.v1", "role": "controller", "security_mode": "open_lab",
                     "environment": "netcore-openlab", "conflicts": {},
                     "endpoints": {"observability": {"url": "http://10.0.20.199:8210"},
                                   "node-gateway": {"url": "http://10.0.20.10:8080"}}}

    def tearDown(self):
        self.tmp.cleanup()

    def test_vm_offline_uses_bootstrap_then_cached_address(self):
        def offline(url):
            raise OSError("VM down")
        client.update(self.cfg, offline, lambda _: None, lambda: None)
        self.assertIn('target="10.0.20.26"', self.target.read_text())
        client.update(self.cfg, lambda _: self.data, lambda _: None, lambda: None)
        self.assertIn('target="10.0.20.199"', self.target.read_text())
        before = self.target.read_bytes()
        self.assertFalse(client.update(self.cfg, offline, lambda _: self.fail(), lambda: self.fail()))
        self.assertEqual(self.target.read_bytes(), before)

    def test_conflict_and_invalid_network_keep_previous_config(self):
        self.target.write_text("existing")
        self.data["conflicts"] = {"observability": ["node-a", "node-b"]}
        client.update(self.cfg, lambda _: self.data, lambda _: self.fail(), lambda: self.fail())
        self.assertEqual(self.target.read_text(), "existing")
        self.data["conflicts"] = {}
        self.data["endpoints"]["observability"]["url"] = "http://203.0.113.9:8210"
        client.update(self.cfg, lambda _: self.data, lambda _: self.fail(), lambda: self.fail())
        self.assertEqual(self.target.read_text(), "existing")

    def test_validation_failure_is_atomic(self):
        self.target.write_text("previous")
        def invalid(path):
            raise ValueError("invalid generated rsyslog config")
        with self.assertRaises(ValueError):
            client.update(self.cfg, lambda _: self.data, invalid, lambda: self.fail())
        self.assertEqual(self.target.read_text(), "previous")

    def test_restart_failure_restores_previous_target_and_can_retry(self):
        self.target.write_text("previous")
        attempts = []
        def unavailable():
            attempts.append(self.target.read_text())
            raise OSError("sender restart failed")
        with self.assertRaisesRegex(OSError, "sender restart failed"):
            client.update(self.cfg, lambda _: self.data, lambda _: None, unavailable)
        self.assertEqual(self.target.read_text(), "previous")
        self.assertEqual(len(attempts), 2)
        self.assertIn('target="10.0.20.199"', attempts[0])
        self.assertEqual(attempts[1], "previous")
        self.assertEqual(list(self.target.parent.iterdir()), [self.target])
        self.assertTrue(client.update(self.cfg, lambda _: self.data, lambda _: None, lambda: None))

    def test_restart_failure_without_prior_config_does_not_cache_failed_target(self):
        def unavailable():
            raise OSError("sender restart failed")
        with self.assertRaisesRegex(OSError, "sender restart failed"):
            client.update(self.cfg, lambda _: self.data, lambda _: None, unavailable)
        self.assertFalse(self.target.exists())
        self.assertEqual(list(self.target.parent.iterdir()), [])

    def test_migration_updates_legacy_and_preserves_manual_targets(self):
        example = tomllib.loads((ROOT / "config/observability.example.toml").read_text())
        cfg = {"targets": [dict(example["targets"][0])], "retention": {"max_logs": 100000}}
        cfg["targets"][0]["base_url"] = "http://127.0.0.1:8080"
        updated = seed.migrate(copy.deepcopy(cfg), example)
        self.assertEqual(updated["targets"][0]["base_url"], "http://10.0.20.10:8080")
        self.assertTrue(updated["discovery"]["enabled"])
        self.assertEqual(updated["retention"]["max_logs"], 10000)
        self.assertEqual(seed.migrate(copy.deepcopy(updated), example), updated)
        cfg["targets"][0]["labels"] = {"discovery": "manual"}
        self.assertEqual(seed.migrate(cfg, example)["targets"][0]["base_url"], "http://127.0.0.1:8080")

    def test_migration_preserves_existing_management_addresses_rules_and_disabled_targets(self):
        example = tomllib.loads((ROOT / "config/observability.example.toml").read_text())
        for address in ("http://10.0.20.99:8080", "http://10.0.1.179:8080"):
            cfg = {"targets": [dict(example["targets"][0])],
                   "retention": {"max_logs": 1234}, "alert_rules": [{"rule_id": "custom"}],
                   "discovery": {"enabled": False, "controller_url": "http://10.0.1.131:8320"}}
            cfg["targets"][0].update(base_url=address, enabled=False)
            updated = seed.migrate(copy.deepcopy(cfg), example)
            self.assertEqual(updated["targets"][0]["base_url"], address)
            self.assertFalse(updated["targets"][0]["enabled"])
            self.assertEqual(updated["retention"], cfg["retention"])
            self.assertEqual(updated["alert_rules"], cfg["alert_rules"])
            self.assertEqual(updated["discovery"], cfg["discovery"])


if __name__ == "__main__":
    unittest.main()
