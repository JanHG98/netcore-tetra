# Z01 – Deployment / Discovery / Imagebuilder / Pi-VPN

Stand: 2026-10-07 Europe/Berlin. Arbeitszweig: `feature/z01-deployment-consolidation`.
Main-Baseline: `dae9363062a1442664a083e1fc32e6944b3be9a6`. Historischer vollständiger Quellbaum: `bbf039729b9b05f8d623b11195ca24a124f68d16`.
Die nachstehenden Änderungen sind Arbeitsbaum-Implementierung, keine Anlageninstallation oder main-Merge.

## Übernahme und Verträge

- Historisches `system-backend/deployment-core/` (49 Dateien) sowie `.github/workflows/deployment-discovery-tests.yml` kontrolliert übernommen. Im Ausgangsmain fehlt der gesamte Ordner. Keine neueren TBS-, Control-Room- oder Dienst-WebUIs durch historische Komplettbäume ersetzt.
- Neue Komponente 0.3.0: neue Controller/Agent-Konfigurationen und LXC-Bootstrap wählen `main`; bestehende TOMLs, `settings.json`, Ref-Auswahl und Standortkonfigurationen bleiben erhalten. Alte Feature-Refs müssen ausdrücklich im UI auf den gewünschten Stand umgestellt werden.
- Katalog: 27 verwaltbare Rollen = 26 Backend-Rollen (einschließlich optionalem Provisioning Core) plus TBS. Der Controller-/Image-VM-Selbstupdate bleibt eigener `update.sh vm`-/`controller`-Pfad und wird nicht als weitere Agent-Rolle erfunden. Der tatsächliche Beispielbetrieb folgt dem zentralen Inventory, nicht historischen IP-/Hostzahlen.
- Alle 27 Installer und Konfigurationsvorlagen sind vorhanden; die ExecStart-Verträge aller 26 Backendrollen stimmen exakt mit deren aktuellen systemd-Unitdateien überein. TBS ist wegen bewusst beibehaltener Legacy-/benutzerdefinierter Units separat geprüft.
- `bindings.py` löst die aktuellen, top-level `subscriber_core`-/`group_core`-Felder des Provisioning Core korrekt auf; explizite Rollen einschließlich Observability unterstützt. Unverwandte URLs und URLs mit eingebetteten Zugangsdaten bleiben erhalten.
- `launch.py` erhält UI-Änderungen an Runtime-TOMLs bei Neustart, priorisiert ausdrückliche neue Source-Änderungen und erhält vorhandene TBS-Fallbackdateien. Diese vorhandenen Verträge sind durch Tests abgedeckt.
- Root übernimmt die zugehörigen Shared-LXC-/Installer-Anschlüsse, IoT-Bash-Migrationskorrektur und aktuelle TBS-Update-Unitauflösung. Der historisch übernommene Regressionstest hatte den fehlenden Bash-Aufruf als echten Fehler nachgewiesen; nach der gezielten Rootkorrektur bestanden.

## Ready-Schranke und API

- Discovery löst weiterhin lebendige, noch eingeschränkte Dienste auf; dies verhindert gegenseitige Abhängigkeitsblockaden bei der Erstinstallation und bleibt unabhängig vom Controller verfügbar.
- **Deploymentabschluss verlangt jetzt Readiness.** Ein lebendiger, noch nicht bereiter Dienst wird für bis zu 60 Sekunden weiter geprüft. Fehlende Readiness führt zu fehlgeschlagenem Job; weder neuer Commitmarker noch Scan-Erfolg werden gesetzt. Ein gestarteter Installer kann Dateien geändert haben: Marker behält leeren aktuellen Commit plus `previous_commit` und `requested_commit`, Konfigurationssicherung bleibt erhalten.
- Ein neuer Controller lehnt auch das historische Agent-Ergebnis `status=succeeded, ready=false` als fehlgeschlagen ab; er sendet den Installations-POST nicht erneut.
- Echte öffentliche `/metrics`- und `/openapi.json`-Endpunkte ergänzen den gemeinsamen Management-Vertrag. Metrics zählt nur cached Peers und die letzten höchstens 100 Aufträge; ein Scrape führt keine Discovery-, Git-, Installations- oder Healthoperation aus. Keine Zugangsdatenlabels. OpenAPI 3.0.3 dokumentiert die gemeinsamen lesenden API-Pfade.

## Pakete, Tests und Rückweg

