# Brainstorming: Funkgeräte-Inventarliste mit Excel-Stammdaten ohne Zubehörbindung

**Archivhinweis, 09.10.2026:** Diese Datei dokumentiert das im Titel beziehungsweise Quellenrahmen genannte Gespräch und dessen damalige Entscheidungen. Historische Kommandos, Planungen, Versionsangaben und Fehlerbilder bleiben erhalten; der heutige technische Stand steht in der [Gesamtroadmap](../roadmaps/gesamtroadmap.md) und in den [aktuellen Dienstanleitungen](../services/README.md). Die Archivaufnahme bestätigt keine spätere Implementierung oder Live-Abnahme.

**Stand der Notizen und ergänzenden Prüfungen: 2026-10-04.** Historische Entwürfe, nachgewiesene Umsetzung und ausgeführte Tests sind jeweils getrennt gekennzeichnet.

> **Planungsstand:** Eine Excel-Liste mit 17 bestätigten Spalten bildet die Gerätestammdaten ab. Wechselbare Akkus und Zubehör sollen keinem Funkgerät dauerhaft zugeordnet werden. Zusätzliche Spalten bleiben optional.
>
> **Zusätzlich überprüfter Repository-Stand:** Auf `Archiving` existiert bereits ein Asset-Management-Dienst mit Geräte-, Personen-, Ausgabe- und Wartungsverwaltung. Das ist ein eigenständiger geprüfter Codebefund, kein im damaligen Planungsstand nachgewiesenes Entwicklungsergebnis. Das dortige Datenmodell und die Export-/Importwege bilden die 17 Excel-Spalten nicht verlustfrei ab.

## 1. Kontext und Quellenlage

| Merkmal | Stand |
|---|---|
| Projekt | NetCore-Tetra |
| Thema | Sinnvolle Spalten für eine Funkgeräte-Inventarliste; Trennung von Gerätestammdaten und wechselbarem Zubehör |
| Historischer Zeitraum | Bestätigte Spaltenliste und Zubehörkorrektur sind ergänzend dem 11.12.2025 zugeordnet; eigene Zeitstempel des ursprünglichen Bestands fehlen. |
| Erstellungsdatum | 2026-10-04 |
| Repository | `JanHG98/netcore-tetra` |
| Geprüfter Zielbranch | `Archiving` |
| Geprüfter Code-/Dokumentationsstand | `10214c87c9ce302ee3b35da504b978504fea0eb5` |
| Zugehöriger Root-Tree | `8c7b3381ce121acdf9bf42d965d856476ce22dc7` |
| Archivdatei | `Docs/archive/2026-10-04_funkgeraete-inventarliste-excel-stammdaten-ohne-zubehoerbindung.md` |
| Index | `Docs/archive/README.md` |

Die Planung umfasst vorhandene Spalten, Erweiterung auf 17 Felder und den ausdrücklichen Ausschluss gerätefester Zubehör-/Akkubindung. Der zusätzlich betrachtete VPN-Text ist sachfremd und begründet keine Architekturentscheidung.

**Nicht verfügbar:** die eigentliche Excel-Arbeitsmappe, Gerätezeilen, Zellformate, Formeln, Dropdowns, Codeplugdateien, Herstellerlizenzlisten, Geräteauslesungen, Betriebsprotokolle und eigenständige historische Bilder. Deshalb sind weder Datenqualität noch tatsächliche Gerätekonfiguration oder laufender Betrieb geprüft.

Die Files-Bestandsaufnahme ergab **25 Projekt-PDFs**, keine separat hochgeladene Excel-Datei und keine eigenständigen Bilddateien. Die PDFs sind allgemeine TETRA-Referenzen, keine Bestandsnachweise. Sie wurden inventarisiert und nur an fachlich einschlägigen Stellen vertieft gelesen; keine vollständige Auswertung aller Normseiten wird behauptet. Zwei während dieser Archivierung erzeugte Seitenrenderings dienten ausschließlich der Sichtkontrolle von Normabbildungen und sind keine ursprünglichen Bilder.

### 1.1 Statusbegriffe

| Status | Bedeutung in diesem Dokument |
|---|---|
| **Idee** | Vorgeschlagen, ohne ausdrückliche Freigabe oder Umsetzungsnachweis. |
| **Beschlossen/geplant** | Ausdrücklich vorgegeben oder bestätigt; noch kein Nachweis einer Repository-Implementierung. |
| **Implementiert** | Konkreter Code oder ein konkretes Artefakt wurde im angegebenen Repository-Stand gelesen. |
| **Getestet** | Ein bestimmter Test wurde tatsächlich ausgeführt; Umfang und Grenzen müssen benannt sein. |
| **Im Betrieb bestätigt** | Durch zuordenbare Laufzeitnachweise oder ausdrücklich abgegrenzte Betriebsbeobachtung bestätigt. |

Tabellenänderung, Softwareimplementierung und Betrieb sind unterschiedliche Nachweisstufen. Ein hinterlegter Smoke-Testplan ist noch kein bestandenes Testergebnis.

## 2. Ziel, Ausgangslage und Planungsverlauf

Ausgangspunkt ist eine bestehende Excel-Tabelle für Funkgeräte mit zwölf Feldern; gesucht sind sinnvolle Ergänzungen:

```text
Eigentümer | Typ | Modell | Seriennummer | TEI | MCC | MNC | ISSI | ITSI | Anzeigename | Tactical | Firmware
```

Ein früher Text zu „9.4 VPN-Integration“ war sachfremd und bleibt aus der Inventarplanung ausgeschlossen.

Betrachtet wurden Erweiterungen für Inventarisierung, Programmierung, Geräteverwaltung, Zustandsüberwachung und Automatisierung. Der darüber hinaus gedachte Ausbau zur umfassenden Gerätedatenbank ist nicht beschlossen.

Festgelegt wurde eine Erweiterung auf 17 Felder: **Marke, Frequenzband, Codeplug, E2EE und SIM** kommen hinzu. Gerätefeste Zubehör-/Akkufelder sind ausgeschlossen, weil diese Komponenten keine dauerhafte Einheit mit dem Funkgerät bilden.

Nach dem Zubehör-Ausschluss bleiben sechs optionale Felder: Status, Programmierdatum, Gruppenprofil, Standortzuordnung, Lizenzoptionen und Fehler-/Bemerkungsfeld. Ihre tatsächliche Aufnahme in die Tabelle ist nicht bestätigt.

## 3. Endgültig bestätigte Anforderungen und Entscheidungen

### 3.1 Bestätigte Spaltenliste

Die Reihenfolge und Begriffe werden aus Jans letzter vollständiger Liste übernommen; lediglich versehentliche Rand-Leerzeichen sind entfernt:

```text
Eigentümer;Typ;Marke;Modell;Seriennummer;TEI;MCC;MNC;ISSI;ITSI;Anzeigename;Tactical;Firmware;Frequenzband;Codeplug;E2EE;SIM
```

Diese Zeile dokumentiert die Überschriften. Sie ist **kein** fertig spezifiziertes CSV-Importformat.

