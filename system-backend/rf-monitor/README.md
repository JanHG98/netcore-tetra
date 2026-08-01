# NetCore RF Monitor – Phase 7

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
