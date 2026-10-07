# Brainstorming: ISSI-Nummernplan, Vergaberichtlinie und RBAC

> **Planungsstand:** Nummernplan und RBAC sind konzeptionell beschrieben, noch nicht vollständig validiert oder produktiv ausgerollt. Die zuletzt angenommene Struktur `[D][K][EE][NNNN]` wird historisch bewahrt. Ihre verbleibenden Fehler werden ausdrücklich getrennt dokumentiert: insbesondere der Überlauf bei `D=1/K=9` und der Widerspruch zwischen einer stabilen ISSI und einem durch Nummernwechsel dargestellten Betriebsmodus.

## Zielbild und Festlegungen

- Acht Organisationen erhalten getrennte ISSI-Blöcke für HRT, MRT und eigene Leitstellen.
- Rollen und Berechtigungen werden außerhalb der ISSI geführt; GSSI folgt als eigenes Thema.
- Letzter Entwurf: `[D][K][EE][NNNN]`. Der 24-Bit-Maximalwert **16.777.215** hat Vorrang vor dem Dezimalschema.
- Offen: Überlauf bei `D=1/K=9`, stabile Identität versus Betriebsmodus sowie konsolidierte Richtlinie und Geräteprüfung.

## 1. Arbeitsstand und Quellenbasis

| Merkmal | Wert |
|---|---|
| Projekt | NetCore-Tetra |
| Thema | Organisationseigene ISSI-Blöcke für Leitstellen, HRT, MRT und Sonderteilnehmer; Rollen außerhalb der Nummer; formale Vergaberichtlinie und Karteikarten-Kurzfassung |
| Erstellungsdatum dieser Zusammenfassung | **2026-10-04**, Europe/Berlin |
| Repository | `JanHG98/netcore-tetra` |
| Geprüfter Repository-Commit vor den Archivänderungen | `e1a3bbcdce7c25015882126a8371642a55eb206b` |
| Zugehöriger Root-Tree | `6bd7ce8e27004c5d6b16cb0a8fc224b4943c2d2f` |
| Erneute Branchprüfung vor der Erstellung | Weiterhin derselbe Commit |
| Historisches Canvas | Titel „ISSI-Vergaberichtlinie NetCore-Tetra“; interne Dokumentreferenz `693dbd74fcd08191979b27386e9b367d`, **interne Referenz ohne öffentliche URL** |

### 1.1 Was zugänglich war

Grundlagen sind die Entwürfe zu Eigentümer-/Geräteblöcken, deren Korrekturen und die Karteikartenfassung der ISSI-Richtlinie. Die dokumentierten Canvas-Änderungen einschließlich fehlgeschlagener Vollanpassung wurden mit den Repository-Dateien des genannten Prüfstands abgeglichen. Einzelbelege stehen in Abschnitt 15.

Alle **25 PDF-Anhänge** wurden nach Datei, Titel, Version, Seitenzahl und Prüfsummenpräfix inventarisiert. Die einschlägigen Identitätsabschnitte aus EN 300 392-1 wurden gezielt gelesen; Abbildung 3 auf PDF-Seite 29 wurde visuell geprüft. Die **8.061 Dateiseiten** umfassen die 4.100-seitige Zusammenstellung `ETSI.pdf` und sind weder vollständig fachlich geprüft noch alle unterschiedlich.

### 1.2 Ausdrückliche Grenzen

- Ein vollständiger, erneut ausgelesener **am Prüfdatum vorliegender Canvas-Endstand** stand für diese Quellenprüfung nicht zur Verfügung. Überliefert sind dessen Erstellung und die sichtbaren Änderungen. Daraus lässt sich insbesondere nicht ableiten, dass alle Abschnitte konsistent aktualisiert wurden.
- Für die einzelnen historischen Entwurfsstände liegen keine belastbaren Zeitstempel vor. Ihre Reihenfolge ist bekannt, nicht ihr genaues Erstellungsdatum.
- Namen und vollständige Zuordnung der acht Organisationen wurden in dieser Entwicklungsphase nicht genannt.
- Es wurden keine Motorola-/Sepura-Programmierprofile, CPS-/Radio-Manager-Ausgaben, Firmwarestände oder realen Funkmessungen bereitgestellt.
- Die Repository-Prüfung war eine gezielte statische Prüfung, kein vollständiges Sicherheitsaudit und kein Vergleich aller Branches. Bei der dokumentierten Prüfung wurde kein anderer Branch als Quellstand untersucht.
- Es wurden keine Basisstationen, Leitstellen, Verzeichnisserver oder Endgeräte kontaktiert, um einen Live-Betrieb zu bestätigen.
- Passwörter, Tokens, private Schlüssel, Authentisierungswerte und andere Zugangsdaten werden nicht archiviert.

### 1.3 Bilder und sonstige Assets

In den zugänglichen Fachunterlagen und im ursprünglichen gemounteten Dateibestand gab es **keine eigenständigen Originalbilder, Fotos, Screenshots oder generierten Designbilder**. Vorhanden waren die 25 PDFs mit ihren eingebetteten Titelbildern, Tabellen und Abbildungen. Diese sind Normanhänge und keine zusätzlichen ursprünglichen Bilddateien.

## 2. Ziel, Ausgangslage und verbindliche Anforderungen

Benötigt wird ein nachvollziehbarer Nummernplan für ein Netz mit zunächst **acht Eigentümern beziehungsweise Organisationen**, die HRT und MRT besitzen. Geräte mit Leitstellenfunktion sollen einen eigenen Nummernbereich erhalten. Der Aufbau soll professionell und ausbaufähig sein, aber nicht durch mehrfach codierte Rollen und Sonderfälle unnötig kompliziert werden.

Die ausdrücklich formulierten Anforderungen, in ihrer zuletzt gültigen Lesart:

| Anforderung | Herkunft und Status |
|---|---|
| Eigene ISSI-Blöcke für Leitstellen | Als Anforderung festgehalten; **beschlossen/geplant** |
| Je Eigentümer getrennte HRT- und MRT-Blöcke | Als Anforderung festgehalten; **beschlossen/geplant** |
| Aktuell acht Eigentümer | Als Ausgangslage festgehalten; keine vollständige Namensliste vorhanden |
| Jede Organisation darf eigene LST betreiben | Ausdrücklich festgelegt; **beschlossen/geplant** |
| LST sollen vollständig berechtigt sein | Als Ziel festgehalten; genauer organisationsübergreifender Geltungsbereich und administrative Ausnahmen nicht abschließend bestimmt |
| Ursprünglich achtstellige ISSI | Darstellungswunsch; später durch den ausdrücklich genannten Maximalwert begrenzt |
| Maximalwert 16.777.215 beachten | Ausdrückliche Korrektur; Vorrang vor sämtlichen früheren, zu großen Beispielen |
| Nummern durch verständliches Aneinanderhängen von Feldern bilden | Gewünscht; keine Multiplikationsformel als notwendige Bedienlogik |
| Rolle `R` aus der ISSI herausnehmen | Für die Richtlinienanpassung festgelegt; Rollen sollen außerhalb der Nummer liegen |
| Richtlinie und Ergänzungen im Canvas | Mehrfach ausdrücklich beauftragt; historische Dokumentbearbeitung, keine automatische Codeimplementierung |
| GSSI später als eigenes Thema/Dokument behandeln | Festgelegte Abgrenzung; keine GSSI-Vergaberichtlinie in dieser Entwicklungsphase abgeschlossen |
| Sehr kurze Karteikartenfassung | Geplant; als Canvas-Anhang überliefert |

„Feuerwehr Stadt“ und „Rettungsdienst“ waren Platzhalter, keine registrierten Eigentümer. Auch die zwischenzeitliche Mischung aus Betreiber `01` plus acht weiteren Organisationen war keine belastbare Bestandsliste.

## 3. Statusbegriffe und erreichtes Ergebnis

Für diese Dokumentation gelten folgende Unterscheidungen:

| Kennzeichnung | Bedeutung |
|---|---|
| **Idee** | Als Ansatz vorgeschlagen, aber nicht als konkrete Umsetzung beauftragt oder abschließend ausgewählt |
| **Beschlossen/geplant** | Festgelegt oder zur Aufnahme in die Dokumentation angenommen; noch kein Nachweis von Software oder Betrieb |
| **Implementiert** | Im untersuchten Repository ist zugehöriger ausführbarer Code erkennbar; sein Einsatz ist damit nicht bestätigt |
| **Getestet** | Eine konkrete Prüfung wurde tatsächlich ausgeführt, mit hier genannter Reichweite |
| **Im Betrieb bestätigt** | Durch konkrete Live-Beobachtung belegt; für den neuen Nummernplan und die beschriebenen Sicherheitsfunktionen in dieser Entwicklungsphase **nicht erreicht** |

### 3.1 Ergebnis der historischen Planung

Erreicht wurde ein mehrstufig entwickelter **Dokumentationsentwurf** mit Organisations- und Geräteklassen, Rollenentkopplung, Vergabegrundsätzen, Sonderbetriebsüberlegungen und einer Kurzfassung. Die dokumentierten Revisionen bestätigen mehrere Textänderungen. Sie belegen weder einen Nummernvergabedienst noch funktionierende RBAC-, MFA-, Break-Glass- oder Mandantentrennung im Netz.

Die wiederholten historischen Aussagen „final“, „verbindlich“, „100 % kompatibel“, „BOS-Niveau“, „auditfest“ und „zukunftsfest“ gingen über die Nachweise hinaus. Sie werden hier **nicht als Abnahmeergebnis übernommen**.

### 3.2 Zusätzlich am Prüfdatum überprüft

Im Repository existieren inzwischen unter anderem eine organisatorische ISSI-Wiki-Seite, numerische SSI-Datentypen, lokale Control-Room-Rollen und Directory-Konfigurationsfelder. Diese Bausteine werden in Abschnitt 9 getrennt beschrieben. Sie sind kein Beleg dafür, dass genau die vollständige Richtlinie dieser Planung implementiert oder produktiv eingeschaltet ist.

## 4. Entwicklung des Nummernplans und ersetzte Ansätze

### 4.1 Erste Vorschläge mit Kategorie, Eigentümer und Rolle

