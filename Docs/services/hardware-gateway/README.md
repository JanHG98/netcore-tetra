# Hardwareüberwachung

**Quellen:** [system-backend/hardware-gateway](../../../system-backend/hardware-gateway) · [Repository-Root](../../..). Bei Befehlen das in der Anleitung angegebene Arbeitsverzeichnis beachten.

Stand: **9. Oktober 2026**. Die Anleitung beschreibt die im Hauptzweig vorhandene Umsetzung; Anlagen- und Funkabnahmen stehen in der [Gesamtroadmap](../../roadmaps/gesamtroadmap.md).

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

## Installation und Betriebsdaten

Vom Repository-Hauptverzeichnis als root:

```bash
sudo bash system-backend/hardware-gateway/install/install.sh
```

Konfiguration: `/etc/netcore/hardware-gateway.toml`. Zustand und Ereignisse: `/var/lib/netcore-hardware-gateway/state.json` und `events.ndjson`. Brokeradresse, Gerätekennung und Grenzwerte an die tatsächlichen Sensoren anpassen. Die Voreinstellung `outputs_enabled = false` kennzeichnet den reinen Überwachungsbetrieb; ein Konfigurationsschalter allein stellt keinen geprüften Hardwaretreiber bereit.

Weitere Anleitungen: [Architektur](architektur.md) und [HTTP-Telemetrieprüfung](tests/funktionspruefung-im-labor.md). Der historische Entwicklungsumfang heißt Phase 6.
