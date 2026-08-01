# OPEN-LAB-Smoke-Test

```bash
curl -fsS http://IOT-GATEWAY-IP:8240/health/live
curl -i   http://IOT-GATEWAY-IP:8240/health/ready
curl -fsS http://IOT-GATEWAY-IP:8240/api/v1/status | python3 -m json.tool
curl -fsS http://IOT-GATEWAY-IP:8240/api/v1/sources | python3 -m json.tool
curl -fsS http://IOT-GATEWAY-IP:8240/api/v1/topics | python3 -m json.tool
```

Broker-Ausgabe beobachten:

```bash
mosquitto_sub -h IOT-GATEWAY-IP -p 1883 -t 'netcore/v1/#' -v
```

Testpublish:

```bash
curl -fsS -X POST -H 'Content-Type: application/json' \
  -d '{"payload":{"message":"Hallo MQTT"}}' \
  http://IOT-GATEWAY-IP:8240/api/v1/test/publish \
  | python3 -m json.tool
```

Command nur beobachten:

```bash
mosquitto_pub -h IOT-GATEWAY-IP -p 1883 \
  -t 'netcore/v1/commands/test/button' \
  -m '{"command_id":"test-1","requested_state":true}'

curl -fsS http://IOT-GATEWAY-IP:8240/api/v1/commands \
  | python3 -m json.tool
```

Der Command muss `observed_only` anzeigen und darf keine Aktion auslösen.
