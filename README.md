# NetCore-Tetra

NetCore-Tetra verbindet eine softwarebasierte TETRA-Basisstation mit zentralen Backend-Diensten, Leitstellenoberflächen und Integrationen für ein kontrolliertes Labor. Zeitkritische Funkprozeduren bleiben an der TBS; die Backend-Dienste übernehmen netzweite Verwaltung und Vermittlung.

**Dokumentation:** [Systemhandbuch](Docs/handbooks/systemhandbuch.md) · [Inbetriebnahme](Docs/handbooks/inbetriebnahme.md) · [Themenindex](Docs/README.md) · [Gesamtroadmap](Docs/roadmaps/gesamtroadmap.md)

## Aktueller Quellenumfang

**Quellenabgleich: 10. Oktober 2026, `main` bei `c3ccdb4`.** Das [Betriebsinventar](deploy/open-lab/inventory.example.toml) enthält **26 reguläre Rollen**. Die [Dienstregistry](system-backend/services.toml) enthält diese Dienste und das gemeinsame Paket `shared`, also 27 Einträge. Provisioning Core ist eine weitere optionale Verwaltungsrolle außerhalb dieses Inventars und dieser Registry.

Deployment-Controller, Discovery, Imagebuilder, Pi-Firstboot/VPN und Syslog sind auf main vorhanden. Die bisherigen Anlagenprüfungen stehen in den [Integrationsnachweisen](Docs/integration/Z01-2026-10-07/README.md). Ein erfolgreicher Build ist von Downloadintegrität, Pi-/SXceiver-Boot, VPN-Wechsel und vollständiger Betriebsübernahme zu unterscheiden.

Wichtige Integrationsgrenzen des aktuellen Stands:

- Subscriber Core erzeugt zentrale Teilnehmerrichtlinien. Die aktive TBS kündigt `subscriber_policy = false` an; ihre MM-Registrierung verwendet weiterhin die lokale Whitelist.
- Mobility Core besitzt die Orchestrierung für Kontextexport, Import und Bereinigung. Die dazugehörigen Kommandos werden im aktiven TBS-MM noch nicht verarbeitet.
- Zentrale Gruppenaufträge, Restore und Paketdaten besitzen weitere offene Übergänge zwischen Core und aktivem Funkpfad. Die jeweilige Fachanleitung nennt den konkreten Umfang.
- Protokolltypen, statische Prüfungen und Labor-Mocks belegen für sich keine ETSI-Konformität oder erfolgreiche Funkabnahme.

Die lokale Basisstation und die Dienste besitzen eigene Bedien- und Konfigurationswege. Managementzugriff ist je Dienst unterschiedlich geregelt: Manche Fachkerne laufen offen im Labor, andere verwenden lokale Konten oder Tokens. Die [Dienstanleitungen](Docs/services/README.md) beschreiben die tatsächlichen Verträge.

## Einstieg für Entwicklung und Betrieb

- [Basisstation installieren](Docs/wiki/basisstation-installieren.md) und [Hardware einordnen](Docs/wiki/hardware-sdr-und-hf-aufbau.md).
- [Backend-Dienste](Docs/services/README.md), [Deployment-Controller](Docs/services/deployment-core/README.md) und [Open-Lab-Deployment](Docs/deployment/open-lab/README.md).
- [Prüfwerkzeuge](Docs/development/README.md), [E2E-Anleitungen](Docs/testing/e2e/README.md) und [Protokollinventur](Docs/protocols/README.md).
- [NINA/KATWARN](Docs/services/alert-service/README.md), [SIP und TBS-Fallback](Docs/services/sip-switch/README.md), [Medien/TTS](Docs/services/media-library/README.md) und [MQTT](Docs/services/iot-gateway/README.md).

## Hier anfangen

| Du möchtest … | Passender Einstieg |
|---|---|
| das Gesamtsystem verstehen | [Systemhandbuch](Docs/handbooks/systemhandbuch.md) |
| eine Anlage einrichten oder aktualisieren | [Inbetriebnahme](Docs/handbooks/inbetriebnahme.md) |
| einen bestimmten Dienst konfigurieren | [Dienstübersicht mit Ports und Betriebswegen](Docs/services/README.md) |
| den Projektstand und nächste Schritte kennen | [Gesamtroadmap](Docs/roadmaps/gesamtroadmap.md) |
| eine Anleitung oder einen Nachweis finden | [Dokumentationsindex](Docs/README.md) |

## Was wo liegt

| Verzeichnis | Inhalt |
|---|---|
| [`crates/`](crates/) | Funkprotokolle, Datenstrukturen, Konfiguration und TBS-Entities |
| [`bins/`](bins/) | Basisstationsprogramm und Rust-Control-Core |
| [`system-backend/`](system-backend/README.md) | Fachdienste, Leitstellenanwendungen und gemeinsame Bausteine |
| [`deploy/open-lab/`](deploy/open-lab/README.md) | Inventar, Konfigurationsvorlagen und SSH-Deployer |
| [`ms-mode/`](ms-mode/README.md) | MS-Modus mit eigenem Workspace und eigener Konfiguration |
| [`tests/e2e/`](tests/e2e/README.md) | Laborprüfungen und Nachweisvalidierung |
| [`tools/`](tools/README.md) | Quellenprüfungen und Dokumentationsgeneratoren |
| [`Docs/`](Docs/README.md) | Aktuelle Fachtexte, Roadmaps und historische Nachweise |

Die native Leitstellenoberfläche unter `system-backend/control-room/ui/`, der Python-Control-Room und der Rust-Core unter `bins/` sind unterschiedliche Anwendungen. Für Buildziel und Konfiguration die jeweilige Anleitung verwenden.

## Erste Prüfungen ohne Deployment

Im vollständigen Checkout ab Repository-Root, mit Python 3.11 oder neuer:

```bash
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.example.toml validate
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.example.toml plan
python3 tools/check_full_system_integration.py
python3 tools/protocol_inventory.py --check
```

`validate` und `plan` prüfen lokale Konfiguration und Abhängigkeiten. Der Systemaudit schreibt seinen generierten Bericht; der Protokollcheck vergleicht die Inventur mit dem Rust-Quellbaum. Diese Befehle installieren keine Dienste und ersetzen keine erreichbare Anlage. Für Buildvoraussetzungen, Rendern, gezielte Updates und echte Prüfungen die [Inbetriebnahme](Docs/handbooks/inbetriebnahme.md) lesen.

## Dokumentation pflegen

Fachtexte liegen zentral unter `Docs/`; kurze lokale READMEs erklären den jeweiligen Quellordner und verlinken zur Anleitung. Dateinamen sind verständliche deutsche Themenbezeichnungen. Aufgaben-IDs, Versionsstände und historische Ergebnisse bleiben im Inhalt erhalten.

Die [alten Handbuchausgaben](Docs/archive/handbooks/README.md) und [Projektnotizen](Docs/archive/README.md) behalten ihren damaligen Geltungszeitraum. Die [Ablageregeln](Docs/dokumentationsablage.md) und das [Pfadverzeichnis](Docs/documentation-paths.json) dokumentieren die heutige Zuordnung.

Release-, Workspace- und Komponentenversionen sind unterschiedliche Angaben. Für eine Installation den vollständigen Git-SHA und die tatsächlichen Artefakte festhalten; die [Release-Historie](Docs/releases/README.md) ersetzt diese Zuordnung nicht.

[Verhaltenskodex](CODE_OF_CONDUCT.md) · [Hinweise für Arbeit am Repository](AGENTS.md)
