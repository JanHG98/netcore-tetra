# KMF – API-Beispiele im Labor

Diese Beispiele setzen eine laufende isolierte KMF mit Loopback- oder Wildcard-Listener auf `127.0.0.1:8190` voraus. Nach LXC-Installation stattdessen die tatsächliche WebUI-Adresse aus `/etc/netcore/lxc-network.env` verwenden. Die ausgelieferte Policy steht auf `shadow`: Jobs lassen sich vorbereiten, der Claim bleibt leer. IDs aus den Antworten jeweils in die markierten Platzhalter einsetzen.

## GCK und Node-Profil erzeugen

```bash
curl --fail-with-body http://127.0.0.1:8190/api/v1/keys \
  -H 'Content-Type: application/json' \
  -d '{"kind":"GCK","scope":"group","scope_value":"15501","label":"Lab GCK 15501","key_bytes":16,"notes":"lab"}'

curl --fail-with-body http://127.0.0.1:8190/api/v1/nodes \
  -H 'Content-Type: application/json' \
  -d '{"node_id":"tbs-04010001","display_name":"Lab TBS 1","notes":"lab"}'
```

Für einen CCK stattdessen `"kind":"CCK","scope":"network","scope_value":null` verwenden. Schlüsselantworten enthalten keine Rohschlüssel. Das Node-Profil nennt den serverseitigen Bootstrap-Pfad; das Secret bleibt in der dortigen Datei.

## Schlüssel für Zustellung vorbereiten

Ein neu erzeugter Schlüssel steht auf `draft`; daraus akzeptiert die KMF noch keinen OTAR-Job. Für diesen Erstaufbau den Lab-Schlüssel aktivieren. Eine echte Rotation erzeugt stattdessen einen `staged`-Nachfolger, der vor seiner Aktivierung zugestellt werden kann.

```bash
curl --fail-with-body http://127.0.0.1:8190/api/v1/keys/KEY_ID/activate \
  -H 'Content-Type: application/json' -d '{"reason":"Lab-Erstaufbau"}'
```

## Job anlegen, freigeben und queueen

```bash
curl --fail-with-body http://127.0.0.1:8190/api/v1/otar/jobs \
  -H 'Content-Type: application/json' \
  -d '{"key_id":"KEY_ID","target_nodes":["tbs-04010001"],"target_issis":[],"target_gssis":[15501],"notes":"Lab GCK rollout"}'

curl --fail-with-body http://127.0.0.1:8190/api/v1/otar/jobs/JOB_ID/approve \
  -H 'Content-Type: application/json' -d '{"actor":"lab-operator-a"}'
curl --fail-with-body http://127.0.0.1:8190/api/v1/otar/jobs/JOB_ID/approve \
  -H 'Content-Type: application/json' -d '{"actor":"lab-operator-b"}'
curl --fail-with-body http://127.0.0.1:8190/api/v1/otar/jobs/JOB_ID/queue \
  -H 'Content-Type: application/json' -d '{}'
```

Die zwei Actor-Namen sind ohne Anmeldung keine geprüften Identitäten. Zum Wechsel nach `authoritative` die vollständige Ausgabe von `GET /api/v1/policy` übernehmen und nur `operating_mode` ändern; `POST /api/v1/policy` erwartet alle Policy-Felder.

## Claim und Anwendung quittieren

```bash
curl --fail-with-body http://127.0.0.1:8190/api/v1/edge/actions/claim \
  -H 'Content-Type: application/json' \
  -d '{"node_id":"tbs-04010001","max_actions":10}'

curl --fail-with-body http://127.0.0.1:8190/api/v1/edge/actions/ACTION_ID/ack \
  -H 'Content-Type: application/json' -d '{"success":true}'
```

Den ACK erst nach tatsächlicher lokaler Anwendung im Lab-Adapter senden. Die Antwort enthält nodegebundene Envelopes, keine Rohschlüssel; sie ist noch kein D-OTAR-PDU.

Aus dem Repository-Hauptverzeichnis öffnet der Lab-Helfer eine einzelne beanspruchte Action aus einer JSON-Datei mit dem zugehörigen Bootstrap. Dateiplatzhalter durch lokale Lab-Dateien ersetzen:

```bash
python3 system-backend/kmf/tests/lab_edge_unwrap.py BOOTSTRAP_JSON CLAIMED_ACTION_JSON
```

**Quellabgleich vom 9. Oktober 2026:** [API-Routen](../../../../system-backend/kmf/src/http.rs) und [JSON-Anfragefelder](../../../../system-backend/kmf/src/protocol.rs). Weiter: [OTAR-Ablauf](../otar-zustellablauf.md).
