# Image-Nachbesserungen: KDE, Bedienung und Dienste

Stand: 12.10.2026. Diese Arbeiten gehören zu **Z01.4**. Ausgangspunkt ist das tatsächlich gestartete Image `53a280bd` am Commit `0770e903`, Profil **SRV-M-TBS-01**. Der laufende Pi wurde für diese Quelländerungen nicht verändert.

## Was belegt ist

| Befund | Korrektur | Nächster Nachweis |
| --- | --- | --- |
| Zweiter Carrier: doppeltes `cell_info` nach Speichern | Semantischer TOML-Editor erkennt auch die vom Image geschriebenen quoted Headers; gemeinsame Sperre und atomare Veröffentlichung | Linux-Server-/BS-Test und Bedienung im neuen Image |
| Telegram: fehlender Token und missverständliche Meldung | Deutscher Einrichtungsstatus, Token-/Chat-Prüfung, bestehendes Geheimnis bleibt erhalten; quote-aware TOML-Schreiben | Eigene gültige Zugangsdaten eintragen; echte Zustellung gesondert testen |
| WLAN-Aktion scheitert / rfkill vermutet | Verbindungsaktion aktiviert und prüft NetworkManager-Funk; Hardwareblock bleibt klarer Fehler; gezielte Polkit-Regel für netcore | Tatsächliche WLAN-Verbindung und Wiederkehr auf dem Pi |
| Recorder/Audioplayer: Permission denied | Kanonische Aufnahme-/Audio-/Cache-Verzeichnisse über systemd provisioniert, PR #70 übernommen | Neues Image: Aufnehmen und Abspielen |
| Zentrale Media Library und Observability reagieren verzögert | Readiness/Status/Metrics aus unabhängigen Snapshots; Inventur bei Maintenance mit Zeitstempel. Observability speichert geordnete Snapshots außerhalb der Zustandssperre | Zentralrollen aktualisieren; Readiness und echte Speicherstörung prüfen |
| Packet Core / IP-Gateway: Lesen endet mit os error 11, HTTP503 | Begrenzter HTTP-Leser statt Warten auf TCP-EOF; Packet-Core-Datenspeicherung außerhalb der Zustandssperre, weiter geordnet und atomar | Zentrale Packet-Core-/Gateway-Verbindung nach Deployment prüfen |
| Systemzustand zeigt Dienste doppelt | Kompakte Dienstindikatoren; vollständige Karten bleiben unter Dienste | Browser-CI und Bedienung im Image |
| Nachbarzellen nur angezeigt | Editor für Anlegen/Bearbeiten/Löschen mit vollständigen Feldern, max. sieben, Validierung und Revisionsschutz | Speichern im neuen Image und Wiederkehr nach kontrolliertem Neustart |

Der HTTP-EOF-Fehler ist reproduziert. Er beweist nicht, dass jeder beobachtete Packet-Core-Timeout dieselbe Ursache hatte. Status darf bei realem Dienstausfall weiterhin 503 melden. Codeprüfung und CI ersetzen keinen Abnahmetest auf den zentralen Rollen.

## KDE Plasma im Image

Das neue Imageformular wählt **KDE Plasma** vor. **Ohne Desktop (Lite)** bleibt auswählbar; alte API-Aufträge ohne `desktop` behalten Lite. Beide Varianten besitzen unterschiedliche Recipekeys. KDE wird bereits im ARM64-Gast installiert: Plasma/X11, SDDM, Konsole, Dolphin und Netzwerk-/Systemeinstellungen. Paketinstallation und vorhandene Plasma-Sitzung werden vor erfolgreichem Build geprüft; `graphical.target` und SDDM werden offline aktiviert.

Die grafische Anmeldung verwendet den im Formular festgelegten persönlichen Pi-Benutzer und dessen Betriebssystem-Passwort. Ein SSH-Public-Key allein ermöglicht keine grafische Anmeldung; das Formular lehnt diese KDE-Kombination vor dem Auftrag ab. Es wird kein gemeinsames Desktop-Passwort und keine automatische Anmeldung eingerichtet. `station.json` und Manifest nennen die Desktopvariante. Ein tatsächlicher KDE-Boot mit Bildschirm und Anmeldung ist eine separate Abnahme.

## SIP neu einrichten

