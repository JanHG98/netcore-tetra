# Technische Abschlussdokumentation: TETRA-Fachbuch, Canvas-Kapitel, Normenbasis und Korrekturbedarf

> **Archivstatus:** Arbeits- und Übergabedokument, keine fachliche Freigabe des Buches und kein Nachweis eines produktiven TETRA-Netzes. Der Chat hat überwiegend Lehrbuchtexte erzeugt. Zahlreiche als „technisch korrekt“ bezeichnete Antworten enthalten nachweisbare Fehler oder unbelegte Verallgemeinerungen. Diese Dokumentation bewahrt die Entstehungsgeschichte und trennt sie von nachträglich überprüften Quellen- und Repository-Befunden.

## 1. Metadaten

| Feld | Wert |
|---|---|
| Projekt der Archivierung | NetCore-Tetra |
| Thema dieses Chats | Kapitelweise Erstellung eines allgemeinen TETRA-Fachbuchs einschließlich Praxis, Sicherheit, Signalverarbeitung, Standards und Anhängen |
| Rekonstruierter Buchtitel | „TETRA – Was ist das?“; aus ergänzend abgerufenem früherem Gesprächskontext, nicht als tatsächlicher UI-Chattitel bestätigt |
| Ursprünglicher Chattitel | Nicht zuverlässig verfügbar |
| Ursprünglicher Chatlink | Nicht verfügbar; keine URL aus Canvas-IDs konstruiert |
| Erstellungsdatum dieser Dokumentation | 2026-10-04 |
| Historischer Zeitraum | Nicht vollständig rekonstruierbar; ergänzender Gesprächskontext verweist auf die ursprüngliche Buchanforderung vom 2025-10-02. Einzelne Kapitelzeitpunkte sind nicht belastbar verfügbar. |
| Repository | `JanHG98/netcore-tetra` |
| Ausschließlicher Schreibbranch | `Archiving` |
| Für die technische Stichprobe festgehaltener Commit | `4e8a633e821b21440057672e8f6dc622fbb64119` |
| Zugehöriger Tree | `6bfcdac4cd3b63d8323553fd5a6d70d0418fda90` |
| Zeitpunkt dieses Ausgangscommits laut GitHub | 2026-10-04T01:14:09Z |
| Repository-Metadaten bei Prüfung | Öffentlich, nicht archiviert; Default-Branch `main`; Schreibberechtigung auf das Repository vorhanden |
| Ablage dieser Zusammenfassung | `Docs/archive/2026-10-04_tetra-fachbuch-canvas-normen-und-korrekturbedarf.md` |
| Archivindex | `Docs/archive/README.md` |
| Historischer Software-Commit dieses Buchchats | Nicht nachgewiesen |
| Historische PRs dieses Buchchats | Nicht nachgewiesen |

Der oben genannte Commit ist der **Lesestand der technischen Prüfung**, nicht ein behaupteter historischer Implementierungscommit des Buches. Der spätere Archivierungscommit ergibt sich aus der Git-Historie dieser Datei und wird in der Abschlussmeldung genannt. Zwischenzeitliche Branchänderungen sind beim Speichern durch erneutes Lesen des Branchkopfes und Erhalten des bestehenden Archivindexes zu berücksichtigen.

## 2. Quellenlage, Abgrenzung und Auswertungslücken

### 2.1 Tatsächlich zugängliche Grundlagen

Ausgewertet wurden die sichtbaren Nutzeranforderungen, die verfügbaren Kapiteltexte und historischen Canvas-Werkzeugantworten, die ausdrücklich mitgeteilte manuelle Canvas-Fassung, ergänzend abgerufener früherer Gesprächskontext, die 25 bereitgestellten ETSI-PDF-Dateien sowie gezielt gelesene Dateien des oben festgehaltenen Repository-Commits.

Die PDF-Dateien wurden vollständig **inventarisiert**, aber nicht sämtlich fachlich von der ersten bis zur letzten Seite geprüft. Gelesen wurden insbesondere Titel, Versionsstände, Dokumentenfamilien sowie ausgewählte Stellen zu Security Classes, Authentifizierung, Schlüsseln, Sprachcodec, Kanalcodierung, Interleaving, CRC, Modulation, Status und Schnittstellen. Ausgewählte Formeln und Tabellen wurden zusätzlich als Seitenbilder kontrolliert.

Der ursprüngliche Nutzerlink zu Crypto Museum wurde für diese Archivierung tatsächlich aufgerufen. Ergänzend wurden offizielle ETSI-Quellen zur TEA-Verwaltung und veröffentlichten Sicherheitsbefunden geprüft. Diese **neue Quellenprüfung** wird nicht rückwirkend als damals durchgeführte Recherche ausgegeben.

### 2.2 Nicht vollständig zugänglich

- Der Beginn des Chats und viele frühe Antworten sind gekürzt beziehungsweise als ausgelassene Nachrichten erkennbar. Für zahlreiche Kapitel bis einschließlich 52 stehen nur Anforderungen oder Titel zur Verfügung.
- Die vollständige ursprüngliche Gliederung mit allen 117 Kapiteln konnte nicht wiederhergestellt werden. Fehlende Titel werden nicht geraten.
- Die aktuellen vollständigen Canvas-Dokumente und ihre gesamte Versionshistorie konnten nicht erneut exportiert werden. Erhaltene Werkzeugpayloads und Erfolgsantworten sind historische Zustände, kein Beweis der heutigen Canvas-Fassung.
- Ein vollständiges Word-/PDF-Buch, eine konsolidierte Manuskriptdatei und ein verifizierter Gesamtexport aller Kapitel liegen in diesem Auftrag nicht vor.
- Eigenständige historische Bilddateien, Fotos oder Screenshots dieses Buchchats sind im anfänglich verfügbaren Dateibestand nicht enthalten. Die PDF-Abbildungen sind Bestandteil der Referenzdokumente, keine separat hochgeladenen Chatbilder.
- Kein Zugriff auf laufende Basisstationen, Funkgeräte, Leitstellen oder Schlüsselverwaltung wurde für diese Archivierung verwendet. Es gibt keine neue On-Air-, Last-, Sicherheits- oder Betriebsabnahme.

### 2.3 Keine Vermischung mit anderen Projektchats

Der umgebende Projektkontext nennt unter anderem Discovery, Syslog-LXC, Raspberry Pi, SXceiver, MQTT, Control Room und weitere NetCore-Arbeiten. Diese Themen dürfen nicht pauschal als in diesem Buchchat implementiert gelten. Sie erscheinen hier nur, soweit ein Buchtext sie behauptet oder ein gezielter Repository-Abgleich eine relevante Schnittstelle zeigt.

Insbesondere ist das ursprüngliche Buchvorhaben nach dem ergänzend abgerufenen Gesprächskontext **allgemein über TETRA und ohne NetCore-/ZKN-Schwerpunkt** angelegt worden. Die heutige Archivierung im NetCore-Repository ändert diese historische redaktionelle Anforderung nicht.

## 3. Ziel, Ausgangslage und endgültige Anforderungen

### 3.1 Ziel des Buchprojekts

Geplant war ein umfangreiches deutschsprachiges TETRA-Lehr- und Nachschlagewerk: vom verständlichen Einstieg über Einsatzfelder, Architektur, Protokolle und Betrieb bis zu mathematischen Herleitungen, Normen und Verzeichnissen. Einsteiger sollten folgen können, ohne dass die technische Substanz durch unbelegte Vereinfachungen ersetzt wird.

Der Nutzer lieferte die Kapitelnummern und Titel nacheinander. Aus „wie immer“, „go“ oder einer bloßen Kapitelnummer ergab sich die Erwartung, direkt das entsprechende Kapitel zu erstellen, nicht erneut um dieselbe Freigabe zu bitten.

### 3.2 Verbindliche redaktionelle Festlegungen

| Anforderung | Status | Begründung beziehungsweise Konsequenz |
|---|---|---|
| Deutsch, verständlich, technisch belastbar | Beschlossen/geplant | Einsteigerfreundlichkeit und technische Tiefe sollen zusammen funktionieren. |
| Kapitelweise Bearbeitung nach Nutzer-Titel | Beschlossen/geplant | Die Nummerierung des Nutzers hat Vorrang vor Vorschlägen des Assistenten. |
| Keine unnötigen Wiederholungen | Beschlossen/geplant | Mehrfach ausdrücklich verlangt; Grundlagen und Vertiefungen müssen unterschiedliche Aufgaben erhalten. |
| Kapitel 8 von Kapitel 9 abgrenzen | Beschlossen/geplant | Energie/Bahn/Industrie nicht mit Flughäfen/Häfen/Logistik doppeln. |
| Kapitel ab 22 im Canvas | Beschlossen/geplant; historische Textanlage teilweise belegt | Historische Tool-Ergebnisse liegen vor, heutige Vollständigkeit ist nicht gesichert. |
| Kapitel 65 isoliert bearbeiten | Ausdrücklich beschlossen; separate Anlage historisch bestätigt | Reaktion auf wiederholte Fehlkorrekturen und falsches Zieldokument. |
| Spätere Kapitel jeweils als eigenes Dokument | Historisch durch viele `create_textdoc`-Antworten belegt | Verhindert das zuvor beobachtete Überschreiben eines gemeinsamen Dokuments. |
| Schriftart Cambria | Ausdrücklich beschlossen | Frühere HTML-Span-Angaben beweisen keine tatsächliche Schriftsetzung. Für einen späteren Word-/PDF-Export ist eine echte Formatvorlage erforderlich. |
| Kapitel 64 ausführlicher | Beschlossen; Text im Verlauf erweitert | Umfangserweiterung ist belegt, fachliche Freigabe nicht. |
| Abkürzungen und Glossar vollständig A–Z | Beschlossen; Ergänzungen V–Z historisch angelegt | „Vollständig“ und „über 200“ wurden nicht durch eine belastbare Bestandsprüfung nachgewiesen. |
| Umfangreiches FAQ | Beschlossen; Entwurf erstellt | „Go nuts“ erlaubte inhaltliche Breite, keine unbelegten Sicherheitsgarantien. |
| Literaturrecherche nach Büchern, gegebenenfalls Internet Archive | Beschlossen/geplant; nur Kandidatenliste im sichtbaren Verlauf | Rund 50 Bücher waren ein Wunsch, nicht ein nachgewiesenes Rechercheergebnis. |
| Wissenschaftliche Literaturangaben | Beschlossen/geplant | Autor, Titel, Ausgabe, Jahr, Verlag und Identifier müssen geprüft werden. |
| Allgemeines Buch ohne NetCore/ZKN; SDR-Laboratorium nicht als eigenes Buchthema | Ergänzend aus früherem Gesprächskontext rekonstruiert | Nicht durch spätere beiläufige Assistentenbehauptungen über NetCore-Funktionen aufgehoben. |
| Archivierung ausschließlich unter `Docs/archive/` in `Archiving` | Aktueller ausdrücklicher Auftrag | Keine Änderung von Code, Wiki, produktiver Roadmap, anderen Branches oder Canvas in diesem Archivierungsauftrag. |

## 4. Chronologie, Korrekturen und ersetzte Ansätze

### 4.1 Frühe Kapitel und Übergang zum Canvas

Zu Beginn wurden Kapitel überwiegend einzeln angefordert. Die sichtbare Themenfolge beginnt nach größeren Lücken mit Kapitelnummern und später konkreten Überschriften. Ab Kapitel 22 verlangt der Nutzer ausdrücklich Canvas. Die fehlenden frühen Textkörper dürfen nicht durch nachträgliche freie Rekonstruktionen als Originale ersetzt werden.

### 4.2 Gemeinsames Canvas wurde wiederholt ersetzt statt erweitert

Ein zentrales Dokument trug den Namen **„Teds Ofdm Chapter“** und die historische ID `68eadfa675c88191937cba41008be8f5`. Viele Aufrufe verwendeten `update_textdoc` mit dem Muster `.*` und ersetzten damit den gesamten Inhalt durch das nächste Kapitel. Die dazugehörigen Antworten sagten dennoch häufig „ergänzt“ oder „vollständig eingetragen“.

Die ausdrücklich mitgeteilte manuelle Canvas-Fassung dieses Dokuments enthielt später **Kapitel 72**, nicht ein Hauptbuch mit allen Kapiteln 55–72. Das ist entscheidend: Ein erfolgreicher Vollersetzungsaufruf beweist nicht, dass frühere Kapitel daneben erhalten geblieben sind.

Bei dem Versuch, Kapitel 65 über einen Bereich ab `# 65.` beziehungsweise bis `# 66.` zu ersetzen, meldete das Werkzeug zweimal `pattern not found`. Aus dem dokumentierten Zustand ist ein falsches beziehungsweise inzwischen überschriebenes Zieldokument die naheliegende Erklärung. Die frühere Aussage, der Editor habe die Überschrift bloß nicht erkannt, war nicht belegt.

**Verworfener Ansatz:** ein einziges Dokument ohne vorherige Inhaltsprüfung mittels Ganztext-RegEx als vermeintlichen Kapitelspeicher behandeln. **Historisch erfolgreicherer Ansatz:** Kapitel 65 und danach viele weitere Kapitel separat anlegen. Eine spätere Zusammenführung muss auf verifizierten Exporten beruhen.

### 4.3 Sichtbarkeitsproblem bei Kapitel 56

Der Nutzer meldete, dass nichts im Canvas stehe. Der Assistent versuchte denselben Text erneut und verwies anschließend auf eine möglicherweise hängende Anzeige. Eine erfolgreiche Sichtprüfung durch den Nutzer ist nicht belegt. Das Problem ist als **offene Inhalts-/Darstellungsfrage** zu erhalten; ein Browser-Neuladen war nur vorgeschlagen, nicht als wirksame Reparatur bestätigt.

### 4.4 Mehrfache TEA-Korrekturen

Der Nutzer beanstandete Kapitel 65 wiederholt als faktisch falsch. Zu korrigieren waren insbesondere die Einsatzbereiche von TEA1–TEA4. Er lieferte eine eigene Zuordnung und anschließend den Crypto-Museum-Link.

