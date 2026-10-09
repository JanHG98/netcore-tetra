# Open-Lab E2E Runbook

## 1. Ziel

Dieses Runbook führt den ersten vollständigen Integrationstest der 17 NetCore-Tetra-Backend-LXCs durch. Es prüft nicht nur, ob Prozesse laufen, sondern ob fachliche Zustände zwischen Node Gateway, Teilnehmer-, Gruppen-, Call-, Media-, SDS- und Packet-Data-Diensten tatsächlich weitergereicht und nach Neustarts wiederhergestellt werden.

Der Test bleibt Teil der ausdrücklich offenen Laborstufe:

```text
keine Benutzeranmeldung
keine Management-Tokens
kein TLS
isoliertes Management-VLAN zwingend
```

## 2. Voraussetzungen

- alle 17 Dienste anhand `deploy/open-lab/inventory.toml` installiert;
- Management-Endpunkte vom Testhost erreichbar;
- Python 3.11 oder neuer auf dem Testhost;
- für das Fault-Profil: schlüsselbasierter Root-SSH-Zugriff auf die LXCs;
- korrekte systemd-Units gemäß Inventory;
- ausreichend freier Speicher für Recorder- und Diagnoseartefakte;
- für echte Packet-Data-Tests: TUN-/nftables-Voraussetzungen des IP Gateways;
- das Netz ist frei von produktiven Teilnehmern, Gruppen und Schlüsseln.

## 3. Vorprüfung

```bash
cp deploy/open-lab/inventory.example.toml deploy/open-lab/inventory.toml
$EDITOR deploy/open-lab/inventory.toml

python3 deploy/open-lab/netcore-deploy.py \
  --inventory deploy/open-lab/inventory.toml \
  validate

python3 deploy/open-lab/netcore-deploy.py \
  --inventory deploy/open-lab/inventory.toml \
  status

python3 deploy/open-lab/netcore-deploy.py \
  --inventory deploy/open-lab/inventory.toml \
  test --profile full --validate-only
```

Der letzte Befehl prüft Inventory und Szenarionamen ohne Netzwerkzugriff. Er verändert nichts.

## 4. Teststufen

### 4.1 Smoke

```bash
python3 deploy/open-lab/netcore-deploy.py \
  --inventory deploy/open-lab/inventory.toml \
  test --profile smoke
```

Erwartung:

- alle Dienste antworten auf Liveness, Status, OpenAPI, Metrics und WebUI;
- Readiness darf in der Standardausführung `200` oder nachvollziehbar `503` liefern;
- der Mock-TBS-Knoten wird im Node Gateway sichtbar und beantwortet einen Ping;
- der Control Room hat alle 15 konfigurierten Fachdienste mindestens einmal gepollt.

Mit `--strict-ready` wird jede `503` zu einem Fehler.

### 4.2 Vollständiger funktionaler Lauf

```bash
python3 deploy/open-lab/netcore-deploy.py \
  --inventory deploy/open-lab/inventory.toml \
  test --profile full \
  --allow-mutations \
  --timeout 35
```

Der Runner erzeugt pro Lauf eigene ISSI/GSSI aus der Run-ID. Standardmäßig werden die Testdaten am Ende entfernt. `--keep-fixtures` ist nur zur Fehlersuche gedacht.

Der Lauf prüft:

1. gemeinsame Management-Verträge aller 17 Dienste;
2. Node-Gateway-WebSocket und TBS-Capabilities;
3. Subscriber-/Group-Profile, Registrierung und Affiliation;
4. Gruppenruf, Sprecherwechsel, TACELP-Frames, Media Switch und Recorder;
5. SDS-Einzelzustellung und Offline-Store-and-forward;
6. PDP-Kontext, IPv4-N-PDU und IP-Gateway-Synchronisation;
7. Control-Room-Federation und metadata-only Managementansichten der Plattformdienste;
8. zentralen Metrics-Scrape und strukturierten Log-Ingest.

### 4.3 Restart- und Ausfallmatrix

```bash
python3 deploy/open-lab/netcore-deploy.py \
  --inventory deploy/open-lab/inventory.toml \
  test --profile fault \
  --allow-mutations \
  --allow-restarts \
  --timeout 45
```

Dieser Lauf darf Dienste absichtlich stoppen und neu starten. Er gehört nicht in ein produktiv belegtes Netz.

Geprüft werden:

- Subscriber- und Group-State nach systemd-Neustart;
- `call-control` ausgefallen → `media-switch` degradiert → beide erholen sich;
- `packet-core` ausgefallen → `ip-gateway` degradiert → beide erholen sich;
- `media-switch` ausgefallen → `recorder` degradiert → beide erholen sich.

## 5. Artefakte und Abnahme

Die Ergebnisse liegen unter:

```text
tests/e2e/artifacts/<run-id>/
├── report.json
├── junit.xml
└── summary.txt
```

Abnahme für einen Lauf:

- `failed=0`;
- keine unerklärten Skips;
- Mock-TBS-Fehlerliste leer;
- keine liegen gebliebenen Test-ISSI/GSSI;
- Recorder-Integritätsprüfung erfolgreich;
- alle absichtlich gestoppten Dienste wieder `active` und ready;
- bei Fault-Läufen: die Abhängigkeit wurde tatsächlich degradiert, nicht nur der Test übersprungen.

## 6. On-Air-Abnahme

Der Mock TBS belegt Core-Verträge und Zustandsflüsse, aber keine HF-, MAC-, LLC-, MLE-, MM- oder CMCE-Konformität. On-Air-Evidenz wird deshalb getrennt geführt:

```bash
cp tests/e2e/on_air_template.json tests/e2e/on_air_evidence.json
python3 tests/e2e/validate_on_air_evidence.py \
  tests/e2e/on_air_evidence.json \
  --require-complete \
  --require-two-vendors
```

Mindestens zu dokumentieren:

- TBS-Softwarestand und Konfiguration;
- MCC, MNC, LAC, DCC/Colour Code, Träger und Duplexparameter;
- Endgerätehersteller, Modell und Firmware;
- Registrierungs-, Gruppenruf-, SDS- und Packet-Data-Ergebnis;
- Log-, PCAP-, Audio- oder Screenshot-Referenz;
- Tester und UTC-Zeitpunkt.

## 7. Fehlerbehandlung

- Erst `report.json` lesen; jeder Check enthält Szenario, Dienst, Dauer und Evidenz.
- Danach die eigenständige WebUI des betroffenen Fachdienstes öffnen.
- Correlation-/Trace-ID aus dem Report in Observability suchen.
- Node-Gateway-Knotenstatus und letzte Control-Acks prüfen.
- Bei persistenzbezogenen Fehlern State-Dateien und dienstspezifische Backups prüfen, nicht blind löschen.
- Nach einem abgebrochenen Fault-Lauf die drei möglichen Opferdienste explizit starten:

```bash
systemctl start netcore-call-control.service
systemctl start netcore-packet-core.service
systemctl start netcore-media-switch.service
```

Die tatsächlichen Hosts und Units bleiben dem Inventory zu entnehmen.
