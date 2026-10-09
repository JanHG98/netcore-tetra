# Media Library: virtuelle Ordnerstruktur in der Basisstation

Die Audio-Zentrale der Basisstation zeigt die zentrale Media Library nicht mehr als flache Asset-Liste, sondern als virtuellen Katalog:

```text
Media Library/
└── 2026/
    └── 07 – Juli/
        └── 29/
            └── 21-39-15_Gruppenruf_GSSI-15201_von-ISSI-5102_12s_6ef665e4.wav
```

Die Ordner werden aus `archive_path` des Media-Library-Assets übernommen. Ist ein Asset noch nicht archiviert, verwendet die Basisstation `source_metadata.recorded_at`, ersatzweise `created_at`.

Die angezeigte Datei ist weiterhin ein Media-Library-Asset. Vorschau und Aussendung laden die freigegebene WAV-Vorschau anhand der Asset-ID vollständig in den lokalen Cache; die Basisstation liest nicht direkt aus dem NFS-Share.

## Update

```bash
cd /opt/netcore-tetra
sudo ./install/update-basisstation.sh
```

Danach Browser-Cache umgehen (`Strg+F5`) und in der Audio-Zentrale die Quelle **Media Library** auswählen.
