#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import math
import os
import signal
import subprocess
import threading
import time
import tomllib
import uuid
from datetime import datetime, timezone
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import unquote, urlparse


def utc_now() -> str:
    return datetime.now(timezone.utc).isoformat().replace("+00:00", "Z")


def safe_id(value: object) -> str:
    text = str(value or "")
    return "".join(ch for ch in text if ch.isalnum() or ch in "-_.")[:128]


def severity_rank(value: str) -> int:
    return {"ok": 0, "info": 0, "warning": 1, "critical": 2, "offline": 3}.get(value, 0)


def finite_number(value: object) -> float | None:
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        return None
    number = float(value)
    return number if math.isfinite(number) else None


class Config:
    def __init__(self, raw: dict):
        self.raw = raw

    @property
    def bind(self) -> tuple[str, int]:
        host, port = self.raw["server"]["bind"].rsplit(":", 1)
        return host, int(port)

    @property
    def mqtt(self) -> dict:
        return self.raw["mqtt"]

    @property
    def storage(self) -> dict:
        return self.raw["storage"]

    @property
    def monitoring(self) -> dict:
        return self.raw["monitoring"]

    @property
    def thresholds(self) -> dict:
        return self.raw["thresholds"]


class RFMonitor:
    def __init__(self, config: Config):
        self.config = config
        self.lock = threading.RLock()
        self.stations: dict[str, dict] = {}
        self.events: list[dict] = []
        self.started_at = utc_now()
        self.stop_requested = False
        self.state_path = Path(config.storage["state_file"])
        self.event_path = Path(config.storage["event_log"])
        self.state_path.parent.mkdir(parents=True, exist_ok=True)
        self.event_path.parent.mkdir(parents=True, exist_ok=True)
        self._load_state()

    def _load_state(self) -> None:
        try:
            data = json.loads(self.state_path.read_text(encoding="utf-8"))
            self.stations = data.get("stations", {}) if isinstance(data, dict) else {}
        except Exception:
            self.stations = {}

        try:
            lines = self.event_path.read_text(encoding="utf-8").splitlines()
            parsed = []
            for line in lines[-int(self.config.monitoring.get("event_memory_limit", 1000)) :]:
                try:
                    parsed.append(json.loads(line))
                except Exception:
                    continue
            self.events = parsed
        except Exception:
            self.events = []

    def persist(self) -> None:
        temp = self.state_path.with_suffix(self.state_path.suffix + ".tmp")
        temp.write_text(
            json.dumps({"stations": self.stations}, ensure_ascii=False, indent=2),
            encoding="utf-8",
        )
        temp.replace(self.state_path)

    def mqtt_publish(self, topic: str, payload: dict, retain: bool = False, qos: int = 0) -> bool:
        mqtt = self.config.mqtt
        if not mqtt.get("enabled", True):
            return False
        command = [
            "mosquitto_pub",
            "-h",
            str(mqtt["host"]),
            "-p",
            str(mqtt["port"]),
            "-q",
            str(qos),
            "-t",
            topic,
            "-m",
            json.dumps(payload, ensure_ascii=False, separators=(",", ":")),
        ]
        if retain:
            command.append("-r")
        result = subprocess.run(
            command,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            check=False,
        )
        return result.returncode == 0

    def add_event(self, event_type: str, station_id: str, severity: str, payload: dict) -> dict:
        event = {
            "schema": "netcore-event-v1",
            "event_id": str(uuid.uuid4()),
            "event_type": event_type,
            "source": {
                "service": "netcore-rf-monitor",
                "instance": os.uname().nodename,
                "node_id": station_id,
            },
            "timestamp": utc_now(),
            "severity": severity,
            "subject": {"type": "rf_station", "id": station_id},
            "payload": payload,
            "deduplication_key": f"netcore-rf-monitor:{station_id}:{event_type}:{payload.get('alarm_key', '')}",
        }
        with self.lock:
            self.events.append(event)
            limit = int(self.config.monitoring.get("event_memory_limit", 1000))
            self.events = self.events[-limit:]
            with self.event_path.open("a", encoding="utf-8") as handle:
                handle.write(json.dumps(event, ensure_ascii=False, separators=(",", ":")) + "\n")
        prefix = self.config.mqtt["topic_prefix"]
        self.mqtt_publish(f"{prefix}/events/{event_type.replace('.', '/')}", event, qos=1)
        return event

    @staticmethod
    def derive_metrics(metrics: dict) -> dict:
        result = dict(metrics)
        forward = finite_number(result.get("forward_power_w"))
        reflected = finite_number(result.get("reflected_power_w"))
        if forward is not None and reflected is not None and forward > 0:
            ratio = max(0.0, reflected / forward)
            result.setdefault("reflected_power_ratio_percent", ratio * 100.0)
            gamma = min(math.sqrt(ratio), 0.999999)
            result.setdefault("vswr", (1.0 + gamma) / (1.0 - gamma))
            if gamma > 0:
                result.setdefault("return_loss_db", -20.0 * math.log10(gamma))
            else:
                result.setdefault("return_loss_db", 99.0)
        return result

    def evaluate_alarms(self, station: dict) -> dict[str, dict]:
        metrics = station.get("metrics", {})
        inputs = station.get("inputs", {})
        tx_active = bool(station.get("tx_active"))
        thresholds = self.config.thresholds
        alarms: dict[str, dict] = {}

        def compare_high(key: str, metric: str, warning: str, critical: str, unit: str = "") -> None:
            value = finite_number(metrics.get(metric))
            if value is None:
                return
            warning_value = finite_number(thresholds.get(warning))
            critical_value = finite_number(thresholds.get(critical))
            severity = None
            limit = None
            if critical_value is not None and value >= critical_value:
                severity, limit = "critical", critical_value
            elif warning_value is not None and value >= warning_value:
                severity, limit = "warning", warning_value
            if severity:
                alarms[key] = {
                    "alarm_key": key,
                    "severity": severity,
                    "metric": metric,
                    "value": value,
                    "limit": limit,
                    "unit": unit,
                    "reason": "high",
                }

        def compare_low(
            key: str,
            metric: str,
            warning: str,
            critical: str,
            unit: str = "",
            only_when_tx: bool = False,
        ) -> None:
            if only_when_tx and not tx_active:
                return
            value = finite_number(metrics.get(metric))
            if value is None:
                return
            warning_value = finite_number(thresholds.get(warning))
            critical_value = finite_number(thresholds.get(critical))
            severity = None
            limit = None
            if critical_value is not None and critical_value > 0 and value <= critical_value:
                severity, limit = "critical", critical_value
            elif warning_value is not None and warning_value > 0 and value <= warning_value:
                severity, limit = "warning", warning_value
            if severity:
                alarms[key] = {
                    "alarm_key": key,
                    "severity": severity,
                    "metric": metric,
                    "value": value,
                    "limit": limit,
                    "unit": unit,
                    "reason": "low",
                }

        compare_high("vswr_high", "vswr", "vswr_warning", "vswr_critical")
        compare_high(
            "reflected_power_high",
            "reflected_power_ratio_percent",
            "reflected_ratio_warning_percent",
            "reflected_ratio_critical_percent",
            "%",
        )
        compare_high("pa_temperature_high", "pa_temperature_c", "pa_temp_warning_c", "pa_temp_critical_c", "°C")
        compare_high("sdr_temperature_high", "sdr_temperature_c", "sdr_temp_warning_c", "sdr_temp_critical_c", "°C")
        compare_high("cabinet_temperature_high", "cabinet_temperature_c", "cabinet_temp_warning_c", "cabinet_temp_critical_c", "°C")
        compare_high("evm_high", "evm_pct", "evm_warning_pct", "evm_critical_pct", "%")
        compare_high("papr_high", "papr_db", "papr_warning_db", "papr_critical_db", "dB")
        compare_low(
            "forward_power_low",
            "forward_power_w",
            "forward_power_warning_below_w",
            "forward_power_critical_below_w",
            "W",
            only_when_tx=True,
        )
        compare_low("supply_voltage_low", "pa_voltage_v", "pa_voltage_warning_below_v", "pa_voltage_critical_below_v", "V")
        compare_low("fan_speed_low", "fan_rpm", "fan_warning_below_rpm", "fan_critical_below_rpm", "rpm", only_when_tx=True)

        boolean_rules = {
            "antenna_fault": ("antenna_fault", "critical"),
            "pa_fault": ("pa_fault", "critical"),
            "high_swr_trip": ("high_swr_trip", "critical"),
            "fan_fault": ("fan_fault", "warning"),
        }
        for alarm_key, (input_key, severity) in boolean_rules.items():
            if inputs.get(input_key) is True:
                alarms[alarm_key] = {
                    "alarm_key": alarm_key,
                    "severity": severity,
                    "input": input_key,
                    "value": True,
                    "reason": "digital_input_active",
                }
        if inputs.get("pll_lock") is False:
            alarms["pll_unlocked"] = {
                "alarm_key": "pll_unlocked",
                "severity": "critical",
                "input": "pll_lock",
                "value": False,
                "reason": "digital_input_inactive",
            }
        return alarms

    def station_health(self, station: dict) -> str:
        if not station.get("online", False):
            return "offline"
        alarms = station.get("alarms", {})
        if any(item.get("severity") == "critical" for item in alarms.values()):
            return "critical"
        if any(item.get("severity") == "warning" for item in alarms.values()):
            return "warning"
        return "ok"

    def ingest(self, payload: dict, source: str = "http") -> dict:
        if not isinstance(payload, dict):
            raise ValueError("Payload muss ein JSON-Objekt sein")
        schema = payload.get("schema", "netcore-rf-telemetry-v1")
        if schema != "netcore-rf-telemetry-v1":
            raise ValueError("schema muss netcore-rf-telemetry-v1 sein")
        station_id = safe_id(payload.get("station_id") or payload.get("device_id"))
        if not station_id:
            raise ValueError("station_id fehlt")
        metrics = payload.get("metrics", {})
        inputs = payload.get("inputs", {})
        metadata = payload.get("metadata", {})
        spectrum = payload.get("spectrum")
        if not isinstance(metrics, dict) or not isinstance(inputs, dict) or not isinstance(metadata, dict):
            raise ValueError("metrics, inputs und metadata müssen Objekte sein")
        metrics = self.derive_metrics(metrics)
        if spectrum is not None:
            if not isinstance(spectrum, dict):
                raise ValueError("spectrum muss ein Objekt sein")
            bins = spectrum.get("bins_db")
            if isinstance(bins, list):
                max_bins = int(self.config.monitoring.get("max_spectrum_bins", 512))
                spectrum = dict(spectrum)
                spectrum["bins_db"] = [finite_number(value) for value in bins[:max_bins]]

        timestamp = utc_now()
        with self.lock:
            previous = self.stations.get(station_id)
            previous_online = bool(previous and previous.get("online"))
            previous_tx_active = previous.get("tx_active") if previous else None
            previous_alarms = dict(previous.get("alarms", {})) if previous else {}
            station = {
                "schema": "netcore-rf-state-v1",
                "station_id": station_id,
                "node_id": safe_id(payload.get("node_id")) or station_id,
                "name": str(payload.get("name") or station_id),
                "location": payload.get("location"),
                "source": source,
                "last_seen": timestamp,
                "captured_at": payload.get("captured_at") or timestamp,
                "online": True,
                "tx_active": bool(payload.get("tx_active", False)),
                "metrics": metrics,
                "inputs": inputs,
                "metadata": metadata,
                "spectrum": spectrum,
                "alarms": {},
            }
            station["alarms"] = self.evaluate_alarms(station)
            station["health"] = self.station_health(station)
            self.stations[station_id] = station
            self.persist()

        if previous is None:
            self.add_event(
                "rf.station_registered",
                station_id,
                "info",
                {"name": station["name"], "node_id": station["node_id"], "source": source},
            )
        elif not previous_online:
            self.add_event("rf.station_online", station_id, "info", {"last_seen": timestamp})
        if previous_tx_active is not None and previous_tx_active != station["tx_active"]:
            self.add_event(
                "rf.tx_state_changed",
                station_id,
                "info",
                {"tx_active": station["tx_active"]},
            )

        current_alarms = station["alarms"]
        for key, alarm in current_alarms.items():
            previous_alarm = previous_alarms.get(key)
            if previous_alarm != alarm:
                self.add_event("rf.alarm_raised", station_id, alarm["severity"], alarm)
        for key, alarm in previous_alarms.items():
            if key not in current_alarms:
                self.add_event(
                    "rf.alarm_cleared",
                    station_id,
                    "info",
                    {"alarm_key": key, "previous": alarm},
                )

        prefix = self.config.mqtt["topic_prefix"]
        self.mqtt_publish(f"{prefix}/state/rf/{station_id}", station, retain=True, qos=1)
        return station

    def watchdog_loop(self) -> None:
        timeout = int(self.config.monitoring["heartbeat_timeout_secs"])
        while not self.stop_requested:
            cutoff = time.time() - timeout
            changed: list[str] = []
            with self.lock:
                for station_id, station in self.stations.items():
                    try:
                        last_seen = datetime.fromisoformat(station["last_seen"].replace("Z", "+00:00")).timestamp()
                    except Exception:
                        last_seen = 0.0
                    if station.get("online", False) and last_seen < cutoff:
                        station["online"] = False
                        station["health"] = "offline"
                        changed.append(station_id)
                if changed:
                    self.persist()
            for station_id in changed:
                self.add_event(
                    "rf.station_offline",
                    station_id,
                    "warning",
                    {"heartbeat_timeout_secs": timeout},
                )
                prefix = self.config.mqtt["topic_prefix"]
                self.mqtt_publish(
                    f"{prefix}/state/rf/{station_id}",
                    self.stations[station_id],
                    retain=True,
                    qos=1,
                )
            time.sleep(2)

    def mqtt_loop(self) -> None:
        mqtt = self.config.mqtt
        if not mqtt.get("enabled", True):
            return
        topic = f"{mqtt['topic_prefix']}/rf/+/telemetry"
        while not self.stop_requested:
            command = [
                "mosquitto_sub",
                "-h",
                str(mqtt["host"]),
                "-p",
                str(mqtt["port"]),
                "-v",
                "-t",
                topic,
            ]
            process = subprocess.Popen(
                command,
                stdout=subprocess.PIPE,
                stderr=subprocess.DEVNULL,
                text=True,
            )
            while not self.stop_requested and process.poll() is None:
                line = process.stdout.readline() if process.stdout else ""
                if not line:
                    break
                try:
                    message_topic, raw = line.rstrip().split(" ", 1)
                    self.ingest(json.loads(raw), source=f"mqtt:{message_topic}")
                except Exception as error:
                    station_id = safe_id(message_topic.split("/")[-2]) if "message_topic" in locals() else "unknown"
                    self.add_event(
                        "rf.telemetry_invalid",
                        station_id or "unknown",
                        "warning",
                        {"error": str(error)},
                    )
            process.terminate()
            try:
                process.wait(timeout=2)
            except subprocess.TimeoutExpired:
                process.kill()
            time.sleep(2)

    def status(self) -> dict:
        with self.lock:
            stations = list(self.stations.values())
        health_counts = {"ok": 0, "warning": 0, "critical": 0, "offline": 0}
        for station in stations:
            health_counts[station.get("health", "ok")] = health_counts.get(station.get("health", "ok"), 0) + 1
        return {
            "service": "netcore-rf-monitor",
            "phase": 7,
            "security_mode": "open_lab",
            "started_at": self.started_at,
            "mqtt": {
                "enabled": self.config.mqtt.get("enabled", True),
                "host": self.config.mqtt["host"],
                "port": self.config.mqtt["port"],
                "topic_prefix": self.config.mqtt["topic_prefix"],
            },
            "stations_total": len(stations),
            "stations_online": sum(1 for item in stations if item.get("online")),
            "tx_active": sum(1 for item in stations if item.get("tx_active")),
            "active_alarms": sum(len(item.get("alarms", {})) for item in stations),
            "health_counts": health_counts,
            "warning": "OPEN LAB: keine Anmeldung, keine Tokens und kein TLS",
        }

    def prometheus(self) -> str:
        lines = [
            "# HELP netcore_rf_station_up RF station heartbeat state",
            "# TYPE netcore_rf_station_up gauge",
            "# HELP netcore_rf_station_alarm_active Active RF alarm count",
            "# TYPE netcore_rf_station_alarm_active gauge",
        ]
        with self.lock:
            stations = list(self.stations.values())
        for station in stations:
            station_id = station["station_id"].replace('"', "")
            lines.append(f'netcore_rf_station_up{{station_id="{station_id}"}} {1 if station.get("online") else 0}')
            lines.append(f'netcore_rf_station_alarm_active{{station_id="{station_id}"}} {len(station.get("alarms", {}))}')
            for metric in (
                "forward_power_w",
                "reflected_power_w",
                "reflected_power_ratio_percent",
                "vswr",
                "return_loss_db",
                "pa_temperature_c",
                "sdr_temperature_c",
                "fan_rpm",
                "evm_pct",
                "papr_db",
            ):
                value = finite_number(station.get("metrics", {}).get(metric))
                if value is not None:
                    lines.append(f'netcore_rf_{metric}{{station_id="{station_id}"}} {value}')
        return "\n".join(lines) + "\n"