Mehrere darauf folgende Assistentenfassungen wurden wiederum als „korrigiert“ bezeichnet, enthielten aber weiterhin problematische Aussagen: TEA1 ausschließlich EU-Industrie, TEA2 als automatisch stärkste Stufe, TEA4 als frei verfügbare schwache Variante, pauschales Schengen-/Exportverbot, eingeschränkte Algorithmenliste für Class 2, automatische Replay-Immunität und angebliche TEA-E2EE-Derivate.

**Endgültige Nutzerentscheidung:** fachliche Berichtigung anhand belastbarer Quellen und isolierte Bearbeitung. **Nicht erreicht:** eine verlässlich geprüfte Schlussfassung. Die jüngste historische Assistentenfassung ist deshalb nicht allein wegen ihres Datums als sachlich richtig zu übernehmen. Abschnitt 9 trennt die historische Korrekturabsicht von den neuen Prüfbefunden.

### 4.5 Nummerierungs- und Themenkonflikte

| Nummer | Verlauf | Für die Fortsetzung festzuhalten |
|---|---|---|
| 11 | Zunächst nur Nummer, dann ausdrückliche Präzisierung „militärische Anwendungen“ | Präzisierter Nutzer-Titel gilt. |
| 59 | Nochmals Zeit-/Frequenzsynchronisation angefragt; Assistent erzeugte Synchronisationsburst/Frame Alignment; Nutzer stellte anschließend auf Latenzen & QoS um | Letzter ausdrücklicher Titel: **Latenzen & QoS im Funkkanal**. Die zusätzliche Synchronisationsfassung ist nicht ein zweites Kapitel 59. |
| 65 | Mehrere falsche Korrekturen; schließlich separates Canvas | Inhalt bleibt fachlich nachzuarbeiten, nicht nur formal zu übernehmen. |
| 89/90 | Nach 89 DMR kurz 89 P25, unmittelbar danach 90 P25 | **89 DMR, 90 P25** gemäß späterer expliziter Nummer. |
| 101 | Kein belastbarer Titel im sichtbaren Material | Lücke; nicht frei ergänzen. |
| 102/103 | Reihenfolge der Anfragen zeitweise vertauscht | **102 Rauschmodelle**, **103 Shannon & Nyquist**. |
| 104 | QoS-Berechnung ohne Nummer wurde vom Assistenten als 104 angelegt; danach ausdrückliche Nutzeranforderung 104 Sprachruf-Beispielrechnung | **104 Sprachruf → Bits → Luftschnittstelle** gilt. QoS-Entwurf bleibt erhaltenes Thema ohne endgültig bestätigte neue Nummer. |
| 115 | Kein belastbarer vollständiger Kapitelauftrag im sichtbaren Material | Lücke; nicht aus dem Themenumfeld erraten. |
| 116/117 | Zunächst 116 Quellen & Literaturverzeichnis; später 116 Weiterführende Links. Ergänzender ursprünglicher Plan nennt 117 für Links. | Beide Inhalte bewahren. Die letzte ausdrückliche Link-Anforderung lautet 116, der Gesamtnummernplan bleibt zu bereinigen. Keine stille Umnummerierung. |
| 111/112 | Als A–Z bezeichnet, erste Anlage endete jeweils bei U; Nutzer forderte V–Z nach | Nachträge sind vorhanden, Vollständigkeit und Begriffsqualität dennoch offen. |

## 5. Kapitelbestand und thematische Abdeckung

Die Tabelle ist eine **Bestands- und Übergabematrix**, keine Aussage, dass alle Texte exportiert, fachlich freigegeben oder im Repository als Buch vorhanden sind. „Entwurf“ bezeichnet den im Verlauf sichtbaren Text; „Titel“ bedeutet, dass der eigentliche Textkörper nicht ausreichend zugänglich ist.

| Kapitel | Gegenstand | Zugänglicher Zustand / wichtiger Anschluss |
|---|---|---|
| 1–7 | Einstiegskapitel | Frühe Nachrichten fehlen; einzelne Nummern sichtbar, Titel und Inhalte nicht vollständig. |
| 8 | Energie, Bahn & Industrie | Titel und ausdrückliche Abgrenzung zu 9. |
| 9 | Flughäfen, Häfen & Logistik | Titel; nicht mit 8 doppeln. |
| 10 | Nicht sicher rekonstruierbar | Keine Titelergänzung aus Vermutung. |
| 11 | Militärische Anwendungen | Präzisierter Nutzer-Titel. |
| 12 | Internationale Netze: Airwave, C2000, Nødnett, Rakel, Virve, BOSNet | Titel/Themenliste. |
| 13–20 | Grundaufbau; Basisstation; SwMI; Leitstellenintegration; Topologien; Redundanz; GPS/NTP; Energie/Fallback | Titel sichtbar; frühe Kapiteltexte teilweise ausgelassen. |
| 21–28 | Sprachdienste; Prioritätslogik; PSTN/ISDN/SIP; SDS; Status; IP/TEDS; GPS/LIP; Logging/Recording/Dispatcher | Titel sichtbar; ab 22 Canvas ausdrücklich gewünscht. |
| 29–38 | OSI; PHY; π/4-DQPSK; Symbol-/Bitrate; Bursts; TDMA; FEC/CRC; Interleaving/Scrambling; Synchronisation; Link Budget | Titel sichtbar; spätere Deep-Dive-Kapitel sollen vertiefen, nicht unkontrolliert duplizieren. |
| 39–49 | MAC; Header; Scheduling; RACH; Retransmission; Network Layer; ISSI/GSSI; Location Update; MM; Rufaufbau/Release/Handover; Higher Layers | Titel sichtbar. |
| 50–52 | ACELP-Überblick; SDS Type 1–4; Layer-3-PDUs | Titel und teilweise Abschlussantworten, vollständige frühe Texte fehlen. |
| 53 | IP over TETRA | Entwurf; SDS und Paketdaten wurden unzulässig vermischt. |
| 54 | TETRA Release 2 | Entwurf; Release-, Sicherheits- und Schnittstellenbehauptungen prüfen. |
| 55 | TEDS, im historischen Titel mit OFDM | Entwurf; Subträgerzahlen und pauschale OFDM-Zuordnung nacharbeiten. |
| 56 | ACELP intern: Parameter und Vektorquantisierung | Entwurf; Bitzuweisung und Codebook-/Excitation-Beschreibung korrigieren; Sichtbarkeitsproblem. |
| 57 | π/4-DQPSK mathematisch, Konstellation, IQ-Rotation | Entwurf; Symbolkonstellation von kontinuierlicher Hüllkurve trennen. |
| 58 | Zeit-/Frequenzsynchronisation | Entwurf; Slotlänge, NTP/GNSS-Rollen, Toleranzen und Reichweitenannahmen prüfen. |
| 59 | Latenzen & QoS im Funkkanal | Letzter ausdrücklich verlangter Titel; pauschale Garantien und QCI-Übertragung nicht übernehmen. |
| 60 | BER & FER | Entwurf; Messpunkt, Nenner, Fehlermodell und Aussagegrenzen fehlen. |
| 61 | Komplette SDS: Mikrofon → Bits → Funkwelle → Empfänger | Entwurf vermischt Sprache, Status und SDS; Beispiel-PDUs nicht als reale Codierung verwenden. |
| 62 | Authentifizierung, SIM, Key-Material | Entwurf; GSM-Terminologie fälschlich als TETRA-Verfahren verwendet. |
| 63 | AIE | Entwurf; Schlüsselarten, Schutzumfang, Integrität und Replay unterscheiden. |
| 64 | E2EE | Mehrfach erweitert; kryptografische und NetCore-bezogene Behauptungen nicht freigegeben. |
| 65 | TEA1–TEA4 | Wiederholt beanstandet, zuletzt separates Dokument; zentrale offene Fachkorrektur. |
| 66 | AES-Integration | Neuer isolierter Entwurf; Modi, Blockgröße, Schlüssel und Integrität trennen. |
| 67 | Key-Management & Distribution | Isolierter Entwurf; Cambria-Anforderung ab hier ausdrücklich; OTAK nicht als allgemeiner Schlüsseltyp behandeln. |
| 68 | Jamming, Replay, Fake-BS | Isolierter Entwurf; Sicherheitsgarantien zurücknehmen, Bedrohungen und Schutzgrenzen sauber beschreiben. |
| 69 | Schutzmechanismen & Redundanzstrategien | Entwurf; Architekturmaßnahme nicht mit garantierter Verfügbarkeit verwechseln. |
| 70 | Monitoring & SLA-Management | Entwurf; Kennzahlenziele nur als begründete Beispiele, nicht ETSI-Garantien. |
| 71 | Logging & Netzdiagnose | Entwurf; Rechtsbezüge, Aufbewahrungszeiten und Forensikbehauptungen prüfen. |
| 72 | Netzausbau & Kapazitätsplanung | Entwurf; Rohslots nicht gleich nutzbare Sprachkanäle. |
| 73 | Indoor-Systeme, DAS, Tunnel | Entwurf; Ausleuchtung, Kapazität, Laufzeit und Isolation auseinanderhalten. |
| 74 | Campuslösungen für Industrie | Entwurf; keine automatische On-Premise-, Krypto- oder Verfügbarkeitsgarantie. |
| 75 | Fallback-Betrieb, USV, Diesel, Solar | Entwurf; Energieautonomie und lokaler Funkbetrieb sind unterschiedliche Abhängigkeiten. |
| 76–81 | Teil 9: HRT; MRT/FRT; Zubehör; Repeater/Gateways; Firmware/Service; Bedienfehler | Einzelentwürfe; Hersteller-/Gerätezuordnung, Repeaterarten und sichere Bedienhinweise prüfen. |
| 82–88 | Teil 10: Funkdisziplin; Status; Dispatcher; Großevents; Katastrophen; Deutschland; internationale Beispiele | Einzelentwürfe; Statusmatrix besonders kritisch, reale Fallstudien benötigen echte Einsatz-/Betreiberquellen. |
| 89–93 | Teil 11: DMR; P25; NXDN; LTE/5G/MCX; 6G/AI/Edge/SDN | Einzelentwürfe; wertende Generalisierungen und Zukunftsprognosen durch belastbare Vergleichsbedingungen ersetzen. |
| 94 | Teil 12: Herleitung π/4-DQPSK | Entwurf; Phase, Symbolfolge und Pulsform präzisieren. |
| 95 | IQ-Diagramme & Konstellationspunkte | Entwurf; Beispielkoordinaten widersprechen der eigenen Phasenrekursion. |
| 96 | Viterbi-Decoder | Entwurf; ML/MAP nicht an Hard-/Soft-Decision festmachen. |
| 97 | Faltungscodes und Generatorpolynome | Entwurf; behauptete TETRA-Polynome und Constraint Length falsch. |
| 98 | Interleaving-Matrizen | Entwurf; TETRA-Sprachkanal ist nicht das behauptete 456-Bit-/24×19-Schema. |
| 99 | CRC-Polynome | Entwurf; falsche Kanalzuordnungen, widersprüchliches Rechenbeispiel, Restkonvention unzureichend. |
| 100 | Symbol-zu-Bit-Mapping | Entwurf; Demapper liegt vor Deinterleaving/FEC, nicht nach CRC. |
| 101 | Unbekannt | Lücke. |
| 102 | AWGN, Rayleigh, Fading | Entwurf; additives Rauschen und multiplikatives Fading sauber unterscheiden. |
| 103 | Shannon & Nyquist | Entwurf; Abtasttheorem, ISI-Kriterium und RF-/Basisbandbandbreite wurden vermischt. |
| 104 | Sprachruf → Bits → Luftschnittstelle | Letzter Nutzer-Titel; Beispielkette muss neu gerechnet werden. |
| ohne endgültige Nummer | QoS-Berechnung anhand BER/FER | Vorher als 104 angelegt; Inhalt nicht verlieren, Nummer offen. |
| 105 | ETSI-Standards | Entwurf; Dokumententeile mehrfach falsch zugeordnet. |
| 106 | ITU-Einordnung | Entwurf; Aufgaben von ITU, CEPT und nationalen Regulierern sowie Dokumententitel prüfen. |
| 107 | Frequenzpläne Europa/international | Entwurf; keine belastbare Freigabeliste für konkrete Länder/Netze. |
| 108 | Regulierungen Deutschland | Entwurf; Gesetze, CE/RED, Frequenzzuteilung und BOS-Zulassung nicht gleichsetzen. |
| 109 | Lizenzmodelle & Betriebskosten | Entwurf mit unbelegten Preis-/Gebühren- und Lebensdauerwerten. |
| 110 | Interoperabilitätstests & Herstellerzertifizierung | Entwurf; Zertifikate belegen jeweils bestimmte Produkte, Versionen und Prüfumfänge, keine grenzenlose Kompatibilität. |
| 111 | Abkürzungsverzeichnis | A–U angelegt, V–Z ergänzt; Umfangs- und Begriffsaudit offen. |
| 112 | Glossar A–Z | Ebenfalls zunächst bei U abgebrochen, dann ergänzt; mehrere zweifelhafte Begriffe. |
| 113 | Vergleichstabellen | Entwurf; übernimmt Fehler aus Codec-/Modulations-/Reichweitenkapiteln. |
| 114 | FAQ | Umfangreicher gewünscht und erstellt; absolute Aussagen über Sicherheit, Reichweite und Netzausfälle müssen revidiert werden. |
| 115 | Nicht sicher rekonstruierbar | Lücke. |
| 116, frühere Fassung | Quellen & Literaturverzeichnis | 53 nummerierte, heterogene Kandidaten; keine nachgewiesenen 50 Bücher. |
| 116, spätere Fassung | Weiterführende Links | Letzter expliziter Nutzer-Titel; Konflikt mit Literaturkapitel bleibt offen. |
| 117 | Im ursprünglichen Plan weiterführende Links | Keine entsprechend gesicherte finale Kapitelanlage im verfügbaren Verlauf. |

## 6. Historische Canvas-Referenzen und Wiederherstellung

Die folgenden IDs sind interne historische Dokumentreferenzen. Sie sind **keine Chatlinks, Dateipfade oder öffentliche Download-URLs**. Sie helfen bei einem späteren Export beziehungsweise beim Abgleich mit der Benutzeroberfläche.

