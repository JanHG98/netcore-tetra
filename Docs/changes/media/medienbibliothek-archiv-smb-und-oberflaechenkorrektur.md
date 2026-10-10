# Media Library: Archiv-, SMB- und Basisstations-UI-Fix

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: Media Library: Archiv-, SMB- und Basisstations-UI-Fix. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Für heutige Medienfreigabe, Archivierung und Stations-Playout [Media Library](../../services/media-library/README.md), [Piper/TTS](../../services/tts/README.md) und [Quellcode](../../../system-backend/media-library) verwenden. Die Basisstation verarbeitet die vorbereitete Datei über ihren [lokalen Audiopfad](../../../crates/tetra-entities/src/net_audio_player); historische Migrationen/Rechtereparaturen sind vor erneutem Einsatz gegen die tatsächlichen Archivpfade zu prüfen.

Dieser Bericht hält den damaligen Audio-/TTS-/Pagingfehler oder Ausbau fest. Beobachtete Einzelrufe und Logursachen gelten für den dokumentierten Test; daraus folgt keine neue allgemeine Funk-/Dateisystemabnahme.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### Media Library: Archiv-, SMB- und Basisstations-UI-Fix

## Behobene Punkte

- Archivordner und Dateien werden explizit mit `0777` beziehungsweise `0666`
  angelegt, damit das parallel bereitgestellte SMB-Share zugänglich bleibt.
- Der Media-Library-Installer und das Update reparieren bestehende Rechte rekursiv.
- Das bisherige UUID-Archiv wird automatisch nach `Jahr/Monat/Tag` migriert.
- Aufzeichnungen erhalten verständliche Dateinamen aus den Rufmetadaten.
- Die Basisstation zeigt die Quelle korrekt als `MEDIA LIBRARY ONLINE` an.
- Fertige, aber noch nicht freigegebene Medien bleiben sichtbar und können
  vorgehört werden. Die Funkaussendung bleibt bis zur Freigabe gesperrt.

## Neues Recording-Layout

```text
/mnt/nfs-share/Recordings/YYYY/MM/DD/
  HH-MM-SS_Gruppenruf_GSSI-2000_von-ISSI-4010001_12s_6ef665e4_original.wav
  HH-MM-SS_Gruppenruf_GSSI-2000_von-ISSI-4010001_12s_6ef665e4_preview.wav
  HH-MM-SS_Gruppenruf_GSSI-2000_von-ISSI-4010001_12s_6ef665e4_tetra.tacelp
  HH-MM-SS_Gruppenruf_GSSI-2000_von-ISSI-4010001_12s_6ef665e4_metadata.json
```

## Migration

`system-backend/media-library/install/update.sh` stoppt den Dienst kurz, erstellt
vor der ersten Migration eine Sicherung von `state.json`, verschiebt alte
UUID-Archive und aktualisiert die gespeicherten `archive_path`-Werte.

Backup:

```text
/var/lib/netcore-media-library/state.json.pre-archive-layout-v2.bak
```

Die Migration ist idempotent und kann erneut ausgeführt werden:

```bash
python3 /opt/netcore-media-library/bin/migrate-archive-layout.py \
  --config /etc/netcore/media-library.toml
```
