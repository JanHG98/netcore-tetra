# NetCore-Tetra: So hängt das System zusammen

**Quellabgleich: 9. Oktober 2026, `main` bei `c3ccdb4`.** Dieses Handbuch beschreibt die vorhandenen Komponenten und ihre Grenzen. Den belegten Anlagenstand und die nächsten Abnahmen führt die [Gesamtroadmap](../roadmaps/gesamtroadmap.md). Für die Arbeitsfolge zur Einrichtung siehe [Inbetriebnahme](inbetriebnahme.md).

## Was NetCore-Tetra betreibt

NetCore verbindet TETRA-Basisstationen mit zentralen Diensten für Teilnehmer, Gruppen, Rufe, Sprache, Nachrichten und Paketdaten. Darüber liegen Leitstellenoberflächen, Integrationen und Werkzeuge für Installation und Überwachung. Diese Aufgaben sind auf getrennte Prozesse verteilt, damit eine ausgefallene Verwaltungskomponente den Funkbetrieb nicht pauschal mitreißt.

Eine **TBS** ist eine Basisstation. Sie führt die zeitkritischen Funkverfahren und verwaltet ihre lokalen Funkressourcen. **SwMI** bezeichnet die Vermittlungs- und Verwaltungsinfrastruktur des TETRA-Netzes. **ISSI** identifiziert einen Teilnehmer, **GSSI** eine Gruppe. Ein **Call Leg** ist die lokale Verbindung eines netzweiten Rufs auf genau einer TBS; die logische Call-ID verbindet die beteiligten Legs.

Die Trennung ist technisch vorhanden, aber kein pauschaler Nachweis für unterbrechungsfreie Wiederherstellung oder ETSI-Konformität. Quellcode, lokale Tests, Mehrdienst-Laborlauf und echte Funkabnahme sind unterschiedliche Nachweise.

## Welche Dienste dazugehören

Das [Beispielinventar](../../deploy/open-lab/inventory.example.toml) enthält **26 reguläre Rollen**. Die [Dienstregistrierung](../../system-backend/services.toml) enthält zusätzlich `shared`, also 27 Einträge; `shared` ist gemeinsame Infrastruktur und kein weiterer Listener. Die Ports unten sind die Management-Standardports dieses Inventars, keine vollständige Liste aller SIP-, MQTT-, Discovery- oder Nutzdatenports.

Die IP-Adressen `10.0.20.*` sind Beispiele und keine Aussage über die vorhandene Anlage. Eine Rolle muss nicht mit genau einem LXC gleichgesetzt werden: Der Imagebuilder-Controller gehört auf eine geeignete VM; Hardwarezugriffe benötigen passende Hostrechte.

