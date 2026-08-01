#!/usr/bin/env python3
from __future__ import annotations

import argparse
import http.cookiejar
import json
import os
import shlex
import subprocess
import time
import tomllib
from datetime import datetime, timezone
from pathlib import Path
from urllib.error import HTTPError, URLError
from urllib.parse import urlencode
from urllib.request import HTTPCookieProcessor, Request, build_opener, urlopen


def now() -> str:
    return datetime.now(timezone.utc).isoformat().replace("+00:00", "Z")


def merge_dict(target: dict, source: dict | None) -> None:
    if isinstance(source, dict):
        target.update(source)


class DashboardClient:
    def __init__(self, base_url: str, username: str = "", password: str = "", timeout: float = 4.0):
        self.base_url = base_url.rstrip("/")
        self.username = username
        self.password = password
        self.timeout = timeout
        self.cookies = http.cookiejar.CookieJar()
        self.opener = build_opener(HTTPCookieProcessor(self.cookies))
        self.logged_in = False

    def login(self) -> None:
        if not self.username:
            self.logged_in = True
            return
        data = urlencode({"user": self.username, "password": self.password}).encode("utf-8")
        request = Request(
            f"{self.base_url}/api/login",
            data=data,
            headers={"Content-Type": "application/x-www-form-urlencoded"},
            method="POST",
        )
        with self.opener.open(request, timeout=self.timeout) as response:
            response.read()
        self.logged_in = True

    def snapshot(self) -> dict:
        request = Request(f"{self.base_url}/api/rf-monitor", headers={"Accept": "application/json"})
        try:
            with self.opener.open(request, timeout=self.timeout) as response:
                return json.loads(response.read())
        except HTTPError as error:
            if error.code == 401 and self.username:
                self.logged_in = False
                self.login()
                with self.opener.open(request, timeout=self.timeout) as response:
                    return json.loads(response.read())
            raise


def post_json(url: str, payload: dict, timeout: float) -> None:
    body = json.dumps(payload, ensure_ascii=False).encode("utf-8")
    request = Request(
        url,
        data=body,
        headers={"Content-Type": "application/json", "Accept": "application/json"},
        method="POST",
    )
    with urlopen(request, timeout=timeout) as response:
        if response.status not in (200, 201, 202, 204):
            raise RuntimeError(f"RF Monitor antwortete mit HTTP {response.status}")
        response.read()


def run_probe(command: str, timeout: float) -> dict:
    if not command.strip():
        return {}
    result = subprocess.run(
        shlex.split(command),
        capture_output=True,
        text=True,
        timeout=timeout,
        check=False,
    )
    if result.returncode != 0:
        raise RuntimeError(f"Probe-Kommando fehlgeschlagen ({result.returncode}): {result.stderr.strip()}")
    value = json.loads(result.stdout)
    if not isinstance(value, dict):
        raise ValueError("Probe-Kommando muss ein JSON-Objekt ausgeben")
    return value


def transform(snapshot: dict, config: dict, probe: dict) -> dict:
    agent = config["agent"]
    visual = snapshot.get("tx_visual") or {}
    quality = snapshot.get("tx_quality") or {}
    sdr = snapshot.get("sdr_health") or {}
    system = snapshot.get("system_health") or {}

    metrics = {
        "center_freq_hz": visual.get("center_freq_hz"),
        "sample_rate": visual.get("sample_rate"),
        "tx_rms_dbfs": visual.get("rms_dbfs"),
        "tx_peak_dbfs": visual.get("peak_dbfs"),
        "papr_db": quality.get("papr_db"),
        "evm_pct": quality.get("evm_pct"),
        "dc_offset_i": quality.get("dc_offset_i"),
        "dc_offset_q": quality.get("dc_offset_q"),
        "iq_amplitude_imbalance_db": quality.get("iq_amplitude_imbalance_db"),
        "iq_phase_imbalance_deg": quality.get("iq_phase_imbalance_deg"),
        "carrier_leakage_db": quality.get("carrier_leakage_db"),
        "occupied_bandwidth_hz": quality.get("occupied_bandwidth_hz"),
        "sdr_temperature_c": sdr.get("temperature_c"),
        "host_total_power_w": system.get("total_power_w"),
    }
    metrics = {key: value for key, value in metrics.items() if value is not None}
    inputs: dict = {}
    metadata = {
        "agent_version": "phase7-openlab-1",
        "dashboard_schema": snapshot.get("schema"),
        "active_calls": snapshot.get("active_calls", 0),
        "tx_gains": sdr.get("tx_gains", []),
        "rx_gains": sdr.get("rx_gains", []),
        "system_sensors": system.get("sensors", []),
    }
    merge_dict(metrics, probe.get("metrics"))
    merge_dict(inputs, probe.get("inputs"))
    merge_dict(metadata, probe.get("metadata"))

    payload = {
        "schema": "netcore-rf-telemetry-v1",
        "station_id": agent["station_id"],
        "node_id": agent.get("node_id") or agent["station_id"],
        "name": agent.get("name") or agent["station_id"],
        "location": agent.get("location") or None,
        "captured_at": now(),
        "tx_active": bool(snapshot.get("tx_active", False)),
        "metrics": metrics,
        "inputs": inputs,
        "metadata": metadata,
    }
    include_spectrum = bool(agent.get("include_spectrum", False))
    if include_spectrum and isinstance(visual.get("spectrum_db_tenths"), list):
        payload["spectrum"] = {
            "center_freq_hz": visual.get("center_freq_hz"),
            "sample_rate": visual.get("sample_rate"),
            "bins_db": [value / 10.0 for value in visual["spectrum_db_tenths"]],
        }
    if isinstance(probe.get("spectrum"), dict):
        payload["spectrum"] = probe["spectrum"]
    return payload


def load_config(path: str) -> dict:
    with open(path, "rb") as handle:
        config = tomllib.load(handle)
    if config.get("security", {}).get("mode") != "open_lab":
        raise SystemExit("Der Phase-7-Agent unterstützt nur security.mode=open_lab")
    return config


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--config", default="/etc/netcore/rf-agent.toml")
    parser.add_argument("--once", action="store_true")
    args = parser.parse_args()
    config = load_config(args.config)
    agent = config["agent"]
    dashboard = config["dashboard"]
    monitor = config["rf_monitor"]
    probe_config = config.get("external_probe", {})
    client = DashboardClient(
        dashboard["base_url"],
        dashboard.get("username", ""),
        dashboard.get("password", ""),
        float(dashboard.get("timeout_secs", 4)),
    )
    interval = max(1.0, float(agent.get("interval_secs", 5)))

    while True:
        try:
            snapshot = client.snapshot()
            probe = run_probe(
                probe_config.get("command", "") if probe_config.get("enabled", False) else "",
                float(probe_config.get("timeout_secs", 2)),
            )
            payload = transform(snapshot, config, probe)
            post_json(monitor["telemetry_url"], payload, float(monitor.get("timeout_secs", 4)))
            print(f"{now()} OK station={payload['station_id']} tx={payload['tx_active']} metrics={len(payload['metrics'])}", flush=True)
        except (HTTPError, URLError, OSError, ValueError, RuntimeError, subprocess.TimeoutExpired) as error:
            print(f"{now()} ERROR {error}", flush=True)
        if args.once:
            break
        time.sleep(interval)


if __name__ == "__main__":
    main()
