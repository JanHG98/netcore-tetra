# Brainstorming: Control Room, Windows-UI, RBAC, Status-Tableau und Directory-API

> **Arbeitsstand:** Historische Entwicklungspakete, beobachteter Betrieb und Repository-Code vom 03.10.2026 sind getrennt zu betrachten. Die zuletzt vorgeschlagenen Änderungen wurden auf den Zielsystemen nicht vollständig bestätigt.
>
> **Offen:** Namensauflösung sowie Konsistenz von Statustext, Statusnummer und Farbe wurden nicht vollständig Ende-zu-Ende abgenommen. Die Directory-Anbindung im Repository ist ein Codebefund, kein Betriebsnachweis. Die historische Anleitung, das vollständige LXC-Repository zu löschen und nur das Komponenten-ZIP zu entpacken, ist falsch und darf nicht verwendet werden.

## 1. Rahmen und Quellenstand

| Merkmal | Angabe |
|---|---|
| Projekt | NetCore-Tetra |
| Thema dieses Archivs | Aufbau des Control-Room-Kerns; native Windows-Bedienoberfläche; Benutzeranmeldung/RBAC; Multi-Monitor-Fenster; Live-Karte; Status-Tableau; Anbindung des vorhandenen NetCore Directory Servers; wiederholte Build- und Integrationsfehler |
| Historische Zeitanker | Aus den erhaltenen Terminalausgaben: 30.06.2026 und 01.07.2026. Das ist keine gesicherte Datierung sämtlicher Nachrichten. |
| Erstellungsdatum | 03.10.2026; lokale Zeitzone Europe/Berlin, UTC+02:00 |
| Zielrepository | `JanHG98/netcore-tetra` |
| Geprüfter Branch | `Archiving` |
| Geprüfter Archivbranch vor dieser Dokumentation | `df575519c7cf066d771511743175f22fede0826d` |
| Zugehöriger Tree | `2e2cafefda12ed804ab756bbb724cc2e9cc888ab` |
| Zusätzlich gelesener Default-Branch | `main` bei `6aa9be8f74ab731f72dc133a5f8e90c5018c626d` |

Der initial gelesene Branch `Archiving` lag zwölf Commits vor dem geprüften `main`, ohne Rückstand. Dieser Vorsprung bestand **nicht ausschließlich aus Archivdateien**: Er enthielt auch bereits vorhandene Dashboard-/Dienst-WebUI-Änderungen und Tests. Der Branch-Vorsprung lässt sich deshalb nicht pauschal als reine Archivierung einordnen. [R01] [R02] [R03]

### 1.1 Was zugänglich war – und was nicht

Grundlagen sind Anforderungen, Terminalausgaben, Screenshots, Codeänderungen und spätere Korrekturen. Erhalten sind sechs Textdateien, die ZIP-Paketinventare und relevante Repository-Dateien. Beim letzten ZIP wurden Integrität, Inhalt und der gemeldete `resolved.len()`-Fehler gezielt geprüft.

Frühe Implementierungs-, Installations- und Diagnoseabschnitte sind nur teilweise erhalten. Generator-Skripte oder Fertigmeldungen ersetzen fehlende Compiler- und Testausgaben nicht.

Die beigefügten ETSI-Dateien wurden für diese Entwicklungsnotizen **als Referenzbestand eingeordnet**, nicht vollständig fachlich geprüft. Es gibt im sichtbaren Verlauf keinen belastbaren Nachweis, dass die konkreten UI-/Directory-Fixes anhand dieser Normen validiert wurden. Weder die Software noch die Statusnummernzuordnung erhalten durch ihre Beilage einen Konformitätsnachweis.

Produktive Konfigurationen, Prozesse, Datenbanken, Directory-Inhalte und Funkgeräte waren für die Quellenprüfung vom 03.10.2026 nicht direkt erreichbar. Repository-Lesezugriffe waren möglich; der lokale Clone scheiterte an der GitHub-Namensauflösung. Ein vollständiger lokaler Repository-Build wurde deshalb nicht ausgeführt.

### 1.2 Verwendete Statusbegriffe

| Status | Bedeutung in diesem Dokument |
|---|---|
| **Idee** | Angeregt, aber nicht als fertige Anforderung oder Umsetzung nachgewiesen. |
| **Beschlossen/geplant** | Ausdrücklich verlangt oder ausdrücklich bestätigt; Umsetzung kann noch fehlen. |
| **Implementiert – Artefakt** | In untersuchtem ZIP beziehungsweise sichtbarem erzeugtem Quelltext enthalten; nicht automatisch im Git oder auf einem Zielgerät vorhanden. |
| **Implementiert – Repository** | Am ausdrücklich genannten Commit im Quellcode nachvollzogen. |
| **Getestet** | Ein konkreter Test und sein Ergebnis sind sichtbar. Umfang und Grenzen werden benannt. |
| **Im Betrieb bestätigt** | Betriebslog, Ausgabe oder Screenshot belegt das betreffende Verhalten auf einem Zielsystem. Das gilt nur für das beobachtete Verhalten, nicht pauschal für das gesamte Paket. |
| **Unbestätigt/offen** | Behauptung, Diagnose oder Funktion ohne ausreichenden Nachweis beziehungsweise mit späterem Gegenbefund. |
| **Überholt/zurückgezogen** | Durch spätere Projektentscheidung ersetzt oder fachlich/technisch nicht als weiterer Arbeitsweg geeignet. |

## 2. Ziel, Ausgangslage und Ergebnis in einem Überblick

Aus einer zunächst per CLI bedienten zentralen Control-Room-Komponente sollte ein tatsächlicher Leitstellenarbeitsplatz entstehen: Ein Linux-LXC sammelt Basisstationsdaten, speichert Ereignisse und vermittelt Steuerbefehle. Eine native Windows-Anwendung stellt diese Daten modular dar, erlaubt berechtigte Bedienhandlungen und verteilt Module auf mehrere Monitore. Ein vorhandener Directory Server soll Namen, Gerätetypen, Gruppen und Statusdefinitionen liefern, ohne diese Daten auf mehreren Geräten nochmals zu pflegen.

Die Arbeit verlief in drei Schwerpunkten:

1. **Backend und Betrieb:** Node-Anbindung, HTTP-Snapshots, Operator-CLI, SQLite, systemd, Authentifizierung und Berechtigungen.
2. **Bedienoberfläche:** Installation unter Windows, echte OS-Fenster, responsive Gestaltung, Kartenkacheln, Interaktion, Geräteinformationen und Clustering.
3. **Fachliche Datenintegration:** Status-Tableau, Gruppen-/Einzelgeräteanzeige, Namensauflösung, Status-SDS und schließlich der tatsächlich notwendige Abruf des vorhandenen Directory Servers.

### 2.1 Belastbar erreicht

Der historische Verlauf belegt einen laufenden Control-Room-Dienst mit verbundenem TBS-Node, abrufbaren API-Daten und aktivierter SQLite-Persistenz. Eine Command-Historie überstand einen Dienstneustart. Die native Windows-UI wurde gestartet; Screenshots zeigen reale Karten, Teilnehmer-/Statusansichten, Cluster und später getrennte Gerätekarten. Eine zwischenzeitlich wegen Node-Authentifizierung offline erscheinende Basisstation wurde ausdrücklich wieder als online bestätigt.

### 2.2 Nicht als abgeschlossen anzusehen

Die korrekten Directory-Namen waren bis zu den letzten eindeutigen UI-Rückmeldungen weiterhin nicht vorhanden. Status-SDS wurden schließlich erkannt und als Text dargestellt; die daneben angezeigte Nummer und Farbe entsprachen jedoch nicht zuverlässig der erwarteten Definition. Die abschließenden „immernoch“-Nachrichten enthielten teilweise keine neue Fehlerausgabe. Daraus lässt sich weder ein wiederholter identischer Compilerfehler noch ein bestimmter Laufzeitfehler sicher ableiten.

Die Labels „Final-Fix“, „Directory-First“, „Deep-Index“ und „Verified“ waren Bezeichnungen der gelieferten Entwicklungsstände. Sie sind **keine Abnahmekriterien und keine belastbaren Qualitätsnachweise**.

## 3. Endgültige Anforderungen und Entscheidungen aus dieser Entwicklungsphase

### 3.1 Rollen der Systeme

**Beschlossen/geplant:** Die Basisstation betreibt den Funkstack und ihre vorhandene lokale Weboberfläche. Der Control-Room-LXC soll für diesen historischen Entwurf Backend und CLI bleiben; die eigentliche Operator-Oberfläche soll auf dem Windows-Rechner laufen. Die GUI soll nicht auf der TBS oder dem LXC gebaut werden müssen.

Der TBS-Maschinentoken darf ausdrücklich in der `config.toml` der TBS verbleiben. Dafür wird kein auffälliges Token-Bedienfeld in der Basisstations-WebUI benötigt. Menschliche Benutzer und Maschinenanmeldung sind getrennte Sachverhalte.

**Am 03.10.2026 geprüfte Abweichung:** Der geprüfte Repository-Stand enthält inzwischen eine Control-Room-WebUI und weitere Backend-Funktionen. Das wird in Abschnitt 12 als spätere Repository-Entwicklung dokumentiert und nicht rückwirkend zur ursprünglichen Entscheidung dieses Vorhabens erklärt.

### 3.2 Menschlicher Login und RBAC

Die zuerst aufgebaute Bedienung mit Operator-/Admin-Tokens wurde durch eine spätere ausdrückliche Projektentscheidung ersetzt:

- Beim Start der Windows-EXE sind Benutzername und Passwort manuell einzugeben.
- Die Anwendung zeigt Funktionen entsprechend der Benutzerrolle.
- `viewer` darf lesen, aber keine administrativen Benutzermenüs sehen.
- Operatoren dürfen die vorgesehenen Bedienbefehle ausführen; administrative Benutzerverwaltung bleibt Admins vorbehalten.
- Admins dürfen insbesondere Benutzer anlegen, bearbeiten, aktivieren/deaktivieren und löschen.
- Die Prüfung muss serverseitig erfolgen; ausgeblendete Menüs allein stellen keine Zugriffskontrolle dar.
- Abgetrennte Modulfenster dürfen die RBAC-Ausblendung nicht umgehen.

Der TBS-Token für `/node` bleibt dabei Maschinenauthentifizierung. „Weg vom Token-Kram“ bezog sich auf die menschliche Bedienung, nicht auf eine pauschale Abschaffung sämtlicher Machine-to-Machine-Authentifizierung.

**Historisch bestätigt:** Loginmaske und angemeldete Admin-UI sind sichtbar. Eine vollständige Negativtestserie aller Rollen und Endpunkte ist nicht belegt.

### 3.3 Das Directory ist ein bestehender Server – keine zweite Stammdatendatei

Dies ist die wichtigste mehrfach wiederholte Korrektur der Projektplanung:

> Namen, Statusdefinitionen und Gruppenzuordnungen sollen aus der API des vorhandenen NetCore Directory Servers kommen. Sie sollen nicht auf Windows oder im Control-Room-LXC erneut in TOML gepflegt werden.

Erforderlich sind insbesondere Gerätenamen, Kurzbezeichnungen, Gerätetypen, Gruppen-/Statusgruppenbezeichnungen, Mitgliedschaften, Statustexte und die dazugehörigen Codes/Farben. Ein lokaler Clientcache ist als technische Umsetzung denkbar, aber keine zusätzliche administrative Datenquelle. Die Pflege bleibt zentral.

Die in den Arbeitsnotizen vorgeschlagenen lokalen TOML-Einträge und die später als Test angebotenen manuellen JSON-Importe erfüllen diese Anforderung nicht als Dauerlösung. Auch ein neuer Import-Endpunkt reicht nicht, solange keine tatsächliche Quelle ihn automatisch befüllt.

### 3.4 Windows-UI, Bedienbarkeit und Multi-Window

Die gesamte Anwendung soll responsive sein: passende Eingabefeldgrößen, keine abgeschnittenen Menüleisten, kein stufenweises Driftverhalten und keine unbenutzbare Verteilung von Schaltflächen. Die mehrfach gezeigte diagonale beziehungsweise treppenförmige Anordnung war ausdrücklich ein Fehler, keine gewünschte Gestaltung.

Das obere Menü muss Aktionen tatsächlich auslösen. Reine Dekoration oder Schaltflächen ohne Funktion wurden beanstandet. Festgelegt ist eine aufgeräumte Arbeitsansicht ohne ständig sichtbare API-/Profilangaben und ohne einstellbare Refresh-Bedienung; die Aktualisierung soll fest bei einer Sekunde liegen.

Jedes Modul soll als **echtes Betriebssystemfenster** aus dem Hauptfenster herauslösbar sein. Das Verschieben muss über alle Monitore funktionieren. Nur innerhalb des Hauptfensters verschiebbare GUI-Panels genügen nicht.

### 3.5 Karte und Standorte

**Beschlossen/geplant:** Eine echte Online-Karte statt weißer Pseudofläche; Verschieben mit der Maus; dosiertes Mausrad-Zoom; flüssige Bedienung; Geräteinformationen beim Klick. Die beobachteten ungefähr acht Zoomstufen pro Mausradraster sind zu empfindlich.

Im Tab `Standorte` bleibt die Tabelle, im Tab `Karte` die Kartenansicht. Die Karte soll nicht zusätzlich unter `Standorte` und die komplette Standortliste nicht noch einmal unter der Karte erscheinen.

Pro Gerät soll standardmäßig nur die jüngste Position sichtbar sein. Historische Punkte dürfen nicht als aktuelle Geräte weiterleben. Infrastruktur und fehlerhafte Einträge sollen nicht ungeprüft als Teilnehmer auftauchen. Die Bereinigung muss anhand des Datenmodells erfolgen; eine kleine numerische ISSI ist für sich allein noch kein Beweis eines ungültigen Geräts.

Bei mehreren Geräten an einem Ort werden Cluster mit Anzahl, Aufklappen beziehungsweise Spiderfy und individuelle Details benötigt. Identische Koordinaten dürfen nicht zu unlesbaren Stapeln führen. Namen aus dem Directory sollen sowohl Marker als auch Cluster- und Detailansichten beschriften.

### 3.6 Status-Tableau und Gruppen

Als visuelle Referenz wurden SELECTRIC-Statusanzeigen eingebracht. Gefordert ist eine kompakte Leitstellenansicht mit gut lesbarem Namen und einem passenden Statusfeld. **Text, Zahl und Farbe müssen zusammengehören.** Die im Beispielbild enthaltenen fremden Namen und Statuspläne sind keine Stammdaten dieses Projekts.

