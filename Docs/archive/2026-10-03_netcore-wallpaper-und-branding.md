# Brainstorming: NetCore-Tetra Wallpaper und Branding

## Rahmen und Quellenstand

- **Thema:** Erstellung eines NetCore-Tetra Desktop- und Smartphone-Hintergrunds unter Verwendung des vorhandenen NetCore-Logos.
- **Notizstand:** 2026-10-03.
- **Zielrepository:** `JanHG98/netcore-tetra`
- **Zielbranch:** `Archiving`
- **Repository-Referenz bei der Prüfung:** Branch `Archiving`; das dort vorhandene Original-Logo `crates/tetra-entities/src/net_dashboard/ui/netcore-logo.png` wurde gefunden. Der geprüfte Default-Branch-Stand enthält außerdem den Merge-Commit `7137e0dd69877e1b604bf89148fd8b6b590c1a97` („NetCore-Design mit Dark Mode für Basisstation und alle Dienst-WebUIs“). Dieser Commit wird nur als am 03.10.2026 geprüfter Repository-Vergleich genannt; er ist nicht der Nachweis für den Stand des Branches `Archiving`.

## 1. Ziel und Ausgangslage

Ziel ist die Gestaltung von Desktop- und Smartphone-Hintergründen für NetCore-Arbeitsgeräte. Das vorhandene NetCore-Logo ist verbindlicher Bestandteil.

Als visuelle Referenz wurde in den Arbeitsnotizen ein quadratisches NetCore-Tetra-Logo bereitgestellt. Sichtbare Kernelemente:

- dunkles, aus vier Knoten und Verbindungen aufgebautes Netzwerk-/Mesh-Symbol,
- drei blaue Funk-/Radiowellen oberhalb/rechts des Symbols,
- Wortmarke **NetCore-Tetra**,
- Claim **„digital. dezentral. skalierbar.“**.

Verbindliche Weiterentwicklung: eine **reduzierte Variante**, eine **Hochkantversion fürs Handy** und zuletzt **keine untere Leiste**.

## 2. Statusklassifikation

| Gegenstand | Status | Nachweis / Bemerkung |
|---|---|---|
| NetCore-Logo als Branding-Basis | **beschlossen/geplant** und im Repository **implementiert** | Im aktuellen Branch `Archiving` ist `crates/tetra-entities/src/net_dashboard/ui/netcore-logo.png` vorhanden. |
| Desktop-Wallpaper | **Idee / Designauftrag** | Der zugängliche Verlauf enthält die Anforderung, aber das tatsächlich erzeugte Desktopbild ist in der Quellenprüfung vom 03.10.2026 nicht als separat zugängliche Originaldatei verfügbar. |
| Cleaner Designstil | **beschlossen** | Spätere Festlegung: reduzierte, saubere Gestaltung. |
| Hochkant-Wallpaper fürs Handy | **als Grafikentwurf umgesetzt** | Eine Hochkantgrafik wurde durch das Bildwerkzeug erzeugt. |
| Mobile Version ohne untere Leiste | **als Grafikentwurf umgesetzt** | Daraufhin wurde eine weitere Hochkantgrafik ohne die unerwünschte untere Leiste erzeugt. |
| Aufnahme der generierten Bilder ins Repository | **offen / teilweise technisch nicht möglich in der Quellenprüfung vom 03.10.2026** | Siehe Abschnitt „Offene Nachweise und Assets“. |

## 3. Endgültige Anforderungen und Entscheidungen

Die spätesten Anforderungen haben Vorrang vor früheren Varianten.

1. **NetCore-Branding muss sichtbar sein.** Das Logo sollte nicht lediglich durch generische Funk-/Netzwerksymbole ersetzt werden.
2. **Cleaner statt überladener Look.** Nach der ersten Designrichtung wurde eine reduzierte, sauberere Variante bevorzugt.
3. **Mobile Hochkantversion.** Das Design sollte auf Smartphone-Displays funktionieren und entsprechend vertikal komponiert werden.
4. **Keine untere Leiste.** Eine in einer vorherigen mobilen Variante vorhandene untere Leiste war unerwünscht und wurde für die nachfolgende Version ausdrücklich entfernt.
5. **Visuelle Richtung der letzten erzeugten Fassung:** dunkler, futuristischer High-Tech-Look; blau leuchtende Netzwerkknoten/-linien; NetCore-Tetra Branding zentral; darunter eine vernetzte Erde bzw. Europa als Kommunikations-/Netzwerkmetapher.

