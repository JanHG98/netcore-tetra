# Brainstorming: Basisstations-Dashboard, Deutsch, Integrationen, Wiki und DGNA-Gruppenanzeige

## Rahmen und Quellenstand

| Feld | Wert |
|---|---|
| Projekt | NetCore-Tetra |
| Thema | Deutschsprachiges Basisstations-Dashboard; versteckte Integrationen; Wiki-Neuaufbau; DGNA-Gruppenbuch; Gruppennamen und Überlauf-Hover |
| Historischer Zeitraum | Einzelschritte sind nicht zuverlässig datiert; die dokumentierte Reihenfolge der Änderungen und Betriebsrückmeldungen ist maßgeblich. |
| Notizstand | 2026-10-03, Zeitzone Europe/Berlin |
| Archivziel | `JanHG98/netcore-tetra`, ausschließlich Branch `Archiving`, Verzeichnis `Docs/archive/` |
| Geprüfter Archivbranch vor dem Schreiben | `Archiving` bei `c4ac047e0e167f38969804f5cc4da3e7fad0b721` |
| Zugehöriger Root-Tree | `0ad7b2f05c8d2aeb1f603bb47b7f270234ea9abe` |
| Zusätzlich gelesener geprüfter Standardbranch | `main` bei `7137e0dd69877e1b604bf89148fd8b6b590c1a97` |
| Am 03.10.2026 geprüfter relevanter Merge | PR #59, `feat/netcore-dashboard-design`, Merge-Commit `7137e0dd69877e1b604bf89148fd8b6b590c1a97` |
| Ursprünglich genannte Repository-Adresse | `JanHG98/flowstation`; die Repository-Abfrage löste diese Adresse am 03.10.2026 auf `JanHG98/netcore-tetra` auf, in beiden Fällen Repository-ID `1281497427`. |

**Befund:** DGNA-Dropdown und Gruppennamen-/Hover-Anzeige wurden jeweils im Betrieb bestätigt. Die beiden erhaltenen Enddateien enthalten jedoch nicht denselben Funktionsumfang: Die DGNA-Datei enthält das Dropdown, die spätere Hover-Datei nicht mehr. Der Repository-Befund vom 03.10.2026 zeigt den alten DGNA-Dialog bei vorhandener Gruppennamen-Anzeige. Eine gemeinsame, dauerhaft erhaltene Umsetzung beider Änderungen ist nicht nachgewiesen.

**Nicht als aktuelle Installationsdatei verwenden:** Die historischen vollständigen `html.rs`-Anhänge stammen aus der monolithischen Oberfläche. Im am 03.10.2026 geprüften Repository ist `html.rs` ein Asset-Wrapper mit `include_str!` und `include_bytes!`. Ein Austausch gegen einen alten historischen Anhang würde die inzwischen geänderte Struktur zurücksetzen.

## 1. Quellenlage, Quellenstand und Grenzen

Grundlagen sind historische Anforderungen, Screenshots, Rust-Dateien, Markdown-Anleitungen und ZIP-/Patch-Artefakte. Geprüft wurden UI-, Sprach-, Gruppen-, DGNA-, Navigations- und Startfunktionen sowie Paketintegrität und relevante Wiki-Seiten.

Der Repository-Abgleich vom 03.10.2026 umfasst Branchstände, Dashboard-Wrapper, relevante Bereiche von `ui/dashboard.html`, die UI-Shell `ui/netcore.js` sowie Login- und Wiki-Dateien. Die Befunde stehen in Abschnitt 10.

Die historische Dokumentation enthält Lücken. Nicht erhaltene Zwischenschritte werden nicht rekonstruiert; maßgeblich sind die vorhandenen Anforderungen, Artefakte und Betriebsrückmeldungen.

Mehrere Downloads wurden früher unter demselben Namen `html.rs` angeboten. Die am 03.10.2026 vorhandene Datei dieses Namens beweist nicht den Inhalt jedes früheren Downloads. Die einzelnen historischen Zwischenstände und deren damalige Erzeugungsschritte sind nicht vollständig rekonstruierbar. Wo ein eigenständig benannter Endanhang erhalten ist, wird dessen tatsächlicher Inhalt und neu berechneter Hash verwendet.

Nicht verfügbar sind ein Zugriff auf die laufende Basisstation, die tatsächliche Systemd-Unit, Build- und Kopierprotokolle des Zielgeräts, Browser-Konsole/Netzwerkmitschnitt sowie ein aktueller On-Air-Test. Im zugänglichen Verlauf existiert keine eindeutige Zuordnung des damals laufenden Binarys zu einem Git-Commit.

Die zusätzlich bereitgestellten ETSI-PDFs sind Projektquellen. Im sichtbaren Verlauf wurden daraus keine konkreten Protokolländerungen für diese UI-Arbeit abgeleitet. Sie werden als verfügbare Anhänge inventarisiert, aber nicht als vollständig durchgearbeitete Normprüfung oder Konformitätsnachweis ausgegeben. Insbesondere wurden für diesen Dokumentation keine tausendseitigen Normen pauschal als geprüft erklärt.

### 1.1 Verbindliche Statusbegriffe

| Status | Bedeutung in diesem Dokument |
|---|---|
| **Idee** | Erwogene Möglichkeit, ohne verbindliche Umsetzungszusage. |
| **Beschlossen/geplant** | Ausdrückliche Anforderung oder im Verlauf gewählter Ansatz; noch kein Codebeweis. |
| **Implementiert im Artefakt** | In einer konkret vorhandenen Datei nachlesbarer Code. Keine Aussage über GitHub oder Deployment. |
| **Implementiert im Repository** | Am angegebenen geprüften Commit tatsächlich nachgelesen. Keine Aussage über die laufende Anlage. |
| **Getestet** | Konkrete Prüfung mit benanntem Gegenstand und Ergebnis; die Prüfgrenze gehört dazu. |
| **Im Betrieb bestätigt** | Explizite Rückmeldung der Projektplanung zur jeweiligen Funktion. Kein automatischer Nachweis aller Nebenfunktionen. |
| **Historisch behauptet / unbestätigt** | Frühere Aussage ohne am 03.10.2026 reproduzierbaren Beleg oder ohne passende Betriebsrückmeldung. |
| **Überholt / Regression** | Durch spätere Festlegung ersetzt oder in einem späteren Artefakt wieder verlorengegangen. |

## 2. Ziel, Ausgangslage und Themen

Ausgangspunkt war das Dashboard des zunächst als `JanHG98/flowstation` bezeichneten Repositories. Ziel war eine auf NetCore zugeschnittene Basisstationsoberfläche: Deutsch als einzige Sprache, keine Sprachauswahl, keine regulär sichtbaren Menüpunkte für DAPNET, EchoLink, MeshCom und GeoAlarm. Versteckte Aufrufmöglichkeiten sind als Alternative zur vollständigen Entfernung ausdrücklich zulässig.

Im weiteren Verlauf kamen der vollständige Wiki-Umbau, kontextgerechtes Branding, eine separate interne Aufrufdokumentation, die Beseitigung englischer Resttexte, ein Logoabstand sowie zwei konkrete Gruppenfunktionen hinzu. Dabei sollte bestehende Telefonbuch-/Directory-Funktionalität wiederverwendet werden, statt eine weitere Datenquelle anzulegen.

| Reihenfolge | Inhalt und belastbare Einordnung |
|---|---|
| A | Wunsch Deutsch-only und vier Integrationen entfernen oder verstecken. Gewählt wurde die versteckte Variante. |
| B | Erstes Austauschpaket, vollständiges Repository-ZIP, Patch und Build-Anleitung wurden angeboten. Ein Git-Push war zu diesem Zeitpunkt nicht erfolgt. |
| C | Auftrag, alle Wiki-Dateien auf den aktuellen Stand zu bringen, sinnvoll zu ergänzen und sichtbares „Flowstation“ durch „NetCore“ oder „Basisstation“ zu ersetzen. |
| D | Wiki-Paket mit 29 Markdown-Dateien und ungefähr 7.000 Wörtern sowie Patch und Bericht wurde geliefert. |
| E | Für die versteckten Links wurde ausschließlich eine Markdown-Datei festgelegt. `VERSTECKTE_INTEGRATIONEN.md` wurde erstellt. |
| F | Screenshots zeigten weiterhin englische Oberfläche und Sprachauswahl. Zulässig waren die Übersetzung des Standard-/Englischblocks oder Deutsch als tatsächlicher Standard. Die späteren Artefakte benutzen Deutsch direkt. |
| G | Beim Schriftzug „NET CORE“ wurde `gap:8px` in `.logo-title` als Abstand zwischen zwei Flex-Elementen identifiziert; vorgeschlagene Änderung `gap:0`. |
| H | DGNA-Dialog sollte zusätzlich zur GSSI-Eingabe eine Gruppenauswahl aus dem vorhandenen Gruppenbuch erhalten. Mehrere Lieferungen änderten die sichtbare Oberfläche nicht. |
| I | Zwischenzeitlich wurde unbelegt ein altes Binary als sichere Ursache behauptet. Später wurde diese Diagnose durch die Feststellung fehlenden Codes in der gelieferten Datei korrigiert. |
| J | Eigenständig benannte Datei `html_DGNA_Gruppenbuch_FINAL.rs` wurde geliefert; Funktion im Betrieb bestätigt. |
| K | Wunsch Gruppennamen statt Nummern und Hover für weitere Gruppen. Eine erste Lieferung blieb wirkungslos; Betriebsprüfung ohne sichtbare Änderung. |
| L | `html_Gruppennamen_Hover_FINAL.rs` wurde geliefert; Gruppenanzeige im Betrieb bestätigt. Der geprüfte Inhaltsvergleich zeigt zugleich den Verlust des DGNA-Dropdowns in dieser Datei. |
| M | Dieser Dokumentation: technische Dokumentation und Index in `Archiving`, keine Produktcodeänderung und kein Merge. |

