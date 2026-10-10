# Mehrzellenbetrieb, Mobility und Edge-Fallback

Mehrere TBS können über den Node Gateway zu einem verteilten Netz gehören. Mobility Core kennt aktuelle Serving-TBS und Zellzustände; Transit behandelt Regionen/Peers. Die konkrete Umschaltung eines **laufenden** Funkrufs erfordert zusätzlich zusammenhängende MM/CMCE-Restore-Kontexte, neue Rufzweige und einen gültigen Medienpfad. Das ist nicht allein durch zwei erreichbare Basisstationen bewiesen.

## Was heute auseinanderzuhalten ist

| Vorgang | Aussage |
|---|---|
| Registrierung in einer anderen Zelle | am Endgerät mit Zell-/Standortparametern und Logs nachweisen |
| Neuer Ruf zur aktuellen Serving-TBS | Mobility- und SIP-/Call-Control-Route prüfen |
| Kontexttransfer/Restore-PDUs im Code | vorhandene Implementierung sagt noch nichts über On-Air-Abnahme |
| Laufender Ruf ohne hörbare Unterbrechung von A nach B | eigener E2E-Funk- und Medientest; für MAIN-COMPAT nicht pauschal zugesichert |

Der [Rollout-Stand](../roadmaps/zentraler-netzbetrieb.md) nennt die Grenzen der aktuellen zentralen Medienbrücke. [Mobility-Core-Docs](../services/mobility-core/README.md) enthalten den Kontexttransfervertrag. Der zentrale Mobility-Transfer besitzt eine Core-Orchestrierung, seine Export-/Import-/Remove-Kommandos werden vom aktiven TBS-MM noch nicht verarbeitet. Für Teilnehmerpolicy gilt `subscriber_policy = false`; die lokale MM-Whitelist bleibt wirksam.

[Projektstand und Nachweisgrenzen](projektstand-und-nachweise.md)

## Ausfall einer Zentrale

Die TBS verwendet Service-Matrix, Lease und Hysterese. In der [sanitisierten Beispielkonfiguration](../basisstation.config.sanitized.example.toml) sind `enter_after_secs`, `recover_after_secs`, `service_matrix_lease_secs` und `required_services` sichtbar. Ein Ausfall einzelner Fachkerne kann `degraded` auslösen; ein Gateway-Ausfall `isolated`. Lokal verfügbare Rufe, Registrierung, SDS und bereits installierte Schlüssel folgen dem jeweiligen Fallback; netzweites Routing oder OTAR kann ausfallen.

Bei Rückkehr: Service-Matrix aktualisieren, Spool/Queues und Deduplizierung prüfen, dann Registrierung, Affiliation, neue Rufe und gegebenenfalls SIP-/SDS-Pfade testen. Es werden keine alten Sprachframes nachgeholt. [Backup und Fallback](datensicherung-und-fallback.md) · [Fehlersuche](fehlersuche.md)

Für den Ausbau zunächst Registrierung und neue Rufe zur richtigen Serving-TBS abnehmen; laufende Rufmigration folgt als eigener Funk-/Medientest. Der vorhandene Transit-Dienst ist eine NetCore-Regionenkopplung, kein pauschaler Nachweis einer normgerechten ETSI-ISI-Verbindung.

## Quellen zur Pflege dieser Seite

[Aktiver MAIN-COMPAT-Einstieg](../../bins/bluestation-bs/src/main.rs) · [Service-Matrix und Fallback](../../crates/tetra-entities/src/net_control_room/worker.rs) · [Fallback-Konfiguration](../../crates/tetra-config/src/bluestation/sec_edge_fallback.rs).
