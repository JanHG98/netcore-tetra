# IoT-Anbindung: Installation im LXC

Für eine bestehende Installation, mit Rust-Toolchain für den Installationsbenutzer:

```bash
cd /opt/netcore-tetra/system-backend/iot-gateway
sudo bash install/update.sh
```

Für eine Neuinstallation aus derselben Arbeitskopie `sudo bash install/install.sh` verwenden.

Der Dienst bleibt auf TCP 8240. Ein vorhandener Mosquitto bleibt beim Update unverändert. `migrate-phase5-config.sh` ergänzt nur fehlende Abschnitte, Storage-Schlüssel und Standard-Policies.

Danach:

```bash
systemctl status netcore-iot-gateway --no-pager --full
journalctl -u netcore-iot-gateway -n 100 --no-pager
```

**Quellabgleich: 9. Oktober 2026.** [Quellcode](../../../system-backend/iot-gateway) · [Konfigurationsvorlagen](../../../system-backend/iot-gateway/config).
