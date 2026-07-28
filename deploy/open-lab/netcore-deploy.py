#!/usr/bin/env python3
# NETCORE-KOMMENTAR – Was: Beschreibt Installation oder Betrieb von netcore deploy.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

"""Inventory-driven Open-Lab deployment helper for NetCore-Tetra LXCs.

The helper is deliberately conservative:
- validation and rendering are offline;
- apply is an explicit subcommand;
- no password, token or secret is written into generated files;
- SSH host verification is delegated to the configured SSH options;
- bundles exclude PDFs, VCS data and build output.
"""

from __future__ import annotations

import argparse
import csv
import gzip
import hashlib
import io
import json
import os
import re
import shlex
import subprocess
import sys
import tarfile
import tempfile
import time
import tomllib
import urllib.error
import urllib.request
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable

ROOT = Path(__file__).resolve().parents[2]
DEFAULT_INVENTORY = Path(__file__).with_name("inventory.example.toml")
GENERATED = Path(__file__).with_name("generated")
URL_RE = re.compile(r"(?P<scheme>https?|wss?)://(?P<host>\[[^]]+\]|[^/:\s\"']+):(?P<port>[0-9]{2,5})")
SERVICE_NAME_RE = re.compile(r"^[a-z0-9][a-z0-9-]*$")


# Was: Bündelt Daten und Verhalten für deploy error.
# Warum: Zusammengehöriger Zustand und seine Operationen bleiben dadurch an einer klaren Stelle.
class DeployError(RuntimeError):
    pass


# Was: Bündelt Daten und Verhalten für Dienst.
# Warum: Zusammengehöriger Zustand und seine Operationen bleiben dadurch an einer klaren Stelle.
@dataclass(frozen=True)
class Service:
    name: str
    host: str
    port: int
    unit: str
    user: str
    install: Path
    config_template: Path
    config_target: str
    depends_on: tuple[str, ...]

    # Was: Führt den Arbeitsschritt `base_url` für base url aus.
    # Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    @property
    def base_url(self) -> str:
        return f"http://{self.host}:{self.port}"


# Was: Bündelt Daten und Verhalten für inventory.
# Warum: Zusammengehöriger Zustand und seine Operationen bleiben dadurch an einer klaren Stelle.
@dataclass(frozen=True)
class Inventory:
    path: Path
    version: int
    contract_version: str
    mode: str
    ssh_user: str
    ssh_options: tuple[str, ...]
    remote_source_root: str
    health_timeout_secs: int
    services: tuple[Service, ...]

    # Was: Führt den Arbeitsschritt `by_name` für by name aus.
    # Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    @property
    def by_name(self) -> dict[str, Service]:
        return {service.name: service for service in self.services}

    # Was: Führt den Arbeitsschritt `by_port` für by port aus.
    # Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    @property
    def by_port(self) -> dict[int, Service]:
        return {service.port: service for service in self.services}


