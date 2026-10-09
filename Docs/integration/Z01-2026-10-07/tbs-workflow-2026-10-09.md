# TBS-Assistent: Profile löschen und automatisch aktuelles main bauen

Stand: 09.10.2026, Europe/Berlin. Nutzerauftrag: alte gespeicherte TBS-Profile
löschen können und für neue Pi-Images keine Git-Referenz mehr eingeben müssen.
Geprüfter Ausgangsstand: `main@1595259a2a76abfc9eff08842409156473b585a7`.
Die Umsetzung gehört zu Z01.4; die Gesamtaufgabe bleibt in Arbeit.

## Verhalten

| Aktion | Ergebnis |
| --- | --- |
| TBS-Assistent → Profil löschen → bestätigen | Entfernt genau das gespeicherte Profil; Dropdowns werden aktualisiert. Ein zuvor ausgewähltes, gelöschtes Profil wird abgewählt. |
| Löschung abbrechen | Kein API-Aufruf und keine Änderung. |
| Profil wird noch verwendet | HTTP409 bei wartenden, laufenden oder `remote_uncertain`-Deployment-/Imageaufträgen. Sämtliche Datenbankzeilen werden geprüft, auch hinter den öffentlichen Listenlimits. |
| Workerstatus nicht sicher prüfbar | Löschung bleibt gesperrt; insbesondere ein alter Worker ohne `active_profiles` wird nicht als leer behandelt. |
| Datei geändert, Persistenzbestätigung fehlgeschlagen | HTTP503 mit `durability_uncertain` und `profile_change_applied`; Oberfläche und Controller gleichen den gespeicherten Stand ab und zeigen die Unsicherheit. Kein erneuter Lösch-POST. |
| Image erstellen | Eingaben zuerst validieren, dann Git frisch abrufen und exakt `refs/remotes/origin/main` auf einen vollständigen SHA auflösen. Einen Auftrag mit diesem festen Commit anlegen. |
| Git-Abruf fehlgeschlagen / main fehlt | Fehler anzeigen; kein Imageauftrag und kein Rückgriff auf gespeicherten Deployment-Ref, einen gleichnamigen Tag oder alten Cache. |
| main ändert sich während des Builds | Der laufende Auftrag behält seinen Commit. Erst der nächste Auftrag bekommt den dann frisch aufgelösten main-Stand. |

Die Löschung betrifft Profilmetadaten. Sie deinstalliert keine Basisstation und
entfernt keine fertigen Images oder historischen Aufträge. Das Standorttemplate
und andere Profile bleiben erhalten. Der globale Deployment-Ref bleibt unabhängig
von der automatischen Auswahl bei Imageaufträgen.

Profiländerung, Löschung und Auftragsaufnahme sind im Controller koordiniert.
Ein Deployment erhält beim Absenden eine eigene Profilkopie. Der Imageworker
meldet seine aktiven Profile zusätzlich zu seiner begrenzten öffentlichen Jobliste.
Profilpersistenz und Fehlermeldungen bleiben auch bei Schreibfehlern konsistent.

## Gezieltes Update auf VM119

[vm119-tbs-workflow-update.py](vm119-tbs-workflow-update.py) übernimmt genau sieben
geprüfte Dateien aus demselben Checkout: `main.py`, `deploy.py`, `jobs.py`,
`image_worker.py`, `static/index.html`, `static/app.js` und `static/style.css`.
Controller und Worker müssen zusammen aktualisiert werden, damit der Löschschutz
den vollständigen Workerstatus auswerten kann. Die enthaltene jobs.py umfasst
auch die bereits geprüfte SQLite-Verbindungskorrektur.

Als `jhoffmeister` auf **VM-H-DEPLOY-01** den bereitgestellten vollständigen
Quellcommit auschecken und aus diesem Checkout ausführen:

```bash
sudo python3 -B "$NC_SRC/Docs/integration/Z01-2026-10-07/vm119-tbs-workflow-update.py"
```