Die spätere Präzisierung ersetzt die zuerst gelieferte Sammelliste:

- Jedes Gerät **ohne** zugehörige Status-/Gerätegruppe erhält eine eigene Kachel beziehungsweise ein eigenes Tableau-Element.
- Eine Gruppe erhält ein Element mit ihrem Gruppennamen; enthaltene Geräte sollen über Details beziehungsweise Aufklappen erkennbar sein.
- Nicht alle gruppenlosen Geräte sollen in einem einzigen Block `Einzelgeräte` verschwinden.
- Die Elemente sollen per Maus verschoben und sinnvoll angeordnet werden können.
- Die Legende soll sauber ausgerichtet sein, nicht in einem Bogen oder durch fortlaufendes Layoutwachstum driften.

Die Anforderung beschreibt einzeln angeordnete „Tabs“; die Implementierungen verwenden Karten/Kacheln. Ob zusätzlich individuell abtrennbare OS-Fenster pro Gerät gewünscht sind, wurde nicht abschließend festgelegt; die sichere Anforderung ist die individuelle Anzeige und Maus-Anordnung innerhalb des Tableaus.

### 3.7 Lieferung, Git und Anleitungen

Für Lieferungen sind komplette betroffene Dateien in **einem ZIP** verbindlich, keine Patchdateien und keine verteilten manuellen Quellcodekorrekturen auf vielen Geräten. Jede Lieferung benötigt eine vollständige, systemspezifische Schrittfolge: Wo entpacken beziehungsweise aus Git beziehen, welche Komponenten bauen, welche Prozesse schließen, welcher Dienst neu starten, welche Version prüfen.

Alte UI-Binaries sollen kontrolliert entfernt oder ersetzt werden. Dies autorisiert nicht das Löschen des gesamten Repositorys oder produktiver Konfigurationen und Datenbanken. `.vs` und Build-Erzeugnisse sollen nicht versioniert werden. Nach einer `.gitignore`-Bereinigung waren nur noch sieben statt ungefähr 1.800 Änderungen sichtbar; das ist eine historische Betriebsbeobachtung, kein am 03.10.2026 geprüfter Repository-Diff.

## 4. Historische Architektur und Datenfluss

### 4.1 Zielbild der Komponenten

```text
Funkgeräte
    ↕ TETRA
Basisstation / FlowStation
    ↕ Node-WebSocket: Hello, Heartbeat, Telemetrie, Befehle/Antworten
Control-Room-LXC
    ├─ Zustand, Ereignisse, SDS-/Positionshistorie
    ├─ SQLite-Persistenz und Command-Audit
    ├─ HTTP-API für Operator-CLI und Windows-UI
    └─ Directory-Client zum vorhandenen NetCore Directory Server
                ↕ HTTP-API
        zentrale Stammdaten / Statusdefinitionen

Windows-Operator-UI
    ↕ HTTP-API des Control Rooms
    ├─ Rollenabhängige Module
    ├─ unabhängige OS-Modulfenster
    ├─ Karte / Cluster / Geräteinformationen
    └─ Status-Tableau
```

Der wichtige Unterschied: Ein TBS-Node-WebSocket überträgt nicht allein dadurch sämtliche Directory-Stammdaten. Ebenso ist `/api/directory` am Control Room nicht automatisch die API des eigenständigen Directory Servers. Der zwischengeschaltete Dienst muss diese Daten tatsächlich beziehen und in der richtigen Struktur weitergeben.

### 4.2 Backend-Bausteine im historischen Entwurf

| Baustein | Verantwortung |
|---|---|
| `bins/netcore-control-room/src/main.rs` | Argumente, Konfiguration, Authentifizierung/Persistenz initialisieren, Server starten |
| `config.rs` | Server-, Auth- und Persistenzeinstellungen; ursprünglich auch statischer Directory-Wert |
| `server.rs` | TCP-Listener; Unterscheidung HTTP und WebSocket; Weitergabe gemeinsamen Zustands |
| `http.rs` | HTTP-Routen, Rollenprüfung, Command-Annahme, später Directory-Import und Upstream-Abruf |
| `ws.rs` | Node- und UI-WebSocket; Subprotokoll, Empfang, Ping, Reconnect-Schnittstellen |
| `state.rs` | Node-/Teilnehmer-/Gruppen-/Rufzustand, Ereignisse, SDS, Positionen und Command-Historie |
| `persistence.rs` | SQLite-Verbindung, Schema und Wiederherstellung der gespeicherten Historie |
| `auth.rs` | Historisch zunächst Tokenrollen; später Benutzer/Passwort und Rollen |
| `system-backend/control-room/operator/src/main.rs` | Native CLI für Anzeige, Commands, Profile und Administration |
| `system-backend/control-room/ui/src/main.rs` | Native egui/eframe-Anwendung; im untersuchten Artefakt sehr großer monolithischer Einstiegspunkt |

Die in den Quelltexten verwendeten Typen umfassen unter anderem `SharedControlRoom`, `ControlRoomState`, `ControlRoomCodecJson`, `ControlCommandEnvelope`, `ControlCommandAck`, `ControlResponseEnvelope`, `NodeTelemetryEnvelope`, `NodeToControlRoomMessage` und `ControlRoomToNodeMessage`. Die Zustandsfreigabe erfolgt unter anderem über `Arc<Mutex<...>>` und Nachrichtenkanäle.

Als Node-Nachrichten waren `Hello`, `Heartbeat`, `Telemetry`, `ControlAck`, `ControlResponse` und `Error` sichtbar. Die Telemetrie umfasst Registrierung/Abmeldung, Gruppenmitgliedschaften, RF-Werte, Rufe, SDS und weitere Systemzustände. Die Typdefinitionen liegen in den TETRA-Crates des vollständigen Repositorys; sie sind im letzten Komponenten-ZIP nicht vollständig mitgeliefert.

### 4.3 Technologie und Build-Abhängigkeiten

Im untersuchten letzten ZIP baut die UI als eigenständiges Cargo-Projekt mit eigenem `[workspace]` über `--manifest-path`. Das UI-Paket trägt weiterhin die Cargo-Version `1.3.0`; die sichtbaren UI-Bezeichnungen `v5.x` sind davon getrennte Zeichenketten. Deshalb beweist eine Compilerzeile mit `netcore-control-room-ui v1.3.0` nicht, welche UI-Unterversion im Fenster läuft.

Die UI-Manifeste nennen `eframe`/`egui_extras` 0.27; Buildausgaben zeigten konkret 0.27.2. Weitere Abhängigkeiten sind `reqwest` 0.12 mit Blocking/JSON/rustls, `serde`, `serde_json`, `toml` 0.8, `dirs-next` 2 und `image` 0.25 mit PNG/JPEG. Der Operator verwendet ebenfalls `reqwest` und ist in die Root-Workspace-Vererbung eingebunden. Der Kern verwendet unter anderem `tetra-core`, `tetra-entities`, `clap`, `chrono`, `tungstenite`, `uuid`, `tracing`, `rusqlite` 0.32 mit gebündeltem SQLite sowie Auth-Hilfsbibliotheken.

Im Kernmanifest des letzten ZIP steht `tetra-entities` mit `default-features = false`. Außerdem ist ein leeres Kompatibilitätsfeature `asterisk` vorgesehen. Hintergrund ist die Vermeidung einer unnötigen RF-/Codec-Abhängigkeitskette beim Bau des LXC-Dienstes. Das erklärt den Ansatz, ersetzt aber keinen vollständigen Feature-Graph- und Buildtest.

## 5. Historische System-, Pfad- und Parameterübersicht

Alle Angaben in diesem Abschnitt sind historische Beobachtungen oder explizit gekennzeichnete Vorgaben. Sie sind keine Abfrage der am 03.10.2026 tatsächlich laufenden Infrastruktur.

| Element | Wert / Bedeutung | Nachweisgrenze |
|---|---|---|
| Control-Room-LXC | `10.0.1.25` | In Operator-Aufrufen und UI sichtbar |
| Control-Room-HTTP/WS | TCP `9010`; historisch `0.0.0.0:9010` | Startlog und erfolgreiche lokale API-Aufrufe |
| Node-Pfad | `/node` | Startlog, akzeptierte und zurückgewiesene WebSockets |
| UI-WebSocket-Pfad | `/ui` | Vorhandener Protokollpfad; nicht gleichbedeutend mit einer fertigen Browser-UI |
| TBS | `10.0.1.20` | Peeradresse im LXC-Log |
| TBS-Hostname | `SRV-M-TBS-01` | Shell-/Journalpräfix |
| Historische Node-ID | `SRV-M_TBS-01` | API/Commands; Unterstrich und Bindestrich nicht stillschweigend vereinheitlichen |
| Control-Room-Repo auf LXC | `/opt/netcore/flowstation` | Arbeitsverzeichnis und systemd-ExecStart |
| TBS-Repo | `/home/jan/flowstation` beziehungsweise `~/flowstation` | Terminal und TBS-Log |
| Windows-Repo | `C:\Users\janho\Documents\GitHub\flowstation` | Compiler-/CMD-Ausgaben |
| Dienst | `netcore-control-room.service` | systemd bestätigt |
| Dienstkonto | `netcore` | Anlage und Unit gezeigt |
| Backendkonfiguration | `/etc/netcore-control-room/control-room.toml` | ExecStart und Nutzerbefehle |
| Umgebungsdatei | `/etc/netcore-control-room/control-room.env` | Tokenphase und Unit-/Drop-in-Diagnose |
| SQLite | `/var/lib/netcore-control-room/control-room.sqlite3` | Startlog und CLI-Abfrage |
| Windows-Profil | `%APPDATA%\netcore\control-room\operator.toml` | UI und Startaufrufe |
| Früherer Windows-Tokenpfad | `%APPDATA%\netcore\control-room\operator.token` | Tokenphase, später für menschlichen Login überholt |
| Kartenkachelcache | `%LOCALAPPDATA%\netcore\control-room\tiles` | Kartenansicht/Implementierung |
| TBS-Dashboard | TCP `8080` | Startlog |
| Brew-Anbindung | `ws://10.0.1.22:8081` | TBS-Startlog |
| Directory-Port im späteren Pull-Code | TCP `8095` | Quellcode-Default, nicht bestätigte reale Directory-Adresse |
| Pull-Default | `http://127.0.0.1:8095` | Gilt relativ zum LXC-Prozess; nicht automatisch der separate Directory-Server |

Wichtige Umgebungsvariablennamen waren `NETCORE_CONTROL_ROOM_NODE_TOKEN`, in der überholten Tokenphase `NETCORE_CONTROL_ROOM_OPERATOR_TOKEN` und `NETCORE_CONTROL_ROOM_ADMIN_TOKEN`, später `NETCORE_CONTROL_ROOM_API`, `NETCORE_CONTROL_ROOM_USER`, `NETCORE_CONTROL_ROOM_NODE_ID` und `NETCORE_CONTROL_ROOM_OPERATOR_ID`. Für die Directory-Anbindung sind am 03.10.2026 insbesondere `NETCORE_DIRECTORY_API`, `NETCORE_DIRECTORY_URL` und `NETCORE_DIRECTORY_BASE_URL` relevant. **Es werden hier nur Bezeichner dokumentiert, keine Werte von Zugangsdaten.**

### 5.1 Funkparameter als Kontext der beobachteten TBS

Die Übersicht zeigte MCC `901`, MNC `1510`, Location Area `1`, Colour Code `1`, System Code `1`, Main Carrier `720`, Secondary Carrier `721` und aktivierten Dual-Carrier-Betrieb. Der erhaltene TBS-Startlog nennt die Trägerpaare DL/UL `418.000/408.000 MHz` und `418.025/408.025 MHz`, mit expliziten SDR-Mittenfrequenzen TX/RX `418.0125/408.0125 MHz`.

Der gleiche Log nennt SXceiver-Hardwareversion `1.2`, eine Abtastrate von `600000` und den logischen Timeslot-Mapper v2.8 mit `C2 TS1 control/guard`. Diese Angaben beschreiben den damaligen Trägerkontext, nicht eine in dieser Entwicklungsphase abgenommene Dual-Carrier-Funktion. Für die eigentliche Dual-Carrier-Historie existieren gesonderte Archivdokumente.

Der TBS-Log nennt `service_name=tetra`. Aus dem Journal-Prozessnamen `bluestation-bs` darf deshalb nicht ungeprüft auf den systemd-Unitnamen geschlossen werden. Ebenso ist ein Fehler der Control-Room-Anmeldung nicht automatisch ein RF- oder Trägerproblem.

## 6. API- und Zustandsverträge

### 6.1 Historisch benutzte Control-Room-Endpunkte

| Endpunkt | Verwendung | Historischer Beleg |
|---|---|---|
| `GET /health` | Erreichbarkeit des Kerns | Mehrfach HTTP 200 / `ok: true` |
| `GET /api/state` | Umfangreicher Gesamtzustand | Frühe eingefügte Terminalausgabe |
| `GET /api/overview` | Node-, Teilnehmer-, Ruf- und Notrufübersicht | Wiederholt erfolgreich |
| `GET /api/rf` | RF-Snapshot | Frühe API-Testausgabe |
| `GET /api/health/full` | Erweiterte Gesundheitsdaten | Frühe API-Testausgabe |
| `GET /api/subscribers` | Teilnehmer und Onlinezustand | Leere und später befüllte Ansichten |
| `GET /api/groups` | Gruppeninformationen | API-/UI-Nutzung |
| `GET /api/calls` | Rufzustände | API-/UI-Nutzung; leer ist kein Nachweis eines aktiven Rufablaufs |
| `GET /api/sds` | SDS-Historie | Nachricht im Screenshot sichtbar |
| `GET /api/locations` | Positionsdaten | Karte/Standortansicht |
| `GET /api/emergencies` | Notrufzustände | API-/UI-Modul; vollständiger Realtest nicht belegt |
| `GET /api/commands` | Command-Audit | Persistenztest nach Neustart |
| `GET /api/events` | Ereignisse; teilweise Filter/quiet | Frühe API-Ausgaben |
| Node-Command-Routen | Kick, DGNA, Emergency Clear | Kick-Nachrichtenweg konkret getestet; weitere vollständige Funktionstests nicht dokumentiert |
| `/api/admin/tokens` | Frühere Tokenverwaltung | Historische Liste und RBAC-Warnung; später ersetzt |
| `POST /api/login`, `GET /api/me`, `/api/admin/users` | Benutzeranmeldung/Rollen/Benutzerverwaltung | Spätere Implementierung; Login-UI sichtbar |

