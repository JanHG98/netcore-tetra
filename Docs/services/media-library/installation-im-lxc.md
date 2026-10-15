# Medienbibliothek: Installation im LXC

Empfohlen: Debian-LXC mit 2 vCPU, 2 GiB RAM und ausreichend lokalem Storage.

Aus dem Repository-Hauptverzeichnis, mit installierter Rust-Toolchain:

```bash
sudo apt install build-essential pkg-config ffmpeg
sudo bash system-backend/media-library/install/install.sh
```

Anschließend:

```bash
sudo nano /etc/netcore/media-library.toml
sudo systemctl restart netcore-media-library
curl http://MEDIA-LIBRARY-IP:8230/health/ready
```

Für Archivierung wird das NFS-Share außerhalb des Dienstes nach `/mnt/nfs-share` gemountet. Der Dienst benötigt keinen privilegierten Container und keinen Zugriff auf `/dev`.

Das Installations- und Update-Skript erkennt den Mount und legt automatisch die gemeinsam genutzten Ordner an:

```text
/mnt/nfs-share/
├── Media-Library/
├── Recordings/
└── TTS-Dateien/
```

Für den parallelen SMB-Zugriff werden diese drei Ordner im OPEN-LAB-Modus auf `0777` gesetzt. Der lokale Dienstzustand bleibt mit `UMask=0077` geschützt; erst die ausdrücklich erzeugten Archivkopien erhalten Verzeichnisse `0777` und Dateien `0666`. Ist `/mnt/nfs-share` beim Installieren nicht gemountet, wird nur der Mountpunkt angelegt und eine Warnung ausgegeben; dadurch landen keine Archivdateien versehentlich im lokalen LXC-Dateisystem.

Der gemeinsame Installer setzt `server.bind` auf die erkannte Management-IP. Die
Prüf-URL muss deshalb diese IP verwenden; `127.0.0.1` ist nur bei einem entsprechend
konfigurierten Listener erreichbar. Bei mehreren Interfaces kann
`NETCORE_LXC_IP` beim Installieren die Adresse festlegen.

**Quellabgleich: 15. Oktober 2026.** [Quellcode](../../../system-backend/media-library) · [Konfigurationsvorlagen](../../../system-backend/media-library/config).


## Bestehende Media-Library auf den NAS-Cache-Fix aktualisieren

Auf **Media-Library als root** ausführen. Der Abruf löst `main` einmal in einen
vollständigen Commit auf und verwendet genau diesen Stand für Quellprüfung,
Build und Runtime-Update. Der Runtime-Updater erhält Konfiguration, Piper,
Modelle und Medienbestand und baut vor dem kurzen Dienstneustart.
Eine volle Platte zuerst über das [Logarchiv](../../deployment/logarchiv-auf-nas.md)
entlasten.

```bash
(
  set -Eeuo pipefail
  NC_MEDIA_SRC=$(mktemp -d /var/tmp/netcore-media-update.XXXXXX)
  git -C "$NC_MEDIA_SRC" init -q
  git -C "$NC_MEDIA_SRC" remote add origin https://github.com/JanHG98/netcore-tetra.git
  git -C "$NC_MEDIA_SRC" config http.version HTTP/1.1
  git -C "$NC_MEDIA_SRC" config remote.origin.promisor true
  git -C "$NC_MEDIA_SRC" config remote.origin.partialclonefilter blob:none
  git -C "$NC_MEDIA_SRC" fetch --depth 1 --filter=blob:none origin refs/heads/main
  NC_REF=$(git -C "$NC_MEDIA_SRC" rev-parse FETCH_HEAD)
  printf 'Media-Library Zielcommit: %s\n' "$NC_REF"
  git -C "$NC_MEDIA_SRC" sparse-checkout init --no-cone
  printf '/*\n!/Docs/\n!/wiki/\n' \
    | git -C "$NC_MEDIA_SRC" sparse-checkout set --no-cone --stdin
  git -C "$NC_MEDIA_SRC" checkout --detach "$NC_REF"
  [[ "$(git -C "$NC_MEDIA_SRC" rev-parse HEAD)" == "$NC_REF" ]]
  python3 -B "$NC_MEDIA_SRC/tools/operator/update-netcore-service.py" \
    --service media-library --commit "$NC_REF" --prepare-build-deps --apply
)
```

Die großen Dokumentations- und Wiki-Dateien werden ausgeschlossen; vollständige
Laufzeitquellen und Cargo-Mitglieder bleiben erhalten. Ein Git-/Buildfehler
beendet den Block vor dem Austausch. Die neuen
[Cacheparameter](dateien-und-archivstruktur.md#automatische-lokale-freigabe-und-rückholung)
gelten auch ohne Ergänzung der bestehenden TOML-Datei.
Das NAS bleibt extern unter `/mnt/nfs-share` gemountet.

Anschließend in der WebUI NAS-/Speicherkarten und einen älteren Eintrag prüfen.
Nach dessen Freigabe muss die Vorschau wieder funktionieren und eine freigegebene
Aufnahme erneut über die TBS aussendbar sein. Ein Updatebeleg bestätigt den
installierten Stand; Datei- und Funk-Fachprüfung werden separat bestätigt.
Die Logarchivierung hat ihren eigenen
[Einrichtungsblock](../../deployment/logarchiv-auf-nas.md#auf-media-library-installieren).

Nativ geprüft auf Ubuntu24.04/26.04:39 Rust-Fälle sowie ein echter HTTP-Durchlauf
mit NAS-Ausfall, Rückholung, Wellenform und WAV-Übergabe an eine simulierte
Basisstation. Ein weiterer nativer Fall baut die tatsächlich isoliert abgerufenen
Laufzeitquellen und lädt den Updater. Das ist kein neuer Funkbeleg auf CT138/TBS.
