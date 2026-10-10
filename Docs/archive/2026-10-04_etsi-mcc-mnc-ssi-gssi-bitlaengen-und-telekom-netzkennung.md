# Brainstorming: ETSI-Bitlängen von MCC, MNC, SSI und GSSI

**Archivhinweis, 09.10.2026:** Diese Datei dokumentiert das im Titel beziehungsweise Quellenrahmen genannte Gespräch und dessen damalige Entscheidungen. Historische Kommandos, Planungen, Versionsangaben und Fehlerbilder bleiben erhalten; der heutige technische Stand steht in der [Gesamtroadmap](../roadmaps/gesamtroadmap.md) und in den [aktuellen Dienstanleitungen](../services/README.md). Die Archivaufnahme bestätigt keine spätere Implementierung oder Live-Abnahme.

**Stand der Notizen und ergänzenden Prüfungen: 2026-10-04.** Historische Entwürfe, nachgewiesene Umsetzung und ausgeführte Tests sind jeweils getrennt gekennzeichnet.

**Ziel:** TETRA-Feldbreiten und Normfundstellen klären sowie Zahlenwert, Netzidentität und Anzeigeformat trennen. Maßgeblich sind MCC 10 Bit, MNC 14 Bit, SSI 24 Bit und TSI 48 Bit; Vergabegrenzen und reservierte Werte bleiben zusätzlich zu beachten.

## 1. Kontext und Geltungsbereich

| Merkmal | Wert |
|---|---|
| Projekt | NetCore-Tetra |
| Thema | TETRA-Adressierung: Feldbreiten, Dezimaldarstellung, Normfundstellen, Telekom-Netzkennung und Zero-Padding |
| Historische Datierung | Einzelne fachliche Klärungsschritte ohne verlässliche Zeitstempel. |
| Erstellungsdatum dieses Archivs | 2026-10-04 |
| Repository | `JanHG98/netcore-tetra` |
| Geprüfter Repository-Branch | `Archiving` |
| Geprüfter Repository-Commit | `9e61695884b6dc90174e7d33dec0afddbd2faae1` |
| Zugehöriger Root-Tree | `15528ed1dd73f66804277f80b04b07ce79f8c217` |
| Archivdatei | `Docs/archive/2026-10-04_etsi-mcc-mnc-ssi-gssi-bitlaengen-und-telekom-netzkennung.md` |
| Archivindex | `Docs/archive/README.md` |
| Art des Ergebnisses | Technische Klärung und Dokumentationskorrektur; kein in diesem Planungsstand durchgeführtes Implementierungs- oder Deploymentprojekt. |

Der geprüfte Commit fixiert den gelesenen Repository-Stand. Die Dokumenthistorie ist separat in Git nachvollziehbar.

Historische Einordnung in Abschnitt 3 und zusätzliche Normen-/Repository-Prüfung vom 2026-10-04 bleiben getrennt. Spätere Codebefunde belegen keine historische Implementierung oder Geräteabnahme.

### Statusbegriffe

- **Idee:** angeboten oder als mögliche Folgearbeit genannt, ohne Umsetzungsauftrag.
- **Beschlossen/geplant:** ausdrücklich beauftragt oder verbindlich festgelegt, aber nicht allein dadurch implementiert.
- **Implementiert:** in einer tatsächlich gelesenen Repository-Datei vorhanden; die Aussage ist auf den genannten Commit und Codepfad begrenzt.
- **Getestet:** eine konkrete Prüfung wurde tatsächlich ausgeführt; Prüfgegenstand und Grenzen sind anzugeben.
- **Im Betrieb bestätigt:** Nachvollziehbarer Lauf oder Endgeräte-/Funkbeobachtung. Ein solcher Nachweis liegt hier nicht vor.

## 2. Ziel, Ausgangslage und behandelte Themen

Zu klären sind ETSI-Feldbreiten für MCC, MNC und GSSI, belastbare Normfundstellen, 24-Bit-SSI, Aktualität von EN 300 392-1 V1.6.1 (2020-04), Telekom-Netzkennung als Vergleich und Bedeutung führender Nullen.

Die für eine Fortsetzung wesentliche Trennung lautet:

1. **Protokollfeld:** Anzahl der übertragenen Bits.
2. **Zahlenwert:** die durch dieses Feld repräsentierte Zahl.
3. **Darstellung:** beispielsweise `1`, `01`, `0001` oder ein mit Nullen aufgefülltes Binärwort.
4. **Zuteilung und Identität:** wer den Wert vergeben darf, welchem Netz er gehört und ob er in einem bestimmten Kontext verwendet werden darf.

Gleiche Zahlenwerte oder ähnlich benannte Felder machen ein öffentliches Mobilfunknetz und ein TETRA-Netz nicht zu demselben Netz. Insbesondere entsteht durch Zero-Padding keine neue Zuteilung.

## 3. Historischer Planungsverlauf

### 3.1 Erste Einordnung der Feldlängen

Die erste Einordnung umfasst MCC mit drei Dezimalstellen und 10 Bit, TETRA-MNC mit 14 Bit und höchstens vier Dezimalstellen sowie GSSI mit 24 Bit. Hauptquelle ist ETSI EN 300 392-1 V1.6.1. Individual- und Gruppenkennungen stammen aus demselben SSI-Adressraum.

**Historisches Ergebnis:** Die grundlegenden Feldbreiten waren richtig. Eine konkrete NetCore-Konfiguration, eine Teilnehmerliste oder ein Nummernvergabeverfahren wurden dadurch nicht festgelegt.

### 3.2 Präzise Normfundstellen

SSI-Länge steht in 7.2.1, Verkürzung GTSI zu GSSI in 7.2.3 und Bitaufbau in 7.2.4. Die anfängliche Zuordnung von MNC-Vergabe und Vier-Dezimalstellen-Grenze zu 7.2.4 war ungenau; maßgeblich ist **7.2.5, Seite 30** der bereitgestellten Ausgabe.

Ein ergänzend erwähnter historischer Verweis auf ETS 300 396-01 wurde für die Abschlussdokumentation nicht als maßgebliche Belegstelle übernommen. Die vorhandene EN 300 392-1 enthält den benötigten MCC-Nachweis selbst.

