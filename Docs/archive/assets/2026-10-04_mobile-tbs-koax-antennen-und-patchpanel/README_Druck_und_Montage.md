# NetCore-TETRA · RF / I/O Patchpanel
## 19 Zoll / 2 HE · Version 1 · 03.10.2026

**Mehrteilige, mehrfarbig druckbare Konstruktion für die besprochene Dual-SDR-Belegung.**

Die Dateien enthalten echte geschlossene Volumennetze: Platte, Schrift und Logo/Gruppierungslinien sind getrennte Teile einer Baugruppe. Das ist kein Bild und keine nur aufgemalte Textur. Die 3MF-Dateien sind **nicht gesliced**; Drucker, Filamente, AMS-Zuordnung und Druckparameter müssen im Slicer gewählt werden.

**Vor dem großen Druck zuerst den Passformtest drucken.** Die genauen Artikelnummern der vorhandenen Neutrik-Komponenten sind noch nicht bekannt. Insbesondere POWER ist vorerst als normales D-Lochbild angelegt, nicht als bestätigte Passung für jede powerCON-Variante. Die Konstruktion ist rechnerisch geprüft, aber nicht probegedruckt, mechanisch belastungsgeprüft oder in der Bambu-Studio-Oberfläche probeweise gesliced.

## 1. Welche Datei wofür?

| Datei | Inhalt / Verwendung |
|---|---|
| `NetCore_Tetra_Patchpanel_A1_Mehrfarbig.3mf` | Beide Frontplattenhälften auf einem 256 × 256 mm großen Druckbett angeordnet; je drei Farbteile. Hauptdatei. |
| `Links_RADIO_A1.3mf` | Linke Hälfte separat: RADIO 1 und RADIO 2. |
| `Rechts_DUP_POWER_A1.3mf` | Rechte Hälfte separat: DUPLEXER, POWER und Reserven. |
| `ZUERST_DRUCKEN_Neutrik_Passformtest.3mf` | Zweireihiger Passformtest mit Ø 23,6 / 24,0 / 24,2 mm, dem vorgesehenen Reihenabstand und Befestigungslöchern. |
| `Zubehoer_Verbindung_und_3_Blindplatten.3mf` | Rückseitige Verbindungsplatte und drei D-Blindplatten. Separat drucken. |
| `Montageansicht_482mm_NICHT_A1_Drucklayout.3mf` | Zusammengebautes Panel einschließlich rückseitiger Verbindung; zur Kontrolle. **Nicht als A1-Drucklayout verwenden.** |
| `STL_Fallback/` | Getrennte Farbteile jeder Hälfte als STL; Notlösung, falls ein Slicer die 3MF-Baugruppen nicht korrekt übernimmt. |
| `Lochkoordinaten_mm.csv` | Alle Lochmittelpunkte, von vorn gesehen; Ursprung links unten an der tatsächlichen Platte. |
| `Geometriepruefung.json` | Abmessungen, Textpositionen und Prüfwerte der erzeugten Netze. |
| `3MF_Validierung.json` | Unabhängige Prüfung der 3MF-Dateien mit lib3MF und trimesh. |
| `build_panel.py` | Bearbeitbarer Python-Konstruktionsgenerator. Keine Schriftdateien erforderlich, solange eine passende Systemschrift installiert ist. |

## 2. Farben zuweisen

Jede Plattenhälfte hat drei Teile:

| Namensende im Objektbaum | Geometrie | Voreingestellte Darstellungsfarbe |
|---|---|---|
| `_Platte` | Frontplatte samt rückseitigen Versteifungsrippen | Anthrazit |
| `_Schrift` | Titel und Anschlussbeschriftungen | Hell / Weiß |
| `_Logo_Linien` | Vereinfachtes Antennenlogo, Gruppenüberschriften und Rahmen | Blau |

Für **zwei Farben** bekommen `_Schrift` und `_Logo_Linien` einfach dasselbe Filament. Für drei Farben werden sie unterschiedlich zugeordnet. Die angezeigten Farben sind keine Vorgabe für die tatsächliche AMS-Belegung.

Die Schrift und die Linien sind **0,6 mm tiefe, bündige Einlagen**. Die Frontfläche bleibt eben, insbesondere unter den Buchsenflanschen. Das Antennenlogo ist eine für den Druck vereinfachte geometrische Umsetzung des besprochenen Motivs; eine originale Logo-Vektordatei lag nicht vor.

### Import in Bambu Studio

