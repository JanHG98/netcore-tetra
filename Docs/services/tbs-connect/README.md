# Historischer TBS-Brew-Connector

Stand: **9. Oktober 2026**. Dieser vorhandene Altbaustein ist ein Python-Brew-Router auf **Port 8081**. Der zentrale NetCore-Netzbetrieb verwendet den implementierten [Node Gateway](../node-gateway/README.md). TBS Connect ist weder dessen aktueller Installationspfad noch ein Ersatz für die typisierten Core-Verträge.

Quellen: [server.py](../../../system-backend/tbs-connect/server.py) und [WebUI-Dateien](../../../system-backend/tbs-connect/web-ui).

## Vorhandenes Verhalten

- Brew-Endpunkt `/brew/` und WebSocket `/brew/{uuid}`.
- Lokale Verwaltungsanmeldung unter `/login`, Oberfläche unter `/dashboard`.
- Verbindungs-, Gruppenruf- und Logansichten über `/api/status`, `/api/connections`, `/api/group_calls` und `/api/logs`.
- Live-Aktualisierung über Server-Sent Events unter `/api/events`.
- Verbindungs- und Rufzustände sowie ein begrenztes Log im Arbeitsspeicher.

Der Quellcode benötigt `aiohttp`, `aiohttp-session` und `cryptography`; er ist kein reiner Standardbibliothek-Dienst. Es liegt kein eigener aktueller Installer oder systemd-Dienst für diesen Baustein bei. Ein vorhandener Altbetrieb muss mit seiner tatsächlichen Startumgebung abgeglichen werden.

## Zugang und Abgrenzung

`BREW_ADMIN_USER`, `BREW_ADMIN_PASS` und `BREW_SESSION_KEY` steuern lokalen Verwaltungszugang und Sitzungsschlüssel. Die Codevorgaben `admin`/`admin123` sind keine geeigneten Betriebszugangsdaten. Ohne festen Sitzungsschlüssel erzeugt jeder Prozessstart einen neuen Schlüssel; vorhandene Sitzungen werden dadurch ungültig. `nodes.json` wird laut Quellcode nicht mehr zur Node-Authentisierung verwendet. Es gibt keinen TLS-Listener und keine neuen `/health/*`-/`/metrics`-Verträge.

Vor einer Ablösung Verbindungen, tatsächlichen Brew-Vertrag und Ausfallverhalten mit [Node Gateway](../node-gateway/README.md) vergleichen. Eine weitere WebUI-Konsolidierung nach dem [Backend-WebUI-Standard](../../design/dienstoberflaechen-gestaltungsstandard.md) ist ein möglicher späterer Schritt und keine bereits erledigte Migration.