Der erste Ansatz war eine Hierarchie aus Teilnehmerkategorie, Eigentümer, Geräte-/Rollenart und Laufnummer. Kategorien waren früh `1=LST`, `2=HRT`, `3=MRT`, `9=Test`; später kamen `4=Infrastruktur` und `5=virtuell` hinzu.

Eine vereinfachte Alternative bestand aus ausdrücklich vergebenen Bereichen wie `1000000–1000999` für LST, `2001000–2001999` für HRT eines Eigentümers und `3001000–3001999` für MRT. Diese Alternative wurde nicht als endgültiger Plan ausgewählt.

Danach wurde ein angeblich achtstelliges Schema `[K][EEE][G][NNNN]` genannt. Tatsächlich sind das **neun Stellen**. Das Beispiel `1 001 1 0001` ergibt `100110001` und ist nicht nur zu lang, sondern auch größer als der später genannte 24-Bit-Raum. Andere Beispiele desselben Entwurfs ließen stillschweigend Stellen weg. Dieser Ansatz ist überholt und darf nicht als konsistente Spezifikation verwendet werden.

### 4.2 Achtstelliger Entwurf `[K][EE][R][NNNN]`

Die anschließende Struktur hat tatsächlich acht Positionen: eine Klasse, zwei Eigentümerziffern, eine Rolle, vier Laufnummernziffern. Sie wurde in die erste Canvas-Richtlinie aufgenommen.

Historische Rollenkennungen:

| Klasse | Historische Rollenwerte, inzwischen nicht mehr ISSI-Felder |
|---|---|
| LST | `1` Hauptleitstelle, `2` Backup, `3` Dispatcher, `4` Supervisor, `5` Voice-Services/Recorder, `8` Reserve, `9` Technik/Wartung |
| HRT/MRT | `0` Standard, `1` Führung/Teamlead, `2` Sondergerät, `8` Reserve, `9` Service/Test |
| Virtuell | `5` Sprecher, `6` Bot/Automatik, `7` Analyse/KI, `9` Wartung |

Damit wurden Rollen und Sonderfälle teils doppelt codiert: als Klasse, als Rolle und zusätzlich als Laufnummernfenster. HRT/MRT wurden zeitweise zusätzlich über unterschiedliche Laufnummern getrennt, obwohl bereits die Klasse unterschied. Recorder tauchten sowohl bei LST als auch bei Infrastruktur auf. Für diese Mehrfachbelegungen entstand keine abschließende Vorrangregel.

Wesentlich schwerer wiegt der Wertebereich: Beispiele wie `20300042`, `30300007`, `50350001` und `90300001` überschreiten `16777215`. Ein achtstelliges Dezimalschema ist nicht beliebig mit allen Ziffern kombinierbar. Der ursprüngliche Entwurf wurde nach der Korrektur des Maximalwerts ersetzt.

### 4.3 Siebenstelliger Entwurf `[K][EE][NNNN]`

Nach dem Hinweis auf die Grenze wurde die Rolle entfernt und zunächst die Formel

```text
ISSI = K · 1.000.000 + EE · 10.000 + NNNN
```

eingeführt. Für die genannten Feldbreiten beschreibt sie genau dasselbe wie das Aneinanderhängen `[K][EE][NNNN]`. Die behauptete prinzipielle Trennung zwischen „echten numerischen Blöcken“ und „Bedeutung einzelner Dezimalstellen“ war hier irreführend: Beide Schreibweisen waren äquivalent.

Die festgehaltenen Beispiele für Organisation `03` waren:

| Klasse | Zerlegung | Numerischer Wert |
|---|---|---:|
| LST | `1 / 03 / 0001` | `1030001` |
| HRT | `2 / 03 / 0042` | `2030042` |
| MRT | `3 / 03 / 0007` | `3030007` |
| Virtuell | `5 / 03 / 0001` | `5030001` |
| Test | `9 / 03 / 0001` | `9030001` |

Der Entwurf ergibt siebenstellige Zahlen. Eine optionale Anzeige mit einer führenden Null wurde ergänzt. Die Kurzform `[K][EE][NNN]` wurde nicht übernommen; vier Laufnummernziffern blieben das Ziel.

Die Begründung, drei Laufnummernziffern würden „morgen garantiert zu klein“, war eine unbelegte Größenprognose. Vier Stellen sind eine geplante Reserve, keine aus dem Bestand von acht Organisationen abgeleitete zwingende technische Voraussetzung.

Korrigierte Kapazitätsangaben: Mit `K=9`, `EE=99`, `NNNN=9999` ist der größte Wert dieses siebenstelligen Schemas **9.999.999**. Die früher genannten Grenzen `5.999.999` beziehungsweise `9.099.999` decken nicht alle damals erlaubten Kombinationen ab. Ein 10.000er-Fenster enthält 10.000 Zahlen einschließlich `0000`; bei Vergabe nur von `0001–9999` sind es **9.999 Teilnehmerkennungen**.

### 4.4 Letzter angenommener Entwurf `[D][K][EE][NNNN]`

Für die erste Ziffer `0` oder `1` wurde die Betriebsdomäne `D` vorgeschlagen. Diese Struktur wurde anschließend zur Canvas-Anpassung angenommen. Die Rolle sollte ausdrücklich als Metadatum außerhalb der Nummer bleiben.

**Dies ist der letzte historische Zielentwurf, aber nicht in allen Kombinationen gültig.** Seine noch offenen Korrekturen werden in Abschnitt 8 erklärt, nicht stillschweigend in die historische Entscheidung hineingeschrieben.

## 5. Letzter historischer Zielentwurf im Detail

```text
[D][K][EE][NNNN]
 1  1   2    4    Positionen
```

| Feld | Historisch festgehaltene Bedeutung |
|---|---|
| `D` | `0` Produktivbetrieb; `1` Sonderbetrieb, beispielhaft Übung, Katastrophenlage und Mutual Aid |
| `K` | `1` LST; `2` HRT; `3` MRT; `4` Infrastruktur/Gateway; `5` virtuelle Teilnehmer; `9` Test/Übung |
| `EE` | `01–08` zunächst acht Organisationen; `09` Reserve/Mutual Aid; `90–99` Labor/Hersteller/Sondernetze |
| `NNNN` | Vierstellige laufende Teilnehmernummer; in den Beispielen und Vergabefenstern ab `0001` |
| `R` | **Entfällt als Nummernfeld**; Aufgaben und Rechte sollen über Rollen, Profile und Policies geführt werden |

`10–89` wurden nicht endgültig organisatorisch belegt. `00` ist ebenfalls nicht als reguläre Eigentümerkennung festgelegt. Weder eine komplette Eigentümerregistratur noch ein verbindlicher neuer Eigentümeraufnahmeprozess wurde tatsächlich ausgeführt.

### 5.1 Anzeige und numerischer Wert

| Achtstellige Darstellung | Numerische ISSI | Historische Bedeutung |
|---|---:|---|
| `01030001` | `1030001` | LST, Organisation 03, Produktiv |
| `02030042` | `2030042` | HRT, Organisation 03, Gerät 42, Produktiv |
| `03070007` | `3070007` | MRT, Organisation 07, Gerät 7, Produktiv |
| `05030001` | `5030001` | Virtueller Teilnehmer, Organisation 03, Produktiv |
| `09030001` | `9030001` | Testteilnehmer, Organisation 03; begrifflicher Konflikt mit D=0/Produktiv ist offen |
| `11030001` | `11030001` | LST, Organisation 03, Sonderbetrieb |
| `12030042` | `12030042` | HRT, Organisation 03, Sonderbetrieb |
| `19030001` | `19030001` | Rechnerisches Gegenbeispiel: passt zum Feldschema, **überschreitet aber 24 Bit** |

Die führende Null gehört nur zur Dezimaldarstellung. `02030042` und `2030042` sind als dezimale Zahl identisch. Dagegen sind `12030042` und `2030042` unterschiedliche Teilnehmerkennungen. Für die hier festgelegten Klassen entstehen bei `D=0` siebenstellige numerische Werte und bei gültigem `D=1` achtstellige Werte; die wiederholt genannte pauschale Aussage „sechs- oder siebenstellig“ ist unzutreffend.

### 5.2 Daraus ableitbare Produktivblöcke für die acht Organisationen

Die folgende Tabelle ist eine **rechnerische Ableitung des historischen Schemas**, keine Liste bereits programmierter oder tatsächlich vollständig vergebener Geräte. Sie verwendet die achtstellige Darstellung und das gesamte Laufnummernfenster `0001–9999`; die Bedeutung einzelner Teilfenster ist noch zu konsolidieren.

| Eigentümer | LST-Anzeigebereich | HRT-Anzeigebereich | MRT-Anzeigebereich |
|---|---|---|---|
| 01 | `01010001–01019999` | `02010001–02019999` | `03010001–03019999` |
| 02 | `01020001–01029999` | `02020001–02029999` | `03020001–03029999` |
| 03 | `01030001–01039999` | `02030001–02039999` | `03030001–03039999` |
| 04 | `01040001–01049999` | `02040001–02049999` | `03040001–03049999` |
| 05 | `01050001–01059999` | `02050001–02059999` | `03050001–03059999` |
| 06 | `01060001–01069999` | `02060001–02069999` | `03060001–03069999` |
| 07 | `01070001–01079999` | `02070001–02079999` | `03070001–03079999` |
| 08 | `01080001–01089999` | `02080001–02089999` | `03080001–03089999` |

### 5.3 Laufnummernfenster und Restwidersprüche

Im letzten eindeutig erfolgreichen vollständigen Update von Canvas-Abschnitt 7 stand:

| `NNNN` | Dort dokumentierter Zweck |
|---|---|
| `0001–1999` | Produktivbetrieb |
| `2000–6999` | Wachstum/Rollout |
| `7000–7999` | Sonderlagen |
| `8000–8999` | Reserve |
| `9000–9999` | Service/Test |

Später wurde `0001–6999` gemeinsam als Produktivbereich vorgeschlagen. Die entsprechende größere Canvas-Änderung schlug jedoch fehl; der anschließend erfolgreiche Wiederholungsversuch änderte Abschnitt 7 nicht. Deshalb wird hier nicht behauptet, die alternative Fensterdefinition sei im gesamten Canvas wirksam geworden.