| Rolle | Port | Aufgabe und Fachanleitung |
|---|---:|---|
| `node-gateway` | 8080 | [TBS-Verbindungen, Telemetrie, Kommandos und Dienstzustandsmatrix](../services/node-gateway/README.md) |
| `mobility-core` | 8090 | [Teilnehmerlage, Routenresolver und MM-Kontexttransfer](../services/mobility-core/README.md) |
| `subscriber-core` | 8100 | [Teilnehmerprofile und zentrale Zulassungsrichtlinie mit offener TBS-Anbindung](../services/subscriber-core/README.md) |
| `group-core` | 8110 | [Gruppen, Mitgliedschaften, Affiliationen und DGNA](../services/group-core/README.md) |
| `call-control` | 8120 | [Logische Rufe, lokale Legs, Sprechersteuerung und Restore](../services/call-control/README.md) |
| `media-switch` | 8130 | [Verteilung bereits codierter TETRA-Sprachframes](../services/media-switch/README.md) |
| `recorder` | 8140 | [Aufzeichnung, Integritätsprüfung und Aufbewahrung](../services/recorder/README.md) |
| `sds-router` | 8150 | [SDS-/Statusrouting und Zustellwarteschlangen](../services/sds-router/README.md) |
| `packet-core` | 8160 | [PDP-/NSAPI-Kontexte, Zustände, Reassembly und Referenzaktionen](../services/packet-core/README.md) |
| `ip-gateway` | 8170 | [HTTP-/TUN-Kopplung, IPv4, Firewall, NAT und Testdienste](../services/ip-gateway/README.md) |
| `security-core` | 8180 | [Funk-Sicherheitsrichtlinien und Lab-Authentisierungsablauf](../services/security-core/README.md) |
| `kmf` | 8190 | [Schlüsselmetadaten, Lab-Vault und OTAR-Aufträge](../services/kmf/README.md) |
| `transit` | 8200 | [Verbindung und Routing zwischen Regionen](../services/transit/README.md) |
| `application-gateway` | 8220 | [Anwendungsanschlüsse oberhalb des SDS Router](../services/application-gateway/README.md) |
| `media-library` | 8230 | [Medienbestand und TETRA-Audioformat](../services/media-library/README.md) |
| `control-room` | 9010 | [Leitstellenbedienung, Betriebslage und Dienstübersicht](../services/control-room/README.md) |
| `observability` | 8210 | [Zentrale Ereignisse, Logs, Metriken und Alarmierung](../services/observability/README.md) |
| `iot-gateway` | 8240 | [MQTT-Ereignisse, Zustände und geregelte Commands](../services/iot-gateway/README.md) |
| `hardware-gateway` | 8250 | [Rack-Hardware und ihre Anbindung an MQTT](../services/hardware-gateway/README.md) |
| `rf-monitor` | 8260 | [RF-Messwerte und Überwachung](../services/rf-monitor/README.md) |
| `alarm-workflow` | 8270 | [Alarmzustände und Eskalationsabläufe](../services/alarm-workflow/README.md) |
| `task-workflow` | 8280 | [Aufgaben, Formulare und WAP-Arbeitsabläufe](../services/task-workflow/README.md) |
| `asset-management` | 8290 | [Geräte- und Ressourcenbestand](../services/asset-management/README.md) |
| `sip-switch` | 8300 | [SIP-/Asterisk-Anbindung und Nummernrouting](../services/sip-switch/README.md) |
| `alert-service` | 8310 | [Warnmeldungsimport und kontrollierte SDS-Ausgabe](../services/alert-service/README.md) |
| `deployment-core` | 8320 | [Discovery, Hostaufträge und Pi-Images](../services/deployment-core/README.md) |

Der zusätzliche [Provisioning Core](../services/provisioning-core/README.md) auf `8125` bündelt Subscriber- und Group-Core-APIs in einer gemeinsamen Verwaltungsoberfläche. Er ist im Quellbaum und im Deployment-Agent-Rollenverzeichnis vorhanden, **nicht** Teil des 26-Rollen-Beispielinventars und **nicht** Eintrag der Dienstregistrierung. Er ersetzt die beiden fachlichen Cores nicht.

[Directory](../services/directory/README.md) und [TTS](../services/tts/README.md) sind weitere dokumentierte Funktionen beziehungsweise Komponenten; sie erhöhen diese Inventaranzahl nicht automatisch.

## Was auf der Basisstation bleibt

PHY, MAC, LLC, konkrete Air-PDU-Erzeugung, TDMA-Ressourcen und zeitkritische MLE-/MM-/CMCE-Verfahren bleiben im Funkstack. Die zentralen Cores erhalten normalisierte Telemetrie und geben fachliche Aufträge zurück. Eine HTTP-Annahme im Core kann deshalb einer noch ausstehenden Funkwirkung vorausgehen.

Die wesentlichen Grenzen sind in [TBS-Node-Protokoll](../../crates/tetra-entities/src/net_control_room/protocol.rs), [Steuerkommandos](../../crates/tetra-entities/src/net_control/commands.rs) und [gemeinsamen Verträgen](../../system-backend/shared/contracts) beschrieben. Der vorhandene Node-Transport nutzt weiterhin seine Laufzeitdatentypen; gemeinsame Verträge ersetzen ihn nicht allein durch ihre Existenz.