Die vorgesehenen Wege bleiben **native TBS → lokale Asterisk → zentraler SIP-Switch**, mit exklusivem PBX-Ersatzweg. Das Image installiert Asterisk und den vorhandenen Failoverdienst. Im Imageformular wird eine private, zum Stationsprofil passende SIP-Fallback-TOML importiert. Der Helfer prüft sie vor dem Auftrag, setzt feste lokale Pfade und rendert sechs Asterisk-Includes mit richtigen Gruppenrechten. Nur der lokale TBS-/Asterisk-Link bekommt bei Bedarf ein frisch generiertes Geheimnis. Zentrale SIP- und PBX-Zugänge müssen tatsächlich eingerichtet sein.

Ohne Import bleibt eine bisher unbelegte Loopback-SIP-Anmeldung deaktiviert und der Build meldet **Einrichtung erforderlich**. Eine ausdrücklich konfigurierte externe Legacy-SIP-Verbindung bleibt erhalten. Die private Importdatei, OS-/WLAN-Passwörter, Tokens und andere Zugangsdaten gehören weder in Git noch in den Chat; die öffentliche Job-API und das Manifest enthalten keine Importgeheimnisse.

Der zentrale SIP-Switch liegt gemäß aktuellem Inventar auf `10.0.1.125`, der PBX-Ersatzweg auf `10.0.1.21`. Zum Einrichten ist der tatsächliche FreePBX-Zugang beziehungsweise dessen bewusst eingerichtetes IP-Vertrauen erforderlich. Bestehende zentrale Stationszugänge werden wiederverwendet. Der vorbereitete Operatorablauf wird nicht als bereits ausgeführte Einrichtung oder erfolgreicher Ruf beschrieben.

## Wo die Änderungen übernommen werden

1. **Repo:** Linux-/Browser-CI prüfen und den getesteten Stand übernehmen.
2. **Deployment-VM `10.0.1.131`:** den zusammengefassten, gepinnten Rezept-/Builder-UI-Updater bei leerer Auftragswarteschlange ausführen. Einstellungen, Profile, Standorttemplate und vorhandene Downloads bleiben erhalten; kein neues Image startet dadurch.
3. **Zentrale Rollen:** Packet Core, IP-Gateway, Observability und Media Library benötigen ihre aktualisierten Backends. Ein Pi-Image alleine aktualisiert diese Server nicht.
4. **Deployment-WebUI:** Profil **SRV-M-TBS-01**, KDE und persönliches OS-Passwort auswählen; bei eingerichteten SIP-Zugängen die private Importdatei laden. Einen neuen Build starten und genau dessen Manifest/Checksumme prüfen.
5. **Pi:** neues Image auf SD schreiben und booten. KDE-Anmeldung, Carrier-/Nachbar-/Telegram-/WLAN-Bedienung, Aufnahme/Abspielen, zentrale Readiness und echte SIP-Rufe separat abnehmen.

Ein Nachbarzellen-Speichern mit Änderungen kündigt einen TBS-Neustart an; unverändertes Speichern startet nicht neu. Eine veraltete Revision liefert 409 und lässt den Entwurf zur Entscheidung des Bedieners stehen. Diese Implementierung hat den laufenden Funkversuch nicht selbst neu gestartet.

## Prüfungen

Die nativen systemd-Medienverzeichnistests von PR #70 bestanden an `4595be7a` auf Ubuntu24.04 und26.04, einschließlich tatsächlichem Schreiben, Wiederholungsstart und Erhalt fremder privater Daten. Beide ARM64-Personalisierungsjobs, Discovery, Quellgate und TBS-Link waren erfolgreich. [CI](https://github.com/JanHG98/netcore-tetra/actions/runs/38300376697).

Der vollständige Nachbesserungsstand `d3ea8283f3f837296742a2445070364ad7fa4813` ist nach allen sechs erfolgreichen Workflows über PR #71 auf `main@4041ceee34383adf451f4e3689bdb192eaa8417c` übernommen. Linux: 120/120 VM-Guardfälle, 14/14 Desktopfälle, 10/10 SIP-Provisionierung, 9/9 SIP-Rechte; 72/72 Dashboard-Rustfälle und 64/64 zentrale Backendfälle. Browser: 149/149 Dashboardprüfungen sowie vollständiger Deployment-Desktop-/Mobilfall. Beide realen ARM64-Personalisierungsjobs und nativen systemd-Verzeichnistests sind erfolgreich. [Strukturierter CI-/Übernahmebeleg](evidence/image-ui-kde-ci-2026-10-12.json). Der spätere vollständige KDE-ARM64-NetCore-Build sowie Bildschirm-, WLAN-, SIP-, Medien-, Dauerlauf- und On-Air-Abnahmen bleiben eigene Nachweise.


