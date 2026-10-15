# Logs automatisch auf dem Archiv-NAS ablegen

Stand: 15.10.2026. Betreiberauftrag Z01.4: Media-Library CT138.

Die Messung auf Media-Library zeigt 22 GB physisch belegte Logs auf ZFS. Davon hat
`/var/log/syslog.1` 316805205418 logische Bytes, `syslog.3.gz` 3311426550 Bytes
und das aktive `syslog` 1204001875 Bytes. Das Journal belegt nur 49,8 MB,
die Medien 20 MB. Die bisherige rsyslog-Regel rotiert wöchentlich. Die letzten
20 Zeilen enthalten einzelne HTTP-Broken-Pipe-Warnungen und normale Meldungen;
sie belegen noch nicht die historische Quelle des riesigen Backlogs.

## Ablage und automatische Freigabe

Der Operator `tools/operator/archive-netcore-logs.py` installiert zwei unabhängige
systemd-Timer. Jede Minute prüft logrotate die vorhandene rsyslog-Regel mit
`maxsize 64M`. Feste Namen wie `syslog-20261015-223000` bleiben während langer
NAS-Übertragungen unverändert. Die globalen Ubuntu-Vorgaben für `su` und `create` werden vor dem Distributions-`include` übernommen und explizit in die eigenständige Regel geschrieben. Bereits in der rsyslog-Regel gesetzte Vorgaben bleiben erhalten; ohne globale Vorgaben gelten `su root root` und `create` mit den Originalattributen. Damit funktionieren auch gruppenbeschreibbare `/var/log`-Verzeichnisse. Das Distributionsskript zum Wiederöffnen der
aktiven Logs bleibt erhalten; `copytruncate` wird abgelehnt. `rotate -1` ohne
`maxage` bewahrt noch nicht übertragene Rotationen.

Die Archivierung legt einen eigenen Hostordner an:

```text
/mnt/nfs-share/Logs/Media-Library/2026/10/15/
```

Jahr, Monat und Tag folgen der letzten Änderung der geschlossenen Datei in
`Europe/Berlin`. Eine Datei kann Meldungen aus mehreren Tagen enthalten.
Unterordner unter `/var/log` bleiben darunter erhalten. Ein langer Dateiname
wird auf 100 Zeichen begrenzt und um den vollständigen SHA-256 ergänzt.
Textlogs werden direkt auf dem NAS mit gzip komprimiert. Bereits komprimierte
Rotationen bleiben bytegenau erhalten; binäre Journale behalten das Format
`.journal`, damit `journalctl --file …` sie öffnen kann.

Die lokale Freigabe erfordert einen tatsächlich eingehängten NFS-/SMB-Pfad,
eine unveränderte reguläre Quelldatei ohne offene Prozess-FDs, eine vollständig
geschriebene und synchronisierte NAS-Datei, deren unabhängiges Wiederlesen den
SHA-256 und die Länge der Quelle bestätigt, sowie einen lesbaren JSON-Beleg.
Aktive Logs, Symlinks und fremde Dateisysteme unter `/var/log` werden ausgelassen.
Bei fehlendem NAS, voller Freigabe, geänderten Dateien oder fehlgeschlagenen
Prüfungen bleibt die Quelle lokal erhalten. Ein neuer Versuch erzeugt keine
zusätzliche riesige Teildatei für dieselbe Quelle. Im NAS werden Dateien mit
0640 und neue Verzeichnisse mit 0750 angelegt; vorhandene Freigaberechte bleiben
maßgeblich.

Die 317-GB-Datei braucht zwei vollständige Leseläufe: Übertragen/Komprimieren
und unabhängiges Prüfen. Fortschrittsmeldungen erscheinen alle fünf Sekunden.
Das läuft als Hintergrunddienst mit eigenem Prozesslock. Die unabhängige
Rotation läuft währenddessen weiter. Ein SSH-Abbruch beendet die Archivierung
nicht. Der Timer wiederholt fehlgeschlagene Durchläufe. Bei NAS-Ausfall können
geschlossene Logs lokal weiter anwachsen; der Fehler ist im Dienststatus und im
letzten Bericht sichtbar. Journalds vorhandene Aufbewahrung wird nicht geändert.

## Auf Media-Library installieren

Als root auf **Media-Library**, nicht auf der Deployment-VM, ausführen. Den
vollständigen geprüften Commit aus dem Übergabebeleg verwenden. Der Download
betrifft nur das kleine Python-Programm; weder Git-Klon noch Rust-Build sind
nötig.

```bash
(
  set -Eeuo pipefail
  NC_REF=a3323cd40e3d8b2276cd80ba7139545a3a470e73
  NC_LOG_SRC=$(mktemp /tmp/netcore-log-archive.XXXXXX.py)
  curl --http1.1 --fail --show-error --retry 3 \
    "https://raw.githubusercontent.com/JanHG98/netcore-tetra/$NC_REF/tools/operator/archive-netcore-logs.py" \
    -o "$NC_LOG_SRC"

  # Erst Plan, Mount und Kandidaten anzeigen; danach genau diesen Plan aktivieren.
  python3 -B "$NC_LOG_SRC" --install \
    --archive-root /mnt/nfs-share/Logs
  python3 -B "$NC_LOG_SRC" --install \
    --archive-root /mnt/nfs-share/Logs --apply

  systemctl list-timers 'netcore-log-*' --no-pager
  journalctl -u netcore-log-archive.service -n 12 --no-pager -o cat
)
```

`phase=installed` und `archive_status=scheduled` bestätigen die Einrichtung,
noch nicht den Abschluss der großen Übertragung. Die Archivierung startet
sofort im Hintergrund. Ihr tatsächlicher Abschluss steht im Dienststatus und
im Bericht.

## Fortschritt und tatsächliche Entlastung prüfen

```bash
journalctl -fu netcore-log-archive.service -o cat
```

Strg+C beendet nur die Anzeige. Nach `phase=completed` prüfen:

```bash
systemctl show netcore-log-archive.service \
  -p ActiveState -p SubState -p Result -p ExecMainStatus
cat /var/lib/netcore-log-archive/last-run.json
df -hT /
du -xhd1 /var/log
```

`released_bytes` zählt ausdrücklich logische Quellbytes, bei gzip also vor der
Kompression; durch ZFS-Kompression entspricht das nicht dem frei werdenden
Platz. `free_bytes_before/after` und `df` zeigen die tatsächliche verfügbare
Kapazität, während der Rechner weiterarbeitet.

Die Konfiguration liegt in `/etc/netcore/log-archive.json`, das installierte
Programm in `/opt/netcore-log-archive/`. Die ursprüngliche rsyslog-Regel liegt
als `/var/lib/netcore-log-archive/rsyslog-logrotate-*.before` vor.
Zum Pausieren beide Timer stoppen; eine laufende Übertragung muss separat
gestoppt werden. Bis zur bestätigten Freigabe bleibt ihre lokale Quelle bestehen.

## Prüfgrenze

Die native CI prüft reale gzip-/Journaldateien, SHA-256, offene Linux-FDs,
unterbrochene und veränderte Kopien, Symlink-/Mountschutz, idempotente Einrichtung,
echtes logrotate und die systemd-Einheiten auf Ubuntu24.04/26.04.
Die Mountabgrenzung wird mit NFS-/lokalen Mounttabellen geprüft; ein erfolgreicher
NAS-Livedurchlauf auf Jans Anlage und die tatsächlich frei gewordene ZFS-Kapazität
müssen noch durch den Betreiber bestätigt werden.