| Kapitel / Dokument | Historische Canvas-ID |
|---|---|
| Gemeinsames „Teds Ofdm Chapter“, später nur Kapitel 72 im mitgeteilten Stand | `68eadfa675c88191937cba41008be8f5` |
| 65, isolierte TEA-Korrektur | `691e61e1882081919391c6563e970024` |
| 66 AES | `691e6470c4fc81918184eae9a47aeecc` |
| 67 Key Management | `691e65bc11a08191b41d7c4286d50d10` |
| 68 Angriffe | `691e69b6cc50819199e6ef392e03a69a` |
| 69 Redundanz | `691e6bbe2a508191acf8c80f0f560ce6` |
| 70 Monitoring | `691e6cee0f4881919b3ff321ec92462a` |
| 71 Logging | `691e886d89a08191b38d21132f156b14` |
| 72 Kapazität | `691e96035ac08191b4eba6649dc123f2` |
| 73 Indoor | `691e96c83a508191aadb10fbdbbbd636` |
| 74 Campus | `691e980b50b0819188d870a3ab028d3b` |
| 75 Fallback | `691e99af51308191912e28ab5b057dad` |
| 76 HRT | `691e9a8b95e481918111e6dde93b0b2a` |
| 77 MRT/FRT | `691e9bb57180819199a9099a9f931cfe` |
| 78 Zubehör | `691e9cbfdc188191a3b5828348114666` |
| 79 Repeater/Gateways | `691e9da7a0548191b4394bd99005bd64` |
| 80 Firmware | `691fa0e585888191b0e91fd544b77f9c` |
| 81 Bedienfehler | `691fa18fe3448191ac7ddacff28c6bb2` |
| 82 Funkdisziplin | `691fa22a72248191811df64703400e7e` |
| 83 Status | `691fa35c8da48191997f4e4e21419e3e` |
| 84 Dispatcher | `691fa45aacb48191863d509e6664736a` |
| 85 Großevents | `691fa9f9d40c81919a392524b2217023` |
| 86 Katastrophen | `691faabf53148191b8d5b7169c44a19a` |
| 87 Deutschland | `691fabd26f988191b4189adec6aea369` |
| 88 International | `691fafb4fbf881919b1ead6b671cea0d` |
| 89 DMR | `691fb07766f4819194c6039bc332f0c9` |
| 90 P25 | `691fb204f3008191a259f8b63292b09a` |
| 91 NXDN | `691fb2f1aa2c819185aaab79d2013976` |
| 92 LTE/5G | `691fb54980bc8191b57e967928126517` |
| 93 Zukunft | `691fb8d83abc8191ad718fa059f353cc` |
| 94 DQPSK-Herleitung | `691fb98ec9688191aff5a182aac70206` |
| 95 IQ | `691fc6d8f02c81919e518c9492dae641` |
| 96 Viterbi | `691fc96223208191b69095212aba8ae2` |
| 97 Faltungscodes | `691fca6ee7bc8191a7bf882caad8c92b` |
| 98 Interleaving | `691fcc91680c81919d2e7c200e3206d0` |
| 99 CRC | `691fcd78ed90819195394ea66f2e8747` |
| 100 Mapping | `691fcfa354808191b49408a72d5f1cdd` |
| 102 Rauschmodelle | `691fd4846800819181dcaeadc312cbd1` |
| 103 Shannon/Nyquist | `691fd2240e648191b3d0abb432a31e17` |
| Vorläufiges 104 QoS | `691fd5e4c7b081918fdd01aaa90adf91` |
| Endgültig angefordertes 104 Sprachruf | `691fd97669c88191b8e83dc6691eda6c` |
| 105 ETSI | `691fdb87e5d4819184ae42826f52859f` |
| 106 ITU | `691fe964ed988191a2719fcc6e409b1d` |
| 107 Frequenzen | `691fea27b0288191bc222042f7c240b4` |
| 108 Regulierung | `691febeaa290819182b7741102c7ae2e` |
| 109 Kosten | `6920d2380cf08191a126a6cddb7d2341` |
| 110 IOP | `6926d85972588191a17fae8cf34c37ab` |
| 111 Abkürzungen | `6926da1eee4481919c49a532cfb33386` |
| 112 Glossar | `6926e5818d548191bbc8f9e785090d4a` |
| 113 Tabellen | `69275f79f768819193e97e2233ee8195` |
| 114 FAQ | `692760dffa748191b33c33b0bb3fd5bd` |
| 116 Literatur | `691e72834e80819182a9ce2ef5493367` |
| 116 Links | `69276322182881919c03dd18a352b658` |

Nicht aufgeführte frühe Kapitel-IDs sind nicht zuverlässig verfügbar. Die Tabelle ist kein Inhaltsbackup sämtlicher Canvas-Dokumente. Vor einer späteren Zusammenführung sind Titel, Nummer, tatsächlicher Textkörper und Versionsstand jedes exportierten Dokuments abzugleichen.

## 7. Behandelte Architektur, Komponenten und Abhängigkeiten

### 7.1 Allgemeines TETRA-Modell des Buches

Die Buchtexte beschreiben im Kern folgende Ebenen:

```text
Benutzer / Leitstellenprozess / Datenanwendung
             |
HRT, MRT, FRT bzw. angeschlossene Sprechstelle
             |
TETRA-Funkschnittstelle: PHY, MAC/LLC, MLE/MM/CMCE
             |
Basisstation(en) und SwMI
             |
Leitstellen, Recording, SDS-/IP-Anwendungen, Fremdnetz-Gateways
             |
Betrieb: Monitoring, Logging, Identitäten, Schlüssel, Energie, Backhaul
```

Dies ist eine redaktionelle Zusammenfassung der behandelten Architektur, **kein im Buchchat aufgebautes Deployment**. DMO, TMO, lokaler Rückfallbetrieb, ISI, Telefonie- und Breitbandkopplung müssen als unterschiedliche Betriebs- und Schnittstellenmodelle getrennt bleiben. Referenzen: Anhänge A01, A02, A05, A06, A08 und A10.

### 7.2 Lehrbuchabhängigkeiten

Die nachvollziehbare fachliche Reihenfolge ist: System-/Adressierungsmodell → Dienste und Rufsteuerung → physikalische Übertragung → Fehlerschutz → Sicherheit → Betrieb → praktische Beispiele → Vergleiche → mathematische Vertiefung → Normen und Anhänge. Die Verzeichnisse und Vergleichstabellen hängen von den freigegebenen Hauptkapiteln ab, nicht umgekehrt.

Die mathematischen Beispiele brauchen denselben Satz von Definitionen für Bittypen, Kanalnamen, Zählrichtungen, Startzustände und Zeitbezug. Ein Wechsel zwischen ACELP-Sprachframe, Kanalcodierungsblock, Burst, Zeitschlitz, TDMA-Frame und Multiframe darf nicht unbemerkt erfolgen.

### 7.3 Keine technische Implementierung aus Markennamen ableiten

„NetCore-KMF“, „Watchtower“, „NetCore-Analyzer“, „NetCore-SOC“, „NetCore Planner“ und „NetCore-SDR“ wurden in Entwürfen als Beispiele oder Fähigkeiten dargestellt. Dafür gab es im historischen Buchverlauf keine verifizierten Dateien, Schnittstellen oder Tests. Der spätere Repository-Fund eines KMF-Dienstes macht die alten Behauptungen weder rückwirkend wahr noch zu einer Freigabe eines AES-/TEA-/HSM-Systems.

## 8. Erreichter historischer Entwicklungs- und Betriebsstand

| Gegenstand | Idee | Beschlossen/geplant | Implementiert / Text angelegt | Getestet | Im Betrieb bestätigt |
|---|---|---|---|---|---|
| Umfangreiches TETRA-Buch | Ja | Ja | Viele Kapitelentwürfe im Chat/Canvas historisch belegt | Keine Gesamtprüfung | Nicht anwendbar / kein Publikationsnachweis |
| Vollständiges Manuskript aller 117 Kapitel | Ja | Gliederungsziel | Nicht nachgewiesen | Nein | Nein |
| Separate spätere Kapitel | Ja | Ja | Historische Canvas-Anlagen belegt | Keine vollständige Export-/Renderprüfung | Kein heutiger Vollständigkeitsnachweis |
| Cambria-Satz | Ja | Ja | HTML-Stilangaben in Payloads | Tatsächlicher Font nicht geprüft | Nein |
| Fachlich berichtigtes Kapitel 65 | Ja | Mehrfach ausdrücklich | Mehrere Ersatztexte, aber mit Restfehlern | Erst jetzt begrenzte Quellenstichprobe | Nein |
| Rund 50 wissenschaftlich belegte Bücher | Ja | Rechercheauftrag | Kandidatenliste mit 53 gemischten Quellen | Keine vollständige bibliografische Prüfung | Nein |
| Vollständige Glossare A–Z | Ja | Ja | A–U und Nachträge V–Z | Begriffe/Anzahl nicht vollständig geprüft | Nein |
| NetCore-Funk-, Krypto- oder Leitstellenimplementierung aus diesem Chat | Teilweise beiläufig behauptet | Kein belastbarer Entwicklungsauftrag innerhalb des sichtbaren Buchverlaufs | Historisch nicht nachgewiesen | Keine Builds/On-Air-Tests | Nein |
| ETSI-Quellenbasis | Ja | Nutzung gewünscht | 25 PDFs jetzt zugänglich und inventarisiert | Ausgewählte Text-/Tabellenstellen geprüft | Kein Ersatz für Systemabnahme |
| Diese Archivierung | Ja | Aktuell ausdrücklich autorisiert | Abschlussdokument und Index als eigener Archivauftrag | Speicherung separat zu verifizieren | Kein Eingriff in Funkbetrieb |

**Wichtig:** Ein „Fertig ✅“ im früheren Chat ist allenfalls eine Abschlussbehauptung. Aussagekräftiger sind Werkzeugantwort, tatsächlich vorhandener Inhalt, Quellenbeleg, ausführbarer Code, reproduzierbarer Test und schließlich Betriebsnachweis. Diese Ebenen wurden historisch oft vermischt.

## 9. Technischer Korrekturkatalog und neue Quellenprüfung

Die folgenden Befunde dienen dem Schutz einer späteren Fortsetzung. Sie ändern die historischen Canvas-Texte nicht heimlich. Unterschieden werden **durch Quelle/Rechnung widerlegt**, **eingeschränkt belegbar** und **weiter zu prüfen**. Die Nummern sind Roadmap-Kandidaten dieses Archivs, keine neu angelegten GitHub-Issues.

### 9.1 Sicherheit, TEA und Schlüssel: höchste redaktionelle Priorität

**E01 – TEA-Nummern sind keine aufsteigenden Sicherheitsstufen.** Historisch wurden Zielgruppen, Zulassung, Schlüsselbreite und kryptografische Stärke vermengt. Für die Einordnung sind Algorithmusvariante, zugelassener Nutzerkreis, Schlüsselverwaltung und belegte Sicherheitsanalyse getrennte Größen. Die neu gelesene ETSI-Erklärung von 2023 beschreibt unter anderem die TEA1-Problematik; daraus folgt keine allgemeine Rangfolge „TEA2 am sichersten, TEA3 mittel, TEA4 Basis“. Quellen W01–W04.

Die historische Nutzerkorrektur bleibt als Anforderung erhalten: TEA1 industriell/kommerziell, TEA2 Behörden im europäischen Zulassungsraum, TEA3 Behörden außerhalb dieses Bereichs, TEA4 kommerzielle Variante im Kontext von Exportbeschränkungen. Zusätze wie „TEA1 nur EU-Binnenmarkt“ oder „TEA4 frei und lizenzlos“ wurden damit nicht belastbar belegt. Crypto Museum beschreibt den kommerziellen beziehungsweise Behördenbezug und die nominalen 80-Bit-Schlüssel der ursprünglichen vier Verfahren; bei TEA1 ist die Reduktion auf einen effektiven 32-Bit-Schlüsselraum ein zentraler veröffentlichter Befund. W01 ist eine historische Überblicksseite, keine aktuelle Zulassungsentscheidung.

**E02 – Pauschales TEA2-Schengen-/Exportverbot ist keine ausreichende Beschreibung.** Die geprüfte ETSI TS 101 053-2 V3.1.1, Abschnitte 5.1 und 5.2, verwendet einen eigenen Kreis zulässiger Staaten/Gebiete und die verbindliche Liste des Custodian. Sie enthält außerdem genehmigungsabhängige Regeln für bestimmte militärische Auslandsverwendungen. Deshalb darf die historische Aussage „ausschließlich Schengen, Export ausnahmslos verboten“ nicht als aktuelle allgemeine Rechtsauskunft stehen bleiben. Spezifikationsweitergabe, Geräteexport und Einsatz eines kontrollierten Netzes sind gesondert zu prüfen. Maßgeblich sind jeweils geltende Lizenzbedingungen und zuständige Stellen. Quelle W02; keine individuelle Exportfreigabe durch dieses Archiv.

**E03 – Security Class nicht mit TEA-Variante gleichsetzen.** A09, Abschnitt 4.0 und Tabelle 4.1, unterscheidet Class 1 ohne AIE, Class 2 mit Verschlüsselung und optionaler Authentifizierung sowie Class 3 mit erforderlicher Authentifizierung für DCK und obligatorischem CCK-OTAR. Die frühere Tabelle „Class 2 nur TEA1/TEA4“ ist daraus nicht ableitbar. Ebenso ist „Class 3 bedeutet in jeder Implementierung automatisch erfolgreich erzwungene gegenseitige Authentifizierung“ zu pauschal. Authentifizierungsmechanismen, Betreiberpolicy und tatsächliche Aktivierung sind separat zu beschreiben.

**E04 – GSM-Begriffe durch TETRA-Begriffe ersetzen.** Die Kapitel 62–67 verwenden A3/A8, SRES und Kc als vermeintliche universelle TETRA-Kette. A09 beschreibt dagegen TAA1-Verfahren, unter anderem TA11/TA12, KS, Antworten und DCK; die Anhänge zur TSIM-Schnittstelle bestätigen die TETRA-spezifischen Operationen. Die normativen Schlüsselkategorien DCK, CCK, SCK, GCK, MGCK und ECK dürfen nicht in einem pauschalen „Session Key Kc“ verschwinden. Referenzen A09, A20–A25.