## 3. Endgültige Anforderungen und Entscheidungen

| ID | Anforderung / Entscheidung | Begründung und Ergebnis |
|---|---|---|
| UI-DE-01 | Deutsch als einzige nutzbare UI-Sprache; Sprachauswahl entfernen. | Eine deutsche Anzeige darf nicht von einer manuellen Sprachwahl abhängen. Deutsch ist in den Endartefakten fest verdrahtet; Resttexte bleiben als offener Punkt. |
| UI-DE-02 | Auch dynamisch erzeugte Texte, Lesbarkeit, Themenwahl, Login und Konfiguration berücksichtigen. | Nur statische HTML-Texte zu ändern genügt nicht, wenn JavaScript sie später ersetzt. |
| UI-INT-01 | DAPNET, EchoLink, MeshCom und GeoAlarm im normalen Dashboard ausblenden, nicht den Backend-Code entfernen. | Gewählte Alternative erhält vorhandene Integrationen für gezielten Zugriff. |
| UI-INT-02 | Versteckte Integrationen sollen auch die normale Health-Übersicht nicht belasten. | Historisch wurden ausgeblendete Statuskarten und vermiedene Health-Abfragen angekündigt; nicht gleichbedeutend mit dem Abschalten sämtlicher Hintergrunddienste. |
| UI-INT-03 | Aktivierung über URL-Parameter, Zustand für den Browser-Tab, explizite Deaktivierung. | Im erhaltenen Code und geprüften Dashboard nachlesbar. Keine Authentifizierung oder zusätzliche Rolle. |
| DOC-01 | Wiki umfassend aktualisieren und sinnvoll erweitern. | Erhaltenes Paket enthält 29 Markdown-Dateien inklusive `_Sidebar.md`; geprüftes Wiki ist inzwischen weiterentwickelt. |
| DOC-02 | In Produkttexten „NetCore“ oder „Basisstation“ statt „Flowstation“. | Kontextabhängiges Branding. Technische Dateinamen, historische Repo-Adressen, Codebezeichner und Herkunftsbelege werden nicht irreführend umbenannt. |
| DOC-03 | Versteckte Aufrufsyntax separat als Markdown bereitstellen. | `VERSTECKTE_INTEGRATIONEN.md` wurde geliefert und ist am 03.10.2026 unter `Docs/` vorhanden. |
| UI-LOGO-01 | Sichtbaren Zusatzabstand zwischen „NET“ und „CORE“ beseitigen. | Reiner CSS-Hinweis `gap:8px` → `gap:0`; Ausführung nicht separat bestätigt. Spätere Anhänge enthalten weiterhin `gap:8px`. |
| UI-DGNA-01 | Gruppenbuch-Dropdown im DGNA-Dialog zusätzlich zur manuellen Eingabe. | Auswahlfehler durch bloße Zahlen vermeiden, bestehendes Directory wiederverwenden. Separates DGNA-Endartefakt implementiert und historisch bestätigt. |
| UI-DGNA-02 | Name und GSSI anzeigen, zugewiesene Gruppen markieren, Auswahl ins GSSI-Feld übernehmen. | In der DGNA-Enddatei implementiert; manuelle Eingabe bleibt bei Directory-Ausfall möglich. |
| UI-GROUP-01 | Gruppennamen statt bloßer GSSI in der Funkgerätetabelle. | In der Hover-Enddatei und am 03.10.2026 im Repository implementiert; Funktion im Betrieb bestätigt. |
| UI-GROUP-02 | Maximal zwei Gruppen sichtbar; weitere als `+N weitere` mit Namens-/GSSI-Hover. | Gewählter konkreter Ansatz zum Anforderung „Hover oder so“. Der Zähler zählt nur tatsächlich ausgeblendete Gruppen. |
| UI-GROUP-03 | Ausgewählte Gruppe zuerst/blau; unbekannte Gruppe als `GSSI <Nummer>`. | Verhindert den Verlust der technischen Identität bei fehlender Namensauflösung. Gilt für eine ausgewählte Gruppe, die in der Gruppenliste enthalten ist. |

Nicht beschlossen wurden die vollständige Entfernung der Integrations-Backends, die Änderung von Funkfrequenzen, ein neuer Directory-Dienst, eine neue zentrale RBAC-Implementierung oder ein genereller Umbau der Luftschnittstelle in dieser Entwicklungsphase.

## 4. Historische Architektur und Implementierungsdetails

### 4.1 Einbettung der Oberfläche

Die historischen Dateien enthalten zwei Rust-Konstanten mit eingebettetem HTML, CSS und JavaScript: `DASHBOARD_HTML` und `LOGIN_HTML`. Die Texte gelangen damit über den Rust-Build in die ausführbare Basisstation. Ein bloßer Dateiaustausch ohne passenden Build und Start des zugehörigen Binarys verändert eine so ausgelieferte Oberfläche nicht.

Das ist eine Eigenschaft des Codes, aber **kein Beweis**, dass die wiederholten Fehlversuche dieses Vorhabens durch ein altes Binary verursacht wurden. Die später gelesenen Dateien enthielten die angekündigten Änderungen teilweise schlicht nicht.

Die Raw-String-Begrenzer müssen zum jeweiligen Öffner passen. In den beiden eigenständig benannten Enddateien lautet der Dashboard-Öffner `r#"` mit Abschluss `"#;`; der Login verwendet `r##"` mit Abschluss `"##;`. Deshalb darf ein scheinbar „überzähliges“ `#` nicht pauschal in der ganzen Datei entfernt werden. Die frühere Behauptung einer notwendigen Korrektur von `"##;` lässt sich nicht auf jede Konstante übertragen. Die am 03.10.2026 erhaltenen beiden Endartefakte haben an ihren HTML-Abschlüssen passende Begrenzer.

### 4.2 Deutsch-only und Textquellen

In der Hover-Enddatei steht ab etwa Zeile 5135 `const currentLang='de'`; `t(k,v)` liest aus `LANGS.de`. Die Dokumentensprache wird auf `de` gesetzt und die alte Sprachauswahl nicht mehr als Auswahloberfläche verwendet.

Die Übersetzungsarbeit erfasste laut Lieferbeschreibung Lesbarkeit, Farbschema, Funkgeräte, Basisstationsdetails, Dual-Carrier, Nachbarzellen, Nachlaufzeit, Registrierungszugang, Zeitschlitze, Systemzustand, Konfiguration, WLAN, Telegram, Integrationen und Exportbezeichnungen. Nicht jede dieser Flächen wurde in den Arbeitsnotizen anschließend einzeln abgenommen.

Die geprüften Quellen widerlegen eine pauschale Aussage „alles vollständig deutsch“: Beispielsweise existieren weiterhin englische Kartenmeldungen sowie ein englischer DGNA-Platzhalter. Kommentare, Protokollnamen, JSON-Schlüssel und interne Bezeichner sind davon zu unterscheiden; sie dürfen nicht blind übersetzt werden.

### 4.3 Versteckte Integrationen

Die Implementierung verwendet diese Modulnamen:

```text
dapnet
echolink
meshcom
geoalarm
```

Beispielaufrufe, mit standortspezifischer Adresse und tatsächlich konfiguriertem Port:

```text
http://<BASISSTATION-IP>:<PORT>/?intern=netcore&modul=dapnet
http://<BASISSTATION-IP>:<PORT>/?intern=netcore&modul=echolink
http://<BASISSTATION-IP>:<PORT>/?intern=netcore&modul=meshcom
http://<BASISSTATION-IP>:<PORT>/?intern=netcore&modul=geoalarm
http://<BASISSTATION-IP>:<PORT>/?intern=off
```

`HIDDEN_INTEGRATIONS` definiert die vier Namen. `URLSearchParams` liest `intern` und `modul`. `intern=netcore` setzt `sessionStorage['fs_hidden_integrations']='1'`; `intern=off` entfernt den Eintrag. `hiddenIntegrationsVisible` steuert die Navigation. Anschließend entfernt `history.replaceState` die beiden Parameter aus der sichtbaren Adresse, andere Query-Parameter und den Hash lässt der Code stehen.

`applyHiddenIntegrationVisibility()` setzt die Darstellung der Navigationspunkte. Die historische `showPage()`-Funktion lenkt einen unerlaubten regulären Aufruf versteckter Seiten auf die Funkgeräteansicht um. Die neue Shell lenkt entsprechend auf ihre Hauptseite um.

Die tatsächlich belegte Persistenz ist `sessionStorage`, kein serverseitiges Benutzerrecht. Explizites `?intern=off` ist der dokumentierte Ausschalter. Aus dem Code allein wurde kein Test von Tab-Duplizierung, Browser-Sitzungswiederherstellung oder übernommenem Tab-Zustand durchgeführt; „jeder neue Tab ist garantiert aus“ sollte daher nicht als geprüfte Eigenschaft fortgeschrieben werden.

