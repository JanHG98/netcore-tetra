# Abschlussdokumentation: POCSAG versus LoRa – Alarmierung, Argumente und Faktenkorrektur

> **Archivstatus:** Historischer Diskussions- und Dokumentationschat, kein Implementierungs- oder Betriebsnachweis. Die damaligen Assistenzantworten enthalten erhebliche fachliche Fehler. Abschnitt 5 kennzeichnet diese ausdrücklich; Abschnitt 6 enthält eine davon getrennte, bei der Archivierung ergänzte Einordnung. Aus diesem Chat folgt weder eine pauschale Ablehnung von LoRa noch eine freigegebene NetCore-Alarmierungsarchitektur.

## 1. Metadaten und Prüfgrundlage

| Feld | Inhalt |
|---|---|
| Projekt | NetCore-Tetra |
| Thema | Zunächst allgemeine Gegenargumente zu LoRa; anschließend ausdrücklich POCSAG versus LoRa für Alarmierung/Funkruf |
| Ursprünglicher Chattitel | Nicht als UI-Metadatum verfügbar. Die erste sichtbare Nutzerfrage lautet: „Argumente GEGEN LoRa mit denen ich jede Diskussion Gewinne?“ |
| Arbeitsbezeichnung | POCSAG versus LoRa: Alarmierungsargumente und fachliche Korrekturen |
| Chatlink | Nicht verfügbar; kein Link rekonstruiert oder erfunden |
| Erstellungsdatum dieser Zusammenfassung | 2026-10-04 |
| Historische Nachrichtenzeitpunkte | Im unmittelbar verfügbaren Verlauf nicht belastbar ausgewiesen; keine exakten Datierungen aus Hilfskontext übernommen |
| Repository | `JanHG98/netcore-tetra` |
| Geprüfter und alleiniger Schreibbranch | `Archiving` |
| Geprüfter Ausgangscommit | `3aa13c5a6d283325841a262ac93264bfa22ea168` |
| Ausgangs-Tree | `f171847cd9174081a4249e432efec1f92be5b102` |
| Datum des Ausgangscommits | 2026-10-04T01:35:35Z; dies ist ein Repository-Zeitpunkt, nicht das Datum der ursprünglichen LoRa-Diskussion |
| Ausgangscommit-Nachricht | `docs(archive): preserve TETRA textbook chat, standards audit and correction roadmap` |
| Historischer Canvas | Erfolgreich angelegt als „Argumente Gegen Lora“; sichtbarer Erstellungstext im Chat vorhanden |
| Archivdatei | `Docs/archive/2026-10-04_pocsag-versus-lora-alarmierung-argumente-und-faktenkorrektur.md` |
| Archivindex | `Docs/archive/README.md` |
| Publikationsnachweis | Der diese Datei hinzufügende Git-Commit und die anschließende Abschlussmeldung; der Ausgangscommit oben ist ausdrücklich nicht der neue Archivcommit |

Der technische Abgleich ist auf den genannten Snapshot beschränkt. Der Defaultbranch heißt laut Repository-Metadaten `main`; die zunächst verwendete GitHub-Codesuche durchsucht diesen Defaultbranch. Suchtreffer wurden deshalb nicht ungeprüft als Befund für `Archiving` ausgegeben. Der hier beschriebene DAPNET-Code und die Root-README wurden zusätzlich explizit am Ausgangscommit gelesen. [R1–R3]

### 1.1 Evidenz- und Statusbegriffe

- **Idee:** Vorschlag ohne verbindliche Annahme oder Ausarbeitung.
- **Beschlossen/geplant:** Im Chat ausdrücklich gewünschte Arbeit oder Festlegung; noch kein Umsetzungsbeleg.
- **Implementiert:** Ein Artefakt oder konkreter Code ist nachweisbar vorhanden. Bei Quellcode bedeutet dies noch keinen erfolgreichen Build oder Betrieb.
- **Getestet:** Ein benannter Test wurde tatsächlich ausgeführt und sein Ergebnis ist verfügbar.
- **Im Betrieb bestätigt:** Beobachteter Betrieb auf einem identifizierbaren Zielsystem; eine Beschreibung oder ein Codekommentar genügt nicht.

Die Kennzeichnungen **historisch**, **neu recherchiert** und **aus Code abgeleitet** benennen unterschiedliche Quellenebenen. Insbesondere sind die fachlichen Korrekturen dieses Archivs keine nachträglich erfundenen Nutzerentscheidungen.

### 1.2 Zugänglichkeit und Auswertungslücken

Ausgewertet wurden die sichtbaren Nutzer- und Assistenzbeiträge einschließlich des erfolgreichen Canvas-Erstellungsaufrufs sowie der spätere Archivierungsauftrag. Ein aktueller Live-Abruf des Canvas ist nicht verfügbar. Damit ist dessen damaliger Erstellungstext belegt, nicht aber ein möglicherweise außerhalb des sichtbaren Verlaufs geänderter Endstand.

Es stehen 25 projektweit bereitgestellte ETSI-PDFs zur Verfügung. Diese wurden inventarisiert und im maschinell extrahierten Volltext nach den zentralen Vergleichsbegriffen durchsucht. Das ist ausdrücklich keine vollständige fachliche oder visuelle Durcharbeitung aller 8.061 PDF-Seiten. Einzelheiten und Grenzen stehen in Abschnitt 13.

Nicht verfügbar sind ein authentischer ursprünglicher UI-Chattitel, der Chatlink, eine reale Vergleichsanlage, Pager-/LoRa-Gerätedaten, Messprotokolle, Frequenzzuteilungen und Betreiberanforderungen. Weitere versteckte Chatteile werden nicht behauptet. Allgemeine Erinnerungen an andere NetCore-Chats wurden nicht zu Festlegungen dieses Chats umgedeutet.

## 2. Ziel, Ausgangslage und behandelte Themen

Jan wollte zunächst schlagkräftige Argumente gegen LoRa für eine Diskussion. Der erste Auftrag war rhetorisch formuliert und enthielt keinen technischen Anforderungskatalog. Die erste Assistenzantwort lieferte entsprechend eine stark zugespitzte Negativliste. Anschließend wurde deren Übernahme in ein Canvas ausdrücklich gewünscht und ausgeführt.

Die entscheidende spätere Präzisierung lautete: **„es geht bei der diskussion um POCSAG, daher mal bitte da gute argumente“**. Damit wurde der Vergleichsmaßstab auf POCSAG verschoben. Es ging nicht mehr sinnvoll um beliebige IoT-Sensorik, sondern um die Eignung einer Funklösung für Alarmierungsnachrichten und viele Empfänger.

Behandelt wurden Reichweite und Gebäudedämpfung, Sendeleistung, Duty Cycle, Airtime, Gruppenadressierung, Skalierung, Latenz, Zustellzuverlässigkeit, Rückmeldungen, Netzarchitektur, Cloud-Abhängigkeiten, Sicherheit, Interoperabilität, Batterielaufzeit und Kosten. Daneben wurden ein einseitiges Argumentationsblatt, eine Folie, grafische Symbole und eine Ergänzung des bestehenden Canvas vorgeschlagen.

**Nicht behandelt oder festgelegt wurden** ein konkretes LoRa-System, ein LoRaWAN-Profil, ein Mesh-Protokoll, eine POCSAG-Sendeanlage, eine NetCore-Gateway-Spezifikation oder eine Beschaffung. LoRa, LoRaWAN und ein darauf aufgebautes Alarmierungsprodukt dürfen deshalb in der Fortsetzung nicht als dasselbe behandelt werden.

## 3. Historischer Verlauf und tatsächlich entstandene Artefakte