Ebenfalls nur ergänzend vorgeschlagen wurden LST-Unterfenster `0001–0099` für Haupt-/Backupplätze, `0100–0199` für Dispatcher, `0200–0299` für Supervisor und `0900–0999` für Technik. Diese Idee würde Rollen erneut indirekt in der Nummer codieren und widerspricht bei häufigem Rollenwechsel dem erklärten Stabilitätsziel. Sie ist **nicht Teil einer abschließend konsolidierten Vergabevorschrift**.

## 6. Richtlinieninhalt, Rollen und Governance

### 6.1 In die Dokumentation aufgenommene Grundsätze

Die Canvas-Richtlinie zielte auf eindeutige Identitäten, erkennbare Organisation und Geräteklasse, Wachstum, zentrale Dokumentation, kontrollierte Vergabe und eine Trennung von Regelbetrieb und Tests. In frühen Fassungen hieß es „zentral dokumentiert, dezentral vergeben“; später wurde eine kontrollierte zentrale Dokumentation betont. Ein technisches Verfahren für konkurrierende Vergaben mehrerer Organisationen wurde nicht implementiert.

Weitere dokumentierte Ziele: keine undokumentierte Wiederverwendung, Freigabe und Versionsnummer für Änderungen, nachvollziehbarer Lebenszyklus und keine Änderung der ISSI nur wegen einer geänderten Aufgabe oder Rolle. Die Regel „ISSI bleibt stabil“ ist als organisatorisches Ziel zu verstehen, nicht als Aussage, ein Endgerät könne technisch niemals umprogrammiert werden.

### 6.2 RBAC und Identität

RBAC wurde als **Role-Based Access Control / rollenbasierte Zugriffskontrolle** erläutert. Die gewählte Grundidee lautet: Die Teilnehmerkennung identifiziert; ein zugeordnetes Rechteprofil entscheidet über erlaubte Aktionen. Ein Rollenwechsel soll nicht automatisch einen Nummernwechsel erfordern.

Beispiele in der Planung waren Dispatcher, Supervisor, Admin, Technik, Führung sowie eingeschränkte Bot-/Automatikprofile. Für ein HRT mit ISSI `2030042` sollte beispielsweise eine zusätzliche Führungsrolle vergeben werden können, ohne die ISSI zu ändern.

Die Rollenlisten sind Konzeptbeispiele, keine nachgewiesenen produktiven Profile. „Hauptleitstelle“ versus „Backup“ bezeichnet zudem eine betriebliche Funktion, „ATEX“ eine Geräteeigenschaft, „Reserve“ einen Lebenszykluszustand und „Supervisor“ ein Berechtigungsprofil. Diese unterschiedlichen Eigenschaften sollten bei der Fortsetzung nicht unbesehen in ein einziges Rollenfeld gepackt werden.

Frühe Regeln wie `ISSI beginnt mit 1 → darf alles` oder eine globale Freigabe des LST-Blocks wurden später ausdrücklich durch Policy-/RBAC-Überlegungen ersetzt. Sie sind **verworfen**. Das gilt auch dann, wenn jede Organisation eigene voll funktionsfähige Leitstellen erhalten soll. Eine Nummer ist kein Authentisierungsnachweis.

ABAC als spätere Ergänzung für Kontextbedingungen wie Zeit, Ort und Lage wurde erwähnt. Ein vollständiges ABAC-Modell wurde nicht entworfen; andere Zugriffsmodelle sind damit nicht fachlich ausgeschlossen.

### 6.3 Leitstellen und Organisationsgrenzen

Jede Organisation darf eigene LST-Adressen besitzen. Vollständige Berechtigung ist als Ziel festgehalten. Der Governance-Vorschlag sieht vor: standardmäßig Steuerung der eigenen Organisation, fremde Bereiche nur nach ausdrücklicher Freigabe.

Die konkrete Frage, ob LST dauerhaft fremde Organisationsgruppen steuern dürfen, wurde nicht abschließend beantwortet. Daraus folgt weder eine Freigabe globaler Administratorrechte für jede LST noch der Nachweis einer technisch durchgesetzten strikten Mandantentrennung.

Offen bleiben insbesondere die Trennung zwischen Dispositionsrechten, Teilnehmerverwaltung, Netzkonfiguration, Schlüsselverwaltung und organisationsübergreifender Aufsicht sowie die Zuordnung von menschlichem Leitstellenbenutzer, Arbeitsplatz und technischer Funkidentität.

### 6.4 Ergänzte Richtlinienkapitel

| Kapitel/Thema | Historisch aufgenommener Inhalt | Planungs- und Umsetzungsstatus |
|---|---|---|
| ISSI → Rechte → GSSI | Grundsätze, dass Rechte nicht allein aus einer Nummer folgen und Gruppen-Policies nötig sind | **Beschlossen/geplant als Dokumentation**; keine vollständige Rechte-/Gruppenmatrix |
| Organisationsübergreifende Steuerung | Explizite und zeitlich begrenzte Freigaben, Protokollierung | **Geplant**; kein implementierter Mandantenfreigabedienst nachgewiesen |
| Break-Glass | Standardmäßig aus; zeitlich begrenzter erweiterter Zugriff, MFA, Alarmierung, Audit und nachträgliche Prüfung | **Geplant**; kein erfolgreicher Test oder Live-Nachweis |
| Übungsmodus | Getrennte Teilnehmer-/Gruppenbereiche | **Geplant**, Nummern-/Policy-Details widersprüchlich |
| Katastrophenmodus | Priorisierte Gruppen/LST; Umschaltung über UI/App, Zeitplan oder Ereignis | **Idee/geplanter Richtlinieninhalt**, keine belegte Funktion |
| Lifecycle | Beantragen → prüfen → vergeben/dokumentieren → betreiben → stilllegen oder Reserve | **Geplant**, kein durchgeführter Vergabevorgang belegt |
| Audit | An-/Abmeldung, Gruppensteuerung, Break-Glass und organisationsübergreifende Aktionen protokollieren | **Geplant**; Aufbewahrungsdauer, Integrität und Zugriffsschutz nicht fertig spezifiziert |
| Revision | Änderungen versionieren, dokumentieren und freigeben | **Geplant**; keine externe Zertifizierung oder betriebliche Abnahme |

Die anfängliche Formulierung „NetCore-Tetra unterstützt“ diese Betriebsmodi war im Kontext eines Konzeptdokuments zu stark. Eine Beschreibung im Canvas ist keine Funktionsimplementierung. Ebenso ist ein Audit-Log nicht allein durch seine Erwähnung technisch revisionssicher.

### 6.5 Bewusste GSSI-Abgrenzung

Eine eigenständige GSSI-Richtlinie wurde ausdrücklich zurückgestellt. Gruppenprioritäten, Gruppenaufbau, genaue ISSI-/GSSI-Rechtematrix und detaillierte Mutual-Aid-Gruppenschaltung wurden nicht beschlossen. Erhalten bleiben nur die notwendigen Schnittstellen zum späteren Gruppenplan.

Die Trennung der Dokumente darf bei der späteren Vergabe nicht mit unabhängiger Kollisionsfreiheit verwechselt werden: EN 300 392-1 beschreibt SSI-Vergabe und Eindeutigkeit im gemeinsamen Netzkontext. Bestehende Gruppen-, Alias- und Sonderadressen müssen bei der Bereichsplanung berücksichtigt werden, auch wenn ihre eigene Richtlinie erst später folgt. [N1]

## 7. Canvas-Bearbeitung: Erfolg, Fehler und Konsistenzgrenzen

Die ursprüngliche Richtlinie enthielt die Abschnitte 1–10 zu Zweck, Grundsätzen, Format, Klassen, Eigentümern, Rollen, Laufnummern, Vergabe, Sicherheit und Revision. Als Erweiterung wurden die Abschnitte 11–16 für Rechte-/Gruppenbezug, organisationsübergreifende Steuerung, Break-Glass, Betriebsmodi, Lifecycle und Audit ergänzt. Später folgten Beispiele und die Karteikartenfassung.

| Historischer Bearbeitungsschritt | Sichtbares Ergebnis |
|---|---|
| `canmore.create_textdoc` mit der ersten Richtlinie | Tool meldete erfolgreiche Erstellung des oben referenzierten Dokuments |
| Ergänzung der Abschnitte 11–16 | Erfolgreiche Änderungsmeldung |
| Umstellung auf das siebenstellige 24-Bit-Blockmodell | Erfolgreiche Änderungsmeldung; unter anderem Formel, Klassenbereiche, Rollenmetadaten und Beispiele geändert |
| Großer Änderungsversuch auf `[D][K][EE][NNNN]` | **Fehlgeschlagen**: das Suchmuster für die vollständige Überschrift von Abschnitt 3 wurde nicht gefunden |
| Wiederholung mit allgemeinerem Muster | Erfolgreich, aber **nur drei Änderungen**: Abschnitt 3, Abschnitt 6 und Abschnitt 2 |
| Ergänzung der letzten ISSI-/RBAC-Karteikarte | Erfolgreiche Änderungsmeldung |

Die relevante Fehlermeldung lautete sinngemäß `updates.0.pattern: pattern not found in the document`. Das Muster begann mit `## 3\. Verbindliches ISSI-Format` und enthielt zusätzlich die genaue Klammerüberschrift. Beim Wiederholungsversuch wurde nur der allgemeinere Überschriftsanfang verwendet.

**Verbleibendes Dokumentproblem:** Die erfolgreiche Wiederholung übernahm nicht die gesamte zuvor vorgesehene Änderungsliste. Daher waren insbesondere Klassenbereiche, Vergaberegeln, Laufnummernfenster und Anhang nicht nachweislich vollständig an D angepasst. Nach der sichtbaren Änderungskette können die neue D-Struktur in Abschnitt 3, ältere K-Millionenformeln in anderen Abschnitten und die später ergänzte Kurzfassung gleichzeitig im Dokument stehen.

Die damalige Behauptung „konsistent überall angepasst“ war daher nicht durch eine vollständige Rücklese gedeckt. Für eine Fortsetzung ist ein vollständiger Canvas-Export beziehungsweise eine komplett neu konsolidierte Richtlinienfassung erforderlich. Der Canvas-Endstand bleibt zu konsolidieren.

