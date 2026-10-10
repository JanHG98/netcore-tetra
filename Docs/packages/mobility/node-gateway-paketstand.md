# Node Gateway – TBS-Anbindung – SWMI-Paketstand

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: SWMI Mobility 1 / Paket D. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Die aktuellen Implementierungseinstiege sind [node-gateway](../../services/node-gateway/README.md) und [Quellcode](../../../system-backend/node-gateway). Lokale Zellwechsel-/Restore-Bausteine und ein Quelltest belegen kein unterbrechungsfreies Handover laufender Gespräche auf der Anlage.

Die Beschreibungen „umgesetzt“, „noch nicht enthalten“ und „nächster Baustein“ gelten innerhalb der damaligen Paketfolge. Sie ersetzen keinen Abgleich mit dem heutigen Quellcode oder mit der installierten Konfiguration.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### SWMI Mobility 1 – Paket D: Node Gateway

## Ziel

Mit Paket D beginnt die tatsächliche Trennung der heutigen TBS-Monolithen vom zentralen Backend. `system-backend/node-gateway/` ist der erste eigenständig deploybare LXC-Dienst.

## Architektur

```text
NetCore TBS
  │ WebSocket: netcore-control-room-node-v1
  ▼
Node Gateway
  ├── Session Registry
  ├── Heartbeat Watchdog
  ├── Event Journal
  ├── Command Router
  ├── Audit
  ├── eigene WebUI/API
  └── optionaler transparenter Bridge-Modus
          │
          ▼
     bestehender Control Room
```

Der Gateway verwendet absichtlich zunächst das bereits produktiv genutzte Node-Protokoll. Damit muss der Air-Interface-Stack nicht gleichzeitig mit der Backend-Auslagerung umgebaut werden.

## Sicherheitsmodell

- TBS-Verbindungen verwenden HTTP Basic Authentication mit separatem Node-Token.
- Die Management-API unterscheidet Viewer, Operator und Admin.
- Management-Tokens werden ausschließlich aus Umgebungsvariablen gelesen.
- Node-Sperren und Wartungszustände werden persistent gespeichert.
- Die Management-WebUI wird im LXC per Caddy über HTTPS bereitgestellt.
- Ein erfolgreicher WebSocket-Upgrade reicht nicht: Pfad, Subprotokoll, Hello und Node-ID werden zusätzlich geprüft.

## Ereignismodell

Das API-Ereignisjournal besitzt monotone Event-IDs:

```text
GET /api/v1/events?after=<cursor>&limit=<n>
```

Spätere Dienste können einen Cursor speichern und neue Ereignisse reproduzierbar abrufen. Das ist zunächst eine einfache, gut prüfbare Grundlage; ein dauerhafter Event Bus folgt erst nach Stabilisierung der fachlichen Backend-Schnittstellen.

## Übergang zum bestehenden Control Room

Für jede TBS-Session kann der Gateway eine entsprechende Upstream-Session öffnen. Binäre Protokollframes werden unverändert transportiert. `HelloAck` wird vom Gateway selbst terminiert; reguläre Commands und Pings des Control Room werden an die TBS weitergeleitet.

## Definition of Done

- eigener Workspace-Binary `netcore-node-gateway`
- eigene WebUI und REST-API
- WebSocket-TBS-Ingress
- Authentisierung und Protokollprüfung
- Duplicate-Session-Handling
- Heartbeat-Timeout
- Node-Sperre und Wartungsmodus
- Audit und Metrics
- optionaler Control-Room-Bridge-Modus
- systemd-, Caddy-, Konfigurations- und Installationsdateien
- Unit- und statische Strukturtests