**E05 – OTAR ist ein Verfahren, kein allgemeiner Schlüsseltyp OTAK.** Der alte Stammbaum `Master → KEK → OTAK → EEK/Kc` war ein unbelegtes Universalmodell. Für AIE ist die TETRA-Schlüsselverwaltung aus A09 maßgeblich; ein E2EE-Key-Management kann ein eigenes, profilabhängiges Modell haben. Der aktuelle NetCore-KMF-Ansatz ist wiederum eine konkrete Softwarearchitektur und nicht automatisch identisch mit beiden.

**E06 – Keine standardweiten festen Rotationsintervalle behaupten.** „AIE alle 24–48 h, E2EE 7–30 Tage, Master jährlich“ wurde ohne Beleg als Standard dargestellt. A09, Tabelle 4.4, nennt für DCK die Authentifizierungssitzung und lässt Lebensdauern mehrerer anderer Schlüssel ausdrücklich undefiniert. Betreiberseitige Kryptoperioden müssen als Policy mit Begründung, Verfügbarkeit und Wiederanlaufverhalten dokumentiert werden.

**E07 – Verschlüsselung, Integrität und Replay-Schutz unterscheiden.** XOR mit einem Schlüsselstrom und ein zeitabhängiger Initialisierungswert sind kein allgemeiner Nachweis, dass jede Nachricht authentifiziert und jeder Replay verhindert wird. Die Entwürfe zu AIE, Fake-BS und Replay sind deshalb nicht als Sicherheitsanleitung freizugeben. CRC/FEC ersetzen ebenfalls keine kryptografische Authentizitätsprüfung. Quellen A09 sowie W03/W05; detaillierte Angriffsbewertung bleibt gesonderter Prüfauftrag.

**E08 – E2EE nicht als absolute Vertraulichkeitsgarantie darstellen.** Endpunkte, Schlüsselbesitzer, Leitstellenanbindung, Recorder, Klartextgrenzen und Gateways müssen benannt werden. Ein Gateway, das Audio oder Daten entschlüsselt und in ein anderes System überträgt, erhält nicht automatisch dasselbe Ende-zu-Ende-Sicherheitsmodell. Ungeprüfte „TEA2/E2EE-Derivate“, universelle AES-Modi oder behauptete KMF-Zertifizierungen werden nicht übernommen.

**E09 – AES-Grundlagen und Profilzuordnung nachprüfen.** Die historischen Angaben verwechseln teilweise Schlüssel- und Blockgröße und schreiben CTR Integrität zu. Vor Übernahme von Kapitel 66 sind Algorithmus, Betriebsmodus, Integritätsschutz, Nonce-/Zählerverwaltung, Schlüsseltrennung und tatsächliches TETRA-E2EE-Profil anhand einschlägiger Spezifikationen zu belegen. CPU-Prozente, Zusatzlatenzen und behauptete TPM-/Secure-Element-Eigenschaften sind nicht gemessen. Dieser Archivlauf führt keinen vollständigen AES-/E2EE-Profilaudit durch.

### 9.2 Nachgeprüfte Sprachkanalkette: 274 → 286 → 432 Bit

**E10 – Kapitel 56, 97–99 und 104 gemeinsam korrigieren.** Der Anhang A19, EN 300 395-2 V1.3.3, ist hierfür eine geeignete konkrete Referenz. Seine Tabelle 1 auf Seite 13 weist pro 30-ms-Sprachframe aus:

| Parametergruppe | Bits pro Frame |
|---|---:|
| LP-Filterparameter | 26 |
| Pitch Delay | 23, aufgeteilt 8 + 5 + 5 + 5 |
| Algebraischer Code | 64, aufgeteilt 4 × 16 |
| Vektorquantisierung der zwei Gains | 24, aufgeteilt 4 × 6 |
| Summe | 137 |

Daraus folgt rechnerisch `137 / 0,030 s = 4 566,666… bit/s`, üblicherweise als 4 567 bit/s angegeben. Der Standard beschreibt für normalen Sprachkanalbetrieb die gemeinsame Codierung von **zwei** Sprachframes, somit 274 Quellbits. Die 274 Bit sind **nicht** das Ergebnis einer pauschalen Verdopplung eines einzelnen Frames durch Rate-1/2-FEC.

A19, Abschnitte 5.4 und 5.5, unterscheidet Empfindlichkeitsklassen. Für zwei Frames ergibt sich:

| Anteil | Eingang / Ergänzungen | Nach Schutz und Punktierung |
|---|---:|---:|
| Class 0 | 2 × 51 = 102 Bit | 102 Bit, nicht faltungs-codiert |
| Class 1 | 2 × 56 = 112 Bit | 168 Bit, effektive Rate 2/3 |
| Class 2 einschließlich Prüfbits und Terminierung | 2 × 30 + 8 + 4 = 72 Bit | 162 Bit, effektive Rate 8/18 |
| Gesamt | 274 + 8 + 4 = 286 Bit | **432 Bit** |

Die acht Prüfbits bestehen hier aus sieben CRC-Bits zum Polynom `1 + X^3 + X^7` sowie einem Gesamtparitätsbit; sie beziehen sich auf die besonders empfindlichen Bits. Die Faltungscodierung läuft über Class 1 und Class 2 kontinuierlich. Anschließend wird eine **24×18-Matrix transponiert**; die Anzahl bleibt 432. Interleaving erzeugt keine zusätzlichen Redundanzbits.

Diese Rechnungen wurden für die Archivierung nachgerechnet. Sie sind ein Quellen-/Arithmetikabgleich, kein Test des gesamten Codec- oder Funkpfads. Der Frame-Stealing-Fall besitzt eigene Regeln und darf nicht allein aus dem Normalfall abgeleitet werden.

**Überholt/falsch:** `137 → 274 durch FEC → 456 durch Auffüllen/Interleaving → vier Bursts zu 114 Bit`. Dieses in mehreren Entwürfen verwendete Muster ist keine belastbare TETRA-Sprachkanalbeschreibung.

### 9.3 Generatorpolynome und Decoder

**E11 – Unterschiedliche Muttercodes für Steuerkanal und Sprache.** A02, Abschnitt 8.2.3.1.1, beschreibt für den dortigen RCPC-Pfad einen 16-Zustands-Muttercode der Rate 1/4. A19, Abschnitt 5.4.3.1, beschreibt einen anderen 16-Zustands-Sprachmuttercode der Rate 1/3. Beide besitzen vier Speicherbits; die übliche Constraint Length ist hier K = 5.

Für den Sprachmuttercode sind in A19 angegeben:

```text
G1(D) = 1 + D + D² + D³ + D⁴
G2(D) = 1 + D + D³ + D⁴
G3(D) = 1 + D² + D⁴
```

Der im Repository gelesene Steuerkanalcoder verwendet:

```text
G1(D) = 1 + D + D⁴
G2(D) = 1 + D² + D³ + D⁴
G3(D) = 1 + D + D² + D⁴
G4(D) = 1 + D + D³ + D⁴
```

Damit sind die historischen Aussagen „TETRA typischerweise (7,5)“ beziehungsweise „TETRA K=4, (15,17)“ als universelle Implementierungsvorgaben ungeeignet. Kleine Lehrbeispielcodes bleiben möglich, müssen aber eindeutig als solche bezeichnet werden. Vor Oktalangaben ist die Bit-/Polynomdarstellungsrichtung festzulegen.

**E12 – Viterbi optimal nur unter benannten Modellannahmen.** Die Behauptung „Hard Decision ergibt ML, Softbits ergeben MAP“ ist keine gültige Unterscheidung. Kapitel 96 braucht eine saubere Beschreibung von Pfadmetrik, Kanalmodell, möglichen A-priori-Wahrscheinlichkeiten, Survivor-Pfaden und Traceback. Die im Anhang A19 enthaltene Viterbi-Beschreibung ist eine Implementierungsreferenz, keine Lizenz für pauschale Optimalitätsversprechen.

### 9.4 CRC, Interleaving und Rechenbeispiele

**E13 – CRC-16 des Steuerpfads ist konkret nachgewiesen.** A02, Abschnitt 8.2.3.3, enthält `X^16 + X^12 + X^5 + 1`. Der geprüfte Repository-Code verwendet `0x1021`, Initialwert `0xffff`, MSB-first-Verarbeitung und nach angehängter komplementierter Prüfsumme den erwarteten Rest `0x1d0f`. Ein pauschales „Empfang korrekt genau dann, wenn Rest null“ beschreibt diese Implementierung nicht vollständig. Referenzen R02 und R06.

Die alten Tabellen „SDS/PAD CRC-10, Traffic-Signalisierung CRC-11“ sind nicht durch die geprüften TETRA-Stellen belegt. Prüfsummen sind nach **Schicht und konkretem Kanal/PDU** zuzuordnen; SDS ist nicht selbst eine allgemeine CRC-Länge. Ein CRC-Erfolg bedeutet zudem weder Fehlerfreiheit mit absoluter Sicherheit noch Authentizität.

**E14 – Das CRC-10-Beispiel in Kapitel 99 ist intern widersprüchlich.** Die dortige Binärfolge `11010100011` entspricht nicht dem daneben ausgeschriebenen Polynom `X^10 + X^9 + X^5 + X^4 + X + 1`; dieses würde `11000110011` ergeben. Für die angegebene Nachricht `1011001`, zehn angehängte Nullen und die tatsächlich geschriebene Binärfolge liefert die einfache GF(2)-Division den Rest `1000101100`, nicht `0110100011`. Diese Nachrechnung prüft ausschließlich das historische Lehrbeispiel, keine standardisierte TETRA-CRC-10-Variante.

**E15 – Interleaver nicht auf ein einziges Matrixmodell reduzieren.** Der gelesene Code enthält sowohl eine arithmetische Blockpermutation `1 + ((a*i) mod k)` als auch Matrixtransposition. Eine Matrixpermutation ändert die Reihenfolge, nicht die Bitanzahl. Die Reparatur eines Burstfehlers ist nicht garantiert; FEC-Leistung hängt unter anderem von Fehlerverteilung, Codierung und verbleibender Information ab.

### 9.5 Modulation, Zeitbasis und Informationsgrenzen

**E16 – DQPSK-Mapping ist in der geprüften Norm eindeutig.** A02, Abschnitt 5.4 und Tabelle 5.1, gibt an:

| Bitpaar | Phasenänderung |
|---|---:|
| 00 | +π/4 |
| 01 | +3π/4 |
| 11 | −3π/4 |
| 10 | −π/4 |

Die Rekursion lautet `S(k)=S(k−1)·exp(j·Δφ(k))`. Die beiden alternierenden Vierpunktmengen liegen auf **demselben Kreis**, nicht auf zwei Kreisringen oder in unterschiedlichen Ebenen. Eine Menge enthält die Achsenpunkte, die andere die Diagonalpunkte. Die Darstellung muss Symbolzeitpunkte von einer gefilterten zeitkontinuierlichen Wellenform unterscheiden.

**E17 – Konstante Symbolbeträge sind keine konstante HF-Hüllkurve.** A02, Abschnitt 5.5, definiert die Überlagerung pulsgeformter Symbole mit einem Root-Raised-Cosine-Spektrum und Roll-off 0,35. Aus `|S(k)|=1` folgt nicht, dass die Summe der zeitlich überlappenden Pulse jederzeit Betrag 1 hat. Aussagen wie „nie Nulldurchgang“, „immer konstante Amplitude“ oder pauschale Eignung stark nichtlinearer Endstufen sind deshalb zu revidieren.

**E18 – IQ-Beispiel nachgerechnet.** Bei Startphase π/4 und Bitpaaren `00, 10, 11` ergeben sich nacheinander π/2, π/4 und −π/2, also ungefähr `(0,1)`, `(0,7071,0,7071)` und `(0,−1)`. Die historischen Koordinaten in Kapitel 95 und Teile von 104 passen nicht dazu. Bei Implementierungen sind zusätzlich die normierte Startreferenz, Bitreihenfolge und I/Q-Vorzeichenkonvention zu beachten.

**E19 – Grundraten und Zeitbezüge.** Für den betrachteten π/4-DQPSK-Pfad ergeben sich aus 18 000 Symbolen/s und zwei Bit/Symbol 36 000 Modulationsbit/s. 255 Symbolintervalle ergeben rund 14,1667 ms, vier Slots rund 56,6667 ms und 18 Frames 1,02 s. Die frühere Angabe „2550 Bits pro Slot“ ist damit nicht vereinbar. Ebenso sind 36 kbit/s Trägerbruttorate, nutzbare Kanalbits und Anwendungsgoodput nicht gleichzusetzen.

**E20 – Nyquist-Faktor und Bandbreitenbezug.** Kapitel 103 vermischt Abtasttheorem, ISI-freie Übertragung und die gesamte RF-Kanalbreite. Aus 25 kHz RF-Kanalabstand folgt nicht ohne weitere Definition „maximal 50 kSym/s komplexe QPSK“. Mit RRC-Roll-off 0,35 und 18 kSym/s beträgt die ideale vollständige spektrale Breite `(1+0,35)·18 kHz = 24,3 kHz`. Diese Modellrechnung erklärt einen Bezug zur 25-kHz-Kanalisierung; sie ersetzt keine Messung der Spektralmaske. A02, Abschnitt 5.5.

Shannon-Hartley muss mit linearem SNR, eindeutigem Bandbreitenbezug und dem passenden Kanalmodell verwendet werden. Ein hoher theoretischer AWGN-Grenzwert beweist weder reale Netzeffizienz noch Verfügbarkeitsreserven oder einen bestimmten TEDS-Goodput.

**E21 – TEDS-Zahlen und Wellenform.** A02, Abschnitte 5.8–5.18, beschreibt QAM-Subträger mit 4-/16-/64-QAM, 2 400 Symbolen/s pro Subträger und **8/16/32/48 Subträgern bei 25/50/100/150 kHz**. Die historischen Zahlen 48/96/192/288 sind falsch. Eine pauschale Gleichsetzung mit gewöhnlichem OFDM ist ohne präzise Wellenformdefinition nicht zu übernehmen. Bruttowerte einer Modulationskombination dürfen nicht als garantierte IP-Nettoraten erscheinen. Das QAM-Fehlerschutzkapitel besitzt außerdem eigene PCCC-Regeln; ein einziger FEC-Text deckt nicht alle TETRA-Modi ab.

