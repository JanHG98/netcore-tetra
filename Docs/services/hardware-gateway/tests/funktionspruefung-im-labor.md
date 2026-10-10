# Hardwareüberwachung: Funktionsprüfung im Labor

Die HTTP-Beispiele verwenden synthetische Telemetrie. `IP` durch die Managementadresse des Hardware-Gateways ersetzen. Ein gespeicherter Gerätewert belegt den Ingress und die Anzeige, keine echte Sensor- oder Aktorfunktion.


```bash
curl -X POST -H 'Content-Type: application/json' http://IP:8250/api/v1/telemetry -d '{"device_id":"rack-01","name":"Mobiles Rack","kind":"rack_monitor","metrics":{"temperature_c":32.5,"humidity_percent":45,"supply_voltage_v":12.3},"inputs":{"door_open":false,"water_alarm":false},"outputs":{}}'
curl http://IP:8250/api/v1/devices | python3 -m json.tool
```

**Quellabgleich: 9. Oktober 2026.** [Quellcode](../../../../system-backend/hardware-gateway) · [Konfigurationsvorlagen](../../../../system-backend/hardware-gateway/config).
