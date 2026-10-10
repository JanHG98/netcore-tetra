# Backend-Dienste – Architektur, Betrieb und Quellen

Die Backend-Dienste sind vom zeitkritischen Funkstack getrennt. Lokale PHY-, MAC-, LLC-, MLE-, MM- und CMCE-Abläufe bleiben an der TBS; zentrale Dienste koordinieren Fachzustände und Anwendungen. Diese Übersicht beschreibt die Repository-Deklaration an `main@c3ccdb4bb7daa78f38c529c95ce51993ab0d0afc` und bestätigt keine installierte oder betriebsbereite Gesamtanlage.

## Dienstinventar und Zugriffsmodelle

Das [Beispiel-Inventory](../../deploy/open-lab/inventory.example.toml) deklariert **26 reguläre Dienste**. Die [Registry](../../system-backend/services.toml) enthält **27 Einträge: diese 26 Runtime-Dienste und `shared` mit `runtime = false`**. Der [Agentenkatalog](../../system-backend/deployment-core/catalog.json) beschreibt getrennt davon verwaltbare Rollen und kennt den optionalen Provisioning Core; dieser gehört weder zur Registry noch zum regulären Beispiel-Inventory. Beispieladressen im Netz `10.0.20.*` sind keine ermittelten Live-Adressen.

Der aktuelle Inventory-Managementzugriff verwendet dienstspezifische HTTP-Ports. Die meisten Registry-Dienste stehen im offenen Labormodus ohne Managementanmeldung oder TLS. Die Warnzentrale verlangt standardmäßig einen API-Token und aktiviert den Funkversand gesondert. Bediener-/Node-Authentisierung des Control-Room-Stacks ist separat in dessen Anleitung beschrieben; sie macht die übrigen Dienst-WebUIs nicht automatisch geschützt. HTTPS auf Port 8443 und gemeinsame zentrale IAM/RBAC-Rollen sind Zielvorgaben, keine bereits flächendeckend implementierte Betriebsform.

| Dienst / Anleitung | Aufgabe | Beispielport | Registry-Zugriff | Hostmodell |
| --- | --- | ---: | --- | --- |
| [node-gateway](node-gateway/README.md) | TBS-Sessions, Telemetrie und Aufträge an die Funkkante | 8080 | Open Lab, kein TLS | eigener LXC / geeigneter Servicehost |
| [mobility-core](mobility-core/README.md) | Serving-TBS-Lage und Core-Kontexttransfermodell; aktive MM-Anbindung offen | 8090 | Open Lab, kein TLS | eigener LXC / geeigneter Servicehost |
| [subscriber-core](subscriber-core/README.md) | Teilnehmerprofile und Teilnehmerzulassung | 8100 | Open Lab, kein TLS | eigener LXC / geeigneter Servicehost |
| [group-core](group-core/README.md) | Gruppenstammdaten, Mitgliedschaften, Affiliation und DGNA | 8110 | Open Lab, kein TLS | eigener LXC / geeigneter Servicehost |
| [call-control](call-control/README.md) | Netzweite logische Rufe, Legs, Floor und Restore | 8120 | Open Lab, kein TLS | eigener LXC / geeigneter Servicehost |
| [media-switch](media-switch/README.md) | Transport gepackter TETRA-Sprachframes | 8130 | Open Lab, kein TLS | eigener LXC / geeigneter Servicehost |
| [recorder](recorder/README.md) | Passive Sprachframe-Aufzeichnung und Export | 8140 | Open Lab, kein TLS | eigener LXC / geeigneter Servicehost |
| [sds-router](sds-router/README.md) | SDS-/Statuszustellung und Store-and-forward | 8150 | Open Lab, kein TLS | eigener LXC / geeigneter Servicehost |
| [packet-core](packet-core/README.md) | SNDCP-/PDP-Kontexte und zentrale Packet-Data-Lage | 8160 | Open Lab, kein TLS | eigener LXC / geeigneter Servicehost |
| [ip-gateway](ip-gateway/README.md) | Linux-TUN, IPv4-Routing, NAT, Firewall und Diagnose | 8170 | Open Lab, kein TLS | eigener LXC / geeigneter Servicehost |
| [security-core](security-core/README.md) | Authentisierung, Policy und kurzlebige Sicherheitskontexte | 8180 | Open Lab, kein TLS | eigener LXC / geeigneter Servicehost |
| [kmf](kmf/README.md) | Schlüssellebenszyklus, Vault und OTAR-Orchestrierung | 8190 | Open Lab, kein TLS | eigener LXC / geeigneter Servicehost |
| [transit](transit/README.md) | Regionenübergreifende Vermittlung und Failover | 8200 | Open Lab, kein TLS | eigener LXC / geeigneter Servicehost |
| [application-gateway](application-gateway/README.md) | Externe Connectoren, Webhooks und Anwendungszustellung | 8220 | Open Lab, kein TLS | eigener LXC / geeigneter Servicehost |
| [media-library](media-library/README.md) | Audio-Assets, Vorschau, Freigabe, TTS und Stations-Playout | 8230 | Open Lab, kein TLS | eigener LXC / geeigneter Servicehost |
| [control-room](control-room/README.md) | Lageaggregation, Bedienung, Incidents und Schichtbuch | 9010 | Open Lab, kein TLS | eigener LXC / geeigneter Servicehost |
| [observability](observability/README.md) | Metriken, Logs, Traces, Alarme und Syslog-Archiv | 8210 | Open Lab, kein TLS | eigener LXC / geeigneter Servicehost |
| [iot-gateway](iot-gateway/README.md) | MQTT-Ereignisse, Richtlinien, Befehle und Quittierungen | 8240 | Open Lab, kein TLS | eigener LXC / geeigneter Servicehost |
| [hardware-gateway](hardware-gateway/README.md) | Edge-I/O, Sensorik und Racküberwachung | 8250 | Open Lab, kein TLS | eigener LXC / geeigneter Servicehost |
| [rf-monitor](rf-monitor/README.md) | HF-/DSP-Telemetrie und Grenzwertüberwachung | 8260 | Open Lab, kein TLS | eigener LXC / geeigneter Servicehost |
| [alarm-workflow](alarm-workflow/README.md) | Persistente Alarmakten und Eskalation | 8270 | Open Lab, kein TLS | eigener LXC / geeigneter Servicehost |
| [task-workflow](task-workflow/README.md) | Aufträge, WAP-Formulare und Statusaktionen | 8280 | Open Lab, kein TLS | eigener LXC / geeigneter Servicehost |
| [asset-management](asset-management/README.md) | Physische Assets, Personen, Ausgaben und Wartung | 8290 | Open Lab, kein TLS | eigener LXC / geeigneter Servicehost |
| [sip-switch](sip-switch/README.md) | PBX-/TBS-SIP-Vermittlung mit lokaler Notvermittlung | 8300 | Open Lab, kein TLS | eigener LXC / geeigneter Servicehost |
| [alert-service](alert-service/README.md) | NINA/KATWARN und eigene standortbezogene Warnmeldungen | 8310 | API-Token, kein TLS | eigener LXC / geeigneter Servicehost |
| [deployment-core](deployment-core/README.md) | Discovery, Git-Pinning, Deployments und Pi-Imagebuilder | 8320 | Open Lab, kein TLS | vollständige Ubuntu-VM |

