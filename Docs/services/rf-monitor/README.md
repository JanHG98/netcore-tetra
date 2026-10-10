# HF-Überwachung

**Quellen:** [system-backend/rf-monitor](../../../system-backend/rf-monitor) · [Repository-Root](../../..). Bei Befehlen das in der Anleitung angegebene Arbeitsverzeichnis beachten.

Stand: **9. Oktober 2026**. Die Anleitung beschreibt die im Hauptzweig vorhandene Umsetzung; Anlagen- und Funkabnahmen stehen in der [Gesamtroadmap](../../roadmaps/gesamtroadmap.md).

OPEN-LAB-Dienst zur zentralen HF- und Senderzustandsüberwachung mehrerer TBS.

## Datenquellen

1. **TBS-Softwaretelemetrie** über den mitgelieferten `netcore-rf-agent`:
   - TX aktiv / aktive Calls
   - Mittenfrequenz und Abtastrate
   - RMS und Peak vor dem Leistungsverstärker
   - EVM, PAPR, DC-/IQ-Fehler, Trägerrest und belegte Bandbreite
   - SDR-Temperatur und Ist-Gainstufen
   - optional 512 Spektrumsbins
2. **Externe kalibrierte RF-Probe** über ein frei konfigurierbares JSON-Kommando:
   - Vorlauf- und Rücklaufleistung
   - PA-Spannung, Strom und Temperatur
   - Lüfterdrehzahl
   - Antennen-, PA-, SWR-, Lüfter- und PLL-Kontakte

Der Dienst berechnet aus Vorlauf- und Rücklaufleistung automatisch Reflektionsanteil, VSWR und Return Loss.

## MQTT

- Ingress: `netcore/v1/rf/<station-id>/telemetry`
- Retained State: `netcore/v1/state/rf/<station-id>`
- Events: `netcore/v1/events/rf/...`

## API

- `GET /api/v1/status`
- `GET /api/v1/stations`
- `GET /api/v1/stations/<station-id>`
- `GET /api/v1/alarms`
- `GET /api/v1/events`
- `POST /api/v1/telemetry`
- `GET /metrics`
- `GET /health/live`
- `GET /health/ready`

WebUI und API laufen standardmäßig auf Port `8260`.

## Sicherheitsgrenze

Phase 7 ist reine Überwachung. Der Dienst kann weder den Sender schalten noch Gain, Frequenz, PA oder Antennenpfade verändern. OPEN LAB bedeutet außerdem: keine Anmeldung, keine Tokens und kein TLS.

## Installation und Messwerte prüfen

Vom Repository-Hauptverzeichnis als root:

```bash
sudo bash system-backend/rf-monitor/install/install.sh
```

Konfiguration: `/etc/netcore/rf-monitor.toml`. Zustand und Ereignisse liegen unter `/var/lib/netcore-rf-monitor/`. Für jede tatsächliche TBS den [mitgelieferten Agenten](../../../system-backend/rf-monitor/examples/tbs-agent) konfigurieren und nach `install/install-tbs-agent.sh` einrichten. Beispielprobe und simulierte HTTP-Telemetrie sind keine Kalibrierung einer HF-Messkette.

Heartbeat-Timeout ist in der Vorlage 20 Sekunden. Mindestleistung, PA-Spannung und Lüfterdrehzahl bleiben mit Grenzwert 0 deaktiviert, bis passende Messwerte und Grenzwerte vorliegen. Software-Spektrumswerte werden auf höchstens 512 Bins begrenzt.

Weitere Anleitungen: [Architektur](architektur.md) und [Telemetrie-Funktionsprüfung](tests/funktionspruefung-im-labor.md).