| Schritt | Historischer Inhalt | Nachweis und Status |
|---|---|---|
| H1 | Wunsch nach Argumenten gegen LoRa | Sichtbare Nutzerfrage; Diskussionsziel, keine technische Entscheidung |
| H2 | „Anti-LoRa“-Antwort mit sieben Themenblöcken, Einzeilern, Gegenargumentetabelle und Alternativen | Text vorhanden; Aussagen waren überwiegend unbelegt und teilweise falsch |
| H3 | „bitte im Canvas“ | Expliziter Dokumentationsauftrag |
| H4 | Canvas „Argumente Gegen Lora“ wurde erstellt | Erfolgreiche Werkzeugantwort vorhanden; **Dokument implementiert**, kein Funk- oder Softwaretest |
| H5 | Präzisierung auf POCSAG | Spätere ausdrückliche Nutzerkorrektur; hat Vorrang vor dem allgemeinen Vergleichsrahmen |
| H6 | Zweite Antwort mit zehn Pro-POCSAG-/Contra-LoRa-Blöcken und Vergleichstabelle | Text vorhanden; erhebliche zusätzliche Übertreibungen und Fehler |
| H7 | Angebot, den POCSAG-Vergleich als zweiten Canvas-Abschnitt einzufügen | Nur Assistenzangebot; kein sichtbarer Update-Aufruf und keine nachgewiesene Ausführung |
| H8 | Auftrag zur technischen Archivierung in `Archiving` | Ausdrücklich autorisiert; Dokumentationsänderungen ausschließlich unter `Docs/archive/` |

### 3.1 Gesicherter Canvas-Umfang

Die im Chat sichtbare Erstfassung trägt die Überschrift **„Argumente GEGEN LoRa – Das Anti-LoRa-Arsenal“**. Ihre Abschnitte heißen sinngemäß:

1. Physik und Regulatorik;
2. Kapazität und Skalierung;
3. Latenz, Zuverlässigkeit und Verfügbarkeit;
4. Sicherheit;
5. Netzwerk und Interoperabilität;
6. Datenmengen und Use-Case-Missverständnisse;
7. Wirtschaftlichkeit;
8. „Mic-Drop“-Einzeiler;
9. typische Pro-LoRa-Argumente und Konter;
10. sinnvolle LoRa-Einsatzfälle und Alternativen.

Die Erstfassung enthält insbesondere die problematischen Behauptungen über verdoppelte Airtime durch ACKs, unmögliche Firmwareupdates, praktisch fehlende Mobilität und angeblich grundsätzlich ungeeignete Alarmierung. Der spätere POCSAG-Text ist im sichtbaren Verlauf **nicht nachweisbar in dieses Canvas übernommen worden**. Die frühere Erfolgsmeldung bezieht sich ausschließlich auf dessen Erstellung.

## 4. Endgültige Anforderungen und Entscheidungen

| ID | Anforderung/Festlegung | Status und Begründung |
|---|---|---|
| D1 | Argumentation soll POCSAG als Vergleichsgegenstand berücksichtigen | **Beschlossen:** ausdrückliche spätere Präzisierung durch Jan |
| D2 | Argumentationsmaterial soll im Canvas vorliegen | **Teilweise erfüllt:** allgemeine Erstfassung erstellt; POCSAG-Ergänzung nicht belegt |
| D3 | Historischen Verlauf technisch archivieren | **Beschlossen:** aktueller Auftrag mit Zielpfad und Schreibautorisierung |
| D4 | Historie, heutiger Code und Vorschläge unterscheiden | **Beschlossen:** ausdrückliche Archivierungsanforderung |
| D5 | Vorhandene Archive bewahren; ausschließlich `Archiving` und `Docs/archive/` ändern | **Beschlossen:** keine Freigabe für andere Branches, Roadmap-Dateien oder Produktivcode |
| D6 | Verfügbare Originalbilder mitsichern | **Beschlossen, hier ohne eigenständige Bildquelle:** keine Chatbilder verfügbar; keine Ersatzbilder erzeugt |

**Keine Entscheidung aus diesem Chat:** POCSAG als verpflichtende Primäralarmierung, Abschaffung von LoRa, Einführung eines neuen DAPNET-Dienstes, Verwendung einer bestimmten Frequenz, ein garantierter Alarmierungs-SLA, eine konkrete maximale Latenz oder eine Zulassung für sicherheitskritischen Betrieb.

Das rhetorische Ziel, jede Diskussion zu gewinnen, ist keine Abnahmebedingung. Als neue redaktionelle Konsequenz der Archivprüfung wird die Argumentationsgrundlage auf überprüfbare Anforderungen zurückgeführt; dies ersetzt keine noch ausstehende technische Entscheidung durch Jan.

## 5. Historische Aussagen und Korrekturregister

**Leseregel:** Die linke Spalte dokumentiert Aussagen der damaligen Assistenz. Sie ist keine Empfehlung. „Nicht belegt“ bedeutet nicht automatisch das Gegenteil, sondern dass der Chat keinen belastbaren Nachweis liefert. Die rechte Spalte ist die neue Prüfung vom 2026-10-04.

### 5.1 Regulatorik, Leistungsdaten und Reichweite

| ID | Historische Aussage | Bewertung und Archivkorrektur |
|---|---|---|
| K01 | EU-868 bedeute generell 1 % oder 0,1 % Duty Cycle; LoRa sei deshalb grundsätzlich nicht echtzeitfähig | **Zu pauschal.** Bedingungen hängen von Band, Geräteklasse und Kanalzugangsverfahren ab. Der gesonderte aktuelle Beleg steht in Abschnitt 6.2. [W1] |
| K02 | POCSAG habe grundsätzlich keine Duty-Cycle-Beschränkung | **Unbelegt.** Eine Protokollbezeichnung ist kein Frequenznutzungsrecht. Die reale Zuteilung wurde nicht vorgelegt. |
| K03 | POCSAG sende mit 50–100 W ERP, LoRa mit 14 dBm/25 mW | **Unzulässige Verallgemeinerung.** Es wurden weder Senderdaten noch Zuteilungen verglichen. Die Zahlen sind keine NetCore-Konfiguration. ERP, EIRP und Geräteleistung müssen beim späteren Vergleich getrennt werden. |
| K04 | POCSAG gehe durch Beton, LoRa bleibe an der Wand hängen | **Nicht belastbar.** Es fehlen Linkbudget, Antennen, Gebäudetyp, Empfangsreserve und Messung. Aus der Modulation allein folgt keine solche Rangfolge. |
| K05 | „169 MHz / BOS 4m“ als gemeinsamer Bandhinweis | **Falsch zugeordnet.** Schon die Rechnung aus Wellenlänge = Lichtgeschwindigkeit/Frequenz ergibt bei 169 MHz ungefähr 1,77 m, nicht 4 m. Daraus folgt noch keine Aussage über zulässige Nutzung. |
| K06 | Lizenzierte Nutzung bedeute keinen Fremdfunk; ein beliebiger günstiger Sensor könne das gesamte LoRa-Netz lahmlegen | **Beide Absolute nicht belegt.** Interferenzfreiheit und Gesamtausfall wurden weder gemessen noch aus einem konkreten Kanal-/Netzmodell hergeleitet. Nicht als Planungsannahme übernehmen. |

### 5.2 Übertragungsmodell, Latenz und Kapazität

| ID | Historische Aussage | Bewertung und Archivkorrektur |
|---|---|---|
| K07 | POCSAG sei one-way und liefere deshalb deterministisch erfolgreich an alle | **Falsche Schlussfolgerung.** POCSAG ist unidirektional; aus geplanter Aussendung folgt kein Empfangsnachweis. Ein Mobilfunk-Rückkanal moderner Pager ist eine zusätzliche Technik. [W2] |
| K08 | LoRa müsse jeden Empfänger einzeln bedienen | **Falsch für LoRaWAN als Pauschalaussage.** Multicast-Unterstützung ist dokumentiert, beispielsweise in LoRaWAN 1.0.3 für Class B. Konkrete Geräteunterstützung ist trotzdem zu prüfen. [W4] |
| K09 | POCSAG habe immer weniger als eine Sekunde Latenz | **Falsch als Garantie.** Bereits Präambel, Datenrate und Nachrichtenformat verbrauchen Zeit; Abschnitt 6.3 enthält eine ausdrücklich berechnete Gegenprobe. Warteschlangen wurden historisch ignoriert. [W3] |
| K10 | LoRa habe zwangsläufig Sekunden bis Minuten Verzögerung und immer Cloud-/MQTT-Zwischenstufen | **Vermischung von Funktechnik und Systemdesign.** LoRaWAN-Klassen unterscheiden Empfangsgelegenheiten; aus IP-Anbindung eines Network Servers folgt keine verpflichtende öffentliche Cloud. [W5] |
| K11 | ACKs verdoppelten immer die Airtime | **Nicht hergeleitet.** Datenpaket, ACK und Wiederholungen können unterschiedlich lang sein. Ohne Paketgrößen, Datenraten und Wiederholungsregeln ist der Faktor zwei unbegründet. |
| K12 | POCSAG skaliere linear; LoRa breche exponentiell ein | **Keine belastbare technische Aussage.** Weder Bezugsgröße noch Lastmodell oder Messreihe wurden benannt. Empfängerzahl, Zahl unterschiedlicher Nachrichten und Rückmeldeverkehr wurden verwechselt. |
| K13 | Hohe Spreading-Faktoren blockierten zwangsläufig das Netz; wenige Geräte führten zum Kollaps | **Aus einem realen Trade-off wurde eine unbelegte Ausfallgarantie.** Reichweite, Datenrate und Nachrichtendauer müssen für die konkrete Konfiguration bilanziert werden. [W5] |
| K14 | ADR mache Mobilität praktisch unmöglich; LoRaWAN habe kein Roaming/Handover | **Pauschalaussage widerlegt.** Die LoRa Alliance nennt für Version 1.1 ausdrücklich Handover-Roaming. Das ist kein Nachweis für nahtlose Sprachkommunikation oder jedes Endgerät. [W6] |
| K15 | Firmwareupdates seien unmöglich; Payload grundsätzlich nur wenige Dutzend bis etwa 200 Bytes | **Unmöglichkeitsbehauptung falsch.** Multicast und Firmware-Over-The-Air-Anwendungen werden beschrieben. Historisch wurde Paketgröße mit übertragbarer Gesamtdatei verwechselt; Grenzen bleiben profilabhängig. [W5] |

