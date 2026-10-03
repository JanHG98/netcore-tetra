# Technische Abschlussdokumentation: WERMA-Racksignalisierung, Rack-Aufbau und BPI-R4 Pro

> **Ergebnis des Fachchats:** Machbarkeits- und Architekturplanung, keine in diesem Chat ausgeführte Hardwareintegration. Fünf vorhandene WERMA-Farben sollen am Rack nutzbar werden. Vorgeschlagen wurde eine Trennung zwischen Zustandsauswertung in NetCore und elektrischer Ansteuerung durch einen separaten Pi. Als spätere Planungsrichtung kam ein eigenständiger Banana Pi BPI-R4 Pro für Routing/Switching hinzu; Jan ergänzte ausdrücklich einen kleinen unmanaged 5-Port-Switch als Bedarfsoption.
>
> **Zusätzlich geprüft am 03.10.2026:** Die damalige Repository-Adresse `JanHG98/flowstation` löst inzwischen auf `JanHG98/netcore-tetra` auf. Control-Room-Telemetrie und APIs bestehen weiterhin. Ein Hardware-Gateway für Rack-/Umgebungsdaten ist inzwischen vorhanden; ein fertiger WERMA-Ausgangstreiber ist in den geprüften Integrationsstellen nicht nachgewiesen. OpenWrt-PR #21083 für den BPI-R4 Pro 8X wurde inzwischen gemergt. Keine dieser Feststellungen ist eine Bestätigung, dass das geplante Rack oder seine Lampenansteuerung vor Ort läuft.

## 1. Metadaten und Geltungsbereich

| Feld | Inhalt |
|---|---|
| Projekt | NetCore-Tetra |
| Thema | Fünffarbige WERMA-Signalisierung, separater Rack-Agent, kompakter Rack-Aufbau, BPI-R4 Pro als Router/Switch und unmanaged Porterweiterung |
| Ursprünglicher Chattitel | Nicht verfügbar; der Dokumenttitel ist eine nachträgliche Sachbezeichnung. |
| Ursprünglicher Chatlink | Nicht verfügbar; kein Chatlink konstruiert. |
| Historische Fachabstimmung | Der gezielte Kontextabruf zu genau diesem Gespräch liefert Nachrichtenzeitstempel vom **18.07.2026**. Die frühere BPI-Antwort nennt ebenfalls diesen Stand. Das ist keine vollständige Exportmetadatenprüfung. |
| Erstellungsdatum dieser Zusammenfassung | **2026-10-03**, Zeitzonenbezug Europe/Berlin; Laufzeitdatum zusätzlich als 2026-10-03 UTC geprüft. |
| Ursprünglich betrachtete Repo-Adresse | `https://github.com/JanHG98/flowstation` |
| Aktuelles Zielrepository | `JanHG98/netcore-tetra` |
| Repository-ID | `1281497427`; sowohl der historische Repo-Abruf als auch die heutige Auflösung beider Namen liefern diese ID. |
| Historischer Codebezug | `main` bei `6143ed50ae23decb37eff2e3c04237b78a9ffd0b`, Merge von PR #4 „Control room“. |
| Heute geprüfter Zielbranch | `Archiving` |
| Eingangsstand der heutigen Codeprüfung | `465ab5350e804d00343ea23713aff601a22cba1e` |
| Tree des Eingangsstands | `cb11ca957f5beed5d7a288d0c5ca3bf8c8c68274` |
| Zusätzlich erfasster Default-Branch | `main` bei `6aa9be8f74ab731f72dc133a5f8e90c5018c626d` |
| Ablage dieser Datei | `Docs/archive/2026-10-03_werma-racksignalisierung-bpi-r4-pro-und-rack-architektur.md` |
| Zugehöriger Index | `Docs/archive/README.md` |
| Änderungsumfang dieses Auftrags | Nur diese Abschlussdokumentation und der Archivindex. Keine Implementierung, keine Konfigurationsänderung an Geräten, kein Branch-Merge. |

Der spätere Archivcommit ist über die Git-Historie dieser Datei und die Abschlussmeldung des Archivlaufs nachvollziehbar. Der **geprüfte Codecommit** oben bleibt davon getrennt: Ein neuer Dokumentationscommit bedeutet keinen neuen Implementierungsstand.

### 1.1 Quellenbasis und Auswertungslücken

Ausgewertet wurde der gesamte hier sichtbare fachliche Gesprächsverlauf: die ursprüngliche Repo-/WERMA-/Rack-Frage, die Antwort mit Architekturvorschlägen und Codebefunden, die Nachfrage zum BPI-R4 Pro sowie die letzte Ergänzung zum unmanaged 5-Port-Switch. Auch die damaligen sichtbaren GitHub-Leseergebnisse wurden berücksichtigt.

Der gezielte zusätzliche Kontextabruf diente der Suche nach **Metadaten genau dieses Chats**. Er lieferte keinen belastbaren Originaltitel oder Chatlink. Ähnliche WERMA-, GPIO- oder Rack-Gespräche wurden nicht als Inhalt dieses Chats übernommen. Insbesondere wird aus einem anderen Gespräch kein angeblich hier beschlossener Kernel-Boot-Hook, MQTT-Treiber oder fertiges Codex-Briefing abgeleitet.

Alle 25 bereitgestellten PDF-Dateien waren in der Arbeitsumgebung vorhanden und konnten für eine Bestandsprüfung geöffnet werden. Ihre Deckblätter und Seitenzahlen wurden geprüft; die Anhänge wurden **nicht vollständig normativ durchgearbeitet**. Sie sind TETRA-Referenzmaterial und enthalten im hier geprüften Umfang keine gerätespezifische Freigabe für die WERMA-Verdrahtung oder den BPI-Router. Das vollständige Anhangsverzeichnis steht in Abschnitt 15.

Nicht verfügbar sind insbesondere: WERMA-Typenschilder/Artikelnummern und Anschlusspläne, das konkrete Relais-/Ausgangsmodul, das tatsächlich verwendete Pi-Modell, ein BPI-Kauf-/Inventarnachweis, Rackmaße, Leistungsaufnahme, echte Gerätekonfigurationen, ein vollständiger Chat-Export und Mess-/Abnahmeprotokolle. Für dieses Rack wurden keine laufenden Dienste oder Geräte direkt erreicht.

### 1.2 Verwendete Statusbegriffe

| Status | Bedeutung in diesem Dokument |
|---|---|
| **Idee** | Im Gespräch vorgeschlagen oder als spätere Erweiterung genannt; keine verbindliche Umsetzung daraus ableitbar. |
| **Beschlossen/geplant** | Ausdrücklicher Nutzerwunsch oder erkennbar aufgegriffene Planungsrichtung. Eine Empfehlung der Assistenz allein ist kein Nutzerbeschluss. |
| **Implementiert** | An einer angegebenen Repository-Version als Code, Konfiguration oder Dienst nachweisbar; noch kein Laufzeitnachweis. |
| **Getestet** | Ein konkreter Test wurde nachweislich ausgeführt und sein Ergebnis dokumentiert. Quelltextlesen zählt nicht als Funktionstest. |
| **Im Betrieb bestätigt** | Erfolgreicher Betrieb wurde durch belastbare Laufzeitdaten oder ausdrückliche konkrete Rückmeldung belegt. |

**Für die physische WERMA-Ansteuerung, den BPI-Router und das neue Rack wurde in diesem Chat weder „getestet“ noch „im Betrieb bestätigt“ erreicht.**

## 2. Ziel, Ausgangslage und Themen

Jan wollte wissen, ob sich vorhandene **WERMA-Lampen in Rot, Grün, Gelb, Weiß und Blau** am Rack über Relais ansteuern lassen und ob die Funktion direkt in Flowstation oder über eine API zu einem zweiten Pi mit Relaiskarte sinnvoller wäre. Das Wort „Geld“ in der ersten Nachricht wurde im Gespräch als Tippfehler für „Gelb“ verstanden, nicht als zusätzliche Farbe oder Funktion.

Parallel wurde folgender erster Rack-Entwurf eingebracht:

| Nutzerseitiger Ausgangsentwurf | Platzansatz |
|---|---:|
| Router beziehungsweise Proxmox mit Router-VM | 1 HE |
| Switch | 1 HE |
| Basisstation | 2 HE |
| PDU | Noch nicht festgelegt |
| TrueNAS | 1 oder 2 HE |
| Wi-Fi-Access-Point | Noch nicht festgelegt |

Die weitere Diskussion behandelte fünf zusammenhängende Fragen:

1. Welche vorhandenen NetCore-/Flowstation-Zustände können die Lampen ansteuern?
2. Wo sollen Zustandslogik und Hardwaretreiber laufen, ohne den Funkbetrieb unnötig zu koppeln?
3. Welche elektrische Ausgangsstufe, Versorgung und Fehlerbehandlung sind erforderlich?
4. Wie wird aus den Einzelkomponenten ein wartbares Rack mit Strom-, Netzwerk- und RF-Konzept?
5. Kann ein eigenständiger BPI-R4 Pro Router und zunächst auch einen separaten Switch ersetzen?

Es handelte sich um Architektur- und Machbarkeitsplanung. Weder eine WERMA-Implementierung noch ein OpenWrt-Installationsauftrag wurde damals erteilt oder ausgeführt.

## 3. Verlauf, Planungsrichtung und Verbindlichkeit

### 3.1 Chronologie

| Schritt | Inhalt | Einordnung |
|---|---|---|
| 1 | Jan nennt die fünf vorhandenen WERMA-Farben und fragt nach Relaisansteuerung direkt in Flowstation oder über einen zweiten Pi. | Nutzerseitiges Ziel; genaue Hardware offen. |
| 2 | Jan skizziert Router/Proxmox, Switch, Basisstation, PDU, TrueNAS und AP im Rack. | Nutzerseitiger Ausgangsentwurf. |
| 3 | Das Repository wird gelesen. Control-Room-APIs, Telemetrieereignisse und der Fanout werden als geeignete Integrationsstellen identifiziert. | Historisch belegter Quelltextbefund. |
| 4 | Empfohlen wird zentrale Zustandsauswertung plus separater `netcore-rack-agent` auf einem zweiten Pi. | Architekturvorschlag, nicht implementiert. |
| 5 | Ein Farb-/Prioritätsmodell, neue Indicator-APIs, Lampentest, Watchdog und Erweiterungen zur Racküberwachung werden vorgeschlagen. | Ideen beziehungsweise Entwurf. |
| 6 | Für das Rack werden USV, Servicepanel, Kühlung, RF-/Potentialausgleich, WAN-Fallback und Reserve ergänzt. | Empfehlungen; keine Bestellung oder Montage bestätigt. |
| 7 | Jan fragt nach einem **Banana Pi BPI-R4 Pro als eigenständigem Router/Switch**. | Spätere Planungsrichtung anstelle der zunächst erwogenen Router-VM. |
| 8 | Die Assistenz empfiehlt den 8X mit OpenWrt, gegebenenfalls AP-Injektor und späterem größeren Switch. | Empfehlung; die Auswahl 8X und ein bestimmtes Image sind nicht ausdrücklich von Jan final freigegeben. |
| 9 | Jan ergänzt: Bei Bedarf kommt ein kleiner unmanaged **5-Port-Switch** hinzu. | Ausdrückliche Bedarfsoption des Nutzers. |
| 10 | Dafür wird ein einzelnes Rack-Management-Segment vorgeschlagen. | Konkreter Anschlussvorschlag; VLAN-ID und Portkonfiguration bleiben offen. |

