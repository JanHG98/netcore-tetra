# Security Core – API-Beispiele im Labor

Diese Beispiele verwenden eine laufende isolierte Instanz mit Loopback- oder Wildcard-Listener auf dem lokalen Host. Nach der LXC-Installation stattdessen den Host aus `/etc/netcore/lxc-network.env` beziehungsweise die tatsächliche Listener-Adresse verwenden. Mit der ausgelieferten `shadow`-Policy bleibt der Edge-Claim leer. Für einen vollständigen Lab-Authentisierungsablauf muss die Policy bewusst auf `authoritative` gestellt werden.

## Profil und Authentisierung

```bash
curl --fail-with-body -X POST http://127.0.0.1:8180/api/v1/profiles \
  -H 'Content-Type: application/json' \
  -d '{"issi":4010001,"display_name":"Lab HRT","preferred_security_class":3,"minimum_security_class":1}'

curl --fail-with-body -X POST http://127.0.0.1:8180/api/v1/auth/start \
  -H 'Content-Type: application/json' \
  -d '{"node_id":"tbs-04010001","issi":4010001,"requested_security_class":3,"supported_security_classes":[1,3]}'

curl --fail-with-body -X POST http://127.0.0.1:8180/api/v1/edge/actions/claim \
  -H 'Content-Type: application/json' \
  -d '{"node_id":"tbs-04010001","limit":10}'
```

Profilanlage und -aktualisierung erfolgen über `POST /api/v1/profiles`; `PUT /api/v1/profiles/{issi}` existiert nicht. Die Startantwort liefert die Kontext-ID, der Claim die Challenge-Aktion samt ID und Payload.

## Challenge quittieren, danach Antwort senden

Die Platzhalter aus den jeweiligen Antworten einsetzen. Der folgende ACK bestätigt die lokale Challenge-Ausführung, noch keine DCK-Installation:

```bash
curl --fail-with-body -X POST http://127.0.0.1:8180/api/v1/edge/actions/ACTION_ID/ack \
  -H 'Content-Type: application/json' -d '{"success":true}'

curl --fail-with-body -X POST http://127.0.0.1:8180/api/v1/auth/CONTEXT_ID/response \
  -H 'Content-Type: application/json' \
  -d '{"node_id":"tbs-04010001","response_hex":"LAB_RESPONSE_HEX"}'
```

Eine Lab-Antwort berechnet [lab_response.py](../../../../system-backend/security-core/tests/lab_response.py) mit `--seed`, `--issi`, `--node`, `--context` und `--challenge`. Aus dem Repository-Hauptverzeichnis zunächst die Parameter ansehen:

```bash
python3 system-backend/security-core/tests/lab_response.py --help
```

Danach muss bei Class 3 die folgende DCK-Aktion gesondert beansprucht, installiert und quittiert werden. Claim-Payloads und Antworten mit Secret-Material gehören nicht in Tickets, Screenshots oder normale Logs.

**Quellabgleich vom 9. Oktober 2026:** [API-Routen](../../../../system-backend/security-core/src/http.rs) und [Anfragefelder](../../../../system-backend/security-core/src/protocol.rs). Weiter: [Authentisierungsablauf](../authentisierungsablauf.md).
