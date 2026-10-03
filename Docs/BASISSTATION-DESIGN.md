# NetCore-Tetra Basisstationsoberfläche

Die Basisstation verwendet eine gemeinsame helle Oberfläche mit weißen Arbeitsflächen, kühlem graublauem Hintergrund, dunkler Schrift und blauem Aktionsakzent. Die Hauptnavigation liegt horizontal über den jeweiligen Bereichsseiten. Das vom Betreiber gelieferte Logo wird unverändert als PNG eingebettet; die Darstellung im Kopf ordnet dessen vorhandenes Zeichen und Wortmarke über CSS an.

Die semantischen Farben, Abstände und Komponenten liegen in `crates/tetra-entities/src/net_dashboard/ui/netcore.css`. Das gemeinsame Grundprinzip gilt auch für die Dienstoberflächen in `system-backend/shared/web-ui/`. Dieser Branch enthält Basisstation und alle vorhandenen Dienst-WebUIs; der Dienst-Rollout steht in [DIENST-WEBUI-DESIGN-UPDATE.md](DIENST-WEBUI-DESIGN-UPDATE.md). Die vorhandenen dunklen und blauen Themes sowie Lesbarkeit und Touchbedienung bleiben verfügbar.

## Ansichten

| Bereich | Ansichten | Daten und Verhalten |
|---|---|---|
| Anmeldung | Login | Bestehende Cookie-Sitzung; optional öffentliche Aggregate rechts neben dem Formular. |
| Öffentlich | Öffentlicher Status | Bestehende freigegebene Übersicht, ausschließlich bei `public_overview = true`. |
| Hauptseite | Funklage | Bestehende Teilnehmer-/Rufzahlen, Trägerbelegung, Systemdaten, Stationsprofil und letzte Aktivität. |
| Funkbetrieb | Funkgeräte, Rufe, Zuletzt gehört, SDS-Protokoll, Paketdaten, RF-Monitor | Bestehende Telemetrie und Aktionen. Funkgeräte erhalten einen lokalen Suchfilter und eine Detailauswahl. |
| Netzkarte | Karte, Nachbarzellen | Bestehende Positionen; echte konfigurierte Nachbarzellen. Nachbarn sind keine gemessene Verfügbarkeits- oder Handover-Anzeige. |
| Diagnose | Systemzustand, Live-Protokoll, Dienste | Bestehende Zustände, Logs und vollständige Matrix der 17 zentralen Dienste mit lokalem Ersatzverhalten. |
| Verwaltung | System, Konfiguration, WLAN, Asterisk SIP, Audio-Zentrale, Aufzeichnungen, Telegram, Hilfe | Bestehende Bedienfunktionen; Aufzeichnungen werden von der Audio-Zentrale getrennt dargestellt. Hilfe enthält Navigation und später zu ergänzende Themen. WLAN erscheint weiterhin nur bei verfügbarem NetworkManager. |
| Interne Integrationen | DAPNET, EchoLink, MeshCom, GeoAlarm | Bestehende, sitzungsabhängig ausgeblendete Ansichten und Aktionen. Ihre bisherige Sichtbarkeitsregel bleibt erhalten. |

Damit gibt es 26 Ansichten einschließlich Anmeldung und öffentlicher Übersicht. Die neu gegliederten Seiten benutzen dieselben Daten und Handler; Demo-Werte kommen ausschließlich in den Browser-Testfixtures vor.

## RF

Spektrum und Wasserfall stehen links, DSP-Qualität und SDR-Istwerte rechts. Die Konstellation ist als aufklappbare Detailansicht verfügbar. Darunter liegt die aktuelle Belegung der tatsächlich bekannten Träger. Beim Sekundärträger ist Funk-TS1 Steuerung/Guard; Funk-TS2–4 entsprechen logischem TS5–7.

Die Diagramme analysieren das erzeugte TX-Basisband vor dem Leistungsverstärker. RMS/Peak und Teilnehmer-Signalwerte verwenden dBFS; sie liefern keine kalibrierte Antennenleistung. Alle bisherigen DSP-Qualitätswerte und SDR-Gain-Readbacks bleiben vorhanden.

## Einbettung und Zugriff

`html.rs` bindet die Templates und Assets mit `include_str!` beziehungsweise `include_bytes!` beim Rust-Build ein. Ein zusätzlicher npm-Build, Webserver oder eine Laufzeit-Dateikopie ist nicht erforderlich. Für ein Update immer den vollständigen Branch verwenden; nur `html.rs` zu kopieren reicht nicht aus.

Statische Assets werden über eine feste Allowlist ausgeliefert. Der neue Bootstrap-Endpunkt `/api/session` liefert nur drei Zugriffsflags und prüft den vorhandenen serverseitigen SessionStore. Erst danach lädt die Oberfläche private Verzeichnisse und öffnet ihren WebSocket. Ein fehlgeschlagener Sitzungscheck lässt die Bedienoberfläche gesperrt.

`/api/public` bleibt ausdrücklich opt-in. Temperatur und Prozesslaufzeit stammen aus vorhandener Telemetrie; nicht vorhandene Werte werden als fehlend angezeigt. CPU-Auslastung ist in der öffentlichen Projektion derzeit nicht vorhanden. Die angemeldete Hauptseite verwendet dafür weiterhin den bestehenden Host-Snapshot. Teilnehmerkennungen, Konfiguration, Nachrichten, Logs und Kartenpositionen werden durch die neue öffentliche Projektion nicht veröffentlicht.

## Prüfen und installieren

```bash
cargo check --locked -p bluestation-bs
cargo test --locked -p tetra-entities --features recording,asterisk --lib net_dashboard::server::tests::
python3 system-backend/media-library/tests/central_tts_reference.py
```

Die Browser-Suite verwendet einen isolierten HTTP-/WebSocket-Server mit Testfixtures:

```bash
npm install --prefix /tmp/netcore-dashboard-tests --no-save playwright@1.62.1
/tmp/netcore-dashboard-tests/node_modules/.bin/playwright install chromium
NODE_PATH=/tmp/netcore-dashboard-tests/node_modules node tools/test_dashboard_ui.mjs
```

Ein bereits installierter Chromium kann über `CHROMIUM_EXECUTABLE_PATH` gewählt werden. Die Suite prüft alle Seiten, Mobilansichten, Navigation, RF-Anordnung, Filter, Aufzeichnungen, Themes, Fallbacks und anonyme Zugriffssperren. Die GitHub-Workflowdatei `dashboard-ui-tests.yml` führt Browser- und Rust-Prüfungen für entsprechende Pull Requests aus.

Die [Update- und Rollback-Anleitung](BASISSTATION-DESIGN-UPDATE.md) beschreibt den Wechsel einer vorhandenen Basisstation auf diesen Branch.
