# KMF – Schlüssellebenszyklus und Crypto Periods

Die API verwendet die Zustände `draft`, `staged`, `active`, `retiring`, `retired`, `revoked` und `destroyed`. Die Abläufe sind derzeit API-gesteuert; sie bilden keine vollständig automatisch durchlaufene Produktions-State-Machine.

| Aktion | Vorhandenes Verhalten |
| --- | --- |
| Erzeugen | Neuer Schlüssel in `draft`, Secret im Lab-Vault |
| Rotieren | Neue Zufallsbytes und nächste Version in `staged`, Vorgänger-/Nachfolgerverweise; aktiver Vorgänger wird `retiring` |
| Aktivieren | Separater Aufruf setzt den Schlüssel auf `active`; `revoked`/`destroyed` oder fehlendes Material blockieren dies |
| Retire / Revoke | Explizite Zustandsänderung; `destroyed` ist gesperrt |
| Destroy | Entfernt den Secret-Eintrag und Materialverweis, erhält Metadaten und Audit; offene OTAR-Jobs blockieren dies |

Für OTAR-Jobs akzeptiert die KMF nur `staged`, `active`, `retiring` und `retired` mit vorhandenem Material; ein neuer `draft` muss vorher entsprechend vorbereitet werden.

Die Aktivierung ist nicht auf `staged` beschränkt: auch ein `draft` oder `retired` mit verfügbarem Material kann aktiviert werden. Ebenso prüft `destroy` derzeit keine Einschränkung auf bestimmte Ausgangszustände. Diese vorhandenen API-Regeln sind bei Lab-Tests zu berücksichtigen.

## Versionierung und Überlappung

Versionen zählen pro `kind + scope + scope_value`, etwa `GCK / group / 15501 / v1` und `v2` oder `CCK / network / - / v1`.

Die Policy erlaubt standardmäßig überlappende Crypto Periods. Bei `allow_overlapping_crypto_periods=false` blockiert eine Überschneidung mit einem anderen aktiven Schlüssel desselben Scopes die Aktivierung. `auto_retire_predecessor=true` setzt bei Aktivierung andere aktive Schlüssel desselben Scopes auf `retired`.

Crypto-Period-Metadaten und `activate_at` planen den Übergang; der Prozess startet keinen Timer, der staged Schlüssel zur Startzeit automatisch aktiviert. Eine erfolgreiche Edge-Zustellung und die anschließende Aktivierung sind getrennte Schritte.

## Zeitbezogene Wartung

`POST /api/v1/maintenance/tick`:

- setzt abgelaufene `active`- und `retiring`-Schlüssel auf `retired`;
- beendet abgelaufene Jobs und Aktionen;
- gibt nicht quittierte In-Flight-Aktionen nach Wartezeit erneut frei oder markiert sie nach dem Versuchslimit als fehlgeschlagen.

Der Endpoint muss im Lab ausdrücklich aufgerufen werden; KMF hat keinen eigenen periodischen Wartungsworker.

**Quellabgleich vom 9. Oktober 2026:** [Lifecycle- und Wartungslogik](../../../system-backend/kmf/src/state.rs), [Policy-Beispiel](../../../system-backend/kmf/config/kmf.example.toml). Weiter: [OTAR-Workflow](otar-zustellablauf.md).
