# Abschlussdokumentation: USB-Signatur, Hybrid-Control, BREW-Health, Backups und Receipt-Server

<!-- archive-chat-key: usb-ed25519-multi-stick-hybrid-control-brew-health-backup-receipts-apr2026 -->

## 1. Metadaten, Quellenumfang und Leseregeln

| Feld | Wert |
|---|---|
| Projekt | NetCore-Tetra |
| Thema | Signierte USB-Konfiguration; Mehrfach-Stick-Verwaltung; Netzwerkbackup; zentraler Control-Server; Netzwerk-vor-USB-Hybridmodell; BREW-Backend und Health; intelligenter Thermobondruck |
| Ursprünglicher Chattitel | Nicht im zugänglichen Verlauf enthalten; auch die gezielte Rücksuche lieferte keinen belastbaren Titel. Der Titel dieses Dokuments ist eine Themenbeschreibung. |
| Ursprünglicher Chatlink | Nicht verfügbar; kein Link rekonstruiert oder erfunden. |
| Historischer Zeitraum | Nutzerlogs vom 18. April 2026 und 27. April 2026; weitere Beispiele enthalten den 28. April. Nicht jede Nachricht besitzt einen sichtbaren Zeitstempel. Beispieldaten sind keine Datierung der gesamten Unterhaltung. |
| Tatsächliches Erstellungsdatum | 2026-10-03, Europe/Berlin |
| Repository | `JanHG98/netcore-tetra` |
| Ausschließlicher Schreibbranch | `Archiving` |
| Für den Inhaltsabgleich fixierter Commit | `2b08bababf8b72bd2cb82d780a7ba0e526e1f1a9` |
| Zugehöriger Root-Tree | `65913124d84998a680781cc9c1f89c0d93b31979` |
| Ergänzend gelesener Defaultbranch-Ref | `main` → `7137e0dd69877e1b604bf89148fd8b6b590c1a97` |
| Ablage | `Docs/archive/2026-10-03_usb-signatur-hybrid-control-brew-backup-und-receipt-server.md` |
| Umfang des Archivauftrags | Dieses Dokument und sein Eintrag in `Docs/archive/README.md`; keine Betriebs-, App-, Schlüssel-, Firmware-, Roadmap- oder Dienständerungen. |

**Wichtigste Übergabe:** Der signierte USB-Start wurde im damaligen Betrieb bestätigt. Ein veränderter Konfigurationsinhalt wurde tatsächlich abgelehnt. Daraus folgt weder Klonschutz für einen normalen USB-Stick noch die Abnahme eines vollständig abgesicherten Systems. Der später gelieferte Netzwerk-/USB-Entwurf war ein unvollständiger Einmalabruf, kein dauerhaft laufender Hybrid-Manager. Die Control-WebUI zeigt einen signierten **Sollzustand**, nicht bereits einen bewiesenen tatsächlichen Sendebetrieb. Der hinzugefügte BREW-Health-Endpunkt liefert im gezeigten Code selbst bei Problemen mit `nodes.json` weiterhin `status: ok`. Diese Grenzen müssen bei der Fortsetzung erhalten bleiben. (C01–C13)

### 1.1 Auswertungsgrundlage und Lücken

Ausgewertet wurden die in dieser Unterhaltung zugänglichen Nutzerangaben, sämtliche sichtbaren aufeinanderfolgenden Skriptfassungen und Korrekturen, Terminalausgaben, der Control-UI-Screenshot, das Original-Logo und die beiden generierten Bonentwürfe. Die Dateien waren in der Arbeitsumgebung tatsächlich vorhanden; Bildmaße und Prüfsummen wurden zusätzlich ermittelt. Die angehängten 25 PDF-Dateien wurden als Anlagenbestand mit Titel-/Versionsangaben und Seitenzahlen inventarisiert. Ihre teilweise mehrere Tausend Seiten umfassenden Inhalte wurden **nicht vollständig normativ ausgewertet**: Im sichtbaren Arbeitsverlauf wurden daraus keine konkreten Anforderungen an den USB-/Control-/Druckeraufbau hergeleitet.

Nicht vorhanden sind ein vollständiger Chat-Metadatenexport mit Originaltitel und URL, ein unveränderlicher Export der zuletzt tatsächlich auf allen drei Rechnern installierten Dateien, der komplette reale Inhalt der signierten `config.toml`, echte Schlüsseldateien, ein vollständig abgenommener Hybrid-Agent sowie Daten eines tatsächlich gekauften Druckers. Das ist kein Anlass, diese Informationen zu erfinden.

Der heutige Repository-Abgleich ist eine **gezielte Stichprobe**, keine Vollprüfung jedes Pfades, jeder historischen Commitversion oder jedes laufenden Containers. Große Baumantworten waren teilweise in der Werkzeugausgabe gekürzt. Der lokale Git-Clone scheiterte an der DNS-Auflösung der Arbeitsumgebung; lesender und schreibender GitHub-Connectorzugriff war dagegen verfügbar. Es bestand kein Zugriff auf das reale Verwaltungsnetz, die TBS, die beiden Anwendungsserver oder den Backup-LXC. Eine vorhandene spätere Android-/Control-Archivdatei wurde ausschließlich als separate Fortsetzungsquelle gelesen, nicht als Ersatz für fehlende Primärbelege dieses Chats. (R01–R11)

### 1.2 Statusbegriffe

| Kennzeichnung | Bedeutung |
|---|---|
| Idee | Besprochen, aber weder fertig spezifiziert noch umgesetzt nachgewiesen. |
| Beschlossen/geplant | Vom Nutzer verlangt oder ausdrücklich festgelegt; kein automatischer Implementierungsbeleg. |
| Implementiert im Chatcode | Konkreter Quelltext wurde geliefert bzw. vom Nutzer gepostet. Das sagt zunächst nichts über Git oder die tatsächlich installierte letzte Fassung aus. |
| Implementiert im Repository | Im angegebenen Commit konkret nachweisbarer Quelltext; Prüfumfang wird genannt. |
| Getestet | Konkreter Test mit Ergebnis; historische Nutzertests und isolierte Archivprüfungen bleiben getrennt. |
| Im Betrieb bestätigt | Nutzerbeobachtung oder Terminal-/Bildbeleg zeigt die damalige Funktion. Kein Nachweis dauerhafter Fehlerfreiheit oder des heutigen Deployments. |
| Unbestätigt / überholt | Fehlender Nachweis oder durch spätere Angaben ersetzter Vorschlag. |

**Quellenkonvention:** `Cxx` bezeichnet Beleggruppen aus diesem Chat, `Axx` Anlagen, `Rxx` die heutige Repositoryprüfung und `Exx` extern nachgelesene technische Grundlagen. Die Zuordnung steht in Abschnitt 17. Neue technische Bewertungen sind ausdrücklich als **Archivprüfung** bzw. **Vorschlag für die Fortsetzung** markiert und keine nachträglich erfundenen Entscheidungen des Nutzers.

## 2. Ergebnisübersicht

| Teilbereich | Historisch letzter belastbarer Stand | Grenze |
|---|---|---|
| Signierte USB-Konfiguration | Gültige Config startet; veränderte Config wird zurückgewiesen; Entfernen des Sticks beendet nach Nutzerbeobachtung das Senden | Kein Klon-, Mehrfach-Stick-, Race- oder umfassender Hardware-/Angriffstest |
| Restart-Verhalten | Nutzer bestätigt ausdrücklich `Restart=no` | Gilt für `bluestation.service`, nicht pauschal für alle späteren Dienste |
| Mehrere UUIDs und Menü | Array-/udev-Verwaltung als Code vorhanden; USB-Erkennung nach Korrektur wieder funktionsfähig; nach Wiederherstellung des richtigen Startskripts läuft die BS wieder | Kein vollständiger Test aller Menüfunktionen oder zweier gleichzeitiger erlaubter Sticks |
| Automount | Nutzer bestätigt deaktiviert; vorher konkreter RW-/RO-Mountkonflikt | Globales Maskieren von UDisks hatte Nebenwirkungen; sauberere gerätespezifische Lösung offen |
| Backup | Manueller Config-/Setup-Upload mit erfolgreichem Log; täglicher und wöchentlicher Cron-Eintrag laut Nutzer gesetzt | Vollimage-Erfolg, Cron-Ausführung, Rotation und Restore nicht nachgewiesen |
| Backupkapazität | Backup-LXC auf 70 GB erweitert | Keine reale Imagegröße oder dauerhafte Kapazitätsrechnung belegt |
| Control-Server | Flask-/Gunicorn-Umgebung nach Reparatur vorhanden; WebUI mit RUN/STOP und Downloadlinks sichtbar; Nutzer akzeptiert Mehrstations-UI | Letzte komplette Datei nicht aus dem laufenden Server zurückgelesen; keine vollständige Abnahme aller CRUD-/Lock-Aktionen |
| Config-Verteilung | Routen und Ablage vorgesehen; zunächst 404 wegen fehlender Dateien | Erfolgreiche spätere Ablage eines bytegenau passenden Config-/Signaturpaars nicht ausdrücklich bestätigt |
| BREW-Systemdienst / Health | Aiohttp-Quelle vollständig bekannt; Nutzer bestätigt funktionierende Health-Erweiterung | Keine Protokoll-/Readiness-/Autostart-/Lastabnahme daraus ableitbar |
| BREW-Lampe im Control-UI | Finale vollständige Pythonfassung auf `/health` und JSON-Auswertung umgestellt | Funktionsumfang der Lampe implementiert im Chatcode; kein eigener belegter Fehlerfalltest |
| Hybridbetrieb | Netzwerk zuerst, USB bei fehlender Netzwerkverfügbarkeit ausdrücklich gewünscht | Gelieferte Shellkombination ist nicht vollständig; kein laufender Polling-/Supervisorbetrieb bestätigt |
| Thermodrucker | Kurze Startinfo und wichtige Fehler mit Kommentar, Original-Logo, Log-QR und Auto-Cutter gewünscht; erst USB, später intelligenter Pi-Receiver/Netzwerk | Kein Gerätekauf, kein ESC/POS-Test, kein fertiger Receipt-Dienst oder verifizierter QR |

## 3. Ausgangslage und endgültige Anforderungen

### 3.1 Ausgangsproblem

Die Basisstation startete zunächst, sobald ein Stick mit bestimmter Dateisystem-UUID angeschlossen war. Das ursprüngliche Skript wartete drei Sekunden auf den Desktop-Automount, suchte das Gerät mit `blkid -U`, erwartete `/media/jan/<UUID>/config.toml` und führte danach `bluestation-bs` mit genau dieser Datei aus. udev startete und stoppte `bluestation.service` beim Hinzufügen bzw. Entfernen des Geräts. Die Frage war ausdrücklich, wie sich das gegen gefälschten Stick und manipulierte Konfiguration härten lässt. (C01)

Die ursprüngliche UUID war zunächst im Skript als Platzhalter dargestellt; die udev-Regeln nannten `6263-60C0`. Nach der Neuformatierung wurde **`D000-334F`** tatsächlich ausgegeben und anschließend als richtige UUID verwendet. Sie ist ein Selektor, kein geheimzuhaltender Schlüssel.

### 3.2 Verbindliche Nutzervorgaben

| Vorgabe | Bedeutung für eine Fortsetzung |
|---|---|
| Nur `config.toml` als fachliche Konfiguration | Andere Inhalte eines Sticks sollen ignoriert werden. Die später ergänzte `.sig` ist notwendige Prüfmetadatei, kein weiterer frei auszuwertender Inhalt. |
| TOML fachlich nicht umbauen | Zusätzliche Geräte-/Versionsinformationen nur als Kommentare; keine willkürlichen neuen TOML-Schlüssel in den bestehenden Parser einführen. |
| Hauptrechner Windows 11, kein vorausgesetztes OpenSSL | Erzeugung und Signieren mit Python; umgesetzt wurde `cryptography`, nicht PyNaCl. |
| Manuelle Übertragung per Strg+C / Strg+V | Keine SCP-/CP-Dateiübertragung zwischen Windows und Linux in den Anleitungen voraussetzen. Spätere SCP-Beispiele der Assistenz widersprachen dieser Vorgabe. |
| Ganze Dateien und konkrete Pfade | Wiederholt vollständige `.sh` bzw. `.py` statt einzusetzender Fragmente verlangt. Ausführungsbenutzer, Arbeitsverzeichnis und venv müssen eindeutig sein. |
| Gültiger Stick startet, Entfernen stoppt | Ursprüngliches und getestetes USB-Verhalten; im Hybridmodell muss die Quellenzuständigkeit neu sauber definiert werden. |
| Weitere autorisierte Sticks | Beispielsweise für einen weiteren Bediener; komfortabel über ein mit `sudo` gestartetes Menü verwalten. |
| Menüumfang | Registrieren, registrierte Sticks anzeigen, UUID entfernen, aufräumen/synchronisieren und aktuell angeschlossene USB-Geräte anzeigen. |
| Hybrid-Priorität | **Netzwerk zuerst; wenn nicht vorhanden, USB.** Der früher angebotene USB-Vorrang wurde dadurch ersetzt. |
| Zentrales UI | Stationen erstellen, umbenennen/bearbeiten, löschen, sperren/entsperren sowie RUN/STOP je Station; Dateien direkt über die WebUI erreichbar machen. |
| Server zuerst | Zunächst serverseitige Funktionen und Backend-Statusanzeige fertigstellen, bevor die TBS-Integration fortgesetzt wird. |
| Physischer Eventausdruck | Wenige relevante Bons, insbesondere Start und Fehler; keine Ausgabe jedes Logs, Frames oder jeder Teilnehmeranmeldung. |
| Drucker und Gestaltung | Auto-Cutter; zunächst USB, langfristig Netzwerk bzw. intelligenter Pi-Receiver; echtes Original-Logo statt der abgelehnten ASCII-Annäherungen; QR zur passenden Logseite; kurze humorvolle Kommentare. |

### 3.3 Schlüsseltrennung

Die später bevorzugte Architektur trennt zwei Ed25519-Schlüsselpaare:

- **Config-Key:** privater Schlüssel auf dem Windows-Administrationsrechner; signiert `config.toml`. Auf der TBS liegt nur der zugehörige öffentliche Schlüssel. Der Control-Server verteilt das bereits signierte Paar.
- **State-Key:** privater Schlüssel auf dem Control-Server; signiert RUN/STOP-/Lock-Zustände. Auf der TBS liegt später dessen separater öffentlicher Schlüssel.

Das ist als Architektur festgelegt bzw. im Code vorgesehen. Die tatsächlichen Schlüsselwerte wurden nicht mitgeteilt und werden hier nicht archiviert. Der State-Key darf nicht als harmlos bewertet werden: Er autorisiert Betriebszustände und ist ebenfalls sicherheitsrelevant. Die frühere optionale Idee, den Config-Private-Key auf den Server zu übertragen, wurde nicht als ausgeführter Schritt bestätigt. (C09–C11)

## 4. Historische Architektur und technische Bestandsliste

### 4.1 Komponenten und Datenfluss

```text
Windows-Administrationsrechner
    erzeugt Config-Schlüsselpaar
    signiert die exakten Bytes von config.toml
        |
        +--> USB-Stick: config.toml + config.toml.sig
        |
        +--> Control-Server: Verteilung desselben signierten Dateipaars

Control-LXC CT-H-DEV-04
    Flask + Gunicorn, HTTP :8080
    nodes.json: verwaltete Basisstationen
    State-Key: signiert gewünschten Zustand
    BREW-Health-Abfrage --> 10.0.1.163:8081/health
        |
        | geplanter Pull über LAN/VPN
        v
TBS SRV-M-RPi-TBS01
    Config- und später State-Prüfung
    geplanter dauerhafter Hybrid-Manager
    bluestation.service --> bluestation-bs
        |
        +--> Backup per SSH-Datenstrom --> Backup-LXC 10.0.1.161
        |
        +--> später ausgewählte Events --> intelligenter Receipt-Pi
                                              |
                                              +--> USB-Thermodrucker
                                              +--> später Netzwerkdrucker

Separater BREW-Server 10.0.1.163
    /var/opt/brew-server/app.py, aiohttp
    :8080 HTTP-Bootstrap + BREW-WebSocket + /health
    :8081 ISSI-WebUI + /health + /api/status
```

Nicht alle Pfeile sind implementiert: Insbesondere kontinuierliches Hybrid-Polling, gesicherte Rückmeldung des tatsächlichen Senderzustands und Receipt-Events wurden nur entworfen. Die Control-LXC-IP wurde in diesem Chat nicht belastbar genannt; Platzhalter wie `10.0.1.XXX` sind keine realen Konfigurationswerte.

### 4.2 Rechner und Ports

