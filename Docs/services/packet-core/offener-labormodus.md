# Open-Lab-Modus

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/config.rs](../../../system-backend/packet-core/src/config.rs) · [src/http.rs](../../../system-backend/packet-core/src/http.rs) · [src/state.rs](../../../system-backend/packet-core/src/state.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

Der aktuelle Packet Core hat absichtlich:

- keine Benutzerkonten,
- keine Token,
- keine Session-Authentifizierung,
- kein TLS,
- keine rollenbasierte Freigabe.

Damit kann jeder erreichbare Client Kontexte deaktivieren, Teilnehmer pagen, Zustände modifizieren und Payloads lesen oder einspeisen. `security.expose_payloads` ist zwar ein Konfigurationsfeld, wird im aktuellen HTTP-/State-Pfad aber nicht als Zugriffsschutz ausgewertet. Ein Wert `false` darf deshalb nicht als Payload-Sperre behandelt werden. Das ist für die offene Testumgebung gewollt, aber für einen produktiven oder fremd erreichbaren Betrieb nicht akzeptabel.

Vor einem produktiven Einsatz müssen mindestens Management-Netztrennung, mTLS oder ein vergleichbarer Dienstidentitätsmechanismus, RBAC und Audit-Trails ergänzt werden.

## Management ist getrennt von Funk-Sicherheit

Dieser Abschnitt beschreibt die HTTP-/WebSocket-Erreichbarkeit des Dienstes. Funkseitige Teilnehmerauthentisierung oder ein vorhandener Security Core/KMF schützen diese Managementschnittstelle nicht automatisch. Die Beispielkonfiguration aktiviert den offenen Managementzugriff; Änderungen an Sicherheitsoptionen anhand der verlinkten Konfiguration und Implementierung prüfen.
