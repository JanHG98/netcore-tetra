#!/usr/bin/env python3
"""Recover lost AstDB routing gates without changing the active SIP registration."""
import importlib.util
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import tomllib
import types
import unittest
from unittest.mock import patch

from check_sip_switch_phase11c import FakeCLI


ROOT = Path(__file__).resolve().parents[1]
BASE = ROOT / "system-backend/sip-switch/tbs-fallback"
spec = importlib.util.spec_from_file_location("sip_failover_reconciliation", BASE / "src/netcore_tbs_sip_fallback.py")
fallback = importlib.util.module_from_spec(spec)
if os.name == "nt":
    # These tests exercise the controller, not the Unix process lock.
    with patch.dict(sys.modules, {"fcntl": types.ModuleType("fcntl")}):
        spec.loader.exec_module(fallback)
else:
    spec.loader.exec_module(fallback)


class TrackingCLI(FakeCLI):
    def __init__(self, active_file):
        super().__init__(active_file)
        self.mode_writes = []

    def set_mode_db(self, mode):
        self.mode_writes.append(mode)
        super().set_mode_db(mode)


class FailoverReconciliationTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        root = Path(self.directory.name)
        cfg = tomllib.loads((BASE / "config/tbs-sip-fallback.example.toml").read_text(encoding="utf-8"))
        for key in ("state_file", "active_registration_file", "central_registration_file", "pbx_registration_file"):
            cfg["failover"][key] = str(root / Path(cfg["failover"][key]).name)
        cfg["failover"].update(unregister_grace_secs=0, central_registration_grace_secs=0)
        cfg["central"]["health_url"] = ""
        self.cli = TrackingCLI(Path(cfg["failover"]["active_registration_file"]))
        with patch.object(fallback, "load", return_value=cfg):
            self.controller = fallback.FailoverController(root / "fixture.toml", self.cli, sleeper=lambda _: None)
        fallback.write_asterisk_config(self.controller.active_file, fallback.render_active_registration(cfg, "central"))

    def test_healthy_central_repairs_missing_key_at_first_tick_and_after_loss(self):
        controller = self.controller
        original = controller.active_file.read_bytes()
        self.assertIsNone(self.cli.db_mode)
        for now in (100, 101):
            self.cli.db_mode = None
            state = controller.tick(now)
            self.assertEqual(self.cli.db_mode, "central")
            self.assertEqual((state["mode"], state["phase"]), ("central", "CENTRAL_ACTIVE"))
            self.assertTrue(state["last_probe_ok"])
        controller.tick(102)
        self.assertEqual(self.cli.mode_writes, ["central", "central"])
        self.assertEqual(self.cli.reloads, 0)
        self.assertEqual(self.cli.unregistered, [])
        self.assertEqual(controller.active_file.read_bytes(), original)

    def test_direct_pbx_repairs_missing_stale_and_interrupted_modes_without_switching(self):
        controller = self.controller
        controller.switch_mode("pbx_direct", "fixture central outage", 100)
        self.cli.aor_ok = False
        original = controller.active_file.read_bytes()
        reloads, unregistered = self.cli.reloads, list(self.cli.unregistered)
        self.cli.mode_writes.clear()
        for now, lost_mode in enumerate((None, "central", "transition", "invalid"), 101):
            with self.subTest(lost_mode=lost_mode):
                self.cli.db_mode = lost_mode
                state = controller.tick(now)
                self.assertEqual(self.cli.db_mode, "pbx_direct")
                self.assertEqual((state["mode"], state["phase"]), ("pbx_direct", "PBX_DIRECT_ACTIVE"))
        self.assertEqual(self.cli.mode_writes, ["pbx_direct"] * 4)
        self.assertEqual(self.cli.reloads, reloads)
        self.assertEqual(self.cli.unregistered, unregistered)
        self.assertEqual(controller.active_file.read_bytes(), original)

    def test_disabled_freepbx_repairs_central_gate_without_enabling_fallback(self):
        controller = self.controller
        controller.cfg["fallback_pbx"] = {"enabled": False}
        self.cli.aor_ok = False
        for now in range(100, 105):
            controller.tick(now)
        self.assertEqual(self.cli.db_mode, "central")
        self.assertEqual(controller.state["phase"], "CENTRAL_UNAVAILABLE")
        self.assertEqual(self.cli.mode_writes, ["central"])
        self.assertEqual(self.cli.reloads, 0)
        self.assertEqual(self.cli.unregistered, [])

    def test_unreachable_asterisk_does_not_write_or_switch_the_registration(self):
        original = self.controller.active_file.read_bytes()
        with patch.object(self.cli, "get_mode_db", side_effect=RuntimeError("Asterisk unavailable")):
            with self.assertRaisesRegex(RuntimeError, "Asterisk unavailable"):
                self.controller.tick(100)
        self.assertEqual(self.cli.mode_writes, [])
        self.assertEqual(self.cli.reloads, 0)
        self.assertEqual(self.cli.unregistered, [])
        self.assertEqual(self.controller.active_file.read_bytes(), original)


class ModeReadTests(unittest.TestCase):
    def test_missing_key_is_distinct_from_cli_connection_failure(self):
        cli = fallback.AsteriskCLI("asterisk")
        for output, code, expected in (("Value: central\n", 0, "central"),
                                       ("Value: pbx_direct\n", 0, "pbx_direct"),
                                       ("Database entry not found.\n", 0, None)):
            with self.subTest(output=output), patch.object(cli, "run", return_value=subprocess.CompletedProcess([], code, output, "")):
                self.assertEqual(cli.get_mode_db(), expected)
        for output, code in (("Unable to connect to remote asterisk\n", 1),
                             ("No such command 'database get'\n", 0)):
            with self.subTest(output=output), patch.object(cli, "run", return_value=subprocess.CompletedProcess([], code, output, "")):
                with self.assertRaises(RuntimeError):
                    cli.get_mode_db()


if __name__ == "__main__":
    unittest.main(verbosity=2)
