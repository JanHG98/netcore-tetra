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

**Quellabgleich: 9. Oktober 2026.** [Quellcode](../../../system-backend/media-library) · [Konfigurationsvorlagen](../../../system-backend/media-library/config).
