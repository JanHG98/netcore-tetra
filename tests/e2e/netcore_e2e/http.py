# NETCORE-KOMMENTAR – Was: Enthält automatische Prüfungen für HTTP.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

from __future__ import annotations

import json
import time
import urllib.error
import urllib.parse
import urllib.request
from dataclasses import dataclass
from typing import Any, Iterable


# Was: Bündelt Daten und Verhalten für HTTP failure.
# Warum: Zusammengehöriger Zustand und seine Operationen bleiben dadurch an einer klaren Stelle.
class HttpFailure(RuntimeError):
    pass


# Was: Bündelt Daten und Verhalten für HTTP response.
# Warum: Zusammengehöriger Zustand und seine Operationen bleiben dadurch an einer klaren Stelle.
@dataclass
class HttpResponse:
    status: int
    headers: dict[str, str]
    body: bytes
    elapsed_ms: float

    # Was: Führt den Arbeitsschritt `text` für text aus.
    # Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    def text(self) -> str:
        return self.body.decode("utf-8", errors="replace")

    # Was: Führt den Arbeitsschritt `json` für JSON-Daten aus.
    # Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    def json(self) -> Any:
        if not self.body:
            return None
        # Was: Führt einen fehleranfälligen Abschnitt mit geregelter Fehlerbehandlung aus.
        # Warum: Ein einzelner Fehler soll kontrolliert gemeldet oder aufgefangen werden, statt den gesamten Dienst unverständlich abzubrechen.
        try:
            return json.loads(self.body)
        except json.JSONDecodeError as error:
            raise HttpFailure(f"response is not valid JSON: {error}; body={self.text()[:300]!r}") from error


# Was: Bündelt Daten und Verhalten für HTTP client.
# Warum: Zusammengehöriger Zustand und seine Operationen bleiben dadurch an einer klaren Stelle.
class HttpClient:
    # Was: Diese Funktion initialisiert den vorgesehenen Arbeitsschritt.
    # Warum: Alle benötigten Startwerte werden so in einer festen Reihenfolge eingerichtet.
    def __init__(self, timeout: float = 8.0, user_agent: str = "netcore-open-lab-e2e/1") -> None:
        self.timeout = timeout
        self.user_agent = user_agent

    # Was: Diese Funktion fordert den vorgesehenen Arbeitsschritt.
    # Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    def request(
        self,
        method: str,
        url: str,
        *,
        json_body: Any | None = None,
        data: bytes | None = None,
        headers: dict[str, str] | None = None,
        expected: Iterable[int] | None = None,
    ) -> HttpResponse:
        request_headers = {
            "Accept": "application/json, text/plain, text/html;q=0.9, */*;q=0.8",
            "User-Agent": self.user_agent,
        }
        if headers:
            request_headers.update(headers)
        if json_body is not None:
            data = json.dumps(json_body, separators=(",", ":")).encode("utf-8")
            request_headers["Content-Type"] = "application/json"
        request = urllib.request.Request(url, data=data, headers=request_headers, method=method.upper())
        started = time.monotonic()
        # Was: Führt einen fehleranfälligen Abschnitt mit geregelter Fehlerbehandlung aus.
        # Warum: Ein einzelner Fehler soll kontrolliert gemeldet oder aufgefangen werden, statt den gesamten Dienst unverständlich abzubrechen.
        try:
            with urllib.request.urlopen(request, timeout=self.timeout) as response:
                body = response.read()
                result = HttpResponse(
                    status=response.status,
                    headers={key.lower(): value for key, value in response.headers.items()},
                    body=body,
                    elapsed_ms=(time.monotonic() - started) * 1000.0,
                )
        except urllib.error.HTTPError as error:
            body = error.read()
            result = HttpResponse(
                status=error.code,
                headers={key.lower(): value for key, value in error.headers.items()},
                body=body,
                elapsed_ms=(time.monotonic() - started) * 1000.0,
            )
        except (urllib.error.URLError, TimeoutError, OSError) as error:
            raise HttpFailure(f"{method.upper()} {url} failed: {error}") from error
        if expected is not None and result.status not in set(expected):
            raise HttpFailure(
                f"{method.upper()} {url} returned HTTP {result.status}, expected {sorted(set(expected))}; "
                f"body={result.text()[:500]!r}"
            )
        return result

    # Was: Diese Funktion liest den vorgesehenen Arbeitsschritt.
    # Warum: Der Zugriff auf den Wert bleibt dadurch gekapselt und kann später zentral angepasst werden.
    def get(self, url: str, *, expected: Iterable[int] | None = (200,)) -> HttpResponse:
        return self.request("GET", url, expected=expected)

    # Was: Führt den Arbeitsschritt `post` für post aus.
    # Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    def post(self, url: str, body: Any | None = None, *, expected: Iterable[int] | None = (200, 201, 202, 204)) -> HttpResponse:
        return self.request("POST", url, json_body=body, expected=expected)

    # Was: Führt den Arbeitsschritt `put` für put aus.
    # Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    def put(self, url: str, body: Any, *, expected: Iterable[int] | None = (200, 204)) -> HttpResponse:
        return self.request("PUT", url, json_body=body, expected=expected)

    # Was: Diese Funktion löscht den vorgesehenen Arbeitsschritt.
    # Warum: Das Entfernen wird dadurch kontrolliert durchgeführt und hinterlässt keine verwaisten Verweise.
    def delete(self, url: str, *, expected: Iterable[int] | None = (200, 204, 404)) -> HttpResponse:
        return self.request("DELETE", url, expected=expected)


# Was: Führt den Arbeitsschritt `query_url` für query url aus.
# Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
def query_url(base: str, path: str, **query: Any) -> str:
    pairs = {key: value for key, value in query.items() if value is not None}
    suffix = urllib.parse.urlencode(pairs)
    return base + path + (("?" + suffix) if suffix else "")
