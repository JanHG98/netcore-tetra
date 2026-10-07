# Brainstorming: Event-Funkplan als Word-Formular

**Stand der Notizen und ergänzenden Prüfungen: 2026-10-04.** Historische Entwürfe, nachgewiesene Umsetzung und ausgeführte Tests sind jeweils getrennt gekennzeichnet.

> **Artefaktstand:** Ein einfacher DOCX-Entwurf wurde erzeugt und unverändert gesichert. Das historische Downloadpaket enthält Dokumentation, Index-Snapshot und Word-Original; der damalige Git-Import war noch offen.

>
> **Ziel:** Professionelle technologieübergreifende Event-Funkplanung mit Gruppen-, Bereichs- und Personenzuordnung. Der vorhandene DOCX-Entwurf benötigt noch Logo, professionelles Layout, gegebenenfalls Formularsteuerelemente sowie Word-/Druck- und Eventabnahme.

## 1. Metadaten

| Merkmal | Festgehaltener Wert |
|---|---|
| Projekt | NetCore-Tetra |
| Thema dieser Zusammenfassung | Event-Funkplan-Formular für mehrere Funkarten sowie Gruppen-, Bereichs- und Personengruppenzuordnung; Ausgabe als Word-Datei |
| Historischer Dokumentzeitpunkt | `2025-12-01T20:59:35.359080+00:00`, laut Metadaten des erzeugten Originalartefakts; ergänzend dem 01.12.2025 zugeordnet. |
| Erstellungsdatum dieser Zusammenfassung | **2026-10-04** |
| Sprache | Deutsch |
| Repository | `JanHG98/netcore-tetra` |
| Repository-Branch des Abgleichs | `Archiving` |
| Geprüfter Repository-Commit | `555b49fd20d8dd841a104d981eb2fa7789d6c069` |
| Commitzeit des geprüften Standes | `2026-10-04T00:45:03Z`, laut Branch-Metadaten |
| Commitbetreff des Prüfstands | `docs(archive): document RTW status sync, correct ISSI/OPTA claims and audit existing HMD path` — technischer Ausgangsstand, kein Implementierungsnachweis des Formulars. |
| Gelesener Index | `Docs/archive/README.md`, Git-Blob `9306a255e23e8fab38e42ff5cd158b3e5a10bf14` |
| Vorgesehene Archivdatei | `Docs/archive/2026-10-04_event-funkplan-formular-word-und-gruppenzuordnung.md` |
| Zugehörige Anlagen | `Docs/archive/assets/2026-10-04_event-funkplan-formular-word-und-gruppenzuordnung/` |

Die DOCX-Werte `created`/`modified` vom 23.12.2013 stammen aus der Vorlage und datieren das Eventformular nicht. Maßgeblich ist die zeitliche Zuordnung des Originalartefakts.

## 2. Quellenbasis, Zugänglichkeit und Nachweisstufen

Geprüft wurden die vorhandene `Event_Funkplan_Formular.docx`, vollständiger Text und OOXML-Struktur. Am 2026-10-04 wurde die Originaldatei zur Layoutkontrolle gerendert; der Repository-Abgleich erfolgte separat am bezeichneten Commit.

| Quelle | Umfang und Grenze |
|---|---|
| Historische Planung | Formularanforderung, bereinigter Tabellenentwurf, DOCX-Auswahl, ausgeführter Erzeugungscode und Word-Download. |
| Word-Anhang | Vollständig als Binärdatei verfügbar, mit `python-docx` lesbar; sieben Tabellen sowie Absatztexte und Paketstruktur geprüft. Im Archivpaket unverändert erhalten. |
| Ergänzende zeitliche Einordnung | Metadaten zum Originalartefakt; keine zusätzlichen fachlichen Festlegungen gefunden. |
| Projekt-PDFs | 25 Quellen mit `source_kind=project` inventarisiert; keine historische Entscheidungsgrundlage des Formulars und nicht vollständig ausgewertet. |
| Historische Bilder | Keine eigenständigen Bilder und kein originales NetCore-Logo im Dateienbestand. |
| GitHub | Branch, Archivindex, ausgewählte Dokumentationen und ein Quelltext-Einstiegspunkt lesend geprüft. Kein vollständiger Build-, Quelltext-, Deployment- oder Sicherheitsaudit. |

Die Nachweisstufen werden in diesem Archiv wie folgt verwendet:

- **Idee:** angeboten oder diskutiert, ohne ausdrückliche Auswahl beziehungsweise Umsetzungsnachweis.
- **Beschlossen/geplant:** ausdrücklich gewünschter Inhalt oder ausgewähltes Ausgabeformat. Das belegt noch keine vollständige Umsetzung.
- **Implementiert:** ein konkretes Artefakt oder eine konkrete Codefunktion ist nachgewiesen. Ein erzeugtes Dokument ist keine Implementierung eines Backend-Dienstes.
- **Getestet:** ein benannter Test wurde ausgeführt und sein Ergebnis ist vorhanden. Historische und ergänzende Prüfungen werden getrennt.
- **Im Betrieb bestätigt:** reale Nutzung mit belastbarer Rückmeldung. Für das Eventformular liegt eine solche Bestätigung nicht vor.

## 3. Ziel, Ausgangslage und Planungsverlauf

### 3.1 Ursprüngliches Ziel

Ziel ist ein **professionelles Formular zur Funkplanung einer Veranstaltung**. Es soll mehrere Funkarten, darunter **TETRA, DMR und PMR**, berücksichtigen und folgende Zuordnungen darstellen:

1. welche Funkgruppen beziehungsweise Kanäle vorgesehen sind;
2. wo beziehungsweise auf welchem System oder in welchem Bereich diese genutzt werden;
3. welche Personengruppen sie verwenden, beispielsweise **Orga** und **Medic**.

Ein **NetCore-Logo** war ausdrücklich erwünscht („gerne auch“), aber kein bereitgestelltes Logoasset und keine exakte Corporate-Design-Vorgabe vorhanden. Die Vorlage sollte nicht ausschließlich ein TETRA-spezifisches Konfigurationsblatt sein.

Es wurden kein konkretes Event, keine tatsächlich zu verwendenden Frequenzen, keine Gruppen-IDs und keine Teilnehmerliste übergeben. Der Auftrag war eine **wiederverwendbare Planungsunterlage**, kein ausgefüllter Einsatzfunkplan und keine Funkgeräteprogrammierung.

### 3.2 Chronologie und ausdrückliche Korrekturen

| Schritt | Tatsächlicher Verlauf | Bedeutung für die Fortsetzung |
|---|---|---|
| 1 | Technologieübergreifendes Event-Funkformular mit Personengruppen. | Maßgebliches Ziel. |
| 2 | Früher Themenabzweig zu „9.4 VPN-Integration“. | Sachfremd; verworfen. |
| 3 | Rückkehr zur Formularplanung. | VPN bleibt außerhalb des Umfangs. |
| 4 | Annahme eines vorgegebenen VPN-Kapitelanfangs. | Unbelegt und zurückgezogen. |
| 5 | Formularanforderung beibehalten. | Fachliches Ziel unverändert. |
| 6 | Textentwurf mit Eventdaten und sieben Inhaltsabschnitten. | Fachlicher Entwurf; Tabellen unten erhalten. |
| 7 | Word als Ausgabeformat wählen. | DOCX festgelegt. |
| 8 | Python-Code mit `from docx import Document` wird ausgeführt und speichert die Datei. | Tatsächlich erzeugtes lokales Artefakt, nicht bloß ein Angebot. |
| 9 | DOCX-Auslieferung; weitere Ausbauoptionen. | Zusatzoptionen weder beauftragt noch umgesetzt nachgewiesen. |

