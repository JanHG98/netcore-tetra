# Z01.4 – Observability-/Syslog-Befund CT136, 2026-10-08

**Dokumenttyp: datierter Arbeits-, Prüf- und Betreiberbefund.** CT136 ist als konkreter Betreiberbefund mit UID/GID, Mountquelle, APIs und datierten Messwerten dokumentiert. Erfolgreicher NFS-Dateizugriff und ein archivierter TCP-Marker gelten für diesen Pilot, nicht für alle Sender oder reale NFS-Stalls.

Heutiger Einstieg: [Observability-Anleitung](../../services/observability/README.md) · [Syslog-Betrieb](../../services/observability/systemlogs-sammeln-und-archivieren.md) · [Integrationsübersicht](README.md) · [Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md). Quellbeschreibung und tatsächliche installierte Version bleiben getrennt.

## Datierte Originalbefunde

### Z01.4 – Observability-/Syslog-Befund CT136, 2026-10-08

## Tatsächlicher Betreiberbefund

Quelle: bereitgestellte Ausgabe von `pct exec 136`, kein direkter Zugriff des Assistenten.
Quellprüfung an `main@c85d56f91e358f361d738acebdbf46ca306186f6`; der installierte Rust-Commit ist nicht erhoben.
[Strukturierter Befund](evidence/ct136-stocktake-2026-10-08.json).

| Gegenstand | Befund |
| --- | --- |
| Host / Netz | CT136 `Observability`, IPv4 `10.0.1.143/24`, Python3.13.3 |
| Dienst | Discovery-Launcher → `/opt/netcore-observability/bin/netcore-observability`; Dienstbenutzer `netcore-observability` |
| Benutzer / LXC-Abbildung | UID999 / GID989; Map0→100000 über65536 IDs, daher Host-UID100999 / GID100989 |
| HTTP beim ursprünglichen Befund | `10.0.1.143:8210` bereit; `127.0.0.1:8210` Connection refused; nach Update beide bereit (siehe Nachtrag) |
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

Shellsyntax, Verweigerung auf einem anderen Host und ein isolierter Dateisystem-Rücknahmelauf sind geprüft. Der Updateblock ist inzwischen auf CT136 wie im folgenden Nachtrag tatsächlich ausgeführt.

## Tatsächlicher Binary-Update-Nachweis

Der Betreiber hat `ct136-update.sh` aus `3d96a9a4d9d2c0cdc8cd814970b076a551fcbfe2` auf CT136 ausgeführt. Vollständiger Releasebuild des Observability-Pakets in3m26s bestanden; Vorprüfung vor/nach dem Build, Prüfung des alten Binarys und unveränderter Originalkonfiguration bestanden. Der atomische Austausch und Neustart enden mit beiden bereiten HTTP-Adressen und erhaltenem Management-Bind. `/etc/netcore/observability.toml` und `/etc/netcore/syslog.json` bestehen die vollständige SHA-256-Prüfung nach dem Neustart. [Strukturierter Betreiberbefund](evidence/ct136-update-2026-10-08.json).

Der gelesene Previewstatus vom2026-10-08T12:07:14.133121Z meldet `preview_pending=0` und `preview_error=null`. Dies bestätigt keine Markerzustellung, keinen Sender und kein tatsächliches gzip-Archiv. Quellcheckout auf CT136: `/var/tmp/netcore-obs-source.VCkhk2`; vorheriges Binary / Prüfsummen / Buildzustand: `/var/tmp/netcore-obs-update.xDnYZ2`. Eine Rücknahme war bei diesem erfolgreichen Lauf nicht erforderlich.

Der nächste [Markerhelfer](ct136-syslog-acceptance.py) sendet genau einen eindeutigen RFC5424-Marker an den tatsächlichen TCP514-Empfänger, verfolgt ihn über die NMS-Vorschau und prüft nach einem einmal ausgelösten echten Archivdienstlauf denselben Record im eigenen gzip-Archiv. Er benötigt keinen Observability-/Receiver-Neustart. Ein nicht abgeschlossener Archivlauf führt zu STOP mit bestehendem Nachweis, nicht zu einem weiteren Archivstart. Syntax und sechs isolierte Prüfungen am tatsächlichen Helfer bestehen: normaler Ablauf, bereits aktiver Archiver, über60s laufender Auftrag mit erhaltenen Rohdaten, veralteter Unit-Erfolg, falscher gzip-Datensatz sowie abgeschnittener gzip-Footer/CRC. Die Fixtures verwenden echte temporäre Raw-/gzip-/Nachweisdateien; Host, systemd, TCP, HTTP und Mountprüfung sind simuliert. [Prüfbericht](evidence/ct136-marker-helper-validation-2026-10-08.json). Der ursprüngliche echte CT136-Lauf und seine Unterbrechung sind im folgenden Nachtrag dokumentiert; die vollständige Zustell-/Archivabnahme bleibt offen.

