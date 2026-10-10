# Security Core – Offene Testumgebung

Der Security Core akzeptiert aktuell nur `security.mode="open_lab"`. TCP **8180** bietet keine Anmeldung, Tokens oder TLS. Jeder erreichbare Client kann Richtlinien ändern, Teilnehmer sperren, Kontexte widerrufen und im authoritative-Modus passende Edge-Aktionen beanspruchen.

## Konkrete Betriebsgrenze

Die [Beispielkonfiguration](../../../system-backend/security-core/config/security-core.example.toml) bindet an `0.0.0.0:8180` und erlaubt Remote-Management. Deshalb den Dienst nur im isolierten Lab-Netz mit freigegebenen Testhosts betreiben; keine Internet-Portweiterleitung oder produktiven Schlüssel- und Teilnehmerbestände verwenden.

Für einen ausschließlich lokalen Test müssen **beide** Werte zusammenpassen:

```toml
[server]
bind = "127.0.0.1:8180"

[security]
mode = "open_lab"
allow_remote_management = false
expose_ephemeral_edge_material = true
```

`allow_remote_management=false` verlangt einen Loopback-Bind. Es fügt keine Benutzerprüfung hinzu. `expose_ephemeral_edge_material=false` sperrt die Secret-Ausgabe über Claims; damit lässt sich der vollständige Challenge-/DCK-Edge-Workflow nicht durchführen.

## Shadow bleibt ein Arbeitsmodus

`shadow` verhindert die Herausgabe von Edge-Aktionen, aber weder Konfigurationsänderungen noch die Erzeugung von Lab-Kontexten und Audit-Daten. Auch die String-Zuordnung `node_id` ist keine authentisierte Knotenidentität. Produktive Anmeldung, TLS und ein produktiver Authentisierungsprovider sind derzeit offene Implementierungsschritte.

**Quellabgleich vom 9. Oktober 2026:** [Konfigurationsprüfung](../../../system-backend/security-core/src/config.rs), [HTTP-Listener](../../../system-backend/security-core/src/http.rs) und [Claims](../../../system-backend/security-core/src/state.rs).