### 3.2 Was als Ergebnis erhalten bleibt

**Nutzerseitig festgehalten/geplant:** fünf vorhandene WERMA-Farben am Rack nutzbar machen; Rack mit Basisstation, Netzwerk, Storage und AP; eigenständigen BPI-R4 Pro als Router-/Switch-Option prüfen; einen kleinen unmanaged 5-Port-Switch bei Portmangel akzeptieren.

**Empfohlene, noch zu bestätigende Ausgestaltung:** separater I/O-Pi statt GPIO im Funkprozess; zentrale Indicator-Logik; OpenWrt auf dem BPI; 8X statt 4E; Transistorausgänge für häufige Lichtwechsel; 2-HE-TrueNAS; USV; externer AP; 12–15 HE mit Reserve; das Management-Segment für den Kleinswitch.

**Nicht beschlossen:** konkreter Hersteller/Typ der Ausgangskarte, Betriebsspannung der vorhandenen Lampen, GPIO-Pins, finaler Farbcode, Blinkfrequenzen, eine bestimmte Firmwareversion, VLAN-IDs/IP-Plan, USV-Leistung und Laufzeit, Datenträgerlayout, Racktiefe und endgültige Höheneinheiten.

Der spätere BPI-Ansatz ersetzt die Router-VM **als bevorzugte Diskussionsrichtung**, nicht als dokumentierten Umbau. Die Router-VM bleibt eine besprochene Alternative. Ein separater großer Switch wurde nicht grundsätzlich verboten; er wurde für die erste kleine Ausbaustufe als verzichtbar betrachtet.

## 4. Historische Softwarearchitektur der WERMA-Ansteuerung

### 4.1 Empfohlener Datenweg

```text
Flowstation-Basisstation(en)
        |
        | Telemetrie / Zustandsereignisse
        v
NetCore Control Room
  - Zustandsauswertung
  - Prioritätsregeln
  - begründeter Indicator-Zustand
        |\        WebSocket, optional HTTP-Snapshot
        v
Separater NetCore Rack Agent auf Pi
  - Empfang und Validierung
  - Zeitüberwachung / Fallback
  - lokale Lichtmuster
  - Hardwaretreiber
        |
        v
Passende Relais- oder Transistorausgänge
        |
        v
WERMA Rot / Gelb / Grün / Weiß / Blau
```

Im historischen Vorschlag sollte die fachliche Bedeutung der Lampe zentral berechnet werden. Der I/O-Pi sollte nicht unabhängig aus vielen Rohwerten eine zweite, womöglich widersprüchliche Prioritätslogik entwickeln. Er sollte die gewünschte Anzeige umsetzen und bei Verbindungsverlust einen eigenen definierten Ersatzstatus herstellen.

Die Trennung wurde damit begründet, dass GPIO-Zugriffe, Lampenumbauten, Netzwerk-Reconnects und Fehler der Ausgangskarte nicht unnötig an den Prozess `bluestation-bs` gekoppelt werden sollen. Der zweite Pi könnte später auch USV, NAS, Switch und Umgebung überwachen. **Eine echte ausfallsichere Anlage oder garantierte Echtzeitreaktion wurde damit nicht nachgewiesen.**

### 4.2 Alternative: Direktintegration im Basisstationsprozess

Der vorhandene Thread `telemetry-fanout` wurde als Integrationsstelle identifiziert. Dort werden Telemetrieereignisse bereits an Dashboard, Telegram, Snom, Netztelemetrie und Control Room verteilt. Der historische illustrative Zusatz lautete:

```rust
// Historisches Prinzipbeispiel; nicht in diesem Chat implementiert.
if let Some(tower) = &signal_tower {
    tower.send_event(event.clone());
}
```

Dazu wurden ein neuer Konfigurationstyp `CfgSignalTower`, eine Erweiterung von `StackConfig` und ein optionaler Worker vorgeschlagen. Der Codebezug ist in [R01] und [R02] nachvollziehbar.

**Vorbehalte:** zusätzliche Hardwareabhängigkeit des Basisstations-Binaries, GPIO-Zugriffsrechte im Dienst, Fehlerkopplung und erschwerte Wiederverwendung zur Überwachung des restlichen Racks. Die zusätzliche Variante „separater Prozess auf demselben Basisstations-Pi“ wurde als besser getrennt als In-Process-GPIO, aber weniger unabhängig als ein zweiter Rechner beschrieben.

**Präzisierung des Archiv-Reviews:** Ein konfigurierbarer Pin- oder Farbwechsel erfordert nicht zwangsläufig einen Neubau des Funk-Binaries. Das frühere Argument „jeder Lampenumbau benötigt einen Neubau“ war zu pauschal. Ein Neubau ist bei Änderungen am eingebauten Treiber oder an der implementierten Logik nötig, nicht automatisch bei jeder Konfigurationsänderung. Ein zweiter Thread allein garantiert ebenfalls keine Isolation: blockierende I/O im Fanout kann andere Empfänger verzögern.

### 4.3 Vorgeschlagene Dateien und Dienste — nicht als vorhanden behandeln

| Bezeichnung | Historischer Zweck | Nachweisstand |
|---|---|---|
| `bins/netcore-control-room/src/indicator.rs` | Fachliche Zustands-/Prioritätslogik | Vorgeschlagener neuer Modulpfad; keine fertige Implementierung nachgewiesen. |
| `bins/netcore-rack-agent/` | Eigenständiges Binary für den I/O-Pi | Vorgeschlagener Pfad; kein entsprechender Workspace-Member im geprüften Stand. |
| `netcore-rack-agent.service` | systemd-Dienst für den I/O-Pi | Vorgeschlagener Dienstname; kein Deployment belegt. |
| `IndicatorState` | Transportierbarer Lampenzustand | Entwurf. |
| `UiMessage::IndicatorState` | Neue WebSocket-Nachrichtenvariante | Entwurf; im heute geprüften `UiMessage`-Enum nicht enthalten. |
| `[signal_tower]` / `CfgSignalTower` | Optionale Direktansteuerung | Historischer Alternativentwurf, keine freigegebene aktuelle Konfiguration. |

### 4.4 Historischer API-Entwurf

```text
GET  /api/indicator/state
POST /api/indicator/test
POST /api/indicator/reset
POST /api/indicator/ack
```

Diese Routen waren **Vorschläge**, keine damals oder heute durch diesen Chat bereitgestellten Endpunkte. Der Stand der heutigen Control-Room-Routingfunktion ist separat in Abschnitt 9 beschrieben.

Der Rack-Agent sollte für normale Statusabfragen nur lesende Rechte erhalten. Lampentest und andere Schreibaktionen sollten Operator/Admin vorbehalten bleiben. Eine maschinelle Authentifizierung für genau diesen Agenten wurde nicht spezifiziert oder getestet.

**Offene Semantik:** `ack` darf nicht stillschweigend einen TETRA-Notruf löschen; `reset` kann Lampenoverride, lokale Quittierung, Treiberreset oder Gerätestart meinen. Diese Aktionen müssen vor Umsetzung getrennt definiert werden. Ein vorhandener Funkbefehl `clear-emergency` ist kein Ersatz für eine reine Quittierung der Signallampe.

### 4.5 Historisches Zustandsbeispiel

Das folgende JSON bewahrt den damals vorgeschlagenen Entwurf. Es ist kein gemessener Zustand und kein bestätigter API-Vertrag:

```json
{
  "revision": 1842,
  "mode": "emergency",
  "outputs": {
    "red": "flash",
    "yellow": "off",
    "green": "off",
    "white": "off",
    "blue": "steady"
  },
  "reasons": ["active_emergency", "active_group_call"],
  "valid_for_ms": 5000
}
```

`revision = 1842` und die Gültigkeit von 5000 ms waren Beispielwerte. Weder diese TTL noch eine dazu passende Heartbeat-Frequenz wurden vereinbart.

Die Erweiterung des vorhandenen UI-WebSockets wurde beispielsweise so beschrieben:

```rust
// Historische Zielvorstellung; kein aktueller Enum-Nachweis.
UiMessage::IndicatorState { state: IndicatorState }
```

### 4.6 Historische GPIO-Beispielkonfiguration

```toml
# NICHT ungeprüft in eine laufende Flowstation-Konfiguration übernehmen.
# Historischer Entwurf für die nicht implementierte Direkt-GPIO-Alternative.
[signal_tower]
enabled = true
backend = "gpio"
red_pin = 17
yellow_pin = 27
green_pin = 22
white_pin = 23
blue_pin = 24
active_low = true
```

Diese Nummern sind **keine geprüfte Pinbelegung**. Die verwendete Nummerierung wurde im Gespräch nicht ausdrücklich festgelegt. Raspberry-Pi-BCM-Nummern, physische Headerpositionen und Linux-GPIO-Line-Offsets dürfen nicht verwechselt werden. Vor einer Umsetzung müssen das konkrete Board, belegte SDR-/HAT-Pins und das Verhalten der Ausgangskarte geprüft werden. Die Werte gelten insbesondere nicht automatisch für den Banana Pi.

## 5. Farb-, Prioritäts- und Fehlermodell

### 5.1 Historischer Vorschlag für die fünf Farben

| Anzeige | Vorgeschlagene Bedeutung |
|---|---|
| Grün dauerhaft | Basisstation verbunden, Health OK, SDR OK |
| Gelb dauerhaft | Eingeschränkter Zustand: beispielsweise BREW-Verbindung ausgefallen, erhöhte Temperatur oder einzelner Dienst gestört |
| Gelb blinkend | Verbindung zum Control Room oder Datenversorgung des Rack-Agenten verloren |
| Rot dauerhaft | Kritischer Systemfehler, SDR ausgefallen oder Basisstation offline |
| Rot blinkend | Aktiver TETRA-Notruf |
| Blau dauerhaft | Aktiver Gruppen- oder Einzelruf |
| Blau kurz pulsierend | Funk-/Timeslot-Aktivität; im Vorschlag bevorzugt mit Halbleiterausgängen |
| Weiß dauerhaft | Wartungsmodus oder lokaler manueller Betrieb |
| Weiß blinkend | Boot, Update oder Lampentest |

Die Farben und Muster sind eine **projektspezifische Bedienidee**, keine aus den angehängten ETSI-Dokumenten abgeleitete Vorschrift und kein endgültig freigegebenes Betriebskonzept.

Historische Prioritätsidee:

```text
Notruf > kritischer Fehler > degraded > normal
```

Blau war als zusätzliche Aktivitätsanzeige vorgesehen. Grün sollte bei Gelb oder Rot unterdrückt werden, damit „alles gut“ nicht neben einer Störungsanzeige stehen bleibt. Weiß und Blau müssen trotzdem ausdrücklich in die spätere Mehrfachfehler-/Override-Logik aufgenommen werden.

### 5.2 Beim Archiv-Review ergänzte offene Designpunkte

Die folgenden Punkte sind **heutige technische Prüfhinweise**, keine nachträglich erfundenen Beschlüsse des Fachchats:

- **Geltungsbereich:** Ein Rack mit einer lokalen TBS darf nicht ohne Auswahlfilter den Alarm irgendeiner fremden Node des gesamten Netzes anzeigen. `rack_id`, Standort-/Node-Zuordnung und gewünschter netzweiter Notrufumfang sind festzulegen.
- **Datenfrische:** Eine neue HTTP-Antwort oder ein aktuelles Wrapperfeld `now` beweist nicht, dass enthaltene Node-/RF-Daten frisch sind. Alter der tatsächlichen Beobachtung, Verbindung und Sensorwerte getrennt auswerten. Fehlende Werte sind nicht automatisch „OK“.
- **Gespräch versus Träger:** Ein aktiver Call, ein aktuell belegtes Sprechrecht und tatsächliche Sprachframes sind unterschiedliche Sachverhalte. Für die blaue Lampe muss eine Bedeutung gewählt werden; nicht einfach beliebige TX-Energie als „Gespräch“ behandeln.
- **Stille Anlage:** Ohne angemeldete Funkgeräte kann eine Basisstation trotzdem betriebsbereit sein. Ein bloßer Teilnehmerzähler von null darf nicht automatisch einen Anlagenfehler erzeugen.
- **Historische Fehlerlisten:** Ein Eintrag in `errors` kann Historie sein. Daraus ohne Lebenszyklus-/Aktivstatus eine dauerhafte rote Anzeige abzuleiten, wäre fehleranfällig.
- **Verbindungsverlust:** „TBS offline“, „Control Room nicht erreichbar“, „WAN weg“ und „NAS weg“ sind verschiedene Fehler. Ein lokaler Funkbetrieb kann trotz WAN- oder Backendstörung weiter möglich sein.
- **Lease/Heartbeat:** Ein nur bei Änderungen gesendeter Zustand mit 5-s-TTL verfällt auch bei unverändert gesunder Anlage. Es braucht periodische Verlängerung oder einen separat definierten Heartbeat. WebSocket-Ping/Pong allein bestätigt lediglich den Transport, nicht die Aktualität der Messwerte.
- **Wiederanlauf:** Nach Agent-/Backendstart wird zunächst ein vollständiger aktueller Snapshot benötigt. Ein altes gespeichertes Grün oder eine retained MQTT-Nachricht darf ohne Altersprüfung nicht als neuer Status gelten.
- **Mehrfachfehler:** Notruf und kritische Hardwarestörung können gleichzeitig vorliegen. Die Lampe soll wichtige Hinweise nicht unwiederbringlich verdecken; die vollständigen Ursachen gehören ins UI/Eventlog.
- **Quittierung:** Lokale Quittierung, Ende eines Notrufs, Rückkehr in den Normalzustand und manueller Test müssen getrennte Aktionen bleiben. Mindestanzeigedauer und gegebenenfalls eine gespeicherte Alarmanzeige sind noch festzulegen.
- **Entkopplung:** Ausgangsfehler, langsame Netzwerkaufrufe und volle Warteschlangen dürfen den Funk-Fanout nicht blockieren. Ein begrenzter Zustandskanal mit dokumentiertem Überlaufverhalten ist als Implementierungsoption zu prüfen.

## 6. Elektrik, Ausgangsstufe und Rack-Agent

### 6.1 Ungeklärte elektrische Grundlage

Im Fachchat wurde mit einer **24-V-WERMA-Säule** geplant. Tatsächlich genannt hatte Jan jedoch nur Hersteller und Farben. Artikelnummer, Lampentechnologie, AC/DC-Ausführung, Nennstrom, Einschaltstrom und gemeinsamer Anschluss sind unbekannt.

Deshalb ist „24 V“ bis zur Prüfung der vorhandenen Elemente eine **Planungsannahme**. Die damals erwähnten WERMA-Serien KombiSIGN 71/72 und Beispielströme dürfen nicht als Identifikation von Jans Lampen behandelt werden. Auch ein vollständiger vorhandener Säulenfuß beziehungsweise dessen Anschlussklemmen ist nicht belegt.

Der GPIO liefert nur das Steuersignal. Raspberry Pi dokumentiert 3,3-V-GPIO; eine 24-V-Lampe oder Relaiswicklung gehört nicht direkt daran. Die konkrete Treiber-/Eingangskompatibilität bleibt zu prüfen. [E04]

### 6.2 Historischer Versorgungsentwurf

```text
Rack-Stromverteilung / gegebenenfalls USV
        |
        +--> Pi-Versorgung
        |
        +--> passendes Netzteil für die Signalgeräte
                 |
                 +--> abgesicherte Ausgangskanäle
                           |
Pi --> Treiber-/I/O-Modul --+--> WERMA-Elemente
```

Vorgeschlagen waren ein DIN-Netzteil, eine Klemmenleiste, Absicherung je Kanal und räumliche Trennung von Leistungs-/Schaltleitungen und empfindlicher RF-Verkabelung.

Ein **24-V-/2-A-Netzteil** wurde als voraussichtlich großzügige LED-Planungsgröße genannt. Das ist kein Dimensionierungsnachweis. Die ebenfalls erwähnten Größenordnungen von etwa 30 mA beziehungsweise 130 mA bezogen sich auf unterschiedliche Beispielprodukte, nicht auf die vorhandenen Lampen. Alle fünf Kanäle gleichzeitig, etwaige interne Blitzmodule, Einschaltstrom, zusätzliche Sensorik und Temperaturreserve sind vor Auswahl zu berücksichtigen.

Für dieses Archiv wurde keine Netzspannungsverdrahtung erstellt oder freigegeben. Der tatsächliche Aufbau benötigt die passenden Herstellerunterlagen und eine fachgerechte mechanische, elektrische und Schutzleiter-/Potentialausgleichsausführung.

### 6.3 Relais gegenüber Halbleiterausgängen

| Option | Im Chat genannter Vorteil | Einschränkung / offener Nachweis |
|---|---|---|
| Relaiskarte | Für seltenes Ein/Aus anschaulich; potentialfreie Kontakte können verschiedene Lastanschaltungen bedienen. | Passende Kontaktbelastbarkeit für die reale Last und den Einschaltstrom, Steuereingänge, Spulenversorgung und Entstörung prüfen. Kontakt und Pi sind nicht allein aufgrund des Wortes „Relaiskarte“ überall sicher getrennt. |
| Transistor-/MOSFET-Ausgänge | Geräuschlos, keine mechanischen Schaltkontakte, besser für häufige Lichtwechsel. | Nur passend zur DC-/AC-Last, gemeinsamen Anschlussführung und erforderlichen High-/Low-Side-Schaltung auswählen. |
| Optoisoliertes 8-Kanal-Modul | Fünf Farben plus drei Reservekanäle; Trennung der Steuersignale als Ziel. | Noch kein konkretes Produkt ausgewählt. Optokoppler allein garantieren bei verbundenen Massen/Jumpern keine vollständige galvanische Trennung des Systems. |

Die Assistenz bevorzugte ein 8-Kanal-24-V-Transistorausgangsmodul für häufigere Blink-/Aktivitätsmuster. Jan hatte zunächst Relais vorgeschlagen; ein endgültiger Wechsel wurde nicht ausdrücklich beschlossen.

**Präzisierung:** Relais können technisch auch blinken. Die frühere Formulierung „nicht geeignet“ ist als Empfehlung gegen häufige Schaltspiele und sehr kurze Aktivitätsimpulse zu verstehen, nicht als absolutes Funktionsverbot. Eine AC-Triac-SSR ist wiederum nicht automatisch ein geeigneter Schalter für DC-Leuchten.

### 6.4 Geplante Agent-Funktionen und Hardwaredetails

Als Funktionen des eigenen Dienstes wurden genannt: WebSocket-Verbindung, Empfangen des berechneten Zustands, fünf Ausgänge schalten, lokales Blinken, Lampentest beim Start, Watchdog, definierter Verbindungsverluststatus, optionales lokales Webinterface und spätere Racküberwachung.

Als physische Ergänzungen wurden genannt: Lampentest-Taster, Resetmöglichkeit für den I/O-Pi, Sicherungen beziehungsweise elektronische Absicherungen, Klemmen, definierte Initialzustände und gefilterte/sauber geführte Leitungen.

**Wichtige Korrektur zur historischen Empfehlung „Pull-downs“:** Gleichzeitig wurde im Beispiel `active_low = true` vorgeschlagen. Ein Pull-down kann bei einem aktiv-low-Eingang gerade **einschalten**. Der sichere Initialpegel ist aus der realen Schaltung abzuleiten; häufig wäre für aktiv-low ein geeigneter Pull-up zur zulässigen Logikspannung erforderlich. Ohne Schaltplan ist keine konkrete Widerstands-/Pinvorgabe freigegeben. Boot, Neustart, Treiberfreigabe, Prozessabsturz und Stromausfall benötigen getrennte Tests.

Ein Software-Watchdog ersetzt außerdem keinen unabhängigen Hardware-Watchdog. Ein unversorgter Pi kann seine eigene ausgefallene Lampe nicht mehr aktiv umschalten. Soll ein Gesamtausfall sichtbar bleiben, sind Versorgungspfad, Ruhestrom-/Heartbeat-Konzept und gegebenenfalls unabhängige Ausfallanzeige gesondert zu entwickeln. Der Begriff „ausfallsicherer“ aus dem Gespräch beschreibt eine relative Entkopplung, keine nachgewiesene Sicherheitsfunktion.

## 7. Rack-Architektur und Erweiterungswünsche

### 7.1 Zwei historische Layoutvarianten

Die erste Assistenzempfehlung addierte USV, Servicefeld und Reserve zum Nutzerentwurf. **12 HE wurden als Unterkante und 15 HE als komfortablere Größe empfohlen.** Das war eine Platzabschätzung ohne konkrete Gehäuse-/Einbaumaße.

| HE, unten nach oben | Früher Vorschlag mit separatem Switch |
|---|---|
| U01–U02 | USV |
| U03–U04 | TrueNAS |
| U05 | Proxmox-/Router-Server |
| U06 | Managed PoE-Switch |
| U07 | Patchpanel / externe Anschlüsse |
| U08–U09 | Flowstation-Basisstation |
| U10 | 24-V-/I/O-/Service- oder Lüfterfeld |
| U11–U15 | Reserve / Ausbau |

Nach der BPI-Nachfrage entstand folgender kompakterer Vorschlag:

| HE, unten nach oben | Späterer Vorschlag mit eigenständigem BPI-Router |
|---|---|
| U01–U02 | USV |
| U03–U04 | TrueNAS |
| U05 | Proxmox-Server für Dienste, nicht zwingend Router |
| U06–U07 | Flowstation-Basisstation |
| U08 | BPI-R4-Pro-Servicepanel / Einbaugehäuse |
| U09 | Patchpanel |
| U10 | WERMA-/24-V-/Rack-Agent-Technik |
| U11–U12 | Reserve |

Eine vertikale rückseitige PDU und ein außen beziehungsweise oben am Rack montierter AP waren vorgeschlagen. Die WERMA-Signale sollen sichtbar am Rack sitzen; die angegebene HE für WERMA/I/O meint den möglichen Elektronikbereich, nicht eine bereits festgelegte Säulenmontage.

