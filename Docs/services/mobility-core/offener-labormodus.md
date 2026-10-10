# Offener Labormodus

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/config.rs](../../../system-backend/mobility-core/src/config.rs) · [src/http.rs](../../../system-backend/mobility-core/src/http.rs) · [src/state.rs](../../../system-backend/mobility-core/src/state.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

Der Mobility Core läuft in dieser Ausbaustufe ausschließlich im Modus `open_lab`.

Es gibt keine Tokens, Benutzerkonten, Passwörter, Client-Zertifikate oder TLS-Verbindung. Bei `allow_remote_management = true` kann jeder Client, der die WebUI oder REST-API erreicht, Context Transfers auslösen. Ein Abbruch ist nur vor bestätigtem Zielimport möglich. Das Abschalten einzelner Managementfunktionen ersetzt keine Authentisierung.

Der Dienst darf deshalb nur in einem isolierten Test- beziehungsweise Managementnetz betrieben werden. Ein anderer Wert als `security.mode = "open_lab"` führt absichtlich zum Startabbruch.

## Management ist getrennt von Funk-Sicherheit

Dieser Abschnitt beschreibt die HTTP-/WebSocket-Erreichbarkeit des Dienstes. Funkseitige Teilnehmerauthentisierung oder ein vorhandener Security Core/KMF schützen diese Managementschnittstelle nicht automatisch. Die Beispielkonfiguration aktiviert den offenen Managementzugriff; Änderungen an Sicherheitsoptionen anhand der verlinkten Konfiguration und Implementierung prüfen.
