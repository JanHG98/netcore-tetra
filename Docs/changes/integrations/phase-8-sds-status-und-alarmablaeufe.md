# Phase 8 – SDS-, Status- und Alarm-Workflows

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: Phase 8 – SDS-, Status- und Alarm-Workflows. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Die gepflegte Implementierung und die aktuelle Betriebsanleitung stehen unter [alarm-workflow](../../services/alarm-workflow/README.md) und [Quellcode](../../../system-backend/alarm-workflow). Ein offener Lab-Verwaltungsweg und ein historischer Szenariotest sind keine Abnahme der Alarmierung auf der Anlage.

Phasennummern, damalige Ports, Testidentitäten und Befehle dokumentieren die Einführung. Neue Installationen folgen der heutigen Komponentenbeschreibung und dem tatsächlich gewählten Inventory.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### Phase 8 – SDS-, Status- und Alarm-Workflows

Neuer Dienst: `system-backend/alarm-workflow/` auf Port `8270`.

Der Dienst verarbeitet `netcore-event-v1` über MQTT, eröffnet deduplizierte Alarmakten, eskaliert zeitgesteuert über den bestehenden SDS Router und verarbeitet Rückmeldungen per SDS-Text oder pre-coded Status.

## Alarmzustände

`open → acknowledged/assigned → in_progress → resolved → closed`

Zusätzlich sind `cancelled` und `reopen` vorhanden. Alle Übergänge werden persistent auditiert.

## OPEN LAB

Keine Anmeldung, Tokens oder TLS. Jeder erreichbare Client kann Alarmzustände verändern.
