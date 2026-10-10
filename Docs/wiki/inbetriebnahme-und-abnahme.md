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

Für mutierende oder neustartende E2E-Profile die [aktuelle Deployer-README](../deployment/open-lab/README.md) lesen und eine isolierte Testumgebung nutzen. Ein Mock-TBS-E2E-Test ersetzt den RF-Nachweis nicht. Die PDU-[Bestandsmatrix](../protocols/etsi-pdu-konformitaetsmatrix.md) benennt offene Tests und Platzhalter. [ETSI-Quellen, Tests und Konformität](etsi-normen-und-tests.md)

## Fehlerdokumentation

Für jeden Fehlschlag **ersten Fehlerzeitpunkt**, Host, Dienst, ISSI/GSSI, Carrier/Slot, Konfigurations-Hash und letzte erfolgreiche Stufe notieren. SIP-Dialog, RTP und Funk-Audio getrennt protokollieren. Nach einer Korrektur den kompletten Ablauf bis zur Freigabe erneut prüfen. [Fehlersuche](fehlersuche.md)

## Bereitstellung und Image-Recovery

Für Deployment-Aufträge zusätzlich vorhandene Konfiguration, semantisch negative Readiness, Wiederholungsupdate und Rückweg prüfen. Für Pi-Images Imagejob, Artefakt-ID, Download, Manifestcommit und SHA-256 korrelieren; danach physisch booten, SDR erkennen und bei eingerichtetem OpenVPN zwischen Heimnetz und anderem Netz wechseln. `boot_tested=false` bleibt korrekt, solange der tatsächliche Hardwareboot nicht nachgewiesen ist. [Z01-Abnahme](../integration/Z01-2026-10-07/anlagenabnahme-z01-4.md) · [Imagebuilder-Befund](../integration/Z01-2026-10-07/imagebuilder-vm119.md)

## Quellen zur Pflege dieser Seite

[E2E-Infrastruktur](../../tests/e2e/README.md) · [Z01-Betreibernachweise](../integration/Z01-2026-10-07/anlagenabnahme-z01-4.md).
