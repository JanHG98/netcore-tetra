# NetCore-Tetra einrichten, prüfen und betreiben

**Quellabgleich: 9. Oktober 2026, `main` bei `c3ccdb4`.** Diese Arbeitsfolge basiert auf [Inventar-Deployer](../../deploy/open-lab/netcore-deploy.py), [Beispielinventar](../../deploy/open-lab/inventory.example.toml), [E2E-Runner](../../tests/e2e/netcore_open_lab_e2e.py) und [Imagebuilder](../../system-backend/deployment-core/image_build.py). Sie protokolliert keine in diesem Dokument bereits ausgeführte Anlagenabnahme. Für Rollen und Datenwege siehe [Systemhandbuch](systemhandbuch.md).

## 1. Den Zielumfang festhalten

Für den vorgesehenen Lauf Commit, Hosts, Rollen und Abnahmekriterium festlegen. Eine kleine Gateway-/Teilnehmerprüfung benötigt einen anderen Umfang als netzweite Sprache, Paketdaten oder ein Pi-Image. Die [Gesamtroadmap](../roadmaps/gesamtroadmap.md) ordnet offene Abhängigkeiten und vorhandene Nachweise ein.

Das Beispielinventar hat 26 reguläre Rollen. `shared` ist ein zusätzlicher Registrierungseintrag ohne eigenen Listener. Provisioning Core ist eine separate optionale Verwaltungsoberfläche und steht nicht im Beispielinventar oder in der Dienstregistrierung.

Vor einem Update Konfiguration und persistenten Zustand sichern. Ein erfolgreicher Build ersetzt weder die Betriebsprüfung noch die Wiederherstellung aus einer Sicherung. Testgeräte und Laborgruppen eindeutig zuordnen; Hardware-/Funkwerte aus der geprüften Standortkonfiguration übernehmen.

## 2. Hosts und Netzwerk vorbereiten

- Vollständiger Repository-Checkout und Python 3.11 oder neuer auf dem Steuerhost.
- Zielhosts mit zum jeweiligen Installer passenden Build-/Laufzeitwerkzeugen und systemd; Rust-Toolchain auch für den ausführenden root-/sudo-Prozess erreichbar.
- Erreichbare Management-IP pro Rolle; DHCP mit Reservierung ist möglich. Hostnamen müssen dort auflösbar sein, wo die jeweilige Verbindung aufgebaut wird.
- Für SSH-Deployer Schlüsselzugriff entsprechend `ssh_user` und `ssh_options`; das Beispiel verwendet root und `StrictHostKeyChecking=accept-new`.
- Für IP Gateway `/dev/net/tun`, `iproute2`, `nftables` und erlaubte Capabilities; für Rack-/RF-Dienste die jeweils notwendigen Gerätezugriffe.
- Imagebuilder auf einer geeigneten VM statt einem beliebigen unprivilegierten LXC; Voraussetzungen stehen in der [Deployment-Core-Anleitung](../services/deployment-core/README.md).

Die offenen Backend-Managementports nur in einem kontrollierten Labor-/Managementnetz erreichbar machen. Alert Service hat standardmäßig eine Tokenprüfung; Control-Room-Login ist separat konfigurierbar. Das Inventarlabel `open_lab` hebt diese Unterschiede nicht auf.

## 3. Inventar an die Anlage anpassen

Alle folgenden relativen Befehle ab Repository-Root ausführen. Den Beispielpfad des Checkouts bei Bedarf ändern:

```bash
cd /opt/netcore-tetra
cp deploy/open-lab/inventory.example.toml deploy/open-lab/inventory.toml
editor deploy/open-lab/inventory.toml
```

Die Kopie nur bei der Ersteinrichtung anlegen. Vorhandenes Inventar direkt bearbeiten, damit lokale Hostzuordnungen erhalten bleiben. `host`, Managementport, `unit`, Dienstkonto, Konfigurationsziel, Abhängigkeiten und `remote_source_root` gegen die tatsächlichen Hosts prüfen.

Die Beispieladressen sind keine automatische Erkennung der Anlage. Auch der TBS-Abschnitt `[tbs_site]`, Gateway-Adresse, Standort-TOML, MQTT-Broker, SIP/PBX und externe Ziele müssen zum Netz passen. Passwörter oder private Schlüssel nicht in generierte öffentliche Kataloge schreiben.