## 8. Zusätzliche fachliche Prüfung: Korrekturen und offene Architekturfragen

Dieser Abschnitt ist eine **zusätzliche technische Prüfung bei der Quellenprüfung**. Er ersetzt nicht unbemerkt die historische Festlegung durch einen neuen Nummernplan.

### 8.1 24 Bit sind ein Wertebereich, keine freie achtstellige Dezimalnummer

Die bereitgestellte EN 300 392-1 V1.6.1, Abschnitt 7.2.1 auf Seite 27, definiert TSI mit 48 Bit und SSI mit 24 Bit. Abschnitte 7.2.3 und 7.2.4 auf Seite 29 beschreiben die netzspezifische SSI und die Zusammensetzung aus MCC, MNC und SSI. Abbildung 3 zeigt 10 Bit MCC, 14 Bit MNC und 24 Bit SSI. [N1]

Rechnerisch gilt `2^24 - 1 = 16777215`. Allerdings ist dies **nicht gleichbedeutend mit der größten frei vergebbaren normalen Individualadresse**: Abschnitt 7.7.8 auf Seiten 37–38 reserviert die aus 24 Einsen bestehende SSI für Gruppen-Broadcast. Deshalb darf der Zahlenraum nicht ohne weitere Regeln als vollständiger regulärer ISSI-Pool freigegeben werden. [N1]

Der Hinweis zum Eingabelimit von Motorola und Sepura wird als Gerätehinweis bewahrt. Mangels konkreter Geräte-/Softwaretests wird daraus keine allgemeine Hersteller-Kompatibilitätszusage abgeleitet.

### 8.2 D/K-Kombinationen des letzten Entwurfs

| Kombination | Rechnerisches Ergebnis bei `EE=01–99`, `NNNN=0001–9999` | Bewertung |
|---|---|---|
| `D=0`, `K=1,2,3,4,5,9` | Größter enthaltene Wert `9999999` | Innerhalb von 24 Bit; fachliche Vergabe-/Routingprüfung bleibt nötig |
| `D=1`, `K=1,2,3,4,5` | Größter enthaltene Wert `15999999` | Innerhalb von 24 Bit; Sonderbetriebssemantik weiterhin offen |
| `D=1`, `K=9` | Schon `19010001` liegt über `16777215` | **Durchgängig unzulässig** im 24-Bit-Raum |
| Andere Klassen wie 6, 7 oder 8 | Nicht Teil der abschließend genannten Klassenliste | Keine Erweiterung automatisch freigegeben |

Alle Beispiele einzeln unterhalb des Limits zu wählen genügt nicht. Die zugelassene **Menge der Feldkombinationen** muss validiert werden. Eine bloße Prüfung auf acht Ziffern oder ein generisches Präfix ist unzureichend.

### 8.3 Betriebsmodus versus stabile Identität

`02030042` und `12030042` unterscheiden sich numerisch um **10.000.000**. Wenn D in der Funkadresse mitgeführt wird, erzeugt ein Betriebsmoduswechsel eine andere ISSI. Das widerspricht der mehrfach begründeten Stabilität und würde Änderungen an Teilnehmerregistratur, Konfigurationen, Rufzielen, Berechtigungen und Historienzuordnung erfordern.

EN 300 392-1, Abschnitt 7.2.5, Seite 30, beschreibt die ISSI-Zuteilung als langfristig; Abschnitt 7.2.2, Seite 28, weist darauf hin, dass keine Standardverfahren zur dynamischen Zuteilung von ITSIs definiert sind. Daraus ergibt sich insbesondere kein allgemeiner standardisierter „D umschalten“-Mechanismus. [N1]

**Neue Empfehlung zur Klärung, noch nicht entschieden:** Entweder D ist eine dauerhaft zugeteilte Adressdomäne mit zulässigen Kombinationen, oder ein veränderlicher Betriebsmodus wird ausschließlich als Metadatum geführt. D zugleich als wechselnden Modus und als stabile Funkidentität zu behandeln ist nicht konsistent. Vor Abschluss dieser Entscheidung keine Massenumprogrammierung vornehmen.

### 8.4 Übungen, reale Sonderlagen und Reserve sind nicht dasselbe

Der Entwurf benutzt D=1 für Übung, reale Katastrophenlage und Mutual Aid; daneben gibt es K=9, Laufnummern ab 9000 und ein Sonderlagenfenster ab 7000. Außerdem steht `09030001` gleichzeitig für D=0/Produktiv und K=9/Test.

Offen ist damit, welches Merkmal Vorrang hat, welche Teilnehmer überhaupt echte Einsatzgruppen erreichen dürfen und wie aus einem Reservegerät ein aktives Gerät wird, ohne unnötig seine Nummer zu ändern. Eine reale Sonderlage darf nicht versehentlich mit einem isolierten Testbetrieb gleichgesetzt werden. Umgekehrt darf ein Testpräfix keine automatische Rechteausweitung auslösen.

Eine neu festzulegende Policy kann diesen Unterschied ausdrücken; die Nummer allein tut es nicht. Automatische Rechteerweiterung allein bei Ausfall mehrerer Nodes war nur ein Vorschlag und ist kein abgenommener Sicherheitsmechanismus.

### 8.5 Eigentum, Geräteklasse und Lebenszyklus

Auch nach Entfernen von R bleiben K und EE in der Identität codiert. Für Eigentümerwechsel, dauerhafte Umwidmung eines MRT zur LST, Reparaturtausch, Ausleihe und Wiederverwendung wurden keine vollständigen Regeln beschlossen. „Die ISSI ändert sich nie“ ist daher als pauschale Lebenszyklusregel zu weitgehend.

Zu entscheiden ist, ob eine Identität an ein physisches Gerät, einen logischen Teilnehmer oder eine längerfristige Funktion gebunden ist. Geräteseriennummer beziehungsweise TEI, Eigentümer, Benutzer, Rolle und technische Teilnehmeridentität dürfen nicht automatisch als dasselbe Objekt behandelt werden. Bei Mehrnetzbetrieb gehört zudem der Netzkontext zur eindeutigen Zuordnung; eine SSI ist nach der bereitgestellten Norm nicht weltweit allein eindeutig. [N1]

### 8.6 Darstellung, Speicherung und Berechtigungen

Für positionsbasierte Auswertung muss zuerst eine eindeutig dezimale Interpretation beziehungsweise eine normalisierte achtstellige Anzeige vorliegen. Ein String-Präfixvergleich auf mal sieben-, mal achtstelligen Werten ist fehleranfällig. Führende Nullen sollten nicht zu doppelten Datensätzen oder unterschiedlichen Vergaben führen.

Numerische Gültigkeit, Belegungskollision, organisatorische Klassifikation, Authentisierung und Autorisierung sind unterschiedliche Prüfungen. Selbst ein formal richtiger Wert aus dem LST-Bereich darf keine selbstbehauptete Leitstellenberechtigung verleihen. UI-Rollen, API-Zugriff und tatsächliche Durchsetzung im Funk-/SwMI-Pfad sind getrennt nachzuweisen. Ein in der Weboberfläche angezeigtes Profil belegt keine entsprechende Kontrolle jedes HRT/MRT-Verkehrs oder des DMO-Betriebs.

## 9. Geprüfter Repository-Stand, getrennt vom Planungsergebnis

Codebefunde beziehen sich auf `Archiving` bei `e1a3bbcdce7c25015882126a8371642a55eb206b`. Die Quelltexte wurden direkt am Prüfstand gelesen.

### 9.1 Nummernplan und numerische Adressen

| Datei | Heute statisch überprüfter Inhalt | Grenze des Nachweises |
|---|---|---|
| `wiki/ISSI-and-GSSI.md` | Enthält `D K EE NNNN` ausdrücklich als **mögliches organisatorisches Schema**, erklärt numerische Luftschnittstellenadresse, separate System-ISSIs, Directory-Dokumentation und lokale SSI-Bereiche | Kein vollständiger freigegebener Organisationsplan; keine Behandlung des D=1/K=9-Überlaufs in der gelesenen Seite [R1] |
| `system-backend/shared/contracts/src/address.rs` | `MAX_SSI = 0x00ff_ffff`; Makro erzeugt `Ssi`, `Issi`, `Gssi`; `new`, `TryFrom<u32>` und `FromStr` führen die obere Bereichsprüfung aus; `Display` und Serde-Darstellung sind numerisch | Die Prüfung akzeptiert auch 0 und den reinen Maximalwert. Keine D/K/EE-/Reservierungs-/Belegungsprüfung in dieser Datei [R2] |
| Dieselbe Contract-Datei | Tests `accepts_full_24_bit_range`, `rejects_values_above_24_bits`, `serde_is_numeric_and_roundtrips`; Beispiel `4_010_001` wird als JSON-Zahl `4010001` serialisiert | **Tests vorhanden**, hier nicht als Rust-Testlauf ausgeführt [R2] |
| `crates/tetra-core/src/address.rs` | `TetraAddress` enthält öffentliche Felder `ssi: u32` und `ssi_type`; `new` übernimmt die Werte ohne Bereichsprüfung; `issi` delegiert an `new` | An dieser Stelle kein Schutz gegen zu große Eingabewerte; keine Behauptung, alle vorgelagerten Aufrufer seien ungeprüft [R3] |
| `crates/tetra-config/src/bluestation/sec_directory.rs` | `bs_issi: u32`, Default `4010001`; bei 0 wird der Default eingesetzt; im gelesenen `apply_directory_patch` keine obere SSI-Bereichsprüfung | Directory-Identität ist vorhanden, aber kein generischer Zuteilungsdienst nach diesem Nummernplanentwurf [R4] |

Zusätzlicher **statischer Prüfkandidat**: Der Contract-Typ leitet `Deserialize` direkt für einen transparenten `u32`-Wrapper ab. Es ist in dieser Datei kein eigener Deserialisierungsweg über `new` erkennbar. Deshalb muss ausdrücklich geprüft werden, ob JSON-Eingaben oberhalb von 24 Bit dieselbe Prüfung durchlaufen. Das ist hier ein begründeter Code-Review-Befund, **kein ausgeführter negativer Serde-Test und kein behaupteter nachgewiesener Angriffspfad**. [R2]

