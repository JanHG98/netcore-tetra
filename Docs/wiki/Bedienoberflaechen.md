# Bedienoberflächen und Rollen

NetCore besitzt mehrere Oberflächen mit **unterschiedlichen Aufgaben**. Nicht jedes Problem lässt sich im TBS-Dashboard lösen, und Control Room ist kein Ersatz für die Datenbank eines Fachkerns.

| Oberfläche | Inhalt | Eingriff |
|---|---|---|
| lokales TBS-Dashboard | Funkgeräte, Rufe, RF, SDS, Karte, Health, Audio, Konfiguration | lokale TBS- und Systemaktionen; [Dashboard](Dashboard.md) |
| Control Room | Operatoren, Arbeitsplätze, netzweite Ansichten und Befehle | rollenabhängige Leitstellenoperationen; [Control-Room](Control-Room.md) |
| Directory | Geräte-/Gruppennamen, Statuslabels und Statusgruppen | Metadatenpflege; [NetCore-Directory](NetCore-Directory.md) |
| Provisioning Core | Teilnehmer und Gruppen über Subscriber/Group Core | Profile/Berechtigungen; [Provisioning](Provisioning.md) |
| 24 Fach-WebUIs | Dienstbezogene Fachverwaltung, Zustand, Ereignisse, Konfiguration, Wartung | abhängig vom Dienst teils tiefgreifende Aktionen; [Dienstkatalog](Dienstkatalog.md) |
| Observability | Metriken, Logs, Traces, Alerts | Diagnose, Alarmregeln, Retention |

Die [WebUI-Service-Matrix](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/design/BACKEND_WEBUI_SERVICE_MATRIX.md) nennt die besonders geschützten Aktionen. Das TBS-Dashboard kann Basic-/Cookie-Anmeldung verwenden; die Open-Lab-Fach-WebUIs im Beispiel sind dagegen vielfach **ohne Login oder TLS**. Ein gemeinsames Netzwerk muss diese Unterschiede ausdrücklich berücksichtigen. [Security-and-Operations](Security-and-Operations.md)

## Maßstab für eine Anzeige

- **„Registriert“**: letzte bekannte Präsenz und aktueller Mobility-/TBS-Status prüfen.
- **„Gruppe“**: Directory-Name, Policy im Group Core und tatsächliche Affiliation getrennt lesen.
- **„Connected“**: WebSocket/Broker/SIP-Verbindung, nicht automatisch Ende-zu-Ende-Datenfluss.
- **„Healthy“**: Liveness, Readiness, betroffene Abhängigkeit und Nutzerfunktion getrennt prüfen.
