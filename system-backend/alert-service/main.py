#!/usr/bin/env python3
"""NetCore warning service. Python 3.11+; no pip dependencies."""
import argparse
import hmac
import json
import logging
import math
import mimetypes
import os
import signal
import threading
import tomllib
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import unquote, urlsplit

from nina import NinaClient, ALLOWED_SOURCES
from service import AlertService
from store import timestamp

STATIC = Path(__file__).parent / "static"


def load_config(path):
    with open(path, "rb") as stream:
        cfg = tomllib.load(stream)
    for section in ("server", "storage", "netcore", "nina", "delivery"):
        cfg.setdefault(section, {})
    cfg["server"].setdefault("bind", "127.0.0.1")
    cfg["server"].setdefault("port", 8310)
    cfg["storage"].setdefault("database", "/var/lib/netcore-alert-service/alerts.sqlite3")
    nc = cfg["netcore"]
    nc.setdefault("poll_seconds", 5)
    nc.setdefault("gps_max_age_seconds", 3600)
    nc.setdefault("node_max_age_seconds", 120)
    nc.setdefault("http_timeout_seconds", 10)
    for key in ("control_room_url", "sds_router_url"):
        parsed = urlsplit(nc[key])
        if parsed.scheme not in {"http", "https"} or not parsed.hostname or parsed.username or parsed.query or parsed.fragment:
            raise ValueError(f"Ungültige URL: netcore.{key}")
    if isinstance(nc.get("source_issi"), bool) or not isinstance(nc.get("source_issi"), int) or not 1 <= nc["source_issi"] <= 16777215:
        raise ValueError("netcore.source_issi muss eine gültige reservierte ISSI sein")
    cfg["delivery"].setdefault("enabled", False)
    cfg["delivery"].setdefault("ttl_seconds", 300)
    cfg["delivery"].setdefault("max_text_length", 120)
    cfg["nina"].setdefault("enabled", True)
    cfg["nina"].setdefault("sources", ["mowas", "katwarn", "biwapp", "dwd", "lhp"])
    if not isinstance(cfg["nina"]["sources"], list) or not cfg["nina"]["sources"] or any(source not in ALLOWED_SOURCES for source in cfg["nina"]["sources"]):
        raise ValueError("nina.sources muss eine Liste unterstützter BBK-Feeds sein")
    cfg["nina"].setdefault("poll_seconds", 60)
    cfg["nina"].setdefault("max_stale_seconds", 300)
    for section, key, minimum, maximum in (
        ("server", "port", 1, 65535), ("netcore", "poll_seconds", 1, 300),
        ("netcore", "gps_max_age_seconds", 1, 86400 * 30), ("netcore", "http_timeout_seconds", 1, 60),
        ("netcore", "node_max_age_seconds", 10, 3600),
        ("nina", "poll_seconds", 30, 86400), ("nina", "max_stale_seconds", 60, 86400),
        ("delivery", "ttl_seconds", 1, 86400), ("delivery", "max_text_length", 20, 200)):
        value = cfg[section][key]
        if isinstance(value, bool) or not isinstance(value, (int, float)) or not math.isfinite(value) or not minimum <= value <= maximum or int(value) != value:
            raise ValueError(f"{section}.{key} muss zwischen {minimum} und {maximum} liegen")
        cfg[section][key] = int(value)
    for section, key in (("delivery", "enabled"), ("nina", "enabled"), ("server", "allow_unauthenticated")):
        if key in cfg[section] and not isinstance(cfg[section][key], bool):
            raise ValueError(f"{section}.{key} muss true oder false sein")
    token = os.environ.get("NETCORE_ALERT_TOKEN", cfg["server"].get("admin_token", ""))
    if not isinstance(token, str) or (not cfg["server"].get("allow_unauthenticated", False) and len(token) < 24):
        raise ValueError("NETCORE_ALERT_TOKEN muss mindestens 24 Zeichen lang sein")
    cfg["server"]["admin_token"] = token
    return cfg


