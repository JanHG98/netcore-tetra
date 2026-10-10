# Queues, Wiederholungen und Reports

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/state.rs](../../../system-backend/sds-router/src/state.rs) · [src/http.rs](../../../system-backend/sds-router/src/http.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

## Zustände

- `received`: angenommen, noch nicht geplant
- `queued`: mindestens ein Zustellweg wartet
- `offline`: aktuell keine zuständige oder erreichbare TBS
- `in_flight`: Kommando an eine TBS übergeben
- `delivered`: alle vorgesehenen Legs angenommen beziehungsweise bestätigt
- `partial`: nur ein Teil der Legs erfolgreich
- `failed`: Zustellung fehlgeschlagen
- `expired`: TTL abgelaufen
- `cancelled`: operatorseitig gestoppt
- `dead_letter`: endgültig nicht zustellbar

Jede TBS und jede Anwendung bildet ein eigenes Delivery Leg. Gruppen-SDS können deshalb teilweise erfolgreich sein, ohne dass bereits alle Standorte erreicht wurden.

## Retry

Der Backoff startet mit `initial_retry_secs` und wächst exponentiell bis `max_retry_secs`. Nach `max_attempts` wird ein TBS-Leg endgültig fehlgeschlagen. Die Bedienoberfläche kann Nachrichten manuell erneut einreihen.

## Zustellberichte

Die Antwort `SdsDeliveryResponse` bestätigt zunächst nur, dass die lokale TBS den Auftrag für die Air-Interface-Zustellung angenommen hat. Terminalberichte, etwa SDS-TL Delivery Reports, werden als eigene Meldung verarbeitet und im Nachrichtenobjekt gespeichert.

## Idempotenz und einmaliger Versuch

`idempotency_key` speichert den ursprünglichen API-Auftrag dauerhaft. Derselbe Schlüssel mit identischem Auftrag liefert dieselbe Message-ID; ein anderer Inhalt mit demselben Schlüssel führt zu HTTP 409. Die Bindung bleibt nach Löschung der Nachricht verbraucht. `GET /api/v1/idempotency/{key}` zeigt diesen Zustand auch mit `retained = false`. Nach einem HTTP-Timeout den unveränderten ursprünglichen Auftrag verwenden.

`at_most_once = true` benötigt einen Schlüssel und einen Individualempfänger. Es erlaubt höchstens einen zentralen TBS-Dispatch und sperrt manuelles Retry/Requeue; es garantiert keine Zustellung am Funkgerät. `expires_at` setzt eine absolute RFC-3339-Grenze zusätzlich zur TTL. Die normale Queue bleibt dagegen ein Verfahren mit möglichen Wiederholungen.