| System | Historisch belegte Identität | Port / Protokoll |
|---|---|---|
| Basisstation | Prompt `jan@SRV-M-RPi-TBS01`; Linux auf SD/eMMC-Blockgerät `mmcblk0` | Lokale systemd-/udev-Steuerung; VPN vorgesehen, dessen Produkt/Port hier nicht eingerichtet oder nachgewiesen |
| Control-Server | LXC `CT-H-DEV-04`; Dienstbenutzer `netcore` | TCP 8080, HTTP, Flask hinter Gunicorn |
| BREW-Backend | `10.0.1.163`; Hostname in diesem Chat nicht sicher zugeordnet | TCP 8080 für HTTP/WebSocket, TCP 8081 für WebUI/Health |
| Backupserver | Lokaler LXC `10.0.1.161`, auch für OPNsense-Backups verwendet; 70-GB-Datenträger laut Nutzer | SSH/SFTP; historischer fertiger Backupcode nutzt SSH-Remote-Befehle, nicht das SFTP-Subsystem |
| Receipt-Server | Künftiger Pi, kein konkretes Gerät/IP festgelegt | HTTP-Event-API geplant; ESC/POS per USB, später ggf. TCP 9100, jeweils druckermodellabhängig |

**Korrektur der früheren Zuordnung:** Aus dem Kontonamen und dem bereits vorhandenen OPNsense-Backup folgt nicht, dass `10.0.1.161` die OPNsense-Firewall selbst ist. Die spätere Angabe eines LXC mit 70 GB spricht für einen separaten Backupdienst. Die genaue Distribution dieses LXC wurde nicht nachgewiesen.

### 4.3 Pfade auf der Basisstation

| Pfad | Bedeutung / Status |
|---|---|
| `/home/jan/netcore-tetra/target/release/bluestation-bs` | Im ursprünglichen und späteren USB-Startskript verwendetes Binary; tatsächlicher Build-Commit unbekannt |
| `/home/jan/netcore-tetra` | Historisches Arbeitsverzeichnis vor dem Senderstart |
| `/usr/local/lib/bluestation/import_and_start.sh` | Zunächst USB-Startskript, später als Hybrid-Wrapper vorgeschlagen; Versionsverwechslung vermeiden |
| `/usr/local/lib/bluestation/import_and_start_usb.sh` | Vorgeschlagene separate USB-Fassung für den Hybridumbau; tatsächliches korrektes Anlegen nicht bestätigt |
| `/usr/local/lib/bluestation/verify_config.py` | Gelieferter Python-Verifier für Configsignatur, TOML, Geräte-ID und Mindestversion |
| `/usr/local/lib/bluestation/venv/bin/python` | Historisch vorgesehener Interpreter mit `cryptography`; `tomllib` erfordert Python 3.11 oder neuer |
| `/etc/bluestation/signing_pubkey.pem` | Öffentlicher Config-Verifikationsschlüssel; kein Schlüsselmaterial im Archiv |
| `/etc/bluestation/device_id` | Erwartete Geräte-ID; Beispiel bzw. Anfangszuordnung `bs01` |
| `/var/lib/bluestation/last_version` | In diesem Chat ausdrücklich gezeigter Versionsstand als reine Dezimalzahl, kein JSON |
| `/run/bluestation-usb` | Vom Dienst verwendeter USB-Mountpunkt |
| `/run/bluestation/config.toml` | Lokale übernommene Config mit Modus 0600 |
| `/etc/udev/rules.d/99-bluestation.rules` | Start-/Stop-Ereignisse für freigegebene UUIDs |
| `/etc/systemd/system/bluestation.service` | Senderstart über Importskript; zuletzt `Restart=no` bestätigt |
| `/usr/local/sbin/register_blst_usb.sh` | Frühe Einzweck-Registrierung; später vom Menüansatz ersetzt |
| `/usr/local/sbin/bluestation-usb-admin.sh` | USB-Menü, darf nicht im Startskript stehen |
| `/usr/local/sbin/backup-bluestation.sh` | Manueller Upload erfolgreich; für Root-Cron vorgesehen |
| `/var/log/bluestation-backup.log` | Cron-Logziel |
| `/home/jan/.ssh/opnsense_backup_key` | Im Code referenzierter privater SSH-Dateipfad; Inhalt nicht übernommen |
| `/opt/bluestation/net/fetch_and_run.sh` | Unvollständiger Netzwerkabruf-Entwurf |
| `/opt/bluestation/keys/state_public_key.pem` | Vorgesehener öffentlicher State-Schlüssel |
| `/opt/bluestation/runtime/` | Ziel für die vier heruntergeladenen Dateien im Netzwerkentwurf; anders als `/run` persistent |

Die SSH-Dateibezeichnung wird zur Zuordnung dokumentiert; Anmeldename und Remote-Home in Befehlsvorlagen werden zusätzlich als `<BACKUP_USER>` pseudonymisiert. Keine Passwörter, Tokens, privaten PEM-Inhalte oder verwendbaren Signaturen werden übertragen.

### 4.4 Pfade und Dienste auf den Servern

```text
/opt/netcore-tetra-control/
    app.py
    venv/bin/python
    venv/bin/gunicorn
    generate_state_keys.py
    sign_state.py
    nodes.json
    keys/state_private_key.pem      # Inhalt ausgeschlossen
    keys/state_public_key.pem       # Inhalt ebenfalls nicht benötigt
    data/<node_id>/state.json
    data/<node_id>/state.json.sig
    static/configs/<node_id>/config.toml
    static/configs/<node_id>/config.toml.sig

/etc/systemd/system/netcore-tetra-control.service

/var/opt/brew-server/app.py
/var/opt/brew-server/nodes.json     # bei passendem WorkingDirectory und Defaultwert
/etc/systemd/system/brew-server.service

Backupziel:
/home/<BACKUP_USER>/backups/netcore-tetra/
```

Die beiden `nodes.json` sind **nicht austauschbar**: Control speichert Stationsmetadaten nach Node-ID; BREW speichert eine Map aus ISSI-Strings und Passwortfeldern.

## 5. USB-Konfiguration und Signaturverfahren

### 5.1 Ausgearbeiteter Vertrag

Auf dem Stick werden genau diese Dateien erwartet:

```text
config.toml
config.toml.sig
```

Zusätzliche Dateien, etwa Windows-`System Volume Information`, sind irrelevant. Die Config enthält als Kommentare beispielsweise:

```toml
# TBS_ID=bs01
# CONFIG_VERSION=1
# ID der TBS
```

Diese Kommentare gehören zu den signierten Bytes. Der gelieferte Verifier sucht die Metadaten mit regulären Ausdrücken im gesamten dekodierten Text. Die tatsächlichen Ausdrücke erlauben Leerraum um `#`, Schlüssel und `=`; die spätere Behauptung, keinerlei Leerzeichen seien erlaubt, war daher falsch. Bei mehrfach vorhandenen passenden Kommentaren wird jeweils der erste Treffer verwendet; doppelte oder widersprüchliche Metadaten werden nicht explizit beanstandet.

**Signaturformat:** Ed25519 über den unveränderten Datei-Byteinhalt, nicht über normalisiertes TOML. Private PEM-Serialisierung als PKCS8 mit `NoEncryption()`, Public Key als SubjectPublicKeyInfo. Die `.sig` enthält Base64-ASCII und einen abschließenden Zeilenumbruch. Eine 64-Byte-Ed25519-Signatur ergibt 88 Base64-Zeichen bzw. 89 Bytes mit LF. Die vom Nutzer gelistete Datei hatte genau 89 Bytes; das passt zum Format, ersetzt aber keine kryptografische Prüfung. (C02, C04; E01)

**Folge für Copy/Paste:** Änderungen an Kommentaren, Einrückung, Zeilenenden, BOM oder abschließendem Zeilenumbruch können die Signatur ungültig machen. Eine in `nano` neu angelegte, optisch gleiche Datei ist nicht automatisch bytegleich zur auf Windows signierten Datei. Maßgeblich ist immer das tatsächlich verteilte Dateipaar. (Archivprüfung, T-A02)

### 5.2 Windows-Werkzeuge

Historisch vorgesehener Arbeitsordner: `C:\bluestation-signing`. Dort wurden `generate_keys.py`, `sign_config.py`, eine `.venv` sowie Config-Private-/Public-Keydateien vorgesehen. `sign_config.py` nimmt einen Configpfad als Argument, liest den Private Key aus dem aktuellen Arbeitsverzeichnis und legt daneben `config.toml.sig` an. Signieren und Verifizieren verwenden die Pythonbibliothek `cryptography`. Die anfänglich genannten OpenSSL-, minisign-, signify- und PyNaCl-Varianten wurden nicht zum endgültigen in diesem Chat verwendeten Verfahren.

Relevante historische Befehle, ohne Nachweis jedes einzelnen Installationsschritts:

```powershell
py --version
py -m venv .venv
.\.venv\Scripts\Activate.ps1
python -m pip install --upgrade pip
python -m pip install cryptography
python .\generate_keys.py
python .\sign_config.py "C:\PFAD\config.toml"
```

Die Verwendung einer Ausführungsrichtlinien-Ausnahme für das Aktivieren wurde nur als Ausweichmöglichkeit vorgeschlagen. Für spätere Anleitungen ist ein expliziter Aufruf von `.venv\Scripts\python.exe` weniger fehleranfällig als eine stillschweigend angenommene Aktivierung. Das ist eine neue Dokumentationsempfehlung, keine bereits abgenommene Änderung. (E02, E03)

**Wichtiger Defekt der Generatorbeispiele:** Wiederholtes Ausführen überschreibt vorhandene Schlüsseldateien ohne Bestätigung. Eine erneute Erzeugung ist keine Reparatur einer fehlenden Pythonbibliothek und darf nicht versehentlich die bestehende Vertrauenskette ersetzen. Ein künftiger Generator muss vorhandene Schlüssel erkennen, abbrechen oder eine ausdrücklich bestätigte Rotation ausführen.

### 5.3 Verhalten des gelieferten `verify_config.py`

Der historische Verifier:

1. Erwartet zwei Pfadargumente und prüft die Existenz beider Dateien.
2. Begrenzt die Config auf mehr als 0 und höchstens 1 MiB, die Signaturdatei auf mehr als 0 und höchstens 4096 Bytes.
3. Liest die Config als Bytes und die Signatur als UTF-8-Text, entfernt außenliegenden Leerraum und dekodiert Base64 mit `validate=True`.
4. Lädt ausschließlich `/etc/bluestation/signing_pubkey.pem` und verlangt einen `Ed25519PublicKey`.
5. Prüft die Signatur über die gelesenen Configbytes.
6. Dekodiert UTF-8 streng und prüft die TOML-Syntax mit `tomllib.loads`.
7. Extrahiert `TBS_ID` und `CONFIG_VERSION` aus Kommentaren.
8. Vergleicht die Geräte-ID mit `/etc/bluestation/device_id`.
9. Liest `/var/lib/bluestation/last_version` als Dezimalzahl; fehlt die Datei, gilt 0; bei ungültigem Inhalt erfolgt Abbruch.
10. Lehnt `config_version < last_version` ab und gibt bei Erfolg zwei maschinenlesbare Zeilen aus.

```text
OK:TBS_ID=bs01
OK:CONFIG_VERSION=1
```

Die Version wird vom Verifier **nicht geschrieben**. Das tut im USB-Modell erst das nachgeschaltete Shellskript. Gleiche Versionen sind zulässig, damit derselbe Stick erneut starten kann. Unterschiedliche, gültig signierte Configs mit identischer Version werden mangels gespeicherten Hashes ebenfalls nicht voneinander unterschieden. Das Verfahren ist daher ein einfacher Mindestversionsschutz, kein vollständiger Schutz gegen jede Wiederholung. Gerätebindungs- und Rückstufungstests wurden vorgeschlagen, aber nicht mit eigenen Ergebnissen bestätigt.

### 5.4 Letzter wieder funktionierender USB-Startablauf

Die letzte vor dem Hybridvorschlag wiederhergestellte Fassung hatte ein Array:

```bash
AUTHORIZED_UUIDS=(
    "D000-334F"
)
```

Sie suchte die Einträge in Arrayreihenfolge mit `blkid -U` ab, nahm den ersten vorhandenen Treffer, legte `/run/bluestation-usb` und `/run/bluestation` an, löste einen vorhandenen Mount am eigenen Mountpunkt und mountete mit:

```bash
mount -o ro,noexec,nodev,nosuid "$DEVICE" /run/bluestation-usb
```

Für `config.toml` und `.sig` wurde je `-f` und zusätzlich `! -L` verlangt. Danach wurde der Verifier mit dem **absoluten venv-Interpreter** ausgeführt. Das Skript extrahierte die Version mit `awk`, kopierte die Config mittels `install -m 600` nach `/run/bluestation/config.toml`, schrieb die Versionszahl nach `/var/lib/bluestation/last_version`, setzte dort ebenfalls 0600, wechselte nach `/home/jan/netcore-tetra` und ersetzte sich per `exec` durch das Binary.

Kern der historischen Übernahmereihenfolge, zur Quellenanalyse und **nicht als erneut freigegebener Härtungscode**:

```bash
VERIFY_OUTPUT="$("$VENV_PY" "$VERIFY_SCRIPT" "$CONFIG_PATH" "$SIG_PATH")"
CONFIG_VERSION="$(echo "$VERIFY_OUTPUT" | awk -F= '/^OK:CONFIG_VERSION=/{print $2}')"
install -m 600 "$CONFIG_PATH" "$LOCAL_CONFIG"
echo "$CONFIG_VERSION" > "$LOCAL_VERSION_FILE"
chmod 600 "$LOCAL_VERSION_FILE"
cd "$WORKDIR"
exec "$BINARY" "$LOCAL_CONFIG"
```

**Archivprüfung — verbleibende Risiken:**

- Verifikation und Kopie lesen den USB-Inhalt getrennt. Ein manipulierbares Medium kann zwischen beiden Zugriffen andere Bytes liefern. Sicherer wäre, zunächst begrenzt in einen geschützten lokalen Snapshot zu lesen, genau diesen zu prüfen und ausschließlich ihn zu aktivieren. Der bisherige Negativtest beweist nicht die Abwesenheit dieses Prüf-/Nutzungsrennens.
- Der `EXIT`-Trap sollte unmounten, wird nach einem erfolgreichen `exec` aber nicht beim späteren Ende des Binaries ausgeführt. Dieser Shellmechanismus wurde im Archivlauf isoliert bestätigt. Mount-Lebenszyklus gehört in einen ausdrücklich verantwortlichen Manager bzw. systemd-Mount-/Cleanup-Ablauf. (E04, T-A03)
- `umount ... || true` verdeckt einen fehlgeschlagenen Unmount. Das Skript macht dann dennoch mit dem nächsten Mountversuch weiter.
- `last_version` wird nicht atomar geschrieben und bereits vor dem erfolgreichen Senderbetrieb erhöht. Ein beschädigter Versionsstand oder eine gültig signierte, aber betrieblich unbrauchbare höhere Version kann den späteren Start blockieren.
- Das rootgestartete Binary liegt unter einem Benutzer-Homepfad. Wer diesen Quell-/Buildpfad schreibend kontrolliert, kann den ausführbaren Programmcode ersetzen. Die vollständige Vertrauenskette ist nicht allein durch den öffentlichen Schlüssel geschützt.
- Die Mountoptionen begrenzen bestimmte Dateisystemfunktionen; sie authentisieren weder die USB-Hardware noch schließen sie alle USB-/Treiberangriffe aus.

### 5.5 Dienst und udev

Der zuletzt ausdrücklich bestätigte Dienstparameter ist `Restart=no`. Die frühere Endlosschleife mit `Restart=on-failure` und drei Sekunden Abstand wurde damit bewusst beendet. Historisch gezeigte Unit:

```ini
[Unit]
Description=BlueStation mit geprüfter USB-Konfiguration
After=local-fs.target
Wants=local-fs.target

[Service]
Type=simple
ExecStart=/usr/local/lib/bluestation/import_and_start.sh
Restart=no
User=root
Group=root

[Install]
WantedBy=multi-user.target
```

`systemctl enable` war zuvor angewiesen worden; sein aktuell geladener Zustand wurde nicht separat zurückgelesen. Es besteht dadurch grundsätzlich ein Boot-Startpfad zusätzlich zu udev. Für einen späteren Hybrid-Manager muss ausdrücklich festgelegt werden, welcher Dienst dauerhaft läuft und welcher nur den Sender trägt.

Die mit der neuen UUID gezeigten udev-Regeln verwenden für `add` `TAG+="systemd"` und `SYSTEMD_WANTS`, für `remove` einen direkten Stop-Aufruf:

```udev
ACTION=="add", SUBSYSTEM=="block", ENV{DEVTYPE}=="partition", ENV{ID_FS_UUID}=="D000-334F", TAG+="systemd", ENV{SYSTEMD_WANTS}="bluestation.service"
ACTION=="remove", SUBSYSTEM=="block", ENV{DEVTYPE}=="partition", ENV{ID_FS_UUID}=="D000-334F", RUN+="/bin/systemctl stop bluestation.service"
```