def handler_for(service):
    class Handler(BaseHTTPRequestHandler):
        server_version = "NetCoreAlert/1.0"

        def setup(self):
            super().setup()
            self.connection.settimeout(15)

        def log_message(self, fmt, *args):
            # Never log headers, request bodies or credentials.
            logging.getLogger("netcore-alert-http").info(fmt, *args)

        def send(self, status, data, content_type="application/json; charset=utf-8"):
            if not isinstance(data, bytes):
                data = json.dumps(data, ensure_ascii=False, allow_nan=False).encode("utf-8")
            self.send_response(status)
            self.send_header("Content-Type", content_type)
            self.send_header("Content-Length", str(len(data)))
            self.send_header("Cache-Control", "no-store")
            self.send_header("X-Content-Type-Options", "nosniff")
            # OSM requires a valid Referer; send only our origin cross-site, never
            # tokens (which are headers only) or application paths.
            self.send_header("Referrer-Policy", "strict-origin-when-cross-origin")
            self.send_header("Content-Security-Policy", "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https://tile.openstreetmap.org; connect-src 'self'; frame-ancestors 'none'; base-uri 'none'; form-action 'self'")
            self.end_headers()
            self.wfile.write(data)

        def authorized(self):
            cfg = service.config["server"]
            if cfg.get("allow_unauthenticated", False):
                return True
            expected = "Bearer " + cfg["admin_token"]
            return hmac.compare_digest(self.headers.get("Authorization", "").encode(), expected.encode())

        def read_json(self):
            if self.headers.get("Transfer-Encoding"):
                raise ValueError("Chunked requests sind nicht erlaubt")
            length = int(self.headers.get("Content-Length", "0"))
            if not 1 <= length <= 16384:
                raise ValueError("JSON-Anfrage darf höchstens 16 KiB groß sein")
            if self.headers.get_content_type() != "application/json":
                raise ValueError("Content-Type application/json erforderlich")
            raw = self.rfile.read(length)
            if len(raw) != length:
                raise ValueError("Unvollständige Anfrage")
            data = json.loads(raw, parse_constant=lambda _: (_ for _ in ()).throw(ValueError("Ungültige Zahl")))
            if not isinstance(data, dict):
                raise ValueError("JSON-Objekt erwartet")
            return data

        def do_GET(self):
            path = unquote(urlsplit(self.path).path)
            if path == "/health/live":
                return self.send(200, {"status": "live"})
            if path == "/health/ready":
                ready = service.last_cycle is not None and timestamp() - service.last_cycle < max(120, service.config["netcore"].get("poll_seconds", 5) * 3) and not service.errors
                return self.send(200 if ready else 503, {"status": "ready" if ready else "degraded"})
            if path.startswith("/api/"):
                if not self.authorized():
                    return self.send(401, {"error": "Zugriffsschlüssel erforderlich"})
                snapshot = service.snapshot()
                if path == "/api/v1/status":
                    return self.send(200, snapshot)
                collections = {"/api/v1/alerts": "alerts", "/api/v1/deliveries": "deliveries", "/api/v1/devices": "devices"}
                if path in collections:
                    return self.send(200, snapshot[collections[path]])
                return self.send(404, {"error": "Nicht gefunden"})
            target = STATIC / ("index.html" if path == "/" else path.lstrip("/"))
            target = target.resolve()
            if not target.is_relative_to(STATIC.resolve()) or not target.is_file():
                return self.send(404, {"error": "Nicht gefunden"})
            kind = {".js": "text/javascript", ".css": "text/css", ".html": "text/html"}.get(target.suffix) or mimetypes.guess_type(target.name)[0] or "application/octet-stream"
            return self.send(200, target.read_bytes(), kind)

        def do_POST(self):
            if not self.authorized():
                return self.send(401, {"error": "Zugriffsschlüssel erforderlich"})
            if urlsplit(self.path).path != "/api/v1/alerts":
                return self.send(404, {"error": "Nicht gefunden"})
            try:
                return self.send(201, service.create_manual(self.read_json()))
            except (ValueError, KeyError, TypeError, OverflowError) as exc:
                return self.send(400, {"error": str(exc)})

        def do_DELETE(self):
            if not self.authorized():
                return self.send(401, {"error": "Zugriffsschlüssel erforderlich"})
            path = unquote(urlsplit(self.path).path)
            if not path.startswith("/api/v1/alerts/"):
                return self.send(404, {"error": "Nicht gefunden"})
            try:
                service.delete_manual(path[len("/api/v1/alerts/"):])
                return self.send(200, {"deleted": True, "history_preserved": True})
            except KeyError:
                return self.send(404, {"error": "Meldung nicht gefunden"})
            except ValueError as exc:
                return self.send(400, {"error": str(exc)})

    return Handler


def main():
    parser = argparse.ArgumentParser(description="NetCore NINA/KATWARN Warnungsdienst")
    parser.add_argument("--config", default="/etc/netcore/alert-service.toml")
    parser.add_argument("--check-config", action="store_true")
    args = parser.parse_args()
    logging.basicConfig(level=logging.INFO, format="%(asctime)s %(levelname)s %(name)s %(message)s")
    cfg = load_config(args.config)
    if args.check_config:
        print("Konfiguration gültig")
        return
    nina = NinaClient(base_url=cfg["nina"].get("base_url", "https://warnung.bund.de/api31"), timeout=cfg["netcore"]["http_timeout_seconds"], sources=cfg["nina"]["sources"]) if cfg["nina"]["enabled"] else None
    service = AlertService(cfg, nina=nina)
    server = ThreadingHTTPServer((cfg["server"]["bind"], cfg["server"]["port"]), handler_for(service))
    server.daemon_threads = True
    worker = threading.Thread(target=service.run, name="alert-worker", daemon=True)
    worker.start()

    def shutdown(*_):
        service.stop.set()
        threading.Thread(target=server.shutdown, daemon=True).start()

    signal.signal(signal.SIGTERM, shutdown)
    signal.signal(signal.SIGINT, shutdown)
    logging.info("WebUI auf %s:%s; Versand %s", cfg["server"]["bind"], cfg["server"]["port"], "aktiv" if cfg["delivery"]["enabled"] else "deaktiviert")
    try:
        server.serve_forever(poll_interval=0.5)
    finally:
        service.stop.set()
        server.server_close()
        worker.join(timeout=60)
        # An interrupted network call may still use the store; process exit will
        # close it safely if the bounded shutdown deadline was reached.
        if not worker.is_alive():
            service.store.close()


if __name__ == "__main__":
    main()
