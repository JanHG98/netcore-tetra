# Media Library: getrennte Archivkategorien für Recordings und TTS

Die Media Library verwendet drei getrennte Archivwurzeln:

- `recording` -> `/mnt/nfs-share/Recordings`
- `tts` -> `/mnt/nfs-share/TTS-Dateien`
- sonstige Medien -> `/mnt/nfs-share/Media-Library`

Der Basisstations-Dateibrowser spiegelt diese Kategorien als oberste Ordner und
zeigt darunter jeweils `Jahr/Monat/Tag`.

Beim Media-Library-Update verschiebt `migrate-archive-layout.py` bereits
archivierte TTS-Assets, die noch unter `Recordings` oder `Media-Library` liegen,
automatisch nach `TTS-Dateien`. `state.json` und das jeweilige Metadatenmanifest
werden dabei atomar aktualisiert. Die Migration ist wiederholbar.