**Nicht entschieden:** 12 oder 15 HE, Gehäusetiefe, Einbauzeichnungen, Schienen, Kabelbiegeradien, Gewicht, Transportanforderungen, Lüfterhöhe und maximale Innentemperatur. Das frühe „eine HE reicht“ für das BPI-Panel ist ohne Kühlkörper, Erweiterungen und Leitungsführung keine zugesicherte mechanische Passung.

### 7.2 Ergänzungen aus dem Fachchat

**USV und Strommanagement:** Eine PDU verteilt Strom, eine USV soll Unterbrechungen überbrücken und geordnetes Herunterfahren ermöglichen. Vorgeschlagen wurde die Einbindung über USB oder Netzwerk und NUT. Anzeigen für Netzbetrieb, Batteriebetrieb, niedrigen Akkustand und Shutdown sollten später in den Rackstatus einfließen. Weder ein USV-Modell noch Leistung, Laufzeit oder Abschaltreihenfolge wurden bestimmt.

**Patch-/Servicepanel:** WAN, LAN, Management, Glasfaser, TX-/RX-Antenne, GPS/GNSS, USB-Service, HDMI/Console, 24-V-Service und Erdungsanschluss wurden als herauszuführende Anschlüsse genannt. Es existieren aus diesem Chat keine verbindlichen Steckerbelegungen, Stückliste oder CAD-Dateien.

**WAN-Fallback:** Ethernet als primärer WAN-Pfad; LTE/5G als möglicher zweiter Pfad; WLAN-Client als weitere Option. Antennen sollen außerhalb ungünstiger Metallabschirmung platziert werden. Modem, SIM, Tarif, Antennen und Failoverregeln sind offen. WireGuard wurde als Routerfunktion vorgeschlagen, aber keine VPN-Migration beschlossen.

**Umgebung:** Temperatur am Einlass vorne/unten und an der Abluft, Luftfeuchtigkeit, Türkontakt, Lüfterüberwachung und optional ein Rauch-/Brandkontakt wurden genannt. Diese Sensorik ist nicht als installierte Brandmeldeanlage zu verstehen.

**RF-/EMV-Konzept:** Trennung von Antennen-/RX-Leitungen und I/O-/Leistungsverkabelung, passende Filter/Ferrite, Potentialausgleich des Racks und ein standortgerechtes Überspannungs-/Antennenschutzkonzept wurden angeregt. Es liegt keine Messung vor, die Störfreiheit der Relais-/DC-Wandler-/Netzwerktechnik neben dem SDR bestätigt.

**TrueNAS:** 2 HE wurden wegen Platz für Laufwerke, Kühlung und Wartung gegenüber 1 HE bevorzugt. Die Aussagen zu Lautstärke und Luftdurchsatz sind Planungsargumente, keine Eigenschaften jedes beliebigen 2-HE-Gehäuses. Storagepool, Laufwerkstyp, Redundanz und Backupziel sind offen. Vorgeschlagen wurde lokales Puffern von Aufzeichnungen/Logs mit späterer Übertragung; der Funkbetrieb soll nicht hart vom NAS abhängen. Eine solche Puffer-/Uploadlogik wurde in diesem Chat nicht implementiert oder getestet.

### 7.3 Router-VM als frühere Alternative

Für eine Router-VM auf Proxmox wurden dedizierte physische Ports beziehungsweise eine saubere VLAN-/Passthrough-Zuordnung, unabhängiger Managementzugang, Router-Autostart vor abhängigen Diensten sowie ein Notzugang/Fallback vorgeschlagen.

Der zentrale Nachteil der Alternative war die gemeinsame Ausfall-/Wartungsdomäne: Ein Hostneustart unterbricht auch den darauf virtualisierten Router. Der später diskutierte separate BPI trennt diese Wartungsdomänen. Er schafft damit aber noch keine Routerredundanz; sein eigener Neustart und seine eigene Versorgung bleiben relevant.

## 8. BPI-R4 Pro und unmanaged 5-Port-Erweiterung

### 8.1 Historisches Zielbild

Der BPI sollte Routerfunktionen und für die kleine erste Ausbaustufe auch die nötige LAN-Verteilung übernehmen. Proxmox bliebe für Control Room, Asterisk, Monitoring und weitere VMs/Container zuständig.

Die vorgeschlagene Routerfunktionalität umfasste OpenWrt, Firewall/NAT, VLANs, DHCP/DNS, WireGuard, Multi-WAN sowie LTE/5G-Fallback. Diese Funktionsliste ist ein **Zielumfang**, nicht der Nachweis eines eingerichteten Routerimages. OPNsense wurde für dieses BPI-Konzept nicht vorgeschlagen; der zuvor genannte OPNsense-Ansatz gehörte zur x86-/VM-Variante.

### 8.2 Hardwareaussagen und heutiger Nachweis

Der historische Vorschlag bevorzugte **BPI-R4 Pro 8X** gegenüber 4E. Die heute abgerufenen Herstellerangaben und OpenWrt-PR #21083 stützen für den 8X folgenden Portaufbau: vier 2,5-Gbit/s-RJ45-LAN-Ports, ein 1-Gbit/s-RJ45-LAN-Port und zwei 10-Gbit/s-RJ45/SFP+-Kombinationsschnittstellen. Der PR nennt neun äußere Buchsen, von denen sieben gleichzeitig nutzbar sind. Je Combo-Paar darf RJ45/SFP+ nicht als zwei unabhängig gleichzeitig verfügbare Ports gezählt werden. [E01][E02]

Bei Nutzung eines Combo-Interfaces als WAN ergibt sich als reine Portzählung: vier 2,5G-LAN, ein 1G-LAN und ein weiteres 10G-Combo-LAN. Das ist **keine Zusage**, dass jede beliebige VLAN-/Routing-/Firewall-/VPN-Kombination mit voller Portgeschwindigkeit läuft. Portgeschwindigkeit, internes Switching und CPU-/Offload-Pfade sind getrennt zu testen.

Die Herstellerseite nennt zusätzlich einen 1G-LAN-FPC-Anschluss. Dieser wurde im historischen Außenportplan nicht berücksichtigt und wird hier nicht als sofort nutzbare zusätzliche RJ45-Buchse verplant. [E01]

Im früheren Chat wurde die automatische Umschaltung der Combo-Ports beim 8X gegenüber teilweise U-Boot-abhängiger Auswahl beim 4E als Argument genannt. **Diese genaue revisions- und imageabhängige Aussage wurde im Archivlauf nicht vollständig neu verifiziert** und bleibt vor einer Kaufentscheidung anhand der konkreten Hardware/Software zu prüfen.

### 8.3 OpenWrt: historische Warnung inzwischen teilweise überholt

Die historische Antwort lautete sinngemäß: Herstellerimages vorhanden, offizieller OpenWrt-Support für den R4 Pro 8X noch in Arbeit, PR #21083 offen — Stand 18.07.2026.

**Heutiger separater Befund:**

| Merkmal | Am 03.10.2026 per GitHub geprüft |
|---|---|
| Repository / PR | `openwrt/openwrt` #21083 |
| Titel | `mediatek: add support for BananaPi BPi-R4 Pro 8X` |
| Status | `closed`, `merged = true` |
| Mergezeit | **2026-08-24T12:01:16Z** |
| Von der API gemeldeter Mergecommit | `554b5dcc5199d07a84f4fba9f6a56066e5871858` |
| Zielbranch | `main` |
| Umfang laut PR-Metadaten | 2 Commits, 23 geänderte Dateien |

Die frühere Aussage „PR weiterhin offen“ darf heute nicht weitergeführt werden. Ein Merge in OpenWrts Hauptentwicklungszweig belegt jedoch **nicht automatisch**, dass ein bestimmtes Stable-Release, jedes Peripheriemodul oder das konkrete Rack bereits abgenommen ist. Eine konkrete stabile Release-/Imageauswahl wurde hier nicht überprüft. [E02]

Die beiden Banana-Pi-Dokumentationsseiten aus dem Fachchat waren beim heutigen Webabruf erreichbar, lieferten dem Textparser aber keinen verwertbaren Fließtext. Für bestätigte Portangaben wurden deshalb die Herstellerproduktseite und die GitHub-PR-Metadaten herangezogen. Details zu Herstellerimage, automatischer Portumschaltung und exakter Netzteilfreigabe bleiben als separate Verifikationsaufgaben gekennzeichnet. [E01][E03]

### 8.4 Strom, Kühlung und PoE

Historisch vorgeschlagen: eigenes 1-HE-Servicepanel, aktive Kühlung, saubere Netzteilversorgung, nach vorn geführte USB-C-Debug-Konsole, Reset/Status und gegebenenfalls LTE-/5G-Antennenanschlüsse.

Im Chat wurden 12 V/5 A beziehungsweise USB-PD mit 20 V und mindestens 65 W sowie etwa 10 W für das nackte Board erwähnt. **Diese Werte wurden nicht am Gerät gemessen und sind keine heute bestätigte vollständige Versorgungsspezifikation.** Der aktuelle PR bestätigt die grundsätzlichen Versorgungsoptionen USB-C-PD 20 V oder DC-Buchse, aber nicht die dortige gesamte Leistungsdimensionierung. Funkkarten, Modems, SSDs, SFP+-Module und Kühlung sind in der konkreten Auslegung einzubeziehen. [E02]

Die historische PoE-Aussage unterschied die mögliche Versorgung des Boards von PoE-Ausgängen für andere Geräte. **PoE-Out für AP oder Rack-Agent wird für dieses Konzept nicht als vorhanden vorausgesetzt.** Ein passender Injektor für einen externen AP wurde vorgeschlagen; bei mehreren PoE-Verbrauchern bleibt ein separater PoE-Switch die Ausbauoption. Ein nur am unmanaged Switch angeschlossener Pi erhält dadurch nicht automatisch PoE.

### 8.5 Letzte Nutzerergänzung: kleiner unmanaged 5-Port-Switch

Die ausdrückliche Ergänzung von Jan war: Bei Bedarf genügt zunächst ein kleiner „dummer“ 5-Port-Switch. Es wurde kein konkretes Modell gekauft oder getestet.

Der dazu vorgeschlagene Anschlussplan:

```text
BPI-R4-Pro-Port
  als Access-Port für ein einziges Rack-Management-VLAN
        |
        v
Unmanaged 5-Port-Switch
  Port 1: Uplink zum BPI
  Port 2: WERMA-/Rack-Agent-Pi
  Port 3: USV-Netzwerkkarte
  Port 4: PDU
  Port 5: Reserve oder Serviceanschluss
```

Damit bleiben nach dem Uplink **vier Endgeräteports**. Alternativ wurde ein NAS-Managementport als Teilnehmer genannt, sofern das tatsächliche NAS dafür einen passenden Anschluss besitzt. Es gibt keinen Anspruch, alle Beispielgeräte gleichzeitig plus Reserve an fünf Buchsen anschließen zu können.

**Präzisierung gegenüber der vereinfachten Fachchat-Antwort:** Ein unmanaged Switch erzwingt nicht selbst zuverlässig „genau ein ungetaggtes VLAN“. Manche Geräte leiten Tags transparent weiter. Die beabsichtigte Trennung muss der korrekt konfigurierte BPI-Port mit VLAN-Mitgliedschaft, PVID und Filtering gewährleisten; der Kleinswitch bietet keine unabhängige Portsegmentierung. Alle dort angeschlossenen Geräte sollen im selben vorgesehenen Segment bleiben. Eine Router-Firewall trennt nicht automatisch zwei Geräte, die direkt innerhalb dieses kleinen Layer-2-Segments miteinander kommunizieren. Die OpenWrt-Dokumentation trennt entsprechend Bridge-/VLAN-Konfiguration und geroutete Firewall-Zonen. [E05]

