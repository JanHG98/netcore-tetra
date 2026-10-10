# API-Beispiele

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/http.rs](../../../../system-backend/ip-gateway/src/http.rs) · [src/state.rs](../../../../system-backend/ip-gateway/src/state.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

Diese Aufrufe sind eine Bedien- und Prüfanleitung für den IP Gateway, kein Protokoll bereits ausgeführter Tests. Der Dienst muss laufen; `127.0.0.1:8170` funktioniert nur bei einem Listener auf Loopback oder allen Interfaces. Nach LXC-Installation die tatsächliche Adresse aus `/etc/netcore/lxc-network.env` verwenden. Platzhalter für Node-, Ruf- und Aufnahme-IDs durch vorhandene IDs aus den lesenden API-Antworten ersetzen. Schreibende Beispiele verändern den Laborzustand.

## Route

```bash
curl -X POST http://127.0.0.1:8170/api/v1/routes \
  -H 'content-type: application/json' \
  -d '{"name":"Lab","destination":"192.168.50.0/24","gateway":"10.0.1.1","interface":"eth0","enabled":true}'
```

## Firewall

```bash
curl -X POST http://127.0.0.1:8170/api/v1/firewall \
  -H 'content-type: application/json' \
  -d '{"name":"HTTP outbound","chain":"forward","action":"accept","protocol":"tcp","source_cidr":"10.0.0.0/24","destination_port":80,"priority":50,"enabled":true}'
```

## Capture

```bash
curl -X POST http://127.0.0.1:8170/api/v1/captures \
  -H 'content-type: application/json' \
  -d '{"name":"ISSI test","direction":"both","host":"10.0.0.2","protocol":"udp","port":53}'
```

Im Shadow-Modus werden Route, Firewallregel und Capture-Konfiguration in den Zustand aufgenommen; dadurch wird noch kein Kernelpaket weitergeleitet. Das CIDR `10.0.0.0/24` muss zum tatsächlich konfigurierten Paketdatenpool passen. Interface und Gateway der Beispielroute an den Container anpassen, bevor Authoritative aktiviert wird.
