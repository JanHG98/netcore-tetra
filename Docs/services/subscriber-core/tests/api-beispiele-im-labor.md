# API-Beispiele im offenen Labor

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/state.rs](../../../../system-backend/subscriber-core/src/state.rs) · [src/http.rs](../../../../system-backend/subscriber-core/src/http.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

Diese Aufrufe sind eine Bedien- und Prüfanleitung für den Subscriber Core, kein Protokoll bereits ausgeführter Tests. Der Dienst muss laufen; `127.0.0.1:8100` funktioniert nur bei einem Listener auf Loopback oder allen Interfaces. Nach LXC-Installation die tatsächliche Adresse aus `/etc/netcore/lxc-network.env` verwenden. Platzhalter für Node-, Ruf- und Aufnahme-IDs durch vorhandene IDs aus den lesenden API-Antworten ersetzen. Schreibende Beispiele verändern den Laborzustand.

```bash
curl http://127.0.0.1:8100/api/v1/status
curl -X POST http://127.0.0.1:8100/api/v1/subscribers \
  -H 'Content-Type: application/json' \
  -d '{"issi":1234,"display_name":"Test HRT","enabled":true,"registration_allowed":true,"default_groups":[1001]}'
curl -X POST http://127.0.0.1:8100/api/v1/sync
```

Der Beispielteilnehmer ist in der zentral erzeugten Policy zur Registrierung freigegeben. `default_groups` speichert nur Profilinformationen und erzeugt keine Group-Core-Mitgliedschaft. Im aktiven Stack ist wegen `subscriber_policy = false` die Phase `unsupported` in `/api/v1/syncs` zu erwarten; ein HTTP-Erfolg des Sync-Aufrufs belegt keine MM-Anwendung. Auch ein erfolgreicher Mock-TBS-Sync ist kein Nachweis der aktiven Funkzulassung. Die tatsächliche Registrierung wird weiterhin durch die lokale TBS-Whitelist geprüft.
