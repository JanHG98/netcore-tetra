#!/usr/bin/env python3
# NETCORE-KOMMENTAR – Was: Enthält die Logik oder Einstellungen für check full system integration.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

"""Cross-check the complete NetCore-Tetra open-lab deployment and edge fallback.

This is deliberately stricter than the component checkers: it verifies the
inventory, ports, dependency graph, rendered inter-service URLs, common API
contracts, Node-Gateway health distribution and TBS local-autonomy hooks as one
system.
"""
from __future__ import annotations

import json
import re
import subprocess
import sys
import tomllib
from collections import defaultdict, deque
from pathlib import Path
from urllib.parse import urlparse

ROOT = Path(__file__).resolve().parents[1]
INVENTORY = ROOT / "deploy/open-lab/inventory.example.toml"
NODE_GATEWAY = ROOT / "system-backend/node-gateway/config/node-gateway.example.toml"
REPORT = ROOT / "Docs/generated/full-system-integration-audit.md"
EXPECTED = {
    "node-gateway", "mobility-core", "subscriber-core", "group-core", "call-control",
    "media-switch", "recorder", "sds-router", "packet-core", "ip-gateway",
    "security-core", "kmf", "transit", "application-gateway", "media-library",
    "control-room", "observability", "iot-gateway",
}
REQUIRED_EDGE = {
    "subscriber-core", "group-core", "mobility-core", "call-control", "media-switch", "sds-router"
}
API_MARKERS = ("/health/live", "/health/ready", "/metrics", "/openapi.json")

# Was: Bündelt Daten und Verhalten für audit.
# Warum: Zusammengehöriger Zustand und seine Operationen bleiben dadurch an einer klaren Stelle.
class Audit:
    # Was: Diese Funktion initialisiert den vorgesehenen Arbeitsschritt.
    # Warum: Alle benötigten Startwerte werden so in einer festen Reihenfolge eingerichtet.
    def __init__(self) -> None:
        self.errors: list[str] = []
        self.notes: list[str] = []
        self.rows: list[tuple[str, str, int, str, str]] = []

    # Was: Führt den Arbeitsschritt `require` für require aus.
    # Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    def require(self, cond: bool, message: str) -> None:
        if not cond:
            self.errors.append(message)

    # Was: Führt den Arbeitsschritt `note` für note aus.
    # Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    def note(self, message: str) -> None:
        self.notes.append(message)


# Was: Diese Funktion lädt toml.
# Warum: Einlesen und Fehlerbehandlung bleiben dadurch an einer zentralen Stelle.
def load_toml(path: Path) -> dict:
    # Was: Führt einen fehleranfälligen Abschnitt mit geregelter Fehlerbehandlung aus.
    # Warum: Ein einzelner Fehler soll kontrolliert gemeldet oder aufgefangen werden, statt den gesamten Dienst unverständlich abzubrechen.
    try:
        return tomllib.loads(path.read_text(encoding="utf-8"))
    except Exception as exc:
        raise RuntimeError(f"invalid TOML {path.relative_to(ROOT)}: {exc}") from exc


# Was: Führt den Arbeitsschritt `service_source` für Dienst source aus.
# Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
def service_source(name: str) -> str:
    bases = [ROOT / "system-backend" / name]
    if name == "control-room":
        bases.append(ROOT / "bins/netcore-control-room")
    chunks = []
    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    for base in bases:
      # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
      # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
      for path in sorted(base.rglob("*")):
          if path.is_file() and path.suffix in {".rs", ".py", ".html", ".js"}:
              chunks.append(path.read_text(encoding="utf-8", errors="replace"))
    return "\n".join(chunks)


# Was: Diese Funktion prüft graph.
# Warum: Fehler oder unzulässige Zustände werden dadurch früh erkannt.
def check_graph(audit: Audit, services: dict[str, dict]) -> None:
    indegree = {name: 0 for name in services}
    outgoing: dict[str, list[str]] = defaultdict(list)
    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    for name, svc in services.items():
        # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
        # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
        for dep in svc.get("depends_on", []):
            audit.require(dep in services, f"{name}: unknown dependency {dep}")
            if dep in services:
                outgoing[dep].append(name)
                indegree[name] += 1
    queue = deque(name for name, degree in indegree.items() if degree == 0)
    seen = []
    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    while queue:
        name = queue.popleft(); seen.append(name)
        # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
        # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
        for nxt in outgoing[name]:
            indegree[nxt] -= 1
            if indegree[nxt] == 0:
                queue.append(nxt)
    audit.require(len(seen) == len(services), "deployment dependency graph contains a cycle")
    audit.note("Start order: " + " → ".join(seen))


