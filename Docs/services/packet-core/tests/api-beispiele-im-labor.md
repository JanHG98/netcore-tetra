# API-Beispiele im offenen Labor

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/state.rs](../../../../system-backend/packet-core/src/state.rs) · [src/http.rs](../../../../system-backend/packet-core/src/http.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

Diese Aufrufe sind eine Bedien- und Prüfanleitung für den Packet Core, kein Protokoll bereits ausgeführter Tests. Der Dienst muss laufen; `127.0.0.1:8160` funktioniert nur bei einem Listener auf Loopback oder allen Interfaces. Nach LXC-Installation die tatsächliche Adresse aus `/etc/netcore/lxc-network.env` verwenden. Platzhalter für Node-, Ruf- und Aufnahme-IDs durch vorhandene IDs aus den lesenden API-Antworten ersetzen. Schreibende Beispiele verändern den Laborzustand.

## Edge Hello

```bash
curl -sS http://127.0.0.1:8160/api/v1/edge/events \
  -H 'content-type: application/json' \
  -d '{"kind":"hello","protocol_version":"netcore-packet-edge-v1","node_id":"tbs-04010001","station_name":"Lab TBS","mcc":1,"mnc":333,"location_area":1}'
```

## Kontextaktivierung simulieren

```bash
curl -sS http://127.0.0.1:8160/api/v1/edge/events \
  -H 'content-type: application/json' \
  -d '{"kind":"activate_demand","node_id":"tbs-04010001","issi":4010001,"nsapi":1,"requested_ipv4":null,"primary_nsapi":null,"snei":null,"mtu":1500,"priority":3}'
```

## Kontext pagen

```bash
curl -sS -X POST http://127.0.0.1:8160/api/v1/contexts/4010001:1/wake \
  -H 'content-type: application/json' -d '{}'
```

## N-PDU einspeisen

```bash
curl -sS -X POST http://127.0.0.1:8160/api/v1/downlink \
  -H 'content-type: application/json' \
  -d '{"issi":4010001,"nsapi":1,"payload_hex":"4500001400000000400100000a2c00010a2c0002"}'
```

## Erwartung im Shadow-Modus

Mit der Beispielkonfiguration erzeugt `activate_demand` nur ein Shadow-Ereignis. Für die nachfolgenden Wake-/Downlink-Beispiele zuerst einen bereits existierenden Kontext aus `/api/v1/contexts` verwenden oder über `context_activated` gezielt spiegeln. Alternativ Authoritative nur als bewusst konfigurierte HTTP-Referenzprüfung aktivieren. Dabei entsteht kein automatisch vollständig angebundener Funkdatenpfad. Die oben angegebene IPv4-Payload ist ein synthetisches Formatbeispiel und kein validierter Ping mit gültiger Header-Prüfsumme.
