"""Match active warnings to online GPS radios and submit durable one-shot SDS jobs."""
import base64
import hashlib
import json
import logging
import math
import os
import re
import threading
import unicodedata
import uuid
from urllib.error import HTTPError
from urllib.parse import quote
from urllib.request import Request, urlopen

from geometry import contains
from store import Store, iso, timestamp

LOG = logging.getLogger("netcore-alert-service")
TERMINAL = {"accepted", "cancelled", "expired", "failed", "uncertain"}


class HttpClient:
    def __init__(self, base_url, timeout=10, username="", password=""):
        self.base_url = base_url.rstrip("/")
        self.timeout = timeout
        self.auth = "Basic " + base64.b64encode(f"{username}:{password}".encode()).decode() if username else None

    def request(self, method, path, data=None):
        headers = {"Accept": "application/json", "User-Agent": "NetCore-Alert-Service/1.0"}
        if self.auth:
            headers["Authorization"] = self.auth
        if data is not None:
            headers["Content-Type"] = "application/json"
        request = Request(self.base_url + path, data=json.dumps(data).encode() if data is not None else None,
                          headers=headers, method=method)
        with urlopen(request, timeout=self.timeout) as response:
            raw = response.read(8 * 1024 * 1024 + 1)
            if len(raw) > 8 * 1024 * 1024:
                raise ValueError("API-Antwort überschreitet 8 MiB")
            return json.loads(raw) if raw else {}


def active(alert, now):
    return (not alert.get("removed") and not alert.get("cancelled")
            and timestamp(alert["starts_at"]) <= now
            and (alert.get("expires_at") is None or timestamp(alert["expires_at"]) > now))


def radio_text(alert, limit=120):
    # The existing SDS text encoder uses alphabet 1. ASCII avoids mismatched UTF-8
    # and radio character sets; complete original text remains in the web UI.
    prefix = "EIGEN" if alert["source"] == "manual" else "NINA"
    text = f"{prefix}: {alert['title']}"
    detail = alert.get("instruction") or alert.get("description") or ""
    if detail:
        text += ". " + detail
    for a, b in {"ä": "ae", "ö": "oe", "ü": "ue", "Ä": "Ae", "Ö": "Oe", "Ü": "Ue", "ß": "ss"}.items():
        text = text.replace(a, b)
    text = unicodedata.normalize("NFKD", text).encode("ascii", "ignore").decode()
    text = re.sub(r"\s+", " ", text).strip()
    return text if len(text) <= limit else text[:limit - 3].rstrip() + "..."


