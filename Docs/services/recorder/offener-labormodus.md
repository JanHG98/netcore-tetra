# Open-Lab-Modus

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/config.rs](../../../system-backend/recorder/src/config.rs) · [src/http.rs](../../../system-backend/recorder/src/http.rs) · [src/state.rs](../../../system-backend/recorder/src/state.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

Diese Recorder-Ausbaustufe akzeptiert ausschließlich `security.mode = "open_lab"`. Ein anderer Wert beendet den Start mit einem Konfigurationsfehler.

Es gibt bewusst keine Authentifizierung, Tokenprüfung, Benutzerverwaltung, Zertifikate oder TLS-Terminierung. Die WebUI zeigt dauerhaft einen roten Warnbalken. Auch HTTP-Antworten tragen `X-NetCore-Security-Mode: open_lab`.

## Konsequenz

Jeder erreichbare Client kann Aufnahmen lesen und exportieren sowie – abhängig von `allow_remote_management` und `allow_delete` – Retention, Legal Hold, Finalisierung und Löschung steuern.

## Mindestschutz im Testnetz

- eigener LXC
- eigenes isoliertes Backend-/Management-VLAN
- kein Port-Forwarding aus dem Internet
- Firewall nur für Administratoren und die Leitstelle
- Storage nicht gleichzeitig als ungeschütztes SMB/NFS veröffentlichen

Security Core und KMF sind vorhanden, sichern aber diesen Recorder-HTTP-Zugriff nicht ab. Authentifizierung, Autorisierung, nachvollziehbare Benutzeridentitäten und Transportverschlüsselung für den Aufzeichnungszugriff bleiben offene Integrationsaufgaben. `allow_delete` sperrt nur manuelle Löschungen; automatische Retention bleibt aktiv.

## Management ist getrennt von Funk-Sicherheit

Dieser Abschnitt beschreibt die HTTP-/WebSocket-Erreichbarkeit des Dienstes. Funkseitige Teilnehmerauthentisierung oder ein vorhandener Security Core/KMF schützen diese Managementschnittstelle nicht automatisch. Die Beispielkonfiguration aktiviert den offenen Managementzugriff; Änderungen an Sicherheitsoptionen anhand der verlinkten Konfiguration und Implementierung prüfen.
