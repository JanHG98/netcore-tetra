"""Read the canonical runtime registry shared by deployment consistency checks."""
from pathlib import Path
import tomllib

ROOT = Path(__file__).resolve().parents[1]


def runtime_registry(root: Path = ROOT) -> dict[str, dict]:
    raw = tomllib.loads((root / "system-backend/services.toml").read_text(encoding="utf-8"))
    runtime = [item for item in raw["services"] if item.get("runtime", True)]
    names = [item["name"] for item in runtime]
    if len(names) != len(set(names)):
        raise ValueError("duplicate runtime service names in services.toml")
    return {item["name"]: item for item in runtime}


def registry_inventory_errors(services: dict[str, dict], root: Path = ROOT) -> list[str]:
    registry = runtime_registry(root)
    errors = []
    if set(services) != set(registry):
        errors.append(f"runtime registry/inventory differ: missing={sorted(set(registry) - set(services))} "
                      f"extra={sorted(set(services) - set(registry))}")
    for name in services.keys() & registry.keys():
        if int(services[name]["port"]) != int(registry[name]["management_port"]):
            errors.append(f"{name}: inventory port differs from runtime registry")
        if services[name].get("security_mode", "open_lab") != registry[name].get("security_mode", "open_lab"):
            errors.append(f"{name}: inventory security_mode differs from runtime registry")
    return errors