Die letzte erzeugte Darstellung verwendete die Wortmarke **NetCore-Tetra** mit blau hervorgehobenem „Tetra“ sowie den Claim **„digital. dezentral. skalierbar.“**. Das ist eine Designinterpretation des bereitgestellten Logos und kein Nachweis dafür, dass exakt diese Wortmarkenaufteilung Bestandteil des kanonischen Logoassets ist.

## 4. Architektur, Komponenten, Schnittstellen und Abhängigkeiten

Das Vorhaben betrifft statische Branding-Assets; die technische Systemarchitektur wurde nicht verändert.

Relevante Branding-Abhängigkeit im Repository:

`crates/tetra-entities/src/net_dashboard/ui/netcore-logo.png`

Die Repository-Prüfung zeigt außerdem, dass dieses Asset von der Dashboard-UI eingebunden wird. Unter anderem referenziert `crates/tetra-entities/src/net_dashboard/html.rs` das PNG per `include_bytes!`, und die Login-/UI-Dateien verwenden `/assets/netcore-logo.png`. Das Branding ist damit nicht nur ein loses Grafikasset, sondern Bestandteil der aktuellen WebUI.

Der Repository-Stand auf dem Default-Branch enthält am 2026-10-03 den Merge-Commit `7137e0dd69877e1b604bf89148fd8b6b590c1a97`, dessen Commitnachricht die Übernahme des NetCore-Designs mit Dark Mode für Basisstation und Dienst-WebUIs beschreibt. Das passt gestalterisch zur in den Arbeitsnotizen gewünschten dunklen Wallpaper-Richtung, ist aber **kein Beleg**, dass die Wallpaper selbst dort implementiert wurden.

## 5. Erreichter Entwicklungs- und Betriebsstand

### Historisch erarbeitet

- Logo als verbindliches Gestaltungselement festgelegt.
- Cleanere Gestaltung als bevorzugte Richtung festgelegt.
- Smartphone-Hochkantversion erzeugt.
- Nach Gestaltungsfeedback eine weitere Hochkantversion **ohne untere Leiste** erzeugt.
- Letzte sichtbare Fassung: dunkler Hintergrund, blaue Netzstruktur, zentraler NetCore-Tetra-Schriftzug und Funk-/Netzwerklogo, vernetzte Erde/Europa im unteren Bereich.

### Nicht erreicht bzw. nicht belegt

- Keine belastbare Aussage über die exakte Pixelauflösung der finalen Handydatei aus dem zugänglichen Arbeitsnotizen.
- Kein Betrieb-/Deploymenttest nötig oder durchgeführt; es handelt sich um ein statisches Grafikasset.
- Kein Nachweis, dass Desktop- oder Mobile-Wallpaper vor der Quellenprüfung vom 03.10.2026 bereits in Git eingecheckt wurden.
- Keine Aussage über Lockscreen-Safe-Areas, verschiedene Smartphone-Seitenverhältnisse oder automatische Dark-/Light-Varianten getestet.
- Keine verifizierte Desktop-Endfassung im zugänglichen Dateibestand der Quellenprüfung vom 03.10.2026.

## 6. Relevante Dateien und technische Parameter

### Repository

- `crates/tetra-entities/src/net_dashboard/ui/netcore-logo.png` — kanonisches/aktuell verwendetes Logoasset im geprüften Branch.
- `crates/tetra-entities/src/net_dashboard/html.rs` — bindet das Logo binär in die Dashboard-Anwendung ein.
- `crates/tetra-entities/src/net_dashboard/ui/login.html` — verwendet das Logo in der Login-Oberfläche.
- `crates/tetra-entities/src/net_dashboard/ui/netcore.js` — verwendet das Logo in der NetCore-Branding-/Navigationsdarstellung.
- `tools/embed_service_design.py` — synchronisiert laut aktuellem Repository das Original-PNG in ein Data-URI-Asset für Service-WebUIs.
- `system-backend/shared/web-ui/README.md` — dokumentiert den Logo-Sync für Shared-WebUI-Assets.

### Grafikparameter

