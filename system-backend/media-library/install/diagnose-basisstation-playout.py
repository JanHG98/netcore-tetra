#!/usr/bin/env python3
"""Verify Media Library -> TBS audio-player connectivity and configuration."""

from __future__ import annotations

import argparse
import http.cookiejar
import json
import pathlib
import sys
import urllib.error
import urllib.request

try:
    import tomllib
except ModuleNotFoundError:  # Python 3.10 fallback is intentionally explicit.
    print("Python 3.11 or newer is required (tomllib missing).", file=sys.stderr)
    raise SystemExit(2)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--config", default="/etc/netcore/media-library.toml")
    parser.add_argument("--station-id")
    args = parser.parse_args()

    config_path = pathlib.Path(args.config)
    with config_path.open("rb") as handle:
        config = tomllib.load(handle)

    playout = config.get("playout", {})
    stations = playout.get("stations", [])
    station_id = args.station_id or playout.get("default_station")
    station = next(
        (
            item
            for item in stations
            if item.get("id") == station_id and item.get("enabled", True)
        ),
        None,
    )
    if station is None:
        print(
            f"FEHLER: keine aktive Basisstation mit ID {station_id!r} in [playout] gefunden.",
            file=sys.stderr,
        )
        return 1

    base_url = str(station.get("base_url", "")).rstrip("/")
    if not base_url:
        print("FEHLER: base_url der Basisstation fehlt.", file=sys.stderr)
        return 1

    cookie_jar = http.cookiejar.CookieJar()
    opener = urllib.request.build_opener(urllib.request.HTTPCookieProcessor(cookie_jar))
    username = station.get("username")
    password = station.get("password")
    if bool(username) != bool(password):
        print("FEHLER: username und password müssen gemeinsam gesetzt sein.", file=sys.stderr)
        return 1
    if username and password:
        payload = json.dumps({"user": username, "password": password}).encode("utf-8")
        request = urllib.request.Request(
            f"{base_url}/api/login",
            data=payload,
            headers={"Content-Type": "application/json"},
            method="POST",
        )
        response_json(opener, request, "TBS-Login")
        if not any(cookie.name == "fs_session" for cookie in cookie_jar):
            print("FEHLER: TBS-Login lieferte kein fs_session-Cookie.", file=sys.stderr)
            return 1
        print("OK: Cookie-Login an der Basisstation erfolgreich.")

    status = response_json(
        opener,
        urllib.request.Request(f"{base_url}/api/audio/status"),
        "Audio-Player-Status",
    )
    if not status.get("available"):
        print(
            f"FEHLER: Audio Player nicht verfügbar: {status.get('last_error') or status}",
            file=sys.stderr,
        )
        return 1
    print(
        "OK: Audio Player verfügbar "
        f"(state={status.get('state')}, ffmpeg={status.get('ffmpeg_available')})."
    )

    sources = response_json(
        opener,
        urllib.request.Request(f"{base_url}/api/audio/sources"),
        "Audioquellen",
    ).get("sources", [])
    media_library = next((item for item in sources if item.get("id") == "media-library"), None)
    if media_library is None:
        print(
            "FEHLER: Die TBS meldet keine Audioquelle mit ID 'media-library'. "
            "[media_library].enabled und audio_source_enabled prüfen.",
            file=sys.stderr,
        )
        return 1
    if not media_library.get("available"):
        print(
            f"FEHLER: Media-Library-Audioquelle der TBS ist nicht verfügbar: "
            f"{media_library.get('error') or media_library}",
            file=sys.stderr,
        )
        return 1
    print(
        "OK: TBS-Audioquelle 'media-library' ist verfügbar "
        f"({media_library.get('path')})."
    )
    print(f"FERTIG: {station.get('name', station_id)} ist für delegiertes Playout bereit.")
    return 0


def response_json(
    opener: urllib.request.OpenerDirector,
    request: urllib.request.Request,
    label: str,
) -> dict:
    try:
        with opener.open(request, timeout=15) as response:
            body = response.read()
            final_url = response.geturl()
            content_type = response.headers.get_content_type()
    except urllib.error.HTTPError as error:
        detail = error.read().decode("utf-8", errors="replace")[:500]
        raise SystemExit(f"FEHLER {label}: HTTP {error.code}: {detail}") from error
    except urllib.error.URLError as error:
        raise SystemExit(f"FEHLER {label}: {error.reason}") from error

    if final_url.rstrip("/").endswith("/login") and request.full_url != final_url:
        raise SystemExit(
            f"FEHLER {label}: TBS verlangt eine Anmeldung. "
            "username/password im [[playout.stations]]-Eintrag ergänzen."
        )
    if content_type != "application/json":
        excerpt = body.decode("utf-8", errors="replace")[:300]
        raise SystemExit(
            f"FEHLER {label}: keine JSON-Antwort ({content_type}) von {final_url}: {excerpt}"
        )
    try:
        value = json.loads(body)
    except json.JSONDecodeError as error:
        raise SystemExit(f"FEHLER {label}: ungültiges JSON: {error}") from error
    if not isinstance(value, dict):
        raise SystemExit(f"FEHLER {label}: JSON-Objekt erwartet, erhalten: {type(value).__name__}")
    return value


if __name__ == "__main__":
    raise SystemExit(main())