### 9.2 Bereits vorhandene Rollen, aber keine pauschal aktivierte Netz-RBAC

`bins/netcore-control-room/src/auth.rs` implementiert die Rollen `Node`, `Viewer`, `Operator`, `Admin`. Die Methode `allows` bildet die Hierarchie Admin → Operator → Viewer ab; Node ist eine gesonderte technische Rolle. Außerdem existieren Benutzerstrukturen, Login und HTTP-/WebSocket-Autorisierungsfunktionen. [R5]

Das ist vorhandener Code und darf bei der Weiterentwicklung nicht ignoriert werden. Gleichzeitig bildet er nicht automatisch den vollständigen geplanten ISSI-/Eigentümer-/Gruppen-/Sonderlagenplan dieser Planung ab. In den gelesenen Identitätsstrukturen ist insbesondere keine solche komplette D/K/EE-Zuordnung erkennbar.

Wenn Authentisierung in diesem Modul deaktiviert ist, liefern Login beziehungsweise die gelesenen Autorisierungsfunktionen eine Identität `auth-disabled` mit **Admin**-Rolle. `main.rs` enthält die Schalter `--auth-enabled` und `--no-auth`, übergibt sie an die Konfiguration und protokolliert den Open-Lab-Zustand. [R5][R6]

Die am Prüfdatum vorliegende `system-backend/control-room/Readme.md` beschreibt den Betriebsentwurf dieser Phase ausdrücklich ohne produktive Authentisierungs-/RBAC-Aktivierung, ohne Login, Tokens, TLS oder mTLS und ohne Zertifizierungsbehauptung. Das ist eine wichtige Einschränkung gegenüber den historischen „Profi-/BOS“-Zusicherungen. Die Dokumentation formuliert allgemein Operatorrechte für erreichbare Clients; der gelesene deaktivierte Auth-Code liefert sogar die genannte Admin-Identität. Eine vollständige Prüfung der tatsächlich erreichbaren Aktionen wurde hier nicht durchgeführt. [R7]

**Bewertung:** Rollen-/Auth-Bausteine sind **implementiert**. Ihre produktive Aktivierung, eine organisationsübergreifende ISSI-RBAC, MFA, Break-Glass und eine Funk-Ende-zu-Ende-Durchsetzung sind durch diesen Dokumentationslauf **nicht getestet und nicht im Betrieb bestätigt**.

### 9.3 Directory und Policies: vorhandene Anschlussstellen

Die Directory-Konfiguration beschreibt HTTP/JSON-Laufzeitexport, die logische Basisstationsadresse und mehrere Publish-Schalter. Die Felder `enforce_policies` und `policy_fail_closed` sind vorhanden und standardmäßig `false`. Der Kommentar zu `enforce_policies` nennt lokale SDS-/Rufaufbauten und `/api/policy-check`; der Kommentar zur Fehlerbehandlung beschreibt standardmäßig Fail-open, damit der RF-Betrieb bei Directory-Ausfall weiterläuft. [R4]

Diese Konfigurationsstelle ist ein konkreter Anknüpfungspunkt für weitere Prüfung. Sie beweist weder, dass der Schalter im eingesetzten TOML aktiviert ist, noch dass alle relevanten Codepfade vollständig durch ihn geschützt sind. Vor einer neuen „zentralen RBAC von null“ müssen vorhandene Auth-, Directory- und Policy-Mechanismen zusammengeführt und ihre Grenzen geprüft werden.

Die Directory-Readme beschreibt Geräte-, Basisstations-, Gruppen- und Status-APIs. Der zugehörige Dateibaum enthält `netcore-directory.py`; die Readme nennt in Start-/Kopierbefehlen dagegen `netcore_directory_server.py`. Dieser Namensunterschied ist ein **zum Prüfdatum festgestellter Dokumentationsprüfpunkt**, kein historischer Fehler dieser ISSI-Planung. Die Befehle wurden hier nicht ausgeführt. [R8]

### 9.4 Abgleich mit einem bereits vorhandenen anderen Projektarchiv

`Docs/archive/2026-10-03_basisstation-issi-eigentuemer-und-systemidentitaet.md` dokumentiert eine **andere** Projektfestlegung: Dort wurde `04010001` beziehungsweise numerisch `4010001` für die Basisstations-/Systemidentität gewählt und Eigentümer `01` Jan zugeordnet. Die Zuordnung ist ein ergänzender Quellenbefund. [R9]

Die Adresse passt zu `D=0/K=4/EE=01/NNNN=0001` und zum hier erneut geprüften Directory-Code-Default. Daraus entsteht jedoch keine vollständige Liste aller acht Eigentümer. Die dort beschriebenen weiteren lokalen SDS-Routen und hart codierten Quellen wurden in diesem Dokumentationslauf nicht erneut vollständig geprüft; sie bleiben verlinkte Anschlussbefunde, nicht eigene neue Testergebnisse.

## 10. Architektur, Schnittstellen und technische Parameter

### 10.1 Fachliches Zielbild aus dieser Entwicklungsphase

Die geplanten Zuständigkeiten lassen sich folgendermaßen trennen:

```text
Organisationsregister und Nummernplan
    → kontrollierte Zuteilung einer technischen Teilnehmeridentität
    → Zuordnung zu physischem/logischem Teilnehmer und Eigentümer

Rollen-/Profilverwaltung
    → Berechtigung eines Benutzers oder Dienstes
    → Prüfung einer konkreten Aktion am zuständigen Backend/Funkpfad

Audit und Lifecycle
    → Zuteilung, Änderung, Sperrung, Sonderfreigabe und Wiederverwendung
```

Dies ist ein fachliches Modell, **kein in dieser Entwicklungsphase eingerichteter neuer Dienst**. Es gab keine Festlegung auf einen zusätzlichen LXC, eine bestimmte RBAC-Software, eine Datenbankmigration oder ein verbindliches Auth-Protokoll. Der konkrete Ausbau muss vorhandene Komponenten berücksichtigen.

### 10.2 Relevante am Prüfdatum gelesene Parameter und Pfade

| Bereich | Wert / Schnittstelle | Nachweisstatus |
|---|---|---|
| Numerischer SSI-Raum | 24 Bit; Rohobergrenze `0xFFFFFF = 16777215` | Normfundstelle und Code-Konstante, keine vollständige Vergabepolicy |
| Historischer Anzeigenaufbau | `[D][K][EE][NNNN]`, acht Positionen | Angenommener Entwurf mit offenen Einschränkungen |
| Am Prüfdatum vorliegende Contract-Typen | `Ssi`, `Issi`, `Gssi` | Implementiert in `system-backend/shared/contracts/src/address.rs` |
| RF-Core-Adresse | `TetraAddress { ssi: u32, ssi_type }` | Implementiert in `crates/tetra-core/src/address.rs` |
| Directory-Standardadresse | `http://127.0.0.1:8095` | Code-Default, nicht die bestätigte Live-Adresse |
| Directory-Standard-ISSI | `4010001` | Code-Default; Anzeige im anderen Archiv `04010001` |
| Directory-Timeout | Default `1500 ms`, Begrenzung auf `250–10000 ms` | In `sec_directory.rs` geprüft |
| Directory-Policy-Schalter | `enforce_policies=false`, `policy_fail_closed=false` als Defaults | Konfigurationscode, keine Aussage über aktiv ausgerollte Werte |
| Directory-Policy-Schnittstelle | `/api/policy-check` | Im Konfigurationskommentar als Anschlussstelle genannt; Ende-zu-Ende nicht getestet |
| Control-Room-WebUI | Port `9010`, WebSockets `/node` und `/ui` | Am Prüfdatum vorliegende Readme; kein Live-Portscan |
| Control-Room-Gesundheit/Status | `/health/live`, `/health/ready`, `/api/v1/control-room/overview`, `/metrics` | Am Prüfdatum vorliegende Readme, nicht aufgerufen |
| Control-Room-Benutzerrollen | `viewer`, `operator`, `admin`; technische Rolle `node` | Auth-Code geprüft |
| Control-Room-Persistenz | SQLite für Benutzer-/Audit-Anschlussstellen; weitere Zustände laut Readme in JSON | Code-/Dokumentationsbefund, keine Prüfung von Datenbestand oder Manipulationsschutz |
| Nummernplan-Seite | `wiki/ISSI-and-GSSI.md` | Vorhanden; organisatorischer Vorschlag |
| Historische Richtlinie | Canvas, interne Referenz in Abschnitt 1 | Kein unmittelbar ausgelesener am Prüfdatum vorliegender Export |
| Dokumentation | Diese Datei; `Docs/archive/README.md` | Entwurfs- und Quellenstand |

MCC/MNC, Standort, Rufzeichen, Frequenzen, Keys und Endgeräteseriennummern wurden in diesem spezifischen Nummernplanentwurf nicht verbindlich festgelegt. Werte aus anderen Projektphasen werden hier nicht als neue Anforderungen eingeführt. Die Netzidentität muss bei einer späteren Registrierung trotzdem berücksichtigt werden.

## 11. Tests, Prüfungen und Befehle

### 11.1 Historischer Stand

In den ursprünglichen Planungsunterlagen wurden keine Shellbefehle zur Installation, kein Git-Commit des Nummernplans, keine Datenbankmigration, kein Firmware-/Codeplug-Update und kein Funkversuch nachgewiesen. Die dargestellten Rechenbeispiele waren Erläuterungen, keine erfolgreichen Endgerätetests.

Dokumentiert sind lediglich die Richtlinienrevisionen aus Abschnitt 7. Die vorgeschlagenen Filter wie `^1EE` und Präfix-ACLs waren Konzeptbeispiele; die unsichere Ableitung voller Rechte aus dem Präfix ist überholt.

### 11.2 Tatsächlich bei der Quellenprüfung durchgeführt

