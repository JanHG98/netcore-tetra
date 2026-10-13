"""Host rollout contract tests: mock pct transport, exercise actual planning and scan.

These tests never run pct, systemctl, git, or an installed service.  They assert
the mutation boundaries and failure semantics, rather than installer internals.
"""
from __future__ import annotations

from contextlib import redirect_stdout
import importlib.util
import io
import json
from pathlib import Path
import subprocess
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import Mock, patch


ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location(
    "netcore_lxc_update", ROOT / "tools/operator/update-netcore-lxcs.py")
UPDATE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(UPDATE)


def role(name, dependencies=()):
    return {"name": name, "unit": f"netcore-{name}.service",
            "depends_on": list(dependencies)}


class ContainerParsingTests(unittest.TestCase):
    def test_real_pct_columns_header_locks_and_stopped(self):
        text = ("VMID Status Lock Name\n"
                "136 running       Observability\n"
                "150 stopped backup z014-test-hw\n\n")
        self.assertEqual(UPDATE.containers(text), [
            {"ctid": 136, "status": "running", "name": "Observability"},
            {"ctid": 150, "status": "stopped", "name": "z014-test-hw"}])

    def test_numeric_malformed_container_is_not_silently_ignored(self):
        for text in ("136", "136 migrating mycontainer"):
            with self.subTest(text=text), self.assertRaises(RuntimeError):
                UPDATE.containers(text)

    def test_dependency_order_uses_transitive_edges_and_special_roles(self):
        roles = {
            "view": role("view", ["gateway"]),
            "gateway": role("gateway", ["core"]),
            "core": role("core"),
            "directory": role("directory"),
            "discovery": role("discovery"),
        }
        ordered = UPDATE.ordered(roles)
        self.assertEqual(ordered[:2], ["discovery", "directory"])
        self.assertLess(ordered.index("core"), ordered.index("gateway"))
        self.assertLess(ordered.index("gateway"), ordered.index("view"))

    def test_missing_dependency_is_external_but_cycle_fails(self):
        self.assertEqual(UPDATE.ordered({"gateway": role("gateway", ["absent"])}),
                         ["gateway"])
        with self.assertRaises(RuntimeError):
            UPDATE.ordered({"a": role("a", ["b"]), "b": role("b", ["a"])})


