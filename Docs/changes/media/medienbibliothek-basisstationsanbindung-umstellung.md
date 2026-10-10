# NetCore-TETRA: Basisstation ↔ Media Library

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: NetCore-TETRA: Basisstation ↔ Media Library. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Für heutige Medienfreigabe, Archivierung und Stations-Playout [Media Library](../../services/media-library/README.md), [Piper/TTS](../../services/tts/README.md) und [Quellcode](../../../system-backend/media-library) verwenden. Die Basisstation verarbeitet die vorbereitete Datei über ihren [lokalen Audiopfad](../../../crates/tetra-entities/src/net_audio_player); historische Migrationen/Rechtereparaturen sind vor erneutem Einsatz gegen die tatsächlichen Archivpfade zu prüfen.

Dieser Bericht hält den damaligen Audio-/TTS-/Pagingfehler oder Ausbau fest. Beobachtete Einzelrufe und Logursachen gelten für den dokumentierten Test; daraus folgt keine neue allgemeine Funk-/Dateisystemabnahme.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### NetCore-TETRA: Basisstation ↔ Media Library

Diese Variante ergänzt die bidirektionale Media-Library-Anbindung.

## Enthalten

- Fertige Basisstations-Recordings werden per HTTP bei der Media Library angemeldet.
- Die Media Library lädt die WAV selbst von einer schmalen Export-Route der Basisstation.
- Wiederholte Meldungen sind über `source + source_reference` idempotent.
- Fehlgeschlagene oder unterbrochene Imports werden erneut versucht.
- Die Media Library archiviert fertige Recordings automatisch unter `/mnt/nfs-share/Recordings`.
- TTS-Medien können automatisch unter `/mnt/nfs-share/TTS-Dateien` archiviert werden.
- Allgemeine Assets bleiben unter `/mnt/nfs-share/Media-Library`.
- Die Audio-Zentrale zeigt die Media Library als zusätzliche Quelle an.
- Freigegebene Assets werden vor dem Aussenden vollständig in den lokalen Basisstationscache geladen.
- Während der Funkaussendung gibt es kein Live-Streaming über HTTP oder NFS.
- Media-Library-Installer und Update-Skript bereiten die drei gemeinsamen NFS-/SMB-Ordner vor.
- Die Media-Library-systemd-Unit verwendet für das OPEN LAB `UMask=0000` und besitzt gezielte Schreibfreigaben für alle drei Archivpfade.

## Konfiguration und Rollout

Siehe `Docs/changes/media/medienbibliothek-basisstationsanbindung.md`.

## Bewusst nicht enthalten

- Authentifizierung oder TLS für die M2M-Routen; der aktuelle Stand bleibt OPEN LAB.
- PDFs und GitHub-Workflow-Dateien im ausgelieferten ZIP.

## Parser-/Deployment-Fix

- `[media_library]` ist als regulärer Top-Level-Abschnitt im strikten TOML-Parser registriert.
- Zwei Regressionstests verhindern, dass der Abschnitt künftig wieder als unbekannt abgewiesen wird.
- `install/update-basisstation.sh` baut aus genau diesem entpackten Paket und ersetzt die Binary,
  die von systemd tatsächlich ausgeführt wird. Damit wird nicht versehentlich nur
  `/usr/local/bin/bluestation-bs` aktualisiert, während die Unit eine ältere Kopie an einem
  anderen Pfad startet.
- Das Update-Skript sichert die alte Binary und rollt bei einem fehlgeschlagenen Dienststart zurück.