- Primärer Stil: Dark / High-Tech / futuristisch.
- Akzent: leuchtendes Blau.
- Motivik: Netzwerkgraph, Funkwellen, globale/verteilte Vernetzung, Erde/Europa.
- Mobile Layout: Hochkant.
- Letzte Korrektur: keine zusätzliche Leiste am unteren Bildrand.

Ports, Protokolle, IP-Adressen, Funkparameter oder Dienstkonfigurationen waren in dieser Entwicklungsphase nicht Gegenstand der Arbeit.

## 7. Befehle, Installation, Deployment und Reparatur

Keine Installations-, Deployment- oder Reparaturbefehle wurden für das Wallpaper benötigt oder ausgeführt.

Die Bilder wurden über ein Bildgenerierungswerkzeug erstellt. Das ist **kein Repository-Buildschritt** und kein Bestandteil der NetCore-Tetra-Runtime.

## 8. Fehler, Diagnose und Lösungen

### Designproblem: Logo fehlte zunächst als belastbare Vorlage

**Diagnose:** Für ein sauberes NetCore-Wallpaper sollte nicht irgendein nachgebautes Logo verwendet werden.

**Lösung:** Das bereitgestellte Projektlogo wurde als verbindliche visuelle Referenz verwendet.

**Status:** gelöst.

### Designproblem: erste Variante nicht clean genug

**Diagnose:** Die erste Variante war zu dekorativ; eine reduzierte Gestaltung wurde ausdrücklich gefordert.

**Lösung:** Gestaltung wurde in Richtung reduzierter High-Tech-/Dark-Optik weitergeführt.

**Status:** Designentscheidung übernommen.

### Designproblem: untere Leiste in mobiler Variante

**Diagnose:** Die untere Leiste war ausdrücklich unerwünscht.

**Lösung:** Eine neue vertikale Fassung ohne diese Leiste wurde generiert.

**Status:** als Entwurf umgesetzt.

## 9. Tests und Grenzen

Es wurden keine automatisierten Tests durchgeführt, da es sich um ein Grafikdesign handelt.

Visuelle Abnahme erfolgte iterativ durch Gestaltungsfeedback:

- Logo einbringen → akzeptierte Richtung.
- Cleaner gestalten → neue Anforderung.
- Hochkant fürs Handy → umgesetzt.
- Untere Leiste entfernen → umgesetzt.

Nicht getestet bzw. nicht dokumentiert:

- unterschiedliche Smartphone-Auflösungen und Notch-/Dynamic-Island-Safe-Areas,
- Android-/iOS-Lockscreen-Cropping,
- Desktop-Multi-Monitor-Cropping,
- OLED-optimierte Schwarzwerte,
- Lesbarkeit hinter Desktop-Icons,
- 4K/5K/Ultrawide-Varianten.

## 10. Verworfene oder ersetzte Ansätze

- **Wallpaper ohne originales NetCore-Logo:** verworfen; eigenes Logo ist verbindlich.
- **Stärker dekorierte/ältere Variante:** durch Wunsch nach „cleaner“ ersetzt.
- **Mobile Variante mit unterer Leiste:** durch spätere ausdrückliche Korrektur ersetzt; für zukünftige Ableitungen nicht als Zielversion verwenden.

## 11. Ideen, Wünsche und offene Aufgaben

### Roadmap-Kandidaten

- **[Idee] Wallpaper-Pack statt Einzelbild:** Desktop 16:9, 16:10, 21:9/Ultrawide, 32:9 sowie Mobile 9:16/9:19.5/9:20.
- **[Idee] OLED-Version:** nahezu echtes Schwarz mit sehr dezenten blauen Netzknoten.
- **[Idee] Clean/Minimal-Version:** nur Logo, Claim und extrem subtile Netzstruktur; keine Erde.
- **[Idee] „Infrastructure“-Version:** stilisierte Basisstationen/Nodes, Backhaul und Mesh-Verbindungen als technische, aber nicht überladene Hintergrundgrafik.
- **[Idee] Lockscreen-Version:** Branding höher oder tiefer positionieren, damit Uhr/Benachrichtigungen nichts verdecken.
- **[Idee] Desktop-Safe-Area:** zentrale Motive so platzieren, dass Windows-Taskleiste und Desktop-Icons nicht kollidieren.
- **[beschlossen für Fortsetzungen]** Untere Zusatzleiste bei der mobilen Zielversion weglassen.
- **[beschlossen für Fortsetzungen]** Das echte NetCore-Logo verwenden und nicht frei neu erfinden.
- **[offen]** Finale gewünschte Desktop-Auflösung(en) und konkretes Smartphone-Modell/Displayformat festlegen, wenn pixelgenaue Exporte benötigt werden.
- **[offen]** Die finalen Originaldateien der in den Arbeitsnotizen erzeugten Wallpaper dauerhaft unter `Docs/archive/` ablegen, sobald sie als Binärdateien für einen GitHub-Upload verfügbar sind.