class GuestScanTests(unittest.TestCase):
    def execute_scan(self, managed, properties, config_present=True):
        units = {name: value["unit"] for name, value in {
            "core": role("core"), "gateway": role("gateway"),
            "directory": role("directory"), "discovery": role("discovery")}.items()}
        path = Mock()
        path.is_file.return_value = config_present
        path.read_text.return_value = managed
        calls = []

        def show(args, **kwargs):
            calls.append(args)
            self.assertTrue(kwargs["check"])
            self.assertEqual(args[:2], ["systemctl", "show"])
            unit = args[2]
            value = properties.get(unit, {
                "LoadState": "not-found", "FragmentPath": "", "ActiveState": "inactive"})
            if isinstance(value, Exception):
                raise value
            output = "\n".join(f"{k}={v}" for k, v in value.items()) + "\n"
            return subprocess.CompletedProcess(args, 0, stdout=output, stderr="")

        stdout = io.StringIO()
        with patch("pathlib.Path", return_value=path), \
             patch("subprocess.run", side_effect=show), \
             patch.object(sys, "argv", ["scan.py", json.dumps(units)]), \
             redirect_stdout(stdout):
            exec(UPDATE.SCAN, {"__name__": "__main__"})
        return json.loads(stdout.getvalue()), calls

    @staticmethod
    def loaded(active="active", fragment="/etc/systemd/system/example.service"):
        return {"LoadState": "loaded", "FragmentPath": fragment, "ActiveState": active}

    def test_managed_allowlist_does_not_claim_historic_stray_units(self):
        properties = {f"netcore-{n}.service": self.loaded() for n in
                      ("core", "gateway", "directory", "discovery")}
        found, _ = self.execute_scan('managed_services = ["core"]\n', properties)
        self.assertCountEqual(found["installed"], ["core", "directory", "discovery"])
        self.assertEqual(found["unmanaged"], ["gateway"])

    def test_empty_managed_list_retains_agent_but_no_backend(self):
        properties = {"netcore-core.service": self.loaded(),
                      "netcore-discovery.service": self.loaded(),
                      "brew-server.service": self.loaded()}
        found, _ = self.execute_scan("managed_services = []\n", properties)
        self.assertEqual(found["installed"], ["discovery"])
        self.assertEqual(found["unmanaged"], ["core"])
        self.assertEqual(found["external"], ["brew-server.service"])

    def test_no_explicit_allowlist_accepts_installed_stopped_service(self):
        found, _ = self.execute_scan("", {
            "netcore-core.service": self.loaded(active="inactive")}, config_present=False)
        self.assertEqual(found["installed"], ["core"])

    def test_configuration_or_orphan_dropin_is_not_install_evidence(self):
        found, _ = self.execute_scan("", {
            "netcore-core.service": self.loaded(fragment=""),
            "netcore-gateway.service": {"LoadState": "not-found", "FragmentPath": "",
                                        "ActiveState": "inactive"}}, config_present=False)
        self.assertEqual(found["installed"], [])

    def test_unknown_systemctl_unit_is_a_normal_empty_scan_result(self):
        found, calls = self.execute_scan("", {}, config_present=False)
        self.assertEqual(found, {"installed": [], "unmanaged": [], "external": []})
        self.assertGreater(len(calls), 4)  # External roles are inspected as well.

    def test_bad_role_list_and_bus_failure_are_not_an_empty_inventory(self):
        for value in ('managed_services = "core"\n', "managed_services = [1]\n"):
            with self.subTest(value=value), self.assertRaises(RuntimeError):
                self.execute_scan(value, {})
        with self.assertRaises(subprocess.CalledProcessError):
            self.execute_scan("", {"netcore-core.service": subprocess.CalledProcessError(
                1, ["systemctl", "show"], stderr="Failed to connect to bus")}, False)


class PreliminaryInventoryTests(unittest.TestCase):
    def setUp(self):
        self.roles = {"core": role("core"), "discovery": role("discovery")}

    def test_unrelated_systemd_guest_does_not_need_python(self):
        with patch.object(UPDATE, "run", return_value=SimpleNamespace(
                stdout="cron.service enabled enabled\nssh.service enabled enabled\n")) as run, \
             patch.object(UPDATE, "scan") as scan:
            found = UPDATE.inspect(150, self.roles)
        self.assertEqual(found, {"installed": [], "unmanaged": [], "external": []})
        scan.assert_not_called()
        self.assertEqual(run.call_args.args[0], ["pct", "exec", "150", "--", "systemctl",
                         "list-unit-files", "--type=service", "--no-legend", "--no-pager"])

    def test_external_only_guest_remains_visible_without_python(self):
        with patch.object(UPDATE, "run", return_value=SimpleNamespace(
                stdout="brew-server.service enabled enabled\nasterisk.service disabled enabled\n")), \
             patch.object(UPDATE, "scan") as scan:
            found = UPDATE.inspect(150, self.roles)
        self.assertEqual(found["installed"], [])
        self.assertEqual(found["external"], ["brew-server.service", "asterisk.service"])
        scan.assert_not_called()

    def test_known_runtime_file_uses_full_scan_even_when_disabled(self):
        expected = {"installed": ["core"], "unmanaged": [], "external": []}
        with patch.object(UPDATE, "run", return_value=SimpleNamespace(
                stdout="netcore-core.service disabled enabled\n")), \
             patch.object(UPDATE, "scan", return_value=expected) as scan:
            self.assertEqual(UPDATE.inspect(136, self.roles), expected)
        scan.assert_called_once_with(136, self.roles)

    def test_discovery_on_external_guest_is_a_real_agent_update_target(self):
        expected = {"installed": ["discovery"], "unmanaged": [], "external": ["brew-server.service"]}
        with patch.object(UPDATE, "run", return_value=SimpleNamespace(
                stdout="brew-server.service enabled enabled\nnetcore-discovery.service enabled enabled\n")), \
             patch.object(UPDATE, "scan", return_value=expected) as scan:
            self.assertEqual(UPDATE.inspect(136, self.roles), expected)
        scan.assert_called_once()

    def test_unit_inventory_failure_is_not_an_empty_guest(self):
        with patch.object(UPDATE, "run", side_effect=subprocess.CalledProcessError(
                1, ["pct", "exec"], stderr="systemctl unavailable")), \
             patch.object(UPDATE, "scan") as scan:
            with self.assertRaises(subprocess.CalledProcessError):
                UPDATE.inspect(136, self.roles)
        scan.assert_not_called()


class RolloutTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.report_path = Path(self.temp.name) / "report.json"
        self.report = {"commit": "a" * 40, "containers": [], "plan": [], "results": []}
        self.roles = {"gateway": role("gateway", ["core"]), "core": role("core")}
        self.pct_output = "VMID Status Lock Name\n136 running Core\n137 running Gateway\n"
        self.commands = []
        self.timeline = []
        self.preflight_fail_role = None
        self.exit_codes = []
        self.receipt_mutation = None
        self.apply_phases = {}

        def run(args, **kwargs):
            self.commands.append(args)
            if args[:2] == ["pct", "list"]:
                return SimpleNamespace(stdout=self.pct_output)
            if "list-unit-files" in args:
                return SimpleNamespace(stdout="netcore-core.service enabled enabled\n"
                                       "netcore-gateway.service enabled enabled\n")
            if "archive" in args:
                Path(args[args.index("--output") + 1]).write_bytes(b"trusted source archive")
                return SimpleNamespace(stdout="")
            raise AssertionError("Unexpected mutation or transport: " + repr(args))

        def scan(ctid, roles):
            return {"installed": ["core" if ctid == 136 else "gateway"],
                    "unmanaged": [], "external": []}

        def popen(args, **kwargs):
            name = args[args.index("--service") + 1]
            self.assertEqual(args[args.index("--commit") + 1], self.report["commit"])
            preparing = "--prepare" in args
            mode = "prepare" if preparing else "apply"
            self.assertIn("--prepare" if preparing else "--apply", args)
            self.timeline.append((mode, name))
            if preparing:
                self.assertIn("--prepare-build-deps", args)
                rc = 1 if name == self.preflight_fail_role else 0
            else:
                rc = self.exit_codes.pop(0) if self.exit_codes else 0
            receipt = {"service": name, "source_commit": self.report["commit"],
                       "assets_complete": True,
                       "phase": "prepared" if preparing else "updated_degraded" if rc == 2 else "updated",
                       "files": [{"target": f"/usr/local/bin/netcore-{name}", "sha256": "b" * 64}]}
            if not preparing:
                receipt.update(verified=True, process={"active": True, "MainPID": 1234, "InvocationID": "c" * 32},
                               liveness=True, readiness=(rc == 0))
                phase = self.apply_phases.get(name)
                if phase:
                    receipt["phase"] = phase
                if phase == "inactive_updated":
                    receipt.update(process={"active": False, "MainPID": 0}, liveness=None, readiness=None)
            if self.receipt_mutation:
                receipt = self.receipt_mutation(mode, name, receipt)
            output = "mock runtime update\n" + (json.dumps(receipt) + "\n" if receipt else "")
            return SimpleNamespace(stdout=io.StringIO(output), wait=lambda: rc)

        self.run_mock = Mock(side_effect=run)
        self.scan_mock = Mock(side_effect=scan)
        self.stage_mock = Mock(side_effect=lambda ctid, archive, sha, commit: f"/var/tmp/source-{ctid}")
        self.popen_mock = Mock(side_effect=popen)
        for name, replacement in (("run", self.run_mock), ("scan", self.scan_mock),
                                  ("stage", self.stage_mock)):
            patcher = patch.object(UPDATE, name, replacement)
            patcher.start()
            self.addCleanup(patcher.stop)
        patcher = patch.object(UPDATE.subprocess, "Popen", self.popen_mock)
        patcher.start()
        self.addCleanup(patcher.stop)

    def rollout(self, apply=False, ctid=None):
        with redirect_stdout(io.StringIO()):
            return UPDATE.rollout(SimpleNamespace(apply=apply, ctid=ctid), self.roles,
                                  self.report, self.report_path)

    def test_default_inventory_never_stages_builds_or_restarts_guests(self):
        self.assertEqual(self.rollout(), 0)
        self.assertEqual(self.commands[0], ["pct", "list"])
        self.assertTrue(all("list-unit-files" in command for command in self.commands[1:]))
        self.stage_mock.assert_not_called()
        self.popen_mock.assert_not_called()
        self.assertEqual([entry["service"] for entry in self.report["plan"]],
                         ["core", "gateway"])
        self.assertTrue(self.report_path.is_file())

    def test_stopped_containers_are_reported_without_start_or_scan(self):
        self.pct_output += "150 stopped z014-test-hw\n"
        self.rollout()
        self.assertEqual([call.args[0] for call in self.scan_mock.call_args_list], [136, 137])
        self.assertEqual(self.report["containers"][-1]["phase"], "stopped_not_inspected")
        self.assertEqual(len(self.commands), 3)
        self.assertFalse(any("150" in command for command in self.commands))

    def test_unknown_ctid_refuses_before_scan_or_stage(self):
        with self.assertRaisesRegex(RuntimeError, "999"):
            self.rollout(apply=True, ctid=[999])
        self.scan_mock.assert_not_called()
        self.stage_mock.assert_not_called()

    def test_selected_ctid_limits_actual_work_and_report(self):
        self.rollout(ctid=[137])
        self.assertEqual([call.args[0] for call in self.scan_mock.call_args_list], [137])
        self.assertEqual(self.report["plan"], [{"service": "gateway", "ctid": 137}])

    def test_one_failed_scan_refuses_apply_before_archive_or_staging(self):
        self.scan_mock.side_effect = [
            {"installed": ["core"], "unmanaged": [], "external": []},
            RuntimeError("guest Python absent")]
        with self.assertRaisesRegex(RuntimeError, "Inventarisierung"):
            self.rollout(apply=True)
        self.assertFalse(any("archive" in command for command in self.commands))
        self.stage_mock.assert_not_called()
        self.popen_mock.assert_not_called()
        self.assertEqual(self.report["containers"][-1]["phase"], "scan_failed")
        self.assertEqual(json.loads(self.report_path.read_text())["containers"][-1]["phase"],
                         "scan_failed")

    def test_read_only_scan_failure_returns_nonzero_and_keeps_diagnostic_report(self):
        self.scan_mock.side_effect = RuntimeError("guest Python absent")
        self.assertEqual(self.rollout(), 1)
        self.stage_mock.assert_not_called()
        self.popen_mock.assert_not_called()
        self.assertEqual(len(self.report["containers"]), 2)
        self.assertTrue(all(item["phase"] == "scan_failed" for item in self.report["containers"]))

    def test_preliminary_inventory_failure_prevents_any_guest_staging(self):
        original_run = self.run_mock.side_effect

        def fail_unit_inventory(args, **kwargs):
            if "list-unit-files" in args:
                raise subprocess.CalledProcessError(1, args, stderr="Failed to connect to bus")
            return original_run(args, **kwargs)

        self.run_mock.side_effect = fail_unit_inventory
        with self.assertRaisesRegex(RuntimeError, "Inventarisierung"):
            self.rollout(apply=True)
        self.scan_mock.assert_not_called()
        self.stage_mock.assert_not_called()
        self.popen_mock.assert_not_called()

    def test_unrelated_guest_without_python_does_not_block_managed_rollout(self):
        self.pct_output += "140 running unrelated-app\n"
        original_run = self.run_mock.side_effect

        def unrelated_has_no_netcore_units(args, **kwargs):
            if "list-unit-files" in args and args[2] == "140":
                return SimpleNamespace(stdout="ssh.service enabled enabled\n")
            return original_run(args, **kwargs)

        self.run_mock.side_effect = unrelated_has_no_netcore_units
        self.assertEqual(self.rollout(apply=True), 0)
        self.assertEqual([call.args[0] for call in self.scan_mock.call_args_list], [136, 137])
        self.assertEqual([call.args[0] for call in self.stage_mock.call_args_list], [136, 137])
        self.assertEqual(self.report["containers"][-1]["phase"], "no_managed_runtime")
        self.assertEqual(self.popen_mock.call_count, 4)

    def test_all_layout_preflights_complete_before_first_runtime_exchange(self):
        self.assertEqual(self.rollout(apply=True), 0)
        self.assertEqual(self.timeline, [("prepare", "core"), ("prepare", "gateway"),
                                         ("apply", "core"), ("apply", "gateway")])
        self.assertEqual(self.stage_mock.call_count, 2)

    def test_invalid_second_installation_stops_before_any_apply(self):
        self.preflight_fail_role = "gateway"
        with self.assertRaisesRegex(RuntimeError, "Vorbereitung"):
            self.rollout(apply=True)
        self.assertEqual(self.popen_mock.call_count, 2)
        self.assertEqual(self.timeline, [("prepare", "core"), ("prepare", "gateway")])
        self.assertEqual([item["phase"] for item in self.report["results"]],
                         ["not_attempted", "failed_prepare"])
        self.assertIn("Vorbereitung abgebrochen", self.report["results"][0]["reason"])

    def test_first_failed_apply_stops_dependents_and_preserves_failure_report(self):
        self.exit_codes = [1, 0]
        with self.assertRaisesRegex(RuntimeError, "core"):
            self.rollout(apply=True)
        self.assertEqual(self.popen_mock.call_count, 3)
        self.assertEqual(self.report["results"][0]["phase"], "failed_apply")
        self.assertEqual(self.report["results"][1]["phase"], "not_attempted")
        disk_report = json.loads(self.report_path.read_text())
        self.assertEqual(disk_report["results"][0]["returncode"], 1)
        self.assertIn("mock runtime update", Path(disk_report["results"][0]["log"]).read_text())

    def test_readiness_degraded_continues_and_returns_nonzero(self):
        self.exit_codes = [2, 0]
        self.assertEqual(self.rollout(apply=True), 2)
        self.assertEqual(self.popen_mock.call_count, 4)
        self.assertEqual([item["phase"] for item in self.report["results"]],
                         ["updated_degraded", "updated"])

    def test_prepare_success_without_json_receipt_cannot_start_any_runtime_exchange(self):
        self.receipt_mutation = lambda mode, name, receipt: None if mode == "prepare" else receipt
        with self.assertRaisesRegex(RuntimeError, "JSON"):
            self.rollout(apply=True)
        self.assertEqual(self.timeline, [("prepare", "core")])
        self.assertEqual([item["phase"] for item in self.report["results"]],
                         ["failed_prepare", "not_attempted"])

    def test_applied_sha_must_match_prepared_files_even_with_exit_zero(self):
        def wrong_artifact(mode, name, receipt):
            if mode == "apply" and name == "core":
                receipt["files"][0]["sha256"] = "d" * 64
            return receipt
        self.receipt_mutation = wrong_artifact
        with self.assertRaisesRegex(RuntimeError, "core"):
            self.rollout(apply=True)
        self.assertEqual(self.timeline[-1], ("apply", "core"))
        self.assertIn("Pruefsummen", self.report["results"][0]["error"])
        self.assertEqual(self.report["results"][1]["phase"], "not_attempted")

    def test_wrong_source_commit_cannot_be_counted_as_prepared(self):
        def wrong_commit(mode, name, receipt):
            receipt["source_commit"] = "e" * 40
            return receipt
        self.receipt_mutation = wrong_commit
        with self.assertRaisesRegex(RuntimeError, "Quellcommit"):
            self.rollout(apply=True)
        self.assertEqual(self.timeline, [("prepare", "core")])

    def test_runtime_receipt_requires_running_process_and_liveness(self):
        def missing_process(mode, name, receipt):
            if mode == "apply":
                receipt["process"]["MainPID"] = 0
            return receipt
        self.receipt_mutation = missing_process
        with self.assertRaisesRegex(RuntimeError, "core"):
            self.rollout(apply=True)
        self.assertIn("Prozess", self.report["results"][0]["error"])

    def test_exit_zero_without_verified_install_marker_is_a_failure(self):
        def unverified(mode, name, receipt):
            if mode == "apply":
                receipt["verified"] = False
            return receipt
        self.receipt_mutation = unverified
        with self.assertRaisesRegex(RuntimeError, "core"):
            self.rollout(apply=True)
        self.assertIn("verifiziert", self.report["results"][0]["error"])

    def test_guest_actual_phases_are_preserved_and_inactive_is_explicit(self):
        self.apply_phases = {"core": "already_current", "gateway": "inactive_updated"}
        self.assertEqual(self.rollout(apply=True), 0)
        self.assertEqual([item["phase"] for item in self.report["results"]],
                         ["already_current", "inactive_updated"])
        self.assertEqual(self.report["summary"]["updated"], 0)
        self.assertEqual(self.report["summary"]["already_current"], 1)
        self.assertEqual(self.report["summary"]["inactive_updated"], 1)

    def test_stage_failure_marks_failed_container_and_all_remaining_targets(self):
        self.stage_mock.side_effect = RuntimeError("guest disk full")
        with self.assertRaisesRegex(RuntimeError, "disk full"):
            self.rollout(apply=True)
        self.popen_mock.assert_not_called()
        self.assertEqual([item["phase"] for item in self.report["results"]],
                         ["failed_stage", "not_attempted"])

    def test_preparation_receipt_requires_complete_nonempty_unique_assets(self):
        for corrupt in (
            lambda r: r.update(assets_complete=False),
            lambda r: r.update(files=[]),
            lambda r: r["files"].append(r["files"][0].copy()),
        ):
            with self.subTest(corrupt=corrupt):
                self.timeline.clear()
                def bad_prepare(mode, name, receipt):
                    if mode == "prepare":
                        corrupt(receipt)
                    return receipt
                self.receipt_mutation = bad_prepare
                with self.assertRaises(RuntimeError):
                    self.rollout(apply=True)
                self.assertEqual(self.timeline, [("prepare", "core")])

    def test_no_installed_runtime_refuses_apply_without_archive_or_stage(self):
        self.scan_mock.side_effect = None
        self.scan_mock.return_value = {"installed": [], "unmanaged": [], "external": []}
        with self.assertRaisesRegex(RuntimeError, "Keine installierten"):
            self.rollout(apply=True)
        self.assertFalse(any("archive" in command for command in self.commands))
        self.stage_mock.assert_not_called()