# Was: Diese Funktion lädt inventory.
# Warum: Einlesen und Fehlerbehandlung bleiben dadurch an einer zentralen Stelle.
def load_inventory(path: Path) -> Inventory:
    with path.open("rb") as handle:
        raw = tomllib.load(handle)
    services = tuple(
        Service(
            name=item["name"],
            host=item["host"],
            port=int(item["port"]),
            unit=item["unit"],
            user=item["user"],
            install=Path(item["install"]),
            config_template=Path(item["config_template"]),
            config_target=item["config_target"],
            depends_on=tuple(item.get("depends_on", [])),
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
        remote_source_root=str(raw.get("remote_source_root", "/opt/netcore-tetra-src")),
        health_timeout_secs=int(raw.get("health_timeout_secs", 8)),
        services=services,
    )


# Was: Diese Funktion prüft den vorgesehenen Arbeitsschritt.
# Warum: Unzulässige Werte werden dadurch erkannt, bevor sie im Betrieb Schaden anrichten.
def validate(inventory: Inventory) -> list[str]:
    errors: list[str] = []
    if inventory.version != 1:
        errors.append("inventory version must be 1")
    if inventory.contract_version != "netcore.v1":
        errors.append("contract_version must be netcore.v1 for this deployment package")
    if inventory.mode != "open_lab":
        errors.append("this deployer only supports the explicitly isolated open_lab mode")
    if not inventory.services:
        errors.append("inventory contains no services")

    names: set[str] = set()
    sockets: set[tuple[str, int]] = set()
    by_name = inventory.by_name
    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    for service in inventory.services:
        if not SERVICE_NAME_RE.fullmatch(service.name):
            errors.append(f"invalid service name: {service.name!r}")
        if service.name in names:
            errors.append(f"duplicate service name: {service.name}")
        names.add(service.name)
        socket = (service.host, service.port)
        if socket in sockets:
            errors.append(f"duplicate management socket: {service.host}:{service.port}")
        sockets.add(socket)
        if not 1 <= service.port <= 65535:
            errors.append(f"invalid port for {service.name}: {service.port}")
        # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
        # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
        for relative, label in ((service.install, "install"), (service.config_template, "config template")):
            path = ROOT / relative
            if not path.is_file():
                errors.append(f"{service.name}: {label} does not exist: {relative}")
        unit_path = ROOT / "system-backend" / service.name / "systemd" / service.unit
        if not unit_path.is_file():
            errors.append(f"{service.name}: systemd unit does not exist: {unit_path.relative_to(ROOT)}")
        else:
            unit_text = unit_path.read_text(encoding="utf-8", errors="replace")
            configured_user = next((line.split("=", 1)[1].strip() for line in unit_text.splitlines() if line.startswith("User=")), None)
            if configured_user and configured_user != service.user:
                errors.append(f"{service.name}: inventory user {service.user} differs from unit User={configured_user}")
        # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
        # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
        for dependency in service.depends_on:
            if dependency not in by_name:
                errors.append(f"{service.name}: unknown dependency {dependency}")
            if dependency == service.name:
                errors.append(f"{service.name}: self dependency")

    # Was: Führt einen fehleranfälligen Abschnitt mit geregelter Fehlerbehandlung aus.
    # Warum: Ein einzelner Fehler soll kontrolliert gemeldet oder aufgefangen werden, statt den gesamten Dienst unverständlich abzubrechen.
    try:
        topological_order(inventory)
    except DeployError as error:
        errors.append(str(error))
    return errors


# Was: Führt den Arbeitsschritt `topological_order` für topological order aus.
# Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
def topological_order(inventory: Inventory, selected: Iterable[str] | None = None) -> list[Service]:
    by_name = inventory.by_name
    requested = set(selected or by_name)
    unknown = requested - by_name.keys()
    if unknown:
        raise DeployError(f"unknown selected services: {', '.join(sorted(unknown))}")

    closure = set(requested)
    pending = list(requested)
    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    while pending:
        current = by_name[pending.pop()]
        # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
        # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
        for dependency in current.depends_on:
            if dependency not in closure:
                closure.add(dependency)
                pending.append(dependency)

    indegree = {name: 0 for name in closure}
    outgoing = {name: [] for name in closure}
    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    for name in closure:
        # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
        # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
        for dependency in by_name[name].depends_on:
            if dependency in closure:
                indegree[name] += 1
                outgoing[dependency].append(name)
    queue = sorted(name for name, degree in indegree.items() if degree == 0)
    result: list[Service] = []
    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    while queue:
        name = queue.pop(0)
        result.append(by_name[name])
        # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
        # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
        for downstream in sorted(outgoing[name]):
            indegree[downstream] -= 1
            if indegree[downstream] == 0:
                queue.append(downstream)
                queue.sort()
    if len(result) != len(closure):
        cyclic = sorted(name for name, degree in indegree.items() if degree > 0)
        raise DeployError(f"dependency cycle detected: {', '.join(cyclic)}")
    return result


# Was: Diese Funktion erzeugt Konfiguration.
# Warum: Darstellung und Fachdaten bleiben dadurch voneinander getrennt.
def render_config(template: str, inventory: Inventory) -> str:
    by_port = inventory.by_port

    # Was: Führt den Arbeitsschritt `replacement` für replacement aus.
    # Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    def replacement(match: re.Match[str]) -> str:
        port = int(match.group("port"))
        target = by_port.get(port)
        if target is None:
            return match.group(0)
        return f"{match.group('scheme')}://{target.host}:{port}"

    return URL_RE.sub(replacement, template)


# Was: Diese Funktion schreibt generated.
# Warum: Die Ausgabe wird dadurch einheitlich erzeugt und Schreibfehler können behandelt werden.
def write_generated(inventory: Inventory) -> None:
    GENERATED.mkdir(parents=True, exist_ok=True)
    configs = GENERATED / "configs"
    configs.mkdir(parents=True, exist_ok=True)
    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    for service in inventory.services:
        target_dir = configs / service.name
        target_dir.mkdir(parents=True, exist_ok=True)
        source = (ROOT / service.config_template).read_text(encoding="utf-8")
        output = render_config(source, inventory)
        (target_dir / Path(service.config_target).name).write_text(output, encoding="utf-8")

    catalog = {
        "version": inventory.version,
        "contract_version": inventory.contract_version,
        "mode": inventory.mode,
        "generated_at_epoch": int(os.environ.get("SOURCE_DATE_EPOCH", "0")),
        "services": [
            {
                "name": service.name,
                "host": service.host,
                "port": service.port,
                "base_url": service.base_url,
                "webui": service.base_url + "/",
                "health_live": service.base_url + "/health/live",
                "health_ready": service.base_url + "/health/ready",
                "metrics": service.base_url + "/metrics",
                "unit": service.unit,
                "depends_on": list(service.depends_on),
            }
            for service in inventory.services
        ],
    }
    (GENERATED / "service-catalog.json").write_text(json.dumps(catalog, indent=2) + "\n", encoding="utf-8")
    with (GENERATED / "ports.csv").open("w", newline="", encoding="utf-8") as handle:
        writer = csv.writer(handle)
        writer.writerow(["service", "host", "port", "webui", "unit"])
        # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
        # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
        for service in inventory.services:
            writer.writerow([service.name, service.host, service.port, service.base_url + "/", service.unit])
    hosts = [f"{service.host}\tnetcore-{service.name}" for service in inventory.services]
    (GENERATED / "hosts.example").write_text("\n".join(hosts) + "\n", encoding="utf-8")
    dot = ["digraph netcore_open_lab {", "  rankdir=LR;"]
    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    for service in inventory.services:
        dot.append(f'  "{service.name}" [label="{service.name}\\n{service.host}:{service.port}"];')
        # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
        # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
        for dependency in service.depends_on:
            dot.append(f'  "{dependency}" -> "{service.name}";')
    dot.append("}")
    (GENERATED / "dependency-graph.dot").write_text("\n".join(dot) + "\n", encoding="utf-8")


# Was: Führt den Arbeitsschritt `bundle` für bundle aus.
# Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
def bundle(output: Path) -> str:
    exclusions = {".git", "target", "__pycache__", ".pytest_cache", "node_modules"}
    with output.open("wb") as raw_output:
        with gzip.GzipFile(filename="", mode="wb", fileobj=raw_output, mtime=0) as compressed:
            with tarfile.open(fileobj=compressed, mode="w", format=tarfile.PAX_FORMAT) as archive:
                # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
                # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
                for path in sorted(ROOT.rglob("*")):
                    relative = path.relative_to(ROOT)
                    if any(part in exclusions for part in relative.parts):
                        continue
                    if path.is_file() and path.suffix.lower() == ".pdf":
                        continue
                    if path.resolve() == output.resolve():
                        continue
                    if relative.parts[:3] == ("deploy", "open-lab", "generated") and (
                        path.name.endswith(".tar.gz") or path.name.endswith(".tar.gz.sha256")
                    ):
                        continue
                    info = archive.gettarinfo(str(path), arcname=str(Path("netcore-tetra-swmi") / relative))
                    info.uid = info.gid = 0
                    info.uname = info.gname = "root"
                    info.mtime = 0
                    if path.is_file():
                        with path.open("rb") as handle:
                            archive.addfile(info, handle)
                    else:
                        archive.addfile(info)
    digest = hashlib.sha256(output.read_bytes()).hexdigest()
    output.with_suffix(output.suffix + ".sha256").write_text(f"{digest}  {output.name}\n", encoding="ascii")
    return digest


# Was: Führt den Arbeitsschritt `ssh_command` für ssh command aus.
# Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
def ssh_command(inventory: Inventory, service: Service, remote: str) -> list[str]:
    return ["ssh", *inventory.ssh_options, f"{inventory.ssh_user}@{service.host}", remote]


# Was: Führt den Arbeitsschritt `scp_command` für scp command aus.
# Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
def scp_command(inventory: Inventory, service: Service, local: Path, remote: str) -> list[str]:
    return ["scp", *inventory.ssh_options, str(local), f"{inventory.ssh_user}@{service.host}:{remote}"]


# Was: Diese Funktion führt den vorgesehenen Arbeitsschritt.
# Warum: Der Lebenszyklus des Dienstes bleibt so an einer zentralen Stelle steuerbar.
def run(command: list[str], *, dry_run: bool) -> None:
    print("+", shlex.join(command))
    if not dry_run:
        subprocess.run(command, check=True)


# Was: Diese Funktion wendet den vorgesehenen Arbeitsschritt.
# Warum: Die Änderung wird dadurch nur über einen definierten und prüfbaren Weg wirksam.
def apply(inventory: Inventory, selected: list[str], *, dry_run: bool) -> None:
    write_generated(inventory)
    order = topological_order(inventory, selected or None)
    with tempfile.TemporaryDirectory(prefix="netcore-deploy-") as temp:
        bundle_path = Path(temp) / "netcore-open-lab.tar.gz"
        digest = bundle(bundle_path)
        print(f"bundle sha256={digest}")
        # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
        # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
        for service in order:
            remote_bundle = f"/tmp/netcore-open-lab-{digest[:12]}.tar.gz"
            run(scp_command(inventory, service, bundle_path, remote_bundle), dry_run=dry_run)
            rendered = f"deploy/open-lab/generated/configs/{service.name}/{Path(service.config_target).name}"
            remote = " && ".join(
                [
                    "set -euo pipefail",
                    f"rm -rf {shlex.quote(inventory.remote_source_root)}",
                    f"mkdir -p {shlex.quote(inventory.remote_source_root)}",
                    f"tar -xzf {shlex.quote(remote_bundle)} -C {shlex.quote(inventory.remote_source_root)} --strip-components=1",
                    f"cd {shlex.quote(inventory.remote_source_root)}",
                    f"bash {shlex.quote(str(service.install))}",
                    f"install -o root -g {shlex.quote(service.user)} -m 0640 {shlex.quote(rendered)} {shlex.quote(service.config_target)}",
                    f"systemctl restart {shlex.quote(service.unit)}",
                    f"systemctl is-active --quiet {shlex.quote(service.unit)}",
                    f"rm -f {shlex.quote(remote_bundle)}",
                ]
            )
            run(ssh_command(inventory, service, remote), dry_run=dry_run)
            if not dry_run:
                check_health(service, inventory.health_timeout_secs, ready=True)


# Was: Diese Funktion prüft health.
# Warum: Fehler oder unzulässige Zustände werden dadurch früh erkannt.
def check_health(service: Service, timeout: int, *, ready: bool) -> tuple[bool, str]:
    path = "/health/ready" if ready else "/health/live"
    url = service.base_url + path
    request = urllib.request.Request(url, headers={"Accept": "application/json", "User-Agent": "netcore-deploy/1"})
    # Was: Führt einen fehleranfälligen Abschnitt mit geregelter Fehlerbehandlung aus.
    # Warum: Ein einzelner Fehler soll kontrolliert gemeldet oder aufgefangen werden, statt den gesamten Dienst unverständlich abzubrechen.
    try:
        with urllib.request.urlopen(request, timeout=timeout) as response:
            body = response.read(4096).decode("utf-8", errors="replace")
            return 200 <= response.status < 300, body
    except (urllib.error.URLError, TimeoutError, OSError) as error:
        return False, str(error)


# Was: Führt den Arbeitsschritt `status` für Status aus.
# Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
def status(inventory: Inventory, selected: list[str]) -> int:
    services = topological_order(inventory, selected or None)
    failures = 0
    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    for service in services:
        ok, detail = check_health(service, inventory.health_timeout_secs, ready=True)
        print(f"{'READY' if ok else 'DOWN ':5} {service.name:22} {service.base_url} {detail[:160]}")
        failures += int(not ok)
    return 1 if failures else 0


# Was: Startet das Programm, lädt die benötigten Einstellungen und übergibt an den eigentlichen Dienstablauf.
# Warum: Ein klarer Einstiegspunkt hält Startreihenfolge, Fehlerausgabe und geordnetes Beenden zusammen.
def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--inventory", type=Path, default=DEFAULT_INVENTORY)
    sub = parser.add_subparsers(dest="command", required=True)
    sub.add_parser("validate", help="validate files, ports and dependency graph")
    plan_parser = sub.add_parser("plan", help="print dependency-resolved deployment order")
    plan_parser.add_argument("services", nargs="*")
    sub.add_parser("render", help="render service configs, catalog, hosts and graph")
    bundle_parser = sub.add_parser("bundle", help="create deterministic no-PDF source bundle")
    bundle_parser.add_argument("--output", type=Path, default=GENERATED / "netcore-open-lab.tar.gz")
    apply_parser = sub.add_parser("apply", help="deploy through SSH in dependency order")
    apply_parser.add_argument("services", nargs="*")
    apply_parser.add_argument("--dry-run", action="store_true")
    status_parser = sub.add_parser("status", help="check readiness endpoints")
    status_parser.add_argument("services", nargs="*")
    test_parser = sub.add_parser("test", help="run cross-LXC Open-Lab E2E scenarios")
    test_parser.add_argument("--profile", choices=("smoke", "full", "fault"), default="smoke")
    test_parser.add_argument("--scenario", action="append")
    test_parser.add_argument("--allow-mutations", action="store_true")
    test_parser.add_argument("--allow-restarts", action="store_true")
    test_parser.add_argument("--keep-fixtures", action="store_true")
    test_parser.add_argument("--no-mock-tbs", action="store_true")
    test_parser.add_argument("--strict-ready", action="store_true")
    test_parser.add_argument("--validate-only", action="store_true")
    test_parser.add_argument("--list-scenarios", action="store_true")
    test_parser.add_argument("--timeout", type=float, default=25.0)
    test_parser.add_argument("--run-id")
    test_parser.add_argument("--node-id")
    test_parser.add_argument("--artifacts-dir", type=Path)
    args = parser.parse_args()

    inventory = load_inventory(args.inventory)
    errors = validate(inventory)
    if errors:
        # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
        # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
        for error in errors:
            print(f"ERROR: {error}", file=sys.stderr)
        return 2

    if args.command == "validate":
        print(f"OK: {len(inventory.services)} services, contract={inventory.contract_version}, mode={inventory.mode}")
        return 0
    if args.command == "plan":
        # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
        # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
        for index, service in enumerate(topological_order(inventory, args.services or None), 1):
            dependencies = ",".join(service.depends_on) or "-"
            print(f"{index:02d} {service.name:22} {service.host}:{service.port:<5} depends={dependencies}")
        return 0
    if args.command == "render":
        write_generated(inventory)
        print(f"Rendered deployment assets into {GENERATED.relative_to(ROOT)}")
        return 0
    if args.command == "bundle":
        args.output.parent.mkdir(parents=True, exist_ok=True)
        digest = bundle(args.output)
        print(f"{digest}  {args.output}")
        return 0
    if args.command == "apply":
        apply(inventory, args.services, dry_run=args.dry_run)
        return 0
    if args.command == "status":
        return status(inventory, args.services)
    if args.command == "test":
        command = [
            sys.executable,
            str(ROOT / "tests/e2e/netcore_open_lab_e2e.py"),
            "--inventory",
            str(args.inventory),
            "--profile",
            args.profile,
            "--timeout",
            str(args.timeout),
        ]
        # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
        # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
        for scenario in args.scenario or []:
            command.extend(["--scenario", scenario])
        # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
        # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
        for value, flag in (
            (args.run_id, "--run-id"),
            (args.node_id, "--node-id"),
            (args.artifacts_dir, "--artifacts-dir"),
        ):
            if value is not None:
                command.extend([flag, str(value)])
        # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
        # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
        for enabled, flag in (
            (args.allow_mutations, "--allow-mutations"),
            (args.allow_restarts, "--allow-restarts"),
            (args.keep_fixtures, "--keep-fixtures"),
            (args.no_mock_tbs, "--no-mock-tbs"),
            (args.strict_ready, "--strict-ready"),
            (args.validate_only, "--validate-only"),
            (args.list_scenarios, "--list-scenarios"),
        ):
            if enabled:
                command.append(flag)
        return subprocess.run(command, cwd=ROOT).returncode
    return 2


# Was: Startet den Programmablauf nur dann, wenn diese Datei direkt ausgeführt wird.
# Warum: Beim Import als Modul sollen nur Funktionen bereitstehen und keine Nebenwirkungen automatisch starten.
if __name__ == "__main__":
    raise SystemExit(main())
