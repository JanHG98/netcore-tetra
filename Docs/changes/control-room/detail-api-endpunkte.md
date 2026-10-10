# Leitstelle – Detail API

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: NetCore Control Room – Detail API. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Die aktuelle Server-/Transportbeschreibung steht in [Control Room](../../services/control-room/README.md); die gepflegten Quellen liegen unter [Quellcode](../../../system-backend/control-room) und [Rust-Core](../../../bins/netcore-control-room) . Frühere Startsnippets und Handshake-Korrekturen sind keine vollständige heutige Installationsanleitung.

Dieser Änderungsnachweis bewahrt den beschriebenen UI-/API-/Buildstand. Damalige Tokenregeln, Feldnamen, Überschreib- und Buildbefehle gelten für diese Revision und dürfen nicht als aktuelle Komplettanleitung übernommen werden.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### NetCore Control Room – Detail API

Dieser Patch erweitert den Control-Room-Core um leitstellentaugliche Detail-Endpunkte. `/api/overview` bleibt der schlanke Dashboard-Einstieg, die neuen Endpunkte liefern gezielt Teilnehmer, Gruppen, aktive Rufe, SDS und Notrufe.

## Globale Endpunkte

```bash
curl http://127.0.0.1:9010/api/subscribers | jq
curl 'http://127.0.0.1:9010/api/subscribers?online=true' | jq
curl http://127.0.0.1:9010/api/groups | jq
curl http://127.0.0.1:9010/api/calls | jq
curl 'http://127.0.0.1:9010/api/sds?limit=100' | jq
curl 'http://127.0.0.1:9010/api/emergencies?active=true' | jq
```

## Node-spezifische Endpunkte

```bash
curl http://127.0.0.1:9010/api/nodes/tbs-04010001 | jq
curl 'http://127.0.0.1:9010/api/nodes/tbs-04010001/subscribers?online=true' | jq
curl http://127.0.0.1:9010/api/nodes/tbs-04010001/groups | jq
curl http://127.0.0.1:9010/api/nodes/tbs-04010001/calls | jq
curl 'http://127.0.0.1:9010/api/nodes/tbs-04010001/sds?limit=50' | jq
curl 'http://127.0.0.1:9010/api/nodes/tbs-04010001/emergencies?active=true' | jq
```

## Zweck

- `/api/subscribers`: Teilnehmerliste mit ISSI, Online-Status, Gruppen, RSSI, Emergency-Flag, letzter Aktivität und aktiven Call-Keys.
- `/api/groups`: Gruppenliste mit GSSI, Mitgliedern, Online-Mitgliedern und aktivem Call.
- `/api/calls`: aktive Gruppen- und Einzelrufe mit Carrier, Timeslot, Priority, Sprecher/Teilnehmern und Zeitstempeln.
- `/api/sds`: jüngste SDS-Nachrichten pro Node oder global.
- `/api/emergencies`: aktive oder historische Notrufe.
- `/api/nodes/{node_id}`: kompakte Detailansicht für genau eine Basisstation.
