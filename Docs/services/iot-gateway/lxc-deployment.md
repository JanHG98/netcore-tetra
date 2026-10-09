# LXC-Deployment Phase 5

```bash
cd /opt/netcore-tetra/system-backend/iot-gateway
chmod 755 install/*.sh
./install/update.sh
```

Der Dienst bleibt auf TCP 8240. Ein vorhandener Mosquitto bleibt beim Update unverändert. `migrate-phase5-config.sh` ergänzt nur fehlende Abschnitte, Storage-Schlüssel und Standard-Policies.

Danach:

```bash
systemctl status netcore-iot-gateway --no-pager --full
journalctl -u netcore-iot-gateway -n 100 --no-pager
```