Im vorgeschlagenen Layout bleiben WAN, Proxmox-Trunks, AP-Verbindungen mit mehreren SSID-VLANs und gegebenenfalls der NAS-Datenpfad direkt am BPI beziehungsweise späteren Managed Switch. Das war eine Empfehlung für klare Trennung und Bandbreitenplanung, kein allgemeines technisches Verbot, einen Server oder eine Basisstation hinter einen unmanaged Switch zu hängen.

Noch offen: physischer Uplink-Port, tatsächliche Interfacebezeichnungen des gewählten OpenWrt-Images, VLAN-ID, IP-Netz, zulässige Kommunikationsbeziehungen, Portgeschwindigkeit, AP-/Pi-Stromversorgung und Loopvermeidung. Für eine spätere größere Ausbaustufe soll ein 10G-Uplink zu einem Managed-/PoE-Switch möglich bleiben.

## 9. Zusätzlich geprüfter Repository-Stand vom 03.10.2026

### 9.1 Namensauflösung und Branchabweichung

Die API-Abfragen für `JanHG98/flowstation` und `JanHG98/netcore-tetra` liefern heute denselben kanonischen Namen `JanHG98/netcore-tetra` und dieselbe Repository-ID `1281497427`. Der alte Link ist daher nicht als unabhängiges zweites aktuelles Projekt mit getrenntem Entwicklungsstand zu behandeln. Ein exaktes Umbenennungsdatum wurde nicht ermittelt.

Der historische PR #4 „Control room“ ist weiterhin nachvollziehbar: gemergt am **02.07.2026 um 13:27:32 UTC**, mit Mergecommit `6143ed50ae23decb37eff2e3c04237b78a9ffd0b` und damaligem Head `e09f84fb48ea21889d8a4afa82e2510afeef620b`. Die vorhandene Control-Room-Basis stammte also nicht aus einer hier implementierten Lampenfunktion. [R00]

Beim Vergleich von `main@6aa9be8...` mit `Archiving@465ab53...` war `Archiving` **13 Commits voraus und 0 zurück**. Die Unterschiede bestanden nicht nur aus Archivdateien, sondern enthielten auch Dashboard-/Dienst-WebUI-Änderungen. Deshalb werden hier bewusst **die gelesenen Archiving-Dateien** als maßgeblicher Prüfstand benannt. Dieses Archiv kopiert oder merged keine dieser vorgefundenen Änderungen nach `main`.

### 9.2 Control-Room-APIs und Telemetrie weiterhin vorhanden

An der heutigen Routingfunktion wurden die historischen lesenden Endpunkte erneut bestätigt:

```text
GET /api/overview
GET /api/state
GET /api/rf
GET /api/health/full
GET /api/subscribers
GET /api/groups
GET /api/calls
GET /api/sds
GET /api/emergencies
GET /api/events
GET /api/nodes
```

Die frühere Grundlage für ein Statusabonnement besteht somit weiter. Außerdem gibt es inzwischen unter anderem `/api/v1/status`, `/api/v1/control-room/overview`, `/api/v1/services`, `/health/live`, `/health/ready`, `/metrics` und OpenAPI-Routen. Der aktuelle v1-Status bezeichnet den Control Room mit `authoritative_state = false` und kombiniert Legacy-Zustand und Operations-Überblick. Ein Agent muss deshalb festlegen, welche Quelle für welchen Zustand maßgeblich ist. [R03]

**Bedeutung für eine grüne Lampe:** `/health` und `/health/live` melden im geprüften Handler einen lebenden Control-Room-Dienst, nicht die Funktionsfähigkeit jedes SDRs. `/health/ready` kann trotz HTTP 200 einen JSON-Status `degraded` liefern. Die bloße HTTP-Erreichbarkeit darf nicht mit „Rack betriebsbereit“ gleichgesetzt werden. [R03]

RBAC-Routing ist vorhanden; lesende Standardrouten werden `Viewer` zugeordnet, bestimmte Schreiboperationen `Operator` oder `Admin`. Die Konfigurationsdefaults setzen `auth.enabled = false`. „RBAC-Code existiert“ bedeutet daher nicht, dass Authentifizierung in der tatsächlich gestarteten Instanz aktiv ist. Die historischen pauschalen Sicherheitsformulierungen benötigen diese Einschränkung. [R03][R05]

### 9.3 WebSocket-Bestand und fehlende Indicator-Nachricht

Der aktuelle `UiMessage`-Enum enthält weiterhin:

```text
StateSnapshot
NodeMessage
CommandQueued
Error
```

Eine Variante `IndicatorState` ist dort nicht enthalten. Das UI erhält beim Registrieren einen Snapshot. Der WebSocket-Handler serialisiert JSON in **binäre WebSocket-Nachrichten**; er beantwortet Ping mit Pong und kann auf die Textanfrage `state` einen weiteren Snapshot senden. Ein neuer Client muss diese Transportdetails berücksichtigen. [R04]

Ein separates WERMA-Zustandsmodell, Lampen-Test-/Quittierungsendpunkte und ein eigener `netcore-rack-agent` sind durch diese vorhandenen generischen Funktionen noch nicht implementiert. Die heutige Routingprüfung ergab keine der in Abschnitt 4.4 vorgeschlagenen `/api/indicator/*`-Routen. Der gelesene Cargo-Workspace enthält keinen `bins/netcore-rack-agent`-Member. Ergänzende Default-Branch-Codeabfragen nach `WERMA`, `signal_tower` und `rack-agent` lieferten keine Treffer. Negative Suchergebnisse gelten nur im geprüften Umfang, nicht als Beweis über jede unbesuchte Branchhistorie. [R03][R04][R06]

### 9.4 Der historische Fanout ist weiter nutzbar, aber unverändert keine Lampensteuerung

In `bins/bluestation-bs/src/main.rs` besteht weiterhin `telemetry-fanout` mit den bisherigen Empfängern. Die gelesene Stelle enthält keine WERMA-Ausgangsansteuerung. Sie ist eine mögliche Ereignisquelle, kein fertiger Treiber. Im heutigen Umfeld muss außerdem die neuere Node-Gateway-/Backend-Architektur berücksichtigt werden, anstatt blind eine weitere konkurrierende TBS-Steuerverbindung einzubauen. [R02][R05]

Die Konfiguration enthält inzwischen eine optionale **nur lesende** Node-Gateway-Telemetrieanbindung mit `ws://node-gateway:8080/ws/backend`. Ihr Kommentar stellt ausdrücklich klar, dass die TBS beim Node Gateway bleibt und der Beobachter keinen Command-Transport registriert. Das ist eine relevante Integrationsalternative für eine Fortsetzung, keine hier getestete Aktivierung. [R05]

### 9.5 Inzwischen vorhandenes Hardware-Gateway

**Wichtige neue Ausgangslage:** `system-backend/hardware-gateway/` ist inzwischen als OPEN-LAB-Dienst für Hardware-I/O, Rack- und Umgebungsüberwachung vorhanden. Damit muss die gesamte zentrale Sensor-/Geräteverwaltung für den Rack-Agenten nicht neu erfunden werden. [R07]

Im gelesenen Python-Code sind unter anderem implementiert:

- HTTP- und MQTT-Telemetrie-Ingress;
- Geräteidentität, letzte Sichtung und Onlinezustand;
- Metriken, digitale `inputs` und gemeldete `outputs` als Zustandsdaten;
- Grenzwertbewertung für konfigurierte Messwerte;
- Ereignisse bei Registrierung, Wiederkehr, Timeout, Grenzwertüberschreitung/-rückkehr und Änderungen boolescher Eingänge;
- `netcore-event-v1`-Events, JSON-Zustand, NDJSON-Eventlog und retained MQTT-Zustände;
- eine Watchdog-Schleife zur **Geräte-Heartbeat-Überwachung**, kein Nachweis eines elektrischen Hardware-Watchdogs;
- Prometheus-Metriken und eine HTTP-WebUI. [R08]

**Entscheidende Grenze:** Das Einlesen eines Feldes `outputs` ist noch kein Schalten von GPIOs. Im gelesenen HTTP-Handler existiert als POST-Ingress `/api/v1/telemetry`, kein physischer Relais-Schaltendpunkt. `outputs_enabled` wird im Status ausgegeben; die vorhandenen Funktionen belegen keinen dadurch aktivierbaren WERMA-Treiber. Das bloße Umsetzen des Flags auf `true` ist folglich kein dokumentierter Aktivierungsweg für Lampen. [R08]

Der Dienst unterstützt im gelesenen Startcode nur `open_lab`, und der Status warnt ausdrücklich vor fehlender Anmeldung und fehlendem TLS. Auch die dort aufgerufenen `mosquitto_pub`-/`mosquitto_sub`-Kommandos enthalten in dieser Implementierung keine Auth-/TLS-Parameter. Eine Härtung beziehungsweise klar begrenzte Netzexposition gehört vor einer steuernden Erweiterung zur Arbeit. [R08]

### 9.6 Konsequenz für die Fortsetzung

Der historische Vorschlag „Control Room plus separater Pi“ bleibt als Trennung zwischen fachlicher Bewertung und elektrischer Ausgabe sinnvoll. Für den heutigen Stand lautet der **neue, noch zu bestätigende Integrationsvorschlag**:

```text
TBS / Node Gateway / bestehende Zustandsquellen
                 |
          Control Room / Statusauswertung
                 |
           zukünftiger Indicator-Vertrag
                 |
           separater I/O-Agent
                 |
              WERMA

I/O-Agent -- Sensor-/Gerätetelemetrie --> vorhandenes Hardware-Gateway
```

Vor der Implementierung ist zu entscheiden, ob das Hardware-Gateway auch eine gesicherte Ausgangssteuerung erhalten soll oder ausschließlich die Telemetrie-/Registrierungsseite bedient. Der heutige Code beweist noch keine dieser beiden WERMA-End-to-End-Varianten. Eine zweite unabhängige Geräte-Registry ohne Prüfung der vorhandenen Schnittstellen wäre vermeidbare Doppelarbeit.

## 10. Technisches Nachschlageverzeichnis

### 10.1 Verifizierte Dateien und ihre Rolle

Alle heutigen Angaben beziehen sich auf `Archiving@465ab5350e804d00343ea23713aff601a22cba1e`, soweit nicht ausdrücklich historisch markiert.

