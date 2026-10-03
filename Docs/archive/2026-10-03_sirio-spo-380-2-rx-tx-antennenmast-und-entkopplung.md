# Abschlussdokumentation: Sirio SPO 380-2 – RX/TX-Antennen an einem gemeinsamen Mast

> **Ergebnis des Chats:** Zwei getrennte Sirio SPO 380-2 sollen an einem einfachen, bezahlbaren Mast räumlich versetzt montiert werden. Ein Duplexer ist ausdrücklich nicht gewünscht. Konkrete Einbaumaße, ausreichende TX/RX-Isolation und störungsfreier gleichzeitiger Betrieb sind **nicht nachgewiesen**. Der zwischenzeitlich empfohlene einfache Viertelwellen-Koaxstub wurde im Chat ausdrücklich zurückgenommen und ist **keine gültige Bauempfehlung**.

## 1. Metadaten und Geltungsbereich

| Feld | Inhalt |
|---|---|
| Projekt | NetCore-Tetra |
| Thema | Kostengünstige gemeinsame Mastmontage zweier Rundstrahler für getrennten RX/TX-Betrieb mit 10 MHz Frequenzabstand; Entkopplung, Filteralternativen und Messplanung |
| Ursprünglicher Chattitel | Nicht verlässlich verfügbar; der Titel dieses Dokuments ist ein neu vergebener Archivtitel. |
| Ursprünglicher Chatlink | Nicht verfügbar; kein Link rekonstruiert oder erfunden. |
| Historischer Gesprächszeitraum | Für die einzelnen Nachrichten sind im verfügbaren Verlauf keine verlässlichen Zeitstempel enthalten. Das Dateidatum bezeichnet die Archivierung, nicht den Beginn der Antennendiskussion. |
| Erstellungsdatum | **2026-10-03** |
| Repository | `JanHG98/netcore-tetra` |
| Ausschließlicher Schreibbranch | **`Archiving`** |
| Geprüfter Repository-Snapshot | **`3768905344f885974f0dbf061eb7846ae6675501`** |
| Tree des geprüften Snapshots | `7c20365d69daae0bbaa94546bc586b6c22b751b3` |
| Ablage | `Docs/archive/2026-10-03_sirio-spo-380-2-rx-tx-antennenmast-und-entkopplung.md` |
| Zugehöriger Index | `Docs/archive/README.md` |
| Gegenstand des Archivauftrags | Diese Markdown-Datei und ihr Indexeintrag; keine Änderungen an Konfiguration, Quellcode, Wiki oder Hardware. |
| Archivierungscommit | Über die Git-Historie dieser Datei und die Abschlussmeldung bestimmbar. Die oben genannte SHA ist der **geprüfte Ausgangsstand**, nicht eine vorab behauptete SHA dieses neuen Dokuments. |

**Eindeutige Gesprächsanker:** Die Ausgangsfrage lautet sinngemäß, wie zwei Rundstrahlantennen – eine RX, eine TX, 10 MHz Offset – an einem einzigen Mast beziehungsweise Leerrohr störungsarm angebracht werden können. Später nennt Jan das Modell „sirio spo 380-2“, lehnt einen Duplexer ausdrücklich ab und beendet die technische Diskussion mit „also doch antennen versetzt“.

### 1.1 Quellen- und Statuskonvention

Dieses Archiv unterscheidet:

- **Historischer Chatstand:** Aussagen und Entscheidungen aus dem hier sichtbaren Gespräch. Eine Empfehlung des Assistenten ist weder eine Nutzerentscheidung noch ein Umsetzungsnachweis.
- **Zusätzliche Prüfung am 2026-10-03:** Gelesene Repository-Dateien, gezielt eingesehene Herstellerunterlagen und relevante Abschnitte der bereitgestellten ETSI-PDFs. Diese Befunde werden nicht rückwirkend als damaliges Wissen ausgegeben.
- **Neue Ableitung beziehungsweise Roadmap-Kandidat:** Aus den offenen Punkten abgeleitete Arbeitsschritte; keine bereits beauftragte Implementierung.

Statusbegriffe: **Idee** = diskutiert; **beschlossen/geplant** = gewünschte Richtung, noch ohne Umsetzung; **implementiert** = anhand eines konkreten Artefakts belegt; **getestet** = tatsächlich ausgeführter Test mit Ergebnis; **im Betrieb bestätigt** = ausdrücklicher, nachvollziehbarer Betriebsnachweis. Eine vorhandene Dokumentation oder Konfigurationsdatei belegt allein keinen laufenden Anlagenzustand.

### 1.2 Umfang und Auswertungslücken

Ausgewertet wurde der sichtbare Verlauf von der ersten Mastfrage bis zur abschließenden Rückkehr zur versetzten Montage, einschließlich der späteren Korrekturen. Zusätzlich stehen 25 PDF-Anhänge im Projektkontext zur Verfügung; ihr Inventar und die tatsächlich eingesehenen relevanten Stellen sind in Abschnitt 12 aufgeführt.

Es liegen **keine Fotos des realen Mastaufbaus, keine bemaßte Montagezeichnung, keine S-Parameter-Dateien, keine Spektrummessungen und keine TX-an/TX-aus-Abnahmeprotokolle** für diesen Aufbau vor. Der Besitz eines NanoVNA-H4 wurde in früheren Assistentenantworten vorausgesetzt, im sichtbaren Nutzerverlauf aber nicht ausdrücklich bestätigt. Gleiches gilt für die genaue SDR-/PA-Ausführung und die reale Sendeleistung dieses Aufbaus.

Andere Projektchats werden nicht als Teil dieses Antennenchats ausgegeben. Die zusätzlich gelesenen Wiki-Seiten dienen dem heutigen Repository-Abgleich. Die vollständige ETSI-Sammlung wurde nicht Seite für Seite technisch geprüft; insbesondere `ETSI.pdf` ist kein vollständig ausgewerteter Nachweisbestand.

## 2. Ziel, Ausgangslage und Anforderungen

Jan möchte zwei vorhandene beziehungsweise für den Aufbau festgelegte Rundstrahlantennen an einem einzigen einfachen Mast betreiben: eine als Sendeantenne, eine als Empfangsantenne. Angestrebt sind brauchbare Funkleistung und möglichst geringe Selbststörung ohne teuren professionellen Antennenträger.

Die Nutzeranforderungen wurden im Verlauf zunehmend konkret:

| Anforderung | Verbindlichkeit / Stand |
|---|---|
| Zwei Antennen vom Typ **Sirio SPO 380-2** | Vom Nutzer ausdrücklich genannt; tatsächliche Montage nicht belegt. |
| Getrennte RX- und TX-Antennen | Ausgangskonzept und abschließend beibehaltene Richtung. |
| Frequenzabstand **10 MHz** | Vom Nutzer vorgegeben. |
| Im Gespräch verwendetes Frequenzpaar RX etwa **408 MHz**, TX etwa **418 MHz** | Vom Assistenten im Projektkontext verwendet; heutige Repository-Konfiguration bestätigt diese Werte als eingetragenes Beispiel, nicht als Live-Messung. |
| Ein gemeinsamer einfacher Mast / Teleskoprohr | Nutzeranforderung. Anfangs als „Leerrohr“ beschrieben; Material, Hersteller, Länge und Durchmesser bleiben offen. |
| Wenig vertikaler Platz | Ausdrückliche Einschränkung; mehrere Meter Stapelabstand sind für den gewünschten Aufbau nicht praktikabel. |
| Kostengünstig und einfach | Ausdrücklicher Wunsch; „keine tausende Euro“. Kein konkretes Budgetlimit vereinbart. |
| **Kein Duplexer** | Ausdrückliche Ausschlussentscheidung. Spätere Empfehlungen dürfen nicht stillschweigend wieder auf eine gemeinsame Antenne mit Duplexweiche zurückgehen. |
| Einstellbarer einzelner Passfilter als Alternative | Erfragt und technisch diskutiert; kein Kauf oder konkretes Modell beschlossen. |
| Zunächst räumlich versetzte Montage | Abschließende Planungsrichtung. Kein Nachweis, dass allein dieser Schritt ausreicht. |

Nicht festgelegt sind unter anderem die verfügbare Höhe und Breite, der maximal zulässige Ausleger, die Mastabspannung, Kabeltypen/-längen, die TX-Leistung, die Empfänger-Großsignalfestigkeit und ein akzeptierter Empfindlichkeitsverlust.

## 3. Historischer Entscheidungsverlauf und Korrekturen

### 3.1 Von vertikalem Stacking zum kompakten Versatz

Zunächst empfahl der Assistent eine weitgehend übereinander liegende Anordnung, RX oben und TX darunter. Genannt wurden 1,5 m als vermeintlicher Mindestwert, besser 2–3 m und etwa 2–2,5 m als Ziel. Jan erklärte, dass dafür vertikal nicht genug Platz vorhanden ist.