### 5.3 Sicherheit, Betrieb und Wirtschaftlichkeit

| ID | Historische Aussage | Bewertung und Archivkorrektur |
|---|---|---|
| K16 | Kein Rückkanal bedeute keine Angriffsfläche; POCSAG könne nur abgehört, nicht manipuliert werden | **Zurückzuziehen.** Ein Empfangspfad ist kein Authentizitätsnachweis. Die beschriebene Basisstruktur enthält Fehlerkorrektur; Fehlerkorrektur allein authentisiert keinen Absender. Das ist eine sicherheitstechnische Ableitung, kein ausgeführter Angriffstest. [W3] |
| K17 | AES-128/Join-Prozesse machten LoRaWAN prinzipiell unsicherer als POCSAG | **Nicht belegt.** Die bloße Existenz von Schlüsselmanagement ist kein Sicherheitsmangel. Für beide konkreten Systeme fehlen Bedrohungsmodell, Provisionierung, Updateverfahren und Prüfung. |
| K18 | FSK sei generell robuster, Chirp Spread Spectrum generell empfindlich gegen Überlagerungen und Mehrwege | **Unbelegte pauschale Rangfolge.** Keine vergleichbaren Empfängerdaten oder Störtests vorhanden; kein universelles Urteil archivieren. |
| K19 | Semtech-IP bedeute zwangsläufig unbeherrschbaren Lock-in | **Nur eine zu prüfende Beschaffungsfrage.** Im Chat gab es keine Lieferanten-, Lizenz-, Second-Source- oder Produktlebenszyklusanalyse. |
| K20 | POCSAG sei wartungsarm bis wartungsfrei, brauche keine Sicherheitspatches; Pager liefen generell jahrelang mit einer Batterie | **Nicht belegt.** Geräte, Batterie, Rufaufkommen und Betriebsprofil wurden nicht angegeben. Infrastrukturwartung ist nicht aus one-way ableitbar. |
| K21 | POCSAG sei automatisch billiger, LoRa automatisch eine OPEX-Falle ohne SLA-Anbieter | **Nicht belegt.** Keine Angebote, Betriebskostenrechnung oder Verträge vorhanden. Vorhandene Infrastruktur wurde nur behauptet, nicht für diesen Vergleich nachgewiesen. |
| K22 | LoRa sei nur für Mülltonnen/Feuchtesensoren; kritische Alarmierung sei grundsätzlich ausgeschlossen | **Rhetorik, kein Eignungsnachweis.** Anforderungen und vollständige Alarmkette müssen bewertet werden. Auch POCSAG allein liefert keinen Sicherheitsnachweis für ein Gesamtsystem. |
| K23 | LTE-M, NB-IoT oder RedCap garantierten harte Latenz; HaLow, private LTE/5G und DECT-NR+ seien automatisch deterministische Alternativen | **Nicht belegt und nicht beschlossen.** Die Alternativnamen bleiben historische Ideen, keine freigegebenen Ersatzarchitekturen. |
| K24 | Ein Uplink pro Stunde sei praktisch die Grenze sinnvoller LoRa-Nutzung | **Willkürliches Beispiel, keine technische Grenze.** Es fehlen Payload, Kanäle, Datenrate, Endgerätezahl und zulässige Last. |

Die zugespitzten Sätze „POCSAG sendet, und alle wissen’s“ sowie „Wenn’s egal ist, nimm LoRa“ werden ausdrücklich **nicht** als fachliche Schlussfolgerungen fortgeführt. Die historische Vergleichstabelle mit generellen Aussagen zu Reichweite, Latenz, Sicherheit und Kosten ist durch dieses Korrekturregister als Entscheidungsgrundlage gesperrt.

## 6. Bei der Archivierung ergänzte technische Einordnung

**Status dieses Abschnitts: neu recherchierte Einordnung und eigene Ableitung, nicht historische Projektentscheidung.** Er dient dazu, die Diskussion fachlich sinnvoll fortsetzen zu können, ohne die frühere Polemik unverändert in eine Roadmap zu übernehmen.

### 6.1 Wo belastbare Argumente für ein POCSAG-System ansetzen

POCSAG ist ein Funkrufprotokoll mit Gruppen-Broadcast. Derselbe Ruf an dieselbe Gruppenadresse benötigt nicht pro hörendem Pager eine neue Aussendung. Das ist für eine große Empfängergruppe ein sinnvolles Grundprinzip, aber kein Alleinstellungsmerkmal gegenüber jeder LoRa-Lösung: Multicast muss im Gegenentwurf ausdrücklich mitbewertet werden. [W2, W4]

Bei LoRaWAN Class A folgen kurze Empfangsfenster auf einen Uplink des Endgeräts. Ein spontan eintreffender Serveralarm muss gegebenenfalls warten. Class B eröffnet geplante Empfangsgelegenheiten; Class C hält den Empfänger weitgehend offen und verändert damit die Energiebilanz. Dies sind spezifische Architekturfragen und keine Eigenschaft jedes proprietären LoRa-Protokolls. [W5]

Daraus folgt als **eigene Anforderungsableitung**: Für die Diskussion ist die Frage „Wie schnell erreicht ein ungeplanter Alarm einen batteriebetriebenen Empfänger im ungünstigsten zulässigen Betriebszustand?“ stärker als „Welche Funktechnik ist immer besser?“. Ein Gegnerentwurf, der ausschließlich selten sendende Class-A-Geräte vorsieht, muss diese Lücke erklären. Ein Gegenentwurf mit anderem Empfangsmodell darf nicht mit dem Class-A-Argument widerlegt werden.

Weitere belastbare Prüffragen sind: Gibt es gemessene Innenraumabdeckung? Wie lang ist die letzte Alarmzustellung bei einer Serie verschiedener Alarmgruppen? Was passiert nach Ausfall eines Standorts oder des Backhauls? Welche Versorgung bleibt nach Stromausfall? Wer trägt Wartung und Entstörung? Ein bestehendes POCSAG-Netz kann hier Vorteile haben, **wenn diese konkret nachgewiesen sind**. Für die NetCore-Installation wurde dies in diesem Chat nicht gezeigt.

### 6.2 Korrigierter regulatorischer Bezug

Die am Archivdatum abgerufene BNetzA-Allgemeinzuteilung **Vfg 91/2025** enthält unterschiedliche SRD-Nutzungsbedingungen. Beispielauszug aus PDF-Seite 17, Tabelle, Bandnummern 48 und 54: [W1]

| Frequenzbereich | Maximale Leistung der genannten Tabellenzeile | Arbeitszyklus als dort genannte Alternative zu Anforderungen an Frequenzzugangs-/Störungsminderungstechniken |
|---|---|---|
| 868–868,6 MHz | 25 mW ERP | höchstens 1 % |
| 869,4–869,65 MHz | 500 mW ERP | höchstens 10 % |