Für die Gateway-Anbindung nutzt die TBS den Abschnitt `[control_room]` ihrer Konfiguration. Dieser Name bezeichnet die vorhandene Node-Verbindung; das Ziel kann der Node Gateway mit `/ws/node` sein. Die separate Control-Room-WebUI ist kein notwendiger Zwischenprozess für jeden zentralen Ruf.

## Die vier wichtigsten Datenwege

| Vorgang | Zuständigkeiten und Bestätigungen |
|---|---|
| Registrierung und Bewegung | Subscriber Core erzeugt die Zulassungsrichtlinie und besitzt einen Verteil-/Bestätigungspfad für kompatible TBS. Die aktive TBS meldet `subscriber_policy = false`; der Sync endet als `unsupported`, und MM prüft weiterhin die lokale Whitelist. Mobility Core beobachtet Teilnehmerlage und besitzt die Core-Orchestrierung für bestätigten Export, Zielimport und Quellbereinigung. Die aktive TBS-MM behandelt `MobilityExportContext`, `MobilityImportContext` und `MobilityRemoveContext` jedoch noch als unsupported; der Ende-zu-Ende-Kontexttransfer ist nicht angeschlossen. Call Control löst reguläre Individualziele über den Mobility-Routenresolver auf. |
| Sprache | Call Control verwaltet Ruf und Legs. Media Switch übernimmt Call-Snapshots über `/ws/media`, bildet den Sprachgraphen und bestätigt RouteReady. TBS-Uplinkframes laufen über Node Gateway und Media Switch zu den Ziel-Legs. |
| SDS und Status | TBS erzeugt normalisierten Ingress; SDS Router plant TBS-/Anwendungsziele und hält Nachrichten persistent. Eine TBS-Annahme ist noch keine Endgerätequittung. Spätere Terminalberichte werden getrennt korreliert. |
| IPv4-Paketdaten | Packet Core hält Kontext-/Adresszustand und vollständige N-PDUs. IP Gateway koppelt seine Outbox über HTTP an Linux-TUN. Die vollständige Übertragung sämtlicher zentraler Referenzaktionen in den TBS-Funkpfad bleibt eine Integrationsgrenze. |

Bei Sprache wird der Operator-Floor erst freigegeben, wenn alle erforderlichen Legs aktiv und die benötigte Revision vom Media Switch bestätigt ist. Während des Rufstarts hält dessen begrenzter Kaltstartpuffer die ersten Frames zurück. Das verhindert keine beliebig langen Ausfälle; fehlende Legs, abgelaufene Puffer und Dropzähler bleiben sichtbar.

Der Recorder liest einen separaten Replayring. Ein langsamer Recorder erzeugt keine Backpressure im Sprachpfad. Fällt sein Cursor aus dem Ring, sind Frames nicht mehr nachholbar. Aufzeichnung bedeutet daher unveränderte Ablage übernommener Frames, keine Garantie eines lückenlosen Mitschnitts unter jeder Störung.

## Managementzugang, Funk-Sicherheit und TLS

Das Inventar nennt sich `open_lab`, aber die Dienste haben nicht alle dieselbe Zugriffskontrolle. Insbesondere dürfen die folgenden Ebenen nicht miteinander verwechselt werden:

| Ebene | Implementierter Stand |
|---|---|
| Die meisten Backend-HTTP-/WebSocket-APIs | Offener Managementmodus ohne allgemeine Anmeldung oder TLS; nur in einem kontrollierten Labor-/Managementnetz bereitstellen. |
| Control Room | Eigene Anmelde-/Rollenfunktionen vorhanden; die Beispielkonfiguration setzt `[auth].enabled = false`. Eine Leitstellenanmeldung sichert nicht automatisch direkte Fachdienst-URLs. |
| Alert Service | Standardmäßig Bearer-Token erforderlich; `allow_unauthenticated = false`. Das Inventar weist diese Rolle mit `security_mode = "token"` aus. |
| Security Core | `lab_hmac_sha256` ist ein Lab-Provider; daraus folgt kein vollständiger interoperabler TETRA-TA-/Produktivnachweis. |
| KMF | `lab_file_vault` und Schlüssel-/Jobverwaltung vorhanden. Kein fertiger Hardware-HSM-/PKCS#11- oder vollständiger produktiver OTAR-Nachweis. |
| MQTT und SIP | Brokerzugang, PBX-/TBS-SIP-Zugang und deren jeweilige Konfiguration getrennt betrachten. Ein MQTT-Command-Allow oder SIP-Passwort schützt keine offene Backend-WebUI. |
| Deployment und Pi | Controller-/Agent-WebUI ist Open Lab. OS-Benutzer, SSH-Key und optional OpenVPN schützen jeweils andere Verbindungen; sie führen keine Controller-Anmeldung ein. |

Die [Control-Room-Authentisierung](../services/control-room/anmeldung-und-rollen.md), [Alert-Service-Konfiguration](../../system-backend/alert-service/config/alert-service.example.toml) und die Fachanleitungen zu [Security Core](../services/security-core/README.md) und [KMF](../services/kmf/README.md) sind dafür die maßgeblichen Einstiege. HTTPS beim Download von Git- oder Paketquellen bedeutet nicht, dass ein eigener HTTP-Listener TLS bereitstellt.

## Welche Konfiguration führt

Für einen manuellen Hostbetrieb liegt die lokale Konfiguration meist unter `/etc/netcore/`. Die Komponenteninstaller erzeugen fehlende Dateien und setzen über [lxc-network.sh](../../system-backend/shared/install/lxc-network.sh) den WebUI-Listener auf die erkannte Host-IP. Die tatsächlich angekündigte URL steht in `/etc/netcore/lxc-network.env`.

Der [Inventar-Deployer](../../deploy/open-lab/netcore-deploy.py) erzeugt einen Konfigurationssatz aus Vorlagen und Hostzuordnung. Er ersetzt dabei passende URL-Hosts anhand eindeutiger Managementports. Er korrigiert nicht automatisch jeden Paketdaten-CIDR, jede separate Host-/Port-Einstellung oder jedes hardwarebezogene Feld.

Discovery-/Deployment-Agenten können zusätzlich eine Laufzeitkonfiguration und einen letzten bekannten guten Endpoint-Cache verwenden. Dann ursprüngliche TOML, erzeugte Runtime-Datei, Agentzuordnung und tatsächliche Prozessargumente gemeinsam prüfen. Ein alter guter Cache ist keine aktuelle Readiness-Bestätigung.

Die [Discovery-Anleitung](../services/deployment-core/README.md) beschreibt Konflikte mehrerer Instanzen einer Rolle und den bewussten Bindungswechsel. Discovery darf den Betreiber bei einem Konflikt nicht durch eine geratene Rollenzuordnung überraschen.

## Grenzen, die bei der Bedienung zählen