Daraufhin wurden seitliche Ausleger und kombinierter horizontaler/vertikaler Versatz vorgeschlagen. Die folgenden Maße sind **historische Entwurfsideen, keine berechneten oder getesteten Mindestabstände**:

| Historischer Vorschlag | Genannte Größenordnung | Späterer Stand |
|---|---|---|
| Deutlich übereinander montieren | 1,5 m bis 3 m und mehr; Ziel etwa 2–2,5 m | Für Jans Platzverhältnisse verworfen. |
| Langer seitlicher Ausleger mit kleinem Höhenversatz | 70–100 cm horizontal und 30–50 cm vertikal; häufig 80/40 cm | Als Kompromiss diskutiert, nicht bemaßt freigegeben. |
| Besonders kompakte seitliche Anordnung | 50–60 cm horizontal, 20–30 cm vertikal | Nur als Versuch genannt; keine belastbare „absolute Unterkante“. |
| Symmetrische Quertraverse | 80–100 cm Gesamtlänge, RX links und TX rechts auf gleicher Höhe | Mechanisch ausgewogener, funktechnisch zunächst zu optimistisch dargestellt; nach Nutzerkritik korrigiert. |
| Kürzerer Ausleger mit größerem Höhenversatz | 30–50 cm horizontal, 70–100 cm vertikal; Beispiel 80/40 cm | Später bevorzugter Entwurf, aber weiterhin abhängig vom tatsächlich verfügbaren Platz. |
| Letzte Assistentenempfehlung | 60–100 cm Höhenversatz plus 30–50 cm seitlicher Abstand | Letzter Vorschlagsstand, **keine bestätigten Einbaumaße**. |

Die wiederholte Nennung eines Höhenversatzes bis 100 cm löst Jans Platzproblem nicht automatisch. Bei einer Fortsetzung zuerst messen, was wirklich hineinpasst; nicht dieselben Idealmaße erneut als bereits akzeptierten Plan voraussetzen.

### 3.2 Symmetrische Quertraverse: Mechanik ist nicht HF-Isolation

Der Assistent stellte eine mittig aufgesetzte 80–100-cm-Traverse zunächst als günstige, mechanisch ausgewogene Lösung vor. Jan fragte ausdrücklich, ob die beiden Antennen sich dabei nicht „beißen“.

Die anschließende Korrektur war wesentlich: Die mechanische Symmetrie sagt nichts über ausreichende TX/RX-Entkopplung aus. Zwei vertikal ausgerichtete Rundstrahler auf gleicher Höhe stehen im horizontalen Hauptabstrahlbereich des jeweils anderen. Aus 80–100 cm Querabstand darf daher weder „störungsfrei“ noch „repeater-tauglich“ abgeleitet werden.

**Bewahrter Endstand:** Eine Quertraverse ist nicht prinzipiell unzulässig, aber auch kein Nachweis ausreichender Isolation. Ein kombinierter Versatz ist der bevorzugte Versuch. Ob er besser oder ausreichend ist, muss am vollständigen Aufbau gemessen werden.

### 3.3 Filterdiskussion und ausdrückliche Duplexer-Ablehnung

Als Filterlösung wurden zunächst Duplexer mit einer gemeinsamen Antenne empfohlen, unter anderem Procom DPF 70/6-9/13-N(f), DPF UHF/33-DR 3000-9/13 und eine größere Comprod-Alternative. Jan stellte anschließend ausdrücklich klar, dass ein Duplexer vermieden werden soll.

Danach wurde korrekt zwischen einer Dreitor-Duplexweiche und einem einzelnen Zweitorfilter in der RX-Leitung unterschieden. Diskutiert wurden schmale, abstimmbare Cavity-/Helix-Bandpässe, Notchfilter sowie Pass-/Reject-Filter. Ein zusätzlicher Filter im TX-Zweig blieb eine denkbare Maßnahme gegen Senderrauschen im Empfangsband.

Die früheren Formulierungen „Filter sind Pflichtprogramm“ beziehungsweise „ein bestimmter Filter ist genau erforderlich“ waren ohne reale TX-Leistung, gemessene Entkopplung und Empfängerdaten zu pauschal. Die letzte Gesprächsrichtung war **erst Geometrie und Messung, dann gegebenenfalls gezielte Filterung**. Umgekehrt ist daraus keine Zusage für filterfreien Dauerbetrieb abzuleiten.

### 3.4 Ausdrücklich zurückgezogener Billigvorschlag: offener λ/4-Koaxstub

Zwischenzeitlich wurde ein T-Stück in der RX-Leitung mit einem offenen Viertelwellen-Koaxstummel als günstige Sperre bei 418 MHz empfohlen. Genannt wurden ungefähr 11,8 cm bei Verkürzungsfaktor 0,66 beziehungsweise 12,5 cm bei 0,695, ein längerer Startzuschnitt sowie schrittweises Kürzen. Auch die Kaskadierung zweier Stubs wurde vorgeschlagen.

**Diese Anleitung ist überholt und darf nicht als aktive Bauanweisung übernommen werden.** Der Assistent korrigierte sich später: Bei nur 10 MHz Abstand liegt 408 MHz noch sehr nahe an der Viertelwellenresonanz für 418 MHz. Der unmittelbar am T-Stück parallel angeschlossene Stub belastet daher auch den Nutzkanal erheblich. Die anfängliche Erwartung, 408 MHz nahezu ungedämpft zu erhalten, war für genau diesen einfachen Aufbau nicht gerechtfertigt.

Im Chat wurde dazu die elektrische Länge bei 408 MHz mit etwa `90° × 408/418 = 87,8°` erläutert. Das ist eine theoretische Begründung, **kein gemessenes Filterergebnis**. Die damaligen Zuschnittangaben und das erwähnte Materialbudget von 10–30 Euro dokumentieren lediglich den verworfenen Ansatz. Ein zweiter unberechnet kaskadierter Stub ist keine bestätigte Reparatur dieses Problems.

### 3.5 Verlinktes Elecbee-UAF42-Modul

Jan verlinkte eine Elecbee-Platine mit der Bezeichnung „UAF42 … high pass low pass bandpass … frequency gain Q adjustable“. Die Platine wurde als ungeeignet für die 408/418-MHz-Antennenleitung eingeordnet.

Der entscheidende Unterschied: „Bandpass“ bezeichnet eine Filterfunktion, nicht automatisch einen HF-tauglichen 50-Ω-Antennenfilter. Der UAF42 ist ein aktiver Analogfilter für wesentlich niedrigere Frequenzen. Die Ablehnung wurde bei der Archivierung anhand des TI-Datenblatts überprüft; siehe Abschnitt 8.3. Ein Kauf oder Aufbau mit diesem Modul ist nicht dokumentiert.

### 3.6 Letzte Festlegung

Nach Diskussion der professionellen Filterprodukte kehrte Jan mit „also doch antennen versetzt“ zur mechanischen Entkopplung zurück. Der Assistent empfahl als Ablauf: maximal praktikabler Versatz, passive S21-Messung, Vergleich der Empfangsleistung mit und ohne eigenen Sender und erst bei Bedarf Filter nachrüsten.

**Damit abgeschlossen ist die Entscheidung über die zuerst zu untersuchende Richtung, nicht die technische Abnahme der Anlage.**

## 4. Architektur und Komponenten

### 4.1 Gewünschter funktionaler Aufbau

```text
TBS / SDR TX, etwa 418 MHz
    │
    ├── optional später: passend ausgelegter TX-Filter
    │
    └── eigene 50-Ω-Koaxleitung ── TX-Sirio SPO 380-2
                                      ⋮
                             unerwünschte HF-Kopplung
                                      ⋮
RX-Sirio SPO 380-2 ── eigene 50-Ω-Koaxleitung
    │
    ├── optional später: RX-Bandpass oder TX-Frequenz-Notch
    │
    └── TBS / SDR RX, etwa 408 MHz

Mechanisch: ein gemeinsamer Mast, beide Antennen vertikal,
            möglichst geeigneter räumlicher Versatz.
Elektrisch: zwei getrennte Antennenpfade, kein gemeinsamer ANT-Port.
```

Die optionalen Filter sind **Ideen für eine spätere bedarfsabhängige Ergänzung**, keine bestellte Ausstattung. Ein Filter gegen starke Fremdsignale gehört vor die dadurch gefährdete beziehungsweise übersteuerbare aktive RX-Stufe; eine konkrete LNA-Anordnung wurde hier nicht festgelegt.

### 4.2 Antenne: im Chat genannt, bei Archivierung abgeglichen

Sirio beschreibt die SPO 380-2 als vertikal polarisierten Breitband-Dipol. Für den historischen Katalogstand sind folgende Daten nachvollziehbar [M1, M2]:

| Parameter | Herstellerangabe / Einordnung |
|---|---|
| Bauart | Dipol, Rundstrahlung horizontal |
| Frequenzbereich | 380–470 MHz bei SWR ≤ 1,5 |
| Impedanz / Anschluss | 50 Ω, N-Buchse |
| Gewinn | 0 dBd / 2,15 dBi |
| Vertikale Halbwertsbreite | Etwa 78° |
| Länge / Masse | Etwa 780 mm / 890 g je Antenne |
| Leistungsangabe | 75 W CW bei 30 °C Umgebung; nicht mit der realen TBS-Sendeleistung verwechseln. |
| DC-Verhalten | DC-geerdete Ausführung; Innenleiter erscheint bei Gleichstrom kurzgeschlossen. Das ist für dieses Modell nicht automatisch ein Defekt. |
| Mastaufnahme | Historischer Katalog: 35–54 mm. Der separat abrufbare Handbuchtext nennt 35–60 mm; konkrete mitgelieferte Halterrevision prüfen. |

Beide Frequenzbereiche des Projekts liegen innerhalb des Antennenbandes. Die Zuweisung „RX-Antenne“ und „TX-Antenne“ macht aus zwei identischen Breitbandantennen keine selektiven Filter. Aus SWR oder Bandabdeckung lässt sich aber auch kein exakter Kopplungswert zwischen zwei montierten Antennen ableiten.

### 4.3 Mechanischer Vorschlagsstand

Vorgesehen ist grundsätzlich eine obere, möglichst freie RX-Position und eine tiefere, seitlich versetzte TX-Position. Diese Bevorzugung ist **keine allgemeingültige Funkregel**; sie muss zur tatsächlichen Umgebung und Linkbilanz passen. Die frühere Begründung, der Sender habe „genug Leistung“ und könne eine schlechtere Position problemlos verkraften, wurde nicht durch Messwerte belegt.

Als preiswerte Bauteilideen wurden Teleskop-/Steckmast, Alu-Vierkantprofil, Mastschellen/U-Bügel und kurze Ausleger genannt. Historische Beispielmaße waren 40–50 mm Mastdurchmesser beziehungsweise ein 48-mm-Mast und ein 40 × 40 × 2-mm-Aluprofil; auch 30 × 30 mm wurde erwähnt. **Keines dieser Profile wurde statisch dimensioniert oder am konkreten Mast geprüft.**

Die frühere Annahme „Leerrohr bedeutet Kunststoffmast“ ist nicht gesichert. Für die Fortsetzung sind Material, kleinster belasteter Teleskopabschnitt, Wandstärke, ausgefahrene Länge, Verdrehsicherung, Befestigung und gegebenenfalls Abspannung zu klären. Ein nicht leitender Mast ist kein garantierter störungsfreier Mast; Kabel und Halter bleiben ebenfalls Teile der HF-Umgebung.

Der bei der Archivierung eingesehene Sirio-Handbuchtext fordert, dass der unmittelbar zugehörige Montagemast nicht über das Aluminiumrohr hinausragt und die Drainageöffnung frei bleibt [M3]. Ein gemeinsamer Metallmast, der neben dem unteren Radom weiter nach oben läuft, muss deshalb ausdrücklich in der Montageplanung berücksichtigt werden. Ein beliebiger kurzer Ausleger ist noch kein Nachweis eines herstellerkonformen, HF-günstigen Aufbaus.

### 4.4 Kabel und Anschlüsse

Im Chat vorgeschlagen: hochwertige 50-Ω-Koaxkabel, gute Schirmung, möglichst getrennte Führung an gegenüberliegenden Mastseiten, keine unnötigen Adapterketten, Zugentlastung und keine großen Schleifen unmittelbar an den Strahlern. Mantelwellensperren wurden als mögliche Ergänzung nach Hersteller-/Aufbauvorgabe erwähnt, nicht als beschlossene Lösung.

**Präzisierung für die Fortsetzung:** Getrennte Kabelführung ist ein zu vergleichender Aufbauparameter, kein pauschales Verbot nebeneinander liegender gut geschirmter Koaxleitungen. Der vollständige Pfad einschließlich Steckern, Gehäusen und Gleichtaktströmen entscheidet. Kabel nicht aus Gründen einer nur vermuteten Entkopplung unnötig verlängern; zusätzliche RX-Dämpfung kostet Nutzsignal.

## 5. Technische Parameter und Rechenbeispiele

### 5.1 Frequenzen und Bezugspunkte

| Größe | Chat / heutiger Abgleich |
|---|---|
| Duplexabstand | 10 MHz |
| Hauptträger RX / Uplink | 408,000 MHz |
| Hauptträger TX / Downlink | 418,000 MHz |
| Zweiter diskutierter Träger RX / TX | 408,025 / 418,025 MHz |
| Gemeinsame RX-/TX-SDR-Mitten bei benachbarten Trägern | 408,0125 / 418,0125 MHz |
| Referenzfrequenz für grobe Wellenlängenbetrachtung | 413 MHz |
| Freiraumwellenlänge bei 413 MHz | Rund 0,73 m, berechnet mit λ = c/f; keine Abstandsvorschrift. |
| Abstand zweier gleich hoher Antennen an 80–100-cm-Traverse | Etwa 1,1–1,4 λ bei 413 MHz; daraus folgt keine feste Isolation. |

Die beiden zusätzlichen Träger wurden in Assistentenantworten bereits berücksichtigt. Erst der heutige Abgleich mit `config.toml`, `freqs.rs` und dem Hardware-Wiki stützt ihre Einordnung in den Repository-Stand. Die aktive Zahl der ausgesendeten Träger wurde nicht am Gerät geprüft.

SDR-Mittenfrequenz und einzelne TETRA-Trägerfrequenz sind verschiedene Dinge. Ein späterer Filter muss die vollständigen benötigten Nutzsignalbänder abdecken, nicht nur an zwei punktförmigen Trägermitten einen günstigen Wert zeigen.

### 5.2 Geometrische Beispielrechnungen aus der Diskussion

Für seitlichen Abstand `h` und Höhenversatz `v` gilt geometrisch `d = sqrt(h² + v²)`. So ergeben 80 cm seitlich plus 40 cm Höhe etwa 89 cm, ebenso 40 cm seitlich plus 80 cm Höhe. 50 cm seitlich plus 100 cm Höhe ergeben etwa 112 cm.

**Gleicher diagonaler Abstand bedeutet nicht gleiche HF-Kopplung.** Die Orientierung zum Antennendiagramm ist unterschiedlich. Das Diagramm einer einzelnen Antenne beschreibt außerdem nicht vollständig den Nahfeld-/Übergangsbereich, den gemeinsamen Mast oder Reflexionen am realen Standort. Eine Rechnung mit dem Diagonalabstand ersetzt daher keine S21-Messung.

Bei zwei baugleichen, gleich orientierten Antennen entspricht ein identisch bezogener Montageversatz zwar dem Versatz ihrer jeweiligen Bezugspunkte; die 780-mm-Gehäuselänge darf jedoch nicht unbesehen als Länge oder Lage des inneren aktiven Dipols interpretiert werden. Für Messprotokolle Gehäuse-/Halterbezugspunkte und tatsächliche Geometrie ausdrücklich angeben.

### 5.3 Isolationsbudget: nur Beispiel, keine gemessene Anlage

Im Chat verwendetes Beispiel:

```text
Angenommene TX-Leistung:          +37 dBm ≈ 5 W
Angenommene Pfadentkopplung:       30 dB
Angenommene RX-Sperrdämpfung:      50 dB bei TX-Frequenz
Restlicher TX-Träger am RX:       +37 - 30 - 50 = -43 dBm
Ohne diesen RX-Filter:            +37 - 30      =  +7 dBm
```

Ein weiteres Beispiel war `35 dB Antennenentkopplung + 38 dB Notch = 73 dB` Gesamtdämpfung. **Keiner dieser Ausgangswerte wurde gemessen.** Insbesondere sind 5 W keine bestätigte Sendeleistung und −43 dBm keine allgemein gültige Unbedenklichkeitsgrenze für den eingesetzten RX.

Für eine spätere Auslegung ist zu dokumentieren:

```text
P_TX_am_festgelegten_Bezugspunkt
- Dämpfung_des_Kopplungspfads
- zusätzliche_Filterdämpfung_bei_TX
= erwarteter_Blockerpegel_am_RX
```

Wird S21 zwischen den beiden installationsseitigen Kabelenden gemessen, sind die dabei enthaltenen Kabel- und Steckerverluste bereits Bestandteil des gemessenen Pfades. Diese Verluste nicht anschließend ein zweites Mal abziehen. Das Ergebnis ist gegen die Großsignaldaten und die beobachtete Desensibilisierung des realen Empfängers zu prüfen.

## 6. Entwicklungs- und Betriebsstand am Ende des Chats