HTML = r'''<!doctype html>
<html lang="de"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>NetCore RF Monitor</title>
<style>
:root{color-scheme:dark;--bg:#0c1118;--panel:#141c27;--line:#2a394b;--text:#e9f1fb;--muted:#92a3b7;--ok:#64d98b;--warn:#ffca58;--bad:#ff6f6f;--accent:#63a8ff}
*{box-sizing:border-box}body{font-family:system-ui,sans-serif;background:var(--bg);color:var(--text);margin:0;padding:24px}.wrap{max-width:1500px;margin:auto}h1{margin:0 0 4px}.muted{color:var(--muted)}.cards{display:grid;grid-template-columns:repeat(auto-fit,minmax(180px,1fr));gap:12px;margin:22px 0}.card,.panel{background:var(--panel);border:1px solid var(--line);border-radius:12px;padding:16px}.value{font-size:28px;font-weight:750}.ok{color:var(--ok)}.warning{color:var(--warn)}.critical,.offline{color:var(--bad)}table{border-collapse:collapse;width:100%}th,td{text-align:left;padding:10px;border-bottom:1px solid var(--line);vertical-align:top}code{background:#202c3b;padding:2px 5px;border-radius:5px}button{background:#26364a;color:var(--text);border:1px solid #3c536d;border-radius:7px;padding:6px 10px;cursor:pointer}.grid{display:grid;grid-template-columns:2fr 1fr;gap:12px}@media(max-width:900px){.grid{grid-template-columns:1fr}}canvas{width:100%;height:240px;background:#091019;border-radius:8px}.pill{display:inline-block;padding:2px 8px;border-radius:999px;border:1px solid currentColor;font-size:12px}.small{font-size:12px}
</style></head><body><div class="wrap">
<h1>NetCore RF Monitor · OPEN LAB</h1><div class="muted">HF-Zustand, PA/SDR-Telemetrie, Antennenfehler und Spektrum. Reine Überwachung – keine Sendersteuerung.</div>
<div class="cards" id="cards"></div>
<div class="grid"><div class="panel"><h2>Basisstationen</h2><table><thead><tr><th>Station</th><th>Zustand</th><th>TX</th><th>Leistung</th><th>VSWR</th><th>Temperaturen</th><th>Alarme</th><th></th></tr></thead><tbody id="stations"></tbody></table></div>
<div class="panel"><h2 id="detail-title">Spektrum</h2><canvas id="spectrum" width="700" height="240"></canvas><pre id="detail" class="small muted">Station auswählen</pre></div></div>
<div class="panel" style="margin-top:12px"><h2>Aktive Alarme</h2><table><thead><tr><th>Station</th><th>Schwere</th><th>Alarm</th><th>Details</th></tr></thead><tbody id="alarms"></tbody></table></div>
</div><script>
let selected=null,stations=[];
const f=(v,d=1)=>typeof v==='number'&&Number.isFinite(v)?v.toFixed(d):'—';
function drawSpectrum(st){const c=document.querySelector('#spectrum'),x=c.getContext('2d'),bins=st?.spectrum?.bins_db||[];x.clearRect(0,0,c.width,c.height);x.strokeStyle='#25364a';for(let i=0;i<6;i++){let y=i*c.height/5;x.beginPath();x.moveTo(0,y);x.lineTo(c.width,y);x.stroke()}if(!bins.length){x.fillStyle='#92a3b7';x.fillText('Keine Spektrumsdaten',20,30);return}let lo=Math.min(...bins),hi=Math.max(...bins);if(hi-lo<10){lo=hi-10}x.strokeStyle='#63a8ff';x.lineWidth=2;x.beginPath();bins.forEach((v,i)=>{let px=i/(bins.length-1)*c.width,py=c.height-(v-lo)/(hi-lo)*c.height;i?x.lineTo(px,py):x.moveTo(px,py)});x.stroke();}
function selectStation(id){selected=id;renderDetail();}
function renderDetail(){let st=stations.find(x=>x.station_id===selected)||stations[0];if(!st){drawSpectrum(null);return}selected=st.station_id;document.querySelector('#detail-title').textContent=`${st.name} · Spektrum`;document.querySelector('#detail').textContent=JSON.stringify({station_id:st.station_id,last_seen:st.last_seen,metrics:st.metrics,inputs:st.inputs,metadata:st.metadata},null,2);drawSpectrum(st)}
async function refresh(){let s=await(await fetch('/api/v1/status')).json();stations=await(await fetch('/api/v1/stations')).json();let alarms=await(await fetch('/api/v1/alarms')).json();document.querySelector('#cards').innerHTML=[['Stationen',s.stations_online+'/'+s.stations_total],['TX aktiv',s.tx_active],['Alarme',s.active_alarms],['Kritisch',s.health_counts.critical||0],['Offline',s.health_counts.offline||0]].map(([a,b])=>`<div class="card"><div class="muted">${a}</div><div class="value">${b}</div></div>`).join('');document.querySelector('#stations').innerHTML=stations.map(st=>{let m=st.metrics||{},temps=[m.pa_temperature_c!=null?'PA '+f(m.pa_temperature_c)+' °C':'',m.sdr_temperature_c!=null?'SDR '+f(m.sdr_temperature_c)+' °C':''].filter(Boolean).join('<br>')||'—';return `<tr><td><b>${st.name}</b><br><code>${st.station_id}</code><br><span class="small muted">${st.last_seen}</span></td><td class="${st.health}"><span class="pill">${st.health}</span></td><td>${st.tx_active?'aktiv':'frei'}</td><td>${f(m.forward_power_w,2)} W<br><span class="small muted">Rück ${f(m.reflected_power_w,2)} W</span></td><td>${f(m.vswr,2)}<br><span class="small muted">${f(m.return_loss_db,1)} dB RL</span></td><td>${temps}</td><td>${Object.keys(st.alarms||{}).length}</td><td><button onclick="selectStation('${st.station_id}')">Details</button></td></tr>`}).join('');document.querySelector('#alarms').innerHTML=alarms.map(a=>`<tr><td><code>${a.station_id}</code></td><td class="${a.severity}">${a.severity}</td><td>${a.alarm_key}</td><td><code>${JSON.stringify(a)}</code></td></tr>`).join('')||'<tr><td colspan="4" class="muted">Keine aktiven Alarme</td></tr>';renderDetail()}
refresh();setInterval(refresh,3000);
</script></body></html>'''