Die Tabelle wurde zusätzlich visuell geprüft. Dies ist keine automatische Freigabe für beliebige LoRa-Geräte: Kategorie, Fußnoten, Kanalzugang und Konformität sind mitzulesen. Die Verfügung gewährt keinen allgemeinen Störungsschutz. Der Arbeitszyklus bezieht sich auf Sender und Beobachtungsband; ohne abweichende Festlegung gilt ein gleitender einstündiger Beobachtungszeitraum. [W1]

**Eigene Umrechnung, keine Netzsimulation:** 1 % von 3.600 s sind 36 s; 0,1 % sind 3,6 s; 10 % sind 360 s. Daraus allein folgt weder eine feste Pause nach jeder Nachricht noch eine Zustellgarantie. Die konkret zulässige Konfiguration bleibt offen.

### 6.3 Gegenprobe zur behaupteten POCSAG-Latenz

Raveons technische Beschreibung nennt die Datenraten 512, 1.200 und 2.400 bit/s sowie eine Präambel von 576 Bit. Ein Batch besteht aus einem 32-Bit-Synchronwort und 16 weiteren 32-Bit-Codewörtern, also 544 Bit. Diese Formatangaben werden hier als Herstellerreferenz verwendet, nicht als vollständig geprüfte Normkonformitätsbewertung. [W3, Seiten 1–2]

**Eigene Rechnung für einen neuen Burst mit genau dieser Präambellänge:**

| Datenrate | Nur 576-Bit-Präambel | Präambel plus ein vollständiger 544-Bit-Batch |
|---|---|---|
| 512 bit/s | 1,125 s | 2,1875 s |
| 1.200 bit/s | 0,480 s | ca. 0,9333 s |
| 2.400 bit/s | 0,240 s | ca. 0,4667 s |

Die Werte ergeben sich aus `T = Bitzahl / Datenrate`. Sie sind **keine Messung der Zeit bis zum Benutzeralarm**. Ein Endgerät kann innerhalb eines Bursts früher oder später reagieren; Textlänge, Adressposition, laufende Aussendung, Warteschlange, Wiederholung und Geräteverarbeitung fehlen im Beispiel. Schon die Präambel bei 512 bit/s widerlegt jedoch die universelle historische Aussage „POCSAG immer < 1 s“ für einen so gestarteten Burst.

### 6.4 Was bei einer fairen Kapazitätsrechnung festgelegt werden muss

Als **neuer Arbeitsvorschlag** sind mindestens zu erfassen: Zahl unterschiedlicher Alarmtexte und Zielgruppen, Textlängen inklusive Protokolloverhead, Wiederholungsstrategie, erlaubte gleichzeitige Last, Empfangsmodell, Zahl und Koordination der Sender sowie Rückmeldeverkehr. Eine einzige identische Gruppennachricht an viele passive Empfänger ist ein anderer Fall als viele unterschiedliche Einzelalarme.

Für ACKs ist die Zeit getrennt zu bilanzieren: `T_gesamt = T_daten + T_ack + T_wiederholungen`. Diese Gleichung rechtfertigt keinen konstanten Faktor zwei. Erst eine konkrete Konfiguration erlaubt Aussagen zur maximalen Last. Im ursprünglichen Chat wurde keine solche Konfiguration angegeben.

### 6.5 Sicherheits- und Bestätigungsstufen

Für eine spätere NetCore-Anbindung wird folgende **neue begriffliche Trennung vorgeschlagen**, noch ohne vereinbarten API-Vertrag:

| Stufe | Aussage | Was damit noch nicht bewiesen ist |
|---|---|---|
| Auftrag angenommen | Eingangsservice akzeptiert die Nachricht | Weiterleitung, Aussendung oder Empfang |
| Weitergereicht/eingereiht | Nachgelagerte Software hat den Auftrag erhalten | Erfolgreiche Funkübertragung |
| Ausgesendet | Sender meldet abgeschlossene Aussendung | Empfang und Verarbeitung am Ziel |
| Endgerät bestätigt | Passend korrelierte Geräteantwort liegt vor | Wahrnehmung oder Reaktion eines Menschen |
| Mensch quittiert | Benutzer hat eine definierte Aktion ausgeführt | Tatsächliches Eintreffen oder Erledigung der Aufgabe |

Diese Trennung ist besonders wichtig, weil die heutige DAPNET-Implementierung verschiedene dieser Stufen nicht gleichsetzt; Details folgen in Abschnitt 7. Ein fehlender Rückkanal kann eine Systemanforderung an bestätigte Zustellung nicht erfüllen, indem man bloß die Senderstatistik umbenennt.

## 7. Zusätzlich geprüfter Repository-Stand am 2026-10-04

### 7.1 Prüfumfang und Abgrenzung

Gelesen wurden Branch-Metadaten, Root-Tree, Archivverzeichnis und Archivindex, die Root-README sowie `crates/tetra-entities/src/net_dapnet/mod.rs` in den Zeilen 1–540 am Commit `3aa13c5a6d283325841a262ac93264bfa22ea168`. Die konkrete Zieldatei war vor der Neuanlage nicht vorhanden; der Abruf lieferte HTTP 404.

Die Codesuche nach `POCSAG` fand auf dem Defaultbranch einen Guide-Verweis und das DAPNET-Modul. Die Suche nach `LoRa` ergab dort keine Treffer. Das ist **kein vollständiger Negativnachweis** über sämtliche Dateien, Branches oder Historien. Der Guide-Treffer wurde nicht als abschließend geprüfte Implementierungs- oder Prioritätsaussage übernommen.

### 7.2 Projektkontext aus der Root-README

Die gelesene README beschreibt `v1.9.0` mit NINA/KATWARN, eigenen Warnmeldungen und Funk-/SDS-Korrekturen. Sie nennt `system-backend/alert-service`, den zentralen `system-backend/sip-switch` und einen lokalen Asterisk-Fallback. Dies wird hier ausschließlich als **heutiger Dokumentationsbefund** festgehalten. Diese Komponenten wurden in diesem Archivlauf weder vollständig codegeprüft noch gebaut oder im Betrieb getestet. Sie sind keine Ergebnisse des historischen POCSAG-Chats. [R2]

### 7.3 Tatsächlich vorhandener DAPNET-Baustein

Der Modulheader erklärt ausdrücklich: Der Empfänger verwendet das TCP-Transmitter-Protokoll des DAPNET-RWTH-Cores, **sendet aber kein POCSAG**. Der gelesene Code nimmt Core-Nachrichten an, normalisiert sie und reicht sie in vorhandene FlowStation-/NetCore-Pfade weiter. [R3]

| Element | Geprüfter Codebefund | Evidenzstatus |
|---|---|---|
| Einstieg | `spawn_dapnet_worker(...)` erzeugt den Thread `dapnet-worker` | Quellcode vorhanden; tatsächlicher Aufruf im laufenden System nicht nachgewiesen |
| Konfiguration | `SharedConfig`, `CfgDapnet`, `effective_dapnet()` | Verwendet; konkrete produktive Werte nicht geprüft |
| Verbindung | `std::net::TcpStream`, `BufReader`, Host/Port aus Konfiguration | TCP-Pfad im Modul implementiert; kein TLS-Handshake in diesem gelesenen Pfad sichtbar |
| Lesetimeout | `TCP_READ_TIMEOUT = 30 s` | Codekonstante, keine gemessene Alarmverzögerung |
| Aktivierung | `enabled` und `rwth_core_enabled` bestimmen den Empfangspfad | Codebefund, nicht Live-Zustand |
| Nachrichtenmodell | ID, Rufzeichen, Empfänger, Text, Zeitstempel, optionale Priorität, Nachrichtentyp, Geschwindigkeit, RIC und Funktion | Datentyp vorhanden |
| Textfilter | Weiterverarbeitung des Nachrichtentyps `6`; andere korrekt geparste Typen werden nach ACK ignoriert | Kontrollfluss gelesen |
| Duplikaterkennung | `HashSet` und `VecDeque`, begrenzt über `effective_messages_limit()` | Flüchtiger Worker-Zustand in den gelesenen Strukturen; keine Persistenz dort sichtbar |
| Zielpfade | TETRA-SDS, TPG2200-Call-Out über SDS Type 4, Telegram | Weiterleitungsfunktionen vorhanden |
| Filter je Zielpfad | `sds_allowed_rics`, `callout_allowed_rics`, `telegram_allowed_rics` | Getrennte Prüfungen im Code |
| Telemetrie | `TelemetryEvent::DapnetLog` mit ID, Empfänger, Text, Priorität und Pfadliste | Codebefund; kein Datenschutz-/Logging-Audit durchgeführt |

