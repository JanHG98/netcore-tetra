# Architektur

```text
NetCore Events / RF / Hardware / Node Gateway
                  │ MQTT netcore/v1/events/#
                  ▼
          Alarm Workflow :8270
          ├─ Regeln und Deduplizierung
          ├─ Alarm-State-Machine
          ├─ Eskalationsscheduler
          ├─ persistente Alarmakte
          └─ SDS-Ausgang
                  │ HTTP
                  ▼
             SDS Router :8150
                  │
                  ▼
             passende TBS
```

Rückmeldungen laufen als eingehende SDS oder pre-coded Status wieder über den SDS Router. Der Alarm Workflow liest dessen kanonischen Ereignisstrom und ordnet Textkommandos über den achtstelligen Alarmtoken zu.