**Abgrenzung:** VPN-/Pitch-Kapitel 9.4 und dessen WireGuard-, OpenVPN-, IPsec-, Lighthouse-, Shadow-GSSI-, App-Tunnel-, VLAN-, Failover- und Provisionierungsannahmen gehören nicht zur Formularplanung.

## 4. Anforderungen und Festlegungen

| Anforderung / Entscheidung | Herkunft | Historischer Abschlussstand |
|---|---|---|
| Event-Funkplanung als Formular | Arbeitsauftrag, später wiederholt | **Beschlossen/geplant**; einfacher Entwurf **implementiert** |
| Professionelles Erscheinungsbild | Arbeitsauftrag | **Beschlossen/geplant**; durch vorhandene Datei noch nicht ausreichend eingelöst |
| Mehrere Funkarten einschließlich TETRA, DMR und PMR | Arbeitsauftrag | Als Feld-/Zeilenstruktur **implementiert**, keine technische Interoperabilität damit nachgewiesen |
| Gruppen, deren Zuordnung und nutzende Personengruppen | Arbeitsauftrag | Tabellenüberschriften **implementiert**, reale Zuweisungen offen |
| Orga und Medic als Beispiele | Arbeitsauftrag | Im Textentwurf berücksichtigt, in DOCX nicht als vorbefüllte Personenzeilen enthalten |
| NetCore-Logo | Anforderung | **Geplant/Wunsch**, aber nur Textplatzhalter **implementiert** |
| Word als editierbare Datei | Explizite Formatauswahl | DOCX **implementiert**, Erstellung erfolgreich; Word-/Druckabnahme fehlt |
| PDF, ausfüllbares PDF, Webformular | Ausbauvorschläge | **Idee**, nicht ausgewählt |
| Automatische Formularlogik / echte ausfüllbare Felder | Ausbauvorschläge nach Auslieferung | **Idee**, nicht umgesetzt |
| Konkrete Farbwelt / CI / Logoauswahl | Keine endgültige Festlegung | Offen |

Word ist als editierbares Format festgelegt; einzelne Spalten, Beispiele und Designentscheidungen sind dadurch noch nicht abgenommen. Professionelles Endlayout und Eventbetrieb wurden nicht bestätigt.

## 5. Vollständiger fachlicher Formularentwurf aus der Planung

Dieser Abschnitt bewahrt die **historisch vorgeschlagenen Inhalte**. Er beschreibt nicht automatisch den tatsächlichen Befüllungsgrad der DOCX. Die Unterschiede stehen in Abschnitt 7.

### 5.1 Dokumentkopf und Eventdaten

Vorgesehen waren der Titel **„EVENT FUNKPLAN“**, ein NetCore-Tetra-Logo beziehungsweise dessen Platzhalter sowie diese Felder:

| Feld | Vorgesehene Verwendung |
|---|---|
| Eventname | Bezeichnung der Veranstaltung |
| Datum | Veranstaltungsdatum |
| Veranstaltungsort | Ort / Gelände |
| Einsatzleitung / Funkkoordination | Verantwortliche Stelle oder Person |
| Version | Versionsstand des Plans; nur im Entwurfentwurf vorhanden |

Die Logoposition wurde nicht abschließend festgelegt; erwähnt wurden oben links oder mittig. In der erzeugten Word-Datei steht der Platzhalter unmittelbar unter dem Titel.

### 5.2 Abschnitt 1 — Funktechnologien & Infrastruktur

**Spalten:** `Funkart` · `Ja/Nein` · `Frequenzen / Kanäle` · `Bemerkungen`.

| Vorgeschlagene Zeile | Vorbelegung / Zusatz |
|---|---|
| TETRA | Auswahl und freie technische Angaben |
| DMR | Auswahl und freie technische Angaben |
| PMR/LPD | Gemeinsame Bezeichnung im historischen Vorschlag |
| Analog FM | Zusätzliche Funkart im Entwurf |
| NetCore-Tetra Node | `Standort:` unter Frequenzen/Kanäle, `GSSI:` unter Bemerkungen |
| Repeater / Relais | Infrastrukturzeile |

Der Textentwurf enthält Auswahlkästchen, die DOCX dagegen leere Ja/Nein-Zellen. Frequenzen und Kanalnummern sind nicht festgelegt.

**Klärungspunkt vom 2026-10-04:** Funkarten und Infrastrukturkomponenten sind in einer Liste vermischt. `GSSI:` in der Node-Zeile ist missverständlich, solange der Bezug zu zugeordneten Gruppen fehlt. Dies sind zusätzliche Review-Befunde.

### 5.3 Abschnitt 2 — Funkgruppen / Talkgroups / Channels

**Spalten:** `Gruppe / Kanal` · `Funkart` · `Zweck` · `Priorität` · `Bereich / Zone` · `Bemerkungen`.

Im Textentwurf standen **vier leere Planungszeilen**. Als Funkart-Auswahl war `TETRA / DMR / PMR` dargestellt, für Priorität `niedrig / normal / hoch`. Der Begleittext schlug vor, bei TETRA auch GSSIs einzutragen. Ein separates GSSI-Feld und eine technische Zuordnung der Prioritätsbezeichnungen wurden nicht spezifiziert.

Offen blieb, ob „Bereich / Zone“ eine räumliche Veranstaltungszone, eine Programmierzone des Funkgeräts oder beide Bedeutungen abdecken soll. Ebenso wurde „wo laufen die Gruppen“ nicht bis zu konkreten Netz-, Standort- oder Repeaterzuordnungen ausgearbeitet.

### 5.4 Abschnitt 3 — Einsatzbereiche / Sektoren

**Spalten im Textentwurf:** `Sektor` · `Bereich / Beschreibung` · `Verantwortliche` · `Funkkanal(e)` · `Bemerkungen`.

Vorgeschlagene vorbefüllte Sektorzeilen: **Nord, Süd, West, Ost, Innenbereich, Außenbereich**. Die übrigen Felder sollten frei ausgefüllt werden. Diese Sektornamen sind Beispiele aus dem Entwurf, keine verbindliche Geländeeinteilung eines tatsächlichen Events.

### 5.5 Abschnitt 4 — Personengruppen & Funkzuweisung

**Spalten:** `Personengruppe` · `Aufgaben` · `Primärgruppe` · `Sekundärgruppe` · `Gerätetyp` · `Anmerkung`.

| Vorgeschlagene Personengruppe | Herkunft / Einordnung |
|---|---|
| Orga / Leitung | Konkretisiert das Ausgangsbeispiel Orga |
| Security | Zusätzlicher Entwurfsvorschlag |
| Sanitäter / Medic | Konkretisiert das Ausgangsbeispiel Medic |
| Technik / Aufbau | Zusätzlicher Entwurfsvorschlag |
| Parkplatz / Verkehr | Zusätzlicher Entwurfsvorschlag |
| Künstlerbetreuung | Zusätzlicher Entwurfsvorschlag |

Für Gerätetyp wurden `TETRA / DMR / PMR` als Auswahlbeispiele gezeigt. Die Unterscheidung zwischen Technologie und konkretem Gerätemodell wurde im Entwurf nicht weiter definiert. Primär- und Sekundärgruppe sind Planungsfelder; kein Scan-, Prioritäts- oder automatisches Umschaltverhalten wurde festgelegt.

