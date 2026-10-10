# Security Core – Edge-API und Quittierungen

Protokollkennung: `netcore-security-edge-v1`. Die Management-API liefert Aktionsmetadaten; der getrennte Claim-Pfad kann Challenge- und DCK-Rohmaterial enthalten. Beide verwenden TCP 8180 ohne Client-Authentisierung im Open Lab.

## Erfolgreicher Ablauf

1. `POST /api/v1/auth/start` erzeugt Kontext und Challenge-Aktion.
2. Die Edge holt passende Aktionen über `POST /api/v1/edge/actions/claim` mit `node_id` und `limit`.
3. Sie setzt die Challenge lokal um und quittiert `POST /api/v1/edge/actions/{id}/ack` mit `{"success":true}`.
4. Erst danach sendet sie die Antwort an `POST /api/v1/auth/{context_id}/response` mit `response_hex` und ihrer `node_id`.
5. Für Class 3 holt und installiert sie die folgende `install_dck`-Aktion und quittiert diese separat.

Die API-Beispiele stehen [hier](tests/api-beispiele-im-labor.md).

## Regeln und Fehlerfälle

- Claims sind nach `node_id` gefiltert; globale Aktionen mit `node_id="*"` sind ebenfalls abrufbar. Dies ist eine fachliche Zuordnung, keine verifizierte Geräteidentität.
- `shadow` liefert keine Aktionen. Außerdem muss `security.expose_ephemeral_edge_material=true` sein.
- Das Claim-Limit wird auf 1 bis 250 begrenzt. Jede Aktion besitzt ID, Sequenz, TTL und den Pfad `pending → in_flight → applied|failed`.
- Der erfolgreiche Challenge-ACK schaltet den Kontext für die Antwort frei; der Claim selbst tut dies nicht.
- Jeder verarbeitete ACK entfernt das zugehörige Aktionspayload aus dem Arbeitsspeicher. Erwartete Antworten und aktive DCKs bleiben für ihren eigenen Kontextlebenszyklus verfügbar.
- Offene Claims werden nach Neustart als fehlgeschlagen behandelt; der Security Core bietet hier keinen KMF-artigen Backoff-Zustellungsworkflow.

**Quellabgleich vom 9. Oktober 2026:** [HTTP-Routen](../../../system-backend/security-core/src/http.rs), [Anfragefelder](../../../system-backend/security-core/src/protocol.rs) und [Claim/ACK-Logik](../../../system-backend/security-core/src/state.rs).
