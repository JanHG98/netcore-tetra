# GitHub-CI-Nachtrag: PR #62

Stand: 08.10.2026, Europe/Berlin. PR: https://github.com/JanHG98/netcore-tetra/pull/62. Erster geprüfter PR-Commit: `e5ae2cd5e94c32df0e9b2573e207f9d48b0c1f26`. Die Veröffentlichung wurde durch den Nutzer ausdrücklich freigegeben; der vollständige Git-Baum entspricht dem lokalen Prüfling einschließlich Dateimodi.

## Bereits beobachtete CI-Ergebnisse

| Workflow / Job | Ergebnis am genannten Commit |
| --- | --- |
| [Deployment inventory and source gate](https://github.com/JanHG98/netcore-tetra/actions/runs/37695374102), source-consistency | PASS: gemeinsames Quell-/Inventar-/Ready-/Driftgate und reale Alert-Monitoring-Prüfung |
| [OpenLab deployment and discovery](https://github.com/JanHG98/netcore-tetra/actions/runs/37695374052), discovery | PASS: 50 Tests ohne Skip, einschließlich echtem Unix-Workertransport; tatsächliche Deployment-WebUI |
| OpenLab deployment and discovery, rust-ui | PASS: gesperrter Workspacecheck und Link des TBS-Binaries mit Default-Features |
| [Warning service](https://github.com/JanHG98/netcore-tetra/actions/runs/37695374001) | Beide Jobs PASS einschließlich TBS-SDS und Default-Feature-Binary |
| [Base station dashboard](https://github.com/JanHG98/netcore-tetra/actions/runs/37695374011), rust | PASS; Browserjob beim Erstellen dieses Nachtrags noch laufend |
| Deployment inventory and source gate, syslog-runtime | Erster Lauf FAIL: zwei Wiretests können ihre temporären Konfigurationen nicht lesen; alle 18 Store-/Clienttests bestehen |

Weitere VM-/Image-/Rust-/Browserjobs waren beim Erstellen dieses Nachtrags noch laufend. Ihre abschließenden Ergebnisse sind am jeweiligen PR-Commit und nicht anhand eines früheren lokalen PASS zu bewerten.

## Gefundener und korrigierter Testaufbaufehler

Das Ubuntu-24.04-Paket startet `/usr/sbin/rsyslogd` Version 8.2312.0, verweigert aber bereits bei `-N1 -f /tmp/netcore-.../receiver.conf` bzw. `client.conf` den Zugriff mit Fehler 2104. Das Paketprofil bindet AppArmor an diesen Programmpfad und erlaubt Konfigurationspfade unter `/etc/rsyslog*`, nicht die privaten temporären Testverzeichnisse. Dies passt zur beobachteten Verweigerung; ein Kernel-Auditnachweis des Runners liegt nicht vor.

Die CI stellt deshalb eine identische Kopie des Paketbinaries unter `$RUNNER_TEMP/z01-rsyslogd` bereit und setzt `RSYSLOGD` ausschließlich für die Fixture-Wiretests. RELP und die echten Paketmodule bleiben verwendet; der Systemdaemon und sein Profil werden weder deaktiviert noch umgeschaltet. Die Korrektur ersetzt keinen Test durch einen Mock. Mit einer identischen Kopie des Ubuntu-Binaries bestehen lokal alle 20 Syslogtests, einschließlich der beiden echten TCP-/UDP-/RELP-/Reconnect-Prüfungen. YAML-, Shellsyntax und Whitespaceprüfung bestehen. Der korrigierte Workflow muss zusätzlich auf GitHub erfolgreich laufen.

## Produktionsgrenze

Die Produktionsinstaller enthalten bereits `install/rsyslog-apparmor.sh` und `logging/apparmor-netcore` für Konfiguration, PID-/Queue-/Logpfade und das Python-omprog-Programm. Ein nicht passendes oder nicht ladbares Profil führt dort zum Fehler statt zu einer globalen Abschaltung. Der Fixture-Test bestätigt keine aktive AppArmor-Durchsetzung auf der Anlage: Diese Prüfung gehört mit den tatsächlichen Installationspfaden und dem lokalen Profil-/Namespacezustand weiter zu Z01.4.