### 5.6 Abschnitt 5 — Geräteverwaltung

**Spalten:** `Gerät` · `Typ` · `Rufname / ID` · `Nutzer` · `Gruppe` · `Ausgabezeit` · `Rückgabe`.

Der Textentwurf enthielt **Gerät #1 / TETRA**, **Gerät #2 / DMR** und **Gerät #3 / PMR** als Beispielzeilen. Es wurden keine realen Inventarnummern, Seriennummern, ISSIs, Rufnamen oder Nutzerdaten erfasst. Eine Verbindung zu einem vorhandenen Asset-System wurde damals weder beschrieben noch programmiert.

### 5.7 Abschnitt 6 — Notfall- & Reservefunk

**Spalten:** `Szenario` · `Ersatzgruppe` · `Ersatzgerät` · `Fallback-Verfahren` · `Aktivierungsbefehl`.

Vorgeschlagene Ausfallszenarien: **Hauptgruppe**, **TETRA** und **Repeater**. Die Spalten sind noch offen. „Aktivierungsbefehl“ ist ein Formularfeld, kein implementiertes Kommando. Notfallfunkregelung und Rückfallebene sind nicht freigegeben oder getestet.

### 5.8 Abschnitt 7 — Bemerkungen & Sonderregeln

Freier Notizbereich für veranstaltungsspezifische Regeln. Es wurden keine verbindlichen Sonderregeln, Funkrufverfahren oder Notruftexte vorgegeben.

## 6. Architektur, Komponenten und Abhängigkeiten

### 6.1 Tatsächlich umgesetzter Dokumentpfad

```text
Formularanforderung im Chat
    -> textueller Tabellenentwurf
    -> Python-Code mit python-docx
    -> Event_Funkplan_Formular.docx
    -> Download im Chat
```

Die vorhandene Lösung ist ein **statisches, editierbares Office-Dokument**. Sie umfasst Absätze und Tabellen. Es gibt keine Datenbank, keine API-Anbindung, keinen importierten Gerätebestand, keine automatische Gruppenvalidierung, keine Synchronisation zur TBS und keine automatische Funkgeräteprogrammierung.

| Komponente | Nachweis |
|---|---|
| Python | Sichtbarer und erfolgreich ausgeführter Erzeugungscode |
| `python-docx` / Import `docx` | `from docx import Document` im historischen Code |
| DOCX / OOXML | Reale ZIP-basierte Dokumentdatei mit lesbarer `word/document.xml` |
| Office-Anwendung | Zur Bearbeitung vorgesehen; historische Nutzung in Microsoft Word nicht bestätigt |
| NetCore-Logoasset | Nicht vorhanden / nicht eingebettet |
| NetCore-Backend | Keine Verbindung des Dokuments nachgewiesen |

Historische Python- und Bibliotheksversionen wurden nicht protokolliert. Die am 2026-10-04 verwendete Prüfversion wird separat im JSON-Prüfbericht erfasst und darf nicht rückwirkend als Erzeugungsversion gelten.

### 6.2 Logische Beziehungen der Vorlage

Aus der vorgeschlagenen Struktur ergibt sich diese manuelle Zuordnung:

```text
Event -> Funktechnologien / Infrastruktur
Event -> Funkgruppen / Kanäle -> Zweck und Bereich
Event -> Sektoren -> dort verwendete Funkkanäle
Event -> Personengruppen -> Primär- und Sekundärgruppe
Event -> Geräteausgabe -> Nutzer / Gruppe / Ausgabe / Rückgabe
Event -> Ausfallszenario -> Ersatzgruppe / Ersatzgerät / Verfahren
```

Das ist eine Beschreibung des Formularmodells, **kein implementiertes Datenbankschema**. Beziehungen werden durch manuelles Eintragen hergestellt. Eindeutige Schlüssel, Pflichtfelder, Mehrfachzuordnungen und Konsistenzregeln wurden nicht definiert.

### 6.3 Bewusste technische Abgrenzung

Mehrere Funkarten in einem Formular bedeuten **nicht**, dass TETRA, DMR und PMR miteinander gekoppelt wurden. Es existiert hier kein Gateway- oder Sprachbrückenauftrag. Ebenso sind die Rollen Orga/Medic keine in diesem Planungsstand implementierte zentrale RBAC. Ein Dokumenteintrag setzt keine Mitgliedschaft oder Berechtigung im Funknetz.

## 7. Erzeugtes Word-Artefakt und Soll-Ist-Abgleich

### 7.1 Identität und unveränderte Sicherung

[Word-Original: Event_Funkplan_Formular.docx](assets/2026-10-04_event-funkplan-formular-word-und-gruppenzuordnung/Event_Funkplan_Formular.docx)

| Parameter | Geprüfter Wert |
|---|---|
| Ursprünglicher Laufzeitpfad | `/mnt/data/Event_Funkplan_Formular.docx` |
| Dateigröße | **37.579 Bytes** |
| SHA-256 | `52de38d92a191861e4a806c3a52c5e175d267dcbe10110be61c8ccefb794e2a6` |
| Abschnittszahl | 1 |
| Absätze außerhalb der Tabellen | 11 |
| Tabellen | 7 |
| Eingebettete Mediendateien | 0 |
| Inhaltssteuerelemente `w:sdt` | 0 |
| Legacy-Formularfelder `w:ffData` | 0 |
| Word-Feldanweisungen `w:instrText` | 0 |
| Explizite wiederholte Tabellenkopfzeilen `w:tblHeader` | 0 |
| Explizite Seitenumbrüche | 0 |
| Tabellenstil | `Normal Table` bei allen sieben Tabellen |
| Papiergröße | 215,9 × 279,4 mm, also Letter; A4 wurde nicht eingestellt. |
| Ränder links/rechts | jeweils 31,75 mm |
| Ränder oben/unten | jeweils 25,4 mm |
| Heutiges Prüfrendering | 2 Seiten; keine Garantie identischer Paginierung in jeder Word-Installation |

Das Original wurde für diese Archivierung **nicht repariert, umgestaltet oder neu generiert**. Die dokumentierten Mängel bleiben damit nachvollziehbar. Die Datei ist ein historisches Artefakt und keine neu freigegebene, druckfertige Vorlage.

### 7.2 Tatsächlicher Tabellenaufbau

Zeilenzahlen schließen eine vorhandene Kopfzeile ein; die Eventdatentabelle hat keine separate Kopfzeile.

| Nr. | Tabelle | Zeilen × Spalten | Tatsächliche Datenzeilen |
|---|---|---|---|
| 1 | Eventdaten | 4 × 2 | Vier Feldnamen mit Unterstrichlinien |
| 2 | Funktechnologien & Infrastruktur | 12 × 4 | Kopfzeile, **fünf vollständig leere Zeilen**, dann sechs beschriftete Technologie-/Infrastrukturzeilen |
| 3 | Funkgruppen | 2 × 6 | Kopfzeile und nur eine leere Zeile |
| 4 | Einsatzbereiche / Sektoren | 2 × 5 | Kopfzeile und nur eine leere Zeile |
| 5 | Personengruppen & Funkzuweisung | 2 × 6 | Kopfzeile und nur eine leere Zeile |
| 6 | Geräteverwaltung | 2 × 7 | Kopfzeile und nur eine leere Zeile |
| 7 | Notfall- & Reservefunk | 2 × 5 | Kopfzeile und nur eine leere Zeile |

Abschnitt 7 „Bemerkungen & Sonderregeln“ ist keine weitere Tabelle, sondern eine Überschrift mit anschließendem Absatz aus fünf Zeilenumbrüchen.

