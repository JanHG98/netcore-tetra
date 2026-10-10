# Security Core – Ablauf der Authentisierung

Die Policy prüft beim Start ISSI, Knotenzulassung, Sperren und angebotene Security Classes. Class 3 erzwingt Authentisierung; der Pfad ohne Authentisierung ist nur zulässig, wenn die wirksame Policy ihn erlaubt.

| Schritt | Zustand und Wirkung |
| --- | --- |
| `POST /api/v1/auth/start` | Ohne erforderliche Authentisierung direkt `authenticated`; sonst `challenge_pending` |
| Edge-Claim | Challenge-Aktion wird `in_flight`; der Kontext wartet weiterhin auf die Challenge-Quittierung |
| Erfolgreicher Challenge-ACK | Kontext wechselt nach `awaiting_response` |
| Gültige Antwort | Kontext wird `authenticated`; Class 3 erzeugt einen DCK in `pending_install` |
| Erfolgreicher DCK-ACK | DCK wird `active`; erst dies bestätigt die Installation durch die Edge |
| Ungültige Antwort | Versuchszähler steigt; weitere Antworten auf denselben Kontext sind bis zum Limit möglich, danach `rejected` und Lockout |
| Ablauf oder Widerruf | Kontext wird `expired` beziehungsweise `revoked` |

Ein fehlgeschlagener Challenge-ACK beendet die Authentisierung. Ein fehlgeschlagener DCK-Installations-ACK setzt den DCK auf `install_failed` und widerruft den Authentisierungskontext. Ein positiver Authentisierungsstatus allein bestätigt deshalb keine erfolgreiche DCK-Installation.

## Beispielwerte und Neustart

Die [Beispielkonfiguration](../../../system-backend/security-core/config/security-core.example.toml) verwendet 30 Sekunden Challenge-TTL, drei Antwortversuche, 300 Sekunden Lockout und 3.600 Sekunden DCK-TTL. Individuelle Profile können das Versuchslimit beeinflussen.

Roh-Challenge, erwartete Antwort und DCK werden nicht im Zustand serialisiert. Nach Neustart werden offene Authentisierungen abgebrochen und aktive oder ausstehende DCK-Kontexte widerrufen. Für gezielte Ablaufprüfungen existiert `POST /api/v1/maintenance/expire`; der Prozess startet keinen eigenen periodischen Wartungsworker.

**Quellabgleich vom 9. Oktober 2026:** [Zustandsmaschine](../../../system-backend/security-core/src/state.rs), insbesondere `start_authentication`, `acknowledge_edge_action`, Antwortprüfung und `recover_after_restart`. Weiter: [Edge-API](edge-api-und-quittierungen.md).