| Gegenstand | Status | Begründung |
|---|---|---|
| Zwei SPO 380-2 an einem Mast | **Beschlossen/geplant** als Aufbaukonzept | Modell und Richtung genannt; kein Montagenachweis. |
| Duplexer vermeiden | **Beschlossen** | Ausdrückliche Nutzerfestlegung. |
| Kombinierter räumlicher Versatz | **Beschlossen/geplant** als erster Versuch | Abschließende Rückkehr zu diesem Ansatz. |
| RX oben, TX tiefer/seitlich | **Vorgeschlagener Entwurf** | Letzter Assistentenvorschlag; genaue Ausführung nicht bestätigt. |
| Konkrete Maße / Stückliste / statische Freigabe | **Offen** | Keine Zeichnung, Materialentscheidung oder Belastungsprüfung. |
| Einzelner RX-Pass-/Reject-/Notchfilter | **Idee / bedingte spätere Maßnahme** | Kein Kauf und kein abgestimmtes Exemplar nachgewiesen. |
| TX-Filter gegen Rauschen im RX-Band | **Idee** | Bedarfsabhängig; kein Senderrauschtest. |
| UAF42 in der Antennenleitung | **Verworfen** | Falscher Frequenz-/Schnittstellenbereich. |
| Einfacher offener λ/4-T-Stub | **Zurückgezogen** | Nutzkanaldämpfung bei nur 10 MHz Abstand nicht ausreichend berücksichtigt. |
| S21-/SWR-Messungen | **Geplant, nicht durchgeführt** | Keine Kurven oder Messdateien vorhanden. |
| TX-an/TX-aus-Empfindlichkeitstest | **Geplant, nicht durchgeführt** | Kein Ergebnis dokumentiert. |
| Störungsfreier gleichzeitiger TX/RX-Betrieb | **Nicht im Betrieb bestätigt** | Schlussfolgerung „wird schon reichen“ ist nicht belegt. |
| Code-/Firmwareänderung aus diesem Antennengespräch | **Nicht nachgewiesen** | Im Verlauf keine Implementierung, kein Patch und kein einschlägiger PR. |

Die im Rahmen dieses Auftrags neu angelegte Abschlussdokumentation ist von der weiterhin ungetesteten Hardwareplanung zu unterscheiden.

## 7. Zusätzlich geprüfter Repository-Stand am 2026-10-03

### 7.1 Prüfverfahren und Grenzen

Der Branch `Archiving` wurde über die GitHub-Verbindung gelesen; der oben dokumentierte Commit wurde als fester Bezugspunkt für die Dateiprüfung verwendet. Vor dem Schreiben wurde der Branch erneut abgerufen. Die GitHub-Code-Suche diente nur zum Auffinden relevanter Pfade, weil sie den Default-Branch durchsucht; Aussagen zu `Archiving` beruhen auf danach ausdrücklich am geprüften Commit gelesenen Dateien.

Ein ergänzend versuchter lokaler Clone scheiterte in der Arbeitsumgebung an `Could not resolve host: github.com`. Das ist ein Fehler des verfügbaren lokalen Zugriffswegs, kein nachgewiesener Fehler des Repositorys. Die GitHub-Verbindung konnte Branch und Dateien lesen und stellt den vorgesehenen Schreibweg über Git-Objekte bereit. Es wurden keine Software-Builds und keine Funk-Hardwaretests ausgeführt.

Der aktuelle Abgleich war gezielt, nicht repositoryweit vollständig. Eine Code-Suche nach `Sirio` ergab keinen Treffer im durchsuchten Default-Branch; daraus wird **nicht** die Abwesenheit sämtlicher Antennendokumentation in allen Branches abgeleitet.

### 7.2 Gelesene Dateien und belastbare Befunde