Das Entfernen irgendeines registrierten Sticks stoppt bei diesen Regeln denselben globalen Dienst, unabhängig davon, von welchem Stick oder vom Netzwerk die laufende Konfiguration stammt. Diese Einschränkung ist für Mehrfach-Sticks und Hybridbetrieb offen. Ein ereignisspezifischer Manager muss Quelle und tatsächliches Gerät zusammenhalten; bloß weitere UUID-Regeln zu kopieren löst das nicht.

Das Nachladen von Regeln ist nicht identisch mit einem erneuten Einsteckereignis: `SYSTEMD_WANTS` wird bei der Aktivierung des Geräts ausgewertet, nicht beliebig erneut bei jeder Regeländerung an einem bereits aktiven Gerät. Das globale `udevadm trigger` aus den alten Skripten ist deshalb kein universeller erfolgreicher Starttest. (E05)

## 6. USB-Fehlersuche und Mehrfach-Stick-Menü

### 6.1 Tatsächlich beobachtete Reihenfolge

Der Nutzer zeigte zunächst `blkid` ohne USB-Eintrag. Später meldete `dmesg` einen rund 4-GB-Stick als `sda`, während Fehler beim Unmount eines `sdb` auftraten. Die Assistenz erklärte daraus vorschnell einen defekten Stick bzw. eine zerstörte Partitionierung und behauptete zusätzlich eine nie vom Nutzer genannte Ventoy-/Rufus-Vorgeschichte. Diese Diagnose und Vorgeschichte waren **nicht belegt**.

Der Nutzer führte anschließend `fdisk /dev/sda` aus. Das Programm warnte ausdrücklich, dass der Datenträger benutzt wurde. Trotzdem wurde eine neue DOS-Partitionstabelle und eine Partition geschrieben. `mkfs.vfat -F 32 /dev/sda1` scheiterte zunächst an `Device or resource busy`. Die darauf folgenden Vorschläge `partprobe`, `partx`, `wipefs` und wiederholte Formatierung sind nicht sämtlich als ausgeführt dokumentiert. Schließlich lieferte der Nutzer:

```text
/dev/sda1: UUID="D000-334F" BLOCK_SIZE="512" TYPE="vfat" PARTUUID="0d6e3151-01"
```

Damit war das Dateisystem wieder identifizierbar. Dies belegt weder einen vorangegangenen Hardwaredefekt noch vollständige Medienintegrität. Insbesondere ist das Fehlen einer Partitionstabelle allein kein Defektnachweis; ein Dateisystem kann auch direkt auf einem Gesamtgerät liegen. Die vorherigen Lösch-/Formatierbefehle sind **kein künftig blind zu wiederholender Reparaturablauf**.

### 6.2 Die Dateien waren vorhanden; der Mountpunkt war falsch

Eine spätere Ausgabe unter `/mnt` zeigte:

```text
config.toml          7064 Bytes
config.toml.sig        89 Bytes
System Volume Information/
```

Zugleich war `/run/bluestation-usb` laut `umount` nicht gemountet. Der anschließende gehärtete Mount scheiterte, weil `/dev/sda1` bereits unter `/media/jan/D000-334F` eingebunden war:

```text
already mounted on /media/jan/D000-334F
sda1: Can't mount, would change RO state
```

**Belegte Ursache in diesem Schritt:** bereits vorhandener Desktop-Mount und Konflikt beim Read-only-Zustand. Die vorherige pauschale Erklärung, Windows habe die Dateien mit 99 Prozent Wahrscheinlichkeit nicht geschrieben oder falsch benannt, wurde nicht belegt und wird nicht übernommen.

Der Nutzer entschied sich für das Deaktivieren des Automounts. Vorgeschlagen waren `systemctl mask udisks2.service` sowie Stop/Unmount. Das funktionierte als Teil des USB-Ablaufs, war aber eine globale Änderung. Später erkannte der SD Card Copier die Backupkarte nicht. Ob dessen konkrete Störung durch UDisks-Abschaltung, Mounts oder andere Faktoren verursacht wurde, wurde nicht abschließend diagnostiziert. Die Aussagen, der Copier erkenne ausschließlich bestimmte Removable-Geräte und `dd` sei immer besser, waren zu pauschal.

### 6.3 UUID-Wechsel und Restart-Schleife

Der Dienstlog zeigte wiederholt `FEHLER: Autorisierter Stick nicht gefunden.`, steigende Restart-Zähler bis 148 und schließlich `Start request repeated too quickly`. Ursache für die Suche war nach dem gezeigten Ablauf die noch alte UUID. Vorgeschlagen und durch späteren Betrieb mittelbar bestätigt wurde die Aktualisierung **sowohl im Startskript als auch in udev** auf `D000-334F` sowie Stop/Reset des Dienstes.

Relevante historische Reparaturbefehle:

```bash
sudo systemctl stop bluestation.service
sudo systemctl reset-failed bluestation.service
sudo nano /usr/local/lib/bluestation/import_and_start.sh
sudo nano /etc/udev/rules.d/99-bluestation.rules
sudo udevadm control --reload-rules
sudo systemctl daemon-reload
```

Die damaligen Ausgaben wurden nicht bei jedem Einzelbefehl gepostet; die nachfolgende Nutzerbestätigung ist der Betriebsnachweis des Gesamtwegs. Das zwischenzeitlich angebotene Beispiel mit `StartLimitIntervalSec` und `StartLimitBurst` im Abschnitt `[Service]` war falsch eingeordnet; diese Parameter gehören in `[Unit]`. Die abschließende Wahl `Restart=no` ersetzt diesen Zwischenvorschlag. (E06)

### 6.4 Menüumfang und Datenhaltung

Das endgültig angeforderte Menü enthielt:

| Auswahl | Funktion |
|---|---|
| 1 | Aktuell eingesteckten Stick registrieren |
| 2 | Registrierte UUIDs und passende udev-Regeln anzeigen |
| 3 | UUID per nummerierter Auswahl und Bestätigung entfernen |
| 4 | udev-Regeln aufräumen |
| 5 | Aktuell angeschlossene USB-Sticks/Partitionen anzeigen |
| 0 | Beenden |

Das Menü verlangte root, las und änderte den Textblock `AUTHORIZED_UUIDS=(...)` im Startskript mit eingebettetem Python, ergänzte für neue UUIDs Add-/Remove-Regeln und lud udev neu. Es signierte keine Config und verteilte keine Schlüssel. `regstick` bzw. später `bsusb` wurden als optionale Shell-Aliase vorgeschlagen; die tatsächliche Installation dieser Aliase ist nicht belegt.

Zunächst verwendete die USB-Erkennung:

```bash
lsblk -rpno NAME,TRAN,TYPE | awk '$2=="usb" && $3=="part" {print $1}'
```

Das setzt voraus, dass `TRAN=usb` direkt in der Partitionszeile erscheint. Die korrigierte Variante ermittelte zuerst USB-**Disk**-Geräte mit `lsblk -dpno NAME,TRAN,TYPE`, danach deren `part`-Kinder. Der Nutzer bestätigte: „jetzt findet er ihn wieder“. Der tatsächliche Ursache-Nachweis mittels gepostetem `lsblk` fehlt, aber der Erfolg nach der Änderung ist belegt.

Danach startete die BS nicht: Der Nutzer stellte selbst fest, das Menü versehentlich in `import_and_start.sh` gespeichert zu haben. Nach der erneuten Ausgabe und Wiederherstellung des Startskripts meldete er **„geht wieder“**. Diese konkrete Fehlerursache ersetzt die zuvor spekulierten Funk-/Config-/Binary-Ursachen. (C06)

### 6.5 Grenzen des Menücodes — Archivprüfung

Die zugesagte vollständige Synchronisierung ist im gezeigten Code nur teilweise umgesetzt. Menüpunkt 4 entfernt Regeln für nicht mehr im Array enthaltene UUIDs; er entfernt jedoch nicht zuverlässig Dubletten, normalisiert nicht das Array und ergänzt nicht alle fehlenden Sollregeln. Die Dateibearbeitung ist nicht transaktional, hat kein Backup, keine Schreibsperre und keine abschließende `bash -n`-Validierung.

Weitere offene Punkte:

- Es wird genau eine USB-Partition vorausgesetzt. Mehrere Partitionen, gleichzeitig eingesteckte Sticks und ein Dateisystem direkt auf dem USB-Disk-Gerät sind nicht vollständig abgedeckt.
- Entfernen von Regeln geschieht teils per Teilstringvergleich, nicht durch eine robuste strukturierte Zuordnung.
- Ein Fehler aus einer Menüfunktion kann unter `set -e` das gesamte Menü beenden, statt zur Auswahl zurückzukehren.
- Die UUID wird nicht als kryptografisches Merkmal authentisiert; doppelte UUIDs sind nicht eindeutig einem physischen Gerät zuzuordnen.
- Das Löschen einer UUID stoppt eine bereits laufende Station nicht ausdrücklich. Wird zugleich die zugehörige Remove-Regel entfernt, ist auch das spätere Ausstecken als Stop-Auslöser nicht mehr verlässlich vorhanden.

**Vorschlag für die Fortsetzung:** eine zentrale, rootgeschützte Registry als einzige Datenquelle; atomare Speicherung mit Backup/Lock; verwalteter udev-Block; echte Auswahl unter mehreren Geräten; getrennte Zuständigkeit des Startmanagers. Das ist eine neue Ableitung, keine bereits durchgeführte Migration.

## 7. Backups zum lokalen Netzwerkserver

### 7.1 Gewünschter und tatsächlich benutzter Weg

Nach den Schwierigkeiten mit dem SD Card Copier wollte der Nutzer die Basisstation in den bereits für OPNsense verwendeten Backupbereich sichern. Zielunterordner: **`netcore-tetra`**. Ein privater SSH-Schlüssel war vorhanden; dessen Inhalt ist nicht Bestandteil des Chats oder dieses Archivs.

Zuerst schlug der interaktive SSH-Zugang mit `This account is currently not available` fehl. Die Assistenz vermutete eine `nologin`-Shell und schlug eine Shelländerung vor. Welcher konkrete serverseitige Befehl ausgeführt wurde, ist nicht sichtbar. Der Nutzer bestätigte anschließend, dass ein SSH-Aufruf mit einem einfachen Remote-`echo` funktioniert. Damit ist **Remote-Command-Ausführung** bestätigt, nicht eine angebliche zeitliche Blockierung des Kontos durch OPNsense.

Die fertige Backuplösung überträgt einen Standardausgabestrom mit `ssh ... "cat > ..."`. Das ist **kein SFTP-Upload**, obwohl derselbe Server über SFTP angesprochen werden kann. Ein reines SFTP-Konto ohne Remote-Shell würde diesen Code nicht ausführen können. Der frühere Vorschlag `tar ... | sftp user@host:/datei` ist kein gültiger entsprechender Streaming-Upload und wurde ersetzt.

### 7.2 Final gelieferter Backupskript-Vertrag

Datei: `/usr/local/sbin/backup-bluestation.sh`.

Konfiguration:

```bash
SERVER="<BACKUP_USER>@10.0.1.161"
SSH_KEY="/home/jan/.ssh/opnsense_backup_key"
TARGET_DIR="/home/<BACKUP_USER>/backups/netcore-tetra"
DATE="$(date +%Y-%m-%d_%H-%M-%S)"
HOSTNAME_SHORT="$(hostname -s)"
DO_FULL_BACKUP="${DO_FULL_BACKUP:-false}"
```

Pflichtpfade in der gelieferten Liste:

```text
/usr/local/lib/bluestation
/etc/bluestation
/etc/udev/rules.d/99-bluestation.rules
/var/lib/bluestation
/etc/systemd/system/bluestation.service
```

Optionale Pfade:

```text
/usr/local/sbin/bluestation-usb-admin.sh
/usr/local/sbin/register_blst_usb.sh
```

Das Skript prüft die Werkzeuge `ssh`, `tar`, `gzip`, `dd` und `hostname` sowie die Lesbarkeit der SSH-Keydatei. Es benutzt `BatchMode=yes`, `StrictHostKeyChecking=accept-new` und `ConnectTimeout=10`, testet die SSH-Verbindung, legt den Remote-Unterordner an, erstellt mit `mktemp` eine Dateiliste und überträgt ein `tar.gz`. Bei `DO_FULL_BACKUP=true` folgt zusätzlich der Datenstrom von `/dev/mmcblk0` durch `gzip` über SSH. Es gibt keine interaktive Frage mehr. Die Shell verwendet `set -euo pipefail`.

Dateinamen:

```text
<HOST>-config-backup-YYYY-MM-DD_HH-MM-SS.tar.gz
<HOST>-full-image-YYYY-MM-DD_HH-MM-SS.img.gz
```

Der gelieferte `cleanup()`-Trap ist leer; die temporäre Liste wird nur nach erfolgreicher Übertragung explizit entfernt. Fehlende als Pflichtpfade bezeichnete Dateien werden mit Warnung übersprungen, nicht als Abbruch behandelt.

### 7.3 Tatsächlich bestätigter Upload

Vom Nutzer geposteter Ablauf am 18. April 2026:

```text
22:25:20 Start für SRV-M-RPi-TBS01
22:25:20 SSH-Verbindung wird geprüft
22:25:21 Zielordner wird sichergestellt
22:25:21 Config-/Setup-Backup wird übertragen
22:25:23 Config-/Setup-Backup erfolgreich übertragen
22:25:23 Full-Image-Backup deaktiviert
22:25:23 Backup abgeschlossen
```

Die beiden `tar`-Hinweise über entfernte führende Schrägstriche waren keine gemeldeten Abbrüche. Das konkrete Archiv hieß `SRV-M-RPi-TBS01-config-backup-2026-04-18_22-25-20.tar.gz`. Der Log belegt erfolgreichen Ablauf der Pipeline, aber weder vollständigen Archivinhalt noch einen erfolgreichen Restore.

### 7.4 Cron — letzte ausdrücklich bestätigte Festlegung

Der Nutzer bestätigte, dass **beide** Einträge vorhanden sind. Die abschließende Anleitung verwendete die Root-Crontab, nicht die anfangs irrtümlich vorgeschlagene normale Benutzer-Crontab:

```cron
0 3 * * * /usr/local/sbin/backup-bluestation.sh >> /var/log/bluestation-backup.log 2>&1
0 4 * * 0 DO_FULL_BACKUP=true /usr/local/sbin/backup-bluestation.sh >> /var/log/bluestation-backup.log 2>&1
```

Bedeutung: täglich 03:00 Config-/Setup-Backup, sonntags 04:00 zusätzlich Vollimage; Zeitbasis ist die Zeitzone der Basisstation bzw. des Cron-Dienstes. Die tatsächliche Zeitzoneneinstellung wurde nicht abgefragt. Das Full-Skript führt dabei zunächst ebenfalls ein Configbackup aus.

Die Ausführung der geplanten Jobs wurde im sichtbaren Verlauf nicht mit einem späteren Cronlog nachgewiesen. Ebenso fehlt ein erfolgreiches Vollimage. Die korrekte manuelle Umgebungsvariablen-Form in der finalen Anleitung lautete:

```bash
sudo DO_FULL_BACKUP=true /usr/local/sbin/backup-bluestation.sh
```

### 7.5 Kapazität, Integrität und noch fehlender Restore

**70 GB sind eine bestätigte LXC-Größe, keine bestätigte ausreichende Aufbewahrungskapazität.** Die früher genannten Imagegrößen von etwa 4–12 bzw. 5–10 GB waren Schätzungen ohne gemessene SD-/Imagegröße. Ein Rohimage umfasst auch nicht aktuell belegte Sektoren; deren Inhalt ist nicht zwingend gut komprimierbar. Der belegte Dateisystemplatz allein bestimmt deshalb die gzip-Größe nicht.

Für die Planung müssen mindestens gleichzeitig aufbewahrte Vollimages, Platz für das nächste Image, tägliche Archive, vorhandene OPNsense-Backups, LXC-Systembedarf und Reserve berücksichtigt werden. Eine mögliche Rechenhilfe ist: `(behaltene Images + 1) × gemessene Imagegröße + übrige Daten + Reserve`. Das ist eine neue Planungsableitung, keine Messung aus diesem Chat.

Die vorgeschlagenen Retentionwerte wechselten: 14 Tage insgesamt, später getrennte Fristen, zeitweise 3–4 bzw. 7 Images, zuletzt 30 Tage für Configs und mehr als 14 Tage für Vollimages bzw. optional fünf Images. **Eine endgültig ausgewählte, installierte und getestete Rotation wurde nicht bestätigt.** Die breite Variante `find ... -type f -mtime ... -delete` darf nicht ungeprüft in einen gemeinsam mit anderen Sicherungen genutzten Pfad übernommen werden.

Weitere Befunde der Archivprüfung:

