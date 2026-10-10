# KMF – Ablauf der Lab-OTAR-Zustellung

Der vorhandene Workflow orchestriert eine Zustellung an TBS-Nodes über `netcore-kmf-otar-edge-v1`. Er kodiert **keine D-OTAR-Air-Interface-PDUs** und bestätigt keine Schlüsselübernahme durch ein Funkendgerät.

## Ablauf

1. CCK/GCK/SCK und Node-Transportprofile anlegen. Der zu verteilende Schlüssel muss `staged`, `active`, `retiring` oder `retired` sein; ein neu erzeugter `draft` wird für Jobs abgelehnt.
2. Job mit Key-ID, Zielnodes und optionalen ISSI-/GSSI-Zielen erzeugen.
3. Job freigeben; standardmäßig sind zwei unterschiedliche Actor-Namen erforderlich.
4. `POST /api/v1/otar/jobs/{id}/queue` erzeugt pro Node eine Delivery und Action.
5. Die Edge beansprucht über `POST /api/v1/edge/actions/claim` die für ihre `node_id` vorgesehenen Aktionen.
6. Sie prüft Envelope-Context, öffnet das Envelope mit ihrem Bootstrap-Geheimnis und quittiert die lokale Anwendung über `POST /api/v1/edge/actions/{id}/ack`.

Die Schlüsselaktivierung erfolgt separat über die Lifecycle-API. Ein Edge-ACK beschreibt die vom Adapter gemeldete Anwendung, keinen automatisch geprüften On-Air-Ablauf.

## Freigabe und Betriebsmodus

Ohne Login sind die Actor-Namen deklarativ. Bei `require_dual_approval=true` verlangt die KMF zwei unterschiedliche Namen und lehnt eine erneute Freigabe desselben Actors ab.

In `shadow` bleiben Actions und Deliveries `staged` und Claims leer. Ein Policy-Wechsel nach `authoritative` stuft staged Aktionen auf `pending` hoch. Der KMF-Policy-Endpunkt erwartet die vollständigen Policy-Felder; vor Änderung `GET /api/v1/policy` lesen und alle Werte erhalten.

## Envelope

Ein Claim enthält unter anderem:

```json
{
  "key_id": "KEY_ID",
  "key_fingerprint": "FINGERPRINT",
  "envelope": {
    "algorithm": "lab_sha256_stream_mac_v1",
    "nonce_hex": "HEX",
    "ciphertext_hex": "HEX",
    "mac_hex": "HEX"
  },
  "envelope_context": "netcore-kmf-otar-edge-v1:ACTION_ID:NODE_ID:KEY_ID"
}
```

Die genauen dynamischen IDs aus der Antwort verwenden. Der [Lab-Unwrap-Helfer](../../../system-backend/kmf/tests/lab_edge_unwrap.py) prüft die MAC unter Einbeziehung des gelieferten Contexts und gibt nur Fingerprint und Länge des geöffneten Schlüssels aus.

## Retry und Wartung

Ein negatives ACK stellt die Action mit Backoff erneut auf `pending`, solange `max_attempts` nicht erreicht ist. Nicht quittierte In-Flight-Aktionen werden durch `POST /api/v1/maintenance/tick` erneut freigegeben beziehungsweise beendet. Der Prozess hat keinen automatischen KMF-Wartungstimer.

Die Beispielwerte sind 600 Sekunden Action-TTL, fünf Claims und 15 Sekunden Retry-Backoff. Der Jobzustand wird aus allen Deliveries berechnet, etwa `completed`, `partial_failure`, `failed` oder `in_progress`.

**Quellabgleich vom 9. Oktober 2026:** [Jobs, Claims und ACKs](../../../system-backend/kmf/src/state.rs), [Anfragefelder](../../../system-backend/kmf/src/protocol.rs). Weiter: [API-Beispiele](tests/api-beispiele-im-labor.md).
