# Z01.4 – Observability-/Syslog-Befund CT136, 2026-10-08

## Tatsächlicher Betreiberbefund

Quelle: bereitgestellte Ausgabe von `pct exec 136`, kein direkter Zugriff des Assistenten.
Quellprüfung an `main@c85d56f91e358f361d738acebdbf46ca306186f6`; der installierte Rust-Commit ist nicht erhoben.
[Strukturierter Befund](evidence/ct136-stocktake-2026-10-08.json).

| Gegenstand | Befund |
| --- | --- |
| Host / Netz | CT136 `Observability`, IPv4 `10.0.1.143/24`, Python3.13.3 |
| Dienst | Discovery-Launcher → `/opt/netcore-observability/bin/netcore-observability`; Dienstbenutzer `netcore-observability` |
| Benutzer / LXC-Abbildung | UID999 / GID989; Map0→100000 über65536 IDs, daher Host-UID100999 / GID100989 |
| HTTP | `10.0.1.143:8210` bereit; `127.0.0.1:8210` Connection refused |
| Logging | Receiver, Preview und Archivtimer aktiv; Aktivität beweist noch keine Zustellung |
| Syslog | `nms_url=http://127.0.0.1:8210`, Collector `observability-10.0.1.143`, Allowlist10.0.1.0/24 +127.0.0.0/8 |
| NAS | nfs4/rw, `/mnt/nfs-share` → `10.0.1.148:/mnt/MassStorage/SRV-M-TBS-01`; Schreiben/fsync/Lesen/Entfernen als Dienstbenutzer999:989 im eigenen Collector-Ordner bestanden |
| Anlagenstatus | 24 Targets up,21 ready;4 firing Alerts;0/4 Stackdienste ready;0 Logs in der bereitgestellten Antwort |

Die bereits vorhandene Standortkonfiguration passt zum Netz. Die Ursache der drei nicht bereiten Targets und der vier Stackdienste ist aus dieser Ausgabe nicht bestimmbar; sie wird nicht durch den Preview-Fix als erledigt erklärt.

## Zugeordneter Fehler und gezielte Korrektur

Der gemeinsame LXC-Installer setzt die konkrete Management-IP als HTTP-Bind.
Der Python-Preview-Vertrag verlangt dagegen genau `http://127.0.0.1:8210`.
Rust nahm bisher nur Verbindungen am konfigurierten Bind an. Die bereitgestellte Ausgabe bestätigt den Widerspruch in der Anlage.

`spawn_http_server` bindet bei einer konkreten IPv4-Adresse zusätzlich `127.0.0.1` auf demselben tatsächlichen Port. Beide Listener bedienen denselben Router und denselben Zustand. Management-Bind, API-Konfiguration, vorhandene TOML, Syslog-URL, Discovery und UI bleiben erhalten. Bei Port0 wird der tatsächlich zugeteilte Port verwendet. Bei0.0.0.0 und genau127.0.0.1 gibt es keinen doppelten Listener. IPv6-Verhalten ist unverändert; dafür wird keine Preview-Kompatibilität behauptet.

Beide Bind-Vorgänge erfolgen vor dem Start der HTTP-Threads. Ein belegter lokaler Port verhindert den Serverstart und gibt den zuvor geöffneten Management-Listener frei. Der Fehler wird nicht als funktionierende Preview verschluckt.

## Regression und Nachweisstufen

- Drei Rust-Tests öffnen echte TCP-Listener: getrennte Management-/Loopback-Adresse auf demselben Port, Wildcard/localhost ohne Doppelbindung, belegter Previewport mit Bereinigung.
- `tests/native-syslog-loopback.py` startet das echte Binary an127.0.0.2:8210. Es lädt die Syslog-Datei über den strikten Validator, schreibt in den echten SQLite-/Rohpuffer, leitet einen Marker über127.0.0.1 weiter und liest denselben Marker über die Management-Adresse. Konfigurationsdatei und API-Bind bleiben unverändert.
- Der bestehende native Smoke-/Browsertest bleibt erhalten. Der neue Test läuft danach im bestehenden `syslog-runtime`-CI-Job.
- Lokale Python-Suite:20 Tests,18 bestanden,2 Wiretests wegen fehlendem rsyslog ausdrücklich übersprungen. Rust-Toolchain fehlt in der lokalen Arbeitsumgebung. Die native CI an Quellcommit `56eb6068465015c4a7e95c6b172a0347202026fb` besteht:20/20 Syslogtests einschließlich realem TCP/UDP/RELP und Reconnect,10/10 Rusttests einschließlich drei neuer Listenerprüfungen, bestehender nativer Browsertest und neue IP-only-/localhost-/SQLite-Regression. [Nachweis](evidence/ct136-preview-ci-2026-10-08.json).