- Ein `dd`-Lesevorgang auf dem laufenden, veränderlichen Rootdatenträger garantiert keinen konsistenten Zeitpunkt. Er ist kein verlässlich abgenommenes Bare-Metal-Backup nur deshalb, weil die Übertragung fehlerfrei endet.
- Das kleine Backup enthält nicht den gesamten Source-/Binarypfad `/home/jan/netcore-tetra`, nicht automatisch die tatsächlich auf dem USB-Stick liegende Config und nicht die spätere `/opt/bluestation`-Hybridablage. Es ist ausdrücklich kein vollständiger Wiederaufbau des Systems.
- Das Backupskript selbst, die Root-Crontab, VPN-Dateien, SSH-Host-Vertrauen und die späteren Control-/BREW-Server sind nicht vollständig in dieser Liste enthalten.
- Remote-Dateien werden direkt unter dem endgültigen Namen angelegt. Fehlgeschlagene Uploads können unvollständige Archive mit normalem Namen zurücklassen.
- Kein belegter `gzip -t`-/Archivlistentest, keine unabhängige Prüfsummenprüfung und kein Restoretest.
- Keine Laufzeitsperre gegen Überlappung, keine kontrollierte `.partial`-Veröffentlichung, keine Kapazitätsprüfung und keine bewiesene Logrotation.
- Root-Cron benutzt nicht automatisch die `known_hosts`-/Agentumgebung des interaktiven Benutzers. Im manuellen Root-Lauf klappte der Zugriff; ein dauerhafter Cronnachweis fehlt dennoch.

**Priorität für die Fortsetzung:** zunächst Inhalt und Wiederherstellung abnehmen; danach belastbare Aufbewahrung aus realen Größen. Keine weiteren blinden Medienlöschungen und kein produktives Live-Image als bereits sichere Rettung deklarieren.

## 8. Control-Server: Installation, Datenmodell, UI und Signieren

### 8.1 Installation und tatsächlich gefundene Fehler

Der Server sollte als Ubuntu-LXC eingerichtet werden; der tatsächliche Hostprompt lautet `CT-H-DEV-04`. Eine konkrete Ubuntu-Releaseversion wurde nicht durch `/etc/os-release` belegt. In einem Python-Fehler wird ein distributionsverwalteter Python-3.13-Pfad genannt; daraus wird keine exakte Distributionsversion abgeleitet.

Vorgesehen waren Python, venv, Flask, `cryptography`, Gunicorn, SSH, `curl` und `nano`. Die App sollte unter `/opt/netcore-tetra-control` dem Benutzer `netcore` gehören. Der Gunicorn-Dienst war bereits angelegt, als der Projektordner nachweislich noch leer war. Deshalb:

```text
Unable to locate executable '/opt/netcore-tetra-control/venv/bin/gunicorn'
status=203/EXEC
```

Nach fehlgeschlagener Aktivierung versuchte der folgende nackte `pip install` das System-Python zu ändern und traf auf `externally-managed-environment`. Die richtige Korrektur war die tatsächliche Erstellung des venv und Installation darin, nicht `--break-system-packages`. Später zeigte `ls` die vorhandene ausführbare `venv/bin/gunicorn` mit Besitzer `netcore` und 192 Bytes. (C09; E02, E03)

Dann fehlten PEM-/State-Dateien. `generate_state_keys.py` lag zunächst unter `/home/netcore`, wurde aber in `/opt/netcore-tetra-control` aufgerufen. Zusätzlich wurde `python` außerhalb des aktivierten venv nicht gefunden und `python3` konnte dort `cryptography` nicht importieren. Diese Fehler wurden durch inkonsistente Anleitungen zu Arbeitsverzeichnis und Interpreter verursacht; sie dürfen nicht als mangelnde Befolgung durch den Nutzer umgedeutet werden. Nach einem vollständigen Copy/Paste-Block mit Dateierzeugung am richtigen Ort meldete der Nutzer, dass es zu laufen scheint.

### 8.2 Historische systemd-Unit

```ini
[Unit]
Description=NetCore-Tetra Control UI
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=netcore
Group=netcore
WorkingDirectory=/opt/netcore-tetra-control
ExecStart=/opt/netcore-tetra-control/venv/bin/gunicorn -w 2 -b 0.0.0.0:8080 app:app
Restart=on-failure
RestartSec=5

[Install]
WantedBy=multi-user.target
```

Die zwei Worker sind für die ungesperrte JSON-Dateiverwaltung des gelieferten Codes relevant: mehrere gleichzeitige Requests können denselben Stand lesen und überschreiben. Keine Mehrprozesssynchronisierung wurde eingebaut oder getestet.

Für künftige Reparaturen sind absolute Interpreter-/Skriptpfade vorzuziehen. Ein Prüfaufruf sähe bei vorhandenen Dateien zum Beispiel so aus; er ist **keine hier zusätzlich durchgeführte Serveraktion**:

```bash
/opt/netcore-tetra-control/venv/bin/python -c 'import flask, cryptography, gunicorn'
```

Ein Keygenerator soll ausdrücklich nicht als Bibliotheks- oder Pfadtest erneut gestartet werden.

### 8.3 Entwicklung von einer festen Node zu mehreren Stationen

Die erste Fassung hatte ein festes `NODES`-Dictionary mit `bs01` / `TBS01`. Auf ausdrücklichen Wunsch wurde dies durch eine persistente `nodes.json` ersetzt. Deren Form:

```json
{
  "bs01": {
    "name": "TBS01",
    "device_id": "bs01",
    "locked": false,
    "created_at": "<UTC-Zeitstempel>",
    "updated_at": "<UTC-Zeitstempel>"
  }
}
```

`node_id` ist der unveränderte Map-Schlüssel und URL-/Verzeichnisbestandteil. „Umbenennen“ in dieser Fassung ändert den **Anzeigenamen**, nicht diesen Schlüssel. Das Editformular erlaubt zusätzlich die Änderung der `device_id`. Dies ist nicht bloß kosmetisch: Die lokale ID der TBS und der signierte Config-Kommentar müssen dazu passen.

Bei fehlender Registry wird eine Defaultstation angelegt. Fehlende State-Dateien werden auch beim Lesen über `ensure_node_files()` erzeugt. Ein GET auf die UI/Registry ist damit nicht in jedem Fall dateisystemseitig schreibfrei. Die Initialisierung ist nicht als transaktionaler Mehrworkerprozess ausgeführt.

### 8.4 Signiertes State-Format und Semantik

```json
{
  "device_id": "bs01",
  "desired_state": "STOP",
  "version": 1,
  "locked": false,
  "updated_at": "<UTC-Zeitstempel>"
}
```

Der Server schreibt JSON mit `indent=2`, `sort_keys=True` und abschließendem LF, liest die Datei erneut ein, signiert die Bytes mit dem State-Key und schreibt die Base64-Signatur mit LF nach `state.json.sig`. Initialzustand einer neuen Station ist STOP. Jede Aktion bzw. Metadatenänderung erhöht die State-Version.

Sperren setzt `locked=true` und erzwingt STOP. Ein RUN-Versuch für eine gesperrte Station wird im Servercode zu STOP umgewandelt. Entsperren setzt die Sperre zurück; nach zuvor erzwungenem STOP bleibt die Station normalerweise auf STOP, bis separat RUN gewählt wird. Das Löschen entfernt den Registryeintrag und versucht Dateien/Verzeichnisse zu löschen; es versendet keinen abschließend quittierten STOP und behält keinen signierten Lösch-/Sperrstatus für später wiederkehrende Clients.

**Nicht gleichsetzen:** `state.version` ist die Version des Sollzustands; `CONFIG_VERSION` gehört zur Konfiguration. Keine der beiden Zahlen ist die Rust-/Firmware-/Git-Version. Beispielwerte wie Configversion 42 auf einem Bon sind keine Betriebswerte.

### 8.5 HTTP-Vertrag der letzten in diesem Chat gelieferten Control-App

| Methode und Pfad | Zweck / Einschränkung |
|---|---|
| `GET /` | Gesamte Verwaltungs-WebUI |
| `GET /set/<node_id>/<desired_state>` | RUN/STOP setzen und zur UI umleiten; zustandsändernder GET |
| `POST /node/create` | Neue Station anlegen |
| `POST /node/<node_id>/edit` | Anzeigename und Geräte-ID ändern |
| `POST /node/<node_id>/delete` | Station und Dateien löschen |
| `GET /node/<node_id>/lock` | Sperren und STOP signieren |
| `GET /node/<node_id>/unlock` | Entsperren |
| `GET /state/<node_id>/state.json` | Signierten Sollzustand ausliefern |
| `GET /state/<node_id>/state.json.sig` | Base64-State-Signatur ausliefern |
| `GET /configs/<node_id>/config.toml` | Config ausliefern, sofern vorhanden |
| `GET /configs/<node_id>/config.toml.sig` | Zugehörige Signatur ausliefern |
| `GET /api/nodes` | Nach Node-ID indiziertes JSON-Objekt mit `meta`, `state`, `urls` |
| `GET /api/backend-status` | Ergebnis der BREW-Health-Abfrage |
| `GET /health` | Control-Status, Anzahl Nodes, eingebetteter BREW-Status, Zeitstempel |

Die Datei-Routen begrenzen die Dateinamen auf die genannten festen Namen. Ein Aufruf der Links ist nicht automatisch ein erzwungener Download, weil `send_from_directory` nicht mit `as_attachment=True` aufgerufen wurde. Der Nutzerwunsch war ein komfortabler Zugriff ohne Serververzeichnisnavigation; sichtbare Links wurden bestätigt. Die frühere Behauptung, fehlende Links lägen zu 99 Prozent am Browsercache, war nicht belegt.

**API-Versionsgrenze:** Eine später vorhandene Android-/Control-Archivdatei dokumentiert bereits ein Pluginlayout und `/stations/set/...`. Das ist eine andere historische Entwicklungsphase. Die hier dokumentierte Route `/set/...` darf nicht als Beweis für den heutigen Live-Endpunkt verwendet werden. Sie ist der Vertrag der in **diesem** Chat ausgegebenen monolithischen App. (R11)

### 8.6 UI und Nachweise

Die dunkle Oberfläche zeigt Stationskarten, Soll-RUN/STOP, Device-ID, State-Version, Erstellungs-/Änderungszeit, Start/Stop, Lock/Unlock, Edit-/Deleteformulare sowie Links bzw. Fehlanzeigen zu State und Config. Der Nutzer bestätigte, dass die erweiterte Oberfläche zunächst passt.

Der vorhandene Screenshot zeigt die frühe Einzelstationsfassung mit `TBS01 (bs01)`, RUN, Version 12, Änderungszeit `2026-04-27T23:29:37.958040+00:00` sowie Links auf die vier Dateien. Er beweist Darstellung und vorhandene Links, nicht die erfolgreiche Auslieferung aller Dateien oder tatsächliches Senden. Später meldete der Nutzer ausdrücklich 404 für Config und Signatur, weil diese noch nicht auf dem Server lagen. (A01, C10)

Die letzte App nutzt HTML-Meta-Refresh alle 15 Sekunden. Das aktualisiert die gesamte Seite und kann noch nicht abgeschickte Formulareingaben verwerfen. Ein gezieltes asynchrones Statusupdate wäre eine künftige Verbesserung.

### 8.7 Config-Ablage und nicht abgeschlossener Signierworkflow

Die Config-Dateien gehören historisch unter:

```text
/opt/netcore-tetra-control/static/configs/<node_id>/config.toml
/opt/netcore-tetra-control/static/configs/<node_id>/config.toml.sig
```

Der Nutzer wollte die Config zunächst per `nano` erstellen. Dafür fehlt ohne Config-Private-Key weiterhin eine neue passende Signatur. Der bevorzugte Weg bleibt: am Administrationsrechner signieren, exakte Bytes und zugehörige Base64-Signatur verteilen. Alternativ kann ein bereits gültiges Paar vom vorhandenen Stick verwendet werden, ohne einen neuen Schlüssel zu erzeugen. Dies setzt tatsächliche Dateiverfügbarkeit und unveränderte Bytes voraus.

Die vorgeschlagenen SCP-Befehle widersprachen dem manuellen Copy/Paste-Wunsch. Der optionale serverseitige Config-Signer setzt einen Config-Private-Key auf dem Server voraus und ist **nicht** als gewählt oder erfolgreich ausgeführt dokumentiert. Der State-Key darf nicht stillschweigend zum Config-Key umgewidmet werden.

### 8.8 Sicherheits- und Integritätsbefunde der Archivprüfung

Die folgenden Punkte betreffen den konkret gelieferten monolithischen Chatcode, nicht automatisch jeden heutigen Serverstand:

1. **Keine Authentifizierung oder Rollenprüfung.** Erreichbarkeit im LAN/VPN allein autorisiert keinen Bediener. Jede erreichbare Person bzw. jeder kompromittierte Client könnte administrative Routen benutzen.
2. **Zustandsändernde GETs und fehlender CSRF-Schutz.** Start, Stop und Sperren dürfen nicht allein durch Aufruf eines Links erfolgen. Eine künftige Reparatur braucht konsistente POST-/CSRF-/Authentifizierungsregeln; nicht nur eine versteckte Schaltfläche. (E07)
3. **Unzureichende Node-ID-Prüfung.** `^[a-zA-Z0-9._-]+$` akzeptiert auch `.` und `..`. Zusammensetzen dieser Werte mit Datenpfaden ohne aufgelöste Pfadgrenzen ist unsicher. Das wurde isoliert nachgestellt, ohne den realen Server aufzurufen. (T-A04)
4. **Nicht atomare Registry und State-Paare.** Ein Fehler zwischen JSON- und Signaturdatei oder konkurrierende Worker kann inkonsistente Daten und gleiche Versionsnummern erzeugen. Zwei separate HTTP-Downloads können zusätzlich unterschiedliche Generationen erwischen.
5. **Keine bestätigte Istzustandsrückmeldung.** Die App weiß nicht, ob die TBS den Befehl gesehen, akzeptiert oder ausgeführt hat. Erfolg der HTTP-Aktion ist keine Senderquittung.
6. **Lock/Löschen sind nicht ausfallsicher widerrufend.** Eine entfernte Node ergibt 404; der alte Client kann dies als Netzwerkfehler behandeln und auf USB starten. Eine bereits empfangene Sperre muss im geplanten Client gegenüber Fallback bewusst definiert und gespeichert werden.
7. **Schlüsselinitialisierung überschreibt bestehende Keys.** Die Generatorbeispiele sind nicht wiederholsicher; kein versehentlicher Einsatz als Setup-Reparatur.
8. **Berechtigungstrennung unvollständig.** Anwendung, mutable Daten und Schlüssel liegen unter einem vom Dienstbenutzer beschreibbaren Projektbaum. Ein kompromittierter Webprozess kann dort mehr ändern als nur den gewünschten Zustand.
9. **HTTP und fehlende Zugriffskontrolle schützen keine Config-Geheimnisse.** Signaturen bieten Authentizität, keine Vertraulichkeit. Auch der standardmäßige Flask-Staticpfad muss in ein späteres Zugriffskonzept einbezogen werden.
10. **Fehler-/Größenbehandlung der Health-Abfrage ist einfach.** Synchrones Lesen ohne Antwortgrößenlimit, breite Exceptionbehandlung und fehlender Schemazwang können unpassende Backendantworten als Offline statt als ungültige Healthmeldung darstellen.

Diese Punkte werden hier dokumentiert, aber nicht außerhalb von `Docs/archive/` repariert.

## 9. BREW-Backend und Health-Erweiterung

### 9.1 Tatsächlich gelieferte Anwendung

Der Nutzer stellte die vollständige `/var/opt/brew-server/app.py` bereit. Sie verwendet **aiohttp und asyncio**, nicht Flask. Sie startet zwei `web.Application`-Instanzen über `AppRunner` und `TCPSite` in einem Prozess. Der vorher pauschal angebotene Gunicorn-Befehl `app:app` passt nicht automatisch zu diesem Programmaufbau.

Konfiguration des geposteten Codes:

| Parameter | Wert |
|---|---|
| `LISTEN_HOST` | `0.0.0.0` |
| `LISTEN_PORT` | 8080 |
| `WEBUI_PORT` | `BREW_WEBUI_PORT`, Standard 8081 |
| `REALM` | `brew-router`; keine verwendete Basic-Auth daraus ableiten |
| `RELOAD_INTERVAL_SECONDS` | 60 |
| `NODES_FILE` | `BREW_NODES_FILE`, sonst relativer Pfad `nodes.json` |
| JSON-Struktur | Objekt mit ISSI-Strings als Schlüsseln und Passwort-Strings als Werten; keine echten Werte archiviert |

Als systemd-Konfiguration waren `WorkingDirectory=/var/opt/brew-server`, `ExecStart=/usr/bin/python3 /var/opt/brew-server/app.py`, Benutzer/Gruppe `netcore`, `Restart=always`, `RestartSec=5`, Journallogging und `LimitNOFILE=4096` vorgeschlagen. Vor einer Fortsetzung sind der auf diesem **separaten** Server tatsächlich vorhandene Benutzer und Interpreter mit installiertem `aiohttp` zu prüfen; der Benutzer auf dem Control-LXC beweist dessen Existenz auf BREW nicht.

### 9.2 Endpunkte und binärer Nachrichtenvertrag