| Nr. | Bestätigte Spalte | Gesicherter Inhalt / noch offene Bedeutung |
|---:|---|---|
| 1 | Eigentümer | Eigentümerzuordnung gewünscht; Personenname, Organisation oder Eigentümercodierung nicht festgelegt. Nicht automatisch identisch mit aktuellem Nutzer. |
| 2 | Typ | Eigene Spalte neben Marke und Modell; konkrete Typenliste nicht definiert. |
| 3 | Marke | Als neue Spalte bestätigt; konkrete Gerätewerte fehlen. |
| 4 | Modell | Modellbezeichnung; konkrete Gerätevarianten nicht aus einer Bestandsdatei bekannt. |
| 5 | Seriennummer | Seriennummer als eigener Identifikator vorgesehen. |
| 6 | TEI | Als eigener Identifikator vorgesehen; Darstellung und Erfassungsweg im historischen Planungsstand nicht definiert. |
| 7 | MCC | Eigene Spalte; keine tatsächlichen Netzdaten dieses Bestands genannt. |
| 8 | MNC | Eigene Spalte; keine verbindliche Format- oder Validierungsregel festgelegt. |
| 9 | ISSI | Eigene Spalte; kein Nummernplan in diesem Planungsstand beschlossen. |
| 10 | ITSI | Zusätzlich zur Komponentenaufteilung enthalten; Formel, Anzeigeformat und Pflegeverantwortung offen. |
| 11 | Anzeigename | Vorgesehen; Bezug auf Gerätedisplay, Verzeichnis oder Leitstellenanzeige nicht näher definiert. |
| 12 | Tactical | Originalbezeichnung beibehalten; Abgrenzung zu Anzeigename/Funkrufname offen. |
| 13 | Firmware | Firmwarestand vorgesehen; Version gegenüber Build nicht endgültig getrennt. |
| 14 | Frequenzband | Als neue Spalte bestätigt; Hardwareband gegenüber programmiertem Frequenzprofil abgrenzen. |
| 15 | Codeplug | Als neue Spalte bestätigt; Name, Version, Dateireferenz und Änderungsdatum nicht separat spezifiziert. |
| 16 | E2EE | Als neue Spalte bestätigt; Fähigkeit, Lizenz, Konfiguration und nachgewiesene Nutzung unterscheiden. |
| 17 | SIM | Als neue Spalte bestätigt; Vorhandensein, Kartentyp oder Kartenkennung noch definieren. |

**Nachweisstand:** Tabelle und Überschriften sind als Bestand bestätigt. Die Arbeitsmappe wurde nicht bereitgestellt und konnte nicht geprüft oder geändert werden. Excel-, CSV- und Google-Sheets-Vorlagen wurden nicht erzeugt.

### 3.2 Keine feste Akku- oder Zubehörbindung

**Beschlossen:** Wechselbare Akkus und Zubehör bleiben außerhalb fester Funkgeräte-Stammdaten. Ausschlaggebend ist die fehlende dauerhafte Zuordnung.

Damit sind die zuvor empfohlenen gerätefesten Felder für Akku-Typ, Akku-Seriennummer, Kapazität, Zyklen, Akkutausch, Ladegerät, Headset/PTT und ähnliches Zubehör für diese Liste verworfen. Das ist keine pauschale Aussage, dass Zubehör niemals separat inventarisiert werden dürfe; ein separates Zubehörprojekt wurde hier jedoch ebenfalls nicht beauftragt.

Die Fortsetzung darf die abgelehnten Bindungen nicht über eine umfangreichere „Profi“-Vorlage wieder einführen. Die Zahl der Spalten ist kein Qualitätsmaßstab.

### 3.3 Nicht beschlossen

Es gibt keine Freigabe für eine Datenbankmigration, neue API, automatische Netzfreigabe, zentrale Benutzerverwaltung, Pflichtwartungsakte oder feste Standort-/Personenzuordnung. Ebenso fehlen eine endgültige Dropdown-Auswahl, konkrete Pflichtfelder, Prioritäten, Termine und eine Abnahme für zusätzliche Spalten.

## 4. Ideenbestand und verworfene Erweiterungen

Die folgenden Erweiterungen sind Ideen und keine verbindlichen Anforderungen.

| Themenfeld | Betrachtete Ideen | Abschließende Einordnung |
|---|---|---|
| Betriebs-/Inventarstatus | Aktiv, Reserve, defekt, Reparatur/eingeschickt, außer Dienst, verloren, Testgerät; früh auch „vergriffen“ | **Idee**. Administrative Verfügbarkeit, Einsatzzweck und Netzregistrierung nicht ungeprüft in ein einziges Statusfeld mischen. |
| Programmierstand | Letzte Programmierung, Codeplug-Datum, Codeplug-Version | **Idee**. Nur `Codeplug` selbst ist bestätigt. |
| Gruppen | GSSI-/Gruppenprofil, Gruppenliste, Haupt-/Zusatzgruppen, Prioritätsgruppe, Template | **Idee**. Kein festgelegter Gruppenplan oder vollständiger Codeplugexport. |
| Zuordnung | Standort, Abteilung, Nutzer, Verantwortlicher, Fahrzeug, Wache, Einsatzgruppe, Techniklager | **Idee**. Beispiele waren keine tatsächlichen Zuweisungen. |
| Funktionen/Lizenzen | GPS, Bluetooth, Man-Down, Lone Worker, Call-Out, DMO-Repeater, E2EE, SDS-bezogene Optionen | **Idee**. Welche Funktion lizenzpflichtig oder vorhanden ist, muss modellbezogen belegt werden. |
| Frequenz-/Netzprofil | TMO-Bandplan, Frequenzprofil, Node-/Netzprofil; beispielhafte BOS-, AFU-, Test- oder Lab-Zuordnung | **Idee**. Die damals beispielhaft genannte Kombination `901/999` ist kein hier bestätigter Betriebsparameter und keine Zuteilung. |
| Firmwaredetails | Zusätzliche Build-/Release-Nummer | **Idee**. Kein tatsächlich gemessener Build bekannt; beispielhafte Versionsnummern sind keine Bestandsdaten. |
| SDS | Unterstützte SDS-Typen bzw. SDS-Profil | **Idee**. Begriffe wie „SDS-Status-Entscheider“ wurden nicht technisch definiert. |
| Netz-/Sicherheitszustand | ISSI aktiv/authentifiziert/gesperrt; „TEI-Status“; SIM-/Crypto-Modul- oder Schlüsselreferenz | **Idee** mit begrifflichem Klärungsbedarf. Keine Geheimnisse als Inventardaten. |
| Hybridgeräte | IMEI bzw. Modem-ID | **Idee**, nur bei tatsächlich entsprechender Hardware. Das damalige SC20-/SC21-Beispiel ist zu korrigieren, siehe Abschnitt 5. |
| Zustand | Betriebsstunden, letzte Fehlermeldungen/Events | **Idee**. Keine Auslesbarkeit oder konkreten Messwerte belegt. |
| Administration | Kaufdatum, Lieferant, Garantieende, Kostenstelle, Projektzuordnung | **Idee**, weder verworfen noch übernommen. |
| Wartung/Fehler | Reparaturhistorie, Tickets, Fehler-/Bemerkungsfeld | **Idee**. „No Service“, „RegFail“ und „Authentication Rejected“ waren illustrative Fehlertexte, keine dokumentierten Vorfälle dieser Geräte. |
| Konfigurationsvergleich | Codeplug-Checksumme bzw. Konfigurations-Hash | **Idee**. Keine Implementierung oder normalisierte Vergleichsdefinition. |
| Automatisierung | Gruppen-/Funktionsrolle wie Operator, Gateway, Dispatch oder Monitoring; Datenbank-/CSV-Import | **Idee**. Kein Vertrag mit NetCore-Komponenten vereinbart. |
| Identifikation | QR-Link zur Gerätedetailseite | **Idee**. Keine Ziel-URL, Inventarnummernlogik oder QR-Datei erstellt. |
| Vorlagen/UI | Excel- oder Google-Sheets-Vorlage; kompakte/vollständige Variante; farbliche Bereiche, Spaltenbreiten und Dropdowns | **Angeboten, nicht beauftragt/erstellt**. Auch die vorgeschlagenen Standort-, Lizenz-, Firmware- und Gerätetyp-Auswahllisten sind nicht final. |
| Akkus und Zubehör | Gerätefeste Zuordnung samt Gesundheits-/Tauschdaten | **Verworfen für diese Liste**, wegen der wechselnden Zuordnung. |

