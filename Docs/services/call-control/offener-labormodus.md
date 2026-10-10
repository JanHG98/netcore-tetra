# Open-Lab-Modus

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/config.rs](../../../system-backend/call-control/src/config.rs) · [src/http.rs](../../../system-backend/call-control/src/http.rs) · [src/state.rs](../../../system-backend/call-control/src/state.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

Call Control läuft in diesem Paket ausschließlich mit `security.mode = "open_lab"`.

Es existieren keine Tokens, Passwörter, Benutzerkonten, Client-Zertifikate oder TLS-Endpunkte. Jeder Client mit Netzwerkzugriff auf Port 8120 kann Rufe starten und beenden, Floor-Zustände ändern und Restore Context koordinieren.

Andere Security-Modi werden beim Start abgewiesen. Dadurch entsteht keine Scheinsicherheit durch unvollständige Tokenfelder.

Der LXC gehört bis zur Integration abgesicherter Dienstidentitäten und RBAC in ein isoliertes Managementnetz beziehungsweise eine Test-VLAN.

## Management ist getrennt von Funk-Sicherheit

Dieser Abschnitt beschreibt die HTTP-/WebSocket-Erreichbarkeit des Dienstes. Funkseitige Teilnehmerauthentisierung oder ein vorhandener Security Core/KMF schützen diese Managementschnittstelle nicht automatisch. Die Beispielkonfiguration aktiviert den offenen Managementzugriff; Änderungen an Sicherheitsoptionen anhand der verlinkten Konfiguration und Implementierung prüfen.
