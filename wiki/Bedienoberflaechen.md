# Bedienoberflächen und Rollen

NetCore besitzt mehrere Oberflächen mit **unterschiedlichen Aufgaben**. Nicht jedes Problem lässt sich im TBS-Dashboard lösen, und Control Room ist kein Ersatz für die Datenbank eines Fachkerns.

| Oberfläche | Inhalt | Eingriff |
|---|---|---|
| lokales TBS-Dashboard | Funkgeräte, Rufe, RF, SDS, Karte, Health, Audio, Konfiguration | lokale TBS- und Systemaktionen; [[Dashboard]] |
| Control Room | Operatoren, Arbeitsplätze, netzweite Ansichten und Befehle | rollenabhängige Leitstellenoperationen; [[Control-Room]] |
| Directory | Geräte-/Gruppennamen, Statuslabels und Statusgruppen | Metadatenpflege; [[NetCore-Directory]] |
| Provisioning Core | Teilnehmer und Gruppen über Subscriber/Group Core | Profile/Berechtigungen; [[Provisioning]] |
| 24 Fach-WebUIs | Dienstbezogene Fachverwaltung, Zustand, Ereignisse, Konfiguration, Wartung | abhängig vom Dienst teils tiefgreifende Aktionen; [[Dienstkatalog]] |
| Observability | Metriken, Logs, Traces, Alerts | Diagnose, Alarmregeln, Retention |

Die [WebUI-Service-Matrix](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/BACKEND_WEBUI_SERVICE_MATRIX.md) nennt die besonders geschützten Aktionen. Das TBS-Dashboard kann Basic-/Cookie-Anmeldung verwenden; die Open-Lab-Fach-WebUIs im Beispiel sind dagegen vielfach **ohne Login oder TLS**. Ein gemeinsames Netzwerk muss diese Unterschiede ausdrücklich berücksichtigen. [[Security-and-Operations]]

## Maßstab für eine Anzeige

- **„Registriert“**: letzte bekannte Präsenz und aktueller Mobility-/TBS-Status prüfen.
- **„Gruppe“**: Directory-Name, Policy im Group Core und tatsächliche Affiliation getrennt lesen.
- **„Connected“**: WebSocket/Broker/SIP-Verbindung, nicht automatisch Ende-zu-Ende-Datenfluss.
- **„Healthy“**: Liveness, Readiness, betroffene Abhängigkeit und Nutzerfunktion getrennt prüfen.