### 3.3 Bestätigung der 24-Bit-SSI

Bestätigt sind 24 Bit für SSI einschließlich ISSI/GSSI und insgesamt 48 Bit für TSI.

Dabei wurde jedoch ein englischer Satz als wörtliches Zitat aus 7.2.3 dargestellt, der dort in dieser Form nicht steht. Die zutreffende Aussage zur Feldlänge wird durch **7.2.1** getragen; **7.2.3** erklärt die netzspezifische Kurzidentität und ihre Bildung. Das damalige angebliche Direktzitat darf nicht als ETSI-Wortlaut weiterverwendet werden.

Die Aussage zur Vermeidung von Überschneidungen benötigt außerdem den Netzkontext: Es geht um eindeutige Vergabe innerhalb des relevanten TETRA-Netzes, nicht um ein weltweit einmaliges Vorkommen jeder bloßen 24-Bit-Zahl.

### 3.4 Versionsstand von EN 300 392-1

Historisch war keine neuere veröffentlichte EN 300 392-1 als V1.6.1 (2020-04) belegt. Eine umfassende damalige ETSI-Statusauswertung ist nicht dokumentiert.

**Historischer Status:** Aussage zur seinerzeit gefundenen Ausgabe; kein belastbarer Nachweis, dass sämtliche TETRA-Spezifikationen seit 2020 unverändert oder auf demselben Versionsstand sind.

### 3.5 Telekom als Vergleich zum öffentlichen Mobilfunk

Die frühere Telekom-Vergleichstabelle enthielt `262-01`, `262-06`, `262-11` und `262-43` mit teilweise falschen oder unbelegten Telekom-/Congstar-/IoT-Zuordnungen. Abschnitt 6 enthält die am 2026-10-04 überprüften Zuteilungsinhaber.

Zusätzlich wurde behauptet, ein BOS- oder Betriebsnetz könne beispielsweise MNC `9999` unter MCC `262` selbst definieren, solange kein E.212-Registry-Mapping exportiert werde. Diese Begründung wird zurückgezogen: Eine solche Freigaberegel folgt nicht aus der angehängten ETSI-Norm. Diese beschreibt die nationale MNC-Zuteilung.

### 3.6 Führende Nullen

Nullauffüllung wurde zunächst als binäre Festbreitendarstellung eingeordnet und anhand eines Zahlenbeispiels erläutert.

Binäre Feldkodierung und dezimale Anzeige sind getrennte Ebenen. Ein verbindliches UI- oder Importschema ist durch das Beispiel nicht beschlossen.

### 3.7 Noch offene Vertiefungen

Mögliche Vertiefungen sind Zuordnung des konkreten MCC-/MNC-/GSSI-/ISSI-Schemas, Kollisionsprüfung und Bitfeldgrafik. Umsetzung und eigenständiger Arbeitsauftrag fehlen.

**Status: Idee.** Das gezeigte Beispiel ist keine Festlegung, NetCore-Tetra auf die Telekom-Kennung umzustellen.

## 4. Dokumentengeprüfter Normenstand

### 4.1 Maßgebliche Quelle und Prüfverfahren

Die zentrale Quelle ist die bereitgestellte Datei **`en_30039201v010601p.pdf`**, ETSI EN 300 392-1 V1.6.1 (2020-04), *Part 1: General network design*, 182 Seiten. Sie wurde für die einschlägigen Seiten direkt aus der vorhandenen PDF gelesen; Seite 28 wurde zusätzlich gerendert und betrachtet. Der Text der Seiten 27, 29, 30 und 35–37 wurde lokal aus der PDF extrahiert.

SHA-256 der tatsächlich geöffneten Datei:

```text
788722e566098957bea1a9a5096e9dae1eace1da9f5d38c530c80230e5b9b9cb
```

Bei der Dateiabfrage über die indexierte Leseschnittstelle waren nicht durchgängig verwertbare beziehungsweise konsistente Abschnittsinhalte verfügbar. Für die folgenden Fundstellen ist daher die direkt geöffnete PDF maßgeblich, nicht ein automatisch erzeugter Suchauszug. Die Abschnittszuordnung wurde anhand ihrer tatsächlichen Seitenüberschriften abgeglichen. Die PDF selbst war zugänglich; es liegt hier keine fehlende Quelldatei vor.

### 4.2 Verbindliche Fundstellen in der angehängten Ausgabe

| Gegenstand | Fundstelle | Geprüfter Inhalt |
|---|---|---|
| TSI und SSI: Länge | **7.2.1, Seite 27** | TSI 48 Bit; SSI 24 Bit; SSI ist eine Verkürzung der TSI. |
| Gültigkeitsraum | **7.2.1, Seite 27; 7.2.3, Seite 29** | TSI über den TETRA-Gesamtbereich eindeutig; SSI innerhalb eines TETRA-Netzes eindeutig. Derselbe SSI-Zahlenwert kann in verschiedenen Netzen vorkommen. |
| Bildung von ISSI/GSSI | **7.2.3, Seite 29** | ITSI ohne MCC und MNC ergibt ISSI; GTSI wird entsprechend zu GSSI verkürzt. |
| Bitaufbau | **7.2.4, Abbildung 3, Seite 29** | MCC 10 Bit, MNC 14 Bit und SSI 24 Bit. Die Abbildung trägt den Titel *Contents of TSI*. |
| MCC: Dezimalzahl und Kodierung | **7.2.5, Seite 29** | Drei-Dezimalstellen-Landescode nach ITU-T E.218, binär in 10 Bit kodiert. Die undefinierten Werte 1000–1023 sind reserviert und dürfen nicht verwendet werden. |
| MNC: Kodierung, Obergrenze und Vergabe | **7.2.5, Seite 30** | Binärkodierung mit 14 Bit; höchstens 9999 dezimal beziehungsweise 0x270F. Zuteilung durch die nationale Verwaltung, eindeutiger MNC je Betreiber. |
| SSI-Vergabe | **7.2.5, Seite 30** | ISSI, ASSI und GSSI werden vom Netzbetreiber vergeben; ihre eindeutige Zuordnung ist durch diesen sicherzustellen. |
| Verwendung in Adressierung und Routing | **7.2.6, Seite 30** | SSI als untere Luftschnittstellenadresse; TSI als Netzroutingadresse; zusätzliche Regeln für interne und netzübergreifende Rufe. |
| Reservierte Gruppenadresse | **7.7.8, Seite 37** | 24 Einsen als spezielle SSI für Informationsbroadcast an alle MS eines TETRA-Netzes. |

