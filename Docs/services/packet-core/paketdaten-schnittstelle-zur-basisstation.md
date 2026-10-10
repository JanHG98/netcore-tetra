# Packet Edge Protocol v1

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/protocol.rs](../../../system-backend/packet-core/src/protocol.rs) · [src/state.rs](../../../system-backend/packet-core/src/state.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

Protokollkennung:

```text
netcore-packet-edge-v1
```

Die HTTP-Referenzschnittstelle ist `POST /api/v1/edge/events`. Sie dient zunächst als klar testbare Grenze, bis dieselben Events direkt in die dauerhafte Edge/Core-Verbindung der TBS übernommen werden.

Unterstützte Ereignisfamilien:

- `hello`, `heartbeat`, `node_lost`
- `subscriber_location`
- `activate_demand`, `context_activated`
- `data_transmit_request`, `end_of_data`, `reconnect`
- `modify`, `deactivate`
- `bearer`, `packet_counters`
- `fragment`

Im Authoritative-Modus antwortet der Core mit versionierten Aktionen wie `activate_accept`, `activate_reject`, `page`, `end_of_data`, `modify`, `deactivate` oder `fragment`.

Zusätzlich nutzt der Core den Node Gateway für bestehende TBS-Kommandos. Die neuen `PacketData*`-Kommandos werden im Stack zur SNDCP-Entity geroutet und mit `PacketDataActionResult` korreliert.

## Zwei getrennte Transportwege

HTTP-Edge-Ereignisse liefern beziehungsweise hinterlegen versionierte Referenzaktionen. Die bestehende Node-Gateway-Bridge übersetzt derzeit nur `deactivate`, `modify`, `page` und `end_of_data` in `PacketData*`-Kommandos. `activate_accept`, `activate_reject`, Reconnect-/Data-Transmit-Antworten und `fragment` besitzen dort keine vollständige TBS-Kommandoumsetzung. Ein erfolgreich angenommenes HTTP-Fragment beweist deshalb noch keine Weiterleitung über das Funkinterface.
