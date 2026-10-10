# Brainstorming: Raspberry Pi OS – OpenVPN-Client mit GUI, Profilimport und Autostart

**Archivhinweis, 09.10.2026:** Diese Datei dokumentiert das im Titel beziehungsweise Quellenrahmen genannte Gespräch und dessen damalige Entscheidungen. Historische Kommandos, Planungen, Versionsangaben und Fehlerbilder bleiben erhalten; der heutige technische Stand steht in der [Gesamtroadmap](../roadmaps/gesamtroadmap.md) und in den [aktuellen Dienstanleitungen](../services/README.md). Die Archivaufnahme bestätigt keine spätere Implementierung oder Live-Abnahme.

Stand der Repository- und Quellenprüfung: **06.10.2026**. Historische Ergebnisse beziehen sich auf die jeweils genannten Daten und Commits.

Einrichtungsvorschlag für einen OpenVPN-Client auf Raspberry Pi OS mit Desktop: Pakete über NetworkManager bereitstellen, ein vorhandenes Clientprofil importieren, den Tunnel manuell starten, Erreichbarkeit prüfen und bei Bedarf an ein WLAN- oder Ethernet-Profil koppeln. Ein Start vor der Desktop-Anmeldung ist als Ausbauoption beschrieben.

**Erreicht ist ein dokumentierter Einrichtungsvorschlag.** Es wurden keine Ausgaben des Raspberry Pi, kein importiertes Profil, kein erfolgreicher Verbindungsaufbau und keine Betriebsbestätigung zurückgemeldet. Die unten beschriebenen Linux-Befehle wurden für diesen Entwicklungsstand nicht auf dem Zielgerät ausgeführt. Die Archivprüfung ergänzt einen gesondert belegten Repository-Stand; sie ist keine VPN-Abnahme.

## 1. Projektstand und Quellenumfang

