# KMF – Offene Testumgebung

Die KMF akzeptiert aktuell nur `security.mode="open_lab"` und `vault.provider="lab_file_vault"`. Der HTTP-Listener auf TCP **8190** besitzt keine Anmeldung, Tokens oder TLS. Damit kann jeder erreichbare Client Schlüssel erzeugen, rotieren, aktivieren, widerrufen oder zerstören, Node-Bootstraps anlegen, OTAR freigeben und Backups auslösen.

## Konfiguration und Grenzen

Die Beispielkonfiguration bindet an `0.0.0.0:8190` und verlangt ein isoliertes Lab-Managementnetz. Für ausschließlich lokale Tests gemeinsam setzen:

```toml
[server]
bind = "127.0.0.1:8190"

[security]
mode = "open_lab"
allow_remote_management = false
expose_raw_keys = false
```

Die Prüfung erzwingt bei deaktiviertem Remote-Management einen Loopback-Bind und lehnt `expose_raw_keys=true` ab. Dies ersetzt keine Benutzer- oder Geräteidentität. `shadow` unterbindet nur Edge-Claims, nicht die übrigen Managementaktionen.

## Was die Lab-Grenze tatsächlich schützt

Managementantworten enthalten keine Rohschlüssel oder Bootstrap-Geheimnisse. OTAR-Claims liefern nodegebundene Lab-Envelopes; die lokale Bootstrap-Datei bleibt separat zu schützen. Zwei verschiedene Actor-Namen verhindern doppelte Freigabe mit demselben Namen, verifizieren aber keine unterschiedlichen Menschen.

TLS/mTLS, zentrale Anmeldung und RBAC, verifizierte Vier-Augen-Identitäten, HSM/PKCS#11, Schlüsselzeremonien und produktive Audit-Sicherung sind offene Ausbauschritte. Weder ein gesetzter HSM-Bibliothekspfad noch der authoritative-Modus implementiert diese Funktionen.

**Quellabgleich vom 9. Oktober 2026:** [Konfigurationsprüfung](../../../system-backend/kmf/src/config.rs), [API](../../../system-backend/kmf/src/http.rs) und [Claim-/Freigabelogik](../../../system-backend/kmf/src/state.rs).
