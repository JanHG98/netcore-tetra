# Open-Lab-Modus

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/config.rs](../../../system-backend/subscriber-core/src/config.rs) · [src/http.rs](../../../system-backend/subscriber-core/src/http.rs) · [src/state.rs](../../../system-backend/subscriber-core/src/state.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

Der Subscriber Core besitzt in dieser Ausbaustufe keine Authentisierung und kein TLS. Die WebUI zeigt dies dauerhaft an. `security.mode` muss `open_lab` sein; andere Werte werden beim Start abgewiesen.

Der Container gehört ausschließlich in ein isoliertes Management-/Testnetz. Port 8100 darf nicht ins Internet oder in unvertrauenswürdige Netze veröffentlicht werden.

## Management ist getrennt von Funk-Sicherheit

Dieser Abschnitt beschreibt die HTTP-/WebSocket-Erreichbarkeit des Dienstes. Funkseitige Teilnehmerauthentisierung oder ein vorhandener Security Core/KMF schützen diese Managementschnittstelle nicht automatisch. Die Beispielkonfiguration aktiviert den offenen Managementzugriff; Änderungen an Sicherheitsoptionen anhand der verlinkten Konfiguration und Implementierung prüfen.
