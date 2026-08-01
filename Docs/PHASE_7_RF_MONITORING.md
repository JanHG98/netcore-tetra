# Phase 7 – RF-Monitoring

Phase 7 ergänzt `system-backend/rf-monitor/` auf Port 8260 und einen kleinen Agenten für jede Basisstation.

## Messgrenze

Die bereits in der Basisstation vorhandenen DSP-Werte (Spektrum, EVM, PAPR, RMS/Peak, IQ-Fehler) beschreiben die Signalqualität **vor dem PA**. Der neue Dashboard-Endpunkt `/api/rf-monitor` stellt genau diese Werte sowie SDR-/Systemzustände maschinenlesbar bereit.

Reale Vorlauf- und Rücklaufleistung, VSWR, PA-Strom und Antennenfehler können Software allein nicht verlässlich liefern. Der TBS-Agent kann deshalb zusätzlich ein lokales Probe-Kommando ausführen, das kalibrierte Messwerte als JSON liefert. Ein Mock-Probe-Skript ist enthalten.

## Zustände und Ereignisse

- `rf.station_registered`
- `rf.station_online`
- `rf.station_offline`
- `rf.tx_state_changed`
- `rf.alarm_raised`
- `rf.alarm_cleared`
- `rf.telemetry_invalid`

Alarme werden transitionsbasiert erzeugt, damit ein dauerhaft hoher VSWR-Wert nicht alle fünf Sekunden ein neues Ereignis produziert.
