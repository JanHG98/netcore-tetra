# MQTT-Vertrag Phase 4

## Topics

```text
netcore/v1/events/<domain>/<action>
netcore/v1/state/<subject-type>/<subject-id>
netcore/v1/commands/#
netcore/v1/acks/<command-id>
```

Events verwenden `netcore-event-v1`. Commands und Acks verwenden getrennte transportneutrale Verträge:

- `netcore-command-v1`
- `netcore-command-ack-v1`

## Transport und Fachquittung

Ein MQTT PUBACK bestätigt ausschließlich, dass der Broker ein QoS-1-Paket angenommen hat. Es bestätigt **nicht**, dass NetCore den Command erlaubt oder ausgeführt hat. Die fachliche Antwort erscheint auf:

```text
netcore/v1/acks/<command-id>
```

Mögliche Stati:

```text
accepted
executing
rejected
succeeded
failed
duplicate
```

## Retain

- Events: standardmäßig nicht retained.
- Zustände: retained.
- Acks: standardmäßig nicht retained.
- Commands: retained Commands werden standardmäßig abgewiesen.

Der letzte Punkt verhindert, dass ein alter Schaltbefehl nach Broker- oder Gateway-Reconnect erneut wirkt.

## Dubletten

Ein bereits im persistenten Ledger vorhandenes `command_id` wird nie erneut ausgeführt. Stattdessen publiziert der Gateway ein Ack mit Status `duplicate` und verweist im Ergebnis auf den ursprünglichen Terminalstatus.