class AlertService:
    def __init__(self, config, store=None, control=None, router=None, nina=None):
        self.config = config
        self.store = store or Store(config["storage"]["database"])
        self.lock = threading.RLock()
        self.work_lock = threading.Lock()
        nc = config["netcore"]
        self.control = control or HttpClient(nc["control_room_url"], nc.get("http_timeout_seconds", 10),
                                             nc.get("control_room_username", ""),
                                             os.environ.get("NETCORE_CONTROL_ROOM_PASSWORD", nc.get("control_room_password", "")))
        self.router = router or HttpClient(nc["sds_router_url"], nc.get("http_timeout_seconds", 10))
        self.nina = nina
        self.devices = []
        self.device_diagnostics = []
        self._device_node_ages = {}
        self.subscribers_seen = 0
        self.errors = {}
        self.last_cycle = None
        self.last_nina_poll = 0
        self.router_ready = False
        self.stop = threading.Event()

    def create_manual(self, data, now=None):
        now = timestamp(now)
        title = str(data.get("title", "")).strip()
        description = str(data.get("description", "")).strip()
        if not 1 <= len(title) <= 160 or len(description) > 4000:
            raise ValueError("Titel: 1–160 Zeichen, Meldung: höchstens 4000 Zeichen")
        lat, lon, radius = float(data["latitude"]), float(data["longitude"]), float(data["radius_m"])
        if not all(math.isfinite(v) for v in (lat, lon, radius)) or not (-90 <= lat <= 90 and -180 <= lon <= 180 and 50 <= radius <= 200000):
            raise ValueError("Ungültige Koordinaten oder Radius (50–200000 m)")
        expires = timestamp(data["expires_at"])
        if not now + 10 < expires <= now + 366 * 86400:
            raise ValueError("Ablauf muss zwischen 10 Sekunden und 366 Tagen in der Zukunft liegen")
        severity = data.get("severity", "Moderate")
        if severity not in {"Minor", "Moderate", "Severe", "Extreme"}:
            raise ValueError("Ungültige Warnstufe")
        record = {"id": "manual:" + uuid.uuid4().hex, "source": "manual", "provider": "NetCore",
                  "title": title, "description": description, "instruction": "", "severity": severity,
                  "starts_at": iso(now), "expires_at": iso(expires), "sent_at": iso(now), "cancelled": False,
                  "geometry": {"type": "Circle", "coordinates": [lon, lat], "radius_m": radius}}
        with self.lock:
            self.store.add_manual(record)
        return record

    def delete_manual(self, alert_id):
        with self.lock:
            self.store.remove_manual(alert_id)
            # The worker observes the tombstone before any further submission.
            # Preserve history permanently, including for deleted warnings.

    def _devices(self, snapshot, now, nodes):
        if not isinstance(snapshot, dict) or not isinstance(snapshot.get("subscribers"), list):
            raise ValueError("Ungültige Teilnehmerantwort der Leitstelle")
        if not isinstance(nodes, list):
            raise ValueError("Ungültige TBS-Antwort der Leitstelle")
        online_nodes = set()
        node_reasons, node_ages = {}, {}
        for node in nodes:
            if not isinstance(node, dict):
                continue
            try:
                if not isinstance(node.get("last_seen"), str) or not node["last_seen"]:
                    node_reasons[node["node_id"]] = ("node_timestamp_missing", "Zeitpunkt der letzten TBS-Verbindung fehlt")
                    continue
                age = now - timestamp(node["last_seen"])
                node_ages[node["node_id"]] = int(age)
                if node.get("connected") is True and node.get("transport_connected") is not False and -60 <= age <= self.config["netcore"].get("node_max_age_seconds", 120):
                    online_nodes.add(node["node_id"])
                elif node.get("connected") is not True or node.get("transport_connected") is False:
                    node_reasons[node["node_id"]] = ("node_offline", "TBS ist nicht verbunden")
                elif age < -60:
                    node_reasons[node["node_id"]] = ("node_timestamp_future", "TBS-Zeitpunkt liegt in der Zukunft; Uhrzeit prüfen")
                else:
                    limit = self.config["netcore"].get("node_max_age_seconds", 120)
                    node_reasons[node["node_id"]] = ("node_stale", f"Letzte TBS-Meldung ist älter als {limit} Sekunden")
            except (KeyError, TypeError, ValueError, OverflowError):
                if isinstance(node, dict) and isinstance(node.get("node_id"), str):
                    node_reasons[node["node_id"]] = ("node_timestamp_invalid", "Zeitpunkt der letzten TBS-Verbindung ist ungültig")
                continue
        diagnostics = []

        def reject(row, code, reason):
            row = row if isinstance(row, dict) else {}
            node_id = row.get("node_id") if isinstance(row.get("node_id"), str) else None
            try:
                issi = int(row["issi"])
            except (KeyError, TypeError, ValueError, OverflowError):
                issi = None
            entry = {"issi": issi, "node_id": node_id, "reason_code": code, "reason": reason}
            if node_id in node_ages:
                entry["node_age_seconds"] = node_ages[node_id]
            location = row.get("last_location") or {}
            try:
                if isinstance(location, dict) and isinstance(location.get("updated_at"), str) and location["updated_at"]:
                    entry["gps_age_seconds"] = int(now - timestamp(location["updated_at"]))
            except (TypeError, ValueError, OverflowError):
                pass
            diagnostics.append(entry)

        newest = {}
        for row in snapshot["subscribers"]:
            if not isinstance(row, dict):
                reject(row, "device_data_invalid", "Teilnehmerdaten sind ungültig")
                continue
            try:
                if row.get("online") is not True:
                    reject(row, "subscriber_offline", "Funkgerät ist nicht als online gemeldet")
                    continue
                if row.get("node_id") not in online_nodes:
                    code, reason = node_reasons.get(row.get("node_id"), ("node_missing", "Zugehörige TBS fehlt in der Leitstellenantwort"))
                    reject(row, code, reason)
                    continue
                location = row.get("last_location") or {}
                if not isinstance(location, dict):
                    reject(row, "device_data_invalid", "GPS-Daten sind ungültig")
                    continue
                if not isinstance(location.get("updated_at"), str) or not location["updated_at"]:
                    reject(row, "gps_missing", "Keine GPS-Position mit gültigem Zeitpunkt vorhanden")
                    continue
                age = now - timestamp(location["updated_at"])
                lat, lon, issi = float(location["latitude"]), float(location["longitude"]), int(row["issi"])
                if not (math.isfinite(lat) and math.isfinite(lon) and -90 <= lat <= 90 and -180 <= lon <= 180 and 1 <= issi <= 16777215):
                    reject(row, "device_data_invalid", "GPS-Koordinaten oder ISSI sind ungültig")
                    continue
                if age < -60 or age > self.config["netcore"].get("gps_max_age_seconds", 3600):
                    if age < -60:
                        reject(row, "gps_timestamp_future", "GPS-Zeitpunkt liegt in der Zukunft; Uhrzeit prüfen")
                    else:
                        limit = self.config["netcore"].get("gps_max_age_seconds", 3600)
                        reject(row, "gps_stale", f"GPS-Position ist älter als {limit} Sekunden")
                    continue
                item = {"issi": issi, "node_id": row["node_id"], "latitude": lat, "longitude": lon,
                        "updated_at": location["updated_at"], "age_seconds": max(0, int(age))}
                if issi not in newest or timestamp(item["updated_at"]) > timestamp(newest[issi]["updated_at"]):
                    if issi in newest:
                        previous = newest[issi]
                        reject({"issi": issi, "node_id": previous["node_id"], "last_location": previous},
                               "duplicate_position", "Für diese ISSI wird eine neuere GPS-Position verwendet")
                    newest[issi] = item
                else:
                    reject(row, "duplicate_position", "Für diese ISSI wird eine gleich neue oder neuere GPS-Position verwendet")
            except (KeyError, TypeError, ValueError, OverflowError):
                reject(row, "device_data_invalid", "GPS-Daten oder ISSI fehlen oder sind ungültig")
                continue
        self.device_diagnostics = diagnostics
        self._device_node_ages = node_ages
        self.subscribers_seen = len(snapshot["subscribers"])
        return list(newest.values())

    def _eligible(self, alert, now):
        if not active(alert, now):
            return False
        if alert["source"] == "nina":
            return self.config["nina"].get("enabled", True) and now - self.store.meta("nina_success", 0) <= self.config["nina"].get("max_stale_seconds", 300)
        return True

    def tick(self, now=None):
        # Provider/network reads do not block WebUI snapshots or editing. Only an
        # individual send is serialized with deletion; SQLite commits before I/O.
        with self.work_lock:
            self._tick(now)

    def _tick(self, supplied_now=None):
        now = timestamp(supplied_now)
        if self.nina and now - self.last_nina_poll >= self.config["nina"].get("poll_seconds", 60):
            self.last_nina_poll = now
            try:
                result = self.nina.fetch()
                self.store.ingest(result.alerts, complete=result.complete, now=now)
                if result.errors:
                    self.errors["nina"] = "; ".join(result.errors)[:1200]
                else:
                    self.errors.pop("nina", None)
            except Exception as exc:
                self.errors["nina"] = str(exc)[:1200]
                LOG.exception("NINA-Abruf fehlgeschlagen")
        try:
            subscribers = self.control.request("GET", "/api/subscribers?online=true")
            nodes = self.control.request("GET", "/api/nodes")
            now = timestamp(supplied_now)
            self.devices = self._devices(subscribers, now, nodes)
            self.errors.pop("control_room", None)
        except Exception as exc:
            self.devices = []
            self.device_diagnostics = []
            self._device_node_ages = {}
            self.subscribers_seen = 0
            self.errors["control_room"] = str(exc)[:500]
        try:
            status = self.router.request("GET", "/api/v1/status")
            self.router_ready = status.get("durable_idempotency") is True and status.get("at_most_once") is True
            if not self.router_ready:
                raise ValueError("SDS-Router benötigt das Update mit dauerhafter Duplikatsperre")
            self.errors.pop("sds_router", None)
        except Exception as exc:
            self.router_ready = False
            self.errors["sds_router"] = str(exc)[:500]
        if self.router_ready:
            self._dispatch(now)
        self.last_cycle = now

    def _dispatch(self, now):
        alerts = {a["id"]: a for a in self.store.alerts()}
        delivery = self.config["delivery"]
        enabled = delivery.get("enabled", False)
        devices = {d["issi"]: d for d in self.devices}
        if enabled:
            for alert in alerts.values():
                if not self._eligible(alert, now):
                    continue
                for device in devices.values():
                    if not contains(alert["geometry"], device["latitude"], device["longitude"]):
                        continue
                    expires = min(timestamp(alert["expires_at"]) if alert.get("expires_at") else now + delivery.get("ttl_seconds", 300),
                                  now + delivery.get("ttl_seconds", 300))
                    ttl = int(expires - now)
                    if ttl < 1:
                        continue
                    key = "alert:" + hashlib.sha256(f"{alert['id']}:{device['issi']}".encode()).hexdigest()
                    request = {"source_issi": self.config["netcore"]["source_issi"], "dest_issi": device["issi"],
                               "is_group": False, "sds_type": 4, "protocol_id": 130,
                               "text": radio_text(alert, delivery.get("max_text_length", 120)),
                               "ttl_secs": ttl, "priority": 2, "ingress": "alert-service",
                               "expires_at": iso(expires), "idempotency_key": key, "at_most_once": True,
                               "force_nodes": [device["node_id"]]}
                    self.store.enqueue(alert["id"], device["issi"], request, expires, now)
        for row in self.store.deliveries():
            with self.lock:
                self._deliver(row, devices, enabled, now)

    def _deliver(self, row, devices, enabled, now):
        if row["state"] in TERMINAL:
            return
        # Re-read because a user may have deleted a warning during the cycle.
        alert = self.store.alert(self.store.canonical_id(row["alert_id"]))
        expired = row["expires_at"] <= now
        withdrawn = row["suppressed"] or not alert or not active(alert, now)
        if row["next_attempt"] > now and not (expired or withdrawn):
            return
        try:
            # Resolve any ambiguous submission before considering another POST,
            # even when the radio went offline or left the area in the meantime.
            if not row["router_id"] and row["attempts"]:
                try:
                    found = self.router.request("GET", "/api/v1/idempotency/" + quote(row["idempotency_key"], safe=":"))
                    if found.get("message_id"):
                        if not found.get("retained"):
                            self.store.update_delivery(row, state="uncertain", last_error="Router-Historie bereits bereinigt; keine erneute Sendung", updated_at=now)
                            return
                        row["router_id"] = found["message_id"]
                        self.store.update_delivery(row, router_id=row["router_id"], state="submitted")
                except HTTPError as exc:
                    exc.close()
                    if exc.code != 404:
                        raise
            if row["router_id"]:
                self._reconcile(row, expired or withdrawn, now)
                return
            if withdrawn:
                self.store.update_delivery(row, state="expired" if expired else "cancelled", updated_at=now)
                return
            if expired:
                # No accepted/consumed router key exists. A request delayed
                # from the old reservation now fails its absolute deadline.
                # Keep the same key, but give a still-relevant warning another
                # opportunity when the device returns after a network outage.
                device = devices.get(row["issi"])
                if not enabled or not self._eligible(alert, now) or device is None or not contains(alert["geometry"], device["latitude"], device["longitude"]):
                    return
                delivery = self.config["delivery"]
                expires = min(timestamp(alert["expires_at"]) if alert.get("expires_at") else now + delivery.get("ttl_seconds", 300), now + delivery.get("ttl_seconds", 300))
                if expires - now < 1:
                    return
                request = json.loads(row["request_json"])
                request.update(expires_at=iso(expires), ttl_secs=int(expires-now), force_nodes=[device["node_id"]])
                self.store.renew_unsubmitted(row, request, expires, now)
                row["request_json"] = json.dumps(request)
                row["expires_at"] = expires
            if enabled and alert and self._eligible(alert, now):
                device = devices.get(row["issi"])
                if device is None or not contains(alert["geometry"], device["latitude"], device["longitude"]):
                    return
                self.store.update_delivery(row, state="pending", attempts=row["attempts"] + 1, updated_at=now)
                result = self.router.request("POST", "/api/v1/messages", json.loads(row["request_json"]))
                if not result.get("id"):
                    raise ValueError("SDS-Router hat keine Nachrichten-ID geliefert")
                self.store.update_delivery(row, state="submitted", router_id=result["id"], last_error=None, updated_at=now)
        except HTTPError as exc:
            exc.close()
            permanent = 400 <= exc.code < 500 and exc.code not in {408, 429}
            self.store.update_delivery(row, state="uncertain" if exc.code in {404, 409} else "failed" if permanent else row["state"],
                                       last_error=f"SDS-Router HTTP {exc.code}", next_attempt=now + 30, updated_at=now)
        except Exception as exc:
            self.store.update_delivery(row, last_error=str(exc)[:500], next_attempt=now + min(60, 5 * 2 ** min(row["attempts"], 4)), updated_at=now)

    def _reconcile(self, row, cancel, now):
        path = "/api/v1/messages/" + quote(row["router_id"], safe="")
        result = self.router.request("GET", path)
        state = result.get("state", "")
        if state == "delivered":
            self.store.update_delivery(row, state="accepted", last_error=None, updated_at=now)
        elif state in {"failed", "dead_letter", "expired", "cancelled"}:
            self.store.update_delivery(row, state="uncertain" if state == "dead_letter" else state,
                                       last_error=result.get("last_error"), updated_at=now)
        elif cancel:
            self.router.request("POST", path + "/cancel", {})
            self.store.update_delivery(row, state="cancelled", updated_at=now)

    def _device_overview(self, alerts, deliveries):
        """Join the current device assessment with warning and delivery history.

        This is read-only presentation data. Matching, reservations and retries
        remain exclusively owned by the existing dispatch worker.
        """
        canonical = {}

        def canonical_id(alert_id):
            if alert_id not in canonical:
                canonical[alert_id] = self.store.canonical_id(alert_id)
            return canonical[alert_id]

        def delivery_rank(delivery):
            # An alias merge can retain several original delivery keys. Preserve
            # evidence of an accepted/attempted send over an unused reservation.
            return (delivery["state"] == "accepted",
                    delivery["attempts"] > 0 or bool(delivery["router_id"]),
                    not delivery["suppressed"], delivery["updated_at"])

        history = {}
        for delivery in deliveries:
            key = (canonical_id(delivery["alert_id"]), delivery["issi"])
            if key not in history or delivery_rank(delivery) > delivery_rank(history[key]):
                history[key] = delivery

        overview = {}
        for diagnostic in self.device_diagnostics:
            issi, node_id = diagnostic["issi"], diagnostic["node_id"]
            key = issi if issi is not None else ("unknown", node_id)
            overview.setdefault(key, {
                "issi": issi, "node_id": node_id, "available": False,
                "reason_code": diagnostic["reason_code"], "reason": diagnostic["reason"],
                "gps_age_seconds": diagnostic.get("gps_age_seconds"),
                "node_age_seconds": diagnostic.get("node_age_seconds"),
                "area_status": "unknown", "matching_alerts": [],
            })
        active_alerts = [alert for alert in alerts if alert["active"]]
        for device in self.devices:
            matches = []
            for alert in active_alerts:
                if not contains(alert["geometry"], device["latitude"], device["longitude"]):
                    continue
                delivery = history.get((canonical_id(alert["id"]), device["issi"]))
                eligibility_reason = None
                if not alert["eligible"]:
                    eligibility_reason = ("NINA-Abruf ist deaktiviert" if not self.config["nina"].get("enabled", True)
                                          else "NINA-Daten sind nicht aktuell genug für neue Zustellungen")
                matches.append({
                    "id": alert["id"], "title": alert["title"], "eligible": alert["eligible"],
                    "eligibility_reason": eligibility_reason,
                    "delivery_state": delivery["state"] if delivery else None,
                    "delivery_error": delivery["last_error"] if delivery else None,
                    "delivery_updated_at": delivery["updated_at"] if delivery else None,
                })
            # A usable current position always supersedes rejected duplicate rows
            # for the same ISSI; show each known radio only once.
            overview[device["issi"]] = {
                "issi": device["issi"], "node_id": device["node_id"], "available": True,
                "reason_code": "ready", "reason": "Online mit aktueller GPS-Position",
                "gps_age_seconds": device["age_seconds"],
                "node_age_seconds": self._device_node_ages.get(device["node_id"]),
                "latitude": device["latitude"], "longitude": device["longitude"],
                "updated_at": device["updated_at"],
                "area_status": "affected" if matches else "outside" if active_alerts else "no_alerts",
                "matching_alerts": matches,
            }
        rows = sorted(overview.values(), key=lambda row: (row["issi"] is None, row["issi"] or 0, row["node_id"] or ""))
        available = sum(row["available"] for row in rows)
        summary = {"total": len(rows), "available": available,
                   "affected": sum(row["area_status"] == "affected" for row in rows),
                   "blocked": len(rows) - available}
        return rows, summary

    def snapshot(self):
        # Keep alias resolution and the matching history on one database view.
        with self.lock, self.store.lock:
            now = timestamp()
            alerts = [{**a, "active": active(a, now), "eligible": self._eligible(a, now),
                       "radio_text": radio_text(a, self.config["delivery"].get("max_text_length", 120))} for a in self.store.alerts()]
            deliveries = self.store.deliveries(public=True)
            device_overview, device_summary = self._device_overview(alerts, deliveries)
            return {"service": "netcore-alert-service", "now": iso(now), "last_cycle": self.last_cycle,
                    "delivery_enabled": self.config["delivery"].get("enabled", False), "router_ready": self.router_ready,
                    "nina_last_success": self.store.meta("nina_success"), "errors": dict(self.errors),
                    "alerts": alerts, "devices": list(self.devices),
                    "device_diagnostics": list(self.device_diagnostics), "subscribers_seen": self.subscribers_seen,
                    "device_overview": device_overview, "device_summary": device_summary,
                    "deliveries": deliveries}

    def run(self):
        while not self.stop.is_set():
            try:
                self.tick()
                self.errors.pop("worker", None)
            except Exception as exc:
                LOG.exception("Warnungszyklus fehlgeschlagen")
                self.errors["worker"] = str(exc)[:500]
            self.stop.wait(self.config["netcore"].get("poll_seconds", 5))