## Gepinnter VM-Updatebefehl

In der SSH-Sitzung als **jhoffmeister auf VM-H-DEPLOY-01 (`10.0.1.131`)** ausführen. Das ist der konsolidierte Rezept-/Formular-Updateweg; nicht auf dem Pi und nicht auf einem zentralen Dienstserver. Bei aktiven oder ungeklärten Jobs verweigert der Helfer den Eingriff.

```bash
NC_FIX_COMMIT=d3ea8283f3f837296742a2445070364ad7fa4813
NC_FIX_SRC=$(mktemp -d /tmp/netcore-image-features.XXXXXX)
git clone --filter=blob:none --no-checkout https://github.com/JanHG98/netcore-tetra.git "$NC_FIX_SRC"
git -C "$NC_FIX_SRC" checkout --detach "$NC_FIX_COMMIT"
test "$(git -C "$NC_FIX_SRC" rev-parse HEAD)" = "$NC_FIX_COMMIT" && \
sudo python3 -B "$NC_FIX_SRC/Docs/integration/Z01-2026-10-07/vm119-image-features-update.py"
```

Erwartetes Ergebnis: `phase: passed` oder bei identischen Dateien `already_current`. Anschließend [Deployment-WebUI](http://10.0.1.131:8320/#images) neu laden und einen neuen Imageauftrag mit **KDE Plasma**, **SRV-M-TBS-01** und persönlichem OS-Passwort anlegen. Der bisherige Download53a280bd bekommt durch das Rezeptupdate keinen Desktop. Das Formular löst für den neuen Auftrag main frisch auf.

## Zentrale Backends gezielt übernehmen

In der Deployment-WebUI unter **Gezielt ausrollen** für jeden registrierten Zielagenten den zugehörigen Dienst **Packet Core**, **IP-Gateway**, **Observability** beziehungsweise **Media Library** wählen, Aktion **Aktualisieren**, Git-Referenz `d3ea8283f3f837296742a2445070364ad7fa4813`. **Plan anzeigen** prüft Ziel/Dienst, dann **Diesen Plan ausführen**. Keine TBS-Aktualisierung am laufenden Funkversuchs-Pi aus diesem zentralen Schritt ableiten. Der Dienst-Update startet den ausgewählten zentralen Dienst neu; danach dessen Readiness und die Verbindungen im TBS-Dashboard prüfen. Ist ein Zielagent nicht registriert oder offline, zuerst genau diese Registrierung klären; keine Ersatz-IP oder Zugangsdaten raten.

## Private SIP-Importdatei vorbereiten

Der [vorbereitete Operatorhelfer](setup-image-sip-01.sh) wird ausschließlich in einer SSH-Sitzung auf dem **zentralen SIP-Switch `10.0.1.125`** mit `bash setup-image-sip-01.sh` ausgeführt. Er ruft sudo selbst auf. Zuerst muss der wirkliche Fallback-Zugang für TBS-01 in **FreePBX `10.0.1.21`** eingerichtet sein; dessen Kennung und Passwort beziehungsweise bewusst eingerichtetes IP-Vertrauen werden lokal am Terminal eingegeben. Diese Vorbereitung wurde hier nicht auf der Anlage ausgeführt.

Der Helfer verwendet einen vorhandenen echten zentralen TBS-01-Zugang oder legt den fehlenden Zugang über den vorhandenen `add-tbs-openlab.sh`-Installer an. Er reserviert die private Ausgabe vor dieser Änderung und verweigert vorhandene Dateien oder Symlinks. Als sudo-Benutzer landet die Datei0600 im tatsächlich zugehörigen Home; bei einer root-Sitzung unter `/root`. Die Ausgabe nennt den konkreten Pfad und Besitzer. Diese Datei als Besitzer per SCP auf den Bedienrechner herunterladen und in **SIP-Fallback-Konfiguration** im Imageformular importieren. Passwort/Dateiinhalt nicht in Git oder den Chat übernehmen.

Die Operatorprüfung umfasst Bash-Syntax und die AST des tatsächlich eingebetteten Python-Programms; sie startet keine Dienste und erzeugt keinen echten SIP-Zugang. Ein erfolgreicher SIP-Register-/Ruftest bleibt nach dem neuen Pi-Boot erforderlich.
