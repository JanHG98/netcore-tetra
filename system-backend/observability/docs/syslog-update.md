# Observability-Update: Discovery, Syslog und tägliches Share-Archiv

Der Observability-LXC **10.0.1.143** bleibt ein eigener Dienst. Die Deployment-VM
**10.0.1.131:8320** liefert Dienstadressen über ihre bestehende Discovery-API.
Observability gleicht diese alle 30 Sekunden ab und speichert gültige Ziele lokal.
VM-Ausfälle, fehlende Dienste und mehrdeutige Discovery-Einträge entfernen keine
bekannten Ziele. Die Zuordnung erfolgt über den Dienstnamen, nicht über Portnummern.

## Enthalten

- 24 bekannte NetCore-Dienstendpunkte in `observability.example.toml`; vollständiges
  Inventar mit 29 Maschinen in `openlab-hosts.json`, übernommen aus
  `NetCore-Discovery.bat` vom 27.09.2026. PBX, Brew und TBS sind Logquellen;
  für sie werden keine ungeprüften `/metrics`-Endpunkte erfunden.
- Separater rsyslog-Empfänger auf TCP/UDP **514** und RELP **20514**.
  Der Empfang benötigt weder die Deployment-VM noch die NMS-WebUI.
- Lokale JSONL-Segmente, tägliche gzip-Archivierung um **02:15 UTC** (bis 5 Minuten
  zufällige Verzögerung) in
  `/mnt/nfs-share/Logs/NetCore/observability-10.0.1.143/YYYY-MM-DD/`.
  Ein ausgefallener Termin wird nach dem Boot nachgeholt.
- Archivkopien werden per gzip vollständig zurückgelesen und anhand SHA-256
  mit der Quelle verglichen; erst anschließend wird die lokale Quelle entfernt.
  Die Segmente enthalten Host, Quell-IP, Service/Programm, Severity und Zeitstempel.
- Eine begrenzte Vorschau gelangt in die vorhandene Logsuche auf Port **8210**.
  Empfangsquellen, Puffer, Verluste und Archivfehler stehen in derselben Ansicht.
- Optionales Prometheus verwendet die Ziel-Liste von Observability über HTTP SD.
  Kein zweiter Satz fest eingetragener IP-Adressen ist erforderlich.

## Update auf dem Observability-LXC

Voraussetzungen: Debian 12+/Ubuntu 24.04+, Python 3.11+, Rust-Toolchain für den
vorhandenen Workspace und ein als Netzwerkdateisystem eingebundenes Medien-Share.
Der Installer installiert rsyslog samt RELP-Modul. Ein auswertbares vorhandenes
AppArmor-Profil wird um passende Pfade ergänzt und bleibt aktiv.

Als root auf **10.0.1.143**, aus dem Repository auf dem Update-Branch:

```bash
git fetch origin
git switch feature/observability-syslog-discovery
bash system-backend/observability/install/update.sh
```

Bei einer neuen Arbeitskopie kann zuvor geklont werden:

```bash
git clone --single-branch --branch feature/observability-syslog-discovery \
  https://github.com/JanHG98/netcore-tetra.git /opt/netcore-observability-update
cd /opt/netcore-observability-update
bash system-backend/observability/install/update.sh
```

Das Update sichert eine geänderte bestehende TOML als
`observability.toml.before-syslog-<Zeitstempel>`. Alte ausgelieferte localhost-
und `10.0.20.*`-Ziele werden migriert, fehlende NetCore-Ziele ergänzt.
Eigene URLs, Regeln und deaktivierte Targets bleiben bestehen. Auch die alten
Ziele im persistenten NMS-Zustand werden migriert. Das nächste gültige Discovery-
Ergebnis aktualisiert anschließend die verwalteten Dienstadressen.

Ein Target mit `labels = { discovery = "manual" }` bleibt fest eingestellt.
Dieses Label kann auch in der TOML für ein vorhandenes Target gesetzt werden;
`discovery = "auto"` nimmt es wieder in die Synchronisierung auf.
`[discovery].enabled = false` schaltet den gesamten Abgleich aus.

## Vorhandenen Ordner `Logs` anbinden

