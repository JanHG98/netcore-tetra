# Projektstand und Nachweisgrenzen

**Quellenstand: `main@c3ccdb4`, 09.10.2026.** Z01.1–Z01.3 sind gemäß [Quell- und CI-Nachweis](../integration/Z01-2026-10-07/README.md) auf `main` integriert. Z01.4 führt die tatsächliche Installation, Recovery und Image-/Hardwareabnahme fort. Den neuesten Betreiberbefund und nächsten Schritt führt die [zentrale Roadmap](../roadmaps/gesamtroadmap.md); diese Wiki-Seite ist keine Live-Abfrage der Anlage.

| Aussage | Beleg im Repository | Was offen bleibt |
|---|---|---|
| Lokale TBS mit SDR, Dashboard, Medien und Integrationen | `bins/bluestation-bs`, `crates/tetra-*`, [sanitisiertes Konfigurationsbeispiel](../basisstation.config.sanitized.example.toml) | SDR-Timing, Endgeräteverhalten und tatsächliche Version am Standort |
| 26 deklarierte Dienste im konsolidierten Open-Lab-Deployment | [Inventory](../../deploy/open-lab/inventory.example.toml), Installer und Service-Verzeichnisse | keine Live-Flottenabnahme; Deployment-Core benötigt eine volle VM, übrige Rollen haben eigene Installationsnachweise |
| Provisioning Core als Zusatzdienst | `system-backend/provisioning-core`, Beispielport 8125 | nicht Teil des regulären 26er-Inventories; installierte Upstream-Adressen und Dienstkonfiguration prüfen |
| Directory, Piper, separater Brew-Server | `system-backend/directory`, `system-backend/tts`, `misc/brew-server` | jeweils eigene Einrichtung und Kompatibilitätsprüfung |
| MQTT und Home Assistant | `system-backend/iot-gateway` und [MQTT-Vertrag](../services/iot-gateway/mqtt-nachrichten.md) | SDS-Ende-zu-Ende bis HA an der konkreten Anlage ist nicht allein durch verbundene MQTT-Clients belegt |
| SIP Switch mit lokalem TBS-Asterisk | [SIP-Architektur](../services/sip-switch/architektur.md) | Rufsignalisierung und bidirektionales RTP je Standort prüfen |
| ETSI-PDU-Bestand | [statische Konformitätsmatrix](../protocols/etsi-pdu-konformitaetsmatrix.md) | kein umfassender Interoperabilitäts- oder Konformitätsnachweis |

## Lesart von „funktioniert“

1. **Im Code:** Implementierung und Konfiguration existieren.
2. **Statisch geprüft:** Tests oder Vertragsprüfungen prüfen Teile der Logik.
3. **Im Labornetz geprüft:** Dienste tauschen tatsächlich Ereignisse und Medien aus; Ergebnis ist dokumentiert.
4. **On Air geprüft:** RF-Signal, Endgerät, Registrierung, Rufe und Release wurden mit dem angegebenen Build und Mess-/Logdaten geprüft.

Eine der ersten beiden Stufen darf im Wiki nicht stillschweigend als vierte ausgegeben werden. Für E2E-Tests gibt es [Open-Lab-Deployer](../../deploy/open-lab/netcore-deploy.py) und [E2E-Anleitung](../testing/e2e/README.md). Das **aktuelle Inventory** ist für Dienstzahl und Beispielports maßgeblich; `shared` in der Dienstregistry ist eine gemeinsam genutzte Bibliothek, keine 27. deploybare Rolle.

## Offene technische Punkte

- **Konfigurationsdrift:** Der strenge `[tbs_site]`-Abgleich erhält die eingecheckte TBS-Adresse `10.0.1.179:8080/ws/node`; das Backend-Beispielinventory verwendet `10.0.20.10`. Vor dem Start die installierte `[control_room]`-Adresse mit dem echten Gateway abgleichen; Port und Pfad allein reichen nicht.
- **Dual Carrier:** Der zweite Träger beweist keinen zweiten selbstständigen Kontrollkanal. Slotbelegung und MS-Verhalten messen. [Zwei Funkträger mit Dual Carrier betreiben](zwei-funktraeger.md)
- **Mehrzellenbetrieb:** Teile von Mobility/Restore liegen im Code; vollständiger Handover eines laufenden Rufs im MAIN-COMPAT-Pfad ist nicht als abgenommen dokumentiert. [Mehrzellenbetrieb, Mobility und Edge-Fallback](mehrzellenbetrieb-und-ausfallverhalten.md)
- **IoT:** Reale HA-/Homematic-Schreibaktionen sind in der Open-Lab-Vorlage standardmäßig gesperrt. [MQTT, Home Assistant und Homematic](mqtt-home-assistant-und-homematic.md)
- **Sicherheit:** Open-Lab-Management ist ohne die Netzgrenze offen. Vor einer Nutzung außerhalb eines isolierten Labs sind Schutz und Abnahme nötig. [Sicherheit und Betrieb](sicherheit-im-betrieb.md)

## Bereitstellung und Imagebuilder

Deployment Core, Discovery-Agent, zentraler Imagebuilder, Syslog-Anbindung und die optionale VPN-Heimnetzregel sind integriert. Ein vollständiger Imagebuild ist durch einen datierten Betreiberbefund bestätigt; Artefaktverfügbarkeit, Download und physischer Pi-/SXceiver-/VPN-Nachweis sind eigenständige Prüfungen. Der [Imagebuilder-Befund](../integration/Z01-2026-10-07/imagebuilder-vm119.md) beschreibt diese Grenzen. Die [Ausbauziele und offene Entscheidungen](ausbauziele-und-prioritaeten.md) ordnet die noch offenen Erweiterungen ein.

## Quellen zur Pflege dieser Seite

[Inventar der Backend-Rollen](../../deploy/open-lab/inventory.example.toml) · [Quell- und CI-Abnahme](../integration/Z01-2026-10-07/README.md).
