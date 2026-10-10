# API-Beispiele im offenen Labor

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/state.rs](../../../../system-backend/recorder/src/state.rs) · [src/http.rs](../../../../system-backend/recorder/src/http.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

Diese Aufrufe sind eine Bedien- und Prüfanleitung für den Recorder, kein Protokoll bereits ausgeführter Tests. Der Dienst muss laufen; `127.0.0.1:8140` funktioniert nur bei einem Listener auf Loopback oder allen Interfaces. Nach LXC-Installation die tatsächliche Adresse aus `/etc/netcore/lxc-network.env` verwenden. Platzhalter für Node-, Ruf- und Aufnahme-IDs durch vorhandene IDs aus den lesenden API-Antworten ersetzen. Schreibende Beispiele verändern den Laborzustand.

Status und Suche:

```bash
curl http://127.0.0.1:8140/api/v1/status
curl http://127.0.0.1:8140/api/v1/active
curl 'http://127.0.0.1:8140/api/v1/recordings?gssi=2000&limit=100'
curl 'http://127.0.0.1:8140/api/v1/recordings?issi=1001&emergency=true'
```

Integrität prüfen:

```bash
curl -X POST http://127.0.0.1:8140/api/v1/recordings/RECORDING-ID/verify
```

Retention und Legal Hold:

```bash
curl -X POST http://127.0.0.1:8140/api/v1/recordings/RECORDING-ID/retention \
  -H 'Content-Type: application/json' \
  -d '{"days":90}'

curl -X POST http://127.0.0.1:8140/api/v1/recordings/RECORDING-ID/hold \
  -H 'Content-Type: application/json' \
  -d '{"legal_hold":true}'
```

Export:

```bash
curl -OJ http://127.0.0.1:8140/api/v1/recordings/RECORDING-ID/export
```

Achtung: Diese API ist im aktuellen Stand vollständig offen.

Export und Hashprüfung benötigen eine bereits finalisierte Aufnahme. Retention setzt die Frist ab Rufende, nicht ab API-Aufruf. Nach Entfernen eines Legal Holds kann eine bereits abgelaufene Aufnahme beim nächsten automatischen Retention-Lauf verschwinden; `allow_delete = false` sperrt diese Automatik nicht.
