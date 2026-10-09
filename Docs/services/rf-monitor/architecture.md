# Architektur

```text
Bluestation/TBS Dashboard
  GET /api/rf-monitor
          │
          ▼
netcore-rf-agent ─── optional externes Probe-Kommando
          │          (Richtkoppler/ADC/PA/Relaiskontakte)
          ▼
NetCore RF Monitor :8260
  ├── Ableitung VSWR/Return Loss
  ├── Alarmtransitionen und Heartbeat
  ├── persistenter Stationszustand
  ├── WebUI/API/Prometheus
  └── MQTT retained State + netcore-event-v1
```

Softwarewerte aus dem TBS-DSP liegen **vor** dem Leistungsverstärker. Sie belegen Modulationsqualität, nicht die tatsächlich abgestrahlte HF-Leistung. Vorlauf, Rücklauf, VSWR und reale Antennenfehler benötigen daher eine kalibrierte externe Messquelle.
