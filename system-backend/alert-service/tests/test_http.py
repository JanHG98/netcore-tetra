"""Real loopback HTTP checks; fake dependencies make radio/network dispatch impossible."""
from datetime import datetime, timedelta, timezone
import http.client
from http.server import ThreadingHTTPServer
import json
import os
from pathlib import Path
import sys
import tempfile
import threading
import unittest
from unittest.mock import patch
from urllib.parse import quote

SERVICE_DIR = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(SERVICE_DIR))
from main import handler_for, load_config
from service import AlertService
from store import Store, timestamp


TOKEN = "test-token-not-for-production-0123456789"
CONTROL_PASSWORD = "control-password-must-not-appear"


class NoNetwork:
    def __init__(self):
        self.calls = []

    def request(self, *args, **kwargs):
        self.calls.append((args, kwargs))
        raise AssertionError("HTTP API tests must never contact a dependency")


def example_config():
    with patch.dict(os.environ, {"NETCORE_ALERT_TOKEN": TOKEN}):
        result = load_config(SERVICE_DIR / "config/alert-service.example.toml")
    result["storage"]["database"] = ":memory:"
    result["netcore"]["control_room_password"] = CONTROL_PASSWORD
    result["nina"]["enabled"] = False
    return result


class HttpApiTests(unittest.TestCase):
    def setUp(self):
        self.dependencies = NoNetwork()
        self.store = Store(":memory:")
        self.service = AlertService(example_config(), store=self.store,
                                    control=self.dependencies, router=self.dependencies)
        self.server = ThreadingHTTPServer(("127.0.0.1", 0), handler_for(self.service))
        self.server.daemon_threads = True
        self.worker = threading.Thread(target=self.server.serve_forever,
                                       kwargs={"poll_interval": 0.02}, daemon=True)
        self.worker.start()

    def tearDown(self):
        self.server.shutdown()
        self.worker.join(timeout=3)
        self.server.server_close()
        self.store.close()
        self.assertFalse(self.worker.is_alive())
        self.assertEqual(self.dependencies.calls, [])

    def request(self, method, path, data=None, token=TOKEN, raw=None, headers=None):
        request_headers = dict(headers or {})
        if token is not None:
            request_headers["Authorization"] = "Bearer " + token
        body = raw
        if data is not None:
            body = json.dumps(data).encode()
        if body is not None:
            request_headers.setdefault("Content-Type", "application/json")
        connection = http.client.HTTPConnection(*self.server.server_address, timeout=3)
        try:
            connection.request(method, path, body=body, headers=request_headers)
            response = connection.getresponse()
            return response.status, dict(response.getheaders()), response.read()
        finally:
            connection.close()

    def warning(self, **overrides):
        value = {"title": "Lokaler Test", "description": "Testmeldung – keine Funkübertragung",
                 "latitude": 52.52, "longitude": 13.405, "radius_m": 1000,
                 "expires_at": (datetime.now(timezone.utc) + timedelta(hours=1)).isoformat(),
                 "severity": "Moderate"}
        value.update(overrides)
        return value

    def test_missing_and_wrong_tokens_cannot_list_create_or_delete(self):
        alert = self.service.create_manual(self.warning())
        delete_path = "/api/v1/alerts/" + quote(alert["id"], safe="")
        for token in (None, "incorrect-token"):
            for method, path, data in (
                    ("GET", "/api/v1/alerts", None),
                    ("GET", "/api/v1/devices", None),
                    ("GET", "/api/v1/deliveries", None),
                    ("GET", "/api/v1/status", None),
                    ("POST", "/api/v1/alerts", self.warning()),
                    ("DELETE", delete_path, None)):
                with self.subTest(token=token, method=method, path=path):
                    status, _, body = self.request(method, path, data, token=token)
                    self.assertEqual(status, 401, body)
        self.assertEqual(len(self.store.alerts()), 1)
        self.assertFalse(self.store.alerts()[0]["removed"])

    def test_authenticated_create_list_status_and_delete_preserve_history(self):
        status, headers, body = self.request("POST", "/api/v1/alerts", self.warning())
        self.assertEqual(status, 201, body)
        alert = json.loads(body)
        self.assertEqual(alert["geometry"]["coordinates"], [13.405, 52.52])
        self.assertEqual(alert["geometry"]["radius_m"], 1000)
        self.assertEqual(headers["Cache-Control"], "no-store")
        self.assertEqual(headers["X-Content-Type-Options"], "nosniff")

        # A retained ledger entry must survive removal; this never submits it.
        self.store.enqueue(alert["id"], 12345, {"idempotency_key": "private-key",
                            "text": "private-router-payload"}, timestamp() + 60, timestamp())
        status, _, body = self.request("GET", "/api/v1/status")
        self.assertEqual(status, 200)
        snapshot = json.loads(body)
        self.assertEqual(snapshot["alerts"][0]["id"], alert["id"])
        self.assertTrue(snapshot["alerts"][0]["active"])
        self.assertFalse(snapshot["delivery_enabled"])
        self.assertEqual(snapshot["deliveries"][0]["issi"], 12345)
        self.assertNotIn("request_json", snapshot["deliveries"][0])
        self.assertNotIn("idempotency_key", snapshot["deliveries"][0])

        status, _, body = self.request("DELETE", "/api/v1/alerts/" + quote(alert["id"], safe=""))
        self.assertEqual(status, 200, body)
        self.assertTrue(json.loads(body)["history_preserved"])
        status, _, body = self.request("GET", "/api/v1/alerts")
        self.assertEqual(status, 200)
        retained = json.loads(body)
        self.assertEqual(len(retained), 1)
        self.assertTrue(retained[0]["removed"])
        self.assertFalse(retained[0]["active"])
        self.assertEqual(len(self.store.deliveries()), 1)

    def test_invalid_geometry_expiry_and_severity_are_rejected_without_mutation(self):
        cases = [
            {"radius_m": 0}, {"radius_m": -100}, {"radius_m": 200001},
            {"latitude": 91}, {"longitude": -181}, {"radius_m": None},
            {"expires_at": "not-a-date"}, {"expires_at": "2030-01-01T12:00:00"},
            {"expires_at": (datetime.now(timezone.utc) - timedelta(seconds=30)).isoformat()},
            {"severity": "Danger"}, {"title": ""},
        ]
        for overrides in cases:
            with self.subTest(overrides=overrides):
                status, _, body = self.request("POST", "/api/v1/alerts", self.warning(**overrides))
                self.assertEqual(status, 400, body)
                self.assertIn("error", json.loads(body))
        self.assertEqual(self.store.alerts(), [])

    def test_malformed_oversized_non_object_and_nonfinite_json_are_rejected(self):
        for raw in (b"{broken", b"[]", b"null", b'{"radius_m":NaN}', b" " * 16385):
            with self.subTest(payload=raw[:30]):
                status, _, body = self.request("POST", "/api/v1/alerts", raw=raw)
                self.assertEqual(status, 400, body)
        status, _, body = self.request("POST", "/api/v1/alerts", raw=b"{}",
                                       headers={"Content-Type": "text/plain"})
        self.assertEqual(status, 400, body)
        self.assertEqual(self.store.alerts(), [])

    def test_static_traversal_never_exposes_service_files(self):
        for path in ("/../main.py", "/%2e%2e/main.py", "/%2e%2e%2fmain.py",
                     "/..%5cmain.py", "/vendor/../../config/alert-service.example.toml",
                     "/%2e%2e%2fconfig%2falert-service.example.toml"):
            with self.subTest(path=path):
                status, _, body = self.request("GET", path, token=None)
                self.assertEqual(status, 404, body)
                self.assertNotIn(b"admin_token", body)
                self.assertNotIn(b"def load_config", body)
        status, headers, body = self.request("GET", "/", token=None)
        self.assertEqual(status, 200)
        self.assertIn("text/html", headers["Content-Type"])
        self.assertIn(b"<html", body.lower())

    def test_health_reports_dependency_readiness_without_credentials(self):
        self.assertEqual(self.request("GET", "/health/live", token=None)[0], 200)
        self.assertEqual(self.request("GET", "/health/ready", token=None)[0], 503)
        self.service.last_cycle = timestamp()
        self.assertEqual(self.request("GET", "/health/ready", token=None)[0], 200)
        self.service.errors["nina"] = "upstream unavailable"
        status, _, body = self.request("GET", "/health/ready", token=None)
        self.assertEqual(status, 503)
        self.assertEqual(json.loads(body), {"status": "degraded"})

    def test_status_errors_and_access_logs_do_not_leak_credentials(self):
        collected = []
        with self.assertLogs("netcore-alert-http", level="INFO") as logs:
            for method, path, kwargs in (
                    ("GET", "/api/v1/status", {}),
                    ("GET", "/api/v1/status", {"token": CONTROL_PASSWORD}),
                    ("DELETE", "/api/v1/alerts/nonexistent", {}),
                    ("POST", "/api/v1/alerts", {"raw": b"[]"})):
                _, headers, body = self.request(method, path, **kwargs)
                collected.append(body.decode())
                collected.append(json.dumps(headers))
        combined = "\n".join(collected + logs.output)
        self.assertNotIn(TOKEN, combined)
        self.assertNotIn(CONTROL_PASSWORD, combined)
        self.assertNotIn("control_room_password", combined)


