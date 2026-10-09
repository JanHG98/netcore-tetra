# Integrationen und Zuständigkeitsgrenzen

Eine Integration ist jeweils ein eigener Daten- oder Medienweg. Sie sollte erst freigeschaltet werden, wenn Sender, Empfänger, Authentisierung, Fehlerfall, doppelte Zustellung und Rückweg feststehen.

| Integration | Komponente und Weg | Prüfpunkte |
|---|---|---|
| Telefonie | native TBS-Bridge → lokaler Asterisk → zentraler SIP Switch → PBX | Wählplan, Mobility, PJSIP, RTP, Fallback; [SIP-und-Brew](SIP-und-Brew.md) |
| Brew | TBS `[brew]` → externer Peer/separater Server | GSSI/ISSI-Grenzen, Ruf- und SDS-Rückkopplung; [SIP-und-Brew](SIP-und-Brew.md) |
| MQTT/HA | IoT Gateway ↔ MQTT Broker ↔ Home Assistant | Topics, Discovery, Acks, Policy, HA-Automation; [MQTT-und-Home-Assistant](MQTT-und-Home-Assistant.md) |
| Homematic | IoT Gateway ↔ HA-Brücke oder CCU XML-RPC | State-Ingress, aktiver Datenpunkt, Write-Policy; [MQTT-und-Home-Assistant](MQTT-und-Home-Assistant.md) |
| TTS/Audio | Piper/Media Library ↔ TBS-Cache/Playout | WAV/Codec, NFS, Freigabe, Release; [Audio-Zentrale](Audio-Zentrale.md) |
| Meldungen/Workflows | Application Gateway, Alarm/Task Workflow | Empfänger, Eskalation, Audit und Fehlerqueue; [Dienstkatalog](Dienstkatalog.md) |
| Karten/Position | LIP-Daten → TBS/Directory/Dashboard | ISSI-Zuordnung, Zeitstempel, Sichtbarkeit; [LIP-and-GPS](LIP-and-GPS.md) |

Technische Endpunkte und Beispielports: [Netzwerk-und-Ports](Netzwerk-und-Ports.md). Ein HTTP-Health-Check bestätigt nur den Dienstzustand, nicht die Verarbeitung einer konkreten SDS, Sprache oder HA-Aktion. [Abnahme](Abnahme.md)
