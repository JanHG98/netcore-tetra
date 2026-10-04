# Technische Abschlussdokumentation: Mobiles 4-HE-Rack, Sophos/Proxmox-Core, WLAN und DECT

## 1. Metadaten und Geltungsbereich

| Feld | Wert |
|---|---|
| Projekt | NetCore-Tetra |
| Thema | Mobile TETRA-Basisstation als kompakter Kommunikationsknoten; Rackverkleinerung durch Virtualisierung und Verzicht auf eine interne USV |
| Ursprünglicher Chattitel | Nicht verfügbar. Der Titel dieses Dokuments ist ein beschreibender Archivtitel, kein rekonstruierter Originaltitel. |
| Ursprünglicher Chatlink | Nicht verfügbar; kein Link erfunden. |
| Zusammenfassung erstellt | **2026-10-05**, Zeitzone Europe/Berlin |
| Historischer Gesprächszeitraum | Der ergänzende Kontextdienst ordnet die Beiträge zu 6 HE, Sophos und USV-Verzicht dem **29.07.2026, 20:58–21:06 UTC** zu. Diese Datierung ist durch keinen vollständigen Chatexport unabhängig bestätigt. Die direkt bereitgestellten Nachrichten enthalten keine eigenen Zeitstempel. |
| Repository | [JanHG98/netcore-tetra](https://github.com/JanHG98/netcore-tetra) |
| Ausschließlicher Schreibbranch | **`Archiving`**, exakt diese Groß-/Kleinschreibung |
| Geprüfter Archivbranch vor diesem Auftrag | [`b4736a4d37fe2044fff998d017a36c1c950a8788`](https://github.com/JanHG98/netcore-tetra/commit/b4736a4d37fe2044fff998d017a36c1c950a8788) |
| Zusätzlich nur lesend geprüfter Defaultbranch | `main`, [`7137e0dd69877e1b604bf89148fd8b6b590c1a97`](https://github.com/JanHG98/netcore-tetra/commit/7137e0dd69877e1b604bf89148fd8b6b590c1a97) |
| Dokumentpfad | `Docs/archive/2026-10-05_mobiles-4he-rack-sophos-proxmox-core-wlan-und-dect.md` |
| Archivindex | [README.md](README.md) |
| Archivierungscommit | Der oben genannte Prüfcommit ist der Stand **vor** der Archivänderung. Der tatsächliche neue Archivierungscommit ist über die Dateihistorie und die Abschlussmeldung dieses Auftrags feststellbar; er kann nicht als eigene Commit-ID in seinen Inhalt eingeschrieben werden. |

### 1.1 Tatsächlich zugängliche Quellen

- **H1: Direkter Verlauf.** Zwölf bereitgestellte Gesprächsnachrichten (`msg_idx` 0–11), davon sechs Nutzerbeiträge und sechs Assistenzantworten, vom ersten Rackentwurf bis zur letzten 4-HE-Empfehlung. Die über mehrere Kontextblöcke verteilten Antworten wurden zusammenhängend berücksichtigt. Hinzu kommt der aktuelle Archivauftrag.
- **H2: Ergänzende Kontextsuche.** Sie bestätigt die Sequenz 6 HE/550 mm → Sophos mit Proxmox → zunächst ohne USV/4 HE und liefert die oben vorsichtig gekennzeichnete historische Datierung. Sie liefert keinen Originaltitel, keinen Chatlink und keinen vollständigen zusätzlichen Export dieses Chats.
- **A1–A25: Anhänge.** 25 lokal bereitgestellte, lesbare PDF-Dateien mit TETRA-Normen, einschließlich der Sammeldatei `ETSI.pdf`. Identitäten, Versionen, Umfang und Prüfsummen sind in Abschnitt 12 inventarisiert.
- **R1–R15: Repository.** Branchreferenzen, Git-Bäume, Dienstkatalog, Beispielkonfigurationen, Dokumentation und einschlägige Fallback-/SIP-/Storage-Quelltexte wurden am oben genannten Stand gelesen. Die Nachweise stehen in Abschnitt 13.
- **W1–W3: Externe Primärquellen.** Die drei in den historischen Antworten verlinkten PoE-/Montage-/DECT-Quellen wurden erneut geöffnet und in Abschnitt 13 eingeordnet.

### 1.2 Auswertungslücken und Abgrenzung

Dies ist eine ausführliche Auswertung des **zugänglichen** Verlaufs, keine Behauptung, alle je in diesem Chat entstandenen Nachrichten oder Dateien gesehen zu haben. Nicht vorhanden sind der Originalchatexport, der Originaltitel/-link, ein Gerätedatenblatt der vorgesehenen Sophos, eine Stückliste, ein maßhaltiger Rackplan, Fotos des Aufbaus, reale Installationsprotokolle und Messdaten dieses Rackprojekts.

Die PDFs wurden vollständig zur Textextraktion eingelesen, ihre Titelblätter und Seitenzahlen inventarisiert und für die Rackfragen relevante Stellen selektiv geprüft. Die **8.061 PDF-Seiten einschließlich Dopplungen** wurden nicht vollständig fachlich oder grafisch begutachtet. Die Normensammlung ersetzt weder einen Hardwareabnahmetest noch eine vollständige ETSI-Konformitätsprüfung.

Andere Projektchats über WERMA, BPI-Router, Koax-/3D-Druck-Patchpanel, FRN, größere Flottenracks oder die spätere B&W/Peli-Beschaffung sind benachbarter Kontext. Deren Entscheidungen werden hier nicht nachträglich als Beschlüsse dieses Chats ausgegeben. Passende vorhandene Archive werden lediglich als Querverweise genannt. Insbesondere ist in **diesem** Verlauf kein konkreter Sophos-, Switch-, AP-, DECT- oder Flightcase-Typ gewählt.

## 2. Ziel, Ausgangslage und behandelte Themen

Jan wollte eine mobile Basisstation in einem transportablen Rack zusammenstellen. Der Ausgangsentwurf enthielt **USV, PDU, TBS, NAS, Router, Switch und Licht**. Hinzu kamen die Fragen, ob ein WLAN-Access-Point und eine DECT-Basis sinnvoll seien und ob beide am gemeinsamen Mast unterhalb zweier UHF-Antennen montiert werden könnten.

Das Ziel entwickelte sich zu einem lokal nutzbaren Kommunikationsknoten mit:

- TETRA-Funkzelle und zugehörigen SwMI-Diensten;
- lokalem Ethernet und WLAN für Administration, Operatorgeräte und Dateizugriff;
- SIP-/DECT-Telefonie über eine eigene PBX;
- Medien, Aufzeichnungen, TTS-Dateien und Monitoring;
- optionalem externen WAN beziehungsweise späterem LTE-/5G-Uplink;
- möglichst wenigen Höheneinheiten und überschaubarem Transportgewicht.

Der Entwicklungsstand wurde vom Nutzer ausdrücklich als frühe Projektphase beschrieben. Verkleinerung und ein späterer Umzug in ein größeres Case waren deshalb akzeptabel. Es ging um **Konzept und Komponentenorganisation**, nicht um einen in diesem Gespräch ausgeführten Rackbau oder eine bestätigte hochverfügbare Einsatzanlage.

## 3. Entscheidungsverlauf und zuletzt gültiger Plan

| Reihenfolge / H1-Nachricht | Aussage oder Vorschlag | Bedeutung und Status |
|---|---|---|
| 0–1 | Ausgangsrack mit USV/PDU/TBS/NAS/Router/Switch/Licht; WLAN und DECT ergänzen? | Nutzeridee. WLAN wurde empfohlen; DECT bei konkretem Telefoniezweck ebenfalls. Noch keine Geräteauswahl. |
| 2–3 | WLAN und DECT am Mast unterhalb der beiden UHF-Antennen | Nutzerseitig gewünschte Platzierungsrichtung. Seitlicher Ausleger und Abstände sind darauf folgende Assistenzvorschläge, keine geprüfte Konstruktion. |
| 4–5 | Einschätzung als interessante mobile Kommunikationszentrale | Zielbild verdichtet; Gewicht, Wärme, Leistung und HF-Verträglichkeit als Planungsfragen benannt. |
| 6–7 | Möglichst **6 HE**, Flightcase mit maximal „550 cm“ Einbautiefe | Der Assistent interpretierte „550 cm“ als **550 mm**. Das ist eine naheliegende, im weiteren Gespräch verwendete Interpretation; eine ausdrückliche separate Nutzerkorrektur der Einheit ist nicht sichtbar. |
| 7 | 1 HE Panel, 1 HE Router/Switch, 2 HE TBS, 1 HE NAS, 1 HE USV | Erster **Assistenzlayoutvorschlag** für sechs HE, durch spätere Konsolidierung ersetzt. |
| 8–9 | NAS, Router, SwMI und PBX auf einer Sophos unter Proxmox; mehrere Dienste in **1 HE** | **Nutzeridee und bevorzugte Planungsrichtung**, noch ohne konkrete Appliance oder Installationsnachweis. Der Assistent nahm eine ausgemusterte x86-SG-/XG-Appliance an; Modellfamilie und Hardware sind unbestätigt. |
| 9 | 1 HE Panel, 1 HE PoE-Switch, 1 HE Core, 2 HE TBS, 1 HE USV | Zweiter sechs-HE-Vorschlag; separate NAS-/Router-Gehäuse entfallen, Dienste bleiben logisch getrennt. |
| 10 | USV aus Gewichtsgründen zunächst weglassen; später ggf. größeres Case; dadurch sogar **4 HE** | **Jüngste ausdrückliche Nutzerfestlegung zur ersten Ausbaustufe.** Die interne USV ist vorläufig aus dem Umfang genommen; vier HE sind das neue Ziel. |
| 11 | 1 HE PoE-Switch, 1 HE Sophos/Proxmox-Core, 2 HE TBS; PDU/Licht/Anschlüsse ohne eigene Front-HE | Letzter **Assistenzlayoutvorschlag**, konsistent mit der Nutzerentscheidung. Machbarkeit muss an konkreten Geräten nachgewiesen werden. |

### 3.1 Verbindlich von Vorschlägen unterscheiden

**Vom Nutzer festgelegt oder als klare Richtung benannt:** kompakt und mobil; zunächst ohne interne USV; Ziel vier HE; späteres größeres Case möglich; Dienste auf einer Sophos mit Proxmox zusammenfassen, sofern technisch machbar; WLAN/DECT am Mast unterhalb der UHF-Antennen erwägen.

**Vom Assistenten vorgeschlagen, noch nicht gesondert freigegeben oder nachgewiesen:** genaue HE-Belegung, TBS mit zwei HE, PDU als 0-HE-/Seiteneinbau, PoE-Versorgung, konkrete VLAN-Trennung, seitlicher Mastarm, SSD-/ZFS-Mirror, CPU-/RAM-Zielgrößen, Lüfter, Servicepanel, lokale Notfalladressen und spätere externe USV.

Die letzte Antwort „Ja, mach zunächst 4 HE“ ist eine Empfehlung des Assistenten. Die stärkste Nutzerquelle ist die unmittelbar vorherige Entscheidung, die USV zunächst wegzulassen und damit vier HE zu ermöglichen. Es wurde keine Bestellung, Installation oder Fertigstellung bestätigt.

### 3.2 Statusbegriffe dieses Archivs

| Status | Bedeutung |
|---|---|
| **Idee** | Erwähnt oder erwogen, ohne abgeschlossene Auswahl beziehungsweise Ausführung. |
| **beschlossen/geplant** | Ausdrückliche Nutzerentscheidung oder dokumentierter Arbeitsplan; genaue Vorschläge bleiben als solche erkennbar. |
| **implementiert** | Passender Code, Konfiguration oder Artefakt ist im geprüften Repository tatsächlich vorhanden. Das beweist kein Deployment. |
| **getestet** | Ein konkreter Test wurde ausgeführt und sein Ergebnis liegt vor. Vorhandener Testcode oder eine Chatbehauptung allein genügt nicht. |
| **im Betrieb bestätigt** | Reale Anlage, Logs, Messung oder eindeutig zugehörige Nutzerbestätigung belegen den Betrieb. |

## 4. Physische Architektur des mobilen Racks

### 4.1 Jüngster 4-HE-Entwurf

| Einbau von oben nach unten | Höhe | Aufgabe | Nachweisstand |
|---|---:|---|---|
| PoE-Switch | 1 HE | Ethernet-Verteilung, VLANs und PoE für die Mastgeräte | Geplant als Baugruppe; Modell, Portzahl und PoE-Budget offen. |
| Sophos/Proxmox-Core | 1 HE | Router-VM, Backend-LXCs, PBX, Storage-/Share-Dienste, Monitoring | Konsolidierungsrichtung vom Nutzer; Appliance und Virtualisierung unbestätigt. |
| TETRA-Basisstation | 2 HE | Separater funk-/echtzeitnaher TBS-Aufbau | Zwei HE sind Layoutannahme des Assistenten; reale Höhe, Gehäuse und HF-Aufbau nicht vermessen. |
| **Summe** | **4 HE** | Keine eigene Front-HE für USV, separates NAS oder separates Patchpanel | Reine Höhenbilanz, noch kein mechanischer Passnachweis. |

Außerhalb dieser Frontbelegung wurden vorgeschlagen:

- PDU beziehungsweise Stromverteilung hinten oder seitlich;
- Einspeisung, Potentialausgleich, WAN, Service- und Mastanschlüsse am rückseitigen Anschlussfeld;
- HF-Anschlüsse direkt an der TBS-Front;
- Statusanzeigen in der TBS-Front beziehungsweise einer schmalen Zusatzblende;
- LED-Leiste am oberen Rahmen oder im Deckel;
- Lüfter und Zugentlastungen innerhalb des tatsächlich verfügbaren Einbauraums;
- WLAN-AP und DECT-Basis außen am Mast.

**Offener Widerspruch im Lichtvorschlag:** Beide Case-Deckel sollen im Betrieb entfernt werden. Eine nur im abgenommenen Deckel verbaute Leuchte braucht eine passende Aufstellung und Steckverbindung; eine Rahmenleuchte vermeidet diese Abhängigkeit. Es wurde keine Variante endgültig gewählt.

### 4.2 Tiefe, Steckraum und Wartbarkeit

Die weiterverwendete Zielgröße ist **ca. 550 mm verfügbare Einbautiefe**. Der Assistent schlug etwa **400–450 mm maximale Chassistiefe** vor, damit hinten Platz für Stromstecker, Netzwerk-/Koaxkabel, Biegeradien, PDU und Abluft bleibt. Das ist eine Planungsreserve, keine bestätigte Herstellerangabe oder universelle Grenze.

Vor der Beschaffung sind Front-/Rückschienenabstand, nutzbarer Raum bei montierten Deckeln, Geräteohren, Steckerlängen, Koax-Biegeradien und PDU-Einbau gemeinsam zu prüfen. Ein nominell passendes Gehäuse reicht nicht, wenn der Stecker am Deckel ansteht. Ein seitlicher oder rückseitiger PDU-Einbau ist nur dann 0 HE, wenn Case und Komponenten den notwendigen Platz und sichere Befestigung tatsächlich bieten.

Für die Wartung wurden abnehmbare Front-/Rückdeckel, steckbare interne Verbindungen und Zugentlastungen empfohlen. Ein einzelnes Gerät soll entnehmbar sein, ohne alle anderen Baugruppen auszubauen. Schwere Geräte benötigen eine ihrem Hersteller entsprechende Abstützung; die Frontohren allein sind kein in diesem Chat geprüfter Transportnachweis.

### 4.3 Gewicht und Wärme

Die historische Grobschätzung bezog sich auf das **frühere sechs-HE-Rack mit USV und separatem NAS**:

| Baugruppe | Historische Schätzung |
|---|---:|
| Flightcase | 10–15 kg |
| USV | 10–20 kg |
| TBS | 5–10 kg |
| NAS mit Laufwerken | 5–9 kg |
| Router, Switch, PDU | 3–6 kg |
| Kabel, Panel und Zubehör | 3–5 kg |
| Gesamt | 36–65 kg |

Diese Zahlen sind **keine Messwerte und kein gültiges Gewichtsbudget des neuen vier-HE-Entwurfs**. Der Verzicht auf USV und separate Gehäuse spart Bauraum und voraussichtlich Gewicht; die verbleibende Last wurde nicht berechnet. Eine Lithium-/LiFePO₄-USV war eine frühere Assistenzoption, kein gewähltes Produkt.

Für den ursprünglichen größeren Aufbau wurden vier Klappgriffe und Tragen durch zwei Personen vorgeschlagen. Die finale Transportplanung hängt weiterhin von den realen Gerätemassen und den zulässigen Lasten des Cases ab. Die USV „ganz unten“ betrifft nur eine spätere Variante mit internem Akku.

Kühlung war durchgängig ein offener Prüfpunkt. Die historischen Antworten schlugen offene Front/Rückseite und ein bis zwei temperaturgeregelte 120-mm-Lüfter hinten oben vor. Der tatsächliche Luftweg muss zu den jeweiligen Gerätelüftern passen. Das frühere schematische Durchströmen sämtlicher Geräte nacheinander von unten nach oben ist **kein verifizierter Kühlungsentwurf**; gemeinsame Front-/Rückrichtung und Vermeidung von Warmluftkurzschlüssen sind an den konkreten Geräten zu prüfen. Ob 120-mm-Lüfter im vier-HE-Case sinnvoll befestigbar sind, wurde nicht gezeigt.

## 5. Mast, WLAN und DECT

### 5.1 Einsatzzweck und Platzierung

Der WLAN-AP wurde für Notebook, Tablet/Zebra, TBS-/Router-/NAS-/SwMI-WebUIs, Operatoroberfläche, Wartung und lokale Dateien empfohlen. Lokales WLAN soll auch ohne Internet nutzbar sein. Spätere WAP-/Packet-Data-Versuche wurden als Nebenidee genannt; gewöhnliche WLAN-Clients und ein TETRA-Paketdatenpfad sind dabei unterschiedliche Zugänge.

Die IP-DECT-Basis soll SIP-Nebenstellen für Technik, Operator/Einsatzleitung, Störungsannahme und interne Telefonie bereitstellen. Genannt wurden zunächst **eine Basis und zwei bis vier Mobilteile** als sinnvolle Größenordnung. Tischtelefone, Alarmierungs-/Bereitschaftstelefone und späteres kontrolliertes Interworking zu TETRA-Einzelrufen waren zusätzliche Ideen. Ein konkretes DECT-Modell, Outdoor-Schutz, Mobilteile, gleichzeitige Kanäle oder Reichweite wurden nicht gewählt beziehungsweise gemessen.

Zuerst wurde ein kleiner Mast für WLAN/DECT/optional LTE neben einem getrennten TETRA-Mast vorgeschlagen. Jan fragte danach ausdrücklich nach Montage **am UHF-Antennenmast unterhalb der zwei UHF-Antennen**. Der spätere Assistenzvorschlag folgt dieser Richtung: UHF oben, WLAN/DECT darunter auf einem seitlichen gemeinsamen Träger. Der separate Mast bleibt eine frühere Alternative, nicht der letzte Entwurf.

### 5.2 Historische Maßvorschläge, ohne Abnahme

| Abstand / Anordnung | Im Chat genannter Wert | Einordnung |
|---|---|---|
| UHF-Antenne 1 zu UHF-Antenne 2 | 0,8–1,5 m | Assistenzskizze; keine gemessene RX/TX-Entkopplung und keine Vorgabe aus den beigefügten Normen. |
| Untere UHF-Antenne zu WLAN-/DECT-Träger | mindestens ca. 1 m, besser 1,5–2 m; Schlussformulierung etwa 1–1,5 m | Nicht völlig einheitliche Assistenzwerte. Als grobe Prüfvarianten erhalten, nicht als verbindlicher Mindestabstand harmonisiert. |
| WLAN/DECT seitlich zum Metallmast | ca. 30–50 cm | Heuristik zur Verringerung der Mastbeeinflussung; modell-/antennenabhängig. |
| WLAN zu DECT | ca. 30–50 cm, ggf. gegenüberliegende Seiten des Arms | Mechanischer Vorschlag; kein Nachweis einer erforderlichen HF-Trennung. |

Der allgemeine Hinweis, leitfähige Objekte in Antennennähe zu vermeiden, wird durch den Cisco-Montageleitfaden gestützt. Daraus lassen sich **keine universellen 30-/50-cm- oder 1-/2-m-Grenzen** für diesen Mast ableiten. Auch „die sauberste Lösung“ war eine zu weitgehende Bewertung ohne Gerätemodelle, Sendeleistung und Standortmessung. Der gemeinsame Mast ist ein Kandidat, dessen Eignung noch zu prüfen ist. [W2]

Classic DECT-Telefonie nutzt in Europa typischerweise **1.880–1.900 MHz**. Die historische ETSI-Quelle ist ein Bericht zu **DECT-2020 NR**, enthält aber in seinem Abschnitt 7.2 auch den Hintergrund des europäischen DECT-Bandes. Eine hier gewünschte SIP-DECT-Telefonbasis darf daraus nicht als bereits ausgewähltes DECT-NR+-System ausgegeben werden. Der Frequenzabstand zum UHF-TETRA-Pfad beweist allein keine Störungsfreiheit. [W3]

### 5.3 Verkabelung und Wetterbetrieb

Vorgeschlagen wurden zwei PoE-Leitungen vom Rack zum Mast, jeweils eine für WLAN und DECT; eine wettergeschützte kleine Anschlussbox am Mastfuß; etherCON oder geeignete wetterfeste RJ45-Durchführungen; Schnellhalterungen sowie UV-beständige Kabel und Tropfschleifen.

Weitere Planungsprüfpunkte waren Außenbetriebstauglichkeit, Schirmung, Ethernet-Überspannungsschutz am Rackeintritt, Potentialausgleich und saubere Kabel-/Zugentlastungsführung ohne unnötige Schleifen direkt neben HF-Leitungen. Kein Typ einer Schutzkomponente oder Erdungsaufbau wurde konstruiert oder abgenommen. Die IP-Eignung gilt für die **vollständige gesteckte Kombination**, nicht automatisch für jeden etherCON-Stecker.

PoE-Daten-/Stromversorgung kann separate Netzteile am Mast vermeiden. Ob IEEE-PoE oder eine passive Versorgung verwendet werden darf, ist am gewählten AP und an der DECT-Basis zu prüfen; im Chat wurde kein Verfahren gewählt. Das Switchgesamtbudget muss beide Geräte und etwaige weitere PoE-Verbraucher unter Last versorgen. Im finalen Aufbau ohne USV entfällt die frühere Aussage, beide Funkgeräte seien dadurch automatisch batteriegepuffert. [W1]

Für den Transport war zunächst eine gepolsterte **1-HE-Schublade** für den AP genannt worden. Die letzte vier-HE-Belegung hat dafür keine freie Front-HE. Als überholte Einbauidee erhalten; Aufbewahrung der absetzbaren Geräte ist noch zu lösen.

### 5.4 Vorgeschlagener HF-Verträglichkeitstest

Nicht ausgeführt wurde der folgende, ausdrücklich vorgeschlagene Vergleich:

1. TETRA-Empfang, Rauschboden, gegebenenfalls RSSI/BER und Spektrum ohne WLAN/DECT erfassen.
2. WLAN und DECT einschalten und dieselben Empfangsbedingungen wiederholen.
3. WLAN unter Datenlast betreiben, mehrere DECT-Rufe aufbauen.
4. TETRA mit der tatsächlich vorgesehenen Sendeleistung betreiben; RX/TX-Zusammenwirkung und Störungen am SDR prüfen.
5. Pegel, Fehlerquote beziehungsweise reproduzierbare Empfangsschwelle vergleichen; Geräteposition, Mastmaße und Betriebszustände protokollieren.

**Grenze:** Ein Rauschboden- oder RSSI-Vergleich allein beweist noch keine unveränderte Empfindlichkeit. Die beigefügte EN 300 394-1 unterscheidet unter anderem Blocking und Intermodulationsprüfungen; ein allgemeiner Praxistest ersetzt deren definierte Konformitätsmessung nicht. Es wurden keine Grenzwerte als bereits erreicht übernommen. [A18, Abschnitte 7.1.8 und 7.2.5]

## 6. Proxmox-Core auf der Sophos

### 6.1 Geplante Funktionsverteilung

Proxmox ist in dieser Planung der **Hypervisor auf der physischen Sophos-Appliance**. Router, SwMI-Dienste, PBX und Shares laufen als dessen Gäste beziehungsweise Dienstinstanzen. Es wurde nicht beschlossen, Proxmox selbst verschachtelt in eine Sophos-Firewall-VM einzubauen oder das originale Sophos-Firewallbetriebssystem beizubehalten.

| Ebene | Geplante Aufgabe | Noch ungeklärt |
|---|---|---|
| Physische x86-Appliance, 1 HE | CPU, RAM, SSDs, mehrere NICs und Proxmox | Exaktes Modell, CPU/Virtualisierung, BIOS, RAM-Ausbau, Laufwerksplätze, Leistungsaufnahme und Gerätemaße. |
| Router-/Firewall-VM | WAN, optional zweites WAN/LTE, VLAN-Routing, DHCP und Regeln | Routersoftware, Port-/VLAN-Plan und Passthrough-Möglichkeiten. |
| Backend-LXCs | Node Gateway, Subscriber/Group/Mobility/Call Control, Media Switch, SDS/Packet/IP-Dienste und weitere tatsächlich benötigte Dienste | Inventory, Ressourcen, isolierte Adressen, Start-/Readiness-Abhängigkeiten. |
| PBX-VM oder -LXC | Asterisk/FreePBX für DECT und weitere SIP-Nebenstellen | VM versus LXC, Installationsstand, Netz, Nebenstellen, Trunks, Backup und Codec-/DTMF-Abnahme. |
| Media-/Share-Dienst | Recordings, Media Library, TTS und Dateifreigaben | Abgrenzung zur vorhandenen Media-Library-API, Dataset-/Mount-/Rechtekonzept. |
| Monitoring-Dienste | Logs, Metriken und Betriebsstatus | Welche vorhandenen NetCore-Dienste lokal gebraucht werden. |
| Separater TBS-Knoten | Air Interface, HF und lokale zeitkritische Funktionen | Unabhängige Strom-/Netz-/Storage-Abhängigkeiten und Fallbackabnahme. |

„Mehrere Dienste in einer HE“ bezeichnet das gemeinsame **physische** Gehäuse. Es ist kein Beschluss, alle Programme in einen einzigen Gast oder denselben Netzwerknamespace zu packen. Der heutige Repository-Ansatz mit Dienst-LXCs lässt sich grundsätzlich auf einem Host unterbringen, benötigt aber weiter echte Speicher-/CPU- und Netzplanung.

### 6.2 Sophos-Prüfpunkte und historische Ressourcenempfehlung

Der Chat hat keinen Nachweis, dass die vorhandene oder zu beschaffende Appliance Proxmox bootet, genug RAM aufnehmen kann oder ihre Netzwerkkarten getrennte IOMMU-Gruppen besitzen. Die Annahme „viele SG-/XG-Geräte sind normale x86-Server“ war eine allgemeine Assistenzannahme, keine Modellfreigabe.

| Ressource | Historische Assistenzgröße „Minimum“ | Historische Assistenzgröße „sinnvoll“ |
|---|---|---|
| CPU | 6 Kerne / 12 Threads | 8 Kerne / 16 Threads |
| RAM | 32 GB | 64 GB |
| Systemspeicher | 2 × 1 TB SSD/NVMe | 2 × 2 TB |
| Medienspeicher | 2 × 2 TB SSD | 2 × 4 TB oder mehr |
| Netzwerk | 4 × 1 GbE | 4–6 Ports, teilweise 2,5/10 GbE |
| RAM-Eigenschaft | ohne Festlegung | ECC, wenn unterstützt |

**Diese Zahlen sind keine gemessenen Mindestanforderungen der SwMI und kein von Jan festgelegtes Beschaffungsprofil.** Es wurden weder VM-/LXC-Ressourcen verteilt noch Sprach-/TTS-/Storage-Last gemessen. Ein konkretes Sophos-Gerät kann weniger Kerne, RAM oder Laufwerksplätze besitzen. Die sinnvolle Dimensionierung muss mit der wirklich lokalen Dienstmenge ermittelt werden.

### 6.3 Storage-Vorschlag

Der Assistent schlug **SSDs statt mechanischer Laufwerke** für den mobilen Aufbau und ZFS direkt auf dem Proxmox-Host vor. Ein eigener Media-/Share-LXC sollte Datasets über Bind-Mounts erhalten und bei Bedarf SMB/NFS ausliefern. Eine komplette TrueNAS-VM wurde bei wenigen Laufwerksplätzen als unnötig komplexe erste Variante bewertet. Das ist eine Architekturpräferenz dieses Chats, kein bereits umgebauter NAS.

Illustratives Datasetlayout aus dem Gespräch, **keine existierenden Pfade**:

| Beispiel-Dataset | Gedachter Inhalt |
|---|---|
| `tank/media` | Medien |
| `tank/recordings` | Aufzeichnungen |
| `tank/tts` | fertige Sprachdateien |
| `tank/backups` | lokale Sicherungskopien |
| `tank/proxmox` | Gast-/Hostbezug des Beispielplans |

Die ideale, aber hardwareseitig unbestätigte Laufwerksausstattung war **2 × NVMe im Mirror für Proxmox/VMs/LXCs plus 2 × SATA-SSD im Mirror für Medien**. Bei nur zwei Laufwerken wurde ein gemeinsamer Mirror mit zusätzlicher externer USB-SSD oder Sicherung auf einem anderen NAS vorgeschlagen. Kein Laufwerk, RAID-/ZFS-Pool oder Dataset wurde in diesem Chat angelegt.

Ein Mirror schützt nur gegen bestimmte Laufwerksausfälle. Sicherungen im selben Pool beziehungsweise Host sind kein unabhängig wiederherstellbares Backup bei Hostverlust. Bind-Mounts benötigen ein separates, konkret geprüftes Backup-/Restore-Verfahren; die Erwähnung von „VM-Backups“ belegt nicht, dass sämtliche extern eingebundenen Daten mitgesichert werden. Diese Abhängigkeit bleibt vor einer Migration zu klären.

**Abgleich mit dem heutigen Repository:** Die Media Library ist bereits eine eigene fachliche API mit Verarbeitung, Freigabe und lokalem TBS-Cache. Sie ist nicht automatisch ein NAS-/SMB-/NFS-Server. Die vorhandene NFS-Archivintegration kann auf einem lokalen Dataset statt dem bisherigen Share neu konfiguriert werden, dafür ist jedoch eine ausdrücklich geprüfte Mount-/Pfad-/Rechteanpassung nötig. Keine fertige Sophos-/ZFS-/Samba-Migration wurde gefunden. [R6, R7]

## 7. Netzwerk, Schnittstellen und Dienstabhängigkeiten

### 7.1 Vorgeschlagene logische Netze

| Netz | Historischer Einsatzzweck | Aktueller Festlegungsstand |
|---|---|---|
| Management | Proxmox, Switch, TBS-Verwaltung, ursprünglich USV/PDU | Logische Trennung vorgeschlagen; VLAN-ID, Subnetz und Freigaben offen. |
| TETRA/SwMI | TBS und Backenddienste | In der Sophos-Architektur gesondert vorgeschlagen; Inventory für das mobile Rack fehlt. |
| NetCore Operations / WLAN-Clients | Notebook, Tablets, Zebra, Operator | SSIDs, Clientzugang und Regeln offen. |
| Voice | DECT, PBX, weitere SIP-Telefone | Logisch vorgesehen; Basis-/Nebenstellenkonfiguration fehlt. |
| Guest/Service | temporäre Geräte | Früher genannt, nicht zwingend freigegebener Bestandteil der ersten Stufe. |

Verbindungsvorschläge: AP als VLAN-Trunk am PoE-Switch, DECT in einem Voice-VLAN, TBS im vorgesehenen TETRA-Netz und ein LAN-Trunk vom Switch zur VLAN-aware Proxmox-Bridge. Ob der gewählte AP mehrere SSIDs/VLANs und die gewählte DECT-Basis PoE/VLAN-Tagging unterstützt, ist offen.

### 7.2 Historischer physischer NIC-Vorschlag

| Portbeispiel | Vorgeschlagene Verwendung |
|---|---|
| NIC 1 | direkter Proxmox-Managementzugang |
| NIC 2 | WAN, möglichst exklusiv der Router-VM zugeordnet |
| NIC 3 | LAN-/VLAN-Trunk zum Switch |
| NIC 4 | optional LTE-/5G- oder zweites WAN |
| NIC 5 | optional direkter TBS-Servicezugang |

PCIe-Passthrough des WAN war eine Empfehlung, **keine geprüfte technische Voraussetzung**. NIC-Anzahl, IOMMU-Gruppen und Treiber der konkreten Appliance fehlen. Alternativ-/Brückenbetrieb wurde nicht abschließend spezifiziert.

Der Servicezugang soll Proxmox und möglichst Switch/TBS bei ausgefallener Router-VM weiter erreichbar halten. Vorgeschlagen wurden ein Service-LAN am Panel und statische Notfalladressen. Noch offen ist, ob alle diese Geräte dann tatsächlich im gleichen unabhängig versorgten Layer-2-Segment erreichbar sind; ein vorhandener Port allein erzeugt diesen Rettungsweg nicht.

### 7.3 Abhängigkeiten im konsolidierten Aufbau

- Der physische Proxmox-Host trägt die gemeinsamen Ressourcen aller Core-Gäste.
- Die Router-VM kann WAN/VPN und Inter-VLAN-Kommunikation bereitstellen. Innerhalb eines funktionierenden gleichen Layer-2-Segments ist ein lokaler Dienstzugriff nicht zwingend vom Router abhängig; über VLAN-Grenzen gegebenenfalls sehr wohl.
- Der PoE-Switch versorgt und verbindet WLAN/DECT. Bei seinem Ausfall fehlen auch diese Mastgeräte, sofern keine separate Versorgung geplant wird.
- DECT-Nebenstellen brauchen die konfigurierte und erreichbare PBX. Internet ist für rein lokale Rufe nicht grundsätzlich nötig; PBX-Verfügbarkeit bleibt nötig.
- Der TBS-Fallback benötigt die separat weiterlaufende TBS und ihre lokalen Funktionen/Daten. Er ersetzt weder Stromversorgung noch ein ausgefallenes gemeinsames NAS.
- Eine im selben Sophos-Host liegende PBX teilt dessen Ausfall. Ein direkter SIP-Fallback zur **selben ausgefallenen PBX** bringt keine Telefonie zurück.
- Gaststartreihenfolge und Dienst-Readiness sind getrennte Dinge: ein gestarteter Container muss noch nicht seine Datenbanken, Shares oder APIs betriebsbereit haben.

### 7.4 Heutige Repository-Ports: Beispiele, keine mobile Live-Konfiguration

Die folgenden HTTP-Management-/API-Ports wurden aus `system-backend/services.toml`, README und Beispielkonfigurationen gegengeprüft. Die einzelnen Dienste haben normalerweise eigene Gast-IP-Adressen. Das geplante Rack hat noch keinen zugehörigen IP-/Port-/Firewallplan. [R2, R3]

| Dienst | Port | Aufgabe im heutigen Repository |
|---|---:|---|
| Node Gateway | 8080 | TBS-WebSocket, Backendvermittlung, Core-Health |
| Mobility Core | 8090 | Teilnehmerlage und Mobility-Kontexte |
| Subscriber Core | 8100 | Teilnehmerprofile und Admission |
| Group Core | 8110 | Gruppen und Mitgliedschaften |
| Call Control | 8120 | logische Calls, Floor und Restore |
| Provisioning Core | 8125 | Geräte-/Gruppenmatrix |
| Media Switch | 8130 | Routing codierter Sprachframes |
| Recorder | 8140 | passive Aufzeichnung und Export |
| SDS Router | 8150 | SDS/Status und Store-and-forward |
| Packet Core | 8160 | PDP-/NSAPI- und Paketdatenzustand |
| IP Gateway | 8170 | IP-/TUN-/Routing-Funktion |
| Security Core | 8180 | Sicherheitsrichtlinien und Kontexte |
| KMF | 8190 | Schlüsselmanagementfunktion; keine Schlüssel archiviert |
| Transit | 8200 | regionale Routen und Sessions |
| Observability | 8210 | Logs, Metriken und Diagnose |
| Application Gateway | 8220 | Connectoren und TTS-Orchestrierung |
| Media Library | 8230 | Assets, Vorschau, Freigabe und Playout |
| IoT Gateway | 8240 | MQTT-Ereignisse und Zustände |
| Hardware Gateway | 8250 | Rack-/Umgebungs-I/O |
| RF Monitor | 8260 | zentrale HF-/PA-/Antennenüberwachung |
| Alarm Workflow | 8270 | Alarm-/Eskalationsabläufe |
| Task Workflow | 8280 | strukturierte Aufgaben |
| Asset Management | 8290 | Geräte-/Personen-/Bestandsverwaltung |
| SIP Switch | 8300 | Management-API des zentralen SIP-B2BUA |
| Warnzentrale / alert-service | 8310 | Warnungen; eigener Zugangsmodus |
| Control Room | 9010 | zentrale Bedienoberfläche |

**Katalogabweichung am Prüfstand:** Für den Provisioning Core sind README und Beispielkonfiguration mit Port 8125 vorhanden, aber kein eigener `provisioning-core`-Eintrag in `system-backend/services.toml`. Diese Tabellenzeile stammt deshalb aus der konkreten Dienstkonfiguration; eine automatische Aufnahme in das vorhandene Deploymentinventory ist damit nicht belegt. Vor einer mobilen Gesamtinstallation ist dieser Unterschied zu klären.

Die Beispielkonfigurationen des Packet Core und IP Gateway enthalten teilweise **`shadow`**-Betriebsmodi. Ein vorhandener Dienst oder WebUI-Port beweist deshalb keinen aktiv durchgeschalteten TETRA-IP-Nutzdatenpfad.

Zusätzliche überprüfte Parameter:

| Schnittstelle / Parameter | Gelesener Wert / Pfad | Grenze |
|---|---|---|
| TBS → Node Gateway | WebSocket `/ws/node`, normalerweise Port 8080 | Bei verteilt betriebenem Backend laut Fallbackdokumentation erforderlich; tatsächlich installierte Konfiguration fehlt. |
| TBS-Dashboard | Root-Beispiel: `0.0.0.0:8080` | Gleicher Zahlenport wie Node Gateway, aber auf anderem Knoten/Gast. Zusammenlegung in denselben Namespace würde neu zu planen sein. |
| Root-`config.toml` Control-Room-Verbindung | `enabled = true`, Host `10.0.1.179`, Port 8080, Node-ID `SRV-M-TBS-01` | Repositorybeispiel am Prüfcommit, **keine gelesene mobile Live-TBS-Konfiguration**. |
| Core-Readiness | `/health/ready`, außerdem `/health/live` und `/metrics` | Ein HTTP-Erfolg allein ist keine Funk-/Telefonieabnahme. |
| TBS-Fallbackdiagnose | `GET /api/edge-fallback` | Aktuelle TBS-Ansicht; nur bei real laufender Anlage live prüfbar. |
| Gateway-Core-Matrix | `GET /api/v1/core-services` | Quelle der Serviceverfügbarkeit. |
| Lokaler TBS-Asterisk | SIP-Beispiel `0.0.0.0:5060`; native Bridge zu `127.0.0.1:5060`, nativer lokaler Bind-Port 5062 | Genaue Transport-/Firewallfreigaben am gewählten Deployment zu prüfen. |
| TBS-Asterisk RTP | Beispiel UDP 30200–30300 | Aus lokalem Fallback-TOML; nicht mit zentralem RTP-Bereich verwechseln. |
| Zentraler SIP-Switch/Asterisk | SIP-Beispiel Port 5060; RTP 10000–20000 | Port 8300 ist dessen HTTP-Management, kein SIP-Port. |
| SIP-Switch MQTT | Beispiel Port 1883 | Ereignisanbindung; kein nachgewiesener mobiler Broker. |
| IP-Gateway-Beispiel | DNS `10.0.0.1:53`, Testserver `0.0.0.0:8088` | Beispielwerte; nicht automatisch als Rack-Routing übernehmen. |
| IP Gateway im LXC | `/dev/net/tun`, passende Namespace-/NET_ADMIN-Konfiguration | Im Deployment dokumentiert; im mobilen Host nicht hergestellt. |
| WLAN, DECT-Luftschnittstelle | WLAN nach gewähltem AP; DECT nach gewählter Basis | Keine WLAN-Kanäle, Bandbreiten, Reichweiten oder DECT-Kanalzahlen festgelegt. |
| SMB/NFS | Vorgeschlagene Dateifreigaben | Keine Serverexporte oder konkreten SMB-/NFS-Versionen/Ports im Rack festgelegt. |

Die frühen Antworten beschrieben überwiegend offene Testdienste. Heute enthält das Repository unterschiedliche Zugangsmodi; beispielsweise hat die Warnzentrale standardmäßig Tokenzugang. „Alles offen“ darf nicht pauschal als heutiger Zustand aller Dienste ausgegeben werden. Die im Repository bezeichneten Open-Lab-APIs gehören bei einem mobilen WLAN weiterhin in die bewusst vorgesehenen internen Netze, nicht automatisch in ein frei zugängliches Gastnetz. Zugangsdaten werden hier nicht kopiert.

## 8. Erreichter Stand: historischer Chat versus heutiger Quellstand

### 8.1 Ergebnis dieses Chats

| Gegenstand | Historischer Ergebnisstand |
|---|---|
| Vier-HE-Ziel ohne interne USV | **beschlossen/geplant** für die erste Stufe; keine gebaute Anlage bestätigt. |
| Sophos als gemeinsamer Core | **Idee / geplante Richtung**, abhängig vom konkreten Modell. |
| WLAN/DECT am UHF-Mast | **Idee / Platzierungsrichtung**; Ausleger, Abstände und Geräte offen. |
| Rackplan, VLANs, Storage und Bootreihenfolge | **Vorschläge**, kein vollständiger ausführbarer Deploymentstand. |
| Installation/Hardwarebau | Nicht nachgewiesen. |
| RF-/DECT-/WLAN-/Telefonietest | Nicht durchgeführt beziehungsweise kein Ergebnis zugänglich. |
| Betrieb | Keine zu diesem Rack gehörende Bestätigung. |

### 8.2 Zusätzlich geprüfter heutiger Repository-Stand

Am **05.10.2026** ist `Archiving` auf dem oben genannten Prüfcommit erreichbar. Der Branch wurde frisch geladen und vor dem Schreiben erneut per Fast-forward-only-Pull geprüft. Bereits vorhandene Archivdateien und der Index wurden gelesen. Es war keine eindeutig diesem Chat entsprechende Sophos/4-HE/WLAN/DECT-Abschlussdatei vorhanden; die benachbarten Rackarchive bleiben unverändert.

`main` wurde nur lesend zusätzlich geladen. Der direkte Baumvergleich von `main` und dem geprüften `Archiving`-Stand zeigt außerhalb des Archivs lediglich die einzeilige, inhaltlich leere `Docs/Control_Room/Readme.md` zusätzlich in `Archiving`. Die geprüften **Programmquellen sind damit in beiden Snapshots identisch**. Unterschiedliche Commit-Historien sind nicht als unterschiedlich implementierte Rackfunktion ausgelegt worden. Es wird kein Branch zusammengeführt.

| Aussage aus der Planung | Heute prüfbarer Befund | Bewertung |
|---|---|---|
| SwMI-Dienste können auf dem Core zusammengefasst werden | Eigenständige Backenddienste, Konfigurationen, systemd-Units und inventory-basierter LXC-Deployer vorhanden | **Implementiert** als allgemeine Backendbasis. Kein Nachweis einer Sophos-Installation oder einer passenden Ressourcenverteilung. |
| TBS behält lokalen Fallback | Konfigurationsmodul, Workerzustände, Policycache, Eventspool, zentrale Verfügbarkeitsprüfung und Dashboarddiagnose vorhanden | **Implementiert**; konkrete Rack-/Funkabnahme ausstehend. [R4, R5] |
| Lokale Gespräche sollen trotz Coreverlust möglich bleiben | Lokale Air-Interface-/SDS-Pfade und serviceabhängige Rückfallentscheidung vorhanden; Dokumentation benennt lokale Weiterarbeit | Code und dokumentierter Sollbetrieb vorhanden; keine Garantie für jeden bereits laufenden Ruf oder alle Dienste. |
| Telefonie kann per SIP gekoppelt werden | Zentraler SIP Switch und lokaler TBS-Asterisk-Fallback nach Phase 11c vorhanden | **Implementiert** als Routing-/Failoverbasis; reale DECT-/PBX-/RF-Ende-zu-Ende-Funktion dieses Racks ungeprüft. [R8–R10] |
| NAS/Media Library auf demselben Host | Media Library, Recorder und NFS-Archivpfade vorhanden | Fachliche Medienintegration **implementiert**; allgemeiner NAS-/ZFS-/SMB-Aufbau und mobile Migration nicht nachgewiesen. |
| Alle Funktionen passen in vier HE | Kein zugehöriges Sophos-Geräteprofil, vollständiger maßhaltiger Rackplan oder mechanischer Test gefunden | **Unbestätigt**, nicht aus Code ableitbar. |
| WLAN/DECT funktionieren am selben Mast störungsfrei | Keine passende Koexistenzmessung oder Gerätekonfiguration dieses Rackentwurfs vorliegend | **Unbestätigt**. |

### 8.3 Konkreter Edge-Fallback heute

Der heutige Code unterscheidet **Online, Degraded, Isolated, Recovering**. Maßgeblich sind die Node-Gateway-Verbindung und die Core-Service-Matrix, nicht die Erreichbarkeit eines öffentlichen Internetservers. Unbekannte Dienste werden in der Standardkonfiguration nicht als verfügbar angenommen. [R4, R5]

| Parameter aus Code/Root-Beispiel | Wert am Prüfstand |
|---|---|
| `edge_fallback.enabled` | `true` |
| `enter_after_secs` | 15 s |
| `recover_after_secs` | 20 s |
| `service_matrix_lease_secs` | 60 s |
| Standardkritische Dienste | subscriber-core, group-core, mobility-core, call-control, media-switch, sds-router |
| Policycache | `/var/lib/flowstation/edge-policy-cache.json` |
| Spool | `/var/lib/flowstation/edge-event-spool.jsonl` |
| Policycache-Alter für Diagnose | 7 Tage |
| `keep_last_known_policy` | `true` |
| Spoolbegrenzung | 10.000 Einträge, 16 MiB |
| Replay-Batchgröße | 128 |

Diese Werte sind keine in diesem Chat gemessenen Umschaltzeiten. Die Bezeichnung `flowstation` in den existierenden Zustandspfaden ist ein aktueller Repositorypfad und wird im Rahmen der Archivierung nicht umbenannt.

Der Spool betrifft geeignete SDS-/Steuer-/Telemetrieereignisse. **Zeitkritische Sprachframes werden nicht nachträglich aus diesem Spool ausgesendet.** Lokale Aufzeichnung und ein späterer Medienimport sind getrennte Funktionen. Ohne lokalen Recorder beziehungsweise lokalen Cache bleiben sie bei Ausfall des Core-Storage nicht automatisch vollständig verfügbar.

### 8.4 SIP-Fallback heute und seine Grenze im Ein-Host-Konzept

Phase 11b ist im heutigen Repository ausdrücklich durch **Phase 11c** ersetzt. Im Normalbetrieb führt der lokale TBS-Asterisk die einzige absichtlich aktive externe Registrierung zum zentralen SIP-Switch. Beim bestätigten zentralen Ausfall wird auf direkte PBX-Registrierung umgestellt; bei stabiler Erholung wieder zurück. Die native TBS-Bridge bleibt lokal angebunden. [R8–R10]

Beispielparameter: Prüfung alle **2 s**, Startgrace **10 s**, **3** Fehler für Umschaltung, stabile Erholung **30 s**, kurze Registrierungsexpiration **30 s**. Zustände: `CENTRAL_ACTIVE`, `FAILOVER_PENDING`, `PBX_DIRECT_ACTIVE`, `RECOVERY_PENDING`.

Eine harte Zentralstörung kann alte Kontakte bis zum Ablauf der Registrierung sichtbar lassen. Das ist nicht dasselbe wie zwei bewusst aktive TBS-Registrierungen. Laufende SIP-Dialoge werden laut Dokumentation nicht mitten im Gespräch auf den Ersatzweg verschoben; der neue Weg betrifft neue Rufe.

**Für das hier geplante Rack entscheidend:** Läuft die PBX ebenfalls auf der ausgefallenen Sophos, ist der direkte PBX-Weg nicht verfügbar. Dieser Mechanismus hilft bei einem einzelnen SIP-Switch-Ausfall oder bei einer unabhängig erreichbaren PBX. Er ist keine Hostredundanz und keine Ersatz-PBX.

### 8.5 Ausfallmatrix für die Fortsetzung

| Störung | Realistisch verbleibende Funktionen / Abhängigkeiten | Status |
|---|---|---|
| Externes Internet/WAN fällt aus | Lokales Ethernet/WLAN, interne PBX und lokale TETRA können bleiben, sofern lokale Netze/Dienste weiterlaufen | Architekturziel; Racktest offen. |
| Router-VM fällt aus | Gleiche Layer-2-Segmente und direkter Managementzugang können bleiben; VLAN-übergreifende und WAN-Pfade können fehlen | Konfiguration entscheidend, nicht automatisch garantiert. |
| Einzelner SwMI-Dienst fällt aus | Servicebezogener TBS-Fallback statt pauschalem Abschalten aller lokalen Funktionen | Codebasis vorhanden; Test offen. |
| Gesamter Sophos-/Proxmox-Host fällt aus | Router, PBX, NAS/Media und zentrale SwMI gleichzeitig weg; lokale TBS nur bei eigener weiter funktionierender Versorgung und lokalen Daten | Wichtigster gemeinsamer Ausfallpunkt. |
| PoE-Switch fällt aus | WLAN/DECT und die an ihm geführten Netzwege fehlen; TBS-HF kann bei unabhängiger Versorgung lokal weiterlaufen | Physischer Aufbau offen. |
| Netzversorgung des Racks fällt aus | **Ohne USV gehen auch TBS, Switch und Core aus.** Der TBS-Fallback hilft nicht gegen fehlende Energie. | Direkte Folge des beschlossen reduzierten Umfangs. |
| Ein Laufwerk fällt aus | Ein korrekt aufgebauter Mirror kann den Betrieb erhalten; hier wurde keiner gebaut oder getestet | Storageplan unbestätigt. |

## 9. Relevante Dateien, Pfade, Befehle und Abläufe

### 9.1 Dateien und Betriebsdaten

| Quelle / Betriebspfad | Bedeutung |
|---|---|
| `system-backend/services.toml` | Tatsächlicher Dienstkatalog; nicht `services.toml` im Repositoryroot. |
| `deploy/open-lab/inventory.example.toml` | Vorhandenes Beispielinventory, noch nicht mobile Sophos-/Rackkonfiguration. |
| `deploy/open-lab/netcore-deploy.py` | Validate/Plan/Render/Apply/Status und Testprofile. |
| `deploy/open-lab/lxc/ip-gateway.conf.example` | Beispiel für TUN-/LXC-Sonderrechte. |
| `config.toml` | Gelesene Root-TBS-Beispielkonfiguration; enthält sensible Felder, daher hier nicht vollständig kopiert. |
| `/etc/netcore/config.toml` | In Installationsdokumentation verwendeter TBS-Konfigurationspfad; reale Unit kann anders konfiguriert sein. |
| `/etc/netcore/<dienst>.toml` | Beispielhafte installierte Dienstkonfigurationen, unter anderem recorder/media-library/sip-switch. |
| `/var/lib/netcore-recorder/recordings/YYYY/MM/DD/<recording-id>/` | Recorderarchiv mit `audio.tacelp`, `frames.jsonl`, Metadaten und Integritätsdatei. |
| `/var/lib/netcore-media-library/assets` | Standardarbeits-/Assetverzeichnis der Media Library. |
| `/mnt/nfs-share/Media-Library` | Heutiger konfigurierter Medienarchiv-Beispielpfad. |
| `/mnt/nfs-share/Recordings` | Heutiger konfigurierter Aufzeichnungsarchiv-Beispielpfad. |
| `/mnt/nfs-share/TTS-Dateien` | Heutiger konfigurierter TTS-Archiv-Beispielpfad. |
| `/etc/netcore/tbs-sip-fallback.toml` | Lokaler SIP-Fallback; Inhalte können Zugangsdaten enthalten. |
| `/etc/asterisk/netcore-active-registration.conf` | Einzelner aktiver Include für zentrale oder direkte PBX-Registrierung. |
| `/etc/asterisk/netcore-registration-central.conf` | Primärer Registrierungsweg. |
| `/etc/asterisk/netcore-registration-pbx-direct.conf` | Direkter PBX-Ersatzweg. |
| `/var/lib/netcore-tbs-sip-fallback/state.json` | Zustand der lokalen SIP-Umschaltung. |
| `netcore-tbs-sip-failover.service` | Lokale Umschaltüberwachung. |
| `netcore-piper.service` | Heutige zentrale TTS-Komponente im Media-Library-Umfeld. |

### 9.2 Tatsächlich im historischen Rackchat ausgeführte Befehle

**Keine nachweisbar.** Es wurde kein `qm`, `pct`, Proxmox-Installer, ZFS-Befehl, VLAN-/Switchbefehl oder NetCore-Deploylauf aus diesem Rackgespräch erfolgreich ausgeführt. Die damaligen Antworten enthielten überwiegend Skizzen, Tabellen und Aufbauempfehlungen.

### 9.3 Vorhandene Repositoryabläufe für die spätere Umsetzung

Die folgenden Befehle sind heute im Repository dokumentiert beziehungsweise ausgelesen, in diesem Auftrag jedoch **nicht auf einer Anlage ausgeführt**. Sie sind kein fertiges Installationsskript für die unbekannte Sophos. Inventory, Mounts und Konfigurationen sind zuerst an den mobilen Aufbau anzupassen.

```bash
# Repositorydokumentierte Planung/Validierung, kein Live-Deployment in diesem Chat:
python3 deploy/open-lab/netcore-deploy.py \
  --inventory deploy/open-lab/inventory.example.toml validate
python3 deploy/open-lab/netcore-deploy.py \
  --inventory deploy/open-lab/inventory.example.toml plan
python3 deploy/open-lab/netcore-deploy.py \
  --inventory deploy/open-lab/inventory.example.toml render
python3 deploy/open-lab/netcore-deploy.py \
  --inventory deploy/open-lab/inventory.example.toml apply --dry-run
```

Die Dokumentation nennt außerdem `apply`, `status` sowie Smoke/Full/Fault-Profile. Full- und Fault-Läufe können Fachdaten verändern beziehungsweise Dienste stoppen; sie sind **nur für die spätere bewusst eingerichtete Testanlage**, nicht im Rahmen dieses Archivierauftrags erfolgt. Ein erfolgreiches statisches Validate wäre kein Beleg für einen realen LXC-, SIP- oder RF-Test.

Lesende Diagnosebeispiele für eine spätere laufende Anlage:

```bash
# Auf der betreffenden TBS, sobald installiert:
systemctl status netcore-tbs-sip-failover.service --no-pager
journalctl -u netcore-tbs-sip-failover.service --no-pager -n 100
asterisk -rx 'pjsip show registrations'

# Auf dem betreffenden Recorder-LXC, sobald installiert:
curl http://127.0.0.1:8140/health/live
curl http://127.0.0.1:8140/health/ready
```

Vorhandene Installations-/Reparaturpfade:

- Zentraler SIP-Switch: `system-backend/sip-switch/install/install.sh` und `update.sh`.
- Lokaler SIP-Fallback: `system-backend/sip-switch/tbs-fallback/install/install-tbs-local-fallback.sh` beziehungsweise die dokumentierten Wrapper in `system-backend/sip-switch/install/`; Konfiguration für die jeweilige TBS vorbereiten, anschließend native Bridge lokal anbinden.
- Lokale TBS-Anbindung: `system-backend/sip-switch/install/apply-tbs-local-asterisk-config.sh --config /etc/netcore/config.toml`.
- Medienrechte: `system-backend/media-library/install/repair-permissions.sh` existiert für den dokumentierten Archiv-Migrationsfehler; **kein in diesem Rackchat aufgetretener oder hier behobener Fehler**.

Die Installerparameter mit SIP-/PBX-Zugangsdaten werden bewusst nicht in diese Abschlussdatei übernommen. Auch historische Demo-Passwörter werden nicht kopiert.

### 9.4 Für die Archivierung tatsächlich ausgeführte Prüfungen

- GitHub-Repository-/Branchreferenzen und Commitmetadaten gelesen.
- `Archiving` frisch geklont; `main` zusätzlich nur lesend gefetcht.
- `git status`, `git rev-parse` und Baum-/Dateivergleiche ausgeführt.
- Vor dem Schreiben `git pull --ff-only origin Archiving` ausgeführt; Ergebnis: bereits aktuell auf dem genannten Prüfcommit.
- PDF-Metadaten mit `pdfinfo` geprüft; Text mit `pdftotext -layout` extrahiert; SHA-256-Identitäten berechnet.
- TOML-Beispielkonfigurationen mit Python `tomllib` gelesen und ausgewählte nicht geheime Ports/Pfade abgeglichen.

Die endgültigen Dokument-/Index-/Scope- und Remote-Speicherprüfungen gehören zur Archivierung selbst. Sie ändern den Status der Hardware-/Funkplanung nicht in „getestet“ oder „im Betrieb bestätigt“.

## 10. Fehler, Korrekturen, Tests und überholte Ansätze

### 10.1 Tatsächliche und vermutete Probleme unterscheiden

Es gibt **keinen im Rackchat berichteten Installationsabbruch, Hardwaredefekt oder Betriebsfehler**. Gewicht, thermische Engpässe, mangelnder Steckraum, störende HF-Kopplung, ein gemeinsamer Hostausfall und ein Stromausfall ohne USV wurden vorausblickend diskutiert. Sie sind offene Design-/Abnahmerisiken, keine bereits beobachteten Defekte.

| Frühere Aussage / Ansatz | Abschlussbewertung und Korrektur |
|---|---|
| „550 cm“ Einbautiefe | Als 550 mm interpretiert; explizite Bestätigung der Einheit ist nicht sichtbar. |
| „6 HE sind definitiv gut machbar“ / später „4 HE reichen“ | Nur rechnerische HE-Belegung. Ohne Modelle, Tiefe, Befestigung und Kabelraum keine belastbare Passfreigabe. |
| Gemeinsamer Mast sei grundsätzlich die sauberste Lösung | Als Empfehlung mit fehlender HF-/Mechanikprüfung herabgestuft. |
| Feste Mindestabstände am Mast | Im Chat widersprüchliche Heuristiken; keine normative oder messtechnische Freigabe. |
| CPU/RAM-Tabelle als „Minimum“ | Keine Benchmarkbasis; nur damalige Assistenzzielgrößen. |
| TBS-Fallback garantiere Weiterbetrieb nach beliebigem Coreausfall | Lokale Funkfunktion ist bedingt möglich; eine mitausgefallene PBX, fehlender Strom oder ausschließlich zentral vorhandene Medien werden nicht ersetzt. |
| Ohne USV sei der Ausfall für die TBS „nicht dramatisch“ | Keine Betriebserfahrung dieses Racks. Auch TBS-Speicher, Dienste und Aufnahmen benötigen geprüfte Recovery; fehlender Strom stoppt den Funk. |
| SSD/Mirror/Dateisystem verhindere sämtliche Stromausfallschäden | Keine solche Garantie. Sicherer Schreibcache, Datenbankkonsistenz und Restore müssen separat geprüft werden. |
| Deckellicht bei abgenommenen Deckeln | Befestigung und Versorgung offen; Rahmenmontage als ebenfalls genannte Option. |
| AP in einer 1-HE-Schublade | In vier HE keine freie Frontposition; Transportlagerung offen. |

### 10.2 Ersetzte oder nach hinten verschobene Varianten

- **6 HE mit separatem NAS und Router/Switch in gemeinsamer HE:** durch den gemeinsamen Sophos-/Proxmox-Core ersetzt.
- **6 HE mit separater Panel-HE und interner USV:** durch den jüngsten vier-HE-Entwurf ohne eigene Panel-HE und ohne USV ersetzt.
- **Interne USV sofort:** aus Gewichtsgründen vom Nutzer vorläufig gestrichen. Spätere Nachrüstung bleibt möglich.
- **WLAN/DECT tief im Metallrack:** vom Assistenten zugunsten absetzbarer Mastgeräte nicht empfohlen; kein tatsächlich vorgenommener Einbau.
- **Zwei getrennte Funkmasten als Erstvorschlag:** durch die anschließende Nutzerfrage zum gemeinsamen UHF-Mast als jüngste Variante abgelöst, nicht technisch verboten.
- **TrueNAS-VM als erste Storagevariante:** zugunsten Host-ZFS plus Share-LXC vom Assistenten zurückgestellt; keine ausdrückliche Nutzerwahl eines Dateisystems.
- **Phase 11b SIP-Fallback:** im heutigen Repository historisch ersetzt durch Phase 11c. Dies ist ein heutiger Code-/Dokubefund, kein damaliger Rackumbau.

### 10.3 Testnachweise und Grenzen

| Prüfung | Ergebnis / Status | Grenze |
|---|---|---|
| PDF-Dateien lesbar | 25 Dateien, 8.061 Seiten einschließlich Sammeldopplungen inventarisiert | Kein vollständiger Review sämtlicher Normanforderungen. |
| Repository erreichbar | Zielbranch und exakte SHAs gelesen, frischer Checkout erfolgreich | Kein Zugriff auf Sophos, Pi, Switch oder PBX. |
| `main`/`Archiving` Programmquellen | In den geprüften Snapshots identisch; zusätzlicher leerer Readme-Pfad in `Archiving` | Aussage nur für diese beiden Commitstände. |
| Beispielports/-pfade | Aus vorhandenen Dateien ausgelesen; Dienstkatalog im richtigen Unterordner gefunden | Beispielkonfigurationen sind keine Betriebsnachweise. |
| Edge-/SIP-Fallback | Passende Code-/Konfigurations-/Dokumentationsbasis vorhanden | Kein Rust-Build, kein realer Failoverlauf in diesem Auftrag. |
| Proxmox auf der Sophos | **Nicht getestet** | Modell und Hardware fehlen. |
| DECT/PBX/TETRA-Ende-zu-Ende | **Nicht getestet** | Keine Mobilteile, Nebenstellen, SIP-/RTP- oder Funkmesslogs dieses Racks. |
| WLAN-/DECT-/UHF-Koexistenz | **Nicht getestet** | Keine Messwerte oder konstruktive Freigabe. |
| Casepassung, Transport, Kühlung, Leistungsbudget | **Nicht getestet** | Keine tatsächliche Stückliste, Masse oder Lastmessung. |
| Stromausfall-, Wiederanlauf- und Backup/Restore-Test | **Nicht getestet** | Keine Anlage im Chat aufgebaut. |

Vorhandene Regressionstests und die README-Aussage „deploybar“ werden nicht als in diesem Auftrag ausgeführte Tests übernommen. Ein Netz-/Hardwaretest darf später nur mit eigenem Datum, Aufbau, Softwarestand und Ergebnis ergänzt werden.

## 11. Offene Aufgaben, Nebenideen und nächste Schritte

Es wurde keine nummerierte Umsetzungsroadmap ausdrücklich verabschiedet. Die folgende Reihenfolge ist eine **für die Fortsetzung aus den Abhängigkeiten abgeleitete Empfehlung**. Die einzigen klaren Prioritätsentscheidungen des Nutzers sind: kompakte erste Stufe, USV zunächst weglassen, späteres größeres Case akzeptabel.

| Kandidat | Empfohlene Reihenfolge | Konkrete Aufgabe | Voraussetzung / Abnahmekriterium |
|---|---|---|---|
| MRACK-01 | P0: vor Beschaffung | Exaktes Sophos-Modell und Ist-Hardware aufnehmen | CPU/Virtualisierung, BIOS, RAM, Laufwerksplätze, NICs, IOMMU, Maße und Leistung belegt. |
| MRACK-02 | P0 | Vier-HE-Passplan mit realen Geräten erstellen | TBS-Höhe, 550-mm-Einheit/-Nutzraum, Stecker/Biegeradien, PDU, Lüfter, Deckel und Befestigung passen gemeinsam. |
| MRACK-03 | P0 | Gewicht, Strom- und Wärmelast erfassen | Tatsächliche Stückliste und Volllast inklusive PoE; Case/Griffe/Transport zulässig. |
| MRACK-04 | P1: Core-Grundbetrieb | Proxmox-Boot-/NIC-/Storage-Test auf der gewählten Appliance | Installationsstand, Gastfähigkeit und erreichbarer unabhängiger Managementport dokumentiert. |
| MRACK-05 | P1 | Netz-/VLAN-/IP-/Serviceplan für mobilen Knoten | Routerausfall lässt Servicezugang erhalten; keine Namespace-/Portkollision; nur notwendige Gastnetzfreigaben. |
| MRACK-06 | P1 | Mobile SwMI-Dienstmenge und Ressourcen festlegen | Vorhandene Komponenten verwenden; Inventory und Abhängigkeiten gegen lokalen Host planen. |
| MRACK-07 | P1 | Storage/Media-/Share-Konzept konkretisieren | Mounts, UID/GID, NFS-/SMB-Rollen, Archiv-/Live-State-Trennung und Backupumfang festgelegt. |
| MRACK-08 | P1 | Router, PBX und Backend kontrolliert starten/recovern lassen | Hoststart, Gastautostart, Dienst-Readiness und Recovery nach Unterbrechung nachgewiesen. |
| MRACK-09 | P1 | Lokalen TBS-Edge-Fallback und SIP-Phase-11c prüfen | Einzelner Dienstausfall, Core-/Routerausfall und Wiederkehr mit Logs; PBX-Hostverlust als echte Grenze dokumentieren. |
| MRACK-10 | P2: Mastgeräte | AP/DECT/Mobilteile wählen und Nebenstellen konfigurieren | Wetter-/Montage-/PoE-Budget, Kanal-/Kapazitätsdaten, SIP/RTP/Codec/DTMF geprüft. |
| MRACK-11 | P2 | Gemeinsamen Mastarm konstruieren und HF-Verträglichkeit abnehmen | Reale Antennen-/Geräteabstände, Kabel-/Schutzführung und reproduzierbarer RX-Vergleich unter Last. |
| MRACK-12 | P2 | Abschließenden Gesamtsystem-/Transporttest durchführen | Mechanik, thermischer Dauerlauf, WLAN/DECT/TETRA, lokale Autarkie und sichere Wiederherstellung dokumentiert. |
| MRACK-13 | Später | Externe USV beziehungsweise separates Powercase | Tatsächliches Last-/Laufzeitbudget und sicherer Shutdown zuerst ermitteln; interne HE-Belegung nicht zwingend ändern. |
| MRACK-14 | Später | Größeres 6-/8-HE-Case erwägen | Erst bei realem Platz-/Leistungsbedarf; Übernahme vorhandener Baugruppen geplant. |

Weitere weiterhin erwähnenswerte Ideen:

- ausklappbarer Arbeitsplatz am Rack, ohne ausgearbeiteten Mechanikplan;
- LTE-/5G-WAN und Mastantenne als optionale spätere Uplinkvariante;
- Service-/Wartungsnetz und temporärer Clientzugang;
- Statusanzeige, HDMI/USB am Anschlussfeld, robuste WAN-/LAN-/PoE-Verbindungen;
- frühe Panelidee mit `POWER IN`, WAN/LAN, zwei Mast-PoE, RX/TX, USB/HDMI, Master/Not-Aus und Statusanzeige; genaue Funktionen und sichere elektrische Ausführung ungeplant;
- lokale Rufnummer für Störungsannahme, Technik-/Leitungsnebenstellen und Bereitschaftstelefone;
- später kontrollierte TETRA-/SIP-/DECT-Einzelrufkopplung mit realer Audioabnahme;
- transportierbare externe USV beziehungsweise ein separates **2-HE-Powercase** als modulare Alternative;
- Backup auf abziehbarer USB-SSD beziehungsweise anderes NAS und bewusst geprüfter Restore;
- lokale Logs/Monitoring, TTS und Aufzeichnungszugriff auch bei externem WAN-Ausfall;
- späterer Umzug in ein größeres Case, wenn mehr Storage, zweite Serverinstanz oder zusätzliche HF-Technik nötig werden.

Diese Kandidaten werden **nur in diesem Archiv erfasst**. Es werden keine zentrale Roadmap, Produktkonfiguration, Dienste oder sonstige Dateien außerhalb `Docs/archive/` geändert.

## 12. Anhänge und Chatbilder

### 12.1 Tatsächliche Bildlage

Im direkt zugänglichen Rackverlauf existieren **textuelle Rack-/Mastskizzen**, aber keine mitgelieferten eigenständigen PNG-, JPEG-, WebP- oder SVG-Originalbilder dieses Chats. Auch die lokal für diesen Auftrag bereitgestellten Anhänge sind ausschließlich PDFs. Die Skizzeninhalte sind in den Rack-, Mast- und Schnittstellentabellen dieses Dokuments erhalten.

Es kann deshalb kein tatsächlich vorhandenes Original-Rackfoto oder Chatbild zusätzlich hochgeladen werden. Eventuell im nicht zugänglichen Altverlauf enthaltene Bilder bleiben eine ausdrücklich benannte Quellenlücke. Bilder aus anderen Chats oder neu erzeugte Norm-/Konzeptgrafiken werden nicht als wiedergefundene Originalbilder ausgegeben. In PDFs eingebettete Normabbildungen sind Quellenmaterial, keine Fotos des hier geplanten Racks.

Die 25 Norm-PDFs werden durch Identität, Version, Seitenzahl und Prüfsumme dokumentiert; sie werden in diesem Auftrag nicht als zusätzliche Normenbibliothek in das Repository kopiert.

### 12.2 PDF-Inventar

| ID | Originaldatei | Dokument / Version | Seiten | Einordnung für diesen Abschluss |
|---|---|---|---:|---|
| A1 | `en_3003920308v010401p.pdf` | EN 300 392-3-8 V1.4.1 (2020-04), ISI Generic Speech Format | 22 | Spätere Interworkingreferenz; kein Racktest. |
| A2 | `en_30039209v010701p.pdf` | EN 300 392-9 V1.7.1 (2020-04), Supplementary Services: General Requirements | 46 | TETRA-Funktionsreferenz. |
| A3 | `ts_10081201v020205p.pdf` | TS 100 812-1 V2.2.5 (2003-10), SIM-ME/UICC | 8 | Keine SIM-Integration im Rackchat beschlossen. |
| A4 | `en_3003921201v010202p.pdf` | EN 300 392-12-1 V1.2.2 (2007-08), Call Identification Stage 3 | 56 | Hintergrund für Telefonieidentitäten. |
| A5 | `en_3003920304v010301p.pdf` | EN 300 392-3-4 V1.3.1 (2010-08), ISI SDS | 28 | Kein ISI-Rackdeployment nachgewiesen. |
| A6 | `en_3003921117v010102p.pdf` | EN 300 392-11-17 V1.1.2 (2002-01), Include Call Stage 2 | 18 | Zusatzdienst, kein Auftrag dieses Chats. |
| A7 | `en_3003921114v010101p.pdf` | EN 300 392-11-14 V1.1.1 (2002-07), Late Entry Stage 2 | 23 | Zusatzdienst-/Recoveryhintergrund. |
| A8 | `es_20081202v020401m.pdf` | **Final draft** ES 200 812-2 V2.4.1 (2005-08), TSIM Application | 139 | Titelblattstatus beachtet; keine TSIM-Umsetzung beschlossen. |
| A9 | `es_20081201v020205p.pdf` | ES 200 812-1 V2.2.5 (2003-12), TSIM-ME/UICC | 8 | Kartenreferenz ohne konkrete Rackaufgabe. |
| A10 | `en_300812v020101p.pdf` | EN 300 812 V2.1.1 (2001-12), Security/SIM-ME | 156 | Kein Sicherheitskonformitätsnachweis. |
| A11 | `en_3003921101v010201p.pdf` | EN 300 392-11-1 V1.2.1 (2004-01), Call Identification Stage 2 | 44 | Telefonie-/Identitätshintergrund. |
| A12 | `en_3003921006v010401p.pdf` | EN 300 392-10-6 V1.4.1 (2006-08), Call Authorized by Dispatcher | 20 | Kein CAD-Auftrag beschlossen. |
| A13 | `en_3003921018v010301p.pdf` | EN 300 392-10-18 V1.3.1 (2003-10), Barring of Outgoing Calls | 17 | Zusatzdienstreferenz. |
| A14 | `en_3003921216v010400a.pdf` | **Draft** EN 300 392-12-16 V1.4.0 (2026-03), Pre-emptive Priority Call | 67 | Entwurf, nicht als abgeschlossene aktuelle Norm ausgegeben. |
| A15 | `en_30039201v010601p.pdf` | EN 300 392-1 V1.6.1 (2020-04), General Network Design | 182 | Netz-/SwMI-Begriffe selektiv geprüft, insbesondere S. 24. |
| A16 | `ets_30039214e01v.pdf` | **Final draft pr ETS 300 392-14** (1997-09), PICS | 61 | Historisches Proforma, kein ausgefülltes Konformitätsstatement des Projekts. |
| A17 | `en_30039207v030501p.pdf` | EN 300 392-7 V3.5.1 (2019-07), Security | 216 | Hintergrund; offene Testdienste sind keine Security-Konformität. |
| A18 | `en_30039401v030301p.pdf` | EN 300 394-1 V3.3.1 (2015-04), Radio Conformance Testing | 169 | Blocking/Intermodulation, §§ 7.1.8/7.2.5 selektiv gelesen; keine Abnahme. |
| A19 | `en_3003920313v010201p.pdf` | EN 300 392-3-13 V1.2.1 (2020-04), Transport-independent ISI Group Call | 191 | Zukunfts-/Interworkingreferenz. |
| A20 | `en_30039502v010303p.pdf` | EN 300 395-2 V1.3.3 (2025-02), TETRA Codec | 94 | Codecreferenz; keine realen DECT-/TETRA-Audiotests. |
| A21 | `en_3003920303v010301p.pdf` | EN 300 392-3-3 V1.3.1 (2011-11), ISI Group Call | 251 | Keine Multi-SwMI-Rackimplementierung bestätigt. |
| A22 | `en_30039205v020701p.pdf` | EN 300 392-5 V2.7.1 (2020-04), PEI | 320 | Kein neuer PEI-Anschluss beschlossen. |
| A23 | `en_3003920315v010500a.pdf` | **Draft** EN 300 392-3-15 V1.5.0 (2026-04), ISI Mobility Management | 380 | Entwurfstatus; kein Handovertest dieses Racks. |
| A24 | `en_30039202v030801p.pdf` | EN 300 392-2 V3.8.1 (2016-08), Air Interface | 1445 | Grundreferenz, keine vollständige AI-Konformitätsprüfung. |
| A25 | `ETSI.pdf` | Zusammengesetzte Normensammlung | 4100 | Beginnt mit EN 300 812; enthält u. a. zusätzlich TS 100 812-2 V2.4.1 ab Sammelseite 312. Keine neue eigenständige Norm. |

Die Einzeldokumente A1–A24 umfassen zusammen **3.961 Seiten**. Die zusätzliche Sammeldatei umfasst **4.100 Seiten**; ihre wiederholten Inhalte werden nicht als zusätzliche unabhängige Nachweise gezählt. Die bloße Vorlage einer Norm legt keine Implementierung oder vollständige Unterstützung aller darin genannten Dienste fest.

### 12.3 Dateiprüfsummen

| Datei | SHA-256 |
|---|---|
| `en_3003920308v010401p.pdf` | `4d993a25e35da7b1f2291ae281cb3ea5b3a7b702ab7eb3233bdf430a2c70909d` |
| `en_30039209v010701p.pdf` | `cad45938bcdf2fa4e8063d4aa9e7d7c06c45f729d3c55dc033e00c27af9f2e06` |
| `ts_10081201v020205p.pdf` | `96df75f5ddf1c3ae4ac75f796ee97babca68fa81aca22f39ba0e0658bd57eed1` |
| `en_3003921201v010202p.pdf` | `4d5b56a188fb73c417d67b27da619cad2e49efb3b57306fd4ae51e140c018018` |
| `en_3003920304v010301p.pdf` | `8a38cc6238c6da726e4f07d7d377b2c827447a1448a60ee5ef2a126cb3a0ef9d` |
| `en_3003921117v010102p.pdf` | `69ce800e352b1ecfc337416fd7b54c1bdf2d9270171af519ac2f5acc1ba85ca6` |
| `en_3003921114v010101p.pdf` | `ba1882df71dbb84a0c4f0f7dcea1dc8df968044457da03df9290a50e8821df32` |
| `es_20081202v020401m.pdf` | `330f045a908eefd249fd7450e5b71bd08e8a7b20ce5b86dc9b87c9138a5c7268` |
| `es_20081201v020205p.pdf` | `346dc545049a7397ca86831731bbc05e96f61ff047d6e7f6b7de56da3aa408e9` |
| `en_300812v020101p.pdf` | `196effeaae473a98244c89538e1562daa58bb1d74cbfe7de78e58ceeafbad18b` |
| `en_3003921101v010201p.pdf` | `852c17ea566757e80ecf0d2718ae7839d9e218aa4384b63d1277ed7e25d86c69` |
| `en_3003921006v010401p.pdf` | `32b6ee43602ef88a0b5c209b1c69ad35ba8b2e660502257c2dfaa6205a346523` |
| `en_3003921018v010301p.pdf` | `4cc10bcd94d39201538639cefb529809729563f13f87cfe6ae52b44c253b0b8a` |
| `en_3003921216v010400a.pdf` | `c0ee7859f448c4072befc25905c29b751b5151d3590a880a2d654309a3ba6c02` |
| `en_30039201v010601p.pdf` | `788722e566098957bea1a9a5096e9dae1eace1da9f5d38c530c80230e5b9b9cb` |
| `ets_30039214e01v.pdf` | `2b703f3ebb883c90dab40e1b3d1a634a6acf9c9a20d468fd108ea02226c2bf2c` |
| `en_30039207v030501p.pdf` | `df47af9a642b6eba7d7d5cf5fb7aa3f961e9e03bb912316bd2d189ce47344e08` |
| `en_30039401v030301p.pdf` | `2d9c327e5ccf1476335e1b7a4f58a0ca7afc93911c30e8aad5ed7dc9ccf3276a` |
| `en_3003920313v010201p.pdf` | `b47d63853f83a6a4adda08c0e325c8ca13d358e2f8840bfba4bce430e600b1bd` |
| `en_30039502v010303p.pdf` | `ac716ca18082cc0fd2ee768a14f03deb48fa78a77e83751b1a6b319c7fc2106a` |
| `en_3003920303v010301p.pdf` | `94ca61038b3ff826e6adcfde56e00473dd996e1a47f9f84e7b9d2aa6c3133fd2` |
| `en_30039205v020701p.pdf` | `10aaf78988ae4080c3dfe90165ee7f4f3fe8eda402e09fc1e6fa0a0ad890758d` |
| `en_3003920315v010500a.pdf` | `e959709667192e22d2dfeea51b48f9a579a4c107d952999056b7fad93a6d2100` |
| `en_30039202v030801p.pdf` | `3f07b1e4ad73fabc16277a900006fc19844b3a882bbc2bc1d74d78f6156daf28` |
| `ETSI.pdf` | `9434dad1e7bc80ca39b0edadd8e3b9995fda5ae5f5c3dd565af05708d5059e38` |

## 13. Relevante Quellen und Repository-Nachweise

Repositorylinks beziehen sich soweit angegeben auf den geprüften Archiv-Snapshot `b4736a4d37fe2044fff998d017a36c1c950a8788`, nicht auf einen frei erfundenen Implementierungscommit des Rackprojekts.

| ID | Quelle | Verwendungszweck |
|---|---|---|
| R1 | [Repositorybaum am Prüfcommit](https://github.com/JanHG98/netcore-tetra/tree/b4736a4d37fe2044fff998d017a36c1c950a8788) | Existenz, Dateipfade und Scopeprüfung. |
| R2 | [System-Backend-README](https://github.com/JanHG98/netcore-tetra/blob/b4736a4d37fe2044fff998d017a36c1c950a8788/system-backend/README.md) | Dienstzuständigkeiten, Trennung funknahe Echtzeit/Backend. |
| R3 | [Dienstkatalog](https://github.com/JanHG98/netcore-tetra/blob/b4736a4d37fe2044fff998d017a36c1c950a8788/system-backend/services.toml) und `system-backend/*/config/*.example.toml` | Vorhandene Dienste, Abhängigkeiten und Beispielports. |
| R4 | [Edge-Fallback-Dokumentation](https://github.com/JanHG98/netcore-tetra/blob/b4736a4d37fe2044fff998d017a36c1c950a8788/Docs/EDGE_FALLBACK.md) | Lokale Autonomie, Service-Matrix und Grenzen der Abnahme. |
| R5 | [Edge-Fallback-Konfiguration](https://github.com/JanHG98/netcore-tetra/blob/b4736a4d37fe2044fff998d017a36c1c950a8788/crates/tetra-config/src/bluestation/sec_edge_fallback.rs); `crates/tetra-config/src/bluestation/config.rs`, `state.rs`; `crates/tetra-entities/src/net_control_room/{worker.rs,edge_store.rs}`; `crates/tetra-entities/src/cmce/subentities/sds_bs.rs` | Code-/Zustands-/Cache-/Spoolnachweise. |
| R6 | [Media-Library-README](https://github.com/JanHG98/netcore-tetra/blob/b4736a4d37fe2044fff998d017a36c1c950a8788/system-backend/media-library/README.md), Konfiguration und `install/shared-storage.sh` | Medienfunktion, Archivpfade, Mount-/Rechteabhängigkeit. |
| R7 | [Recorder-README](https://github.com/JanHG98/netcore-tetra/blob/b4736a4d37fe2044fff998d017a36c1c950a8788/system-backend/recorder/README.md), Konfiguration und systemd-Unit | Passive Aufzeichnung, Recoverylayout und Dienstpfade. |
| R8 | [SIP-Switch-README](https://github.com/JanHG98/netcore-tetra/blob/b4736a4d37fe2044fff998d017a36c1c950a8788/system-backend/sip-switch/README.md), `src/netcore_sip_switch.py`, `src/netcore_sip_runtime.py` | Zentraler Vermittlungsweg und Grenzen. |
| R9 | [Phase 11c](https://github.com/JanHG98/netcore-tetra/blob/b4736a4d37fe2044fff998d017a36c1c950a8788/Docs/PHASE_11C_EXCLUSIVE_SIP_REGISTRATION_FAILOVER.md), `Docs/PHASE_11B_LOCAL_TBS_SIP_FALLBACK.md` | Phase 11b ersetzt, exklusive Registrierungsumschaltung. |
| R10 | [Lokaler SIP-Fallback](https://github.com/JanHG98/netcore-tetra/tree/b4736a4d37fe2044fff998d017a36c1c950a8788/system-backend/sip-switch/tbs-fallback), insbesondere Konfiguration, Python-State-Machine, Installationsdoku und Unit | TBS-Asterisk, Zeitparameter und PBX-Ersatzweg. |
| R11 | [Open-Lab-LXC-Deployment](https://github.com/JanHG98/netcore-tetra/blob/b4736a4d37fe2044fff998d017a36c1c950a8788/Docs/OPEN_LAB_LXC_DEPLOYMENT.md) und `deploy/open-lab/` | Vorhandenes Inventory-/Deploymentkonzept. |
| R12 | [Root-TBS-Konfiguration](https://github.com/JanHG98/netcore-tetra/blob/b4736a4d37fe2044fff998d017a36c1c950a8788/config.toml) | Gelesene Beispielwerte; keine sensiblen Inhalte in den Abschluss übernommen. |
| R13 | [Aktueller main-Snapshot](https://github.com/JanHG98/netcore-tetra/tree/7137e0dd69877e1b604bf89148fd8b6b590c1a97) | Separater heutiger Vergleich. Der zugehörige Mergecommit nennt PR #59 zum Dashboarddesign; kein Implementierungs-PR dieses Rackchats. |
| R14 | [Archivindex](README.md) | Bereits vorhandene Einträge bewahren, diesen Abschluss ergänzen. |
| R15 | `crates/tetra-entities/src/net_dashboard/server.rs` | Vorhandene `/api/edge-fallback`-Diagnose. |
| W1 | [Ubiquiti: Power over Ethernet](https://help.ui.com/hc/en-us/articles/360015399993-Intro-to-Networking-Power-Over-Ethernet-PoE) | Historischer Link, erneut geöffnet; aktive/passive PoE-Varianten und Gerätekompatibilität. |
| W2 | [Cisco Aironet 1850 Deployment Guide](https://www.cisco.com/c/en/us/td/docs/wireless/controller/technotes/8-1/1850_DG/b_Cisco_Aironet_Series_1850_Access_Point_Deployment_Guide.html) | Historischer Link, erneut geöffnet; Metall/leitfähige Objekte beeinflussen die Antennenabstrahlung. Keine Freigabe dieses Masts oder seiner Abstände. |
| W3 | [ETSI TR 103 943 V1.1.1](https://www.etsi.org/deliver/etsi_tr/103900_103999/103943/01.01.01_60/tr_103943v010101p.pdf), insbesondere § 7.2, PDF-S. 35–36 | Historischer Link erneut geprüft; DECT-Frequenzhintergrund, Bericht zu DECT-2020 NR. |

Ein ergänzender Abruf des Proxmox-Administrationshandbuchs war mit HTTP 403 blockiert. Daraus wurde kein aktuelles Proxmox-Versionsergebnis und keine behauptete Modellkompatibilität abgeleitet. Die Proxmox-Angaben dieses Abschlusses bleiben die gekennzeichnete Chatplanung und deren offene Verifikationspunkte.

Querverweise auf **andere**, bereits archivierte Chats; keine Vermischung ihrer Beschlüsse:

- [WERMA, BPI-Router und Rackarchitektur](2026-10-03_werma-racksignalisierung-bpi-r4-pro-und-rack-architektur.md)
- [UHF-Antennenmast und RX/TX-Entkopplung](2026-10-03_sirio-spo-380-2-rx-tx-antennenmast-und-entkopplung.md)
- [Koax, Antennen und 3D-Druck-Patchpanel](2026-10-04_mobile-tbs-koax-antennen-und-3d-druck-rf-patchpanel.md)
- [Recorder, Edge-Fallback und Echtzeitpfad](2026-10-04_recorder-lxc-edge-fallback-echtzeit-und-buildfehler.md)

## 14. Fortsetzungsstand

Die Fortsetzung beginnt bei einem **geplanten vier-HE-Entwicklungsknoten ohne interne USV**, einer noch zu prüfenden Sophos als gemeinsamem Proxmox-Core und absetzbaren WLAN-/DECT-Geräten am vorgeschlagenen UHF-Mast. Das Repository liefert inzwischen umfangreiche Backend-, Medien- und Fallbackbausteine. Es liefert noch keinen Nachweis, dass genau diese Geräteauswahl mechanisch passt, auf der Appliance läuft oder am Mast unter Last störungsfrei funktioniert.

Der nächste sachliche Schritt ist die konkrete Hardwareaufnahme der Sophos und des Cases, gefolgt von einem Pass-/Leistungsplan und einem lokalen Proxmox-/Netz-/Storageversuch. Erst danach sind DECT-/SIP-/TETRA-Ende-zu-Ende-, Fallback-, thermische und HF-Tests sinnvoll abnehmbar. Die Chatarchivierung selbst erfolgt nach Nutzerprüfung manuell durch Jan.