| Pfad | Relevanz |
|---|---|
| `Cargo.toml` | Rust-Workspace; Basisstation, Control Room und zahlreiche Backend-Crates, aber kein Rack-Agent-Member. |
| `bins/bluestation-bs/src/main.rs` | Aufbau der TBS, Telemetrie-Fanout und Control-Room-Anbindung. |
| `bins/netcore-control-room/src/http.rs` | Tatsächliche API-Routen, Healthantworten und Rollenzuordnung. |
| `bins/netcore-control-room/src/state.rs` | UI-Nachrichtentypen und Snapshot-/Broadcastverwaltung. |
| `bins/netcore-control-room/src/ws.rs` | JSON in binären UI-WebSocket-Nachrichten, Snapshotanfrage und Verbindungsbehandlung. |
| `bins/netcore-control-room/src/config.rs` | Server-/WebSocketdefaults, Auth, Persistenz, Federation und optionales Node-Gateway-Abonnement. |
| `crates/tetra-config/src/bluestation/config.rs` | Historisch gelesene `StackConfig`-Erweiterungsstelle; die vorgeschlagene Lampensektion ist damit nicht bestätigt. |
| `system-backend/hardware-gateway/README.md` | Bestehender OPEN-LAB-Sensor-/Rack-Dienst und Schnittstellenübersicht. |
| `system-backend/hardware-gateway/src/netcore_hardware_gateway.py` | Tatsächlicher Ingress, Registry, Events, MQTT, Watchdog und HTTP-Handler. |
| `system-backend/hardware-gateway/config/hardware-gateway.example.toml` | Geprüfte Beispielwerte für Bind, MQTT, Speicherpfade und Grenzwerte. |

Ausgewählte heutige Blob-SHAs zur zusätzlichen Nachvollziehbarkeit: `http.rs` = `d05ea89c6c176a156bd55211d42a36ef010b5182`; `state.rs` = `a953823e7ba06876b57dfa77837c52efcde7f6e1`; `config.rs` = `e85bb5de2fee1210ce6a3f24d800c9ec545b4394`; Hardware-Gateway-Code = `f24d9c4d52b3960026d6e69cad937b7faa6d3694`; Hardware-Gateway-Beispielkonfiguration = `e4f894a802cc5dd897d6e960acc90cdabad02432`.

### 10.2 Ports, Protokolle, Topics und Laufzeitpfade

| Schnittstelle / Wert | Geprüfter Wert | Einschränkung |
|---|---|---|
| Control-Room-Serverdefault | `127.0.0.1:9010` | Code-Default, keine bestätigte Rackadresse. Für entfernten Agenten muss die tatsächlich freigegebene Bind-/Proxyadresse bestimmt werden. |
| Control-Room-Node-WebSocket | `/node` | Konfigurierbar; kein neu erfundener Indicator-Endpunkt. |
| Control-Room-UI-WebSocket | `/ui` | Konfigurierbar; JSON wird als Binary-Frame verschickt. |
| Optionales Node-Gateway-Abonnement | `ws://node-gateway:8080/ws/backend` | Default `enabled = false`, nur lesender Beobachter. |
| Node-Gateway-Abonnement-Defaults | Reconnect 3 s, Timeout 10 s, stale 30 s | Nicht als zugesicherte Alarmverzögerung verwenden. |
| Federation-Defaults | Poll 5 s, Request-Timeout 1200 ms, Failure-Threshold 3 | Von der 5-s-Indicator-Beispiel-TTL unabhängig. |
| Control-Room-Datenbankpfad | `/var/lib/netcore-control-room/control-room.sqlite3` | Persistenz im Code-Default deaktiviert. |
| Hardware-Gateway-Beispielbind | `0.0.0.0:8250` | OPEN LAB ohne Anmeldung/TLS im gelesenen Dienst; Exposition prüfen. |
| Hardware-Gateway-MQTT-Beispiel | `127.0.0.1:1883` | Kein Nachweis des realen Brokerstandorts. |
| MQTT-Telemetrie | `netcore/v1/hardware/<device-id>/telemetry` | Gateway-Abonnement mit `+` als Geräte-Wildcard. |
| MQTT-normalisierter Zustand | `netcore/v1/state/hardware/<device-id>` | Retained; Empfänger müssen Datenalter beachten. |
| MQTT-Events | `netcore/v1/events/<event_type mit Punkten als Schrägstrichen>` | Payloadschema `netcore-event-v1`. |
| Gateway-Konfigurationspfad | `/etc/netcore/hardware-gateway.toml` | Laufzeitdefault aus dem Startcode; Repository-Beispieldatei heißt anders. |
| Gateway-Zustand | `/var/lib/netcore-hardware-gateway/state.json` | Beispielkonfiguration. |
| Gateway-Ereignisse | `/var/lib/netcore-hardware-gateway/events.ndjson` | Beispielkonfiguration. |
| Gateway-Heartbeat-Timeout | 30 s | Watchdog prüft in der gelesenen Implementierung alle 2 s. |
| Gateway `stale_after_secs` | 20 s in der Beispielkonfiguration | In der gelesenen Watchdog-/Ingress-Logik keine eigenständige Stale-Auswertung dieses Wertes festgestellt. Nicht als wirksame zusätzliche Sicherheitsstufe voraussetzen. |
| Gateway-Ausgangsflag | `outputs_enabled = false` | Kein Nachweis eines physischen Ausgangstreibers bei `true`. |

Gateway-Routen im geprüften Code: `GET /api/v1/status`, `/api/v1/devices`, `/api/v1/events`, `/health/live`, `/health/ready`, `/metrics`, `/openapi.json` sowie `POST /api/v1/telemetry`. [R05][R07][R08][R09]

Die Beispielgrenzwerte lauten Temperatur Warnung ab 40 °C/kritisch ab 55 °C, Feuchte ab 75 %/90 % und Versorgung unter beziehungsweise gleich 11,5 V/10,8 V nach der tatsächlichen Vergleichslogik. Das sind **Konfigurationsbeispiele**, keine für dieses Rack freigegebenen Grenzwerte. Insbesondere passen die Spannungswerte nicht automatisch zur angenommenen 24-V-Lampenversorgung. [R08][R09]

Nicht festgelegt wurden eigene Rack-Agent-Ports, GPIO-Chip/Line-Zuordnung, BPI-IP-Adressen, VLAN-Nummern, SNMP-Versionen/Objekte, TrueNAS-API-Version, NUT-Netzparameter oder endgültige Firewallregeln. Zugangsdaten wurden nicht in dieses Archiv übernommen.

## 11. Befehle, Installations- und Diagnoseabläufe

### 11.1 Was im Fachchat tatsächlich geschah

Es wurden GitHub-Metadaten, Quelltexte und PR-/Commitbezüge gelesen. Es wurden **keine** GPIO-Kommandos, Relaisprogramme, Lampentests, OpenWrt-Flashbefehle, Proxmox-Umbauten, TrueNAS-Installationen oder Rack-Deployments nachweislich ausgeführt.

Die Rust-, JSON- und TOML-Blöcke waren Architekturbeispiele. Insbesondere ist `netcore-rack-agent.service` kein durch seine Erwähnung installierter Dienst. Ein fertiger Installations-, Deployment- oder Reparaturablauf für WERMA beziehungsweise BPI liegt aus diesem Chat nicht vor.

### 11.2 Tatsächliche Diagnose im Archivlauf

Ein lokaler Git-Clone wurde versucht:

```bash
git clone --filter=blob:none --no-checkout --single-branch \
  --branch Archiving \
  https://github.com/JanHG98/netcore-tetra.git \
  /mnt/data/netcore-archive-work/repo
```

**Ergebnis: fehlgeschlagen, Exit 128, `Could not resolve host: github.com`.** Die angehängten lokalen Fetch-/Switch-Schritte wurden wegen des Abbruchs nicht ausgeführt. Das war eine DNS-/Netzzugriffsgrenze der Arbeitsumgebung, kein Beleg für einen defekten Zielbranch.

Die Repo-Prüfung erfolgte stattdessen erfolgreich über den verbundenen GitHub-Connector: `get_repo`, Branch-/Contents-/Tree-GETs, `fetch_file`, `compare_commits`, gezielte Codesuche und `get_pr_info`. Die autorisierte Dokumentationsspeicherung verwendet die GitHub-Git-Data-API; ein lokaler Shell-Push ist dafür nicht erforderlich.

Zwei Pfadfehler sind zu unterscheiden: Historisch lieferte der Abruf von `README.md` im Repo-Root 404, worauf relevante Dateien direkt gelesen wurden. Im Archivlauf lieferte der zunächst vermutete Repository-Pfad `system-backend/hardware-gateway/config/hardware-gateway.toml` 404; die Verzeichnisprüfung zeigte die tatsächlich vorhandene Datei `hardware-gateway.example.toml`. Der Laufzeitpfad unter `/etc/netcore/` bleibt davon getrennt.

### 11.3 Vorgeschlagene nächste Verifikation, noch nicht ausgeführt

Nach Festlegung und Freigabe einer Testumgebung wäre zuerst der lesende Datenweg zu prüfen, dann eine Ausgabe ohne angeschlossene Last, dann ein einzelner korrekt versorgter Kanal und erst danach der vollständige Lampentest. Dabei sind Datenfrische, Authentifizierung, Fallback und Rückmeldung vor einem automatischen Dauerbetrieb zu prüfen.

Für einen späteren OpenWrt-Test sind Image-/Boardrevision und Recoveryweg vor dem Flashen zu dokumentieren; für das Rack sind Leistungsbudget, USV-Verhalten und Netzwerk-/VLAN-Abnahme getrennte Schritte. Dieses Dokument gibt bewusst keinen angeblich bereits erfolgreichen Flash- oder Installationsbefehl vor.

## 12. Fehler, Korrekturen und verbleibende Risiken

| Punkt | Historische Aussage / Beobachtung | Einordnung bei Abschluss |
|---|---|---|
| WERMA-Spannung | In der Antwort als 24-V-Säule weitergeplant. | Nicht durch Artikelnummer/Typenschild belegt; vor Verdrahtung offen. |
| Konkrete GPIO-Pins | 17/27/22/23/24 im TOML-Beispiel. | Nur Beispiel; Nummerierung und Belegung ungeprüft. |
| Pull-down plus aktiv-low | Gleichzeitig empfohlen. | Widersprüchlich als pauschale Boot-Sicherheitsmaßnahme; sicheren Pegel aus Schaltung ableiten. |
| Relais und Blinken | Sehr pauschal als ungeeignet bezeichnet. | Häufiges/kurzes Schalten ist eine Auswahl- und Lebensdauerfrage, kein absolutes Verbot. |
| „Jeder Lampenumbau braucht Neubau“ | Argument gegen Direktintegration. | Für konfigurierbare Werte zu pauschal; Quellcode-/Treiberänderung von Konfiguration trennen. |
| Fertige Indicator-Ansteuerung | Endpunkte und Dienstnamen wirkten konkret. | Entwurf, nicht implementiert oder getestet. |
| API-/Health-Erreichbarkeit | Als Grundlage für Systemstatus genannt. | Kein hinreichender RF-/Rack-Gesundheitsnachweis; Payload und Alter prüfen. |
| RBAC | Im Repository vorhanden. | Nicht automatisch aktiv; aktueller Auth-Default ist aus, Hardware-Gateway ausdrücklich OPEN LAB. |
| BPI/OpenWrt-PR offen | Stand 18.07.2026. | Durch Merge am 24.08.2026 überholt; Stable-Image-/Geräteabnahme weiterhin offen. |
| BPI als „Managed Switch“ | Als Ersatz eines kleinen Switches empfohlen. | VLAN-/Bridge-Funktionalität, interne Pfade, Offload und Portgeschwindigkeit müssen konkret geprüft werden. Keine pauschale Enterprise-Switch-Gleichwertigkeit. |
| Unmanaged = automatisch ein VLAN | Vereinfachte Antwort zum Kleinswitch. | Trennung muss der korrekt konfigurierte Access-Port leisten; Tagweiterleitung nicht ausschließen. |
| 5-Port-Kapazität | Mehrere mögliche Managementgeräte genannt. | Uplink belegt einen Port, vier Endgeräteports bleiben. |
| „Ausfallsicherer“ | Vorteil eines separaten Agenten. | Relative Entkopplung, keine garantierte Anzeige bei Pi-/Versorgungs-/Gesamtausfall. |
| Aktuelles Hardware-Gateway | Registry und gemeldete `outputs` vorhanden. | Kein Nachweis einer physischen Ausgangssteuerung; Flag nicht mit Treiber verwechseln. |
| Lokaler Clone | DNS-Auflösung scheiterte. | Connectorzugriff funktioniert; kein Anlass für Force-Push, Branchwechsel oder Repo-Reparatur. |

