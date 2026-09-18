"""End-to-end worker/store scenarios without radio, network or wall-clock waits."""
import copy
import io
from pathlib import Path
import sys
import tempfile
import unittest
from urllib.error import HTTPError
from urllib.parse import unquote

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from nina import FetchResult
from service import AlertService, radio_text
from store import Store, iso, timestamp

NOW = timestamp("2026-09-18T12:00:00Z")


def missing_response(path, message):
    error = HTTPError(path, 404, message, {}, io.BytesIO())
    error.close()  # The fake has no response stream to consume.
    return error


def warning(identifier="incident-a", *, now=NOW, aliases=None, cancelled=False, expires=None):
    return {
        "id": identifier, "provider_id": identifier, "incident_id": identifier,
        "aliases": aliases or [identifier], "source": "nina", "provider": "mowas",
        "title": "Rauchentwicklung", "description": "Fenster schließen.",
        "instruction": "Im Gebäude bleiben.", "severity": "Severe",
        "starts_at": iso(now - 60), "sent_at": iso(now),
        "expires_at": iso(expires) if expires is not None else None,
        "cancelled": cancelled,
        "geometry": {"type": "Circle", "coordinates": [13.4, 52.5], "radius_m": 2000},
    }


def subscriber(issi=1001, *, online=True, latitude=52.5, longitude=13.4, updated=NOW):
    return {"issi": issi, "online": online, "node_id": "tbs-one", "last_location": {
        "latitude": latitude, "longitude": longitude, "updated_at": iso(updated),
    }}


class FakeControl:
    def __init__(self):
        self.rows = []
        self.now = NOW
        self.error = None
        self.nodes_error = None
        self.node_connected = True
        self.transport_connected = True
        self.node_seen = None
        self.calls = []

    def request(self, method, path, data=None):
        self.calls.append((method, path))
        if self.error:
            raise self.error
        if path == "/api/nodes":
            if self.nodes_error:
                raise self.nodes_error
            return [{"node_id": "tbs-one", "connected": self.node_connected,
                     "transport_connected": self.transport_connected,
                     "last_seen": iso(self.now if self.node_seen is None else self.node_seen)}]
        if path == "/api/subscribers?online=true":
            return {"subscribers": copy.deepcopy(self.rows)}
        raise AssertionError(f"Unexpected control API call: {method} {path}")


class FakeNina:
    def __init__(self, alerts=None):
        self.result = FetchResult(alerts or [], True, [], [])
        self.error = None
        self.calls = 0

    def fetch(self):
        self.calls += 1
        if self.error:
            raise self.error
        return copy.deepcopy(self.result)


class FakeRouter:
    """Models durable idempotency, including a timeout after accepting a POST."""
    def __init__(self):
        self.ready = True
        self.error = None
        self.state = "delivered"
        self.timeout_once = False
        self.timeout_before_once = False
        self.retained = True
        self.posts = []
        self.cancels = []
        self.messages = {}
        self.keys = {}

    def request(self, method, path, data=None):
        if self.error:
            raise self.error
        if path == "/api/v1/status":
            return {"durable_idempotency": self.ready, "at_most_once": self.ready}
        if method == "POST" and path == "/api/v1/messages":
            self.posts.append(copy.deepcopy(data))
            if self.timeout_before_once:
                self.timeout_before_once = False
                raise TimeoutError("Request did not reach the router")
            key = data["idempotency_key"]
            if key not in self.keys:
                message_id = f"message-{len(self.messages) + 1}"
                self.keys[key] = message_id
                self.messages[message_id] = {"id": message_id, "state": self.state}
            message_id = self.keys[key]
            if self.timeout_once:
                self.timeout_once = False
                raise TimeoutError("Response lost after router accepted the message")
            return copy.deepcopy(self.messages[message_id])
        if method == "GET" and path.startswith("/api/v1/idempotency/"):
            key = unquote(path.removeprefix("/api/v1/idempotency/"))
            if key not in self.keys:
                raise missing_response(path, "Unknown key")
            return {"retained": self.retained, "message_id": self.keys[key]}
        if path.startswith("/api/v1/messages/"):
            message_id = unquote(path.removeprefix("/api/v1/messages/").removesuffix("/cancel"))
            if message_id not in self.messages:
                raise missing_response(path, "Unknown message")
            if method == "POST" and path.endswith("/cancel"):
                self.cancels.append(message_id)
                self.messages[message_id]["state"] = "cancelled"
            return copy.deepcopy(self.messages[message_id])
        raise AssertionError(f"Unexpected router API call: {method} {path}")


class ServiceTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.database = str(Path(self.directory.name) / "alerts.sqlite3")
        self.config = {
            "storage": {"database": self.database},
            "netcore": {"control_room_url": "http://control.invalid", "sds_router_url": "http://router.invalid",
                        "source_issi": 9999, "gps_max_age_seconds": 3600, "node_max_age_seconds": 120},
            "nina": {"enabled": True, "poll_seconds": 1, "max_stale_seconds": 60},
            "delivery": {"enabled": True, "ttl_seconds": 300, "max_text_length": 120},
        }
        self.control, self.router, self.nina = FakeControl(), FakeRouter(), FakeNina([warning()])
        self.store = Store(self.database)
        self.service = self.make_service()

    def tearDown(self):
        self.store.close()
        self.directory.cleanup()

    def make_service(self):
        return AlertService(self.config, store=self.store, control=self.control, router=self.router, nina=self.nina)

    def tick(self, elapsed=0):
        self.control.now = NOW + elapsed
        self.service.tick(NOW + elapsed)

    def restart(self):
        self.store.close()
        self.store = Store(self.database)
        self.service = self.make_service()

    def test_login_receives_existing_warning_once_per_issi_even_after_restart(self):
        self.tick()
        self.assertEqual(self.router.posts, [])
        self.control.rows = [subscriber()]
        self.tick(1)
        self.assertEqual(len(self.router.messages), 1)
        self.tick(2)
        self.assertEqual(self.store.deliveries()[0]["state"], "accepted")
        self.control.rows[0]["online"] = False
        self.tick(3)
        self.control.rows[0]["online"] = True
        self.tick(4)
        self.restart()
        self.tick(5)
        self.assertEqual(len(self.router.posts), 1)
        self.control.rows.append(subscriber(1002))
        self.tick(6)
        self.assertEqual({p["dest_issi"] for p in self.router.posts}, {1001, 1002})

    def test_new_warning_reaches_already_logged_in_radio(self):
        self.nina.result.alerts = []
        self.control.rows = [subscriber()]
        self.tick()
        self.nina.result.alerts = [warning("new-incident", now=NOW + 1)]
        self.tick(1)
        self.assertEqual(len(self.router.posts), 1)
        self.assertEqual(self.router.posts[0]["dest_issi"], 1001)
        self.assertTrue(self.router.posts[0]["at_most_once"])

    def test_movement_into_area_sends_only_once(self):
        self.control.rows = [subscriber(latitude=53)]
        self.tick()
        self.assertFalse(self.router.posts)
        self.control.rows[0]["last_location"] = subscriber(updated=NOW + 1)["last_location"]
        self.tick(1)
        self.control.rows[0]["last_location"]["latitude"] = 53
        self.tick(2)
        self.control.rows[0]["last_location"]["latitude"] = 52.5
        self.tick(3)
        self.assertEqual(len(self.router.posts), 1)

    def test_update_reference_chains_reuse_persisted_delivery_history(self):
        self.control.rows = [subscriber()]
        self.tick()
        update = warning("incident-b", now=NOW + 1, aliases=["incident-b", "incident-a"])
        update["incident_id"] = "incident-a"
        self.nina.result.alerts = [update]
        self.tick(1)
        self.restart()
        update = warning("incident-c", now=NOW + 2, aliases=["incident-c", "incident-b"])
        update["incident_id"] = "incident-b"
        self.nina.result.alerts = [update]
        self.tick(2)
        self.assertEqual(len(self.router.posts), 1)
        current = [a for a in self.store.alerts() if not a["removed"]]
        self.assertEqual(len(current), 1)
        self.assertEqual(current[0]["id"], "nina:incident-a")
        self.assertEqual(current[0]["provider_id"], "incident-c")

    def test_cancel_stops_queued_warning_without_sending_cancel_as_new_sds(self):
        self.router.state = "queued"
        self.control.rows = [subscriber()]
        self.tick()
        cancel = warning("incident-cancel", now=NOW + 1, aliases=["incident-cancel", "incident-a"], cancelled=True)
        cancel["incident_id"] = "incident-a"
        self.nina.result.alerts = [cancel]
        self.tick(1)
        self.assertEqual(self.router.cancels, ["message-1"])
        self.assertEqual(self.store.deliveries()[0]["state"], "cancelled")
        self.assertEqual(len(self.router.posts), 1)

    def test_expired_and_future_warnings_do_not_send(self):
        self.control.rows = [subscriber()]
        expired = warning("expired", expires=NOW - 1)
        future = warning("future")
        future["starts_at"] = iso(NOW + 60)
        self.nina.result.alerts = [expired, future]
        self.tick()
        self.assertFalse(self.router.posts)
        self.tick(60)
        self.assertEqual(len(self.router.posts), 1)

    def test_complete_disappearance_withdraws_warning_and_cancels_queue(self):
        self.router.state = "queued"
        self.control.rows = [subscriber()]
        self.tick()
        self.nina.result.alerts = []
        self.tick(1)
        self.assertTrue(self.store.alerts()[0]["removed"])
        self.assertEqual(self.router.cancels, ["message-1"])

    def test_partial_failure_retains_warning_but_stale_data_stops_new_delivery(self):
        self.tick()
        self.nina.result = FetchResult([], False, ["mowas unavailable"], [])
        self.control.rows = [subscriber()]
        self.tick(61)
        self.assertFalse(self.store.alerts()[0]["removed"])
        self.assertEqual(self.store.meta("nina_success"), NOW)
        self.assertIn("nina", self.service.errors)
        self.assertFalse(self.router.posts)
        self.nina.result = FetchResult([warning()], True, [], ["incident-a"])
        self.tick(62)
        self.assertEqual(len(self.router.posts), 1)

    def test_offline_missing_stale_future_and_invalid_gps_do_not_send(self):
        self.control.rows = [
            subscriber(1001, online=False), subscriber(1002, updated=NOW - 3601),
            subscriber(1003, updated=NOW + 61), subscriber(1004, latitude=float("nan")),
            subscriber(1005, longitude=190), {"issi": 1006, "online": True, "node_id": "tbs-one"},
        ]
        self.tick()
        self.assertEqual(self.service.devices, [])
        self.assertFalse(self.router.posts)

    def test_null_location_or_node_timestamp_never_means_current_time(self):
        now = timestamp()
        rows = [subscriber(updated=now)]
        nodes = [{"node_id": "tbs-one", "connected": True, "transport_connected": True, "last_seen": iso(now)}]
        self.assertEqual(len(self.service._devices({"subscribers": rows}, now, nodes)), 1)
        for missing in (None, "", 0):
            with self.subTest(field="updated_at", value=missing):
                rows[0]["last_location"]["updated_at"] = missing
                self.assertEqual(self.service._devices({"subscribers": rows}, now, nodes), [])
        rows[0]["last_location"]["updated_at"] = iso(now)
        for missing in (None, "", 0):
            with self.subTest(field="last_seen", value=missing):
                nodes[0]["last_seen"] = missing
                self.assertEqual(self.service._devices({"subscribers": rows}, now, nodes), [])

    def test_snapshot_explains_skipped_gps_without_changing_dispatch(self):
        self.control.rows = [subscriber(1001, updated=NOW - 3601),
                             {"issi": 1002, "online": True, "node_id": "tbs-one"},
                             subscriber(1003)]
        self.tick()
        result = self.service.snapshot()
        self.assertEqual(result["subscribers_seen"], 3)
        self.assertEqual([d["issi"] for d in result["devices"]], [1003])
        rejected = {d["issi"]: d for d in result["device_diagnostics"]}
        self.assertEqual(set(rejected), {1001, 1002})
        self.assertEqual(rejected[1001]["reason_code"], "gps_stale")
        self.assertEqual(rejected[1001]["gps_age_seconds"], 3601)
        self.assertEqual(rejected[1001]["node_age_seconds"], 0)
        self.assertIn("3600 Sekunden", rejected[1001]["reason"])
        self.assertEqual(rejected[1002]["reason_code"], "gps_missing")
        self.assertNotIn("gps_age_seconds", rejected[1002])
        self.assertEqual([p["dest_issi"] for p in self.router.posts], [1003])
        self.assertIn(("GET", "/api/subscribers?online=true"), self.control.calls)

    def test_snapshot_explains_offline_and_stale_node_then_clears_on_recovery(self):
        self.control.rows = [subscriber()]
        self.control.node_connected = False
        self.tick()
        result = self.service.snapshot()
        self.assertEqual(result["device_diagnostics"][0]["reason_code"], "node_offline")
        self.assertEqual(result["device_diagnostics"][0]["gps_age_seconds"], 0)
        self.assertFalse(self.router.posts)
        self.control.node_connected = True
        self.control.node_seen = NOW - 121
        self.tick(1)
        result = self.service.snapshot()
        self.assertEqual(result["device_diagnostics"][0]["reason_code"], "node_stale")
        self.assertEqual(result["device_diagnostics"][0]["node_age_seconds"], 122)
        self.assertFalse(self.router.posts)
        self.control.node_seen = None
        self.tick(2)
        self.assertEqual(self.service.snapshot()["device_diagnostics"], [])
        self.assertEqual(len(self.router.posts), 1)

    def test_control_failure_clears_previous_device_diagnostics(self):
        self.control.rows = [subscriber(updated=NOW - 3601)]
        self.tick()
        self.assertEqual(len(self.service.snapshot()["device_diagnostics"]), 1)
        self.control.nodes_error = OSError("nodes unavailable")
        self.tick(1)
        result = self.service.snapshot()
        self.assertEqual(result["device_diagnostics"], [])
        self.assertEqual(result["subscribers_seen"], 0)
        self.assertEqual(result["devices"], [])
        self.assertIn("control_room", result["errors"])
        self.assertFalse(self.router.posts)

    def test_duplicate_subscriber_rows_use_newest_position(self):
        self.control.rows = [subscriber(updated=NOW - 60), subscriber(latitude=53, updated=NOW)]
        self.tick()
        self.assertFalse(self.router.posts)
        self.assertEqual(len(self.service.devices), 1)

    def test_disconnected_or_stale_node_blocks_stale_online_subscriber(self):
        self.control.rows = [subscriber()]
        self.control.node_connected = False
        self.tick()
        self.assertFalse(self.router.posts)
        self.control.node_connected = True
        self.control.transport_connected = False
        self.tick(1)
        self.assertFalse(self.router.posts)
        self.control.transport_connected = True
        self.control.node_seen = NOW - 121
        self.tick(2)
        self.assertFalse(self.router.posts)
        self.control.node_seen = None
        self.tick(3)
        self.assertEqual(len(self.router.posts), 1)

    def test_control_or_node_api_failure_blocks_delivery(self):
        self.control.rows = [subscriber()]
        self.control.error = OSError("control room unavailable")
        self.tick()
        self.assertFalse(self.router.posts)
        self.assertIn("control_room", self.service.errors)
        self.control.error = None
        self.control.nodes_error = OSError("nodes unavailable")
        self.tick(1)
        self.assertFalse(self.router.posts)

    def test_old_router_fails_closed_until_required_features_are_available(self):
        self.control.rows = [subscriber()]
        self.router.ready = False
        self.tick()
        self.assertFalse(self.service.router_ready)
        self.assertFalse(self.router.posts)
        self.assertIn("sds_router", self.service.errors)
        self.router.ready = True
        self.tick(1)
        self.assertEqual(len(self.router.posts), 1)

    def test_timeout_after_acceptance_resolves_original_key_after_restart(self):
        self.control.rows = [subscriber()]
        self.router.timeout_once = True
        self.tick()
        self.assertEqual(len(self.router.messages), 1)
        self.assertEqual(self.store.deliveries()[0]["attempts"], 1)
        self.restart()
        self.tick(30)
        self.assertEqual(len(self.router.posts), 1)
        self.assertEqual(len(self.router.messages), 1)
        self.tick(31)
        self.assertEqual(self.store.deliveries()[0]["state"], "accepted")

    def test_timeout_before_acceptance_retries_identical_payload_and_key(self):
        self.control.rows = [subscriber()]
        self.router.timeout_before_once = True
        self.tick()
        self.assertEqual(len(self.router.messages), 0)
        self.assertEqual(self.store.deliveries()[0]["attempts"], 1)
        self.restart()
        self.tick(30)
        self.assertEqual(len(self.router.posts), 2)
        self.assertEqual(self.router.posts[0], self.router.posts[1])
        self.assertEqual(timestamp(self.router.posts[0]["expires_at"]), NOW + 300)
        self.assertEqual(len(self.router.messages), 1)
        self.tick(31)
        self.assertEqual(self.store.deliveries()[0]["state"], "accepted")

    def test_expired_never_submitted_reservation_is_renewed_when_still_relevant(self):
        self.store.ingest([warning()], complete=True, now=NOW)
        request = {"source_issi": 9999, "dest_issi": 1001, "text": "NINA: Rauchentwicklung",
                   "idempotency_key": "alert:never-submitted", "at_most_once": True,
                   "expires_at": iso(NOW - 1), "ttl_secs": 300, "force_nodes": ["old-tbs"]}
        self.store.enqueue("nina:incident-a", 1001, request, NOW - 1, NOW - 301)
        self.control.rows = [subscriber()]
        self.tick()
        self.assertEqual(len(self.router.posts), 1)
        self.assertEqual(self.router.posts[0]["idempotency_key"], request["idempotency_key"])
        self.assertEqual(timestamp(self.router.posts[0]["expires_at"]), NOW + 300)
        self.assertEqual(self.router.posts[0]["force_nodes"], ["tbs-one"])
        self.assertEqual(self.store.deliveries()[0]["state"], "submitted")

    def test_expired_failed_attempt_is_renewed_only_after_key_is_unknown(self):
        self.control.rows = [subscriber()]
        self.router.timeout_before_once = True
        self.tick()
        self.assertFalse(self.router.messages)
        original = copy.deepcopy(self.router.posts[0])
        self.restart()
        self.tick(301)
        self.assertEqual(len(self.router.posts), 2)
        self.assertEqual(self.router.posts[1]["idempotency_key"], original["idempotency_key"])
        self.assertEqual(timestamp(original["expires_at"]), NOW + 300)
        self.assertEqual(timestamp(self.router.posts[1]["expires_at"]), NOW + 601)
        self.assertEqual(len(self.router.messages), 1)

    def test_consumed_but_not_retained_router_key_is_never_renewed(self):
        self.control.rows = [subscriber()]
        self.router.timeout_once = True
        self.tick()
        original_expiry = self.store.deliveries()[0]["expires_at"]
        self.router.retained = False
        self.restart()
        self.tick(301)
        self.assertEqual(len(self.router.posts), 1)
        self.assertEqual(self.store.deliveries()[0]["state"], "uncertain")
        self.assertEqual(self.store.deliveries()[0]["expires_at"], original_expiry)
        self.restart()
        self.tick(302)
        self.assertEqual(len(self.router.posts), 1)

    def test_retained_expired_router_message_is_never_renewed(self):
        self.control.rows = [subscriber()]
        self.router.state = "expired"
        self.router.timeout_once = True
        self.tick()
        original_expiry = self.store.deliveries()[0]["expires_at"]
        self.tick(301)
        self.assertEqual(len(self.router.posts), 1)
        self.assertEqual(self.store.deliveries()[0]["state"], "expired")
        self.assertEqual(self.store.deliveries()[0]["expires_at"], original_expiry)

    def test_timeout_then_expiry_looks_up_and_cancels_without_reposting(self):
        self.control.rows = [subscriber()]
        self.router.state = "queued"
        self.router.timeout_once = True
        self.tick()
        self.tick(301)
        self.assertEqual(len(self.router.posts), 1)
        self.assertEqual(self.router.cancels, ["message-1"])
        self.assertEqual(self.store.deliveries()[0]["state"], "cancelled")

    def test_deleted_manual_warning_is_tombstoned_and_queued_job_is_cancelled(self):
        self.nina.result.alerts = []
        self.router.state = "queued"
        self.control.rows = [subscriber()]
        manual = self.service.create_manual({"title": "Straße gesperrt", "description": "Umleitung beachten",
            "latitude": 52.5, "longitude": 13.4, "radius_m": 1000, "expires_at": iso(NOW + 3600)}, now=NOW)
        self.tick()
        self.assertEqual(len(self.router.posts), 1)
        self.assertTrue(self.router.posts[0]["text"].startswith("EIGEN:"))
        self.service.delete_manual(manual["id"])
        self.tick(1)
        self.assertEqual(self.router.cancels, ["message-1"])
        self.restart()
        self.tick(2)
        self.assertEqual(len(self.router.posts), 1)
        self.assertTrue(self.store.alerts()[0]["removed"])
        self.assertEqual(len(self.store.deliveries()), 1)

    def test_nina_warning_cannot_be_deleted_as_manual(self):
        self.tick()
        with self.assertRaises(ValueError):
            self.service.delete_manual("nina:incident-a")

    def test_manual_input_bounds_and_text_encoding(self):
        data = {"title": "Test", "latitude": 52.5, "longitude": 13.4, "radius_m": 1000, "expires_at": iso(NOW + 3600)}
        for field, value in [("title", ""), ("radius_m", 49), ("latitude", float("inf")), ("expires_at", iso(NOW - 1)), ("severity", "anything")]:
            with self.subTest(field=field), self.assertRaises((ValueError, TypeError)):
                self.service.create_manual({**data, field: value}, now=NOW)
        text = radio_text({**warning(), "title": "Öl auf Straße", "instruction": "Lüftung schließen. " * 20}, 120)
        self.assertLessEqual(len(text), 120)
        self.assertIn("Oel auf Strasse", text)
        self.assertTrue(text.isascii())

    def test_disabled_delivery_and_nina_disable_do_not_enqueue(self):
        self.control.rows = [subscriber()]
        self.config["delivery"]["enabled"] = False
        self.tick()
        self.assertEqual(self.store.deliveries(), [])
        self.config["delivery"]["enabled"] = True
        self.config["nina"]["enabled"] = False
        self.tick(1)
        self.assertEqual(self.store.deliveries(), [])

    def test_alias_merge_preserves_attempted_history_over_unsent_conflict(self):
        # Initially disconnected chains A and B; only B was already delivered.
        self.store.ingest([warning("a"), warning("b")], complete=True, now=NOW)
        pending = {"idempotency_key": "a-key", "dest_issi": 1001, "text": "A"}
        sent = {"idempotency_key": "b-key", "dest_issi": 1001, "text": "B"}
        self.store.enqueue("nina:a", 1001, pending, NOW + 300, NOW)
        self.store.enqueue("nina:b", 1001, sent, NOW + 300, NOW)
        delivered = next(r for r in self.store.deliveries() if r["alert_id"] == "nina:b")
        self.store.update_delivery(delivered, state="accepted", attempts=1, router_id="already-sent")
        joined = warning("c", now=NOW + 1, aliases=["a", "b", "c"])
        joined["incident_id"] = "a"
        self.nina.result.alerts = [joined]
        self.control.rows = [subscriber()]
        self.tick(1)
        self.assertEqual(self.router.posts, [], "Joining alias chains must never retransmit an already delivered incident")
        self.assertTrue(any(row["state"] == "accepted" for row in self.store.deliveries()))
        self.restart()
        self.tick(2)
        self.assertEqual(self.router.posts, [])

    def test_alias_merge_retains_newer_cancellation_from_any_linked_chain(self):
        self.store.ingest([warning("a"), warning("b", now=NOW + 30, cancelled=True)], complete=True, now=NOW + 30)
        joined = warning("c", now=NOW + 20, aliases=["a", "b", "c"])
        joined["incident_id"] = "a"
        self.nina.result.alerts = [joined]
        self.control.rows = [subscriber()]
        self.tick(31)
        current = [a for a in self.store.alerts() if not a["removed"]]
        self.assertEqual(len(current), 1)
        self.assertTrue(current[0]["cancelled"], "An older linking update cannot undo a newer cancellation")
        self.assertEqual(current[0]["provider_id"], "b")
        self.assertEqual(self.router.posts, [])


if __name__ == "__main__":
    unittest.main()
