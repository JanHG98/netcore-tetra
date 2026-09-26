# Installation – MQTT Phase 2 / Gemeinsames Ereignismodell

## Betroffene LXCs

- Node Gateway
- Mobility Core
- Call Control
- SDS Router

`shared/contracts` und `shared/service-common` sind Bibliotheken und benötigen keinen eigenen LXC.

## Vorgehen

Auf jedem betroffenen LXC muss derselbe neue Repository-Stand unter `/opt/netcore-tetra` liegen. Danach wird ausschließlich der dort laufende Dienst aktualisiert.

### Node Gateway

```bash
cd /opt/netcore-tetra/system-backend/node-gateway
./install/update.sh
```

### Mobility Core

```bash
cd /opt/netcore-tetra/system-backend/mobility-core
./install/update.sh
```

### Call Control

```bash
cd /opt/netcore-tetra/system-backend/call-control
./install/update.sh
```

### SDS Router

```bash
cd /opt/netcore-tetra/system-backend/sds-router
./install/update.sh
```

Die Skripte müssen ausführbar sein. Falls ein Kopierwerkzeug die Rechte entfernt hat:

```bash
find /opt/netcore-tetra/system-backend -path '*/install/*.sh' -type f -exec chmod 755 {} +
```

## Test

Portzuordnung im Standardpaket:

```text
Node Gateway  8080
Mobility Core 8090
Call Control  8120
SDS Router    8150
```

Beispiel:

```bash
curl -fsS http://IP-DES-MOBILITY-CORE:8090/api/v1/events/netcore?limit=5 \
  | python3 -m json.tool
```

Direkt nach einem Neustart kann die Liste leer sein. Dann ein passendes Ereignis erzeugen, beispielsweise ein Funkgerät registrieren, einen Ruf starten oder eine SDS senden.

Jeder Eintrag muss mindestens enthalten:

```text
schema = netcore-event-v1
event_id
event_type
source.service
source.instance
timestamp
severity
payload
```
