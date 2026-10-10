# Z01.3 – Inventory / Ready-Schranke / gemeinsame CI

**Dokumenttyp: datierter Arbeits-, Prüf- und Betreiberbefund.** Der Bericht hält die Beseitigung der damaligen 25/24-Dienstdrift und die semantische Readiness fest. Das aktuelle Beispiel-Inventory deklariert 26 Runtime-Dienste; die Registry enthält zusätzlich `shared` mit `runtime = false`, insgesamt 27 Einträge. Der optionale Provisioning Core gehört zum getrennten Agentenkatalog, nicht zur Registry oder zum regulären Inventory. Installierte Anlagenzustände bleiben eigene Nachweise.

Heutiger Einstieg: [Deployment-/Imagebuilder-Anleitung](../../services/deployment-core/README.md) · [Integrationsübersicht](README.md) · [Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md). Quellbeschreibung und tatsächliche installierte Version bleiben getrennt.

## Datierte Originalbefunde

### Z01.3 – Inventory / Ready-Schranke / gemeinsame CI

Stand 2026-10-07; Ausgangspunkt `main@dae9363062a1442664a083e1fc32e6944b3be9a6`, historische Deployment-/Syslog-Quelle `bbf039729b9b05f8d623b11195ca24a124f68d16`. Arbeitsbranch `feature/z01-deployment-consolidation`. Keine Live-LXC-/VM-/Pi-/Funk-Abnahme durch diesen Teilauftrag.

## Befund und Entscheidung

- Main-Inventory hatte 25 Dienste, generierter Katalog und Full-System-/E2E-Erwartungssets 24. `alert-service` fehlte in Node-Gateway-/TBS-Fallback-Matrix und generiertem Katalog. Das `security_mode=token` des aktuellen Warning-Dienstes wurde von alten Open-Lab-Prüfungen fälschlich als Fehler bewertet; E2E versuchte dessen Status ohne Token zu lesen. Der Dienst besaß keine echten `/metrics`-/`/openapi.json`-Routen.
- Deployment/Discovery-Controller wird als 26. Backend-Rolle integriert: beispielhaft VM `10.0.20.35:8320`, User `netcore-deploy`, Unit `netcore-deployment.service`, Installer `install-vm.sh`. VM/Imagebuilder-Rolle benötigt eine vollständige Ubuntu-VM. Hardware-Abnahme folgt Z01.4.
- `tools/deployment_inventory.py` liest die Runtime-Registry aus `system-backend/services.toml`; Full-System-, Shared-Platform- und E2E-Prüfung vergleichen Inventory dagegen (einschließlich Port und Security-Modus). Wiederholt hartkodierte 17/24-Dienstsets wurden entfernt. Katalog, Hosts, CSV, Graph und alle Templates sind neu erzeugt, mit LF statt CRLF für CSV.
- Node Gateway hat jetzt 25 Health-Targets; TBS-Fallback-Map hat 26 explizite Einträge. Alert-/Deployment-Ausfall sind nicht kritische Radio-Ausfälle: lokale Radio-/SDS-Dienste bzw. bereits installierte Dienste laufen weiter. Observability ergänzt seine 26 Targets im parallelen Syslog-Paket.
- Bestehende TBS-Standortadresse `10.0.1.179:8080/ws/node` bleibt erhalten. Backend-Inventar verwendet Beispielnetz `10.0.20.*`. Neue explizite Inventory-Tabelle `[tbs_site]` wird streng gegen `config.toml` geprüft; Unterschiede zu Backend-Beispieladressen sind als Z01.4-Live-Zuordnungsgrenze in README und Audit ausgewiesen. Kein statisches PASS behauptet Identität/Erreichbarkeit der zwei Adressen.
- Bestehende pauschale PDF-Sperren machten Prüfungen mit 39 absichtlich eingecheckten Handbuch-/ETSI-/Hardware-PDFs rot. Dokumente bleiben zugelassen; Deployment-Bundle schließt PDFs weiterhin aus, bestätigt durch eine Reproduzierbarkeits-/Ausschlussregression. 62 eingecheckte Python-Caches werden vom Root-Agent gezielt bereinigt. Fünf dokumentierte CLI-Dateien und die neue gemeinsame CLI benötigen Modus 100755.

## Umgesetztes Ready-/Config-Verhalten

`deploy/open-lab/netcore-deploy.py`:

- `ready_timeout_secs` (Default 60, Bereich 1–3600) begrenzt die Startphase; `health_timeout_secs` begrenzt die Einzelanfrage. Nach jedem Restart wird `/health/ready` geprüft und bis zur Frist erneut angefragt. Fehler beendet `apply` mit Exit 1, bevor abhängige Installer starten. `--dry-run` bleibt ohne Netzwerk-Probes/Remote-Änderungen.
- Ready-HTTP2xx allein genügt nicht: gültiges JSON-Objekt ist Pflicht; explizites `ready:false` oder `status` degraded/failed/unavailable/not_ready/error/unhealthy/down wird abgewiesen. Bestehende Status-Objekte ohne uniforme `ready`-/`status`-Keys (KMF/IP/Media/SIP) bleiben kompatibel; Ready-Body ist bis1MiB begrenzt statt der früheren4096-Diagnosebytes. Liveness bleibt unabhängig transportbasiert.
- Bereits bestehende Host-Konfiguration wird vor dem Installer temporär gesichert und danach wiederhergestellt. Ein EXIT-Trap stellt sie auch bei einem fehlgeschlagenen Installer zurück. Frische Installationen bekommen gerenderte Dependency-URLs; der Alert-Token bleibt im lokalen separaten EnvironmentFile seines Installers.
- Nur `apply --replace-config` ersetzt bewusst lokale Einstellungen; vorherige Datei bleibt als `<config>.pre-netcore-<UTC-Zeit>-<PID>`. Der Schalter stellt keine pauschale Binär-/Datenbank-Rückkehr bereit.
- `check-generated` rendert Vergleichsausgaben in ein temporäres Verzeichnis und prüft fehlende/veraltete/überzählige Assets ohne die geprüften Artefakte zu reparieren. Damit kann CI Katalogdrift wirklich erkennen. Nach beabsichtigten Template-/Inventaränderungen wird explizit `render` ausgeführt.
- Doppelte Managementports werden abgewiesen (Port-basiertes URL-Rendering erfordert Eindeutigkeit). Unbekannte Dependencies liefern `DeployError` statt KeyError.