Im ursprünglichen Fachchat traten keine belegten Hardware-/Betriebsstörungen auf, weil keine entsprechende Inbetriebnahme dokumentiert wurde. Die Tabelle beschreibt daher überwiegend Planungsrisiken und Präzisierungen, nicht erfolgreich behobene Anlagenfehler.

## 13. Tests und Nachweisgrenzen

### 13.1 Tatsächlich nachgewiesene Prüfungen

| Prüfung | Ergebnis | Grenze |
|---|---|---|
| Historische GitHub-Leseprüfung | Workspace, APIs, State-Typen, Fanout und PR #4 nachvollziehbar. | Keine Kompilierung und kein Gerätebetrieb. |
| Heutige Repo-/Branchprüfung | Kanonischer Reponame, Archiving-/main-SHAs und Unterschiede erfasst. | Nicht jeder Branch oder jede Datei vollständig auditiert. |
| Heutige relevante Quelltextprüfung | APIs/Fanout bestätigt; keine Indicator-Variante an den geprüften Stellen; Hardware-Gateway eingeordnet. | Statische Prüfung, kein Integrationstest. |
| OpenWrt-PR-Metadaten | Merge von #21083 bestätigt. | Kein lokaler Imagebau, kein Test auf BPI, kein Stable-Release-Nachweis. |
| PDF-Bestandsprüfung | Alle 25 Dateien öffnbar; Titel-/Seitenzahlinventar erstellt. | Keine vollständige Normen-/Konformitätsprüfung. |
| Lokaler Git-Clone | DNS-Fehler dokumentiert. | Keine lokale Arbeitskopie durch diesen Versuch. |

Es wurden im Archivlauf keine Cargo-Tests, Python-Diensttests, CI-Neuläufe, RF-Messungen, Relais-Langzeittests oder Netzwerkbenchmarks ausgeführt. Vorhandene Testverzeichnisse oder Tests in fremden PRs sind keine hier erzielten Testergebnisse.

### 13.2 Vorgeschlagene spätere Abnahmematrix

Alle folgenden Tests sind **offen**:

| Testfall | Erwartete Fragestellung |
|---|---|
| Einzelkanal und alle fünf Kanäle | Stimmen Farbe, elektrische Last, Versorgung und Einschaltreserve? |
| Boot/Reset/Prozessabsturz | Sind die Initialpegel eindeutig und gibt es keine unbeabsichtigte Freigabe? |
| Lampentest | Ist er berechtigt, zeitbegrenzt und von realen Alarmen unterscheidbar? |
| Gesunder Leerlauf | Bleibt die Anzeige auch ohne Calls/MS korrekt, ohne TTL-Scheinfehler? |
| Gruppen-/Einzelruf | Entspricht Blau der vereinbarten Bedeutung? |
| Notruf/Quittierung/Ende | Bleiben Funknotruf und Lampenquittierung getrennt und priorisiert? |
| Mehrfachfehler | Werden kritische Ursachen nicht verdeckt oder vorschnell gelöscht? |
| TBS-/Backend-/WAN-Verlust | Sind Fallback, Zeitgrenzen und Fehlerursache eindeutig? |
| Reconnect/retained Zustand | Wird kein veraltetes Grün als aktueller Normalzustand übernommen? |
| Ausgangskarte-/Versorgungsfehler | Was kann tatsächlich erkannt und noch angezeigt werden? |
| RF während Schaltvorgängen | Entstehen messbare Störungen durch Treiber, Netzteile, Lüfter oder Verkabelung? |
| BPI-VLAN/Firewall | Stimmen Segmenttrennung, Access-/Trunk-Zuordnung und Managementzugang? |
| BPI-Last/Offload | Reichen Switching, Routing, NAT und gewünschtes VPN unter realer Konfiguration? |
| PoE/Strom/Kühlung | Ist jedes Gerät versorgt und bleibt der Aufbau thermisch im zulässigen Bereich? |
| USV/Shutdown/NAS-Ausfall | Werden Dienste geordnet beendet, Daten geschützt und unnötige Funkabhängigkeiten vermieden? |

## 14. Offene Aufgaben, Roadmap-Kandidaten und nächste Schritte

Es wurden im Fachchat **keine verbindlichen Termine, Aufwandszusagen oder Prioritätsnummern** vereinbart. Die folgenden Prioritäten sind Vorschläge des Archiv-Reviews, damit ein späterer Statuslauf die Aufgaben eindeutig übernehmen kann. Sie werden ausschließlich hier erfasst; keine andere Roadmap-Datei wird durch diesen Auftrag verändert.

| ID | Vorgeschlagene Priorität | Aufgabe / Ergebnis | Abhängigkeiten und Status |
|---|---|---|---|
| RACK-01 | P0: vor Hardwareanschluss | WERMA-Artikelnummern, Spannung, AC/DC, Ströme, Anschlussführung und Montage inventarisieren. | Nutzerhardware erforderlich; offen. |
| RACK-02 | P0: Architekturfreigabe | Separater Pi oder lokaler Prozess entscheiden; heutigen Control Room, Node Gateway und Hardware-Gateway passend einbeziehen. | RACK-01 teilweise unabhängig; Idee konkretisieren. |
| RACK-03 | P0: Anzeigesemantik | Farben, Prioritäten, Rack-/Node-Scope, Quittierung, Override und Bedeutung von Blau festlegen. | Nutzerfreigabe; historischer Vorschlag liegt vor. |
| RACK-04 | P0: Fehlermodell | Frische/Lease, Heartbeat, Neustart, gespeicherte Zustände und Versorgungsausfall definieren. | RACK-02/03; 5-s-TTL nur Beispiel. |
| RACK-05 | P1: Software-MVP | Gesicherten lesenden Statusadapter, versionierten Indicator-Vertrag und simulierte Ausgänge implementieren. | RACK-02–04; noch nicht implementiert. |
| RACK-06 | P1: I/O-MVP | Konkrete Relais-/Transistorkarte auswählen, Pinbelegung/Initialpegel prüfen, Agent und systemd-Paket implementieren. | RACK-01/04/05; Hardwaretreiber fehlt. |
| RACK-07 | P1: End-to-End-Abnahme | Einzelkanal, fünf Farben, Call-/Notrufpfad, Reconnect und Fehlerszenarien testen. | RACK-05/06; keine Tests durchgeführt. |
| RACK-08 | P1: Routerentscheidung | BPI-R4-Pro-Revision, Image/Release, Combo-Portverhalten, Recovery, Strom und Kühlung verbindlich auswählen. | OpenWrt-Upstreammerge inzwischen vorhanden; Gerätetest offen. |
| RACK-09 | P1: Netzwerkplan | Port-/VLAN-/IP-/Firewallplan einschließlich optionalem unmanaged 5-Port-Switch und AP-Versorgung erstellen. | RACK-08; vier Endgeräteports hinter Uplink berücksichtigen. |
| RACK-10 | P1: Mechanik/Strom/RF | 12-/15-HE-Entwurf gegen reale Tiefe, Schienen, Last, USV, PDU, Potentialausgleich und RF-Verkabelung prüfen. | Stückliste und Anforderungen fehlen. |
| RACK-11 | P2: Monitoringausbau | Temperatursensoren, Feuchte, Tür, Lüfter sowie USV/NUT, PDU, NAS, Proxmox, WAN/VPN an vorhandene Zustandsquellen anbinden. | Gateway-Telemetrie vorhanden, konkrete Adapter/Betrieb unbestätigt. |
| RACK-12 | P2: Komfort/Ausbau | Lokales Agent-Webinterface, physischer Lampentest, Diagnoseansicht mit Ursachen, LTE/5G-Fallback und größerer PoE-Switch bei Bedarf. | Nach stabilem Grundbetrieb; keine Beschaffungsfreigabe. |

### Empfohlene Reihenfolge für die Wiederaufnahme

Zuerst die **vorhandenen WERMA-Elemente identifizieren** und entscheiden, was die Farben zuverlässig bedeuten sollen. Dann den heutigen Hardware-Gateway-/Control-Room-Bestand als Integrationsgrundlage festlegen und die Zustandsauswertung zunächst ohne reale Ausgänge testen. Erst danach die elektrische Karte und ihre sicheren Initialpegel anbinden.

Parallel kann die BPI-Routervariante mit einem dokumentierten Image und Recoveryweg getestet werden. Der kleine unmanaged Switch bleibt eine einfache, ausdrücklich gewünschte Bedarfsoption; er ersetzt keine fehlende VLAN-/Firewallplanung. Die endgültige Rackhöhe und USV sollten aus realer Stückliste, Leistung und Einbautiefe abgeleitet werden, nicht allein aus der frühen HE-Skizze.

## 15. Anhänge und ihre Relevanz

Alle nachfolgend genannten Dateien waren lokal zugänglich. Angegeben sind die durch Öffnen geprüfte Seitenzahl und die Identität des Deckblatts. Die Dateien werden mit diesem Archivauftrag **nicht erneut ins Repository kopiert**.