## 12. Entwicklungsstand und Repository-Befund vom 03.10.2026

### Historischer Entwicklungsstand

Ergebnis des Designlaufs ist die mobile Hochkantfassung ohne untere Leiste. Funkfunktionen oder Dienste wurden nicht verändert.

### Zusätzlich überprüfter Repository-Stand am 2026-10-03

- Branch `Archiving` existiert.
- `Docs/archive/README.md` existiert bereits und enthält Archivierungen anderer Themen; diese müssen erhalten bleiben.
- Das Logoasset `crates/tetra-entities/src/net_dashboard/ui/netcore-logo.png` ist im Branch `Archiving` vorhanden.
- Im am 03.10.2026 geprüften Repository-Hauptstand existiert der Merge-Commit `7137e0dd69877e1b604bf89148fd8b6b590c1a97` zum NetCore-Design/Dark-Mode.
- Es wurde **kein vorhandenes Archivdokument für genau dieses Wallpaper-Vorhaben** unter dem vorgesehenen Dateinamen gefunden.
- Aus diesen Repository-Befunden folgt **nicht**, dass die in den Arbeitsnotizen generierten Wallpaper bereits Bestandteil des Produktcodes oder eines Releases sind.

## 13. Relevante Quellen, Anhänge, Branches, Commits und PRs

### Visuelle Arbeitsgrundlagen

1. Ausdrücklich in den Arbeitsnotizen bereitgestelltes quadratisches NetCore-Tetra-Logo mit Netzwerkmarke, Funkwellen, Wortmarke und Claim.
2. In den Arbeitsnotizen generierte vertikale NetCore-Tetra-Handygrafik.
3. Nachfolgend generierte vertikale Version ohne unerwünschte untere Leiste.

### Repository

- Repository: `JanHG98/netcore-tetra`
- Archivbranch: `Archiving`
- Aktuelles Logoasset: `crates/tetra-entities/src/net_dashboard/ui/netcore-logo.png`
- Am 03.10.2026 geprüfter Default-Branch-Vergleich: `7137e0dd69877e1b604bf89148fd8b6b590c1a97`, Merge von PR #59 („NetCore-Design mit Dark Mode für Basisstation und alle Dienst-WebUIs“).

## 14. Offene Nachweise und Assets

Die Wallpaper sind als Bildvorschauen dokumentiert. Original-Binärdateien standen für die Git-Ablage am 03.10.2026 nicht direkt zur Verfügung; eine vollständige Übernahme ins Repository ist deshalb nicht nachgewiesen.

Das **kanonische NetCore-Logo selbst** ist dagegen bereits im Repository vorhanden und wurde am 03.10.2026 im Branch `Archiving` verifiziert. Es muss daher nicht dupliziert werden.

Für eine spätere vollständige Asset-Archivierung sollten die finalen generierten Bilder, sofern wieder als Dateien verfügbar, ausschließlich unter einem Unterpfad von `Docs/archive/` abgelegt und aus diesem Dokument relativ referenziert werden.

## 15. Konkrete nächste Schritte

1. Finales Wallpaper-Pack mit festen Zielauflösungen definieren.
2. Mobile Safe-Area für das tatsächlich verwendete Smartphone prüfen.
3. Optional Desktop-Varianten für die konkret verwendeten Monitorauflösungen erzeugen.
4. Final ausgewählte PNGs unter `Docs/archive/` archivieren und hier verlinken.
5. Keine Designvariante mit unterer Leiste als aktuelle mobile Zielversion weiterverwenden.
6. Bei zukünftigen Generierungen das vorhandene Repository-Logo als Branding-Referenz verwenden.

---

Die Notizen betreffen ausschließlich das Wallpaper- und Branding-Vorhaben.
