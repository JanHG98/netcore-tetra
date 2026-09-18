"""Synthetic CAP fixtures exercise the observed BBK shape without live traffic."""
import copy
from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from geometry import contains
from nina import NinaClient, normalize_alert, plain_text

BASE = "https://example.invalid/api31"
POLYGON = {"type": "FeatureCollection", "features": [{"type": "Feature", "geometry": {
    "type": "Polygon", "coordinates": [[[6, 49], [8, 49], [8, 51], [6, 51], [6, 49]]],
}}]}


def warning(identifier="mow.test-002", **extra):
    item = {
        "identifier": identifier, "sender": "test-authority", "sent": "2026-09-18T14:00:00+02:00",
        "status": "Actual", "scope": "Public", "msgType": "Alert",
        "info": [{"language": "de", "severity": "Severe", "headline": "Rauchentwicklung",
                  "description": "<p>Fenster &amp; Türen schließen.</p>", "instruction": "Radio einschalten.<br/>Innen bleiben.",
                  "area": [{"areaDesc": "Testgebiet"}]}],
    }
    item.update(extra)
    return item


class FakeFeed:
    def __init__(self, documents):
        self.documents, self.calls = documents, []

    def __call__(self, url):
        path = url.removeprefix(BASE)
        self.calls.append(path)
        result = self.documents[path]
        if isinstance(result, Exception):
            raise result
        return copy.deepcopy(result)


def fixtures(detail=None):
    detail = detail or warning()
    identifier = detail["identifier"]
    return {
        "/mowas/mapData.json": [{"id": identifier, "version": 1, "type": detail["msgType"]}],
        "/katwarn/mapData.json": [],
        f"/warnings/{identifier}.json": detail,
        f"/warnings/{identifier}.geojson": POLYGON,
    }


class NormalizationTests(unittest.TestCase):
    def test_live_shaped_warning_without_expiry(self):
        item = normalize_alert(warning(), geojson=POLYGON)
        self.assertEqual(item["starts_at"], "2026-09-18T12:00:00Z")
        self.assertIsNone(item["expires_at"])
        self.assertEqual(item["description"], "Fenster & Türen schließen.")
        self.assertEqual(item["instruction"], "Radio einschalten.\nInnen bleiben.")
        self.assertTrue(contains(item["geometry"], 50, 7))

    def test_updates_use_earliest_reference_and_return_all_aliases(self):
        item = normalize_alert(warning(msgType="Update", references=
            "sender,mow.test-001,2026-09-18T11:00:00Z sender,mow.test-original,2026-09-18T10:00:00Z"), geojson=POLYGON)
        self.assertEqual(item["incident_id"], "mow.test-original")
        self.assertEqual(set(item["aliases"]), {"mow.test-original", "mow.test-001", "mow.test-002"})

    def test_cancel_without_info_or_geometry_keeps_incident(self):
        item = normalize_alert(warning(msgType="Cancel", info=[], references="sender,mow.test-001,2026-09-18T11:00:00Z"))
        self.assertTrue(item["cancelled"])
        self.assertEqual(item["incident_id"], "mow.test-001")
        self.assertFalse(contains(item["geometry"], 50, 7))

    def test_only_actual_public_alerts(self):
        for field, value in [("status", "Test"), ("status", "Exercise"), ("status", "Draft"), ("scope", "Restricted"), ("scope", "Private"), ("msgType", "Ack")]:
            with self.subTest(field=field, value=value):
                self.assertIsNone(normalize_alert(warning(**{field: value})))

    def test_german_standard_info_precedes_simple_language(self):
        item = warning()
        item["info"].insert(0, {"language": "de-LS", "headline": "Einfache Sprache", "severity": "Minor"})
        item["info"].insert(0, {"language": "en", "headline": "English", "severity": "Minor"})
        self.assertEqual(normalize_alert(item, geojson=POLYGON)["title"], "Rauchentwicklung")

    def test_effective_time_expiry_and_cap_circle(self):
        item = warning()
        item["info"][0].update({"effective": "2026-09-18T15:00:00+02:00", "expires": "2026-09-18T17:00:00+02:00", "area": [{"circle": "50,7 2"}]})
        parsed = normalize_alert(item)
        self.assertEqual(parsed["starts_at"], "2026-09-18T13:00:00Z")
        self.assertEqual(parsed["expires_at"], "2026-09-18T15:00:00Z")
        self.assertEqual(parsed["geometry"]["radius_m"], 2000)
        item["info"][0]["expires"] = "2026-09-18T10:00:00Z"
        with self.assertRaises(ValueError):
            normalize_alert(item)

    def test_invalid_time_identifier_reference_and_missing_area(self):
        for item in (warning(sent="2026-09-18T12:00:00"), warning(identifier="../bad"), warning(references="malformed"), warning()):
            with self.subTest(item=item), self.assertRaises(ValueError):
                normalize_alert(item)

    def test_plain_text_discards_embedded_script_and_retains_entities(self):
        self.assertEqual(plain_text("<b>Warnung</b><script>alert(1)</script><br/>A &amp; B"), "Warnung\nA & B")