| Port | Endpunkt | Funktion |
|---|---|---|
| 8080 | `GET /brew/` | Neue UUID erzeugen und einen relativen WebSocketpfad `/brew/<uuid>` als Text liefern |
| 8080 | WebSocket `/brew/<uuid>` | Binäre BREW-Nachrichten; angebotenes Subprotokoll `brew` |
| 8080 | `GET /health` | Hinzugefügter JSON-Status |
| 8081 | `GET /` | ISSI-/Node-Editor |
| 8081 | `POST /save` | Gesamte ISSI-Tabelle speichern |
| 8081 | `POST /delete` | Einzelnen ISSI-Eintrag löschen |
| 8081 | `GET /health` | Derselbe JSON-Status |
| 8081 | `GET /api/status` | Alias auf denselben Handler |

Für den späteren Wiederaufbau wichtige Konstanten des geposteten Codes:

| Klasse | Bedeutung / Layout |
|---|---|
| `0xf0` | Subscriber: zwei Headerbytes, `number` als little-endian 32 Bit, Zeitfeld 64 Bit, Fraction 32 Bit; danach Gruppen als 32-Bit-Werte |
| `0xf1` | Call control: Klasse/Typ, 16 UUID-Bytes, zusätzliche Bytes |
| `0xf2` | Frame: Klasse/Typ, 16 UUID-Bytes, little-endian 16-Bit-Bitlänge, Nutzdaten |
| `0xf3` | Fehlerklasse mit Typ und optionalen Bytes |
| `0xf4` | Service: Klasse/Typ, UTF-8-JSON, Nullterminator |

Subscriber-Typen: Deregister 0, Register 1, Reregister 2, Affiliate 8, Deaffiliate 9. Deaffiliate ist deklariert, aber in der gezeigten Dispatcherfunktion nicht bearbeitet.

Service-Typen: Query subscribers 1, Subscriber profiles 2, Allowed-ISSIs request 3 und Response 4. Auf Typ 3 folgt ein Service-Frame Typ 4 mit `{"allowed_issis": [...]}`. Diese Liste stammt aus dem gecachten Integer-Set der erlaubten ISSIs.

Call-State-Konstanten: Group TX 2, Group Idle 3, Setup Request 4, Setup Accept 5, Setup Reject 6, Call Alert 7, Connect Request 8, Connect Confirm 9, Call Release 10, Short Transfer 11, Simplex Granted 12, Simplex Idle 13. Der gezeigte Dispatcher verarbeitet davon Setup Request, Connect Request, Release und Group TX. Frame-Typen: Traffic 0, SDS Transfer 1, SDS Report 2, DTMF 3, Packet Data 4.

**Grenze:** Das Vorhandensein dieser Konstanten ist kein Beweis für vollständige BREW- oder ETSI-Konformität. Sprachframes werden im gezeigten Backend im Wesentlichen protokolliert; ein vollständiger Mehrstations-Medienrouter ist dort nicht implementiert. Setup/Connect werden einfach beantwortet, Calls lokal vermerkt, Release entfernt den Eintrag und Group TX führt zur Group-Idle-Antwort. Keine reale Interoperabilitätsabnahme wurde in diesem Chat gezeigt.

### 9.3 Node-Reload und WebUI

`load_nodes()` liest JSON, protokolliert die Schlüsselliste und liefert bei fehlender Datei, ungültigem JSON oder anderen Fehlern `{}` zurück. Numerische Schlüssel werden in ein Integer-Set überführt. Der Hintergrundtask prüft alle 60 Sekunden die mtime und aktualisiert das Set bei Änderungen. `save_nodes()` schreibt zunächst eine Datei mit `.tmp` und ersetzt dann die Zieldatei mit `os.replace`.

Die UI kann Zeilen löschen und die gesamte Tabelle speichern. Die neu gespeicherte Datei ist nicht automatisch bereits im in-memory-Allowset angekommen; dafür läuft der periodische Reload. Im HTML stehen Löschformulare innerhalb eines Speicherformulars; diese verschachtelten Formulare sind strukturell problematisch. Es gibt keinen sichtbaren Authentifizierungs-/CSRF-Schutz und keine umfassende Schema-/ISSI-Validierung vor dem Speichern.

**Wesentliche Authentifizierungsgrenze:** Die Passwortwerte aus `nodes.json` werden im gezeigten Registrierungsablauf nicht überprüft. Zulassung erfolgt allein anhand der gemeldeten ISSI. Eine ISSI-Allowlist ist keine Identitätsprüfung eines Gegenübers, das diese ISSI behauptet. Außerdem sind nicht alle Call-/Frame-Handler durch `state.registered` abgeschirmt. Änderungen an der Allowlist erzwingen nicht sichtbar die Beendigung bereits bestehender Sessions.

### 9.4 Hinzugefügte Health-Payload

Die Erweiterung ergänzt an jeder Connection `connected_at` und liefert folgende Felder:

```text
status, service, timestamp
listen_host, brew_port, webui_port
nodes_file, nodes_file_exists, nodes_file_mtime
nodes_count, allowed_issis_count, allowed_issis
active_connections_count, active_connections
```

Jeder Connection-Eintrag enthält:

```text
connection_id, issi, registered, affiliated_groups,
active_calls, connected_at
```

Der Nutzer bestätigte nach der vollständigen neuen `app.py`: **„klappt“**. Das ist der historische Funktionsnachweis des Endpunkts, nicht jeder Eigenschaft seiner Daten oder der systemd-Startkette.

### 9.5 Health-Grenzen — wichtig für die Lampe

`build_health_payload()` ruft `load_nodes()` bei jeder Anfrage erneut auf und setzt dennoch **immer** `status: ok`. Dadurch kann ein fehlender oder ungültiger Nodebestand mit HTTP 200 und `ok` erscheinen. `nodes_count` kann den frisch gelesenen Zustand zeigen, während `allowed_issis_count` noch den bis zu 60 Sekunden alten Cache abbildet. Bei einer verschwundenen Datei behält der Reloadcode das bestehende Set zunächst bei. Ein verlorener Hintergrundtask oder Fehler eines Ports werden nicht als Readinessfehler in der Payload modelliert.

Die `active_connections_count` ist die Zahl verwalteter WebSocketobjekte, nicht die Zahl real sendender Basisstationen. Ein Feld `registered` ist ein Anwendungsflag, kein kryptografischer Echtheitsnachweis. Die öffentliche Statusantwort enthält außerdem die gesamte ISSI-Allowlist und Verbindungsdetails; für einen Minimal-Healthcheck ist das mehr Information als erforderlich.

Der Server erzeugt in Subscriberprofilen mit `isoformat(...)+"Z"` potenziell einen Zeitstring mit `+00:00Z`. Dies ist eine weitere Detailkorrektur für später, nicht Teil der hier vorgenommenen Archivierung.

**Vorschlag für die Fortsetzung:** Liveness und Readiness trennen; erfolgreiches Einlesen/Schemavalidieren, Alter des Reloads, Taskzustand und Listenerzustand auswerten; bei Nichtbereitschaft gezielt 503; Diagnoseinformationen und Teilnehmerlisten nur über berechtigte Detailendpunkte. Die bestehenden Endpunkte sind nicht automatisch ein solcher Readinessnachweis.

## 10. BREW-Statuslampe im Control-Server

Die ursprüngliche Lampe rief `http://10.0.1.163:8081/` mit zwei Sekunden Timeout ab und bewertete auch HTTP-Fehler unter 500 als erreichbar. Das war ein einfacher Erreichbarkeitscheck, keine Funktionsprüfung. Nach der Health-Erweiterung wurde die Control-App vollständig auf folgenden Vertrag umgestellt:

```text
BACKEND_STATUS_URL = http://10.0.1.163:8081/health
BACKEND_UI_URL     = http://10.0.1.163:8081/
Timeout           = 2 Sekunden
```

Die finale `check_backend_status()`:

- verlangt eine 2xx-Antwort;
- parst JSON;
- setzt Grün bei `payload.status == "ok"`;
- zeigt Nodes, erlaubte ISSIs und aktive Verbindungen aus den drei Count-Feldern;
- setzt Gelb bei HTTP-Fehler, ungültigem JSON oder anderem Healthstatus;
- setzt Rot bei Verbindungs-/sonstigem Abruffehler;
- liefert `state`, `label`, `details`, UI-/Health-URLs und optional die Originalpayload zurück.

Die Oberfläche und `/api/backend-status` verwenden diese Funktion; `/health` des Control-Servers bettet das Ergebnis ein, gibt aber selbst weiterhin `status: ok` zurück. Die Lampe ist somit nur so belastbar wie das BREW-Signal. Der abschließend gelieferte Code enthält **keine automatische Stop-/Startsperre bei roter BREW-Lampe**. Eine solche Kopplung wurde nicht beschlossen oder implementiert und darf nicht hineininterpretiert werden.

## 11. Hybridmodell: beschlossenes Ziel, unvollständiger Entwurf

### 11.1 Gewünschtes Ziel

Die Basisstation soll per VPN/LAN den Heimserver erreichen, von dort Konfiguration und signierten Sollzustand selbst abrufen und bei fehlendem Netzwerk auf eine gültige USB-Konfiguration zurückgreifen. Im UI soll Start/Stop bzw. Sperren je Station bedienbar sein. Die lokale Prüfung bleibt zwingend; eine gültige VPN-Verbindung allein ersetzt sie nicht.

Der Nutzer korrigierte die zuvor angebotene Priorität ausdrücklich auf **Netzwerk → USB**. Zunächst wurden Funktionen am Server priorisiert. Fortlaufendes Polling, Iststatusmeldungen, automatische Erstbereitstellung und ein vollständiges Supervisor-Modell waren noch nicht umgesetzt nachgewiesen.

### 11.2 Was der gelieferte Shellentwurf tatsächlich tut

`/opt/bluestation/net/fetch_and_run.sh` enthält einen noch nicht ersetzten Server-IP-Platzhalter, `NODE=bs01`, vier `curl -fsS`-Downloads nach `/opt/bluestation/runtime`, die Configprüfung, eine Python-State-Signaturprüfung und danach einen einmaligen Stringtest auf RUN. Für STOP wird `pkill -f bluestation-bs` verwendet; für RUN `exec "$BINARY" "$TMP_DIR/config.toml"`.

Der vorgeschlagene Wrapper ruft dieses externe Skript auf und wählt bei dessen Fehler die separate USB-Fassung. Es fehlt die dauerhaft aktive Instanz, die auch **nach** einem Senderstart weitere Zustandsänderungen abruft. Der Elternwrapper wartet bei erfolgreichem RUN auf den laufenden Prozess; ein späterer UI-STOP wird von diesem Ablauf nicht wieder abgefragt.

Die Umbenennungs-/Kopieranleitung war zusätzlich in falscher Reihenfolge formuliert: Zuerst sollte `import_and_start.sh` durch den Wrapper ersetzt werden, danach sollte genau diese Datei als USB-Fassung kopiert werden. Befolgt man das wörtlich, sichert man nicht das bisherige USB-Skript, sondern den neuen Wrapper. Das kann zu einem rekursiven falschen Fallback führen. Der Ablauf ist **nicht erneut auszurollen**.

### 11.3 Weitere konkrete Defizite

- Es wird globales `python3` statt des bekannten Verifier-venv verwendet; genau diese Bibliotheks-/Interpreterverwechslung war bereits aufgetreten.
- Die State-Signatur wird geprüft, nicht aber vollständig das State-Schema, die erwartete Device-ID, monotoner State-Stand, Gültigkeitszeit oder `locked`.
- `desired_state` wird mit `grep` und `cut` aus JSON extrahiert, nicht durch einen streng validierenden JSON-Parser.
- Der Netzwerkpfad aktualisiert die persistente Config-Mindestversion nicht wie der USB-Startpfad.
- Config und Signatur sowie State und Signatur werden in getrennten Requests ohne atomare Generation geholt; ein UI-Toggle kann dazwischenliegen.
- Der Netzwerkcode lädt zuerst Config und Signatur. Fehlen sie, erreicht er einen vorhandenen signierten STOP gar nicht; stattdessen kann der Wrapper USB versuchen.
- Abruf-Timeouts waren im Konzept genannt, im gezeigten `curl`-Code fehlen jedoch explizite Gesamt-/Verbindungszeitlimits.
- Ein fehlendes `WorkingDirectory` kann relative Configpfade anders auflösen als das alte USB-Skript.
- `pkill -f` ist keine präzise Zuständigkeit für einen einzelnen systemd-Senderprozess und ersetzt keinen Manager mit sauberem Child-/Unit-Lifecycle.
- Alte udev-Remove-Regeln würden weiterhin die globale Unit stoppen, auch bei Netzwerkbetrieb.
- Ein globaler persistenter Configversionsstand kann einen älteren gültigen USB-Fallback nach einem neueren Netzwerkupdate berechtigt ablehnen. Dafür fehlt eine festgelegte Pflege-/Freigabepolitik.

### 11.4 Erforderliche Zustandsentscheidung — neue Ableitung, noch freizugeben

Die im Chat teilweise verwendete Kurzform „Netzwerk nicht verfügbar oder ungültig → USB“ ist sicherheitlich zu ungenau. Für die Fortsetzung sollte mindestens folgende Fallunterscheidung spezifiziert und getestet werden:

| Eingang | Vorzuschlagende Behandlung |
|---|---|
| Authentischer aktueller RUN und gültige passende Config | Start zulassen, Quelle und angewandte Version dokumentieren |
| Authentischer STOP oder Lock | Stoppen bzw. nicht starten; kein USB als Umgehung dieses Befehls |
| Netzwerk wirklich nicht erreichbar | USB-Fallback nur gemäß ausdrücklich festgelegter Offline-/Sperrpolitik |
| Ungültige Signatur, falsche Geräte-ID, unzulässiger Rückstand | Sicherheitsfehler, nicht stillschweigend normaler Offlinefall |
| Unbekannte/gelöschte Node, 401/403/404 | Eigene Diagnose und Freigabepolitik, nicht automatisch Netzwerkabwesenheit |
| Kein gültiger Input | Sender bleibt aus |
| Netz kehrt zurück | Kontrollierter Quellenwechsel ohne Doppelstart oder unnötige Neustartschleifen |

Ob ein bereits empfangener STOP/Lock über Neustart und Netzverlust hinweg verriegelt bleibt, wie lange eine RUN-Freigabe gilt und ob ein bewusst freigegebener Offline-Override erlaubt ist, wurde **noch nicht endgültig vereinbart**. Ein nicht zugestellter Remote-STOP kann bei echter Netztrennung nicht sofort wirken; eine Lease begrenzt nur die Weiterlaufdauer. Das muss offen benannt werden, statt absolute Fernabschaltbarkeit zu behaupten.

Ein zukünftiger Agent braucht getrennten Manager- und Senderdienst, Zeitlimits, persistente Versions-/Sperrentscheidungen, atomare Downloads, eindeutige Quelle, aktive Rückmeldung und abgesicherte Bootstrap-Schlüssel. Öffentliche Vertrauensschlüssel dürfen nicht bei jeder Anfrage ungeprüft von derselben untrusted Quelle neu akzeptiert werden.

## 12. Thermobondruck und intelligenter Receipt-Receiver

### 12.1 Endgültig gewünschter Umfang

Aus einer bewusst humorvollen Nebenidee wurde ein klares Bedienkonzept: kurze Startinfo und relevante Fehler sollen einen kleinen Bon erzeugen, mit Original-Logo, technischem Kontext, kurzem frechen Kommentar und QR-Verweis auf das passende Log. Es soll **nicht jeder normale Vorgang** gedruckt werden.

Gewünschte Hardwareeigenschaften sind USB für den ersten Aufbau, automatische Schneideeinheit statt bloßer Abrisskante und später Netzwerkfähigkeit. Ein Pi als USB-Druckserver wurde akzeptiert; der Nutzer entschied ausdrücklich, ihn dann als **intelligenten Receiver** zu verwenden und nicht nur als rohe Netzwerk-/USB-Brücke.

Es wurde kein Drucker gekauft oder ein konkretes Modell festgelegt. Genannte Marken-/Serienbeispiele waren Epson TM-T20/TM-T88, Bixolon SRP-350 und Star TSP100 sowie preiswerte Alternativen. Das waren unüberprüfte Suchideen, keine nachgewiesene Gerätekompatibilität oder verbindliche Einkaufsliste. Modellvarianten unterscheiden sich bei USB/LAN, ESC/POS-Emulation, Bild-/QR-Unterstützung, Cutter und Zeichensätzen. Die damaligen Preisannahmen sind nicht als aktuelle Marktpreise zu verwenden.

### 12.2 Vorgeschlagener Receiver-Vertrag

```text
TBS / Control / BREW
    -> ausgewähltes HTTP-Event
    -> Receipt-Server auf Pi
    -> Vorlagenwahl, Logo, Kommentar, QR, Queue
    -> USB/ESC-POS bzw. später Netzwerkdruck
```

