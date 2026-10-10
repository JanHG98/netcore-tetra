# Leitstelle: Installation im LXC

## Voraussetzungen

```bash
apt update
apt install -y build-essential pkg-config curl ca-certificates iproute2
```

Rust für den ausführenden Installationsbenutzer installieren, `main` auschecken und vom Repository-Hauptverzeichnis ausführen. Der Installer legt den Dienstnutzer an:

```bash
sudo bash system-backend/control-room/install/install.sh
```

Konfiguration:

```text
/etc/netcore-control-room/control-room.toml
```

State:

```text
/var/lib/netcore-control-room/control-room.sqlite3
/var/lib/netcore-control-room/operations.json
/var/lib/netcore-control-room/operations.json.bak
```

Nach Anpassung der LXC-Adressen:

```bash
systemctl restart netcore-control-room
curl http://CONTROL-ROOM-IP:9010/health/live
curl http://CONTROL-ROOM-IP:9010/health/ready
```

Der gemeinsame Installer setzt `server.bind` auf die erkannte Management-IP. Die
Prüf-URL muss deshalb diese IP verwenden; `127.0.0.1` ist nur bei einem entsprechend
konfigurierten Listener erreichbar. Bei mehreren Interfaces kann
`NETCORE_LXC_IP` beim Installieren die Adresse festlegen.

**Quellabgleich: 9. Oktober 2026.** [Quellcode](../../../bins/netcore-control-room/src) · [Konfigurationsvorlagen](../../../system-backend/control-room/config).
