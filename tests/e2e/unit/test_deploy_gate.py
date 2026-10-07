"""Offline regressions for dependency gates, source drift and retained host config."""
from dataclasses import replace
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import importlib.util
import json
from pathlib import Path
import shlex
import subprocess
import sys
import tempfile
import tarfile
import threading
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "tools"))
from deployment_inventory import registry_inventory_errors, runtime_registry
from check_full_system_integration import Audit, check_tbs_gateway
SPEC = importlib.util.spec_from_file_location("netcore_deploy_gate_tests", ROOT / "deploy/open-lab/netcore-deploy.py")
DEPLOY = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = DEPLOY
SPEC.loader.exec_module(DEPLOY)


class DeploymentGateTests(unittest.TestCase):
    def setUp(self):
        self.inventory = DEPLOY.load_inventory(DEPLOY.DEFAULT_INVENTORY)
        self.first = self.inventory.by_name["node-gateway"]
        self.second = replace(self.inventory.by_name["mobility-core"], depends_on=(self.first.name,))
        self.inventory = replace(self.inventory, services=(self.first, self.second))

    def test_unready_prerequisite_stops_before_any_dependent_install(self):
        with patch.object(DEPLOY, "write_generated"), patch.object(DEPLOY, "bundle", return_value="a" * 64), \
             patch.object(DEPLOY, "run") as run, \
             patch.object(DEPLOY, "wait_ready", side_effect=DEPLOY.DeployError("not ready")):
            with self.assertRaisesRegex(DEPLOY.DeployError, "not ready"):
                DEPLOY.apply(self.inventory, [self.second.name], dry_run=False)
        self.assertEqual(run.call_count, 2)  # scp and ssh for prerequisite only
        commands = "\n".join(str(call.args) for call in run.call_args_list)
        self.assertNotIn(str(self.second.install), commands)

    def test_ready_prerequisites_permit_dependency_order_and_dry_run_never_probes(self):
        with patch.object(DEPLOY, "write_generated"), patch.object(DEPLOY, "bundle", return_value="b" * 64), \
             patch.object(DEPLOY, "run") as run, patch.object(DEPLOY, "wait_ready") as gate:
            DEPLOY.apply(self.inventory, [self.second.name], dry_run=False)
            self.assertEqual([call.args[0].name for call in gate.call_args_list], [self.first.name, self.second.name])
            self.assertEqual(run.call_count, 4)
            gate.reset_mock()
            DEPLOY.apply(self.inventory, [self.second.name], dry_run=True)
            gate.assert_not_called()

    def test_cli_returns_failure_for_ready_rejection(self):
        with patch.object(sys, "argv", ["netcore-deploy", "apply", "node-gateway"]), \
             patch.object(DEPLOY, "apply", side_effect=DEPLOY.DeployError("ready rejected")):
            self.assertEqual(DEPLOY.main(), 1)

    def test_real_http_readiness_recovers_and_timeout_fails_closed(self):
        replies = [503, 200]

        class Handler(BaseHTTPRequestHandler):
            def do_GET(self):
                self.send_response(replies.pop(0) if len(replies) > 1 else replies[0])
                self.end_headers()
                self.wfile.write(b'{"status":"ready"}')

            def log_message(self, *_):
                pass

        server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
        worker = threading.Thread(target=server.serve_forever, kwargs={"poll_interval": 0.01}, daemon=True)
        worker.start()
        try:
            service = replace(self.first, host="127.0.0.1", port=server.server_port)
            DEPLOY.wait_ready(service, 1, 2)
            replies[:] = [503]
            with self.assertRaisesRegex(DEPLOY.DeployError, "readiness failed"):
                DEPLOY.wait_ready(service, 1, 0.05)
        finally:
            server.shutdown()
            worker.join(timeout=2)
            server.server_close()

    def test_real_http_200_explicit_unready_and_error_statuses_fail_closed(self):
        body = b'{"ready":false}'

        class Handler(BaseHTTPRequestHandler):
            def do_GET(self):
                self.send_response(200)
                self.send_header("Content-Type", "application/json")
                self.end_headers()
                self.wfile.write(body)

            def log_message(self, *_):
                pass

        server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
        worker = threading.Thread(target=server.serve_forever, kwargs={"poll_interval": 0.01}, daemon=True)
        worker.start()
        try:
            service = replace(self.first, host="127.0.0.1", port=server.server_port)
            for payload in ({"ready": False}, {"ready": False, "status": "ready"},
                            {"status": "degraded", "ok": True}, {"status": "failed"},
                            {"status": "unavailable"}, {"status": "not_ready"}, {"status": "error"}):
                with self.subTest(payload=payload):
                    body = json.dumps(payload).encode()
                    self.assertFalse(DEPLOY.check_health(service, 1, ready=True)[0])
            body = b'{"ready":false}'
            with self.assertRaisesRegex(DEPLOY.DeployError, "readiness failed"):
                DEPLOY.wait_ready(service, 1, 0.05)
            # Liveness reports that the process answers, independently of readiness.
            self.assertTrue(DEPLOY.check_health(service, 1, ready=False)[0])
        finally:
            server.shutdown()
            worker.join(timeout=2)
            server.server_close()

    def test_real_http_readiness_rejects_non_json_and_keeps_legacy_status_snapshots(self):
        body = b'{"vault_ready":true,"node_gateway_connected":true}'

        class Handler(BaseHTTPRequestHandler):
            def do_GET(self):
                self.send_response(200)
                self.end_headers()
                self.wfile.write(body)

            def log_message(self, *_):
                pass

        server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
        worker = threading.Thread(target=server.serve_forever, kwargs={"poll_interval": 0.01}, daemon=True)
        worker.start()
        try:
            service = replace(self.first, host="127.0.0.1", port=server.server_port)
            self.assertTrue(DEPLOY.check_health(service, 1, ready=True)[0])
            # SIP/media status snapshots can be larger than the old 4096-byte
            # diagnostic read cap; valid objects without uniform keys still pass.
            body = json.dumps({"node_gateway_connected": True, "details": "x" * 8192}).encode()
            self.assertTrue(DEPLOY.check_health(service, 1, ready=True)[0])
            for invalid in (b"<html>Login</html>", b"{broken", b"[]", b"null"):
                with self.subTest(body=invalid):
                    body = invalid
                    self.assertFalse(DEPLOY.check_health(service, 1, ready=True)[0])
        finally:
            server.shutdown()
            worker.join(timeout=2)
            server.server_close()

    def test_generator_check_detects_stale_and_removed_assets_without_writing(self):
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory)
            DEPLOY.write_generated(self.inventory, output)
            self.assertEqual(DEPLOY.check_generated(self.inventory, output), [])
            catalog = output / "service-catalog.json"
            catalog.write_text('{"services":[]}\n')
            before = catalog.read_bytes()
            errors = DEPLOY.check_generated(self.inventory, output)
            self.assertTrue(any("service-catalog.json" in error for error in errors))
            self.assertEqual(catalog.read_bytes(), before)
            (output / "obsolete.toml").write_text("old = true\n")
            self.assertTrue(any("obsolete generated asset" in error for error in DEPLOY.check_generated(self.inventory, output)))

    def test_duplicate_management_ports_and_unknown_dependencies_are_rejected(self):
        duplicate = replace(self.second, port=self.first.port)
        self.assertTrue(any("duplicate management port" in error for error in DEPLOY.validate(replace(self.inventory, services=(self.first, duplicate)))))
        missing = replace(self.second, depends_on=("missing",))
        with self.assertRaisesRegex(DEPLOY.DeployError, "unknown dependency"):
            DEPLOY.topological_order(replace(self.inventory, services=(self.first, missing)))

    def test_existing_configuration_is_restored_even_if_installer_overwrites_it(self):
        with tempfile.TemporaryDirectory() as directory:
            target = Path(directory) / "host.toml"
            backup = Path(directory) / "backup.toml"
            backup.write_text('admin_token = "local-secret"\ndelivery_enabled = true\n')
            target.write_text('admin_token = ""\ndelivery_enabled = false\n')
            service = replace(self.first, config_target=str(target))
            command = ("netcore_had_config=1; netcore_config_backup=" + shlex.quote(str(backup)) + "; "
                       + DEPLOY.config_install_command(service, "never-installed", replace_config=False))
            subprocess.run(["bash", "-e", "-c", command], check=True)
            self.assertIn('admin_token = "local-secret"', target.read_text())
            self.assertIn("delivery_enabled = true", target.read_text())
            self.assertFalse(backup.exists())

    def test_bundle_is_reproducible_and_excludes_documentation_builds_and_caches(self):
        with tempfile.TemporaryDirectory() as directory:
            source = Path(directory) / "source"
            source.mkdir()
            (source / "app.py").write_text("print('source')\n")
            (source / "reference.pdf").write_bytes(b"source documentation")
            for excluded in ("target", "node_modules", "__pycache__", ".git"):
                path = source / excluded
                path.mkdir()
                (path / "ignored.txt").write_text("excluded")
            output = Path(directory) / "release.tar.gz"
            with patch.object(DEPLOY, "ROOT", source):
                first = DEPLOY.bundle(output)
                self.assertEqual(DEPLOY.bundle(output), first)
            with tarfile.open(output) as archive:
                names = archive.getnames()
            self.assertIn("netcore-tetra-swmi/app.py", names)
            self.assertFalse(any(name.endswith(".pdf") for name in names))
            self.assertFalse(any(part in name for name in names for part in ("target", "node_modules", "__pycache__", ".git")))

    def test_registry_missing_service_wrong_port_and_weakened_authentication_fail(self):
        import tomllib
        services = {item["name"]: item for item in tomllib.loads(DEPLOY.DEFAULT_INVENTORY.read_text())["services"]}
        self.assertEqual(registry_inventory_errors(services), [])
        missing = dict(services)
        del missing["alert-service"]
        self.assertTrue(any("alert-service" in error for error in registry_inventory_errors(missing)))
        wrong_port = dict(services, **{"deployment-core": dict(services["deployment-core"], port=8080)})
        self.assertTrue(any("port differs" in error for error in registry_inventory_errors(wrong_port)))
        wrong_auth = dict(services, **{"alert-service": dict(services["alert-service"], security_mode="open_lab")})
        self.assertTrue(any("security_mode differs" in error for error in registry_inventory_errors(wrong_auth)))

    def test_site_gateway_mapping_is_strict_and_network_difference_is_visible(self):
        site = {"gateway_host": "10.0.1.179", "gateway_port": 8080, "endpoint_path": "/ws/node"}
        control = {"host": site["gateway_host"], "port": site["gateway_port"], "endpoint_path": site["endpoint_path"]}
        gateway = {"host": "10.0.20.10", "port": 8080}
        audit = Audit()
        check_tbs_gateway(audit, control, gateway, site)
        self.assertEqual(audit.errors, [])
        self.assertTrue(any("Confirm actual Node Gateway" in note for note in audit.notes))
        for altered in (dict(control, host="10.0.1.180"), dict(control, endpoint_path="/ws/operator"), dict(control, port=9010)):
            audit = Audit()
            check_tbs_gateway(audit, altered, gateway, site)
            self.assertTrue(audit.errors)
        audit = Audit()
        check_tbs_gateway(audit, control, gateway, {})
        self.assertTrue(audit.errors)


if __name__ == "__main__":
    unittest.main()
