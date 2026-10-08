# Z01.4: erster Anlagenpilot und SQLite-Korrektur

Stand: 08.10.2026, Europe/Berlin. Quellstand des Anlagenpilots: `c45a2ec1f5b7cdcfb766a2d81e8901b65f3dbacf` (PR #62, main). Korrekturcommit: `31829fc920e751c3d59acb78fc0157814ca0beb7` (PR #63), übernommen mit `main@ad7dd054730cfb38930ba41689369f4deb9bfb90`. Betreiberbefunde stammen aus den bereitgestellten Konsolenausgaben; die Assistenz hat keinen direkten Zugriff auf die Anlage.

## Übernahme und CI

PR #62 ist übernommen. Am Mergecommit sind diese main-Workflows abgeschlossen und erfolgreich:

- [Deployment inventory and source gate](https://github.com/JanHG98/netcore-tetra/actions/runs/37696847358).
- [OpenLab deployment and discovery](https://github.com/JanHG98/netcore-tetra/actions/runs/37696847450).
- [Warning service](https://github.com/JanHG98/netcore-tetra/actions/runs/37696847442).

Am Korrekturcommit 31829fc sind auch alle drei PR-Workflows erfolgreich abgeschlossen: [Deployment-/Quellgate](https://github.com/JanHG98/netcore-tetra/actions/runs/37766382339), [Deployment/Discovery](https://github.com/JanHG98/netcore-tetra/actions/runs/37766382241) und [NetCore service WebUIs](https://github.com/JanHG98/netcore-tetra/actions/runs/37766382141). PR #63 ist gemerged. Diese Ergebnisse bestätigen den genannten Quellstand, nicht die nachfolgende Anlagenprüfung.

## Tatsächlich beobachtete Anlagenbefunde

| Prüfung | Nachweis / Ergebnis | Grenze |
| --- | --- | --- |
| Deployment-VM119 | Ubuntu26.04.1, Python3.14.4, x86_64/KVM, 8CPU/8GiB, `10.0.1.131`; Deployment- und Imagebuilder-Units installiert und gestartet | Host-/Unitbefund, kein vollständiger Imagebuild |
| Controllerupdate / Git-Pinning | VM-Installer aus c45a2ec; vorhandene Seeds und Bindings erhalten; Checkjob `e326e58f15b04e7985ba52af392d3026` succeeded, Commit exakt c45a2ec | Controllerupdate, kein Flottenrollout |
| Isolierter Agent CT150 | Ubuntu24.04, `z014-test-hw`, DHCP derzeit `10.0.1.191`, API8321; DNS, Route und Agent-Readiness bestanden | DHCP-Adresse nicht dauerhaft reserviert |
| Rollenisolierung | `managed_services=[]`; Manifest kündigt keine Dienste an; Hardware-Pilot manuell auf diesen Agent geplant | Verhindert Dienstankündigungen, begrenzt nicht generell erlaubte Installationsaufträge |
| Hardware-Neuinstallation | Controllerjob `b6c2fe6d36214030b20e68d0fe659147`, Remotejob `d31da0151d8644139cb1a55c137d683a`: succeeded, Commit c45a2ec, live/ready true | HTTP-/Installationsabnahme; kein MQTT-, Aktor- oder RF-Nachweis |
| Hardware-Wiederholungsupdate | Controllerjob `5c21cacdf0e14fc4935f6820ad9284e0`, Remotejob `f133536132a441eeac5f84240f27ca49`: succeeded, Commit c45a2ec, live/ready true | Derselbe Commit, kein Versionswechsel des Hardware-Dienstes |
| Konfigurationserhalt | Original-TOML-SHA256 `ff5bcb8b78386e810e1ec47a6c8907080b57cadff484a5d96466a8a80e997264` vor/nach Update gleich; Original und Runtime-TOML `heartbeat_timeout_secs=37`, `outputs_enabled=false`; Betreiber-Skript meldet PASS | Keine physischen Ausgänge aktiviert |
| Installation der Agentkorrektur | Betreiber installiert 31829fc auf CT150; Quellvergleich und Hardware-Konfigurationsprüfung durchlaufen, Management-Readiness true nach Neustart | Nachfolgende tatsächliche Auftragsprüfung siehe Negativ-/Recovery-Nachweis |
| Negative Readiness | Lokaler Agentjob `b094841cf2e348c48e842182c902c5fb`: erwartetes failed; normaler Live-Endpunkt weiter erreichbar; Marker commit leer / state installing / previous und requested exakt c45a2ec | Absichtlich nicht vorhandener Ready-Pfad, kein realer MQTT-Abhängigkeitenausfall; direkte Agentprüfung |
| Recovery nach negativer Readiness | Lokaler Agentjob `3bdd68552b8c4e1e99b060a90b027978`: succeeded / live / ready; Marker installed mit c45a2ec; ursprüngliche Agent-TOML wiederhergestellt | Wiederherstellung der Prüfkonfiguration und bestätigten Installation, kein Binärrollback / Versionswechsel |
| Konfiguration und Status unter korrigiertem Agent | Hardware-SHA256 unverändert wie oben, Original-/Runtime-Wert37 und outputs false; Job-Polling 0 GET-Fehler / 0 HTTP500 | Zwei allgemeine GET-Fehler in separaten Wiederanlaufprüfungen; aus Ergebnis allein keine konkrete Einzelursache ableiten |
| Temporärer Statusausfall | Controller protokolliert GET-Timeout/HTTP500 und verfolgt dieselbe Remote-ID weiter bis succeeded | Kein erneuter Installations-POST als Fehlerbehandlung; Ursache des dokumentierten HTTP500 siehe unten |
| NAS-Einbindung CT136 | Hostpfad `/mnt/netcore-nfs/SRV-M-TBS-01`, CT-Pfad `/mnt/nfs-share`: tatsächlich nfs4/rw von `10.0.1.148:/mnt/MassStorage/SRV-M-TBS-01` | Kein Dienstbenutzer-Schreibtest, Archiv-/Ausfallnachweis |

## Bestätigter Fehler und Korrektur

Agentjournal am 08.10.2026 um 10:25:51 UTC: GET `/api/v1/jobs/<id>` scheitert in `jobs.py`, `SELECT * FROM jobs WHERE id=?`, mit `sqlite3.OperationalError: database is locked`; Handler antwortet HTTP500. Die Installationsarbeit läuft weiter. Auch der Wiederholungsupdate enthält einen vorübergehenden HTTP500; sein eigener Trace liegt hier nicht vor.

Die Quelle öffnet bisher für Lesen und Log-/Statusschreiben unabhängige SQLite-Verbindungen. Der Connection-Contextmanager beendet Transaktionen, schließt aber die Verbindung nicht. `list()` öffnet zudem eine Abfrage und anschließend pro Job eine weitere Verbindung.

Gezielte Änderung:

- Eigene `db_lock` pro Jobs-Instanz koordiniert ausschließlich kurze Datenbankzugriffe: Öffnen, SQL, Commit/Rollback und Schließen. Der separate Submit-/Queue-Lock bleibt erhalten; Installer und Remote-Polling laufen außerhalb der Datenbanksperre.
- `connect()` schließt die Verbindung im `finally`, auch bei Fehlern; erfolgreiche Transaktionen werden weiter committed, fehlerhafte zurückgenommen.
- `list()` liest alle höchstens100 vollständigen Datensätze in einer Abfrage und dekodiert sie danach.
- Datenbankschema, SQLite-Journalmodus, Job-IDs, Logs und Wiederanlaufregeln werden beibehalten. Externe DB-Schreiber oder Datenträgerfehler werden damit nicht generell beseitigt.

Die früheren BrokenPipe-Tracebacks betreffen Antworten an bereits getrennte Discovery-Clients. Sie sind ein gesonderter Journalbefund; diese Korrektur beansprucht nicht, ihre Ursache oder jede Netzverzögerung zu beheben.

## Prüfung der Korrektur

Neue Regressionen prüfen echte SQLite-/HTTP-Zugriffe:

1. Eine offene EXCLUSIVE-Schreibtransaktion hält die DB gesperrt; echte parallele HTTP-Abfragen von Jobdetail und Liste warten bis zum Commit und liefern anschließend HTTP200 mit dem committed Loginhalt. Ein verkürzter SQLite-Busytimeout macht den ehemaligen Fehler ohne zehn Sekunden Wartezeit reproduzierbar.
2. Verbindungen sind nach Commit und Rollback tatsächlich geschlossen; zurückgenommene Werte bleiben unsichtbar.
3. Vier Statusleser begleiten150 Logschreibvorgänge; der Auftrag wird genau einmal ausgeführt, endet dauerhaft succeeded und behält das100.000-Zeichen-Loglimit.

Mit unveränderter jobs.py aus c45a2ec reproduziert der erste Test HTTP500 / `database is locked`; der zweite bestätigt die fehlende Verbindungsschließung. Mit der Korrektur bestehen beide. Die vollständige Deployment-Suite wird mit `python3 -m unittest discover -s system-backend/deployment-core/tests -v` geprüft; Ergebnis am Korrekturstand: 53 Tests, 52 bestanden, 1 ausdrücklich übersprungen (AF_UNIX in dieser Ausführungsumgebung nicht verfügbar; native CI vorgesehen).

## Tatsächliche Negativ-/Recovery-Abnahme auf CT150

Der Betreiber hat den Helper aus `b2c0208` auf dem installierten Agentfix `31829fc` ausgeführt. Ergebnis **PASS** mit den oben genannten IDs und Markern. Der vollständige bereitgestellte Ergebnisbericht ist unter [evidence/ct150-readiness-2026-10-08.json](evidence/ct150-readiness-2026-10-08.json) abgelegt. Der lokale Originalnachweis liegt unter `/var/tmp/netcore-z014-readiness-c45a2ec1/result.json` auf CT150. Das Helper-PASS erzwingt außerdem vollständigen Hardware-Konfigurationserhalt, Wert37 in Original-/Runtime-TOML, deaktivierte Ausgänge, bytegenaue Rückstellung der Agent-TOML und ein erfolgreiches Recovery mit bestätigtem Marker.

Damit sind die isolierten Agent-Readiness-/Recovery-Prüfungen tatsächlich im Lab bestanden; die SQLite-Korrektur zeigt bei diesen zwei Aufträgen keine Jobstatusfehler. Der vorherige Controllerpilot und diese direkten lokalen Aufträge bleiben getrennte Nachweise. Neue Fehlerweitergabe durch den Controller, realer MQTT-/Aktorbetrieb, ein echter Versionswechsel, Controllerausfall / Wiederkehr und NAS-/Pi-/On-Air-Prüfungen sind damit nicht abgeschlossen.

## Nächster Betriebsnachweis und Rückweg

1. **CT136 erfasst:** Python3.13.3, tatsächliche Units/ExecStart/User, Management-API bereit, localhost:8210 verweigert Verbindungen, Standort-Syslogkonfiguration und nfs4/rw-Einbindung bestätigt. [CT136-Nachtrag](observability-ct136.md). Der anschließende Schreib-/fsync-/Lese-/Entfernungstest als Dienstbenutzer999:989 besteht auf dem echten NFS-Export; gzip-Logarchivierung bleibt separat offen.
2. **CT136-Rollout bestanden:** Betreiberbuild aus3d96a9a, beide API-Adressen bereit, Management-Bind und Originalkonfiguration erhalten. Die lokale HTTP-Annahme ist nativ geprüft und tatsächlich ausgerollt: Shared-LXC-Installer setzt Management-IP, `log_store.py` verlangt exakt `http://127.0.0.1:8210`. Die vorhandene Management-Bindung soll erhalten bleiben und der lokale Preview-Pfad tatsächlich erreichbar werden; `nms_url` nicht auf die Management-IP ändern.
3. NAS-Dateizugriff als Dienstbenutzer besteht. Die TCP-Markerprüfung stoppt in der Vorschau-GET-Phase mit Timeout; der einzelne Marker ist bereits gesendet, der Archivstart noch nicht versucht. Den gespeicherten Lauf `/var/tmp/netcore-ct136-syslog-oggg829q` mit `--resume` bis ins gzip-NAS-Archiv fortsetzen. [Diagnose](evidence/ct136-marker-timeout-2026-10-08.json); der Zustellnachweis und der isolierte Archiv-Fehlerfall bleiben offen.
4. Vollständigen ARM64-NetCore-Imagebuild, echten Pi/SXceiver-Boot und VPN-Wechsel durchführen. Controllerausfall / Wiederkehr, NAS-Ausfall, Controller-Fehlerweitergabe und echter Hardware-Versionswechsel bleiben eigene offene Nachweise.

Der Operatorhelfer ist auf root / Hostname `z014-test-hw`, leeres Dienstmanifest und deaktivierte Hardware-Ausgänge begrenzt. Er legt die ursprüngliche Agent-TOML und Phasen-/Auftragsnachweise privat unter `/var/tmp/netcore-z014-readiness-c45a2ec1` ab, speichert die POST-Absicht vor dem Anlegen jedes Auftrags und wiederholt keinen POST. Bei einer unklaren Antwort oder einem noch offenen Auftrag hält er an; einen bestehenden Teststand überschreibt er nicht. Nach einem bekannten terminalen Negativauftrag stellt er zuerst die Agent-TOML wieder her. Erfolgreiche Abnahme erfordert anschließend einen gültigen Auftrag, den bestätigten Commitmarker und unveränderte Original-/Runtime-Konfiguration mit Wert37. Startup-Verbindungsfehler werden getrennt von Fehlern beim Polling der Job-ID gezählt; ein neuer HTTP500 dort verhindert ein Gesamt-PASS. Syntax und simulierte Zustands-/Fehlerabläufe wurden vorab geprüft; die tatsächliche CT150-Ausführung ist inzwischen wie oben dokumentiert bestanden.

Der Agent-Code kann bei abgeschlossenen Aufträgen über denselben Installer aus c45a2ec zurückgesetzt werden. Die Korrektur benötigt keine Datenbankmigration; Jobs, Logs, Marker, Cache und Hardware-Konfiguration bleiben bestehen. Agent-Neustart mit einem offenen Auftrag ist kein Rückweg: er markiert laufende/queued Jobs als interrupted. Bei `remote_uncertain` zuerst den bestehenden Agentauftrag klären.