Die geprüfte Modulversion besitzt den Blob-SHA **`61dfa444d179835a66bf7e1fb93638b8d9c1b5f4`**. [R3]

### 7.4 ACK-Semantik: keine Zustellbestätigung am Pager

In `handle_rwth_message` wird nach erfolgreichem Parsen zuerst ein positives Core-ACK geschrieben. Erst danach folgen Typprüfung, Duplikaterkennung und `forward_message`. Fehler einzelner Weiterleitungsziele werden anschließend protokolliert. Folglich kann ein Core-ACK vorliegen, obwohl keine nachgelagerte Zustellung erfolgreich war. Das ist eine **direkte Kontrollflussableitung**, kein ausgeführter Fehlertest. [R3, Zeilen 240–540]

Auch ein erfolgreicher `tx.send(ControlCommand::SendSds { ... })` oder `SendRawSdsType4` bedeutet in der betrachteten Funktion zunächst die Übergabe an den internen Kanal. Die Funktion wartet dort nicht auf die Quittierung eines Funkgeräts. Die Telegram-Funktion bestätigt die Weitergabe an den Sink, nicht die Wahrnehmung beim Benutzer. [R3]

**Bedeutung für diesen Chat:** Das bestehende Modul darf weder als POCSAG-Sender noch als Beleg für garantierte Alarmzustellung beworben werden. Es ist ebenso wenig ein hier neu entwickelter LoRa-/POCSAG-Konverter.

### 7.5 Schnittstellen und relevante Parameter

| Name/Pfad | Bedeutung im geprüften Bereich |
|---|---|
| `crates/tetra-entities/src/net_dapnet/mod.rs` | DAPNET-Worker und hier untersuchter Kontrollfluss |
| `rwth_core_host`, `rwth_core_port` | Konfigurierbarer TCP-Endpunkt; numerischer Port und produktiver Host nicht verifiziert |
| `rwth_core_callsign`, `rwth_core_authkey` | Login-Konfiguration; **keine Werte oder Zugangsdaten archiviert** |
| `effective_poll_interval_secs()` | Warte-/Wiederanlaufintervall aus effektiver Konfiguration; kein Wert in diesem Lauf bestätigt |
| `forward_sds`, `sds_source_issi`, Zielauflösung | Weiterleitung als `ControlCommand::SendSds`; Gruppen-/Einzelziel über aufgelöste Zielparameter |
| `forward_callout`, `callout_dest_issi`, `callout_source_issi`, `callout_tpg_ric` | Weiterleitung an Call-Out-Ziel über `ControlCommand::SendRawSdsType4` |
| `CALLOUT_TEXT_MAX_CHARS` | Begrenzung auf 80 Zeichen in dieser Call-Out-Aufbereitung; **keine allgemeine POCSAG-Payloadgrenze** |
| Call-Out-Priorität | Wird im gelesenen Pfad auf höchstens 15 begrenzt |
| Call-Out-ID | Im gelesenen Pfad Bereich 0–255 mit Umlauf; keine systemweite Eindeutigkeitsgarantie daraus abgeleitet |
| `DapnetRuntimeStatus` | Zustände und Diagnose, beispielsweise `disabled`, `connecting`, `connected`, `logged in`, `error` |
| `README.md` | Dokumentierter Release-/Warnfunktionskontext; Blob `80dbc3d5f5ff94083ed1b90c4d0256d40fb126fe` |
| `Docs/archive/README.md` vor Ergänzung | Archivindex; Blob `84fbf81ac0b27e1ad9e5ea713fa1610f3d113a09` |

Es wurden keine historischen Betriebssystempfade, Systemd-Units, RIC-/ISSI-Zuteilungen, HF-Frequenzen oder Ports für einen LoRa-/POCSAG-Aufbau festgelegt. Fehlende Werte bleiben offen; insbesondere wurden keine üblichen DAPNET-Ports als vermeintliche lokale Konfiguration ergänzt.

### 7.6 Neu erkannte Prüfaufgaben, keine behaupteten Fehlerbehebungen

Aus dem gelesenen Code ergeben sich mögliche Fortsetzungspunkte: Bedeutung des Core-ACKs im UI verständlich darstellen, Verhalten bei Zielpfadausfall prüfen, Nachrichten-ID-Lebensdauer und Wiederholung nach Neustart bewerten sowie Absicherung der TCP-Verbindung und Umgang mit Telemetrie-Nutztexten dokumentieren. Ob das heutige System diese Punkte an anderer Stelle bereits abdeckt, wurde nicht untersucht.

Es wird **kein behobener Produktfehler** behauptet. Der historische Chat enthielt ohnehin keinen implementierten Funkpfad und keinen Fehlerbericht zu diesem Modul. Seine heutige Existenz ist ein zusätzlich erhobener Repository-Befund und lässt sich aus diesem Chat keinem damaligen Commit oder PR zuordnen.

## 8. Erreichter Entwicklungs- und Betriebsstand

| Gegenstand | Idee | Beschlossen/geplant | Implementiert | Getestet | Im Betrieb bestätigt |
|---|---|---|---|---|---|
| Allgemeines Anti-LoRa-Dokument | Historisch formuliert | Canvas ausdrücklich gewünscht | Canvas-Erstellung belegt | Kein fachlicher Test bei damaliger Erstellung | Nicht anwendbar |
| POCSAG-Vergleich | Im Chat ausgearbeitet | Themenfokus ausdrücklich gewünscht | Als Chattext vorhanden; Canvas-Ergänzung nicht belegt | Keine Vergleichsmessung | Nein |
| Quellenbasierte Korrektur | Neu im Archivlauf | Bestandteil dieser nachvollziehbaren Abschlussprüfung | In dieser Archivdatei dokumentiert | Quellen-/Plausibilitätsprüfung, keine Funkabnahme | Nicht anwendbar |
| NetCore-LoRa-/POCSAG-Sendeintegration aus diesem Chat | Nicht spezifiziert | Nicht beschlossen | Nicht nachgewiesen | Nein | Nein |
| Vorhandener DAPNET-Empfangs-/Weiterleitungscode | Nicht Ergebnis dieses Chats | Historische Beauftragung hier unbekannt | Im geprüften Repository-Snapshot vorhanden | In diesem Lauf nicht ausgeführt | Nicht nachgewiesen |
| Einseitige Folie/Cheat Sheet | Von der Assistenz angeboten | Nicht ausdrücklich beauftragt | Nicht erstellt | Nein | Nicht anwendbar |

**Gesamtergebnis des historischen Chats:** Dokumentationsmaterial und eine fachliche Eingrenzung, aber keine belastbare Auswahlentscheidung. **Gesamtergebnis dieses Archivlaufs:** gesicherte Verlaufseinordnung, Korrekturregister, gezielter Repository-Abgleich und klar begrenzte Fortsetzungsaufgaben.

## 9. Befehle, Arbeitsabläufe und aufgetretene Probleme

### 9.1 Historische Installation und Reparatur

Im ursprünglichen Chat wurden keine Shellbefehle, Installations- oder Deploymentabläufe ausgeführt. Es gab keine Logs eines Pagers, Gateways oder Senders und keine erfolgreiche Fehlerbehebung an einem Zielsystem. Der einzige nachgewiesene historische Werkzeugschritt war die Canvas-Erstellung.

### 9.2 Tatsächliche Arbeiten während der Archivierung

| Arbeitsschritt | Ergebnis | Grenze |
|---|---|---|
| GitHub-Branch, Metadaten, Archivverzeichnis und Index lesen | Erfolgreich über den verbundenen GitHub-Zugang | Snapshotprüfung; erneuter Branchabgleich vor Veröffentlichung erforderlich |
| Root-README und DAPNET-Zeilen 1–540 am festen Commit lesen | Erfolgreich | Kein vollständiges Repository-Audit |
| Projektdateien über Files suchen/listen | 25 projektgebundene PDFs identifiziert | Kein unabhängiger historischer Bildanhang gefunden |
| PDF-Seitenzahlen, Dateigrößen und SHA-256 lokal ermitteln | Erfolgreich für alle 25 Dateien | Dateiintegrität/Inventar, kein fachlicher Volltest |
| PDF-Text mit `pdftotext -layout` extrahieren | Erfolgreich für alle 25 Dateien | Nichttextuelle Figuren werden nicht vollständig durch Textsuche erschlossen |
| Exakte Begriffe `POCSAG`, `LoRa`, `LoRaWAN` im extrahierten Text suchen | Jeweils null Treffer über den gesamten Bestand | Kein Beweis, dass jede verwandte Fragestellung unter anderer Terminologie fehlt |
| BNetzA-Tabelle zu den Beispielbändern lesen | Text und Screenshot von PDF-Seite 17 geprüft | Keine Prüfung einer realen Geräte-/Frequenzzulassung |
| Technische Onlinequellen abgleichen | Verwendbare Primärquellen für Klassen, Multicast, Roaming und POCSAG-Format gefunden | Einige gesuchte Primärdokumente waren nicht direkt abrufbar |

