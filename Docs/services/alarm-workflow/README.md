# Alarmbearbeitung und Eskalation

**Quellen:** [system-backend/alarm-workflow](../../../system-backend/alarm-workflow) · [Repository-Root](../../..). Bei Befehlen das in der Anleitung angegebene Arbeitsverzeichnis beachten.

Stand: **9. Oktober 2026**. Die Anleitung beschreibt die im Hauptzweig vorhandene Umsetzung; Anlagen- und Funkabnahmen stehen in der [Gesamtroadmap](../../roadmaps/gesamtroadmap.md).

Der Alarm Workflow (historisch Phase 8) verbindet `netcore-event-v1`, MQTT, den zentralen SDS Router und pre-coded Status zu einem persistenten Alarm- und Eskalationsdienst.

## Funktionen

- Regeln für `raise` und `clear` auf beliebige NetCore-Ereignisse
- zustandsbasierte Alarm-Deduplizierung
- Alarmzustände `open`, `acknowledged`, `assigned`, `in_progress`, `resolved`, `closed`, `cancelled`
- zeitgesteuerte Eskalationsprofile
- SDS-Benachrichtigungen an ISSI oder GSSI über den vorhandenen SDS Router
- Verfolgung des SDS-Zustands
- ACK/TAKE/START/RESOLVE/CLOSE per SDS-Text mit Alarmtoken
- frei konfigurierbare pre-coded Statusaktionen
- persistente Alarmakte, Ereignisse und Auditlog
- MQTT-Ereignisse und retained Alarmzustände
- WebUI, REST, OpenAPI, Health und Prometheus

## Open Lab

Der Dienst verwendet absichtlich keine Anmeldung, Tokens oder TLS. Jeder Client mit Netzzugriff kann Alarme anlegen, quittieren, übernehmen, lösen und schließen. Nur in einem isolierten Testnetz verwenden.

Standardport: `8270`.

## Installation und Konfiguration

Vom Repository-Hauptverzeichnis als root:

```bash
sudo bash system-backend/alarm-workflow/install/install.sh
```

`/etc/netcore/alarm-workflow.toml` enthält Broker, SDS Router, Empfänger, Regeln und Eskalationsprofile. Beispieladressen und `technik-gruppe` vor dem ersten Test anpassen. Zustand, Ereignisse und Audit liegen unter `/var/lib/netcore-alarm-workflow/`; diese Dateien bei Updates erhalten. Standardmäßig stoppen ACK oder Übernahme die Eskalation, ein `clear` schließt einen Alarm jedoch nicht automatisch.

Weitere Anleitungen: [Architektur](architektur.md) und [Funktionsprüfung](tests/funktionspruefung-im-labor.md).
