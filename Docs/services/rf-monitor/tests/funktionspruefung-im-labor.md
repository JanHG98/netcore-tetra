# HF-Überwachung: Funktionsprüfung im Labor

Die Beispiele verwenden synthetische RF-Werte. `RF-MONITOR-IP` ersetzen; angezeigte VSWR-/Alarmwerte belegen Berechnung und Zustandsübergang, keine kalibrierte Messung am Sender.


```bash
curl -fsS -X POST -H 'Content-Type: application/json' \
  http://RF-MONITOR-IP:8260/api/v1/telemetry \
  -d '{
    "schema":"netcore-rf-telemetry-v1",
    "station_id":"TBS-TEST",
    "tx_active":true,
    "metrics":{
      "forward_power_w":5.0,
      "reflected_power_w":0.2,
      "pa_temperature_c":52.0,
      "sdr_temperature_c":43.0,
      "evm_pct":3.1
    },
    "inputs":{"pll_lock":true}
  }' | python3 -m json.tool

curl -fsS http://RF-MONITOR-IP:8260/api/v1/stations | python3 -m json.tool
curl -fsS http://RF-MONITOR-IP:8260/api/v1/alarms | python3 -m json.tool
```

**Quellabgleich: 9. Oktober 2026.** [Quellcode](../../../../system-backend/rf-monitor) · [Konfigurationsvorlagen](../../../../system-backend/rf-monitor/config).