`/etc/netcore/syslog.json` enthält `archive_mount` und `archive_directory`.
Standard ist der vorhandene NFS-/SMB-Mount `/mnt/nfs-share`, darin `Logs/NetCore`.
Der Installer kennt den NAS-Export nicht und legt keinen neuen Mount an. Bei
abweichendem Mountpfad nur `archive_mount` ändern. Das Archiv prüft auch NFS-/SMB-
Bind-Mounts, die Proxmox in den LXC durchreicht.

Der Benutzer `netcore-observability` braucht Schreib- und Leserechte für
`Logs/NetCore`, einschließlich Unterordner. Bei unprivilegierten LXCs muss die
UID-/GID-Abbildung auf Proxmox/NAS dazu passen. **Keine rekursive Änderung an
Recordings, TTS-Dateien oder dem gesamten Share durchführen.**

Prüfen und einen ersten Lauf auslösen:

```bash
findmnt -T /mnt/nfs-share
id netcore-observability
systemctl start netcore-syslog-archive.service
journalctl -u netcore-syslog-archive.service -n 40 --no-pager
curl -fsS http://127.0.0.1:8210/api/v1/syslog
```

Ohne passenden NFS-/CIFS-Mount verweigert der Archiver jeden Schreibzugriff.
Er erstellt kein lokales Ersatzarchiv unter einem leeren Mountverzeichnis.
Bei NAS- oder Berechtigungsfehlern läuft der Empfänger weiter. Ein hängender
NFS-Aufruf blockiert ausschließlich den separaten Archivprozess.

## Sender auf jedem LXC, der VM und dem Pi

Auf jeder Maschine, deren **journald**-Logs zentral ankommen sollen, als root
aus derselben Arbeitskopie:

```bash
bash system-backend/observability/install/install-log-client.sh
logger -t netcore-log-test 'Test zur Observability'
systemctl status netcore-log-client.service --no-pager
```

Der separate rsyslog-Sender liest journald und sendet per RELP. Sein Ziel kommt
ebenfalls alle 30 Sekunden von der Deployment-VM; der erste Rückfallwert lautet
**10.0.1.143**. Wenn Discovery ausfällt, läuft er mit seiner letzten Konfiguration
weiter. Eine neue Adresse wird vor dem Austausch mit `rsyslogd -N1` geprüft.

Beim ersten Start beginnt er bei neuen Journal-Meldungen. Vorhandene Archive oder
alte Journal-Inhalte werden nicht nachträglich importiert. Danach bleibt der
Journal-Cursor bei Neustarts erhalten. Den älteren optionalen HTTP-Journal-
Forwarder nicht parallel für dieselben Meldungen verwenden.

Der Installer begrenzt das Quell-Journal auf 256 MiB persistent / 64 MiB flüchtig
mit Freiplatzreserve und startet journald neu. Vorhandene **Dateilogs außerhalb
von journald**, beispielsweise `/var/log/syslog` oder eigene Anwendungsdateien,
brauchen weiterhin passende `logrotate`-Regeln. Für deren zusätzliche zentrale
Übertragung ist ein gezielter rsyslog-`imfile`-Input nötig. Einfaches Kopieren
oder Weiterleiten würde diese lokalen Dateien nicht verkleinern.

## Speichergrenzen und Ausfallverhalten

| Bereich | Voreinstellung | Bei Erreichen der Grenze |
|---|---:|---|
| Lokale Originalsegmente | 2 GiB, mindestens 512 MiB frei | Älteste ungearchivierte Segmente verwerfen; Verlustzähler steigt |
| Einzelnes Segment | 16 MiB | Neues Segment |
| Einzelne Meldung | 16 KiB kodiert, höchstens 8.192 Nachrichtzeichen | Kürzung mit `truncated: true`; Vorschau höchstens 4.096 Zeichen |
| Receiver-rsyslog-Queue | ca. 128 MiB plus Segment-Overhead | Begrenzte Queue; keine unbegrenzte Nachlieferung |
| Sender-rsyslog-Queue | ca. 64 MiB plus Segment-Overhead | Begrenzte Queue; bei langem Ausfall ist Verlust möglich |
| UI-Ausgangspuffer | 5.000 Meldungen, separate SQLite-Datenbank | Älteste Vorschauen verwerfen; Originalsegmente bleiben |
| Native Logsuche | 10.000 Meldungen bei Standardmigration | Älteste Einträge entfernen; gzip-Archiv ist separat |
| Archiv dieses Collectors | 90 Tage oder 20 GiB, mindestens 1 GiB Share-Freiplatz | Älteste eigenen Archive entfernen |