| Datei | Seiten | Identität / Gegenstand des Deckblatts |
|---|---:|---|
| `en_3003920308v010401p.pdf` | 22 | EN 300 392-3-8 V1.4.1 (2020-04), ISI Generic Speech Format Implementation |
| `en_30039209v010701p.pdf` | 46 | EN 300 392-9 V1.7.1 (2020-04), allgemeine Anforderungen an Supplementary Services |
| `ts_10081201v020205p.pdf` | 8 | TS 100 812-1 V2.2.5 (2003-10), SIM-ME/UICC physikalische und logische Eigenschaften |
| `en_3003921201v010202p.pdf` | 56 | EN 300 392-12-1 V1.2.2 (2007-08), Call Identification, Stage 3 |
| `en_3003920304v010301p.pdf` | 28 | EN 300 392-3-4 V1.3.1 (2010-08), ISI Short Data Service |
| `en_3003921117v010102p.pdf` | 18 | EN 300 392-11-17 V1.1.2 (2002-01), Include Call, Stage 2 |
| `en_3003921114v010101p.pdf` | 23 | EN 300 392-11-14 V1.1.1 (2002-07), Late Entry, Stage 2 |
| `es_20081202v020401m.pdf` | 139 | Final draft ES 200 812-2 V2.4.1 (2005-08), TSIM-Anwendung |
| `es_20081201v020205p.pdf` | 8 | ES 200 812-1 V2.2.5 (2003-12), TSIM-ME/UICC-Eigenschaften |
| `en_300812v020101p.pdf` | 156 | EN 300 812 V2.1.1 (2001-12), Security/SIM-ME |
| `en_3003921101v010201p.pdf` | 44 | EN 300 392-11-1 V1.2.1 (2004-01), Call Identification, Stage 2 |
| `en_3003921006v010401p.pdf` | 20 | EN 300 392-10-6 V1.4.1 (2006-08), Call Authorized by Dispatcher, Stage 1 |
| `en_3003921018v010301p.pdf` | 17 | EN 300 392-10-18 V1.3.1 (2003-10), Barring of Outgoing Calls, Stage 1 |
| `en_3003921216v010400a.pdf` | 67 | DRAFT EN 300 392-12-16 V1.4.0 (2026-03), Pre-emptive Priority Call, Stage 3 |
| `en_30039201v010601p.pdf` | 182 | EN 300 392-1 V1.6.1 (2020-04), General network design |
| `ets_30039214e01v.pdf` | 61 | FINAL DRAFT prETS 300 392-14 (1997-09), PICS proforma |
| `en_30039207v030501p.pdf` | 216 | EN 300 392-7 V3.5.1 (2019-07), Security |
| `en_30039401v030301p.pdf` | 169 | EN 300 394-1 V3.3.1 (2015-04), Radio conformance testing |
| `en_3003920313v010201p.pdf` | 191 | EN 300 392-3-13 V1.2.1 (2020-04), transportunabhängiger ISI Group Call |
| `en_30039502v010303p.pdf` | 94 | EN 300 395-2 V1.3.3 (2025-02), TETRA codec |
| `en_3003920303v010301p.pdf` | 251 | EN 300 392-3-3 V1.3.1 (2011-11), ISI Group Call |
| `en_30039205v020701p.pdf` | 320 | EN 300 392-5 V2.7.1 (2020-04), Peripheral Equipment Interface |
| `en_3003920315v010500a.pdf` | 380 | Draft EN 300 392-3-15 V1.5.0 (2026-04), transportunabhängiges ISI Mobility Management |
| `en_30039202v030801p.pdf` | 1445 | EN 300 392-2 V3.8.1 (2016-08), Air Interface |
| `ETSI.pdf` | 4100 | Sammeldatei; erstes Deckblatt EN 300 812 V2.1.1 (2001-12). Die weiteren enthaltenen Dokumente wurden hier nicht vollständig inventarisiert. |

Die Titel identifizieren Standards beziehungsweise Entwürfe in der **bereitgestellten Fassung**, nicht automatisch die heute jüngste veröffentlichte Version. Insbesondere werden „Draft“ und „Final draft“ nicht in verabschiedete Normen umgedeutet. Die Anhänge sind für spätere TETRA-Protokollarbeit relevant; der hier behandelte Rack-Indikator beobachtet zunächst Implementierungszustände und wurde nicht gegen eine vollständige TETRA-Konformitätsprüfreihe abgenommen.

## 16. Quellen, Dateien, Commits und PRs

### Historischer Chat und Repositorybezug

- **[CHAT]** Sichtbarer Fachverlauf mit Jans WERMA-/Rack-Frage, BPI-R4-Pro-Nachfrage und letzter 5-Port-Switch-Ergänzung. Originaltitel und Chatlink fehlen; Abschnitt 1 beschreibt die Grenze.
- **[R00]** [PR #4 „Control room“](https://github.com/JanHG98/netcore-tetra/pull/4) und [historischer Mergecommit 6143ed50](https://github.com/JanHG98/netcore-tetra/commit/6143ed50ae23decb37eff2e3c04237b78a9ffd0b). PR-Metadaten am 03.10.2026 erneut gelesen.
- **[R01]** Historischer Stand: [TBS-main](https://github.com/JanHG98/netcore-tetra/blob/6143ed50ae23decb37eff2e3c04237b78a9ffd0b/bins/bluestation-bs/src/main.rs), [Control-Room-HTTP](https://github.com/JanHG98/netcore-tetra/blob/6143ed50ae23decb37eff2e3c04237b78a9ffd0b/bins/netcore-control-room/src/http.rs), [State](https://github.com/JanHG98/netcore-tetra/blob/6143ed50ae23decb37eff2e3c04237b78a9ffd0b/bins/netcore-control-room/src/state.rs), [WebSocket](https://github.com/JanHG98/netcore-tetra/blob/6143ed50ae23decb37eff2e3c04237b78a9ffd0b/bins/netcore-control-room/src/ws.rs), [StackConfig](https://github.com/JanHG98/netcore-tetra/blob/6143ed50ae23decb37eff2e3c04237b78a9ffd0b/crates/tetra-config/src/bluestation/config.rs). Grundlage sind die im Fachchat sichtbaren Dateiabrufe.

### Heutige Repositoryprüfung

- **[R02]** [TBS-main/Fanout am Prüfcommit](https://github.com/JanHG98/netcore-tetra/blob/465ab5350e804d00343ea23713aff601a22cba1e/bins/bluestation-bs/src/main.rs), insbesondere gelesener Abschnitt ab Zeile 750.
- **[R03]** [Control-Room-HTTP am Prüfcommit](https://github.com/JanHG98/netcore-tetra/blob/465ab5350e804d00343ea23713aff601a22cba1e/bins/netcore-control-room/src/http.rs), insbesondere Routing und `required_role_for_request`.
- **[R04]** [Control-Room-State](https://github.com/JanHG98/netcore-tetra/blob/465ab5350e804d00343ea23713aff601a22cba1e/bins/netcore-control-room/src/state.rs) und [UI-WebSocket-Handler](https://github.com/JanHG98/netcore-tetra/blob/465ab5350e804d00343ea23713aff601a22cba1e/bins/netcore-control-room/src/ws.rs) am Prüfcommit.
- **[R05]** [Control-Room-Konfiguration](https://github.com/JanHG98/netcore-tetra/blob/465ab5350e804d00343ea23713aff601a22cba1e/bins/netcore-control-room/src/config.rs), Server-, Auth-, Persistenz-, Federation- und Node-Gateway-Defaults.
- **[R06]** [Cargo-Workspace](https://github.com/JanHG98/netcore-tetra/blob/465ab5350e804d00343ea23713aff601a22cba1e/Cargo.toml) am Prüfcommit; ergänzende Default-Branch-Suchen nach den vorgeschlagenen WERMA-/Rack-Agent-Bezeichnungen.
- **[R07]** [Hardware-Gateway-README](https://github.com/JanHG98/netcore-tetra/blob/465ab5350e804d00343ea23713aff601a22cba1e/system-backend/hardware-gateway/README.md).
- **[R08]** [Hardware-Gateway-Implementierung](https://github.com/JanHG98/netcore-tetra/blob/465ab5350e804d00343ea23713aff601a22cba1e/system-backend/hardware-gateway/src/netcore_hardware_gateway.py), insbesondere Python-Logik vor dem eingebetteten HTML und HTTP-/Startcode ab Zeile 460.
- **[R09]** [Hardware-Gateway-Beispielkonfiguration](https://github.com/JanHG98/netcore-tetra/blob/465ab5350e804d00343ea23713aff601a22cba1e/system-backend/hardware-gateway/config/hardware-gateway.example.toml).
- **[R10]** [Erfasster main-Stand](https://github.com/JanHG98/netcore-tetra/commit/6aa9be8f74ab731f72dc133a5f8e90c5018c626d) und [Vergleich zum Eingangsstand Archiving](https://github.com/JanHG98/netcore-tetra/compare/6aa9be8f74ab731f72dc133a5f8e90c5018c626d...465ab5350e804d00343ea23713aff601a22cba1e).

### Externe Quellen und heutige Verifikation

- **[E01]** [Banana Pi: offizielle BPI-R4-Pro-Produktseite](https://www.banana-pi.com/en/bananapi-router/205.html), am 03.10.2026 für Port-/Hardwareangaben gelesen.
- **[E02]** [OpenWrt-PR #21083](https://github.com/openwrt/openwrt/pull/21083), heute per GitHub-Connector geprüft; [gemeldeter Mergecommit](https://github.com/openwrt/openwrt/commit/554b5dcc5199d07a84f4fba9f6a56066e5871858). Quelle für Supportmerge und PR-spezifische Hardwarebeschreibung, kein eigener Funktionstest.
- **[E03]** [Banana-Pi-Boarddokumentation](https://docs.banana-pi.org/en/BPI-R4_Pro/BananaPi_BPI-R4_Pro) und [Getting Started](https://docs.banana-pi.org/en/BPI-R4_Pro/GettingStarted_BPI-R4_Pro), historische Referenzen. Beim heutigen Abruf keine verwertbaren Textinhalte im Webparser; Detailaussagen deshalb nicht damit als frisch verifiziert ausgegeben.
- **[E04]** [Raspberry Pi: GPIO-/Hardwaredokumentation](https://www.raspberrypi.com/documentation/computers/raspberry-pi.html), Abschnitte GPIO Outputs/Inputs und Voltage specifications, am 03.10.2026 gelesen.
- **[E05]** [OpenWrt DSA Mini-Tutorial](https://openwrt.org/docs/guide-user/network/dsa/dsa-mini-tutorial) und [Firewall-Zonen für VLANs](https://openwrt.org/docs/guide-user/network/dsa/dsa-mini-tutorial/dsa-common-config/5-firewall-zones-for-vlans), am 03.10.2026 zur Abgrenzung von Bridging/VLANs und geroutetem Firewallverkehr herangezogen. Beispiele sind nicht als geprüfte BPI-Konfiguration übernommen.
- **[E06]** Historisch genannte [WERMA-KombiSIGN-71-Übersicht](https://api.werma.com/de/s_c1017/Signalsaeulen/Modulare_Signalsaeulen/KombiSIGN_71/) und [WERMA-Beispielelement 64444055](https://www.werma.com/KS71-LED-EVS-Element-24VDC-CL/64444055). **Im Archivlauf nicht als Identifikation von Jans Hardware verifiziert.**

## 17. Übergabefazit

Die Machbarkeit wurde auf Architektur- und Quelltextebene plausibel begründet: NetCore besitzt die erforderlichen Arten von Zustandsdaten, und ein unabhängiger I/O-Agent kann daraus eine Rackanzeige machen. Die direkte Relais-/Transistoransteuerung und ihr sicherer Fehlerzustand müssen aber erst implementiert und an den echten Lampen geprüft werden.

Die letzte Planungsrichtung ist ein **eigenständiger BPI-R4 Pro für Routing/Switching**, bei Bedarf ergänzt um den von Jan genannten **unmanaged 5-Port-Switch**. Die Wahl des 8X, die Firmware, die VLAN-/Portbelegung und der endgültige Rackaufbau sind noch keine bestätigte Installation.

Für die Weiterarbeit sind zwei Aktualisierungen besonders wichtig: **OpenWrt-Support für den 8X ist inzwischen in den Hauptzweig gemergt; zentrale Hardwaretelemetrie ist in NetCore inzwischen vorhanden.** Daraus folgt weniger Neuentwicklung auf der Datenseite, aber nicht das Vorhandensein eines fertigen Lampentreibers. Dieses Archiv bewahrt die Entwürfe, ohne sie als bereits laufende Technik auszugeben.