**Sicherheitsgrenze:** Diese Parameter sind UI-Schalter, keine Zugangsdaten. Das Ausblenden sperrt weder APIs noch Backend-Dienste. Das Entfernen aus der Adresszeile beweist keine Entfernung aus Zugriffslogs. Die separate Anleitung war absichtlich nicht im damaligen öffentlichen Wiki verlinkt; sie liegt am 03.10.2026 dennoch im öffentlichen Repository unter `Docs/VERSTECKTE_INTEGRATIONEN.md`. Die Aufrufsyntax ist daher nicht als Geheimnis oder Sicherheitsbarriere zu behandeln.

### 4.4 DGNA-Gruppenbuch: tatsächlich funktionierendes Einzelartefakt

Maßgeblicher historischer Anhang: `html_DGNA_Gruppenbuch_FINAL.rs`, SHA-256 siehe Abschnitt 14. Der Dropdown steht dort an Zeile 4691. Relevante Elemente:

```text
dgna-modal
dgna-issi
dgna-current
dgna-group-select
dgna-group-select-hint
dgna-gssi
```

Der Dialog zeigt „Gruppe aus dem Gruppenbuch“ und lädt über `GET /api/groups` mit `cache:'no-store'` und `credentials:'same-origin'`. Er akzeptiert die im Code unterstützten Formen eines Dictionaries oder von Arrays direkt beziehungsweise unter `groups` oder `results`. ID-Felder werden aus `gssi`, `id` oder `ssi` gelesen, Bezeichnungen aus `name`, `label`, `title` oder `callsign`.

`normalizeDgnaGroups()` verwirft IDs außerhalb `1..0xFFFFFF` beziehungsweise nicht ganzzahlige Werte. Die Dropdown-Zeilen werden nach Namen mit deutscher/numerischer Sortierung geordnet. Die Anzeige enthält den Namen, `GSSI <Nummer>` und gegebenenfalls „bereits zugewiesen“.

Beteiligte Funktionen:

| Funktion / Zustand | Aufgabe |
|---|---|
| `dgnaGroupRegistry` | Gruppendaten des Dialogs |
| `dgnaGroupsLoading` | Verhindert parallele Ladeaufrufe |
| `normalizeDgnaGroups(raw)` | Eingabeformate normalisieren und IDs begrenzen |
| `renderDgnaGroupSelect(issi)` | Optionen und Zugewiesen-Markierung aufbauen |
| `loadDgnaGroups(issi)` | HTTP-Abfrage, Ladezustand, Fehler- und Leerzustand |
| `selectDgnaGroup()` | Auswahl in `dgna-gssi` übernehmen |
| `chooseDgnaCurrentGroup(gssi)` | Bereits zugewiesene Gruppe per Schaltfläche übernehmen |
| `openDgna(issi)` | Werte zurücksetzen, Dialog öffnen, Gruppenbuch laden |
| `sendDgna(attach)` | Bestehenden DGNA-Befehl per WebSocket senden |

Bei leerem Gruppenbuch oder Ladefehler wird die Auswahl deaktiviert und ein entsprechender Hinweis gesetzt. Das numerische GSSI-Feld bleibt als Rückfallebene erhalten. Die vorhandenen Gruppen sind in dieser Datei anklickbare Schaltflächen, zeigen aber weiterhin Nummern; eine Namensanzeige dieser aktuellen Gruppen ist dort nicht umgesetzt.

Der ausgehende Befehl bleibt:

```javascript
wsSend({type:'dgna', issi, gssi, attach});
```

„Zuweisen“ verwendet `attach:true`, „Entfernen“ `attach:false`. Die Änderung betrifft die Auswahlhilfe, nicht nachgewiesenermaßen das DGNA-Protokoll der Basisstation. `sendDgna()` schließt den Dialog nach dem Sendeversuch; die Existenz eines UI-Sendeversuchs ist kein Beweis für Empfang oder Übernahme durch das Funkgerät. Die vollständige Eingabe-/ACK-Behandlung war nicht Gegenstand einer bestätigten Abnahme.

### 4.5 Gruppennamen und `+N weitere`

Maßgeblicher Endanhang: `html_Gruppennamen_Hover_FINAL.rs`. Relevante Stellen sind ungefähr Zeilen 5720–5773 für die Registry, 6505–6571 für `renderStations()` und 10531 für den Boot-Aufruf.

`loadStationGroupRegistry()` ruft ebenfalls `/api/groups` ohne Browsercache und mit Same-Origin-Sitzung auf. Bei erfolgreicher Antwort baut `normalizeStationGroups()` ein Lookup auf. Bei HTTP-/JSON-/Netzwerkfehlern wird die Registry geleert; anschließend läuft `renderStations()` erneut. Der numerische Fallback bleibt dadurch sichtbar. Es gibt in dieser Funktion keinen benannten Timeout, keinen Retry und keine differenzierte Fehlermeldung in der Tabelle.

`stationGroupName(gssi)` liefert den Namen oder `GSSI <Nummer>`. `stationGroupFullLabel(gssi)` liefert `Name · GSSI <Nummer>` oder nur die GSSI. Das normale Tabellen-Rendering verwendet `state.ms`, insbesondere `m.groups` und `m.selected_group`.

Die Gruppenliste wird in Zahlen umgewandelt, auf positive Ganzzahlen gefiltert und mit `Set` dedupliziert. Die ausgewählte Gruppe wird bevorzugt einsortiert, danach wird nach deutschem Namen numerisch und ohne strikte Groß-/Kleinschreibung sortiert. Die ersten zwei Gruppen erscheinen direkt. Für den Rest ergibt sich `hidden.length` als Zähler.

```javascript
const visible = sortedGroups.slice(0, 2);
const hidden = sortedGroups.slice(2);
// Die sichtbare Beschriftung lautet: +<hidden.length> weitere.
```

Ein `title`-Attribut mit Zeilenumbrüchen enthält alle ausgeblendeten Namen und GSSIs. Es ist ein nativer Browser-Tooltip, kein eigenes Popover. Die ausgewählte Gruppe erhält ein blaues Badge mit Marker, andere Gruppen ein dezenteres Badge. Namen werden für HTML beziehungsweise Attribute escaped.

Beispiel zur Semantik, nicht Jans tatsächliches Telefonbuch:

```text
Vier eindeutige Gruppen, davon zwei direkt sichtbar:
[Ausgewählte Gruppe] [Weitere sichtbare Gruppe] [+2 weitere]

Hover über +2 weitere:
Weitere Gruppen:
Dritte Gruppe · GSSI 15501
Vierte Gruppe · GSSI 15502
```

Bei nur zwei Gruppen erscheint kein Überlauf-Badge. Bei keiner Gruppe erscheint ein Strich. Eine nur in `selected_group` genannte, aber nicht in `m.groups` enthaltene GSSI wird nicht automatisch ergänzt. Das historische „+2 zugeordnet“ war dagegen kein sauberer Überlaufzähler, weil zusätzlich alle Gruppen gerendert wurden.

Der Normalizer der Namensanzeige ist nicht identisch mit dem strengeren DGNA-Normalizer: Er validiert Dictionary-Schlüssel nicht auf denselben numerischen Bereich. Verschachtelte Dictionary-Wrapper und führende Nullen sollten vor einer Vereinheitlichung gezielt getestet werden; sie werden hier nicht als allgemein unterstützt behauptet.

## 5. Wiki-Umbau und gelieferte Dokumentation

Das erhaltene `netcore-wiki-aktuell-v1.3.0.zip` enthält 29 Markdown-Dateien einschließlich Navigation. Die Nachzählung anhand von durch Whitespace getrennten Wörtern ergab 7.053 Wörter. „v1.3.0“ ist die Paket-/Dokumentationsbezeichnung dieses Artefakts, kein Beleg für einen veröffentlichten Git-Tag oder eine bestimmte installierte Version.

| Themenbereich | Enthaltene Dateien unter `wiki/` |
|---|---|
| Einstieg und Architektur | `Home.md`, `Architecture.md` |
| Installation und Konfiguration | `Installation.md`, `Build-and-Update.md`, `Configuration.md`, `Systemd-Service.md` |
| Funkbetrieb | `Dual-Carrier.md`, `ISSI-and-GSSI.md`, `Registration-and-Affiliation.md`, `Calls.md` |
| Daten und Status | `SDS-and-U-STATUS.md`, `Status-Messages.md`, `Home-Mode-Display.md`, `LIP-and-GPS.md` |
| Directory | `NetCore-Directory.md`, `Directory-API.md`, `Devices.md`, `Basestations.md`, `Groups.md`, `Device-Groups.md` |
| Bedienung und Medien | `Dashboard.md`, `Audio-Zentrale.md`, `Control-Room.md` |
| Betrieb und Fehler | `Backup-and-Fallback.md`, `Security-and-Operations.md`, `Troubleshooting.md`, `Journalctl-Filters.md`, `Common-Build-Errors.md` |
| Navigation | `_Sidebar.md` |

Der damalige Umbau zielte auf die Entfernung alter HTML-Wrapper, kaputter Namen wie `.md.md`, `.ms.md` und `#U2010`, konsistente interne Links und kontextgerechtes Branding. Der Bericht erklärt außerdem, lokale Zugangsdaten und konkrete interne Adressen nicht übernommen sowie die versteckte Aufrufsyntax nicht veröffentlicht zu haben.