Die tatsächlich für SDS verwendeten Datenfelder sind besonders wichtig:

```text
source_issi
dest_issi
protocol_id
text
timestamp
direction
```

Frühere Tableau-Fallbacks suchten nur nach `source`, `src`, `from`, `proto` oder `protocol`. Dass die SDS-Tabelle eine Nachricht anzeigen konnte, bewies nicht, dass der separat geschriebene Tableau-Parser dieselben Felder las.

### 6.2 Später hinzugefügte Directory-Routen

`GET /api/directory` existierte zunächst als Ausgabe eines Konfigurationswertes des Control Rooms. Erst später wurden ein dynamischer gemeinsamer Wert und folgende Routen geliefert:

```text
GET  /api/directory/resolved
GET  /api/directory/upstream
POST /api/directory/import
POST /api/directory/merge
POST /api/directory/refresh
```

`import` und `merge` sind nicht gleichbedeutend mit automatischem Bezug der Stammdaten. Im v5.13-Ansatz wurde ein Shared-Memory-Wert befüllt. Ohne Aufrufer und ohne zentrale Quelle blieb er leer. Eine dauerhafte Speicherung dieser Imports war in den betrachteten Import-Routinen nicht implementiert.

Die späteren Pull-Routinen greifen auf diese eigenständigen Directory-Server-Endpunkte zu:

```text
GET /api/devices
GET /api/basestations
GET /api/groups
GET /api/device-groups
GET /api/status
```

Die fachliche Quelle ist damit der Directory Server; das Control-Room-API bildet eine übersetzte Sicht darauf. Ein `ok: true` des Upstream-Debug-Endpunkts genügt nicht zur Abnahme: Es kann bereits durch vorhandene Status- oder Gruppendaten entstehen, obwohl kein passender Gerätename für die gesuchte ISSI vorhanden ist.

## 7. Chronologie und Entwicklungsstände

Die folgende Chronologie unterscheidet Entwicklungsbezeichnungen von geprüften Releases. Nicht für jedes Paket liegt eine vollständige Build- oder Ausführungsaufzeichnung vor.

### 7.1 Kern, API, Persistenz und Authentifizierung

Zu Beginn wurden Node-Anbindung, Overview-/Detail-API, Operator-Client, Konfiguration und Persistenz in aufeinanderfolgenden vollständigen Dateipaketen bereitgestellt. Frühe Tests liefen noch im Shellkontext der TBS. Später wurde der Control-Room-LXC als separates Zielsystem verwendet.

Ein LXC-Build zog zunächst `soapysdr-sys` in den Abhängigkeitsgraphen. Die Bibliothek beziehungsweise `SoapySDR.pc` fehlte, und `pkg-config` schlug fehl. Dazu wurden Dependency-/Cargo-Fixes geliefert. Die spätere Trennung der Backendabhängigkeiten ist in den Manifestauszügen sichtbar; eine genaue Zuordnung sämtlicher frühen Paketversionen zu einzelnen Commits ist nicht möglich.

Der folgende Rust-Ownership-Fehler war im Betriebslog eindeutig:

```text
E0382: use of moved value: response_value
```

`response_value` wurde in eine bestehende Antwortliste verschoben und danach nochmals zur Anlage einer neuen Liste benötigt. Die Clone-Korrektur und die Entfernung von `OptionalExtension` aus dem Import wurden damals ausgeführt. Danach meldete Cargo einen erfolgreichen Release-Build. Der spätere Wunsch nach vollständigen Dateien ersetzt diese Art verteilter manueller Hotfixes als bevorzugten Lieferweg.

Nach Anlage der Konfiguration, des Dienstkontos und der Unit startete der Dienst mit SQLite. Ein Kick-Auftrag wurde angenommen, beantwortet, in SQLite abgelegt und nach einem Dienstneustart wieder angezeigt. Anschließend folgten Tokenauthentifizierung, Tokenregistry und dann der ausdrückliche Wechsel auf Benutzername/Passwort.

### 7.2 Native Oberfläche bis v5.10.1

| Entwicklungsstand | Thema | Einordnung |
|---|---|---|
| Native UI v1 | Erste native Windows-Oberfläche mit Modulnavigation und Tabellen | Im Betrieb durch Screenshot bestätigt; noch sehr einfache, unpassende Größenverhältnisse |
| v2 | Multiwindow-/Kartenansatz | Unveränderte Ansicht beziehungsweise nur interne Fenster beanstandet |
| v3 | Echte OS-Fenster über mehrere Monitore | Als Paket geliefert; Fensteransatz bestätigt, echte Karte als nächster Ausbau festgelegt |
| v4 / v4.1 | Reale Kartenkacheln; Fix eines E0502-Borrow-Konflikts | Reale Karte später sichtbar |
| v4.2–v4.4 | Pan/Zoom, flüssigere Karte, Geräteinfos, geringere Mausradempfindlichkeit | Gestaltungsfeedback dokumentiert Defizite und Verbesserungswünsche; keine vollständige Performance-Abnahme |
| v4.6 / v4.7 | Teilnehmer-/Positionsbereinigung, Directory-Nutzung | Namen-/Gruppenintegration später weiterhin fehlerhaft |
| v5 / v5.1 / v5.2 | Benutzer/Passwort; CLI- und UI-Ownership-Buildfixes | Loginmaske und spätere Adminansicht sichtbar |
| v5.3 | Node-Token/Basic-Kompatibilität nach Offline-/Broken-pipe-Schleife | TBS wieder online bestätigt |
| v5.4 / v5.5 | Responsive Login/UI und Rollen-Ausblendung | Gefordert und geliefert; lückenhafte Rollenabnahme |
| v5.6–v5.8 | Leitstellenlayout, responsive Nachbesserungen, kompakter Header | Wiederholte Screenshots mit abgeschnittenem Header und driftenden Controls |
| v5.9 | Clean Workspace, funktionierendes Menü, feste Aktualisierung | Neue Feldnamenfehler verhindern zunächst Build |
| v5.9.1 / v5.9.2 | Korrektur nicht vorhandener Command-Eingabefelder | Folgefehler dokumentiert; spätere UI wieder sichtbar |
| v5.9.3 | Feste Navigationszeile statt driftender Pfeilbuttons | Implementierung im Generator sichtbar; allein noch kein Beleg für alle Größen/DPI |
| v5.10 | Kartencluster, Spiderfy, Auswahl und Cluster-Geräteliste | Clusteransicht in späteren Screenshots sichtbar |
| v5.10.1 | Standorttabelle unter der Karte entfernt | Explizite Anforderung; späterer Sourcebefund erhält die Trennung |

Die visuellen Referenzen umfassten neben SELECTRIC unter anderem Einsatzleitplätze, Telefonie-/Kommunikationsoberflächen, Objektstatus, Personalverwaltung und Sirenenkarten. Daraus darf nicht abgeleitet werden, dass Telefonie, Video, Einsatzdisposition, Personalmanagement oder Sirenensteuerung in dieser Entwicklungsphase vollständig implementiert wurden. Die Bilder dienten zunächst der gewünschten Informationsdichte und Bediengestaltung.

### 7.3 Status-Tableau v5.11 bis v5.11.9

| Stand | Änderung / Diagnose | Ergebnis und Grenze |
|---|---|---|
| v5.11 | Neues Status-Tableau; Statusgruppen und Gerätezeilen | 16 gemeldete Compilerfehler: fehlende Typen, falsche Arraylänge, nicht vorhandene Methoden/Funktionen |
| v5.11.1 | Typen und Methodenverdrahtung; `Tab::ALL` von 10 auf 11 | Doppeltes `#[derive(Debug, Clone)]` erzeugt E0119 |
| v5.11.2 | Doppeltes Derive entfernt | UI-Screenshot vorhanden; Status-SDS erscheinen im Tableau noch nicht korrekt |
| v5.11.3 | SDS-Fallback und einzeilige Legende | Unterschiedlich lange innere Arrays erzeugen E0308 |
| v5.11.4 | Slice-Liste `&[(u64, &[&str])]` | Buildfix geliefert, fachlicher Status weiterhin nicht übernommen |
| v5.11.5 | IDs/Protokolle als JSON-Zahl oder numerischer String | Hypothese über Datentypen; weiterhin kein erfolgreicher Statuswechsel laut Betriebsbeobachtung |
| v5.11.6 | Tatsächliche Felder `source_issi` und `protocol_id`; Diagnosezähler | Statuskandidaten und Text sichtbar; Nummer/Farbe und Namen weiterhin fehlerhaft |
| v5.11.7 | Eigene Karten für ungruppierte Geräte; erweitertes Namensfallback | Screenshots zeigen teils noch v5.11.6; keine sichere Namensabnahme |
| v5.11.8 | Warning-Cleanup und weitere Directory-/Statusheuristik | `id_key_variants` versehentlich gelöscht, entfernte Struct-Felder noch initialisiert |
| v5.11.9 | Fehlende ID-Hilfe und Initializer korrigiert | Screenshot bestätigt eigene Gerätekarten; erneuter Layoutdrift, weiterhin ISSIs und falsche Statuszahl |

Die Zahlen wie „16/50 SDS-Zeilen als Statuskandidaten erkannt“ belegen nur die Treffer einer Erkennungsheuristik in einem begrenzten Nachrichtenfenster. Sie bestätigen weder den letzten gültigen Status jedes Geräts noch korrekte Code-/Farbzuordnung oder eine erfolgreiche Directory-Abfrage.

### 7.4 Namens- und Directory-Arbeiten v5.12 bis v5.14.2

| Stand | Ansatz | Einordnung |
|---|---|---|
| v5.12.0 | Freie Maus-Anordnung; Canvas statt driftendem Wrap-Layout; tolerant normalisiertes Directory-JSON | Screenshot zeigt ausgerichtetes Raster und Resetbutton. Tatsächliches Drag-Verhalten, Layoutpersistenz und korrekte Namen nicht umfassend bestätigt. |
| v5.12.1 | Zentrale Daten bevorzugen; lokale Werte nur ergänzend; Rohdaten durchsuchen | Erneute Nutzerbeanstandung fehlender Namen |
| v5.12.2 | Rekursiver Deep-Index für ISSI/Namensfelder | Wieder keine Namen; zahlreiche unbenutzte Variablen/Hilfsfunktionen im erfolgreichen Build |
| Diagnose danach | Control-Room-API liefert nur `hide_infrastructure` | Konkreter Nachweis, dass diese API-Sicht keine Stammdaten enthält |
| Zwischenvorschlag | Namen manuell in LXC-TOML eintragen | Ausdrücklich als nicht zielführend zurückgewiesen |
| v5.13.0 | Dynamisches Directory, Import-/Merge-API, resolved-Sicht, CLI-Import | Kein automatischer Bezug des vorhandenen Directory Servers; Kernproblem allein dadurch nicht gelöst |
| v5.14.0 | Directory-API-Pull über fünf native Endpunkte | Sourceansatz vorhanden; Buildfehler wegen `.len()` auf `serde_json::Value` |
| v5.14.1 | Objektlänge über `as_object()` | Im Artefakt korrigiert; keine eindeutige abschließende Nutzerabnahme |
| v5.14.2 | Grep-Prüfung und Versionsmarker | Kein Nachweis eines vollständigen Builds oder funktionierenden Live-Directory. Gefährliche Neuinstallationsempfehlung wird hier ausdrücklich zurückgezogen. |

Der wesentliche Erkenntnisgewinn entstand nicht durch immer mehr mögliche Namensfelder, sondern durch die Abfrage der tatsächlich konsumierten Datenquelle. Die vorherige Annahme, die Basisstation habe ihre Namen zwingend nur lokal, war nicht belegt. Der am 03.10.2026 geprüfte TBS-Code besitzt ausdrücklich einen Client für den vorhandenen Directory Server. [R10] [R11]

## 8. Konkrete Betriebs- und Testnachweise

### 8.1 Frühe API-Abfragen auf der TBS

`Eingefügter Text(45).txt` enthält am 30.06.2026 eine erfolgreiche Health-Antwort und einen umfangreichen State-Dump. `Eingefügter Text(46).txt` ergänzt Overview, RF, erweiterte Health-Informationen, Commands und Ereignisse. `Eingefügter Text(47).txt` enthält Detailabfragen zu Teilnehmern, Gruppen, Rufen und weiteren Daten.

Diese Ausgaben bestätigen frühe HTTP-Funktionalität und die Übermittlung von Zustandsdaten. Sie bestätigen nicht den späteren separaten LXC-Directory-Pull. Einzelne Tippfehler in Shellzeilen ändern nichts daran, dass die nachfolgenden JSON-Ausgaben vorhanden sind; sie sollten aber nicht ungeprüft als ausführbare Anleitung kopiert werden.

### 8.2 LXC-Start, Telemetrie und SQLite

Der erfolgreiche Release-Build nach dem Ownership-Fix ist durch die Ausgabe vom 01.07.2026 belegt. Danach wurden Dienstkonto, Konfigurationsverzeichnis, Datenverzeichnis und systemd-Unit eingerichtet.

Im Startlog um 11:13:59 UTC standen SQLite-Persistenz, null geladene Historieneinträge, `bind=0.0.0.0:9010`, `node_path=/node`, `ui_path=/ui` und eine WebSocket-Verbindung von `10.0.1.20`. Die anschließende Overview-Abfrage zeigte einen verbundenen Node, unter anderem 1.357 Telemetrieereignisse und zwölf Heartbeats, aber noch keine Teilnehmer. Spätere Ausgaben zeigten weitere steigende Zähler.

`sqlite3` war zunächst als CLI nicht installiert. Eine später erfolgreiche `.tables`-Abfrage belegt die Tabellen `commands`, `events`, `node_sessions`, `sds_log`, `emergencies`, `locations` und `schema_migrations`. Das Fehlen des CLI-Werkzeugs war nicht gleichbedeutend mit einem Ausfall der in Rust eingebundenen SQLite-Persistenz.

### 8.3 Command-Audit über einen Neustart

Der konkrete Test adressierte ISSI `2010002` am Node `SRV-M_TBS-01`. Die Command-ID lautete:

```text
a3d87a44-4b04-4bad-83a3-bac695fb3c09
```

Die HTTP-/CLI-Annahme meldete `queued`. In SQLite stand kurz darauf `completed`. Nach `systemctl restart netcore-control-room` zeigte die CLI den Auftrag weiterhin, einschließlich einer Antwort:

```json
{
  "KickMsResponse": {
    "issi": 2010002,
    "success": false
  }
}
```