Diskutierte, noch nicht implementierte Schnittstellen: `POST /print/event`, zuvor allgemeiner `POST /print`, `GET /health`, `GET /last`. Ein API-Key-Header wurde als Mindestidee genannt; kein echter Token ist dokumentiert. Queue/Puffer bei Offline-Drucker, mehrere Drucker, standortbezogene Ausgabe und Druckhistorie sind weiterführende Ideen.

Als Eventfelder wurden `type`, `node`, `source`, `version`, `message` und optional `error` vorgeschlagen. Beispieltypen waren Start und Security-/Configfehler. Eine endgültige versionierte Eventspezifikation, Authentifizierung, Idempotenz oder Zustellquittung wurde nicht erstellt.

`/dev/usb/lp0`, USB-VID/PID, `python-escpos`, QR-Bibliothek und RAW-TCP 9100 waren mögliche Transport-/Bibliotheksansätze. Die Existenz von `/dev/usb/lp0` ist nicht für jedes USB-Druckermodell garantiert. ESC/POS-Cutbefehle und Full-/Partial-Cut müssen am konkreten Modell geprüft werden. Der scherzhafte Vorschlag mehrerer Schnitte für einen kritischen Fehler ist keine benötigte technische Funktion.

### 12.3 Druckpolitik

Die belastbare Nutzervorgabe ist: **kurzer Startbon, wichtige Fehler, begrenztes Volumen**. Anfangs sehr weitreichend vorgeschlagenes Drucken jeder ISSI-Anmeldung oder jedes Rufbeginns ist damit überholt. Normale Logs und Funkframes gehören ins digitale Log.

Als neue technische Umsetzungsempfehlung: Start/Config/Signatur/VPN/BREW-Ergebnis in einem einzigen Betriebsbeginnbon bündeln; gleiche Fehler über Event-ID bzw. Fehlerklasse deduplizieren; Wiederholungen zählen statt sofort erneut drucken; Schwellen und Beruhigungszeiten für Netz-/Temperaturflattern; Statusbericht nur manuell oder selten. Ein ausgefallener Drucker darf die Senderfreigabe weder blockieren noch selbst erteilen. Ein Bon ist ein Beobachtungsartefakt, kein manipulationssicheres Auditarchiv.

### 12.4 Erhaltene Bon-Ideen und Kommentare

Die folgende Sammlung konserviert alle 35 im Chat konkret nummerierten Motive. **Sie ist eine Vorlagenbibliothek, keine automatische Druck-Whitelist.** Die Kommentare sind Beispiele; wichtige technische Befunde dürfen nicht durch Humor ersetzt oder als falsche Sicherheitsgarantie formuliert werden.

| Nr. | Motiv / Ereignis | Historischer Kommentar bzw. Motivtext | Einordnung |
|---|---|---|---|
| 1 | Systemstart | „Ich bin wach. Kaffee optional.“ | Gewünschter Kernbon |
| 2 | Start mit USB | „Oldschool. Gefällt mir.“ | Als Quellangabe in Startbon integrieren |
| 3 | Config geladen | „Sieht sauber aus. Ich vertraue dir… diesmal.“ | In Start-/Änderungsbon bündeln |
| 4 | Config konnte nicht geladen werden | „Das war nix. Versuch’s nochmal.“ | Relevanter Fehler, deduplizieren |
| 5 | Signatur ungültig | „Netter Versuch. Aber nicht mit mir.“ | Sicherheitsereignis |
| 6 | Unbekannter USB | „Den kenn ich nicht. Der darf hier nix.“ | Optional, nicht jedes beliebige USB-Gerät ausdrucken |
| 7 | Netzwerk wieder verfügbar | „Ah, Internet. Ich hab dich vermisst.“ | Besser konkret VPN/Control nennen; LAN ist nicht Internet |
| 8 | Netzwerk ausgefallen | „Ich bin jetzt allein. Wird schon.“ | Nur anhaltende/betriebsrelevante Störung |
| 9 | USB-Fallback aktiviert | „Plan B läuft. Wie immer.“ | Quellenwechsel, keine Umgehung einer Sperre |
| 10 | Zurück zum Netzwerk | „Zurück im Team.“ | Quellenwechsel nach stabiler Rückkehr |
| 11 | Remote-Start | „Ich wurde geweckt. Nicht freiwillig.“ | Angewandte Aktion vom bloßen Auftrag unterscheiden |
| 12 | Remote-Stop | „Alles klar. Ich hör ja schon auf.“ | Nur als ausgeführt ausgeben, wenn bestätigt |
| 13 | Lokaler Stop | „Ich geh schlafen. Mach keinen Blödsinn.“ | Geplanter Betriebsabschluss |
| 14 | BREW verbunden | „Ich hab Freunde gefunden.“ | Meist im Startbon statt eigener Dauerbon |
| 15 | BREW getrennt | „Alle weg. War klar.“ | Anhaltender Ausfall / Zusammenfassung |
| 16 | Erster Ruf nach Start | „Jetzt geht’s los.“ | Optionales Demo-/Highlightmotiv |
| 17 | Langer Funkruf | „Das war kein Smalltalk.“ | Optional, Schwelle nicht vereinbart |
| 18 | Interner Fehler | „Ich war’s nicht. Wirklich.“ | Technischer Fehlertext zusätzlich erforderlich |
| 19 | Temperatur hoch | „Mir ist warm. Ganz schön warm.“ | Nur mit realem Sensor und vereinbarter Schwelle |
| 20 | Manueller Statusbericht | „Läuft. Einfach läuft.“ | Keine unbelegten All-systems-OK-Aussagen |
| 21 | Update verfügbar | „Ich könnte besser werden.“ | Optionale Wartungsinfo, nicht bei jedem Poll |
| 22 | Update installiert | „Neuer Tag, neue Features.“ | Version und tatsächlichen Abschluss nennen |
| 23 | Debugmodus aktiv | „Ich rede jetzt mehr als nötig.“ | Optionaler Wartungsstatus |
| 24 | Security-Check bestanden | „Heute ist keiner eingebrochen. Langweilig.“ | Wortlaut nicht als Sicherheitsbeweis verwenden; nur geprüfte Checks nennen |
| 25 | Unbekannter Zustand / Chaos | „Ich weiß auch nicht mehr. Frag Ole.“ | Humorvorlage; realen Fehlercode nicht weglassen |
| 26 | Watchdog-Neustart | „Ich hab mich selbst neu gestartet. Weil ich kann.“ | Nur bei tatsächlich erkanntem Watchdogereignis |
| 27 | Speicher knapp | „Ich werde langsam vergesslich.“ | Warnung mit Hysterese |
| 28 | CPU gelangweilt | „Langweilig. Gib mir Arbeit.“ | Spaß-/Testbon, nicht automatisch drucken |
| 29 | Node registriert | „Neuer Freund im Mesh.“ | Optionale gezielte Inbetriebnahmeinfo |
| 30 | Node verloren | „Der ist weg. Ich such ihn später.“ | Nicht jede kurze Sessionunterbrechung |
| 31 | Zeit synchronisiert | „Jetzt weiß ich wieder wann ich bin.“ | Relevant etwa nach Zeitfehler, sonst digital loggen |
| 32 | GPS-Fix | „Ich weiß wo ich bin. Du auch?“ | Nur Idee; GPS-Hardware nicht bestätigt |
| 33 | OTA beginnt | „Bitte nicht ausschalten. Wirklich nicht.“ | Optionales Wartungsereignis |
| 34 | OTA fehlgeschlagen | „Das Update wollte nicht. Ich auch nicht mehr.“ | Relevanter Fehler |
| 35 | Debugmodus beendet | „Ich halt jetzt wieder die Klappe.“ | Optionaler Wartungsstatus |

Weitere konkret gewünschte bzw. beispielhaft genannte Texte: „Basisstation wach. Kaffee wäre nett, aber Strom reicht.“, „USB erkannt. Vertrauen ist gut, Ed25519 ist besser.“, „Netter Versuch. Aber Ed25519 sagt: Nö.“ und „BREW offline. Der Server macht wohl Pause.“ Sie sind austauschbare Vorlagen, keine neuen Betriebszustände.

### 12.5 QR und Logo — letzte ausdrückliche Korrektur

Gewünscht ist ein QR, der den Bon mit dem Log der betreffenden Basisstation verbindet. Vorgeschlagene Pfade wie `/logs/bs01/latest`, `/node/bs01/log` oder Varianten mit `/node/tbs01/log` waren **Platzhalter**. In der gelieferten Control-App gibt es diese Routen nicht. Ein funktionierender Logendpunkt, dessen Zugriffsrechte, stabile Event-ID und ein tatsächlich gescannter QR fehlen. Ein QR soll keine Passwörter, API-Keys oder privaten Logdetails als Klartexttransport enthalten.

Das Original-Logo ist `Dunkles Design plus Text.png`: weißes Vierknoten-/Funkwellensymbol, Schriftzug **NetCore-Tetra**, Claim **digital. dezentral. skalierbar.**, dunkler Hintergrund. Zwei und danach weitere ASCII-/Unicode-Annäherungen wurden ausdrücklich abgelehnt. **Endentscheidung: Originalbild verwenden, nicht ASCII.**

Für den späteren Thermodruck sind Invertieren auf schwarze Bildanteile vor weißem Papier, Zuschneiden, Skalieren auf die tatsächlich nutzbare Druckbreite und eine geeignete Schwarzweißaufbereitung als Arbeitsschritte vorgesehen. Die genannten 384/576 Pixel sind beispielhafte Breiten, keine bereits passende Gerätespezifikation. Logo und QR müssen für das echte Modell und Papier getestet werden.

Auf die Bitte um eine passende Bildfassung wurden zwei generierte Gesamtbon-Mockups geliefert, keine nachgewiesene exakte 1-Bit-Konvertierung des Original-Logos. Die Mockups sind **Gestaltungsartefakte**, kein fertiger Drucktreiber, kein realer Testausdruck und kein bewiesen funktionsfähiger QR. Diese unerledigte Produktionsaufbereitung bleibt offen. (A02–A04)

## 13. Überholte oder zu korrigierende Assistenzansätze

Diese Tabelle verhindert, dass die Archivierung frühere Fehlanleitungen nachträglich legitimiert:

| Früherer Ansatz / Behauptung | Letzte belastbare Einordnung |
|---|---|
| Passende UUID authentisiert den Stick | Falsch als Sicherheitsbehauptung; UUID dient der Auswahl und ist kopierbar |
| Signatur macht Stickkopieren unmöglich | Falsch; identische Datei plus gültige Signatur bleibt gültig. Mit kopierter UUID kann der normale Stick weiterhin geklont werden. |
| Nur root dürfe Public Key lesen | Nicht erforderlich für seine Vertraulichkeit; Integrität/Schreibschutz ist entscheidend. Die späteren Dateien hatten 0644. |
| Fehlender `blkid`-Eintrag beweist fehlendes Blockdevice | Nicht belegt; Cache, Rechte, Signatur und tatsächliche Geräteübersicht müssen unterschieden werden |
| `sda`/`sdb`-Fehler beweisen kaputte Partitionstabelle oder Flashdefekt | Nicht ausreichend; alte/entfernte Geräte und Mountzustand waren nicht getrennt geprüft |
| Nutzer habe Ventoy/Rufus verwendet | Erfundene Vorgeschichte; verwerfen |
| Formatieren trotz Warnung sei vollständig richtig gewesen | Warnung war real; keine Freigabe zur Wiederholung ohne Identifikation und Unmount |
| Leere Anzeige beweise fehlende Windows-Dateien | Hier durch späteren Mount-/Dateibeleg widerlegt |
| UDisks global abzuschalten sei generell die einzig saubere Lösung | Für den damaligen Test gewählt, aber Nebenwirkungen/gezielte Ausnahmeregel offen |
| Zwei Partitionen verhinderten grundsätzlich einen Mount von Partition 1 | Nicht zutreffend; jede passende Partition ist einzeln mountbar, Ziel und Mountzustand müssen bekannt sein |
| `dd` funktioniere immer und sei derselbe bessere Copier | Zu pauschal; Livekonsistenz, Zielgröße, Sektoren und Wiederherstellung bleiben relevant |
| SSH-Server sei automatisch OPNsense selbst | Nicht belegt; später separater LXC-Kontext |
| Normale Benutzer-Crontab sei ausreichend | Final Root-Crontab, wegen Geräte-/Systemdateizugriff und Logpfad |
| Sieben Images passten sicher in 70 GB | Nicht gemessen; Kapazität und Rotation offen |
| Netzwerkfehler und ungültige Signatur seien derselbe Fallbackfall | Sicherheitsrelevante Unterscheidung erforderlich |
| Nach einmaligem `exec` reagiere die TBS weiter auf UI-STOP | Im gelieferten Einmalabruf nicht umgesetzt |
| USB-Fassung erst nach Ersetzen des Startskripts kopieren | Reihenfolge fehlerhaft; alten Stand vor Ersetzen sichern und Identität prüfen |
| Beliebig `python`/`python3` verwenden | Venv und absolute Pfade sind für diese Installation entscheidend |
| Fehlende `.pem` durch beliebig erneute Generatorläufe beheben | Gefahr der Schlüsselüberschreibung; vorhandene Vertrauensanker zuerst prüfen |
| Sichtbares RUN bedeute reales Senden | Nein, Sollzustand ohne nachgewiesene Telemetrie |
| BREW-`status: ok` sei vollständige Gesundheit | Der gezeigte Handler setzt es konstant; Dateifehler werden nicht zur Nichtbereitschaft |
| Alle Drucker einer genannten Serie hätten USB+LAN+ESC/POS | Modellvariante ungeprüft; keine Kaufgarantie |
| ASCII/Unicode sei die finale Logoform | Vom Nutzer ausdrücklich verworfen |
| Generierter Bon-QR sei bereits ein nutzbarer Loglink | Keine Decodier-/Endpunktprüfung; nur Mockup |

## 14. Heutiger Repository-Stand — separat geprüft

### 14.1 Branchprüfung und Reichweite

Der Live-GitHub-Connector bestätigte das öffentliche Repository und den bereits vorhandenen Branch `Archiving`. Der Inhaltsabgleich wurde auf `2b08bababf8b72bd2cb82d780a7ba0e526e1f1a9` fixiert. Der vorhandene Archivindex und bereits vorhandene Dokumente wurden gelesen; eine diesem Chat eindeutig zugeordnete Archivdatei war am vorgesehenen Pfad noch nicht vorhanden. Andere Chatarchive bleiben unverändert.

Der ergänzende Defaultbranch-Ref lautet `main` bei `7137e0dd69877e1b604bf89148fd8b6b590c1a97`. GitHub meldete beim Vergleich **diverged**, 20 Commits auf der Archiving-Seite und einen auf der Main-Seite, gemeinsamen Merge-Base `2fe2a1939a8795db3816d45973282781dae856f0`. Die Archiving-seitige Compare-Dateiliste enthielt Dokumentations-/Archivdateien; der umgekehrte Vergleich zeigte keine main-seitigen Dateiänderungen. Diese Angaben sind ein Ref-/Compare-Befund und kein Anlass zum Mergen oder Branchwechsel. Vor dem Speichern muss der dann aktuelle Archiving-Head erneut berücksichtigt werden. (R01–R03)

Codesuchen nach `import_and_start` und `signing_pubkey` ergaben keine Treffer. Laut Werkzeugvertrag beziehen sich diese Suchfunktionen auf den Defaultbranch; daraus folgt **keine vollständige Abwesenheit auf allen Branches oder an allen Pfaden**. Große rekursive Baumantworten waren gekürzt. Die damaligen lokalen Python-/Shell-Dateien konnten in diesem Prüfpass keinem autoritativen aktuellen Repositorypfad sicher zugeordnet werden. Es wird deshalb nicht behauptet, der Chatcode sei bereits im Hauptrepository implementiert oder inzwischen gelöscht.

### 14.2 Tatsächlich gelesene aktuelle Dateien

