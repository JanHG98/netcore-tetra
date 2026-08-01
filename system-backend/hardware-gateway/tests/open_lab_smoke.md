# Smoke Test

```bash
curl -X POST -H 'Content-Type: application/json' http://IP:8250/api/v1/telemetry -d '{"device_id":"rack-01","name":"Mobiles Rack","kind":"rack_monitor","metrics":{"temperature_c":32.5,"humidity_percent":45,"supply_voltage_v":12.3},"inputs":{"door_open":false,"water_alarm":false},"outputs":{}}'
curl http://IP:8250/api/v1/devices | python3 -m json.tool
```