| Prüfung/Ablauf | Ergebnis und Grenze |
|---|---|
| GitHub-Branch-, Tree- und Dateiabfragen mit explizitem `Archiving` beziehungsweise geprüftem Commit | Erfolgreich; relevante Quellstände und vorhandenen Archivindex gelesen |
| Prüfen des geplanten neuen Archivpfads | Datei war vor der Erstellung nicht vorhanden; keine fremde Zusammenfassung ersetzt |
| Anonymer `git clone --depth 1 --single-branch --branch Archiving ...` in der Arbeitsumgebung | Fehlgeschlagen: Git konnte für HTTPS keinen Benutzernamen beziehen; kein Zugriff auf Zugangsdaten versucht oder benötigt |
| Fortsetzung über verbundenen GitHub-Connector | Repository lesbar; Archivschreiben über die ausdrücklich autorisierte Contents-Schnittstelle vorgesehen |
| Zunächst `control-room/README.md` lesen | 404 wegen Dateiname; tatsächliche Datei über Tree als `Readme.md` gefunden und erfolgreich gelesen |
| PDF-Inventur | 25 Dateien, 8.061 physische Seiten einschließlich Zusammenstellung; Titel-/Versionsdaten und SHA-256-Präfixe erfasst |
| Gezielte Identitätsprüfung | EN 300 392-1, Seiten 27–30 und 37–38 gelesen; Abbildung 3, Seite 29, visuell geprüft |
| Originale eigenständige Bilddateien suchen | Keine gefunden; neu erzeugte Arbeitsansicht wird nicht als historisches Bild ausgegeben |
| Vollständige arithmetische Prüfung für acht Eigentümer | **959.904** Kombinationen geprüft; **879.912** innerhalb und **79.992** oberhalb von 24 Bit |
| Zusätzliche Randwertprüfung für `EE=01–99` | **2.376** Endpunktprüfungen; exakt die Kombination `D=1/K=9` ist für die festgelegten Klassen außerhalb der Grenze |
| Darstellungs-Roundtrip | Für alle 959.904 Kandidaten Dezimalparse und achtstelliges Zurückformatieren geprüft |
| Rust-Testlauf, Programmstart, Gerätetest, RBAC-/Mandanten-Penetrationstest | **Nicht ausgeführt** |

Die arithmetische Vollprüfung umfasst `D ∈ {0,1}`, `K ∈ {1,2,3,4,5,9}`, `EE=01–08`, `NNNN=0001–9999`. Die 79.992 ungültigen Kandidaten entsprechen acht Eigentümern mit jeweils 9.999 Nummern in `D=1/K=9`. Das Testergebnis widerlegt eine frühere Behauptung; es ist **kein bestandener Abnahmetest des gesamten Designs**.

### 11.3 Reproduzierbarer Kern der ausgeführten Zahlenprüfung

Der folgende Ablauf wurde in der temporären Arbeitsumgebung tatsächlich ausgeführt. Er ist keine neue NetCore-Laufzeitimplementierung und wird nicht als produktiver Validator installiert:

```python
MAX_SSI = (1 << 24) - 1
classes = (1, 2, 3, 4, 5, 9)
checked = valid = invalid = 0
for d in (0, 1):
    for k in classes:
        for ee in range(1, 9):
            for n in range(1, 10000):
                text = f"{d}{k}{ee:02d}{n:04d}"
                value = int(text, 10)
                assert f"{value:08d}" == text
                ok = value <= MAX_SSI
                assert ok == (d == 0 or k != 9)
                checked += 1
                valid += ok
                invalid += not ok
assert (checked, valid, invalid) == (959904, 879912, 79992)
assert int("12030042") - int("02030042") == 10000000
```

Ergänzend wurden für beide D-Werte, alle sechs Klassen und `EE=01–99` jeweils `NNNN=0001` und `NNNN=9999` sowie die genannten Einzelbeispiele geprüft. Die Prüfungen behandeln den Maximalwert absichtlich nur als Rechengrenze; Reservierungen, Belegung und Berechtigung sind nicht Bestandteil dieses kleinen Skripts.

### 11.4 Nur dokumentierte, nicht ausgeführte Betriebsbefehle

Die am Prüfdatum gelesene Control-Room-Readme zeigt `cargo build --locked --release --package netcore-control-room` und einen Start mit `--config ... --no-auth`. Diese Befehle wurden **nicht ausgeführt** und sind hier keine Empfehlung für einen produktiven Sicherheitsbetrieb. Auch Directory-Start, Seed-Import und `systemctl enable --now` aus dessen Readme wurden nicht ausgeführt; zuvor ist insbesondere der Dateinamenswiderspruch aus Abschnitt 9.3 zu klären.

## 12. Fehlerregister und Wirkung der Korrekturen

| ID | Fehler/Unklarheit | Ursache/Diagnose | Stand bzw. nächste Maßnahme |
|---|---|---|---|
| ISSI-01 | Früher VPN-Nebenentwurf außerhalb des ISSI-Themas | Thematisch unpassende Entwurfsfassung | Nicht als beschlossene ISSI-Architektur verwenden; Nebenidee in Abschnitt 13 bewahrt |
| ISSI-02 | Angeblich achtstelliges Schema hatte neun Stellen | Feldbreiten nicht addiert, inkonsistente Beispiele | Durch späteren Entwurf ersetzt |
| ISSI-03 | Viele achtstellige Beispiele größer als 24 Bit | Reiner Dezimalaufbau ohne Zahlenraumprüfung | Korrektur hat Vorrang; alte Beispiele nicht vergeben |
| ISSI-04 | Formel und Aneinanderhängen als grundsätzlich unterschiedliche Systeme dargestellt | Missverständliche Erklärung desselben Dezimalaufbaus | Präfixschreibweise ist als Bedienlogik ausreichend; Validator bleibt nötig |
| ISSI-05 | Falsche Kapazität/Maximalwerte | 0000 nicht sauber berücksichtigt; Klassen/Eigentümer nicht vollständig in Maximalwert einbezogen | 9.999 nutzbare Laufnummern bei 0001–9999; K/EE/NNNN-Maximum 9.999.999 |
| ISSI-06 | D=1/K=9 überschreitet Grenze | D vor siebenstelligen K9-Block gesetzt | **Offen, hohe Priorität**; Kombination sperren oder Entwurf ausdrücklich ändern |
| ISSI-07 | Betriebsmoduswechsel soll Identität stabil lassen, ändert aber D in der Zahl | Adressdomäne und veränderlicher Zustand vermischt | **Offen, hohe Priorität**; eindeutiges D-Modell beschließen |
| ISSI-08 | D=0-Test, D=1-Übung/KatS, K=9 und Laufnummernfenster überlappen | Mehrfache, teils gegensätzliche Betriebskennzeichnung | Test-, Einsatz- und Reservepolitik konsolidieren |
| ISSI-09 | Präfix soll volle LST-Rechte geben | Identität mit Autorisierung gleichgesetzt | Später ausdrücklich ersetzt durch Policy-/RBAC-Zuordnung |
| ISSI-10 | Pauschale Hersteller-/BOS-/Audit-Freigaben | Dokumentationssprache ohne Abnahme | Nicht übernommen; konkrete Geräte-/Sicherheitsprüfungen erforderlich |
| ISSI-11 | Canvas nicht überall umgestellt | Großer Patch fehlgeschlagen, Wiederholung änderte nur drei Abschnitte | Vollständige Rücklese/Neufassung erforderlich |
| ISSI-12 | Voller Rohzahlenraum als frei vergebbar betrachtet | Reservierte SSI und Netzkontext nicht berücksichtigt | Norm- und Netzbelegungsprüfung in Vergabeprozess aufnehmen |
| ISSI-13 | Lokale Rollen vorhanden, aber Sicherheit pauschal angenommen | Implementierung, Konfiguration und Live-Betrieb vermischt | Vorhandene Auth-Codepfade, Open-Lab und tatsächliche Aktivierung getrennt abnehmen |
| ISSI-14 | Unterschiedliche Eingangsvalidierungen im Code | Generischer u32-Core, teilweise geprüfte Wrapper, eigener Config-Pfad | Validator-/Deserialisierungsprüfung als Roadmap-Kandidat, keine Codeänderung bei der dokumentierten Prüfung |

## 13. Weitere Ideen und Wünsche, die erhalten bleiben

### 13.1 Teilnehmer und Organisation

Virtuelle Sprecher, Bots, Automatik, KI-/Analyseagenten, Gateways, Recorder und Crosspatch-/SIP-Kopplungen wurden als mögliche eigene Teilnehmerklassen beziehungsweise Profile genannt. Begriffe wie Shadow-ISSI und Virtual Speaker waren Ideen, keine spezifizierten oder durch diese Planung implementierten Dienste. Nicht jeder Backenddienst benötigt automatisch eine eigene Funkteilnehmerkennung; der jeweilige tatsächliche Sende-/Empfangsbedarf ist noch festzulegen.

Weitere Wünsche waren temporäre Gast-/Übungspools, Erweiterungsreserve für weitere Organisationen, einfache Log-/UI-Filter, eine maschinenlesbare Registratur, Zuordnung von ISSI zu Eigentümer und Verwendungszweck sowie automatisierte, konfliktfreie Vergabe. Die Reservevorschläge schwankten zwischen 10–20 Prozent und festen Nummernfenstern; es gibt keine abgeschlossene Kapazitätsrechnung oder verbindliche Quote.

### 13.2 Sicherheit und Betrieb

Erhalten bleiben die geplanten separaten Rollenprofile, Organisations-/Mandantentrennung, zeitbegrenzte Mutual-Aid-Freigaben, auditierter Break-Glass, Benutzer-/Geräte-/Leitstellenprofile, eine spätere Gruppen-/Rechtematrix und der Lebenszyklus einschließlich Verlust, Ersatz, Sperre, Stilllegung, Wiederfreigabe und dokumentierter Wiederverwendung. Das spätere GSSI-Dokument ist weiterhin ausdrücklich ein eigener Arbeitsschritt.

MFA und Break-Glass wurden nicht nach Benutzerkonsole, Geräteidentität und Funkgerät differenziert. Ein menschlicher zweiter Faktor lässt sich nicht allein dadurch realisieren, dass ein HRT eine besondere Nummer bekommt. Diese Abgrenzung muss in einer konkreten Implementierung nachgeholt werden.

