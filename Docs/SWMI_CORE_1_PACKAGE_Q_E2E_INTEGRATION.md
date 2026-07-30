# SWMI Core 1 – Paket Q: Cross-LXC E2E-Integration

## Ergebnis

Paket Q ergänzt die bisher einzeln deploybaren Backend-Dienste um einen reproduzierbaren, inventory-gesteuerten End-to-End-Test. Der Testhost kann damit alle 17 Runtime-LXCs als zusammenhängendes System prüfen, ohne zusätzliche Python-Pakete oder einen separaten Testcontainer zu benötigen.

`shared/` bleibt Library und wird nicht als achtzehnter Dienst gezählt.

## Bestandteile

```text
tests/e2e/
├── netcore_open_lab_e2e.py
├── netcore_e2e/
│   ├── context.py
│   ├── http.py
│   ├── inventory.py
│   ├── mock_tbs.py
│   ├── model.py
│   ├── report.py
│   ├── scenarios.py
│   ├── wait.py
│   └── websocket.py
├── fixtures/open_lab_test_plan.toml
├── on_air_evidence.schema.json
├── on_air_template.json
├── validate_on_air_evidence.py
└── unit/test_e2e_support.py
```

Der bestehende Deployer besitzt zusätzlich den Befehl:

```bash
python3 deploy/open-lab/netcore-deploy.py test ...
```

## Technische Eigenschaften

- ausschließlich Python-Standardbibliothek;
- Inventory `netcore.v1` als einzige Adress- und Unit-Quelle;
- eigener RFC6455-WebSocket-Client;
- Mock TBS für `netcore-control-room-node-v1`;
- eindeutige ISSI/GSSI je Run-ID;
- explizite Schutzschalter für Mutationen und Neustarts;
- automatische Fixture-Bereinigung;
- JSON-, JUnit- und Textberichte;
- strukturierte Evidenz pro Einzelprüfung;
- getrenntes Schema für reale On-Air-Nachweise.

## Fachliche Szenarien

### Management-Verträge

Alle 17 Dienste werden auf Liveness, Readiness, `/api/v1/status`, OpenAPI 3.x, Prometheus-Metriken und die unabhängig ausgelieferte WebUI geprüft.

### TBS Edge und zentrale Stammdaten

Ein Mock TBS verbindet sich mit dem Node Gateway, meldet Capabilities und Heartbeats und erzeugt Registrierungs- sowie Gruppen-Affiliationstelemetrie. Subscriber Core und Group Core müssen daraus ihre beobachteten Zustände ableiten und Policies zum Node zurückspielen können.

### Call, Media und Recording

Der Test erzeugt einen Gruppenruf, Sprecherwechsel und deterministische 35-Byte-TETRA-ACELP-Frames. Media Switch und Recorder müssen Session, Frames, Abschluss und Integritätsprüfung konsistent abbilden.

### SDS

Geprüft werden eine erfolgreiche Einzelzustellung und ein zunächst nicht erreichbarer Teilnehmer mit Store-and-forward-Zustand.

### Packet Data

Der Test führt Edge-Hello, Location Update und PDP-Aktivierung aus, wartet auf den verankerten Kontext, stellt ein minimales IPv4/UDP-N-PDU in den Downlink und prüft die Context-Sicht des IP Gateways.

### Control Room und Plattformdienste

Der Control Room löst einen sofortigen Federation-Poll aus, muss alle 15 dort konfigurierten Fachdienste mit Zeitstempel sehen und stellt das aggregierte Lagebild ausdrücklich nicht-autoritativ bereit. Security Core, KMF, Transit, Application Gateway und Media Library werden zusätzlich auf ihre managementseitigen Metadatenverträge geprüft. Dabei dürfen weder Rohschlüssel, Secrets noch hochgeladene Binärdaten in Listen- oder Konfigurationsantworten erscheinen.

### Observability

Ein zentraler Scrape wird ausgelöst und ein strukturierter Logmarker mit Trace-ID eingespeist und wieder gesucht.

### Neustart und Abhängigkeitsausfall

Subscriber- und Group-Daten werden über einen echten systemd-Neustart hinweg geprüft. Zusätzlich stoppt die Fault-Matrix ausgewählte Upstream-Dienste und verlangt eine nachvollziehbare Readiness-Degradation sowie vollständige Erholung der abhängigen Dienste. Das Szenario `edge-service-outages` stoppt darüber hinaus jeden der 16 vom Node Gateway überwachten Remote-Dienste einzeln und prüft die Zustandsänderung bis zur verbundenen Mock-TBS sowie die anschließende Erholung.

## Sicherheitsgrenze

Paket Q ändert den Sicherheitsmodus nicht. Es akzeptiert ausschließlich Inventories mit:

```toml
contract_version = "netcore.v1"
mode = "open_lab"
```

Mutierende Tests benötigen `--allow-mutations`; systemd-Störungen zusätzlich `--allow-restarts`. Der Runner darf deshalb nur im isolierten Testnetz verwendet werden.

## Keine vorgetäuschte Konformität

Der Mock TBS ist ein Core-Integrationspeer. Er simuliert weder die DQPSK-Luftschnittstelle noch vollständige MAC-/LLC-/MLE-/MM-/CMCE-Prozeduren. Erfolgreiche E2E-Tests sind kein ETSI-PICS- oder On-Air-Konformitätsnachweis. Dafür liegen ein separates Evidenzschema und ein Validator bei.

## Abnahmekriterium

Paket Q ist statisch abgeschlossen, wenn:

```text
Inventory mit 17 Diensten validiert
alle 13 Szenarien maschinenlesbar registriert
read-only, mutating und fault gates wirksam
Mock TBS handshake- und control-response-fähig
JSON- und JUnit-Reports erzeugt
On-Air-Evidenzschema validierbar
CI-Selbsttest ohne Live-LXC erfolgreich
keine PDFs oder Laufzeitartefakte im Paket
```

Die reale Systemabnahme erfolgt erst durch einen vollständigen Lauf gegen die 17 installierten LXCs und anschließend dokumentierte On-Air-Tests mit echten Endgeräten.
