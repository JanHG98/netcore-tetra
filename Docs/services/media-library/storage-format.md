# Media-Library-Speicherformat

## Aktive Bibliothek

Die aktive, vom Dienst verwaltete Bibliothek bleibt lokal unter
`/var/lib/netcore-media-library/assets/<asset-id>/`. Dort liegen weiterhin
`original.*`, `preview.wav` und optional `audio.tacelp`.

## Gemeinsames NFS-/SMB-Archiv

Archivierte Dateien werden ohne UUID-Zwischenordner nach Kalenderdatum abgelegt:

```text
/mnt/nfs-share/Recordings/
└── 2026/
    └── 07/
        └── 29/
            ├── 23-39-15_Gruppenruf_GSSI-2000_von-ISSI-4010001_12s_6ef665e4_original.wav
            ├── 23-39-15_Gruppenruf_GSSI-2000_von-ISSI-4010001_12s_6ef665e4_preview.wav
            ├── 23-39-15_Gruppenruf_GSSI-2000_von-ISSI-4010001_12s_6ef665e4_tetra.tacelp
            └── 23-39-15_Gruppenruf_GSSI-2000_von-ISSI-4010001_12s_6ef665e4_metadata.json
```

Für TTS- und normale Medien gelten dieselben Jahres-/Monats-/Tagesordner. Der
Dateiname enthält Zeit, Typ, Titel und eine kurze Asset-ID zur Kollisionsvermeidung.
Die Metadaten-Datei enthält den vollständigen Asset-Datensatz, Prüfsummen und
Dateigrößen.

Archivordner werden explizit mit `0777`, Archivdateien mit `0666` angelegt. Das
ist in der OPEN-LAB-Umgebung beabsichtigt, damit NFS- und SMB-Clients dieselben
Dateien bearbeiten können.