Die Packet-Core-Vorlage vergibt `10.44.0.0/24`, während die IP-Gateway-Vorlage `10.0.0.0/24` nutzt. Vor gemeinsamem Paketbetrieb Adresspool, TUN-Adresse/-Netz, DNS-Bind, Firewall-CIDRs und Routen konsistent setzen.

## 4. Offline prüfen und die Reihenfolge ansehen

```bash
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml validate
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml plan
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml render
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml check-generated
```

| Befehl | Tatsächliche Wirkung |
|---|---|
| `validate` | Prüft lokale Dateien, Ports, Dienstkonten, Betriebsmodus und Abhängigkeitsgraph; keine Hostabnahme. |
| `plan [rollen ...]` | Zeigt die Abhängigkeiten in Installationsreihenfolge. Ausgewählte Rollen ziehen ihre deklarierten Voraussetzungen mit. |
| `render` | Schreibt Konfigurationen, Katalog, Hostsdatei, Portliste und Graph unter `deploy/open-lab/generated/`. |
| `check-generated` | Vergleicht vorhandene generierte Dateien mit dem erwarteten Satz, ohne sie zu aktualisieren. |

Der Renderer ersetzt passende URL-Hosts anhand eindeutig zugeordneter Ports. Separate Host-/Port-Felder, Netz-CIDRs, RF-Werte und Secrets werden dadurch nicht automatisch richtig. Die erzeugten Konfigurationen vor Verwendung fachlich prüfen.

```bash
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml apply --dry-run
```

`--dry-run` führt kein SSH-/SCP-Deployment aus, erzeugt aber lokal den Konfigurationssatz und ein temporäres Bundle. Die ausgegebenen Zielhosts, Konfigurationspfade und Installationskommandos prüfen. Der echte Apply-Lauf ersetzt auf jedem Ziel den dedizierten Quellbaum `remote_source_root`; dieser Pfad darf keine unabhängigen lokalen Arbeiten enthalten.

## 5. Bewusst installieren oder aktualisieren

Nach der Vorprüfung ist der echte Lauf:

```bash
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml apply
```

Bei gezieltem Teilumfang können Rollen angegeben werden, etwa `apply node-gateway mobility-core`. Abhängigkeiten werden weiterhin berücksichtigt. Der Installer stoppt beziehungsweise startet die jeweilige Instanz; dieser Schritt gehört in den geplanten Betriebszeitraum.

Bestehende Hostkonfiguration bleibt standardmäßig erhalten. Das bedeutet zugleich: Ein neues gerendertes Inventar korrigiert vorhandene falsche Hostwerte nicht stillschweigend. `apply --replace-config` ist eine ausdrückliche Ersetzung durch den gerenderten Satz und legt für eine vorhandene Datei eine datierte Rückfallkopie an. Vorher Unterschiede und Secrets prüfen.

Der Deployer wartet nach jeder Rolle begrenzt auf Readiness. Eine noch nicht bereite Voraussetzung blockiert die folgende Installation. Bei Abbruch Ursache, tatsächlich gestartete Units und wirksame Konfiguration prüfen; das Verfahren ist kein automatischer Gesamtrollback aller zuvor installierten Rollen.

Deployment-Core-VM, TBS-Bootstrap, zusätzliche Provisioning-Rolle und spezielle Hardwarewege jeweils nach ihrer Fachanleitung einrichten. Der Inventarlauf ersetzt diese standortabhängigen Voraussetzungen nicht.

## 6. An der tatsächlichen Host-IP prüfen

```bash
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml status
```

`status [rollen ...]` prüft die Readiness-URLs aus dem Inventar. Das lokal angenommene `127.0.0.1` ist nach Komponenteninstallation oft der falsche Endpunkt: Der gemeinsame LXC-Netzwerkhelfer bindet WebUI/API an die erkannte Host-IP.

Auf dem jeweils betroffenen Host, beispielsweise beim Node Gateway:

```bash
source /etc/netcore/lxc-network.env
curl --fail "${NETCORE_WEBUI_URL}health/live"
curl --fail "${NETCORE_WEBUI_URL}health/ready"
systemctl status netcore-node-gateway --no-pager
journalctl -u netcore-node-gateway -n 100 --no-pager
```

Diese Umgebungsdatei enthält die vom Installer veröffentlichte Adresse. Bei gemeinsamer Installation mehrerer Rollen oder Discovery-Runtime zusätzlich den dienstspezifischen Listener und die tatsächlichen Prozessargumente prüfen. API-Aufrufe aus Fachbeispielen mit `127.0.0.1` entsprechend anpassen.

