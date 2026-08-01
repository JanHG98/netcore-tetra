# NetCore Alarm Workflow

Phase 8 verbindet `netcore-event-v1`, MQTT, den zentralen SDS Router und pre-coded Status zu einem persistenten Alarm- und Eskalationsdienst.

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