Die sechs zuletzt wiederholten Kandidaten waren: **Status, letzte Programmierung, Gruppenprofil, Standortzuordnung, Lizenzoptionen, Fehler/Bemerkungen**. Sie bleiben optional und nicht freigegeben.

## 5. Fachliche Nachprüfung bei der Archivierung

Der nachträgliche fachliche Abgleich vom 2026-10-04 ergänzt den historischen Planungsstand. Grundlage sind die bereitgestellten Normausgaben und gesondert recherchierte Hersteller-/Microsoft-Primärquellen.

### 5.1 Geräteidentität und Teilnehmeridentität

Die bereitgestellte EN 300 392-1 V1.6.1 unterscheidet die vom Hersteller vergebene, netzunabhängige **TEI** von Teilnehmeridentitäten. Die **ISSI** ist der netzspezifische Teil der **ITSI**; gleiche SSI-Werte können in unterschiedlichen Netzen vorkommen. Abbildung 3 auf Seite 29 zeigt MCC mit 10 Bit, MNC mit 14 Bit und SSI mit 24 Bit. Abbildung 5 auf Seite 33 beschreibt die TEI mit insgesamt 60 Bit bzw. 15 Hexadezimalstellen. Beide Abbildungen wurden zusätzlich im Seitenbild geprüft. [S1]

**Folgerung für eine spätere Umsetzung:** TEI, Seriennummer und Teilnehmeridentität nicht als austauschbare Schlüssel behandeln. Bei mehreren Netzen reicht die ISSI allein nicht zur eindeutigen Netzzuordnung. Ein ITSI-Anzeigeformat oder eine Excel-Formel wurde nicht vereinbart; bloßes Aneinanderhängen variabel langer Dezimaltexte wäre deshalb kein spezifiziertes Austauschformat.

### 5.2 E2EE, SIM und Funktionsfreischaltungen

EN 300 392-7 V3.5.1, Abschnitt 6.1 auf Seite 114, trennt Luftschnittstellenverschlüsselung ausdrücklich von Ende-zu-Ende-Verschlüsselung. Ein einziges Feld `E2EE` ohne definierte Semantik belegt somit weder Luftschnittstellensicherheit noch einen erfolgreichen verschlüsselten Ende-zu-Ende-Ruf. [S2]

Die bereitgestellten TSIM-/SIM-ME-Dokumente betreffen TETRA-Teilnehmermodule und deren Schnittstellen; `SIM` darf in dieser Liste daher nicht ungeprüft als LTE-SIM interpretiert werden. Die konkreten Geräte und Karten sind unbekannt. Die UICC-Schnittstellenbeschreibung definiert zudem keine Inventarverwaltung. [S3, S4]

Für eine spätere Präzisierung sind „unterstützt“, „freigeschaltet“, „konfiguriert“ und „im Betrieb geprüft“ unterschiedliche Aussagen. Ob diese Trennung zusätzliche Spalten erfordert, bleibt offen. Authentifizierungsschlüssel, PINs/PUKs, private Schlüssel, Tokens und andere Zugangsdaten gehören weder in dieses Archiv noch in Beispielbestandsdaten.

### 5.3 Fehlerhafte oder zu starke frühere Aussagen

| Frühere Aussage | Einordnung für die Fortsetzung |
|---|---|
| SC20/SC21 als Beispiele für LTE-/Hybridgeräte | Die am Archivierungstag gelesene Sepura-Produktübersicht führt beide als TETRA-Geräte und unterscheidet Hybridgeräte gesondert. Die pauschale Zuordnung aus der früheren Einordnung wird nicht übernommen. Daraus folgt keine IMEI-Pflichtspalte für diesen Bestand. [S6] |
| Gruppenprofil als pauschale Erklärung für „Kein Dienst“ | Nicht diagnostisch belegt. Fehlende Gruppennutzung und fehlender Netzdienst dürfen ohne weitere Befunde nicht gleichgesetzt werden. |
| Gleicher Konfigurations-Hash bedeutet gleich programmierte Geräte | Nur bei identischer Vergleichsgrundlage sinnvoll. Dateiprüfsumme, fachlich normalisiertes Profil und gerätespezifische Identitäten sind zu unterscheiden. Keine Vergleichslogik wurde implementiert. |
| Hashvergleich werde in großen BOS-Organisationen besonders häufig genutzt | Unbelegte Allgemeinaussage; keine technische Anforderung oder belastbare Projektquelle. |
| „70 % einer perfekten Datenbank“, „unverzichtbar“ und zahlreiche „MUST-HAVES“ | Keine messbaren Vollständigkeitskriterien. Diese Wertungen werden durch die konkrete Festlegung und den tatsächlichen Zweck der Liste ersetzt. |
| „TEI-Status (aktive Zuweisung)“ | Kein hinreichend definierter Feldname. Hardwareidentität, Teilnehmerzulassung, Sperrung und aktuelle Registrierung müssen fachlich getrennt werden. |
| GPS/BT/E2EE/SDS-Funktionen pauschal als Lizenzen | Die konkrete Ausstattung und Freischaltung ist geräte-/herstellerbezogen zu prüfen; keine bestätigte Feature-Matrix liegt vor. |
| Beginn eines VPN-Handbuchkapitels | Sachfremde Fehlantwort, kein beschlossenes oder implementiertes Teilprojekt dieser Planung. |

### 5.4 Excel-Datentreue als neuer Prüfhinweis

Microsoft dokumentiert, dass Excel führende Nullen entfernen und Zahlencodes in wissenschaftliche Schreibweise oder Datumswerte umwandeln kann; numerische Genauigkeit ist auf 15 signifikante Dezimalstellen begrenzt. Nachträgliche Textformatierung stellt bereits verlorene Stellen nicht wieder her. [S5]

**Neuer, nicht historisch beschlossener Prüfpunkt:** Seriennummern, TEI, Kartenkennungen und textuelle Identitätsdarstellungen vor Eingabe/Import bewusst als Text behandeln und mit einem Originalauszug vergleichen. Die 15 Hexadezimalstellen einer TEI sind nicht mit Excels 15 signifikanten Dezimalstellen gleichzusetzen. Keine bestehende Zelle wurde in diesem Auftrag verändert oder getestet.

## 6. Repository-Abgleich am Dokumentdatum

### 6.1 Prüfmethode und Reichweite

Alle fachlichen Repository-Befunde unten beziehen sich auf `Archiving` bei `10214c87c9ce302ee3b35da504b978504fea0eb5`. Die GitHub-Codesuche diente nur zum Auffinden möglicher Pfade; da sie den Default-Branch durchsucht, wurden die tatsächlich verwendeten Dateien anschließend ausdrücklich am genannten Commit gelesen.

Der Abgleich ist gezielt, nicht repositoryweit vollständig: Phase-10-Dokumentation, Dienst-README, Datenmodell/Normalisierung, Persistenz, MQTT-Veröffentlichung, Subscriber-/Mobility-Abgleich, HTTP-Handler, Beispielkonfiguration, systemd-Unit und lokale Smoke-Testbeschreibung. Die umfangreichen eingebetteten UI-Styles wurden nicht vollständig geprüft. Kein anderer Branch wurde geschrieben oder gemergt.