Ein lokaler Clone wurde versucht:

```sh
git clone --depth 1 --single-branch --branch Archiving \
  https://github.com/JanHG98/netcore-tetra.git \
  /mnt/data/netcore-tetra-archive-work
```

**Ergebnis: fehlgeschlagen, Exitcode 128**, mit `Could not resolve host: github.com`. Das betraf den DNS-/Netzzugang des Arbeitscontainers, nicht die Verfügbarkeit des verbundenen GitHub-Zugangs. Die Repository-Arbeit erfolgte deshalb über dessen Lese- und Git-Objekt-Werkzeuge. Ein lokaler CLI-Push wird nicht behauptet.

Die ausgeführte PDF-Textextraktion verwendete pro Datei das Muster:

```sh
pdftotext -layout DATEI.pdf -
```

`DATEI.pdf` ist hier ein Platzhalter für die tatsächlich verfügbaren PDFs, keine zusätzliche Datei. Die Prüfung erfolgte ohne OCR. Der lokale Arbeitsbericht `attachment-inventory.json` diente der Inventarisierung; sein Containerpfad ist kein produktiver NetCore-Pfad.

### 9.3 Fachliche Fehlerursache und funktionierende Abhilfe

Die Fehler der alten Antworten entstanden erkennbar auf Argumentationsebene: ein vorgegebenes Pro-/Contra-Ergebnis wurde über technische Einschränkungen gestellt, Eigenschaften verschiedener Protokollschichten wurden vermischt und hypothetische Worst Cases als allgemeiner Normalfall ausgegeben. Dies ist eine Bewertung der sichtbaren Antworttexte, keine Behauptung über unbekannte interne Entstehungsvorgänge.

Die hier angewandte Abhilfe besteht aus expliziter Rücknahme unzutreffender Absolutaussagen, Primärquellenabgleich und Trennung von Systemanforderungen, Funkverfahren, Nutzungsrechten und tatsächlichem Code. Die alte Canvas-Fassung ist dadurch **nicht automatisch technisch korrigiert**; deren Überarbeitung bleibt gesondert offen.

## 10. Tests, Ergebnisse und Grenzen

Tatsächlich durchgeführt wurden Dokumenten-/Metadatenprüfung, gezielte Quellcodelektüre, maschinelle Anhangssuche und die regulatorische Tabellenkontrolle. Die Zahlen in Abschnitt 6.3 sind nachvollziehbare Rechenbeispiele, nicht experimentelle Testergebnisse.

**Nicht durchgeführt:** Rust-Build, `cargo test`, CI-Abnahme, gestarteter DAPNET-Worker, TCP-Testserver, LoRaWAN-Join, OTA-Update, Störfestigkeitsprüfung, POCSAG-Aussendung, Empfangsmessung, Pager-Quittierung, Lasttest, Ausfalltest oder produktiver Alarm. Auch der vorhandene Quellcode ist kein Beleg, dass diese Tests bestanden wurden.

### 10.1 Vorgeschlagene spätere Abnahmematrix

| Testkandidat | Vorbereitung | Zu erfassendes Ergebnis | Status |
|---|---|---|---|
| Ungeplanter Alarm an ruhenden Empfänger | Definiertes Geräte-/Empfangsprofil | Zeit bis Gerätealarm und Anteil erfolgreicher Empfänger | **Idee**, nicht ausgeführt |
| Alarmserie verschiedener Gruppen | Reales Last- und Textprofil | Letzter erreichter Empfänger, Warteschlange, Wiederholungen | **Idee**, nicht ausgeführt |
| Versorgung in Gebäuden | Festgelegte Messpunkte und Endgeräte | Erfolgsquote und dokumentierte Empfangsreserve | **Idee**, nicht ausgeführt |
| Ausfall von Sender, Backhaul oder Versorgung | Freigegebene Labor-/Testumgebung | Wiederanlauf, verbleibende Abdeckung, Diagnose | **Idee**, nicht ausgeführt |
| DAPNET-Eingang mit fehlendem Zielpfad | Test-Core und kontrollierter interner Empfänger | Core-ACK versus Weiterleitungsfehler sauber getrennt | **Neu aus Code abgeleitete Idee**, nicht ausgeführt |
| Wiederholung, Neustart und Duplikate | Identifizierte Nachrichten-IDs und Lebensdauer | Verlust-/Duplikatverhalten ohne falsche Zustellanzeige | **Neu aus Code abgeleitete Idee**, nicht ausgeführt |
| Sicherheits-/Betriebsabnahme | Verantwortlichkeit, Zugriffs- und Schlüsselkonzept | Nachweis gegen vereinbarte Anforderungen | **Idee**, keine Freigabe für Realalarmierung |

Für keinen Test wurden im Chat numerische Abnahmeschwellen vereinbart. Diese müssen vor der Durchführung festgelegt werden; nachträgliches Anpassen an ein gewünschtes Ergebnis wäre kein belastbarer Vergleich.

## 11. Ersetzte Ansätze und erhaltene Nebenideen

Der allgemeine Angriff auf LoRa ohne Anwendungsbezug wurde durch Jans POCSAG-Präzisierung als Gesprächsrahmen überholt. Die technische Wahl selbst blieb offen. Die Rücknahme der falschen Aussagen stammt aus dieser Archivprüfung und wird nicht als frühere ausdrückliche Nutzerkorrektur dargestellt.

Erhalten bleiben als **Ideen**: ein einseitiges Argumentationsblatt, eine optisch aufbereitete Folie, ein kompaktes Cheat Sheet, eine Grafik zu Sendezeitbudget und Downlink-Flaschenhals sowie ein zweiter Abschnitt im bestehenden Canvas. Das früher vorgeschlagene Jamming-Symbol war lediglich eine Gestaltungsidee; es entstand weder ein Bild noch ein technischer Störversuch.

Die genannten Alternativen LTE-M, NB-IoT, 5G RedCap, Wi-Fi HaLow, private LTE/5G und DECT-NR+ wurden nicht untersucht oder ausgewählt. Sie bleiben nur als damals erwähnte Suchrichtungen erhalten. Ebenso wurde die Aussage, LoRa könne für seltene kleine Sensordaten sinnvoll sein, nicht zu einer bindenden Nutzungseinschränkung oder einem NetCore-Sensorprojekt ausgearbeitet.

## 12. Offene Aufgaben, Roadmap-Kandidaten und nächste Schritte

Die folgenden Prioritäten sind **Vorschläge dieser Abschlussprüfung**, keine bereits früher vereinbarten Liefertermine. Es wurde keine Roadmap außerhalb des Archivs verändert.