1. Hauptdatei öffnen/importieren und das eigene A1-Profil sowie die tatsächlich eingesetzte Düse und Druckplatte wählen.
2. Die zwei Baugruppen im Objektbaum ausklappen. Die oben genannten Teile einzeln den gewünschten Filamenten zuweisen.
3. **Teile nicht voneinander lösen, separat automatisch anordnen oder einzeln auf das Bett absenken.** Ihre relativen Positionen definieren die Einlagen.
4. Vor dem Druck in der geslicten Vorschau prüfen: erste drei Schichten enthalten die Farbeinlagen; dahinter ist das Panel geschlossen. Löcher müssen offen bleiben.

3MF-Kerngeometrie und Teil-Metadaten sind enthalten; je nach Slicer-Version kann die anfängliche Übernahme der Darstellungsfarben unterschiedlich aussehen. Maßgeblich ist die manuell kontrollierte Filamentzuordnung der drei Teile, nicht das Vorschaubild.

**STL-Notlösung:** Die drei zusammengehörigen STL-Dateien einer Hälfte gleichzeitig importieren und als **ein Objekt mit mehreren Teilen** behandeln. Anschließend die Teile einfärben. Die gemeinsame Orientierung und ihre relativen Koordinaten bleiben dabei unverändert.

## 3. Druckorientierung und Startparameter

Die Druckdateien sind bereits mit der **beschrifteten Front nach unten** ausgerichtet. Die Rückseite mit den Rippen zeigt nach oben. Aus der Rückansicht im Slicer kann die Schrift dadurch spiegelverkehrt wirken; nach dem Wenden der gedruckten Platte ist sie lesbar. **Nicht zusätzlich spiegeln.**

Die beiden Hälften belegen zusammen ungefähr X = 7,4 bis 248,6 mm und Y = 10 bis 196,1 mm. Der hintere Bereich bleibt für einen Spülturm frei. Der konkrete Platzbedarf von Spülturm, Brim und Drucker-Sperrflächen ist vor dem Slicen zu kontrollieren. Alternativ die Hälften einzeln drucken.

Als **nicht erprobten Ausgangspunkt**, nicht als fertiges Prozessprofil:

- 0,4-mm-Düse; erste Schicht und weitere Schichten 0,20 mm. Die Einlagenhöhe entspricht damit drei Schichten.
- 4–5 Wände, etwa 5 obere/untere Schichten und 30–40 % Infill.
- Stützen sind für die vorgesehene Orientierung geometrisch nicht vorgesehen. Die Slicer-Vorschau entscheidet, ob das gewählte Profil dennoch unerwartete Bereiche erzeugt.
- Bei Bedarf etwa 4 mm Außen-Brim; anschließend prüfen, ob Brim und Spülturm noch aufs Bett passen.
- Vor allem die erste Schicht und die kleinen Logo-/Schriftlinien kontrollieren. Dünne Linien dürfen im Slicer nicht verschwinden.

Der A1 ist vom Hersteller für PLA und PETG vorgesehen. Für einen ersten Passformtest ist das vorhandene PLA+ ein sinnvoller Ausgangspunkt. Für die spätere mechanische Blende würde ich PETG als Kandidaten erproben; das ist **keine Zusage einer bestimmten Dauerfestigkeit oder Temperaturbeständigkeit**. Die tatsächlich eingesetzte Sorte und Racktemperatur sind maßgeblich. Innerhalb eines Druckteils möglichst dieselbe Materialfamilie verwenden; nicht ungeprüft PLA-Schrift in eine PETG-Platte einplanen.

## 4. Tatsächliche Modellmaße

Alle Maße in Millimetern. Bezug für die folgenden Koordinaten: **Vorderansicht, tatsächliche linke Unterkante der montierten Platte**.

| Merkmal | Modellmaß |
|---|---:|
| Gesamte Breite montiert | 482,6 |
| Tatsächliche Plattenhöhe | 88,1 |
| Nominaler 2-HE-Bauraum | 88,9 |
| Freiraum oben/unten innerhalb des nominalen Rasters | je 0,4 |
| Frontplattenstärke | 3,0 |
| Zusätzliche Rippenhöhe auf der Rückseite | 6,0 |
| Gesamttiefe mit Rippen | 9,0 |
| Einlagentiefe Schrift/Logo/Linien | 0,6 |
| Breite je Druckhälfte | 241,2 |
| Geplante Fuge zwischen beiden Hälften | 0,2 |
| Untere Lochreihe, Y ab tatsächlicher Unterkante | 22,1 |
| Obere Lochreihe, Y ab tatsächlicher Unterkante | 63,1 |
| Reihenabstand | 41,0 |
| X-Mittelpunkte aller acht Spalten | 48 / 94 / 158 / 204 / 274 / 320 / 366 / 430 |
| Große Anschlussbohrungen | Ø 24,2 |
| Kleine Anschlussbohrungen | Ø 3,3 |
| Anschluss-Schraubenraster horizontal × vertikal | 19,0 × 24,0 |
| Rack-Befestigungsraster horizontal | 465,1 |
| Rack-Befestigungsraster vertikal | 76,2 |
| Rack-Langlöcher | 10,0 × 6,6 |

