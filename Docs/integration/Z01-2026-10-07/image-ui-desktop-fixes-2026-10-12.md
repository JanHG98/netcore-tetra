# Image-Nachbesserungen: KDE, Bedienung und Dienste

**Nachtrag vom 16.10.2026:** [Fertig provisioniertes VPN, korrekter Zeitbroadcast und vollständiger VM-Updateblock](image-vpn-und-zeitbroadcast-2026-10-16.md). Der neue Imagepfad verwendet ein sichtbares NetworkManager-Profil; die Funkzeit enthält den korrekten lokalen Offset einschließlich Sommer-/Winterzeit.

**Neu: [direkt kopierbare Updatebefehle für jeden Dienst](#updatebefehle-für-jeden-dienst)** mit Zielhost, Quellcommit und anschließender Prüfung. Der Dienstindex steht direkt bei den Blöcken.

Stand: **14.10.2026**. Diese Arbeiten gehören zu **Z01.4**. Der aktuelle [VM119-Updateblock](#vm119-auf-den-aktuellen-main-stand-aktualisieren) übernimmt Controller, Builder, Image-Rezept und Formular gemeinsam aus dem beim Start aufgelösten `main`. Historischer Ausgangspunkt war das gestartete Image `53a280bd` am Commit `0770e903`, Profil **SRV-M-TBS-01**; der später praktisch getestete KDE-Stand ist unten gesondert belegt.

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
| Nachbarzellen nur angezeigt | Editor für Anlegen/Bearbeiten/Löschen mit Träger-/Netzparametern, max. sieben, Validierung und Revisionsschutz; bestehende optionale Dienstparameter bleiben erhalten | Speichern im neuen Image und Wiederkehr nach kontrolliertem Neustart |

Der HTTP-EOF-Fehler ist reproduziert. Er beweist nicht, dass jeder beobachtete Packet-Core-Timeout dieselbe Ursache hatte. Status darf bei realem Dienstausfall weiterhin 503 melden. Codeprüfung und CI ersetzen keinen Abnahmetest auf den zentralen Rollen.

## Desktop im Image auswählen

Nach dem VM-Update bietet das Imageformular diese fünf Varianten. **KDE Plasma** bleibt vorausgewählt. Jede Variante hat einen eigenen Recipekey; alte API-Aufträge ohne `desktop` behalten Lite.

| Auswahl | Kennung | Sitzung und Anmeldung |
| --- | --- | --- |
| KDE Plasma | `kde` | Plasma/X11 mit SDDM |
| XFCE | `xfce` | XFCE mit LightDM |
| GNOME | `gnome` | GNOME/Wayland mit GDM3 |
| Raspberry Pi OS Desktop | `pios` | Raspberry Pi Labwc-Sitzung mit LightDM und Pi-Greeter |
| Ohne Desktop (Lite) | `none` | Konsole und SSH |

Das Rezept installiert die gewählte Umgebung im ARM64-Gast und prüft Pakete, Anmeldesitzung, Display-Manager und Netzwerk-/VPN-Editoren. Desktop-Images erhalten außerdem OpenVPN-Core, GTK-Editor/Auth-Dialog und `nm-connection-editor`; KDE erhält seinen Plasma-OpenVPN-Editor. Diese Rezeptprüfungen sind keine bereits ausgeführten Bildschirm-/Anmeldetests der drei neuen Desktopvarianten.

Die grafische Anmeldung verwendet den im Formular festgelegten persönlichen Pi-Benutzer und dessen **Betriebssystem-Passwort**. Ein SSH-Public-Key allein ermöglicht keine grafische Anmeldung; diese Kombination wird für alle Desktopvarianten vor dem Auftrag abgewiesen. Es gibt keine automatische Anmeldung. `station.json` und Manifest nennen die Desktopvariante.

## VPN im Buildformular vorbereiten

Im [Imageformular](http://10.0.1.131:8320/#images) **WLAN und VPN vorbereiten** öffnen und die `.ovpn`-Datei unter **OpenVPN-Profil** auswählen. Danach erscheinen **VPN-Benutzer** und **VPN-Passwort**. Das sind die Zugangsdaten des VPN-Servers; sie sind unabhängig vom Betriebssystem-Passwort des Pi.

Bei `auth-user-pass` müssen beide VPN-Felder ausgefüllt sein. Ein vollständig eingebetteter Login kann übernommen werden; ein vorhandener Auth-Dateiverweis wird nicht vom Bedienrechner gelesen. Das Image schreibt die Zugangsdaten in `/etc/openvpn/client/netcore.auth` mit root-Besitzer und Modus `0600`. Passwort und Profilmaterial erscheinen nicht in der öffentlichen Job-/Manifestausgabe. Zertifikatbasierte Profile ohne Benutzeranmeldung können ohne diese beiden Felder verwendet werden.

Seit dem Fix vom **14.10.2026** akzeptiert der Import auch die in Client-Exporten enthaltene Direktive `lport 0`. Numerische lokale Ports von `0` bis `65535` bleiben im Profil erhalten; `0` lässt das Betriebssystem den lokalen Port wählen. Zertifikatsprüfung mit `verify-x509-name`, Inline-Zertifikate/-Schlüssel und `tls-auth` bleiben erhalten. `lport` und `nobind` dürfen laut [OpenVPN-Handbuch](https://build.openvpn.net/man/openvpn-2.6/openvpn.8.html) nicht gemeinsam verwendet werden. Bei weiteren abgewiesenen Optionen nennt die Fehlermeldung die Zeile der Originaldatei, ohne deren Inhalt auszugeben. Den unten stehenden aktuellen VM119-Updateblock ausführen und das Formular danach mit **Strg+F5** neu laden; für den Import die unveränderte eigene `.ovpn` und beide VPN-Zugangsdaten verwenden.

Mit diesem Profil übernimmt der vorhandene VPN-Timer die Umschaltung: in den eingetragenen lokalen WLANs oder im LAN `10.0.1.0/24` VPN aus, in anderen Netzen VPN an. Die Zugangsdaten gehören in das Formular; sie müssen nicht im Chat oder im Repository hinterlegt werden.

### VPN auf dem bereits laufenden KDE-Pi bearbeiten

Das VM-Update ändert den Builder und zukünftige Images. Für den schon gebooteten Pi `10.0.1.20` lassen sich die fehlenden Editoren direkt ergänzen. **In der SSH-Sitzung als `jan` auf dem Pi**:

```bash
sudo apt-get update
sudo apt-get install -y --no-install-recommends \
  network-manager-openvpn network-manager-openvpn-gnome \
  network-manager-gnome plasma-nm
```

Danach auf dem **KDE-Bildschirm des Pi** die Netzwerkverbindungen neu öffnen oder `nm-connection-editor` starten. Über **Hinzufügen → Gespeicherte VPN-Konfiguration importieren** die `.ovpn`-Datei auswählen. Im OpenVPN-Editor je nach Profil **TLS mit Passwort** beziehungsweise **Passwort** wählen und VPN-Benutzer/Passwort eintragen. Zertifikate und CA aus dem eigenen Profil beibehalten. Wenn Plasma noch den alten Editor zeigt, einmal ab- und wieder anmelden. Für die automatische Umschaltung des neuen Images ist der zuvor beschriebene Import im Buildformular vorgesehen.

Hat dieses Image bereits den automatischen NetCore-OpenVPN-Client für **dasselbe Profil** eingerichtet und soll die Verbindung jetzt manuell über NetworkManager laufen, zuerst den bisherigen Timer und Client auf dem Pi abschalten:

```bash
sudo systemctl disable --now netcore-vpn-policy.timer
sudo systemctl stop netcore-vpn-policy.service openvpn-client@netcore.service
```

Ohne zuvor eingerichtetes Image-VPN ist dieser Schritt nicht nötig. Der Desktopeditor verwaltet seine eigene NetworkManager-Verbindung; die im Buildformular vorbereitete Verbindung wird durch den OpenVPN-Dienst und den NetCore-Timer verwaltet.

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

Ein Nachbarzellen-Speichern mit einer geänderten Liste oder korrigierter Broadcast-Ankündigung kündigt einen TBS-Neustart an. Eine bereits unveränderte Liste mit korrekter Ankündigung startet nicht erneut neu. Eine veraltete Revision liefert 409 und lässt den Entwurf zur Entscheidung des Bedieners stehen. Diese Implementierung hat den laufenden Funkversuch nicht selbst neu gestartet.

## Pi-Funktionstest vom 13.10.2026

Jan hat nach dem Schreiben des KDE-Artefakts `c2982243` (`e1617b28`) den KDE-Start sowie Rufe, SDS, lokale Aufzeichnung, RF-Wiedergabe einer Aufzeichnung, WLAN ein/aus, WLAN verbinden und die Nachbarzellen-Eingabe im UI praktisch bestätigt. [Strukturierter Abnahmebeleg](evidence/pi-kde-acceptance-2026-10-13.json).

Der mitgelieferte Laufzeitausschnitt zeigt TBS und Discovery aktiv. TX-Schreibvorgänge steigen in etwa 40 Sekunden um 26.664; die softwareseitigen Late-Zähler bleiben bei 4 Ereignissen/76 Blöcken. `hw_status_supported=false` liefert dabei keinen Messnachweis für Hardware-Unterläufe. `NRestarts=2` lässt sich ohne früheres Journal nicht zuordnen; ein geändertes Nachbarzellen-Speichern löst absichtlich einen kontrollierten Neustart über Exit 75 aus.

### Nachbarzellen-Ankündigung korrigieren

Im beobachteten Image protokolliert die Software `D-NWRK-BROADCAST ... neighbours=1`, während der Serving-Cell-Fingerprint `neighbour_broadcast=0` meldet. Nach ETSI EN 300 392-2 V3.8.1, 18.3.6.1 und 18.5.16/Tabelle 18.61 muss die Ankündigung zur Broadcast-Unterstützung passen: Broadcast-Bit = 2, Enquiry-Bit = 1. Das belegt eine Ankündigungsinkonsistenz; die Interpretation durch ein MS und tatsächliche Zellwechsel sind noch nicht geprüft.

Die korrigierte Nachbarzellen-Bedienung setzt das Broadcast-Bit, wenn eine Nachbarliste oder eine Zeitzone einen periodischen Broadcast aktiviert. Ein vorhandenes Enquiry-Bit bleibt erhalten und wird nicht automatisch ergänzt. Das Speichern einer unveränderten alten Liste mit falscher Ankündigung repariert den Wert einmal und fordert den vorgesehenen Neustart an. Danach ist identisches Speichern wieder ohne Neustart. Konfigurierter und laufender Ankündigungswert fließen in die Anzeige einer noch ausstehenden Übernahme ein.

Für das aktuell gebootete `e1617b28` mit Ankündigung 0 reicht eine Konfigurationskorrektur. Im vorhandenen Abschnitt `[cell_info]` von `/etc/netcore/config.toml` den folgenden Wert eintragen beziehungsweise ändern:

```toml
neighbor_cell_broadcast = 2
```

Nach beendetem Ruf die Syntax prüfen und die Konfiguration einmal übernehmen:

```bash
sudo python3 -c 'import tomllib; tomllib.load(open("/etc/netcore/config.toml", "rb"))' &&
sudo systemctl restart tetra.service
```

Nach dem Start muss der Serving-Cell-Fingerprint `neighbour_broadcast=2` ausgeben. Diese begrenzte Konfigurationskorrektur benötigt keinen Rust-Build und kein neues SD-Image. Die automatische GUI-Korrektur kommt mit dem aktualisierten TBS-Programm.

Offen bleiben SIP/Fallback, VPN-Umschaltung, ein längerer Dauerlauf, MS-Nachbarzelleninterpretation/Zellwechsel und die tatsächlichen Updatebelege der zentralen LXCs.

## Prüfungen

Die nativen systemd-Medienverzeichnistests von PR #70 bestanden an `4595be7a` auf Ubuntu24.04 und26.04, einschließlich tatsächlichem Schreiben, Wiederholungsstart und Erhalt fremder privater Daten. Beide ARM64-Personalisierungsjobs, Discovery, Quellgate und TBS-Link waren erfolgreich. [CI](https://github.com/JanHG98/netcore-tetra/actions/runs/38300376697).

Der vollständige Nachbesserungsstand `d3ea8283f3f837296742a2445070364ad7fa4813` ist nach allen sechs erfolgreichen Workflows über PR #71 auf `main@4041ceee34383adf451f4e3689bdb192eaa8417c` übernommen. Linux: 120/120 VM-Guardfälle, 14/14 Desktopfälle, 10/10 SIP-Provisionierung, 9/9 SIP-Rechte; 72/72 Dashboard-Rustfälle und 64/64 zentrale Backendfälle. Browser: 149/149 Dashboardprüfungen sowie vollständiger Deployment-Desktop-/Mobilfall. Beide realen ARM64-Personalisierungsjobs und nativen systemd-Verzeichnistests sind erfolgreich. [Strukturierter CI-/Übernahmebeleg](evidence/image-ui-kde-ci-2026-10-12.json). Der spätere vollständige KDE-ARM64-NetCore-Build sowie Bildschirm-, WLAN-, SIP-, Medien-, Dauerlauf- und On-Air-Abnahmen bleiben eigene Nachweise.


## VM119 auf den aktuellen main-Stand aktualisieren

**In der bestehenden SSH-Sitzung als `jhoffmeister` auf VM-H-DEPLOY-01 (`10.0.1.131`)** ausführen. Das ist der gemeinsame aktuelle Updateweg für Controller, Imagebuilder, Desktop-/VPN-Rezept und Formular. Der historische Zehn-Dateien-Updater bleibt als damaliger Beleg erhalten; er installiert die hier beschriebenen neuen Funktionen nicht.

```bash
(
set -euo pipefail
NC_VM_SRC=$(mktemp -d /tmp/netcore-vm-current.XXXXXX)
git -C "$NC_VM_SRC" init -q
git -C "$NC_VM_SRC" fetch --depth 1 \
  https://github.com/JanHG98/netcore-tetra.git refs/heads/main
NC_VM_COMMIT=$(git -C "$NC_VM_SRC" rev-parse FETCH_HEAD)
git -C "$NC_VM_SRC" checkout --detach "$NC_VM_COMMIT"
printf 'VM119-Update aus main: %s\n' "$NC_VM_COMMIT"
sudo python3 -B "$NC_VM_SRC/tools/operator/update-vm119-image-features.py" \
  --commit "$NC_VM_COMMIT" --apply
)
```

Der Helfer liest die vollständigen Paketdateien aus den Git-Objekten dieses einen Commits. Er prüft beide vollständigen Auftragsdatenbanken einschließlich ungeklärter Remote-Aufträge. Controller und Builder werden nur bei leerer Warteschlange gestoppt; der Worker-Lock verhindert einen Austausch neben einem laufenden Build. Paketdateien werden gesichert, ausgetauscht und nach dem Start über Fingerprints, neue Prozesse und Readiness geprüft. Lokale Konfiguration, Profile, Standorttemplate, effektive VM-Units/Drop-ins sowie Images und Caches bleiben erhalten. Ein Fehler löst den Rückweg auf die vorherigen Paketdateien aus; Rückwegfehler werden ausdrücklich gemeldet.

Erwartet wird **`phase: passed`** oder bei identischen Paketdateien **`already_current`**, jeweils mit dem aufgelösten Quellcommit. Anschließend die [Deployment-WebUI](http://10.0.1.131:8320/#images) mit **Strg+F5** neu laden. **SRV-M-TBS-01**, Desktop und persönliches Pi-Passwort auswählen; bei Bedarf `.ovpn` sowie VPN-Benutzer/Passwort ergänzen und einen **neuen** Imageauftrag starten. Das Rezeptupdate startet keinen Build und verändert keinen bisherigen Download.

## Zentrale Backends gezielt übernehmen

**Update-Nachweis korrigiert, 13.10.2026:** Der gemeinsame Proxmox-Lauf verwendet den beim Start aufgelösten neuesten `main`-Commit, bereitet alle ausgewählten Rollen vollständig vor und prüft anschließend installierte Dateihashes und neue Prozesse. Erst dann werden auch Discoverys Versionsmarker aktualisiert. Inventar-/Planzeilen sind keine Abschlussmeldung. Die [vollständige Updateanleitung](../../deployment/netcore-dienste-aktualisieren.md) beschreibt diesen Ablauf und die Ergebnisse je CT/Dienst.

**Gemeinsamer Proxmox-Host-Block:** [NetCore-Dienste aktualisieren](../../deployment/netcore-dienste-aktualisieren.md). Die Runtime-Blöcke unten verwenden denselben geprüften Helfer jeweils mit bereits eingetragenem Dienstnamen; jeder Dienst hat einen eigenen vollständigen Block.

Die [einzelnen Bash-Updateblöcke](#updatebefehle-für-jeden-dienst) stehen jetzt für **alle Dienste direkt in dieser Anleitung**, einschließlich Packet Core, IP-Gateway, Observability und Media Library. Jeden Block auf dem dort genannten Diensthost ausführen. Die Blöcke enthalten Commit-Checkout, Update-Einstieg und abschließende Prüfung; ein Pi-Image aktualisiert diese zentralen Dienste nicht.

Der vorhandene GUI-Weg bleibt unter **Gezielt ausrollen → Zielhost/Dienst → Aktualisieren → Plan anzeigen → Diesen Plan ausführen** verfügbar. Er verwendet für viele Nicht-TBS-Rollen einen anderen Installer-Einstieg als die manuellen Blöcke; die beiden Wege nicht als identischen Shellaufruf behandeln.

## Private SIP-Importdatei vorbereiten

Der [vorbereitete Operatorhelfer](setup-image-sip-01.sh) wird ausschließlich in einer SSH-Sitzung auf dem **zentralen SIP-Switch `10.0.1.125`** mit `bash setup-image-sip-01.sh` ausgeführt. Er ruft sudo selbst auf. Zuerst muss der wirkliche Fallback-Zugang für TBS-01 in **FreePBX `10.0.1.21`** eingerichtet sein; dessen Kennung und Passwort beziehungsweise bewusst eingerichtetes IP-Vertrauen werden lokal am Terminal eingegeben. Diese Vorbereitung wurde hier nicht auf der Anlage ausgeführt.

Der Helfer verwendet einen vorhandenen echten zentralen TBS-01-Zugang oder legt den fehlenden Zugang über den vorhandenen `add-tbs-openlab.sh`-Installer an. Er reserviert die private Ausgabe vor dieser Änderung und verweigert vorhandene Dateien oder Symlinks. Als sudo-Benutzer landet die Datei0600 im tatsächlich zugehörigen Home; bei einer root-Sitzung unter `/root`. Die Ausgabe nennt den konkreten Pfad und Besitzer. Diese Datei als Besitzer per SCP auf den Bedienrechner herunterladen und in **SIP-Fallback-Konfiguration** im Imageformular importieren. Passwort/Dateiinhalt nicht in Git oder den Chat übernehmen.

Die Operatorprüfung umfasst Bash-Syntax und die AST des tatsächlich eingebetteten Python-Programms; sie startet keine Dienste und erzeugt keinen echten SIP-Zugang. Ein erfolgreicher SIP-Register-/Ruftest bleibt nach dem neuen Pi-Boot erforderlich.

## Updatebefehle für jeden Dienst

**Ergänzt am 13.10.2026.** Unten steht für jede der **26 regulären Backend-Rollen** ein eigener ausführbarer Block. Dazu kommen Imagebuilder, Discovery, Syslog, optionaler Provisioning Core, Directory, dein bestehender Python-Brew und TBS: **33 zugeordnete Dienst-/Komponentenblöcke**.

**Wo:** Die regulären Dienstblöcke laufen direkt in der **Linux-SSH-Sitzung auf dem jeweiligen Diensthost als root**, beispielsweise nach `ssh root@10.0.1.166` für Packet Core. Die VM-Blöcke laufen als **jhoffmeister auf 10.0.1.131**, die Pi-Blöcke nach `ssh jan@10.0.1.20` und `sudo -i`. Bash-Blöcke nicht in die lokale Windows-PowerShell einfügen.

**Stand:** Die Runtime-Blöcke für zentrale Dienste, optionalen Provisioning Core, Directory und Discovery sowie der aktuelle VM119- und TBS-Block lösen `main` beim Start einmal auf und zeigen den vollständigen Quellcommit. Die unter Deployment Core/Imagebuilder eingeklappten festen Pins sind historische Beispiele; für neue Desktop-/VPN-Funktionen gilt der aktuelle VM119-Block oben. Syslog und Python-Brew nennen ihren gesonderten Quellstand im jeweiligen Block. Die IPs der zentralen Rollen stammen aus dem [dokumentierten Rollout-Inventar vom 06.10.2026](../../archive/2026-10-06_tbs-container-updates-katwarn-reparaturen-release-und-brancharchiv.md), VM/Pi aus den späteren Z01.4-Befunden. Dies ist keine neue Erreichbarkeitsmessung.

**Runtime-Update:** Der Helfer prüft geladene Unit, tatsächliche ExecStart-/Konfigurationspfade und vorhandene Dateien. Rust-Build mit `--locked --release` beziehungsweise Python-Syntaxprüfung erfolgen vor dem ersten Dienststopp. Konfigurationen, Zugangsdaten, Datenbanken, Units und Drop-ins bleiben erhalten. Bei Austausch-/Start-/Livenessfehlern versucht er den Rückweg auf die gesicherten Programmdateien. Das ist keine Neuinstallation oder allgemeine Schema-/Unitmigration. Der GUI-Weg unter **Gezielt ausrollen** verwendet andere Installer-Einstiege.

**Ergebnis:** Jeder Runtime-Block gibt ein JSON-Ergebnis aus. `updated` / `already_current` meldet den Dateistand; `updated_degraded` oder `*_degraded` endet mit Code2: Die Runtime bleibt übernommen; Readiness und Abhängigkeiten sind anschließend zu prüfen. Code1 meldet einen Abbruch. Bereits inaktive Dienste bleiben inaktiv und haben keine Laufzeitprüfung; ein zuvor fehlgeschlagener Dienst bekommt einen Recovery-Start. Ein laufender Dienst wird nach dem Austausch auf Liveness geprüft, seine Readiness getrennt bewertet. Die Enabled-/Disabled-Einstellung bleibt erhalten. Ein Quell-/Buildfehler endet vor dem Dienststopp. Details, Rückweggrenzen und der gemeinsame Flottenlauf stehen in [NetCore-Dienste aktualisieren](../../deployment/netcore-dienste-aktualisieren.md).

**Voraussetzungen:** Bestehende Installation, Git und Python3.11 oder neuer; für Rust-Rollen außerdem die vorhandene Root-Cargo-Toolchain und Build-Abhängigkeiten. Ein Neustart unterbricht laufende Dienstverbindungen. Die einzelnen Blöcke sind unabhängig kopierbar. Die gesonderten VM-/Syslog-/Brew-/TBS-Blöcke beschreiben ihren jeweiligen Umfang und Rückweg.

| Dienst | Auf welchem Host ausführen | Unit / Umfang |
| --- | --- | --- |
| [Node Gateway](#node-gateway) | `10.0.1.179` · root | `netcore-node-gateway.service` |
| [Mobility Core](#mobility-core) | `10.0.1.150` · root | `netcore-mobility-core.service` |
| [Subscriber Core](#subscriber-core) | `10.0.1.153` · root | `netcore-subscriber-core.service` |
| [Group Core](#group-core) | `10.0.1.157` · root | `netcore-group-core.service` |
| [Call Control](#call-control) | `10.0.1.155` · root | `netcore-call-control.service` |
| [Media Switch](#media-switch) | `10.0.1.159` · root | `netcore-media-switch.service` |
| [Recorder](#recorder) | `10.0.1.170` · root | `netcore-recorder.service` |
| [SDS Router](#sds-router) | `10.0.1.169` · root | `netcore-sds-router.service` |
| [Packet Core](#packet-core) | `10.0.1.166` · root | `netcore-packet-core.service` |
| [IP Gateway](#ip-gateway) | `10.0.1.142` · root | `netcore-ip-gateway.service` |
| [Security Core](#security-core) | `10.0.1.149` · root | `netcore-security-core.service` |
| [KMF](#kmf) | `10.0.1.180` · root | `netcore-kmf.service` |
| [Transit](#transit) | `10.0.1.151` · root | `netcore-transit.service` |
| [Application Gateway](#application-gateway) | `10.0.1.144` · root | `netcore-application-gateway.service` |
| [Media Library](#media-library) | `10.0.1.154` · root | `netcore-media-library.service` |
| [Control Room](#control-room) | `10.0.1.156` · root | `netcore-control-room.service` |
| [Observability](#observability) | `10.0.1.143` · root | `netcore-observability.service` |
| [IoT Gateway](#iot-gateway) | `10.0.1.119` · root | `netcore-iot-gateway.service` |
| [Hardware Gateway](#hardware-gateway) | `10.0.1.123` · root | `netcore-hardware-gateway.service` |
| [RF Monitor](#rf-monitor) | `10.0.1.122` · root | `netcore-rf-monitor.service` |
| [Alarm Workflow](#alarm-workflow) | `10.0.1.121` · root | `netcore-alarm-workflow.service` |
| [Task Workflow](#task-workflow) | `10.0.1.120` · root | `netcore-task-workflow.service` |
| [Asset Management](#asset-management) | `10.0.1.124` · root | `netcore-asset-management.service` |
| [Sip Switch](#sip-switch) | `10.0.1.125` · root | `netcore-sip-switch.service` |
| [Alert Service](#alert-service) | `10.0.1.129` · root | `netcore-alert-service.service` |
| [Deployment Core](#deployment-core) | VM-H-DEPLOY-01 · `10.0.1.131` · jhoffmeister | `netcore-deployment.service` / gezielte VM-Transaktion |
| [Imagebuilder](#imagebuilder) | dieselbe VM · jhoffmeister | `netcore-image-builder.service` / dieselbe Transaktion |
| [Discovery-Agent](#discovery-agent) | jeweiliger vorhandener Agentenhost · root | `netcore-discovery.service` |
| [Syslog](#syslog) | bestehender Collector, dokumentiert CT136 / `10.0.1.143` · root | Receiver / Preview / Archivtimer |
| [Provisioning Core](#provisioning-core) | `10.0.1.106` · root · nur falls vorhanden | `netcore-provisioning-core.service` |
| [Directory](#directory) | `10.0.1.23` · root | vorhandene `netcore-directory.service` |
| [Python-Brew](#python-brew) | `10.0.1.22` · root | vorhandene `netcore-brew.service` |
| [TBS](#tbs) | Pi `10.0.1.20` · jan → sudo -i | tatsächlich erkannte Radio-Unit |


Die 26 regulären Rollen entsprechen [`services.toml`](../../../system-backend/services.toml) ohne `shared` (`runtime=false`). Provisioning Core und TBS gehören zusätzlich zum [Agentenkatalog](../../../system-backend/deployment-core/catalog.json); Directory/Python-Brew sind gesondert dokumentierte Bestandsdienste. FreePBX und Home Assistant sind keine Updateziele dieses NetCore-Repositorystands.

### Node Gateway

**Auf `10.0.1.179` als root**: `ssh root@10.0.1.179`. Der Runtime-Updater prüft die vorhandene Installation und baut beziehungsweise kompiliert vor dem Dienststopp.

```bash
(
  set -Eeuo pipefail
  [[ $EUID -eq 0 ]] || { echo 'Auf dem Diensthost als root ausführen.' >&2; exit 1; }
  python3 -c 'import tomllib'
  NC_SERVICE=node-gateway
  NC_REF=main
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/node-gateway-update.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" fetch --depth 1 \
    https://github.com/JanHG98/netcore-tetra.git "$NC_REF"
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  NC_REF=$(git -C "$NC_SRC" rev-parse HEAD)
  printf 'Ausgerollter main-Commit: %s\n' "$NC_REF"
  chmod 0755 "$NC_SRC"
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service "$NC_SERVICE" --commit "$NC_REF" --prepare-build-deps --apply
)
```

### Mobility Core

**Auf `10.0.1.150` als root**: `ssh root@10.0.1.150`. Der Runtime-Updater prüft die vorhandene Installation und baut beziehungsweise kompiliert vor dem Dienststopp.

```bash
(
  set -Eeuo pipefail
  [[ $EUID -eq 0 ]] || { echo 'Auf dem Diensthost als root ausführen.' >&2; exit 1; }
  python3 -c 'import tomllib'
  NC_SERVICE=mobility-core
  NC_REF=main
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/mobility-core-update.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" fetch --depth 1 \
    https://github.com/JanHG98/netcore-tetra.git "$NC_REF"
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  NC_REF=$(git -C "$NC_SRC" rev-parse HEAD)
  printf 'Ausgerollter main-Commit: %s\n' "$NC_REF"
  chmod 0755 "$NC_SRC"
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service "$NC_SERVICE" --commit "$NC_REF" --prepare-build-deps --apply
)
```

### Subscriber Core

**Auf `10.0.1.153` als root**: `ssh root@10.0.1.153`. Der Runtime-Updater prüft die vorhandene Installation und baut beziehungsweise kompiliert vor dem Dienststopp.

```bash
(
  set -Eeuo pipefail
  [[ $EUID -eq 0 ]] || { echo 'Auf dem Diensthost als root ausführen.' >&2; exit 1; }
  python3 -c 'import tomllib'
  NC_SERVICE=subscriber-core
  NC_REF=main
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/subscriber-core-update.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" fetch --depth 1 \
    https://github.com/JanHG98/netcore-tetra.git "$NC_REF"
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  NC_REF=$(git -C "$NC_SRC" rev-parse HEAD)
  printf 'Ausgerollter main-Commit: %s\n' "$NC_REF"
  chmod 0755 "$NC_SRC"
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service "$NC_SERVICE" --commit "$NC_REF" --prepare-build-deps --apply
)
```

### Group Core

**Auf `10.0.1.157` als root**: `ssh root@10.0.1.157`. Der Runtime-Updater prüft die vorhandene Installation und baut beziehungsweise kompiliert vor dem Dienststopp.

```bash
(
  set -Eeuo pipefail
  [[ $EUID -eq 0 ]] || { echo 'Auf dem Diensthost als root ausführen.' >&2; exit 1; }
  python3 -c 'import tomllib'
  NC_SERVICE=group-core
  NC_REF=main
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/group-core-update.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" fetch --depth 1 \
    https://github.com/JanHG98/netcore-tetra.git "$NC_REF"
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  NC_REF=$(git -C "$NC_SRC" rev-parse HEAD)
  printf 'Ausgerollter main-Commit: %s\n' "$NC_REF"
  chmod 0755 "$NC_SRC"
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service "$NC_SERVICE" --commit "$NC_REF" --prepare-build-deps --apply
)
```

### Call Control

**Auf `10.0.1.155` als root**: `ssh root@10.0.1.155`. Der Runtime-Updater prüft die vorhandene Installation und baut beziehungsweise kompiliert vor dem Dienststopp.

```bash
(
  set -Eeuo pipefail
  [[ $EUID -eq 0 ]] || { echo 'Auf dem Diensthost als root ausführen.' >&2; exit 1; }
  python3 -c 'import tomllib'
  NC_SERVICE=call-control
  NC_REF=main
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/call-control-update.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" fetch --depth 1 \
    https://github.com/JanHG98/netcore-tetra.git "$NC_REF"
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  NC_REF=$(git -C "$NC_SRC" rev-parse HEAD)
  printf 'Ausgerollter main-Commit: %s\n' "$NC_REF"
  chmod 0755 "$NC_SRC"
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service "$NC_SERVICE" --commit "$NC_REF" --prepare-build-deps --apply
)
```

### Media Switch

**Auf `10.0.1.159` als root**: `ssh root@10.0.1.159`. Der Runtime-Updater prüft die vorhandene Installation und baut beziehungsweise kompiliert vor dem Dienststopp.

```bash
(
  set -Eeuo pipefail
  [[ $EUID -eq 0 ]] || { echo 'Auf dem Diensthost als root ausführen.' >&2; exit 1; }
  python3 -c 'import tomllib'
  NC_SERVICE=media-switch
  NC_REF=main
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/media-switch-update.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" fetch --depth 1 \
    https://github.com/JanHG98/netcore-tetra.git "$NC_REF"
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  NC_REF=$(git -C "$NC_SRC" rev-parse HEAD)
  printf 'Ausgerollter main-Commit: %s\n' "$NC_REF"
  chmod 0755 "$NC_SRC"
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service "$NC_SERVICE" --commit "$NC_REF" --prepare-build-deps --apply
)
```

### Recorder

**Auf `10.0.1.170` als root**: `ssh root@10.0.1.170`. Der Runtime-Updater prüft die vorhandene Installation und baut beziehungsweise kompiliert vor dem Dienststopp.

```bash
(
  set -Eeuo pipefail
  [[ $EUID -eq 0 ]] || { echo 'Auf dem Diensthost als root ausführen.' >&2; exit 1; }
  python3 -c 'import tomllib'
  NC_SERVICE=recorder
  NC_REF=main
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/recorder-update.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" fetch --depth 1 \
    https://github.com/JanHG98/netcore-tetra.git "$NC_REF"
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  NC_REF=$(git -C "$NC_SRC" rev-parse HEAD)
  printf 'Ausgerollter main-Commit: %s\n' "$NC_REF"
  chmod 0755 "$NC_SRC"
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service "$NC_SERVICE" --commit "$NC_REF" --prepare-build-deps --apply
)
```

### SDS Router

**Auf `10.0.1.169` als root**: `ssh root@10.0.1.169`. Der Runtime-Updater prüft die vorhandene Installation und baut beziehungsweise kompiliert vor dem Dienststopp.

```bash
(
  set -Eeuo pipefail
  [[ $EUID -eq 0 ]] || { echo 'Auf dem Diensthost als root ausführen.' >&2; exit 1; }
  python3 -c 'import tomllib'
  NC_SERVICE=sds-router
  NC_REF=main
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/sds-router-update.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" fetch --depth 1 \
    https://github.com/JanHG98/netcore-tetra.git "$NC_REF"
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  NC_REF=$(git -C "$NC_SRC" rev-parse HEAD)
  printf 'Ausgerollter main-Commit: %s\n' "$NC_REF"
  chmod 0755 "$NC_SRC"
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service "$NC_SERVICE" --commit "$NC_REF" --prepare-build-deps --apply
)
```

### Packet Core

**Auf `10.0.1.166` als root**: `ssh root@10.0.1.166`. Der Runtime-Updater prüft die vorhandene Installation und baut beziehungsweise kompiliert vor dem Dienststopp.

```bash
(
  set -Eeuo pipefail
  [[ $EUID -eq 0 ]] || { echo 'Auf dem Diensthost als root ausführen.' >&2; exit 1; }
  python3 -c 'import tomllib'
  NC_SERVICE=packet-core
  NC_REF=main
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/packet-core-update.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" fetch --depth 1 \
    https://github.com/JanHG98/netcore-tetra.git "$NC_REF"
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  NC_REF=$(git -C "$NC_SRC" rev-parse HEAD)
  printf 'Ausgerollter main-Commit: %s\n' "$NC_REF"
  chmod 0755 "$NC_SRC"
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service "$NC_SERVICE" --commit "$NC_REF" --prepare-build-deps --apply
)
```

### IP Gateway

**Auf `10.0.1.142` als root**: `ssh root@10.0.1.142`. Die vorhandene TUN-/nftables-/LXC-Konfiguration muss weiterhin zum installierten Gateway passen.

```bash
(
  set -Eeuo pipefail
  [[ $EUID -eq 0 ]] || { echo 'Auf dem Diensthost als root ausführen.' >&2; exit 1; }
  python3 -c 'import tomllib'
  NC_SERVICE=ip-gateway
  NC_REF=main
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/ip-gateway-update.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" fetch --depth 1 \
    https://github.com/JanHG98/netcore-tetra.git "$NC_REF"
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  NC_REF=$(git -C "$NC_SRC" rev-parse HEAD)
  printf 'Ausgerollter main-Commit: %s\n' "$NC_REF"
  chmod 0755 "$NC_SRC"
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service "$NC_SERVICE" --commit "$NC_REF" --prepare-build-deps --apply
)
```

### Security Core

**Auf `10.0.1.149` als root**: `ssh root@10.0.1.149`. Der Runtime-Updater prüft die vorhandene Installation und baut beziehungsweise kompiliert vor dem Dienststopp.

```bash
(
  set -Eeuo pipefail
  [[ $EUID -eq 0 ]] || { echo 'Auf dem Diensthost als root ausführen.' >&2; exit 1; }
  python3 -c 'import tomllib'
  NC_SERVICE=security-core
  NC_REF=main
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/security-core-update.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" fetch --depth 1 \
    https://github.com/JanHG98/netcore-tetra.git "$NC_REF"
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  NC_REF=$(git -C "$NC_SRC" rev-parse HEAD)
  printf 'Ausgerollter main-Commit: %s\n' "$NC_REF"
  chmod 0755 "$NC_SRC"
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service "$NC_SERVICE" --commit "$NC_REF" --prepare-build-deps --apply
)
```

### KMF

**Auf `10.0.1.180` als root**: `ssh root@10.0.1.180`. Der Runtime-Updater prüft die vorhandene Installation und baut beziehungsweise kompiliert vor dem Dienststopp.

```bash
(
  set -Eeuo pipefail
  [[ $EUID -eq 0 ]] || { echo 'Auf dem Diensthost als root ausführen.' >&2; exit 1; }
  python3 -c 'import tomllib'
  NC_SERVICE=kmf
  NC_REF=main
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/kmf-update.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" fetch --depth 1 \
    https://github.com/JanHG98/netcore-tetra.git "$NC_REF"
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  NC_REF=$(git -C "$NC_SRC" rev-parse HEAD)
  printf 'Ausgerollter main-Commit: %s\n' "$NC_REF"
  chmod 0755 "$NC_SRC"
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service "$NC_SERVICE" --commit "$NC_REF" --prepare-build-deps --apply
)
```

### Transit

**Auf `10.0.1.151` als root**: `ssh root@10.0.1.151`. Der Runtime-Updater prüft die vorhandene Installation und baut beziehungsweise kompiliert vor dem Dienststopp.

```bash
(
  set -Eeuo pipefail
  [[ $EUID -eq 0 ]] || { echo 'Auf dem Diensthost als root ausführen.' >&2; exit 1; }
  python3 -c 'import tomllib'
  NC_SERVICE=transit
  NC_REF=main
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/transit-update.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" fetch --depth 1 \
    https://github.com/JanHG98/netcore-tetra.git "$NC_REF"
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  NC_REF=$(git -C "$NC_SRC" rev-parse HEAD)
  printf 'Ausgerollter main-Commit: %s\n' "$NC_REF"
  chmod 0755 "$NC_SRC"
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service "$NC_SERVICE" --commit "$NC_REF" --prepare-build-deps --apply
)
```

### Application Gateway

**Auf `10.0.1.144` als root**: `ssh root@10.0.1.144`. Der Runtime-Updater prüft die vorhandene Installation und baut beziehungsweise kompiliert vor dem Dienststopp.

```bash
(
  set -Eeuo pipefail
  [[ $EUID -eq 0 ]] || { echo 'Auf dem Diensthost als root ausführen.' >&2; exit 1; }
  python3 -c 'import tomllib'
  NC_SERVICE=application-gateway
  NC_REF=main
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/application-gateway-update.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" fetch --depth 1 \
    https://github.com/JanHG98/netcore-tetra.git "$NC_REF"
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  NC_REF=$(git -C "$NC_SRC" rev-parse HEAD)
  printf 'Ausgerollter main-Commit: %s\n' "$NC_REF"
  chmod 0755 "$NC_SRC"
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service "$NC_SERVICE" --commit "$NC_REF" --prepare-build-deps --apply
)
```

### Media Library

**Auf `10.0.1.154` als root**: `ssh root@10.0.1.154`. Die vorhandene Playout-/Piper-Konfiguration, Modelle, Aufnahmen und Archivdaten bleiben erhalten. Der Block ergänzt keine fehlende `[playout]`-Konfiguration und führt keine Archivmigration aus. Eine fehlende TBS-Zuordnung ist anschließend separat in der bestehenden Konfiguration einzurichten.

```bash
(
  set -Eeuo pipefail
  [[ $EUID -eq 0 ]] || { echo 'Auf dem Diensthost als root ausführen.' >&2; exit 1; }
  python3 -c 'import tomllib'
  NC_SERVICE=media-library
  NC_REF=main
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/media-library-update.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" fetch --depth 1 \
    https://github.com/JanHG98/netcore-tetra.git "$NC_REF"
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  NC_REF=$(git -C "$NC_SRC" rev-parse HEAD)
  printf 'Ausgerollter main-Commit: %s\n' "$NC_REF"
  chmod 0755 "$NC_SRC"
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service "$NC_SERVICE" --commit "$NC_REF" --prepare-build-deps --apply
)
```

### Control Room

**Auf `10.0.1.156` als root**: `ssh root@10.0.1.156`. Der tatsächliche Konfigurationspfad ist `/etc/netcore-control-room/control-room.toml`.

```bash
(
  set -Eeuo pipefail
  [[ $EUID -eq 0 ]] || { echo 'Auf dem Diensthost als root ausführen.' >&2; exit 1; }
  python3 -c 'import tomllib'
  NC_SERVICE=control-room
  NC_REF=main
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/control-room-update.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" fetch --depth 1 \
    https://github.com/JanHG98/netcore-tetra.git "$NC_REF"
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  NC_REF=$(git -C "$NC_SRC" rev-parse HEAD)
  printf 'Ausgerollter main-Commit: %s\n' "$NC_REF"
  chmod 0755 "$NC_SRC"
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service "$NC_SERVICE" --commit "$NC_REF" --prepare-build-deps --apply
)
```

### Observability

**Auf `10.0.1.143` als root**: `ssh root@10.0.1.143`. Das Rust-Backend und der bereits installierte Python-Loghelfer werden aktualisiert. Bei geändertem Loghelfer koordiniert der Helfer die aktiven Vorschau-/Archivunits. Receiver-Konfiguration, Inventar und NAS-Pfade bleiben erhalten. Für das eigenständige Syslog-Paket gibt es den [Syslog-Block](#syslog).

```bash
(
  set -Eeuo pipefail
  [[ $EUID -eq 0 ]] || { echo 'Auf dem Diensthost als root ausführen.' >&2; exit 1; }
  python3 -c 'import tomllib'
  NC_SERVICE=observability
  NC_REF=main
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/observability-update.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" fetch --depth 1 \
    https://github.com/JanHG98/netcore-tetra.git "$NC_REF"
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  NC_REF=$(git -C "$NC_SRC" rev-parse HEAD)
  printf 'Ausgerollter main-Commit: %s\n' "$NC_REF"
  chmod 0755 "$NC_SRC"
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service "$NC_SERVICE" --commit "$NC_REF" --prepare-build-deps --apply
)
```

### IoT Gateway

**Auf `10.0.1.119` als root**: `ssh root@10.0.1.119`. Die vorhandene MQTT-Konfiguration bleibt erhalten; der Block installiert keinen zusätzlichen Broker.

```bash
(
  set -Eeuo pipefail
  [[ $EUID -eq 0 ]] || { echo 'Auf dem Diensthost als root ausführen.' >&2; exit 1; }
  python3 -c 'import tomllib'
  NC_SERVICE=iot-gateway
  NC_REF=main
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/iot-gateway-update.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" fetch --depth 1 \
    https://github.com/JanHG98/netcore-tetra.git "$NC_REF"
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  NC_REF=$(git -C "$NC_SRC" rev-parse HEAD)
  printf 'Ausgerollter main-Commit: %s\n' "$NC_REF"
  chmod 0755 "$NC_SRC"
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service "$NC_SERVICE" --commit "$NC_REF" --prepare-build-deps --apply
)
```

### Hardware Gateway

**Auf `10.0.1.123` als root**: `ssh root@10.0.1.123`. Der Runtime-Updater prüft die vorhandene Installation und baut beziehungsweise kompiliert vor dem Dienststopp.

```bash
(
  set -Eeuo pipefail
  [[ $EUID -eq 0 ]] || { echo 'Auf dem Diensthost als root ausführen.' >&2; exit 1; }
  python3 -c 'import tomllib'
  NC_SERVICE=hardware-gateway
  NC_REF=main
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/hardware-gateway-update.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" fetch --depth 1 \
    https://github.com/JanHG98/netcore-tetra.git "$NC_REF"
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  NC_REF=$(git -C "$NC_SRC" rev-parse HEAD)
  printf 'Ausgerollter main-Commit: %s\n' "$NC_REF"
  chmod 0755 "$NC_SRC"
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service "$NC_SERVICE" --commit "$NC_REF" --prepare-build-deps --apply
)
```

### RF Monitor

**Auf `10.0.1.122` als root**: `ssh root@10.0.1.122`. Der Runtime-Updater prüft die vorhandene Installation und baut beziehungsweise kompiliert vor dem Dienststopp.

```bash
(
  set -Eeuo pipefail
  [[ $EUID -eq 0 ]] || { echo 'Auf dem Diensthost als root ausführen.' >&2; exit 1; }
  python3 -c 'import tomllib'
  NC_SERVICE=rf-monitor
  NC_REF=main
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/rf-monitor-update.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" fetch --depth 1 \
    https://github.com/JanHG98/netcore-tetra.git "$NC_REF"
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  NC_REF=$(git -C "$NC_SRC" rev-parse HEAD)
  printf 'Ausgerollter main-Commit: %s\n' "$NC_REF"
  chmod 0755 "$NC_SRC"
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service "$NC_SERVICE" --commit "$NC_REF" --prepare-build-deps --apply
)
```

### Alarm Workflow

**Auf `10.0.1.121` als root**: `ssh root@10.0.1.121`. Der Runtime-Updater prüft die vorhandene Installation und baut beziehungsweise kompiliert vor dem Dienststopp.

```bash
(
  set -Eeuo pipefail
  [[ $EUID -eq 0 ]] || { echo 'Auf dem Diensthost als root ausführen.' >&2; exit 1; }
  python3 -c 'import tomllib'
  NC_SERVICE=alarm-workflow
  NC_REF=main
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/alarm-workflow-update.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" fetch --depth 1 \
    https://github.com/JanHG98/netcore-tetra.git "$NC_REF"
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  NC_REF=$(git -C "$NC_SRC" rev-parse HEAD)
  printf 'Ausgerollter main-Commit: %s\n' "$NC_REF"
  chmod 0755 "$NC_SRC"
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service "$NC_SERVICE" --commit "$NC_REF" --prepare-build-deps --apply
)
```

### Task Workflow

**Auf `10.0.1.120` als root**: `ssh root@10.0.1.120`. Der Runtime-Updater prüft die vorhandene Installation und baut beziehungsweise kompiliert vor dem Dienststopp.

```bash
(
  set -Eeuo pipefail
  [[ $EUID -eq 0 ]] || { echo 'Auf dem Diensthost als root ausführen.' >&2; exit 1; }
  python3 -c 'import tomllib'
  NC_SERVICE=task-workflow
  NC_REF=main
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/task-workflow-update.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" fetch --depth 1 \
    https://github.com/JanHG98/netcore-tetra.git "$NC_REF"
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  NC_REF=$(git -C "$NC_SRC" rev-parse HEAD)
  printf 'Ausgerollter main-Commit: %s\n' "$NC_REF"
  chmod 0755 "$NC_SRC"
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service "$NC_SERVICE" --commit "$NC_REF" --prepare-build-deps --apply
)
```

### Asset Management

**Auf `10.0.1.124` als root**: `ssh root@10.0.1.124`. Der Runtime-Updater prüft die vorhandene Installation und baut beziehungsweise kompiliert vor dem Dienststopp.

```bash
(
  set -Eeuo pipefail
  [[ $EUID -eq 0 ]] || { echo 'Auf dem Diensthost als root ausführen.' >&2; exit 1; }
  python3 -c 'import tomllib'
  NC_SERVICE=asset-management
  NC_REF=main
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/asset-management-update.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" fetch --depth 1 \
    https://github.com/JanHG98/netcore-tetra.git "$NC_REF"
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  NC_REF=$(git -C "$NC_SRC" rev-parse HEAD)
  printf 'Ausgerollter main-Commit: %s\n' "$NC_REF"
  chmod 0755 "$NC_SRC"
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service "$NC_SERVICE" --commit "$NC_REF" --prepare-build-deps --apply
)
```

### Sip Switch

**Auf `10.0.1.125` als root**: `ssh root@10.0.1.125`. Der Block aktualisiert das Python-Backend, das SIP-Runtime-Modul und die installierte Asterisk-AGI. Asterisk-Konfiguration und bestehende SIP-Zugänge bleiben erhalten. Fehlende PBX-/TBS-Zugangsdaten werden mit dem [SIP-Operatorhelfer](#private-sip-importdatei-vorbereiten) separat vorbereitet.

```bash
(
  set -Eeuo pipefail
  [[ $EUID -eq 0 ]] || { echo 'Auf dem Diensthost als root ausführen.' >&2; exit 1; }
  python3 -c 'import tomllib'
  NC_SERVICE=sip-switch
  NC_REF=main
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/sip-switch-update.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" fetch --depth 1 \
    https://github.com/JanHG98/netcore-tetra.git "$NC_REF"
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  NC_REF=$(git -C "$NC_SRC" rev-parse HEAD)
  printf 'Ausgerollter main-Commit: %s\n' "$NC_REF"
  chmod 0755 "$NC_SRC"
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service "$NC_SERVICE" --commit "$NC_REF" --prepare-build-deps --apply
)
```

### Alert Service

**Auf `10.0.1.129` als root**: `ssh root@10.0.1.129`. Bestehende Konfiguration, Environment-Datei mit Token und Zustellhistorie bleiben erhalten.

```bash
(
  set -Eeuo pipefail
  [[ $EUID -eq 0 ]] || { echo 'Auf dem Diensthost als root ausführen.' >&2; exit 1; }
  python3 -c 'import tomllib'
  NC_SERVICE=alert-service
  NC_REF=main
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/alert-service-update.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" fetch --depth 1 \
    https://github.com/JanHG98/netcore-tetra.git "$NC_REF"
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  NC_REF=$(git -C "$NC_SRC" rev-parse HEAD)
  printf 'Ausgerollter main-Commit: %s\n' "$NC_REF"
  chmod 0755 "$NC_SRC"
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service "$NC_SERVICE" --commit "$NC_REF" --prepare-build-deps --apply
)
```

### Deployment Core

**Aktueller Updateweg:** den [gemeinsamen VM119-Block aus dem aktuellen main](#vm119-auf-den-aktuellen-main-stand-aktualisieren) verwenden. Er übernimmt die neuen Desktop-/VPN-Funktionen und beide VM-Dienste gemeinsam.

<details>
<summary>Historischer Zehn-Dateien-Block vom 12.10.2026 — für neue Desktop-/VPN-Funktionen überholt</summary>

**Auf VM-H-DEPLOY-01 / `10.0.1.131` als `jhoffmeister`**: `ssh jhoffmeister@10.0.1.131`. Für diesen geprüften Stand ist der gezielte Zehn-Dateien-Updater der passende Weg. Er übernimmt Builder-UI, Rezept und die Image-Helfer in einer Idle-/Fingerprint-/Rollback-Transaktion. Einstellungen, Profile, Template und vorhandene Artefakte werden geprüft. Der Controller und Builder werden dafür kurz neu gestartet.

```bash
(
  set -Eeuo pipefail
  NC_REF=e1617b28e6f2e8f79958f88e84bc1dc1606c59ed
  NC_SRC=$(mktemp -d /tmp/netcore-deployment-features.XXXXXX)
  git clone --filter=blob:none --no-checkout https://github.com/JanHG98/netcore-tetra.git "$NC_SRC"
  git -C "$NC_SRC" checkout --detach "$NC_REF"
  [[ "$(git -C "$NC_SRC" rev-parse HEAD)" == "$NC_REF" ]]
  sudo python3 -B "$NC_SRC/Docs/integration/Z01-2026-10-07/vm119-image-features-update.py"
)
```

</details>

### Imagebuilder

**Aktueller Updateweg:** derselbe [gemeinsame VM119-Block](#vm119-auf-den-aktuellen-main-stand-aktualisieren). Ein zweiter Updateaufruf für den Builder ist nicht erforderlich. Danach im Formular einen neuen Imageauftrag mit der gewünschten Desktop-/VPN-Auswahl anlegen.

<details>
<summary>Historischer Imagebuilder-Block vom 12.10.2026 — durch den aktuellen VM119-Block ersetzt</summary>

**Ebenfalls auf VM-H-DEPLOY-01 / `10.0.1.131` als `jhoffmeister`**. Imagebuilder und Deployment-Formular gehören zu derselben geprüften Feature-Transaktion. Dieser eigene Block verwendet deshalb denselben idempotenten Helfer; nach dem vorigen Deployment-Core-Block ist `already_current` das erwartete Ergebnis. Er startet keinen Imageauftrag.

```bash
(
  set -Eeuo pipefail
  NC_REF=e1617b28e6f2e8f79958f88e84bc1dc1606c59ed
  NC_SRC=$(mktemp -d /tmp/netcore-imagebuilder-features.XXXXXX)
  git clone --filter=blob:none --no-checkout https://github.com/JanHG98/netcore-tetra.git "$NC_SRC"
  git -C "$NC_SRC" checkout --detach "$NC_REF"
  [[ "$(git -C "$NC_SRC" rev-parse HEAD)" == "$NC_REF" ]]
  sudo python3 -B "$NC_SRC/Docs/integration/Z01-2026-10-07/vm119-image-features-update.py"
)
```

</details>

### Discovery-Agent

Auf dem jeweiligen bereits mit Discovery ausgestatteten Diensthost als root ausführen; auf dem Pi zuerst `ssh jan@10.0.1.20`, dann `sudo -i`. Auf VM-H-DEPLOY-01 den [Deployment-Core-Block](#deployment-core) verwenden. Dieser Block erhält Modus, Profil, Inventar und Standort und startet keinen TBS-Funkdienst.

```bash
(
  set -Eeuo pipefail
  [[ $EUID -eq 0 ]] || { echo 'Auf dem Diensthost als root ausführen.' >&2; exit 1; }
  python3 -c 'import tomllib'
  NC_SERVICE=discovery
  NC_REF=main
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/discovery-update.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" fetch --depth 1 \
    https://github.com/JanHG98/netcore-tetra.git "$NC_REF"
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  NC_REF=$(git -C "$NC_SRC" rev-parse HEAD)
  printf 'Ausgerollter main-Commit: %s\n' "$NC_REF"
  chmod 0755 "$NC_SRC"
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service "$NC_SERVICE" --commit "$NC_REF" --prepare-build-deps --apply
)
```

### Syslog

**Auf dem bestehenden Logginghost**, zuletzt belegt **CT136 / `10.0.1.143`**. `CT136` ist die Proxmox-ID, keine IPv4-Adresse. Der Logging-only-Block aktualisiert Receiver, Preview-Bridge und Archivtimer, ohne den Rust-Observability-Build auszuführen.

Die beiden vorhandenen JSON-Konfigurationen werden gegen ihre vorherigen SHA-256 geprüft. Aktuelle Quelle erwartet einen lokalen NMS-Endpunkt `http://127.0.0.1:8210`; dies ist kein Bootstrap für einen neuen, davon getrennten Collector. Der Archivdienst darf zwischen Timerläufen inaktiv sein.

```bash
(
  set -Eeuo pipefail
  [[ $EUID -eq 0 ]] || { echo 'Auf dem Logginghost als root ausführen.' >&2; exit 1; }
  test -s /etc/netcore/syslog.json
  test -s /etc/netcore/openlab-hosts.json
  id netcore-observability >/dev/null
  [[ "$(systemctl show netcore-syslog.service -p LoadState --value)" == loaded ]]
  NC_LOG_CONFIG_SHA=$(sha256sum /etc/netcore/syslog.json /etc/netcore/openlab-hosts.json)
  NC_REF=e1617b28e6f2e8f79958f88e84bc1dc1606c59ed
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/syslog-update.XXXXXX)
  git clone --filter=blob:none --no-checkout https://github.com/JanHG98/netcore-tetra.git "$NC_SRC"
  git -C "$NC_SRC" checkout --detach "$NC_REF"
  [[ "$(git -C "$NC_SRC" rev-parse HEAD)" == "$NC_REF" ]]
  chmod 0755 "$NC_SRC"
  cd "$NC_SRC"
  env PREFIX=/opt/netcore-observability bash system-backend/observability/install/install-logging.sh
  printf '%s\n' "$NC_LOG_CONFIG_SHA" | sha256sum --check -
  for NC_LOG_UNIT in netcore-syslog.service netcore-syslog-preview.service netcore-syslog-archive.timer; do
    systemctl is-active --quiet "$NC_LOG_UNIT"
  done
  systemctl is-active netcore-syslog.service netcore-syslog-preview.service netcore-syslog-archive.timer
  systemctl list-timers netcore-syslog-archive.timer --no-pager
)
```

### Provisioning Core

**Auf `10.0.1.106` als root**: `ssh root@10.0.1.106`. Optionaler Dienst: nur auf dem Host einer bereits vorhandenen Installation ausführen.

```bash
(
  set -Eeuo pipefail
  [[ $EUID -eq 0 ]] || { echo 'Auf dem Diensthost als root ausführen.' >&2; exit 1; }
  python3 -c 'import tomllib'
  NC_SERVICE=provisioning-core
  NC_REF=main
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/provisioning-core-update.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" fetch --depth 1 \
    https://github.com/JanHG98/netcore-tetra.git "$NC_REF"
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  NC_REF=$(git -C "$NC_SRC" rev-parse HEAD)
  printf 'Ausgerollter main-Commit: %s\n' "$NC_REF"
  chmod 0755 "$NC_SRC"
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service "$NC_SERVICE" --commit "$NC_REF" --prepare-build-deps --apply
)
```

### Directory

**Auf `10.0.1.23` als root**: `ssh root@10.0.1.23`. Dieser Block setzt für die Prüfung einen laufenden Directory-Dienst mit Standardlistener `0.0.0.0:8095` oder `127.0.0.1:8095` ohne Seed-Startargument voraus. Andere Bindings, Ports und Seed-Startmodi werden vor dem Austausch abgelehnt. Die beiden bekannten ExecStart-Dateinamen `netcore_directory-server.py` und `netcore_directory_server.py` werden geprüft und mit ihrer jeweils passenden Quelle aktualisiert. Der installierte Dateiname, lokale Einstellungen und Datenbankpfad bleiben erhalten. Andere historische Varianten werden abgelehnt.

```bash
(
  set -Eeuo pipefail
  [[ $EUID -eq 0 ]] || { echo 'Auf dem Diensthost als root ausführen.' >&2; exit 1; }
  python3 -c 'import tomllib'
  NC_SERVICE=directory
  python3 - <<'NC_DIRECTORY_PREFLIGHT'
import argparse
from pathlib import Path
import subprocess

unit = "netcore-directory.service"
pid = int(subprocess.check_output(
    ["systemctl", "show", unit, "-p", "MainPID", "--value"], text=True).strip() or "0")
if pid <= 0:
    raise SystemExit("Directory muss für die Prüfung der Startargumente laufen.")
argv = [part.decode() for part in Path(f"/proc/{pid}/cmdline").read_bytes().split(b"\0") if part]
names = {"netcore_directory-server.py", "netcore_directory_server.py"}
indices = [i for i, value in enumerate(argv) if Path(value).name in names]
if len(indices) != 1:
    raise SystemExit("Unbekannter Directory-Startpfad; keine Datei geändert.")
parser = argparse.ArgumentParser(add_help=False)
parser.add_argument("--host", default="0.0.0.0")
parser.add_argument("--port", type=int, default=8095)
parser.add_argument("--db")
parser.add_argument("--seed")
args = parser.parse_args(argv[indices[0] + 1:])
if args.host not in ("0.0.0.0", "127.0.0.1") or args.port != 8095 or args.seed is not None:
    raise SystemExit("Abweichender Listener oder Seed-Startargument; diesen Bestand separat prüfen.")
print("Directory-Startargumente geprüft; vorhandener DB-Pfad bleibt erhalten.")
NC_DIRECTORY_PREFLIGHT
  NC_REF=main
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/directory-update.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" fetch --depth 1 \
    https://github.com/JanHG98/netcore-tetra.git "$NC_REF"
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  NC_REF=$(git -C "$NC_SRC" rev-parse HEAD)
  printf 'Ausgerollter main-Commit: %s\n' "$NC_REF"
  chmod 0755 "$NC_SRC"
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service "$NC_SERVICE" --commit "$NC_REF" --prepare-build-deps --apply
)
```

### Python-Brew

**Auf `10.0.1.22` als root** (`ssh root@10.0.1.22`). Dies ist dein bestehender Python-Brew: Unit `netcore-brew.service`, Skript `/var/opt/netcore-tetra/server.py`, Port **8081**. Die passende Quelle ist `system-backend/tbs-connect/server.py`; `misc/brew-server/` enthält einen getrennten Rust-Workspace.

Der Block übernimmt die sieben lokalen Zuweisungen `LISTEN_HOST`, `LISTEN_PORT`, `REALM`, `ADMIN_USER`, `ADMIN_PASS`, `NODES_FILE` und `FERNET_KEY`, kompiliert den Kandidaten und veröffentlicht atomar am bisherigen Ort. Unit und angrenzende Daten bleiben am vorhandenen Ort. `/login` ist ein HTTP-Smokecheck; Brew hat keinen `/health/ready`-Vertrag.

```bash
(
set -euo pipefail
[[ $EUID -eq 0 ]] || { echo 'Auf dem Brew-Host als root ausführen.' >&2; exit 1; }
  NC_REF=e1617b28e6f2e8f79958f88e84bc1dc1606c59ed
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/python-brew-update.XXXXXX)
  git clone --filter=blob:none --no-checkout https://github.com/JanHG98/netcore-tetra.git "$NC_SRC"
  git -C "$NC_SRC" checkout --detach "$NC_REF"
  [[ "$(git -C "$NC_SRC" rev-parse HEAD)" == "$NC_REF" ]]
  chmod 0755 "$NC_SRC"
  cd "$NC_SRC"
python3 - "$NC_SRC/system-backend/tbs-connect/server.py" <<'PY'
import ast, hashlib, os, re, stat, subprocess, sys, tempfile
from pathlib import Path

unit = 'netcore-brew.service'
target = Path('/var/opt/netcore-tetra/server.py')
source = Path(sys.argv[1])
expected = '6978177494aace38073d9a62be87566698a4a56f1674b7e749c6aca0e7490b5c'
if os.geteuid() != 0:
    raise SystemExit('root erforderlich')
if hashlib.sha256(source.read_bytes()).hexdigest() != expected:
    raise SystemExit('Brew-Quelle passt nicht zum geprüften Commit')
props = subprocess.check_output(
    ['systemctl', 'show', unit, '-p', 'LoadState', '-p', 'FragmentPath', '-p', 'ExecStart'],
    text=True, timeout=10)
if 'LoadState=loaded\n' not in props or not re.search(r'^FragmentPath=.+$', props, re.M):
    raise SystemExit('Die vorhandene Brew-Unit fehlt')
if not re.search(r'(?<![\w/.-])' + re.escape(str(target)) + r'(?=$|[\s;\"\'])', props):
    raise SystemExit('Die Brew-Unit verwendet einen anderen Skriptpfad')
info = target.lstat()
if not stat.S_ISREG(info.st_mode) or info.st_nlink != 1:
    raise SystemExit('Brew-Zieldatei muss eine reguläre Datei ohne Hardlinks sein')
before = target.read_bytes()
old = before.decode('utf-8')
new = source.read_text(encoding='utf-8')
fields = ('LISTEN_HOST', 'LISTEN_PORT', 'REALM', 'ADMIN_USER',
          'ADMIN_PASS', 'NODES_FILE', 'FERNET_KEY')

def assignments(text):
    found = {}
    for node in ast.parse(text).body:
        if isinstance(node, ast.Assign) and len(node.targets) == 1 and isinstance(node.targets[0], ast.Name):
            name = node.targets[0].id
            if name in fields:
                if name in found:
                    raise ValueError('Mehrdeutige lokale Brew-Einstellung: ' + name)
                found[name] = node
    if set(found) != set(fields):
        raise ValueError('Brew-Konfiguration ist abweichend; nichts ersetzt')
    return found

old_nodes, new_nodes = assignments(old), assignments(new)
old_lines, new_lines = old.splitlines(keepends=True), new.splitlines(keepends=True)
for name in sorted(fields, key=lambda name: new_nodes[name].lineno, reverse=True):
    previous, current = old_nodes[name], new_nodes[name]
    new_lines[current.lineno - 1:current.end_lineno] = old_lines[previous.lineno - 1:previous.end_lineno]
payload = ''.join(new_lines).encode('utf-8')
compile(payload, str(target), 'exec')  # Syntax only; never import/run the Brew server.
if target.read_bytes() != before:
    raise SystemExit('Brew-Skript wurde während der Vorbereitung geändert')
if payload == before:
    print('Brew bereits aktuell; kein Neustart erforderlich.')
    raise SystemExit(0)

def publish(data):
    fd, filename = tempfile.mkstemp(prefix='.netcore-brew-', dir=target.parent)
    try:
        with os.fdopen(fd, 'wb') as stream:
            os.fchown(stream.fileno(), info.st_uid, info.st_gid)
            os.fchmod(stream.fileno(), stat.S_IMODE(info.st_mode))
            stream.write(data)
            stream.flush()
            os.fsync(stream.fileno())
        os.replace(filename, target)
        directory = os.open(target.parent, os.O_RDONLY | os.O_DIRECTORY)
        try:
            os.fsync(directory)
        finally:
            os.close(directory)
    finally:
        if os.path.exists(filename):
            os.unlink(filename)

try:
    publish(payload)
    subprocess.run(['systemctl', 'restart', unit], check=True, timeout=180)
    subprocess.run(['systemctl', 'is-active', '--quiet', unit], check=True, timeout=10)
except BaseException:
    if target.read_bytes() == payload:
        publish(before)
        subprocess.run(['systemctl', 'restart', unit], check=False, timeout=180)
    raise
print('Python-Brew aktualisiert; lokale Einstellungen, Besitzer und Rechte erhalten.')
print('Installierte Datei SHA256: ' + hashlib.sha256(target.read_bytes()).hexdigest())
PY
systemctl show netcore-brew.service -p ActiveState -p SubState -p MainPID
curl --fail --show-error --silent --retry 10 --retry-connrefused --retry-delay 2 --retry-max-time 60 --max-time 10 http://10.0.1.22:8081/login >/dev/null
printf 'Brew HTTP-Smokecheck bestanden; Funk-/Verbindungstest folgt separat.\n'
)
```

### TBS

**Auf dem Pi `10.0.1.20`**, zuerst `ssh jan@10.0.1.20`, dann `sudo -i`. Der folgende Block betrifft die bestehende Image-TBS mit `/etc/netcore/config.toml`. Er erkennt die tatsächliche Unit und startet den Funkprozess nach dem Binary-Austausch neu. Der vorhandene Updater kann lokale TTS-Konfiguration migrieren und lokalen Piper deaktivieren.

Der folgende Block löst `main` einmal auf und übernimmt auch die bereits gemergte automatische Nachbarzellen-Ankündigung. Danach einmal die bestehende Nachbarliste im aktualisierten Dashboard speichern; die GUI repariert einen bisher falschen Broadcast-Wert. Für das weiterhin gebootete alte Image bleibt die oben dokumentierte Konfigurationskorrektur auf `2` der unmittelbar verfügbare Weg. Desktopwechsel und neue Build-VPN-Daten kommen über ein neues SD-Image; die Desktop-VPN-Editoren lassen sich mit dem Paketblock oben separat ergänzen.

```bash
(
  set -Eeuo pipefail
  [[ $EUID -eq 0 ]] || { echo 'Auf dem Pi zuerst sudo -i ausführen.' >&2; exit 1; }
  NC_CONFIG=/etc/netcore/config.toml
  test -s "$NC_CONFIG"
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/tbs-update.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" fetch --depth 1 \
    https://github.com/JanHG98/netcore-tetra.git refs/heads/main
  NC_REF=$(git -C "$NC_SRC" rev-parse FETCH_HEAD)
  git -C "$NC_SRC" checkout --detach "$NC_REF"
  [[ "$(git -C "$NC_SRC" rev-parse HEAD)" == "$NC_REF" ]]
  printf 'TBS-Update aus main: %s\n' "$NC_REF"
  chmod 0755 "$NC_SRC"
  cd "$NC_SRC"
  export PATH="/root/.cargo/bin:$PATH"
  NC_UNIT=$(python3 - "$NC_SRC" "$NC_CONFIG" <<'NC_TBS_UNIT'
import sys
sys.path.insert(0, sys.argv[1] + "/system-backend/deployment-core")
from deploy import service_spec
spec = service_spec({"services": [{"name": "tbs", "config_target": sys.argv[2]}]}, "tbs")
assert spec.get("unit_detected"), "Die installierte TBS-Unit ist nicht eindeutig ermittelt."
print(spec["unit"])
NC_TBS_UNIT
  )
  env UNIT="$NC_UNIT" CONFIG_PATH="$NC_CONFIG" NETCORE_GIT_COMMIT="$NC_REF" \
    bash install/update-basisstation.sh
  systemctl show "$NC_UNIT" -p ActiveState -p MainPID -p NRestarts
)
```