| Repositorydatei | Festgestellter Inhalt | Abgrenzung zum Chat |
|---|---|---|
| `install/update-basisstation.sh`, Zeilen 1–180 | Root-Updater; Cargo absichtlich als aufrufender Benutzer; Default-Configpfad `/etc/netcore/config.toml`; Suche nach konfiguriertem Dienstnamen sowie `tetra.service`, `bluestation.service`, `tetra-bluestation.service`, `bluestation-bs.service`; tatsächliches Binary über PID/ExecStart ermitteln | Kein Beweis, dass der historische USB-/Hybridmanager aktualisiert wurde; sein Pfad darf nicht blind ersetzt werden |
| `system-backend/tbs-connect/README.md` | Bestehender TBS-Backendbaustein; langfristige Überführung oder Abgrenzung gegenüber Node Gateway; eigene UI geplant | Kein vollständiger Konfigurationssignatur-/USB-Manager |
| `system-backend/node-gateway/README.md` | Node-/Backend-WebSockets, Heartbeats, Telemetrie, ACK/Responses, Kommandotransport, API/WebUI, Events, `open_lab` | Fachlich verwandte neuere Infrastruktur, keine bewiesene Migration des alten Flask-Servers |
| `system-backend/node-gateway/src/main.rs` | Rust-Einstieg mit Configprüfung, `SharedGateway`, Start des Service-Monitors und expliziter OPEN-LAB-Warnung | Quelltextnachweis, kein hier erfolgtes Deployment |
| `system-backend/node-gateway/src/http.rs`, Zeilen 1–220 | Konkrete Routen für Health, Nodes, Core-Services, Events, Commands, Metriken und OpenAPI | Neuer API-Vertrag; nicht mit `/set/...` der historischen Flask-App austauschbar |
| `system-backend/node-gateway/src/service_monitor.rs` | Konfigurierter Hintergrundthread prüft HTTP-Ziele und veröffentlicht Core-Service-Status; Parser verlangt `http://` und expliziten Port | Bereits vorhandener Ansatz für zentrale Zustandsmatrix; prüft im gelesenen Code die HTTP-Statuszeile, nicht beliebige fachliche JSON-Readiness |
| `system-backend/hardware-gateway/README.md` | MQTT-/HTTP-Telemetrie, Geräte-/Heartbeatregistry, Temperatur-/Feuchte-/Spannungsschwellen, persistente Ereignisse, Port 8250; Hardwareausgänge standardmäßig deaktiviert | Mögliche Anbindung für spätere Bonereignisse; kein dort nachgewiesener Receipt-Druckdienst |
| `system-backend/provisioning-core/README.md` | Zentrale Geräte-/ISSI-, Gruppen-/GSSI- und Mitgliedschaftsverwaltung; Port 8125; verwendet Subscriber Core 8100 und Group Core 8110; OPEN LAB | Verwaltung ist inzwischen breiter beschrieben, aber nicht identisch mit dem Control-LXC auf 8080 oder BREW-`nodes.json` |

### 14.3 Heutige Schnittstellen, die eine Fortsetzung berücksichtigen sollte

Der gelesene Node-Gateway-Code implementiert unter anderem:

```text
GET  /health/live
GET  /health/ready
GET  /api/v1/status
GET  /api/v1/core-services
GET  /api/v1/nodes
GET  /api/v1/nodes/<node_id>
GET  /api/v1/events
GET  /api/v1/events/netcore?limit=100
POST /api/v1/nodes/<node_id>/ping
POST /api/v1/nodes/<node_id>/disconnect
POST /api/v1/nodes/<node_id>/commands
GET  /metrics
GET  /openapi.json
```

Die Commandroute nimmt ein Objekt mit optionalem `operator_id` und typisiertem `ControlCommand` entgegen; sie liefert bei angenommener Einreihung HTTP 202 mit `command_id`. **Einreihung ist keine bestätigte Ausführung.** Die README nennt das gemeinsame Ereignisformat `netcore-event-v1`, das sich als Integrationsgrundlage für den Receipt-Receiver anbietet. Diese Verwendung ist eine neue Roadmap-Ableitung, keine bereits implementierte Druckkopplung.

Auch bei diesen neueren Healthrouten darf Readiness nicht überinterpretiert werden: Die gelesenen einfachen Handler liefern 200 und feste positive Werte; der separate Service-Monitor liest 2xx-Statuszeilen. Weder die Dateibenennung noch der Endpunktname allein beweist eine vollständige fachliche Gesundheitsprüfung. Der Modus `open_lab` wird ausdrücklich ohne Authentifizierung/Tokens/TLS beschrieben; er ist keine Produktivhärtung. (R05–R09)

### 14.4 Spätere, separate Fortsetzungsquelle

Bereits vorhanden ist [Android-Control-App, Flask-API und Hybrid-Manager](2026-10-03_android-control-app-flask-api-und-hybrid-manager.md). In den gelesenen Abschnitten beschreibt dieses spätere Archiv ein Pluginlayout unter `/opt/netcore-tetra-control/plugins`, eine Route `/stations/set/...` und einen als `bluestation_hybrid_manager.sh` bezeichneten späteren Manager. Das sind **sekundäre Hinweise aus einem anderen Chatarchiv**, keine in diesem Lauf direkt aus der damaligen Liveinstallation gelesenen Dateien.

Für die Fortsetzung ist der Hinweis dennoch wichtig: Nicht die hier konservierte monolithische April-App ungeprüft über eine spätere Plugininstallation kopieren. Ebenso kann die hier im frühen Verifier belegte Datei `/var/lib/bluestation/last_version` nicht ohne Quellenabgleich auf jeden späteren Manager übertragen werden. Keiner der offenen Security-/Hybrid-/Backup-Punkte wird allein aufgrund eines ähnlich benannten neueren Bausteins als behoben markiert.

## 15. Tests und ihre Grenzen

### 15.1 Historische Nutzertests und Betriebsbestätigungen

| Test / Beobachtung | Ergebnis | Beleggrenze |
|---|---|---|
| Neuer FAT-Stick nach Formatierungsphase | UUID `D000-334F`, TYPE vfat, Partition `sda1` sichtbar | Keine vollständige Medienintegritätsprüfung |
| Dateiliste unter richtigem Mount | Config 7064 Bytes, Signatur 89 Bytes vorhanden | Dateiexistenz, nicht Signaturvalidität |
| USB einstecken / entfernen | Nutzer bestätigt Start bzw. Ende des Sendens | Damalige Einzelstationsbeobachtung, kein HF-Messprotokoll |
| Configkommentar ohne Neusignierung verändert | Journal meldet Signaturprüfung fehlgeschlagen; Dienst endet mit Fehler | Konkreter Manipulationsfall bestätigt, kein vollständiger Securitytest |
| Passende Config wiederhergestellt | Start/Stop per Stick funktioniert wieder | Bestätigt |
| `Restart=no` | Nutzer bestätigt eingetragen | Genaue geladene Unit nicht noch einmal als Dump geliefert |
| USB-Menü-Erkennung nach Disk-/Partitionskorrektur | Stick wird wieder gefunden | Kein vollständiger Mehrgeräte-Test |
| Falsche Datei mit Menü überschrieben | Nach Wiederherstellung `import_and_start.sh`: „geht wieder“ | Ursache und Reparatur konkret bestätigt |
| SSH-Remote-Befehl zum Backupserver | Nutzer bestätigt Erfolg | Shelländerungsbefehl selbst nicht belegt |
| Manueller Configbackup-Lauf | Erfolgslog vom 18. April 22:25:20–23 | Kein Restore, kein Vollimage |
| Zwei Cron-Einträge | Nutzer: beide drin | Kein Nachweis eines späteren automatischen Laufs |
| Backup-LXC | Nutzer: 70 GB Datenträger | Keine Belegungs-/Retentionmessung |
| Gunicorn-Datei | Executable im richtigen venv mit `ls` nachgewiesen | App/Keys mussten danach noch ergänzt werden |
| Control-UI | Screenshot mit RUN, Version und Links; spätere Mehrstations-UI vom Nutzer akzeptiert | Kein Nachweis der tatsächlichen Senderwirkung oder aller CRUD-Aktionen |
| Config-Download | 404 vom Nutzer gemeldet; fehlende Serverdateien erkannt | Späterer erfolgreicher Download nicht bestätigt |
| BREW-Health | Nutzer bestätigt „klappt“ | Keine Readiness-Negativtests |
| Finale Control-Health-Anpassung | Vollständiger Code ausgegeben | Kein gesondertes Ergebnisprotokoll danach |
| QR-/Logo-/Cutterdruck | Kein Hardwaretest | Ausschließlich Idee und Mockups |

**Nicht durchgeführt oder nicht belegt:** korrekt neu signierter falscher Device-ID-Test, korrekt signierter Rollbacktest, Stickklon, zwei gleichzeitige erlaubte Sticks, Abziehen des nicht aktiven Sticks, Widerruf eines laufenden Sticks, Netzverlust während RUN, STOP bei verlorener Config, persistente Lockwirkung, Reboot-/Powerlossfälle, Druckertest, vollständiger Restore und BREW-Protokollkonformität.

Die früher vorgeschlagenen Device-ID-/Versions-Negativtests waren teils unzureichend beschrieben: Ändert man die Datei ohne neue Signatur, testet man erneut nur die Signatur. Um Gerätebindung oder Mindestversion isoliert zu prüfen, braucht der absichtlich falsche Kandidat eine **gültige neue Signatur**. Das ist eine neue Testklarstellung.

### 15.2 Tatsächlich durchgeführte isolierte Archivprüfungen am 2026-10-03

Diese Tests liefen ausschließlich in der Arbeitsumgebung, nicht auf der TBS und nicht gegen das reale Netz:

| ID | Test | Tatsächliches Ergebnis |
|---|---|---|
| T-A01 | Ephemeren Ed25519-Key im Speicher erzeugen; Daten signieren; byteidentische Kopie mit derselben Signatur prüfen | Erfolgreich verifiziert; illustriert fehlenden physischen Klonschutz |
| T-A02 | An dieselben signierten Testbytes Kommentar anhängen bzw. LF in CRLF ändern | Beide Varianten lösen `InvalidSignature` aus |
| T-A03 | Bash 5.2.37: `EXIT`-Trap setzen, anschließend erfolgreich `exec /bin/true` | Exitcode 0 ohne Trap-Ausgabe |
| T-A04 | Historischen Node-ID-Regex auf `.`, `..`, `bs01`, `bs/01` anwenden; ungefährlichen temporären Pfad auflösen | `.`, `..`, `bs01` akzeptiert; Slashwert abgelehnt; `data/..` verlässt den vorgesehenen Unterordner |
| T-A05 | Historischen Vergleich mit `last_version=2` für Werte 0, 1, 2, 3 nachstellen | 0/1 abgelehnt, 2/3 angenommen |
| T-A06 | Länge der Testsignatur und Base64darstellung prüfen | 64 Rohbytes, 88 Base64zeichen, 89 Bytes mit LF |
| T-A07 | Vorhandene PNG/PDF-Dateien inventarisieren | Vier PNGs und 25 PDFs vorhanden; Bildmaße, Seitenzahlen und SHA-256 ermittelt |

Es wurden keine echten privaten Schlüssel ausgegeben oder persistiert. Diese kleinen Mechanismustests sind keine vollständige Ausführung der historischen Apps, keine Regressionstests des Rust-Repositories und keine Reparaturabnahme. Der GitHub-Quelltext wurde gelesen, aber nicht gebaut oder im realen System gestartet.

## 16. Offene Aufgaben, Roadmap-Kandidaten und Prioritäten

### 16.1 Historisch vereinbarte Reihenfolge

Die ausdrücklich genannte Reihenfolge war: **zuerst Serverfunktionen ausbauen**, danach die Basisstation auf zuverlässigen Netzwerk-/USB-Betrieb bringen. Beim Drucker: erst ein geeignetes günstiges Gerät beschaffen und per USB testen, anschließend intelligenter Receiver bzw. Netzwerk. Ein terminierter Releaseplan oder konkrete Issue-Nummern wurden nicht vereinbart.

### 16.2 Priorisierte Fortsetzung — aus den Befunden abgeleitet

Die Kennungen in dieser Tabelle sind lokale Archiv-Arbeitspakete, keine angelegten GitHub-Issues.

| Priorität / Kennung | Aufgabe | Status und Abnahmekriterium |
|---|---|---|
| P0 / SNAPSHOT | Tatsächlich installierte TBS-, Control- und BREW-Dateien mit Units, Interpreter und Dateihashes erfassen | Offen; vor erneutem Kopieren einer alten Gesamtdatei nötig, insbesondere wegen späterem Plugin-/Managerstand |
| P0 / CONTROL-TRUST | Anmeldung/RBAC, POST-Aktionen, CSRF, sichere IDs/Pfadgrenzen und geringere Schreibrechte entwerfen und implementieren | Nicht durch April-App erledigt; berechtigte und unberechtigte Zugriffe testen |
| P0 / STATE-STORE | Registry/State atomar und mehrworkersicher speichern; konsistente Generation für Daten und Signatur ausliefern | Offen; parallele Toggles, Fehler beim Schreiben und Neustart testen |
| P0 / HYBRID-POLICY | Netzwerkabwesenheit, ungültige Daten, STOP, Lock, Löschung, Lease, Offlinefreigabe und Bootzustand ausdrücklich unterscheiden | Nutzerfreigabe zur endgültigen Policy fehlt |
| P0 / HYBRID-AGENT | Dauerhafter Agent/Supervisor mit getrenntem Senderdienst, Polling, Timeouts, exakten verifizierten Bytes, Versionpersistenz und Quellenverfolgung | Einmalabruf ersetzen; kein Doppelprozess; spätere UI-Aktionen im Betrieb nachweisen |
| P0 / USB-LIFECYCLE | Verifikation auf lokaler Stagingkopie, atomare Versiondatei, definiertes Mountcleanup und passende udev-Zuständigkeit | Einzelsticktest reicht nicht; zwei Sticks, Widerruf und Netzwerkbetrieb abnehmen |
| P0 / BACKUP-RESTORE | Archivinhalt, fehlende Pfade, konsistente Systemrettung und Wiederherstellung testen | Erfolgreiche Übertragung allein nicht ausreichend |
| P1 / BREW-READINESS | Echte Liveness/Readiness, Dateivalidität, Reloadstatus/-alter, sparsame Diagnosepayload und passende Fehlercodes | Fehlende/ungültige `nodes.json` darf nicht unbegründet grün sein |
| P1 / BREW-AUTH | Gegenüber authentisieren und Autorisierung auch in Call-/Framepfaden durchsetzen; Widerruf vorhandener Sessions definieren | ISSI-Behauptung und unbenutztes Passwortfeld nicht als Authentifizierung bezeichnen |
| P1 / TELEMETRY | Tatsächlichen Agent-/Dienst-/Senderzustand mit Quittung und letzter Kontaktzeit an Control zurückmelden | Soll-RUN von ausgeführt/abgelehnt/stale/unknown unterscheiden |
| P1 / CONFIG-WORKFLOW | Bytegenaue Übertragung, lokales Configsignieren, geschützter Upload, Schema-/Metadatenprüfung und Download | 404-Auflösung plus echte Verifikation des verteilten Paars erforderlich |
| P1 / USB-REGISTRY | Menü mit einer autoritativen Registry, Backups, Lock, echter Synchronisierung und Geräteselektion | Alle Menüpunkte einschließlich laufendem Widerruf und Duplikaten testen |
| P1 / BACKUP-RETENTION | Messung eines Vollimages, 70-GB-Kapazitätsbudget, sichere getrennte Rotation, Platzwarnung und Logrotation | Gewählte Anzahl/Frist dokumentieren; fremde OPNsense-Backups nicht berühren |
| P1 / KEY-LIFECYCLE | Nicht überschreibende Keyinitialisierung, Sicherung, Fingerprints, Rotations-/Widerrufsverfahren und getrennte Rollen | Config-Key bleibt vom State-Key getrennt; Bootstrap manuell vertrauenswürdig |
| P1 / INTEGRATION | Historischen Control-/BREW-Ansatz mit Node Gateway, Provisioning Core, Subscriber/Group Core und gemeinsamem Eventmodell abgleichen | Keine zweite konkurrierende Registry ohne Zuständigkeitsentscheidung |
| P2 / RECEIPT-HW | Druckermodell, Schnittstellen, ESC/POS, Cutter, Druckbreite, Netzteil und Zustandsabfrage prüfen | Erst echter Text-/Bild-/QR-/Cuttest, dann Integration |
| P2 / RECEIPT-ENGINE | Intelligenter Receiver, Authentifizierung, Queue, Idempotenz, Dedupe, Ratenbegrenzung, Templates, Druckhistorie | Druckerausfall darf Senderbetrieb nicht beeinflussen |
| P2 / LOG-QR | Geschützte Log-/Eventseite und stabile Links implementieren; echte QR-Decodierung testen | Keine Platzhalterroute und keine Secrets im QR |
| P2 / LOGO | Original unverfälscht zuschneiden/invertieren und druckermodellgerecht rastern | Keine weitere ASCII-Annäherung oder generative Neuzeichnung als Original ausgeben |

### 16.3 Weitere erhaltene Nebenideen

Nicht vergessen, aber nicht als umgesetzt deklarieren: getrennte Import-/Senderdienste; zusätzliche lokale Freigabe per Taster/GPIO/NFC/Masterkarte; mehrere Config-Public-Keys mit eigenen Schlüsselpaaren für weitere Signierer; Key-ID/Manifest statt Probieren aller Schlüssel; Configverschlüsselung bei sensiblen Inhalten; Secure Boot, schreibgeschütztes System und Hardware-Vertrauensanker; LED-/GPIO-Anzeige der Configprüfung; kleine Statusdatei mit Ablehnungsgrund; WebUI-Anzeige der aktiven Quelle; automatisches Enrollment/Image/Wizard; Configversionierung und kontrollierte Rollouts; API-Authentifizierung bzw. mTLS/Pinning; Watchdog/Health; Telegram-/Mailwarnungen; Serverbackup; spätere Mehrdrucker-/Standortlogik und manuell auslösbare Statusbons.

