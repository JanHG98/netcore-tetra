# Call Control

## Zweck

Call Control ist der zentrale, eigenständig deploybare Dienst für netzweite logische Gruppen- und Individualrufe. Die TBS behalten weiterhin die zeitkritischen CMCE-, Funkkanal- und Floor-Prozeduren; Call Control koordiniert die lokalen Call Legs über mehrere Zellen hinweg.

## Kernaufgaben

- logische Gruppen- und Individualrufe verwalten
- passende TBS anhand von Affiliationen und Registrierungen auswählen
- lokale Call Legs starten und beenden
- Priorität, Notrufstatus und Floor-Zustand zusammenführen
- Floor-Anforderungen, Queueing und Operator-Handover koordinieren
- laufende TBS-Rufe aus Telemetrie erkennen
- Restore Context zwischen Quell- und Ziel-TBS übertragen
- Zustände, Fehler, Timeouts und Teilstarts persistent dokumentieren

## WebUI

Die eigene WebUI läuft standardmäßig unter `http://<LXC-IP>:8120/` und bleibt unabhängig vom Control Room erreichbar.

Sie zeigt logische Calls, TBS-Legs, Floor Holder, Queue, Teilnehmerlage, Restore-Vorgänge, Basisstationen und Ereignisse. Gruppen- und Individualrufe sowie Floor- und Restore-Aktionen können direkt ausgeführt werden.

## Open-Lab-Modus

Diese Ausbaustufe besitzt absichtlich keine Tokens, Passwörter, Benutzeranmeldung, TLS oder RBAC. Sie darf nur in einem isolierten Testnetz betrieben werden.

## Datenhaltung

- `/var/lib/netcore-call-control/calls.json`
- `/var/lib/netcore-call-control/calls.json.bak`

## Abhängigkeiten

- Node Gateway auf `/ws/backend`
- kompatible TBS mit `call_control` und für Restore zusätzlich `call_restore_context`
- Teilnehmer- und Gruppenlage aus TBS-Telemetrie
- später Media Switch für den eigentlichen netzweiten Sprachtransport

## Echtzeit-Medienereignisse

Der Media Switch pollt den Callgraphen nicht mehr im Zwei-Sekunden-Takt. Call Control
stellt auf demselben Port den WebSocket `ws://<LXC-IP>:8120/ws/media` mit dem
Subprotokoll `netcore-call-control-media-v1` bereit. Bei relevanten Zustandswechseln wird
sofort ein revisionsbehafteter Snapshot ausgesendet. Die Ereignisarten entsprechen dem
Medienlebenszyklus: `call_created`, `leg_ready`, `floor_changed`, `call_updated` und
`call_released`.

Jedes Ereignis enthält bewusst den vollständigen kompakten Snapshot aller nicht-terminalen
Calls und Legs. Dadurch bleibt der Media Switch auch nach Verbindungsunterbrechung,
Reconnect oder Prozessneustart ohne eine komplizierte Delta-Replay-Logik konsistent.

Operator-Floor-Anforderungen werden erst angenommen, wenn alle nicht-terminalen Legs
eine aktive lokale Call-ID und einen Timeslot besitzen **und** der Media Switch den
Routinggraphen über `POST /api/v1/media/route-ready` für die aktuelle Ereignisrevision
bestätigt hat. Ein älteres oder für eine andere Topologie gesendetes ACK wird abgewiesen.
Bei funkausgelösten Rufen schützt der Kaltstart-Vorpuffer des Media Switch zusätzlich die
ersten Sprachframes, während die restlichen Ziel-Legs aufgebaut werden.

## Mobility-Core-Routing

Individualrufe ohne explizite Ziel-TBS werden über den Mobility Core aufgelöst. Die Einstellungen befinden sich im Abschnitt `[mobility_core]`. Im Normalbetrieb bleiben lokaler Fallback und stale Routen deaktiviert.

## Gemeinsames Ereignismodell (MQTT Phase 2)

Der Dienst behält `GET /api/v1/events` für die bestehende WebUI bei. Jeder lokale Datensatz enthält zusätzlich `canonical`. Für neue Verbraucher steht ausschließlich das gemeinsame Format unter `GET /api/v1/events/netcore?limit=100` bereit. Das Wire-Schema ist `netcore-event-v1`; MQTT-spezifische Topic-, QoS- und Retain-Regeln folgen erst im IoT Gateway.
