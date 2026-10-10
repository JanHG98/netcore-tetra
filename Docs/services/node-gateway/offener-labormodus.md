# Offener Testmodus

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/config.rs](../../../system-backend/node-gateway/src/config.rs) · [src/http.rs](../../../system-backend/node-gateway/src/http.rs) · [src/state.rs](../../../system-backend/node-gateway/src/state.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

Der Node Gateway wird in dieser Ausbaustufe absichtlich ohne Authentifizierung betrieben.

## Nicht vorhanden

- keine Node-Tokens
- keine Benutzerkonten
- keine Passwörter
- keine API-Schlüssel
- keine Client-Zertifikate
- kein TLS
- kein RBAC

Alle erreichbaren Clients können Statusdaten lesen. Wenn `allow_remote_management = true` gesetzt ist, können sie außerdem Nodes trennen und TBS-Kommandos auslösen.

## Verbindliche Schutzmaßnahme

Der LXC darf nur in einem isolierten Test- beziehungsweise Managementnetz erreichbar sein. Port `8080/tcp` darf nicht aus dem Internet oder aus unkontrollierten Clientnetzen erreichbar sein.

## Bewusste technische Sicherung

Die Konfiguration akzeptiert ausschließlich:

```toml
[security]
mode = "open_lab"
```

Andere Werte führen zu einem Startfehler. Dadurch wird kein noch nicht implementierter Token-Modus als vermeintlich sicherer Produktivbetrieb dargestellt.

Ein eigener Security Core und KMF existieren bereits für Funk- und Schlüsselrichtlinien. Sie authentifizieren diesen Management-WebSocket jedoch nicht automatisch. Ein abgesicherter Transport-/Managementmodus bleibt eine offene Integrationsaufgabe.

## Management ist getrennt von Funk-Sicherheit

Dieser Abschnitt beschreibt die HTTP-/WebSocket-Erreichbarkeit des Dienstes. Funkseitige Teilnehmerauthentisierung oder ein vorhandener Security Core/KMF schützen diese Managementschnittstelle nicht automatisch. Die Beispielkonfiguration aktiviert den offenen Managementzugriff; Änderungen an Sicherheitsoptionen anhand der verlinkten Konfiguration und Implementierung prüfen.
