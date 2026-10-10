# Bedienoberflächen und Rollen

NetCore besitzt mehrere Oberflächen mit **unterschiedlichen Aufgaben**. Nicht jedes Problem lässt sich im TBS-Dashboard lösen, und Control Room ist kein Ersatz für die Datenbank eines Fachkerns.

| Oberfläche | Inhalt | Eingriff |
|---|---|---|
| lokales TBS-Dashboard | Funkgeräte, Rufe, RF, SDS, Karte, Health, Audio, Konfiguration | lokale TBS- und Systemaktionen; [Lokales Dashboard der Basisstation](dashboard-der-basisstation.md) |
| Control Room | Operatoren, Arbeitsplätze, netzweite Ansichten und Befehle | Arbeitsplatz- und Operatorfunktionen; Open-Lab-Anmeldung ist deaktiviert; [Control Room und Node Gateway](leitstelle-und-node-gateway.md) |
| Directory | Geräte-/Gruppennamen, Statuslabels und Statusgruppen | Metadatenpflege; [Namen und Metadaten im NetCore Directory](namen-und-metadaten-im-directory.md) |
| Provisioning Core | Teilnehmer und Gruppen über Subscriber/Group Core | Profile/Berechtigungen; [Provisioning und Datenhoheit](teilnehmer-und-gruppen-anlegen.md) |
| 26 Fach-WebUIs | Dienstbezogene Fachverwaltung, Zustand, Ereignisse, Konfiguration, Wartung | abhängig vom Dienst teils tiefgreifende Aktionen; [Dienstkatalog](dienstkatalog.md) |
| Observability | Metriken, Logs, Traces, Alerts | Diagnose, Alarmregeln, Retention |

Die [WebUI-Service-Matrix](../design/dienstoberflaechen-funktionsmatrix.md) nennt die besonders geschützten Aktionen. Das TBS-Dashboard verwendet bei gesetzten Zugangsdaten eine Cookie-Sitzung mit Loginformular; die Open-Lab-Fach-WebUIs im Beispiel sind dagegen vielfach **ohne Login oder TLS**. Ein gemeinsames Netzwerk muss diese Unterschiede ausdrücklich berücksichtigen. [Sicherheit und Betrieb](sicherheit-im-betrieb.md)

## Maßstab für eine Anzeige

- **„Registriert“**: letzte bekannte Präsenz und aktueller Mobility-/TBS-Status prüfen.
- **„Gruppe“**: Directory-Name, Policy im Group Core und tatsächliche Affiliation getrennt lesen.
- **„Connected“**: WebSocket/Broker/SIP-Verbindung, nicht automatisch Ende-zu-Ende-Datenfluss.
- **„Healthy“**: Liveness, Readiness, betroffene Abhängigkeit und Nutzerfunktion getrennt prüfen.

Deployment Core ergänzt eine eigene Oberfläche für Discovery, Dienste, Aufträge und Pi-Images. Im Beispiel ist auch sie ohne Anmeldung oder TLS erreichbar. Alert Service verlangt dagegen einen Management-Token. Vor einer Aktion deshalb die konkrete Oberfläche und ihren Sicherheitsmodus prüfen, statt einen gemeinsamen Login vorauszusetzen.

## Quellen zur Pflege dieser Seite

[TBS-Login und Sitzung](../../crates/tetra-entities/src/net_dashboard/server.rs) · [Control-Room-Authvorlage](../../system-backend/control-room/config/control-room.example.toml) · [Dienstregistry](../../system-backend/services.toml).
