# Systemprotokolle gezielt lesen

## Live-Log

```bash
sudo journalctl -u tetra.service -f
```

## Aktueller Boot

```bash
sudo journalctl -u tetra.service -b --no-pager
```

## Registrierung und Roaming

```bash
sudo journalctl -u tetra.service -f | \
  grep --line-buffered -iE 'register|location|roaming|attract|affiliate'
```

## Rufe, Carrier und Timeslots

```bash
sudo journalctl -u tetra.service -f | \
  grep --line-buffered -iE 'call|setup|connect|release|floor|carrier|timeslot|TCH'
```

## SDS, Status und HMD

```bash
sudo journalctl -u tetra.service -f | \
  grep --line-buffered -iE 'SDS|U-STATUS|status|HMD|home.mode|emergency'
```

## Audio, Recorder und TTS

```bash
sudo journalctl -u tetra.service -f | \
  grep --line-buffered -iE 'AudioPlayer|record|recording|TTS|Piper|ffmpeg|archive|NFS'
```

## SDR und Timing

```bash
sudo journalctl -u tetra.service -f | \
  grep --line-buffered -iE 'Soapy|SDR|sample|buffer|underflow|overflow|timing|passband|PHY'
```

## Directory und Leitstelle

```bash
sudo journalctl -u tetra.service -f | \
  grep --line-buffered -iE 'directory|control.room|websocket|connect|auth|TLS|broken.pipe'
```

## Zeitfenster exportieren

```bash
sudo journalctl -u tetra.service \
  --since '2026-10-09 12:00:00' \
  --until '2026-10-09 12:15:00' \
  -o short-iso --no-pager > tetra-zeitfenster.log
```

Vor Weitergabe Zugangsdaten, Nachrichtentexte, ISSIs und Standorte prüfen und bei Bedarf schwärzen.

## Weiterführend

[Fehlersuche](fehlersuche.md) · [Inbetriebnahme und Abnahme](inbetriebnahme-und-abnahme.md) · [Betrieb, Wartung und Reparatur](betrieb-wartung-und-reparatur.md)

## Zentrale Bereitstellung und Syslog

Controller, Agent und Builder verwenden unterschiedliche Units: `netcore-deployment.service`, `netcore-discovery.service` und `netcore-image-builder.service`. Für einen zentralen Fehler das passende Journal mit dem Log im Observability-Dienst vergleichen. Ein lokaler Marker ist erst dann ein Weiterleitungsbeleg, wenn er im zentralen Empfänger mit Quelle und Zeitstempel ankommt. Die zentralen [Syslog-Vorlagen](../../system-backend/observability/logging/receiver.rsyslog.conf) ergänzen die jeweiligen Unit-Logs.

## Quellen zur Pflege dieser Seite

[TBS-Logstellen](../../bins/bluestation-bs/src/main.rs) · [Deployment-Units](../../system-backend/deployment-core/systemd/netcore-deployment.service).
