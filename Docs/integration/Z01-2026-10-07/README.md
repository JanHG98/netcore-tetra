# Z01.1–Z01.3: Quellvergleich, Integration und gemeinsames Prüfgate

Quell-/Prüfbericht: 07.10.2026; Statusnachtrag: 08.10.2026, Europe/Berlin. Auftrag: vollständiger Vergleich, kontrollierte Übernahme von Deployment/Discovery/Imagebuilder/Pi-VPN/Syslog, Inventar- und Readiness-Korrektur, Tests und Rückwege. Arbeitszweig: `feature/z01-deployment-consolidation`. Geprüfter Implementierungscommit: `67b2f0d6e2fbcbf7013ee9b0dae9b2a25c2d9315`.

## Ergebnis und Nachweisgrenze

| Aufgabe | Ergebnis | Nachweisstufe |
| --- | --- | --- |
| Z01.1 | Vollständige Tip-Bäume verglichen; Konflikte, Pakete, Übernahmeentscheidungen und Rückwege dokumentiert | Quellen, Dateiinhalte und Verträge geprüft |
| Z01.2 | Fehlende Entwicklung integriert; aktuelle UI/Fachänderungen erhalten; funktionale Fehler zusätzlich korrigiert | Implementierung und gezielte lokale Unit-/HTTP-/Wire-/Native-/Browserprüfungen |
| Z01.3 | Registry/Inventory/generierte Assets/Health/Fallback konsistent; Ready-Gates und gemeinsame CI ergänzt | Gemeinsames Quell-/Konfigurationsgate bestanden; eigene CI-/Betriebsnachweise separat |
| Z01.4 | In Arbeit; CT150-Installation / Readiness / Recovery und CT136-TCP-/Vorschau-/NAS-Pfad samt isoliertem Mountfehler bestanden | Reale Teilabnahmen dokumentiert; vollständiger ARM64-Build, Pi/SXceiver, VPN und weitere Ausfälle / Versionswechsel offen |

