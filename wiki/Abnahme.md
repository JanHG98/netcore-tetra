# Inbetriebnahme und Abnahme

Eine installierte Binärdatei und ein erreichbares WebUI sind ein guter Anfang. Für einen belastbaren Funkbetrieb müssen **Funk, Steuerung, Medien, Daten und Ausfallverhalten** getrennt nachgewiesen werden. Die Tabelle ist ein Arbeitsblatt; Ergebnisse mit Datum, Commit/Tag, TOML-Revision, Gerätetyp/Firmware und Mess-/Logverweisen festhalten.

| Bereich | Prüfungen | Akzeptanzbeleg |
|---|---|---|
| Host/SDR | Versorgung, Clock, Treiber/Channels, Sample-Rate, Temperatur, Dauertest | `SoapySDRUtil`, Journal, Messwerte |
| HF | Ausgangsleistung, Spektrum, Duplexer-Isolation, RX bei TX, Last/Antenne | HF-Messprotokoll und zulässige Konfiguration |
| Zelle | SYNC/SYSINFO, MCC/MNC, LA/CC, Carrier, Endgeräte-Camping | Air-Log und getestete Codeplug-Parameter |
| Mobilität | Registrierung, Periodik, Deregistration, erneuter Join, Affiliation | Ereignisfolge und keine ungewollte Gruppenauflösung |
| Ruf | Gruppe, Einzelruf, Floor-Wechsel, Hangtime-Retake, Abbruch, Release | hörbare Sprache in beide Richtungen und freie Slots danach |
| Daten | SDS Text/Status, Gruppenstatus, LIP, Packet Data/WAP nach Aktivierung | Quell-, Vermittlungs- und Ziel-Logs derselben Test-ID |
| Backend | Node Gateway, Health-Matrix, Policy/Serving-TBS, E2E-Routing | aufgelöste Abhängigkeiten, Readiness und fachliche Ereignisse |
| Telefonie/Integration | TETRA↔PBX, zentraler Ausfall, MQTT→HA, Brew falls aktiv | SIP/RTP in beide Richtungen, Topic-/Ack-Trace und Peer-Logs |
| Ausfall | Gateway, Directory, NFS, Piper, Media Switch gezielt ausfallen lassen | dokumentierter Fallback, Wiederkehr ohne doppelte Aktionen |

## Softwareprüfungen

```bash
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml validate
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml test --profile full --validate-only
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml test --profile smoke
```

Für mutierende oder neustartende E2E-Profile die [aktuelle Deployer-README](https://github.com/JanHG98/netcore-tetra/blob/main/deploy/open-lab/README.md) lesen und eine isolierte Testumgebung nutzen. Ein Mock-TBS-E2E-Test ersetzt den RF-Nachweis nicht. Die PDU-[Bestandsmatrix](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/ETSI_CONFORMANCE_MATRIX.md) benennt offene Tests und Platzhalter. [[Normen-und-Tests]]

## Fehlerdokumentation

Für jeden Fehlschlag **ersten Fehlerzeitpunkt**, Host, Dienst, ISSI/GSSI, Carrier/Slot, Konfigurations-Hash und letzte erfolgreiche Stufe notieren. SIP-Dialog, RTP und Funk-Audio getrennt protokollieren. Nach einer Korrektur den kompletten Ablauf bis zur Freigabe erneut prüfen. [[Troubleshooting]]
