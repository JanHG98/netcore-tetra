# Z01.2 – ergänzende Übernahme gemeinsamer Pfade

Quellen: main `dae9363062a1442664a083e1fc32e6944b3be9a6` und historischer Stand `bbf039729b9b05f8d623b11195ca24a124f68d16`.

## Prüfung und Entscheidung

Gemeinsame Rust-Core-HTTP-Dateien unterscheiden sich weitgehend durch moderne WebUI-Renderer, `include_str!`-Templates, HTML-Lifetimes und Logo-CSP. Gemeinsame Python-Dienste unterscheiden sich weitgehend durch HTML-Generierung. Diese main-Implementierungen bleiben erhalten. Die historischen Änderungen an `tools/check_*` und `central_tts_reference.py` verweisen auf überholte Inline-HTML-Quellen und werden nicht übernommen. Rust-Brews eigenständiger `[workspace]` und Docker-`COPY web-ui` bleiben erhalten.

Die historische Fassung von `net_dashboard/server.rs` entfernt Asset-Allowlist, `/api/session`, sichere öffentliche Projektionen/No-Cache-Antworten, Nachbarzellendetails und entsprechende Tests. Keine dieser Rückwärtsänderungen wurde übernommen.

Nötige funktionale Übernahmen außerhalb Deployment/Observability: LXC-Agent-Enrollment, TBS-Updater-Unit-Erkennung/Installationsguard, IoT-Installer-`bash`/Echo-Reparatur, SIP-f-string für Python 3.11 und Hardware-Shutdown. Root übernimmt Enrollment, TBS-/IoT-Installer sowie Hardware. Diese Teilaufgabe übernimmt SIP und modernisierte Discovery-Verweise.

## Änderungen dieser Teilaufgabe

- `shared/web-ui/assets/service-design.js`: normaler Management-Link in der vorhandenen Werkzeugleiste. URL aus aktuellem Host mit Port 8321, HTTP, Pfad `/` und Query `scan=1`; bestehende Seitenparameter, Fragment und Credentials werden verworfen. IPv6 bleibt korrekt geklammert. Keine Anfrage, kein Scan beim Rendern. Neuer Tab mit `noopener`.
- Shell kann Discovery über `config.discovery === false` oder `body[data-nc-discovery="disabled"]` ausblenden. Letzteres ist für die TBS-Connect-/Python-Brew-Anmeldeseite gesetzt.
- Shared-CSS: Werkzeugleiste darf mobil umbrechen. Vorhandene Theme-Farben und Komponenten werden verwendet.
- Basisstation `ui/netcore.js`: Link im vorhandenen Bereich Dienste, erst nach `netcoreSessionReady` sichtbar; bei öffentlicher Ansicht wieder verborgen. `netcore.css` erzwingt das Verbergen des Links trotz `.btn { display: inline-flex }`. Kein Login-/Server-Routing geändert.
- Control Room nutzt bereits `service_design::render`; erhält denselben Link aus Shared-Assets, kein eigener historischer Renderer nötig.
- SIP Switch: innere einfache Quotes in `registration_expiration_secs`-f-string, kompatibel zum Python-3.11-Ziel.
- Vorhandene Generatoren für Auxiliary, Edge und Workflow wurden auf aktuellen Dateien ausgeführt. Sie ersetzen nur markierte HTML-Blöcke bzw. Shared-Asset-Kopien und erhalten funktionale Änderungen anderer Pakete.

## Prüfung

PASS:

- `python3 tools/embed_auxiliary_service_design.py --check`
- `python3 tools/embed_edge_service_design.py --check`
- `python3 tools/embed_workflow_service_design.py --check`
- `node --check` für Shared-Shell, Dashboard-Shell sowie beide geänderten Browser-Testdateien.
- Python-Parserprüfung für SIP Switch und Hardware-Gateway mit `ast.parse(..., feature_version=(3, 11))` unter Python 3.12. Dies ist kein Laufzeitnachweis mit einem installierten Python 3.11.

Erweiterte Browserprüfungen:

- Shared-Shell: korrektes Linkziel bei HTTP/HTTPS/IPv6, keine Discovery-Anfrage bei Laden/Theme/Mobile, wirkliche Navigation erst auf Klick, Popup ohne `opener`, Login-Opt-out sowie Kontrast der Linkfläche in beiden Themes.
- Dashboard: sichtbarer Link im authentifizierten Servicebereich, genaue Zieladresse, keine Discovery-Anfragen bei Navigation/Theme/Mobile, kein sichtbarer Link in anonymer Ansicht, kein Link im Login-Template.

Die finale Browserabnahme mit funktionsfähiger Headless-Runtime steht unten; alle erweiterten Prüfungen bestanden.

## Reproduzierter Hardware-Befund

Historischer echter HTTP-/SIGTERM-Test in separaten temporären Source-Kopien, jeweils Originalconfig plus stiller Fake-MQTT-Kindprozess:

- main `dae9363`: FAIL, `process.wait(timeout=5)` läuft nach SIGTERM in `TimeoutExpired`.
- historisch `bbf0397`: PASS, ca. 0,61 s, MQTT-Kindprozess ebenfalls beendet.

Der Fix ist funktional erforderlich, damit systemd Stop/Restart nicht deadlockt. Der Test läuft ohne Funkhardware.

## Grenzen / Rollback

Management-Verweise setzen einen Agenten auf demselben Host und Port 8321 voraus; sie ermitteln dessen Erreichbarkeit ausdrücklich erst nach Öffnen durch den Betreiber. Die aktuelle Infrastruktur nutzt HTTP für den Open-Lab-Agenten. Keine realen Geräte/LXCs/Pi oder On-Air-Abnahme wurden aus dieser Teilaufgabe kontaktiert.

Rollback: Shared-Shell/CSS und Dashboard-Linkänderung revertieren, Auxiliary-/Edge-/Workflow-Bundles neu generieren bzw. denselben Paketcommit revertieren. SIP-Quote-Fix ist unabhängig. Kein Dienstzustand oder Benutzerkonto wurde verändert.


## Finale Browserabnahme im isolierten Testnetz

Alle Suites bestanden mit offiziellem Chrome-for-Testing Headless Shell 155.0.8059.39 (Playwright, Standard-Sandbox, echte DOM-/Klickprüfungen). Die zuerst fehlgeschlagene Browserbereitstellung wurde behoben; sie ist kein offener Produktfehler. Aktuelle Quelltemplates und generierte Bundles wurden getestet.

| Suite | Ergebnis |
| --- | --- |
| Shared-Shell | 93 Checks PASS |
| Basisstationsdashboard | 137 Checks PASS |
| Core | 405 Checks, neun Dienste, 16 Schreibaktionen PASS |
| Media/Observability | 390 Checks, neun Dienste PASS; neue Discovery-/Syslog-Fixtures und zwei zusätzliche Assertions |
| Workflow | 15 Tests PASS |
| Auxiliary/Brew | 431 Checks, fünf Dienste/15 Ansichten PASS |
| Brew Rust Renderer | Ein echter Renderer-Test PASS, --locked |
| Edge/Hardware/RF | PASS |

Screenshots des Dashboard-Dienstbereichs, Subscriber Core, RF-Monitor mobil, Login mobil und Asset Management wurden visuell geprüft: Themes, Layout und vorhandene Bedienelemente erhalten. Testbilder unter target sind reproduzierbare Buildartefakte, keine produktiven Funk-/Gerätedaten.

VM-/Pi-/NAS- und On-Air-Grenzen bleiben bestehen.