`health/live` belegt nur den laufenden Prozess. Readiness, Status, Verbindung zur Abhängigkeit und fachliche Bestätigungen getrennt ansehen. Eine Gateway-Command-ID zeigt zunächst Einreihung; die TBS-Response und gegebenenfalls die beobachtete Funkwirkung folgen später.

Vor einer behaupteten zentralen Zulassungsprüfung die [Teilnehmerzulassung](../services/subscriber-core/teilnehmerzulassung.md) lesen: Die aktive TBS annonciert `subscriber_policy = false`, der Core-Sync endet als `unsupported`. `enabled` und `registration_allowed` filtern die zentrale Policy, werden aber nicht dadurch zu einer aktiven MM-Sperre. Die lokale TBS-Konfigurations-/Dashboard-Whitelist bleibt wirksam; ihre leere Liste öffnet das Netz.

## 7. Laborprüfung stufenweise durchführen

```bash
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml test --list-scenarios
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml test --profile full --validate-only
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml test --profile smoke
```

`--validate-only` prüft Inventar und Szenarionamen ohne Netzwerkzugriff. Der Smoke-Lauf verwendet Managementverträge und standardmäßig eine Mock-TBS-Verbindung; er ist keine HF-Prüfung. Readiness 503 kann ohne `--strict-ready` zulässig sein und muss anhand des Reports bewertet werden.

Für den bewusst freigegebenen funktionalen beziehungsweise Ausfalllauf:

```bash
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml test --profile full --allow-mutations
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml test --profile fault --allow-mutations --allow-restarts
```

`--allow-mutations` erlaubt Testprofile und fachlichen Testverkehr. `--allow-restarts` erlaubt die zusätzlichen SSH-/systemd-Ausfalltests. `--scenario NAME` wählt konkrete Szenarien; `--keep-fixtures` lässt Testdaten zur Diagnose stehen; `--no-mock-tbs` unterdrückt die simulierte Node-Verbindung. Diese Optionen sind keine austauschbaren Qualitätsstufen.

Ergebnisse liegen standardmäßig unter `tests/e2e/artifacts/<run-id>/` als `report.json`, `junit.xml` und `summary.txt`. Für Abnahme fehlgeschlagene Checks, Skips, Cleanup und die Erholung absichtlich gestoppter Dienste auswerten. `failed=0` bei übersprungenem Fachtest belegt diesen Fachtest nicht.

Details und die aktuelle Szenarioliste stehen in [Laborprüfungen](../testing/e2e/README.md) und [E2E-Arbeitsfolge](../deployment/open-lab-integrationstest-anleitung.md).

## 8. Pi-Image vom Build bis zur Geräteprüfung

Das [Pi-Image-Verfahren](../services/deployment-core/README.md) verwendet ein geprüftes TBS-Profil mit Standort-TOML und pinnt für den Build einen vollständigen Commit. Controller-UI, Imagebuilder-Auftrag und physische Station sind dabei unterschiedliche Prüfobjekte.

Nach abgeschlossenem Build über die UI **Image, SHA-256-Datei und Manifest** herunterladen. Die API bietet `/api/v1/images/<id>/image`, `/sha256` und `/manifest`; `<id>` stammt aus der vorhandenen Image-Liste. Für ein heruntergeladenes Artefakt mit dem Originalnamen:

```bash
cd /pfad/zum/download
sha256sum -c netcore-STATION-COMMIT-BUILD.img.xz.sha256
xz -t netcore-STATION-COMMIT-BUILD.img.xz
```

Die Platzhalternamen durch die zusammengehörigen Downloadnamen ersetzen. Die Prüfsummendatei referenziert den konkreten Image-Dateinamen. Im Manifest Build-ID, Hostname/Profil, vollständigen Commit, Softwareversionen, Größe und SHA-256 gegen die gewählte Station prüfen.

SHA-256 und `xz -t` prüfen Download beziehungsweise Kompressionsintegrität. Das Manifest führt ausdrücklich `boot_tested = false`; auch der erfolgreiche ARM64-Build oder VM-Dateisystemtest beweist keinen Pi-Start. Das Image anschließend mit dem in der Fachanleitung beschriebenen Verfahren auf das **bewusst ausgewählte Zielmedium** schreiben und dessen Inhalt als ersetzt behandeln.

