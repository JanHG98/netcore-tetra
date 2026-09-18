"""Bounded, offline-testable adapter for the BBK NINA feeds.

Feeds: https://warnung.bund.de/api31/{mowas,katwarn}/mapData.json
Details/areas: /warnings/{identifier}.{json,geojson}
CAP semantics: https://docs.oasis-open.org/emergency/cap/v1.2/CAP-v1.2-os.html

This imports warnings made available by BBK; it is not an independent KATWARN
subscription and does not guarantee that every KATWARN message is available.
BBK's app endpoints are an upstream dependency whose format may change.
"""
from __future__ import annotations

import copy
from dataclasses import dataclass, field
from datetime import datetime, timezone
from html.parser import HTMLParser
import json
import re
import time
from typing import Callable, Any
from urllib.parse import quote, urlsplit
from urllib.request import Request, urlopen

from geometry import cap_geometry, validate_geometry

DEFAULT_BASE_URL = "https://warnung.bund.de/api31"
ALLOWED_SOURCES = frozenset(("mowas", "katwarn", "biwapp", "dwd", "lhp", "police"))
SEVERITIES = frozenset(("Extreme", "Severe", "Moderate", "Minor", "Unknown"))
_ID_PATTERN = re.compile(r"[A-Za-z0-9][A-Za-z0-9._~-]{0,255}\Z")


class ProviderError(Exception):
    """Upstream failure; callers must retain their last valid snapshot."""


@dataclass
class FetchResult:
    alerts: list[dict] = field(default_factory=list)
    complete: bool = True
    errors: list[str] = field(default_factory=list)
    seen_ids: list[str] = field(default_factory=list)


class _PlainText(HTMLParser):
    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.parts = []
        self.ignored = 0

    def handle_starttag(self, tag, attrs):
        if tag in ("script", "style"):
            self.ignored += 1
        elif tag in ("br", "p", "div", "li") and not self.ignored:
            self.parts.append("\n")

    def handle_endtag(self, tag):
        if tag in ("script", "style"):
            self.ignored = max(0, self.ignored - 1)
        elif tag in ("p", "div", "li") and not self.ignored:
            self.parts.append("\n")

    def handle_data(self, data):
        if not self.ignored:
            self.parts.append(data)


def plain_text(value: Any) -> str:
    if value is None:
        return ""
    if not isinstance(value, str) or len(value) > 128_000:
        raise ValueError("Invalid or oversized warning text")
    parser = _PlainText()
    parser.feed(value)
    return "\n".join(line.strip() for line in "".join(parser.parts).splitlines() if line.strip())


def _identifier(value: Any) -> str:
    if not isinstance(value, str) or not _ID_PATTERN.fullmatch(value):
        raise ValueError("Invalid warning identifier")
    return value


def utc_timestamp(value: Any, *, optional: bool = False) -> str | None:
    if value in (None, "") and optional:
        return None
    if not isinstance(value, str):
        raise ValueError("Missing warning timestamp")
    parsed = datetime.fromisoformat(value.replace("Z", "+00:00"))
    if parsed.tzinfo is None:
        raise ValueError("Warning timestamp requires a timezone")
    return parsed.astimezone(timezone.utc).isoformat().replace("+00:00", "Z")


def _references(detail: dict) -> list[tuple[str, str]]:
    value = detail.get("references") or ""
    if not isinstance(value, str) or len(value) > 65536:
        raise ValueError("Invalid CAP references")
    result = []
    for ref in value.split():
        parts = ref.split(",")
        if len(parts) != 3:
            raise ValueError("Invalid CAP reference tuple")
        result.append((_identifier(parts[1]), utc_timestamp(parts[2])))
    return sorted(result, key=lambda item: (datetime.fromisoformat(item[1]), item[0]))


def select_info(detail: dict) -> dict:
    infos = detail.get("info") or []
    if not isinstance(infos, list) or any(not isinstance(i, dict) for i in infos):
        raise ValueError("Invalid CAP info blocks")
    def rank(info):
        language = str(info.get("language", "")).lower()
        return 0 if language in ("de", "de-de") else 1 if language.startswith("de-") else 2 if language.startswith("en") else 3
    return min(infos, key=rank) if infos else {}