**Präzisierung:** Die Strukturabbildung gehört korrekt zu **7.2.4**. Zu korrigieren ist die frühere Zuordnung der Vergaberegeln, nicht die Abbildungszuordnung.

### 4.3 Feldkapazität ist nicht der frei vergebbare Wertebereich

| Feld | Bits | Rein mathematische Kapazität | Einschränkung für die Interpretation |
|---|---:|---|---|
| MCC | 10 | 0 bis 1023 | Die angehängte Ausgabe reserviert 1000–1023; die bloße Darstellbarkeit ist keine Erlaubnis zur Verwendung. Ein numerisch passender Landescode ist ebenfalls nicht automatisch eine beliebige eigene Netzzuweisung. |
| MNC | 14 | 0 bis 16383 | 7.2.5 begrenzt auf höchstens 9999 und beschreibt die Zuteilung. 10000 passt zwar in 14 Bit, erfüllt aber diese Dezimalgrenze nicht. |
| SSI, darunter ISSI/GSSI | 24 | 0 bis 16777215 | Eindeutige Vergabe und kontextabhängige Sonderwerte beachten. `0xFFFFFF` ist als Gruppenbroadcast reserviert. Diese Tabelle ist keine vollständige Liste sämtlicher SSI-Sonderwerte. |
| TSI | 48 | 2^48 verschiedene Bitmuster | Nicht jedes Bitmuster ist eine regulär zugeteilte Teilnehmeridentität. |

Die Regeln dürfen nicht auf eine bloße Prüfung der Anzahl dezimaler Zeichen reduziert werden. Eine achtstellige SSI ist beispielsweise nicht automatisch zulässig: `16777216` überschreitet bereits das 24-Bit-Feld.

### 4.4 ISSI und GSSI: gemeinsamer Nummernraum mit Netzkontext

7.2.4 beschreibt die netzinterne Aufteilung zwischen ITSI, ATSI und GTSI; 7.2.5 legt die Verantwortung für eindeutige SSI-Zuteilungen beim Netzbetreiber fest. Eine Typkennzeichnung in einer Datenbank ersetzt diese Vergaberegeln nicht. Ein willkürliches Nebeneinander derselben Nummer als Individual- und Gruppenadresse im selben Netz darf nicht allein damit gerechtfertigt werden, dass die Anwendung zusätzlich `Issi` oder `Gssi` speichert.

Umgekehrt ist die gleiche numerische SSI in unterschiedlichen TETRA-Netzen nicht schon für sich eine Kollision. Bei netzübergreifenden Prüfungen muss die vollständige Identität beziehungsweise der MCC-/MNC-Kontext berücksichtigt werden. Außerdem behandelt die Norm Alias- und Migrationsidentitäten; diese dürfen nicht ungeprüft wie gewöhnliche statische ISSIs verwaltet werden.

**Status:** Aus der Quelle abgeleitete Anforderung an eine spätere Vergabe-/Validierungsprüfung; in diesem Planungsstand wurde kein Register geändert und keine existierende Belegung auf Kollisionen untersucht.

## 5. Zero-Padding und das konkrete Rechenbeispiel

### 5.1 Erhaltenes Beispiel

Die folgenden Werte waren ausschließlich ein Lehrbeispiel:

| Feld | Zahlenwert | Binärdarstellung in der TETRA-Feldbreite |
|---|---|---|
| MCC | 262 | `0100000110` |
| MNC | 1; im öffentlichen Telekom-Beispiel als `01` geschrieben | `00000000000001` |
| SSI | `0x123456` = 1193046 | `000100100011010001010110` |

Das zusammengefügte 48-Bit-Wort lautet:

```text
MCC        MNC            SSI
0100000110 00000000000001 000100100011010001010110
```

Als mathematisch gepackte Zahl beziehungsweise in der hier gewählten Big-Endian-Byteansicht ergibt sich:

```text
TSI = (262 << 38) | (1 << 24) | 0x123456
    = 0x418001123456
Bytes: 41 80 01 12 34 56
```

Die Byteansicht ist eine zusätzliche Rechenhilfe dieses Archivierungslaufs. Sie behauptet weder, dass jeder NetCore-Speicherpfad diese Byteordnung verwendet, noch dass ein vollständiges Luftschnittstellen-PDU ausschließlich aus diesen sechs Bytes besteht.

### 5.2 Was mit Null aufgefüllt wird

Bei einer **binären Festbreitendarstellung** bleiben Zahlenwert und Identität durch führende Nullen unverändert; die Darstellung wird lediglich auf die Feldbreite gebracht. `1` ergibt in einem 14-Bit-Feld das oben gezeigte Wort.

Eine **dezimale Anzeige** wie `0001` ist etwas anderes. Aus der maximal vierstelligen TETRA-MNC-Zahl folgt nicht automatisch eine verbindliche vierstellige Anzeige in jedem Dashboard, Export oder Formular. Ein solches Anzeigeformat wurde hier nicht beschlossen.

Auch `01` aus einem öffentlichen Mobilfunk-Nummernplan darf nicht ohne Beachtung des jeweiligen Formats in einen allgemeinen Zahlenparser geworfen und anschließend als vollständig rekonstruierte Netzkennung behandelt werden. In dieser Dokumentation wird lediglich der Zahlenwert 1 für das TETRA-Rechenbeispiel verwendet; eine automatische Konvertierung öffentlicher Netzzuweisungen in TETRA-Zuweisungen ist nicht definiert.

## 6. Zusätzliche öffentliche Quellenprüfung vom 2026-10-04

### 6.1 Telekom: Berichtigung der früheren Zusatztabelle

