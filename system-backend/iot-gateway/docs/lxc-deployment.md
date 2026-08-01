# LXC-Deployment Phase 4

```bash
cd /opt/netcore-tetra/system-backend/iot-gateway
chmod 755 install/*.sh
./install/update.sh
```

Der Dienst bleibt auf TCP 8240. Ein bereits installierter Mosquitto bleibt unverändert. Das Update führt `migrate-phase4-config.sh` aus und ergänzt ausschließlich fehlende Phase-4-Abschnitte und Storage-Schlüssel.

Danach:

```bash
systemctl status netcore-iot-gateway --no-pager --full
journalctl -u netcore-iot-gateway -n 100 --no-pager
ss -ltnp | grep 8240
```