### 9.6 Dienste, Status und Kapazität

**E22 – Status, SDS und Sprachdaten trennen.** A02 unterscheidet TNSDS-STATUS und TNSDS-UNITDATA sowie D-STATUS und D-SDS-DATA. Das Pre-coded-status-Feld ist 16 Bit lang. Daher ist „Status = SDS Type 1“ keine korrekte allgemeine technische Definition. Das Mikrofonbeispiel in Kapitel 61 darf ohne zusätzliche, ausdrücklich beschriebene Anwendung nicht von ACELP-Sprachdaten in eine SDS-Textnachricht springen. Quelle A02, Abschnitte 13.3.2.1, 14.5.5 und Tabelle 14.14; ergänzend A04/A08.

**E23 – BOS-Statusmatrix nicht verwenden.** Die in Kapitel 83 ausgegebene Tabelle vertauscht beziehungsweise erfindet zentrale Bedeutungen. Sie ist als Schulungs- oder Einsatzgrundlage zu sperren. Eine korrigierte Matrix muss anhand der tatsächlich einschlägigen offiziellen Organisations-/Landesvorgaben erstellt werden; technische Statuswerte, Tastenbelegung, Leitstellenannahme und Rückmeldung sind getrennte Ebenen. Dieser Archivlauf erfindet keine bundesweit universelle Ersatzmatrix.

**E24 – IP über TETRA nicht aus SDS-Type-3-Segmenten erfinden.** Kapitel 53 nennt nicht belegte 255-Byte-Segmente, „bis 2 kB“ und einen pauschalen SDS-IP-Tunnel. Für eine Fortsetzung sind normativer Paketdatenpfad, Anpassungsschicht, Bearer, Fragmentierung und PEI/TNP getrennt von anwendungsspezifischen SDS-Tunneln zu beschreiben. Referenzen A01, A02, A08. Keine Ableitung einer implementierten IPv6-/TEDS-Unterstützung aus der bloßen Erwähnung im Buch.

**E25 – Rohslotzahl ist nicht Sprachkapazität.** Die Formel `BTS × Carrier × 4` zählt zunächst physische Slotressourcen. Kontrollkanalbelegung, Datenreservierungen, lokale Belegung und Gruppenrufe über mehrere Zellen fehlen. Ein üblicher Zwei-Träger-Fall mit einem reservierten Kontrollslot wäre beispielsweise mit sieben statt acht unmittelbar verfügbaren Traffic-Slots zu betrachten; die konkrete Konfiguration kann hiervon abweichen. Netzwerkweit belegte Kanalressourcen sind nicht gleich viele unabhängige Gespräche. Erlang-B-Ziele für SDS dürfen nicht ohne passendes Warteschlangen-/Zugriffsmodell übernommen werden.

**E26 – Repeater/DAS schaffen nicht automatisch neue Netzkapazität.** Ausleuchtung, HF-Verteilung und eigenständige Basisstationsressourcen sind zu unterscheiden. Die historischen „Type-1 = Frequency Shift / Type-2 = Same Frequency“-Zuweisungen dürfen nicht als TETRA-DMO-Repeaterklassifikation verwendet werden. Die genannten Produktbeispiele müssen auf Funkstandard und konkrete Funktion geprüft werden. Auch die Behauptung, jedes Gateway erhalte AIE/E2EE unverändert, ist unhaltbar ohne Angabe seiner Terminierungspunkte.

### 9.7 QoS, Resilienz, Betrieb und Praxis

**E27 – BER/FER allein liefert keine vollständige QoS.** Vor-/Nach-FEC-BER, Framefehler, Frame-Erasure/Bad-Frame-Indicator, Paketverlust, Blockierung, Jitter und Latenz sind unterschiedliche Messgrößen. Die Rechnung `FER=1−(1−p)^N` setzt unabhängige Bitfehler mit Wahrscheinlichkeit p am betrachteten Block voraus. Sie ist keine universelle Umrechnung einer Demodulator-BER in FER nach FEC.

Als rein mathematisches, ungeschütztes Beispiel mit N=456 ergeben sich für p=10⁻⁴ etwa 4,4578 % und für p=10⁻³ etwa 36,6331 %. Die Zahl 456 ist dabei **kein bestätigter TETRA-Sprachblock**. Aus einer angenommenen BER und einem nicht gemessenen SNR folgen keine Aussagen über „90 % der Randbereiche“. Pauschale ETSI-Qualitätsklassen A–D, feste 4-%-Betriebsfähigkeitsgrenzen und universelle MOS-Zuordnungen wurden im historischen Chat nicht belegt.

**E28 – Redundanz und Fallback sind keine Unausfallbarkeit.** „TETRA fällt nicht aus“, „Notrufe funktionieren immer“, „Jamming nur lokal und leicht umgehbar“ oder „kritische SDS laufen immer durch“ sind nicht als technische Garantien verwendbar. Backhaul, Energie, Frequenzressourcen, lokale Rufsteuerung, Schlüsselzustand, Netzzugang und Nachversorgung sind eigene Ausfallpfade. A09 nennt ausdrücklich mögliche SCK-Nutzung beim Rückfall einer normalerweise Class-3-Zelle; das widerspricht einem pauschalen Modell „lokale Kc immer weiter“. Konkrete Fallbackfunktionen hängen von System und Konfiguration ab.

**E29 – SLA-, Energie- und Kostenwerte sind unbelegt.** 99,999 %, <300 ms, feste Failoverzeiten, 5–15 Minuten USV, 24–72 Stunden Batterie, 5–20 kVA Diesel, feste Wartungsintervalle, pauschale Mobilfunkausfälle nach 30–60 Minuten und behauptete 40-%-Verbesserungen durch KI wurden nicht durch Messung oder verlässliche Quellen dieses Chats gestützt. Sie dürfen allenfalls nachträglich als klar gekennzeichnete Annahmen oder belastbare Betreiberwerte erscheinen. Gleiches gilt für Gerätepreise, Campus-CAPEX/OPEX, Gebühren und „TEDS immer lizenzpflichtig“.

**E30 – Bedien- und Sicherheitskapitel benötigen eine gesonderte Fachprüfung.** Nicht übernehmen: PTT mehrfach kräftig betätigen, unkontrollierte Resets/Codeplugwechsel im Einsatz, Ortung pauschal zur Akkuschonung abschalten, aus IP67 eine feste Trocknungszeit ableiten oder aus ATEX eine besondere Kryptohärtung folgern. Das sind keine bestätigten Workarounds. Bedienhinweise müssen gerätebezogen sein und operative Vorgaben beachten.

**E31 – Reale Großlagen und internationale Netze nicht aus generischen Maßnahmen rekonstruieren.** EM, G20, Loveparade, Ahrtal und nationale Netze wurden als reale Beispiele angekündigt, aber mit weitgehend allgemeinen technischen Maßnahmen beschrieben. Es fehlen belastbare Belege, welche Funktechnik, Kapazitätserweiterung, Schlüsselkonfiguration oder Ausfallursache bei welchem Ereignis tatsächlich vorlag. Insbesondere dürfen spätere TETRA-Konzepte nicht rückwirkend historischen Veranstaltungen zugeschrieben werden. Diese Fallstudien sind Recherchekandidaten, keine abgeschlossenen Einsatzanalysen.

**E32 – Vergleichskapitel neutralisieren.** TETRA versus DMR/P25/NXDN/MCX braucht gemeinsame Vergleichsbedingungen: Frequenz, Sendeleistung, Antennen, Kanalbelegung, Dienst, Sicherheitsprofil, Infrastruktur und Messverfahren. „TETRA alternativlos“, „P25 nur Fläche“, „DMR nicht sicherheitskritisch“, fixe Reichweitenrangfolgen und ein pauschales MCX-Latenz-/Notstromdefizit sind so nicht belegt. Auch vier Slots pro 25 kHz gegenüber zwei pro 12,5 kHz beweisen für sich keinen doppelten spektralen Kapazitätsvorteil.

### 9.8 Standards, Verzeichnisse und Quellen

**E33 – Kapitel 105 enthält falsche Normzuordnungen.** Die Vorworte der beigefügten EN-300-392-Dokumente benennen die Familie konsistent:

| Teil/Familie | Gegenstand für die Korrektur |
|---|---|
| 300 392-1 | General network design |
| 300 392-2 | Air Interface, nicht die Abkürzung AIE für die gesamte Schnittstelle |
| 300 392-3-x | Interworking at the Inter-System Interface |
| 300 392-4 | Gateways basic operation |
| 300 392-5 | Peripheral Equipment Interface |
| 300 392-7 | Security |
| 300 392-9 | Allgemeine Anforderungen an Zusatzdienste |
| 300 392-10/11/12 | Zusatzdienste Stage 1/2/3 |
| 300 392-14 | PICS; vorhandener Anhang historischer Entwurf |
| 300 392-15 | Frequenzbänder, Duplexabstände, Kanalnummerierung |
| 300 392-16 | Network Performance Metrics |
| 300 394 | Konformitätsprüfung; vorliegend Teil 1 Radio |
| 300 395-2 | TETRA-Sprachcodec und zugehörige Sprachkanalcodierung |
| 102 361 | DMR; nicht TEDS |

EN, TS, ES, TR und ETS sind zu unterscheiden, ebenso Version, Datum, Entwurfsstatus und normative/informative Abschnitte. Eine EN ist nicht allein aufgrund ihres Präfixes für jeden Betreiber unmittelbar ein Gesetz. Die Prüfung der tatsächlichen rechtlichen Einbindung bleibt gesondert. Quellen A01–A19 und W06.

**E34 – Abkürzungen nicht durch erfundene Füllbegriffe vervollständigen.** Zu prüfen sind insbesondere MCC versus MCCH, CMCE, doppelte Bedeutungen von ISI, HF im deutschen beziehungsweise englischen Gebrauch sowie fehlende zentrale Kürzel wie DCK, SCK, TEI, ITSI, TMO, LLC oder PEI. „BURST“ und „YAGI“ sind nicht einfach Abkürzungen desselben Typs. „Knob-Model“, „Witschleife“, pauschal BOS-spezifische ZAC-/ZSK-/ZTD-Angaben und weitere Randbegriffe sind ohne belastbaren Beleg zu entfernen beziehungsweise als offen zu markieren. Quellen für TETRA-Begriffe sind die Abkürzungsabschnitte der einschlägigen Normen, nicht die selbst erzeugten Glossare.

**E35 – Literatur und Links sind Kandidaten, keine fertig verifizierte Bibliografie.** Die ältere Liste umfasst 53 Einträge aus Büchern, Standards, Webseiten, Whitepapers, Artikeln und Projekten. Mehrere Angaben sind unvollständig oder mit unsicheren Nummern/Versionen versehen. In den sichtbaren historischen Antworten ist die behauptete Netz-/Internet-Archive-Recherche nicht dokumentiert. Weder „50 Bücher“ noch „alle Links dauerhaft verfügbar“ darf als Ergebnis behauptet werden. Die spätere DAMM-Adresse enthält sogar ein Leerzeichen. Quellenpflege muss systematisch neu erfolgen.

## 10. Zusätzlich geprüfter heutiger Repository-Stand

### 10.1 Methode und Grenzen

Die folgende Prüfung bezieht sich ausdrücklich auf den Lesestand `4e8a633e821b21440057672e8f6dc622fbb64119` in `Archiving`. Die GitHub-Codesuche wurde nur zur Dateisuche benutzt; sie lieferte Treffer des Default-Branches, die relevanten Inhalte wurden anschließend erneut mit dem festgehaltenen Commit gelesen. Ein Suchtreffer des Default-Branches ist damit nicht stillschweigend Beleg für den Archivbranch.

Die Verzeichnis-/Tree-Ausgaben waren teilweise umfangreich und in der Anzeige gekürzt. Die Prüfung ist eine gezielte Stichprobe, kein vollständiger Audit des gesamten Repositorys. Eine Titelsuche nach dem rekonstruierten Buchtitel ergab keinen Treffer; das beweist nicht die Abwesenheit jedes möglicherweise anders benannten Manuskripts.

### 10.2 Gelesene Dateien und Befunde

| Ref. | Datei | Nachgewiesener Befund | Grenze |
|---|---|---|---|
| R01 | `README.md` | Projektbeschreibung v1.9.0, Alert-Service und zentrale SIP-Anbindung mit lokalem Asterisk-Fallback beschrieben | README-Aussage, kein Live-Test |
| R02 | `crates/tetra-entities/src/lmac/components/crc16.rs` | 0x1021, Init 0xffff, erwarteter Rest 0x1d0f, MSB-first | Funktion gelesen, nicht in diesem Auftrag kompiliert |
| R03 | `crates/tetra-entities/src/lmac/components/convenc.rs` | Getrennte `ConvEncState`- und `SpeechConvEncState`-Implementierungen, vier Speicherbits | Keine vollständige Testvektorabnahme |
| R04 | `crates/tetra-entities/src/lmac/components/interleaver.rs` | Blockpermutation und Matrixtransposition mit inversen Funktionen | Roundtrip-Tests vorhanden, nicht ausgeführt |
| R05 | `crates/tetra-entities/src/lmac/components/errorcontrol_params.rs` | Kanalbezogene Größen und CRC16-Flags; mehrere Zweige `unimplemented!()` | Aussage nur über diese Dispatch-Funktion, nicht jede mögliche Implementierung des Repositorys |
| R06 | `crates/tetra-entities/src/lmac/components/errorcontrol.rs` | Steuerpfad und spezialisierter Sprach-UEP-Pfad mit 432 Bit, CRC/Parität und 24×18-Interleaving | Keine vollständige Norm-/On-Air-Zertifizierung |
| R07 | `Docs/BACKEND_WEBUI_SERVICE_MATRIX.md` | Dienste und Oberflächen beschrieben; zahlreiche OPEN-LAB-Ausnahmen ausdrücklich dokumentiert | Dienstbeschreibung nicht mit getesteter Produktivreife gleichsetzen |
| R08 | `system-backend/kmf/README.md` | Lifecycle-KMF, WebUI 8190, Shadow/Authoritative, explizite Labor-/OTAR-Grenzen | Keine echte TA-/D-OTAR-Air-Implementierung laut Dokument |
| R09 | `system-backend/kmf/src/crypto.rs` | Konkreter Labor-Envelope `lab_sha256_stream_mac_v1`, `SealedBlob`, lokale Zufallsbytes, private Dateierzeugung | Kein AES-/HSM-/TETRA-Zertifizierungsbeleg |