### Warum weichen einzelne Maße von der alten Konzeptgrafik ab?

**Höhe:** 88,9 mm ist das nominale 2-HE-Raster. Die tatsächliche Platte ist hier 88,1 mm hoch, damit sie oben und unten etwas Luft hat. Eine 482,6 × 88,1 × 3 mm große 2-HE-Frontplatte findet sich auch bei METCASE. Die bereits besprochenen Reihenhöhen von 22,5 und 63,5 mm im nominalen Raster bleiben erhalten; durch den 0,4-mm-Randfreiraum werden daraus 22,1 und 63,1 mm ab der tatsächlichen Unterkante. Nicht die ganze Datei skalieren, um andere Lochdurchmesser zu erhalten.

**D-Lochbild:** Die vom Nutzer gezeigte Zeichnung nennt Ø 23,6 mm. Die zusätzlich geprüfte offizielle NE8FDP-Zeichnung nennt dagegen mindestens Ø 24 mm und Befestigungsbohrungen ab Ø 3,2 mm. Für diese erste FDM-Fassung sind Ø 24,2 und Ø 3,3 mm als Konstruktionszugabe gewählt. Das ersetzt nicht die Prüfung der konkreten Buchsen.

**Schraublöcher:** Die frühere generierte Grafik mit vier Löchern pro Flansch war falsch. Die Druckfassung hat zwei diagonal gegenüberliegende Löcher: in der Vorderansicht **links oben und rechts unten**, jeweils ±9,5 mm in X und ±12 mm in Y zum großen Lochmittelpunkt.

**Rackbefestigung:** Das frühere Bildmaß 453,6 mm wird nicht verwendet. Die Druckfassung verwendet 465,1 mm zwischen den linken und rechten Befestigungsmitten.

**Beschriftung:** Alle Schrift- und Akzentkonturen sind rechnerisch außerhalb angenommener 26,2 × 31,2 mm großer Buchsenflansche angeordnet. Konkrete Steckergehäuse, Verriegelungen, Gummidichtungen und Bedienung mit eingesteckten Kabeln müssen trotzdem am Passformtest geprüft werden.

## 5. Portbelegung

| Spalte | Oben | Unten |
|---:|---|---|
| 1 | RADIO 1 TX | RADIO 1 USB |
| 2 | RADIO 1 RX | RADIO 1 LAN |
| 3 | RADIO 2 TX | RADIO 2 USB |
| 4 | RADIO 2 RX | RADIO 2 LAN |
| 5 | DUPLEXER TX | RESERVE |
| 6 | DUPLEXER RX | PWR 1 |
| 7 | DUPLEXER ANT | PWR 2 |
| 8 | AUX RF | AUX |

RADIO 1 und RADIO 2 sind jeweils als durchgehender 2×2-Funktionsblock eingerahmt. DUPLEXER und POWER sind getrennt. Die drei derzeit unbelegten Positionen können mit den mitgelieferten Blindplatten geschlossen werden.

## 6. Zusammenschrauben

Die beiden Frontplattenhälften werden an der Mittelnaht von hinten mit einer **36 × 72 × 4 mm** großen Verbindungsplatte verschraubt. Geplant sind sechs Durchgangsverschraubungen. Die Verbindungslöcher liegen montiert bei X = 233,3 / 249,3 und Y = 14 / 44 / 74 mm.

Als vorgeschlagene Hardware für diese Verbindung: **6 × M3×12**, sechs Muttern und passende Unterlegscheiben. Länge am tatsächlich verwendeten Schrauben-/Scheibenpaket prüfen. Die Muttern sind nicht als Gewindeeinsätze im Kunststoff modelliert; sie werden normal auf der Rückseite aufgeschraubt.

Die Hälften mit 0,2 mm Fuge und den Rack-Langlöchern ausrichten. Beim Anziehen nicht mit Gewalt die Fuge schließen oder das Panel verziehen. Die separaten Blindplatten werden über das identische diagonale D-Raster befestigt. Für die Neutrik-Buchsen selbst sind die zum jeweiligen Artikel passenden Befestigungsschrauben maßgeblich; die M3-Verbindungsschrauben sind nicht automatisch für deren Kunststoffgewinde geeignet.