def normalize_alert(detail: dict, summary: dict | None = None, geojson: dict | None = None, *, provider: str = "mowas") -> dict | None:
    """Normalize actual public CAP alerts; return None for exercises/private data.

    An incident's reference aliases must be unioned durably by the storage layer:
    update C may reference only B, while B originally referenced A.
    """
    if not isinstance(detail, dict):
        raise ValueError("Warning detail must be an object")
    if detail.get("status") != "Actual" or detail.get("scope") != "Public":
        return None
    if detail.get("msgType") not in ("Alert", "Update", "Cancel"):
        return None
    summary = summary or {}
    alert_id = _identifier(detail.get("identifier"))
    if summary.get("id") and alert_id != summary["id"]:
        raise ValueError("Warning detail identifier does not match its index entry")
    if summary.get("type") and summary["type"] != detail["msgType"]:
        raise ValueError("Warning detail type does not match its index entry yet")
    cancelled = detail["msgType"] == "Cancel"
    info = select_info(detail)
    if not info and not cancelled:
        raise ValueError("Active warning has no information block")
    sent = utc_timestamp(detail.get("sent"))
    starts = utc_timestamp(info.get("effective") or sent)
    expires = utc_timestamp(info.get("expires") or summary.get("expiresDate"), optional=True)
    if expires and datetime.fromisoformat(expires) < datetime.fromisoformat(starts):
        raise ValueError("Warning expires before it becomes effective")
    refs = _references(detail)
    aliases = list(dict.fromkeys([alert_id] + [ref[0] for ref in refs]))
    # Ordering comes from CAP timestamps, not lexicographic identifiers.
    incident_id = refs[0][0] if refs else alert_id
    severity = info.get("severity", summary.get("severity", "Unknown"))
    if severity not in SEVERITIES:
        raise ValueError("Unknown CAP severity")
    if cancelled:
        area = {"type": "GeometryCollection", "geometries": []}
    elif geojson is not None:
        area = validate_geometry(geojson)
    else:
        area = cap_geometry(info)
        if area is None:
            raise ValueError("Active warning has no geographic area")
    titles = summary.get("i18nTitle") or {}
    title = plain_text(info.get("headline") or titles.get("de") or info.get("event") or "Entwarnung")
    return {
        "id": alert_id, "provider_id": alert_id, "incident_id": incident_id,
        "aliases": aliases, "source": "nina", "provider": provider,
        "title": title, "description": plain_text(info.get("description")),
        "instruction": plain_text(info.get("instruction")), "severity": severity,
        "starts_at": starts, "expires_at": expires, "sent_at": sent,
        "cancelled": cancelled, "geometry": area,
        "url": "https://warnung.bund.de/meldung/" + quote(alert_id, safe=""),
    }


