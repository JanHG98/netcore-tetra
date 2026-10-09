# NetCore-Tetra System Backend

**Quellen:** [system-backend](../../system-backend) · [Repository-Root](../..). Bei Befehlen das in der Anleitung angegebene Arbeitsverzeichnis beachten.

Dieser Ordner enthält alle Dienste, die später unabhängig von der TBS als LXC, VM oder zentraler Backend-Prozess betrieben werden.

## Dienstanleitungen

[Dokumentationsindex](../README.md) · [Gesamtroadmap](../roadmaps/ROADMAP.md)

- [alarm-workflow](alarm-workflow/README.md)
- [alert-service](alert-service/README.md)
- [application-gateway](application-gateway/README.md)
- [asset-management](asset-management/README.md)
- [call-control](call-control/README.md)
- [control-room](control-room/README.md)
- [deployment-core](deployment-core/README.md)
- [directory](directory/README.md)
- [group-core](group-core/README.md)
- [hardware-gateway](hardware-gateway/README.md)
- [iot-gateway](iot-gateway/README.md)
- [ip-gateway](ip-gateway/README.md)
- [kmf](kmf/README.md)
- [media-library](media-library/README.md)
- [media-switch](media-switch/README.md)
- [mobility-core](mobility-core/README.md)
- [node-gateway](node-gateway/README.md)
- [observability](observability/README.md)
- [packet-core](packet-core/README.md)
- [provisioning-core](provisioning-core/README.md)
- [recorder](recorder/README.md)
- [rf-monitor](rf-monitor/README.md)
- [sds-router](sds-router/README.md)
- [security-core](security-core/README.md)
- [shared](shared/README.md)
- [sip-switch](sip-switch/README.md)
- [subscriber-core](subscriber-core/README.md)
- [task-workflow](task-workflow/README.md)
- [tbs-connect](tbs-connect/README.md)
- [transit](transit/README.md)
- [tts](tts/README.md)

## Grundregeln

- Jeder deploybare Dienst besitzt einen eigenen Unterordner.
- Funknahe Echtzeitkomponenten bleiben außerhalb von `system-backend/`.
- Gemeinsamer Backend-Code liegt unter `shared/`.
- ZIP-Lieferungen behalten den vollständigen Pfad `system-backend/<dienst>/...` bei.
- **Jeder eigenständig laufende Container oder jede VM besitzt eine eigene WebUI zur Verwaltung.**
- Die WebUI wird vom jeweiligen Dienst selbst ausgeliefert; dafür wird kein zusätzlicher Frontend-Container benötigt.
- Ein Ausfall der WebUI darf niemals die fachliche Runtime des Dienstes stoppen.
- Der Control Room verlinkt und aggregiert die Service-WebUIs, ersetzt sie aber nicht.

## Verbindlicher WebUI-Standard

Die gemeinsame Vorgabe steht in:

```text
Docs/design/BACKEND_WEBUI_STANDARD.md
```

Die dienstspezifischen Verwaltungsbereiche stehen in:

```text
Docs/design/BACKEND_WEBUI_SERVICE_MATRIX.md
```

Gemeinsame UI-Bausteine und Service-Verträge liegen unter:

```text
system-backend/shared/
├── contracts/
├── service-common/
├── database-common/
├── telemetry-common/
└── web-ui/
```

## Standardzugriff

Langfristig verwenden neue Dienste mit eigener LXC-IP einheitlich:

```text
https://<LXC-IP>:8443/
```

Die bisher umgesetzten Dienste verwenden je Dienst einen eigenen HTTP-Port in der isolierten Testumgebung. Die verbindliche Zuordnung steht in `services.toml`; die fortlaufende Dienstreihe reicht aktuell vom Recorder auf Port 8140 bis zur Warnzentrale auf Port 8310. Der Control Room bleibt auf Port 9010. Die Warnzentrale benötigt standardmäßig ein API-Token; die älteren Dienste verwenden überwiegend den offenen Labormodus.

## Bereits deploybare Dienste

Bereits deploybar sind:

