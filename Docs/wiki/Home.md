# NetCore-Tetra: Einstieg ins Systemwiki

Dieses Wiki erklärt die Basisstation, die zentralen Netzdienste und den Betrieb im Open Lab. Es ist die kurze, thematische Orientierung innerhalb von [Docs](../README.md); ausführliche Komponentenhandbücher und datierte Arbeitsnachweise sind verlinkt.

**Quellenstand dieser Überarbeitung: `main@c3ccdb4`, 09.10.2026.** Die Quellübernahme aus Z01 ist auf `main` angekommen. Code, lokale Tests, Betreiberbefunde und eine vollständige Abnahme der Anlage sind unterschiedliche Nachweise. Den aktuellen Betriebs- und Prioritätsstand führt die [zentrale Roadmap](../roadmaps/gesamtroadmap.md).

## Hier anfangen

| Ziel | Einstieg | Danach |
|---|---|---|
| Eine Basisstation aufbauen | [Installation der lokalen Basisstation](basisstation-installieren.md) | [Konfiguration](basisstation-konfigurieren.md) · [Hardware und HF](hardware-sdr-und-hf-aufbau.md) · [Inbetriebnahme und Abnahme](inbetriebnahme-und-abnahme.md) |
| Die 26 Backend-Rollen verstehen | [Architektur](architektur-und-datenwege.md) | [Dienstkatalog](dienstkatalog.md) · [Netzwerk und Ports](netzwerk-und-ports.md) |
| Dienste oder Pi-Images bereitstellen | [Bereitstellung im Open Lab](dienste-und-pi-images-bereitstellen.md) | [Deployment Core](../services/deployment-core/README.md) · [Provisioning und Datenhoheit](teilnehmer-und-gruppen-anlegen.md) |
| Funk- und Datenwege prüfen | [Rufe](gruppen-und-einzelrufe.md) | [SDS und Status](kurznachrichten-und-status.md) · [Paketdaten und WAP](paketdaten-und-wap.md) |
| Leitstelle und Integrationen anbinden | [Control Room](leitstelle-und-node-gateway.md) | [SIP und Brew](telefonie-sip-und-brew.md) · [MQTT und Home Assistant](mqtt-home-assistant-und-homematic.md) |
| Fehler eingrenzen | [Fehlersuche](fehlersuche.md) | [Betrieb und Wartung](betrieb-wartung-und-reparatur.md) · [Backup und Fallback](datensicherung-und-fallback.md) |

## System in vier Ebenen

1. **Funkkante:** `bluestation-bs` betreibt SDR, Luftschnittstelle, lokale Registrierung, Rufe, SDS, Dashboard und lokalen Fallback.
2. **Netzkern:** Node Gateway verbindet die TBS mit den Fachkernen. Subscriber, Group, Mobility, Call Control, Media Switch, SDS Router und Packet Core haben eigene Daten und Zuständigkeiten.
3. **Anwendungen und Betrieb:** SIP Switch, IoT Gateway, Media Library, Workflows, Alert Service und Deployment Core ergänzen Telefonie, Automationen, Medien, Meldungen und Bereitstellung.
4. **Bedienung:** TBS-Dashboard, Directory, Provisioning Core, Control Room und Fach-WebUIs zeigen unterschiedliche Ausschnitte. [Bedienoberflächen](bedienoberflaechen-und-zustaendigkeiten.md)

## Aktueller Umfang und Grenzen

- Die [Inventoryvorlage](../../deploy/open-lab/inventory.example.toml) enthält **26 Backend-Rollen**. Deployment Core mit Imagebuilder benötigt eine vollständige Ubuntu-VM; die übrigen Rollen haben dienstspezifische LXC-Verfahren. TBS, Directory, Piper, lokaler Asterisk und optionaler Provisioning Core haben eigene Betriebswege.
- **Discovery, Agenten-Rollout, Pi-Imagebuilder und die optionale VPN-Heimnetzregel sind im Code vorhanden.** Physischer Pi-/SXceiver-Boot, VPN-Wechsel, Download und Recovery müssen für die konkrete Anlage mit den [Z01-Nachweisen](../integration/Z01-2026-10-07/README.md) abgeglichen werden.
- Der Rust-Workspace nennt `1.3.0`; der laufende Host benötigt trotzdem seinen tatsächlichen Commit, Build und Konfigurationsstand.
- Die Open-Lab-Beispiele stellen viele Management-APIs ohne Anmeldung oder TLS bereit. Alert Service ist eine Ausnahme mit tokenpflichtiger Verwaltung; das TBS-Dashboard hat optionale Cookie-Anmeldung. [Sicherheit und Betrieb](sicherheit-im-betrieb.md)
- ETSI-Quellen und die statische PDU-Matrix helfen bei der Entwicklung. Sie ersetzen keinen Interoperabilitäts- oder Funknachweis. [Normen und Tests](etsi-normen-und-tests.md)

Alle Wiki-Seiten stehen im [Themenindex](README.md). Das [Systemhandbuch](../archive/handbooks/systemhandbuch-2026-09-26-undatierte-ausgangsfassung.md) ergänzt technische Details; [Projektstand und Nachweisgrenzen](projektstand-und-nachweise.md) erklärt, welche Aussagen der jeweilige Nachweis trägt.

## Quellen zur Pflege dieser Seite

[Dienstregistry](../../system-backend/services.toml) · [Open-Lab-Inventory](../../deploy/open-lab/inventory.example.toml).
