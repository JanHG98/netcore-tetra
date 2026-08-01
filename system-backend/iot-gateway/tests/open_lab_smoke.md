# Phase 4 OPEN-LAB-Smoke-Test

> Keine Anmeldung, keine Tokens und kein TLS. Nur im isolierten Labornetz verwenden.
> Phase 4 schaltet ausschließlich virtuelle Ziele mit einer `lab-`-Kennung.

## Dienst und Broker

```bash
curl -fsS http://IOT-GATEWAY-IP:8240/health/live
curl -i   http://IOT-GATEWAY-IP:8240/health/ready
curl -fsS http://IOT-GATEWAY-IP:8240/api/v1/status | python3 -m json.tool
curl -fsS http://IOT-GATEWAY-IP:8240/api/v1/policies | python3 -m json.tool
curl -fsS http://IOT-GATEWAY-IP:8240/api/v1/virtual-devices | python3 -m json.tool
```

Broker-Ausgabe einschließlich Acks beobachten:

```bash
mosquitto_sub -h IOT-GATEWAY-IP -p 1883 -v -t 'netcore/v1/#'
```

## Erlaubten virtuellen Relaisbefehl senden

```bash
COMMAND_ID=$(python3 -c 'import uuid; print(uuid.uuid4())')
REQUESTED_AT=$(date -u +%Y-%m-%dT%H:%M:%SZ)
EXPIRES_AT=$(date -u -d '+30 seconds' +%Y-%m-%dT%H:%M:%SZ)

mosquitto_pub -h IOT-GATEWAY-IP -p 1883 \
  -t 'netcore/v1/commands/virtual_relay/lab-relay-01/set' \
  -m "{
    \"schema\":\"netcore-command-v1\",
    \"command_id\":\"${COMMAND_ID}\",
    \"command_type\":\"virtual.relay.set\",
    \"source\":{\"service\":\"manual-test\",\"instance\":\"shell-01\"},
    \"requested_at\":\"${REQUESTED_AT}\",
    \"expires_at\":\"${EXPIRES_AT}\",
    \"target\":{\"type\":\"virtual_relay\",\"id\":\"lab-relay-01\"},
    \"payload\":{\"state\":true},
    \"dry_run\":false
  }"
```

Erwartet werden auf `netcore/v1/acks/${COMMAND_ID}` die Zustände
`accepted`, `executing` und `succeeded`. Zusätzlich entsteht ein retained
Zustand auf:

```text
netcore/v1/state/virtual_relays/lab-relay-01
```

API kontrollieren:

```bash
curl -fsS http://IOT-GATEWAY-IP:8240/api/v1/commands/${COMMAND_ID} \
  | python3 -m json.tool
curl -fsS http://IOT-GATEWAY-IP:8240/api/v1/virtual-devices \
  | python3 -m json.tool
```

## Duplikatschutz

Denselben MQTT-Publish mit derselben `command_id` erneut senden. Erwartet:

```text
status = duplicate
reason_code = duplicate_command_id
```

Der virtuelle Aktor darf nicht erneut ausgeführt werden.

## Default-Deny prüfen

Zielkennung absichtlich außerhalb des Lab-Namensraums setzen:

```json
"target":{"type":"virtual_relay","id":"real-relay-01"}
```

Erwartet:

```text
status = rejected
reason_code = default_deny
```

## Retained-Replay-Schutz prüfen

Einen neuen gültigen Befehl mit `mosquitto_pub -r` senden. Erwartet:

```text
status = rejected
reason_code = retained_command_rejected
```

Damit kann ein alter Schaltbefehl nach Broker- oder Gateway-Neustart nicht
ungefragt erneut ausgeführt werden.