Neu geprüft wurden die 29 Paketdateien auf die alte Zeichenfolge `flowstation` und auf `intern=`: keine Treffer. Die Prüfung von Wiki-/relativen Links außerhalb von Codebeispielen ergab 39 geprüfte Links ohne fehlendes Ziel. Das ist keine vollständige fachliche Prüfung jeder Konfigurationsaussage und kein allgemeiner Secret-Scanner-Nachweis.

Die historischen Kapitel beschreiben beispielsweise Konfigurationsformat `0.6`, `stack_mode = "Bs"`, optionale Directory-/Control-Room-/TTS-/NFS-Komponenten und eine beispielhafte Systemd-Unit. Diese Angaben stammen aus der damaligen Dokumentation und sind nicht automatisch aktuelle Laufzeitparameter.

**Neu festgestellte Patchgrenze:** Der Wiki-Patch lässt sich in isolierten Kopien sowohl auf den `wiki/`-Inhalt von `flowstation-main(3).zip` als auch von `flowstation-main(4).zip` anwenden. Alle 29 erwarteten Zieldateien stimmen danach bytegenau mit dem Wiki-ZIP überein. Die Verzeichnisse sind dennoch nicht identisch: Beim Stand `(4)` bleiben vier alte Unicode-Bindestrich-Dateien erhalten, beim Stand `(3)` insgesamt 28 zusätzliche Dateien. Dazu gehören dort auch ältere Control-Room-Anleitungen.

Die vier im Stand `(4)` verbliebenen Namen sind `Device‐Groups.md.md`, `NetCore‐Directory.md.md`, `Status‐Messages.md.md` und `Systemd‐Service.md.md`. Erfolgreiches `git apply` allein beweist also keine vollständige Altdateibereinigung. Es rechtfertigt insbesondere nicht, unbekannte zusätzliche Dokumentation am 03.10.2026 ungeprüft zu löschen.

Das geprüfte `wiki/Home.md` ist bereits eine andere, breitere Systemwiki-Fassung mit explizitem Bezugsstand vom 26.09.2026. Das alte Paket ist deshalb kein aktueller Komplett-Ersatz für den Repository-Wikiordner.

## 6. Dateien, Dienste, Pfade, Schnittstellen und Parameter

| Eintrag | Bedeutung und Beleggrenze |
|---|---|
| `crates/tetra-entities/src/net_dashboard/html.rs` | Historischer monolithischer Austauschpfad; am 03.10.2026 nur Asset-Einbindung. |
| `crates/tetra-entities/src/net_dashboard/ui/` | Am 03.10.2026 geprüfter Ablagebereich der Dashboard-/Login-Dateien und Design-Assets. |
| `wiki/` | Tatsächliche Schreibweise des Dokumentationsordners in den Paketen und im am 03.10.2026 geprüften Repository. „Wiki“ war auch die natürliche Bezeichnung im Nutzertext. |
| `Docs/VERSTECKTE_INTEGRATIONEN.md` | Am 03.10.2026 nachgewiesene Anleitung für die vier versteckten Module. |
| `Docs/archive/` | Ausschließlicher Schreibbereich der Notizen. |
| `~/flowstation` | Historisch verwendetes lokales Arbeitsverzeichnis; nicht als am 03.10.2026 verbindlich festgestellt. |
| `target/release/bluestation-bs` | Historischer Build-Ausgabepfad in den vorgeschlagenen Befehlen. |
| `/usr/local/bin/bluestation-bs` | Beispiel für einen separaten Deploymentpfad; kein ausgelesenes `ExecStart` des Zielgeräts. |
| `tetra.service` | In den historischen Installations-/Reparaturbefehlen verwendeter Systemd-Dienstname. |
| `/etc/systemd/system/tetra.service` | Ablage der Beispielunit im damaligen Wiki. |
| `/etc/netcore/config.toml`, `/opt/netcore`, Benutzer/Gruppe `netcore` | Beispiele der damaligen Wiki-Unit, keine gesicherte Live-Konfiguration. |
| `netcore-directory.service`, `netcore-piper.service`, `netcore-control-room.service` | Im historischen Wiki genannte ergänzende Units; in dieser Entwicklungsphase nicht als installiert getestet. |
| `GET /api/groups` | Gruppenmetadaten für Dropdown und Namensauflösung. Historischer Dashboard-Server leitet auf die Directory-Rohdatenroute weiter; geprüfter Frontendaufruf und Wiki-API sind nachgelesen. |
| `/api/devices` / `devices.json` | Im historischen Dashboard verwendete Geräte-/ISSI-Namensquelle; nicht mit Gruppen oder DGNA gleichsetzen. |
| `/ws` | Historischer Dashboard-WebSocket für Zustandsereignisse und Befehle, insbesondere `type:'dgna'`. |
| `/api/btsinfo`, `/api/dualcarrier` | Historische und geprüfte UI-Aufrufe für Zell-/Trägerinformationen. |
| `/api/login`, `/login` | Loginpfade der historischen Oberfläche. |
| `/api/session`, `/api/public` | Am 03.10.2026 nachgelesene Sitzungsprüfung und öffentliche Statusansicht. |
| HTTP-Port der Basisstation | In den Arbeitsnotizen in Aufruflinks bewusst `<PORT>`; `8080` ist ein Beispiel aus dem damaligen Wiki, kein verifizierter Live-Port. |
| Directory-Port | `8095` ist ein dokumentiertes Directory-Beispiel, unter anderem im geprüften `wiki/Directory-API.md`; nicht der automatisch gültige Dashboardport. |
| `config_version = "0.6"` | Historische Wiki-Angabe; kein erneuter Parser-Kompatibilitätstest der Quellenprüfung. |
| `fs_hidden_integrations` | Schlüssel des tabbezogenen UI-Schalters in `sessionStorage`. |
| `fs_lang` | Alter Sprachspeicherschlüssel; der am 03.10.2026 gelesene Code entfernt ihn über `netcoreStorage.removeItem`. |
| `fs_uisize`, `fs_theme`, `netcore-theme` | Lesbarkeits-/Themezustände; neue Theme-Synchronisierung gehört zum geprüften Designstand, nicht zum ursprünglichen DGNA-Auftrag. |

### 6.1 In Screenshots sichtbare Betriebswerte

Die frühen Screenshots zeigen TX `418.0000 MHz`, RX `408.0000 MHz`, angezeigten Duplexabstand `−10.000 MHz`, MCC `901`, MNC `1510`, Hauptträger `720`, Sekundärträger `721`, Dual-Carrier als konfiguriert und laufend, Nachbarzelle aus und Nachlaufzeit `5 s`. Der Registrierungszugang wurde als offen angezeigt; zum Aufnahmezeitpunkt standen null registrierte Funkgeräte und null aktive Rufe in der Übersicht.

Spätere DGNA-Screenshots zeigen Terminal-ISSI `5102` und beispielhaft GSSIs `15201`, `15202` und `15501`. Das sind Beobachtungen des damaligen UI-Inhalts, keine Messprotokolle und keine neu erteilte Konfigurations- oder Sendefreigabe. Aus dieser Entwicklungsphase wurde kein Auftrag zur Änderung dieser Parameter abgeleitet.

## 7. Installations-, Deployment- und Diagnoseabläufe

### 7.1 Historisch vorgeschlagener Austausch einer monolithischen Datei

Die folgenden Schritte wurden in den Arbeitsnotizen empfohlen. Ihre vollständige erfolgreiche Ausführung auf dem Zielhost ist nicht durch Terminalausgaben belegt. Sie gelten nur für den damaligen monolithischen Stand und sind **keine geprüfte Austausch-Anleitung**:

```bash
sudo systemctl stop tetra.service
cd ~/flowstation
cp crates/tetra-entities/src/net_dashboard/html.rs \
   crates/tetra-entities/src/net_dashboard/html.rs.backup-$(date +%Y%m%d-%H%M%S)
# Historisch folgten das Entfernen der alten Datei und das Kopieren des passenden Anhangs.
cargo clean
rm -rf target
cargo build --release --features asterisk
sudo systemctl start tetra.service
sudo systemctl status tetra.service --no-pager
sudo journalctl -u tetra.service -n 150 --no-pager
```

`cargo clean` und zusätzliches Löschen von `target` wurden beide vorgeschlagen. Das ist kein Ersatz für die Prüfung des Quellstands und des gestarteten Binarys. Das vorherige Entfernen der Quelldatei ist für einen normalen überprüften Kopiervorgang nicht nötig. Ein fehlerhafter Build darf nicht als erfolgreiches Deployment weiterbehandelt werden.

Im Wiki stehen außerdem `cargo build --release -p bluestation-bs` und ein minimaler Build mit `--no-default-features`. Diese Varianten dürfen nicht ohne Prüfung der Features des tatsächlich verwendeten Repository-Stands vermischt werden.

### 7.2 Historische Source-/Binary-Diagnose

Vorgeschlagen, aber ohne erhaltene Zielhost-Ergebnisse:

```bash
systemctl cat tetra.service | grep -E 'ExecStart|WorkingDirectory'
grep -n 'dgna-group-select' crates/tetra-entities/src/net_dashboard/html.rs
PID=$(systemctl show -p MainPID --value tetra.service)
sudo readlink -f /proc/$PID/exe
```

