# Z01.2 / Z01.3 – Observability / Syslog Integrationsnotizen

Stand: 2026-10-07 Europe/Berlin. Ausgangsbaum `main@dae9363062a1442664a083e1fc32e6944b3be9a6`, historischer Quellbaum `bbf039729b9b05f8d623b11195ca24a124f68d16`, Arbeitsbranch `feature/z01-deployment-consolidation`.

## Direkter Tipvergleich und Entscheidungen

`git diff --name-status dae9363 bbf0397 -- system-backend/observability tools/check_observability.py` zeigt 37 differierende Pfade (historisch 2.125 hinzugefügte / 156 entfernte Zeilen). Neue Discovery-, log-store-/client-, Konfigurations-, Installer-, Unit-, Archiv- und Testdateien fehlen auf main; README, config, Collector, State, HTTP, main, Installer und Prometheus überschneiden sich. Der historische HTTP-Code enthält die WebUI als eingebetteten String und würde die neuere externe `web-ui/index.html`, den gemeinsamen `service_design::render`, Darkmode und `img-src`-CSP verlieren. Daher keine vollständige historische http.rs und keine historische Löschung von index.html: nur neue API/OpenAPI-Hunks sowie die fachlichen Discovery-/Syslog-Blöcke in die aktuelle HTML-Datei übernommen. Der aktuelle gemeinsame Renderer und CSP bleiben erhalten.

| Paket | Geänderte / neue Pfade | Ergebnis |
| --- | --- | --- |
| Discovery | src/discovery.rs, config.rs, main.rs, collector.rs, state.rs, http.rs, web-ui/index.html | Controller-Protokoll/Environment prüfen; IPv4-Allowlist; Konflikte überspringen; letzte gültige Ziele bleiben bei Controllerausfall und Restart; manuelle Targets erhalten; veraltete laufende Scrapes verworfen |
| Syslog-Empfang / Vorschau | logging/log_store.py, receiver.rsyslog.conf, logging/apparmor-netcore, config/syslog.example.json, systemd/netcore-syslog* | Separater TCP/UDP/RELP-Receiver; begrenzte JSONL-Segmente und SQLite-Vorschau; übersetzte Severity; Vorschau-IDs in NMS dedupliziert; Status/Verluste in aktueller WebUI |
| Verifiziertes Archiv | log_store.py, netcore-syslog-archive.service/.timer | Echter NFS/CIFS-Mount erforderlich; kein lokales Ersatzarchiv; gzip-Readback und SHA-256 vor Löschen; Retention nur eigener Collector-Namespace; NAS-I/O hält Receiver-Lock nicht |
| Journald-Sender | logging/log_client.py, log-client.example.json, netcore-log-client.service/.timer, install-log-client.sh | Discoveryvalidierung vor atomarem rsyslog-Dateiaustausch; Fallback nur ohne vorherige Konfiguration; Queue/Journald begrenzt; neuer zusätzlicher Rückweg bei fehlgeschlagenem Senderneustart |
| Install/Update | install.sh, update.sh, uninstall.sh, install-logging.sh, rsyslog-apparmor.sh, seed-openlab.py | Install und Update gemeinsamer Pfad; Binary atomar ausgetauscht; bestehende Konfigurationen erhalten; seed erstellt Backup nur bei Änderung; eigene URLs/Regeln/manuell/deaktiviert erhalten |
| Inventory/Probevertrag | observability.example.toml, openlab-hosts.json, log-client/syslog JSON, stack/prometheus/prometheus.yml, tools/check_observability.py | 26 reguläre Inventory-Dienste einschließlich self, alert-service, deployment-core; gleiche .20-Beispieladressen; Controller .35:8320, Observability .26:8210; Prometheus verwendet NMS HTTP-SD statt zweite statische Liste |

Historische 10.0.1.*-Beispieladressen / eine alte 29-Maschinen-Aufzählung wurden nicht als aktuelle Installation übernommen. Config-Samples verwenden das reguläre `deploy/open-lab/inventory.example.toml` (26 Dienste). Ein bestehendes individuelles .20- oder .1-Ziel wird vom Seed und vom Neustart nicht auf die Samples zurückgestellt. Nur ältere Loopback-Adressen werden initial auf die Konfigurationsadresse übernommen; `labels.discovery=manual` optiert vollständig aus. Im aktuellen Rust Default fehlt provisioning-core bewusst: das ist kein regulärer 26er-Inventory-Dienst. Alert Service behält Management-Token; öffentliche Health-/Metrics-Verträge werden von Inventory-Agent ergänzt. Deployment-Core wird von Deployment-Agent mit realem /metrics-Vertrag ergänzt.

Zusätzlich zu historischen Hunks behoben:

- Eine fehlgeschlagene `systemctl try-restart`-Änderung hätte eine neue rsyslog-Konfiguration auf Disk hinterlassen, die beim nächsten Discoverytick als bereits angewendet galt. Jetzt wird die vorherige Datei atomar zurückgesetzt, ein Start des vorherigen Zieles versucht und der Originalfehler bleibt sichtbar. Ohne vorige Konfiguration wird der fehlgeschlagene neue Cache entfernt.
- Bei einer geänderten Zieladresse werden auch Metrics-Erfolg, Antwortzeit und letzte Probezusagen invalidiert; alte Erfolgsmessungen können das neue Ziel nicht als bereits geprüft ausweisen.
- Die statische Paketprüfung prüft jetzt Inventory-/Target-Gleichheit, Controller, Senderfallback und Quellenbeispiele sowie JSON-Limits. Statt abgefangene Vertragsfehler zu verschweigen, meldet sie einen Fehler.

