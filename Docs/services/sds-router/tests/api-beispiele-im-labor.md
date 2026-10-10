# API-Beispiele im offenen Labor

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/state.rs](../../../../system-backend/sds-router/src/state.rs) · [src/http.rs](../../../../system-backend/sds-router/src/http.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

Diese Aufrufe sind eine Bedien- und Prüfanleitung für den SDS Router, kein Protokoll bereits ausgeführter Tests. Der Dienst muss laufen; `127.0.0.1:8150` funktioniert nur bei einem Listener auf Loopback oder allen Interfaces. Nach LXC-Installation die tatsächliche Adresse aus `/etc/netcore/lxc-network.env` verwenden. Platzhalter für Node-, Ruf- und Aufnahme-IDs durch vorhandene IDs aus den lesenden API-Antworten ersetzen. Schreibende Beispiele verändern den Laborzustand.

## SDS-TL-Text an Einzelteilnehmer

```bash
curl -X POST http://127.0.0.1:8150/api/v1/messages \
  -H 'Content-Type: application/json' \
  -d '{
    "source_issi": 9999,
    "dest_issi": 4010001,
    "is_group": false,
    "sds_type": 4,
    "protocol_id": 130,
    "text": "NetCore SDS Router Test",
    "priority": 3,
    "ttl_secs": 300,
    "ingress": "curl"
  }'
```

## Pre-coded Status

```bash
curl -X POST http://127.0.0.1:8150/api/v1/messages \
  -H 'Content-Type: application/json' \
  -d '{
    "source_issi": 9999,
    "dest_issi": 4010001,
    "sds_type": 0,
    "status_code": 32770,
    "ttl_secs": 60
  }'
```

## Protocol-ID an Anwendung routen

```bash
curl -X POST http://127.0.0.1:8150/api/v1/routes \
  -H 'Content-Type: application/json' \
  -d '{
    "name": "LIP Collector",
    "enabled": true,
    "kind": "protocol",
    "match_value": 10,
    "target_kind": "application",
    "target": "lip-service",
    "mode": "tap",
    "notes": "Open-Lab LIP route"
  }'
```

## Dauerhafte Idempotenz prüfen

Für einen wiederholbaren API-Auftrag `"idempotency_key":"lab-20261009-001"` ergänzen und bei erneutem Aufruf den gesamten Auftrag unverändert lassen. Der Schlüssel muss für einen neuen fachlichen Auftrag neu gewählt werden. `GET /api/v1/idempotency/lab-20261009-001` zeigt die zugehörige Message-ID. `delivered` bezeichnet die Bestätigung der geplanten Legs; einen tatsächlichen Funkgerätebericht separat prüfen.