Die Datentypen und Pfade zeigen einen real vorhandenen Softwarestand. Sie belegen nicht, dass dieser Stand im historischen Buchchat entstanden ist oder aktuell auf Jans Anlagen läuft.

### 10.3 Wichtige Abweichung: vorhandener PHY-/LMAC-Code statt falscher Buchformeln

`SpeechConvEncState` verwendet die in Abschnitt 9.3 genannten Sprachgeneratoren. `encode_tp` trennt die Klassen 102/112/60 Bit, ergänzt Prüfbits und Tail, punktiert zu 102/168/162 Bit und ruft `matrix_interleave(24,18,...)` auf. `decode_tp` führt den Gegenweg aus. Der Code ist damit an wesentlichen Stellen spezifischer und mit der geprüften Norm konsistenter als die historischen Buchentwürfe.

**Folgerung für eine Fortsetzung:** Nicht den Code auf das falsche 456-Bit-Schema zurückbauen. Zuerst die Dokumentation mit Norm, tatsächlichem Funktionsaufruf und unabhängigen Testvektoren in Einklang bringen.

Gleichzeitig wurden konkrete Prüf- und Kommentarprobleme gefunden:

- Ein Kommentar bei `ConvEncState` spricht von Rate 1/2, obwohl `encode` vier Ausgänge pro Eingangsbit erzeugt.
- Die generische TCH/S-Parametertabelle nennt 288 Type-2-Bits und kommentiert „274 + 4 tail + 10 padding; No CRC“. Der spezialisierte Sprachpfad verwendet dagegen die 286-Bit-Struktur einschließlich Sprach-CRC/Parität. Das Flag `have_crc16=false` bedeutet nicht „überhaupt keine Sprachprüfung“.
- `encode_rate1_3` der allgemeinen Encoderklasse übernimmt nur drei Steuerkanalgeneratoren; der tatsächlich gelesene Sprachpfad verwendet die separate Sprachklasse. Der Hilfsfunktionsname allein ist kein Nachweis des richtigen Sprachcodes.
- Beim gelesenen `blk_num != 1`-Pfad wird der volle Block codiert und die zweite 216-Bit-Hälfte zurückgegeben. Seine Übereinstimmung mit dem gesonderten Frame-Stealing-Schema ist als offener Testpunkt zu behandeln.
- Die gelesene Sprachdecodierung bildet bereits entschiedene Bits auf −1/+1 und Erasures auf 0 ab. Das beweist nicht die lückenlose Weitergabe echter I/Q-Konfidenzwerte eines Soft-Demappers.
- `get_params` enthält nicht implementierte Zweige unter anderem für Tch24, Tch48 und Tch72. Keine generelle Vollständigkeitsbehauptung über alle Datenbearer aus der Buchbeschreibung ableiten.

Diese Befunde sind **neue Roadmap-Kandidaten**. In diesem Auftrag wurden keine Codekommentare, Funktionen oder Tests außerhalb des Archivs verändert.

### 10.4 Wichtige Abweichung: KMF vorhanden, aber ausdrücklich Laborbetrieb

R08 beschreibt CCK/GCK/SCK, Versionen, Kryptoperioden, Rotation, vorbereitete OTAR-Jobs, nodegebundene Transportprofile, Backups, Audit und WebUI. R09 belegt dazu tatsächlich einen Labor-Envelope im Quelltext.

Die dokumentierten Grenzen sind entscheidend:

```text
lab_file_vault                 != HSM
lab_sha256_stream_mac_v1        != zertifiziertes Produktionsverfahren
vorbereiteter OTAR-Job          != D-OTAR-PDU auf der Luftschnittstelle
verschiedene Actor-Namen        != echte Vier-Augen-Authentifizierung
OPEN LAB                       != produktive PKI/RBAC/TLS-Absicherung
```

Laut KMF-README sind noch keine TETRA-TA-Algorithmen und keine D-OTAR-Air-Interface-PDUs implementiert. Der Modus `shadow` gibt keine Edge-Aktion frei; `authoritative` erlaubt das Beanspruchen entsprechend freigegebener Jobs durch das passende Node. Das ist nicht gleichbedeutend mit erfolgreicher Schlüsselinstallation in einem realen Funkgerät.

**Historisch unbestätigt bleiben daher:** Watchtower als produktive NetCore-KMF, ein HSM-Cluster, vollwertiges OTAR auf Funk, zertifizierte AES-/TEA-Interoperabilität, automatisches sicheres Remote-Wipe und pauschale CPU-/Latenzwerte.

## 11. Relevante Dateien, Schnittstellen, Parameter und Pfade

### 11.1 Protokoll- und Diensteebenen

Als redaktionelle Themen wurden TETRA AI, PHY/MAC/LLC, MM, CMCE/SDS, TMO/DMO, ISI, PEI, SIP/PSTN/ISDN, IP, MQTT/IoT, Logging und Krypto behandelt. Das ist keine Liste freigegebener NetCore-Protokollimplementierungen.

Die beigefügte EN 300 392-3-8 beschreibt ISI-Sprachtransport unter anderem über E1 mit LAPF/HDLC sowie über RTP/UDP bei IP-Transport. EN 300 392-5 behandelt PEI einschließlich AT-Kommandos und TNP1. Diese Referenzen ersetzen die ungenauen allgemeinen „REST/SNMP in allen Schichten“-Behauptungen aus den früheren Release-Kapiteln. Quellen A05/A08.

### 11.2 Aktuell gelesene Kanalparameter des Steuerpfads

Die Bezeichnungen in R05 sind `type1_bits`, `type2_bits`, `type345_bits`, `interleave_a`, `have_crc16`.

| Kanal | Type 1 | Type 2 | Type 3/4/5 | a | CRC16 |
|---|---:|---:|---:|---:|---|
| BSCH | 60 | 80 | 120 | 11 | Ja |
| SCH/HD, auch STCH/BNCH in der Tabelle | 124 | 144 | 216 | 101 | Ja |
| AACH | 14 | 30 | 30 | 0 | Nein; eigener Codierungspfad |
| SCH/F | 268 | 288 | 432 | 103 | Ja |
| SCH/HU | 92 | 112 | 168 | 13 | Ja |

Für **Sprache** nicht einfach diese Steuerkanalformeln übernehmen; dort gilt der separat gelesene UEP-Pfad. Der widersprüchliche TCH/S-Kommentar ist in Abschnitt 10.3 festgehalten.

### 11.3 KMF-Schnittstellen aus dem geprüften Repository

| Gegenstand | Gelesener Stand |
|---|---|
| WebUI-Port | 8190, laut KMF-README |
| Konfigurationsbeispiel | `system-backend/kmf/config/kmf.example.toml` laut Verzeichnisbeschreibung; keine produktive Datei ausgelesen |
| Service-Unit | `system-backend/kmf/systemd/netcore-kmf.service` laut README |
| Betriebsmodus | `[policy] operating_mode = "shadow"` beziehungsweise `"authoritative"` |
| Status und Inventar | `GET /api/v1/status`, `GET /api/v1/keys` |
| Schlüssel-Lifecycle | `POST /api/v1/keys`, `POST /api/v1/keys/{id}/rotate`, `POST /api/v1/keys/{id}/activate` |
| Nodes | `POST /api/v1/nodes` |
| OTAR-Jobsteuerung | `POST /api/v1/otar/jobs`, `POST /api/v1/otar/jobs/{id}/approve`, `POST /api/v1/otar/jobs/{id}/queue` |
| Edge-Aktionshülle | `POST /api/v1/edge/actions/claim`, `POST /api/v1/edge/actions/{id}/ack` |
| Backup / Export | `POST /api/v1/backups`, `GET /api/v1/export.json` |
| Schlüsseldateien | Keine Rohschlüssel oder konkreten Geheimnisinhalte übernommen |

Diese Endpunkte wurden **gelesen, nicht auf einem laufenden Dienst aufgerufen**. Es wurden keine Schlüssel erzeugt, verteilt, rotiert, freigegeben oder gelöscht.

### 11.4 Historische generische Port- und Betriebsangaben

Die Logging-Entwürfe nennen Syslog und Port 514 sowie SNMP, TLS, JSON/XML, PCAP, SIEM und UTC/NTP. Daraus folgt keine aktuell konfigurierte Syslog-Transportart, TLS-Absicherung oder Aufbewahrungsfrist in NetCore. Ein Portplan, produktive IPs, Zugangsdaten oder verifizierte LXC-Zuordnungen wurden in diesem Buchchat nicht erarbeitet.

## 12. Befehle, ausgeführte Arbeit und Reparaturstatus

| Aktion | Status in diesem Auftrag | Ergebnis / Grenze |
|---|---|---|
| Repository-Metadaten und `Archiving` lesen | Erfolgreich ausgeführt | Branch und Lesekommit festgestellt |
| Archivverzeichnis und README lesen | Erfolgreich ausgeführt | Vorhandene Einträge müssen erhalten bleiben |
| Neuen Zielpfad auf Existenz prüfen | Erfolgreich ausgeführt | Zum Prüfzeitpunkt nicht vorhanden; keine fremde Zusammenfassung als Ziel gewählt |
| Lokalen Branch klonen | Fehlgeschlagen | Container konnte `github.com` nicht auflösen |
| GitHub-Connector als Lese-/Schreibweg | Lesen erfolgreich; Archivspeicherung über Git-Objekte vorgesehen | Kein lokaler Clone erforderlich |
| PDF-Inventar mit Dateigröße, Seiten und SHA-256 | Erfolgreich ausgeführt | 25 Dateien erfasst |
| Ausgewählte Normstellen mit Files lesen | Erfolgreich ausgeführt | Teilweise Seitenbildlücken; keine Gesamtauswertung aller Seiten |
| Zusätzliche lokale PDF-Seitenbilder erzeugen | Erfolgreich ausgeführt | Nur Referenzprüfung, keine historischen Chatbilder |
| Arithmetik-/Polynomchecks | Erfolgreich ausgeführt | Abschnitt 13; keine Funksystemtests |
| Funkgeräte, TBS oder Dienste konfigurieren | Nicht ausgeführt | Nicht Gegenstand der Archivierung |
| Historische Canvas-Kapitel heute verändern | Nicht ausgeführt | Nur Historie ausgewertet |

Der tatsächlich versuchte Clone-Befehl lautete:

```bash
git clone --depth 1 --single-branch --branch Archiving \
  https://github.com/JanHG98/netcore-tetra.git /mnt/data/netcore-archiving
```

Er endete mit `Could not resolve host: github.com`. Dies war ein Problem des Container-Netzzugangs, kein Nachweis fehlender GitHub-Berechtigung. Der verbundene GitHub-Connector konnte dieselben Repository-Ressourcen lesen und bietet separate Schreibaktionen.

Folgender Befehl wurde lediglich in R08 als Hersteller-/Projektanleitung gelesen und **nicht ausgeführt**:

```bash
sudo system-backend/kmf/install/install.sh
```

Es gibt im sichtbaren historischen Buchchat keine belastbar erfolgreich ausgeführten Installations-, Deployment- oder Reparaturbefehle für NetCore. Generische Beispielabläufe in den Kapiteln dürfen nicht nachträglich in ein Installationsprotokoll umgedeutet werden.

## 13. Tests und Prüfergebnisse einschließlich Grenzen

### 13.1 Tatsächlich durchgeführt

| Prüfung | Ergebnis |
|---|---|
| Anzahl bereitgestellter Referenzdateien | 25 PDFs |
| Umfang der 24 Einzel-PDFs | 3 961 Seiten |
| Umfang `ETSI.pdf` | 4 100 Seiten |
| Summierte Dateiseiten | 8 061; **keine 8 061 verschiedenen Normseiten**, da Sammeldatei und Einzeldateien überlappen |
| Dateiidentität | SHA-256 pro Datei berechnet; Tabelle in Abschnitt 16 |
| Codecbitrate | 137 / 0,03 = 4 566,666… bit/s |
| Normaler Sprachblock | 274 Quellbits; 286 einschließlich CRC/Parität/Tail vor UEP; 432 nach Schutz/Punktierung |
| Matrix | 24 × 18 = 432 |
| DQPSK-Bruttorate | 18 000 × 2 = 36 000 bit/s |
| Zeitraster-Rechnung | Slot 14,1667 ms; Frame 56,6667 ms; Multiframe 1,02 s |
| RRC-Modellbreite | 1,35 × 18 kHz = 24,3 kHz |
| Historisches CRC-Lehrbeispiel | Polynomdarstellung und behaupteter Rest widersprüchlich; Details E14 |
| Historisches IQ-Lehrbeispiel | Phasenrekursion und ausgegebene Koordinaten widersprüchlich; Details E18 |
| Norm-/Codevergleich | Getrennte Sprach-/Steuerkanalencoder und 432-Bit-Sprachpfad im Code vorhanden |
| KMF-Sicherheitsgrenze | Labormodus und fehlende TA-/D-OTAR-Air-Implementierung ausdrücklich dokumentiert |

Die Arithmetik wurde lokal mit Python gerechnet; PDF-Seitenzahlen und Hashes wurden aus den vorhandenen Dateien ermittelt. Das ist eine reproduzierbare Dokumentationsprüfung, aber keine vollständige unabhängige Implementierung der jeweiligen ETSI-Verfahren.

### 13.2 Nur im Quelltext vorhandene Tests

R04 enthält `test_block_interleave_roundtrip` mit k=10/a=3 und `test_matrix_interleave_roundtrip` mit einer 4×3-Matrix. Ihre Existenz ist überprüft. **Sie wurden in diesem Auftrag nicht ausgeführt.** Ein Roundtriptest allein beweist zudem nicht die richtige Interoperabilität mit einem unabhängig implementierten Encoder/Decoder.

### 13.3 Nicht durchgeführt