**Getestet und im Betrieb bestätigt:** Command-Transport, Verarbeitung einer Antwort und Persistenz/Wiederherstellung der Auditinformation.

**Nicht bestätigt:** Erfolgreiches Kicken eines registrierten Funkgeräts. `completed` bezeichnet hier einen abgeschlossenen Bearbeitungsweg und hebt das fachliche Ergebnis `success: false` nicht auf. Diese Unterscheidung muss auch eine spätere UI sichtbar machen.

### 8.4 Authentifizierung und systemd-Umgebung

Nach einer zunächst ungeschützten beziehungsweise noch nicht wirksam aktualisierten Instanz folgten Startfehler, weil bei eingeschalteter Authentifizierung kein Node-Token aufgelöst wurde. Zwischenzeitlich waren konkrete Werte in Felder eingetragen worden, die Namen von Umgebungsvariablen erwarten.

Die erhaltene Textdatei 50 belegt zusätzlich einen eigenständigen Fehler: `override.conf` enthielt Zuweisungen außerhalb eines Abschnitts. systemd meldete `Assignment outside of section. Ignoring.` Der gezeigte Drop-in enthielt `EnvironmentFile` und `ExecStart`, aber keinen `[Service]`-Header. Somit ist nicht allein ein falscher Tokenwert als Ursache zu dokumentieren.

Spätere Tests zeigten `/health` mit HTTP 200, `/api/overview` ohne gültige Autorisierung mit HTTP 401 und eine erfolgreiche Übersicht mit dem passenden Operator-Token. Eine Admin-Tokenliste war ebenfalls sichtbar. Diese Tests gehören zur **überholten menschlichen Tokenphase**.

Ein weiterer Dienststatus bestätigte um 12:37:53 UTC den laufenden Kern, SQLite, aktivierte Tokenauthentifizierung und eine Node-Verbindung. Im späteren Benutzer-/Passwort-Stand sind Login und angemeldete UI sichtbar; sämtliche früheren Token-CLI-Aufrufe dürfen nicht unverändert als geprüfte Loginanleitung gelten.

### 8.5 „Basisstation offline“ trotz Verbindungsversuchen

Die TBS protokollierte wiederholt `Broken pipe (os error 32)` und Reconnect. Der LXC protokollierte dazu `websocket rejected: unauthorized` auf `/node`, mit sehr kurzen Wiederholungsintervallen. Ein erfolgreicher TCP-/WebSocket-Upgrade und die Logzeile `transport connected` bewiesen in diesem Ablauf noch keine akzeptierte Node-Identität.

Die spätere Nutzeräußerung „jetzt ist die wieder online“ ist ein konkreter Erfolg nach der Authentifizierungsnachbesserung. Es gibt jedoch keinen eindeutig dazu festgehaltenen vollständigen Git-Commit der produktiven Binärdateien. Die Ursache der hier gezeigten Offline-Anzeige war nicht als Funkstörung belegt.

### 8.6 Live-Karte, Cluster und Statusansichten

Die Screenshots belegen eine reale Karte mit Kacheln, mehreren Positionspunkten, Clusteranzeige, aufgefächerten Punkten und einer Geräteübersicht im Kartenoverlay. Sie zeigen auch den Übergang von der Sammelliste zu separaten Statuskarten und schließlich zu einem ausgerichteten Raster.

Gleichzeitig dokumentieren sie die offenen Fehler: rein numerische Namen, doppelte ISSI-Anzeige in früheren Ständen, falsche Statuszahl trotz plausiblen Textes sowie Layoutdrift. Ein Screenshot belegt die angezeigte UI-Version, nicht die dazu passende Backendversion oder die Version des eigenständigen Directory Servers.

### 8.7 Der entscheidende Directory-Befund

Belegte Antwort der **Control-Room-API**:

```json
{
  "hide_infrastructure": true
}
```

Diese Antwort enthält keine Teilnehmer, Namen, Gruppen oder Statusdefinitionen. Ein UI-Parser kann aus ihr keine echten Namen gewinnen. Sie beweist aber **nicht**, dass der separate NetCore Directory Server keine Namen besitzt. Die Betriebsbeobachtung, dass die Basisstation Namen korrekt auflöst, bleibt damit vereinbar.

Die richtige Diagnosefrage lautet: Welche tatsächliche Directory-URL verwendet die funktionierende TBS, welche URL verwendet der LXC, welche Antworten liefern beide, und wie werden diese Antworten zwischen den Diensten übersetzt? Die reale Directory-IP wurde in den zugänglichen Nutzerausgaben nicht eindeutig ermittelt.

## 9. Fehlerkatalog: Ursachen, Lösungen und Grenzen

### 9.1 Compiler- und Abhängigkeitsfehler

| Fehler | Konkreter Sachverhalt | Lösung / geprüfte Einordnung |
|---|---|---|
| `soapysdr-sys` Custom-Build fehlgeschlagen | LXC-Build benötigte unerwartet SoapySDR; `pkg-config` fand `SoapySDR.pc` nicht | Abhängigkeiten/Features des Kerns isolieren. Der spätere Artefaktstand deaktiviert Defaultfeatures von `tetra-entities`; kein Anlass, die gesamte RF-Umgebung blind auf den LXC zu kopieren. |
| E0382 `response_value` | JSON-Wert vor zweiter Verwendung verschoben | Clone vor dem Einfügen; danach erfolgreicher Nutzerbuild belegt |
| E0277 `PersistenceHandle` / `PersistenceInner` ohne `Debug` | Ableitung im Auth-State zog weitere Trait-Anforderungen nach sich | Mehrstufige Korrekturen geliefert; spätere laufende Versionen belegt. Trait-Abhängigkeiten vollständig prüfen statt jeweils nur die nächste Fehlermeldung bearbeiten. |
| E0502 Karte | Unveränderliche Referenz auf `self.locations` blieb während mutabler Kartenrender-Methode aktiv | Datenbesitz/Lebensdauer der Renderdaten trennen; Buildfix geliefert, reale Karte später sichtbar |
| E0382 CLI `cli.command` | `unwrap_or` konsumierte Feld, danach Zugriff auf ganzes `cli` | Ownership der Kommandoauswahl ändern; keine pauschale Annahme, die Compilerhilfe passe unverändert auf die vorhandene Signatur |
| E0382 UI `settings` | Struct-Initializer verschob Settings vor Berechnung des Login-Namens | Abhängige Werte vorher bestimmen oder kontrolliert klonen |
| E0609 Command-Felder | Nicht existierende Namen `command_issi`, `command_gssi`, `emergency_issi`, `command_detach`, später `emergency_clear_issi` | Auf tatsächliche State-Felder abstimmen; wiederholte Folgefehler waren Generatorfehler |
| E0425/E0422 Tableau-Typen | `StatusTableauCard` und `StatusTableauDevice` fehlten im Scope | Typen explizit definieren und Einfügestelle prüfen |
| E0308 `Tab::ALL` | Array deklarierte zehn, enthielt elf Module | Arraylänge korrigieren beziehungsweise pflegesicherer repräsentieren |
| E0599 `can_read` | Methode existierte nicht | Vorhandenes Rollenmodell verwenden |
| E0425 Subscriber-Helfer | Falsche freie Funktion statt vorhandener Methode, teils erfundene Helfer | Gegen tatsächliche Implementierung verdrahten |
| E0119 `Debug` / `Clone` | Doppeltes Derive vor `StatusTableauCard` | Doppeltes Attribut entfernen; nicht nur Deklarationstext ohne Umgebung ersetzen |
| E0308 Statusmuster | Innere Rust-Arrays unterschiedlich lang | Slice-Liste mit referenzierten Slices; in v5.11.4 geliefert |
| E0425 `id_key_variants` | Beim Entfernen einer unbenutzten Nachbarfunktion versehentlich mit gelöscht | Hilfsfunktion wiederherstellen; Funktionsgrenzen nicht per zu breitem Regex entfernen |
| E0560 `ResolvedSettings` | Felder entfernt, aber im Initializer noch vorhanden | Definition und Initialisierung gemeinsam ändern |
| E0599 `serde_json::Value.len()` | Resolved-JSON ist kein direkt längenbestimmbares Map-Objekt | `resolved.as_object().map(|object| object.len()).unwrap_or(0)`; im letzten ZIP und am 03.10.2026 geprüften Repository vorhanden |

### 9.2 Laufzeit-, Integrations- und Datenfehler

**Falsche Quelle:** Wiederholtes Ausbauen der UI-Namensheuristik konnte die leere Control-Room-Directory-Sicht nicht beheben. Die fehlende Anbindung an den tatsächlich vorhandenen Server war eine andere Fehlerklasse als ein einzelner JSON-Feldname.

**Uneinheitliche Parser:** SDS-Tabelle und Tableau griffen ursprünglich auf unterschiedliche Feldnamen zu. Die Korrektur auf `source_issi` und `protocol_id` führte zu sichtbaren Statuskandidaten. Das ist ein wirklicher Fortschritt, aber keine vollständige Fachlogik.

**Statustext versus Statusnummer:** Die gelieferten Implementierungen priorisieren Codes und Texte unterschiedlich. Ein jüngerer SDS-Text kann mit einem älteren Subscriber-/Directory-Code kombiniert werden. Außerdem wurden verschiedene Statusbeschreibungen durch hartcodierte Wörter auf dieselbe Nummer reduziert. Das erklärt mögliche Inkonsistenzen im Code; die jeweilige Live-Ursache muss mit dem konkreten Payload bestätigt werden.

**ESM versus Betriebsstatus:** `ESM aktiv`, `ESM aus`, Registrierung und Onlinezustand sind nicht der ausdrücklich gesendete betriebliche Status. Eine Darstellung als Ersatztext muss als andere Information erkennbar bleiben. Sie darf nicht unbemerkt eine betriebliche Statuszahl bestimmen.

**Begrenztes SDS-Fenster:** Ein aus nur den letzten Nachrichten abgeleiteter Status kann verschwinden, sobald diese Nachrichten aus dem Fenster fallen. Im geprüften UI-Code werden weiterhin nur 50 SDS-Zeilen geladen. Das ist eine konkrete technische Ursache für potenziell unvollständige Zustandsrekonstruktion; für einen stabilen aktuellen Status braucht es einen eigenen, pro Gerät fortgeschriebenen Zustand. [R07]

**Layout-/Interaktionsfehler:** Normaler Wrap-/Horizontal-Layoutfluss, wechselnde verfügbare Breiten, geerbte Ausrichtung und manuell gezeichnete Flächen wurden mehrfach verändert. Die damalige alleinige Erklärung „horizontal verursacht den Drift“ war ohne isolierten Layouttest nicht ausreichend belegt. Freies Platzieren im Canvas behebt nicht automatisch Clipping, Überlappung, Maus-Hit-Tests, DPI oder Layoutpersistenz.

**Falsche Installationsannahmen:** Manche Screenshots zeigten noch eine ältere Versionszeile. Das rechtfertigt eine Kontrolle von EXE-Pfad und laufendem Prozess, Die Screenshots beweisen keine generell falsche Installation späterer Pakete. Weitere Fehlermeldungen sind einer konkreten Fehlerklasse zuzuordnen.

### 9.3 Warnings richtig einordnen

Die Buildausgaben zeigen unter anderem ungenutzte Variablen `config_path`, `username_source`, ungenutzte Struct-Felder wie `owner`, alte Toolbar-/Marker-Helfer und mehrere nicht mehr verwendete Merge-/Raw-Directory-Funktionen. Die Builds endeten teilweise ausdrücklich erfolgreich mit 6 beziehungsweise 19 Warnings.

Eine Warning stoppt diesen Build nicht von sich aus. Daraus folgt aber nicht, dass unbenutzte Integrationsfunktionen bedeutungslos für die Diagnose sind: Wenn die benötigte Datenauflösung nie aufgerufen wird, kann genau das ein Verdrahtungsproblem anzeigen. Die korrekte Aussage ist deshalb: **Die Warning ist nicht automatisch die Laufzeitursache; ihre betroffene Funktion und ihr fehlender Aufruf sind dennoch zu prüfen.**

Warnings wurden in dieser Entwicklungsphase wiederholt durch unkontrolliertes Entfernen von Code bekämpft und führten dabei zu neuen Fehlern. Eine pauschale Unterdrückung mit `allow(dead_code)` wäre ebenfalls keine funktionale Lösung.

## 10. Verworfene, ersetzte und zurückgezogene Ansätze

### 10.1 Menschliche Tokens statt Benutzeranmeldung

Durch die spätere Projektentscheidung überholt. Frühere Admin-/Operator-Tokenverwaltung bleibt historische Entwicklungsarbeit; neue Anleitungen müssen zwischen menschlichem Login und TBS-Token unterscheiden.

### 10.2 Lokale Stammdaten in `operator.toml` oder LXC-TOML

Als dauerhafter Arbeitsweg ausdrücklich verworfen. Verbindungsparameter dürfen weiterhin in einer Konfiguration stehen; das ist etwas anderes als manuell gepflegte Gerätenamen und Statuspläne. Ein technisch begründeter Offlinecache müsste Herkunft und Alter zeigen und darf keine zweite Stammdatenpflege erzwingen.

### 10.3 Manuelle Beispielnamen und JSON-Import

Die vorgeschlagenen Gerätenamen wurden teilweise aus Referenzbildern beziehungsweise Beispielen übernommen. Sie sind keine bereitgestellten tatsächlichen Namenszuordnungen. Solche Einträge dürfen nicht produktiv importiert werden, um einen optischen Erfolg vorzutäuschen.

Der v5.13-Importkanal kann technisch eine Administrationsfunktion sein, erfüllt aber ohne automatischen Client nicht den gewünschten Datenfluss. Ein Neustartverlust eines nur im Speicher gehaltenen Imports muss ebenfalls berücksichtigt werden.

### 10.4 Zwingender TBS-Directory-Push als vermeintliche Dauerlösung

Ein Push der Basisstation wurde als möglicher Weg vorgeschlagen. Er war aber keine nachgewiesene notwendige Architekturentscheidung der Projektplanung. Die spätere Anforderung richtet sich auf den vorhandenen Directory Server. Der aktuelle TBS-Client zeigt, dass ein direkter zentraler API-Abruf bereits vorgesehen ist. Ein zusätzlicher TBS-Push wäre nur nach begründetem Architekturentscheid sinnvoll, nicht als Ersatz für das Lesen der vorhandenen Schnittstelle. [R10] [R11]

### 10.5 Rekursiver „Universal“-Namensindex