| ID | Aufgabe | Status | Vorgeschlagene Priorität / Abhängigkeit |
|---|---|---|---|
| RM-01 | Alte Argumentations- und Canvas-Fassung mit dem Korrekturregister konsolidieren; POCSAG-Fokus sichtbar machen | **Offen; Dokumentationswunsch historisch, konkrete Korrektur neu** | Vor jeder externen Verwendung |
| RM-02 | Gegenstand klären: kommerzieller POCSAG-Betrieb, Amateurfunk/DAPNET, private Alarmierung oder reiner Technikvergleich; Roh-LoRa, LoRaWAN oder anderes Protokoll benennen | **Offen** | Vor Architektur- oder Sicherheitsbewertung |
| RM-03 | Anforderungen an Empfängerzahl, Gruppen, Text, Alarmserie, Maximalverzögerung, Abdeckung, Quittierung und Energieversorgung festlegen | **Idee / noch nicht spezifiziert** | Voraussetzung für einen fairen Vergleich |
| RM-04 | Reale Geräte, Bänder, Nutzungsrechte, Sender-/Empfängerdaten und Betriebsverantwortung dokumentieren | **Offen** | Vor Aussendung und Feldtest |
| RM-05 | Airtime-/Warteschlangenmodell mit konkreten Nutzdaten und ACK-Regeln erstellen | **Idee** | Nach RM-02 bis RM-04 |
| RM-06 | Bestehende NetCore-Warn- und DAPNET-Pfade in einen eventuellen Integrationsentwurf einbeziehen, statt deren Existenz zu übersehen | **Bedingter Kandidat**, kein Neubauauftrag | Nur bei tatsächlichem Integrationswunsch |
| RM-07 | ACK-/Zustellstufen, Zielausfälle, Wiederholung und Duplikate im vorhandenen DAPNET-Pfad testen | **Neu vorgeschlagen** | Reproduzierbare Testumgebung und definierte Semantik |
| RM-08 | Sicherheits-, Protokollierungs-, Wartungs- und Ausfallkonzept für beide konkreten Vergleichssysteme prüfen | **Idee** | Vor Eignungsaussage für kritische Alarmierung |
| RM-09 | Messreihe nach Abschnitt 10.1 durchführen und Ergebnisse versioniert ablegen | **Idee** | Nach festgelegten Abnahmekriterien |
| RM-10 | Quellenbasierte Kurzfassung/Folie mit sachlichen Diskussionsfragen erstellen | **Historische Nebenidee, nicht beauftragt** | Nach RM-01 und vorzugsweise Anforderungsklärung |
| RM-11 | Originalen Chatlink/UI-Titel und einen eventuell abweichenden aktuellen Canvas-Stand nachtragen | **Offene Metadatenlücke** | Nur bei Verfügbarkeit; keine Voraussetzung für Erhalt dieses Archivs |

**Empfohlener nächster fachlicher Schritt:** Zuerst RM-02 und RM-03 bearbeiten. Ohne konkrete Gegenanlage und Erfolgskriterien wäre ein neuer pauschaler Technikvergleich erneut nur Meinung. Bis dahin ist das Archiv eine überprüfbare Gesprächsgrundlage, keine Produktfreigabe.

Für die Diskussion ergeben sich als neue, nicht gemessene Prüfaufforderungen beispielsweise: „Zeig die maximale Alarmverzögerung bei deinem tatsächlichen Empfangsprofil“, „Trenne Gruppen-Broadcast von vielen verschiedenen Alarmtexten“ und „Sag genau, ob dein ACK den Eingang, die Aussendung oder das Endgerät bestätigt“. Diese Formulierungen ersetzen die zurückgezogenen pauschalen Siegesbehauptungen.

## 13. Quellenanhänge und Bildarchiv

### 13.1 Einordnung des bereitgestellten PDF-Bestands

Files meldete **25 Dateien, sämtlich `source_kind=project`**, und keine eigenständigen direkten Chat-Uploads. Der gesamte Bestand war im Arbeitscontainer vorhanden. Er besteht aus TETRA-Normen bzw. Normenentwürfen und einer großen Zusammenstellung. Die Titel/Versionen in der folgenden Tabelle bezeichnen **die bereitgestellten Dateien**, nicht notwendigerweise den heute neuesten oder gültigen Normstand.

| Nr. | Datei | Identität / Themenbereich laut Titel | PDF-Seiten |
|---|---|---|---:|
| A01 | `en_30039201v010601p.pdf` | EN 300 392-1 V1.6.1, 2020-04; General network design | 182 |
| A02 | `en_30039202v030801p.pdf` | EN 300 392-2 V3.8.1, 2016-08; Air Interface | 1445 |
| A03 | `en_3003920303v010301p.pdf` | EN 300 392-3-3 V1.3.1, 2011-11; ISI Group Call | 251 |
| A04 | `en_3003920304v010301p.pdf` | EN 300 392-3-4 V1.3.1, 2010-08; ISI Short Data Service | 28 |
| A05 | `en_3003920308v010401p.pdf` | EN 300 392-3-8 V1.4.1, 2020-04; Generic Speech Format Implementation | 22 |
| A06 | `en_3003920313v010201p.pdf` | EN 300 392-3-13 V1.2.1, 2020-04; transport layer independent ISI Group Call | 191 |
| A07 | `en_3003920315v010500a.pdf` | **Draft** EN 300 392-3-15 V1.5.0, 2026-04; ISI Mobility Management | 380 |
| A08 | `en_30039205v020701p.pdf` | EN 300 392-5 V2.7.1, 2020-04; Peripheral Equipment Interface | 320 |
| A09 | `en_30039207v030501p.pdf` | EN 300 392-7 V3.5.1, 2019-07; Security | 216 |
| A10 | `en_30039209v010701p.pdf` | EN 300 392-9 V1.7.1, 2020-04; General requirements for supplementary services | 46 |
| A11 | `en_3003921006v010401p.pdf` | EN 300 392-10-6 V1.4.1, 2006-08; Call Authorized by Dispatcher, Stage 1 | 20 |
| A12 | `en_3003921018v010301p.pdf` | EN 300 392-10-18 V1.3.1, 2003-10; Barring of Outgoing Calls, Stage 1 | 17 |
| A13 | `en_3003921101v010201p.pdf` | EN 300 392-11-1 V1.2.1, 2004-01; Call Identification, Stage 2 | 44 |
| A14 | `en_3003921114v010101p.pdf` | EN 300 392-11-14 V1.1.1, 2002-07; Late Entry, Stage 2 | 23 |
| A15 | `en_3003921117v010102p.pdf` | EN 300 392-11-17 V1.1.2, 2002-01; Include Call, Stage 2 | 18 |
| A16 | `en_3003921201v010202p.pdf` | EN 300 392-12-1 V1.2.2, 2007-08; Call Identification, Stage 3 | 56 |
| A17 | `en_3003921216v010400a.pdf` | **DRAFT** EN 300 392-12-16 V1.4.0, 2026-03; Pre-emptive Priority Call, Stage 3 | 67 |
| A18 | `en_30039401v030301p.pdf` | EN 300 394-1 V3.3.1, 2015-04; Radio conformance testing | 169 |
| A19 | `en_30039502v010303p.pdf` | EN 300 395-2 V1.3.3, 2025-02; TETRA speech codec | 94 |
| A20 | `en_300812v020101p.pdf` | EN 300 812 V2.1.1, 2001-12; SIM-ME interface / Security aspects | 156 |
| A21 | `es_20081201v020205p.pdf` | ES 200 812-1 V2.2.5, 2003-12; UICC physical and logical characteristics | 8 |
| A22 | `es_20081202v020401m.pdf` | **Final draft** ES 200 812-2 V2.4.1, 2005-08; TSIM application | 139 |
| A23 | `ets_30039214e01v.pdf` | **Final draft prETS** 300 392-14, 1997-09; PICS proforma | 61 |
| A24 | `ts_10081201v020205p.pdf` | TS 100 812-1 V2.2.5, 2003-10; UICC physical and logical characteristics | 8 |
| A25 | `ETSI.pdf` | 4.100-seitige Zusammenstellung; beginnt mit EN 300 812 V2.1.1, enthält weitere TETRA-Dokumente | 4100 |

Die Summe beträgt **8.061 Seiten einschließlich mehrfach enthaltener Dokumente**, nicht 8.061 einzigartige fachliche Seiten. Die vollständige Zusammensetzung und Dublettengleichheit der Sammeldatei wurde nicht rekonstruiert.

Die exakte, groß-/kleinschreibungsunabhängige Wortsuche nach `POCSAG`, `LoRa` und `LoRaWAN` ergab in allen 25 extrahierten PDF-Texten null Treffer. Die semantische Files-Suche lieferte TETRA-Abschnitte, aber keine direkte Vergleichsgrundlage für die damaligen Absolutaussagen. Damit werden diese PDFs **nicht als Quellen für einen Sieg von POCSAG über LoRa verwendet**. Ihre Themen können für andere NetCore-Arbeiten relevant sein, begründen hier aber keine neue Feature-Roadmap.

Besonders zu beachten: Ein PICS-Formular ist kein ausgefüllter Konformitätsnachweis; eine Radio-Testnorm kein bestandenes Testergebnis; die TETRA-Security-Spezifikation kein Audit der NetCore-Installation. Die vorhandenen Entwürfe A07, A17, A22 und A23 werden nicht stillschweigend zu verabschiedeten aktuellen Normen oder zu einer beschlossenen Projektbaseline erklärt.