class NinaClient:
    """Poll indexes with bounded I/O and cache unchanged warning detail/geometry.

    fetch_json(url) can be injected for deterministic offline tests. A partial
    result may add/update warnings but MUST NOT remove unseen stored warnings.
    Cache entries are refreshed after 10 minutes even if an index is unchanged.
    """
    def __init__(self, base_url: str = DEFAULT_BASE_URL, timeout: float = 15,
                 sources=("mowas", "katwarn"), fetch_json: Callable[[str], Any] | None = None,
                 max_alerts: int = 2000, max_requests: int = 200, max_bytes: int = 8 * 1024 * 1024,
                 cache_seconds: float = 600):
        parts = urlsplit(base_url)
        if parts.scheme not in ("https", "http") or not parts.hostname or parts.username or parts.password or parts.query or parts.fragment:
            raise ValueError("NINA base URL must be an HTTP(S) origin/path")
        self.base_url = base_url.rstrip("/")
        self.sources = tuple(dict.fromkeys(sources))
        if not self.sources or any(s not in ALLOWED_SOURCES for s in self.sources):
            raise ValueError("Unsupported NINA source")
        if not 0 < timeout <= 120 or not 1 <= max_alerts <= 10000 or not len(self.sources) + 2 <= max_requests <= 1000 or not 1024 <= max_bytes <= 32 * 1024 * 1024 or not 0 <= cache_seconds <= 3600:
            raise ValueError("Invalid NINA resource limits")
        self.timeout, self.max_alerts, self.max_requests = timeout, max_alerts, max_requests
        self.max_bytes, self.cache_seconds = max_bytes, cache_seconds
        self.fetch_json = fetch_json or self._http_json
        self._cache: dict[str, tuple[str, float, dict | None]] = {}
        self._requests = 0

    def _http_json(self, url: str) -> Any:
        request = Request(url, headers={"User-Agent": "netcore-tetra-alert-service/1.0", "Accept": "application/json", "Accept-Encoding": "identity"})
        with urlopen(request, timeout=self.timeout) as response:
            raw = response.read(self.max_bytes + 1)
        if len(raw) > self.max_bytes:
            raise ProviderError("NINA response exceeds configured size limit")
        return json.loads(raw.decode("utf-8-sig"))

    def _load(self, path: str):
        if self._requests >= self.max_requests:
            raise ProviderError("NINA request budget reached; remaining warnings deferred to next poll")
        self._requests += 1
        try:
            return self.fetch_json(self.base_url + path)
        except Exception as exc:
            raise ProviderError(f"{path}: {type(exc).__name__}: {exc}") from exc

    def fetch(self) -> FetchResult:
        self._requests = 0
        result = FetchResult()
        entries = {}
        for source in self.sources:
            try:
                index = self._load(f"/{source}/mapData.json")
                if not isinstance(index, list) or len(index) > self.max_alerts:
                    raise ProviderError(f"Invalid or oversized {source} index")
                for entry in index:
                    if not isinstance(entry, dict):
                        raise ProviderError(f"Invalid {source} index entry")
                    alert_id = _identifier(entry.get("id"))
                    if alert_id not in entries and len(entries) >= self.max_alerts:
                        raise ProviderError("Combined NINA index exceeds configured warning limit")
                    entries[alert_id] = (source, entry)
            except (ProviderError, ValueError, TypeError) as exc:
                result.errors.append(f"{source}: {exc}")
        result.seen_ids = list(entries)
        now = time.monotonic()
        # New/changed warnings take priority over routine refreshes. Cached
        # successes make progress across polls when the request budget is hit.
        work = []
        for alert_id, (source, entry) in entries.items():
            signature = json.dumps(entry, sort_keys=True, ensure_ascii=True)
            cached = self._cache.get(alert_id)
            if cached and cached[0] == signature and now - cached[1] < self.cache_seconds:
                if cached[2] is not None:
                    result.alerts.append(copy.deepcopy(cached[2]))
            else:
                work.append((0 if cached is None or cached[0] != signature else 1, alert_id, source, entry, signature))
        work.sort(key=lambda item: item[0])
        for _, alert_id, source, entry, signature in work:
            try:
                detail = self._load(f"/warnings/{quote(alert_id, safe='')}.json")
                if not isinstance(detail, dict):
                    raise ValueError("Warning detail must be an object")
                geojson = None
                if detail.get("status") == "Actual" and detail.get("scope") == "Public" and detail.get("msgType") in ("Alert", "Update"):
                    if cap_geometry(select_info(detail)) is None:
                        geojson = self._load(f"/warnings/{quote(alert_id, safe='')}.geojson")
                alert = normalize_alert(detail, entry, geojson, provider=source)
                self._cache[alert_id] = (signature, now, copy.deepcopy(alert))
                if alert is not None:
                    result.alerts.append(alert)
            except (ProviderError, ValueError, TypeError, AttributeError) as exc:
                result.errors.append(f"{alert_id}: {exc}")
        result.complete = not result.errors
        if result.complete:
            self._cache = {key: value for key, value in self._cache.items() if key in entries}
        # Failed sources cannot let stale entries grow without a hard bound.
        while len(self._cache) > self.max_alerts:
            self._cache.pop(next(iter(self._cache)))
        return result
