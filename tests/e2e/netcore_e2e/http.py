from __future__ import annotations

import json
import time
import urllib.error
import urllib.parse
import urllib.request
from dataclasses import dataclass
from typing import Any, Iterable


class HttpFailure(RuntimeError):
    pass


@dataclass
class HttpResponse:
    status: int
    headers: dict[str, str]
    body: bytes
    elapsed_ms: float

    def text(self) -> str:
        return self.body.decode("utf-8", errors="replace")

    def json(self) -> Any:
        if not self.body:
            return None
        try:
            return json.loads(self.body)
        except json.JSONDecodeError as error:
            raise HttpFailure(f"response is not valid JSON: {error}; body={self.text()[:300]!r}") from error


class HttpClient:
    def __init__(self, timeout: float = 8.0, user_agent: str = "netcore-open-lab-e2e/1") -> None:
        self.timeout = timeout
        self.user_agent = user_agent

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

    def get(self, url: str, *, expected: Iterable[int] | None = (200,)) -> HttpResponse:
        return self.request("GET", url, expected=expected)

    def post(self, url: str, body: Any | None = None, *, expected: Iterable[int] | None = (200, 201, 202, 204)) -> HttpResponse:
        return self.request("POST", url, json_body=body, expected=expected)

    def put(self, url: str, body: Any, *, expected: Iterable[int] | None = (200, 204)) -> HttpResponse:
        return self.request("PUT", url, json_body=body, expected=expected)

    def delete(self, url: str, *, expected: Iterable[int] | None = (200, 204, 404)) -> HttpResponse:
        return self.request("DELETE", url, expected=expected)


def query_url(base: str, path: str, **query: Any) -> str:
    pairs = {key: value for key, value in query.items() if value is not None}
    suffix = urllib.parse.urlencode(pairs)
    return base + path + (("?" + suffix) if suffix else "")