| Thema | Aktuelle Grenze und Konsequenz |
|---|---|
| Profilrechte | Subscriber Core speichert Dienstrechte und filtert die **zentrale Policy** anhand von `enabled` und `registration_allowed`. Die aktive TBS-MM-Zulassung liest diese Policy noch nicht; sie nutzt ihre lokale Konfigurations-/Dashboard-Whitelist, deren leere Liste offenes Netz bedeutet. Das Speichern eines Feldes ist keine aktive Netzsperre. |
| Gruppen-SDS | `sds_allowed` wird als Gruppenrichtlinie verteilt; SDS Router liest im abgeglichenen Stand keine gemeinsame Group-Core-Policy. Das Feld allein ist keine netzweite SDS-Sperre. |
| Mobility-Neustart | Teilnehmerlage und Transfer-History sind im Mobility Core flüchtig. Eine laufende Transferoperation ist nach Prozessneustart nicht persistent wiederaufnehmbar. |
| Paketdaten | Authoritative aktiviert das zentrale Referenzmodell, nicht automatisch einen vollständig angebundenen Funkdatenpfad. Packet-Core- und IP-Gateway-Beispiele verwenden zudem unterschiedliche IP-Pools. |
| Recorder | `allow_delete = false` sperrt manuelle API-Löschung; die automatische Retention läuft weiter. Für zu erhaltende Aufnahmen die technische Legal-Hold-Sperre berücksichtigen. |
| Transit | Noch kein ETSI ISI; das vorhandene Regionenprotokoll ist eine interne Referenzkopplung. |
| IoT-Commands | Beobachtung, Policy-Annahme und tatsächliche Ausführung sind getrennte Zustände. Die Beispielkonfiguration lässt externe Geräteaktionen nicht pauschal zu. |
| Pi-Image | Download, SHA-256 und Dateisystemprüfung belegen ein Artefakt; das Manifest führt `boot_tested = false`. Physischer Pi-, SXceiver- und VPN-Lauf sind eigene Abnahmen. |

Diese Grenzen sind Teil der Bedieninformation. Offene Implementierungen werden nicht durch einen grünen UI-Status oder eine erfolgreiche Kompilierung geschlossen.

## Woran ein belastbarer Betriebsstand erkennbar ist

Ein Prozess kann live sein und dennoch auf eine Abhängigkeit warten. `health/ready` beschreibt die jeweilige Dienstbereitschaft; der genaue Inhalt bleibt dienstspezifisch. Ein grüner Controller oder eine erreichbare WebUI sagt noch nichts über tatsächlich zugestellte Funknachrichten aus.

Für einen Betriebsnachweis gehören zusammen: vollständiger Commit, wirksame Konfiguration, beteiligte Hosts und Geräte, Zeitpunkt, Prüfablauf, Ergebnis und zugehörige Logs beziehungsweise Artefakte. Fehlende oder blockierte Prüfschritte bleiben als solche erkennbar.

Die [Laborprüfungen](../testing/e2e/README.md) verwenden HTTP-Verträge und Mock-TBS-Szenarien. Der [On-Air-Validator](../../tests/e2e/validate_on_air_evidence.py) prüft die Struktur manuell erhobener Funknachweise und optional deren Artefakt-Hashes; er führt selbst keine Funkmessung durch.

## Wo man weiterliest

- [Inbetriebnahme und laufender Betrieb](inbetriebnahme.md): Reihenfolge, Deployer, Teststufen und Pi-Artefakte.
- [Dienstübersicht](../services/README.md): vollständige Fachanleitungen und lokale Voraussetzungen.
- [Bereitstellung](../deployment/README.md): besondere Installations- und Migrationswege.
- [Gesamtroadmap](../roadmaps/gesamtroadmap.md): priorisierte Aufgaben und getrennte Abnahmen.
- [Archivierte Handbücher](../archive/handbooks): frühere vollständige Fassungen als historische Referenz.

Die heutigen fachlichen Quellen liegen unter `system-backend/`, der Funkstack unter `crates/` und `bins/`. Änderungen an diesen Grenzen erfordern eine gemeinsame Aktualisierung der betroffenen Fachanleitungen und eine neue passende Abnahme.

Die aktuelle [Teilnehmerzulassung](../services/subscriber-core/teilnehmerzulassung.md) trennt Core-Policy, vorhandenen Sync-Vertrag und fehlende aktive MM-Anbindung. Maßgebliche Quellen sind [TBS-Capabilities](../../crates/tetra-entities/src/net_control_room/protocol.rs) und [MM-Registrierung](../../crates/tetra-entities/src/mm/mm_bs.rs).