Als Übergang zum toleranten Lesen verschiedener Datenformen implementiert, aber nicht als endgültiger Datenvertrag geeignet. Eine beliebige numerische Objekt-ID ist nicht zwingend eine ISSI. Gruppen, Statuscodes, Geräte und Basisstationen benötigen getrennte Namensräume. Auch ein Index mit mehr als null Einträgen beweist nicht, dass die konkrete Teilnehmerkennung korrekt aufgelöst wurde.

### 10.6 Hartcodierte Statusheuristik

Die Zuordnung von Textfragmenten wie `frei`, `bereit`, `einsatz` oder `ziel` zu UI-Zahlen ist kein Ersatz für den zentralen Statusplan. Besonders die Zusammenfassung von „Frei auf Funk“ und „Frei auf Wache“ auf dieselbe Nummer war gegen die erwartete Unterscheidung nicht abgesichert. Dieses Archiv legt keine neuen universellen Statusnummern fest. Maßgeblich sind der tatsächliche übertragene Code und die Definition des Directory Servers.

### 10.7 Gesamtes Repository löschen und Komponenten-ZIP entpacken

**Zurückgezogen – nicht ausführen.** Die zuletzt vorgeschlagene Kombination aus vollständigem Löschen von `/opt/netcore/flowstation` und anschließendem Entpacken des v5.14.2-Komponenten-ZIP ist falsch.

Die Archivprüfung hat am tatsächlichen ZIP festgestellt:

- 59 Dateieinträge, aber **kein Root-`Cargo.toml`**.
- Keine vollständigen Quellen unter `crates/tetra-entities/`.
- Der Backend- und Operator-Build benötigt genau diesen übergeordneten Workspace und weitere Crates.
- Das Entfernen des bestehenden Repositorys kann zusätzlich `.git`, lokale Änderungen und installationsspezifische Dateien vernichten.

Das ZIP war ein Satz vollständiger **betroffener Dateien**, kein vollständiger Ersatz des gesamten Repositorys. Eine erfolgreiche Ausführung der gefährlichen Löschanweisung ist in den Arbeitsnotizen nicht belegt. Künftige Schritte müssen den vorhandenen Arbeitsbaum zunächst inventarisieren und sichern; alte Quellen dürfen nicht ungeprüft über den geprüften Stand kopiert werden.

## 11. Sichere Befehls- und Ablaufreferenz

Dieser Abschnitt bewahrt wichtige Abläufe, trennt aber historische Ausführung von geprüften Vorschlägen. Es werden keine Zugangsdaten eingebettet. Die Prüfungen sollten in einem kontrollierten Wartungsfenster erfolgen, sobald sie den Betrieb verändern.

### 11.1 Historisch erfolgreich benutzte Befehlsformen

Auf dem LXC wurde der Kern zusammen mit der Operator-CLI gebaut:

```bash
cd /opt/netcore/flowstation
cargo build --release \
  -p netcore-control-room \
  -p netcore-control-room-operator
```

Es gibt für frühere Stände einen erfolgreichen Build und einen laufenden Dienst. **Daraus folgt nicht**, dass jeder später angebotene ZIP-Stand mit diesem Befehl erfolgreich gebaut wurde.

Die SQLite-CLI-Abfragen wurden später tatsächlich ausgeführt:

```bash
sqlite3 /var/lib/netcore-control-room/control-room.sqlite3 '.tables'

sqlite3 /var/lib/netcore-control-room/control-room.sqlite3 \
  'select command_id,status,target_node_id,operator_id,updated_at from commands order by updated_at desc limit 5;'
```

Der Dienststatus und die Logs wurden mit `systemctl status netcore-control-room --no-pager -l` und `journalctl -u netcore-control-room` geprüft. Ein Service-Start sowie ein gezielter Neustart zur Persistenzkontrolle sind historisch belegt.

### 11.2 Am 03.10.2026 geprüfte rein lesende Vorprüfung – vorgeschlagen, nicht auf Jans System ausgeführt

```bash
cd /opt/netcore/flowstation
pwd
git status --short --branch
git rev-parse HEAD
test -f Cargo.toml && echo 'Workspace-Manifest vorhanden'
test -d crates/tetra-entities && echo 'TETRA-Abhaengigkeiten vorhanden'
systemctl status netcore-control-room --no-pager -l
systemctl show netcore-control-room -p FragmentPath -p ExecStart -p MainPID
journalctl -u netcore-control-room -n 80 --no-pager
```

`systemctl cat` ist für die lokale Prüfung der Unit und ihrer Drop-ins sinnvoll. Seine Ausgabe kann aber Umgebungswerte beziehungsweise Zugangsdaten enthalten und darf nicht ungeprüft in Git oder Diagnoseprotokolle kopiert werden. Auch vollständige Konfigurationsdateien werden nicht als Diagnosenachweis veröffentlicht.

Zur Directory-Diagnose bei aktivierter HTTP-Basic-Anmeldung kann `curl` das Passwort interaktiv abfragen:

```bash
curl --silent --show-error --fail --user admin \
  http://127.0.0.1:9010/health

curl --silent --show-error --fail --user admin \
  http://127.0.0.1:9010/api/directory/upstream

curl --silent --show-error --fail --user admin \
  http://127.0.0.1:9010/api/directory/resolved
```

Diese Aufrufe sind eine vorgeschlagene Diagnose, kein neuer Testnachweis. Bei geändertem Auth-Modell sind sie dem tatsächlich aktiven Modus anzupassen. Die reale zentrale Directory-URL ist zunächst aus der funktionierenden TBS-Konfiguration zu ermitteln, ohne dabei Secrets zu übernehmen. Dann ist **dieselbe** API vom LXC aus zu prüfen. Loopback-Adressen auf zwei verschiedenen Maschinen bezeichnen nicht denselben Dienst.

Die Auswertung muss für eine konkrete bekannte ISSI kontrollieren: richtiger Name, richtige Gruppenmitgliedschaft, vorhandene Statusdefinition und tatsächlich erfolgreiche Upstream-Antwort. Ein globaler Zähler allein genügt nicht.

### 11.3 LXC-Deployment – gewünschter verbesserter Ablauf, nicht bereits durchgeführt

Vor einem neuen Deployment: Repo-Änderungen, Konfiguration, Datenbank und bisheriges Binary sichern; Versions-/Commitbezug dokumentieren. Den vollständigen passenden Repository-Stand verwenden. **Nicht** den Git-Ordner löschen und nicht die Beispielkonfiguration über die produktive Konfiguration installieren.

Soweit die aktuelle Build-/Installationsstruktur es zulässt, zuerst in einem getrennten Buildverzeichnis bauen und den bisherigen Dienst bis zum erfolgreichen Build weiterlaufen lassen. Erst danach kontrolliert stoppen, das eindeutig geprüfte Binary installieren beziehungsweise atomar ersetzen und neu starten. Anschließend Health, Node-Hello, Rollenprüfung, Directory-Auflösung und Statuswechsel testen. Bei Misserfolg muss ein definierter Rückweg auf das gesicherte Binary bestehen.

`cargo clean` ist kein universeller Reparaturschritt. Es behebt weder eine falsche Directory-URL noch eine nicht angewendete Datei. Das pauschale Stoppen des Dienstes vor einem langen, möglicherweise fehlschlagenden Build vergrößert unnötig die Ausfallzeit.

### 11.4 Windows-Build und eindeutiger EXE-Pfad

Historisch wurde in CMD aus dem Repository-Root gebaut:

```cmd
cargo build --release --manifest-path system-backend\control-room\ui\Cargo.toml
```

Auf Linux müssen in derselben Manifestpfadangabe `/` statt der CMD-Backslashes verwendet werden. Ein Windows-Pfad war auf der TBS-Shell ausgeführt worden; daraus entstand `system-backendcontrol-roomuiCargo.toml`, das nicht existierte.

Die UI besitzt im untersuchten Paket einen eigenen Workspace. Deshalb sollte der tatsächliche Cargo-Targetpfad ermittelt und nicht zwischen zwei möglichen EXEs geraten werden. Folgende PowerShell-Prüfung ist ein **neuer vorgeschlagener Ablauf**, nicht eine historisch ausgeführte Reparatur:

```powershell
$manifest = 'system-backend\control-room\ui\Cargo.toml'
$metadataJson = cargo metadata --format-version 1 --no-deps --manifest-path $manifest
if ($LASTEXITCODE -ne 0) { throw 'Cargo-Metadaten konnten nicht gelesen werden.' }
$metadata = $metadataJson | ConvertFrom-Json
$exe = Join-Path $metadata.target_directory 'release\netcore-control-room-ui.exe'

Get-Process -Name netcore-control-room-ui -ErrorAction SilentlyContinue |
    Stop-Process
if (Test-Path $exe) {
    $backup = "$exe.previous-$(Get-Date -Format yyyyMMdd-HHmmss)"
    Move-Item -LiteralPath $exe -Destination $backup
}

cargo build --release --manifest-path $manifest
if ($LASTEXITCODE -ne 0) { throw 'UI-Build fehlgeschlagen; keine alte EXE starten.' }
if (-not (Test-Path $exe)) { throw 'Erwartetes UI-Binary fehlt.' }
& $exe --config "$env:APPDATA\netcore\control-room\operator.toml" --profile default
```

Die Sicherung des bisherigen Binary verhindert einen unbemerkten Start der alten Version, ohne den vollständigen Workspace, Konfigurationen oder alle Dateien rekursiv zu löschen. Nicht gespeicherte UI-Eingaben sind vor dem Schließen zu beachten. Weitere Verknüpfungen und kopierte EXEs müssen anhand ihrer tatsächlichen Pfade kontrolliert werden. Die sichtbare Versionszeile und der laufende EXE-Pfad sind gemeinsam zu erfassen.

## 12. Zusätzlich geprüfter Repository-Stand am 03.10.2026

Dieser Abschnitt beschreibt **nicht den historischen Entwicklungsendstand**, sondern gelesenen Code am Commit `df575519c7cf066d771511743175f22fede0826d` auf `Archiving`. Es wurde kein Live-Deployment und kein vollständiger Cargo-/GUI-Test dieses Commits durchgeführt.

### 12.1 Der Directory-Pull ist inzwischen im Quellcode vorhanden

`bins/netcore-control-room/src/http.rs` besitzt am 03.10.2026 die Directory-Routen, `SharedDirectory`, `directory_upstream_base`, `fetch_netcore_directory_upstream`, `netcore_directory_api_to_control_room` und `sync_directory_upstream`. Die fehlerhafte Zählung `resolved.len()` ist in den geprüften relevanten Routinen durch eine Objektlängenabfrage ersetzt. Der historische v5.14.2-Marker wird inzwischen in `/health` als `build_fix` ausgegeben. [R04] [R05]

Damit wäre die pauschale Aussage „es gibt noch keinen API-Pull“ für diesen geprüften Commit falsch. Offen bleibt, ob Jans laufender Dienst genau diesen Stand verwendet und ob er mit der richtigen Adresse arbeitet.

### 12.2 Unterschied zwischen TBS-Client und Control-Room-Client

Der TBS-Dashboard-Client `crates/tetra-entities/src/net_dashboard/radioid.rs` trägt nur noch aus historischen Gründen diesen Dateinamen. Sein Kommentar und Code beschreiben ausdrücklich den **NetCore Directory Server im LAN**, nicht einen notwendigen externen RadioID-Aufruf. Er liest `[netcore_directory]` aus der aktiven TBS-Konfiguration, berücksichtigt Umgebungsvariablen und verwendet einen `reqwest::blocking::Client` mit Timeout. [R10] [R11]

Die Gerätenormalisierung der TBS akzeptiert eine native Liste, ein Objekt mit `devices`-Liste oder eine bereits ISSI-indizierte Objektform. Die Control-Room-Pull-Konvertierung erwartet für die fünf Upstream-Ressourcen dagegen zunächst Arrays. Ein funktionierender TBS-Lookup ist daher kein Beweis, dass der LXC mit seiner eigenen Konfiguration und seinem anderen Parser dieselbe Antwort erfolgreich verarbeitet.

### 12.3 Tatsächliches geprüftes Directory-Datenmodell

`system-backend/directory/netcore-directory.py` enthält einen Python-/SQLite-Dienst mit nativen Geräte-, Basisstations-, Gruppen-, Gerätegruppen- und Status-APIs. In den Tabellen liegen unter anderem:

- Geräte: `issi`, `name`, `short`, `type`, `owner`, `role`, `icon`, `color`, `visible` und Zeitstempel.
- Gerätegruppen: `group_id`, `opta`, `name`, `short`, `type`, `color`, `status_sync`, `visible`.
- Mitglieder: `group_id`, `issi`, `role`.
- Statusdefinitionen: `code`, `label`, `severity`, `description`, `color`, `visible`.

Zusätzlich ist `/api/status-group-members?issi=...` in der Schnittstellenbeschreibung vorhanden. Somit sind Namen, Farben und eine explizite Statusgruppenlogik bereits Teil des zentralen Modells; sie müssen nicht aus frei formulierten SDS-Texten erfunden werden. [R12]

Im geprüften Control-Room-Konverter werden Gerätegruppenmitgliedschaften in `status_group` übersetzt, unter anderem aus `members` und `member_devices`. Der dort betrachtete Konverter übernimmt aber nicht alle Eigenschaften, insbesondere ist `status_sync` in diesem Übersetzungspfad nicht durchgereicht. Die Unterscheidung zwischen Funkgruppe, Gerätegruppe und synchronisierter Statusgruppe benötigt einen expliziten Vertrag. [R06]

### 12.4 Verbleibende Pull-Risiken im aktuellen Code

Folgende Punkte sind **statische Prüfbeobachtungen beziehungsweise daraus abgeleitete Risiken**, keine nachträglich gemessenen Produktionsfehler:

| Befund | Bedeutung für die Fortsetzung |
|---|---|
| Upstream-Default ist weiterhin `127.0.0.1:8095` | Bei separatem Directory-LXC muss der tatsächliche Endpunkt ausdrücklich gesetzt werden. |
| Die üblichen Directory-GETs lösen den Sync innerhalb einer gehaltenen Mutex-Sperre aus | Gleichzeitige UI-Abfragen können blockieren; es handelt sich in diesem Pfad nicht um einen unabhängig gecachten Hintergrundsync. |
| Fünf Endpunkte werden nacheinander abgefragt | Die gewünschte Einsekunden-UI-Aktualisierung ist dadurch nicht automatisch erreichbar. |
| Eigener `TcpStream`-HTTP-Client | Nur `http://`, kein im betrachteten Pfad eingebautes HTTPS-/Authorization-Verfahren; Chunked-Encoding und weitere HTTP-Eigenschaften werden nicht erkennbar durch eine vollwertige Clientbibliothek behandelt. |
| Read-/Write-Timeout jeweils 900 ms, aber kein explizites Connect-Timeout im Aufruf | Unterschiedliche Netzfehler können abweichende Wartezeiten verursachen. |
| Fehler werden häufig zu leeren Arrays reduziert | „Keine Daten“ und „Abruf fehlgeschlagen“ sind diagnostisch nicht sauber getrennt. |
| `any_data` prüft auch reine Gruppen-/Statusdaten | Upstream `ok: true` bedeutet nicht zwingend erfolgreiche Teilnehmernamensauflösung. |
| Directory-Merge ergänzt/überschreibt, entfernt aber nicht automatisch verwaiste Einträge | Löschungen, ausgeblendete Geräte und frühere falsche Imports können als Altbestand fortleben. |
| Importpfad verändert einen RAM-Wert | Eine persistente Wiederherstellung dieses Imports ist im betrachteten Pfad nicht nachgewiesen. |

Diese Befunde begründen konkrete Prüf- und Refactoringaufgaben. Sie rechtfertigen nicht, ohne Payload-/Netztest eine einzelne davon als bereits bewiesene Ursache der historischen Fehler zu benennen. [R05] [R06]

### 12.5 Am 03.10.2026 geprüfte UI: Namen vorhanden verdrahtet, Statusmodell weiterhin inkonsistent

Der geprüfte UI-Einstieg trägt jetzt **`Native UI v5.15.0 · Packet Data / Multi-PDCH`**. Das ist eine spätere Entwicklung als die abschließenden v5.14.2-Pakete dieses Vorhabens. Die Datei besitzt weiterhin Directory-, Tableau-, Karten- und Benutzerlogik. [R08]

`refresh_directory()` versucht zuerst `/api/directory/resolved`, dann bei einem Abruffehler `/api/directory`. Ein rekursiver Namensindex wird aufgebaut; strukturierte Directory-Daten werden zusätzlich geladen und mit lokalen Restwerten ergänzt. Bei bestimmten Fehlern wird weiterhin auf `local_directory` zurückgefallen. Die lokale Ergänzungs-/Fallbacklogik ist also im geprüften Code nicht vollständig entfernt, obwohl sie keine zweite Stammdatenpflege erzwingen soll. [R07]

`refresh_all()` ruft die APIs nacheinander auf, darunter weiterhin `/api/sds?limit=50`. Die Tableau-Anzeige kann somit keinen vollständigen historischen Status aller Geräte allein aus diesem Fenster garantieren. Auch die Refresh-Bedienung ist nicht überall verschwunden: Abgetrennte OS-Fenster zeigen im betrachteten Code weiterhin API-Angabe und Refreshbutton. [R07] [R09]

Die Statuslogik besteht weiterhin aus getrennten Entscheidungswegen:

```text
Code:
  Live-Code
  → Live-Text / state / registration_state
  → Directory-Subscriber-Status
  → letzter SDS-Code beziehungsweise Textinferenz

Text auf der Karte/Kachel:
  bevorzugt Text des letzten passenden SDS
  → erst danach andere Fallbacks
```

Damit können Code und Text aus verschiedenen Datensätzen stammen. Ohne gemeinsamen Ereigniszeitpunkt ist selbst ein vorhandener Name beziehungsweise plausibler Text kein Nachweis für die richtige Zahl. Der Code enthält außerdem weiterhin die heuristische gemeinsame Zuordnung von „Frei auf Wache“ und „Frei auf Funk“ zu `1`. [R13]

Der `DirectoryStatusConfig` der UI enthält in dem geprüften Ausschnitt `name`, `label`, `group` und `description`, jedoch **kein `color`-Feld**. Die Tableau-Legende und die Farbwahl sind hartcodiert. Der Server kann Farben liefern, ohne dass die UI sie in diesem Modell verarbeitet. Die zentrale Farbdefinition ist damit noch nicht durchgängig umgesetzt. [R08] [R14]

`subscriber_status_label()` verwendet bei fehlendem betrieblichem Status auch `energy_saving_mode` und Onlinezustand. Dies ist eine explizite technische Quelle der sichtbaren ESM-Texte. Diese Information sollte künftig separat ausgewiesen werden. [R15]

### 12.6 Am 03.10.2026 geprüfte UI: echte Modulfenster und individuelle Kacheln

`render_detached_windows()` verwendet `ctx.show_viewport_immediate` mit stabil abgeleiteten Viewport-IDs. Die zu öffnenden Tabs werden anhand von `can_access_tab()` gefiltert. Das ist ein konkreter Quellcodebeleg für native Modulfenster und RBAC-Berücksichtigung, aber kein neuer Mehrmonitor-/DPI-Test. [R09]

Das Tableau besitzt inzwischen einen Canvas mit pro Karte gespeicherten Offsets, Mausdelta-Verarbeitung und Resetbutton. Die automatische Platzierung verteilt Karten auf Spalten. `.max(480.0)` für die verfügbare Breite und Mindestbreiten der Karten zeigen gleichzeitig, dass sehr schmale Fenster und Clipping eigens getestet werden müssen. Eine dauerhafte Speicherung der Offsets über Programmneustarts ist aus dem betrachteten Tableau-Pfad nicht belegt. [R14] [R13]

### 12.7 Spätere Repository-Entwicklung: Open Lab, WebUI und Federation

Der geprüfte Kern hat zusätzlich die Module `gateway`, `operations` und `webui`. Er startet einen Poller für föderierte Dienste und kann Node-Gateway-Telemetrie aufnehmen. Die HTTP-Routen enthalten unter anderem Health-Live/Ready, Metriken, Services, Incidents, Shift-Log und weitere v1-Sichten. [R04] [R16]

Die aktuelle Beispielkonfiguration bezeichnet den Betrieb ausdrücklich als **Open Lab** und setzt `auth.enabled = false`. Sie nennt zusätzlich Federation-Polling mit fünf Sekunden, 1.200 ms Request-Timeout, eine Fehlergrenze von drei und Operations-Zustandsdateien unter `/var/lib/netcore-control-room/`. Das sind Repository-Vorgaben, nicht die letzte bewiesene Sicherheitskonfiguration des historischen LXC. [R17]

Daraus ergeben sich zwei Übergaberegeln:

1. Die historische Benutzer-/Passwort-Anforderung darf nicht durch blindes Kopieren der neuen Beispielkonfiguration unbemerkt deaktiviert werden.
2. Die neuen WebUI-/Federation-/Packet-Data-Funktionen dürfen nicht durch ein altes Komplettdateien-ZIP zurückgesetzt werden. Für die Fortsetzung ist der geprüfte Git-Stand die Integrationsbasis, nicht das zuletzt in den Arbeitsnotizen angebotene Artefakt.

## 13. Anforderungs- und Nachweismatrix zur Übergabe

| Thema | Historisch beschlossen | Historischer Nachweis | Am 03.10.2026 geprüfter Sourcebefund / verbleibende Arbeit |
|---|---|---|---|
| LXC-Kern und Node-Anbindung | Ja | Laufender Dienst, Hello/Telemetrie | Am 03.10.2026 zusätzlich Gateway/Federation; tatsächlichen Deploymentstand feststellen |
| SQLite-Command-Historie | Ja | Neustarttest erfolgreich | Schema-/Migrationsabgleich gegen aktuellen Betrieb fehlt |
| Kick-Erfolg | Bedienfunktion vorgesehen | Auftrag beendet, aber `success: false` | Erfolgreichen und negativen Fachfall getrennt testen |
| Benutzer/Passwort | Ja, ersetzt menschliche Tokens | Login und Admin-UI sichtbar | Am 03.10.2026 geprüfte Open-Lab-Vorgabe separat behandeln; aktive Konfiguration prüfen |
| Viewer ohne Adminmenü | Ja | Wunsch und Nachbesserung, kein kompletter Rollenbericht | Source-Gating vorhanden; API-/Fenster-/Logout-Negativtests fehlen |
| Echte OS-Fenster | Ja | Zwischenstand bestätigt | `show_viewport_immediate` vorhanden; DPI/Multimonitor kontrollieren |
| Reale Karte | Ja | Kartenkacheln und Geräte sichtbar | Performance-, Cache- und Offlinegrenzen prüfen |
| Cluster/Spiderfy | Ja | Im Screenshot sichtbar | Große Cluster, gleiche Koordinaten und Auswahl testen |
| Nur jüngste Position | Ja | Bereinigung geliefert | Zeit-/Identitätsregeln und Multi-Node-Dedupe abnehmen |
| Directory-Namen | Ja, zentral per API | Bis zuletzt nicht eindeutig erfolgreich | Source-Pull vorhanden; URL, Payload und konkrete ISSI Ende-zu-Ende prüfen |
| Statusnummer, Text und Farbe | Ja | Text teils korrekt, Zahl/Farbe beanstandet | Gemeinsames strukturiertes Statusmodell und Directory-Farben fehlen |
| Gruppe als eigenes Element | Ja | Konzepte/Quelltext; tatsächliche Namen fehlen | Mitgliedschaft, `status_sync`, Mischstatus und Detailbedienung klären |
| Eigene Kachel je ungruppiertem Gerät | Ja | Screenshots v5.11.9/v5.12 | Source vorhanden; Live-Abgleich mit echten Gruppen erforderlich |
| Maus-Anordnung | Ja | Grid/Reset sichtbar; keine vollständige Drag-Abnahme | Source vorhanden; Clipping, Persistenz, Reset und Fensterwechsel testen |
| Komplettdateien statt Patches | Ja | Viele ZIPs geliefert | Reproduzierbares, kompatibles und geprüftes Paket statt ungetesteter Dateisersetzung |
| Warning-freier, verifizierter Build | Cleanup gefordert | Wiederholte Fehler/Warnungen | Vollständige Buildmatrix und sichere Refaktorierung erforderlich |

## 14. Noch relevante Ideen, kleine Wünsche und offene Designfragen

Die folgenden Punkte bleiben als Roadmap-Kandidaten erhalten. Wo sie nur vorgeschlagen wurden, wird keine Nutzerfreigabe behauptet.

**Karte:** Cluster ein-/ausschaltbar; Labels ein-/ausschaltbar; Positionsverlauf nur bewusst zuschalten; identische Koordinaten per Spiderfy oder Geräteliste; Geräteauswahl auch in großen Clustern; Informationen zu Namen, ISSI, Typ, Gruppen, Status und Zeitpunkt; optionale RF-/RSSI-Information nur bei tatsächlich vorhandenen Daten. Eine Notruf-/Warnpriorität in der Clusterfarbe wurde angeregt, aber nicht vollständig abgenommen. Auswahlfarbe und fachliche Statusfarbe sollten unterschiedliche Bedeutungen behalten.

**Tableau:** Kompakteres Raster nach Leitstellenvorbild; Sortierung nach Gruppe/Name oder Status/Name als Vorschlag; Gruppenkarte mit Übersicht und aufklappbaren Mitgliedern; Drag nur an klarer Griffzone, um Geräteaktionen nicht zu blockieren; Rücksetzen und Wiederherstellen der Arbeitsanordnung. Dauerhafte Layouts je Benutzer/Arbeitsplatz sind ein naheliegender, hier noch nicht implementiert bestätigter Ausbau.

**Bedienoberfläche:** Sinnvolle leere Zustände statt falscher Daten; keine langen Diagnose-/API-Texte in der normalen Arbeitsfläche; technische Diagnosen in einen passenden Bereich verschieben, ohne sie zu verlieren; lesbare Uhrzeit statt Debugdarstellung von `SystemTime`; konsistente Modulbezeichnungen; funktionierende Shortcuts und obere Aktionsleiste. Die Screenshots zeigen wiederholt Debugzeit-/Profilreste, obwohl eine saubere Oberfläche verlangt wurde.

**Teilnehmerdaten:** Infrastrukturkennzeichnung, echte Gerätenamen, stabile Identität, keine Phantomteilnehmer durch SDS-Ziele oder falsch interpretierte Statusnummern; Gruppen- und Gerätestatus nicht aus ESM ableiten. Ob reine Offline-Directory-Geräte standardmäßig sichtbar bleiben, ist fachlich zu entscheiden; „alte Sachen weg“ darf nicht zu unkontrolliertem Löschen gültiger Stammdaten führen.

**Betrieb/Lieferung:** Verlässlicher Versionsnachweis für Backend und EXE; ein klarer Installationsort; keine veraltete EXE über eine andere Verknüpfung starten; vollständige Anleitung für jedes Zielsystem; `.vs` und Buildartefakte ausgeschlossen; keine manuelle Nachpflege identischer Namen auf mehreren Geräten.

**Authentifizierung:** Rolle und sichtbare Module konsistent nach Login/Logout aktualisieren; bereits geöffnete unzulässige Fenster schließen; keine erneute Vermischung von TBS-Token, Benutzerpasswort und Directory-Zugriff. Eine spätere zentrale Identity-/RBAC-Roadmap existiert im am 03.10.2026 geprüften Repository-Kontext, wurde hier aber nicht als bereits ausgerollte Lösung geprüft.

## 15. Priorisierte nächste Schritte und Abhängigkeiten

Die Prioritäten in diesem Abschnitt sind eine aus dem dokumentierten Zustand abgeleitete Arbeitsreihenfolge für die Fortsetzung. Sie wurden nicht als separate Issues oder externe Roadmap-Dateien angelegt.

### P0 – Verlässliche Grundlage und Schutz des Betriebs

**CR-ARCH-01: Tatsächlichen Source-/Binary-Stand feststellen.** LXC-Commit, Änderungen, ExecStart, laufendes Binary und Windows-EXE-Pfad erfassen. Zwischen Compilerfehler, leerem Directory, falschem Status und alter Anzeige unterscheiden. Keine weitere Versionsnummer allein als Lösung ausgeben.

**CR-ARCH-02: Reproduzierbare Buildprüfung.** Den geprüften vollständigen Workspace verwenden. Kern und Operator auf dem LXC-Zieltyp, UI für Windows prüfen. Compilation, vorhandene Tests und fachliche Abnahme getrennt protokollieren. Keine globale Regex-Entfernung von Funktionen; keine stillen Fallbacks auf ältere Arbeitsverzeichnisse beim Paketbau.

**CR-ARCH-03: Sichere Updatepakete.** Manifest mit Basiscommit, vollständiger Dateiliste, Zielpfaden und Hashes; Konfigurationen und Daten ausnehmen beziehungsweise explizit migrieren. Erst erfolgreichen Build sichern, dann kontrollierter Dienstwechsel. Die zurückgezogene Löschanleitung nicht weiterverwenden.