### 6.2 Bereits vorhandene Komponenten

| Komponente | Nachgewiesener Stand | Grenze |
|---|---|---|
| `asset-management` | **Implementiert:** Python-Dienst für Assets, Personen, Ausgaben und Wartungsdatensätze. [R1, R2] | Keine Installation oder laufende Instanz in diesem Auftrag bestätigt. |
| Gerätebestand | `kind = tetra_radio`; unter anderem Hersteller, Modell, Seriennummer, Firmware-/Codeplugversion, TEI und ISSI. [R1] | Nicht identisch mit der 17-spaltigen Excel-Liste. |
| Andere Asset-Arten | Unter anderem `accessory`, `rack`, `tbs`, `server`, `tool`. [R1] | Eine generische Zubehörkategorie erzwingt keine feste Zubehörbindung an ein Funkgerät. Keine Löschung dieser Kategorie beauftragt. |
| Persistenz | JSON-Zustand, NDJSON-Ereignisse und Auditdatei; Schreiben des Zustands über temporäre Datei und Ersetzen. [R1, R3] | Keine Wiederherstellung oder Parallelitäts-/Ausfallabnahme durchgeführt. |
| Personen/Ausgaben | Eigenständige Personen und Ausgabe-/Rückgabe-Datensätze. RUI/RUA nur Metadaten; `pin_stored = False`, `network_login_executed = False`. [R1] | Kein Beweis einer Funkbenutzeranmeldung; Aufnahme solcher Zusatzfelder in die Excel-Liste nicht beschlossen. |
| Subscriber Core | In der Dienstbeschreibung autoritativ für ISSI-Zulassung und Dienstberechtigungen; Lesepfade für Profile/beobachtete Teilnehmer sind vorhanden. [R1, R2] | Ein Inventareintrag darf nicht als Netzfreigabe behandelt werden. |
| Mobility Core | In der Dienstbeschreibung autoritativ für bedienende TBS/Registrierung; Snapshot-Abgleich im Code. [R1, R2] | Ein gespeicherter Snapshot ist keine garantierte Live-Erreichbarkeit. |
| Task Workflow | Codepfad zum Erstellen eines Wartungsauftrags vorhanden. [R1] | Kein Wartungsauftrag ausgelöst. |
| Export/Import | CSV-Export sowie JSON-Export/-Import implementiert. [R1] | Kein nachgewiesener XLSX-Import oder verlustfreier Roundtrip mit der vorhandenen Tabelle. |

Damit ist die historische Vorstellung einer irgendwann möglichen Geräte-Datenbank am 2026-10-04 **teilweise durch vorhandenen Code überholt**. Nicht überholt sind die konkreten Feldentscheidungen und die Notwendigkeit, das tatsächliche Tabellenformat vor einer Migration zu verstehen. Ein zusätzlicher paralleler Inventardienst ist daraus nicht abzuleiten.

### 6.3 Abbildung der 17 Excel-Spalten auf das gelesene Datenmodell

Referenz ist `AssetManagement.normalize_asset()` in [R1], nicht eine behauptete produktive Datenbankstruktur.

| Excel-Spalte | Vorhandenes Asset-Feld | Ergebnis / offene Aufgabe |
|---|---|---|
| Eigentümer | `organization` | Nur möglicher Anknüpfungspunkt; Eigentum und Organisation sind semantisch nicht automatisch gleich. Kein explizites `owner`-Feld im gelesenen Normalizer. |
| Typ | `kind` | Grobe Asset-Klasse; eine Funkgeräte-Unterart ist damit nicht automatisch abgebildet. |
| Marke | `manufacturer` | Mögliche Zuordnung, Terminologie festlegen. |
| Modell | `model` | Direktes Gegenstück. |
| Seriennummer | `serial_number` | Direktes Gegenstück; Eindeutigkeitsverhalten bei Änderungen prüfen. |
| TEI | `device_tei` | Feld vorhanden; numerische Normalisierung ist für hexadezimale Eingaben problematisch, siehe Abschnitt 8. |
| MCC | Kein eigenes Feld | Fehlend im gelesenen normalisierten Asset-Modell. |
| MNC | Kein eigenes Feld | Fehlend im gelesenen normalisierten Asset-Modell. |
| ISSI | `issi` | Vorhanden; Prüfung nur auf positive Ganzzahl, kein vollständiger Nummernplan-/24-Bit-Nachweis im Normalizer. |
| ITSI | Kein eigenes Feld | Keine vereinbarte Ableitung oder persistierte vollständige Teilnehmeridentität. |
| Anzeigename | Kein eigenes Asset-Feld | `display_name` ist im Personenmodell vorhanden, nicht als gleichbedeutender Gerätename. |
| Tactical | Kein eigenes Feld | Bedeutung klären, nicht stillschweigend auf Personenname oder `notes` abbilden. |
| Firmware | `firmware_version` | Mögliche direkte Zuordnung. |
| Frequenzband | Kein eigenes Feld | Hardwareband/Profil-Semantik zunächst festlegen. |
| Codeplug | `codeplug_version` | Nur passend, wenn die Excel-Spalte tatsächlich eine Version bezeichnet. |
| E2EE | Kein eigenes Feld | Kein durch dieses Asset-Modell nachgewiesener E2EE-Fähigkeits-/Betriebsstatus. |
| SIM | Kein eigenes Feld | Kartenart/Referenz und zeitliche Bindung unbestimmt. |

`tags` und `notes` existieren zusätzlich, sind aber kein stillschweigender Ersatz für fehlende strukturierte Felder. Der generische JSON-Import kann zwar zusätzliche Schlüssel übernehmen; `normalize_asset()` baut bei späterer Bearbeitung jedoch einen festen Feldsatz neu auf. Unbekannte Importfelder sind dadurch nicht als dauerhaft unterstütztes Schema zu betrachten. [R1]

## 7. Dateien, Dienste, Schnittstellen und technische Parameter

Die Angaben in diesem Abschnitt sind **Repository-Vorgaben**, keine aus Jans Excel-Liste gewonnenen Betriebsdaten.

### 7.1 Dienst und Speicherung

| Parameter | Im geprüften Stand |
|---|---|
| Dienstname / Unit | `netcore-asset-management` / `netcore-asset-management.service` |
| Programm / Startargument | `/usr/local/bin/netcore-asset-management --config /etc/netcore/asset-management.toml` |
| Beispielbindung | `0.0.0.0:8290`, HTTP über TCP |
| Konfiguration | `/etc/netcore/asset-management.toml` |
| Zustand | `/var/lib/netcore-asset-management/state.json` |
| Ereignisse | `/var/lib/netcore-asset-management/events.ndjson` |
| Audit | `/var/lib/netcore-asset-management/audit.ndjson` |
| Zustandsschema | `netcore-asset-management-state-v1` |
| Weitere Schemas | `netcore-asset-v1`, `netcore-person-v1`, `netcore-assignment-v1`, `netcore-event-v1` |
| Implementierung | Python; unter anderem `tomllib`, `ThreadingHTTPServer`, JSON/CSV; MQTT-Veröffentlichung über `mosquitto_pub` |
| systemd | `User=root`, `Group=root`, `Restart=on-failure`, `RestartSec=2`, `NoNewPrivileges=true`, `PrivateTmp=true`, `ProtectSystem=full`, beschreibbarer Datenpfad |

Quellen: [R1, R3, R4]. Die Unit-Einstellungen ersetzen keine Authentifizierung der HTTP-API.

### 7.2 Netzwerk und Abhängigkeiten

