# Auftragsbearbeitung

**Quellen:** [system-backend/task-workflow](../../../system-backend/task-workflow) · [Repository-Root](../../..). Bei Befehlen das in der Anleitung angegebene Arbeitsverzeichnis beachten.

Stand: **9. Oktober 2026**. Die Anleitung beschreibt die im Hauptzweig vorhandene Umsetzung; Anlagen- und Funkabnahmen stehen in der [Gesamtroadmap](../../roadmaps/gesamtroadmap.md).

Task Workflow (historisch Phase 9) verwaltet strukturierte Aufträge und kompakte WAP-Formulare. Der Dienst läuft als eigener LXC auf Port `8280` und bleibt im OPEN-LAB-Modus ohne Login, Token und TLS.

## Funktionen

- strukturierte Aufträge mit `netcore-task-v1`
- Statusfolge `open → assigned → accepted → in_progress/blocked → completed`
- Vorlagen für Störung, Fahrzeugcheck, Materialentnahme, Check-in/out und Wartungsquittierung
- XHTML-Basic- und WML-Formulare unter `/x` und `/w`
- REST-API und WebUI
- SDS-Benachrichtigung über den zentralen SDS Router
- SDS-Kommandos `TAKE`, `START`, `BLOCK`, `DONE`, `CANCEL`, `REOPEN`, `INFO`
- pre-coded Status 5301 bis 5305
- MQTT-Ereignisse und retained Task-Zustände
- Persistenz, Audit und Prometheus

## Einstieg

```text
http://<LXC-IP>:8280/
http://<LXC-IP>:8280/x?issi=4010001
http://<LXC-IP>:8280/w?issi=4010001
```

Die `issi`-Angabe ist im OPEN LAB nur eine ungeschützte Identitätsangabe und kein Authentisierungsmerkmal.

## Installation und Betriebsdaten

Vom Repository-Hauptverzeichnis als root:

```bash
sudo bash system-backend/task-workflow/install/install.sh
```

Konfiguration: `/etc/netcore/task-workflow.toml`. Zustand, Ereignisse und Audit liegen unter `/var/lib/netcore-task-workflow/`. Bei getrennten LXCs Broker- und SDS-Router-Adresse ersetzen. Die Vorlagen und Statusaktionen werden aus der TOML geladen; die Beispiel-ISSI/GSSI sind vor der Nutzung anzupassen.

Weitere Anleitungen: [Architektur](architektur.md) und [Funktionsprüfung](tests/funktionspruefung-im-labor.md). Ein API-Auftrag kann bei aktivierter SDS-Anbindung eine echte Benachrichtigung auslösen; für Tests die vorgesehenen Lab-Empfänger verwenden.
