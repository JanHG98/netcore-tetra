# LXC-Deployment

Empfehlung für die Laborstufe:

- Debian 13 oder Ubuntu LTS
- 1 vCPU
- 512 MiB bis 1 GiB RAM
- 4 GiB Systemdisk plus Platz für die Outbox
- Management-IP im isolierten NetCore-Netz
- TCP 8240 für WebUI/API
- TCP 1883 für MQTT, falls der lokale Broker verwendet wird

Der LXC benötigt ausgehend Zugriff auf die HTTP-APIs der vier Eventproduzenten.
Bei einem externen Broker benötigt er außerdem ausgehend TCP-Zugriff auf dessen
Port 1883.

Die systemd-Unit schreibt ausschließlich nach
`/var/lib/netcore-iot-gateway`.

## Quellen komfortabel setzen

Nach der Erstinstallation können die vier Backend-Adressen in einem Schritt gesetzt werden:

```bash
cd /opt/netcore-tetra/system-backend/iot-gateway
./install/configure-openlab.sh \
  NODE_GATEWAY_IP \
  MOBILITY_CORE_IP \
  CALL_CONTROL_IP \
  SDS_ROUTER_IP
```

Ein fünftes Argument setzt optional den MQTT-Broker. Ohne fünftes Argument bleibt bei der lokalen Open-Lab-Installation `127.0.0.1` erhalten.