| Verbindung | Beispielwert / Funktion |
|---|---|
| MQTT | `127.0.0.1:1883`, Topic-Präfix `netcore/v1` |
| Asset-Ereignisse | Unter anderem `netcore/v1/events/asset/...`; Ereignisse nicht retained |
| Asset-Zustand | `netcore/v1/state/assets/<asset_id>`; Zustandsveröffentlichung retained |
| Subscriber Core | Beispielbasis `http://127.0.0.1:8100`; Lesen von `/api/v1/subscribers` und `/api/v1/observed` |
| Mobility Core | Beispielbasis `http://127.0.0.1:8090`; Lesen von `/api/v1/subscribers` |
| Task Workflow | Beispielbasis `http://127.0.0.1:8280`; Anlegen über `/api/v1/tasks` |
| Wartungsziel | Beispielkonfiguration `default_gssi = 15201`; keine Festlegung für diese Inventarliste |
| Abgleich | Beispielintervall 60 Sekunden; Code begrenzt das Intervall nach unten auf 10 Sekunden |
| Ereignishistorie | Beispielwert 3000 Einträge im Arbeitsspeicher |

Quellen: [R1, R3]. Diese Loopback-Ziele sind keine Aussage über die tatsächliche Verteilung auf LXCs.

### 7.3 Relevante HTTP-Pfade

Implementiert sind unter anderem `GET /health/live`, `GET /health/ready`, `GET /api/v1/status`, `GET/POST /api/v1/assets`, Einzelasset-Lesen/-Ändern/-Löschen, Personen-, Ausgabe- und Wartungsrouten sowie `POST /api/v1/reconcile`. Dazu kommen `/metrics` und eine knappe `/openapi.json`-Pfadübersicht. [R1]

Für einen späteren Tabellenanschluss besonders wichtig:

| Route | Codebefund |
|---|---|
| `GET /api/v1/export/assets.csv` | Exportiert genau zwölf Spalten: `asset_id, inventory_id, kind, status, manufacturer, model, serial_number, firmware_version, codeplug_version, issi, device_tei, location`. |
| `GET /api/v1/export.json` | Exportiert Assets, Personen, Ausgaben und Wartung als JSON. |
| `POST /api/v1/import` | Importiert JSON-Objekte bzw. Listen; keine Excel-Arbeitsmappe. Der Schalter `replace` kann vorhandene Sammlungen ersetzen. |

Der CSV-Export enthält schon das vorhandene `organization`-Feld nicht und ist kein vollständiges Gegenstück zu Jans Überschriften. Ein CSV-Export allein beweist weder einen CSV-Import noch einen verlustfreien Excel-Roundtrip. [R1]

## 8. Probleme, Diagnose und verbleibende Risiken

### 8.1 Historisch tatsächlich aufgetreten

Ein Geräte-, Netz- oder Excel-Laufzeitfehler ist nicht diagnostiziert. Die fachliche Korrektur beschränkt die Liste auf Gerätestammdaten und schließt sachfremde VPN-Planung sowie gerätefeste Zubehör-/Akkufelder aus. Ein Softwarefix war dafür nicht nötig.

### 8.2 Neu erkannte statische Codebefunde

| ID | Befund am geprüften Commit | Bedeutung und Grenze |
|---|---|---|
| INV-R01 | `device_tei` wird über `int_value()` mit Python-`int(value)` normalisiert; bei Konvertierungsfehlern wird `0` eingesetzt. | Hexadezimale TEI-Texte mit Buchstaben bzw. Präfix sind nicht durch diesen Pfad korrekt abgedeckt; rein numerische Hexdarstellungen haben zusätzlich eine Basismehrdeutigkeit. Statische Codeanalyse, kein am Gerät beobachteter Datenverlust. [R1, S1] |
| INV-R02 | `issi` wird auf `> 0` geprüft; im gelesenen Normalizer fehlen obere 24-Bit-Grenze, Reservierungen und vollständiger Netzkontext. | Eine erfolgreiche Normalisierung bedeutet nicht gültige oder eindeutig zugewiesene Teilnehmeridentität. [R1, S1] |
| INV-R03 | `reconcile()` bildet Profile/Beobachtungen/Mobility über die ISSI ab, nicht über MCC/MNC/ISSI. | Bei späterem Mehrnetzbetrieb muss die Schlüsselstrategie geklärt werden. Im jetzigen Bestand ist kein konkreter Kollisionsfall belegt. [R1] |
| INV-R04 | `create_asset()` prüft doppelte Seriennummern; `update_asset()` enthält die entsprechende Prüfung im gelesenen Pfad nicht. | Update-/Importpfade gesondert auf Eindeutigkeit testen; Hersteller-/Modellkontext einer Seriennummer ebenfalls definieren. [R1] |
| INV-R05 | JSON-Import führt direkte Dictionary-Updates bzw. Ersetzungen aus, ohne die regulären Asset-/Personen-Normalizer aufzurufen. | Import und CRUD haben unterschiedliche Validierungswege. Ungeprüftes `replace` kann Bestände ersetzen; unbekannte Felder können bei späterem Normalisieren wieder verschwinden. Kein Import ausgeführt. [R1] |
| INV-R06 | Dienstbeschreibung, Statusantwort und Beispielkonfiguration kennzeichnen OPEN LAB ohne Login, Tokens oder TLS. Der Actor wird aus `X-NetCore-Actor` übernommen. | Nicht als authentifiziertes Benutzer-/Auditkonzept oder produktionsreife sichere Inventarverwaltung ausgeben. [R1–R3] |
| INV-R07 | Fehlende strukturierte Felder und eingeschränkter CSV-Export. | Ein unmittelbarer Import würde die bestätigte Excel-Struktur nicht vollständig und semantisch eindeutig abbilden. [R1] |

Keiner dieser Befunde wurde in diesem Archivierungsauftrag behoben. Änderungen an `system-backend/` oder Konfigurationen liegen ausdrücklich außerhalb des Auftrags. Ebenso ist nicht bewiesen, dass alle Erkenntnisse neue, bisher unbekannte Projektprobleme sind; dokumentiert ist ausschließlich der gelesene Stand.

## 9. Befehle, Abläufe und Tests

### 9.1 Historischer Stand

Im ursprünglichen Austausch wurden keine Installations-, Deployment-, Reparatur- oder Geräteprogrammierbefehle ausgeführt. Es gab keine Funkmessungen, E2EE-Tests, Firmwareupdates, Codeplugvergleiche oder Excel-Abnahmen.

### 9.2 Während der Archivierung tatsächlich durchgeführt

| Prüfung / Aktion | Ergebnis | Grenze |
|---|---|---|
| GitHub-Branch und Archivindex lesen | `Archiving` und Ausgangscommit tatsächlich ermittelt; vorhandene Indexeinträge gelesen. | Kein Nachweis einer laufenden NetCore-Installation. |
| Zielpfad vor dem Schreiben prüfen | Der neu gewählte Archivpfad lieferte `404 Not Found`. | Andere Archive, insbesondere der separate ISSI-Nummernplan, dürfen nicht ersetzt werden. |
| Gezielte Quellcodeprüfung | Die in Abschnitt 6–8 genannten Dateien und Funktionen wurden am festen Commit gelesen. | Kein vollständiger Codeaudit, kein Anwendungstest. |
| Dateienbestand | 25 Projekt-PDFs, keine Arbeitsmappe oder eigenständigen historischen Bilder. | Reale Bestandszeilen und Zellformate nicht prüfbar. |
| PDF-Inventar und Normabbildungen | Dateinamen, Seitenzahlen, Deckblätter und SHA-256-Werte lokal geprüft; Seiten 29/33 aus [S1] gerendert und visuell geprüft. | Übrige Normteile nur nach Relevanz erschlossen, nicht vollständig gelesen. |
| Lokaler Git-Klonversuch | `git clone --depth 1 --single-branch --branch Archiving ...` scheiterte an `Could not resolve host: github.com`. | Der GitHub-Connector war separat lesefähig und ist der gewählte Schreibweg; kein lokaler Push vorgetäuscht. |
| Externe Primärquellenprüfung | Microsoft-Hinweis zur Excel-Datentreue und Sepura-Geräteklassifizierung gelesen. | Kein Herstellerabgleich aller im Bestand befindlichen Geräte möglich. |