class MainReportTests(unittest.TestCase):
    def test_incomplete_inventory_is_not_reported_as_planned(self):
        with tempfile.TemporaryDirectory() as work:
            original_open = open

            def open_lock(path, *args, **kwargs):
                if str(path) == "/run/lock/netcore-lxc-update.lock":
                    return io.StringIO()
                return original_open(path, *args, **kwargs)

            def incomplete(args, roles, report, report_path):
                report["containers"].append({"ctid": 136, "phase": "scan_failed"})
                return 1

            fcntl = SimpleNamespace(flock=Mock(), LOCK_EX=1, LOCK_NB=2)
            with patch.object(UPDATE.os, "geteuid", return_value=0, create=True), \
                 patch.dict(sys.modules, {"fcntl": fcntl}), \
                 patch("builtins.open", side_effect=open_lock), \
                 patch.object(UPDATE, "run", side_effect=[SimpleNamespace(stdout=""),
                         SimpleNamespace(stdout="a" * 40)]), \
                 patch.object(UPDATE.tempfile, "mkdtemp", return_value=work), \
                 patch.object(UPDATE, "load_helper", return_value=SimpleNamespace(specs=lambda: {})), \
                 patch.object(UPDATE, "rollout", side_effect=incomplete), \
                 redirect_stdout(io.StringIO()):
                self.assertEqual(UPDATE.main([]), 1)
            saved = json.loads((Path(work) / "report.json").read_text())
            self.assertEqual(saved["phase"], "incomplete")
            self.assertEqual(saved["containers"][0]["phase"], "scan_failed")


if __name__ == "__main__":
    unittest.main()