Für den Beispielpfad `/usr/local/bin/bluestation-bs` wurde außerdem Kopieren des neuen `target/release/bluestation-bs` und Setzen von Ausführungsrechten vorgeschlagen. Der tatsächliche `ExecStart` ist in den historischen Belegen nicht enthalten.

Ein `strings`-Suchtest auf „Gruppe aus dem Gruppenbuch“ wurde als entscheidender Nachweis dargestellt. Diese damalige Schlussfolgerung war zu absolut: Ein fehlender Treffer unterscheidet nicht allein zwischen falschem Quellstand, falschem Artefakt, anderem Build oder alter Installation. Außerdem sollte bei einer Prozessprüfung die tatsächliche `/proc/<PID>/exe`-Referenz nicht unbesehen mit einer möglicherweise bereits ersetzten Datei am aufgelösten Pfad gleichgesetzt werden.

### 7.3 Am 03.10.2026 empfohlene, nicht ausgeführte Diagnose

Zuerst die **ausgelieferten Quellen und die tatsächlich ausgelieferte HTML-Seite** prüfen, danach das Deployment. Der geprüfte Code liegt unter `ui/`, nicht mehr vollständig in `html.rs`:

```bash
# Auf dem Zielhost im tatsächlichen Repository-Verzeichnis ausführen.
git status --short
git branch --show-current
git rev-parse HEAD
systemctl show tetra.service -p ExecStart -p WorkingDirectory -p MainPID

grep -nE 'dgna-group-select|function openDgna|loadStationGroupRegistry|hidden.length' \
  crates/tetra-entities/src/net_dashboard/ui/dashboard.html
```

Ein fehlendes `dgna-group-select` ist im hier geprüften geprüften Quellstand gerade zu erwarten: Das Dropdown fehlt. Ein erneuter Clean-Build desselben Codes kann es nicht erzeugen. Erst die fehlende Funktion in die aktuelle Asset-Struktur integrieren, dann passende Tests, Build und Deployment durchführen.

Bei Browser-/HTTP-Prüfungen prüfen, ob wirklich das Dashboard oder lediglich eine Login-/Fehlerseite zurückgegeben wird. Ein `401`, eine Umleitung oder ein nicht gestarteter Dienst ist kein negativer Beweis für vorhandenen oder fehlenden UI-Code. Authentifizierungswerte gehören nicht in archivierte Befehlszeilen oder Logs.

### 7.4 Wiki-Anwendung: historisch, nicht am 03.10.2026 blind übernehmen

Historisch vorgesehen waren entweder den gesicherten Komplettaustausch des damaligen `wiki/`-Ordners aus dem ZIP oder:

```bash
git status
git apply --check /PFAD/netcore-wiki-aktuell-v1.3.0.patch
git apply /PFAD/netcore-wiki-aktuell-v1.3.0.patch
```

Nur Wiki-Änderungen benötigen keinen Rust-Build. Der Komplettaustausch mit `rm -rf wiki` war eine damalige Anweisung, keine nachgewiesene Ausführung und keine Empfehlung für den mittlerweile erweiterten Ordner. Die in Abschnitt 5 beschriebenen zusätzlichen Altdateien zeigen, weshalb nach einem Patch das resultierende Verzeichnis geprüft werden muss.

### 7.5 Browser-Neuladen

`Strg+F5` beziehungsweise `Strg+Umschalt+R` wurden mehrfach vorgeschlagen. Die erfolgreiche Betriebsrückmeldung belegt, dass am Ende eine sichtbare Änderung ankam; sie belegt nicht, dass Browsercache die vorherige Ursache war. Im historischen monolithischen Modell war der Build-/Binary-Pfad entscheidend; im geprüften Modell müssen zusätzlich die ausgelieferten Asset-Dateien zusammenpassen.

## 8. Fehler, Ursachen, Korrekturen und verbleibende Probleme

### 8.1 Angekündigte Änderungen fehlten im gelieferten Inhalt

Mehrere als fertig bezeichnete Lieferungen enthielten weiterhin den alten Dialog. Der erneute Inhaltsvergleich zeigte unveränderte DGNA-Blöcke und unverändertes `openDgna()`. Beim ersten Versuch der Gruppennamen-/Hover-Anzeige trat dasselbe Problem auf.

Die am 03.10.2026 vorhandenen `html.rs` und `html(1).rs` sind byteidentisch und enthalten weder `dgna-group-select` noch die spätere `+N weitere`-Implementierung. Ihr SHA-256 ist `130b1e4da4ab838bbbe5d1e7ac5720762204ff178d01a7cb3414d8cfe77e0210`. Diese Dateien dürfen nicht als Beleg für eine erfolgreich ausgelieferte Erweiterung benutzt werden.

Die funktionierenden Einzellieferungen waren die separat benannten Enddateien. Eindeutige Dateinamen und echte Inhaltsprüfung halfen bei der Zuordnung. Eine Prüfsumme muss aber zum tatsächlich bereitgestellten Anhang gehören; eine in den Arbeitsnotizen ausgeschriebene SHA allein beweist keine Umsetzung.

### 8.2 Unbelegte Diagnose „garantiert altes Binary“

Die allgemeine Erklärung der Compile-Time-Einbettung war zutreffend für die historischen Quellen. Die sichere Diagnose eines alten laufenden Binarys war ohne Quell-/Prozessprüfung nicht belegt und wurde durch spätere Befunde fehlenden Codes überholt. Sie wird hier ausdrücklich nicht als festgestellte Fehlerursache übernommen.

### 8.3 Neu belegte Regression: Dropdown und Hover nicht zusammen erhalten

| Erhaltener Stand | DGNA-Dropdown | Gruppennamen / Überlauf-Hover | Historische Rückmeldung |
|---|---|---|---|
| `html.rs` / `html(1).rs` | Nein | Nein | Wiederholte Fehlmeldungen; einzelne frühere gleichnamige Downloads nicht vollständig rekonstruierbar. |
| `html_DGNA_Gruppenbuch_FINAL.rs` | Ja | Nein | Dropdown im Betrieb bestätigt |
| `html_Gruppennamen_Hover_FINAL.rs` | Nein | Ja | Gruppenanzeige im Betrieb bestätigt. |
| Geprüftes `ui/dashboard.html`, geprüfter Commit | Nein, alter Modal- und `openDgna`-Block | Ja | Keine neue Zielhost-Abnahme in der Quellenprüfung. |

Die Betriebsrückmeldungen widersprechen dem Befund nicht: Sie betreffen nacheinander unterschiedliche Funktionen. Es gibt keine anschließende ausdrückliche Bestätigung, dass beide nach dem letzten vollständigen Dateiaustausch weiterhin zusammen funktionierten.

### 8.4 Namensanzeige im DGNA-Dialog nicht belegt

Eine frühere Lieferbeschreibung behauptete zusätzlich namentliche aktuelle Gruppen im DGNA-Fenster. Der erhaltene DGNA-Endanhang rendert dort numerische Schaltflächen; der Hover-Endanhang und der geprüfte Quellstand rendern sogar wieder die alten numerischen Spans. Die Behauptung darf nicht als implementierte Funktion weitergetragen werden.

### 8.5 Sprachreste

Nachgewiesene Beispiele sind der DGNA-Platzhalter `e.g. 100` sowie Kartenmeldungen `No radios registered at BS`, `No registered radios with positions (...)` und der sichtbare Fallback `unknown`. Im historischen Endanhang ist außerdem der Lesbarkeits-Tooltip `Text size &amp; contrast` enthalten. Teilweise wird statisches HTML später durch `data-i18n` ersetzt; nicht jeder englische Quelltext ist daher zwingend dauerhaft sichtbar. Die genannten dynamischen Kartenmeldungen und nicht übersetzten Attribute erfordern gezielte UI-Prüfung.

### 8.6 Logoabstand

Der im Screenshot gezeigte Abstand wurde mit `.logo-title { display:flex; ... gap:8px; }` erklärt. Der vorgeschlagene Nullabstand ist nachvollziehbar, wurde aber nicht als eigener Patch geliefert und nicht separat bestätigt. Die später erhaltenen Komplettdateien enthalten weiterhin `gap:8px`. Für das geprüfte neue Login ist die tatsächlich verwendete Struktur maßgeblich; der alte Selektor ist kein universeller Reparaturbefehl.

### 8.7 Hover-Zugänglichkeit und Aktualität der Gruppendaten

Der Browser-Tooltip ist für Mausbedienung implementiert und ausdrücklich insgesamt bestätigt. Ein gezielter Touch-, Tastatur- oder Screenreader-Test ist nicht überliefert. Der Überlauf ist ein `span` ohne eigenen Fokus-/Klickdialog. Das ist ein Ausbaukandidat, kein in den Arbeitsnotizen ausdrücklich abgelehnter Funktionsumfang.

Die Namensregistry wird beim Start geladen. Spätere Änderungen im Directory erscheinen nicht allein dadurch sicher sofort in einer offenen Ansicht. Fehler führen zum numerischen Fallback; ein belastbarer manueller Refresh oder eine gezielte Aktualisierung ist als Folgeschritt zu prüfen. Die damalige getrennte Registry für DGNA und Tabelle begünstigt auseinanderlaufende Implementierungen.

## 9. Tests und Ergebnisse

### 9.1 Historisch berichtete Prüfungen