Die Budgets betreffen diese Logpipeline; andere Anwendungen, native Metrikdaten
und vorhandene Dateilogs benötigen eigene Limits. Für die Pipeline neben dem
2-GiB-Rohpuffer rund 512 MiB für Queue, SQLite und Metadaten einplanen, dazu NMS-
Zustand und 512 MiB Freiplatzreserve. Der NAS-Hersteller kann zusätzliche Quoten
auf den Ordner `Logs` setzen, damit Aufzeichnungen und TTS genügend Platz behalten.

RELP und persistente Queues verbessern die Zustellung. Bei vollen Puffern,
Datenträgerfehlern oder Stromausfällen ist dies keine Garantie für verlustfreie
oder exakt einmalige Zustellung. UDP ist grundsätzlich unbestätigt. Wiederholte
Übertragung derselben Vorschau-ID wird innerhalb des NMS-Aufbewahrungsfensters
dedupliziert; Raw-Logs können nach einem Crash wiederholte Ereignisse enthalten.
Die Websuche durchsucht die begrenzte Vorschau, nicht sämtliche gzip-Archive.

## Betrieb und Diagnose

```bash
systemctl status netcore-observability netcore-syslog netcore-syslog-preview --no-pager
systemctl list-timers netcore-syslog-archive.timer
curl -fsS http://127.0.0.1:8210/api/v1/discovery
curl -fsS -X POST http://127.0.0.1:8210/api/v1/discovery/sync
curl -fsS http://127.0.0.1:8210/api/v1/targets/prometheus
```

Die neuen Stack-Vorlagen liegen zusätzlich unter `/opt/netcore-observability/stack`.
Bei einem bereits individuell konfigurierten Prometheus dessen Job für
`netcore-services` auf `http_sd_configs` mit URL
`http://127.0.0.1:8210/api/v1/targets/prometheus` umstellen und nach
`promtool check config /etc/prometheus/prometheus.yml` neu laden. Das normale
Observability-Update überschreibt dessen bestehende `/etc/prometheus`-Dateien nicht.
Die vorhandenen Promtail-Dateien sind lediglich Altbestand; diese neue Pipeline
benötigt weder Promtail noch Loki.

Nach Änderungen an `syslog.json` Receiver und Vorschau neu starten; der nächste
Archivlauf liest die Datei selbst neu ein. TCP/UDP 514 und TCP 20514 müssen aus dem
Managementnetz erreichbar sein. Das bestehende OpenLab-Modell ohne Login/TLS bleibt.

## Validierung

```bash
cargo test -p netcore-observability
cargo build -p netcore-observability
python3 tools/check_observability.py
python3 system-backend/observability/tests/observability_reference.py
python3 -m unittest discover -s system-backend/observability/tests -p 'test_*.py' -v
python3 system-backend/observability/tests/native-syslog-smoke.py
# Mit lokal installiertem Playwright/Chromium:
python3 system-backend/observability/tests/native-syslog-smoke.py --browser
```

Die Protokolltests benutzen echte rsyslog-Prozesse auf freien lokalen Ports.
Archivtests injizieren einen temporären Share-Ersatz; auf dem echten NAS bleiben
Mount, Berechtigungen und ein erster Archivlauf als Installationsprüfung nötig.

## Rücknahme

Zuerst `netcore-syslog-archive.timer`, `netcore-syslog-preview` und
`netcore-syslog` stoppen/deaktivieren; auf Sendern entsprechend
`netcore-log-client-discovery.timer` und `netcore-log-client`.
Den vorherigen Observability-Build und die gesicherte TOML wieder installieren.
Der neue optionale Discovery-Status ist für den vorherigen Rust-Deserializer
ein unbekanntes, ignoriertes Feld. Persistente Ziele bleiben deshalb erhalten.
Rohdaten, SQLite, Journal-Cursor und Archive zur Untersuchung aufbewahren.
Den Journald-Drop-in `60-netcore-limits.conf` nur entfernen, wenn die vorherigen
Grenzen wieder gewünscht sind, danach journald neu starten.