Die Bundesnetzagentur nennt in ihrer Liste zugeteilter IMSI-Blöcke folgende Zuteilungsinhaber für die im Entwurf angesprochenen Kombinationen:

| MCC–MNC | Überprüfter Zuteilungsinhaber | Bewertung der früheren Einordnung |
|---|---|---|
| `262-01` | Telekom Deutschland GmbH | Grundzuordnung bestätigt. |
| `262-06` | Telekom Deutschland GmbH | Inhaber bestätigt; der speziell behauptete M2M-/IoT-Verwendungszweck ist mit dieser Liste nicht belegt. |
| `262-11` | Telefónica Germany GmbH & Co. oHG | Frühere Zuordnung zu Congstar/Telekom nicht übernehmen. |
| `262-43` | Vodafone GmbH | Frühere Zuordnung zu Telekom Deutschland IoT nicht übernehmen. |

Quelle: [Bundesnetzagentur, zugeteilte IMSI-Blöcke][B1]. Bei der Abfrage unterschieden sich Datumsangaben der Textausgabe und der gerenderten PDF: Text 03.08.2026, Bild 24.02.2026. Die hier verwendeten vier Zuordnungen stimmen in beiden Ansichten überein. Das Dokument belegt Zuteilungsinhaber, nicht den konkreten Dienst oder das tatsächliche Auftreten jedes Codes auf einer bestimmten SIM.

### 6.2 Versionsaktualität von EN 300 392-1

Der [offizielle ETSI-Ausgabepfad][N2] enthält die veröffentlichte V1.6.1-PDF vom April 2020. Im zusätzlich betrachteten [Teil-1-Verzeichnis][N3] wurde keine neuere veröffentlichte Fassung belastbar bestätigt. Ein weiterer Verzeichniseintrag ließ sich nicht auflösen; eine abschließende Auswertung sämtlicher ETSI-Arbeitsstände und Statusdatensätze wurde nicht erreicht.

Die belastbare Aussage lautet deshalb: **V1.6.1 ist die hier direkt geprüfte veröffentlichte Ausgabe; eine neuere Teil-1-Ausgabe ist in diesem Lauf nicht nachgewiesen.** Eine unbedingte Vollständigkeitsgarantie zum aktuellsten ETSI-Arbeitsstand wird nicht abgegeben.

Schon die Anhänge zeigen unabhängig davon, dass nicht „TETRA insgesamt“ auf April 2020 eingefroren ist: Vorhanden sind beispielsweise eine Codec-Ausgabe von 2025 sowie zwei ausdrücklich als Draft gekennzeichnete Dokumente von 2026. Das sind andere Normteile und keine neuen Ausgaben von EN 300 392-1. Der Versionsstatus jedes Anhangs wurde nicht separat online aktualisiert.

## 7. Repository-Abgleich am Dokumentdatum – getrennt vom historischen Ergebnis

### 7.1 Umfang

Gelesen wurden der Branch-/Tree-Stand, der Archivindex sowie ausgewählte Rust-Dateien zur Konfiguration, Adressdarstellung und MLE-Kodierung. Der Ausgangsstand war `Archiving` bei `9e61695884b6dc90174e7d33dec0afddbd2faae1`.

Dies war eine **gezielte statische Prüfung**, kein Audit aller Konfigurations-, UI-, API-, Teilnehmerverwaltungs-, PDU- oder Gerätepfade. Insbesondere wurde nicht aus dem Vorhandensein eines Rust-Typs auf die Vollständigkeit zentraler Eingabevalidierung geschlossen.

### 7.2 Tatsächlich gefundene Implementierungen

| Datei / Symbol | Nachgewiesener Befund | Status und Grenze |
|---|---|---|
| [`crates/tetra-config/src/bluestation/sec_net.rs`][R1], `CfgNetInfo`, `NetInfoDto`, `net_dto_to_cfg` | MCC und MNC werden als `u16` geführt. Kommentare nennen 10 beziehungsweise 14 Bit und D-MLE-SYNC. Die lokale Konvertierungsfunktion übernimmt beide Werte direkt. | **Implementiert, statisch geprüft.** In dieser Funktion selbst keine Bereichsprüfung; vorgelagerte oder anderweitige Prüfungen wurden nicht umfassend untersucht. |
| [`crates/tetra-core/src/address.rs`][R2], `SsiType`, `TetraAddress` | Unterscheidung unter anderem `Issi`, `Gssi`, `Ussi`, `Smi`, `Esi` und `EventLabel`; `TetraAddress.ssi` ist `u32`. `new` übernimmt Wert und Typ, `issi` ist ein Komfortkonstruktor. | **Implementiert, statisch geprüft.** Der Konstruktor begrenzt selbst nicht auf 24 Bit und kontrolliert keine Vergabekollisionen. Daraus folgt nicht, dass sämtliche Aufrufer unvalidiert sind. |
| [`crates/tetra-pdus/src/mle/pdus/d_mle_sync.rs`][R3], `DMleSync::from_bitbuf` / `to_bitbuf` | MCC wird mit `read_field(10, "mcc")` gelesen und mit 10 Bit geschrieben; MNC entsprechend mit 14 Bit. | **Implementiert, statisch geprüft.** Konkreter Encode-/Decode-Code ist vorhanden. Kein Lauf dieses Codes im Archivierungslauf. |

Die D-MLE-SYNC-Struktur enthält daneben Nachbarzellen-/Last-/Late-Entry-Felder. Sie ist keine vollständige TSI mit SSI-Anteil. Der Befund belegt somit die MCC-/MNC-Feldbreiten in genau diesem PDU-Pfad, nicht eine vollständige 48-Bit-TSI-Serialisierung an allen Schnittstellen.

Die Speicherbreiten `u16` und `u32` sind für sich kein Fehler: Software darf Protokollfelder in größeren CPU-Datentypen halten. Zu prüfen ist, ob unzulässige Werte an den richtigen Grenzen zurückgewiesen werden und beim Serialisieren nicht unerkannt verloren gehen. Das Verhalten des gesamten `BitBuffer`- und Konfigurationspfads wurde hier nicht getestet.

### 7.3 Quellenanker des Codeabgleichs

