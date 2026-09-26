# NetCore Hardware Gateway – Phase 6

OPEN-LAB-Dienst für Hardware-I/O, Rack- und Umgebungsüberwachung.

## Funktionen
- MQTT-Telemetrie von Edge-Nodes
- HTTP-Telemetrie-Ingress
- Geräte- und Heartbeat-Registry
- Thresholds für Temperatur, Feuchte und Versorgungsspannung
- `netcore-event-v1` Alarmereignisse
- retained MQTT-Zustände
- WebUI/API auf Port 8250
- persistenter Zustand und Eventlog
- Hardware-Ausgänge standardmäßig vollständig deaktiviert

## MQTT
Edge-Nodes senden an `netcore/v1/hardware/<device-id>/telemetry`.
Normalisierte Zustände erscheinen unter `netcore/v1/state/hardware/<device-id>`.

## API
- `GET /api/v1/status`
- `GET /api/v1/devices`
- `GET /api/v1/events`
- `POST /api/v1/telemetry`
- `GET /health/live`
- `GET /health/ready`