Kein Cargo-Build, keine Unit-/Integrationstests des gesamten Workspace, keine aktuellen CI-Ergebnisse als bestanden behauptet, kein Codec-Referenzvektorvergleich, keine BER-/FER-Kurve, kein SDR-/HF-Mitschnitt, kein Spektraltest, keine echte SDS-Zustellung, kein Notruf-/Handover-/Fallbackversuch, kein AES-/TEA-/OTAR-Interoperabilitätstest, keine BDBOS-/TCCA-Zertifizierung und kein Word-/Canvas-Fontrenderingtest.

## 14. Verbleibende Ideen, Wünsche und Roadmap-Kandidaten

### 14.1 Bereits ausdrücklich verlangte, weiterhin offene Arbeit

| ID | Aufgabe | Status | Abhängigkeit / Fertigkriterium |
|---|---|---|---|
| B01 | Vollständigen ursprünglichen Kapitelplan wiederherstellen | Beschlossen/geplant, unvollständig | Nutzerexport oder nachweisbarer Originalplan; keine geratenen Titel |
| B02 | Zugängliche Canvas-Kapitel einzeln sichern und konsolidieren | Beschlossen/geplant | Textkörper je ID, eindeutige Nummer, keine verlorenen Vorgänger |
| B03 | Cambria wirklich im späteren Buchsatz verwenden | Beschlossen/geplant | Word-/PDF-Formatvorlagen und Sichtprüfung; nicht bloß HTML-Span |
| B04 | Kapitel 65 mit belastbarer TEA-Zuordnung berichtigen | Mehrfach ausdrücklich verlangt | Algorithmus, Nutzerkreis, aktuelle Lizenzlage und Sicherheitsbefund getrennt belegen |
| B05 | Doppelte Nummern und doppelte Themen bereinigen | Beschlossen/geplant | 59, 89/90, 104, 116/117; QoS-/Literaturentwurf nicht verlieren |
| B06 | Literaturrecherche mit Ziel ungefähr 50 geeigneter Bücher | Beschlossen/geplant | Tatsächliche Bücher identifizieren, Ausgaben/Identifier prüfen; andere Quellentypen separat zählen |
| B07 | Wissenschaftliche Bibliografie herstellen | Beschlossen/geplant | Durchgängiger Zitierstil, präzise Normversionen, DOI/ISBN oder verifizierte Verlags-/Katalogbelege |
| B08 | Abkürzungen und Glossar A–Z vervollständigen | Beschlossen/geplant | Nur belegte Begriffe; vollständiger Alphabet-/Duplikat-/Kontextcheck |
| B09 | FAQ deutlich ausbauen | Beschlossen; Entwurf vorhanden | Mit freigegebenen Fachkapiteln konsistent und ohne absolute Garantien |
| B10 | Inhaltliche Doppelungen vermeiden | Laufende redaktionelle Vorgabe | Überblickskapitel und Deep Dives haben unterschiedliche Lernziele |

### 14.2 Neue Kandidaten aus dieser Archivprüfung, noch nicht als Implementierung beschlossen

| ID | Kandidat | Vorgeschlagene Priorität |
|---|---|---|
| Q01 | Sicherheit, Statusmatrix und operative Bedienhinweise zunächst mit Warnstatus versehen und fachlich neu prüfen | P0: vor jeder Schulungs-/Publikationsfreigabe |
| Q02 | Kapitel 56, 94–100 und 104 aus einem gemeinsamen, normreferenzierten Signalpfad neu ableiten | P0: falsche Zahlen dürfen nicht als Codevorlage dienen |
| Q03 | Normenlandkarte Kapitel 105 und alle davon abhängigen Querverweise reparieren | P0 |
| Q04 | Veraltete Kommentare in `convenc.rs` und TCH/S-Parametertabelle abgleichen | P1; außerhalb dieses Archivauftrags |
| Q05 | Normal-/Frame-Stealing-Pfade mit unabhängigen Codec-/Kanaltestvektoren prüfen | P1 |
| Q06 | Echte Softbit-Konfidenz vom Demapper bis zum Decoder gegen den aktuellen ±1-/Erasure-Pfad abgrenzen | P1 |
| Q07 | KMF-Laborhülle, Authentifizierung, HSM-/Vault-Grenze und echter On-Air-OTAR-Baustein als getrennte Pakete beschreiben | P1; vorhandene KMF weiterverwenden statt Parallelneubau behaupten |
| Q08 | Datenbearer-/Capability-Matrix aus tatsächlichen Funktionspfaden ableiten | P1 |
| Q09 | BER/FER-/Latenz-/Kapazitätsbeispiele mit Annahmen, Messpunkten und Einheiten versehen | P1 |
| Q10 | Vergleichskapitel und Tabellen ohne unbelegte Technologie-Rangfolgen neu aufbauen | P1 |
| Q11 | Reale Fallstudien EM/G20/Loveparade/Hochwasser und internationale Netze recherchieren | P2, aber keine ungesicherten Fallbehauptungen veröffentlichen |
| Q12 | Hersteller-, Regulierungs-, Kosten- und Linkdaten mit Datum/Quelle prüfen | P2 |
| Q13 | Manuskriptprüfung automatisieren: Nummerierung, Querverweise, Begriffe, Einheiten, Quellenlücken | Idee / P2 |
| Q14 | Herkunftsmatrix Kapitel → Normabschnitt → Codepfad → Test aufbauen | Idee / P1 |

P0/P1/P2 sind **hier vorgeschlagene Arbeitsprioritäten**, keine nachträglich erfundenen früheren Termin- oder Sprintzusagen. Eine operative Projektroadmap außerhalb des Archivs wurde nicht verändert.

### 14.3 Kleine Nebenideen aus dem Gespräch

Als optionale, nicht abschließend beauftragte Erweiterungen bleiben erhalten: kommentierte Literaturliste und Top-5-Einstieg, TEA-Härtungs-/Migrationscheckliste, vertiefte KMF-/OTAR-Abläufe, Jamming-Signaturen und Fake-BS-Diagnostik mit Grenzen, Blackout-/Fallback-Szenarien, Indoor-Brandschutz/EMV und objektspezifische Randbedingungen, Zubehörpflege und Beschaffungskriterien, trellisbezogene Viterbi-Erklärungen, freie Distanz und Softbit-Quantisierung, IQ-Cheat-Sheet, RRC-/Spektrenvertiefung, MOS-Bezug und QoS-Klassen, Digital Twins, Edge-/SDN-Architektur sowie vorsichtig eingeordnete Zukunftsthemen.

Die wiederholten Assistentenangebote „300+ Abkürzungen“, „höchste Sicherheit“, „sofort nächstes Kapitel“ oder zusätzliche Kapitelnummern gelten nicht als Nutzerentscheidung. Maßgeblich ist eine belegte inhaltliche Lücke, nicht eine möglichst große Zahl.

## 15. Konkrete nächste Schritte

1. **Quellbestand sichern:** Originalgliederung und erreichbare Einzel-Canvas-Texte exportieren. Die historische gemeinsame Canvas-Datei nicht als vollständiges Hauptbuch behandeln. Fehlende Kapitel/Versionen ausdrücklich auflisten.
2. **Nummerierung festlegen:** 59 sowie 89/90 übernehmen; 104 dem Sprachrufbeispiel zuordnen; QoS-Thema separat erhalten; Konflikt 116 Literatur/Links und ursprüngliches 117 gemeinsam entscheiden.
3. **Fachliche Referenzbasis fixieren:** Für jedes Kapitel die konkrete Normfassung, Klausel und den Status bestimmen. Entwürfe aus 2026 nicht als bereits endgültige Normausgabe deklarieren.
4. **P0-Korrekturen bearbeiten:** Security/TEA/Auth, Status-/Bedienhinweise, Normenfamilie und Sprach-/PHY-Rechenkette zuerst. Danach Glossar, Tabellen und FAQ aus den korrigierten Hauptkapiteln nachziehen.
5. **Repository-Abgleich getrennt führen:** Allgemeines Lehrbuch und NetCore-Implementierungsstand in getrennten Aussagen halten. Vorhandene Encoder-/KMF-Bausteine nicht durch alte Lehrbeispiele ersetzen.
6. **Unabhängig testen:** Referenzvektoren statt nur Encoder/Decoder-Roundtrip; normaler Sprachkanal und Stealing gesondert; CRC-Initialisierung/Bitreihenfolge/Residue; interleaverspezifische Referenzpermutationen; tatsächlicher Softbitpfad.
7. **Praxis und Quellen prüfen:** Offizielle Bedien-/Statusvorgaben, Betreiberberichte, Herstellerdaten, Normenstatus, Gebühren-/Regelungsquellen und bibliografische Identifier recherchieren. Rechercheergebnisse mit Abrufdatum dokumentieren.
8. **Publikationsfassung erstellen:** Cambria-Formatvorlagen, einheitliche Tabellen/Mathematik, vollständige Verzeichnisse, Quellenbelege und sichtbare Kennzeichnung von Beispielen. Erst nach inhaltlicher und visueller Prüfung „vollständig“ oder „freigegeben“ verwenden.

Für diese Schritte ist außerhalb des Archivs ein gesonderter Arbeitsauftrag nötig. Diese Archivierung selbst beinhaltet keine stillschweigende Codekorrektur, Kryptofreischaltung, Funkzulassung oder Gesamtrevision des Buches.

## 16. Anhänge: vollständiges Datei-Inventar

Alle folgenden Dateien waren als PDF-Referenzen unter `/mnt/data/<Dateiname>` verfügbar. Diese Containerpfade sind **keine dauerhaften Repository-Pfade**. Die Dateien wurden nicht als Teil dieses Archivauftrags erneut in Git hochgeladen. Stattdessen werden Identität, Umfang und Verwendung bewahrt. Insbesondere wird die 4 100-seitige Sammeldatei nicht als eigenständige Norm mit einem einzigen Titel behandelt.

| ID | Dateiname | Tatsächlich auf dem Titelblatt / Verwendung | Seiten |
|---|---|---|---:|
| A01 | `en_30039201v010601p.pdf` | EN 300 392-1 V1.6.1, 2020-04; General network design | 182 |
| A02 | `en_30039202v030801p.pdf` | EN 300 392-2 V3.8.1, 2016-08; Air Interface | 1445 |
| A03 | `en_3003920303v010301p.pdf` | EN 300 392-3-3 V1.3.1, 2011-11; ISI Group Call | 251 |
| A04 | `en_3003920304v010301p.pdf` | EN 300 392-3-4 V1.3.1, 2010-08; ISI Short Data Service | 28 |
| A05 | `en_3003920308v010401p.pdf` | EN 300 392-3-8 V1.4.1, 2020-04; Generic Speech Format Implementation | 22 |
| A06 | `en_3003920313v010201p.pdf` | EN 300 392-3-13 V1.2.1, 2020-04; transportunabhängiger ISI Group Call | 191 |
| A07 | `en_3003920315v010500a.pdf` | **Draft** EN 300 392-3-15 V1.5.0, 2026-04; transportunabhängiges ISI Mobility Management | 380 |
| A08 | `en_30039205v020701p.pdf` | EN 300 392-5 V2.7.1, 2020-04; Peripheral Equipment Interface | 320 |
| A09 | `en_30039207v030501p.pdf` | EN 300 392-7 V3.5.1, 2019-07; Security | 216 |
| A10 | `en_30039209v010701p.pdf` | EN 300 392-9 V1.7.1, 2020-04; allgemeine Anforderungen Zusatzdienste | 46 |
| A11 | `en_3003921006v010401p.pdf` | EN 300 392-10-6 V1.4.1, 2006-08; Call Authorized by Dispatcher, Stage 1 | 20 |
| A12 | `en_3003921018v010301p.pdf` | EN 300 392-10-18 V1.3.1, 2003-10; Barring of Outgoing Calls, Stage 1 | 17 |
| A13 | `en_3003921101v010201p.pdf` | EN 300 392-11-1 V1.2.1, 2004-01; Call Identification, Stage 2 | 44 |
| A14 | `en_3003921114v010101p.pdf` | EN 300 392-11-14 V1.1.1, 2002-07; Late Entry, Stage 2 | 23 |
| A15 | `en_3003921117v010102p.pdf` | EN 300 392-11-17 V1.1.2, 2002-01; Include Call, Stage 2 | 18 |
| A16 | `en_3003921201v010202p.pdf` | EN 300 392-12-1 V1.2.2, 2007-08; Call Identification, Stage 3 | 56 |
| A17 | `en_3003921216v010400a.pdf` | **DRAFT** EN 300 392-12-16 V1.4.0, 2026-03; Pre-emptive Priority Call, Stage 3 | 67 |
| A18 | `en_30039401v030301p.pdf` | EN 300 394-1 V3.3.1, 2015-04; Conformance testing, Radio | 169 |
| A19 | `en_30039502v010303p.pdf` | EN 300 395-2 V1.3.3, 2025-02; TETRA codec | 94 |
| A20 | `en_300812v020101p.pdf` | EN 300 812 V2.1.1, 2001-12; SIM–ME interface | 156 |
| A21 | `es_20081201v020205p.pdf` | ES 200 812-1 V2.2.5, 2003-12; UICC physical/logical characteristics | 8 |
| A22 | `es_20081202v020401m.pdf` | **Final draft** ES 200 812-2 V2.4.1, 2005-08; TSIM application | 139 |
| A23 | `ets_30039214e01v.pdf` | **Final draft prETS** 300 392-14, 1997-09; PICS proforma | 61 |
| A24 | `ts_10081201v020205p.pdf` | TS 100 812-1 V2.2.5, 2003-10; UICC physical/logical characteristics | 8 |
| A25 | `ETSI.pdf` | Sammeldatei; beginnt mit EN 300 812 V2.1.1; vollständige interne Dokumentgrenzen nicht inventarisiert | 4100 |

### 16.1 SHA-256 zur Wiedererkennung

