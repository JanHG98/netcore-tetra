# Central Media Library TTS

## Zielbild

- Die Basisstation erzeugt keine TTS-Dateien mehr lokal.
- Piper läuft ausschließlich im Media-Library-LXC auf `127.0.0.1:5005`.
- Texteingabe, Stimmen, Vorlagen, Speichern, Vorschau und Freigabe befinden sich in der Media-Library-WebUI.
- Erzeugte Durchsagen werden als normale Media-Library-Assets mit `kind = "tts"` verarbeitet.
- Die Basisstation sieht fertige TTS-Dateien im bestehenden Media-Library-Dateibrowser und lädt sie vor der Aussendung vollständig in ihren lokalen Audiocache.
- TTS-Archive landen unter `/mnt/nfs-share/TTS-Dateien/YYYY/MM/DD`.

## Installation

Siehe `Docs/MEDIA_LIBRARY_CENTRAL_TTS.md`.

## Migration der Basisstation

`install/update-basisstation.sh` entfernt bei Erfolg den lokalen `[tts]`-Abschnitt sowie die alten TTS-NFS-Felder aus `/etc/netcore/config.toml`, legt vorher ein Backup an und deaktiviert `netcore-piper.service` auf der Basisstation.
