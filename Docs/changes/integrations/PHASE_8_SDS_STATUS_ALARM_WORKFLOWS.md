# Phase 8 – SDS-, Status- und Alarm-Workflows

Neuer Dienst: `system-backend/alarm-workflow/` auf Port `8270`.

Der Dienst verarbeitet `netcore-event-v1` über MQTT, eröffnet deduplizierte Alarmakten, eskaliert zeitgesteuert über den bestehenden SDS Router und verarbeitet Rückmeldungen per SDS-Text oder pre-coded Status.

## Alarmzustände

`open → acknowledged/assigned → in_progress → resolved → closed`

Zusätzlich sind `cancelled` und `reopen` vorhanden. Alle Übergänge werden persistent auditiert.

## OPEN LAB

Keine Anmeldung, Tokens oder TLS. Jeder erreichbare Client kann Alarmzustände verändern.