Historisch berichtet wurden JavaScript-Syntaxchecks, Integritätsprüfungen der ZIPs, sauberes Patch-Anwenden und beim Wiki einen bytegenauen Vergleich. Für den ersten Dashboard-Umbau wurde ausdrücklich kein vollständiger Cargo-Build durchgeführt, da Cargo in der damaligen Umgebung fehlte.

Nicht alle damaligen Toolprotokolle sind zugänglich. Historisch behauptete Prüfungen werden deshalb nicht ungeprüft auf jede gleichnamige Zwischendatei übertragen. Ein JavaScript-Syntaxcheck kann eine fehlende Funktion ohnehin nicht als vorhanden bestätigen.

### 9.2 Bei der Archivierung tatsächlich erneut ausgeführt

| Prüfung | Gegenstand | Ergebnis und Grenze |
|---|---|---|
| SHA-256 | Fünf gemountete Rust-Dateien | Hashes neu berechnet; die beiden separat benannten Enddateien stimmen mit den zuletzt dazu genannten historischen Hashes überein. |
| JavaScript-Parsing mit `node --check` | Je zwei eingebettete Skriptblöcke in `html.rs`, `html(1).rs`, `html(3).rs`, DGNA-Enddatei und Hover-Enddatei | Zehn Prüfungen, jeweils Exitcode 0. Keine vollständige Browser-/DOM-/Backendprüfung. |
| Raw-String-Abschlussvergleich | Dashboard und Login der beiden Enddateien | Öffner und jeweiliger HTML-Abschluss stimmen bezüglich Anzahl der `#` überein. Kein Rust-Compile. |
| Code-Marker und Funktionsvergleich | DGNA-Enddatei, Hover-Enddatei, alte `html.rs` | Getrennter Funktionsumfang und Dropdown-Regression nachgewiesen. |
| Isolierte JavaScript-Funktionsprüfungen | Registry-/Tabellenfunktionen aus der Hover-Enddatei, DGNA-Normalizer aus der DGNA-Enddatei, mit DOM-Stubs | Zwölf Checks bestanden; Details unten. Kein echter Browser und kein Directory-Server. |
| ZIP-CRC-/Integritätsprüfung | Alle sechs vorhandenen ZIP-Pakete dieses Vorhabens | `ZipFile.testzip()` lieferte jeweils keinen beschädigten Eintrag. Das prüft Archivintegrität, nicht fachliche Korrektheit. |
| Wiki-Inhalt | Eigenständiges Wiki-ZIP | 29 Markdown-Dateien, 7.053 Whitespace-Wörter, keine `flowstation`-/`intern=`-Treffer. |
| Wiki-Linkprüfung | Wiki-/relative Links außerhalb von Codebeispielen | 39 Links, keine fehlenden Ziele im Paket. |
| Wiki-Patch | Isolierte Wiki-Kopien aus `(3)` und `(4)` | `git apply --check` und Anwendung erfolgreich; alle 29 erwarteten Dateien bytegleich, zusätzliche Altdateien bleiben vorhanden. |
| GitHub-Leseprüfung | Festgehaltene Branch-/Commitstände und relevante Dateien | Am 03.10.2026 geprüfte Asset-Struktur, Gruppennamen-Code, alter DGNA-Block und separate Integrationsanleitung nachgewiesen. |

Die zwölf isolierten Funktionschecks prüften: Dictionary mit String-/Objektlabels; direktes Array; `groups`-Arraywrapper; `results`-Arraywrapper; numerischen Fallback; Vollbezeichnung mit Name/GSSI; ausgewählte Gruppe zuerst/blau; korrekten `+2`-Zähler inklusive versteckter Namen bei deduplizierter Liste; genau zwei Gruppen ohne Überlauf; leere Gruppenliste; HTML-Escaping eines Testnamens; DGNA-ID-Grenzen `1..0xFFFFFF`.

Der dafür verwendete lokale Prüfharness ist kein bestehender CI-Test des Repositories. Im Produktcode wurden für diesen Dokumentation weder Tests ergänzt noch Fehler repariert. Er bestätigt nur die genannten isolierten historischen Funktionen.

### 9.3 Betriebsrückmeldungen und nicht getestete Bereiche

„jetzt.“ bestätigt die historisch gelieferte DGNA-Auswahloberfläche. „klappt“ bestätigt die anschließend gewünschte Gruppenanzeige. Daraus werden keine automatischen Aussagen zu HF-Qualität, allen Funkgeräten, DGNA-ACKs, Multi-Cell, Medien, SIP, Sicherheitsrollen oder aktuellen Builds abgeleitet.

Ein vollständiger Cargo-/Rust-Build konnte auch bei der Quellenprüfung vom 03.10.2026 nicht erfolgen: `cargo` und `rustc` sind in der Arbeitsumgebung nicht installiert. Es gab keinen Zielhost-, Browser-E2E- oder On-Air-Test. Die geprüften vollständigen UI-Assets wurden nicht als Gesamtsystem ausgeführt. Eine Zertifizierung oder ETSI-Konformitätsprüfung ist nicht Bestandteil dieser Ergebnisse.

## 10. Repository-Befund vom 03.10.2026

Dieser Abschnitt beschreibt ausschließlich den am 2026-10-03 gelesenen Stand, nicht rückwirkend den Zustand des damaligen Betriebs.

### 10.1 Refs, Identität und Vergleich

`JanHG98/netcore-tetra` war über den verbundenen GitHub-Zugang les- und schreibbar. Die alte Adresse `JanHG98/flowstation` wurde auf dasselbe Repository mit ID `1281497427` aufgelöst. Der tatsächliche Standardbranch ist `main`; der Dokumentation schreibt nur `Archiving`.

Der Vergleich `main@7137e0d...` gegen `Archiving@c4ac047...` meldete `diverged`, `ahead_by:13`, `behind_by:1`, Merge-Base `2fe2a1939a8795db3816d45973282781dae856f0`. Die Branches sind deshalb nicht als identisch bezeichnet. Die Vergleichsliste enthält Dokumentationsänderungen einschließlich bereits vorhandener Archive und `Docs/Control_Room/Readme.md`; diese bestehenden Inhalte gehören nicht zu diesem Auftrag und werden nicht verändert.

Für `crates/tetra-entities/src/net_dashboard/ui/dashboard.html` wurde auf beiden genannten Refs derselbe Git-Blob festgestellt: `6161b3ebc6ae76c86f39ed1413dca1ca39ba1b8c`. Die Befunde am gelesenen Dashboard gelten damit für genau diese Datei in beiden überprüften Ständen.

### 10.2 Asset-Struktur statt monolithischer Austauschdatei

Der geprüfte Wrapper enthält:

```rust
pub const DASHBOARD_HTML: &str = include_str!("ui/dashboard.html");
pub const LOGIN_HTML: &str = include_str!("ui/login.html");
pub const NETCORE_CSS: &str = include_str!("ui/netcore.css");
pub const NETCORE_JS: &str = include_str!("ui/netcore.js");
pub const NETCORE_RF_CSS: &str = include_str!("ui/netcore-rf.css");
pub const NETCORE_RF_JS: &str = include_str!("ui/netcore-rf.js");
pub const NETCORE_LOGIN_JS: &str = include_str!("ui/netcore-login.js");
pub const NETCORE_LOGO: &[u8] = include_bytes!("ui/netcore-logo.png");
```

Git-Blob des Wrappers: `caf9cd6932346141021ed2f4ef62fbc79562fece`. Die neue Shell hat den gelesenen Blob `d0a8132bc748a91a3506a8a95a942d5772ef60e9`; `login.html` den Blob `f15ad0fcc056e955c0d3a01408e1e11c52f28b45`.

Diese Umstellung ist im Kontext des am 03.10.2026 sichtbaren PR #59 „NetCore-Design mit Dark Mode für Basisstation und alle Dienst-WebUIs“ einzuordnen. Der PR ist ein geprüfter Bezugspunkt, nicht der nachträglich erfundene Ursprungs-PR der historischen Entwicklungsstand-Anhänge.

### 10.3 Funktionsabgleich

| Bereich | Am 03.10.2026 im Code nachgelesen | Konsequenz |
|---|---|---|
| Sprache | `currentLang='de'`, Zugriff auf `LANGS.de`, Entfernen von `fs_lang`, `document.documentElement.lang='de'` | Deutsch-only-Mechanismus vorhanden; nicht alle Textreste entfernt. |
| Versteckte Module | Vierer-Liste, Query-/SessionStorage-Schalter, Navigation entsprechend eingeschränkt | Mechanismus erhalten; weiterhin keine zusätzliche Autorisierungsgrenze. |
| Namensregistry | `normalizeStationGroups`, `stationGroupName`, `stationGroupFullLabel`, `loadStationGroupRegistry` | Gruppenmetadaten bleiben Quelle der Tabellenbezeichnungen. |
| Tabellenanzeige | Deduplizierung, Auswahlpriorität, zwei sichtbare Gruppen, `title` mit versteckten Namen und GSSIs | Historisch bestätigte Hover-Logik ist im Repository vorhanden. |
| DGNA | Alter Dialog mit Zahlenfeld und alte einzeilige `openDgna()`-Funktion; kein Dropdown im gelesenen Modal | Separat funktionierende Erweiterung wurde in diesem Stand nicht erhalten. |
| Neue Gerätedetails | `renderRadioDetail()` in der Shell verwendet `(device.groups\|\|[]).join(', ')` | Zusätzliche neue Detailansicht zeigt weiterhin Nummern. Konsistenzwunsch als neuer Kandidat, nicht rückwirkende Abnahme. |
| Start-/Sitzungslogik | `/api/session` wird vor privaten Registries und WebSocket geprüft; Fehlerpfad bricht ab | Am 03.10.2026 geprüfte Verbesserung gegenüber dem älteren Anhangeinstieg. Nicht mit alter Gesamtdatei zurücksetzen. |
| Login | Eigenes `ui/login.html`, deutsche Dokumentensprache und deutsche Bedienbeschriftungen im gelesenen Bereich | Historischer Logo-/Raw-String-Eingriff ist nicht direkt auf den neuen Login übertragbar. |
| Integrationsanleitung | `Docs/VERSTECKTE_INTEGRATIONEN.md` vorhanden | Historischer Markdown-Wunsch am 03.10.2026 im Repository nachweisbar. |
| Wiki | `wiki/Home.md` nennt Bezug `d518c9733b6d0474021792d4883206dc42035c2d`, Stand 26.09.2026 | Wiki ist weiterentwickelt, aber der genannte Bezugsstand liegt vor dem geprüften UI-Merge. Dokumentationsabgleich sinnvoll. |