# Was: Diese Funktion prüft rendered urls.
# Warum: Fehler oder unzulässige Zustände werden dadurch früh erkannt.
def check_rendered_urls(audit: Audit, services: dict[str, dict]) -> None:
    subprocess.run(
        [sys.executable, str(ROOT / "deploy/open-lab/netcore-deploy.py"), "--inventory", str(INVENTORY), "render"],
        cwd=ROOT, check=True, stdout=subprocess.DEVNULL,
    )
    endpoint_owner = {(svc["host"], int(svc["port"])): name for name, svc in services.items()}
    generated = ROOT / "deploy/open-lab/generated/configs"
    checked = 0
    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    for path in sorted(generated.rglob("*.toml")):
        load_toml(path)
        text = path.read_text(encoding="utf-8")
        # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
        # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
        for raw in re.findall(r'(?:https?|wss?)://[^\s"\']+', text):
            parsed = urlparse(raw.rstrip(",)]}"))
            if not parsed.hostname or parsed.port is None:
                continue
            if parsed.hostname.startswith("10.0.20."):
                checked += 1
                audit.require(
                    (parsed.hostname, parsed.port) in endpoint_owner,
                    f"{path.relative_to(ROOT)} references unknown open-lab endpoint {parsed.hostname}:{parsed.port}",
                )
    audit.note(f"Rendered inter-service URLs checked: {checked}")