Für die Rackbefestigung sind Langlöcher für übliche M6-Verschraubung vorgesehen. Passende Käfigmuttern und Unterlegscheiben separat vorsehen. Kabelgewichte und kräftige Biegekräfte sollten durch eigene Zugentlastungen aufgenommen werden, nicht durch die Buchsen oder die gedruckte Naht.

## 7. Einsatzgrenzen und noch offene Hardwaredaten

- **Keine Lastfreigabe:** Das Panel ist ein nicht mechanisch erprobter Prototyp. Bei häufigem Patchen, Transport, schweren Leitungen oder hoher Racktemperatur würde ich einen durchgehenden Metallträger vorsehen. Eine bestimmte Last oder Lebensdauer wird nicht zugesichert.
- **POWER:** Artikelnummern, Gehäusekonturen und Anschlussraum fehlen noch. Die beiden D-Ausschnitte sind deshalb vorläufig. Der Ausdruck ist kein geprüftes Netzspannungsgehäuse und keine Grundlage, offene 230-V-Anschlüsse ungeschützt zu betreiben.
- **HF und Masse:** Eine Kunststofffront ist kein Ersatz für Metallabschirmung, Schutzleiterführung oder ein geplantes Masse-/Potentialausgleichskonzept.
- **BNC:** Vor dem Einbau die tatsächliche Impedanz der Paneldurchführungen prüfen. Beispielsweise ist Neutrik NBB75DFI ausdrücklich eine 75-Ω-Durchführung. Die mechanische D-Kompatibilität ist kein Nachweis für die gewünschte 50-Ω-HF-Eignung.

## 8. Was wurde geprüft?

Der Generator prüft geschlossene Netze, konsistente Dreiecksorientierung, positive und geometrisch plausible Volumina, getrennte Farbvolumina und die Freihaltung der angenommenen Buchsenflansche. Die erzeugten 3MF-Dateien werden unabhängig wieder eingelesen. Die separate lib3MF-Prüfung dokumentiert Lesbarkeit und geschlossene, korrekt orientierte Netze; auch die Druckbettgrenzen der Drucklayouts werden geprüft.

**Nicht geprüft:** reale Drucktoleranzen, Schichthaftung, Verzug, Dauerlast, konkrete Steckerteile, Schutzarten, elektrische Sicherheit, Bambu-Studio-GUI-Import oder tatsächlich erzeugte Druckbahnen. Deshalb zuerst den Passformtest und die eigene Slicer-Vorschau kontrollieren.

## 9. Quelle bearbeiten / Geometrie neu erzeugen

Benötigt werden Python 3, numpy, shapely ab 2.1 mit GEOS ab 3.10, trimesh, matplotlib und cairosvg. Schriftkonturen werden aus einer lokal installierten fetten DejaVu-Sans-Schrift erzeugt; es werden keine Fontdateien mitgeliefert.

Beispiel für einen nach dem Passformtest abweichenden Lochdurchmesser:

```sh
python build_panel.py --output ./Panel_angepasst --hole-d 24.0 --screw-d 3.3
```

Die anderen Konstruktionsgrößen stehen als benannte Konstanten im oberen Teil der Datei. Nach Änderungen alle Kollisionen und Testdrucke erneut prüfen. Intern entstehen Kontroll-SVGs/PNGs; die verbindliche Geometrie steckt in den erzeugten 3MF-/STL-Dateien, nicht in alten KI-Konzeptbildern.

## 10. Externe Maß- und Herstellerreferenzen

Das Funktionslayout stammt aus diesem Chat. Ergänzend wurden folgende Herstellerquellen zum Maßabgleich verwendet; Designzugaben, Teilung, Rippen, Farben und Schriftpositionen sind eigene Konstruktionsentscheidungen.

- Neutrik NE8FDP, Produktseite und Zeichnung: https://www.neutrik.com/en/product/ne8fdp
- Neutrik NE8FDP, offizielles Maßblatt: https://www.neutrik.com/media/8668/download/ne8fdp-3.pdf?v=1
- METCASE M6019020, 2-HE-Frontplatte 482,6 × 88,1 × 3 mm: https://www.metcaseusa.com/en/19-Front-Panels/M6019020.htm
- Rack-Maßabgleich, SANJUN Hardware: https://www.sanjunhardware.com/blog/19-inch-rack-dimensions-eia-310-guide
- Bambu A1, Bauraum und geeignete Materialgruppen: https://bambulab.com/pl/a1/tech-specs
- Neutrik NBB75DFI, ausdrücklich 75 Ω: https://www.neutrik.com/en/product/nbb75dfi
- 3MF-Kernformat: https://github.com/3MFConsortium/spec_core/blob/master/3MF%20Core%20Specification.md
