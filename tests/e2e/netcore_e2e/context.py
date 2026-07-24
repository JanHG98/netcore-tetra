from __future__ import annotations

import contextlib
import hashlib
import subprocess
import time
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Callable

from .http import HttpClient
from .model import CheckResult, Inventory, RunReport, Service


def now_iso() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="milliseconds").replace("+00:00", "Z")


@dataclass
class E2EContext:
    inventory: Inventory
    report: RunReport
    artifacts_dir: Path
    timeout: float = 20.0
    strict_ready: bool = False
    allow_mutations: bool = False
    allow_restarts: bool = False
    keep_fixtures: bool = False
    client: HttpClient = field(default_factory=HttpClient)
    mock_tbs: Any | None = None
    cleanup_actions: list[Callable[[], None]] = field(default_factory=list)

    @property
    def services(self) -> dict[str, Service]:
        return self.inventory.by_name

    def service(self, name: str) -> Service:
        try:
            return self.services[name]
        except KeyError as error:
            raise RuntimeError(f"required service missing from inventory: {name}") from error

    def base(self, name: str) -> str:
        return self.service(name).base_url

    def fixture_numbers(self) -> tuple[int, int, int]:
        digest = hashlib.sha256(self.report.run_id.encode("utf-8")).digest()
        offset = int.from_bytes(digest[:3], "big") % 500_000
        issi_a = 7_000_000 + offset
        issi_b = issi_a + 1
        gssi = 2_000_000 + offset
        return issi_a, issi_b, gssi

    def check(
        self,
        name: str,
        callback: Callable[[], dict[str, Any] | None],
        *,
        scenario: str,
        service: str | None = None,
        skip: str | None = None,
    ) -> bool:
        started = time.monotonic()
        if skip is not None:
            self.report.add(
                CheckResult(
                    name=name,
                    status="skipped",
                    duration_ms=(time.monotonic() - started) * 1000.0,
                    detail=skip,
                    service=service,
                    scenario=scenario,
                )
            )
            print(f"SKIP  [{scenario}] {name}: {skip}")
            return True
        try:
            evidence = callback() or {}
            self.report.add(
                CheckResult(
                    name=name,
                    status="passed",
                    duration_ms=(time.monotonic() - started) * 1000.0,
                    service=service,
                    scenario=scenario,
                    evidence=evidence,
                )
            )
            print(f"PASS  [{scenario}] {name}")
            return True
        except BaseException as error:
            self.report.add(
                CheckResult(
                    name=name,
                    status="failed",
                    duration_ms=(time.monotonic() - started) * 1000.0,
                    detail=f"{type(error).__name__}: {error}",
                    service=service,
                    scenario=scenario,
                )
            )
            print(f"FAIL  [{scenario}] {name}: {error}")
            return False

    def add_cleanup(self, action: Callable[[], None]) -> None:
        self.cleanup_actions.append(action)

    def cleanup(self) -> None:
        if self.keep_fixtures:
            return
        for action in reversed(self.cleanup_actions):
            with contextlib.suppress(BaseException):
                action()

    def ssh(self, service_name: str, remote_command: str, *, check: bool = True) -> subprocess.CompletedProcess[str]:
        service = self.service(service_name)
        command = [
            "ssh",
            *self.inventory.ssh_options,
            f"{self.inventory.ssh_user}@{service.host}",
            remote_command,
        ]
        return subprocess.run(command, text=True, capture_output=True, check=check)