Die ursprüngliche Umsetzung entstand auf dem oben genannten Arbeitszweig und wurde mit [PR #62](https://github.com/JanHG98/netcore-tetra/pull/62) nach main übernommen (`c45a2ec1f5b7cdcfb766a2d81e8901b65f3dbacf`). Der aktuelle geprüfte Quellstand ist `001fb84ac566bb0f95e18d22439ee664fe0093e4`; beide zugehörigen main-Workflows bestehen. [Gesicherter Fortsetzungsstand vom 08.10.2026](checkpoint-2026-10-08.md), [Laufzeitnachtrag](runtime-z014.md) und [Imagebuilder-Nachtrag](imagebuilder-vm119.md) ergänzen die späteren Betreiberbefunde. Die Assistenz hat keinen direkten Zugriff auf die Anlage; tatsächliche Teilabnahmen beruhen auf den bereitgestellten Betreiber-Ausgaben. Der folgende Quellvergleich und seine datierten lokalen Prüfergebnisse bleiben als ursprünglicher Bericht erhalten. Ein statischer Audit-PASS bestätigt Verträge und Quellanschlüsse, keine funktionierende Gesamtflotte.

## 1. Beide vollständigen Quellstände

| Quelle | Vollständiger SHA / Umfang |
| --- | --- |
| Aktuelles main vor dem Auftrag | `dae9363062a1442664a083e1fc32e6944b3be9a6` – PR #61 bereits integriert; 2.580 Dateieinträge |
| Historische Implementierung | `bbf039729b9b05f8d623b11195ca24a124f68d16` – 2.234 Dateieinträge |
| Vereinigung | 2.655 unterschiedliche Pfade |
| Identische Pfade | 2.087; Dateityp, Git-Modus und Inhalts-Objekt-ID gleich |
| Nur historisch vorhanden | 75; fehlende Entwicklungsdateien |
| Unterschiedliche gemeinsame Pfade | 72; gezielte Inhaltsprüfung erforderlich |
| Nur im neueren main vorhanden | 421; nicht durch einen historischen Komplettbaum ersetzen |

Direkter rekursiver Tip-Vergleich mit `git ls-tree -rz --full-tree`, einschließlich Binärdateien und Dateimodi. Kein Merge-Base-/Dreipunktvergleich als Ersatz. Gleiche Blob-IDs belegen gleiche Inhalte; unterschiedliche Dateien wurden innerhalb ihrer Fachpakete semantisch geprüft.

Vollständige, reproduzierbare Nachweise:

- [source-comparison.csv](source-comparison.csv): alle 2.655 Pfade mit beiden Blob-SHAs, Modi, Typen und Vergleichsstatus.
- [source-differences.csv](source-differences.csv): sämtliche nicht identischen Pfade, einschließlich main-only.
- [source-summary.json](source-summary.json): Quell- und Tree-SHAs, Zähler und Methode.
- [integration-decisions.csv](integration-decisions.csv): Disposition der Unterschiede und der zusätzlich korrigierten gemeinsamen Dateien.

Reproduktion aus einem Checkout mit beiden Git-Objekten:

```bash
python3 tools/compare_repository_tips.py \
  --main dae9363062a1442664a083e1fc32e6944b3be9a6 \
  --historical bbf039729b9b05f8d623b11195ca24a124f68d16 \
  --output Docs/integration/Z01-2026-10-07
```

### Fehlende Dateien und Konfliktentscheidungen

| Bereich | Ausgangsbefund | Übernahmeentscheidung |
| --- | --- | --- |
| Deployment-Core | 49 historische Dateien fehlen vollständig | Controller, Agent, Job-/Git-Pinning, UI, Imageworker, Firstboot, VPN-Policy, Installer und Tests aufnehmen; neue Defaults auf `main` umstellen |
| Deployment-CI | Ein historischer Workflow fehlt | Wiederaufnehmen und Trigger auf aktuelle Quell-/Inventar-/Installationspfade erweitern |
| Observability/Syslog | 24 neue Dateien fehlen; elf gemeinsame Dateien unterscheiden sich | Discovery, Receiver/Sender, Puffer, Vorschau, Archiv, Units und Tests integrieren; bestehende Konfigurationen und moderne UI erhalten |
| Hardware-Gateway | Historischer echter Stop-Test fehlt; funktionale Shutdown-Hunks überschneiden sich mit neuem HTML | MQTT-Kindprozess verwalten und beenden, HTTP-Shutdown aus dem Signalhandler auslagern; moderne HTML-Blöcke erhalten |
| Dashboard | Historischer Server würde Assets, Session/Public-Kontrolle, Nachbarzellen und Tests zurücksetzen | Server und Assetstruktur aus main erhalten; Discovery-Link ausschließlich im aktuellen authentifizierten Dienstbereich ergänzen |
| Rust-/Python-Dienstoberflächen | Historische Inline-HTML-Versionen kollidieren mit externen Templates, Renderer, CSP und Darkmode | Aktuelle Renderer/Quelltemplates beibehalten; gemeinsame Discovery-Verknüpfung über aktuelle Einbettungswerkzeuge erzeugen |
| Rust Brew / Prüfwerkzeuge | Historische Fassung verliert eigene Workspace-/Docker-WebUI-Verträge und sucht veraltete Inline-Strings | Aktuelle Workspace-/Docker-/Templateprüfungen erhalten |
| Shared-/TBS-/IoT-Installer | Agentanschluss, TBS-Unit-Erkennung, Bash-Aufruf und Echo-Quotes fehlen | Nur die fachlichen Installer-Hunks übernehmen |
| SIP Switch | Historischer f-string nutzt Python-3.11-kompatible Quotes | Kleine Syntaxkorrektur; aktuelles generiertes HTML erhalten |

62 bereits in beiden Quellständen eingecheckte Python-Cachedateien wurden zusätzlich entfernt. Quelltexte bleiben erhalten; `.gitignore` schließt `__pycache__` und `*.py[cod]` aus. Referenz-/Handbuch-PDFs bleiben im Repository; das deterministische Softwarebundle schließt Dokumente, Buildprodukte, Caches und Gitdaten weiter aus.

## 2. Überschaubare Übernahmepakete und Reihenfolge

| Paket | Umfang / wichtige Pfade | Eintritt und Konfigurationsübernahme |
| --- | --- | --- |
| D1 – Controller und Agent | `system-backend/deployment-core/`, Shared-LXC-Anschluss, TBS-/IoT-Installer, Hardware-Shutdown | Lokale Registry-/Unitverträge prüfen; Konfigurationen erhalten; installierte Standort-TBS-ExecStart nicht ungeprüft ersetzen |
| D2 – Imagebuilder und Pi-VPN | `deployment-core/image*`, root-Worker, Firstboot, WLAN/OpenVPN, VM-Installer | Nach D1; volle Ubuntu-VM nötig; Import einer Standort-TOML; vollständiger Quell-SHA pro Auftrag; neue Images/defaults `main` |
| S1 – Discovery und Syslog | `system-backend/observability/`, rsyslog TCP/UDP/RELP, SQLite/JSONL, NFS/CIFS-Archiv | D1-Protokoll und Discoveryvertrag; Receiver/Sender separat vom Funkprozess; eigene URLs/Regeln und deaktivierte Targets erhalten |
| I1 – Inventar und Ready-Gates | `services.toml`, `deploy/open-lab/`, Health/Fallback-Matrix, Alert-Monitoring, Audit/E2E/CI | Nach Vertragsprüfung von D1/S1; explizit rendern, danach Drift ohne Selbstreparatur prüfen; Bereitschaft vor Folgepaketen erzwingen |
| U1 – Moderne Discovery-Verknüpfung | Shared-Service-Shell, generierte Bundles, Dashboard-Dienstbereich und Browsertests | Lokaler Agent auf Port 8321; bewusster Klick, kein Scan beim Seitenladen; Login-/Public-Ansichten erhalten ihre Zugriffsschranken |

Controller/Agent sind Komponente 0.3.0; dies ist keine neue konsolidierte NetCore-Releasefreigabe. Bestehende Ref-Auswahl in TOML/Settings wird nicht auf `main` zurückgesetzt. Vor einem Rollout die gewünschte Referenz auf einen vollständigen SHA auflösen und diesen gemeinsamen Stand verwenden.

Der Agentenkatalog enthält 27 verwaltbare Rollen: 26 Backendrollen einschließlich des optionalen Provisioning Core sowie TBS. Die zentrale Deployment-VM aktualisiert sich über ihren eigenen Controller-/VM-Pfad. Das Betriebsinventar enthält dagegen 26 reguläre Dienste einschließlich Deployment-Core; der optionale Provisioning Core wird dadurch nicht stillschweigend zum installierten Pflichtdienst.

## 3. Zugeordnete Inkonsistenzen und konkrete Korrekturen

| Befund | Korrektur / heutiger Vertrag | Verbleibende Grenze |
| --- | --- | --- |
| Inventory 25, Katalog/Audit 24; ältere feste 17/24-Sets | Gemeinsame Runtime-Registry; Inventory/Port/Security-Modus streng abgleichen; explizit neu generierter Katalog mit 26 Diensten | Deklarierte Rollen sind keine Live-Dienstzahl |
| Alert-Service fehlt in Matrix und Monitoringvertrag | 25 Node-Gateway-Targets, 26 TBS-Fallback-Einträge, 26 Observability-Targets; echte öffentliche Metrics/OpenAPI; Verwaltung weiter tokenpflichtig | Anonyme 401-Prüfung bestätigt Schutz, keine authentifizierte Warnzustellung |
| Deployment-VM fehlt | Beispiel `10.0.20.35:8320`, Unit `netcore-deployment.service`, voller VM-Installer | VM/Image- und Ressourcenabnahme folgt Z01.4 |
| Root-TBS nutzt `10.0.1.179`, Inventarbeispiel `10.0.20.10` | Standort-TBS unverändert; explizites `[tbs_site]` streng gegen Host/Port/`/ws/node` prüfen; Netzunterschied im Audit sichtbar | Tatsächliche Gateway-Identität/Erreichbarkeit beider Netze vor Installation prüfen |
| Negatives Ready-Ergebnis wurde verworfen | Beide Deploymentpfade warten begrenzt; Timeout/Fehler beenden Auftrag und Folgeinstallationen mit Fehlerstatus/Exit 1 | Bereits geänderte Binaries werden nicht pauschal automatisch zurückgerollt |
| HTTP 200 kann `ready:false`/`status:degraded` enthalten | Readiness semantisch prüfen; negative Werte und ungültige JSON-Antworten ablehnen; ältere gültige Statusobjekte ohne einheitliche Keys kompatibel halten | Eigene Health-/Fachabnahme pro Dienst bleibt nötig |
| Controller akzeptiert altes Agent-`succeeded, ready=false` | Ergebnis ablehnen; keinen zweiten Installations-POST senden; keinen falschen neuen Commitmarker setzen | Unsicherer/teilweise geänderter Host wird ausdrücklich als nicht abgenommen geführt |
| Installer können lokale Konfiguration überschreiben | Vorher sichern und per EXIT-Trap erhalten; nur explizites `--replace-config` ersetzt mit datierter Rückwegdatei | Host-Token/Secrets bleiben lokal; keine automatische gesamte Datenbank-/Binärtransaktion |
| Generator kann Drift beim Prüfen reparieren | `check-generated` vergleicht temporäre Sollausgabe ohne Änderung; fehlende/veraltete/überzählige Assets führen zu Fehler | Template-/Inventoryänderungen erfordern bewussten `render` |
| Syslog-Zielwechsel mit fehlgeschlagenem Restart | Konfigurationsdatei atomar zurücksetzen, vorheriges Ziel starten, Fehler sichtbar halten; späteren Retry ermöglichen | Reale systemd-/Journald-/Netzabnahme fehlt |
| Neue Probeadresse übernimmt alte Erfolgsdaten | Alte Metrics/Antwortzeit/Erfolg invalidieren; manuelle/deaktivierte Konfiguration und letztes gültiges Discoveryziel erhalten | Ausfallcache beweist keine Erreichbarkeit |
| Hardware-Gateway hängt bei SIGTERM | Shutdown in eigenen Thread; MQTT-Prozess terminate/wait/kill/reap | Echter lokaler HTTP-/Prozesstest bestanden; Geräte-/LXC-Dauerbetrieb separat |

## 4. Tests und Rückweg je Paket

| Paket | Positive Prüfung | Fehler-/Ausfallfälle | Rückweg |
| --- | --- | --- | --- |
| D1 | Echter HTTP-Controller/Agent; Rollenauflösung; Git-SHA; persistente Aufträge; bestehende Runtime-Änderungen | Controller weg/Cache erhalten; Rollenkonflikt; falsches Netz/Environment; einmaliger verlorener POST; Readiness fehlt | Vor Installation Paketcommit revertieren. Nach Installation vorige geprüfte Agent-/Controller-Version herstellen; aktive Jobs vor Wiederholung am Ziel prüfen; Config/SQLite/Caches sichern und erhalten; gezielt Discovery-Drop-ins zurücknehmen |
| D2 | Personalisierung, SSH-/VPN-Validierung, Hash/Redaktion, Downloadranges; Browser-Imageauftrag/Download | Workerfehler isoliert; ungeeignete VM vor Änderung abgewiesen; Unix-Socket-/native Image-Smokes in passender Umgebung | Aktive Builds zuerst abschließen/gezielt beenden; alte VM-Software aus geprüftem SHA; Images/DB erhalten. Pi mit vorherigem geprüftem SD-Image bzw. gesicherten Units/Config; VPN-Timer und gewünschte explizite VPN-Betriebsweise kontrollieren |
| S1 | Echte TCP/UDP/RELP-Eingänge und Queue-Reconnect; Rust-NMS, SQLite-Outbox, Vorschau und Prometheus-SD | Ungültige Quelle/Adresse; Receiverrestart; Cache/Retry/Deduplizierung; kein Mount; Share-/Prüfsummenfehler; Puffer-/Freiplatzlimits; Symlink-/Retentiongrenzen | Receiver/Vorschau/Archiv- und Sendertimer kontrolliert stoppen; vorheriges Binary und TOML/JSON/Drop-ins herstellen; Segmente/SQLite/Cursor/verifizierte Archive behalten; keine Recordings fremder Namespaces löschen |
| I1 | 26er Quellenvertrag, deterministisches Bundle, echte 503→200-Recovery, Konfig-Erhalt | HTTP 200 mit negativer Ready-Aussage; Timeout stoppt Dependencies; Exit 1; Port/Auth/Missing-Service-Drift; veraltete Assets bleiben beim Check unverändert | Zusammenhängenden Inventory-/Vertragscommit zurücknehmen; gespeicherte Config mit Originalrechten herstellen und Ready prüfen; Serviceinstaller-Rückweg für Binaries/DB verwenden |
| U1 | Echte Browseraktionen, Themes/Reload, Desktop/Mobile, bestehende Schreib-/Navigationspfade | Kein Scan beim Laden/Navigieren/Themewechsel; Login-/Public-Grenzen; IPv6-/URL-Behandlung; keine JS-Fehler/Überläufe | Gemeinsame Shell und Quelltemplates auf vorherigen Stand setzen, vorhandene Generatoren ausführen; Fachzustand/Backend-/Funkcode dabei erhalten |

Rollback ist absichtlich paket- und zustandsbezogen. Weder der Ready-Fehler noch ein Git-Revert stellt automatisch alle bereits veränderten Anlagenbinaries/Datenbanken zurück. Vor Z01.4 Snapshot/Backup, Installations-SHA, Binaryversion, Unit/ExecStart, Konfigpfad und Rückweg auf dem jeweiligen Host erfassen.

## 5. Ausgeführte Nachweise und CI

Die Fachnachweise und Befehle stehen zusätzlich in [deployment.md](deployment.md), [syslog.md](syslog.md), [inventory.md](inventory.md) und [ui.md](ui.md). Gemeinsamer Einstieg:

```bash
PYTHONDONTWRITEBYTECODE=1 python3 tools/check_z01_integration.py
```

| Prüfung | Ergebnis |
| --- | --- |
| Vollständiger Tipvergleich / Inhalts- und Vertragsprüfung | Bestanden; 2.655 Pfade, 75 fehlende Dateien und 72 Überschneidungen erfasst |
| Gemeinsames Z01-Quell-/Konfigurationsgate | PASS: 26 Dienste/Endpunkte, 25 Gateway-Targets, 26 Fallback-Modi, 13 deklarierte E2E-Szenarien |
| Deployment-Unit-/HTTP-/Imagevertrag | 50 Tests: 49 bestanden, ein echter AF_UNIX-Workertransport in dieser Umgebung ausdrücklich übersprungen |
| E2E-/Ready-/Drift-/Config-Regressionen | 22 Tests bestanden |
| Warnzentrale | 77 Tests bestanden; Metrics/OpenAPI und Token-Schutz mit echten Loopback-Anfragen |
| Hardware-Gateway SIGTERM/MQTT | Echter lokaler HTTP-/Prozesstest bestanden |
| Syslog | 18 Unit- und zwei echte rsyslog-Wiretests bestanden; TCP/UDP/RELP und Receiverrestart |
| Observability Rust | Sieben Tests und Binary-Build bestanden |
| Native NMS/SQLite/Prometheus-SD samt WebUI | Bestanden; Discovery-, Archivfehler-, Verlust-/Logfilter- und Darkmode-Persistenzprüfung |
| Deployment-WebUI | Bestanden; Discovery, TBS-Assistent, Remote-/Imageauftrag, Download/Löschung, Desktop/Mobile; Installer/Imagebuild ausdrücklich simuliert |
| Browserregression bestehender Oberflächen | PASS; Shared 93, Dashboard 137, Core 405, Media 390, Workflow 15, Auxiliary 431 sowie Edge und Brew-Renderer; Details in `ui.md` |
| Syntax, generierte Bundles und Whitespace | Bestanden; Generatoren synchron, Python-3.11-SIP-Syntax erhalten |

CI-Verträge:

- `.github/workflows/deployment-consistency.yml`: gemeinsames Quell-/Driftgate, echte Alert-HTTP-Prüfung sowie reale rsyslog/Rust/Native-/Browser-Syslog-Prüfung.
- `.github/workflows/deployment-discovery-tests.yml`: Controller/Agent/Browser, native Ubuntu-24.04/26.04-VM-/Image-Smokes, Workspacecheck und Link des TBS-Imagebinaries.
- Bestehende Service-/Dashboard-WebUI-CI bleibt erhalten; die Tests berücksichtigen neue Discovery-/Syslog-Antworten.

Ein lokales PASS wird nicht als durchgelaufene GitHub-CI ausgegeben. CI-Lauf-SHA und Ergebnis beim PR prüfen; native Unix-Socket-/VM-/Image-Nachweise sind getrennt von den hier ausführbaren Tests. Der reale vollständige NetCore-ARM64-Imagebuild und physische Pi/SXceiver-Boot bleiben Z01.4, auch wenn ein Personalisierungssmoke grün ist.

Der [GitHub-CI-Nachtrag](ci.md) dokumentiert die ersten echten PR-Ergebnisse und die Korrektur des temporären rsyslog-Testaufbaus. Aktuelle main-CI und ihre Quell-SHAs stehen ebenfalls in diesem Nachtrag; historische PR-Ergebnisse bleiben entsprechend datiert.

## 6. Nächster Übergang

**Z01.4 ist in Arbeit.** VM119 und die isolierten CT150-/CT136-Piloten sind zugeordnet; ihre bestandenen Teilabnahmen und Grenzen stehen im [Fortsetzungsstand](checkpoint-2026-10-08.md). Nächster Nachweis ist die tatsächliche Übernahme des Gast-APT-Fixes001fb84 auf VM119 und ein vollständiger ARM64-Build mit verifiziertem Image / Manifest / SHA-256. Anschließend folgen physischer Pi-/SXceiver-Boot und VPN-Wechsel. Einen bereits laufenden Build über den bestehenden Auftrag verfolgen. Gezielte P0-Arbeiten aus Z02 bleiben parallel möglich; die bisherigen Befunde schließen Z01.4 insgesamt noch nicht ab.
