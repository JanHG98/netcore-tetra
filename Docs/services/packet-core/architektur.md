# Architektur

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/state.rs](../../../system-backend/packet-core/src/state.rs) · [src/http.rs](../../../system-backend/packet-core/src/http.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

| Abschnitt | Zuständigkeit und Verbindung |
|---|---|
| TBS SNDCP/PDCH Edge | lokale Funkverfahren, Telemetrie und TBS-Kommandos |
| Node Gateway `8080` | Backend-WebSocket für TBS-Zustand und Steuerung |
| Packet Core `8160` | Kontextzustände, Adresse/Anchor, Bearer-Sicht, Reassembly, Flow Control und Action Queue |
| IP Gateway `8170` | vorhandener HTTP-Verbraucher für N-PDU-Outbox und Kontextzuordnung; Downlink über Packet-Core-API |

Die HTTP-Kopplung an den IP Gateway ist implementiert. Die vollständige Übertragung aller Referenzaktionen über den TBS-Funkdatenpfad ist weiterhin eine eigene Integrationsgrenze.

Der Packet Core ist keine zweite MAC- oder LLC-Implementierung. Zeitkritische Funkentscheidungen bleiben an der TBS. Zentralisiert werden nur Zustände, Policy und TBS-übergreifende Zuordnung.

## Betriebsmodi

- `shadow`: TBS-Telemetrie ist führend; die zentrale State Machine wird zum Vergleich gepflegt.
- `authoritative`: das Edge-Protokoll erzeugt Antworten und Aktionen aus dem zentralen Zustand.

Ein Wechsel in `authoritative` sollte erst nach stabilen Shadow-Vergleichen erfolgen.
