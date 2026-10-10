# Open-Lab-Modus

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/config.rs](../../../system-backend/ip-gateway/src/config.rs) · [src/http.rs](../../../system-backend/ip-gateway/src/http.rs) · [src/state.rs](../../../system-backend/ip-gateway/src/state.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

Dieser Zwischenstand implementiert absichtlich keine Konten, Token, mTLS oder TLS. Das betrifft nicht nur lesende Diagnosezugriffe: Ein erreichbarer Client kann Routen, NAT, Firewall, DNS, Blocklisten und Captures verändern sowie einen Kernel-Reconcile auslösen.

Daher gelten bis zum Security-Ausbau:

- eigener isolierter Management-VLAN,
- keine Portweiterleitung aus fremden Netzen,
- WebUI nicht direkt aus dem TETRA-Datennetz freigeben,
- Konfigurationsdatei und State-Verzeichnis nur für root/netcore,
- `shadow` als sicherer Startmodus.

## Management ist getrennt von Funk-Sicherheit

Dieser Abschnitt beschreibt die HTTP-/WebSocket-Erreichbarkeit des Dienstes. Funkseitige Teilnehmerauthentisierung oder ein vorhandener Security Core/KMF schützen diese Managementschnittstelle nicht automatisch. Die Beispielkonfiguration aktiviert den offenen Managementzugriff; Änderungen an Sicherheitsoptionen anhand der verlinkten Konfiguration und Implementierung prüfen.