class ClientTests(unittest.TestCase):
    def test_cache_avoids_repeated_detail_and_geometry_requests(self):
        feed = FakeFeed(fixtures())
        client = NinaClient(BASE, fetch_json=feed)
        first = client.fetch()
        self.assertTrue(first.complete)
        self.assertEqual(len(first.alerts), 1)
        self.assertEqual(len(feed.calls), 4)
        first.alerts[0]["title"] = "Mutated caller data"
        second = client.fetch()
        self.assertEqual(len(feed.calls), 6)
        self.assertEqual(second.alerts[0]["title"], "Rauchentwicklung")
        feed.documents["/mowas/mapData.json"][0]["version"] = 2
        self.assertTrue(client.fetch().complete)
        self.assertEqual(len(feed.calls), 10)

    def test_empty_feed_is_success_but_failure_never_is(self):
        feed = FakeFeed({"/mowas/mapData.json": [], "/katwarn/mapData.json": []})
        client = NinaClient(BASE, fetch_json=feed)
        result = client.fetch()
        self.assertTrue(result.complete)
        self.assertEqual(result.alerts, [])
        feed.documents["/katwarn/mapData.json"] = OSError("network unavailable")
        result = client.fetch()
        self.assertFalse(result.complete)
        self.assertIn("network unavailable", result.errors[0])

    def test_failed_geometry_is_partial_and_retains_seen_identifier(self):
        documents = fixtures()
        documents["/warnings/mow.test-002.geojson"] = OSError("timeout")
        result = NinaClient(BASE, fetch_json=FakeFeed(documents)).fetch()
        self.assertFalse(result.complete)
        self.assertEqual(result.alerts, [])
        self.assertEqual(result.seen_ids, ["mow.test-002"])

    def test_cap_circle_and_cancel_require_no_geometry_request(self):
        for detail in (warning(msgType="Cancel", info=[]), warning(info=[{"headline": "Test", "severity": "Minor", "area": [{"circle": "50,7 1"}]}])):
            feed = FakeFeed(fixtures(detail))
            result = NinaClient(BASE, fetch_json=feed).fetch()
            self.assertTrue(result.complete, result.errors)
            self.assertEqual(len(result.alerts), 1)
            self.assertEqual(len(feed.calls), 3)

    def test_exercises_are_ignored_without_geometry_request(self):
        feed = FakeFeed(fixtures(warning(status="Exercise")))
        result = NinaClient(BASE, fetch_json=feed).fetch()
        self.assertTrue(result.complete)
        self.assertEqual(result.alerts, [])
        self.assertEqual(len(feed.calls), 3)

    def test_request_budget_makes_progress_in_following_poll(self):
        documents = fixtures()
        documents["/mowas/mapData.json"].append({"id": "mow.test-003", "version": 1, "type": "Alert"})
        documents["/warnings/mow.test-003.json"] = warning("mow.test-003")
        documents["/warnings/mow.test-003.geojson"] = POLYGON
        feed = FakeFeed(documents)
        client = NinaClient(BASE, fetch_json=feed, max_requests=4)
        first = client.fetch()
        self.assertFalse(first.complete)
        self.assertEqual(len(first.alerts), 1)
        self.assertEqual(len(feed.calls), 4)
        second = client.fetch()
        self.assertTrue(second.complete, second.errors)
        self.assertEqual(len(second.alerts), 2)
        self.assertEqual(len(feed.calls), 8)

    def test_index_bounds_and_malformed_entries_fail_closed(self):
        for index in ({"error": "maintenance"}, [{"id": "../../etc/passwd"}], [{"id": "a"}, {"id": "b"}]):
            feed = FakeFeed({"/mowas/mapData.json": index, "/katwarn/mapData.json": []})
            result = NinaClient(BASE, fetch_json=feed, max_alerts=1).fetch()
            self.assertFalse(result.complete)
            self.assertFalse(result.alerts)

    def test_katwarn_is_imported_from_the_bbk_katwarn_feed(self):
        documents = fixtures(warning("kat.test_public_topics"))
        documents["/katwarn/mapData.json"] = documents.pop("/mowas/mapData.json")
        documents["/mowas/mapData.json"] = []
        result = NinaClient(BASE, fetch_json=FakeFeed(documents)).fetch()
        self.assertTrue(result.complete)
        self.assertEqual(result.alerts[0]["source"], "nina")
        self.assertEqual(result.alerts[0]["provider"], "katwarn")

    def test_optional_nina_sources_use_the_same_cap_normalization(self):
        sources = ("mowas", "katwarn", "biwapp", "dwd", "lhp", "police")
        documents = {}
        for source in sources:
            identifier = source + ".test-warning"
            documents[f"/{source}/mapData.json"] = [{"id": identifier, "version": 1, "type": "Alert"}]
            documents[f"/warnings/{identifier}.json"] = warning(identifier)
            documents[f"/warnings/{identifier}.geojson"] = POLYGON
        result = NinaClient(BASE, sources=sources, fetch_json=FakeFeed(documents)).fetch()
        self.assertTrue(result.complete, result.errors)
        self.assertEqual({a["provider"] for a in result.alerts}, set(sources))
        self.assertEqual(len(result.alerts), 6)
        self.assertTrue(all(a["source"] == "nina" for a in result.alerts))

    def test_source_configuration_rejects_unknown_or_empty_sources(self):
        for sources in ((), ("unknown",), ("../warnings",)):
            with self.subTest(sources=sources), self.assertRaises(ValueError):
                NinaClient(BASE, sources=sources)

    def test_cancel_index_with_stale_detail_is_not_cached_as_active(self):
        documents = fixtures()
        documents["/mowas/mapData.json"][0]["type"] = "Cancel"
        result = NinaClient(BASE, fetch_json=FakeFeed(documents)).fetch()
        self.assertFalse(result.complete)
        self.assertFalse(result.alerts)


if __name__ == "__main__":
    unittest.main()