Die frühe Idee einer ungeprüften oder automatisch beliebigen Fallback-Config wird **nicht** als gewünschte Ausnahme von Signatur-, Sperr- oder Versionsprüfung verstanden. Ebenso bedeutet „alles automatisch ziehen“ nicht, ungeprüften Code aus dem Netz auszuführen oder Vertrauensschlüssel beliebig zu ersetzen.

## 17. Quellen und Anhänge

### 17.1 Chatbelege

| ID | Sichtbarer Belegkomplex |
|---|---|
| C01 | Ursprüngliches USB-Startskript und udev-Regeln; Frage nach UUID-/Configfälschung |
| C02 | Windows-11-/Copy-Paste-/Kommentarvorgaben; Schlüssel-, Signier- und Verifiercode |
| C03 | `blkid`, `dmesg`, `fdisk` und Busy-/Formatierungsphase; neue UUID `D000-334F` |
| C04 | Dateiliste 7064/89 Bytes, fehlender Mount, konkreter Automount-/RO-Konflikt und Entscheidung Automount aus |
| C05 | Restart-Logs, UUID-Korrektur, bestätigter Start/Stop, Signatur-Negativtest und `Restart=no` |
| C06 | Mehrfach-UUIDs, Registrierung und Menü, USB-Erkennungskorrektur, Menü versehentlich im Startskript, Wiederherstellung mit „geht wieder“ |
| C07 | SD-Backup-/Copierprobleme; SFTP-Zielwunsch; SSH-Zugang und erfolgreicher Remote-Befehl |
| C08 | Komplettes Backupskript, Erfolgslog 18. April 22:25, bestätigte zwei Cron-Einträge und 70-GB-LXC |
| C09 | Hybridwunsch und korrigierte Netzwerkpriorität; Server zuerst; LXC-/venv-/Gunicorn-/Keypfad-Reparaturen |
| C10 | UI-Screenshot, Downloadlinks, Config-404 und ungeklärter erfolgreicher Configsignaturtransfer |
| C11 | Vollständige Control-Appfassungen mit Nodes, Bearbeiten/Löschen/Sperren, State-Signieren und API; UI zunächst akzeptiert |
| C12 | Vom Nutzer vollständig gepostete BREW-aiohttp-Quelle; vollständige Health-Erweiterung; „klappt“ |
| C13 | Vollständige finale Control-App mit JSON-Health-Auswertung für `10.0.1.163:8081/health` |
| C14 | Thermobondrucker, wenige relevante Ereignisse, Auto-Cutter, USB zuerst, Netzwerk später, intelligenter Receiver |
| C15 | 35 nummerierte Bonmotive, Original-Logo, abgelehnte ASCII-Versuche und zwei generierte Bildentwürfe |

Mangels Original-Chat-URL gibt es keine erfundenen Deep-Links auf diese Nachrichten. Diese Beleggruppen dienen der eindeutigen fachlichen Zuordnung innerhalb des erhaltenen Gesprächs.

### 17.2 Repositoryquellen der Archivprüfung

Alle nachfolgenden Quelltextlinks sind auf den Prüfcommit fixiert, soweit nicht ausdrücklich eine andere Quelle genannt ist.

- R01: [Archiving-Prüfcommit](https://github.com/JanHG98/netcore-tetra/commit/2b08bababf8b72bd2cb82d780a7ba0e526e1f1a9), Root-Tree `65913124d84998a680781cc9c1f89c0d93b31979`; Branch- und Repositorymetadaten über den GitHub-Connector gelesen.
- R02: [Defaultbranch-Refstand](https://github.com/JanHG98/netcore-tetra/commit/7137e0dd69877e1b604bf89148fd8b6b590c1a97); nur lesend betrachtet.
- R03: [Vorhandener Archivindex am Prüfcommit](https://github.com/JanHG98/netcore-tetra/blob/2b08bababf8b72bd2cb82d780a7ba0e526e1f1a9/Docs/archive/README.md); Compare in beide Richtungen und ausgewählte Root-/Backend-/Tools-Bäume gelesen.
- R04: [Basisstations-Updater, gelesener Bereich 1–180](https://github.com/JanHG98/netcore-tetra/blob/2b08bababf8b72bd2cb82d780a7ba0e526e1f1a9/install/update-basisstation.sh#L1-L180), Blob `16083c6868fd3b7f16e41f50b6c84704cd5b9a64`.
- R05: [Node Gateway README](https://github.com/JanHG98/netcore-tetra/blob/2b08bababf8b72bd2cb82d780a7ba0e526e1f1a9/system-backend/node-gateway/README.md), Blob `4537efb2891c74a22b08a0dae911b96f36f8db2e`.
- R06: [Node Gateway main.rs](https://github.com/JanHG98/netcore-tetra/blob/2b08bababf8b72bd2cb82d780a7ba0e526e1f1a9/system-backend/node-gateway/src/main.rs), Blob `9759f18519f5a492bacaa6ceaa324e2e5018449f`.
- R07: [Node Gateway http.rs, gelesener Bereich 1–220](https://github.com/JanHG98/netcore-tetra/blob/2b08bababf8b72bd2cb82d780a7ba0e526e1f1a9/system-backend/node-gateway/src/http.rs#L1-L220), Blob `4671bc7d32835d6c8c562a2979c8d1154735dae9`.
- R08: [Node Gateway service_monitor.rs](https://github.com/JanHG98/netcore-tetra/blob/2b08bababf8b72bd2cb82d780a7ba0e526e1f1a9/system-backend/node-gateway/src/service_monitor.rs), Blob `9e9de386b3b42f83a688a6fd5999e34001f7299e`.
- R09: [Hardware Gateway README](https://github.com/JanHG98/netcore-tetra/blob/2b08bababf8b72bd2cb82d780a7ba0e526e1f1a9/system-backend/hardware-gateway/README.md), Blob `78d0ee441373e1fdee7048d4bc4cc1f8e04938ff`; [Provisioning Core README](https://github.com/JanHG98/netcore-tetra/blob/2b08bababf8b72bd2cb82d780a7ba0e526e1f1a9/system-backend/provisioning-core/README.md), Blob `f44a48815437da922091f3c820274601b569a20a`.
- R10: [TBS Connect README](https://github.com/JanHG98/netcore-tetra/blob/2b08bababf8b72bd2cb82d780a7ba0e526e1f1a9/system-backend/tbs-connect/README.md), Blob `7b80ca3eee81053d0a83f1248688f1a0e68857b8`.
- R11: [Separates Android-/Control-/Hybrid-Archiv](2026-10-03_android-control-app-flask-api-und-hybrid-manager.md), insbesondere die gelesenen Bereiche 1–160 und 360–480; ausdrücklich sekundäre Fortsetzungsquelle aus einem anderen Chat.

In diesem Chat wurde kein bestimmter damaliger Featurebranch, PR oder Implementierungscommit für die USB-/Flask-/Brew-/Receipt-Skripte verlässlich genannt. Es werden deshalb keine nachträglichen PR-Nummern oder Implementierungs-SHAs erfunden.

### 17.3 Externe Grundlagen für die ausdrücklich getrennte Archivprüfung

Diese Quellen wurden zur Einordnung einzelner Mechanismen nachgelesen. Sie sind keine Belege für eine historische erfolgreiche Ausführung:

- E01: [cryptography: Ed25519 signing](https://cryptography.io/en/latest/hazmat/primitives/asymmetric/ed25519/) — Signieren/Prüfen von Bytes und 64-Byte-Signatur.
- E02: [Python: venv](https://docs.python.org/3/library/venv.html) — getrennte Interpreterumgebung; expliziter Interpreterpfad statt vorausgesetzter Aktivierung.
- E03: [PyPA: Externally Managed Environments](https://packaging.python.org/en/latest/specifications/externally-managed-environments/) — distributionsverwaltetes Python und virtuelle Umgebungen.
- E04: [GNU Bash: Bourne Shell Builtins](https://www.gnu.org/software/bash/manual/html_node/Bourne-Shell-Builtins.html) — `exec` und `EXIT`-Trap; zusätzlicher isolierter Test in Abschnitt 15.
- E05: [Ubuntu-Paketdokumentation: systemd.device](https://manpages.ubuntu.com/manpages/bionic/man5/systemd.device.5.html) — Geräteaktivierung und `SYSTEMD_WANTS`; ältere, ausdrücklich versionsgebundene Dokumentation für den hier relevanten Mechanismus, keine Aussage zur installierten Pi-Version.
- E06: [Debian-Paketdokumentation: systemd.unit](https://manpages.debian.org/bullseye/systemd/systemd.unit.5.en.html) — Startlimitparameter im Unit-Kontext; keine Behauptung, Debian bullseye sei auf dem Nutzergerät installiert.
- E07: [Flask: Security Considerations](https://flask.palletsprojects.com/en/stable/web-security/) — CSRF, Webzugriff und anwendungsseitige Sicherheitsverantwortung.

### 17.4 Bildanlagen

Die Anlagen wurden nicht zusätzlich als Binärdateien in diesen Archivcommit kopiert. Dateinamen, Maße und vollständige SHA-256 dienen der späteren eindeutigen Zuordnung. Die ursprünglichen Containerpfade sind temporäre Arbeitsumgebungspfade, keine erfundenen dauerhaften Repositorylinks.

| ID | Datei | Maße | Einordnung |
|---|---|---|---|
| A01 | `3d16b22e-41d3-41ae-90ca-344d06d79a42.png` | 1920 × 686 | Nutzer-Screenshot der frühen Control-WebUI mit TBS01/RUN/Version 12 und Dateilinks |
| A02 | `Dunkles Design plus Text.png` | 2048 × 2048 | Original-Logo; maßgebliche Quelle statt ASCII oder neu gezeichneter Symbole |
| A03 | `a_clean_black_and_white_receipt_printout_style_ima.png` | 1024 × 1536 | Generierter vertikaler Bonentwurf mit Startdaten, Kommentar und QR-Motiv |
| A04 | `a_clean_black_and_white_graphic_logo_image_on_a_wh.png` | 1254 × 1254 | Zweiter generierter Entwurf mit Logo-/Bonlayout und Schlusszeile |

```text
A01 SHA-256 d0a083f32b9fe4689c5a8524f8f69122f65fa17735dfca493979fcdaf8103303
A02 SHA-256 f90a06dfc8afabcca0723b1ae97554e661463e8a23bc9fb0827041f045c1ec9c
A03 SHA-256 43352898063e1b05d64e916b03f6ca1c5e0d731a317c69ab3b6c5c0685a67e47
A04 SHA-256 08c78ca7574bba531a990be58a6f4fb03fa0cbbfbd092b26723c7ac203273b0d
```

A03/A04 tragen beispielhafte Zeit-, Versions- und Logadressdaten. Diese Daten beschreiben keine gemessene Station. Eine Dekodierung der QR-Muster wurde nicht durchgeführt.

### 17.5 PDF-Anlagenbestand

Alle nachfolgenden Dateien waren tatsächlich vorhanden. Erfasst wurden Deckblatt-/Versionsangaben und Seitenzahlen. **Keine vollständige Inhalts-/Konformitätsprüfung und keine Behauptung aktueller Normgeltung.** Insbesondere Drafts bleiben Drafts; die Sammeldatei wird nicht ungeprüft als neuere konsolidierte Norm behandelt.

| Datei | Identität / Thema laut Deckblatt bzw. verfügbarer Titelinformation | Seiten |
|---|---|---:|
| `en_3003920308v010401p.pdf` | EN 300 392-3-8 V1.4.1 (2020-04), Generic Speech Format Implementation | 22 |
| `en_30039209v010701p.pdf` | EN 300 392-9 V1.7.1 (2020-04), General requirements for supplementary services | 46 |
| `ts_10081201v020205p.pdf` | TS 100 812-1 V2.2.5 (2003-10), SIM-ME/UICC physical and logical characteristics | 8 |
| `en_3003921201v010202p.pdf` | EN 300 392-12-1 V1.2.2 (2007-08), Call Identification, Stage 3 | 56 |
| `en_3003920304v010301p.pdf` | EN 300 392-3-4 V1.3.1 (2010-08), ANF-ISISDS | 28 |
| `en_3003921117v010102p.pdf` | EN 300 392-11-17 V1.1.2 (2002-01), Include Call, Stage 2 | 18 |
| `en_3003921114v010101p.pdf` | EN 300 392-11-14 V1.1.1 (2002-07), Late Entry, Stage 2 | 23 |
| `es_20081202v020401m.pdf` | Final draft ES 200 812-2 V2.4.1 (2005-08), TSIM application | 139 |
| `es_20081201v020205p.pdf` | ES 200 812-1 V2.2.5 (2003-12), TSIM-ME/UICC physical and logical characteristics | 8 |
| `en_300812v020101p.pdf` | EN 300 812 V2.1.1 (2001-12), Security aspects / SIM-ME interface | 156 |
| `en_3003921101v010201p.pdf` | EN 300 392-11-1 V1.2.1 (2004-01), Call Identification, Stage 2 | 44 |
| `en_3003921006v010401p.pdf` | EN 300 392-10-6 V1.4.1 (2006-08), Call Authorized by Dispatcher, Stage 1 | 20 |
| `en_3003921018v010301p.pdf` | EN 300 392-10-18 V1.3.1 (2003-10), Barring of Outgoing Calls, Stage 1 | 17 |
| `en_3003921216v010400a.pdf` | DRAFT EN 300 392-12-16 V1.4.0 (2026-03), Pre-emptive Priority Call, Stage 3 | 67 |
| `en_30039201v010601p.pdf` | EN 300 392-1 V1.6.1 (2020-04), General network design | 182 |
| `ets_30039214e01v.pdf` | Final draft prETS 300 392-14 (1997-09), PICS proforma | 61 |
| `en_30039207v030501p.pdf` | EN 300 392-7 V3.5.1 (2019-07), Security | 216 |
| `en_30039401v030301p.pdf` | EN 300 394-1 V3.3.1 (2015-04), Radio conformance testing | 169 |
| `en_3003920313v010201p.pdf` | EN 300 392-3-13 V1.2.1 (2020-04), Transport-independent ANF-ISIGC | 191 |
| `en_30039502v010303p.pdf` | EN 300 395-2 V1.3.3 (2025-02), TETRA speech codec | 94 |
| `en_3003920303v010301p.pdf` | EN 300 392-3-3 V1.3.1 (2011-11), ANF-ISIGC | 251 |
| `en_30039205v020701p.pdf` | EN 300 392-5 V2.7.1 (2020-04), Peripheral Equipment Interface | 320 |
| `en_3003920315v010500a.pdf` | Draft EN 300 392-3-15 V1.5.0 (2026-04), ANF-ISIMM | 380 |
| `en_30039202v030801p.pdf` | EN 300 392-2 V3.8.1 (2016-08), Air Interface | 1445 |
| `ETSI.pdf` | Sammeldatei; beginnt mit EN 300 812 V2.1.1, nicht vollständig in Bestandteile zerlegt | 4100 |

Die Normen liefern in dieser Dokumentation **keinen** Nachweis dafür, dass das provisorische BREW-Skript, die HTTP-Signaturverteilung, der USB-Start oder ein künftiger Receipt-Server normkonform sei. Solche Nachweise waren nicht Gegenstand der damaligen Tests.

## 18. Abschluss und Wiederaufnahme

Der damalige praktische Erfolg bleibt erhalten: signierter USB-Start/Stop funktionierte, eine manipulierte Config wurde abgelehnt, der Configbackup-Upload lief durch, die Control-WebUI wurde aufgebaut und der BREW-Health-Endpunkt antwortete. Ebenso bleiben die offenen Punkte erhalten: Hybridmanager statt Einmalabruf, belastbare Sperr-/Offlinepolitik, abgesicherte und atomare Serververwaltung, echte Readiness, vollständiger Restore sowie Druckerbeschaffung und Receipt-Implementierung.

Bei einer Wiederaufnahme zuerst den **tatsächlich installierten heutigen Stand** sichern und mit diesem Archiv sowie dem separaten späteren Control-/Android-Archiv vergleichen. Keine alte komplette `app.py` über eine neuere Plugininstallation schreiben, keine Schlüssel neu erzeugen, um Pfad-/venv-Fehler zu kaschieren, und keine Mindestversion pauschal auf 0 setzen. Vor einer erneuten RF-Aktivierung die definierte Freigabe- und Stopkette unter kontrollierten Testbedingungen abnehmen.

Dieser Archivauftrag verändert nur Dokumentation unter `Docs/archive/`. Er stellt keinen neuen Betriebsstand her, führt keinen Dienstneustart durch, legt keine produktiven Schlüssel an und enthält keine Zugangsdaten. Der konkrete Speichercommit ergibt sich aus der Git-Historie dieser Datei und wird in der Abschlussmeldung genannt; eine zirkuläre Selbstreferenz im Dateiinhalte-Commit wird vermieden.