| Paket | Änderungen / Reihenfolge | Prüfung | Rückweg |
| --- | --- | --- | --- |
| D1 Controller / Discovery | Gesamter Python-Dienst, Katalog, HTTP/API/UI, Jobs, Git-Pinning, Agent-LXC-Anschluss | Unit/echte HTTP-Verbindungen, Seed-Relay, Multicast, Rollenkonflikt/Ausfallcache, exaktes SHA, Lock-/Cache-Recovery, Runtime-Edit-Erhalt | Vor Installationen Quelländerung separat revertierbar. Auf Zielhost vorherige Agent-/Controller-Version aus geprüftem SHA installieren, TOML/SQLite/Caches erhalten; Remote-Aufträge vorher am Ziel prüfen. Einzelnen Discovery-Drop-in nach Sicherung der gewollten Runtime-Änderungen entfernen, daemon-reload und geplanter Dienstneustart |
| D2 Imagebuilder / Pi-Personalisierung / VPN | Separater root-Worker über Unix-Socket, fest gepinntes offizielles OS-Rezept/Soapy-Commit, ARM64-Build, Firstboot, WLAN/OpenVPN-Heimnetzpolicy | Request-/SSH-/VPNvalidierung, Passwort-Hash, API-Redaktion, Downloadranges, Workerfehler/Controller verfügbar; native Ubuntu-Neu-/Wiederholungsinstall + echtes Image-Chroot in CI; später vollständiger ARM64-Build + Pi/SXceiver | Vorherige VM-Software nach Ende aktiver Jobs aus geprüftem SHA; fertige Images/DB erhalten. Auf Pi vorheriges geprüftes SD-Image oder bewusst gewählte Config/Unit wiederherstellen. VPN-Timer deaktivieren und gewünschte explizite OpenVPN-Betriebsweise setzen; Funkdienst nicht mit VPN-Policy neu starten |
| D3 Ready / Monitoring / CI | Nach D1/D2: deployment/job/marker readiness-only, alter Agentnegativfall, metrics/OpenAPI, breitere Workflow-Trigger | Positiv mit Recovery während Wartezeit; negativ live-only => failed und kein abgenommener Commit; verloren gegangener POST weiterhin genau einmal; API real loopback; Syntax und Katalog-/Unitverträge | Änderungen separat revertierbar, aber Wiedereinführung historischer scheinbarer Erfolge klar vermeiden. Frühere Binary/TOML aus Marker/Backup gezielt erneut deployen und fachlich Readiness prüfen; kein pauschaler atomarer Backend-Rollback behauptet |

## Tatsächlich ausgeführte Prüfungen

1. `python3 -m unittest discover -s system-backend/deployment-core/tests -v`: **50 Tests, 49 bestanden, 1 ausdrücklich übersprungen**. Dauer 18,712 s. Befehle und Ergebnis sind hier dokumentiert; GitHub-CI führt sie am PR-SHA erneut aus. Der übersprungene Test prüft echten Unix-Workertransport inkl. Download; AF_UNIX ist in dieser Ausführungsumgebung verboten und wird im vorgesehenen nativen Ubuntu-CI ausgeführt.
2. `py_compile` alle übernommenen Deployment-Pythondateien, `bash -n` sämtliche Install-/Image-Shellskripte, `node --check` app.js/browser.cjs: bestanden.
3. Katalogprüfung: alle 27 Installer/Configvorlagen vorhanden; alle 26 Backend-ExecStart-Verträge exakt passend: bestanden.
4. Workflow-YAML parsebar; Jobs `discovery`, `image-engine` und `rust-ui` enthalten. Image-Engine Ubuntu24.04/26.04, manuelle Ausführung und Z01-Arbeitszweig/PR/main-Trigger, aktuelle bins/crates/contrib/Cargo/install- und Inventory/E2E-Pfade einbezogen. Ein CI-Lauf wurde durch diese lokale Prüfung noch nicht ausgeführt.
5. **Browserprüfung bestanden** mit offizieller Chrome-for-Testing Headless Shell `155.0.8059.39`, `NETCORE_BROWSER_IMAGE_TRANSPORT=tcp-fixture` und `CHROMIUM_EXECUTABLE_PATH=/workspace/scratch/215875a3693c/test-tools/headless-root/chrome-headless-shell-linux64/chrome-headless-shell`. Tatsächliche Bedienung der WebUI: Discovery, TBS-Assistent/Templateimport, Plan/Remotejob, Imageauftrag/Download/Dateiinhalt/Löschung; Desktop1440×1100 und Mobil390×844 ohne Pageerrors und ohne horizontales Overflow. Screenshots `/tmp/netcore-deployment-desktop.png`, `/tmp/netcore-deployment-mobile.png`; Mobilbild zusätzlich visuell geprüft. Imagebuild und Installer sind in der Browserfixture ausdrücklich simuliert. Der test-only TCP-Transport prüft nicht die Unix-Socket-Rechte; Production/Standard-CI bleiben unverändert Unix. Ein historischer echter False-Positive wurde korrigiert: eine vorzeitig beendete Fixture ohne stdout führte zuvor zu Node exit0; sie führt jetzt zu exit1. Optionale Browserbinary-Auswahl betrifft nur Tests; normale CI verwendet ihren installierten Chromium.
6. Tatsächlicher `install-vm.sh`-Preflight endet erwartbar vor jeder Änderung: `Image-Builds benötigen eine vollständige VM; kein LXC/Container.` Umgebung ist Ubuntu24.04.3 Docker mit PID1 supervisord.
7. Tatsächlicher nativer Image-Smoke-Einstieg `unshare --mount --pid --fork --kill-child --propagation private .../image_smoke.py` endet `unshare failed: Operation not permitted`. ARM64-QEMU und sfdisk fehlen zusätzlich. Kein OS-Download, Mount oder hostweiter Paketinstallationsversuch durchgeführt.

## Verbleibende Grenzen

Die Quelle und gezielte Local-Unit-/HTTP-Tests sind geprüft. Native VM-/Unix-Socket-/ARM64-Image-Smokes im konsolidierten CI, vollständiger ARM64-NetCore-Build, physischer Pi-/SXceiver-Boot, reales Ethernet-/WLAN-/VPN-Umschalten, Quell-SHA/Binaryversion der tatsächlichen Flotte und Anlagenupgrade/Recovery bleiben eigene Nachweise. Kein Controller-/Pi-/LXC-Livezugriff lag vor. Diese Grenzen insbesondere bei Status Z01.2/Z01.3 und bei der späteren Z01.4-Abnahme sichtbar halten.