Die Karteikarte als PDF, Hoch-/Querformat, Schwarz-Weiß beziehungsweise Wiki-Einseite wurde angeboten. Angefordert und im Verlauf umgesetzt wurde die kurze Textfassung im Canvas; eine tatsächlich erzeugte druckfertige PDF-Karte ist in dieser Entwicklungsphase nicht belegt.

### 13.3 Nebenidee: VPN und Netztrennung

Die erste Entwurfsfassung beschrieb ein vermeintliches Kapitel 9.4 zur VPN-Integration: Site-to-Site-, Client- und Mesh-VPN, beispielhaft WireGuard/Nebula; Verbindung von Nodes, Webpanel und App; Trennung von Produktions-, Management- und Service-VLANs; verschlüsselte Management-/Audio-/Steuerdaten; automatische Wiederverbindung; LAN plus LTE/5G; VPN-Verbindungsaufbau vor App-Nutzung; Always-on-, On-demand- und Fallback-Betrieb.

Diese Nebenidee war keinem konkreten ISSI-Anwendungsfall zugeordnet; Umsetzung und Inbetriebnahme sind nicht belegt. Sie werden ausschließlich als **unbestätigte Nebenidee** erhalten. Insbesondere „alle Komponenten ausschließlich per VPN“, vollständige Verschlüsselung oder eine ausfallsichere Mesh-Topologie dürfen daraus nicht als zum Prüfdatum bestehende Eigenschaften abgeleitet werden. Der am Prüfdatum vorliegende Control-Room-Open-Lab-Hinweis ist davon getrennt zu beachten.

## 14. Offene Aufgaben und Roadmap-Kandidaten

### 14.1 Bereits vereinbarte Reihenfolge

Ausdrücklich vereinbart sind die Konzentration auf ISSI, das Auslagern der Rolle und die spätere **separate** Behandlung von GSSI. Weitere Fristen, Zuständige oder nummerierte Prioritäten wurden nicht verbindlich vergeben. Die folgende Priorisierung ist eine **neue technische Empfehlung aus der Quellenprüfung**, keine behauptete frühere Vereinbarung.

| Priorität | Arbeitspaket | Abhängigkeit / konkretes Abnahmekriterium |
|---|---|---|
| P0 | D-Modell entscheiden | Dauerhafte Adressdomäne oder externer Betriebszustand; ein Moduswechsel darf nicht unbeabsichtigt eine neue Funkidentität erzeugen |
| P0 | Zulässige Kombinationen und reservierte Werte definieren | Kein D=1/K=9-Überlauf; normale Individualadressen, Gruppen-/Alias-/Sonderbereiche und lokale Belegungen kollisionsfrei berücksichtigen |
| P0 | Richtlinie vollständig konsolidieren | Alle 16 Kapitel, Tabellen, Beispiele und Karteikarte müssen dasselbe Schema verwenden; vollständiger Rücklese-/Diff-Nachweis statt Teilpatch-Bestätigung |
| P1 | Eigentümerregister erstellen | Acht tatsächliche Organisationen benennen, Nummern zuweisen, Betreiber/Gast/Labor/Wachstum unterscheiden; andere Archiventscheidung zu EE=01 einbeziehen |
| P1 | Identitäts- und Lifecycle-Modell bestimmen | Physisches Gerät, logischer Teilnehmer, Benutzer und Eigentümer auseinanderhalten; Tausch, Verlust, Umwidmung, Transfer und Wiederverwendung regeln |
| P1 | Vorhandene Validierungswege vereinheitlichen | Contract-Konstruktor, JSON-Deserialisierung, Directory-bs_issi, RF-Core und Eingabeoberflächen prüfen; negative Tests für Überlauf/Reservierungen/Formatfehler ergänzen |
| P1 | Vorhandene Rollen und Policies auf das Zielmodell abbilden | Control-Room-Auth nicht neu erfinden; Netz-/API-/Geräteberechtigung, Organisationen und autoritative Zuständigkeiten explizit verdrahten |
| P1 | Produktiv-/Open-Lab-Grenze und Ausfallverhalten festlegen | Aktivierung von Auth/Transportabsicherung, Fail-open/Fail-closed und Rechte bei Backendausfall bewusst entscheiden und testen |
| P1 | Vergabeverfahren implementieren | Atomare Reservierung und Freigabe; doppelte Parallelvergabe verhindern; Anzeige mit/ohne Null darf keine zweite Identität erzeugen |
| P2 | LST-Rechtebereich/MFA/Break-Glass ausarbeiten | Globale und eigene Organisationsrechte trennen; Ablaufzeit, Widerruf, Alarmierung und Nachprüfung nachweisen |
| P2 | Test-, Übungs-, Reserve- und reale Sonderlagenpolitik trennen | Eindeutige Vorrangregeln; keine Rechteeskalation nur wegen eines Präfixes oder eines Ausfalltriggers |
| P2 | Endgerätetests und Migrationsplan | Repräsentative Motorola-/Sepura-Geräte, konkrete Softwarestände; Registrierung, SDS, Einzel-/Gruppenruf, Anzeige und Rechte mit alter/neuer Nummer prüfen |
| P2 | Einheitliche Systemteilnehmer-ISSIs und Routing prüfen | `4010001` und andere bereits verwendete Quellen/Ziele inventarisieren; keine ungeprüfte Umnummerierung von Dashboard, Audio, Gateways oder Directory |
| P3 | GSSI separat fertigstellen | Eigene Gruppenrichtlinie, aber abgestimmter SSI-Raum und spätere Rechte-/Gruppenmatrix |
| P3 | Karteikarte/Wiki/PDF erzeugen | Erst nach konsolidierter Regelversion; identische Beispiele und sichtbare Gültigkeitsgrenzen |

### 14.2 Empfohlene Mindesttests für die Fortsetzung

Noch nicht durchgeführt sind insbesondere Tests der tatsächlichen Eingangswege mit Werten oberhalb von 24 Bit, Sonder-/Broadcastwerten, `NNNN=0000`, unbekannten Organisationen und Mehrfachvergabe. Ergänzend müssen sieben-/achtstellige Anzeigevarianten auf identische kanonische Werte abgebildet werden.

Für die Berechtigungsabnahme sind mindestens Rollenwechsel bei gleichbleibender Identität, unzulässiger organisationsübergreifender Zugriff, fehlende Authentisierung, Testteilnehmer auf produktiven Zielen, abgelaufene Sonderfreigabe, Directory-/Auth-Ausfall und Audit-Nachvollziehbarkeit zu prüfen. Ein erfolgreicher HTTP-Test ersetzt dabei nicht die Prüfung des tatsächlich zuständigen Funk-/SwMI-Codepfads und des verwendeten Endgeräts.

Eine Migration muss bestehende Teilnehmer- und Dienstadressen, Rufziele, Kontakte, SDS-Automationen, Logs, Whitelists und Berechtigungszuordnungen inventarisieren. Solange D und die Rolle von Eigentümerwechseln nicht geklärt sind, gibt es keine belastbare allgemeine Umnummerierungsanleitung.

## 15. Quellen, Repository-Belege und Anhänge

### 15.1 Repository-Quellen dieser Prüfung

Die folgenden Links sind auf den überprüften Quellcommit fixiert. Eine Readme oder ein früheres Archiv ist als Dokumentationsquelle gekennzeichnet und kein Ersatz für einen eigenen Code- oder Laufzeittest.