**Status:** Bestand, NAS-Dateizugriff und Binary-/API-Update im Lab bestanden; Logmarker über den Empfänger bis zum tatsächlichen gzip-NAS-Archiv noch offen.

## Nachtrag: Markerprüfung bei langsamer Vorschau unterbrochen

Der tatsächliche Lauf mit Helper `8b7bc1e` hat genau einen Marker `Z014-SYSLOG-9266c5bd5685499081a00becb5553559` gesendet und ist beim HTTP-Abruf der Vorschau mit `TimeoutError` gestoppt. Der gespeicherte `failed_phase=preview_pending` wird erst nach erfolgreicher Rückkehr von TCP-`sendall` geschrieben; eine passende Empfangszeile ist damit noch nicht nachgewiesen. Nachweisverzeichnis: `/var/tmp/netcore-ct136-syslog-oggg829q`. Ein Archivstart wurde von diesem Helferlauf noch nicht versucht. [Betreiberdiagnose](evidence/ct136-marker-timeout-2026-10-08.json).

Der spätere Receiverstatus von12:27:03UTC meldet Puffer0, keinen Previewfehler und insgesamt zwei empfangene Records. Beide HTTP-Listener gehören demselben Binaryprozess; Receiver, Preview und Observability laufen. Der gelesene Archiv-Erfolg stammt vom früheren Lauf um02:19UTC. Diese Fakten ersetzen weder die Identität des Markers noch dessen gzip-Nachweis. Die HTTP-Broken-Pipe-Warnungen zeigen geschlossene Clientverbindungen; die Ursache der Verzögerung ist noch nicht durch Laufzeitmessungen bestimmt. Quellprüfung zeigt vollständige Zustandsserialisierung und Dateisynchronisierung unter demselben Mutex, den die Logabfrage benötigt. Das ist eine mögliche Verzögerungsquelle, keine bestätigte Ursachenmessung.

Die Fortsetzung erfolgt mit `ct136-syslog-acceptance.py --resume /var/tmp/netcore-ct136-syslog-oggg829q`. Sie erhält den ursprünglichen Befund als separate History-Datei, vergleicht Originalkonfiguration und Archivlaufidentität, sendet keinen weiteren TCP-Marker und wiederholt ausschließlich vorübergehend fehlgeschlagene GET-Abfragen mit begrenzter Wartezeit und Antwortzeitprotokoll. Ein bereits versuchter oder fremder Archivlauf verhindert einen zusätzlichen Start. Erst der tatsächliche PASS dieses Fortsetzungslaufs schließt diesen Zustellnachweis.

Die Fortsetzung ist isoliert geprüft: sieben Testmethoden mit elf Resume-Szenarien bestehen, einschließlich einer sechs Sekunden langen simulierten GET-Antwort, Timeout/HTTP503-Wiederholung, Gesamtbudgetablauf, veränderter Archividentität und fünf verweigerten unklaren Nachweisen sowie einer real belegten Checkpointsperre. Ein nichtblockierender Lock verhindert parallele Fortsetzungen desselben Nachweises. Acht bestehende Szenarien des frischen Markerablaufs bestehen weiterhin. Reale temporäre Raw-/gzip-/History-Dateien werden geprüft; Host, HTTP/TCP, systemd und Netzmount sind simuliert. [Fortsetzungs-Prüfbericht](evidence/ct136-marker-resume-validation-2026-10-08.json). Diese Prüfung verändert kein Laufzeitbinary und behebt oder bestätigt noch keine Ursache der Anlagenlatenz.

## Nachtrag: echte TCP-/Vorschau-/NAS-Abnahme bestanden

Der Betreiber hat die Fortsetzung aus `c5279c7748ccdd6b96650e4fef2cb1e280e08232` im ursprünglichen Nachweisverzeichnis ausgeführt. Ergebnis am2026-10-08 um12:43:08UTC /14:43:08CEST: **PASS**. Ein TCP-Sendeversuch, eine Fortsetzung und ein neuer Archivstart; kein weiterer Marker. Beide API-Adressen zeigen Syslog-ID `d7b42f8bfb384a3b8613bc04a220828b`. Der eigene Raw-Record ist genau einmal im vollständig durch EOF/CRC geprüften NAS-gzip enthalten; der archivierte lokale Raw-Segmentpfad ist entfernt. [Vollständiger Betreiberbefund](evidence/ct136-syslog-2026-10-08.json).