### 7.3 Fehlende oder verkürzte Umsetzung

| Textentwurf / Wunsch | Tatsächlich gelieferte DOCX |
|---|---|
| NetCore-Logo | Nur `[NETCORE-TETRA LOGO HIER]`, keine Bilddatei |
| Versionsfeld im Dokumentkopf | Fehlt |
| Auswahlkästchen und dargestellte Auswahloptionen | Keine echten Steuerelemente; leere Tabellenzellen |
| Vier Gruppenzeilen | Nur eine leere Gruppenzeile |
| Nord/Süd/West/Ost/Innen-/Außenbereich | Nicht als Zeilen vorbefüllt |
| Sechs Personengruppen einschließlich Orga und Medic | Nicht als Zeilen vorbefüllt |
| Drei Gerätebeispiele | Nicht vorbefüllt |
| Drei Ausfallszenarien | Nicht vorbefüllt |
| Hinweis auf GSSI bei Gruppen | Kein eigenes Gruppen-ID-Feld und kein entsprechender Hinweis im Dokument; `GSSI:` steht nur bei der Node-Zeile |
| Professionelles Tabellenlayout | Standardtabellen, im Prüfrendering ohne sichtbare Rasterlinien; keine abgestimmten Spaltenbreiten |

Die DOCX-Sektorspalte heißt **„Beschreibung“**, der ausführliche Textentwurf **„Bereich / Beschreibung“**. Übrige Spaltenbezeichnungen sind im Wesentlichen übernommen.

## 8. Befehle, Erzeugung und Reparaturansätze

### 8.1 Historisch erfolgreich ausgeführt

Der sichtbare Python-Toolaufruf erzeugte das Dokument tatsächlich. Seine zentralen Operationen waren:

```python
from docx import Document

doc = Document()
doc.add_heading("Event Funkplan", level=1)
doc.add_paragraph("[NETCORE-TETRA LOGO HIER]")
# Es folgen Eventdaten, Tabellen und der Notizbereich.
file_path = "/mnt/data/Event_Funkplan_Formular.docx"
doc.save(file_path)
```

Der obige Ausschnitt ist gekürzt. Der vollständig aus dem sichtbaren Code übertragene historische Generator liegt separat als [historischer-generator.py](assets/2026-10-04_event-funkplan-formular-word-und-gruppenzuordnung/historischer-generator.py) bei. Er ist **keine korrigierte neue Version** und wurde bei der Archivierung nicht erneut ausgeführt. Er würde wieder denselben festen `/mnt/data/`-Ausgabepfad verwenden.

Die erfolgreiche Dateierzeugung ist durch die damalige Toolausgabe und die am 2026-10-04 vorhandene, lesbare Datei belegt. Ein `pip install`, ein Deployment auf Jans Rechner oder eine Installation in einem LXC war nicht Bestandteil dieses historischen Ablaufs.

### 8.2 Ursache der zusätzlichen Leerzeilen

Der Generator beginnt die Technologietabelle mit sechs vorhandenen Zeilen:

```python
t1 = doc.add_table(rows=6, cols=4)
# Nur t1.rows[0] wird als Kopfzeile befüllt.
for r in rows:  # sechs Technologie-/Infrastruktureinträge
    row = t1.add_row().cells
```

Dadurch entstehen **6 + 6 = 12 Zeilen**. Die bereits angelegten Zeilen 2 bis 6 bleiben leer. Dieser Befund ist nicht nur aus dem Code abgeleitet, sondern in der OOXML-Auswertung und im geprüften Rendering bestätigt.

**Mögliche Reparatur, am 2026-10-04 nur vorgeschlagen:** Die Tabelle zunächst mit einer Kopfzeile anlegen (`rows=1`) und anschließend die sechs Einträge hinzufügen. Alternativ sieben Zeilen anlegen und diese gezielt befüllen. Keine dieser Änderungen wurde am historischen Original vorgenommen oder als reparierte Ausgabe getestet.

### 8.3 Ergänzende, tatsächlich ausgeführte Prüfungen

```bash
python /home/oai/skills/docx/render_docx.py \
  /mnt/data/Event_Funkplan_Formular.docx \
  --output_dir /mnt/data/_event_funkplan_audit/render --emit_pdf
```

**Ergebnis:** Rendering erfolgreich, zwei Seiten erzeugt und beide visuell geprüft. Die dabei entstandenen PDF-/PNG-Dateien sind interne Prüfprodukte von am 2026-10-04, keine historischen Bilder und keine zusätzlichen freigegebenen Formate.

Zusätzlich wurden `ZipFile.testzip()`, XML-Auswertungen, `python-docx`-Tabellenzählungen und SHA-256-Prüfungen ausgeführt. Das Ergebnis liegt als [docx-pruefbericht.json](assets/2026-10-04_event-funkplan-formular-word-und-gruppenzuordnung/docx-pruefbericht.json) vor.

### 8.4 Nicht anwendbare Betriebsangaben

Für diese statische DOCX wurden keine Dienste, systemd-Units, Container, Netzwerkports, Funkprotokoll-Implementierungen, Konfigurationsdateien oder Zugangsdaten eingerichtet. Die Funkarten sind Formularinhalte, keine neu installierten Softwarekomponenten. Es gibt keinen Installations- oder Reparaturbefehl für eine produktive TBS aus diesem Planungsstand.

## 9. Fehler, Diagnose und verbleibende Probleme

| Befund | Ursache / Nachweis | Lösung und Status |
|---|---|---|
| Zwei sachfremde VPN-Texte | Falscher Themenbezug; Ziel war das Eventformular. | In der bereinigten Planung ausgeschlossen. |
| Angeblich vorgegebener VPN-Kapitelanfang | Nicht belegt. | Annahme zurückgezogen. |
| Logo fehlt | Kein `add_picture()` im Generator, keine Mediendateien im DOCX. | Original-Logo und Layoutentscheidung für spätere V2 beschaffen; nicht erledigt. |
| Fünf überflüssige Leerzeilen | `rows=6` plus zusätzliche `add_row()`-Schleife. | Konkreter Reparaturvorschlag vorhanden; am Original nicht ausgeführt. |
| Tabellen zu knapp und ohne Beispiele | Folgetabellen wurden lediglich mit zwei Zeilen erzeugt. | Personengruppen, Sektoren, Geräte- und Ausfallzeilen wieder aufnehmen; offen. |
| Versionsfeld fehlt | Generator hat nur vier Eventdatenfelder angelegt. | Versions-/Freigabekonzept festlegen und ergänzen; offen. |
| Ungünstige Wortumbrüche | Nicht abgestimmte, schmale Standardspalten. | Spaltenbreiten und gegebenenfalls Seitenausrichtung neu auslegen; nicht umgesetzt. |
| Keine echten Formularfunktionen | Nur normale Absätze und Tabellenzellen. | Nur nach separater Entscheidung Inhaltssteuerelemente / Formularlogik ergänzen. |
| Historischer GitHub-Upload fehlt | Connector ohne Schreibaktion; CLI-DNS-Fehler. | Downloadpaket als damaliger Sicherungsstand vorhanden. |

Die frühere pauschale Aussage „Word-Dokument ist fertig“ belegt die Dateierstellung, **nicht** die vollständige Erfüllung der Design- und Formularanforderungen.

## 10. Tests und Ergebnisse mit Grenzen