class ConfigTests(unittest.TestCase):
    def setUp(self):
        self.example = (SERVICE_DIR / "config/alert-service.example.toml").read_text(encoding="utf-8")
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.path = Path(self.directory.name) / "test.toml"

    def load_text(self, text, token=None):
        self.path.write_text(text, encoding="utf-8")
        with patch.dict(os.environ, {}, clear=True):
            if token is not None:
                os.environ["NETCORE_ALERT_TOKEN"] = token
            return load_config(self.path)

    def test_example_loads_with_environment_token_and_safe_delivery_defaults(self):
        config = self.load_text(self.example, TOKEN)
        self.assertEqual(config["server"]["admin_token"], TOKEN)
        self.assertEqual(config["server"]["port"], 8310)
        self.assertFalse(config["delivery"]["enabled"])
        self.assertFalse(config["server"]["allow_unauthenticated"])
        self.assertEqual(config["netcore"]["node_max_age_seconds"], 120)
        self.assertEqual(config["storage"]["database"], "/var/lib/netcore-alert-service/alerts.sqlite3")

    def test_missing_and_short_tokens_fail_before_service_start(self):
        for token in (None, "", "x" * 23):
            with self.subTest(length=None if token is None else len(token)):
                with self.assertRaisesRegex(ValueError, "mindestens 24"):
                    self.load_text(self.example, token)

    def test_environment_overrides_config_token_and_open_mode_is_explicit(self):
        text = self.example.replace('admin_token = ""', 'admin_token = "' + TOKEN + '"')
        self.assertEqual(self.load_text(text)["server"]["admin_token"], TOKEN)
        override = "environment-token-abcdefghijklmnopqrstuvwxyz"
        self.assertEqual(self.load_text(text, override)["server"]["admin_token"], override)
        with self.assertRaises(ValueError):
            self.load_text(text, "short")
        config = self.load_text(self.example.replace("allow_unauthenticated = false",
                                                      "allow_unauthenticated = true"))
        self.assertTrue(config["server"]["allow_unauthenticated"])

    def test_invalid_endpoint_identity_polling_and_boolean_values_are_rejected(self):
        substitutions = [
            ('http://control-room:9010', 'file:///etc/passwd'),
            ('http://control-room:9010', 'http://user:secret@control-room:9010'),
            ('source_issi = 9999', 'source_issi = 0'),
            ('source_issi = 9999', 'source_issi = true'),
            ('poll_seconds = 5', 'poll_seconds = 0'),
            ('node_max_age_seconds = 120', 'node_max_age_seconds = 9'),
            ('enabled = false', 'enabled = "false"'),
        ]
        for old, new in substitutions:
            with self.subTest(replacement=new):
                self.assertIn(old, self.example)
                with self.assertRaises(ValueError):
                    self.load_text(self.example.replace(old, new), TOKEN)


if __name__ == "__main__":
    unittest.main()
