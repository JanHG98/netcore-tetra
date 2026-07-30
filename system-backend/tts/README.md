# NetCore Media Library Piper TTS provider

Piper läuft zentral im Media-Library-LXC auf `127.0.0.1:5005`. Die Basisstation
betreibt keinen eigenen TTS-Provider mehr. Texte, Stimmen, Vorlagen und das Speichern
als Media-Library-Asset werden in der Media-Library-WebUI auf Port `8230` verwaltet.

## Installierte deutsche Stimmen

Der Installer lädt standardmäßig:

- `de_DE-thorsten-medium`
- `de_DE-thorsten-high`
- `de_DE-karlsson-low`
- `de_DE-pavoque-low`
- `de_DE-thorsten_emotional-medium`

Piper veröffentlicht installierte Modelle über `/voices`. Die Media Library markiert
fehlende konfigurierte Stimmen sichtbar als nicht installiert.

## Installation

Bei einer Media-Library-Installation oder einem Update wird Piper automatisch geprüft
und bei Bedarf eingerichtet:

```bash
sudo ./system-backend/media-library/install/update.sh
```

Eine manuelle Installation ist ebenfalls möglich:

```bash
cd system-backend/tts
sudo \
  SERVICE_USER=netcore-media-library \
  SERVICE_GROUP=netcore-media-library \
  VOICE_DIR=/var/lib/netcore-media-library/piper \
  TTS_CACHE=/var/lib/netcore-media-library/tts/cache \
  TTS_TEMPLATES=/var/lib/netcore-media-library/tts/templates \
  ./install-piper.sh
```

Zusätzliche Stimmen:

```bash
sudo \
  SERVICE_USER=netcore-media-library \
  VOICE_DIR=/var/lib/netcore-media-library/piper \
  ./install-extra-voices.sh
```

## Ablage

```text
/var/lib/netcore-media-library/piper          Piper-Modelle
/var/lib/netcore-media-library/tts/cache      Provider-Arbeitsdaten
/var/lib/netcore-media-library/tts/templates  zentrale TTS-Vorlagen
/mnt/nfs-share/TTS-Dateien/YYYY/MM/DD         archivierte TTS-Assets
```

## Gesundheitsprüfung

```bash
systemctl status netcore-piper --no-pager
curl -fsS http://127.0.0.1:5005/voices
curl -fsS http://127.0.0.1:8230/api/v1/tts/status
```

Synthesetest:

```bash
curl -fsS \
  -H 'Content-Type: application/json' \
  -d '{"text":"Achtung. Dies ist eine Testdurchsage.","voice":"de_DE-karlsson-low","length_scale":1.0526}' \
  http://127.0.0.1:5005/synthesize \
  -o /tmp/netcore-tts-test.wav
file /tmp/netcore-tts-test.wav
```

Der Provider bleibt auf Loopback gebunden. Der Zugriff erfolgt ausschließlich über
die Media-Library-API und deren WebUI.