# Was: Startet das Programm, lädt die benötigten Einstellungen und übergibt an den eigentlichen Dienstablauf.
# Warum: Ein klarer Einstiegspunkt hält Startreihenfolge, Fehlerausgabe und geordnetes Beenden zusammen.
def main() -> int:
    audit = Audit()
    inventory = load_toml(INVENTORY)
    service_list = inventory.get("services", [])
    services = {svc["name"]: svc for svc in service_list}
    audit.require(set(services) == EXPECTED, f"inventory services differ: got={sorted(services)}")
    audit.require(len(service_list) == len(services), "duplicate service names in inventory")

    endpoints = [(svc["host"], int(svc["port"])) for svc in service_list]
    audit.require(len(endpoints) == len(set(endpoints)), "duplicate host/port endpoint in inventory")
    ports = [int(svc["port"]) for svc in service_list]
    audit.require(len(ports) == len(set(ports)), "duplicate management port in inventory")
    check_graph(audit, services)

    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    for name, svc in services.items():
        template = ROOT / svc["config_template"]
        install = ROOT / svc["install"]
        audit.require(template.is_file(), f"{name}: config template missing: {template}")
        audit.require(install.is_file(), f"{name}: installer missing: {install}")
        if template.is_file():
            load_toml(template)
        source = service_source(name)
        missing = [marker for marker in API_MARKERS if marker not in source]
        audit.require(not missing, f"{name}: common API markers missing: {missing}")
        has_webui = any(marker in source for marker in ("<!doctype html", "<!DOCTYPE html", "INDEX_HTML"))
        audit.require(has_webui, f"{name}: no integrated WebUI marker found")
        audit.rows.append((name, svc["host"], int(svc["port"]), ", ".join(svc.get("depends_on", [])) or "—", "yes" if not missing and has_webui else "no"))

    # Node Gateway monitors every other runtime LXC and distributes explicit fallback modes.
    ng = load_toml(NODE_GATEWAY)
    targets = {target["name"]: target for target in ng["service_monitor"]["targets"]}
    audit.require(set(targets) == EXPECTED - {"node-gateway"}, "Node Gateway health target set is not inventory minus node-gateway")
    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    for name, target in targets.items():
        svc = services[name]
        parsed = urlparse(target["url"])
        audit.require(parsed.hostname == svc["host"] and parsed.port == int(svc["port"]), f"{name}: health target does not match inventory")
        audit.require(parsed.path == "/health/ready", f"{name}: health target must use /health/ready")
        audit.require(bool(target.get("fallback_mode")), f"{name}: missing fallback_mode")
        audit.require(bool(target.get("critical_for_edge")) == (name in REQUIRED_EDGE), f"{name}: critical_for_edge differs from TBS required_services")

    bs = load_toml(ROOT / "config.toml")
    fb = bs.get("edge_fallback", {})
    modes = fb.get("service_fallbacks", {})
    audit.require(set(modes) == EXPECTED, "TBS edge_fallback.service_fallbacks does not cover all 18 runtime services")
    audit.require(set(fb.get("required_services", [])) == REQUIRED_EDGE, "TBS required_services mismatch")
    audit.require(fb.get("unknown_service_is_available") is False, "unknown service health must fail closed into fallback")
    audit.require(5 <= int(fb.get("service_matrix_lease_secs", 0)) <= 3600, "TBS service health matrix needs a bounded freshness lease")
    audit.require(fb.get("keep_last_known_policy") is True, "last-known subscriber/group policy must be retained")
    control = bs.get("control_room", {})
    node_gateway = services["node-gateway"]
    audit.require(
        control.get("host") == node_gateway["host"]
        and int(control.get("port", 0)) == int(node_gateway["port"])
        and control.get("endpoint_path") == "/ws/node",
        "TBS sample control_room endpoint must terminate on the Node Gateway so service-health fallback is available",
    )

    hooks = {
        "health protocol": (ROOT / "crates/tetra-entities/src/net_control_room/protocol.rs", "CoreServicesSnapshot"),
        "fallback state machine": (ROOT / "crates/tetra-entities/src/net_control_room/worker.rs", "tick_edge_fallback"),
        "durable replay spool": (ROOT / "crates/tetra-entities/src/net_control_room/edge_store.rs", "EdgeEventSpool"),
        "policy cache restore": (ROOT / "bins/bluestation-bs/src/main.rs", "load_edge_policy_cache"),
        "policy cache persist": (ROOT / "crates/tetra-entities/src/mm/mm_bs.rs", "persist_edge_policy_cache"),
        "dynamic SYSINFO": (ROOT / "crates/tetra-entities/src/umac/umac_bs.rs", "system_wide_services_available"),
        "SDS local fallback": (ROOT / "crates/tetra-entities/src/cmce/subentities/sds_bs.rs", "air_fallback_local_delivered"),
        "SDS remote replay": (ROOT / "crates/tetra-entities/src/cmce/subentities/sds_bs.rs", "air_fallback_queued"),
        "SDS duplicate suppression": (ROOT / "system-backend/sds-router/src/state.rs", "local_delivered"),
        "service probe worker": (ROOT / "system-backend/node-gateway/src/service_monitor.rs", "spawn_service_monitor"),
        "service health lease refresh": (ROOT / "system-backend/node-gateway/src/service_monitor.rs", "publish_core_services"),
        "TBS health lease expiry": (ROOT / "crates/tetra-entities/src/net_control_room/worker.rs", "service_matrix_lease_secs"),
        "fail-closed matrix freshness": (ROOT / "crates/tetra-config/src/bluestation/config.rs", "edge_service_matrix_fresh"),
        "stale revision rejection": (ROOT / "crates/tetra-entities/src/net_control_room/worker.rs", "ignoring stale core-service matrix"),
        "partial-service degraded mode": (ROOT / "crates/tetra-entities/src/net_control_room/worker.rs", "service-specific fallbacks active"),
        "all-service live outage scenario": (ROOT / "tests/e2e/netcore_e2e/scenarios.py", "edge-service-outages"),
        "offline fallback reference tests": (ROOT / "tests/e2e/unit/test_edge_fallback_reference.py", "test_stale_matrix_fails_every_central_service_closed"),
        "TBS fallback API": (ROOT / "crates/tetra-entities/src/net_dashboard/server.rs", "/api/edge-fallback"),
    }
    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    for label, (path, needle) in hooks.items():
        audit.require(path.is_file() and needle in path.read_text(encoding="utf-8"), f"missing integration hook: {label}")

    check_rendered_urls(audit, services)

    REPORT.parent.mkdir(parents=True, exist_ok=True)
    report = [
        "# NetCore-Tetra Full-System Integration Audit",
        "",
        "Generated by `tools/check_full_system_integration.py`.",
        "",
        f"- Runtime services: **{len(services)}**",
        f"- Unique management endpoints: **{len(set(endpoints))}**",
        f"- Node-Gateway backend health targets: **{len(targets)}**",
        f"- Explicit TBS fallback modes: **{len(modes)}**",
        f"- Result: **{'PASS' if not audit.errors else 'FAIL'}**",
        "",
        "## Runtime matrix",
        "",
        "| Service | Host | Port | Dependencies | API/WebUI contract |",
        "|---|---:|---:|---|---|",
    ]
    report += [f"| {n} | {h} | {p} | {d} | {ok} |" for n,h,p,d,ok in sorted(audit.rows)]
    report += ["", "## Edge fallback rules", ""]
    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    for name in sorted(modes):
        report.append(f"- `{name}` → `{modes[name]}`")
    report += ["", "## Audit notes", ""] + [f"- {note}" for note in audit.notes]
    if audit.errors:
        report += ["", "## Errors", ""] + [f"- {error}" for error in audit.errors]
    REPORT.write_text("\n".join(report) + "\n", encoding="utf-8")

    if audit.errors:
        print("Full-system integration audit: FAIL", file=sys.stderr)
        # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
        # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
        for error in audit.errors:
            print(" -", error, file=sys.stderr)
        return 1
    print("Full-system integration audit: OK")
    print(f"  runtime services: {len(services)}")
    print(f"  unique endpoints: {len(set(endpoints))}")
    print(f"  health targets: {len(targets)}")
    print(f"  explicit fallback modes: {len(modes)}")
    print(f"  report: {REPORT.relative_to(ROOT)}")
    return 0

# Was: Startet den Programmablauf nur dann, wenn diese Datei direkt ausgeführt wird.
# Warum: Beim Import als Modul sollen nur Funktionen bereitstehen und keine Nebenwirkungen automatisch starten.
if __name__ == "__main__":
    raise SystemExit(main())