Die historische Hover-Datei startet `loadDeviceRegistry()` und `loadStationGroupRegistry()` noch vor ihrer damaligen Prüfung über `/api/system`. Die am 03.10.2026 gelesene `boot()`-Funktion prüft zunächst `/api/session`; erst nach bestätigter Autorität folgen Registry-Laden und `connect()`. Diese Änderung ist aus den Quellen belegbar, wurde hier aber nicht als Penetrations- oder vollständiger Autorisierungstest bewertet.

### 10.4 Unveränderliche Codebelege für den geprüften Snapshot

- [Wrapper und Asset-Einbindungen](https://github.com/JanHG98/netcore-tetra/blob/c4ac047e0e167f38969804f5cc4da3e7fad0b721/crates/tetra-entities/src/net_dashboard/html.rs).
- [Deutsch-only und Übersetzungsaufruf](https://github.com/JanHG98/netcore-tetra/blob/c4ac047e0e167f38969804f5cc4da3e7fad0b721/crates/tetra-entities/src/net_dashboard/ui/dashboard.html#L5272-L5292).
- [Versteckte Integrationen und Navigation](https://github.com/JanHG98/netcore-tetra/blob/c4ac047e0e167f38969804f5cc4da3e7fad0b721/crates/tetra-entities/src/net_dashboard/ui/dashboard.html#L5376-L5425).
- [Gruppenregistry und Fallback](https://github.com/JanHG98/netcore-tetra/blob/c4ac047e0e167f38969804f5cc4da3e7fad0b721/crates/tetra-entities/src/net_dashboard/ui/dashboard.html#L5866-L5920).
- [Gruppennamen, Sortierung und Überlauf](https://github.com/JanHG98/netcore-tetra/blob/c4ac047e0e167f38969804f5cc4da3e7fad0b721/crates/tetra-entities/src/net_dashboard/ui/dashboard.html#L6653-L6715).
- [Am 03.10.2026 geprüfter alter DGNA-Modalblock](https://github.com/JanHG98/netcore-tetra/blob/c4ac047e0e167f38969804f5cc4da3e7fad0b721/crates/tetra-entities/src/net_dashboard/ui/dashboard.html#L4810-L4832) und [zugehörige Funktionen](https://github.com/JanHG98/netcore-tetra/blob/c4ac047e0e167f38969804f5cc4da3e7fad0b721/crates/tetra-entities/src/net_dashboard/ui/dashboard.html#L8800-L8812).
- [Am 03.10.2026 geprüfte Session-/Boot-Reihenfolge](https://github.com/JanHG98/netcore-tetra/blob/c4ac047e0e167f38969804f5cc4da3e7fad0b721/crates/tetra-entities/src/net_dashboard/ui/dashboard.html#L10800-L10832).
- [Neue Shell einschließlich Gerätedetails](https://github.com/JanHG98/netcore-tetra/blob/c4ac047e0e167f38969804f5cc4da3e7fad0b721/crates/tetra-entities/src/net_dashboard/ui/netcore.js).
- [Am 03.10.2026 geprüfte Integrationsanleitung](https://github.com/JanHG98/netcore-tetra/blob/c4ac047e0e167f38969804f5cc4da3e7fad0b721/Docs/VERSTECKTE_INTEGRATIONEN.md), [Wiki-Startseite](https://github.com/JanHG98/netcore-tetra/blob/c4ac047e0e167f38969804f5cc4da3e7fad0b721/wiki/Home.md) und [Directory-API-Dokumentation](https://github.com/JanHG98/netcore-tetra/blob/c4ac047e0e167f38969804f5cc4da3e7fad0b721/wiki/Directory-API.md).
- [Am 03.10.2026 festgestellter Merge-Commit](https://github.com/JanHG98/netcore-tetra/commit/7137e0dd69877e1b604bf89148fd8b6b590c1a97) und [PR #59](https://github.com/JanHG98/netcore-tetra/pull/59).

## 11. Verworfene, ersetzte und nicht bestätigte Ansätze

Die vollständige Entfernung der vier Integrationen wurde zugunsten eines versteckten UI-Modus nicht verfolgt. Ein Mehrsprachenbetrieb wurde zugunsten von Deutsch-only ersetzt. Die Wahl zwischen Übersetzen des englischen Blocks und Verwendung des deutschen Blocks endete in direktem Zugriff auf den deutschen Sprachbestand; es besteht kein Auftrag, dauerhaft zwei parallele Sprachblöcke zu pflegen.

Die ausschließlich numerische GSSI-Eingabe sollte durch eine zusätzliche Auswahlhilfe ergänzt, nicht ersatzlos abgeschafft werden. Die frühere Kombination aus `+N zugeordnet` und gleichzeitig vollständiger Gruppenliste wurde durch einen echten Überlaufzähler ersetzt.

Die Aussage, ausbleibende Änderungen seien sicher ein Cache-/Binaryproblem, ist als Diagnose dieses Vorhabens überholt. Eine Prüfung der tatsächlichen Dateiinhalte hat Vorrang. Wiederholte Komplettdateiauslieferungen ohne Funktionsvergleich waren keine zuverlässige Fortsetzungsstrategie; die beiden erhaltenen Endstände belegen das Risiko verlorener vorheriger Änderungen.

Die historischen ZIPs und Patches sind am 03.10.2026 Belege und gegebenenfalls Quellen einzelner Funktionen. Sie sind keine freigegebenen Gesamtersatzstände für die aktuelle Basisstation oder das aktuelle Wiki.

## 12. Offene Aufgaben und Roadmap-Kandidaten

Die Historie enthält nach „klappt“ keine weitere explizite priorisierte Implementierungszusage. Die folgende Priorisierung ist eine **Empfehlung aus der Archivprüfung**, keine automatisch genehmigte Produktänderung. Sie bleibt ausschließlich in dieser Zusammenfassung.

| Kandidat | Status | Empfohlene Reihenfolge / Abhängigkeit | Konkretes Abnahmekriterium |
|---|---|---|---|
| DGNA-Dropdown wieder in die geprüfte `ui/dashboard.html` integrieren und Hover erhalten | Historisch beschlossen und einzeln bestätigt; am 03.10.2026 Regression | Zuerst. Aktuelle Asset-Struktur und Sitzungsprüfung beibehalten. | Dropdown, manuelle Eingabe und Gruppennamen-/Überlaufanzeige funktionieren nach demselben Build gleichzeitig. |
| Gemeinsame Gruppen-Namensauflösung für Tabelle und DGNA | Idee aus der Fehleranalyse | Mit der Regression zusammen prüfen; normalisierte Directory-Daten und Fehlerverhalten abstimmen. | Ein Datenvertrag, konsistente IDs/Labels, keine unabhängig auseinanderlaufenden Normalizer. |
| Aktuelle DGNA-Gruppen namentlich anzeigen | Historisch behauptet, nicht im erhaltenen Code nachgewiesen | Beim Dialogabgleich mit aufnehmen. | Aktuelle Gruppen zeigen Name und nachvollziehbare GSSI; Auswahl übernimmt weiterhin die richtige Nummer. |
| Fehlende deutsche Texte und Attribute bereinigen | Bestehende Anforderung, teilweise umgesetzt | Nach/mit Funktionszusammenführung. Dynamische Texte, Placeholder, Titel und ARIA prüfen. | Kein englischer Bedien-/Fehlertext in den betroffenen geprüften Ansichten, ausgenommen bewusst beibehaltene Fach-/Produktnamen. |
| Regressionstests für Dateiauslieferungen | Neue Empfehlung | Vor der nächsten vollständigen UI-Lieferung. | Inhalt, beide gewünschten Funktionen, Syntax und tatsächliches Buildartefakt werden gemeinsam überprüft. |
| Build-/Deployment-Nachweis verbessern | Neue Empfehlung aus wiederholten Fehldiagnosen | An den nächsten Rollout koppeln. | Repo-Commit, Quelldiff, erfolgreiche Buildausgabe, Binarypfad/-Hash und beobachtete UI sind zugeordnet. |
| Hover für Touch/Tastatur erweitern | Idee / Zugänglichkeitskandidat | Nach Mausfunktion; echtes Popover oder fokussierbarer Klickzugriff prüfen. | Versteckte Gruppen ohne Maus erreichbar, Liste bleibt lesbar und wird nicht abgeschnitten. |
| Gruppenbuch-Refresh und Fehlermeldungen | Idee | Gemeinsame Registry voraussetzen; Backendlast beachten. | Umbenennungen werden gezielt aktualisiert; Fehler verursachen keinen RF-Ausfall und bleiben diagnostizierbar. |
| Namen auch in neuer Gerätedetailansicht | Neuer Konsistenzkandidat | Nachgelagert; neue Shell separat berücksichtigen. | Details und Tabelle lösen dieselbe GSSI gleich auf. |
| Wiki gegen neue UI-Struktur und Rollen-/Sitzungslogik abgleichen | Neuer Dokumentationskandidat | Aktuellen Produktstand statt alten ZIP-Gesamtersatz verwenden. | Pfade, Buildhinweise und sichtbare Funktionen passen zum geprüften Commit; historische Grenzen bleiben markiert. |
| Wiki-Altdateien kontrolliert bereinigen | Historisches Bereinigungsziel; Patchgrenze neu belegt | Nur nach vollständigem Verzeichnisvergleich. | Keine Dubletten mit Unicode-Bindestrichen, ohne fremde Control-Room-Dokumentation ungeprüft zu löschen. |
| Logoabstand in tatsächlicher neuer Loginansicht prüfen | Alter Einzelwunsch, Ausführung unbestätigt | Visuelle Prüfung der geprüften Oberfläche; keine blinde alte CSS-Änderung. | Wortmarke entspricht gewünschter Darstellung ohne ungewollten Zusatzabstand. |

Kein Kandidat dieser Tabelle wurde durch den Dokumentation im Produktcode umgesetzt. Eine automatisierte Roadmap-, Wiki- oder Statusaufgabe wurde in dieser Entwicklungsphaseabschnitt nicht neu angelegt oder geändert.

## 13. Konkreter Fortsetzungspunkt

Für die nächste Entwicklung ist der aktuelle `ui/`-Stand die Basis. Aus `html_DGNA_Gruppenbuch_FINAL.rs` nur den funktionierenden Dialog und die zugehörigen Auswahl-/Ladefunktionen als Referenz heranziehen. Die bereits vorhandene Namens-/Hover-Anzeige aus dem geprüften Dashboard nicht überschreiben. Die geprüfte Prüfung von `/api/session` muss vor privaten Registries und WebSocket erhalten bleiben.

Dann in einem gemeinsamen Testlauf mindestens prüfen: Funkgerät mit keiner, einer, zwei und mehreren Gruppen; ausgewählte Gruppe; unbekannte GSSI; doppelter Eintrag; Gruppenname mit Sonderzeichen; erfolgreicher und fehlgeschlagener Gruppenbuchabruf; DGNA-Auswahl, manuelle Eingabe und Entfernen; erneutes Öffnen des Dialogs für ein anderes Gerät; Darstellungsfunktion nach Neustart. Empfang und Übernahme einer DGNA am realen Funkgerät separat protokollieren.

Vor dem anschließenden Rollout die Produktänderungen in einem ausdrücklich dafür bestimmten Entwicklungsbranch speichern. `Archiving` wird durch die Dokumentation nicht zum Deploymentbranch. Historische Gesamt-ZIPs nicht über den aktuellen Quellbaum kopieren.

## 14. Quellen- und Anhangsverzeichnis

### 14.1 Maßgebliche Rust-Artefakte

Alle folgenden Hashes wurden bei der Quellenprüfung vom 03.10.2026 aus den tatsächlich vorhandenen Bytes neu berechnet. Eine SHA-256 ist ein Dateihash und keine Git-Commitnummer.

| Datei | Bytes | SHA-256 | Einordnung |
|---|---:|---|---|
| `html.rs` | 563587 | `130b1e4da4ab838bbbe5d1e7ac5720762204ff178d01a7cb3414d8cfe77e0210` | Am 03.10.2026 gemounteter alter Stand ohne die beiden Erweiterungen. |
| `html(1).rs` | 563587 | `130b1e4da4ab838bbbe5d1e7ac5720762204ff178d01a7cb3414d8cfe77e0210` | Bytegleich zu obiger Datei. |
| `html(3).rs` | 603729 | `302ae4c5b93b72ed05885694133a9f2d7a3c0c9f962b58a2b7441d17a88bf683` | Weiterer historischer Stand; gleiches HTML auch in `flowstation-main(4).zip`. |
| `html_DGNA_Gruppenbuch_FINAL.rs` | 567960 | `ac866de4617ef352f61ee98182874979413dfc82e1d3af890bf2e694e40ebaec` | Dropdown implementiert, historische Bestätigung „jetzt.“. |
| `html_Gruppennamen_Hover_FINAL.rs` | 565071 | `72d38b4afb7c12458a1ea0f7001edf4a66d0ed92f92d68f25ec70c6effd727e8` | Gruppennamen/Hover implementiert, historische Bestätigung „klappt“; Dropdown fehlt. |

Frühere im Verlauf genannte Hashes für andere gleichnamige `html.rs`-Lieferungen werden nicht als am 03.10.2026 vorhandene Versionskette ausgegeben. Insbesondere ist die damals genannte Gruppennamen-SHA `c4026d...` nicht der Hash des am 03.10.2026 vorhandenen `html.rs`. Ohne die vollständigen früheren Bytes bleibt die genaue Zuordnung offen.

### 14.2 Pakete, Patches und Anleitungen

Die folgenden sechs ZIPs waren lesbar und wurden auf Archivintegrität geprüft:

```text
flowstation-main(3).zip
flowstation-main(4).zip
flowstation-dashboard-de-hidden-integrations.zip
flowstation-main-dashboard-de-hidden-integrations.zip
netcore-wiki-aktuell-v1.3.0.zip
netcore-basisstation-mit-aktuellem-wiki-v1.3.0.zip
```

Weitere vorhandene Artefakte:

```text
dashboard_hidden_integrations.patch
DASHBOARD_DE_HIDDEN_INTEGRATIONS_APPLY.md
VERSTECKTE_INTEGRATIONEN.md
netcore-wiki-aktuell-v1.3.0.patch
WIKI_UPDATE_REPORT.md
```

Das eigenständige Wiki-ZIP hat SHA-256 `f04a140642c025b1cce371237a32b8862c6b374f0442ebcf4533e9b897faf2d6`; das vollständige Repository-ZIP mit dem neuen Wiki `0369143232fd816bc5f950489d680c74657cc9c399ca3b963a2c486b87bdbe9e`.

Die Notizen bewahren Namen, Befunde und Hashes der Anhänge. Für die Fortsetzung sind die beiden separat benannten Rust-Enddateien wichtig; ein Hash ersetzt den Originalanhang nicht.

### 14.3 Screenshots

Die Screenshots dokumentieren das englische Lesbarkeitsmenü mit Sprachauswahl, die englische Funk-/Zellübersicht, die Wortmarke mit sichtbarem Abstand, mehrfach den alten DGNA-Dialog ohne Dropdown sowie das numerische Gruppenbadge mit „+2 zugeordnet“. Mehrere erneut gesendete DGNA-Screenshots zeigen denselben unveränderten Zustand.

Es ist kein separater Screenshot nach „klappt“ vorhanden, der die gemeinsame Funktion von Dropdown und Hover nachweist. Die Betriebsrückmeldung bleibt dennoch als Bestätigung der zuletzt angefragten sichtbaren Änderung erhalten.

### 14.4 Zusätzlich bereitgestellte ETSI-Projektquellen

Folgende 25 PDF-Dateien sind im zugänglichen Projekt-/Anhangskontext vorhanden. Sie dienen hier der Quelleninventur, nicht einer neu vorgenommenen vollständigen Normauswertung:

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

Die bereitgestellten Titelseiten kennzeichnen unter anderem die Versionen `EN 300 392-12-16 V1.4.0 (2026-03)` und `EN 300 392-3-15 V1.5.0 (2026-04)` als Entwürfe. Die Versionsbezeichnung eines Dateinamens ist nicht allein maßgeblich; beispielsweise nennt die Titelseite des Include-Call-Dokuments V1.1.2. Der aktuelle externe Normstatus wurde für diesen reinen Dokumentation nicht neu recherchiert. Daraus wird keine aktuelle Normgeltung abgeleitet.

## 16. Abschlussstand für spätere Fortsetzung

Der historische Entwicklungsstand reicht bis zur positiven Betriebsrückmeldung der Gruppennamen-/Hover-Anzeige. Als belastbar erhalten gelten die konkrete Anforderung, zwei einzeln funktionierende UI-Artefakte, das Wiki-Paket, die separate Integrationsanleitung und die geprüfte Wiederauffindbarkeit der Namensanzeige im Repository.

Nicht abgeschlossen ist die zuverlässige Zusammenführung des DGNA-Dropdowns mit dem späteren Gruppen-Rendering. Außerdem bleiben Restlokalisierung, gezielte Zugänglichkeitsprüfung, Aktualität der Gruppendaten und die Zuordnung eines künftigen gemeinsamen Builds zum Zielhost offen. Die spätere Entwicklung muss vom aktuellen `ui/`-Stand ausgehen und die gefundenen Regressionen beheben, statt die bereits widerlegte Cache-/Binarydiagnose zu wiederholen.