| Datei | Befund im geprüften Snapshot | Grenze |
|---|---|---|
| [`config.toml`](https://github.com/JanHG98/netcore-tetra/blob/3768905344f885974f0dbf061eb7846ae6675501/config.toml) | Gelesen: Anfang bis Zeile 180. `stack_mode = "Bs"`, `service_name = "tetra"`, SoapySDR-Backend, TX 418000000 Hz, RX 408000000 Hz, Sample-Rate 600000, Center 418012500/408012500 Hz, Band 4, Carrier 720/721, Offset 0, `reverse_operation = false`. | Eingetragene Konfiguration ist kein Nachweis der tatsächlich geladenen Datei, des laufenden Dienstes oder der aktuellen Ausgangsleistung. |
| [`crates/tetra-core/src/freqs.rs`](https://github.com/JanHG98/netcore-tetra/blob/3768905344f885974f0dbf061eb7846ae6675501/crates/tetra-core/src/freqs.rs) | Frequenz-/Duplexabstandstabelle und Berechnung gelesen. Index 0 / Band 4 ergibt 10000 kHz, also 10 MHz. DL wird aus Band, Carrier und Offset berechnet; ohne Reverse liegt UL um den Duplexabstand darunter. | Quelltextbefund; kein ausgeführter Unit-Test und keine Prüfung des Zielgeräte-Binaries. |
| [`wiki/Hardware-und-RF.md`](https://github.com/JanHG98/netcore-tetra/blob/3768905344f885974f0dbf061eb7846ae6675501/wiki/Hardware-und-RF.md) | Beschreibt SDR/HF-Abnahme, passiven Pfadtest und Prüfung des Uplinks bei aktivem Downlink; nennt die Frequenzen 418,000/418,025 und 408,000/408,025 MHz als frühere Projektbeispiele. | Das dort gezeichnete Duplexer-Beispiel ist **nicht** Jans in diesem Chat gewählte Architektur. Die Wiki-Seite belegt keinen eingebauten Duplexer. |
| [`wiki/Dual-Carrier.md`](https://github.com/JanHG98/netcore-tetra/blob/3768905344f885974f0dbf061eb7846ae6675501/wiki/Dual-Carrier.md) | Unterscheidet Carrier und SDR-Mitte, fordert Passbandreserve und einen gestuften Test. | Dokumentierter Testplan, kein Nachweis der erfolgreichen Abnahme dieses Antennenaufbaus. |
| [`Docs/archive/README.md`](https://github.com/JanHG98/netcore-tetra/blob/3768905344f885974f0dbf061eb7846ae6675501/Docs/archive/README.md) | Vorhandener Archivindex gelesen; bestehende zehn Einträge werden bewahrt. | Kein vorhandener eindeutig diesem Antennengespräch zugeordneter Indexeintrag. Der neue Zielpfad lieferte vor Anlage 404. |

### 7.3 Bestätigte Frequenzableitung und widersprüchliche Kommentare

Aus der gelesenen Implementierung, **rechnerisch und nicht durch Programmausführung**:

```text
DL = 100000000 × Band + 25000 × Carrier + Frequenzoffset

Band 4, Carrier 720, Offset 0:
DL = 418000000 Hz
UL = DL - 10000000 Hz = 408000000 Hz

Band 4, Carrier 721, Offset 0:
DL = 418025000 Hz
UL = 408025000 Hz
```

In `config.toml` steht bei `duplex_spacing = 0` weiterhin der Kommentar „Use 5MHz spacing …“. Für die gelesene Tabelle mit Band 4 ist dieser Kommentar widersprüchlich: Die Implementierung liefert **10 MHz**. Auch der Kommentar neben `main_carrier = 720` enthält noch einen nicht dazu passenden Rechenbezug auf 1521.

**Folgerung:** Die Werte nicht aufgrund dieser Kommentare auf ein anderes Frequenzpaar umstellen. Zuerst Runtime-Konfiguration und Geräteprogrammierung abgleichen. Eine spätere Bereinigung der Kommentare ist ein Roadmap-Kandidat, bleibt aber außerhalb dieses Archivauftrags.

### 7.4 Schnittstellen, Dienste, Pfade und Ports

Für die Antennen-/Filterfrage relevant sind die **HF-Ports TX und RX** sowie 50-Ω-Koaxverbindungen. Das sind keine TCP-/UDP-Ports. Die passiven Antennen und Filter benötigen keine IP-Adresse, keinen Netzwerkdienst und kein Deployment.

Die zusätzlich gelesene Konfiguration verwendet die Abschnitte `[phy_io]`, `[phy_io.soapysdr]`, `[net_info]` und `[cell_info]`. MCC 901, MNC 1510, LA 1 und Colour Code 1 stehen dort als Projektwerte, beeinflussen aber nicht die passive Antennenentkopplung. `service_name = "tetra"` ist ein Konfigurationswert, keine in diesem Auftrag überprüfte systemd-Unit.

Ein konkreter Installationspfad auf der Basisstation wurde im Antennengespräch nicht belegt. Der im Konfigurationskommentar erwähnte Pfad `/opt/tetra/config.toml.fallback` ist ein Beispiel für den Fallback-Mechanismus, kein hier bestätigter Betriebsort. Für dieses Archiv wurde weder ein Dienst neu gestartet noch ein Netzwerkport geändert.

## 8. Zusätzlicher Quellenabgleich: Filter und technische Grenzen

Dieser Abschnitt dokumentiert eine **Prüfung bei der Archivierung**. Er erweitert nicht rückwirkend die Nutzerentscheidung um eine Kaufempfehlung.

### 8.1 Zwei unterschiedliche Störpfade

Die Filterdiskussion muss zwei Aufgaben getrennt behandeln:

1. **Starker TX-Träger im RX-Eingang:** Ein geeigneter RX-Notch auf dem TX-Band oder RX-Bandpass kann diesen unerwünschten Pegel reduzieren.
2. **Vom Sender erzeugtes Rauschen/Nebenprodukte bereits im RX-Nutzband:** Ein RX-Passfilter kann solche Anteile nicht anhand ihrer Herkunft vom Nutzsignal trennen. Eine gezielte Verringerung an der Quelle beziehungsweise im TX-Zweig ist eine andere Aufgabe.

Die Möglichkeit, beide Aufgaben mit getrennten Zweitorfiltern anzugehen, widerspricht der Entscheidung „kein Duplexer“ nicht. Ob und wie viel Filterung nötig ist, bleibt vom Isolationsbudget und den Messungen abhängig. Ein frequenzmäßiger Abstand von 10 MHz oder eine digitale Kanalfilterung allein belegt keine ausreichende analoge Großsignalfestigkeit [A03, A19].

### 8.2 Besprochene Filterprodukte und Zahlen

| Ansatz / Produkt | Historischer Vorschlag | Heutige Einordnung für das Archiv |
|---|---|---|
| 2–4-poliger Cavity-/Helix-RX-Preselektor | Mitte etwa 408,0125 MHz, Bandbreite grob 0,5–1 MHz, Verlust möglichst ≤ 1–2 dB, Sperre bei 418 MHz mindestens 40 dB, besser 50–60 dB; Rückflussdämpfung > 15 dB als Ziel | **Nicht dimensioniertes Wunschprofil**, keine universelle Mindestanforderung und keine bestätigte Eigenschaft eines konkreten Filters. Bandbreitendefinition, Güte, Anpassung, Verlust und Sperrkurve müssen zusammen geprüft werden. |
| **Procom BRF 70/3** | Dreikreisiger Notch auf etwa 418 MHz im RX-Zweig | Hersteller führt 400–470 MHz, 50 Ω, > 38 dB Sperrdämpfung und ≤ 0,5 dB Einfügedämpfung bei den angegebenen Abständen `Fc ±5 MHz`; 208 × 77 × 33 mm. Das sind spezifizierte Bedingungen, nicht pauschal ≤ 0,5 dB im gesamten Band einschließlich der Sperrkerbe [M4]. |
| **Procom BPBR 70/3-9/13 N** | RX durchlassen, höheres TX-Band sperren | Hersteller führt Variante für 9–13 MHz Abstand; Sperrseite bei BPBR oberhalb des Durchlassbereichs. Single-channel: ≥ 80 dB Sperre und < 1,2 dB Durchlassverlust; bei 2 MHz Multi-channel-Bandbreite wird dagegen ≥ 55 dB genannt. Eine Abnahme über beide realen TETRA-Träger darf aus der Einzelfrequenzangabe nicht ungeprüft abgeleitet werden [M5]. |
| **Procom BPF 70/3** | Bandpass als Alternative; im Chat 4 MHz Bandbreite und maximal 1,4 dB Verlust genannt | Kein ausgewähltes Produkt. Diese historischen Einzelangaben wurden bei der Archivierung nicht erneut vollständig geprüft; insbesondere keine zugesicherte Sperrdämpfung bei 418 MHz. |
| SAW-Filter um 408 MHz | Kleine, möglicherweise günstige Alternative | Nur Idee. Keine konkrete Platine, keine Leistungs-/ESD-Grenze, keine verifizierte Pass-/Sperrkurve. Ein fester 433-MHz-Filter wird durch seine Produktklasse nicht passend für 408 MHz. |
| Abstimmbarer LC-/Helixfilter | Preis-/Größenkompromiss | Nur Idee. Keine berechnete Schaltung, Stückliste oder geprüfte Leiterplatte. |
| Gebrauchter UHF-Cavity-Filter | Preiswertere Einzelkomponente | Nur Beschaffungsidee; passender Abstimmbereich, Zustand und Abgleich nicht geprüft. |
| Breiter „400–470-MHz-Bandpass“, TV-/LTE- oder Audiofilter | Als ungeeignete Suchkategorien angesprochen | Die Produktbezeichnung reicht nicht. Ein Filter, dessen Durchlassband sowohl 408 als auch 418 MHz umfasst, trennt genau dieses Paar nicht hinreichend allein durch seine Bandgrenzen. |

BRF/BPBR sind als **einzelne Zweitorfilter** diskutiert worden, nicht als versteckt wieder eingeführte gemeinsame Duplexweiche. Es gibt im Chat weder eine Bestellung noch einen Herstellerabgleich auf die Projektfrequenzen. Preise, Lieferbarkeit und Gebrauchtangebote wurden nicht belastbar festgestellt. Die frühere Behauptung einer generellen Marktlücke zwischen 25-Euro-Platine und professioneller Funktechnik war kein vollständiger Marktvergleich.

### 8.3 UAF42: Ablehnung anhand TI geprüft

Das TI-Datenblatt UAF42, SBFS002B, nennt auf Seite 3 einen Filterfrequenzbereich von 0–100 kHz und ein Gain-Bandwidth-Produkt der Operationsverstärker von 4 MHz. Auf Seite 4 steht ein Betriebsspannungsbereich von ±6 bis ±18 V [M6].

Damit ist das Modulkonzept nicht als 408/418-MHz-HF-Vorfilter geeignet. Die frühere Angabe „±5 bis ±18 V“ wird nicht als belastbare TI-Spezifikation weitergeführt. Das Datenblatt des ICs ist außerdem kein vollständiger Prüfbericht der Elecbee-Platine. Der im Chat genannte Shoppreis von etwa 23,49 Euro bleibt eine **historische, nicht erneut bestätigte Preisangabe**.

### 8.4 Was aus den ETSI-Anhängen tatsächlich folgt

In der bereitgestellten **EN 300 392-2 V3.8.1**, Abschnitt 6.5.1, Seiten 97–98, wird Blocking als Empfang eines gewünschten modulierten Signals trotz eines unerwünschten unmodulierten Signals definiert. Die Pegel beziehen sich auf den RX-Antennenanschluss; die Testanordnung soll 50 Ω präsentieren. Tabelle 6.21 nennt für Phasenmodulation bei einem Abstand über 500 kHz einen Störerpegel von −25 dBm und verbindet dies mit einem Nutzsignal 3 dB oberhalb der statischen Referenzempfindlichkeit [A03].

Die bereitgestellte **EN 300 394-1 V3.3.1**, Abschnitt 7.2.5.2, Seite 43, nennt für den beschriebenen Phasenmodulations-Blockingtest unter anderem Frequenzabstände von ±1, ±2, ±5 und **±10 MHz** sowie mindestens −25 dBm Störerpegel [A19].

**Grenzen dieser Einordnung:** Das sind Prüfbedingungen der konkret vorliegenden Normausgaben. Sie beweisen weder, dass Jans SDR diese Werte einhält, noch ersetzen sie eine Messung mit dem real modulierten TETRA-Downlink und dessen Senderrauschen. Es wurde keine Konformitätsprüfung durchgeführt. Die aktuelle normative Gültigkeit sämtlicher hochgeladener Ausgaben wurde nicht geprüft; Entwürfe werden nicht zu verabschiedeten Normen erklärt.

## 9. Messplanung, Diagnose und Abnahme

### 9.1 Tatsächlich durchgeführte Tests

**Am realen HF-Aufbau: keine im verfügbaren Chat dokumentierten Tests.**

Bei der Archivierung ausgeführt wurden ausschließlich Quellen-/Dateiprüfungen: Branch- und Dateilesen, Quelltextinspektion, Kontrolle der Frequenzableitung, Inventarisierung der PDF-Anhänge und gezielte Sichtung der genannten technischen Stellen. Herstellerkurven sind keine Messkurven von Jans Antennen.

### 9.2 Vorgeschlagener passiver Antennenvergleich – noch nicht ausgeführt

Der Chat schlug einen VNA-Vergleich vor: TX- und RX-Funktechnik vollständig von den beiden Antennenkabeln trennen, einen VNA-Port mit dem TX-Antennenpfad und den anderen mit dem RX-Antennenpfad verbinden und S21 LogMag über etwa 400–425 MHz betrachten. Bevorzugt werden die tatsächlichen Nutz-/Störfrequenzen und die komplette benötigte Bandbreite erfasst.

Für die Fortsetzung sind zusätzlich festzuhalten: VNA-Modell, Kalibrierung und Bezugsebenen, Kabel-/Adapteranordnung, Rausch-/Leckagegrenze des Messaufbaus sowie exakte Montagegeometrie. Vergleichskandidaten sind gleiche Höhe, maximal möglicher Höhenversatz, zusätzlicher seitlicher Versatz und veränderte Kabelführung. Neben S21 sind die Anpassung beider Antennen und Veränderungen durch die Montage relevant.

**Schutzgrenze:** Kein aktiver Sender darf während der passiven Messung an einem VNA-Port oder über einen ungeschützten Pfad mit ihm verbunden sein. Auch Empfänger und eventuell vorhandene aktive Vorstufen sind nicht einfach Teil dieser passiven Zweitor-Messung. Der Assistentenzusatz „mit deinem NanoVNA“ ersetzt weder eine Bestätigung des Geräts noch dessen zulässige Eingangswerte.

Die früher genannten pauschalen Kategorien „−30 dB viel zu wenig“, „−60 dB brauchbar“ und „−70 dB sehr gut“ sind **keine Abnahmekriterien**. Die erforderliche Isolation hängt von TX-Leistung, Gesamtkette, Blockertoleranz und gefordertem Nutzsignal ab. Ein sehr negativer S21-Wert unterhalb der verlässlichen Messdynamik ist keine belastbare quantitative Bestätigung.

### 9.3 Vorgeschlagener aktiver Desensibilisierungstest – noch nicht ausgeführt

Entscheidend ist der Vergleich einer reproduzierbaren schwachen RX-Referenz mit und ohne Störeinfluss des eigenen Senders. Dafür müssen Ausgangsleistung, Spektrum, Frequenzen, Gain-/AGC-Einstellungen und Lastzustand festgelegt sein. Ein Testsignalgenerator beziehungsweise eine geeignete definierte TETRA-Testanordnung muss gegen eingekoppelte TX-Leistung geschützt werden.

**Wichtige Präzisierung bei der Archivierung:** Eine normale TMO-Zelle einfach vollständig auszuschalten und anschließend fehlende Registrierung zu vergleichen, ist kein sauberer RX-Desensibilisierungstest. Der Downlink wird für den normalen Zellbetrieb benötigt. Für den Vergleich sind ein geeigneter Testmodus oder eine unabhängige reproduzierbare Nutzsignalquelle und gleichbleibende Bewertungsbedingungen erforderlich. Ein Handfunkgerät mit schwankender Position und Pegel kann einen praktischen Hinweis liefern, aber keine kalibrierte Empfindlichkeitsmessung ersetzen.

Zu protokollieren sind je nach verfügbarer Messmöglichkeit Empfangsschwelle, BER/FER beziehungsweise Dekodierergebnis, Rausch-/Störpegel und Auffälligkeiten bei Registrierung, SDS und Rufverkehr. Nach erfolgreichem Einzelträgertest ist die tatsächlich geplante Mehrträger-/Lastkonfiguration separat zu prüfen. Dafür ist im heutigen Wiki ein gestufter Ablauf beschrieben, aber nicht als hier bestanden dokumentiert [R3, R4].

### 9.4 Hypothesen statt behaupteter Fehlerdiagnosen

| Mögliches späteres Symptom | Zu prüfende Ursache | Bisheriger Nachweis |
|---|---|---|
| RX wird bei TX-Betrieb schlechter | Blocking/Kompression durch eigenen TX-Träger | Nicht gemessen. |
| RX-Rauschboden steigt trotz RX-Notch | Senderrauschen im RX-Band, Umgehungspfad oder andere Störquelle | Nicht gemessen. |
| S21 ändert sich stark beim Anfassen/Umlagern der Kabel | Gleichtakt-/Mantelwellen, Abschirmungs-/Steckerproblem, veränderte Geometrie | Nicht untersucht. |
| SWR nach Montage schlechter | Einfluss von Metallmast/Halter/Umgebung oder Kabel-/Anschlussfehler | Nicht gemessen. |
| Tiefe VNA-Kurve bleibt unabhängig vom Aufbau gleich | Messgrenze, Port-/Kabelleckage oder ungeeignete Kalibrierung | Nicht untersucht. |
| Funkfehler nur unter hoher Last | Neben HF-Selbststörung auch Timing-/Buffer-/Softwareursachen prüfen | Keine Diagnose aus diesem Chat. |
| DC-Kurzschluss am Sirio-Anschluss | Bei diesem Antennentyp möglicherweise konstruktionsbedingt | Hersteller beschreibt DC-Ground; nicht ohne weitere Prüfung als Defekt werten [M1]. |

### 9.5 Kompaktes künftiges Messprotokoll

```text
Datum / Bearbeiter:
TBS-Build und tatsächlich geladene Konfiguration:
SDR / Treiber / Hardwareversion / PA / Vorverstärker:
TX-Träger, Modulation, mittlere und relevante maximale Leistung:
RX-Träger / Gain / AGC / Referenz-Nutzsignal:
Mastmaterial, Durchmesser, Länge, Befestigung:
Antennenpositionen mit eindeutigem Maßbezug:
Kabeltypen, Längen, Stecker, Schutz-/Filterkomponenten:
VNA, Kalibrierung, Bezugsebenen und Messgrenze:
S21 über RX- und TX-Bänder; S11/S22:
RX-Referenzergebnis ohne / mit eigenem TX:
Einzelträger- und gegebenenfalls Mehrträgerergebnis:
Akzeptanzkriterium / Bewertung / verbleibende Reserve:
Dateien, Bilder und Messkurven:
```

Diese Vorlage ist ein **neuer Dokumentationsvorschlag**, kein ausgefülltes Testprotokoll.

## 10. Befehle, Installation, Deployment und Reparatur

Im historischen Antennengespräch wurden **keine Shell-Befehle ausgeführt**, keine Software installiert und keine Konfigurationsdateien geändert. VNA-Bedienfolgen, Zuschnittschritte und Montagevorschläge waren Anleitungen beziehungsweise Ideen ohne Ausführungsnachweis. Der Stub-Zuschnitt ist zurückgezogen.

Der heute gelesene Hardware-Wiki-Stand enthält folgende Diagnosebefehle; sie wurden in diesem Archivauftrag **nicht auf Jans Basisstation ausgeführt**:

```bash
SoapySDRUtil --info
SoapySDRUtil --find
SoapySDRUtil --probe="driver=<TATSÄCHLICHER-TREIBER>"
```

`<TATSÄCHLICHER-TREIBER>` ist ein zu ersetzender Platzhalter. Diese Befehle können Identität und angebotene Einstellungen helfen zu erfassen, sind aber weder eine TX-Leistungsmessung noch ein Antennenisolationstest [R3].

Für die spätere Montage existiert aus diesem Chat keine freigegebene Reparatur-/Deployment-Prozedur. Die Reihenfolge bleibt: Rahmenbedingungen erfassen, mechanisch geeignete Varianten planen, passiv messen, anschließend kontrolliert aktiv abnehmen. Kein erneuter Software-Rollout ist allein aus dem Wunsch nach Antennenversatz abzuleiten.

## 11. Offene Aufgaben, Roadmap-Kandidaten und Prioritäten

**Historisch vereinbarte Reihenfolge:** Zuerst bezahlbare räumliche Trennung untersuchen, dann messen; Filter nur auf Basis eines festgestellten Bedarfs. Die folgenden Kennungen und Prioritätsstufen wurden zur Archivierung vergeben, nicht bereits im Chat als terminierte Arbeitspakete beschlossen.

| ID | Priorität / Status | Aufgabe | Abhängigkeit / Abschlusskriterium |
|---|---|---|---|
| RF-ANT-01 | P0 – offen | Verfügbare Höhe und Breite, Masttyp/-material/-durchmesser und zulässige Befestigung erfassen; Fotos oder bemaßte Skizze des realen Platzes ergänzen. | Voraussetzung jeder belastbaren Halterentscheidung; Jans begrenzten vertikalen Platz respektieren. |
| RF-ANT-02 | P0 – offen | Tatsächlichen SDR, PA, RX-Vorstufen, TX-Leistung, Gain und Frequenz-/Mehrträgerbetrieb dokumentieren. | Grundlage für Pegelbudget, Messgeräteschutz und späteres Filterlastenheft. |
| RF-ANT-03 | P1 – geplant als Richtung | Zwei SPO 380-2 mit größtmöglichem geeigneten Versatz an einem Mast planen; bevorzugten RX-oben/TX-tiefer-Entwurf gegen reale Grenzen und Herstellerhinweise prüfen. | Keine pauschale Freigabe der im Chat genannten 60–100/30–50-cm-Werte oder Profilquerschnitte. |
| RF-ANT-04 | P1 – geplant, ungetestet | Messmittelverfügbarkeit klären; passive Anpassung und S21 mehrerer Varianten mit konsistentem Kabelaufbau erfassen. | Auswertbare Kurven mit Bezugsebenen und Messgrenze, nicht nur ein einzelner dB-Wert. |
| RF-ANT-05 | P1 – geplant, ungetestet | Kontrollierten Desensibilisierungs-/Betriebstest für den vorgesehenen Einzel- und gegebenenfalls Mehrträgerbetrieb durchführen. | Reproduzierbares Nutzsignal, sichere Pegel, vorab festgelegtes Akzeptanzkriterium. |
| RF-ANT-06 | P2 – bedingte Idee | Nur bei unzureichender Reserve: passenden einzelnen RX-Notch/Bandpass/Pass-Reject auswählen; bei TX-Rauschen auch TX-Seite untersuchen. | Gemessener Bedarf; **kein Duplexer**, kein UAF42, kein zurückgezogener einfacher T-Stub. Kostenobergrenze zuvor mit Jan festlegen. |
| RF-ANT-07 | P2 – neuer Dokumentationskandidat | Widersprüchliche Kommentare zum Duplexabstand und zur Carrierberechnung in `config.toml` später bereinigen. | Gesonderter Änderungsauftrag außerhalb `Docs/archive/`; keine Änderung der funktionierenden Frequenzwerte allein wegen eines falschen Kommentars. |
| RF-ANT-08 | P2 – neuer Dokumentationskandidat | Hardware-Wiki um eine klar gekennzeichnete Zwei-Antennen-Alternative ergänzen und tatsächliche Montage-/Messdaten referenzieren. | Erst nach belastbarer Auslegung/Abnahme; Wiki-Duplexerbeispiel nicht als Nutzerentscheidung behandeln. |

Weitere bewahrte Nebenideen sind die symmetrische Traverse als mechanisch ausgewogener Vergleichskandidat, kurze Halter auf entgegengesetzten Mastseiten, getrennte Koaxführung, mögliche Mantelwellensperren und gebrauchte einzelne Cavity-Filter. Keine davon wurde gekauft, gebaut oder getestet bestätigt.

Eine Frist, Zuständigkeit eines Lieferanten oder fest vereinbarte Bestellung existiert nicht. Der spätere Status-/Roadmap-Lauf darf dieses Archiv als Planungsquelle einlesen, darf die genannten Arbeitspakete aber nicht automatisch auf „implementiert“ oder „im Betrieb“ setzen.

## 12. Relevante Anhänge und tatsächliche Sichtung

Die 25 bereitgestellten PDFs sind lokal zugänglich und wurden anhand von Dateiname, Seitenzahl und Titelseite inventarisiert. Volltexte wurden **nicht vollständig** in diese Dokumentation übernommen. Nur die explizit genannten Radio-/Blocking-Stellen wurden für die fachliche Ergänzung gezielt geprüft. Die übrigen Titel begründen für diesen Mastaufbau keine Filter-, Abstands- oder Installationsvorschrift.

Die Dateinamen in der folgenden Tabelle sind **Anhangsbezeichner, keine behaupteten Repository-Pfade**. Die PDFs wurden durch diesen Auftrag nicht ins Git kopiert.

| ID | Dateiname | Identifikation / Umfang | Bezug zum Archiv |
|---|---|---|---|
| A01 | `ETSI.pdf` | 4100 Seiten; Sammlung beginnt mit EN 300 812 V2.1.1 (2001-12). | Sammlung nicht vollständig in Einzelnormen zerlegt oder ausgewertet; keine eigenständige Versionsangabe für das Gesamtpaket. |
| A02 | `en_30039201v010601p.pdf` | EN 300 392-1 V1.6.1 (2020-04), General network design; 182 Seiten. | Allgemeiner Systemkontext; keine konkrete Mastfreigabe abgeleitet. |
| **A03** | `en_30039202v030801p.pdf` | **EN 300 392-2 V3.8.1 (2016-08), Air Interface; 1445 Seiten.** | **Gezielt geprüft: Abschnitt 6.5.1, Seiten 97–98, einschließlich Tabelle 6.21.** |
| A04 | `en_3003920303v010301p.pdf` | EN 300 392-3-3 V1.3.1 (2011-11), ISI Group Call; 251 Seiten. | Protokollkontext, für die Montage nicht weiter ausgewertet. |
| A05 | `en_3003920304v010301p.pdf` | EN 300 392-3-4 V1.3.1 (2010-08), ISI Short Data Service; 28 Seiten. | Für diese Hardwareentscheidung nicht weiter ausgewertet. |
| A06 | `en_3003920308v010401p.pdf` | EN 300 392-3-8 V1.4.1 (2020-04), Generic Speech Format Implementation; 22 Seiten. | Für diese Hardwareentscheidung nicht weiter ausgewertet. |
| A07 | `en_3003920313v010201p.pdf` | EN 300 392-3-13 V1.2.1 (2020-04), transportunabhängiger ISI Group Call; 191 Seiten. | Für diese Hardwareentscheidung nicht weiter ausgewertet. |
| A08 | `en_3003920315v010500a.pdf` | **Draft** EN 300 392-3-15 V1.5.0 (2026-04), ISI Mobility Management; 380 Seiten. | Entwurfsstatus bewahrt; für die Montage nicht weiter ausgewertet. |
| A09 | `en_30039205v020701p.pdf` | EN 300 392-5 V2.7.1 (2020-04), PEI; 320 Seiten. | Keine PEI-Integration Gegenstand dieses Chats. |
| A10 | `en_30039207v030501p.pdf` | EN 300 392-7 V3.5.1 (2019-07), Security; 216 Seiten. | Keine Security-Änderung Gegenstand dieses Chats. |
| A11 | `en_30039209v010701p.pdf` | EN 300 392-9 V1.7.1 (2020-04), General requirements for supplementary services; 46 Seiten. | Für diese Hardwareentscheidung nicht weiter ausgewertet. |
| A12 | `en_3003921006v010401p.pdf` | EN 300 392-10-6 V1.4.1 (2006-08), Call Authorized by Dispatcher; 20 Seiten. | Für diese Hardwareentscheidung nicht weiter ausgewertet. |
| A13 | `en_3003921018v010301p.pdf` | EN 300 392-10-18 V1.3.1 (2003-10), Barring of Outgoing Calls; 17 Seiten. | Für diese Hardwareentscheidung nicht weiter ausgewertet. |
| A14 | `en_3003921101v010201p.pdf` | EN 300 392-11-1 V1.2.1 (2004-01), Call Identification stage 2; 44 Seiten. | Für diese Hardwareentscheidung nicht weiter ausgewertet. |
| A15 | `en_3003921114v010101p.pdf` | EN 300 392-11-14 V1.1.1 (2002-07), Late Entry; 23 Seiten. | Für diese Hardwareentscheidung nicht weiter ausgewertet. |
| A16 | `en_3003921117v010102p.pdf` | EN 300 392-11-17 V1.1.2 (2002-01), Include Call; 18 Seiten. | Für diese Hardwareentscheidung nicht weiter ausgewertet. |
| A17 | `en_3003921201v010202p.pdf` | EN 300 392-12-1 V1.2.2 (2007-08), Call Identification stage 3; 56 Seiten. | Für diese Hardwareentscheidung nicht weiter ausgewertet. |
| A18 | `en_3003921216v010400a.pdf` | **DRAFT** EN 300 392-12-16 V1.4.0 (2026-03), Pre-emptive Priority Call; 67 Seiten. | Entwurfsstatus bewahrt; keine Montage-/Filterfreigabe. |
| **A19** | `en_30039401v030301p.pdf` | **EN 300 394-1 V3.3.1 (2015-04), Conformance testing, Radio; 169 Seiten.** | **Gezielt geprüft: Abschnitt 7.2.5, Seite 43; Seite zusätzlich lokal gerendert und visuell eingesehen.** |
| A20 | `en_30039502v010303p.pdf` | EN 300 395-2 V1.3.3 (2025-02), TETRA codec; 94 Seiten. | Keine Codec-Änderung Gegenstand dieses Chats. |
| A21 | `en_300812v020101p.pdf` | EN 300 812 V2.1.1 (2001-12), SIM-ME interface; 156 Seiten. | Für diese Hardwareentscheidung nicht weiter ausgewertet. |
| A22 | `es_20081201v020205p.pdf` | ES 200 812-1 V2.2.5 (2003-12), UICC physical/logical characteristics; 8 Seiten. | Für diese Hardwareentscheidung nicht weiter ausgewertet. |
| A23 | `es_20081202v020401m.pdf` | **Final draft** ES 200 812-2 V2.4.1 (2005-08), TSIM application; 139 Seiten. | Entwurfsstatus bewahrt; keine TSIM-Entwicklung Gegenstand dieses Chats. |
| A24 | `ets_30039214e01v.pdf` | **Final draft prETS** 300 392-14 (September 1997), PICS proforma; 61 Seiten. | Keine ausgefüllte Konformitätserklärung und kein Antennentest. |
| A25 | `ts_10081201v020205p.pdf` | TS 100 812-1 V2.2.5 (2003-10), UICC physical/logical characteristics; 8 Seiten. | Für diese Hardwareentscheidung nicht weiter ausgewertet. |

Die übrigen im Gespräch erwähnten Herstellerunterlagen waren Links, keine zusätzlichen hochgeladenen Messdateien. Ein vollständiger Abgleich aller ETSI-PDFs gegen die neueste veröffentlichte Ausgabe gehört nicht zum durchgeführten Prüfprogramm.

## 13. Quellen, Repository-Verweise und historische Produktlinks

### 13.1 Historische Primärquelle des Projektbeschlusses

**H1 – sichtbarer Chatverlauf:** Nutzerfragen und Korrekturen zur Mastmontage, zum Antennenmodell, zum Budget, zur ausdrücklichen Duplexer-Ablehnung, zum Elecbee-Modul und zur abschließenden versetzten Montage. Chattitel, dauerhafter Chatlink und einzelne Nachrichtendaten fehlen. Abschnitt 3 bewahrt die Reihenfolge und trennt zurückgezogene Aussagen von der zuletzt gewählten Richtung.

### 13.2 Am Archivierungstag zusätzlich gelesene Quellen

- **R1:** [`config.toml`, geprüfter Commit](https://github.com/JanHG98/netcore-tetra/blob/3768905344f885974f0dbf061eb7846ae6675501/config.toml), gelesener Bereich Zeilen 1–180.
- **R2:** [`crates/tetra-core/src/freqs.rs`, geprüfter Commit](https://github.com/JanHG98/netcore-tetra/blob/3768905344f885974f0dbf061eb7846ae6675501/crates/tetra-core/src/freqs.rs), insbesondere `TETRA_DUPLEX_SPACING`, `get_default_duplex_spacing` und `get_freqs`.
- **R3:** [`wiki/Hardware-und-RF.md`, geprüfter Commit](https://github.com/JanHG98/netcore-tetra/blob/3768905344f885974f0dbf061eb7846ae6675501/wiki/Hardware-und-RF.md).
- **R4:** [`wiki/Dual-Carrier.md`, geprüfter Commit](https://github.com/JanHG98/netcore-tetra/blob/3768905344f885974f0dbf061eb7846ae6675501/wiki/Dual-Carrier.md).
- **M1:** [Sirio Cellular/LAN-Katalog 2019–2020](https://www.sirioantenne.it/images/pdf/downloads/SIRIO_cell-lan_2019-2020.pdf), gedruckte Seite 6 / PDF-Seite 7; Tabelle und Antennendiagramm visuell geprüft.
- **M2:** [Sirio SPO-380-2 Produktseite](https://www.sirioantenne.it/en/products/uhf/spo-380-2), Hersteller-Suchauszug eingesehen; direkter Seitenabruf lief in einen Timeout. Die wichtigsten Angaben sind zusätzlich durch M1 gestützt.
- **M3:** [Sirio SPO-380-2 Handbuch ID384](https://www.sirioantenne.it/images/pdf/manuals/Id-384_11-04-2027_SPO-380-2_web.pdf), abrufbarer extrahierter Text mit gedrucktem Revisionsvermerk **11/04/2024**. Der URL-Dateiname enthält abweichend „2027“ und wird nicht als Veröffentlichungsdatum interpretiert. Bildabruf dieses Links schlug fehl; Montagehinweise beruhen auf dem zugänglichen Text, nicht auf einer hier visuell bestätigten Handbuchzeichnung.
- **M4:** [Amphenol Procom BRF 70/3](https://amphenolprocom.com/product/brf-70-3/), Herstellerdaten und Anwendungsbeschreibung.
- **M5:** [Amphenol Procom BPBR 70/3 / BRBP 70/3](https://amphenolprocom.com/product/bpbr-70-3-brbp-70-3/), Herstellerdaten einschließlich Single-/Multi-channel-Unterscheidung und Variante `BPBR 70/3-9/13 N`.
- **M6:** [Texas Instruments UAF42, Datenblatt SBFS002B](https://www.ti.com/lit/ds/symlink/uaf42.pdf), insbesondere Seiten 3–4; Seite 3 auch als gerenderte Tabelle eingesehen.
- **A03 / A19:** Bereitgestellte ETSI-Dateien aus Abschnitt 12; konkrete Kapitel und Seiten stehen in Abschnitt 8.4. Keine Behauptung einer vollständigen Normenprüfung.

### 13.3 Weitere im historischen Gespräch erwähnte Quellen – nicht als aktuelle Kaufempfehlung

Diese Verweise werden für eine spätere Rekonstruktion bewahrt. Soweit nicht oben als geprüft aufgeführt, sind Verfügbarkeit, Ausführung und damalige Leistungs-/Preisangaben **nicht erneut verifiziert**.

| Historischer Bezug | Link / Identifikation | Archivbewertung |
|---|---|---|
| Elecbee-UAF42-Platine | [Verlinktes Modul](https://www.elecbee.com/de/product-detail/uaf42-active-high-pass-low-pass-bandpass-filtering-frequency-gain-q-adjustable-general-filter_32506) | Vom Nutzer eingebrachter Produktvorschlag; für UHF-Antennenleitung verworfen. Trackingparameter entfernt. |
| Analog Devices Notch-Labor | [Notch Filter Lab](https://wiki.analog.com/university/courses/fieldsandwaves/m1k-alt-notch-filter-lab) | Historischer Bezug für die allgemeine Stub-Idee; **kein Eignungsnachweis** für 408/418 MHz mit dem vorgeschlagenen T-Stück. |
| Telewave Cavity-Übersicht | [Cavity filters](https://www.telewave.com/cavity-filters/) | Produktklassenbeispiel, kein ausgewählter oder vermessener Filter. |
| Telewave Duplexer | [TMND3-Produktfamilie](https://www.telewave.com/product/tmnd3-1416-1516-1616-1716/) | Historischer Vergleichswert; Duplexer später ausdrücklich ausgeschlossen. |
| Procom DPF 70/6 | [Historischer Herstellerlink](https://amphenolprocom.com/products/filters/867-dpf-70-6) | Empfehlung `DPF 70/6-9/13-N(f)` zurückgestellt/verworfen zugunsten des ausdrücklichen Zwei-Antennen-Konzepts. |
| Procom DPF UHF/33 DR 3000 | [Historischer Herstellerlink](https://amphenolprocom.com/products/filters/produkter/854-dpf-uhf-33-dr-3000) | Ebenfalls nicht gewählte Duplexer-Alternative. |
| Procom BPF 70/3 | [Historischer Herstellerlink](https://amphenolprocom.com/de/produkte/filters-de/produkter/873-bpf-70-3) | Diskutierter Bandpass, nicht beschafft oder abgestimmt. |
| Comprod 66-40-36 | [Historischer Produktlink](https://www.comprodcom.com/product-page/66-40-36-re-re-entrant-base-station-duplexer-uhf) | Größere Duplexer-Alternative, nicht Teil des Endkonzepts. |
| Bird/TX RX Unterlagen | [7-9408](https://birdrf.com/hubfs/discontinued-manuals/7-9408.pdf), [7-9309](https://docs.txrx.com/files/Public/UG/UG031783/UG031783-2/UG031783-2%207-9309.pdf) | Historisch zur Isolation/Filterung/Schirmung genannt; im Archivlauf nicht vollständig erneut gelesen. |
| Einfacher Teleskopmast | [Historischer WiMo-Link](https://www.wimo.com/de/telescopic-mast-portable-transport-length-190cm) | Beispiel für Standard-Masttechnik; keine konkrete Bestellung oder statische Auslegung. |

Für diesen Hardwareplan gibt es im sichtbaren Chat **keinen zugehörigen Implementierungscommit und keinen PR**. Die Referenz-SHA in diesem Dokument bezeichnet den heutigen Prüfstand. Aus anderen Archiven oder dem allgemeinen Projektkontext werden keine fremden Implementierungserfolge diesem Chat zugeschrieben.

## 14. Fortsetzungsnotiz

Für die nächste Bearbeitung nicht erneut bei einem Duplexer anfangen und den zurückgezogenen offenen Koaxstub nicht wieder als Billiglösung empfehlen. Ausgangspunkt sind **zwei Sirio SPO 380-2, ein einfacher Mast, wenig vertikaler Platz und ein enges Kostenbewusstsein**.

Zuerst reale Montageabmessungen und HF-Gerätedaten ergänzen. Danach einen mechanisch tragfähigen, herstellerseitig passenden Versatz mit dokumentierten Messungen vergleichen. Erst die Kombination aus ausreichender Anpassung, nachvollziehbarer Pfadentkopplung und einem kontrollierten Empfangstest bei aktivem eigenem TX kann den Betriebsstand belegen. Bis dahin bleibt das Ergebnis **geplant und ungetestet**, nicht „durch Antennenversatz gelöst“.
