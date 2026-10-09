# Projektstand und Nachweisgrenzen

**Z01-Bezugsstand 07.10.2026:** main vor Integration `dae9363062a1442664a083e1fc32e6944b3be9a6`, historische Quelle `bbf039729b9b05f8d623b11195ca24a124f68d16`. Z01.1–Z01.3 sind auf `feature/z01-deployment-consolidation` dokumentiert/implementiert und lokal geprüft; [Nachweise und offene Grenzen](../integration/Z01-2026-10-07/README.md). PR-/CI-Ergebnisse vor Übernahme prüfen. Keine automatisch ermittelte Live-Installation. Die vorherige Wiki-Momentaufnahme vom 26.09. (`d518c97`) wird dadurch im Z01-Umfang aktualisiert.

| Aussage | Beleg im Repository | Was offen bleibt |
|---|---|---|
| Lokale TBS mit SDR, Dashboard, Medien und Integrationen | `bins/bluestation-bs`, `crates/tetra-*`, [sanitisiertes Konfigurationsbeispiel](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/basisstation.config.sanitized.example.toml) | SDR-Timing, Endgeräteverhalten und tatsächliche Version am Standort |
| 26 deklarierte Dienste im konsolidierten Open-Lab-Deployment | [Inventory](https://github.com/JanHG98/netcore-tetra/blob/main/deploy/open-lab/inventory.example.toml), Installer und Service-Verzeichnisse | keine Live-Flottenabnahme; Deployment-Core benötigt eine volle VM, übrige Rollen haben eigene Installationsnachweise |
| Provisioning Core als Zusatzdienst | `system-backend/provisioning-core`, Beispielport 8125 | nicht Teil des regulären 26er-Inventories; Vorlagen können alte Branch- und IP-Beispiele enthalten |
| Directory, Piper, separater Brew-Server | `system-backend/directory`, `system-backend/tts`, `misc/brew-server` | jeweils eigene Einrichtung und Kompatibilitätsprüfung |
| MQTT und Home Assistant | `system-backend/iot-gateway` und [MQTT-Vertrag](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/services/iot-gateway/mqtt-contract.md) | SDS-Ende-zu-Ende bis HA an der konkreten Anlage ist nicht allein durch verbundene MQTT-Clients belegt |
| SIP Switch mit lokalem TBS-Asterisk | [SIP-Architektur](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/services/sip-switch/architecture.md) | Rufsignalisierung und bidirektionales RTP je Standort prüfen |
| ETSI-PDU-Bestand | [statische Konformitätsmatrix](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/protocols/ETSI_CONFORMANCE_MATRIX.md) | kein umfassender Interoperabilitäts- oder Konformitätsnachweis |

## Lesart von „funktioniert“

1. **Im Code:** Implementierung und Konfiguration existieren.
2. **Statisch geprüft:** Tests oder Vertragsprüfungen prüfen Teile der Logik.
3. **Im Labornetz geprüft:** Dienste tauschen tatsächlich Ereignisse und Medien aus; Ergebnis ist dokumentiert.
4. **On Air geprüft:** RF-Signal, Endgerät, Registrierung, Rufe und Release wurden mit dem angegebenen Build und Mess-/Logdaten geprüft.

Eine der ersten beiden Stufen darf im Wiki nicht stillschweigend als vierte ausgegeben werden. Für E2E-Tests gibt es [Open-Lab-Runbook und Runner](https://github.com/JanHG98/netcore-tetra/tree/main/deploy/open-lab); einige ältere Projektdateien sprechen noch von 17 Diensten und sind historische Momentaufnahmen. Das **aktuelle Inventory** ist hier für Dienstzahl und Beispielports maßgeblich.

## Offene technische Punkte

- **Konfigurationsdrift:** Der strenge `[tbs_site]`-Abgleich erhält die eingecheckte TBS-Adresse `10.0.1.179:8080/ws/node`; das Backend-Beispielinventory verwendet `10.0.20.10`. Vor dem Start die installierte `[control_room]`-Adresse mit dem echten Gateway abgleichen; Port und Pfad allein reichen nicht.
- **Dual Carrier:** Der zweite Träger beweist keinen zweiten selbstständigen Kontrollkanal. Slotbelegung und MS-Verhalten messen. [Dual-Carrier](Dual-Carrier.md)
- **Mehrzellenbetrieb:** Teile von Mobility/Restore liegen im Code; vollständiger Handover eines laufenden Rufs im MAIN-COMPAT-Pfad ist nicht als abgenommen dokumentiert. [Mehrzellenbetrieb](Mehrzellenbetrieb.md)
- **IoT:** Reale HA-/Homematic-Schreibaktionen sind in der Open-Lab-Vorlage standardmäßig gesperrt. [MQTT-und-Home-Assistant](MQTT-und-Home-Assistant.md)
- **Sicherheit:** Open-Lab-Management ist ohne die Netzgrenze offen. Vor einer Nutzung außerhalb eines isolierten Labs sind Schutz und Abnahme nötig. [Security-and-Operations](Security-and-Operations.md)

Die [Roadmap](Roadmap.md) trennt weiterführende Ideen von dieser Bestandsaufnahme.