Auf dem realen Pi Boot, Dateisystem, Netz/SSH, Dienststart, SoapySX-/SXceiver-Erkennung, Standortwerte und die tatsächliche Gateway-Verbindung prüfen. Optionales WLAN/OpenVPN und Heimnetz-Autotoggle gesondert unter den vorgesehenen Netzbedingungen testen. Erst danach folgt der mit echten Funkgeräten dokumentierte Funktionslauf.

## 9. Echte Funkabnahme dokumentieren

Mock-TBS und HTTP-Prüfungen ersetzen keine Funkprüfung. Gerätehersteller, Modell, Firmware und ISSI sowie TBS-Commit, wirksame Standortkonfiguration, Zeitpunkt und Ergebnisse festhalten. Registrierung, Gruppe, Individualruf, SDS, Paketdaten, Restore und Recorder jeweils mit nachvollziehbaren Artefakten prüfen.

Für die vorgegebene Nachweisstruktur:

```bash
cp tests/e2e/on_air_template.json tests/e2e/on_air_evidence.json
editor tests/e2e/on_air_evidence.json
python3 tests/e2e/validate_on_air_evidence.py tests/e2e/on_air_evidence.json --verify-artifacts
```

`--verify-artifacts` prüft die eingetragenen Dateien und Hashes; der Validator führt keine Funkversuche aus. `--require-complete` erst für eine tatsächlich vollständige bestandene Abnahme ergänzen: Es verlangt alle Pflichtprüfungen mit `passed` sowie mindestens zwei Gerätehersteller. Ein `--require-two-vendors`-Schalter existiert hier nicht.

Fehlende Geräte-, Standort- oder Datenpfadtests als `blocked` beziehungsweise `not_run` dokumentieren. Beim Packet Core die noch unvollständige zentralisierte TBS-Anbindung beachten; eine bestandene HTTP-Referenzprüfung nicht als Air-Interface-Erfolg eintragen.

## 10. Sicherung, Betrieb und Wiederanlauf

| Zu sichern beziehungsweise beobachten | Warum |
|---|---|
| Inventar, `/etc/netcore`, wirksame Runtime-Konfiguration und Commit | Grundlage für einen reproduzierbaren Wiederaufbau und den Vergleich nach Updates. |
| Fachdienst-Datenbanken und Backups | Teilnehmer, Gruppen, Rufe, SDS-Idempotenz und andere persistente Zustände haben eigene Speicherorte; Dateien konsistent sichern. |
| Recorder-Archiv und zugehörige Integritäts-/Metadatendateien | Ein Medienexport allein enthält nicht zwingend den vollständigen Betriebs- und Aufbewahrungszustand. |
| KMF-Vault, Master-/Bootstrap-Geheimnisse nach Fachanleitung | Ein KMF-Metadatenbackup enthält nicht automatisch die separat gespeicherten Entschlüsselungsgrundlagen. |
| Deployment-State, Profile und benötigte Image-Manifeste | Erhält Zuordnungen und Artefaktbezug; Discovery-/Quellcaches sind kein Ersatz für Sicherung. |
| Readiness, Queues, Drop-/Fehlerzähler und Speicherplatz | Zeigt funktionale Störungen, auch wenn Prozesse weiterlaufen. |

Sicherungen mit den jeweiligen Dienstschreibvorgängen abstimmen und Wiederherstellung im Labor prüfen. Bei einem Fehler zuerst Dienststatus, letzte Konfigurationsänderung, Abhängigkeiten und korrelierte Events lesen. State-Dateien oder Idempotenzschlüssel nicht zur bloßen Symptombeseitigung löschen.

Mobility-Transfers sind flüchtig; nach Neustart reale Kontexte beider TBS prüfen. Ein Recorder-Ausfall darf den Ruf nicht blockieren, kann aber Tap-Lücken erzeugen. `allow_delete = false` schützt nicht vor automatischer Retention; für bewusst erhaltene Aufnahmen den Hold beachten.

Zum Abschluss tatsächlichen Commit, Ergebnisse, offene Punkte und den nächsten Abnahmeschritt in der [Gesamtroadmap](../roadmaps/gesamtroadmap.md) beziehungsweise im passenden Integrationsnachweis festhalten. Historische Handbücher bleiben im [Archiv](../archive/handbooks) als frühere Fassungen erhalten.