## Weitere Komponenten und klare Zuständigkeiten

- [Provisioning Core](provisioning-core/README.md): optionaler Verwaltungsdienst im Agentenkatalog; kein Eintrag der Registry oder des regulären 26er-Beispiel-Inventorys.
- [Directory](directory/README.md): Namen, Geräte-/Gruppen-/Statusmetadaten; kein Ersatz für Teilnehmerzulassung oder aktuelle Serving-TBS-Lage.
- [Shared](shared/README.md): Verträge, Service-/Datenbank-/Telemetry-Bibliotheken und build-freie WebUI-Assets; kein eigener Container.
- [Piper/TTS](tts/README.md): zentrale Erzeugung im Media-Library-Umfeld; kein weiterer Pflicht-LXC im Inventory.
- [TBS Connect](tbs-connect/README.md): bestehende Anbindung; Zuständigkeit gegenüber Node Gateway anhand seiner Anleitung beurteilen.

## Bereitstellung und Nachweise

[Inventory-Bereitstellung](../deployment/open-lab/README.md) dokumentiert Validierung, Plan, Rendern und Ready-Schranke. Bestehende Host-Konfigurationen werden erhalten; bewusster Konfigurationsersatz benötigt den dafür vorgesehenen Schalter und Rückweg. Ein Readinessfehler stoppt Folgeinstallationen, stellt aber nicht automatisch sämtliche bereits geänderten Binaries oder Datenbanken zurück. Die Deployment-/Imagebuilder-Rolle verwendet den VM-Installer; ein gewöhnlicher LXC ersetzt dessen Image-/Worker-Voraussetzungen nicht.

Prüfung und Abnahme: [Systemtests](../testing/e2e/README.md) · [Z01-Nachweise](../integration/Z01-2026-10-07/README.md) · [On-Air-Abnahme](../wiki/inbetriebnahme-und-abnahme.md) . Quellbestand, statische Gates, isolierte Diensttests, echte Anlagenpiloten und Funkabnahme bleiben getrennte Stufen. Der vollständige VM119-Build an1595259 ist dokumentiert; Download/Dateiprüfsumme und physischer Pi-/SXceiver-/VPN-Test bleiben eigene Nachweise.

Architekturziele: [WebUI-Standard](../design/dienstoberflaechen-gestaltungsstandard.md) · [Schnittstellenverträge](../contracts/README.md) · [Gesamtroadmap](../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../README.md).
