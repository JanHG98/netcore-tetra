# Integrationen und Zuständigkeitsgrenzen

Eine Integration ist jeweils ein eigener Daten- oder Medienweg. Sie sollte erst freigeschaltet werden, wenn Sender, Empfänger, Authentisierung, Fehlerfall, doppelte Zustellung und Rückweg feststehen.

| Integration | Komponente und Weg | Prüfpunkte |
|---|---|---|
| Telefonie | native TBS-Bridge → lokaler Asterisk → zentraler SIP Switch → PBX | Wählplan, Mobility, PJSIP, RTP, Fallback; [SIP, lokaler Asterisk und Brew](telefonie-sip-und-brew.md) |
| Brew | TBS `[brew]` → externer Peer/separater Server | GSSI/ISSI-Grenzen, Ruf- und SDS-Rückkopplung; [SIP, lokaler Asterisk und Brew](telefonie-sip-und-brew.md) |
| MQTT/HA | IoT Gateway ↔ MQTT Broker ↔ Home Assistant | Topics, Discovery, Acks, Policy, HA-Automation; [MQTT, Home Assistant und Homematic](mqtt-home-assistant-und-homematic.md) |
| Homematic | IoT Gateway ↔ HA-Brücke oder CCU XML-RPC | State-Ingress, aktiver Datenpunkt, Write-Policy; [MQTT, Home Assistant und Homematic](mqtt-home-assistant-und-homematic.md) |
| TTS/Audio | Piper/Media Library ↔ TBS-Cache/Playout | WAV/Codec, NFS, Freigabe, Release; [Audio-Zentrale](audio-aufnahmen-und-tts.md) |
| Meldungen/Workflows | Application Gateway, Alarm/Task Workflow | Empfänger, Eskalation, Audit und Fehlerqueue; [Dienstkatalog](dienstkatalog.md) |
| Karten/Position | LIP-Daten → TBS/Directory/Dashboard | ISSI-Zuordnung, Zeitstempel, Sichtbarkeit; [Positionen über LIP und GPS anzeigen](positionen-lip-und-gps.md) |

Technische Endpunkte und Beispielports: [Netzwerk, Ports und Protokolle](netzwerk-und-ports.md). Ein HTTP-Health-Check bestätigt nur den Dienstzustand, nicht die Verarbeitung einer konkreten SDS, Sprache oder HA-Aktion. [Inbetriebnahme und Abnahme](inbetriebnahme-und-abnahme.md)

Discovery und Deployment sind Betriebsintegrationen: sie verteilen oder ermitteln Endpunkte, bestätigen aber keine SDS-, Sprach-, Packet-Data- oder Aktorwirkung. Für neue Integrationen die vorhandenen Fachkerne und ihre bestätigten Rückmeldungen verwenden. [Deployment Core](../services/deployment-core/README.md)

## Quellen zur Pflege dieser Seite

[Integrationskonfiguration der TBS](../basisstation.config.sanitized.example.toml) · [Deployment-Endpunkte](../../system-backend/deployment-core/main.py).