## Warning-Vertrag

`system-backend/alert-service/main.py` ergänzt echte öffentliche Prometheus-Gauges für Ready und Delivery-Enabled sowie OpenAPI 3.0.3. OpenAPI beschreibt die geschützten Verwaltungsrouten und Bearer-Auth. Keine Alert-/Gerätewerte, Credentials oder Fehlertexte in den öffentlichen Metrics. HTTP-Header weist Token-/Open-Lab-Modus aus. Management bleibt tokenpflichtig. E2E prüft beim Token-Dienst den anonymen Statuszugriff auf 401; es speichert/liest dafür keine Tokens. Das ist ein Schutz-/Contract-Nachweis, keine authentifizierte fachliche Warning-Abnahme.

## Tests und Ergebnisse

- `PYTHONDONTWRITEBYTECODE=1 python3 tools/check_z01_integration.py`: PASS. Enthält Inventory-Validierung, unveränderte generierte Assets, Shared Platform, E2E-Paket/Unit-/Validate-only-Auswahl, Full-System-Audit und Observability-Static-Check. Ergebnis: 26 deklarierte Dienste, 26 Management-Endpunkte, 25 Node-Gateway-Targets, 26 TBS-Fallback-Modi, 13 E2E-Szenarien.
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s tests/e2e/unit -v`: 22 Tests PASS, einschließlich 12 neuer Deployment-/Drift-/Site-Mapping-Regressionen: positive Dependency-Folge, negative Ready-Schranke (keine nachfolgenden Installer), CLI Exit 1, echte Loopback-503→200-Wiederkehr, HTTP200+ready:false/negative Statuswerte werden abgewiesen, ungültiges Nicht-JSON wird abgewiesen während legacy Statusobjekte auch >4096Bytes funktionieren, Ready-Timeout, Dry-Run ohne Probe, stale/überzählige Assets ohne Check-Mutation, duplicate Port/unknown Dependency, lokaler Config-Erhalt nach Installer-Überschreibung, deterministisches Bundle ohne PDF/Build/Caches, Registry-Port-/Security-/Missing-Service-Drift sowie strikte Standort-Mapping-Abweisung.
- `PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s system-backend/alert-service/tests -v`: 77 Tests PASS einschließlich neuem echten Loopback-Test für öffentliche OpenAPI-/Metrics-Routen, Ready-Wechsel, keine Secret-Ausgabe und fortbestehenden 401-Schutz.
- `git diff --check`: PASS nach CSV-LF-Korrektur.

Gemeinsame neue CI: `.github/workflows/deployment-consistency.yml`; Einstieg `tools/check_z01_integration.py`. Native VM/Image/ARM64-Prüfungen bleiben im parallel wiederaufgenommenen Deployment-Workflow. Shared-CI führt Quell-/Konfigurationschecks und echten Warning-Loopback-Kontrakt aus; zuletzt git diff gegen generierte Assets/Audit. Kein Live-Netz und keine Funkübertragung.

## Rückweg und offene Betriebsabnahme

1. Source-Rückweg: zusammenhängenden Z01-Integrationscommit auf einem Recovery-Branch revertieren, historische Dateien nicht pauschal über neues main kopieren. Package-unabhängige UI/Funk-Fachänderungen bleiben erhalten.
2. Config-Rückweg bei explizitem `--replace-config`: passende `.pre-netcore-*` mit Originalrechten an den ursprünglichen Konfigurationspfad kopieren, Dienst restart, `/health/ready` prüfen. Datenbanken/Recording/Token-Env bleiben erhalten.
3. Binär-/Dienst-Rückweg: jeweilige Backup-/Recoveryprozedur des Service-Installers verwenden; die Ready-Schranke stoppt Folgepakete, setzt aber keine bereits erfolgreich aktualisierten Binaries automatisch zurück.
4. Z01.4 offen: echte Neu-/Wiederholungsinstallation, Standort-IP-Zuordnung, Dependency-Ausfall/Wiederkehr, lokale Credentials/Settings, Controller-/NAS-Verlust, VM/Pi-Start, vollständiger ARM64-Link-/Image-Build und On-Air-Prüfung mit Geräten. Statische Matrix und Loopback-Tests ersetzen diese Abnahme nicht.

Dateien dieses Teilpakets: `deploy/open-lab/` samt generated, additive Fallback-/Health-Abschnitte in `config.toml` und Node-Gateway/Control-Room-Beispielen, Alert-HTTP-/Tests, E2E-Model/Inventory/Scenarios/Unit-Test, Tools-Checks/README, gemeinsame neue CI, generierter `Docs/generated/systemintegration-pruefbericht.md`. `Docs/roadmaps/gesamtroadmap.md`, Gesamtintegrationsbericht und `services.toml` bleiben unter Root-Ownership.
