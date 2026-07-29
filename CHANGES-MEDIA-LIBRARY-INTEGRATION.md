# NetCore-TETRA: Basisstation ↔ Media Library

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

Siehe `Docs/MEDIA_LIBRARY_BASISSTATION_INTEGRATION.md`.

## Bewusst nicht enthalten

- Authentifizierung oder TLS für die M2M-Routen; der aktuelle Stand bleibt OPEN LAB.
- PDFs und GitHub-Workflow-Dateien im ausgelieferten ZIP.
