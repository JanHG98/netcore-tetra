# NetCore Asset Management

Phase 10 verwaltet physische Assets, Funkgeräte, Personen, Ausgaben und Wartungsakten.

## Zuständigkeitsgrenze

- **Subscriber Core** bleibt autoritativ für ISSI-Freigabe und Dienstberechtigungen.
- **Mobility Core** bleibt autoritativ für die aktuell bedienende TBS.
- **Asset Management** besitzt Inventarnummer, Seriennummer, Firmware-/Codeplugstand, physische Zuordnung und Wartung.
- RUI/RUA-Felder sind in dieser Phase nur Metadaten. Es werden keine PINs gespeichert und keine Netz-Anmeldung ausgelöst.

WebUI: `http://<LXC-IP>:8290/`

OPEN LAB: kein Login, keine Tokens, kein TLS.