## Betriebsabnahme und Rückweg

Nach erfolgreicher nativer Prüfung den konkreten Build auf CT136 installieren. Beide HTTP-Adressen prüfen und einen eindeutigen Marker über TCP514 senden. Der Marker muss in der NMS-Vorschau erscheinen und nach einem tatsächlichen Archivlauf im gzip-Archiv wiedergefunden werden. Ein erfolgreicher leerer Archivlauf allein ist kein Nachweis.

NAS-Schreibprüfung als Dienstbenutzer mit eigenem temporärem Ordner ausschließlich im Collector-Verzeichnis `/mnt/nfs-share/Logs/NetCore/observability-10.0.1.143`; nur eigene Testdatei lesen und entfernen. Keine rekursiven Rechteänderungen und kein Unmount des gemeinsam genutzten Exports. Ein fehlender Mount wird mit separatem Testzustand geprüft, nicht durch Unterbrechung der Anlage.

Vor einem Austausch den vorhandenen Binarypfad und die Konfigurationsprüfsummen festhalten. Ein Rückweg ersetzt ausschließlich den betroffenen Binarybuild bei gestopptem Observability-Dienst und startet denselben Dienst erneut. Rohsegmente, SQLite, Vorschauzustand, Receiver-Queues, Cursor und NAS-Archive erhalten. Der vorherige Build bringt auch den bekannten lokalen Preview-Fehler zurück; NAS- und RF-Abnahme bleiben eigene Aufgaben.

## Ergänzender Betreiberbefund: NAS und Build-Werkzeuge

Der Betreiber hat den echten NFS-Export über die installierte Mountprüfung geöffnet und als `netcore-observability` (UID999/GID989) eine exklusive eigene Datei im Collector-Verzeichnis geschrieben, per fsync persistiert, gelesen und entfernt. Die abschließende Verzeichnis-fsync besteht. [Nachweis](evidence/ct136-nas-write-2026-10-08.json). Cargo1.97.1 / Rust1.97.1 sind vorhanden; Rootfs10GiB,2.2GiB verwendet,7.9GiB verfügbar. Dieser Nachweis ist ein tatsächlicher Dateizugriff, noch kein gzip-Archivlauf der Logpipeline.

PR #64 ist auf `main@96db88d74d3d9f8ffc9f977e231b7ad9a3f19155` übernommen. Der [gezielte Updateblock](ct136-update.sh) baut ausschließlich das vorhandene Observability-Paket mit einem Buildjob und ersetzt den Binarybuild atomisch. Er prüft Standort/Unit, offene Agentaufträge, unveränderte Konfigurations-/Binary-Prüfsummen und Berechtigungen vor dem Austausch. Der vorhandene Agent heißt `netcore-discovery.service`; die Prüfung umfasst diesen und einen gegebenenfalls vorhandenen Controller. Quellcheckout und Buildzustand liegen getrennt von der Agent-Arbeitskopie. Bei fehlgeschlagenem Start/Prüfung oder gefangenem Signal stellt er den vorherigen Build wieder her und prüft dessen Management-Readiness. SIGKILL, Stromausfall und ein neu eintreffender Deploymentauftrag zwischen Vorprüfung und Austausch sind damit nicht ausgeschlossen; die Vorprüfung ist keine serverseitige Auftragssperre.

Shellsyntax, Verweigerung auf einem anderen Host und ein isolierter Dateisystem-Rücknahmelauf sind geprüft. Der Updateblock ist noch nicht auf CT136 ausgeführt.

**Status:** Bestand und NAS-Dateizugriff im Lab bestätigt, Quellkorrektur übernommen / native CI bestanden; tatsächlicher CT136-Rollout und Logmarker bis zum gzip-NAS-Archiv offen.
