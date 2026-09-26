# Mehrzellenbetrieb, Mobility und Edge-Fallback

Mehrere TBS können über den Node Gateway zu einem verteilten Netz gehören. Mobility Core kennt aktuelle Serving-TBS und Zellzustände; Transit behandelt Regionen/Peers. Die konkrete Umschaltung eines **laufenden** Funkrufs erfordert zusätzlich zusammenhängende MM/CMCE-Restore-Kontexte, neue Rufzweige und einen gültigen Medienpfad. Das ist nicht allein durch zwei erreichbare Basisstationen bewiesen.

## Was heute auseinanderzuhalten ist

| Vorgang | Aussage |
|---|---|
| Registrierung in einer anderen Zelle | am Endgerät mit Zell-/Standortparametern und Logs nachweisen |
| Neuer Ruf zur aktuellen Serving-TBS | Mobility- und SIP-/Call-Control-Route prüfen |
| Kontexttransfer/Restore-PDUs im Code | vorhandene Implementierung sagt noch nichts über On-Air-Abnahme |
| Laufender Ruf ohne hörbare Unterbrechung von A nach B | eigener E2E-Funk- und Medientest; für MAIN-COMPAT nicht pauschal zugesichert |

Der [Rollout-Stand](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/CENTRAL_NETWORK_ROLLOUT.md) nennt die Grenzen der aktuellen zentralen Medienbrücke. [Mobility-Core-Docs](https://github.com/JanHG98/netcore-tetra/tree/main/system-backend/mobility-core/docs) enthalten den Kontexttransfervertrag. [[Projektstand]]

## Ausfall einer Zentrale

Die TBS verwendet Service-Matrix, Lease und Hysterese. In der [sanitisierten Beispielkonfiguration](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/basisstation.config.sanitized.example.toml) sind `enter_after_secs`, `recover_after_secs`, `service_matrix_lease_secs` und `required_services` sichtbar. Ein Ausfall einzelner Fachkerne kann `degraded` auslösen; ein Gateway-Ausfall `isolated`. Lokal verfügbare Rufe, Registrierung, SDS und bereits installierte Schlüssel folgen dem jeweiligen Fallback; netzweites Routing oder OTAR kann ausfallen.

Bei Rückkehr: Service-Matrix aktualisieren, Spool/Queues und Deduplizierung prüfen, dann Registrierung, Affiliation, neue Rufe und gegebenenfalls SIP-/SDS-Pfade testen. Es werden keine alten Sprachframes nachgeholt. [[Backup-and-Fallback]] · [[Troubleshooting]]