| Test | Zeitpunkt | Ergebnis | Aussagegrenze |
|---|---|---|---|
| Python-Code ausführen und DOCX speichern | Historischer Planungsstand | Erfolgreich, Dateipfad zurückgegeben | Kein Layout-/Bediennachweis |
| Word-Datei erneut öffnen / Tabellen lesen | 2026-10-04 | Erfolgreich | Kein Test in Microsoft Word |
| ZIP-Integrität | 2026-10-04 | `testzip()` ohne fehlerhaften Eintrag | Keine vollständige OOXML-Schemavalidierung |
| Tabellen- und Feldprüfung | 2026-10-04 | 7 Tabellen, 0 Bilder, 0 Inhaltssteuerelemente, 0 Legacy-Formularfelder | Beschreibt nur das vorhandene Dokument |
| Rendering und Sichtprüfung | 2026-10-04 | 2 Seiten; Logo-Platzhalter, große Leerfläche und problematische Wortumbrüche sichtbar | LibreOffice-basiertes Rendering, keine Word-/Druckerfreigabe |
| Original gegen Archivkopie | 2026-10-04 | Gleiche SHA-256; unverändert | Gilt für die lokale Kopie, nicht für einen GitHub-Upload |
| Übernahme des vorhandenen Index | 2026-10-04 | Rekonstruierter Baseline-Index ergibt exakt Git-Blob `9306a255e23e8fab38e42ff5cd158b3e5a10bf14` | Snapshot des geprüften Commits, keine Garantie gegen spätere parallele Änderungen |
| Binärfähiger Archivpatch | 2026-10-04 | `git apply --check` und Anwendung auf isolierter Index-Baseline erfolgreich; alle sechs Zieldateien danach bytegleich | Lokaler Importtest, kein Remote-Commit; zukünftige parallele Änderungen können weiterhin Konflikte verursachen |
| Generator-Syntax und Anlagenlinks | 2026-10-04 | Python-AST lesbar; alle relativen Anlagenlinks vorhanden; vorhandene 26 Indexeinträge unverändert, genau ein neuer Eintrag | Generator nicht erneut ausgeführt; bestehende fremde Archivdateien sind nicht Bestandteil des Pakets |
| Realer Test bei einer Veranstaltung | Nicht durchgeführt / nicht berichtet | Kein Ergebnis | **Nicht im Betrieb bestätigt** |
| Funkgeräte-/Netz-/Fallbacktest | Nicht durchgeführt | Kein Ergebnis | Formular erzeugt keine funktionierende Funkinfrastruktur |
| Backend-Integration / Build / E2E | Nicht durchgeführt | Kein Ergebnis | Keine technische Integration dieses Formulars nachgewiesen |
| Commit / Push / Remote-Dateiprüfung nach Speicherung | Nicht möglich | Kein neuer Commit | Archiv ist noch nicht im Zielbranch gespeichert |

Die ergänzende Sichtprüfung zeigte insbesondere auf Seite 1 einen großen Leerraum vor den Technologieeinträgen sowie auf Seite 2 mehrzeilig auseinandergerissene Spaltenüberschriften, etwa bei Personengruppe, Primärgruppe, Sekundärgruppe und Ausgabezeit. Sichtbare Tabellenraster fehlen. Damit ist die Datei als einfacher Entwurf lesbar, aber keine belastbar fertiggestellte professionelle Formularvorlage.

## 11. Zusätzlich geprüfter Repository-Stand vom 04.10.2026

### 11.1 Prüfgrenzen und Kollisionen

Der Zielbranch wurde zunächst live gelesen. Anschließende Dateiabrufe wurden auf `555b49fd20d8dd841a104d981eb2fa7789d6c069` fixiert, um einen konsistenten Befundstand zu haben.

Am geprüften Repository-Stand war für den vorgesehenen Dokumentpfad noch kein Eintrag vorhanden: **HTTP 404**. Dies betrifft diesen Pfad, keine umfassende Dateikollisionsprüfung.

Eine rekursive Repository-Baumliste sowie die Archivverzeichnisliste wurden angefordert, ihre Tooldarstellung war jedoch gekürzt. Daraus wird **keine vollständige Abwesenheitsbehauptung** abgeleitet: Ein ähnlich benanntes Formular, eine Exportfunktion oder ein anderer Generator an irgendeinem anderen Repository-Pfad wurde nicht flächendeckend ausgeschlossen. Der geprüft fehlende Zielpfad und der vollständig gelesene Index sind die belastbaren Kollisionsbefunde.

### 11.2 Ergänzende benachbarte Komponenten — getrennt vom historischen Ergebnis

Die folgenden Komponenten sind im geprüften Repository dokumentiert und für eine **spätere, gesondert beauftragte** Digitalisierung thematisch relevant:

| Repository-Datei / Komponente | Ergebnis vom 2026-10-04 | Nachweisstufe |
|---|---|---|
| [`system-backend/README.md`](https://github.com/JanHG98/netcore-tetra/blob/555b49fd20d8dd841a104d981eb2fa7789d6c069/system-backend/README.md) | Führt eigenständige Backend-Dienste einschließlich Group Core, Provisioning Core und Asset Management auf. | Dokumentation vorhanden und gelesen; kein Live-Nachweis. |
| [`system-backend/group-core/README.md`](https://github.com/JanHG98/netcore-tetra/blob/555b49fd20d8dd841a104d981eb2fa7789d6c069/system-backend/group-core/README.md) | Beschreibt GSSI-Stammdaten, Mitgliedschaften, Affiliationen und DGNA; WebUI-Port `8110`; Datenpfad `/var/lib/netcore-group-core/groups.json` plus `.bak`; Node-Gateway-Anbindung über `/ws/backend`. | Dokumentierter möglicher Bezug für TETRA-Gruppendaten; keine Formularanbindung geprüft. |
| [`system-backend/provisioning-core/README.md`](https://github.com/JanHG98/netcore-tetra/blob/555b49fd20d8dd841a104d981eb2fa7789d6c069/system-backend/provisioning-core/README.md) | Beschreibt eine gemeinsame Geräte-/Gruppen-/Mitgliedschaftsmatrix; Standardport `8125/tcp`; nutzt Subscriber Core `8100` und Group Core `8110`, ohne diese als autoritative Dienste zu ersetzen. | Dokumentierter möglicher Bezug für Zuordnungen; kein Eventformular-Exportnachweis. |
| [`system-backend/provisioning-core/src/main.rs`](https://github.com/JanHG98/netcore-tetra/blob/555b49fd20d8dd841a104d981eb2fa7789d6c069/system-backend/provisioning-core/src/main.rs) | Konkreter Rust-Einstiegspunkt vorhanden: Module `config`, `http`, `upstream`; Binärname `netcore-provisioning-core`; `--config` mit Standard `/etc/netcore/provisioning-core.toml`; optional `--bind`; Aufruf `http::serve(config)`. | Quelltext vorhanden und statisch gelesen. Kein Build, kein Test und kein Beleg einer Word-/Eventfunktion. |
| [`system-backend/asset-management/README.md`](https://github.com/JanHG98/netcore-tetra/blob/555b49fd20d8dd841a104d981eb2fa7789d6c069/system-backend/asset-management/README.md) | Beschreibt physische Assets, Funkgeräte, Personen, Ausgaben und Wartung; WebUI-Port `8290`. Subscriber Core bleibt für ISSI-Freigaben, Mobility Core für die bedienende TBS zuständig. | Dokumentierter möglicher Bezug zur Geräteausgabe; keine Import-/Exportschnittstelle des Formulars nachgewiesen. |

Die gelesenen komponentenspezifischen READMEs beschreiben diese Stände als **Open Lab ohne Anmeldung, Token und TLS**; der gelesene Provisioning-Einstiegspunkt enthält ebenfalls eine entsprechende Startwarnung. Diese Repository-Aussagen sind keine Empfehlung, ausgefüllte Personen- oder Veranstaltungsdaten ungeschützt bereitzustellen, und keine Aussage über Jans tatsächlich installierte Systeme.

Die Ports und Pfade gehören zum Repository-Abgleich und sind keine vereinbarte Formular-Laufzeitkonfiguration. Eine DOCX benötigt diese Dienste nicht.

### 11.3 Konsequenz für eine Fortsetzung

Vor einer Softwareintegration vorhandene Zuständigkeiten für Geräte, Gruppen und Mitgliedschaften prüfen. Dies ist ein **zusätzlicher Ansatz aus dem Repository-Abgleich**. Die TETRA-nahen Komponenten belegen noch keine technologieübergreifende Eventplanung oder DMR-/PMR-Unterstützung.

## 12. Verworfene, ersetzte und nicht ausgewählte Ansätze

| Ansatz | Behandlung |
|---|---|
| VPN-/Pitch-Kapitel 9.4 | Außerhalb des Ziels; ausdrücklich ausgeschlossen. |
| Annahme eines vorgegebenen VPN-Textes | Unbelegt und zurückgezogen. |
| Reiner Textentwurf als Lieferung | Durch DOCX als gewähltes Ausgabeformat ergänzt. |
| PDF-Ausgabe und ausfüllbares PDF | Nur angeboten, nicht ausgewählt; kein historisches PDF-Artefakt vorhanden. |
| Webformular | Nur als mögliche spätere Verwendung erwähnt, kein Implementierungsauftrag. |
| Automatische Formularlogik | Späteres Angebot, keine Entscheidung oder Implementierung. |
| Behauptung vollständiger professioneller Fertigstellung | Durch den geprüften Dateibefund zu relativieren: einfacher Entwurf, wesentliche Nacharbeiten offen. |

Es wurde keine historische V2 erstellt. Der bessere Textentwurf und die reduzierte DOCX sind deshalb nicht zwei freigegebene Versionen, sondern Vorschlag und davon abweichende Erstumsetzung.

## 13. Offene Aufgaben und Roadmap-Kandidaten

Es gibt **keine historisch vereinbarten Termine, Sprintzuordnungen oder Prioritätsstufen**. Die nachfolgende Reihenfolge ist eine Empfehlung aus den Anforderungen und geprüften Befunden, nicht ein nachträglich erfundener Projektbeschluss. Keine Roadmap außerhalb von `Docs/archive/` wurde geändert.

| ID | Aufgabe | Herkunft | Status / empfohlene Reihenfolge |
|---|---|---|---|
| EFP-01 | Dokumentationspaket samt unverändertem Word-Original und Index zusammenführen. | Historisch noch offene Ablage | Sicherungs- und Importvorgang separat von der Formularumsetzung betrachten. |
| EFP-02 | Word-Vorlage so überarbeiten, dass der professionelle Formularcharakter erfüllt wird: lesbare Tabellen, ausreichend Zeilen und sinnvolle Seitenaufteilung. | Ursprünglicher Arbeitsauftrag; am 2026-10-04 sichtbare Mängel | **Geplant/unerfüllt**; vor praktischer Nutzung bearbeiten. |
| EFP-03 | Fünf zusätzliche Leerzeilen entfernen; Personengruppen, Sektoren, Gerätebeispiele und Ausfallszenarien des Textentwurfs wieder aufnehmen oder abstimmen. | Soll-Ist-Abweichung | **Reparaturkandidat**, nicht implementiert. |
| EFP-04 | Gewünschtes NetCore-Logo auswählen und als echtes Bild einbauen; Farbwelt und Logoposition abstimmen. | Ausdrücklicher Gestaltungswunsch | **Geplant/Wunsch**, Logoquelle und CI offen. |
| EFP-05 | Version sowie gegebenenfalls Planstatus/Freigabe ergänzen. | Versionsfeld aus Textentwurf; Freigabe als neuer Verbesserungsvorschlag | Version wiederherstellen; zusätzliche Freigabefelder erst abstimmen. |
| EFP-06 | Fachliche Trennung von Funkart, System/Infrastruktur, Gruppen-ID, Kanal, räumlichem Bereich und gegebenenfalls Gerätezone festlegen. | „Welche Gruppen wo laufen“; am 2026-10-04 erkannte Unschärfen | **Klärungskandidat**, noch kein endgültiges Schema. |
| EFP-07 | PMR/LPD-Bezeichnung und GSSI-Position in Node-Zeile bereinigen; Werte erst anhand des Events einsetzen. | Historischer Entwurf | **Prüfkandidat**, Original unverändert. |
| EFP-08 | Word-Inhaltssteuerelemente, Dropdowns und Auswahlkästchen auswählen. | Optionale Ausbauidee | Noch nicht entschieden; normale Editierbarkeit vorhanden. |
| EFP-09 | Realistischen Musterplan mit mehreren Funkarten und mindestens Orga/Medic ausfüllen; Word-, Layout- und Drucktest durchführen. | Geprüfter Abnahmevorschlag | **Testidee**, nicht durchgeführt. |
| EFP-10 | Erst nach eigenständiger Beauftragung eine PDF-/Web-/automatisierte Variante prüfen und vorhandene Group-/Provisioning-/Asset-Bausteine abgleichen. | Historische Optionen plus geprüfter Repository-Abgleich | **Idee**, keine aktuelle Implementierungszusage. |

### Konkreter nächster fachlicher Arbeitsstand

Für eine V2 sollte das unveränderte Original als Referenz erhalten bleiben. Auf einer getrennten Kopie wären zuerst Tabellenumfang, Logo, Papierformat, Spaltenbreiten und Versionsfeld zu bearbeiten. A4 oder Querformat wären Gestaltungsoptionen, aber keine bereits getroffene Festlegung. Anschließend sollte die Zuordnung **Personengruppe → Primär-/Sekundärgruppe → Funkart/System/Bereich** an einem Muster-Event auf Verständlichkeit geprüft werden.

Ein tatsächlich freigegebener Eventplan benötigt danach die realen veranstaltungsspezifischen Angaben. Solche Werte wurden in diesem Planungsstand nicht erhoben; es werden daher weder Frequenzen noch Gruppenkennungen oder Notfallverfahren ergänzt.

## 14. Anhänge und Bildbestand

### 14.1 Originalartefakte und Begleitdateien

| Datei | Inhalt / Behandlung |
|---|---|
| [Event_Funkplan_Formular.docx](assets/2026-10-04_event-funkplan-formular-word-und-gruppenzuordnung/Event_Funkplan_Formular.docx) | Unverändertes historisches Word-Artefakt, einschließlich bekannter Mängel. |
| [historischer-generator.py](assets/2026-10-04_event-funkplan-formular-word-und-gruppenzuordnung/historischer-generator.py) | Aus dem sichtbaren Python-Toolaufruf übertragener vollständiger Generator; nicht erneut ausgeführt, kein historisch separat heruntergeladenes Skript. |
| [docx-pruefbericht.json](assets/2026-10-04_event-funkplan-formular-word-und-gruppenzuordnung/docx-pruefbericht.json) | Ergänzende Struktur- und Layoutbefunde mit klarer Abgrenzung zum historischen Teststand. |
| [anhangsinventar.json](assets/2026-10-04_event-funkplan-formular-word-und-gruppenzuordnung/anhangsinventar.json) | Dateigrößen, Seitenzahlen, SHA-256-Werte und Einordnung der verfügbaren Anlagen. |

### 14.2 Bilder

Es wurden keine eigenständigen historischen Bilder oder Logo-Binärdateien gefunden. Auch im Word-Original sind keine Bilder enthalten. Die als Vorschau sichtbaren ETSI-Deckblätter gehören zu den Projekt-PDFs und sind keine Entwürfe des NetCore-Eventformulars.

Die bei der geprüften Dokumentprüfung erzeugten Seitenbilder werden nicht als historische Bilder ausgegeben und nicht als neues Design in dieses Archivpaket aufgenommen. Es wurde kein Ersatzlogo erfunden und kein neues Bild generiert.

### 14.3 Geteilte Projekt-PDFs

Das Dateiinventar bestätigt **25 Projekt-PDFs**, nicht 25 Anhänge des historischen Formularauftrags. Sie sind als gemeinsame Projektquellen verfügbar und wurden für dieses Archiv mit Namen, Umfang und Prüfsumme inventarisiert. Es liegt hier keine vollständige Auswertung ihrer zusammen **8.061 PDF-Seiten** vor; in dieser Summe ist die große Datei `ETSI.pdf` enthalten, sodass sie nicht als Anzahl einmaliger Normseiten zu verstehen ist.

Die PDFs sind kein zusätzlicher Bestandteil des historischen Formularpakets. Ein daraus abgeleiteter Normenentscheid oder eine TETRA-Konformitätsprüfung fehlt. Vorhandene Fassungen einschließlich Draft-Kennzeichnungen sind anhand ihrer Titelseiten inventarisiert; der neueste Normstatus wurde nicht recherchiert.

| Projektdatei | Seiten | Behandlung |
|---|---:|---|
| `ETSI.pdf` | 4100 | Inventarisiert; keine formularbezogene Normenauswertung, keine erneute Binärkopie |
| `en_30039201v010601p.pdf` | 182 | Inventarisiert; keine formularbezogene Normenauswertung, keine erneute Binärkopie |
| `en_30039202v030801p.pdf` | 1445 | Inventarisiert; keine formularbezogene Normenauswertung, keine erneute Binärkopie |
| `en_3003920303v010301p.pdf` | 251 | Inventarisiert; keine formularbezogene Normenauswertung, keine erneute Binärkopie |
| `en_3003920304v010301p.pdf` | 28 | Inventarisiert; keine formularbezogene Normenauswertung, keine erneute Binärkopie |
| `en_3003920308v010401p.pdf` | 22 | Inventarisiert; keine formularbezogene Normenauswertung, keine erneute Binärkopie |
| `en_3003920313v010201p.pdf` | 191 | Inventarisiert; keine formularbezogene Normenauswertung, keine erneute Binärkopie |
| `en_3003920315v010500a.pdf` | 380 | Inventarisiert; keine formularbezogene Normenauswertung, keine erneute Binärkopie |
| `en_30039205v020701p.pdf` | 320 | Inventarisiert; keine formularbezogene Normenauswertung, keine erneute Binärkopie |
| `en_30039207v030501p.pdf` | 216 | Inventarisiert; keine formularbezogene Normenauswertung, keine erneute Binärkopie |
| `en_30039209v010701p.pdf` | 46 | Inventarisiert; keine formularbezogene Normenauswertung, keine erneute Binärkopie |
| `en_3003921006v010401p.pdf` | 20 | Inventarisiert; keine formularbezogene Normenauswertung, keine erneute Binärkopie |
| `en_3003921018v010301p.pdf` | 17 | Inventarisiert; keine formularbezogene Normenauswertung, keine erneute Binärkopie |
| `en_3003921101v010201p.pdf` | 44 | Inventarisiert; keine formularbezogene Normenauswertung, keine erneute Binärkopie |
| `en_3003921114v010101p.pdf` | 23 | Inventarisiert; keine formularbezogene Normenauswertung, keine erneute Binärkopie |
| `en_3003921117v010102p.pdf` | 18 | Inventarisiert; keine formularbezogene Normenauswertung, keine erneute Binärkopie |
| `en_3003921201v010202p.pdf` | 56 | Inventarisiert; keine formularbezogene Normenauswertung, keine erneute Binärkopie |
| `en_3003921216v010400a.pdf` | 67 | Inventarisiert; keine formularbezogene Normenauswertung, keine erneute Binärkopie |
| `en_30039401v030301p.pdf` | 169 | Inventarisiert; keine formularbezogene Normenauswertung, keine erneute Binärkopie |
| `en_30039502v010303p.pdf` | 94 | Inventarisiert; keine formularbezogene Normenauswertung, keine erneute Binärkopie |
| `en_300812v020101p.pdf` | 156 | Inventarisiert; keine formularbezogene Normenauswertung, keine erneute Binärkopie |
| `es_20081201v020205p.pdf` | 8 | Inventarisiert; keine formularbezogene Normenauswertung, keine erneute Binärkopie |
| `es_20081202v020401m.pdf` | 139 | Inventarisiert; keine formularbezogene Normenauswertung, keine erneute Binärkopie |
| `ets_30039214e01v.pdf` | 61 | Inventarisiert; keine formularbezogene Normenauswertung, keine erneute Binärkopie |
| `ts_10081201v020205p.pdf` | 8 | Inventarisiert; keine formularbezogene Normenauswertung, keine erneute Binärkopie |

## 15. Historisches Übergabepaket und Importverfahren

### 15.1 Ablagestand vom 2026-10-04

Der Ausgangsbranch und die Quellen waren lesbar. Beim ursprünglichen Export am 2026-10-04 fehlte eine schreibende Connectoraktion; der zusätzliche Git-Zugriff scheiterte an DNS. Deshalb wurde zunächst ein lokales Übergabepaket erzeugt.

Historisch versuchter Git-Zugriff:

```bash
GIT_TERMINAL_PROMPT=0 git ls-remote --heads \
  https://github.com/JanHG98/netcore-tetra.git Archiving
```

Ergebnis, Exitstatus 128:

```text
fatal: unable to access 'https://github.com/JanHG98/netcore-tetra.git/':
Could not resolve host: github.com
```

Der DNS-Fehler betrifft die damalige Prüfumgebung, nicht das Repository oder die NetCore-Infrastruktur.

Vor der Paketfertigstellung wurde der Remote-Stand erneut gelesen: `555b49fd20d8dd841a104d981eb2fa7789d6c069`. Dies bestätigt die damalige Ausgangsrevision, keinen erfolgreichen Import.

### 15.2 Vorbereitete Dateien

```text
Docs/archive/
├── README.md                          # vorhandener Index + genau ein neuer Eintrag
├── 2026-10-04_event-funkplan-formular-word-und-gruppenzuordnung.md
└── assets/
    └── 2026-10-04_event-funkplan-formular-word-und-gruppenzuordnung/
        ├── Event_Funkplan_Formular.docx
        ├── historischer-generator.py
        ├── docx-pruefbericht.json
        └── anhangsinventar.json
```

Das Übergabepaket enthält den ursprünglichen Index mit zusätzlichem Eintrag. Die enthaltene Indexzeile lautet:

```markdown
| 2026-10-04 | Event-Funkplan: Word-Formular, mehrere Funkarten und Gruppen-/Personengruppenzuordnung | [Event-Funkplan: Formular und Word-Artefakt](2026-10-04_event-funkplan-formular-word-und-gruppenzuordnung.md) | Professionelles Layout und echtes Logo nacharbeiten; fehlende Beispielzeilen/Version ergänzen; Feldbedeutungen klären und Word-/Drucktest nachholen. Word-Original mit Prüfbericht im Paket; Git-Import noch ausstehend. |
```

Eine **binärfähige Git-Diff-Datei** enthält dieselben sechs Zielpfade einschließlich Word-Original und additiver Indexänderung. Sie ist ein Transportartefakt, kein Commitnachweis.

**Importhinweis:** Einen alten Index-Snapshot mit dem dann aktuellen Index zusammenführen. Vorhandene Einträge erhalten und Patch-Konflikte vor Anwendung prüfen.

### 15.3 Vorgeschlagener späterer Import — nicht ausgeführt

Historischer Importvorschlag für einen sauberen Checkout von `JanHG98/netcore-tetra` auf `Archiving`; bei der ursprünglichen Erstellung nicht ausgeführt:

```bash
# Nur in einem sauberen Checkout von JanHG98/netcore-tetra ausführen.
git status --short
git fetch origin Archiving
git switch Archiving
git pull --ff-only origin Archiving

# Die separat gelieferte Patchdatei zunächst nur prüfen.
git apply --check /pfad/netcore_event_funkplan_2026-10-04.patch
# Nur bei erfolgreicher Prüfung anwenden; andernfalls aktuelle Dateien vergleichen.
git apply --binary /pfad/netcore_event_funkplan_2026-10-04.patch

git diff --check
git diff --name-only

# Ausschließlich die Pfade dieses Auftrags vormerken.
git add -- Docs/archive/README.md \
  Docs/archive/2026-10-04_event-funkplan-formular-word-und-gruppenzuordnung.md \
  Docs/archive/assets/2026-10-04_event-funkplan-formular-word-und-gruppenzuordnung/
git diff --cached --name-only
git diff --cached --stat

# Nur nach Kontrolle des Inhalts und des Branches ausführen.
git commit -m "docs(archive): archive event radio plan form and original Word artifact"
git push origin HEAD:refs/heads/Archiving
```

Vor Anwendung die Dateiauswahl und Remote-Änderungen prüfen. Das Beispiel setzt ausschließlich Änderungen unter `Docs/archive/` voraus; Non-Fast-Forward-Konflikte müssen zuerst aufgelöst werden.

Vorgesehene Kontrolle nach einem Import:

```bash
git fetch origin Archiving
git rev-parse HEAD
git show origin/Archiving:Docs/archive/2026-10-04_event-funkplan-formular-word-und-gruppenzuordnung.md
git show origin/Archiving:Docs/archive/README.md
```

Für die Word-Datei ist nach Import ein erneuter Byte-/Hashvergleich vorgesehen. Historischer Exportzustand und spätere Speicherung sind anhand der Dateihistorie getrennt nachvollziehbar.

## 16. Quellen- und Prüfverzeichnis

### Historische Entwicklung und Originalartefakte

**C1:** Anforderungen an technologieübergreifende Event-Funkplanung mit Orga/Medic und NetCore-Logo.

**C2:** Ausdrücklicher Ausschluss des sachfremden VPN-Themas.

**C3:** Vollständiger Tabellenentwurf und anschließende DOCX-Festlegung.

**C4:** Ausgeführter Python-Generator mit Dateiausgabe; vollständiger Code in `historischer-generator.py`.

**A1:** Originalartefakt `Event_Funkplan_Formular.docx`, Datei-ID `file_00000000a4d8720a848d3a097a23b50a`. Metadatenquelle: `generated`, Zeitstempel `2025-12-01T20:59:35.359080+00:00`. Original, Prüfsumme und Strukturbericht sind im Paket erhalten.

**A2:** Dateiinventar der gemeinsamen Projekt-PDFs (`source_kind=project`); Liste und lokale Metadaten in `anhangsinventar.json`. Keine inhaltliche Normenfreigabe oder vollständige PDF-Auswertung behauptet.

### Repository-Quellen am festgehaltenen Commit

| Quelle | Link | Geprüfter Git-Blob, soweit geliefert |
|---|---|---|
| Geprüfter Commit | [555b49fd20d8dd841a104d981eb2fa7789d6c069](https://github.com/JanHG98/netcore-tetra/commit/555b49fd20d8dd841a104d981eb2fa7789d6c069) | Commit, kein Datei-Blob |
| Vorhandener Archivindex | [Docs/archive/README.md](https://github.com/JanHG98/netcore-tetra/blob/555b49fd20d8dd841a104d981eb2fa7789d6c069/Docs/archive/README.md) | `9306a255e23e8fab38e42ff5cd158b3e5a10bf14` |
| Root-Projektbeschreibung | [README.md](https://github.com/JanHG98/netcore-tetra/blob/555b49fd20d8dd841a104d981eb2fa7789d6c069/README.md) | `80dbc3d5f5ff94083ed1b90c4d0256d40fb126fe` |
| Backend-Übersicht | [system-backend/README.md](https://github.com/JanHG98/netcore-tetra/blob/555b49fd20d8dd841a104d981eb2fa7789d6c069/system-backend/README.md) | `42b56355a540b5082b128f3c3d302cbfb66e2734` |
| Group Core | [group-core/README.md](https://github.com/JanHG98/netcore-tetra/blob/555b49fd20d8dd841a104d981eb2fa7789d6c069/system-backend/group-core/README.md) | `68ee1fac1f81cc5c9616b7d61bf6bc9134a7a1a4` |
| Provisioning Core | [provisioning-core/README.md](https://github.com/JanHG98/netcore-tetra/blob/555b49fd20d8dd841a104d981eb2fa7789d6c069/system-backend/provisioning-core/README.md) | `f44a48815437da922091f3c820274601b569a20a` |
| Provisioning-Einstiegspunkt | [provisioning-core/src/main.rs](https://github.com/JanHG98/netcore-tetra/blob/555b49fd20d8dd841a104d981eb2fa7789d6c069/system-backend/provisioning-core/src/main.rs) | `e1c2aa25c16459f6d835c2cfb30634c7b1c2a29a` |
| Asset Management | [asset-management/README.md](https://github.com/JanHG98/netcore-tetra/blob/555b49fd20d8dd841a104d981eb2fa7789d6c069/system-backend/asset-management/README.md) | `788113db58934b0327f20ead4f55d479de9ef004` |

Für das Formular sind kein historischer Implementierungsbranch, Commit oder PR nachgewiesen. Revisionen anderer Projektbereiche sind keine Umsetzungsbelege.

## 17. Abschließender Stand

**Gesichert im lokalen Archivpaket:** Der historische fachliche Auftrag, alle vorgeschlagenen Formularbereiche und Beispielgruppen, die ausdrückliche Wahl von Word, der tatsächlich erzeugte DOCX-Stand, dessen ursprünglicher Generator, die später festgestellten Abweichungen und konkrete Fortsetzungspunkte.

**Offene Produktarbeit:** Word-V2 mit echtem Logo, nutzbaren Tabellen und klarer Feldsemantik sowie Word-/Druck- und praktische Planungsabnahme. PDF-/Webvarianten und Backendintegration bleiben Optionen.

**Offene Nachweise:** vollständiger Repository-/Normenabgleich und historische Word-/Drucker-/Eventabnahme. Eigenständige Bilder fehlen. Die zugängliche DOCX wurde vollständig gelesen und unverändert erhalten.
