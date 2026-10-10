# Open-Lab-Modus

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/config.rs](../../../system-backend/sds-router/src/config.rs) · [src/http.rs](../../../system-backend/sds-router/src/http.rs) · [src/state.rs](../../../system-backend/sds-router/src/state.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

Dieser Ausbaustand enthält absichtlich noch keine Tokens, Benutzerkonten, Rollen oder TLS. Das ist keine Produktivkonfiguration.

Besonders kritisch ist beim SDS Router, dass jeder erreichbare Client:

- SDS- und Statusinhalte lesen,
- neue Nachrichten aussenden,
- Routingregeln ändern,
- Offline- und Dead-Letter-Nachrichten erneut zustellen,
- Anwendungslegs bestätigen kann.

Der Dienst muss daher in einem isolierten Labor-VLAN betrieben werden. Vor einem produktiven Einsatz sind mindestens mTLS oder signierte Dienstidentitäten, RBAC, Audit-Trails, Payload-Masking und verschlüsselte Datenträger vorzusehen.

## Management ist getrennt von Funk-Sicherheit

Dieser Abschnitt beschreibt die HTTP-/WebSocket-Erreichbarkeit des Dienstes. Funkseitige Teilnehmerauthentisierung oder ein vorhandener Security Core/KMF schützen diese Managementschnittstelle nicht automatisch. Die Beispielkonfiguration aktiviert den offenen Managementzugriff; Änderungen an Sicherheitsoptionen anhand der verlinkten Konfiguration und Implementierung prüfen.
