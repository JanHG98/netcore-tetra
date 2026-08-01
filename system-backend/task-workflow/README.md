# NetCore Task Workflow

Phase 9 ergänzt strukturierte Aufträge und kompakte WAP-Formulare. Der Dienst läuft als eigener LXC auf Port `8280` und bleibt im OPEN-LAB-Modus ohne Login, Token und TLS.

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