| Datei | Vom Connector gemeldeter Blob-SHA |
|---|---|
| `crates/tetra-config/src/bluestation/sec_net.rs` | `cf9062b069cad9b36dd8efe058cc1cd81cb1e146` |
| `crates/tetra-core/src/address.rs` | `25309c0d55c052ff4e3101cfc813886820db16ad` |
| `crates/tetra-pdus/src/mle/pdus/d_mle_sync.rs` | `71d45365618ced91d9f8907e4f394de67f1b5a2d` |

Zusätzlich wurde ein Anfangsausschnitt von `crates/tetra-core/src/typed_pdu_fields.rs` gelesen. Er enthält generische Hilfsfunktionen und wurde nicht als Nachweis einer zentralen Identitätsvalidierung verwendet.

### 7.4 Was nicht nachgewiesen ist

Codeänderung zur historischen Klärung, installierte NetCore-Version, produktive MCC/MNC, vollständige SSI-Belegung, erfolgreicher Build, Endgeräte-/Funkvergleich und Übereinstimmung von UI, Import, APIs und Encoder sind nicht nachgewiesen.

Die Implementierungen am Prüfstand vom 2026-10-04 sind keiner historischen Fehlerbehebung dieser Klärung zugeordnet. Sie dienen ausschließlich als ergänzender Codebefund.

## 8. Architektur, Schnittstellen und technische Parameter

Die für dieses Thema relevante logische Kette ist:

```text
Normen und Zuteilungsregeln
    -> konfigurierte Netzkennung / verwaltete Teilnehmer- und Gruppenkennungen
    -> interne Datentypen
    -> kontextspezifischer Encoder und Decoder
    -> Luftschnittstelle beziehungsweise weitere Schnittstellen

Anzeige und Export sind gesonderte Darstellungen derselben Werte.
```

`tetra-config`, `tetra-core` und `tetra-pdus` decken unterschiedliche Teile der Kette ab. Eine neue Komponente oder ein Dienst ist hier nicht beschlossen.

| Kategorie | Gesicherter Stand |
|---|---|
| Normative Bezugspunkte | EN 300 392-1: Identitäten, Zusammensetzung, Vergabe und Nutzung; E.218 wird dort für den TETRA-Codeaufbau referenziert. |
| Öffentlicher Mobilfunkvergleich | IMSI-/MCC-/MNC-Nummerierung und Zuteilungsinhaber aus der Bundesnetzagentur-Quelle; nicht mit einer TETRA-Zuteilung gleichsetzen. |
| Im Code geprüfte Funknachricht | `D-MLE-SYNC`; der Code verweist auf Abschnitt 18.4.2.1. |
| Relevante Repository-Pfade | Die drei in Abschnitt 7 geprüften Rust-Dateien; außerdem ist `config.toml` im Root-Tree vorhanden. Deren produktive Werte wurden hier nicht geprüft oder geändert. |
| Ports, IP-Adressen, Frequenzen, Geräte-ISSIs | Im Fachverlauf keine entsprechenden Betriebsparameter festgelegt. Das Beispiel `262 / 1 / 0x123456` ist keine Betriebsfreigabe. |
| Dienste, Units und Deploymentabhängigkeiten | Für dieses Thema keine neuen Dienste, Units oder Installationsschritte spezifiziert. |
| Zugangsdaten | Keine Passwörter, Tokens oder Schlüssel übernommen. |

## 9. Fehler- und Korrekturregister

| ID | Frühere Aussage / Risiko | Einordnung vom 2026-10-04 |
|---|---|---|
| K-01 | SSI-Satz als wörtliches Zitat aus 7.2.3 ausgegeben. | Kein entsprechendes Direktzitat. Feldlänge mit 7.2.1 belegen; Bedeutung/Verkürzung mit 7.2.3. |
| K-02 | MNC-Vergabe und Vier-Dezimalstellen-Grenze unter 7.2.4 genannt. | In der angehängten Ausgabe 7.2.5, Seite 30. Strukturabbildung bleibt korrekt 7.2.4, Seite 29. |
| K-03 | Zusätzliche Telekom-Codes mit unbelegten Betreiber- und Dienstzuordnungen. | Tabelle in Abschnitt 6 ersetzt die falschen Zuordnungen für die weitere Arbeit; historischer Fehler bleibt sichtbar. |
| K-04 | Freie Wahl eines TETRA-MNC unter 262 durch Nicht-Export in eine Registry gerechtfertigt. | Nicht durch die Norm gedeckt. Nationale Zuteilung und technische Konfiguration sind getrennte Fragen. |
| K-05 | Gleichheit des Nummernraums ohne klare Netzgrenze formuliert. | Eindeutigkeit und Vergabe innerhalb des jeweiligen Netzes prüfen; vollständigen Netzkontext bei Vergleichen bewahren. |
| K-06 | Zero-Padding möglicherweise als allgemeine öffentliche-Mobilfunk-zu-TETRA-Abbildung verstanden. | Nur Darstellungs-/Kodierschritt; keine Netzzuweisung und kein beschlossener Interworking-Algorithmus. |
| K-07 | „Aktuelle ETSI“ pauschal auf eine 2020-Ausgabe bezogen. | Normteil, Ausgabe und Veröffentlichungs-/Draftstatus getrennt führen; Recherchegrenze in Abschnitt 6 offenlegen. |

Diese Korrekturen stammen aus der Abschlussprüfung. Sie sind keine nachträglichen Festlegungen und wurden nicht stillschweigend in die historische Darstellung hineingeschrieben.

## 10. Ausgeführte Prüfungen und nicht ausgeführte Abläufe

### 10.1 Tatsächlich ausgeführt