### P0 – Zentrale Namen und Status wirklich Ende-zu-Ende

**CR-ARCH-04: Funktionierenden TBS-Directory-Pfad als Referenz verwenden.** Tatsächliche URL, Aktivierung, Schema und gegebenenfalls Authentifizierung vergleichen. Identische native API-Ressource vom Control-Room-LXC abrufen. Erwartete Antwort für mindestens eine reale bekannte ISSI dokumentieren; Testdaten anonymisieren, ohne Identitätsbeziehungen zu zerstören.

**CR-ARCH-05: Definierten Directory-Adapter statt Deep-Search bauen.** `devices`, `basestations`, `groups`, `device_groups` und Statusdefinitionen getrennt modellieren. Unterstützte Antwortformen ausdrücklich festlegen. Name/Shortname/OPTA, `visible`, Mitgliedschaften, `status_sync`, Code, Farbe und Severity sinnvoll übernehmen. Herkunft, letzte erfolgreiche Aktualisierung und Fehlergrund sichtbar machen.

**CR-ARCH-06: Status als eigenen Zustand führen.** Raw-Code, Quell-ISSI, Node/Netzkontext, Empfangszeit, Nachrichtenidentität und Gültigkeit speichern. Nicht auf die letzten 50 SDS beschränken. Code/Text/Farbe aus demselben ausgewählten Zustand und derselben Directory-Definition rendern. Unknown nicht durch irgendeine passende Wortteilnummer ersetzen. Reihenfolge bei verspäteten und duplizierten Nachrichten eindeutig festlegen.

**CR-ARCH-07: Namen auf allen Ansichten abnehmen.** Dieselbe ISSI muss im Tableau, in der Karte, im Cluster und in den Geräteinformationen dieselbe korrekte Bezeichnung erhalten. Eine zusätzliche numerische Kennung ist Detailinformation, kein doppelter Name. Ein Indexzähler genügt nicht als Test.

### P1 – Robustheit, Rollen und Bedienung

**CR-ARCH-08: Directory-Fetch entkoppeln.** Vollwertigen HTTP-Client und definierte Timeouts verwenden; Netzoperationen nicht unter der zentralen Directory-Sperre ausführen; Cache/TTL, Fehler-/Leerzustand und letzte gültige Daten unterscheiden. Löschungen und Änderungen korrekt übernehmen. Bei mehreren UI-Clients keine unnötige fünfteilige Anfragekaskade pro Refresh erzeugen.

**CR-ARCH-09: Gruppenfachlichkeit präzisieren.** Funk-GSSI, organisatorische Gerätegruppe und synchronisierte Statusgruppe unterscheiden. Gruppenstatus bei abweichenden Mitgliedern festlegen: letzter autorisierter Gruppenstatus, definierte Aggregation oder sichtbarer Mischzustand – nicht beliebig das erste Gerät. Mitgliedschaften/Details und gruppenlose Einzelgeräte prüfen.

**CR-ARCH-10: RBAC und Security-Modus abgleichen.** Historische Loginanforderung gegen geprüfte Open-Lab-Konfiguration dokumentiert entscheiden. Viewer-/Operator-/Admin-API-Tests, Direktaufrufe, bereits offene Fenster, Logout und Benutzeränderungen prüfen. Eine Laborkonfiguration nicht als sichere Betriebsfreigabe ausgeben.

**CR-ARCH-11: UI-Interaktion stabilisieren.** Drag-Hit-Tests, Größenwechsel, DPI, Scrollbereich und Mausbedienung zusammen testen. Einstellungen je Arbeitsplatz nur dann persistieren, wenn Format, Rücksetzung und Benutzerbezug definiert sind. API-/Refreshreste in Nebenfenstern bereinigen; Hauptmenüaktionen vollständig durchtesten.

### P2 – Feinschliff und Ausbau

**CR-ARCH-12:** Erweiterte Clusterdarstellung mit Notruf-/Warnsemantik, optionale Label-/Verlaufsschalter, bessere große Clusterlisten und kontrollierte Online-/Offline-Anzeige.

**CR-ARCH-13:** Modulare Aufteilung der sehr großen UI-Datei, gemeinsame DTOs/Resolver und Regressionstests, damit Parser, Kartenlabels und Tableau keine getrennten Regeln für dieselben Daten entwickeln.

**CR-ARCH-14:** Bestehende neuere Packet-Data-, Federation-, Operations- und WebUI-Funktionen in die weitere Planung einbeziehen, ohne die historische Anforderungsliste mit unbelegten Umsetzungserfolgen zu vermischen.

## 16. Empfohlene Abnahmetests für die Fortsetzung

**Nicht bei der Quellenprüfung vom 03.10.2026 ausgeführt.** Diese Matrix definiert, was ein künftiges „fertig“ belegen muss.

| Test | Aufbau | Erwartung |
|---|---|---|
| N1 – Name | Reales Directory-Gerät mit bekannter ISSI und Name | Exakt derselbe Name im Control-Room-resolved-API, Tableau, Karte und Cluster |
| N2 – Namensänderung | Name zentral ändern | Änderung nach definiertem Intervall sichtbar, keine lokale Nachpflege |
| N3 – Leerer/defekter Abruf | Gültig leere Antwort, Timeout und HTTP-Fehler getrennt simulieren | Eindeutig verschiedene Diagnosen; keine falsche Erfolgsmeldung |
| N4 – Namensräume | Geräte-ISSI und Gruppen-/Status-ID numerisch gleich wählen | Keine gegenseitige Verwechslung im Namensindex |
| S1 – Statuswechsel | Zwei zentral definierte verschiedene Codes senden | Zahl, Text und Farbe wechseln gemeinsam und entsprechen dem zentralen Plan |
| S2 – Unbekannter Status | Nicht definierter Code | Rohwert beziehungsweise Unknown sichtbar; keine erfundene Semantik |
| S3 – Reihenfolge | Alte/duplizierte Meldung nach neuer zustellen | Kein Rücksprung ohne fachlich definierte Regel |
| S4 – Historienfenster | Mehr als 50 andere SDS nach einem Status erzeugen | Aktueller Gerätestatus bleibt erhalten |
| S5 – Neustart | Kern und UI nach gespeichertem Status neu starten | Definierte Wiederherstellung beziehungsweise klarer Unknown-Zustand, keine irreführende ESM-Ersatznummer |
| G1 – Gruppen | Eine Gruppe mit mehreren Geräten plus ein gruppenloses Gerät | Eine Gruppenkarte mit Details und eigene Einzelgerätekarte |
| G2 – Statussync | `status_sync` aktiv/inaktiv und gemischte Mitglieder | Verhalten folgt dokumentiertem Gruppenvertrag |
| P1 – Positionen | Mehrere Positionsmeldungen je ISSI, gleiche Koordinaten mehrerer Geräte | Eine aktuelle Position je Gerät; Cluster ohne Verlust individueller Auswahl |
| U1 – Fenster | Module auf mehrere Monitore, unterschiedliche Skalierungen | Echte OS-Fenster, lesbare Inhalte, kein Drift/Clipping |
| U2 – Drag | Einzel-/Gruppenkarten bewegen, scrollen, Fenstergröße ändern | Stabile Position, keine unabsichtlichen Aktionen, Reset funktioniert |
| A1 – Rollen | Viewer, Operator, Admin sowie ungültige Anmeldung | Menü und API konsistent; unzulässige Aktionen verweigert |
| C1 – Command | Erfolgreicher, abgelehnter und unbeantworteter Auftrag | Transportstatus, fachliches Ergebnis und Timeout sauber getrennt |
| D1 – Paket | Sauberer passender Workspace und unveränderte Betriebsdaten | Reproduzierbarer Build; nachvollziehbarer EXE-/Dienstversionswechsel; Rückweg möglich |

## 17. Quellen und Nachweisregister

### 17.1 Historische Arbeitsgrundlagen

Die folgenden Kennungen verweisen auf erhaltene Entwicklungs- und Betriebsbelege.

| Kennung | Beleg | Aussagekraft |
|---|---|---|
| H01 | Eingefügter Text(45).txt | Früher Health-/State-Test am 30.06.2026 im TBS-Shellkontext |
| H02 | Eingefügter Text(46).txt | Overview/RF/Health/Commands/Events der frühen API |
| H03 | Eingefügter Text(47).txt | Detail-API-Abfragen zu Teilnehmern/Gruppen/Rufen und weiteren Zuständen |
| H04 | Eingefügter Text(48).txt | LXC-Buildfehler durch fehlendes SoapySDR/pkg-config |
| H05 | Eingefügter Text(49).txt | TBS-Start, Trägerparameter, SXceiver, Brew, Dashboard, akzeptiertes Control-Room-Hello |
| H06 | Eingefügter Text(50).txt | Fehlendes Node-Token, Restartschleife, Drop-in ohne gültigen Abschnitt |
| H07 | Betriebslog Release-Build nach `response_value.clone()` | Früher erfolgreicher Kern-/Operator-Build |
| H08 | Betriebslog Command-ID und SQLite-/Neustartabfrage | Command-Audit dauerhaft; Kick-Antwort fachlich negativ |
| H09 | Betriebslog HTTP 200/401 und gültiger Operator-Aufruf | Tatsächliche Authentifizierungstests der Tokenphase |
| H10 | Betriebslog `/node` unauthorized plus TBS Broken pipe | Node-Authentifizierungsfehler mit Reconnectschleife |
| H11 | Betriebsrückmeldung „jetzt ist die wieder online“ | Wiederhergestellte Node-Sicht nach Auth-Fix |
| H12 | Screenshots Native UI, Karten-/Cluster- und Tableau-Versionen | Gezeigte Darstellung/Versionslabel; keine vollständige Backend- oder Featureabnahme |
| H13 | Nutzerantwort `{"hide_infrastructure": true}` | Leere Stammdatensicht des konkret abgefragten Control Rooms |
| H14 | Compilerfehler E0599 in `http.rs` | `.len()` auf `serde_json::Value` verhinderte den damaligen Kernbuild |
| H15 | Abschließende kurze Rückmeldungen „immernoch“ | Fortbestehende Unzufriedenheit/Fehler; ohne neue Ausgabe nicht eindeutig einer Fehlerklasse zuordenbar |

Die in den Arbeitsnotizen sichtbaren Stack-Versionen `v1.3.0-02130261` und `v1.3.0-691c6fb7` sind historische Ausgabezeichenketten. Sie wurden nicht zu einem vollständigen Git-Commit und einem konkreten ZIP-/Deploymentstand aufgelöst. Für die historischen UI-/Directory-Pakete wurde kein verlässlicher PR oder Release-Tag nachgewiesen. Deshalb werden keine PR-Nummern oder Commitzuordnungen ergänzt.

### 17.2 Unveränderliche Repository-Referenzen der Archivprüfung

Die folgenden Links beziehen sich auf den festgehaltenen Prüfreferenz-Commit; spätere Branchänderungen verändern diese Belege nicht.

- **R01:** Archivbranch-Prüfreferenz und Commitmetadaten.
- **R02:** Zusätzlich gelesener Main-Commit.
- **R03:** Vergleich beider Prüfreferenzen.
- **R04:** Control-Room-HTTP-Routen, Health-Marker und neue v1-Sichten.
- **R05:** Directory-Import, Refresh, resolved-Ausgabe, Upstream-Auswahl und HTTP-Client.
- **R06:** Directory-API-Übersetzung für Geräte, Gruppen, Mitglieder und Statusdefinitionen.
- **R07:** UI-RBAC, sequentieller Refresh, 50-SDS-Limit und Directory-Refresh.
- **R08:** UI-Version und Directory-DTOs.
- **R09:** Echte OS-Fenster und verbliebene Nebenfenster-API-/Refreshanzeige.
- **R10/R11:** TBS-Directory-Client, Konfiguration, HTTP-Abruf und Gerätenormalisierung.
- **R12:** Directory-Server und zentrales SQLite-Datenmodell.
- **R13:** Tableau-Statusauswahl, Textinferenz und Drag-Verarbeitung.
- **R14:** Tableau-Legende und Canvas-Platzierung.
- **R15:** Namensauflösung und ESM-/Online-Statusfallback der UI.
- **R16:** Am 03.10.2026 geprüfter Backend-Start mit Operations/Gateway.
- **R17:** Am 03.10.2026 geprüfte Open-Lab-Beispielkonfiguration.

[R01]: https://github.com/JanHG98/netcore-tetra/commit/df575519c7cf066d771511743175f22fede0826d
[R02]: https://github.com/JanHG98/netcore-tetra/commit/6aa9be8f74ab731f72dc133a5f8e90c5018c626d
[R03]: https://github.com/JanHG98/netcore-tetra/compare/6aa9be8f74ab731f72dc133a5f8e90c5018c626d...df575519c7cf066d771511743175f22fede0826d
[R04]: https://github.com/JanHG98/netcore-tetra/blob/df575519c7cf066d771511743175f22fede0826d/bins/netcore-control-room/src/http.rs#L1-L390
[R05]: https://github.com/JanHG98/netcore-tetra/blob/df575519c7cf066d771511743175f22fede0826d/bins/netcore-control-room/src/http.rs#L420-L610
[R06]: https://github.com/JanHG98/netcore-tetra/blob/df575519c7cf066d771511743175f22fede0826d/bins/netcore-control-room/src/http.rs#L610-L880
[R07]: https://github.com/JanHG98/netcore-tetra/blob/df575519c7cf066d771511743175f22fede0826d/system-backend/control-room/ui/src/main.rs#L940-L1180
[R08]: https://github.com/JanHG98/netcore-tetra/blob/df575519c7cf066d771511743175f22fede0826d/system-backend/control-room/ui/src/main.rs#L1-L180
[R09]: https://github.com/JanHG98/netcore-tetra/blob/df575519c7cf066d771511743175f22fede0826d/system-backend/control-room/ui/src/main.rs#L1750-L1930
[R10]: https://github.com/JanHG98/netcore-tetra/blob/df575519c7cf066d771511743175f22fede0826d/crates/tetra-entities/src/net_dashboard/radioid.rs#L1-L160
[R11]: https://github.com/JanHG98/netcore-tetra/blob/df575519c7cf066d771511743175f22fede0826d/crates/tetra-entities/src/net_dashboard/radioid.rs#L160-L340
[R12]: https://github.com/JanHG98/netcore-tetra/blob/df575519c7cf066d771511743175f22fede0826d/system-backend/directory/netcore-directory.py#L1-L130
[R13]: https://github.com/JanHG98/netcore-tetra/blob/df575519c7cf066d771511743175f22fede0826d/system-backend/control-room/ui/src/main.rs#L2520-L2730
[R14]: https://github.com/JanHG98/netcore-tetra/blob/df575519c7cf066d771511743175f22fede0826d/system-backend/control-room/ui/src/main.rs#L2200-L2455
[R15]: https://github.com/JanHG98/netcore-tetra/blob/df575519c7cf066d771511743175f22fede0826d/system-backend/control-room/ui/src/main.rs#L3650-L3880
[R16]: https://github.com/JanHG98/netcore-tetra/blob/df575519c7cf066d771511743175f22fede0826d/bins/netcore-control-room/src/main.rs
[R17]: https://github.com/JanHG98/netcore-tetra/blob/df575519c7cf066d771511743175f22fede0826d/system-backend/control-room/config/control-room.example.toml#L1-L190

