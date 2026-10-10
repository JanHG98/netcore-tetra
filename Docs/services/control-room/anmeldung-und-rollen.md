# Leitstelle: Anmeldung und Rollen

Die Rust-Codebasis enthält bereits Benutzer/Passwort und Rollen `viewer`, `operator`, `admin`. Diese Funktionen sind in der aktuellen Open-Lab-Phase absichtlich deaktiviert.

Ausgeliefertes Open-Lab-Profil:

```toml
[auth]
enabled = false
node_token_env = ""
bootstrap_username_env = ""
bootstrap_password_env = ""
```

und im systemd-Service:

```text
--no-auth
```

Vor Produktivbetrieb müssen Authentisierung, TLS, Maschinenidentität, Audit-Schutz und Rollenmodell gemeinsam aktiviert und getestet werden. Einzelne Tokens halb einzuschalten wäre Scheinsicherheit.

**Quellabgleich: 9. Oktober 2026.** [Quellcode](../../../bins/netcore-control-room/src) · [Konfigurationsvorlagen](../../../system-backend/control-room/config).