```text
A01  788722e566098957bea1a9a5096e9dae1eace1da9f5d38c530c80230e5b9b9cb
A02  3f07b1e4ad73fabc16277a900006fc19844b3a882bbc2bc1d74d78f6156daf28
A03  94ca61038b3ff826e6adcfde56e00473dd996e1a47f9f84e7b9d2aa6c3133fd2
A04  8a38cc6238c6da726e4f07d7d377b2c827447a1448a60ee5ef2a126cb3a0ef9d
A05  4d993a25e35da7b1f2291ae281cb3ea5b3a7b702ab7eb3233bdf430a2c70909d
A06  b47d63853f83a6a4adda08c0e325c8ca13d358e2f8840bfba4bce430e600b1bd
A07  e959709667192e22d2dfeea51b48f9a579a4c107d952999056b7fad93a6d2100
A08  10aaf78988ae4080c3dfe90165ee7f4f3fe8eda402e09fc1e6fa0a0ad890758d
A09  df47af9a642b6eba7d7d5cf5fb7aa3f961e9e03bb912316bd2d189ce47344e08
A10  cad45938bcdf2fa4e8063d4aa9e7d7c06c45f729d3c55dc033e00c27af9f2e06
A11  32b6ee43602ef88a0b5c209b1c69ad35ba8b2e660502257c2dfaa6205a346523
A12  4cc10bcd94d39201538639cefb529809729563f13f87cfe6ae52b44c253b0b8a
A13  852c17ea566757e80ecf0d2718ae7839d9e218aa4384b63d1277ed7e25d86c69
A14  ba1882df71dbb84a0c4f0f7dcea1dc8df968044457da03df9290a50e8821df32
A15  69ce800e352b1ecfc337416fd7b54c1bdf2d9270171af519ac2f5acc1ba85ca6
A16  4d5b56a188fb73c417d67b27da619cad2e49efb3b57306fd4ae51e140c018018
A17  c0ee7859f448c4072befc25905c29b751b5151d3590a880a2d654309a3ba6c02
A18  2d9c327e5ccf1476335e1b7a4f58a0ca7afc93911c30e8aad5ed7dc9ccf3276a
A19  ac716ca18082cc0fd2ee768a14f03deb48fa78a77e83751b1a6b319c7fc2106a
A20  196effeaae473a98244c89538e1562daa58bb1d74cbfe7de78e58ceeafbad18b
A21  346dc545049a7397ca86831731bbc05e96f61ff047d6e7f6b7de56da3aa408e9
A22  330f045a908eefd249fd7450e5b71bd08e8a7b20ce5b86dc9b87c9138a5c7268
A23  2b703f3ebb883c90dab40e1b3d1a634a6acf9c9a20d468fd108ea02226c2bf2c
A24  96df75f5ddf1c3ae4ac75f796ee97babca68fa81aca22f39ba0e0658bd57eed1
A25  9434dad1e7bc80ca39b0edadd8e3b9995fda5ae5f5c3dd565af05708d5059e38
```

### 16.2 Bilderstatus

Im anfänglichen Bestand standen ausschließlich die 25 PDF-Dateien zur Verfügung. Im sichtbaren historischen Buchverlauf gibt es schematische Text-/ASCII-Darstellungen und Formeln, aber keine verfügbar gemachten eigenständigen Originalbilddateien dieses Chats.

Für die aktuelle Prüfung wurden fünf Seiten aus A02/A19 lokal gerendert. Diese sind **neu erzeugte Referenzansichten**, keine wiedergefundenen historischen Bilder; sie werden nicht als solche in Git ausgegeben. PDF-Abbildungen wie die Konstellationsdarstellung auf A02 Seite 74 und die Interleaving-Tabelle auf A19 Seite 33 wurden zur Prüfung genutzt und sind über Dokumentidentität und Seite wiederauffindbar.

Daher wurden **keine historischen Originalbilder hochgeladen**, weil keine solchen Dateien zugänglich waren. Falls ein vollständiger Chat-/Canvas-Export weitere Bilder enthält, sind sie unter einem eindeutig diesem Archiv zugeordneten Unterordner von `Docs/archive/` nachzutragen; Quelle, Originaldateiname, Bezugskapitel und Prüfsumme sollten dabei erhalten bleiben. Keine Bilder anderer Projektchats als Ersatz verwenden.

## 17. Literatur- und Linkkandidaten des historischen Chats

Die folgenden Namen sind als **historische Rechercheansätze** erhalten, nicht als jetzt vollständig verifizierte wissenschaftliche Bibliografie:

| Kandidat aus dem Chat | Zweck für die spätere Recherche |
|---|---|
| Dunlop, Girma, Irvine: *Digital Mobile Communications and the TETRA System* | Grundlagen von TETRA und Mobilfunk; Ausgabe/Verlagsdaten prüfen |
| Stavroulakis, Hrsg.: *Terrestrial Trunked Radio (TETRA) – A Global Security Tool* | TETRA-System- und Sicherheitsüberblick |
| Heikkonen, Pesonen, Saaristo: *You and Your TETRA Radio* | Nutzerorientierter Zugang; genauer Untertitel/Ausgabe prüfen |
| Doug Gray: *TETRA – The Advocate’s Handbook* | Historischer Standard-/Marktkontext |
| W. C. Chu: *Speech Coding Algorithms* | Sprachcodierung und Codecs |
| A. Yarali: *Public Safety Networks from LTE to 5G* | Vergleich und Migration kritischer Kommunikation |
| ETSI EN/TS/ES/TR-Reihen | Technische Primärquellen; Version/Status statt unbestimmtem „ff.“ zitieren |
| TCCA Books on TETRA, IOP-Unterlagen und Whitepapers | Literaturzugang, Interoperabilität und strategische Einordnung |
| *All Cops Are Broadcasting: TETRA Under Scrutiny* | Veröffentlichte Sicherheitsanalyse; Metadaten beim Original prüfen |
| OsmocomTETRA, GNU Radio und weitere Softwareprojekte | Weiterführende technische Ressourcen; nicht als freigegebenes Buch-Laborkapitel oder NetCore-Abhängigkeit unterstellen |

Die historische Liste nennt zusätzlich unter anderem IEEE Xplore, SpringerLink, arXiv, ResearchGate, Signal Identification Guide, RadioReference, Herstellerportale und Community-Ressourcen. Diese sind Recherchezugänge mit sehr unterschiedlicher Belegqualität. Eine Portaladresse ist noch kein Beleg für eine konkrete technische Aussage.

Internet Archive war vom Nutzer ausdrücklich als mögliche Suchquelle genannt. Ein erfolgter systematischer Archive-Abgleich, Leih-/Zugriffsstatus oder rechtmäßig verfügbarer Volltextbestand von rund 50 Büchern ist nicht nachgewiesen. Für die spätere Bibliografie sind tatsächliche Bücher separat von Normen, Webseiten, Whitepapers, Abschlussarbeiten und Software zu zählen.

## 18. Quellen der neuen Prüfung und stabile Repository-Referenzen

### 18.1 Externe Quellen, neu geprüft am 2026-10-04

- **W01:** [Crypto Museum: TEA – TETRA Encryption Algorithm](https://www.cryptomuseum.com/crypto/algo/tea/). Vom Nutzer vorgegebener Link. Historische Überblicksseite, zuletzt dort mit 2023-08-12 bezeichnet; nicht als aktuelle Exportgenehmigung oder vollständiger Sicherheitsstand behandeln.
- **W02:** [ETSI TS 101 053-2 V3.1.1 (2022-12), Rules for the management … Part 2: TEA2](https://www.etsi.org/deliver/etsi_ts/101000_101099/10105302/03.01.01_60/ts_10105302v030101p.pdf). Insbesondere Abschnitte 5.1/5.2 und rollenbezogene Nutzungsbedingungen. Geprüfte Ausgabe, keine Behauptung, dass jede heutige Einzelfallfreigabe damit abschließend geklärt ist.
- **W03:** [ETSI/TCCA-Erklärung zu den am 24.07.2023 veröffentlichten Sicherheitsbefunden](https://www.etsi.org/newsroom/news/2260-etsi-and-tcca-statement-to-tetra-security-algorithms-research-findings-publication-on-24-july-2023/). Zeitgebundene Stellungnahme, keine pauschale Immunitätsgarantie.
- **W04:** [ETSI Algorithms and codes](https://www.etsi.org/expertise/algorithms-codes/). Aktuelle Zugangs-/Verwaltungsübersicht; TEA-Familie nicht auf die ursprünglichen vier Varianten als gesamten heutigen Bestand reduzieren.
- **W05:** [USENIX Security 2023: All Cops Are Broadcasting: TETRA Under Scrutiny](https://www.usenix.org/conference/usenixsecurity23/presentation/meijer). Originalpublikationsseite als Einstieg; keine vollständige neue Kryptanalyse in diesem Archivlauf.
- **W06:** [ETSI TS 102 361-1 V2.6.1, Digital Mobile Radio (DMR), Air Interface](https://www.etsi.org/deliver/etsi_ts/102300_102399/10236101/02.06.01_60/ts_10236101v020601p.pdf). Belegt die falsche Zuordnung dieser Reihe zu TEDS im historischen Kapitel 105.

Für die technischen Hauptbefunde wurden vorrangig die konkret bereitgestellten ETSI-Anhänge A01–A25 und die gelesenen Repository-Dateien verwendet. Es erfolgte keine vollständige Aktualitätsprüfung aller weltweiten TETRA-Frequenzen, Gebühren, Herstellerkataloge, Sicherheitsfunde oder Normfassungen.

### 18.2 Repository-Permalinks zum geprüften Lesestand

| Ref. | Permalink | Gelesener Blob-SHA |
|---|---|---|
| R01 | [README](https://github.com/JanHG98/netcore-tetra/blob/4e8a633e821b21440057672e8f6dc622fbb64119/README.md) | `80dbc3d5f5ff94083ed1b90c4d0256d40fb126fe` |
| R02 | [CRC16](https://github.com/JanHG98/netcore-tetra/blob/4e8a633e821b21440057672e8f6dc622fbb64119/crates/tetra-entities/src/lmac/components/crc16.rs) | `e35708a947216f9a251f01e91dd00cc159222946` |
| R03 | [Convolutional Encoder](https://github.com/JanHG98/netcore-tetra/blob/4e8a633e821b21440057672e8f6dc622fbb64119/crates/tetra-entities/src/lmac/components/convenc.rs) | `a623a9f4d17054194c71beddf8591806479123ea` |
| R04 | [Interleaver](https://github.com/JanHG98/netcore-tetra/blob/4e8a633e821b21440057672e8f6dc622fbb64119/crates/tetra-entities/src/lmac/components/interleaver.rs) | `a2ced13e1373e9c4b8fcd601a3162383f09eb798` |
| R05 | [Error-Control-Parameter](https://github.com/JanHG98/netcore-tetra/blob/4e8a633e821b21440057672e8f6dc622fbb64119/crates/tetra-entities/src/lmac/components/errorcontrol_params.rs) | `4d98a9028924c9d9fc2be23093418f7b4b26eb78` |
| R06 | [Error-Control-Pfade](https://github.com/JanHG98/netcore-tetra/blob/4e8a633e821b21440057672e8f6dc622fbb64119/crates/tetra-entities/src/lmac/components/errorcontrol.rs) | `b68b131d225fa53c7deadb5216cf7084aa9a24a5` |
| R07 | [Backend-Service-Matrix](https://github.com/JanHG98/netcore-tetra/blob/4e8a633e821b21440057672e8f6dc622fbb64119/Docs/BACKEND_WEBUI_SERVICE_MATRIX.md) | `29e506d32b491685a77938784de29a4096e2eba6` |
| R08 | [KMF-README](https://github.com/JanHG98/netcore-tetra/blob/4e8a633e821b21440057672e8f6dc622fbb64119/system-backend/kmf/README.md) | `fabd0967cffb7ce9f708d277d42752fa5eb0fd95` |
| R09 | [KMF-Labor-Envelope](https://github.com/JanHG98/netcore-tetra/blob/4e8a633e821b21440057672e8f6dc622fbb64119/system-backend/kmf/src/crypto.rs) | `d022abcd45bb3bf72e0509827b56be3d6b2d1196` |

Es wurden keine historischen Buch-PRs oder angeblichen Feature-Commits nachgetragen. Die genannten SHAs stammen aus tatsächlichen Leseergebnissen dieses Archivierungsauftrags.

## 19. Speicherverfahren, Sicherheit und Abschlussgrenzen

Für den Archivauftrag sind ausschließlich diese beiden Änderungen vorgesehen: die neue fachbuchbezogene Abschlussdokumentation und die zusätzliche Zeile im vorhandenen `Docs/archive/README.md`. Keine bestehende Zusammenfassung eines anderen Chats wird umgeschrieben, keine produktive Roadmap angepasst und kein Branch gemerged.

Die Speicherung erfolgt über GitHub-Git-Objekte mit dem erneut gelesenen Branchkopf als Elterncommit und dem aktuellen Tree als Basis. Die Branchreferenz wird nur mit **Fast-Forward ohne Force** weitergesetzt. Falls zwischenzeitlich ein anderer Chat schreibt, muss der neue Stand erneut gelesen und der Archiveintrag unter Erhalt dieser Änderungen aufgebaut werden. Die erfolgreiche Veröffentlichung ist durch anschließendes Lesen beider Dateien und Kontrolle der geänderten Pfade zu bestätigen; das Ergebnis wird in der Abschlussmeldung festgehalten.

Nicht übernommen werden Passwörter, Tokens, private Schlüssel, produktive Kryptodateien oder fremde personenbezogene Betriebsdaten. Die dokumentierten Schlüsselbezeichnungen sind technische Kategorien, keine Schlüsselwerte. Ein Beispiel-Endpunkt ist keine Zugangserlaubnis zu einem Produktivsystem.

### Schlussbewertung

Der Wert dieses Chats liegt in der breiten Themen- und Kapitelstruktur, den ausdrücklich formulierten Darstellungswünschen und den konkreten Korrekturhinweisen des Nutzers. Sein Textbestand ist jedoch **kein fertig geprüftes Lehrbuch und keine belastbare NetCore-Implementierungsspezifikation**. Für eine sichere Fortsetzung müssen Originalbestand, Normenbelege, Codezustand und Testergebnisse getrennt bleiben.

Die wichtigsten offenen Arbeiten sind die Sicherung der Kapitel, die Nummerierungsbereinigung, die TEA-/Security- und Sprachkanalkorrekturen, die Reparatur der Normenlandkarte sowie eine echte Literatur- und Quellenprüfung. Die vorhandenen NetCore-Bausteine bieten dafür konkrete Vergleichspunkte, dürfen aber weder als Produktivnachweis noch als Begründung für historische unbelegte Aussagen missverstanden werden.
