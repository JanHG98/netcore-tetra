# Media Library: echte Gruppen- und Einzelruf-Aussendung über eine Basisstation

## Ziel

Der Aussendeknopf der Media Library startet einen vollständigen Funkauftrag. Die
Media Library übergibt eine freigegebene WAV-Vorschau, GSSI/ISSI und Priorität an
eine ausgewählte NetCore-TETRA-Basisstation. Die TBS übernimmt Download, lokalen
Cache, nativen TETRA-Codec, Rufaufbau, Audioübertragung und geordnetes Rufende.

Damit wird kein zentraler zweiter TETRA-Codec benötigt und es muss keine künstliche
Media-Switch-Session-ID mehr in der Media-Library-WebUI eingetragen werden.

## Voraussetzungen auf der Basisstation

Die Basisstation muss mit `audio-player` gebaut sein und die Media Library als
Audioquelle kennen:

```toml
[media_library]
enabled = true
base_url = "http://10.0.1.154:8230"
audio_source_enabled = true
only_ready = true
only_approved = true
```

Der Audio Player muss aktiv sein. Die konkreten Werte können je nach bestehender
Konfiguration erweitert sein:

```toml
[audio_player]
enabled = true
```

Die Dashboard-API der TBS muss vom Media-Library-LXC erreichbar sein, typischerweise
unter `http://10.0.1.22:8080`.

## Media-Library-Konfiguration

```toml
[runtime]
operating_mode = "authoritative"

[playout]
mode = "basisstation"
default_station = "srv-m-tbs-01"
request_timeout_secs = 15
completion_timeout_secs = 900
poll_interval_ms = 500

[[playout.stations]]
id = "srv-m-tbs-01"
name = "SRV-M-TBS-01"
base_url = "http://10.0.1.22:8080"
enabled = true
```

Ist das TBS-Dashboard durch Benutzername und Passwort geschützt, werden dieselben
Zugangsdaten im Stationseintrag ergänzt. Der Media-Library-Worker verwendet damit
den Cookie-Login `/api/login`; HTTP Basic Auth wird nicht vorausgesetzt.

```toml
username = "admin"
password = "GEHEIM"
```

## Ablauf

1. Die WebUI nimmt Asset, Basisstation, Zieltyp, GSSI/ISSI und Priorität an.
2. Die Media Library prüft `ready`, `approved` und eine vorhandene WAV-Vorschau.
3. Der Worker prüft den TBS-Audio-Player über `GET /api/audio/status`.
4. Der Worker sendet `POST /api/audio/play` mit `source_id = media-library` und der Asset-ID.
5. Die TBS lädt die vollständige Vorschau aus der Media Library in ihren Cache.
6. Die TBS kodiert lokal, baut den Ruf auf und sendet die Audio-Blöcke.
7. Die Media Library übernimmt Remote-Job-ID und Fortschritt aus dem TBS-Status.
8. Abbruch, Fehler und Abschluss werden in beiden Systemen geordnet abgebildet.

## Diagnose

Vom Media-Library-LXC:

```bash
curl -sS -o /tmp/tbs-audio-status.json -w 'HTTP %{http_code}\n' \
  http://10.0.1.22:8080/api/audio/status
cat /tmp/tbs-audio-status.json
```

Bei aktivem Dashboard-Login zuerst Cookie beziehen:

```bash
curl -sS -c /tmp/tbs.cookie \
  -H 'Content-Type: application/json' \
  -d '{"user":"admin","password":"GEHEIM"}' \
  http://10.0.1.22:8080/api/login

curl -fsS -b /tmp/tbs.cookie \
  http://10.0.1.22:8080/api/audio/status \
  | python3 -m json.tool
```

Erwartet werden mindestens:

```json
{
  "available": true,
  "state": "idle"
}
```

Während einer Aussendung:

```bash
journalctl -u netcore-media-library.service -f
journalctl -u tetra.service -f
```

Jobs der Media Library:

```bash
curl -fsS http://10.0.1.154:8230/api/v1/jobs?limit=10 \
  | python3 -m json.tool
```

Ein laufender delegierter Job enthält `playout_mode = basisstation`, die gewählte
TBS in `target_node` und nach Annahme eine `remote_job_id`.

## Direkte Media-Switch-Einspeisung

Der Altpfad bleibt erhalten:

```toml
[playout]
mode = "media_switch"
```

Dieser Modus benötigt weiterhin `broadcast_ready = true`, einen gültigen
`audio.tacelp`-Cache und eine bereits vorhandene Media-Switch-Session-ID. Für normale
TTS-/WAV-Durchsagen ist der Basisstationsmodus der vorgesehene Weg.
