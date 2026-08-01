# MQTT-Vertrag Phase 3

## Events

Topic:

```text
<prefix>/events/<event_type mit Punkten als Schrägstrichen>
```

Payload: vollständiges `netcore-event-v1` JSON.

Beispiel:

```text
subscriber.route_changed
→ netcore/v1/events/subscriber/route_changed
```

## Zustände

Besitzt das Event ein Subject, wird derselbe Payload zusätzlich retained unter
folgendem Topic veröffentlicht:

```text
<prefix>/state/<pluralisiertes subject.type>/<subject.id>
```

## Gateway-Verfügbarkeit

```text
netcore/v1/state/services/iot-gateway
```

Beim Verbinden wird `online` retained veröffentlicht. Bei ungeordnetem
Verbindungsabbruch veröffentlicht der Broker den MQTT Last Will `offline`.

## Commands

```text
netcore/v1/commands/#
```

Commands sind in Phase 3 nur Eingabematerial für die spätere Phase 4. Sie
werden mit Zeit, Topic und Payload in `command-inbox.ndjson` gespeichert. Es
gibt bewusst noch keinen positiven Ausführungs-Ack.
