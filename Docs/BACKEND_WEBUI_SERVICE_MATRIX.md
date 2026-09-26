# Backend-WebUI-Service-Matrix

| Dienst | Schwerpunkt der eigenen WebUI | Besonders geschützte Aktionen |
| --- | --- | --- |
| Node Gateway | TBS-Sessions, Heartbeats, Protokollversionen, Backend-Transport | Node trennen und Kommandos senden; im ersten Testpaket bewusst offen ohne Tokens |
| Subscriber Core | Teilnehmer, Geräte, Profile, Berechtigungen, TBS-Synchronisation | Sperren, Import, Gerätezuordnung; im Testpaket bewusst offen ohne Tokens |
| Group Core | GSSI, Mitglieder, Affiliationen, DGNA | DGNA, Gruppenrechte, Löschung; im Testpaket bewusst offen ohne Tokens |
| Mobility Core | Registrierungen, Zellen, Migration, Recovery | Kontextfreigabe, Handover-Abbruch |
| Call Control | Calls, Legs, Floor, Priorität, Restore | Call beenden, Floor entziehen, Pre-emption; im Testpaket bewusst offen ohne Tokens |
| Media Switch | Streams, Jitter, Routing, TBS-Legs, Recorder-Taps | Stream stummschalten, Puffer leeren, Testframe einspeisen; im Testpaket bewusst offen ohne Tokens |
| SDS Router | Nachrichten, Queues, Zustelltrace | Nachricht senden, Retry, Queue löschen; im Testpaket bewusst offen ohne Tokens |
| Packet Core | PDP Contexts, NSAPI, READY/STANDBY, Bearer, Fragmentierung und Flow Control | Kontext pagen, modifizieren, beenden oder trennen; im Testpaket bewusst offen ohne Tokens |
| IP Gateway | TUN, PDP-IP-Leases, Routing, NAT, Firewall, DNS, WAP/Testdienste, Flows und PCAP | Kernel-Reconcile, Route/NAT/Firewall ändern, Flow blockieren und Capture starten; im Testpaket bewusst offen ohne Tokens |
| Security Core | Authentisierung, Security Classes, DCK-Metadaten, Sperren, Alarm/Audit | Policy, Disable/Enable, Kontext-/DCK-Widerruf; keine Rohschlüsselanzeige |
| KMF | CCK/GCK/SCK, Key-Versionen, Crypto Periods, Rotation, OTAR, Vault und Backup | Rotation, Revoke/Destroy, OTAR-Freigabe und Backup; keine Rohschlüsselanzeige, im Testpaket bewusst offen ohne Tokens |
| Transit | Regionen, Peers, Teilnehmer-/Gruppenregionen, Routen, Sessions, Queues und Failover | Peer sperren, Route ändern, Envelope einspeisen und Failover auslösen; im Testpaket bewusst offen ohne Tokens |
| Control Room | Operatoren, Arbeitsplätze, Backend-Verknüpfung | Rollen, Tokens, Leitstellenkonfiguration |
| Application Gateway | Connectoren, Webhooks, Routing, Vorlagen, Delivery-/Dead-Letter-Queues und TTS | Connector aktivieren, Secrets ersetzen, Fremdzustellung auslösen und TTS veröffentlichen; im Testpaket bewusst offen ohne Management-Tokens |
| Media Library | Audio-Assets, TTS-/Recorder-Import, Vorschau, Freigabe, TETRA-Cache, Archiv und Playout-Jobs | Upload/Import, Metadaten, Freigabe/Sperre, Vorschau, Archivkopie sowie kontrollierte Einspeisung in bestehende Media-Switch-Sessions |
| IoT Gateway | MQTT-Events, Commands/Acks, Home Assistant Discovery, importierte HA-Entitäten und Homematic-Datenpunkte | Reale Home-Assistant-/Homematic-Aktionen; in Phase 5 standardmäßig gesperrt, OPEN LAB ohne Management-Tokens |
| Recorder | Aufnahmen, Suche, Retention, Integrität | Export, Retention, Hold und Löschung; im Testpaket bewusst offen ohne Tokens |
| Observability | Metriken, Logs, Traces, Alarme und Diagnose | Alarmregeln, Retention, Stummschaltung und Diagnoseexport; im Testpaket bewusst offen ohne Tokens |
| Hardware Gateway | Edge-I/O, Rack-Sensoren, Eingänge, Messwerte und Heartbeats | Physische Ausgänge; in OPEN LAB standardmäßig deaktiviert |
| RF Monitor | TBS-RF-Telemetrie, VSWR, Temperaturen, Senderzustand und RF-Alarme | Schwellwerte, Probe-Konfiguration und Alarmquittierung; OPEN LAB ohne Management-Tokens |
| Alarm Workflow | Alarme, Eskalation, SDS-/Status-Rückmeldungen und Audit-Timeline | Eskalieren, schließen, abbrechen und Empfänger ändern; OPEN LAB ohne Management-Tokens |
| Task Workflow | Strukturierte Aufträge, WAP-Formulare, SDS-/Status-Aktionen und Timeline | Zuweisen, abschließen, abbrechen und Benachrichtigungen auslösen; OPEN LAB ohne Management-Tokens |
| Asset Management | Funkgeräte, Assets, Personen, Ausgaben, Rückgaben und Wartungsakten | Bestandsänderung, Zuordnung und Wartungsabschluss; OPEN LAB ohne Management-Tokens |
| SIP Switch | PBX-Trunk, TBS-Registrierungen, Mobility-Routing, SIP-Rufe und Asterisk-Zustand | Asterisk-Konfiguration rendern/neuladen und SIP-Routen ändern; OPEN LAB ohne Anmeldung oder TLS |
| Shared | kein Container; gemeinsames UI-Kit, API-Verträge und Service-Grundbausteine | nicht zutreffend |

## Gemeinsame Seiten

Unabhängig vom fachlichen Schwerpunkt besitzt jeder deploybare Dienst die Seiten:

```text
Übersicht
Fachverwaltung
Zustand & Abhängigkeiten
Ereignisse & Audit
Konfiguration
Wartung
API
Über
```