## Prüfungen und konkrete Fehlerfälle

Bislang bestanden:

- `python3 tools/check_observability.py` (inkl Node syntax, TOML/JSON, Shellsyntax, Reference und 26er-Inventory-Vertrag).
- Python Syslogunittests: 20 Tests PASS einschließlich der beiden echten Wiretests mit temporär entpacktem rsyslog/RELP; 18 fachliche Tests prüfen insbesondere Offline/Fallback/Cache, Konflikt/unzulässiges Netzwerk, vor dem Austausch abgelehnte rsyslog-Konfiguration, erfolgreicher Retry nach fehlgeschlagenem Restart, kein Mount/kein lokales Ersatzarchiv, Share-Kopierfehler, Prüfsummenfehler, Symlinkschutz, Segment-/Vorschau-/Freiplatzlimits, unvollständige Zeile nach Neustart, Unicode-Kürzung, Quellallowlist und Retention ohne Zugriff auf Recordings oder andere Collectorordner.
- `git diff --check -- system-backend/observability tools/check_observability.py`.

Weitere Ergebnisse:

- Root: `cargo test --locked -p netcore-observability`: 7 Rusttests PASS, darunter Discovery-Protokoll/Identity/Allowlist, persistente Cachemigration, aktuelle eigene Managementadresse/deaktiviertes Target, veraltete Scrape-Erfolge und Preview-Deduplizierung.
- `cargo build --locked -p netcore-observability`: PASS.
- Echte rsyslog-Wiretests: zwei Tests PASS nach temporärem Entpacken von rsyslog-relp/librelp; Journald-Senderkonfiguration tatsächlich mit `rsyslogd -N1` geprüft, echte TCP/UDP/RELP-Eingänge und Queue-Reconnect nach Receiverrestart geprüft. Keine Skips in diesem Lauf.
- `python3 system-backend/observability/tests/native-syslog-smoke.py`: PASS. Tatsächlicher Rust-Prozess auf freiem Loopbackport, echte SQLite-Outbox, Previewforwarder, Ziel-SD und Archivfehler-API geprüft.
- `native-syslog-smoke.py --browser`: PASS mit offiziellem Chrome-for-Testing Headless Shell 155.0.8059.39 auf freiem Loopbackport. Tatsächliche aktuelle Rust-WebUI geprüft: Discovery deaktiviert / bestehende Targetadresse, NAS-Fehlermeldung, Verlustzähler, Suchfilter / genau ein Logtreffer, Darkmode-Umschaltung und gespeichertes Theme nach Reload; keine JavaScript-Seitenfehler. Der erste Full-Chrome-Werkzeugversuch war unbrauchbar, der Headless-Shell-Fallback beseitigt diese lokale Testblockade.

Zusätzliche bestehende WebUI-Regression mit nachgeführten Fixtures: `tools/test_media_service_ui.mjs` benötigte die neuen echten Routen `/api/v1/discovery` und `/api/v1/syslog`; typisierte Vorinstallationsantworten ergänzt und zwei eigene UI-Assertions hinzugefügt, ohne Checks zu lockern. Vollständiger Lauf mit offiziellem Headless Shell: **390 Prüfungen über neun Dienste PASS**, darunter Observability-Tabaktionen, Desktop/Mobilumbruch, Kontrast und gespeicherte Hell-/Dunkel-Themes. Die übrigen bestehenden Dienstoberflächen bleiben in derselben Regression geprüft.

Kein laufender Anlagenhost wurde kontaktiert.

## Rückweg und verbleibende Abnahme

Repository-Rückweg: nur dieses Paket bzw. den konsolidierten Integrationscommit zurücknehmen; keine historischen Commitstapel erneut einspielen. Eine Installation erfolgt erst auf dem gemeinsamen, geprüften Release. Vor Installation Binary, TOML/JSON, aktive Units und Journald-Drop-ins sichern. Bei Rücknahme Receiver/Vorschau/Archivtimer und optional Sender-/Discoverytimer stoppen, vorheriges Binary und gesicherte TOML wiederherstellen. Rohsegmente, SQLite, Journalcursor und bereits verifizierte Archive behalten. Alte Rust-Deserializer ignorieren das zusätzliche Discovery-Statefeld; bestehende Targets bleiben lesbar. Journaldlimit-Drop-in bei Bedarf durch gesicherten vorherigen Stand ersetzen und journald neu starten.

Z01.4 bleibt offen: echte NFS-/CIFS-Mount- und UID/GID-Prüfung, erster verifizierter Archivlauf auf Betreiber-NAS, Senderjournald/RELP auf echten LXCs/VM/Pi, Controller-/NAS-Ausfall und Rechte-/Speicherlimits auf der Anlage, Install-/Upgrade-/Rollbacktest mit echten service units. Lokale Mock-/Loopback-Nachweise sind keine installierte 26-Dienst-Flotte, keine echte NAS-Abnahme und keine On-Air-Abnahme.