### 17.3 Weitere relevante Pfade für die Fortsetzung

```text
bins/netcore-control-room/Cargo.toml
bins/netcore-control-room/src/auth.rs
bins/netcore-control-room/src/config.rs
bins/netcore-control-room/src/gateway.rs
bins/netcore-control-room/src/http.rs
bins/netcore-control-room/src/main.rs
bins/netcore-control-room/src/operations.rs
bins/netcore-control-room/src/persistence.rs
bins/netcore-control-room/src/server.rs
bins/netcore-control-room/src/state.rs
bins/netcore-control-room/src/webui.rs
bins/netcore-control-room/src/ws.rs
system-backend/control-room/config/control-room.example.toml
system-backend/control-room/systemd/netcore-control-room.service
system-backend/control-room/operator/Cargo.toml
system-backend/control-room/operator/src/main.rs
system-backend/control-room/ui/Cargo.toml
system-backend/control-room/ui/src/main.rs
crates/tetra-config/src/bluestation/sec_control_room.rs
crates/tetra-entities/src/net_control_room/
crates/tetra-entities/src/net_dashboard/radioid.rs
crates/tetra-entities/src/net_dashboard/server.rs
crates/tetra-entities/src/net_dashboard/ui/
system-backend/directory/netcore-directory.py
misc/ID-Server/netcore_directory_server.py
.github/workflows/dashboard-ui-tests.yml
.github/workflows/service-ui-tests.yml
```

Ein Teil dieser Pfade wurde vollständig beziehungsweise ausschnittsweise gelesen; andere wurden über das Repository-Inventar als Anschlussstellen festgestellt. Das ist keine Behauptung, jede dieser Dateien und jeden Test inhaltlich vollständig geprüft zu haben. Die beiden genannten Workflow-Dateien betreffen spätere WebUI-Arbeiten und sind nicht automatisch eine Build-/Abnahmeprüfung der nativen Windows-Anwendung.

## 18. Artefaktinventar und Paketgrenzen

### 18.1 Geprüftes letztes ZIP

| Merkmal | Befund |
|---|---|
| Dateiname | `netcore-control-room-v5-14-2-directory-pull-verified-files.zip` |
| Größe | 120.770 Bytes |
| Dateieinträge | 59 |
| SHA-256 | `7b5f342ea1e140ec801dfc011d99e880fb6940730ed5bcbc761c35180495e319` |
| ZIP-CRC-Prüfung | `zipfile.testzip()` ergab keinen fehlerhaften Eintrag. |
| Root-Workspace enthalten | Nein, Root-`Cargo.toml` fehlt. |
| Vollständige `tetra-entities`-Quellen enthalten | Nein. |
| Upstream-Pull-Helfer in Backend-HTTP enthalten | Ja. |
| `resolved.len()` im untersuchten Backend-HTTP | Nicht mehr enthalten. |
| Vollständiger Cargo-Build dieses ZIP | Nicht ausgeführt; ZIP allein ist dafür kein vollständiger Workspace. |
| Produktiver Namens-/Status-End-to-End-Test | Nicht belegt. |

CRC-Prüfung und Hash sichern die Identität beziehungsweise Lesbarkeit des untersuchten Artefakts. Sie beweisen weder Rust-Typkorrektheit noch API-Kompatibilität oder richtige Statussemantik.

### 18.2 Verfügbare Paketfamilien

In der Arbeitsumgebung waren insgesamt 57 ZIP-Dateien vorhanden, darunter `flowstation-main(4).zip` und die folgende Control-Room-Paketserie. Die Liste dokumentiert **Verfügbarkeit und Verlauf**, nicht eine Vollprüfung jedes Quelltextes und nicht die Empfehlung zur erneuten Installation.

```text
netcore-control-room-api-overview-files.zip
netcore-control-room-auth-token-files.zip
netcore-control-room-cargo-inherit-fix-files.zip
netcore-control-room-core-files.zip
netcore-control-room-dependency-fix-files.zip
netcore-control-room-detail-api-files.zip
netcore-control-room-locations-statefix-files.zip
netcore-control-room-native-ui-v1-files.zip
netcore-control-room-native-ui-v2-multiwindow-map-files.zip
netcore-control-room-native-ui-v3-real-os-windows-map-files.zip
netcore-control-room-native-ui-v4-live-map-files.zip
netcore-control-room-native-ui-v4-1-live-map-buildfix-files.zip
netcore-control-room-native-ui-v4-2-interactive-map-files.zip
netcore-control-room-native-ui-v4-3-smooth-map-device-info-files.zip
netcore-control-room-native-ui-v4-4-map-wheel-table-files.zip
netcore-control-room-native-ui-v4-6-directory-cleanup-files.zip
netcore-control-room-native-ui-v4-7-directory-first-files.zip
netcore-control-room-node-files.zip
netcore-control-room-operator-client-files.zip
netcore-control-room-operator-profiles-v1-files.zip
netcore-control-room-persistence-config-files.zip
netcore-control-room-rbac-token-registry-complete-v2-files.zip
netcore-control-room-rbac-token-registry-consolidated-files.zip
netcore-control-room-rbac-token-registry-files.zip
netcore-control-room-v5-user-password-rbac-files.zip
netcore-control-room-v5-1-user-password-rbac-buildfix-files.zip
netcore-control-room-v5-2-user-password-rbac-ui-buildfix-files.zip
netcore-control-room-v5-3-user-password-rbac-node-basic-fix-files.zip
netcore-control-room-v5-4-responsive-ui-files.zip
netcore-control-room-v5-5-rbac-ui-gating-files.zip
netcore-control-room-v5-6-elp-layout-ui-files.zip
netcore-control-room-v5-7-responsive-layout-fix-files.zip
netcore-control-room-v5-8-compact-header-responsive-files.zip
netcore-control-room-v5-9-clean-workplace-ui-files.zip
netcore-control-room-v5-9-1-clean-workplace-buildfix-files.zip
netcore-control-room-v5-9-2-clean-workplace-buildfix-files.zip
netcore-control-room-v5-9-3-clean-workplace-navfix-files.zip
netcore-control-room-v5-10-map-cluster-spiderfy-files.zip
netcore-control-room-v5-10-1-map-no-table-files.zip
netcore-control-room-v5-11-status-tableau-files.zip
netcore-control-room-v5-11-1-status-tableau-buildfix-files.zip
netcore-control-room-v5-11-2-status-tableau-buildfix-files.zip
netcore-control-room-v5-11-3-status-tableau-sds-fallback-files.zip
netcore-control-room-v5-11-4-status-tableau-buildfix-files.zip
netcore-control-room-v5-11-5-status-tableau-sds-robust-files.zip
netcore-control-room-v5-11-6-status-tableau-sds-exakt-files.zip
netcore-control-room-v5-11-7-directory-first-status-layout-files.zip
netcore-control-room-v5-11-8-status-tableau-final-fix-files.zip
netcore-control-room-v5-11-9-status-tableau-buildfix-files.zip
netcore-control-room-v5-12-0-status-tableau-drag-directory-files.zip
netcore-control-room-v5-12-1-directory-names-fix-files.zip
netcore-control-room-v5-12-2-directory-deep-index-files.zip
netcore-control-room-v5-13-0-api-directory-sync-files.zip
netcore-control-room-v5-14-0-directory-api-pull-files.zip
netcore-control-room-v5-14-1-directory-api-pull-buildfix-files.zip
netcore-control-room-v5-14-2-directory-pull-verified-files.zip
```

Die Pakete enthalten teilweise gleichnamige Dateien und viele historische `*_APPLY.md`-Anleitungen. Ihre Paketbezeichnung ist kein Herkunftsbeweis des installierten Binary. Alte Anweisungen können außerdem widersprüchliche Auth-, Directory- und Installationsansätze enthalten; die ausdrücklichen Korrekturen dieses Archivs haben für die Fortsetzung Vorrang vor einem unkritischen Wiederausführen solcher Anleitungen.

Namen und Befunde der ZIPs und Screenshots sind dokumentiert. Die Originaldateien bleiben für eine vollständige Reproduktion erforderlich.

### 18.3 Bilder und übrige Anhänge

Die sichtbaren Programmscreenshots wurden als Nachweis der gezeigten Programmversion und Darstellung berücksichtigt. Sie zeigen keine gesicherte vollständige Backendkonfiguration. Die Referenzbilder zu SELECTRIC, EDP-/Einsatzleitplätzen, Kommunikationssoftware, Objektstatus, Personalübersichten und Sirenenkarten wurden als Gestaltungsvorlagen eingeordnet. Personen-, Patienten- und fremde Namensdaten aus Referenzbildern werden nicht als NetCore-Stammdaten übernommen.

Die sechs eingefügten Textdateien 45 bis 50 sind ausdrücklich als Betriebslogs berücksichtigt. Fehlende Diagnoseausgaben bleiben als Quellenlücke bestehen.

### 18.4 ETSI-Referenzbestand

25 PDF-Dateien waren verfügbar. Die folgende Liste sichert die Dateinamen und die erkennbare fachliche Zuordnung. Es wurde kein vollständiger Normabgleich der Implementierung durchgeführt und keine Aktualitätsrecherche zu späteren Normrevisionen vorgenommen.

| Datei | Erkennbare Zuordnung / Hinweis |
|---|---|
| `en_3003920308v010401p.pdf` | EN 300 392-3-8, Generic Speech Format Implementation |
| `en_30039209v010701p.pdf` | EN 300 392-9, allgemeine Anforderungen an Zusatzdienste |
| `ts_10081201v020205p.pdf` | TS 100 812-1, UICC/TSIM-ME-Grundlagen |
| `en_3003921201v010202p.pdf` | EN 300 392-12-1, Call Identification, Stage 3 |
| `en_3003920304v010301p.pdf` | EN 300 392-3-4, ISI Short Data Service |
| `en_3003921117v010102p.pdf` | EN 300 392-11-17, Include Call, Stage 2 |
| `en_3003921114v010101p.pdf` | EN 300 392-11-14, Late Entry, Stage 2 |
| `es_20081202v020401m.pdf` | ES 200 812-2, TSIM-Anwendung; beigefügte Fassung als Final Draft gekennzeichnet |
| `es_20081201v020205p.pdf` | ES 200 812-1, physische/logische UICC-Eigenschaften |
| `en_300812v020101p.pdf` | EN 300 812, SIM-ME-Schnittstelle |
| `en_3003921101v010201p.pdf` | EN 300 392-11-1, Call Identification, Stage 2 |
| `en_3003921006v010401p.pdf` | EN 300 392-10-6, Call Authorized by Dispatcher |
| `en_3003921018v010301p.pdf` | EN 300 392-10-18, Barring of Outgoing Calls |
| `en_3003921216v010400a.pdf` | EN 300 392-12-16, Pre-emptive Priority Call; Draft 2026-03 |
| `en_30039201v010601p.pdf` | EN 300 392-1, General Network Design |
| `ets_30039214e01v.pdf` | PICS-Proforma; beigefügte Final-Draft-Fassung von 1997 |
| `en_30039207v030501p.pdf` | EN 300 392-7, Security |
| `en_30039401v030301p.pdf` | EN 300 394-1, Radio Conformance Testing |
| `en_3003920313v010201p.pdf` | EN 300 392-3-13, transportunabhängiger ISI-Gruppenruf |
| `en_30039502v010303p.pdf` | EN 300 395-2, TETRA-Codec |
| `en_3003920303v010301p.pdf` | EN 300 392-3-3, ISI-Gruppenruf |
| `en_30039205v020701p.pdf` | EN 300 392-5, Peripheral Equipment Interface |
| `en_3003920315v010500a.pdf` | EN 300 392-3-15, transportunabhängiges ISI-Mobility-Management; Draft 2026-04 |
| `en_30039202v030801p.pdf` | EN 300 392-2, Air Interface |
| `ETSI.pdf` | Umfangreiche zusammengeführte Sammlung; nicht pauschal eine einzelne aktuelle Normfassung |

Insbesondere ist `protocol_id = 218` in diesem Archiv ein **beobachteter projektspezifischer Erkennungspunkt der gezeigten Statusnachrichten**. Aus den beigefügten Normen wurde hier keine universelle Zuordnung dieses Wertes oder der UI-Zahlen 1 bis 8 hergeleitet. Drafts werden nicht als verabschiedete aktuelle Standards behandelt.

## 19. Abschluss und Wiederaufnahmehinweis

Der Entwicklungsstand umfasst einen real verwendeten Control-Room-Kern und eine laufende native Bedienoberfläche, aber keine abgeschlossene Directory-/Statusintegration. Die größte Fehlerquelle der Arbeit war das wiederholte Bearbeiten der Darstellung ohne vorherigen Nachweis der tatsächlich verfügbaren Datenquelle und ihres Vertrags. Dazu kamen unvollständige Buildprüfungen, fehleranfällige Generatoränderungen und teilweise unpräzise Deploymentanleitungen.

Für die Wiederaufnahme gilt: **Zuerst den aktuellen vollständigen Git-Stand und den tatsächlichen Directory-Datenfluss prüfen; dann einen gemeinsamen strukturierten Geräte-/Statuszustand verwenden; erst danach Layout und Komfort weiter ausbauen.** Keine erfundenen Namen, keine Wortteilheuristik als maßgeblicher Statusplan und keine Wiederholung der Repository-Löschanweisung.

Der geprüfte Quellcodebefund ist gegenüber dem historischen Entwicklungsstand fortgeschritten, enthält aber weiterhin relevante offene Stellen. Weder ein positiver Healthcheck noch eine nichtleere Indexzahl, ein erfolgreiches ZIP-CRC-Ergebnis oder ein Versionslabel ersetzt die konkreten Abnahmetests aus Abschnitt 16.