| Prüfung | Ergebnis | Grenze |
|---|---|---|
| Öffnen und Inventarisieren der 25 vorhandenen PDFs | Alle 25 lokalen Dateien konnten geöffnet werden; 8061 Seiten einschließlich der 4100-seitigen Sammeldatei gezählt. | Keine vollständige inhaltliche Auswertung aller Seiten; Sammeldatei nicht als ausschließlich zusätzlicher Inhalt gezählt. |
| Direkter Abgleich EN 300 392-1, relevante Seiten | Feldbreiten und oben aufgeführte Fundstellen geprüft; fehlerhafte Verweise erkannt. | Keine umfassende Konformitätsanalyse aller Normteile. |
| Rechenprüfung des historischen Beispiels mit Python | 48 Bit Gesamtlänge, Übereinstimmung von Bitstring und gepacktem Integer sowie Rückgewinnung `262`, `1`, `1193046` per Assertions bestätigt; Hexwert `418001123456`. | Kein Test des NetCore-Rust-Codes und kein On-Air-Test. |
| Statisches Lesen der drei relevanten Rust-Dateien | Konfigurationsfelder, SSI-Typ und 10-/14-Bit-Encoder-/Decoderaufrufe vorhanden. | Keine Ausführung, kein vollständiger Aufrufer-/Validierungsaudit. |
| Öffentliche Zuteilungsliste, Text und gerenderte Seiten | Vier für die historische Tabelle relevante Betreiberzuordnungen übereinstimmend überprüft. | Unterschiedliche angezeigte Dokumentdatumsstände; keine Aussage über spezielle SIMs oder Dienstnutzung. |
| Archivindex lokal rekonstruiert und gegen Git-Blob geprüft | 35 Zeilen, 13185 Bytes; berechneter Blob-SHA `0d4ba4bf95c0867c4bf39c9e5cb8e3c9ce8e77cf` stimmt mit dem gelesenen Index überein. | Gilt für den gelesenen Ausgangsstand; vor Veröffentlichung ist der Branch erneut zu lesen. |
| Zielpfad am Ausgangscommit prüfen | Dokument am Prüfstand noch nicht vorhanden. | Eigenständiger Themenpfad; ISSI-Nummernplan ist separat dokumentiert. |

### 10.2 Reproduzierbare Rechenprüfung

Der folgende eigenständige Prüfcode bildet die tatsächlich ausgeführte arithmetische Prüfung nach. Er konfiguriert keine Basisstation und ist kein Nummernvergabetool.

```python
mcc, mnc, ssi = 262, 1, 0x123456
value = (mcc << 38) | (mnc << 24) | ssi
bits = f"{mcc:010b}{mnc:014b}{ssi:024b}"
assert len(bits) == 48 and int(bits, 2) == value
assert ((value >> 38) & 1023,
        (value >> 24) & 16383,
        value & 0xFFFFFF) == (mcc, mnc, ssi)
print(f"{value:012X}")  # 418001123456
```

### 10.3 Nur vorgeschlagene Fortsetzungsbefehle

In einem lokalen Checkout lässt sich der geprüfte Quellstand beispielsweise lesend nachvollziehen:

```bash
git show 9e61695884b6dc90174e7d33dec0afddbd2faae1:crates/tetra-config/src/bluestation/sec_net.rs
git show 9e61695884b6dc90174e7d33dec0afddbd2faae1:crates/tetra-core/src/address.rs
git show 9e61695884b6dc90174e7d33dec0afddbd2faae1:crates/tetra-pdus/src/mle/pdus/d_mle_sync.rs
```

**Nicht als ausgeführt verbuchen:** Diese lokalen `git show`-Befehle wurden hier nicht ausgeführt. Der Repository-Lesezugriff erfolgte über den GitHub-Connector. Ein ergänzender direkter HTTP-Download aus der Arbeitsumgebung scheiterte; er war kein erfolgreiches Klonen des Repositories.

Installations-, Deployment-, Reparatur- und Testbefehle sind historisch nicht dokumentiert. Bei der Prüfung wurden keine Dienste angehalten und keine Konfigurationen ausgerollt.

## 11. Erreichter Stand und endgültige Festlegungen

| Gegenstand | Status am Abschluss dieses Fachthemas |
|---|---|
| MCC 10 Bit, MNC 14 Bit, SSI/GSSI/ISSI 24 Bit, TSI 48 Bit | **Fachlich geklärt und quellengeprüft.** |
| Exakte Normabschnitte und frühere Fehler | **Dokumentiert/korrigiert** in diesem Archiv; keine Änderung anderer Handbücher oder Wikiseiten. |
| Führende Nullen als Erklärung fester Binärfelder | **Erklärt und mathematisch geprüft.** Kein verbindlicher neuer Anzeige-/Importstandard beschlossen. |
| Konkreter NetCore-Nummernplan aus diesem Planungsstand | **Nicht beschlossen.** |
| Automatische Kollisionsprüfung oder neuer Validator | **Idee / Prüfbedarf**, hier nicht implementiert. |
| 10-/14-Bit-Kodierung im vorhandenen D-MLE-SYNC-Pfad | **Im Repository implementiert**, hier nur statisch geprüft. |
| Erfolgreicher Zielgerätebetrieb | **Nicht bestätigt.** |

Weitere Entwicklungsansätze bleiben Kandidaten. Aus der technischen Klärung allein entsteht kein zusätzlicher Deploymentauftrag.

## 12. Offene Aufgaben und Roadmap-Kandidaten

Die Prioritäten sind **Vorschläge aus dem fachlichen Abgleich**; eine historische verbindliche Reihenfolge fehlt.