Der Helfer prüft root / Hostname / Controlleridentität, feste Quell- und
Laufzeitprüfsummen sowie alle Zeilen beider Auftragsdatenbanken. Bei einem aktiven
oder ungeklärten Auftrag bricht er vor dem Update ab. Er stoppt zuerst den
Controller, prüft den Worker erneut und stoppt ihn ausschließlich im Leerlauf.
Jede Datei wird atomar ersetzt; Eigentümer und Modus bleiben erhalten. Bei einem
fehlgeschlagenen Austausch oder Wiederanlauf werden die ursprünglichen sieben
Dateien wieder eingesetzt, sofern kein neuer aktiver Auftrag dies verhindert.

Deployment-TOML, Settings, Profile und Standorttemplate bleiben bytegleich.
Der Helfer startet keinen Build und führt keine Paket-, Kernel- oder
Unitinstallation aus. Das separate Gastrezept und die DNS-/Initramfs-Hotfixwege
bleiben unverändert; ihre Übernahme auf VM119 ist weiter anhand der
Betreiber-Ausgabe zu bestätigen.

Nach PASS die WebUI neu laden. Unter **TBS-Assistent** erscheint **Profil löschen**;
unter **Vom Profil zur Basisstation.** wird nur das Profil ausgewählt. Ein Gitfeld
ist dort nicht mehr vorhanden. Der gebaute Commit bleibt im Auftrag und Manifest
sichtbar.

## Nachweise und Grenzen

| Prüfung | Ergebnis am finalen lokalen Quellstand |
| --- | --- |
| Deployment-Core | 84 Tests: 83 bestanden, ein erwarteter Unix-Socket-Skip |
| VM119-Operatorhelfer | 35 Testmethoden bestanden; nach finaler UI-Anpassung Quellpin- und Syntaxprüfung erneut bestanden |
| Tatsächlicher Browser | 69 ausgeführte Assertions bestanden; Desktop und Mobil geprüft, TCP-Testtransport |
| Unabhängiger Review | Keine offenen Blocker; alle sieben finalen Quellpins bestätigt |
| main-CI dieser Anpassung | Am Quellcommit16d0d1f beide Workflows vollständig bestanden: [Deployment/Discovery](https://github.com/JanHG98/netcore-tetra/actions/runs/37891522741) und [Quellgate](https://github.com/JanHG98/netcore-tetra/actions/runs/37891522848); [CI-Beleg](evidence/tbs-workflow-ci-2026-10-09.json) |

Lokale API-/Persistenz-/Nebenläufigkeitstests verwenden echte HTTP- und
SQLite-Zugriffe. Gitprüfungen verwenden ein lokales Bare-Remote mit fortgeschrittenem
main, gleichnamigem Tag, fehlendem main und fehlgeschlagenem frischen Abruf.
Die tatsächliche Browserprüfung umfasst Bestätigung / Abbruch / Löschfehler,
Dropdown- und Leerzustände, doppelte Klicks, Commitanzeige sowie Desktop1440 und
Mobil390 ohne Überbreite oder JavaScriptfehler. Der Imageworker läuft dabei über
den ausdrücklich gewählten TCP-Testtransport. In der inzwischen bestandenen
GitHub-CI ist auch der tatsächliche Unix-Worker-/Downloadpfad bestanden. Die
Image-Engine-Jobs auf Ubuntu24.04 und26.04 prüfen Installation und ARM64-
Personalisierung; sie sind keine vollständige ARM64-NetCore- oder Pi-Abnahme.

Die VM-Updateprüfung verwendet echte temporäre Dateien und SQLite-Datenbanken;
Dienststeuerung und HTTP sind isolierte Fixtures. Teilweiser Austausch,
Wiederanlauffehler, versteckte aktive Aufträge, unveränderte Standortdateien,
Dateirechte und vollständiger Rücktausch werden geprüft.

```bash
python3 -B -m unittest discover -s system-backend/deployment-core/tests -v
python3 -B -m unittest discover -s tests/integration -p 'test_vm119_*update.py' -v
node --check system-backend/deployment-core/static/app.js
node system-backend/deployment-core/tests/browser.cjs
```

Diese Prüfungen ersetzen keine Installation des Updates auf VM119. Ein neuer
vollständiger ARM64-Build, Artefakt / SHA-256 / Manifest und der physische
Pi-/SXceiver-/VPN-Test bleiben separate offene Betriebsnachweise.
