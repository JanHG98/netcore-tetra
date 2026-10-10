# Transit – Offene Testumgebung

Transit bietet TCP **8200** gemeinsam für WebUI, Management und Peer-Ingress. Aktuell sind keine Anmeldung, Tokens, TLS, mTLS-Peeridentität oder signierten Routinginformationen implementiert. Die [Beispielkonfiguration](../../../system-backend/transit/config/transit.example.toml) bindet an alle IPv4-Schnittstellen.

## Was geprüft wird

Ein Peer-Envelope benötigt die passende Protokollkennung. Standardmäßig muss der vorherige Hop zu einer konfigurierten Peer-Region gehören; `allow_dynamic_peers=false` verhindert unbekannte Hop-Regionen. Trace, Hop Limit, TTL und Dedupe werden geprüft.

Diese Prüfungen authentisieren den Absender nicht: ein erreichbarer Client kann vorhandene Region-IDs deklarieren. Ebenso kann er ohne Anmeldung Managementdaten verändern und Peer-Endpunkte ansprechen. Der Dienst gehört daher in ein isoliertes Testnetz ohne Internet-Portweiterleitung und ohne produktive Einsatzdaten oder Schlüssel.

## Lokal begrenzen

Für einen Einzelhost-Test müssen beide Einstellungen zusammenpassen:

```toml
[server]
bind = "127.0.0.1:8200"

[security]
mode = "open_lab"
allow_remote_management = false
tls = false
token_auth = false
```

Ein Zweiregionentest auf getrennten Hosts benötigt hingegen eine erreichbare Lab-Adresse und entsprechende Firewall-Freigaben. `shadow` verhindert ausgehende Envelopes und Heartbeats, aber keine eingehenden API-Aufrufe. Die Konfigurationsprüfung lehnt `tls=true` und `token_auth=true` ab; diese Flags aktivieren keinen implementierten Schutzstack.

Produktive Peer-Identitäten, PKI, Zugriffskontrolle und belastbare Audit-Identitäten bleiben eigene Ausbauarbeiten.

**Quellabgleich vom 9. Oktober 2026:** [Konfigurationsprüfung](../../../system-backend/transit/src/config.rs), [HTTP-API](../../../system-backend/transit/src/http.rs) und [Peer-Ingress](../../../system-backend/transit/src/state.rs).