### 13.2 Bilder und binäre Anlagen

Im sichtbaren historischen LoRa-/POCSAG-Dialog wurden **keine eigenständigen Bilder hochgeladen oder erzeugt**. Die damalige Folien-/Grafikidee wurde nicht umgesetzt. Die bei der Dateibereitstellung dargestellten ETSI-Deckblätter und eingebetteten Abbildungen gehören zu den Projekt-PDFs; sie sind keine Originalbilder dieses Diskussionschats.

Daher wurden für diesen Auftrag keine Bilddateien hinzugefügt, keine Platzhaltergrafiken erzeugt und keine Deckblätter als angebliche Chatbilder exportiert. Die projektweiten Normen wurden inventarisiert, aber nicht als vollständige PDF-Dubletten unter `Docs/archive/` erneut hochgeladen. Sie sind für den konkreten Vergleich keine direkt verwendeten Originalbelege. Auch während der neuen Webprüfung betrachtete Screenshots sind keine historischen Chatassets.

Eine später auftauchende echte Chatgrafik oder ein authentischer Canvas-Export kann diesem eindeutig benannten Archiv nachträglich zugeordnet werden. Ihre gegenwärtige Nichtverfügbarkeit wird nicht durch einen erfundenen Dateilink kaschiert.

## 14. Relevante Quellen, Repository-Bezüge und Versionsgrenzen

### 14.1 Historische Gesprächsquellen

**H1–H8** bezeichnen die in Abschnitt 3 aufgeführten sichtbaren Schritte. Die historischen technischen Aussagen waren nicht mit Primärquellen belegt. Der Canvas-Erstellungserfolg bestätigt ein Dokument, nicht die Richtigkeit seines Inhalts. Es gibt aus diesem Chat keinen nachgewiesenen Codecommit, PR oder Release.

### 14.2 Im Archivlauf tatsächlich gelesene Repository-Quellen

- **R1:** [Geprüfter Ausgangscommit](https://github.com/JanHG98/netcore-tetra/commit/3aa13c5a6d283325841a262ac93264bfa22ea168), zugehörige Branch-/Tree-Metadaten und Archivverzeichnis. Der Commit gehörte vor dieser Arbeit zur Archivierung eines anderen Chats; er wird nicht als POCSAG-Implementierungscommit ausgegeben.
- **R2:** [Root-README am geprüften Commit](https://github.com/JanHG98/netcore-tetra/blob/3aa13c5a6d283325841a262ac93264bfa22ea168/README.md). Dokumentierter Release- und Warnfunktionskontext, kein eigener Laufzeittest.
- **R3:** [DAPNET-Modul am geprüften Commit, Zeilen 1–540](https://github.com/JanHG98/netcore-tetra/blob/3aa13c5a6d283325841a262ac93264bfa22ea168/crates/tetra-entities/src/net_dapnet/mod.rs#L1-L540). Maßgeblicher Codebeleg für Worker, Core-ACK und Weiterleitung.
- **R4:** [Archivindex vor diesem Auftrag](https://github.com/JanHG98/netcore-tetra/blob/3aa13c5a6d283325841a262ac93264bfa22ea168/Docs/archive/README.md). Vorhandene Einträge bleiben erhalten.

Nur als Suchhinweis, **nicht als vollständige Prüfung**: `Docs/NetCore-Tetra-Komplettguide-2026-09-28.md` erschien bei der Defaultbranch-Suche nach POCSAG. Die damalige Suchindex-Version wurde nicht zur Codebaseline dieses Archivs gemacht. Es wurde kein PR erfunden oder ein thematisch benachbarter PR als Ergebnis dieses Chats ausgegeben.

### 14.3 Neue externe Prüfung, Abrufdatum 2026-10-04

- **W1 – Bundesnetzagentur:** [Vfg 91/2025, SRD-Allgemeinzuteilung](https://www.bundesnetzagentur.de/DE/Fachthemen/Telekommunikation/Frequenzen/Allgemeinzuteilungen/_DL/vfg91_2025.pdf?__blob=publicationFile&v=3). Arbeitszyklus-/Nutzungsbedingungen sowie PDF-Seite 17, Bandnummern 48 und 54; Tabelle zusätzlich als Screenshot geprüft. Primärquelle, keine individuelle Frequenzzuteilung für Jan.
- **W2 – Swissphone:** [POCSAG-Technologie](https://www.swissphone.com/de/loesungen/technologien/pocsag/). Verwendet für unidirektionales Funkrufprinzip, Gruppen-Broadcast und zusätzlichen Mobilfunk-Rückkanal. Werbliche Absolutaussagen über praktisch ausgeschlossene Überlastung wurden ausdrücklich nicht übernommen.
- **W3 – Raveon Technologies:** [Technical Brief AN142 Rev A3: The POCSAG Paging Protocol](https://www.raveon.com/pdfiles/AN142%28POCSAG%29.pdf). Seiten 1–2: Datenraten, Präambel und Batchaufbau; Seite 4: Fehlerkorrektur. Herstellerbeschreibung, nicht Ersatz für eine vollständige Norm-/Geräteprüfung. Andere Detailaussagen des Dokuments wurden nicht pauschal übernommen.
- **W4 – LoRa Alliance:** [LoRaWAN Specification v1.0.3](https://lora-alliance.org/resource_hub/lorawan-specification-v1-0-3/). Herausgeberbeschreibung belegt Unicast-/Multicast-Unterstützung für Class B; keine Behauptung, dass 1.0.3 heute die neueste Version sei.
- **W5 – LoRa Alliance:** [What is LoRaWAN Specification](https://lora-alliance.org/about-lorawan-old/). Verwendet für Protokoll-/Architekturabgrenzung, Empfangsklassen, Multicast/FOTA und Datenraten-Trade-off. Aussagen wie „no latency“ oder vollständig interferenzfreie Datenraten wurden nicht als absolute technische Garantie übernommen.
- **W6 – LoRa Alliance:** [LoRaWAN Specification v1.1](https://lora-alliance.org/resource_hub/lorawan-specification-v1-1/). Herausgeberbeschreibung nennt Handover-Roaming, Class B und Security-Erweiterungen; belegt nicht automatisch die Umsetzung in jedem Gerät.

Ein direkter Abruf der zusätzlich gesuchten ITU-Referenz M.584 gelang in diesem Lauf nicht. Semtech-FAQ-Seiten waren teils nur über Suchauszüge erreichbar und wurden bei späterem Direktabruf mit HTTP 403 abgewiesen; die tragenden Korrekturen stützen sich deshalb auf die oben tatsächlich lesbaren Quellen. Diese Lücken werden nicht als erfolgreich gelesene Normtexte ausgegeben.

Die normativen Projektanhänge A01–A25 werden durch Dateiname, Ausgabe und Seitenzahl identifiziert. Ihr aktuellster Veröffentlichungsstatus wurde nicht vollständig online nachverfolgt, weil sie keine direkte Grundlage des LoRa-/POCSAG-Vergleichs bilden.

## 15. Speicherung und Fortsetzungsgrenze

Das Archiv wird als reine Dokumentationsänderung zusammen mit dem bestehenden Index auf `Archiving` veröffentlicht. Der neue Commit baut auf dem unmittelbar vor dem Schreiben kontrollierten Branchstand auf; bei zwischenzeitlichen Änderungen sind Tree und Index neu zu übernehmen. Ein Force-Push, Merge oder eine Änderung an Produktivcode gehört nicht zu diesem Auftrag.

Der Publikationsablauf verwendet die GitHub-Git-Objekte für Blob, Tree und einen Commit mit genau einem Elterncommit sowie eine nicht erzwungene Branchreferenz-Aktualisierung. Das ist eine Veröffentlichung im Remote-Repository, kein behaupteter erfolgreicher lokaler `git push`. Die tatsächliche Commit-ID wird erst vom Schreibwerkzeug vergeben und in der Abschlussmeldung genannt; sie wird nicht vorab in diese Datei erfunden.

**Fortsetzungspunkt:** Die alte Argumentationsfassung ist fachlich überholt. Zuerst den realen Alarmierungsfall und die Gegenanlage spezifizieren, anschließend vorhandene NetCore-Pfade und Nachweise prüfen. Bis dahin sind weder „POCSAG gewinnt immer“ noch „LoRa reicht sicher aus“ durch diesen Chat oder den gezielten Repository-Abgleich belegt.