| ID | Kandidat | Status | Vorgeschlagene Priorität / Abhängigkeit | Abnahmekriterium |
|---|---|---|---|---|
| R-01 | Normfundstellen und Telekom-Korrekturen in weiterverwendeten Projektdokumenten konsolidieren. | Idee / fachlicher Korrekturbedarf | Zuerst vor erneuter Veröffentlichung der alten Aussagen. | Keine falschen Direktzitate; 7.2.4/7.2.5 korrekt; öffentliche Zuteilung nicht mit TETRA-Konfiguration verwechselt. |
| R-02 | Tatsächliches MCC-/MNC-/ISSI-/GSSI-Schema mit Netzkontext auf Zuteilung, Grenzen und Kollisionen prüfen. | Idee, nicht durchgeführt | Vor Nummernvergabe oder Provisionierung. | Vollständige Belegung, begründete Sonderwerte und Kollisionsbericht. |
| R-03 | Vorhandene zentrale Validierung und alle relevanten Eingangswege nachvollziehen, statt allein die Konstruktoren zu beurteilen. | Neuer Prüfauftrag als Kandidat, nicht beschlossen | Nach R-02; betrifft insbesondere Konfiguration, Import, APIs und Teilnehmerverwaltung. | Nachweis, wo ungültige Werte abgewiesen werden; anschließend gezielte Änderung nur bei tatsächlich bestätigten Lücken. |
| R-04 | Grenzwert- und Roundtrip-Tests für die bestehenden Encoder/Decoder ergänzen oder vorhandene Tests zuordnen. | Idee, nicht ausgeführt | Nach Festlegung der validierten Bereiche und Sonderwertpolitik. | Unter anderem MNC 9999/10000, MCC 999/1000, SSI 16777215/16777216 im jeweils richtigen Kontext; keine stille Verwechslung von Feldkapazität und Zuteilung. |
| R-05 | Verlustfreie Anzeige-/Importregeln und Bitfeldgrafik für Dokumentation/UI festlegen. | Teilweise vorgeschlagen, nicht beschlossen | Nach R-02/R-03. | Zahlenwert, Netzkontext und Anzeigeformat getrennt; führende Nullen verursachen keine Umnummerierung. |
| R-06 | Normenregister mit Normteil, Ausgabedatum, Veröffentlichung/Draftstatus und überprüften Quellen pflegen. | Idee | Vor einer Aussage über einen projektweit „aktuellen ETSI-Stand“. | Teil 1 nicht mit Codec-, PEI-, Air-Interface- oder ISI-Revisionsständen verwechseln. |
| R-07 | Zielgeräteabnahme nach einer späteren tatsächlichen Änderung durchführen. | Nicht geplant oder ausgeführt in diesem Planungsstand | Nur nach separat beauftragter Implementierung und erlaubtem Testaufbau. | Soll-/Ist-Netzkennung und Adressierung im Gerät beziehungsweise Protokoll nachvollziehbar; Build-/Commit-/Geräteversion dokumentiert. |

Ein bestehender [Archivbeitrag zum ISSI-Nummernplan und zur Vergaberichtlinie](2026-10-04_issi-nummernplan-rbac-und-vergaberichtlinie.md) ist im gelesenen Index vorhanden und bietet sich für eine spätere Zusammenführung an. Sein Inhalt wurde in diesem Lauf nicht als bereits erfüllte Umsetzung der obigen Kandidaten bewertet.

## 13. Anhänge und Bildinventar

### 13.1 Umfang der tatsächlich zugänglichen Anhänge

Es lagen **25 PDF-Dateien mit zusammen 8061 Seiten** vor. Darin enthalten ist `ETSI.pdf` mit 4100 Seiten. Diese Sammeldatei beginnt mit EN 300 812 V2.1.1; sie wurde nicht vollständig segmentiert oder mit sämtlichen Einzeldateien auf Dubletten abgeglichen. Die Seitenzahl ist somit eine Dateiinventar-Summe, kein Nachweis über 8061 unterschiedliche normative Seiten.

Öffnungsfähigkeit, Seitenzahlen und Titelseiten aller Dateien sind erfasst. EN 300 392-1 wurde thematisch vertieft geprüft; die übrigen Quellen sind keine vollständig analysierten Funktionsanforderungen.

| Datei | Ausgabe / Status laut Titelseite | Seiten | Thema / Auswertung |
|---|---|---:|---|
| `en_30039201v010601p.pdf` | EN 300 392-1 V1.6.1, 2020-04 | 182 | General network design; relevante Identitätsabschnitte direkt geprüft. |
| `en_30039202v030801p.pdf` | EN 300 392-2 V3.8.1, 2016-08 | 1445 | Air Interface; inventarisiert, kein vollständiger PDU-Abgleich. |
| `en_3003920303v010301p.pdf` | EN 300 392-3-3 V1.3.1, 2011-11 | 251 | ISI Group Call; inventarisiert. |
| `en_3003920304v010301p.pdf` | EN 300 392-3-4 V1.3.1, 2010-08 | 28 | ISI Short Data Service; inventarisiert. |
| `en_3003920308v010401p.pdf` | EN 300 392-3-8 V1.4.1, 2020-04 | 22 | Generic Speech Format; inventarisiert. |
| `en_3003920313v010201p.pdf` | EN 300 392-3-13 V1.2.1, 2020-04 | 191 | Transportunabhängiger ISI Group Call; inventarisiert. |
| `en_3003920315v010500a.pdf` | **Draft** EN 300 392-3-15 V1.5.0, 2026-04 | 380 | ISI Mobility Management; Draftstatus erfasst, nicht als verabschiedete Endfassung behandelt. |
| `en_30039205v020701p.pdf` | EN 300 392-5 V2.7.1, 2020-04 | 320 | PEI; inventarisiert, keine vollständige Prüfung von Anzeige-/AT-Formaten. |
| `en_30039207v030501p.pdf` | EN 300 392-7 V3.5.1, 2019-07 | 216 | Security; inventarisiert, kein Security-Audit dieser Planung. |
| `en_30039209v010701p.pdf` | EN 300 392-9 V1.7.1, 2020-04 | 46 | Allgemeine Supplementary-Service-Anforderungen; inventarisiert. |
| `en_3003921006v010401p.pdf` | EN 300 392-10-6 V1.4.1, 2006-08 | 20 | Call Authorized by Dispatcher, Stage 1; inventarisiert. |
| `en_3003921018v010301p.pdf` | EN 300 392-10-18 V1.3.1, 2003-10 | 17 | Barring of Outgoing Calls, Stage 1; inventarisiert. |
| `en_3003921101v010201p.pdf` | EN 300 392-11-1 V1.2.1, 2004-01 | 44 | Call Identification, Stage 2; inventarisiert. |
| `en_3003921114v010101p.pdf` | EN 300 392-11-14 V1.1.1, 2002-07 | 23 | Late Entry, Stage 2; inventarisiert. |
| `en_3003921117v010102p.pdf` | EN 300 392-11-17 V1.1.2, 2002-01 | 18 | Include Call, Stage 2; inventarisiert. |
| `en_3003921201v010202p.pdf` | EN 300 392-12-1 V1.2.2, 2007-08 | 56 | Call Identification, Stage 3; inventarisiert. |
| `en_3003921216v010400a.pdf` | **DRAFT** EN 300 392-12-16 V1.4.0, 2026-03 | 67 | Pre-emptive Priority Call, Stage 3; Draftstatus erfasst. |
| `en_30039401v030301p.pdf` | EN 300 394-1 V3.3.1, 2015-04 | 169 | Radio-Konformitätsprüfungen; inventarisiert, keine solchen Tests durchgeführt. |
| `en_30039502v010303p.pdf` | EN 300 395-2 V1.3.3, 2025-02 | 94 | TETRA-Speech-Codec; inventarisiert, Beispiel eines anderen neueren Normteils. |
| `en_300812v020101p.pdf` | EN 300 812 V2.1.1, 2001-12 | 156 | SIM-ME-Schnittstelle; inventarisiert. |
| `es_20081201v020205p.pdf` | ES 200 812-1 V2.2.5, 2003-12 | 8 | UICC, physikalische/logische Eigenschaften; inventarisiert. |
| `es_20081202v020401m.pdf` | **Final draft** ES 200 812-2 V2.4.1, 2005-08 | 139 | TSIM-Anwendung; Entwurfsstatus erfasst. |
| `ets_30039214e01v.pdf` | **Final draft prETS** 300 392-14, 1997-09 | 61 | PICS-Proforma; keine ausgefüllte NetCore-Konformitätserklärung. |
| `ts_10081201v020205p.pdf` | TS 100 812-1 V2.2.5, 2003-10 | 8 | UICC, physikalische/logische Eigenschaften; inventarisiert. |
| `ETSI.pdf` | Sammeldatei; erstes Dokument EN 300 812 V2.1.1 | 4100 | Nur Inventar und Beginn geprüft; keine eigene einheitliche Normversion. |

