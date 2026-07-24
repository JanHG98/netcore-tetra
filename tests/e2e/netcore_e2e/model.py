from __future__ import annotations

from dataclasses import dataclass, field
from pathlib import Path
from typing import Any


@dataclass(frozen=True)
class Service:
    name: str
    host: str
    port: int
    unit: str
    user: str
    depends_on: tuple[str, ...] = ()

    @property
    def base_url(self) -> str:
        return f"http://{self.host}:{self.port}"


@dataclass(frozen=True)
class Inventory:
    path: Path
    version: int
    contract_version: str
    mode: str
    ssh_user: str
    ssh_options: tuple[str, ...]
    health_timeout_secs: int
    services: tuple[Service, ...]

    @property
    def by_name(self) -> dict[str, Service]:
        return {service.name: service for service in self.services}


@dataclass
class CheckResult:
    name: str
    status: str
    duration_ms: float
    detail: str = ""
    service: str | None = None
    scenario: str | None = None
    evidence: dict[str, Any] = field(default_factory=dict)

    @property
    def failed(self) -> bool:
        return self.status == "failed"

    @property
    def skipped(self) -> bool:
        return self.status == "skipped"


@dataclass
class RunReport:
    run_id: str
    started_at: str
    finished_at: str | None = None
    mode: str = "open_lab"
    inventory: str = ""
    results: list[CheckResult] = field(default_factory=list)
    metadata: dict[str, Any] = field(default_factory=dict)

    def add(self, result: CheckResult) -> None:
        self.results.append(result)

    @property
    def failures(self) -> int:
        return sum(result.failed for result in self.results)

    @property
    def skipped(self) -> int:
        return sum(result.skipped for result in self.results)

    @property
    def passed(self) -> int:
        return sum(result.status == "passed" for result in self.results)
