# LXC-Deployment

Empfohlen: Debian-LXC mit 2 vCPU, 2 GiB RAM und ausreichend lokalem Storage.

```bash
sudo apt install build-essential pkg-config ffmpeg
sudo system-backend/media-library/install/install.sh
```

Anschließend:

```bash
sudo nano /etc/netcore/media-library.toml
sudo systemctl restart netcore-media-library
curl http://127.0.0.1:8230/health/ready
```

Für Archivierung wird das NFS-Share außerhalb des Dienstes nach `/mnt/nfs-share` gemountet. Der Dienst benötigt keinen privilegierten Container und keinen Zugriff auf `/dev`.

Das Installations- und Update-Skript erkennt den Mount und legt automatisch die gemeinsam genutzten Ordner an:

```text
/mnt/nfs-share/
├── Media-Library/
├── Recordings/
└── TTS-Dateien/
```

Für den parallelen SMB-Zugriff werden diese drei Ordner im OPEN-LAB-Modus auf `0777` gesetzt und die Media-Library-systemd-Unit verwendet `UMask=0000`. Ist `/mnt/nfs-share` beim Installieren nicht gemountet, wird nur der Mountpunkt angelegt und eine Warnung ausgegeben; dadurch landen keine Archivdateien versehentlich im lokalen LXC-Dateisystem.
