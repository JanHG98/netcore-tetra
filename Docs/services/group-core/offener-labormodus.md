# Open-Lab-Modus

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/config.rs](../../../system-backend/group-core/src/config.rs) · [src/http.rs](../../../system-backend/group-core/src/http.rs) · [src/state.rs](../../../system-backend/group-core/src/state.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

Der Group Core läuft in dieser Phase ausschließlich mit `security.mode = "open_lab"`.

Es gibt keine Tokens, Passwörter, Anmeldung, TLS oder Client-Zertifikate. Jeder Client mit Netzwerkzugriff kann Gruppenrichtlinien und DGNA verändern. Der Dienst gehört deshalb nur in ein isoliertes Test-VLAN.

## Management ist getrennt von Funk-Sicherheit

Dieser Abschnitt beschreibt die HTTP-/WebSocket-Erreichbarkeit des Dienstes. Funkseitige Teilnehmerauthentisierung oder ein vorhandener Security Core/KMF schützen diese Managementschnittstelle nicht automatisch. Die Beispielkonfiguration aktiviert den offenen Managementzugriff; Änderungen an Sicherheitsoptionen anhand der verlinkten Konfiguration und Implementierung prüfen.