Der frische Archivdienstlauf als `netcore-observability` endet erfolgreich (12:42:56–12:43:05UTC, neue InvocationID und erhöhter Startzeitstempel); `archived_segments=1`, `error=null`. Archiv: `/mnt/nfs-share/Logs/NetCore/observability-10.0.1.143/2026-10-08/20261008T122337-5111148c4bf844b78d7f1b846f37fa49.jsonl.gz`. SHA-256 des dekomprimierten JSONL: `3d6500514a7a43e90f6276308731cf7bc49e611ff4be312aabfff49b8d8a42e6`. Beide Original-Konfigurations-Prüfsummen sind erhalten.

Die Fortsetzung benötigt sechs erfolgreiche GETs ohne Wiederholungsfehler; maximal4,952s pro Abfrage. Damit ist die Markerzustellung abgenommen, die zugrunde liegende HTTP-Latenz jedoch nicht behoben oder vollständig vermessen. Der Nachweis betrifft den lokalen TCP-Sender und den tatsächlichen NAS-Erfolgspfad; er bestätigt keine Flotten-journald-Sender, RELP-Anlagenabnahme oder echte NFS-Störung.

Als kurzer nächster Fehlerfall prüft [ct136-archive-negative.py](ct136-archive-negative.py) den installierten Archiver mit eigenem temporärem Zustand als tatsächlicher Dienstbenutzer. Eine eigene lokale Fixture-Adresse wird als Archiv-Mount konfiguriert; die reale Mountprüfung muss diese ablehnen, denselben Raw-Record vollständig erhalten und darf kein lokales Ersatzarchiv erzeugen. Produktive Konfiguration, Receiver, Timer und echter NAS-Mount werden dabei nicht geändert. Ein PASS bestätigt diese isolierte Fehlermount-Prüfung, keine physische NFS-Unterbrechung oder Stallsicherheit. Danach folgt die VM119-Imagebuilder-Vorprüfung für den vollständigen ARM64-Build.

Der Negativhelfer ist lokal mit dem tatsächlichen `log_store` und unveränderter `/proc/self/mountinfo`-Prüfung geprüft: erwarteter Mountfehler, bytegleicher versiegelter Raw-Record, Outbox1, null archivierte Segmente und leeres lokales Testziel. Syntax und unabhängige Wiederholung bestehen. Die lokale Identität ist0:0 über einen ausschließlich im Python-Testaufruf verwendeten Funktionsparameter; der Operator-CLI bietet keine Overrides und erzwingt999:989 via `runuser`. Die tatsächliche CT136-Ausführung dieses Fehlerfalls bleibt offen. [Prüfbericht](evidence/ct136-archive-negative-helper-validation-2026-10-08.json).

## Nachtrag: isolierter Fehlermount auf CT136 bestanden

Der Betreiber hat `ct136-archive-negative.py` aus `13923e3daffcb697f7ee7f30e116c587b850b488` ausgeführt: **PASS** am2026-10-08 um13:00:57UTC /15:00:57CEST, tatsächliche Workeridentität999:989. Importiert wurde der installierte `/opt/netcore-observability/logging/log_store.py`, SHA-256 `0f90ec8b924edcabb8944d9914ee5b0af3af886aac18ab5b7f7006ca99e741d4`. [Vollständiger Betreiberbefund](evidence/ct136-archive-negative-2026-10-08.json).

Die echte Mountprüfung verweigert das eigene lokale Verzeichnis mit dem erwarteten `Network share is not mounted`-Fehler. Der versiegelte Raw-Record `fc44ffe813b54369ade5f5aff9f6a8dc` bleibt bytegleich erhalten; vor/nach SHA-256 `143cfbf451f248901790e2f4c85042cdad0c8345ba06a0b66f1ebc93482b0580`. Outbox weiterhin1, null archivierte Segmente, lokales Testziel leer; beide Originalkonfigurationen bytegleich erhalten. Nachweis und Testdaten bleiben unter `/var/tmp/netcore-ct136-archive-negative-3dcor89e`.

Damit bestehen der tatsächlich ausgerollte TCP-/Vorschau-/NAS-Erfolgspfad und der isolierte installierte Archiver-Fehlerpfad. Der Fehlerfall betrifft eine eigene lokale Nicht-Mount-Adresse; er ist keine tatsächliche NFS-Unterbrechung oder Stallsimulation. Der nächste ausführbare Z01.4-Auftrag ist die [Imagebuilder-Vorprüfung auf VM119](imagebuilder-vm119.md), anschließend der vollständige ARM64-Build und physische Pi-/SXceiver-/VPN-Nachweis.
