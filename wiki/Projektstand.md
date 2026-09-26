# Projektstand und Nachweisgrenzen

**Bezugsstand:** Repository [`JanHG98/netcore-tetra`, Branch `main`, Commit `d518c97`](https://github.com/JanHG98/netcore-tetra/tree/d518c9733b6d0474021792d4883206dc42035c2d), abgerufen am 26.09.2026. Das Wiki dokumentiert diesen Quellstand, keine automatisch ermittelte Live-Installation.

| Aussage | Beleg im Repository | Was offen bleibt |
|---|---|---|
| Lokale TBS mit SDR, Dashboard, Medien und Integrationen | `bins/bluestation-bs`, `crates/tetra-*`, [sanitisiertes Konfigurationsbeispiel](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/basisstation.config.sanitized.example.toml) | SDR-Timing, Endgeräteverhalten und tatsächliche Version am Standort |
| 24 Dienste im Open-Lab-Deployment | [Inventory](https://github.com/JanHG98/netcore-tetra/blob/main/deploy/open-lab/inventory.example.toml), Installer und Service-Verzeichnisse | keine Bestätigung, dass alle 24 LXCs aktuell laufen oder integriert abgenommen sind |
| Provisioning Core als Zusatzdienst | `system-backend/provisioning-core`, Beispielport 8125 | nicht Teil des 24er-Inventories; Vorlagen können alte Branch- und IP-Beispiele enthalten |
| Directory, Piper, separater Brew-Server | `system-backend/directory`, `system-backend/tts`, `misc/brew-server` | jeweils eigene Einrichtung und Kompatibilitätsprüfung |
| MQTT und Home Assistant | `system-backend/iot-gateway` und [MQTT-Vertrag](https://github.com/JanHG98/netcore-tetra/blob/main/system-backend/iot-gateway/docs/mqtt-contract.md) | SDS-Ende-zu-Ende bis HA an der konkreten Anlage ist nicht allein durch verbundene MQTT-Clients belegt |
| SIP Switch mit lokalem TBS-Asterisk | [SIP-Architektur](https://github.com/JanHG98/netcore-tetra/blob/main/system-backend/sip-switch/docs/architecture.md) | Rufsignalisierung und bidirektionales RTP je Standort prüfen |
| ETSI-PDU-Bestand | [statische Konformitätsmatrix](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/ETSI_CONFORMANCE_MATRIX.md) | kein umfassender Interoperabilitäts- oder Konformitätsnachweis |

## Lesart von „funktioniert“

1. **Im Code:** Implementierung und Konfiguration existieren.
2. **Statisch geprüft:** Tests oder Vertragsprüfungen prüfen Teile der Logik.
3. **Im Labornetz geprüft:** Dienste tauschen tatsächlich Ereignisse und Medien aus; Ergebnis ist dokumentiert.
4. **On Air geprüft:** RF-Signal, Endgerät, Registrierung, Rufe und Release wurden mit dem angegebenen Build und Mess-/Logdaten geprüft.

Eine der ersten beiden Stufen darf im Wiki nicht stillschweigend als vierte ausgegeben werden. Für E2E-Tests gibt es [Open-Lab-Runbook und Runner](https://github.com/JanHG98/netcore-tetra/tree/main/deploy/open-lab); einige ältere Projektdateien sprechen noch von 17 Diensten und sind historische Momentaufnahmen. Das **aktuelle Inventory** ist hier für Dienstzahl und Beispielports maßgeblich.

## Offene technische Punkte

- **Konfigurationsdrift:** Die eingecheckte TBS-Konfiguration und das Beispiel-Inventory können unterschiedliche Node-Gateway-Adressen enthalten. Vor dem Start die installierte `[control_room]`-Adresse mit dem echten Gateway abgleichen; Port und Pfad allein reichen nicht.
- **Dual Carrier:** Der zweite Träger beweist keinen zweiten selbstständigen Kontrollkanal. Slotbelegung und MS-Verhalten messen. [[Dual-Carrier]]
- **Mehrzellenbetrieb:** Teile von Mobility/Restore liegen im Code; vollständiger Handover eines laufenden Rufs im MAIN-COMPAT-Pfad ist nicht als abgenommen dokumentiert. [[Mehrzellenbetrieb]]
- **IoT:** Reale HA-/Homematic-Schreibaktionen sind in der Open-Lab-Vorlage standardmäßig gesperrt. [[MQTT-und-Home-Assistant]]
- **Sicherheit:** Open-Lab-Management ist ohne die Netzgrenze offen. Vor einer Nutzung außerhalb eines isolierten Labs sind Schutz und Abnahme nötig. [[Security-and-Operations]]

Die [[Roadmap]] trennt weiterführende Ideen von dieser Bestandsaufnahme.
