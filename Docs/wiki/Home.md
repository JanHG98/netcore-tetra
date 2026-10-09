# NetCore Tetra · Systemwiki

**Softwarebasierte TETRA-Basisstation, verteilte Netzdienste und Leitstellenwerkzeuge für ein kontrolliertes Open Lab.** Dieses Wiki führt vom ersten Start einer TBS über den Betrieb der zentralen Dienste bis zur Fehlersuche. Die Seiten beschreiben den [Repository-Stand LINK0 vom 26.09.2026 (LINK1)](https://github.com/JanHG98/netcore-tetra/tree/d518c9733b6d0474021792d4883206dc42035c2d). Die tatsächlich installierte Version und Konfiguration einer Anlage können davon abweichen.

> **Betriebsgrenze:** Die Beispiel-Backends im Modus `open_lab` besitzen nach dem aktuellen Inventory und den Dienstvorlagen vielfach **keine Anmeldung, Management-Tokens oder TLS**. Sie gehören ausschließlich in ein isoliertes, kontrolliertes Managementnetz. Der Quellcode und ein erfolgreicher Komponententest sind kein Nachweis eines vollständig abgenommenen Funknetzes. [Projektstand und Grenzen](Projektstand.md)

## Hier anfangen

| Ziel | Einstieg | Danach |
|---|---|---|
| Eine TBS aufbauen | [Installation](Installation.md) | [Konfiguration](Configuration.md) · [Hardware und HF](Hardware-und-RF.md) · [Abnahme](Abnahme.md) |
| 24 Backend-Dienste verstehen | [Architektur](Architecture.md) | [Dienstkatalog](Dienstkatalog.md) · [Netzwerk und Ports](Netzwerk-und-Ports.md) |
| LXCs bereitstellen | [Open-Lab-Deployment](Open-Lab-Deployment.md) | [Provisioning](Provisioning.md) · [Sicherheit und Betrieb](Security-and-Operations.md) |
| Funk- und Datenwege prüfen | [Rufe](Calls.md) | [SDS und Status](SDS-and-U-STATUS.md) · [Paketdaten und WAP](Paketdaten-und-WAP.md) |
| Leitstelle und Integrationen anbinden | [Control Room](Control-Room.md) | [SIP und Brew](SIP-und-Brew.md) · [MQTT und Home Assistant](MQTT-und-Home-Assistant.md) |
| Fehler eingrenzen | [Fehlersuche](Troubleshooting.md) | [Betrieb und Wartung](Betrieb-und-Wartung.md) · [Backup und Fallback](Backup-and-Fallback.md) |

## System in vier Ebenen

1. **Funkkante:** `bluestation-bs` betreibt SDR, Luftschnittstelle, lokale Registrierung, Rufe, SDS, Dashboard und Edge-Fallback. [Basisstation und Träger](Dual-Carrier.md)
2. **Netzkern:** Node Gateway verbindet TBS und zentrale Fachkerne. Subscriber, Group, Mobility, Call Control, Media Switch, SDS Router, Packet Core und weitere Dienste teilen sich die Aufgaben. [Dienstkatalog](Dienstkatalog.md)
3. **Anwendungen:** SIP Switch, IoT Gateway, Media Library, Workflows und andere Adapter binden Telefonie, MQTT, Medien und betriebliche Prozesse an. [Integrationen](Integrationen.md)
4. **Bedienung:** lokales TBS-Dashboard, Directory, Provisioning Core, Control Room und Observability bedienen unterschiedliche Zuständigkeiten. [Bedienoberflächen](Bedienoberflaechen.md)

## Was diese Doku tatsächlich belegt

- `Cargo.toml` nennt für den Rust-Workspace `1.3.0`; das ist **keine Aussage über den Tag oder die Version deiner laufenden TBS**.
- `deploy/open-lab/inventory.example.toml` enthält **24** Backend-Einträge. Provisioning Core liegt zusätzlich außerhalb dieses Inventories. Die lokale TBS, das Python-Directory, Piper und ein separater Brew-Server haben eigene Betriebswege.
- Quellcode, Installer, Konfigurationsbeispiele und Tests liegen im Repository. Ohne Zugriff auf Live-LXC, SDR, Funkgeräte und Messdaten können wir den aktuellen Betriebszustand nicht pauschal bestätigen.
- ETSI-PDFs dienen als Normreferenz; die eigene PDU-Matrix ist eine **statische Bestandsaufnahme**, keine Zertifizierung. [Normen und Tests](Normen-und-Tests.md)
- Imagebuilder, durchgängige automatische Dienstsuche und ein vollständiger Zero-Touch-Wizard sind [Ausbauziele](Roadmap.md) und werden hier nicht als fertig dargestellt.

## Lesepfade

- **Projekt und Architektur:** [Projektstand](Projektstand.md) · [Architecture](Architecture.md) · [Dienstkatalog](Dienstkatalog.md) · [Netzwerk-und-Ports](Netzwerk-und-Ports.md) · [Glossar](Glossar.md)
- **Aufbau und Konfiguration:** [Hardware-und-RF](Hardware-und-RF.md) · [Installation](Installation.md) · [Configuration](Configuration.md) · [Open-Lab-Deployment](Open-Lab-Deployment.md) · [Abnahme](Abnahme.md)
- **Betrieb:** [Dashboard](Dashboard.md) · [Control-Room](Control-Room.md) · [Betrieb-und-Wartung](Betrieb-und-Wartung.md) · [Troubleshooting](Troubleshooting.md) · [Backup-and-Fallback](Backup-and-Fallback.md)
- **Protokolle und Schnittstellen:** [Calls](Calls.md) · [SDS-and-U-STATUS](SDS-and-U-STATUS.md) · [Paketdaten-und-WAP](Paketdaten-und-WAP.md) · [SIP-und-Brew](SIP-und-Brew.md) · [MQTT-und-Home-Assistant](MQTT-und-Home-Assistant.md)

Die [ausführliche Systemhandbuch-Ausgabe im Repository](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/handbooks/NetCore-Tetra-Systemhandbuch.md) ist ein tieferes Nachschlagewerk. Bei Widersprüchen sind **installierte Konfiguration und beobachtete Laufzeit** für die konkrete Anlage zu prüfen; für diese Wiki-Ausgabe wurden der oben angegebene Commit und die dortigen Vorlagen herangezogen.