### 13.2 Bilder

Eigenständige historische Bilder fehlen. Titelblätter stammen aus PDF-Anhängen; die geplante Bitfeldgrafik wurde nicht erstellt.

Das lokal gerenderte PDF-Seitenbild ist ein Prüfmittel, kein wiedergefundenes Originalbild. Der PDF-Bestand ist inventarisiert; die zentrale Norm ist verlinkt.

## 14. Dokumentationsstand und Nachweisgrenzen

Die thematische Notiz und der separate ISSI-Nummernplan behandeln unterschiedliche Fragen. Der Ausgangsindex ist anhand seiner Git-Blob-Prüfsumme identifiziert.

Die Veröffentlichung verwendet den vorhandenen Branch `Archiving`, dessen aktuellen Stand vor der finalen Tree-/Commit-Erstellung erneut zu prüfen ist. Der Index wird um einen Eintrag ergänzt; vorhandene Zeilen bleiben erhalten. Der Archivcommit erhält einen einzelnen Elterncommit und wird nur per nicht erzwungenem Fast-Forward veröffentlicht. Bei zwischenzeitlicher Änderung ist neu zu lesen und auf dem neuen Stand aufzubauen; kein Force-Push und kein Merge sind Bestandteil dieses Auftrags.

**Offene Nachweise:** historische Codezuordnung, vollständige PDF-Lektüre, abschließende ETSI-Statusdatenbankprüfung, Vollprüfung aller Vergabe-/Validierungspfade und Livebetrieb.

## 15. Quellen und Fortsetzungsanker

- **Historische Planungsgrundlage:** Feldbreiten, konkrete Normfundstellen, Netzkontext, Versionseinordnung, Telekom-Vergleich und Nullauffüllung. Frühere Zitatkennungen ersetzen keine überprüfbare Primärquelle.
- **[N1] Hauptnorm:** bereitgestellte EN 300 392-1 V1.6.1, insbesondere 7.2.1, 7.2.3–7.2.6 und 7.7.8; [offizielle PDF][N1]. Die lokal geprüfte Dateiprüfsumme steht in Abschnitt 4.
- **[N2]/[N3] Versionsabfrage:** offizielle ETSI-Verzeichnisse; die Grenzen stehen in Abschnitt 6.
- **[B1] Telekom-Vergleich:** Bundesnetzagentur-Liste der IMSI-Blockzuteilungen, ergänzend [IMSI-Übersichtsseite][B2]. Keine Übertragung der dortigen Zuteilung auf NetCore beschlossen.
- **[R1]–[R3] Codeprüfung:** unveränderliche Dateilinks auf den geprüften Commit, unten angegeben.
- **Verwandter Kontext:** ISSI-Nummernplan-/Vergaberichtlinien-Eintrag; keine inhaltliche Abnahme dieses separaten Dokuments.
- **PRs und historische Implementierungscommits:** Für die Facharbeit dieser Planung sind keine entsprechenden Nachweise vorhanden. Es werden keine PR-Nummern ergänzt.

[N1]: https://www.etsi.org/deliver/etsi_en/300300_300399/30039201/01.06.01_60/en_30039201v010601p.pdf
[N2]: https://www.etsi.org/deliver/etsi_en/300300_300399/30039201/01.06.01_60/
[N3]: https://www.etsi.org/deliver/etsi_en/300300_300399/30039201/
[B1]: https://www.bundesnetzagentur.de/DE/Fachthemen/Telekommunikation/Nummerierung/IMSI/DL/imsi_zugbloecke.html
[B2]: https://www.bundesnetzagentur.de/DE/Fachthemen/Telekommunikation/Nummerierung/IMSI/start.html
[R1]: https://github.com/JanHG98/netcore-tetra/blob/9e61695884b6dc90174e7d33dec0afddbd2faae1/crates/tetra-config/src/bluestation/sec_net.rs
[R2]: https://github.com/JanHG98/netcore-tetra/blob/9e61695884b6dc90174e7d33dec0afddbd2faae1/crates/tetra-core/src/address.rs
[R3]: https://github.com/JanHG98/netcore-tetra/blob/9e61695884b6dc90174e7d33dec0afddbd2faae1/crates/tetra-pdus/src/mle/pdus/d_mle_sync.rs
