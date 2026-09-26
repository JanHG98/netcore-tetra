# NetCore Tetra · Systemwiki

**Softwarebasierte TETRA-Basisstation, verteilte Netzdienste und Leitstellenwerkzeuge für ein kontrolliertes Open Lab.** Dieses Wiki führt vom ersten Start einer TBS über den Betrieb der zentralen Dienste bis zur Fehlersuche. Die Seiten beschreiben den [Repository-Stand `main` vom 26.09.2026 (`d518c97`)](https://github.com/JanHG98/netcore-tetra/tree/d518c9733b6d0474021792d4883206dc42035c2d). Die tatsächlich installierte Version und Konfiguration einer Anlage können davon abweichen.

> **Betriebsgrenze:** Die Beispiel-Backends im Modus `open_lab` besitzen nach dem aktuellen Inventory und den Dienstvorlagen vielfach **keine Anmeldung, Management-Tokens oder TLS**. Sie gehören ausschließlich in ein isoliertes, kontrolliertes Managementnetz. Der Quellcode und ein erfolgreicher Komponententest sind kein Nachweis eines vollständig abgenommenen Funknetzes. [[Projektstand und Grenzen|Projektstand]]

## Hier anfangen

| Ziel | Einstieg | Danach |
|---|---|---|
| Eine TBS aufbauen | [[Installation]] | [Konfiguration](Configuration) · [Hardware und HF](Hardware-und-RF) · [Abnahme](Abnahme) |
| 24 Backend-Dienste verstehen | [Architektur](Architecture) | [Dienstkatalog](Dienstkatalog) · [Netzwerk und Ports](Netzwerk-und-Ports) |
| LXCs bereitstellen | [[Open-Lab-Deployment]] | [Provisioning](Provisioning) · [Sicherheit und Betrieb](Security-and-Operations) |
| Funk- und Datenwege prüfen | [Rufe](Calls) | [SDS und Status](SDS-and-U-STATUS) · [Paketdaten und WAP](Paketdaten-und-WAP) |
| Leitstelle und Integrationen anbinden | [Control Room](Control-Room) | [SIP und Brew](SIP-und-Brew) · [MQTT und Home Assistant](MQTT-und-Home-Assistant) |
| Fehler eingrenzen | [Fehlersuche](Troubleshooting) | [Betrieb und Wartung](Betrieb-und-Wartung) · [Backup und Fallback](Backup-and-Fallback) |

## System in vier Ebenen

1. **Funkkante:** `bluestation-bs` betreibt SDR, Luftschnittstelle, lokale Registrierung, Rufe, SDS, Dashboard und Edge-Fallback. [[Basisstation und Träger|Dual-Carrier]]
2. **Netzkern:** Node Gateway verbindet TBS und zentrale Fachkerne. Subscriber, Group, Mobility, Call Control, Media Switch, SDS Router, Packet Core und weitere Dienste teilen sich die Aufgaben. [[Dienstkatalog|Dienstkatalog]]
3. **Anwendungen:** SIP Switch, IoT Gateway, Media Library, Workflows und andere Adapter binden Telefonie, MQTT, Medien und betriebliche Prozesse an. [[Integrationen|Integrationen]]
4. **Bedienung:** lokales TBS-Dashboard, Directory, Provisioning Core, Control Room und Observability bedienen unterschiedliche Zuständigkeiten. [[Bedienoberflächen|Bedienoberflaechen]]

## Was diese Doku tatsächlich belegt

- `Cargo.toml` nennt für den Rust-Workspace `1.3.0`; das ist **keine Aussage über den Tag oder die Version deiner laufenden TBS**.
- `deploy/open-lab/inventory.example.toml` enthält **24** Backend-Einträge. Provisioning Core liegt zusätzlich außerhalb dieses Inventories. Die lokale TBS, das Python-Directory, Piper und ein separater Brew-Server haben eigene Betriebswege.
- Quellcode, Installer, Konfigurationsbeispiele und Tests liegen im Repository. Ohne Zugriff auf Live-LXC, SDR, Funkgeräte und Messdaten können wir den aktuellen Betriebszustand nicht pauschal bestätigen.
- ETSI-PDFs dienen als Normreferenz; die eigene PDU-Matrix ist eine **statische Bestandsaufnahme**, keine Zertifizierung. [[Normen und Tests|Normen-und-Tests]]
- Imagebuilder, durchgängige automatische Dienstsuche und ein vollständiger Zero-Touch-Wizard sind [[Ausbauziele|Roadmap]] und werden hier nicht als fertig dargestellt.

## Lesepfade

- **Projekt und Architektur:** [[Projektstand]] · [[Architecture]] · [[Dienstkatalog]] · [[Netzwerk-und-Ports]] · [[Glossar]]
- **Aufbau und Konfiguration:** [[Hardware-und-RF]] · [[Installation]] · [[Configuration]] · [[Open-Lab-Deployment]] · [[Abnahme]]
- **Betrieb:** [[Dashboard]] · [[Control-Room]] · [[Betrieb-und-Wartung]] · [[Troubleshooting]] · [[Backup-and-Fallback]]
- **Protokolle und Schnittstellen:** [[Calls]] · [[SDS-and-U-STATUS]] · [[Paketdaten-und-WAP]] · [[SIP-und-Brew]] · [[MQTT-und-Home-Assistant]]

Die [ausführliche Systemhandbuch-Ausgabe im Repository](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/NetCore-Tetra-Systemhandbuch.md) ist ein tieferes Nachschlagewerk. Bei Widersprüchen sind **installierte Konfiguration und beobachtete Laufzeit** für die konkrete Anlage zu prüfen; für diese Wiki-Ausgabe wurden der oben angegebene Commit und die dortigen Vorlagen herangezogen.