class Handler(BaseHTTPRequestHandler):
    app: RFMonitor | None = None

    def send_json(self, obj: object, status: int = 200) -> None:
        body = json.dumps(obj, ensure_ascii=False).encode("utf-8")
        self.send_response(status)
        self.send_header("Content-Type", "application/json; charset=utf-8")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def send_text(self, body: str, content_type: str = "text/plain; charset=utf-8", status: int = 200) -> None:
        data = body.encode("utf-8")
        self.send_response(status)
        self.send_header("Content-Type", content_type)
        self.send_header("Content-Length", str(len(data)))
        self.end_headers()
        self.wfile.write(data)

    def do_GET(self) -> None:
        assert self.app is not None
        path = urlparse(self.path).path
        if path == "/":
            return self.send_text(HTML, "text/html; charset=utf-8")
        if path == "/health/live":
            return self.send_json({"status": "ok", "service": "netcore-rf-monitor", "mode": "open_lab"})
        if path == "/health/ready":
            return self.send_json({"status": "ready", "service": "netcore-rf-monitor", "mode": "open_lab"})
        if path == "/metrics":
            return self.send_text(self.app.prometheus(), "text/plain; version=0.0.4; charset=utf-8")
        if path == "/openapi.json":
            return self.send_json({
                "openapi": "3.0.3",
                "info": {"title": "NetCore RF Monitor", "version": "1.0.0-openlab"},
                "paths": {
                    "/api/v1/status": {"get": {}},
                    "/api/v1/stations": {"get": {}},
                    "/api/v1/alarms": {"get": {}},
                    "/api/v1/events": {"get": {}},
                    "/api/v1/telemetry": {"post": {}},
                    "/health/live": {"get": {}},
                    "/health/ready": {"get": {}},
                    "/metrics": {"get": {}},
                },
            })
        if path == "/api/v1/status":
            return self.send_json(self.app.status())
        if path == "/api/v1/stations":
            with self.app.lock:
                stations = sorted(self.app.stations.values(), key=lambda item: item["station_id"])
            return self.send_json(stations)
        if path.startswith("/api/v1/stations/"):
            station_id = safe_id(unquote(path.split("/", 4)[-1]))
            with self.app.lock:
                station = self.app.stations.get(station_id)
            return self.send_json(station, 200) if station else self.send_json({"error": "not_found"}, 404)
        if path == "/api/v1/events":
            with self.app.lock:
                return self.send_json(self.app.events[-200:])
        if path == "/api/v1/alarms":
            alarms = []
            with self.app.lock:
                for station in self.app.stations.values():
                    for alarm in station.get("alarms", {}).values():
                        alarms.append({"station_id": station["station_id"], **alarm})
            alarms.sort(key=lambda item: severity_rank(item.get("severity", "info")), reverse=True)
            return self.send_json(alarms)
        return self.send_json({"error": "not_found"}, 404)

    def do_POST(self) -> None:
        assert self.app is not None
        path = urlparse(self.path).path
        if path != "/api/v1/telemetry":
            return self.send_json({"error": "not_found"}, 404)
        try:
            length = int(self.headers.get("Content-Length", "0"))
            if length <= 0 or length > int(self.app.config.monitoring.get("max_payload_bytes", 524288)):
                raise ValueError("ungültige Payload-Größe")
            payload = json.loads(self.rfile.read(length))
            station = self.app.ingest(payload, source="http")
            return self.send_json(station, 202)
        except Exception as error:
            return self.send_json({"error": str(error)}, 400)

    def log_message(self, *_args: object) -> None:
        return


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--config", default="/etc/netcore/rf-monitor.toml")
    args = parser.parse_args()
    with open(args.config, "rb") as handle:
        config = Config(tomllib.load(handle))
    if config.raw["security"]["mode"] != "open_lab":
        raise SystemExit("Phase 7 unterstützt ausschließlich security.mode=open_lab")
    app = RFMonitor(config)
    Handler.app = app
    threading.Thread(target=app.watchdog_loop, daemon=True).start()
    threading.Thread(target=app.mqtt_loop, daemon=True).start()
    host, port = config.bind
    server = ThreadingHTTPServer((host, port), Handler)

    def stop(*_args: object) -> None:
        app.stop_requested = True
        threading.Thread(target=server.shutdown, daemon=True).start()

    signal.signal(signal.SIGTERM, stop)
    signal.signal(signal.SIGINT, stop)
    server.serve_forever()


if __name__ == "__main__":
    main()