### 9.3 Im Repository vorhandener, nicht ausgeführter Smoke-Testplan

`system-backend/asset-management/tests/open_lab_smoke.md` beschreibt: Person und Funkgerät anlegen, Ausgabe, Rückgabe, Wartung planen/abschließen, Subscriber-/Mobility-Abgleich sowie MQTT-Topics prüfen. Es handelt sich um eine **Testanleitung**, nicht um einen Testbericht. Für diesen Auftrag wurde sie gelesen, aber nicht ausgeführt. [R5]

### 9.4 Ausschließlich vorgeschlagene spätere Diagnose

Die folgenden lesenden Befehle wurden **nicht ausgeführt**. Sie sind für eine spätere Prüfung auf dem tatsächlich zuständigen Host gedacht; die Beispiele setzen die dokumentierte lokale Portbindung voraus:

```bash
systemctl status --no-pager netcore-asset-management.service
journalctl -u netcore-asset-management.service -n 100 --no-pager
curl --fail --silent --show-error http://127.0.0.1:8290/health/live
curl --fail --silent --show-error http://127.0.0.1:8290/health/ready
curl --fail --silent --show-error 'http://127.0.0.1:8290/api/v1/assets?kind=tetra_radio'
```

Kein schreibender Import-/Ersetzungsbefehl wird als bereits erprobter Ablauf angeboten. Vor einem späteren Import sind ein gesicherter Export, ein abgestimmtes Mapping und ein Testbestand erforderlich.

## 10. Roadmap-Kandidaten und konkrete Fortsetzung

**Historisch vereinbarte Prioritäten:** keine. Die Reihenfolge unten ist ein Vorschlag aus dieser Abschlussprüfung; keine Termin- oder Umsetzungszusage. Die zentrale Roadmap wurde nicht verändert.

| Kandidat | Nächster Schritt / Akzeptanzkriterium | Abhängigkeit und Status |
|---|---|---|
| INV-01: Feldbedeutungen | `Typ`, `Tactical`, `Codeplug`, `Frequenzband`, `E2EE`, `SIM` und Eigentümerbezug in einem kleinen Datenwörterbuch definieren. | **Idee / zur Entscheidung**; Originalarbeitsmappe oder anonymisierte Beispiele erforderlich. |
| INV-02: Datentreue | TEI-/Identitätsformat sowie Text-/Zahlbehandlung festlegen; führende Nullen und Hexwerte mit Originalquelle vergleichen. | **Neuer Prüfauftrag-Kandidat**; keine bestehenden Werte ohne Originalnachweis reparieren. |
| INV-03: Optionale Spalten | Über die sechs letzten Kandidaten einzeln entscheiden; nur tatsächlich gepflegte Informationen hinzufügen. | **Historische Ideen**, nicht pauschal als Pflicht übernehmen; Zubehör-/Akkubindung bleibt ausgeschlossen. |
| INV-04: Bestehenden Dienst abgleichen | Entscheiden, ob Excel führend bleibt oder der vorhandene Asset-Dienst angebunden wird. Feldmapping einschließlich Eigentümer und vollständigem Netzkontext beschließen. | **Idee**; vorhandenen Dienst nutzen statt unbegründeten Parallelneubau starten. |
| INV-05: Validierung/Import | Befunde INV-R01 bis INV-R05 priorisiert bearbeiten: TEI-Darstellung, ISSI-Grenzen/Netzschlüssel, Eindeutigkeit und konsistente Importvalidierung. | **Neuer Roadmap-Kandidat**, keine Codeänderung in diesem Auftrag. |
| INV-06: Verlustfreier Austausch | Export/Import mit festgelegten Überschriften, Zeichensatz und Datentypen sowie stabilen Datensatzschlüsseln spezifizieren. | **Idee**; hängt von INV-01/02/04/05 ab. Kein aktueller XLSX-Roundtrip zugesichert. |
| INV-07: Sicherheits-/Betriebsabnahme | OPEN-LAB-Grenze beachten; Zugriffsschutz und Auditverantwortung vor Nutzung mit schutzbedürftigen Beständen klären. | **Neuer Prüfauftrag-Kandidat**; keine bereits vorhandene zentrale RBAC in diesem Dienst behaupten. |
| INV-08: Optionaler Ausbau | QR-Gerätelink, normalisierter Konfigurationsvergleich, Feature-/Gruppenprofile oder Vorlagen erst bei Bedarf konkretisieren. | **Historische Nebenideen**, nachgelagert und nicht beauftragt. |

Für eine spätere Abnahme sind mindestens Testfälle zu führenden Nullen, hexadezimaler TEI, leerer/ungültiger Kennung, doppelter Seriennummer bei Anlage und Änderung, netzübergreifend gleicher ISSI, unbekannten Importfeldern sowie Export–Import–Bearbeitung–Export sinnvoll. Das sind **vorgeschlagene Tests**, keine ausgeführten Ergebnisse. Reale Betriebsabnahme und eventuell E2EE-Funktionstest bleiben getrennt von der Tabellen-/API-Abnahme.

## 11. Quellen, Repository-Dateien und Querverweise

### 11.1 Historische Planungsgrundlage

| Lokale Quellenkennung | Inhalt |
|---|---|
| C1 | Ausgangsliste mit zwölf Feldern. |
| C2 | Erweiterungsansätze für Inventar, Konfiguration, Verwaltung und Automatisierung. |
| C3 | Bestätigte vollständige 17-Spalten-Liste. |
| C4 | Weitere optionale Felder einschließlich Zubehör/Akku. |
| C5 | Ausschluss fester Zubehör-/Akkuzuordnung. |
| C6 | Bereinigte Liste mit sechs verbliebenen optionalen Feldvorschlägen. |

Die lokalen Quellenkennungen ordnen die historischen Planungsschritte zu. C3 und C5 enthalten die endgültigen Festlegungen.

### 11.2 Gelesene Repository-Quellen

Die Links sind auf den tatsächlich geprüften Commit fixiert:

- **[R1]** [Asset-Management-Implementierung](https://github.com/JanHG98/netcore-tetra/blob/10214c87c9ce302ee3b35da504b978504fea0eb5/system-backend/asset-management/src/netcore_asset_management.py): insbesondere `normalize_asset`, `int_value`, `create_asset`, `update_asset`, `reconcile`, HTTP-Handler, CSV-Export und JSON-Import. Geprüft wurden die abgerufenen Bereiche 1–500, der fachliche Teil ab 501 bis vor dem eingebetteten UI sowie der Handler-/Startteil ab 1000. Blob: `8c6ebe79d9bd9d5c2918732c6988333780449488`.
- **[R2]** [Dienst-README](https://github.com/JanHG98/netcore-tetra/blob/10214c87c9ce302ee3b35da504b978504fea0eb5/system-backend/asset-management/README.md) und [Phase-10-Dokumentation](https://github.com/JanHG98/netcore-tetra/blob/10214c87c9ce302ee3b35da504b978504fea0eb5/Docs/PHASE_10_ASSET_DEVICE_USER_MANAGEMENT.md): Zuständigkeitsgrenzen und OPEN LAB.
- **[R3]** [Beispielkonfiguration](https://github.com/JanHG98/netcore-tetra/blob/10214c87c9ce302ee3b35da504b978504fea0eb5/system-backend/asset-management/config/asset-management.example.toml): Ports, Pfade, MQTT und Upstreams. Blob: `4574d55cd9c274551a53ed60098ff8d837eb8f7f`.
- **[R4]** [systemd-Unit](https://github.com/JanHG98/netcore-tetra/blob/10214c87c9ce302ee3b35da504b978504fea0eb5/system-backend/asset-management/systemd/netcore-asset-management.service): Startkommando und Dienstumgebung. Blob: `5b5128be4b86a61adc0028aa941ea04f2c910564`.
- **[R5]** [OPEN-LAB-Smoke-Testplan](https://github.com/JanHG98/netcore-tetra/blob/10214c87c9ce302ee3b35da504b978504fea0eb5/system-backend/asset-management/tests/open_lab_smoke.md): Anleitung ohne hier belegtes Testergebnis. Blob: `4ad82548fefa328c8b8c11182625661496a0e9fa`.
- **[R6]** [Vorhandener Archivindex vor dieser Ergänzung](https://github.com/JanHG98/netcore-tetra/blob/10214c87c9ce302ee3b35da504b978504fea0eb5/Docs/archive/README.md): vorhandene Einträge bleiben erhalten.

Das [Archiv zum ISSI-Nummernplan, zur Vergabe und zu RBAC](2026-10-04_issi-nummernplan-rbac-und-vergaberichtlinie.md) ist ein thematischer Querverweis. Sein fachlicher Volltext wurde für diesen Abgleich nicht erneut ausgewertet.

Für die Inventarplanung sind keine Umsetzungscommits oder PRs dokumentiert.

### 11.3 Fachquellen der zusätzlichen Prüfung

- **[S1]** Projektanhang `en_30039201v010601p.pdf`, ETSI EN 300 392-1 V1.6.1 (2020-04), Abschnitte 7.2.3–7.2.5 und 7.5, insbesondere Seiten 29 und 33–34. Identitäts-/TEI-Abbildungen auf 29 und 33 visuell geprüft.
- **[S2]** Projektanhang `en_30039207v030501p.pdf`, ETSI EN 300 392-7 V3.5.1 (2019-07), Definition auf Seite 16 sowie Abschnitt 6.1 auf Seite 114: Trennung von AIE und E2EE.
- **[S3]** Projektanhang `ts_10081201v020205p.pdf`, ETSI TS 100 812-1 V2.2.5 (2003-10), Seiten 4–6: Geltungsbereich UICC-Terminal-Schnittstelle; keine administrative Kartenverwaltung.
- **[S4]** Projektanhänge `es_20081202v020401m.pdf`, **Final draft** ETSI ES 200 812-2 V2.4.1 (2005-08), Begriffsdefinitionen auf Seite 10, sowie `en_300812v020101p.pdf`, ETSI EN 300 812 V2.1.1 (2001-12), bereitgestellte SIM-ME-Referenz. Keine Aussage über konkret verbaute Karten abgeleitet.
- **[S5]** [Microsoft: Keeping leading zeros and large numbers](https://support.microsoft.com/en-us/excel/keeping-leading-zeros-and-large-numbers), zusätzlich [Format numbers as text](https://support.microsoft.com/en-us/excel/format-numbers-as-text); abgerufen am 2026-10-04.
- **[S6]** [Sepura: TETRA, Hybrid and 4G/5G hand-portable radios](https://sepura.com/hand-portable-radios/), ergänzend die Herstellerseiten für [SC20](https://sepura.com/devices/sc20-tetra-radio/) und [SC21](https://sepura.com/devices/sc21-tetra-radio/); abgerufen am 2026-10-04.

Die Normen werden als **bereitgestellte Ausgaben** zitiert. Ihr geprüfter Normstatus wurde nicht umfassend recherchiert; vorhandene Drafts werden nicht zu verabschiedeten Normen umdeklariert.

## 12. Anhang: Quellen- und Bildbestand

### 12.1 Umgang mit Bildern und Originaldateien

**Keine eigenständigen historischen Bilder vorhanden:** Im thematischen Verlauf waren weder Gerätefotos noch Tabellen-Screenshots oder entworfene Diagramme enthalten. Die sichtbaren PDF-Deckblätter und eingebetteten Normabbildungen gehören zu den Projekt-PDFs. Sie wurden nicht als vermeintliche Originalbilder neu veröffentlicht. Die während der Prüfung erzeugten Normseiten-Renderings sind ebenfalls keine nachzuarchivierenden historischen Bildassets.

Die 25 PDFs wurden nicht dupliziert und nicht als Gerätebestandsdaten in das Git-Archiv kopiert. Ihr identifizierbarer Bestand ist unten festgehalten. Die eigentliche Arbeitsmappe kann ohne bereitgestellte Datei nicht nacharchiviert werden. Ein vollständiges Archiv der Excel-Inhalte ist daher ausdrücklich **nicht** Bestandteil dieses Ergebnisses.

### 12.2 Inventarisierte Projekt-PDFs

Die Titel/Versionen stammen aus den vorliegenden Deckblättern, die Seitenzahlen aus den verfügbaren Dateien. „Referenz“ bedeutet lediglich vorhanden und thematisch eingeordnet, nicht vollständig gelesen oder als Projektentscheidung übernommen.

| Datei | Ausgabe / Thema | Seiten | Auswertung |
|---|---|---:|---|
| `en_30039201v010601p.pdf` | EN 300 392-1 V1.6.1 (2020-04), General network design | 182 | Identitäten und TEI gezielt geprüft; [S1]. |
| `en_30039207v030501p.pdf` | EN 300 392-7 V3.5.1 (2019-07), Security | 216 | AIE/E2EE gezielt geprüft; [S2]. |
| `ts_10081201v020205p.pdf` | TS 100 812-1 V2.2.5 (2003-10), UICC-Schnittstelle | 8 | Geltungsbereich; [S3]. |
| `es_20081201v020205p.pdf` | ES 200 812-1 V2.2.5 (2003-12), UICC-Schnittstelle | 8 | Verwandte Referenz, nicht als identische Datei mit TS-Ausgabe behandelt. |
| `es_20081202v020401m.pdf` | Final draft ES 200 812-2 V2.4.1 (2005-08), TSIM-Anwendung | 139 | Begriffe gezielt geprüft; [S4]. |
| `en_300812v020101p.pdf` | EN 300 812 V2.1.1 (2001-12), SIM-ME | 156 | Bereitgestellte SIM-Referenz; [S4]. |
| `en_30039202v030801p.pdf` | EN 300 392-2 V3.8.1 (2016-08), Air Interface | 1445 | Referenz, keine daraus abgeleitete Funkimplementierung. |
| `en_30039205v020701p.pdf` | EN 300 392-5 V2.7.1 (2020-04), PEI | 320 | Referenz, kein Geräteausleseverfahren implementiert. |
| `en_3003920303v010301p.pdf` | EN 300 392-3-3 V1.3.1 (2011-11), ISI-Gruppenruf | 251 | Inventarisiert, nicht Gegenstand der Tabellenentscheidung. |
| `en_3003920304v010301p.pdf` | EN 300 392-3-4 V1.3.1 (2010-08), ISI-SDS | 28 | Inventarisiert. |
| `en_3003920308v010401p.pdf` | EN 300 392-3-8 V1.4.1 (2020-04), ISI Speech Format | 22 | Inventarisiert. |
| `en_3003920313v010201p.pdf` | EN 300 392-3-13 V1.2.1 (2020-04), transportunabhängiger ISI-Gruppenruf | 191 | Inventarisiert. |
| `en_3003920315v010500a.pdf` | Draft EN 300 392-3-15 V1.5.0 (2026-04), ISI Mobility Management | 380 | Als Draft inventarisiert; kein Nachweis historischer Entscheidungen. |
| `en_30039209v010701p.pdf` | EN 300 392-9 V1.7.1 (2020-04), Supplementary Services | 46 | Inventarisiert. |
| `en_3003921006v010401p.pdf` | EN 300 392-10-6 V1.4.1 (2006-08), Call Authorized by Dispatcher | 20 | Inventarisiert. |
| `en_3003921018v010301p.pdf` | EN 300 392-10-18 V1.3.1 (2003-10), Barring of Outgoing Calls | 17 | Inventarisiert. |
| `en_3003921101v010201p.pdf` | EN 300 392-11-1 V1.2.1 (2004-01), Call Identification | 44 | Inventarisiert. |
| `en_3003921114v010101p.pdf` | EN 300 392-11-14 V1.1.1 (2002-07), Late Entry | 23 | Inventarisiert. |
| `en_3003921117v010102p.pdf` | EN 300 392-11-17 V1.1.2 (2002-01), Include Call | 18 | Inventarisiert. |
| `en_3003921201v010202p.pdf` | EN 300 392-12-1 V1.2.2 (2007-08), Call Identification | 56 | Inventarisiert. |
| `en_3003921216v010400a.pdf` | Draft EN 300 392-12-16 V1.4.0 (2026-03), Pre-emptive Priority Call | 67 | Als Draft inventarisiert; kein Nachweis historischer Entscheidungen. |
| `en_30039401v030301p.pdf` | EN 300 394-1 V3.3.1 (2015-04), Radio Conformance Testing | 169 | Referenz, kein Konformitätstest durchgeführt. |
| `en_30039502v010303p.pdf` | EN 300 395-2 V1.3.3 (2025-02), TETRA codec | 94 | Inventarisiert. |
| `ets_30039214e01v.pdf` | Final draft prETS 300 392-14 (1997-09), PICS proforma | 61 | Als Entwurf inventarisiert; keine ausgefüllte Konformitätserklärung. |
| `ETSI.pdf` | Sammeldatei; beginnt mit EN 300 812 V2.1.1 (2001-12) | 4100 | Keine vollständige Kollation mit allen Einzeldateien; nicht vollständig ausgewertet. |

### 12.3 SHA-256 der vorliegenden PDF-Dateien

Die Prüfsummen identifizieren die bei dieser Archivierung tatsächlich verfügbaren Dateien, nicht eine behauptete neueste Normausgabe.

```text
9434dad1e7bc80ca39b0edadd8e3b9995fda5ae5f5c3dd565af05708d5059e38  ETSI.pdf
788722e566098957bea1a9a5096e9dae1eace1da9f5d38c530c80230e5b9b9cb  en_30039201v010601p.pdf
3f07b1e4ad73fabc16277a900006fc19844b3a882bbc2bc1d74d78f6156daf28  en_30039202v030801p.pdf
94ca61038b3ff826e6adcfde56e00473dd996e1a47f9f84e7b9d2aa6c3133fd2  en_3003920303v010301p.pdf
8a38cc6238c6da726e4f07d7d377b2c827447a1448a60ee5ef2a126cb3a0ef9d  en_3003920304v010301p.pdf
4d993a25e35da7b1f2291ae281cb3ea5b3a7b702ab7eb3233bdf430a2c70909d  en_3003920308v010401p.pdf
b47d63853f83a6a4adda08c0e325c8ca13d358e2f8840bfba4bce430e600b1bd  en_3003920313v010201p.pdf
e959709667192e22d2dfeea51b48f9a579a4c107d952999056b7fad93a6d2100  en_3003920315v010500a.pdf
10aaf78988ae4080c3dfe90165ee7f4f3fe8eda402e09fc1e6fa0a0ad890758d  en_30039205v020701p.pdf
df47af9a642b6eba7d7d5cf5fb7aa3f961e9e03bb912316bd2d189ce47344e08  en_30039207v030501p.pdf
cad45938bcdf2fa4e8063d4aa9e7d7c06c45f729d3c55dc033e00c27af9f2e06  en_30039209v010701p.pdf
32b6ee43602ef88a0b5c209b1c69ad35ba8b2e660502257c2dfaa6205a346523  en_3003921006v010401p.pdf
4cc10bcd94d39201538639cefb529809729563f13f87cfe6ae52b44c253b0b8a  en_3003921018v010301p.pdf
852c17ea566757e80ecf0d2718ae7839d9e218aa4384b63d1277ed7e25d86c69  en_3003921101v010201p.pdf
ba1882df71dbb84a0c4f0f7dcea1dc8df968044457da03df9290a50e8821df32  en_3003921114v010101p.pdf
69ce800e352b1ecfc337416fd7b54c1bdf2d9270171af519ac2f5acc1ba85ca6  en_3003921117v010102p.pdf
4d5b56a188fb73c417d67b27da619cad2e49efb3b57306fd4ae51e140c018018  en_3003921201v010202p.pdf
c0ee7859f448c4072befc25905c29b751b5151d3590a880a2d654309a3ba6c02  en_3003921216v010400a.pdf
2d9c327e5ccf1476335e1b7a4f58a0ca7afc93911c30e8aad5ed7dc9ccf3276a  en_30039401v030301p.pdf
ac716ca18082cc0fd2ee768a14f03deb48fa78a77e83751b1a6b319c7fc2106a  en_30039502v010303p.pdf
196effeaae473a98244c89538e1562daa58bb1d74cbfe7de78e58ceeafbad18b  en_300812v020101p.pdf
346dc545049a7397ca86831731bbc05e96f61ff047d6e7f6b7de56da3aa408e9  es_20081201v020205p.pdf
330f045a908eefd249fd7450e5b71bd08e8a7b20ce5b86dc9b87c9138a5c7268  es_20081202v020401m.pdf
2b703f3ebb883c90dab40e1b3d1a634a6acf9c9a20d468fd108ea02226c2bf2c  ets_30039214e01v.pdf
96df75f5ddf1c3ae4ac75f796ee97babca68fa81aca22f39ba0e0658bd57eed1  ts_10081201v020205p.pdf
```

## 13. Übergabestand

Die 17 bestätigten Überschriften und der Ausschluss fester Zubehör-/Akkubindung bilden die belastbare Fortsetzungsgrundlage. Zusätzliche Felder bleiben Entscheidungen, nicht vermeintlich schon erledigte Aufgaben. Der ergänzende Asset-Dienst ist als bestehender Anschlusskandidat nachgewiesen; Tabellenmigration, Validierungsfixes, sichere Betriebsfreigabe und reale Tests sind nicht erledigt.

Für die nächste Bearbeitung zuerst die Originaltabelle bzw. anonymisierte Beispiele und die offenen Feldbedeutungen klären. Danach das bestehende Asset-Modell gezielt abgleichen. Keine fremden Archive überschreiben, keine Geheimnisse übernehmen und aus diesem Dokument keinen Auftrag zur Geräteprogrammierung oder Produktionsänderung ableiten.
