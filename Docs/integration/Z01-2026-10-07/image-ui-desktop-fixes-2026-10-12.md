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

Für die weiteren Änderungen laufen fokussierte tatsächliche Rust-/Datei-/HTTP-Fixtures und die vollständigen Linux-/Browser-Gates. Der spätere vollständige KDE-ARM64-NetCore-Build sowie Bildschirm-, WLAN-, SIP-, Medien-, Dauerlauf- und On-Air-Abnahmen bleiben eigene Nachweise.