| Feld | Wert |
|---|---|
| Thema | OpenVPN-Client auf Raspberry Pi OS mit grafischer Einrichtung, manueller Verbindung und profilgebundenem Autostart |
| Fachlicher Arbeitsumfang | „so, schritt für schritt anleitung um auf RaspiOS ein openvpn client zu installieren, damit der pi sich mit dem vpn verbindet, am besten auch mit GUI“ |
| Arbeits- und Erstellungsdatum | 2026-10-06, Europe/Berlin |
| Repository | [JanHG98/netcore-tetra](https://github.com/JanHG98/netcore-tetra) |
| Archivablage | Ausschließlich Branch `Archiving`, Verzeichnis `Docs/archive/` |
| Geprüfter Ausgangsstand von `Archiving` | [`083a2a3fa2b175463450ece09aec0ed06ad4792e`](https://github.com/JanHG98/netcore-tetra/commit/083a2a3fa2b175463450ece09aec0ed06ad4792e) |
| Zusätzlich nur lesend geprüfter `main` | [`9116c15d645458f99e236712b67a1ad970432791`](https://github.com/JanHG98/netcore-tetra/commit/9116c15d645458f99e236712b67a1ad970432791) |
| Archivdatei | `Docs/archive/2026-10-06_raspberry-pi-os-openvpn-client-gui-und-autostart.md` |
| Archivindex | [README.md](README.md) |

Die genannten SHAs bezeichnen die geprüften Quellenstände. Die Veröffentlichung der Notizen ist über die Git-Dateihistorie nachvollziehbar.

### 1.1 Zugänglicher Verlauf und Grenzen

Die Ausarbeitung umfasst acht Einrichtungsschritte und die dazu am 06.10.2026 recherchierten offiziellen Quellen. Eine spätere technische Korrektur oder Erfolgsbestätigung liegt nicht vor.

Die detaillierte Standortpolicy ist separat beschrieben: [OpenVPN-Autoverbindung nach WLAN und Heim-LAN](2026-10-06_raspberry-pi-openvpn-autoverbindung-vertrauenswuerdige-netze.md). Profilgebundener Autostart und eigene Standortpolicy sind unterschiedliche Steuerungsansätze.

## 2. Ziel, Ausgangslage und Anforderungen

Der Pi soll sich als Client mit einem bestehenden OpenVPN-VPN verbinden. Gewünscht ist eine nachvollziehbare Schrittfolge, bevorzugt mit grafischer Bedienung. Ein VPN-Server soll in diesem Auftrag nicht neu eingerichtet werden.

Der Einrichtungsweg setzt Raspberry Pi OS **mit Desktop ab Bookworm beziehungsweise Trixie** voraus. Das ist eine Voraussetzung des vorgeschlagenen Wegs, keine bestätigte Version des konkreten Geräts. Die Raspberry-Pi-Dokumentation beschreibt NetworkManager seit Bookworm als Standard; der reale Dienstzustand muss dennoch geprüft werden.

Für die Einrichtung werden ein vom VPN-Server exportiertes Clientprofil (`.ovpn`), gegebenenfalls separate Zertifikats-/Schlüsseldateien und die tatsächlich erforderlichen Authentisierungsdaten benötigt. Nichts davon wurde im Fachentwurf bereitgestellt.

| Anforderung oder Vorschlag | Herkunft und Status | Begründung / Grenze |
|---|---|---|
| OpenVPN-Client auf Raspberry Pi OS | Ausdrücklicher Anforderung; geplant | Pi soll einen bestehenden VPN-Zugang verwenden |
| Möglichst grafische Einrichtung und Bedienung | Ausdrücklicher Anforderung; geplant | Profilimport und Bedienung sollen am Desktop möglich sein |
| NetworkManager mit OpenVPN-Plugin verwenden | Konkreter technischer Vorschlag, keine spätere ausdrückliche Auswahlbestätigung | Integration in vorhandene Netzwerkverwaltung und grafischen Verbindungseditor |
| Verbindung `NetCore-VPN` nennen | Beispielname der Anleitung | Kein nachgewiesener vorhandener Profilname und keine bekannte UUID |
| Zuerst manuell verbinden und prüfen | Vorgeschlagener Ablauf | Authentisierung und Routing vor automatischem Betrieb klären |
| Autostart mit ausgewählten WLAN-/LAN-Profilen | Optional vorgeschlagener Ausbau | VPN kann zusammen mit einer Basisverbindung aktiviert werden |
| Start ohne Desktop-Anmeldung | Optional vorgeschlagener Ausbau | Profile und benötigte Secrets müssen ohne Benutzersitzung verfügbar sein |
| Unterwegs VPN, in bestimmten Heimnetzen kein VPN | Im Schlussabsatz aufgegriffener Projektwunsch | Einfache Profilzuordnung deckt nicht automatisch jede dynamische LAN-Regel ab |
| Heim-LAN `10.0.1.0/24` erkennen | Bekannter Kontextparameter, hier nicht am Pi bestätigt | Bei gemeinsamem Ethernet-Profil für Heim- und Fremdnetze ist ergänzende Policy nötig |

Unbekannt bleiben Pi-Modell, installierte OS-/Paketversionen, tatsächliche SSIDs und Netzwerkprofilnamen, VPN-Server/Port/Transport, Tunneladressierung, Clientprofilinhalt, Authentisierungsart, DNS, IPv6 und Split-/Full-Tunnel-Anforderung. Es wurde keine konkrete Firewall-, Routing- oder Serverkonfiguration beschlossen.

## 3. Statusmodell und erreichter Entwicklungs-/Betriebsstand

| Status | Bedeutung in dieser Dokumentation | Konkreter Stand |
|---|---|---|
| **Idee** | Erwähnter möglicher Ausbau ohne Umsetzungsbeleg | Dynamische LAN-Erkennung als Ergänzung zur GUI-Einrichtung |
| **Beschlossen/geplant** | Ausdrückliches Ziel oder als solcher gekennzeichneter Einrichtungsvorschlag | Ziel OpenVPN-Client mit GUI; acht Schritte als vorgeschlagener Weg |
| **Implementiert** | Im geprüften Repository oder anhand konkreter Zielgeräteartefakte vorhanden | Bestehende allgemeine WLAN-Verwaltung im Repository; kein durch diesen Fachentwurf implementierter OpenVPN-Client oder Autotoggle |
| **Getestet** | Konkrete Prüfung mit belegtem Umfang | Quellen- und statische Repository-Prüfung im Prüfdurchlauf vom 06.10.2026; keine funktionalen Pi-/VPN-Tests |
| **Im Betrieb bestätigt** | Erfolg am realen Zielsystem durch überprüfbaren Nachweis oder eindeutig eingeordnete Betreiberbestätigung | Für die Einrichtung dieser Arbeitsphase nicht vorhanden |

Die Bereitstellung einer Anleitung ist kein Installationsnachweis. Ebenso beweisen vorhandene `nmcli`-Aufrufe für WLAN nicht, dass ein VPN-Profil eingerichtet ist. Allgemeine Befunde aus anderen NetCore-Arbeitsphasen werden nicht zu einem Betriebserfolg dieses Pis umgedeutet.

## 4. Architektur, Komponenten und Schnittstellen

Der vorgeschlagene Weg verwendet den grafischen `nm-connection-editor` zur Konfiguration eines NetworkManager-Verbindungsprofils. Das OpenVPN-Plugin stellt den VPN-Typ bereit. NetworkManager verwaltet den Verbindungslebenszyklus; der OpenVPN-Client baut über den vorhandenen Internetzugang den Tunnel zum konfigurierten Server auf. `nmcli` dient als Bedien- und Diagnosealternative für dasselbe Profil.

| Komponente | Aufgabe | Abhängigkeit / Einordnung |
|---|---|---|
| Raspberry Pi OS Desktop | Grafische Sitzung und Netzwerkmenü | Desktop auf dem Zielgerät vorausgesetzt; OS Lite wurde nicht um einen Desktop erweitert |
| NetworkManager | WLAN/LAN und VPN-Profile verwalten | Laufender Dienst und verwaltete Basisverbindung erforderlich |
| `openvpn` | VPN-Clientsoftware | Endpunkt, Transport und Kryptoparameter stammen aus dem echten Clientprofil |
| `network-manager-openvpn` | OpenVPN-Unterstützung für NetworkManager | Plugin für importierte OpenVPN-Verbindungen |
| `network-manager-openvpn-gnome` | Grafische OpenVPN-Konfiguration | Ergänzt den Verbindungseditor |
| `network-manager-gnome` | Applet und Editor beziehungsweise Paketweiterleitung | Unter Debian Trixie Übergangspaket zu `network-manager-applet` und `nm-connection-editor` |
| `nm-connection-editor` | Profile importieren und bearbeiten | Aus der grafischen Benutzersitzung ohne `sudo` starten |
| `nmcli` | Verbindung starten/stoppen und Status lesen | Bedient dasselbe NetworkManager-Profil |
| WLAN-/Ethernet-Profil | Trägt den Internetzugang und die optionale VPN-Verknüpfung | Die Verknüpfung gilt pro Basisprofil |
| Secret-Verwaltung | Authentisierungsdaten bereitstellen | Interaktiv oder passend für unbeaufsichtigten Start; keine Secrets im Git-Archiv |

Beim profilgebundenen Autostart verwendet NetworkManager das Konzept `connection.secondaries`: Eine Liste von VPN-UUIDs wird beim Aktivieren der Basisverbindung mit aktiviert. Allgemeines `connection.autoconnect` allein wird für VPN-Profile nicht unterstützt. Dieses Verhalten wurde in der NetworkManager-Referenz geprüft.

Die Desktop-GUI ist keine eigene NetCore-Weboberfläche. Der im Repository vorhandene WLAN-Reiter und seine API sind getrennte Komponenten; ein OpenVPN-Import oder eine VPN-Schaltfläche im NetCore-Dashboard wurde für diesen Entwicklungsstand weder beauftragt noch nachgewiesen.

## 5. Relevante Dateien, Dienste, Pfade und Parameter

| Element | Wert beziehungsweise Beispiel | Nachweisstatus |
|---|---|---|
| OS-Information | `/etc/os-release` | Vorgeschlagene Diagnosequelle |
| Netzwerkdienst | `NetworkManager`, Journal über `journalctl -u NetworkManager` | Dienst auf dem Pi nicht geprüft |
| Lokaler Profil-/Zertifikatsordner | `$HOME/VPN` mit Verzeichnismodus `700` | In der Anleitung vorgeschlagen, nicht angelegt bestätigt |
| Clientprofil | Vorhandene `.ovpn`-Datei | Nicht als Anhang geliefert |
| Mögliche separate Dateien | `ca.crt`, `client.crt`, `client.key`, `ta.key` | Nur Dateinamenbeispiele, keine Inhalts- oder Existenzannahme |
| NetworkManager-VPN-Name | `NetCore-VPN` | Beispiel, muss bei anderer Benennung in Befehlen ersetzt werden |
| VPN-UUID | Unbekannt | Kein Wert erfunden |
| Tunnelinterface | Häufig `tun0` | Beispiel; tatsächlicher Name und Tunneltyp hängen von der Konfiguration ab |
| Heim-LAN | `10.0.1.0/24` | Kontext für spätere Standortregel, nicht gemessen |
| WLAN-SSIDs / Ethernet-Profil | Unbekannt | Vor Automatisierung erfassen |
| OpenVPN-Server und Port | Unbekannt | Aus realem `.ovpn`-Profil bestimmen; kein Port pauschal festgeschrieben |
| Transport | OpenVPN gemäß Clientprofil, UDP/TCP nicht festgelegt | Nicht aus Produktdefaults als Standortwert ableiten |
| Interner Prüfdienst | Ein bekannter erreichbarer NetCore-Dienst im Zielnetz | Kein konkreter Server oder Port im Fachentwurf benannt |
| DNS / Routen / IPv6 | Nicht festgelegt | Müssen zur tatsächlichen Server-/Clientkonfiguration passen |

Die Schlüsseldateinamen dienen allein der Orientierung. Passwörter, Tokens, private Schlüssel, Zertifikatsinhalte und sonstige Zugangsdaten sind in dieser Dokumentation nicht enthalten.

## 6. Historischer Einrichtungsablauf und Befehle

**Ausführungsstatus aller folgenden Linux-Befehle: nur vorgeschlagen, nicht auf dem Raspberry Pi ausgeführt oder durch Betriebsausgaben bestätigt.** Es wird kein Remotezugriff auf das Zielgerät behauptet. Paketversionsnummern und GUI-Bezeichnungen können vom konkreten System abhängen.

### 6.1 OS und Dienstzustand prüfen

```bash
cat /etc/os-release
systemctl is-active NetworkManager
```

Erwartung der Anleitung: Der Dienststatus lautet `active`. Bei `inactive`, fehlendem Dienst oder abweichender Netzwerkverwaltung sollten zuerst die Ausgaben ausgewertet und die Anleitung angepasst werden. Das war ein diagnostischer Abzweig, kein tatsächlich aufgetretener Fehler. Eine pauschale Migration der bestehenden Netzwerkkonfiguration wurde nicht angeordnet.

### 6.2 Pakete installieren

```bash
sudo apt update
sudo apt install openvpn network-manager-openvpn network-manager-openvpn-gnome network-manager-gnome
```

Die Paketgruppe deckt Client, NetworkManager-Plugin und grafischen Editor ab. Die Recherche bestätigte die Paketaufteilung beziehungsweise das Übergangspaket unter Trixie. Es gibt keine Ausgabe eines erfolgreichen `apt`-Laufs auf dem Ziel-Pi.

### 6.3 Clientdateien lokal ablegen

```bash
mkdir -p "$HOME/VPN"
chmod 700 "$HOME/VPN"
```

Danach sollten die `.ovpn`-Datei und gegebenenfalls separate Zertifikate/Schlüssel in diesen Ordner kopiert werden. Ein Dateiname für einen konkreten Kopierbefehl wurde nicht erfunden.

Die Anleitung wies darauf hin, Zertifikats- und Schlüsseldateien nicht nachträglich zu verschieben oder zu löschen: Das importierte Profil kann weiterhin auf deren Pfade verweisen. Vor unbeaufsichtigtem Start muss zusätzlich feststehen, dass diese Pfade bereits verfügbar sind; der bloße Import beweist das nicht.

### 6.4 Grafisch importieren

Im Terminal des Pi-Desktops, ohne `sudo`:

```bash
nm-connection-editor
```

Vorgeschlagene Klickfolge:

1. Mit `+` eine Verbindung hinzufügen.
2. „Gespeicherte VPN-Konfiguration importieren …“ beziehungsweise den sinngleichen Importpunkt wählen.
3. Die vorhandene `.ovpn`-Datei auswählen und importieren.
4. Als Beispielnamen `NetCore-VPN` vergeben.
5. Im Reiter VPN die übernommenen Angaben kontrollieren.
6. Benutzername und Passwort nur ergänzen, wenn der tatsächliche Server diese verlangt.
7. Speichern.

Zertifikatsauthentisierung und zusätzliche Benutzer-/Passwortauthentisierung sind getrennte Verfahren. Bei einem ausschließlich zertifikatsbasierten Zugang muss nicht zwingend ein Benutzerpasswort existieren. Eine eventuell verschlüsselte Schlüsseldatei kann dennoch eine Passphrase benötigen. Die konkrete Authentisierungsart blieb unbekannt.

### 6.5 Verbindung starten und trennen

Als Einstieg vorgesehen sind das Netzwerksymbol in der Taskleiste und ein Eintrag wie „VPN-Verbindungen → NetCore-VPN“, eventuell unter erweiterten Optionen. **Die genaue Menüstruktur wurde nicht auf dem Pi angesehen oder getestet.** Desktop-/Panelversionen können sich unterscheiden.

Wenn kein VPN-Schalter angeboten wird, lautet die Terminalalternative:

```bash
nmcli --ask connection up "NetCore-VPN"
```

`--ask` erlaubt die Abfrage fehlender Angaben beziehungsweise Secrets. Die Prüfung der offiziellen Referenz bestätigt dieses Verhalten. Der Befehl ist interaktiv und kein Vorschlag für einen unbeaufsichtigten Bootjob.

Zum Trennen desselben Profils:

```bash
nmcli connection down "NetCore-VPN"
```

Es wurde kein separater `openvpn-client@…`-Dienst für diesen GUI-Weg aktiviert.

### 6.6 Verbindung und Nutzpfad prüfen

```bash
nmcli connection show --active
ip -br address
```

Erwartung: Das VPN-Profil erscheint aktiv; eine passende Tunnelschnittstelle ist vorhanden. Ein Name wie `tun0` ist ein Beispiel und kein zwingendes Erfolgskriterium.

Zusätzlich sollte ein bekannter interner Dienst im Zielnetz geöffnet werden. Ein aktiver Tunnel allein bestätigt weder richtige Routen noch DNS, Serverfreigaben oder Anwendungserreichbarkeit. Der Fachentwurf enthält kein Ergebnis dieses Tests und keine öffentliche IP-Prüfung als angeblichen Betriebsnachweis.

Vorgeschlagene Fehlerdiagnose:

```bash
sudo journalctl -u NetworkManager -b -n 100 --no-pager
```

Das Journal wurde nicht nachgereicht. Es gibt deshalb keine konkret diagnostizierte TLS-, Authentisierungs-, Routing- oder DNS-Störung.

### 6.7 VPN an eine Basisverbindung koppeln

Erneut `nm-connection-editor` öffnen, diesmal das gewünschte **WLAN- oder Ethernet-Profil** bearbeiten. Unter Allgemein sollte die Option sinngemäß „Automatisch mit VPN verbinden, wenn diese Verbindung verwendet wird“ aktiviert und `NetCore-VPN` ausgewählt werden.

Die Einstellung ist pro gewünschtem Basisprofil zu wiederholen. Sie wirkt beim nächsten Aufbau dieser Basisverbindung; daraus folgt nicht, dass allein das Speichern sofort einen bestehenden Tunnel aktiviert. Zu beachten ist, dass `connection.autoconnect` am VPN-Profil allein nicht genügt.

**Grenze:** Diese Verknüpfung ist keine umfassende Always-on-/Recovery-Policy. Wiederanlauf nach Tunnelverlust, Verhalten bei gleichzeitigen Uplinks und allgemeine Behandlung neuer WLANs wurden damit nicht implementiert oder getestet.

### 6.8 Vor der Desktop-Anmeldung starten

Für einen unbeaufsichtigten Start sollten die beteiligten Basis- und VPN-Profile unter Allgemein allen Benutzern zur Verfügung stehen. Erforderliche VPN-Passwörter beziehungsweise Schlüsselpasswörter müssen ohne interaktive Desktop-Anmeldung verfügbar sein. Die Anleitung nannte dafür, sofern angeboten, „Passwort für alle Benutzer speichern“.

Ein nur in der angemeldeten Sitzung nutzbares Profil oder ein erst dann entsperrter persönlicher Schlüsselbund erfüllt diese Anforderung nicht. Eine manuell erforderliche MFA-Abfrage verhindert den vollständig unbeaufsichtigten Start. Daraus wurde keine Anweisung zur Abschaltung bestehender MFA abgeleitet.

Als abschließender Test ist ein Neustart mit erneuter Statusprüfung vorgesehen. Es ist kein solcher Neustart oder erfolgreicher Boot-VPN-Aufbau bestätigt.

## 7. Standortabhängige Automatik und Abgrenzung anderer Ansätze

Im Schlussabsatz wurde der frühere Projektwunsch aufgegriffen: VPN unterwegs, zu Hause aus. Für ein bekanntes Handy-Hotspot-Profil lässt sich der profilgebundene VPN-Start aktivieren und beim Heim-WLAN weglassen. Damit entsteht jedoch noch keine allgemeine Regel für alle unbekannten oder künftig hinzugefügten Netze.

Wird dasselbe Ethernet-Profil im Heim-LAN und unterwegs verwendet, unterscheidet die `secondaries`-Zuordnung diese Standorte nicht. Die Ausnahme `10.0.1.0/24` braucht dann zusätzliche, bewusst definierte Logik. Hier wurden weder eine konkrete SSID-Liste noch ein Dispatcher-Skript, Timer, systemd-Service oder Kill-Switch erstellt.

| Weg | Einordnung | Konsequenz für die Fortsetzung |
|---|---|---|
| NetworkManager-VPN-Profil und grafischer Editor | Einrichtungsvorschlag dieser Arbeitsphase | Zuerst manuell importieren und real testen |
| VPN als `secondaries` ausgewählter WLAN-/LAN-Profile | Optionaler Ausbau dieser Arbeitsphase | Einfacher profilgebundener Start; begrenzte Standort-/Recovery-Semantik |
| NetworkManager-Dispatcher mit eigener Policy | In einer eigenen Ausarbeitung ausgearbeitet; nur Querverweis hier | [Separates Policy-Archiv](2026-10-06_raspberry-pi-openvpn-autoverbindung-vertrauenswuerdige-netze.md) vor Ausbau lesen |
| Timer und `openvpn-client@netcore.service` | Historische Imagebuilder-Vorschau im Repository | Eigener Lebenszyklus; keine Umsetzung des GUI-Imports nachgewiesen |
| Ausschließlich `connection.autoconnect` am VPN | Als ausreichender Autostartmechanismus ausdrücklich verneint | Stattdessen Basisprofil-Verknüpfung oder definierte Policy |
| Andere VPN-Produkte oder ein neuer Server | Nicht Gegenstand des Fachentwurfs | Kein Auswahlvergleich und keine Verwerfungsentscheidung erfinden |

Keine spätere Korrektur anhand der Anlage ersetzt die Anleitung. Der geprüfte Repository-Abgleich ergänzt ihren Kontext. Vor einem Ausbau ist ein eindeutiger Verantwortlicher für Start/Stop festzulegen; die GUI-Verknüpfung und eine separate Policy dürfen nicht unbeabsichtigt gegeneinander arbeiten.

## 8. Zusätzlich geprüfter Repository-Stand vom 06.10.2026

### 8.1 Methode und Prüfgrenzen

Die Branchköpfe von `Archiving` und `main` wurden direkt von GitHub gelesen. Ihre vollständigen rekursiven Git-Bäume wurden ohne Trunkierung abgerufen: 3.183 Einträge in `Archiving`, 2.822 in `main`, einschließlich Verzeichniseinträgen. Das sind keine Dienst- oder Quelldateizahlen.

Für die Textsuche konnte eine vorhandene lokale Arbeitskopie von `Archiving` rein lesend genutzt werden. Alle **2.228 Blobpfade außerhalb von `Docs/archive/`** stimmten mit ihren Blob-SHAs gegen den aktuellen Remote-Ausgangsstand überein. Dadurch ist die lokale Suche in diesem Bereich dem genannten Remote-Snapshot zuordenbar; die ältere lokale Commitspitze wurde nicht als aktueller Remote-Stand ausgegeben.

Gesucht wurde in getrackten Textdateien außerhalb des Archivs nach `openvpn`, `vpn-policy`, `TRUSTED_SSIDS`, `dispatcher.d`, `connection.secondaries` und `network-manager-openvpn`. Relevante Dateien und Auszüge wurden zusätzlich gelesen. Die Schlussfolgerung ist auf diese Quellen und Suchkriterien begrenzt; ungetrackte Zielgerätedateien und alle anderen Git-Refs wurden nicht untersucht.

### 8.2 Konkrete Befunde

| Quelle | Befund | Aussagegrenze |
|---|---|---|
| [`wifi.rs`](https://github.com/JanHG98/netcore-tetra/blob/083a2a3fa2b175463450ece09aec0ed06ad4792e/crates/tetra-entities/src/wifi.rs) | WLAN-Verwaltung über `nmcli`, einschließlich Scan, gespeicherter WLAN-Profile, Verbindung/Trennung und Radio-Steuerung; Aufruf-Timeout 15 Sekunden | Bestehende WLAN-Funktion, kein OpenVPN-Import oder Heimnetz-VPN-Autotoggle dieser Arbeitsphase |
| [`net_dashboard/server.rs`](https://github.com/JanHG98/netcore-tetra/blob/083a2a3fa2b175463450ece09aec0ed06ad4792e/crates/tetra-entities/src/net_dashboard/server.rs) | Vorhandene `/api/wifi/`-Routen binden die WLAN-Funktionen an | Kein Nachweis einer NetCore-VPN-GUI |
| [`Komplettguide vom 28.09.2026`](https://github.com/JanHG98/netcore-tetra/blob/083a2a3fa2b175463450ece09aec0ed06ad4792e/Docs/NetCore-Tetra-Komplettguide-2026-09-28.md) | Explizite Entwicklungsvorschau zu Pi-Images und VPN-Automatik | Historischer anderer Quellstand, keine geprüfte Installation |
| [`Systemhandbuch vom 28.09.2026`](https://github.com/JanHG98/netcore-tetra/blob/083a2a3fa2b175463450ece09aec0ed06ad4792e/Docs/NetCore-Tetra-Systemhandbuch-2026-09-28.md) | Entsprechende VPN-Vorschau und identische wesentliche Aussagen in den Suchtreffern | Dokumentation ersetzt keine Integration des beschriebenen Dienstes |
| `system-backend/deployment-core/` | In beiden geprüften Remote-Bäumen nicht vorhanden | Fehlende Komponente am geprüften Stand; keine Behauptung über sämtliche Historie oder Zielgeräte |
| Suche nach der konkreten GUI-/Policy-Integration | Die genannten Suchmuster lieferten außerhalb des Archivs nur die Handbuch-/Guide-Passagen zu OpenVPN; keine passende Implementierung des hier beschriebenen Ablaufs | Begrenzter statischer Suchnachweis, kein Beweis vollständiger Abwesenheit jeder denkbaren VPN-Funktion |
| [`ROADMAP.md` auf `main`](https://github.com/JanHG98/netcore-tetra/blob/9116c15d645458f99e236712b67a1ad970432791/ROADMAP.md) | Z01.1 nennt den Abgleich fehlender Deployment-/Syslog-Arbeit einschließlich Pi-VPN-Policy als aktuellen ersten Gesamtschritt; Z01.2 Integration, Z01.4 reale Installations-/Recovery-/Pi-Abnahme bleiben offen | Projektpriorität aus geprüfter Roadmap, nicht nachträglich im Fachentwurf beschlossen |
| Vorhandenes [VPN-Policy-Archiv](2026-10-06_raspberry-pi-openvpn-autoverbindung-vertrauenswuerdige-netze.md) | Getrennte Ausarbeitung zu vertrauenswürdigen WLANs und Heim-LAN vorhanden | Eigenständige Ausarbeitung; nicht überschrieben und nicht als Live-Nachweis verwendet |

`wifi.rs`, `net_dashboard/server.rs`, `net_dashboard/ui/dashboard.html` und die beiden Handbücher haben zwischen den geprüften `Archiving`- und `main`-Snapshots jeweils identische Blob-SHAs. Diese Inhaltsgleichheit bestätigt den gemeinsamen Quellstand der genannten Dateien, nicht ihre erfolgreiche Ausführung.

Die vorhandenen WLAN-Endpunkte sind `GET /api/wifi/status`, `GET /api/wifi/scan`, `GET /api/wifi/saved`, `GET /api/wifi/available` sowie `POST /api/wifi/connect`, `POST /api/wifi/disconnect`, `POST /api/wifi/forget` und `POST /api/wifi/radio`. Sie werden als geprüfter Repository-Kontext dokumentiert; im Fachentwurf waren sie keine Integrationsanforderung.

### 8.3 Historische Imagebuilder-Vorschau und Integrationslücke

Der gelesene Guide kennzeichnet den einschlägigen Anhang ausdrücklich als Entwicklungsvorschau für `feature/openlab-discovery-deployment` bei `bbf039729b9b05f8d623b11195ca24a124f68d16`. Diese SHA-/Branchangabe wurde aus der Dokumentation übernommen; der historische Commit wurde bei der Quellenprüfung vom 06.10.2026 nicht erneut auf Codeumfang oder Laufzeit getestet.

Laut Vorschau prüft eine Richtlinie etwa alle **15 Sekunden** aktive NetworkManager-Verbindungen. Eine konfigurierte vertrauenswürdige SSID oder eine kabelgebundene Adresse im konfigurierten LAN-Präfix schaltet den VPN-Dienst aus; andernfalls wird `openvpn-client@netcore.service` eingeschaltet. Die Tunneladresse zählt nicht als Heimnetz. Richtliniendaten sollen in `/etc/netcore/image-network.json` liegen; ohne eingebettetes OpenVPN-Profil soll der Timer nicht aktiviert werden. Funkdienste sollen dadurch nicht neu starten.

Diese Variante verwendet NetworkManager für Netzerkennung, aber einen eigenen OpenVPN-systemd-Dienst für den Tunnel. Sie ist deshalb nicht identisch mit dem hier vorgeschlagenen importierten NetworkManager-VPN-Profil. Die Vorschau beschreibt außerdem Raspberry Pi OS Lite und streng begrenzte eingebettete Profile; daraus folgt keine Pflicht, die Desktop-Anleitung nachträglich auf dieses Imageverfahren umzuschreiben.

Die geprüfte zentrale Roadmap nennt diese Übernahmelücke ausdrücklich. `PR #57` und der Feature-Commit werden dort als historische Herkunft referenziert; der PR wurde in diesem Auftrag nicht separat neu geprüft. Es wurde kein neuer Implementierungs-PR erzeugt und keine historische Entwicklung nach `main` oder `Archiving` gemergt.

## 9. Fehler, Diagnose, Korrekturen und Tests

### 9.1 Keine belegte Störung auf dem Pi

Im Fachentwurf wurden keine Fehlermeldungen gemeldet. Folgende Fälle waren vorsorgliche Diagnose- oder Konfigurationshinweise:

| Möglicher Fall | In der Anleitung vorgesehener Umgang | Tatsächliches Ergebnis |
|---|---|---|
| NetworkManager fehlt oder ist nicht aktiv | OS-/Dienstinformationen ansehen und Anleitung anpassen | Nicht beobachtet |
| VPN-Eintrag fehlt im Desktopmenü | Importiertes Profil mit `nmcli --ask connection up` starten | Nicht getestet |
| Anmeldung wird benötigt | Erforderliche Zugangsdaten passend zur Authentisierungsart ergänzen | Keine Daten und kein Versuch übermittelt |
| VPN-Aufbau scheitert | NetworkManager-Journal des aktuellen Boots lesen | Kein Journal vorgelegt |
| Tunnel aktiv, interner Dienst nicht erreichbar | Nutzpfad einschließlich Routen/DNS prüfen | Kein solcher Fehler nachgewiesen |
| Automatik startet erst nach Login | Profilberechtigung und Secret-Verfügbarkeit prüfen | Kein Bootproblem nachgewiesen |
| Gleiches Ethernet-Profil zu Hause und unterwegs | Ergänzende Standortpolicy vorsehen | Offene Anforderung, kein reparierter Defekt |

Es gibt deshalb keine „funktionierende Reparatur“, die als ausgeführt dokumentiert werden könnte. Die wichtigste technische Festlegung war die korrekte Zuordnung des VPN-Autostarts zum Basisprofil.

### 9.2 Im Prüfdurchlauf vom 06.10.2026 tatsächlich geprüfte Punkte

- Vollständigen Einrichtungsvorschlag und seine Quellen geprüft.
- Aktuelle Remote-Branchköpfe und ungekürzte Git-Bäume gelesen.
- Vorhandenen Archivindex und thematisch ähnliches Policy-Archiv geprüft; keine bereits vorhandene gleichartige GUI-Ausarbeitung gefunden.
- Lokalen Textbestand außerhalb des Archivs per Pfad/Blob-SHA mit dem Remote-Ausgangsstand abgeglichen.
- Relevante WLAN-Implementierung, Dokumentationsvorschau, fehlenden Deployment-Core-Pfad und aktuelle Gesamtroadmap statisch geprüft.
- Vorhandene offizielle Quellen zur Paketaufteilung, NetworkManager-Integration, `secondaries`, `--ask` und Profil-/Secret-Verfügbarkeit herangezogen.

Diese Nachweise betreffen Dokumentation und Quellen. Es wurde weder Linux-Software auf dem Pi installiert noch ein VPN, eine Firewall oder ein Funkdienst gestartet, verändert oder getestet. Build-/CI-Läufe wären für diese ausschließlich dokumentarische Änderung kein Nachweis des Pi-Betriebs und wurden nicht als solche ausgegeben.

### 9.3 Noch ausstehende Abnahme

Alle folgenden Tests sind Empfehlungen für die Fortsetzung, nicht bereits durchgeführt:

| Test | Abnahmekriterium |
|---|---|
| OS und Netzwerkverwaltung | Versionen bekannt; NetworkManager aktiv und Basisprofil korrekt verwaltet |
| Paketinstallation und GUI | OpenVPN-Import im tatsächlichen Desktop verfügbar; Paketstand protokolliert |
| Profilimport | Endpunkt/Auth-Verfahren stimmen; referenzierte Dateien lesbar und dauerhaft vorhanden |
| Manueller Aufbau und Abbau | Profilstatus und Tunnelzustand entsprechen der Aktion; eventuelle Fehler nachvollziehbar |
| Anwendungspfad | Benötigte NetCore-Dienste über das VPN erreichbar; Routen und DNS passen |
| Profilgebundener Autostart | Gewähltes WLAN/LAN startet das richtige VPN; andere Profile verhalten sich wie festgelegt |
| Start vor Login | Nach Boot ohne Desktop-Anmeldung ist die benötigte Verbindung verfügbar; keine unerfüllte Secret-Abfrage |
| Tunnel-/Serverausfall | Fehler sichtbar; gewünschtes Wiederanlaufverhalten festgelegt und nachgewiesen |
| Heimnetz-/Fremdnetzwechsel | Standortregel arbeitet einschließlich gemeinsamem Ethernet-Profil wie vereinbart |
| Mehrere Uplinks | Priorität und VPN-Zustand bei gleichzeitigem WLAN/LAN eindeutig |
| IPv6, DNS, Split-/Full-Tunnel | Verhalten entspricht der noch zu entscheidenden Anforderung |
| NetCore-Betrieb | Relevante Core-/TBS-Verbindungen funktionieren; Netzwechsel verursacht keinen unbeabsichtigten Funkdienst-Neustart |

## 10. Offene Aufgaben, Roadmap-Kandidaten und nächste Schritte

Die folgende Reihenfolge konkretisiert die Fortsetzung dieses Einrichtungsthemas. Im Fachentwurf wurden keine Kalendertermine oder projekweiten Prioritätsänderungen vereinbart. Für die Gesamtentwicklung bleibt die am Archivdatum gelesene `main`-Roadmap maßgeblich: **Z01.1 Quellvergleich/Integrationsplan**, danach kontrollierte Integration und echte Abnahme.

| Kandidat | Nächster Schritt | Abhängigkeit / Abschlusskriterium |
|---|---|---|
| VPN-GUI-01 | Pi-Modell, OS-/Paketversionen, vorhandene Netzwerkverwaltung und Profilnamen erfassen | Ausgaben von `/etc/os-release`, NetworkManager-Status und aktuellem Netzwerkbestand liegen vor |
| VPN-GUI-02 | Echtes Clientprofil lokal bereitstellen und GUI-Import durchführen | Serverprofil und Authentisierungsart vorhanden; Zugangsdaten verbleiben außerhalb Git |
| VPN-GUI-03 | Manuelle Verbindung einschließlich interner Dienste prüfen | Aufbau/Abbau, Adresse, Routen und Anwendungspfad durch echte Ausgaben belegt |
| VPN-GUI-04 | Optionalen Autostart und Start vor Login konfigurieren | Basisprofile, Berechtigungen und Secret-Verfügbarkeit eindeutig; Boot-/Wiederverbindungstest bestanden |
| VPN-GUI-05 | Standortpolicy mit bestehendem VPN-Policy-Archiv und historischem Imagebuilder abgleichen | Ein Steuerungsweg gewählt; SSIDs, Heim-LAN, Mehr-Uplink- und Fehlerverhalten festgelegt |
| VPN-GUI-06 | Gewählten Weg reproduzierbar in Deployment/Betriebsdokumentation integrieren | Z01-Quellabgleich berücksichtigen; keine doppelte konkurrierende VPN-Steuerung; Pi-/TBS-Abnahme vorhanden |

Weitere offene Detailpunkte: GUI-Schalter auf dem tatsächlichen Panel, dauerhafte Zertifikatspfade, tatsächlicher Profilname/UUID, Passwort- beziehungsweise Schlüsselpassphrasenbedarf, MFA-Bedingungen, DNS/IPv6, Split-/Full-Tunnel und Recovery nach Tunnelverlust. Ein neues GUI-Frontend, VPN-Serveraufbau oder Kill-Switch wurde hier nicht beschlossen.

Die Kandidaten sind noch nicht in die Root-Roadmap übernommen. Profile, Dienste und Zielgeräte wurden nicht verändert.

## 11. Quellen, Querverweise und Anhänge

### 11.1 Offizielle Quellen des Einrichtungsvorschlags

Die folgenden URLs wurden im Gespräch am 06.10.2026 recherchiert beziehungsweise geöffnet. Sie stützen die technischen Aussagen, sind aber kein Zielgeräte-Test. Der Debian-Wiki-OpenVPN-Inhalt war über das Suchergebnis verfügbar; ein zusätzlicher direkter Abruf lieferte HTTP 403. Diese Grenze wurde nicht als erfolgreiche Volltextabfrage ausgegeben.

| Quelle | Relevanz |
|---|---|
| [Raspberry Pi: Configuration](https://www.raspberrypi.com/documentation/computers/configuration.html) | NetworkManager als Standard seit Bookworm; Desktop-/CLI-Kontext |
| [Debian Wiki: NetworkManager](https://wiki.debian.org/NetworkManager) | OpenVPN-Plugin, Import und Verbindungseditor |
| [Debian Wiki: OpenVPN](https://wiki.debian.org/OpenVPN) | Clientpaket, GUI-Integration und Kopplung an Basisverbindungen |
| [Debian Bookworm: network-manager-gnome](https://packages.debian.org/bookworm/network-manager-gnome) | Paketangebot für den grafischen NetworkManager-Zugang |
| [Debian Trixie: network-manager-gnome](https://packages.debian.org/trixie/network-manager-gnome) | Übergangspaket zu Applet und Verbindungseditor |
| [Debian Trixie: OpenVPN-GUI-Metadaten](https://appstream.debian.org/trixie/main/metainfo/network-manager-openvpn-gnome.html) | Grafisches OpenVPN-Plugin |
| [NetworkManager: connection](https://www.networkmanager.dev/docs/api/latest/settings-connection.html) | `secondaries`, fehlendes allgemeines VPN-Autoconnect und Profilberechtigungen |
| [NetworkManager: nmcli](https://www.networkmanager.dev/docs/api/latest/nmcli.html) | `--ask`, Aktivieren/Deaktivieren und Statusbefehle |
| [NetworkManager: Secret flag types](https://networkmanager.dev/docs/api/latest/secrets-flags.html) | Zuständigkeit und Verfügbarkeit von Secrets |

### 11.2 Relevante Repository-Verweise

- [Eigenständige Ausarbeitung zur VPN-Standortpolicy](2026-10-06_raspberry-pi-openvpn-autoverbindung-vertrauenswuerdige-netze.md).
- [Deployment-VM, Imagebuilder und Auto-Discovery](2026-10-05_deployment-vm-tbs-imagebuilder-und-auto-discovery.md) als benachbarte Archivdokumentation; deren historische Prüfungen werden hier nicht als neu ausgeführte Tests übernommen.
- [Aktuelle Gesamtroadmap am geprüften main-SHA](https://github.com/JanHG98/netcore-tetra/blob/9116c15d645458f99e236712b67a1ad970432791/ROADMAP.md).
- [Backend-Roadmap am geprüften main-SHA](https://github.com/JanHG98/netcore-tetra/blob/9116c15d645458f99e236712b67a1ad970432791/system-backend/roadmap.md), zusätzlich gelesen; die zentrale Gesamtroadmap liefert die übergeordnete Reihenfolge.
- Historischer Feature-SHA `bbf039729b9b05f8d623b11195ca24a124f68d16` und `PR #57`: Herkunftsangaben aus den gelesenen Dokumenten, keine neue Prüfung dieses historischen Codes oder PRs.
- Kein Implementierungscommit, Release oder neuer PR entstand im ursprünglichen GUI-Fachdialog.

### 11.3 Bereitgestellte PDFs und Bilder

Der Projektkontext nennt 25 projektweite ETSI-PDFs. Sie enthalten hier keine gelieferte `.ovpn`-Konfiguration oder VPN-Diagnose und waren für diese Betriebssystem-/Clientanleitung nicht erforderlich. Ihre Inhalte wurden deshalb für dieses Archiv nicht ausgewertet oder als OpenVPN-Quelle behauptet; die PDFs werden nicht unnötig dupliziert.

Das bereitgestellte Dateiinventar lautet:

```text
en_3003920308v010401p.pdf
en_30039209v010701p.pdf
ts_10081201v020205p.pdf
en_3003921201v010202p.pdf
en_3003920304v010301p.pdf
en_3003921117v010102p.pdf
en_3003921114v010101p.pdf
es_20081202v020401m.pdf
es_20081201v020205p.pdf
en_300812v020101p.pdf
en_3003921101v010201p.pdf
en_3003921006v010401p.pdf
en_3003921018v010301p.pdf
en_3003921216v010400a.pdf
en_30039201v010601p.pdf
ets_30039214e01v.pdf
en_30039207v030501p.pdf
en_30039401v030301p.pdf
en_3003920313v010201p.pdf
en_30039502v010303p.pdf
en_3003920303v010301p.pdf
en_30039205v020701p.pdf
en_3003920315v010500a.pdf
en_30039202v030801p.pdf
ETSI.pdf
```

Bilder und Screenshots der Einrichtung sind nicht erhalten. Bei späterer Verfügbarkeit können eindeutig zugehörige Originale ergänzt werden.

## 12. Fortsetzungsgrenzen

1. Die fachlichen Anforderungen und der vollständige Einrichtungsvorschlag sind geprüft; weitere frühere Festlegungen sind nicht belegt.
3. Es fehlt jede reale Pi-Ausgabe, Clientdatei, Verbindungsprüfung und Betriebsbestätigung für diese Einrichtung.
4. NetworkManager-GUI, einfache Profilverknüpfung, separate Dispatcher-Policy und historische Imagebuilder-Timer-Variante sind unterschiedliche Nachweis- und Implementierungsstände.
5. Die Repository-Prüfung betrifft die genannten aktuellen Snapshots und gezielt gelesenen Inhalte. Andere historische Refs und Zielgeräte wurden nicht vollständig geprüft.
5. Es waren keine zugehörigen Originalbilder verfügbar. Die 25 ETSI-PDFs wurden nur inventarisiert und nicht als VPN-Fachbelege verwendet.
7. Die Archivierung umfasst genau Dokumentation und Index auf `Archiving`; Installation, Policy-Implementierung, Roadmapänderungen außerhalb des Archivs und Live-Abnahme bleiben Folgearbeit.
