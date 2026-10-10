# Security Core – Architektur und Zuständigkeiten

Der Security Core hält Sicherheitsrichtlinien, Teilnehmerprofile, Authentisierungs- und DCK-Metadaten. Der Node-Gateway-Worker beobachtet verfügbare Knoten; ein Edge-Adapter holt Challenge-, DCK-, Sperr- oder Widerrufsaktionen über die eigene REST-API ab. Ein automatisch angeschlossener TETRA-TA- oder KMF-Provider ist derzeit nicht vorhanden.

| Baustein | Vorhandene Verantwortung |
| --- | --- |
| Security Core | Security Classes, Lab-Authentisierung, DCK-Kontexte, Disable/Enable, Alarme |
| KMF | CCK/GCK/SCK, Versionen, Crypto Periods, Rotation und Lab-OTAR-Orchestrierung |
| Node Gateway | Knotenbeobachtung über `ws://…/ws/backend` |
| Edge-Adapter | Aktionen abrufen, lokal umsetzen und explizit quittieren |

## Daten und Geheimnisse

`state.json` speichert Metadaten, Richtlinien und eine durch `limits.max_audit` begrenzte Audit-Historie. Challenge, erwartete Antwort, DCK-Material und Aktionspayloads liegen in separaten Speicherstrukturen des laufenden Prozesses. Der lokale `lab-auth.seed` bleibt als Wurzel des Lab-Providers auf dem Dateisystem.

Ein Neustart verwirft den flüchtigen Geheimzustand: offene Authentisierungen werden `expired`, offene Aktionen `failed`, installierte oder ausstehende DCK-Kontexte `revoked`. Eine Edge muss danach erneut authentisieren; das Metadatenbackup stellt keinen aktiven DCK wieder her.

## Schnittstellengrenze

`netcore-security-edge-v1` ist ein Lab-Steuerprotokoll. Die Auswahl von Class 2/3 in der Policy ist kein Nachweis implementierter TETRA-Authentisierungsalgorithmen oder tatsächlich aktivierter Funkverschlüsselung. Die [KMF-Architektur](../kmf/architektur-und-geheimnisfluss.md) ergänzt die Schlüsselverwaltung, ohne diese Grenze aufzuheben.

**Quellabgleich vom 9. Oktober 2026:** [state.rs](../../../system-backend/security-core/src/state.rs), [gateway.rs](../../../system-backend/security-core/src/gateway.rs) und [protocol.rs](../../../system-backend/security-core/src/protocol.rs).
