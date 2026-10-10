# Anwendungsgateways und Protocol-ID-Routing

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/state.rs](../../../system-backend/sds-router/src/state.rs) · [src/http.rs](../../../system-backend/sds-router/src/http.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

Protocol-ID-Regeln können SDS-Nachrichten an eine benannte Anwendung weiterreichen. Die Anwendung liest ihre Queue über:

```text
GET /api/v1/application-outbox?application=<name>
```

Nach Verarbeitung bestätigt sie das Leg:

```text
POST /api/v1/application-outbox/<name>/<message-id>/ack
Content-Type: application/json

{"success":true,"message":"accepted"}
```

Die Modi sind:

- `tap`: Anwendung erhält eine Kopie; Funkrouting bleibt bestehen.
- `route`: Anwendung wird als reguläres Ziel ergänzt.
- `intercept`: Anwendung übernimmt die Nachricht; automatische Funkweiterleitung wird unterdrückt.

In der aktuellen Open-Lab-Phase ist die Outbox nicht authentifiziert. Application Gateway und Security Core sind bereits vorhanden. Sie ergänzen in diesem Quellstand jedoch keine Authentisierung der Router-Outbox; Dienstidentitäten, Signaturen und RBAC an dieser Grenze bleiben offen. `security.mask_payload_in_list` kann Listeninhalte reduzieren, schützt aber keine schreibende API.