- `node-gateway/` – TBS- und Backend-Vermittlung, Port 8080
- `mobility-core/` – Teilnehmerlage und MM-Context-Transfer, Port 8090
- `subscriber-core/` – Teilnehmerprofile und Admission, Port 8100
- `group-core/` – Gruppen, Mitgliedschaften und DGNA, Port 8110
- `provisioning-core/` – zentrale Geräte-, Gruppen- und Mitgliedschaftsmatrix, Port 8125
- `call-control/` – logische Calls, Floor Control und Restore, Port 8120
- `media-switch/` – Routing gepackter TETRA-Sprachframes, Port 8130
- `recorder/` – passive Aufnahme, Integrität, Retention und Export, Port 8140
- `sds-router/` – SDS-/Statusvermittlung, Store-and-forward und Anwendungsrouten, Port 8150
- `packet-core/` – PDP-/NSAPI-State-Machine, Mobility Anchoring, Fragmentierung und Flow Control, Port 8160
- `ip-gateway/` – TUN, Routing, NAT, Firewall, DNS, WAP/Testdienste und PCAP, Port 8170
- `security-core/` – Security-Class-Policy, Authentisierung, DCK-Kontexte, Sperren und Audit, Port 8180
- `kmf/` – CCK/GCK/SCK, Crypto Periods, Rotation, versiegelte OTAR-Aktionen und Backups, Port 8190
- `transit/` – regionale Peer-/Route-/Sessionvermittlung und Failover, Port 8200
- `observability/` – Metriken, Logs, Traces, Alarmierung und Diagnose, Port 8210
- `application-gateway/` – externe Connectoren, Webhooks, Routing, Vorlagen und TTS-Orchestrierung, Port 8220
- `media-library/` – Audio-Assets, Vorschau, Freigabe, TETRA-Cache, Archiv und Playout, Port 8230
- `iot-gateway/` – netcore-event-v1 nach MQTT, persistente Outbox, retained Zustände und Command-Beobachtung, Port 8240
- `hardware-gateway/` – Edge-I/O, Rack- und Umgebungsüberwachung, Port 8250
- `rf-monitor/` – zentrale HF-, PA-, Antennen- und Modulationsüberwachung, Port 8260
- `alarm-workflow/` – SDS-, Status-, Alarm- und Eskalationsworkflows, Port 8270
- `task-workflow/` – WAP-Formulare und strukturierte Aufträge, Port 8280
- `asset-management/` – Asset-, Geräte- und Benutzerverwaltung, Port 8290
- `sip-switch/` – zentraler PBX-/TBS-SIP-B2BUA mit Mobility-Core-Routing, Port 8300
- `control-room/` – zentrale Bedien-, Lage-, Incident- und Schichtbuchebene, Port 9010
- `alert-service/` – NINA-/KATWARN-Warnungen und eigene Kartenwarnungen mit GPS-basierter einmaliger SDS-Zustellung, Port 8310; Tokenzugang als Standard. Konkrete Installation und Prüfung pro LXC/TBS: [Schritt-für-Schritt-Anleitung](../deployment/KATWARN_NINA_INSTALL_UPDATE.md).

Alle enthalten REST-API, eigene WebUI, systemd-Unit und Installationsskripte. Die Warnzentrale sowie Hardware Gateway, RF Monitor und mehrere Workflow-Dienste verwenden Python; die übrigen Kernservices verwenden Rust. Die älteren Dienste laufen in der aktuellen Teststufe überwiegend im `open_lab`-Modus ohne Tokens, Benutzeranmeldung oder TLS. Die Warnzentrale aktiviert Tokenzugriff als Standard und startet mit deaktiviertem Funkversand.


## Gemeinsame Plattform und Deployment

Die gemeinsame Vertragsversion ist `netcore.v1`. Die inventory-gesteuerte Open-Lab-LXC-Integration liegt unter `deploy/open-lab/` und erzeugt Servicekatalog, gerenderte Konfigurationen, Portliste, Hosts-Datei und Abhängigkeitsgraph. `shared/` bleibt eine Library und ist kein zusätzlicher Container.

## Cross-LXC-Systemtest

Die Backend-Dienste werden über `tests/e2e/` als Gesamtsystem geprüft. Der inventory-gesteuerte Runner enthält einen Mock TBS für die Node-Gateway-Schnittstelle, fachliche Call-/Media-/Recorder-, SDS- und Packet-Data-Szenarien, Control-Room-Federation, redaktierte Plattform-Managementansichten, Persistenztests sowie eine absichtliche Dependency-Ausfallmatrix. Aufruf und Sicherheitsgrenzen stehen in `Docs/deployment/OPEN_LAB_E2E_RUNBOOK.md`.

- `task-workflow` (`8280`): strukturierte Aufträge, XHTML/WML-Formulare, SDS-/Statusaktionen und persistente Task-Akte.

- `asset-management` (`8290`): physischer Bestand, Funkgeräte, Personen, Ausgaben, Wartung sowie lesender Abgleich mit Subscriber und Mobility Core.