- **[R1]** [wiki/ISSI-and-GSSI.md](https://github.com/JanHG98/netcore-tetra/blob/e1a3bbcdce7c25015882126a8371642a55eb206b/wiki/ISSI-and-GSSI.md) — vollständig gelesen; organisatorischer Vorschlag, System-ISSIs und Vergabehinweise.
- **[R2]** [system-backend/shared/contracts/src/address.rs](https://github.com/JanHG98/netcore-tetra/blob/e1a3bbcdce7c25015882126a8371642a55eb206b/system-backend/shared/contracts/src/address.rs) — vollständig gelesene Datentypen, Konstruktor-/Parse-Prüfung, Serde-Ableitung und vorhandene Tests.
- **[R3]** [crates/tetra-core/src/address.rs](https://github.com/JanHG98/netcore-tetra/blob/e1a3bbcdce7c25015882126a8371642a55eb206b/crates/tetra-core/src/address.rs) — vollständig gelesen; generischer u32-Adresscontainer.
- **[R4]** [crates/tetra-config/src/bluestation/sec_directory.rs](https://github.com/JanHG98/netcore-tetra/blob/e1a3bbcdce7c25015882126a8371642a55eb206b/crates/tetra-config/src/bluestation/sec_directory.rs) — vollständig gelesen; bs_issi, Directory-Defaults, Timeouts und Policy-Schalter.
- **[R5]** [bins/netcore-control-room/src/auth.rs](https://github.com/JanHG98/netcore-tetra/blob/e1a3bbcdce7c25015882126a8371642a55eb206b/bins/netcore-control-room/src/auth.rs) — gezielt Zeilen 1–165 und 180–370 gelesen; Rollen, Identitäten und Authentisierungs-/Autorisierungsfunktionen. Keine vollständige Prüfung aller Passwort-/Persistenzfunktionen.
- **[R6]** [bins/netcore-control-room/src/main.rs](https://github.com/JanHG98/netcore-tetra/blob/e1a3bbcdce7c25015882126a8371642a55eb206b/bins/netcore-control-room/src/main.rs) — Zeilen 1–160 gelesen; CLI, Initialisierung und Open-Lab-Protokollierung.
- **[R7]** [system-backend/control-room/Readme.md](https://github.com/JanHG98/netcore-tetra/blob/e1a3bbcdce7c25015882126a8371642a55eb206b/system-backend/control-room/Readme.md) — vollständig gelesene Betriebs-/Architekturdokumentation; Port/Endpunkte, Open-Lab und bewusste Grenzen.
- **[R8]** [system-backend/directory/Readme.md](https://github.com/JanHG98/netcore-tetra/blob/e1a3bbcdce7c25015882126a8371642a55eb206b/system-backend/directory/Readme.md) — vollständig gelesene API-/Installationsdokumentation; mit Dateibaum auf Namensabweichung geprüft, Dienstcode nicht vollständig geprüft.
- **[R9]** [Anderes Projektarchiv: Basisstations-ISSI und Eigentümer](2026-10-03_basisstation-issi-eigentuemer-und-systemidentitaet.md) — ergänzende historische Quelle; bei der dokumentierten Prüfung nicht verändert. Die dortigen eigenen Tests oder main-Vergleiche werden nicht als Tests der dokumentierten Prüfung ausgegeben.
- **[R10]** [Archivindex vor dieser Ergänzung](https://github.com/JanHG98/netcore-tetra/blob/e1a3bbcdce7c25015882126a8371642a55eb206b/Docs/archive/README.md) — vorhandene fremde Einträge als zu erhaltender Ausgangsbestand.

Für diese historische ISSI-Planung wurden keine zugehörigen Implementierungs-PRs oder Commitnummern genannt. Solche Nummern werden nicht nachträglich erfunden. Die hier angegebene Quellcommitnummer ist ausschließlich der bei der Quellenprüfung untersuchte Stand.

### 15.2 Gezielt verwendete Normfundstelle

**[N1] `en_30039201v010601p.pdf` — ETSI EN 300 392-1 V1.6.1 (2020-04), General network design.**

Relevante Stellen: Abschnitt 7.2.1, Seite 27 (48-/24-Bit-Identitäten und Eindeutigkeit); 7.2.2, Seite 28 (TSI-Familien, keine standardisierte dynamische ITSI-Zuteilung); 7.2.3 und 7.2.4 einschließlich Abbildung 3, Seite 29 (ISSI, Netzkontext und Feldaufteilung); Fortsetzung 7.2.5, Seite 30 (Vergabe und langfristige ISSI-Zuordnung); 7.7.8, Seiten 37–38 (reservierte Broadcast-SSI). Seitenzahlen beziehen sich auf die bereitgestellte PDF-Fassung.

Es wurde keine Behauptung aufgestellt, diese bereitgestellten Fassungen seien am Archivdatum jeweils die neueste gültige Normausgabe. Als Draft beziehungsweise Final draft bezeichnete Dokumente bleiben entsprechend gekennzeichnet.

### 15.3 Vollständiges Inventar der bereitgestellten PDFs

`SHA-256-Präfix` bezeichnet jeweils die ersten 16 Hex-Zeichen der bei der Quellenprüfung berechneten SHA-256-Prüfsumme; es ist kein vollständiger kryptografischer Integritätsnachweis. „Inventar“ bedeutet Titel-/Versions-/Dateiprüfung, nicht vollständiges Lesen jeder Seite.

| Datei | Ausgabe / Thema | Seiten | SHA-256-Präfix | Quellenprüfung bei der dokumentierten Prüfung |
|---|---|---:|---|---|
| `en_30039201v010601p.pdf` | EN 300 392-1 V1.6.1, 2020-04; General network design | 182 | `788722e566098957` | Identitätsabschnitte gezielt gelesen, Abbildung 3 visuell geprüft |
| `en_30039202v030801p.pdf` | EN 300 392-2 V3.8.1, 2016-08; Air Interface | 1445 | `3f07b1e4ad73fabc` | Inventar; keine vollständige PDU-/RF-Prüfung |
| `en_3003920303v010301p.pdf` | EN 300 392-3-3 V1.3.1, 2011-11; ANF-ISIGC | 251 | `94ca61038b3ff826` | Inventar |
| `en_3003920304v010301p.pdf` | EN 300 392-3-4 V1.3.1, 2010-08; ANF-ISISDS | 28 | `8a38cc6238c6da72` | Inventar |
| `en_3003920308v010401p.pdf` | EN 300 392-3-8 V1.4.1, 2020-04; Generic Speech Format | 22 | `4d993a25e35da7b1` | Inventar |
| `en_3003920313v010201p.pdf` | EN 300 392-3-13 V1.2.1, 2020-04; transportunabhängiger Gruppenruf | 191 | `b47d63853f83a6a4` | Inventar |
| `en_3003920315v010500a.pdf` | **Draft** EN 300 392-3-15 V1.5.0, 2026-04; Mobility Management | 380 | `e959709667192e22` | Inventar; Draft nicht als endgültige Ausgabe behandelt |
| `en_30039205v020701p.pdf` | EN 300 392-5 V2.7.1, 2020-04; PEI | 320 | `10aaf78988ae4080` | Inventar; einzelne SSI/ITSI-Suchtreffer, keine Schnittstellenabnahme |
| `en_30039207v030501p.pdf` | EN 300 392-7 V3.5.1, 2019-07; Security | 216 | `df47af9a642b6eba` | Inventar; kein Sicherheitskonformitätstest |
| `en_30039209v010701p.pdf` | EN 300 392-9 V1.7.1, 2020-04; Supplementary Services | 46 | `cad45938bcdf2fa4` | Inventar |
| `en_3003921006v010401p.pdf` | EN 300 392-10-6 V1.4.1, 2006-08; Call Authorized by Dispatcher | 20 | `32b6ee43602ef88a` | Inventar; Existenz einer Norm ist kein Nachweis dieses Dienstes im Netz |
| `en_3003921018v010301p.pdf` | EN 300 392-10-18 V1.3.1, 2003-10; Barring of Outgoing Calls | 17 | `4cc10bcd94d39201` | Inventar |
| `en_3003921101v010201p.pdf` | EN 300 392-11-1 V1.2.1, 2004-01; Call Identification, Stage 2 | 44 | `852c17ea566757e8` | Inventar |
| `en_3003921114v010101p.pdf` | EN 300 392-11-14 V1.1.1, 2002-07; Late Entry | 23 | `ba1882df71dbb84a` | Inventar |
| `en_3003921117v010102p.pdf` | EN 300 392-11-17 V1.1.2, 2002-01; Include Call | 18 | `69ce800e352b1ecf` | Inventar |
| `en_3003921201v010202p.pdf` | EN 300 392-12-1 V1.2.2, 2007-08; Call Identification, Stage 3 | 56 | `4d5b56a188fb73c4` | Inventar |
| `en_3003921216v010400a.pdf` | **DRAFT** EN 300 392-12-16 V1.4.0, 2026-03; Pre-emptive Priority Call | 67 | `c0ee7859f448c407` | Inventar; Draft nicht als endgültige Ausgabe behandelt |
| `en_30039401v030301p.pdf` | EN 300 394-1 V3.3.1, 2015-04; Radio Conformance Testing | 169 | `2d9c327e5ccf1476` | Inventar; keine dieser RF-Prüfungen ausgeführt |
| `en_30039502v010303p.pdf` | EN 300 395-2 V1.3.3, 2025-02; TETRA Codec | 94 | `ac716ca18082cc0f` | Inventar |
| `en_300812v020101p.pdf` | EN 300 812 V2.1.1, 2001-12; SIM-ME | 156 | `196effeaae473a98` | Inventar |
| `es_20081201v020205p.pdf` | ES 200 812-1 V2.2.5, 2003-12; UICC | 8 | `346dc545049a7397` | Inventar |
| `es_20081202v020401m.pdf` | **Final draft** ES 200 812-2 V2.4.1, 2005-08; TSIM application | 139 | `330f045a908eefd2` | Inventar |
| `ts_10081201v020205p.pdf` | TS 100 812-1 V2.2.5, 2003-10; UICC | 8 | `96df75f5ddf1c3ae` | Inventar |
| `ets_30039214e01v.pdf` | **Final draft prETS** 300 392-14, 1997-09; PICS | 61 | `2b703f3ebb883c90` | Inventar; kein ausgefüllter Konformitätsnachweis des Projekts |
| `ETSI.pdf` | Zusammenstellung; beginnt mit EN 300 812 V2.1.1 | 4100 | `9434dad1e7bc80ca` | Inventar; vollständige Zusammensetzung und Dubletten nicht semantisch abgeglichen |

## 16. Wiederaufnahme und Karteikartenstand

### 16.1 Ausgangspunkt für den nächsten Bearbeiter

Nicht mit einem neuen freien achtstelligen Schema beginnen und nicht erneut Rollen als zwingende Ziffer einbauen. Die Anforderungen „acht Eigentümer, getrennte HRT-/MRT-Blöcke, jede Organisation mit eigenen LST, Rolle außerhalb der ISSI“ bleiben erhalten. Zuerst müssen D und die zulässigen Kombinationen geklärt werden. Danach sind der Canvas-Text, das organisatorische Wiki und die tatsächlichen Eingabe-/Vergabewege gemeinsam zu konsolidieren.

Der zusätzliche Bestand ist kein leeres Blatt: numerische Contracts, Directory-Identität und Control-Room-Rollen existieren. Gleichzeitig ist Open-Lab nicht mit produktiver Authentisierung gleichzusetzen. Die produktive Aktivierung bleibt gesondert zu prüfen.

### 16.2 Historische Karteikarte mit notwendigen Warnhinweisen

```text
HISTORISCHER ENTWURF — NOCH NICHT VOLLSTÄNDIG FREIGEGEBEN

Anzeige: [D][K][EE][NNNN]
D: 0 Produktiv; 1 Sonderbetrieb — Bedeutung noch konsolidieren.
K: 1 LST | 2 HRT | 3 MRT | 4 Infra | 5 Virtuell | 9 Test.
EE: 01–08 Organisationen; Namen/Zuteilung noch vervollständigen.
NNNN: vierstellige Laufnummer; keine Rolle in der Nummer.

ISSI = Teilnehmeridentität. RBAC = separat zugeordnete Rechte.
02030042 → numerisch 2030042.

ACHTUNG: D=1/K=9 überschreitet 24 Bit.
D-Wechsel in der Zahl = andere ISSI, nicht bloß anderer Modus.
16777215 ist Rohobergrenze und als Broadcast-SSI reserviert.
```

Diese ergänzten Warnhinweise stammen aus der Quellenprüfung; sie werden nicht als bereits im historischen Canvas vorhandener Text ausgegeben. Eine wirklich druckfertige verbindliche Karteikarte sollte erst nach den P0-Entscheidungen erstellt werden.

---

**Arbeitsstand:** Numerische Contracts, Directory-Identität und lokale Rollen sind vorhanden. Offen bleiben ein konsistenter Nummernplan, zulässige Sonderbetriebsnummern, netzweite Berechtigungsdurchsetzung und Geräteabnahme.
