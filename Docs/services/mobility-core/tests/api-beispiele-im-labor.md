# API-Beispiele

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/state.rs](../../../../system-backend/mobility-core/src/state.rs) · [src/http.rs](../../../../system-backend/mobility-core/src/http.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

Diese Aufrufe sind eine Bedien- und Prüfanleitung für den Mobility Core, kein Protokoll bereits ausgeführter Tests. Der Dienst muss laufen; `127.0.0.1:8090` funktioniert nur bei einem Listener auf Loopback oder allen Interfaces. Nach LXC-Installation die tatsächliche Adresse aus `/etc/netcore/lxc-network.env` verwenden. Platzhalter für Node-, Ruf- und Aufnahme-IDs durch vorhandene IDs aus den lesenden API-Antworten ersetzen. Schreibende Beispiele verändern den Laborzustand.

Transfer starten:

```bash
curl -X POST http://MOBILITY-CORE:8090/api/v1/transfers \
  -H 'Content-Type: application/json' \
  -d '{
    "issi": 1234567,
    "source_node": "tbs-a",
    "target_node": "tbs-b",
    "target_local_issi": 1234567
  }'
```

Transfer abbrechen:

```bash
curl -X POST http://MOBILITY-CORE:8090/api/v1/transfers/TRANSFER-ID/cancel
```

Es werden bewusst keine Authorization-Header oder Tokens verwendet.

Vor dem Transfer Node-IDs aus `/api/v1/nodes` und den Serving Node aus `/api/v1/subscribers` beziehungsweise `/api/v1/subscribers/1234567/route` prüfen. Der POST muss von bestätigtem Export, Zielimport und Quellbereinigung gefolgt werden; die erste API-Annahme allein ist noch kein vollständiger Transfer.
