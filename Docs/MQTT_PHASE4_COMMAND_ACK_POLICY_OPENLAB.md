# MQTT Phase 4 – Command, Ack und Policy im OPEN LAB

## Ziel

Phase 4 macht aus der reinen Command-Beobachtung einen kontrollierten, nachvollziehbaren Befehlsweg. Der IoT Gateway validiert `netcore-command-v1`, verhindert Replay, bewertet Policies, führt ausschließlich virtuelle Sandbox-Aktionen aus und veröffentlicht `netcore-command-ack-v1`.

## Bewusste Grenze

Das Netz bleibt offen: keine Logins, Tokens oder TLS. Deshalb werden noch keine realen Aktoren angebunden. Die Standardkonfiguration erlaubt nur:

```text
virtual.relay.set  → virtual_relay / lab-*
virtual.light.set  → virtual_light / lab-*
virtual.button.press → virtual_button / lab-*
```

Alles andere trifft `default_deny`.

## Installation

```bash
cd /opt/netcore-tetra/system-backend/iot-gateway
chmod 755 install/*.sh
./install/update.sh
```

Der Installer ergänzt eine vorhandene Phase-3-Konfiguration automatisch. Broker- und Backend-Adressen bleiben erhalten.

## MQTT-Test

Acks beobachten:

```bash
mosquitto_sub -h 127.0.0.1 -p 1883 -v -t 'netcore/v1/acks/#'
```

Command erzeugen:

```bash
COMMAND_ID=$(python3 - <<'PY'
import uuid
print(uuid.uuid4())
PY
)
NOW=$(date -u +%Y-%m-%dT%H:%M:%SZ)
EXPIRES=$(date -u -d '+30 seconds' +%Y-%m-%dT%H:%M:%SZ)

mosquitto_pub -h 127.0.0.1 -p 1883 \
  -t 'netcore/v1/commands/virtual_relay/lab-relay-01/set' \
  -m "{
    \"schema\":\"netcore-command-v1\",
    \"command_id\":\"${COMMAND_ID}\",
    \"command_type\":\"virtual.relay.set\",
    \"source\":{\"service\":\"openlab-cli\",\"instance\":\"shell-01\",\"actor\":\"jan\"},
    \"requested_at\":\"${NOW}\",
    \"expires_at\":\"${EXPIRES}\",
    \"target\":{\"type\":\"virtual_relay\",\"id\":\"lab-relay-01\"},
    \"payload\":{\"state\":true},
    \"dry_run\":false
  }"
```

Erwartet werden Acks `accepted`, `executing` und `succeeded`.

Virtuellen Zustand prüfen:

```bash
curl -fsS http://IOT-GATEWAY-IP:8240/api/v1/virtual-devices | python3 -m json.tool
```

## Replay-Test

Denselben Command mit identischer `command_id` erneut publizieren. Es muss ein Ack mit Status `duplicate` erscheinen; der virtuelle Aktor wird nicht erneut ausgeführt.

## Default-Deny-Test

Ziel-ID außerhalb des Prefixes `lab-` verwenden:

```text
real-relay-01
```

Erwartet:

```text
status = rejected
reason_code = default_deny
```

## Retain-Test

```bash
mosquitto_pub -r ...
```

Ein retained Command wird mit `retained_command_rejected` abgewiesen. Danach das retained Topic unbedingt leeren:

```bash
mosquitto_pub -h 127.0.0.1 -p 1883 \
  -t 'netcore/v1/commands/virtual_relay/lab-relay-01/set' \
  -r -n
```

## Persistenz

- Roh-Inbox: `command-inbox.ndjson`
- terminales Ledger: `command-ledger.json`
- Audit: `command-audit.ndjson`
- virtuelle Zustände: `virtual-device-state.json`
- ausstehende Acks: `outbox/`
