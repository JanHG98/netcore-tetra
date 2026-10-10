# Open-Lab-Modus

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/config.rs](../../../system-backend/media-switch/src/config.rs) · [src/http.rs](../../../system-backend/media-switch/src/http.rs) · [src/state.rs](../../../system-backend/media-switch/src/state.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

Der Media Switch besitzt in diesem Paket absichtlich keine Tokens, Passwörter, Benutzerkonten, Client-Zertifikate oder TLS-Konfiguration.

Jeder Client, der Port 8130 oder den Backend-WebSocket des Node Gateways erreicht, kann Medienströme beobachten und über die Management-API stummschalten, puffern oder Testframes einspeisen. Der Dienst darf daher nur in einem isolierten Testnetz betrieben werden.

`security.mode` akzeptiert ausschließlich `open_lab`. Eine andere Angabe stoppt den Start, statt Scheinsicherheit vorzutäuschen.

## Management ist getrennt von Funk-Sicherheit

Dieser Abschnitt beschreibt die HTTP-/WebSocket-Erreichbarkeit des Dienstes. Funkseitige Teilnehmerauthentisierung oder ein vorhandener Security Core/KMF schützen diese Managementschnittstelle nicht automatisch. Die Beispielkonfiguration aktiviert den offenen Managementzugriff; Änderungen an Sicherheitsoptionen anhand der verlinkten Konfiguration und Implementierung prüfen.
