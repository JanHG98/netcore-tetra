from __future__ import annotations

import tomllib
from pathlib import Path

from .model import Inventory, Service


def load_inventory(path: Path) -> Inventory:
    with path.open("rb") as handle:
        raw = tomllib.load(handle)
    services = tuple(
        Service(
            name=str(item["name"]),
            host=str(item["host"]),
            port=int(item["port"]),
            unit=str(item["unit"]),
            user=str(item.get("user", "netcore")),
            depends_on=tuple(str(value) for value in item.get("depends_on", [])),
        )
        for item in raw.get("services", [])
    )
    return Inventory(
        path=path,
        version=int(raw.get("version", 0)),
        contract_version=str(raw.get("contract_version", "")),
        mode=str(raw.get("mode", "")),
        ssh_user=str(raw.get("ssh_user", "root")),
        ssh_options=tuple(str(value) for value in raw.get("ssh_options", [])),
        health_timeout_secs=int(raw.get("health_timeout_secs", 8)),
        services=services,
    )


def validate_inventory(inventory: Inventory) -> list[str]:
    errors: list[str] = []
    if inventory.version != 1:
        errors.append("inventory version must be 1")
    if inventory.contract_version != "netcore.v1":
        errors.append("contract_version must be netcore.v1")
    if inventory.mode != "open_lab":
        errors.append("the current E2E package only supports explicit open_lab mode")
    names = {service.name for service in inventory.services}
    if len(names) != len(inventory.services):
        errors.append("duplicate service names")
    sockets = {(service.host, service.port) for service in inventory.services}
    if len(sockets) != len(inventory.services):
        errors.append("duplicate management sockets")
    for service in inventory.services:
        for dependency in service.depends_on:
            if dependency not in names:
                errors.append(f"{service.name}: unknown dependency {dependency}")
    return errors
