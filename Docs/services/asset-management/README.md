# Inventar und Materialverwaltung

**Quellen:** [system-backend/asset-management](../../../system-backend/asset-management) · [Repository-Root](../../..). Bei Befehlen das in der Anleitung angegebene Arbeitsverzeichnis beachten.

Stand: **9. Oktober 2026**. Die Anleitung beschreibt die im Hauptzweig vorhandene Umsetzung; Anlagen- und Funkabnahmen stehen in der [Gesamtroadmap](../../roadmaps/gesamtroadmap.md).

Asset Management (historisch Phase 10) verwaltet physische Assets, Funkgeräte, Personen, Ausgaben und Wartungsakten.

## Zuständigkeitsgrenze

- **Subscriber Core** bleibt autoritativ für ISSI-Freigabe und Dienstberechtigungen.
- **Mobility Core** bleibt autoritativ für die aktuell bedienende TBS.
- **Asset Management** besitzt Inventarnummer, Seriennummer, Firmware-/Codeplugstand, physische Zuordnung und Wartung.
- RUI/RUA-Felder sind in dieser Phase nur Metadaten. Es werden keine PINs gespeichert und keine Netz-Anmeldung ausgelöst.

WebUI: `http://<LXC-IP>:8290/`

OPEN LAB: kein Login, keine Tokens, kein TLS.

## Installation und Betrieb

Vom Repository-Hauptverzeichnis als root:

```bash
sudo bash system-backend/asset-management/install/install.sh
```

Konfiguration: `/etc/netcore/asset-management.toml`. Persistenz: `/var/lib/netcore-asset-management/{state.json,events.ndjson,audit.ndjson}`. Die Upstream-URLs in der Vorlage stehen auf Loopback und müssen bei getrennten LXCs ersetzt werden. Der Subscriber-/Mobility-Abgleich liest Profile und Routen; Wartungsaufträge können über den Task Workflow angelegt werden.

Die API bietet `/api/v1/assets`, `/api/v1/persons`, `/api/v1/assignments`, `/api/v1/maintenance`, `/api/v1/reconcile` sowie JSON-/CSV-Export. Ausgabe und Rückgabe dokumentieren die physische Zuordnung und schalten kein Funkgerät im Netz frei.

Weitere Anleitungen: [Architektur](architektur.md) und [Funktionsprüfung](tests/funktionspruefung-im-labor.md).
