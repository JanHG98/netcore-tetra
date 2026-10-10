# Brainstorming: Motorola RUA/RUI: SDS-Decodierung und Benutzerverwaltung im Basisstations-WebUI

**Archivhinweis, 09.10.2026:** Diese Datei dokumentiert das im Titel beziehungsweise Quellenrahmen genannte Gespräch und dessen damalige Entscheidungen. Historische Kommandos, Planungen, Versionsangaben und Fehlerbilder bleiben erhalten; der heutige technische Stand steht in der [Gesamtroadmap](../roadmaps/gesamtroadmap.md) und in den [aktuellen Dienstanleitungen](../services/README.md). Die Archivaufnahme bestätigt keine spätere Implementierung oder Live-Abnahme.

## 1. Rahmen und Quellenstand

| Feld | Stand |
|---|---|
| Projekt / Repository | NetCore-Tetra / `JanHG98/netcore-tetra` |
| Thema | Motorola Radio User Assignment / Radio User Identity, Analyse von SDS-Anmeldevorgängen und Planung einer vollständigen Funkbenutzerverwaltung im Basisstations-WebUI |
| Notizstand | 2026-10-03, Europe/Berlin |
| Historischer Codebezug | Im früheren Entwicklungsstand abgerufene Repository-Dateien referenzierten Commit `41d925457bed3cc0f1207d509642f3e003303010`; seinerzeit wurde der Default-Branch untersucht. Ein installierter Binary-Stand wurde nicht ermittelt. |
| Geprüfter Branch | `Archiving`; statischer Quellcodebefund |
| Gepinnter Prüfstand | `27996d2f494add5ae16837acaaba4f6f22f9f49d` |
| Baum des Prüfstands | `335a3d89ce4942acf07b3da24e326188100fa8d0` |
| Archivkennung | `motorola-rua-rui-sds-webui-2026-10-03` |

**Arbeitsstand:** Historische Uplink-Analyse, Repository-Befund vom 03.10.2026 und Umsetzungsentwurf sind getrennt. Parser-Skizzen sind kein Nachweis einer Repository-Implementierung.

### 1.1 Statusbegriffe

| Begriff | Bedeutung in dieser Dokumentation |
|---|---|
| **Idee** | Diskutierter Ausbau oder technische Option, noch nicht verbindlich festgelegt |
| **Beschlossen/geplant** | Explizite Anforderung beziehungsweise ausgewiesener Umsetzungsvorschlag; die jeweilige Herkunft wird genannt |
| **Implementiert** | Am angegebenen Commit tatsächlich vorhandener Code; nicht automatisch gebaut, installiert oder funktionsfähig abgenommen |
| **Getestet** | Ein konkret benannter Test wurde durchgeführt; Testgegenstand und Grenzen gehören zur Aussage |
| **Im Betrieb bestätigt** | Konkrete Beobachtung an der Anlage, hier insbesondere die bereitgestellten Empfangs- und Weiterleitungslogs; keine pauschale Bestätigung des gesamten Systems |
| **Unbestätigt** | Plausible Interpretation, Literaturhinweis oder frühere Aussage ohne ausreichenden Nachweis |

### 1.2 Datenumfang und Quellenlage

Alle in den Arbeitsnotizen genannten PINs sind entfernt, einschließlich der einfachen Labor-PINs. Ebenso fehlen vollständige Logon-Hextelegramme, aus denen sich diese Werte zurückgewinnen ließen. Die beiden personenbezogenen Testkennungen werden als `P_A` und `P_B` bezeichnet. Synthetische Namenslängen, nicht geheime Headerwerte, Geräteadressen und Feldpositionen bleiben für die Weiterarbeit erhalten.

Nicht Bestandteil des Archivs sind PIN-Hashes aus realen Konten, Tokens, Schlüssel, unveränderte Credential-Captures oder Zugangsdaten aus Konfigurationsdateien. Es wurde nicht nach weiteren Geheimnissen im Repository gesucht. Vorhandene Geheimnisse in alten Journals, Brew-Mitschnitten oder Sicherungen werden durch diese Dokumentation nicht entfernt; deren Bereinigung und gegebenenfalls Rotation bleiben ein eigener Betriebsauftrag.

Grundlagen sind Uplink-Beobachtungen, frühere Repository-Befunde, gezielte Repository-Abrufe vom 03.10.2026 und 25 PDFs. Die PDFs wurden inventarisiert und durchsucht; ausgewählte einschlägige Seiten wurden im Detail geprüft. Keine vollständige fachliche Lektüre aller 8.061 Seiteninstanzen. Fehlende Downlink-Captures und Endgerätedaten stehen in Abschnitt 16.

## 2. Ergebnis für die spätere Fortsetzung

**Das gewünschte Ziel ist klar: Funkbenutzer direkt im vorhandenen Basisstations-WebUI verwalten und nachvollziehbar anzeigen, welcher Benutzer welches Funkgerät verwendet.** Die Uplink-Beobachtungen liefern eine brauchbare Datenbasis für einen eingeschränkten Uplink-Decoder, aber keinen funktionsfähigen RUA-Server.

Der belegte Betrieb umfasst den Empfang von SDS-Nachrichten eines Motorola-Endgeräts und deren Weiterleitung zu Brew. Eine erfolgreiche serverseitig bestätigte RUA-Anmeldung, eine Ablehnung am Funkgerät oder eine erzwungene Abmeldung wurde nicht gezeigt. Auch im für die Quellenprüfung vom 03.10.2026 untersuchten `Archiving`-Stand wurde in den geprüften Integrationspunkten keine fertige RUA-Verwaltung gefunden.

Die wichtigsten Ergebnisse der zusätzlichen Archivprüfung sind:

1. Der führende Teil `C1 00 <Referenz>` passt zum **standardisierten SDS-TL-SDS-TRANSFER-Header**. Das vermeintliche unbekannte Versions-/Flags-Byte und der vermeintlich RUA-eigene Sequenzzähler sind daher nicht als beliebige proprietäre Felder zu behandeln.
2. Die bisherige feste Gruppierung `0x86` plus zwei Bits `10` hat eine besser begründete alternative Lesart: drei Bits mit Wert `4`, gefolgt vom sieben Bit breiten Textcodierungswert `26`. Letzterer steht in ETSI für UCS-2 mit UTF-16BE-Erweiterung. Der Bezug des Dreibitwerts auf den RUI-Identitätstyp bleibt eine Hypothese.
3. Die geprüften Logon-Daten erlauben eine reproduzierbare Namensextraktion; die numerische PIN-Interpretation stimmt bei zwölf von dreizehn Beispielen mit der angegebenen Eingabe überein. Der erste Datensatz bleibt widersprüchlich. Das rechtfertigt **keine** Aussage „das ganze Protokoll ist vollständig geknackt“.
4. Der geprüfte SDS-Pfad enthält zusätzlich einen zentralen Handoff und Ausfall-/Spool-Pfade. Die alte Empfehlung „vor Brew abfangen“ muss zu „vor unkontrolliertem Logging und vor jedem generischen Weiterleitungs-/Spool-Pfad klassifizieren“ erweitert werden.
5. Die Oberfläche liegt inzwischen in `net_dashboard/ui/`; die frühere große `html.rs` ist jetzt ein Asset-Einbinder. Alte Komplettdateien dürfen diese Struktur nicht zurücksetzen.

Diese Punkte sind Prüf- und Planungsbefunde; eine Implementierung ist nicht belegt.

## 3. Ziel, Ausgangslage und Entwicklung

### 3.1 Ausgangslage

Ziel ist die Integration von Motorola RUA/RUI in NetCore-Tetra, zunächst mit Recherche zum Systemablauf und einer Roadmap. Anlass waren SDS-Logs beim Ab- und Anmelden eines Funkbenutzers:

| Parameter | In den Arbeitsnotizen belegt |
|---|---|
| Quell-ISSI | `2020004` |
| Ziel-SSI | `16777213`, hexadezimal `0xFFFFFD` |
| Gedruckter SDS-Typ | `type=3`; der Code meint damit SDS Type 4, also variable Nutzdatenlänge |
| Kurzes Ereignis | 21:11:49.552, 33 gültige Bits, mit einer Abmeldung in Zusammenhang gebracht |
| Langes Ereignis | 21:12:07.290, 119 gültige Bits, mit einer Anmeldung in Zusammenhang gebracht |
| Führender Protocol Identifier | `0xC1`, dezimal `193` |
| Weiterleitung | `forwarding to Brew`, danach Versandmeldung an `ws://10.0.1.22:8081` |
| Kalendertag der Logs | Nicht angegeben |
| Endgerät | Motorola; konkretes Modell, Firmware und Codeplug-Auszug fehlen |

Zwischenzeitliche Trainingssequenz-Meldungen und ein `D-NWRK-BROADCAST` mit `Europe/Berlin` und ohne Nachbarzellen wurden nicht als RUA-Inhalt ausgewertet. Sie beweisen weder einen RUA-Fehler noch ein RUA-Accept.

Die lokale Basisstation und der Brew-Endpunkt existierten bereits. Es fehlte ein geeigneter **RUA-Referenzserver** für den Protokollvergleich; die bestehende NetCore-Infrastruktur war davon unabhängig.

### 3.2 Entwicklung der Analyse

| Schritt | Inhalt | Einordnung |
|---|---|---|
| Erste Repo-/Protokollprüfung | SDS Type 4 erkannt; lokale Zustellung gegenüber Brew-Weiterleitung erklärt; RUA von MM-Registrierung unterschieden | Historische Codeprüfung und Architekturberatung |
| WebUI konkretisiert | Benutzer anlegen, löschen, sperren, PIN ändern; aktive Gerätezuordnung anzeigen | Explizite Anforderung |
| Erster bekannter Eingabedatensatz | Benutzerkennung und PIN zur Zuordnung eines vorhandenen Telegramms genannt | Bereitgestellte Messinformation; Zugangsdaten nicht archiviert |
| Vier Vergleichsdatensätze | Zwei gleich lange Namen, jeweils zwei unterschiedliche PIN-Eingaben | Kontrollierte Variationen; Empfangslogs vorhanden |
| Namenslängenserie | Acht synthetische Kennungen mit einer bis acht Stellen, konstante Labor-PIN | Hexdaten vorhanden; separate SDS-Bitlängen wurden hierbei nicht mitgeliefert |
| Öffentliche Recherche verlangt | Mangels RUA-Referenzserver sollten Downlink-Details aus öffentlich verfügbaren Quellen ermittelt werden | Verbindlicher Rechercheauftrag; fehlende Referenzserver-Captures sind keine vorgeschaltete Voraussetzung |
| Rechercheabschluss in den Arbeitsnotizen | TTR 001-17 / IOP 001-17 und Hersteller-/Netzdokumentationen als Spuren genannt; kein belastbarer Accept-/Reject-/Cancel-Encoder geliefert | Rechercheergebnis mit Lücken, kein Implementierungsabschluss |
| Quellenprüfung 03.10.2026 | Codeabgleich, erneute Offline-Prüfung der Bitextraktion und gezielter Standardsabgleich | Zusätzlicher Befund, vom historischen Betriebsstand getrennt |

## 4. Anforderungen und Entscheidungsstand

### 4.1 Verbindliches Ziel

Folgende Anforderungen sind als **beschlossen/geplant auf Anforderungsebene** zu behandeln:

- Motorola RUA/RUI in das eigene System aufnehmen, statt die beobachteten Login-Nachrichten lediglich unbesehen weiterzuleiten.
- Die Verwaltung **direkt im Basisstations-WebUI** anbieten: Benutzer anlegen und löschen, Benutzer sperren, PIN ändern.
- Sichtbare Zuordnung von aktiven Funkgeräten zu angemeldeten Benutzern; später nachvollziehbarer Wechsel eines Benutzers auf ein anderes Gerät.
- Das Protokoll recherchieren und anhand eigener kontrollierter Uplink-Beispiele weiter erschließen.
- Die fehlenden Serverantworten öffentlich recherchieren, weil kein geeigneter RUA-Referenzserver zur Verfügung steht.

Eine Freigabe zur sofortigen aktiven Aussendung unbekannter RUA-PDUs, zur Änderung von Funkrechten oder zu einem Produktiv-Rollout ergibt sich daraus nicht. Eine Implementierung ist noch offen.

### 4.2 Technische Vorschläge, die nicht als endgültiger Nutzerbeschluss gelten

| Vorschlag aus den Arbeitsnotizen | Begründung | Verbindlichkeit |
|---|---|---|
| Eigene User- und Assignment-Verwaltung getrennt von MM | Eine neue Geräte-Registrierung darf nicht unbeabsichtigt sämtliche Benutzerzuordnungen vernichten | Architekturvorschlag; am 03.10.2026 durch Registry-Code gut begründet |
| SQLite für Benutzer, Sitzungen und Audit | Transaktionen und eindeutige Kennungen statt unsynchronisierter TOML-/JSON-Änderungen | Empfehlung, keine Datenbank eingerichtet |
| Argon2id mit individuellem Salt; optional serverseitiger Pepper | Keine reversibel gespeicherten PINs in der Benutzerverwaltung | Sicherheitsentwurf, noch nicht implementiert |
| Beobachtungsmodus vor aktivem Servermodus | Unbestätigte Nachrichtenformate nicht als produktive Anmeldung behandeln | Vorgeschlagene Einführungsstrategie |
| Archivieren statt hart löschen | Historische Ereignisse bleiben einem minimalen Datensatz zuordenbar | Nicht endgültig beschlossen; Lösch- und Aufbewahrungsregeln fehlen |
| Alte Anmeldung beim Gerätewechsel beenden | Eindeutige persönliche Benutzeridentität auf einem Gerät | Entwurfsidee, keine bestätigte Takeover-Policy |
| Acht Stunden Assignment; beispielhaft 60 Sekunden Offline-Grace | Anschauliche Schicht-/Ausfallbeispiele | Keine Standardvorgaben und keine bestätigten Betriebswerte |
| Rechteprofile, Prioritäten, P-ISSI/RUN-Routing | Über die Anzeige hinaus tatsächlich nutzerbezogener Betrieb | Erweiterungsideen, nicht Voraussetzung für das erste Verwaltungsmodul |
| Force Off, Book On und Verlängerung im WebUI | Dispatcher-Steuerung der Benutzerzuordnung | Ausbauziel, abhängig vom bestätigten Downlink-Protokoll |

## 5. Begriffe und Systemgrenzen

**MM-Geräteregistrierung, RUA-Benutzerzuordnung und Web-Anmeldung sind drei verschiedene Zustandsbereiche.** Ein Funkgerät kann im Netz registriert sein, ohne dass ein Funkbenutzer erfolgreich zugewiesen wurde. Ein gültiger Funkbenutzer erhält dadurch nicht automatisch Administrationsrechte im WebUI.

Für das Datenmodell sind auseinanderzuhalten:

| Identität / Zustand | Rolle |
|---|---|
| Physische ITSI/ISSI | Teilnehmeradresse des verwendeten Funkgeräts im TETRA-Netz |
| RUI | Benutzer- oder Funktionsidentität im RUA-Verfahren; nicht pauschal auf eine Zahl reduzieren |
| RUN | Logische Radio User Number, soweit ein entsprechender Nummernplan eingesetzt wird |
| P-ISSI / permanente Benutzeradresse | In den Arbeitsnotizen diskutierte logische Erreichbarkeit; konkretes Mapping noch zu spezifizieren |
| Anzeigename / Alpha-Tag | Darstellbarer Name; darf nicht unbemerkt mit einer internen Benutzer-ID gleichgesetzt werden |
| Web-IAM-Konto | Menschlicher Zugang zur Verwaltungsoberfläche mit eigenen Rollen und Ressourcenrechten |
| Assignment | Zeitlich und fachlich definierte Bindung einer Benutzeridentität an eine Geräteidentität |
| Erreichbarkeit | Gegenwärtiger RF-/MM-Zustand des Geräts; getrennt vom Gültigkeitszustand des Assignments |

Die öffentlich erneut geöffnete ETSI TR 102 300-5 V1.4.1, Annex B, beschreibt RUA als Zuordnung eines Benutzers zu einem Mobilgerät über SDS, gegebenenfalls mit PIN-Prüfung. Das ermöglicht logische Erreichbarkeit und benutzerbezogene Attribute. Der Report stellt jedoch selbst klar, dass seine Informationsflussbilder keine vollständigen bitgenauen Luftschnittstellen-PDUs definieren. Er ist deshalb eine Architekturquelle, kein Ersatz für die gesuchte RUA-Feldtabelle. Siehe [Q1](#q1-erneut-geprüfte-öffentliche-architekturquelle).

**Wichtige Betriebsfolge:** Funkloch, T351-Abmeldung, Zellwechsel oder Neustart dürfen nicht ohne definierte Policy als expliziter Benutzer-Logoff behandelt werden. Umgekehrt darf ein aus einer Datenbank wiederhergestelltes Assignment nicht blind als am Funkgerät bestätigte, weiterhin gültige Anmeldung angezeigt werden.

## 6. Uplink-Daten und reproduzierbare Bitinterpretation

### 6.1 Verfügbare Stichprobe und Schutz der Testdaten

Die Stichprobe besteht aus dreizehn Logon-bezogenen Nachrichten und einer kurzen Logoff-bezogenen Nachricht. Fünf Logon-Logs nennen ausdrücklich 119 gültige Bits. Bei der späteren Serie mit acht unterschiedlichen Namenslängen wurden nur Hexbytes geliefert.

| Datengruppe | Anzahl | Variation | Ergebnis der zusätzlichen Offline-Prüfung |
|---|---:|---|---|
| Ursprüngliches langes Telegramm | 1 | Dreistelliger Name, separat genannte Eingabe | Name passt; numerisches Feld weicht von der angegebenen PIN ab |
| Gleich lange Namen / unterschiedliche PIN-Eingaben | 4 | `P_A` und `P_B`, je zwei PIN-Varianten | Namen und numerische Werte entsprechen den jeweiligen Angaben |
| Synthetische Längenserie | 8 | Alphabetisches Präfix mit einer bis acht Stellen | Namen passen, gleiche numerische PIN-Extraktion in allen acht Telegrammen |
| Kurzes Abmeldeereignis | 1 | 33 Bit | Header und kurzes Anwendungsdatenmuster nachvollziehbar; keine Serverquittung vorliegend |

Die Kenntnis einer bekannten Eingabe diente ausschließlich zur Offline-Interpretation. Sie wurde nicht zur Anmeldung an fremden Systemen eingesetzt. Es wurden keine Zugangsdaten in das Repository geschrieben.

### 6.2 Bitnummerierung

Alle nachfolgenden Offsets sind **nullbasiert und MSB-first**, bezogen auf das erste Nutzdatenbit von `SdsUserData::Type4`, nicht auf den Beginn des gesamten U-SDS-DATA-PDU. Intervallnotation `[a, b)` enthält Bit `a`, nicht Bit `b`.

`len_bits` bleibt eine eigene Größe. `payload.len() * 8` ist bei nicht oktettausgerichteten Daten lediglich die Speicherkapazität, nicht die tatsächliche Protokolllänge. Der vorhandene Datentyp erhält beide Angaben.

### 6.3 Historische Arbeitshypothese und geprüfte Einordnung

| Bits / Bereich | Beobachtung in den vorhandenen Logon-Beispielen | Belastbare Einordnung |
|---|---|---|
| `[0, 8)` | `0xC1` | Beobachteter PID dieses Profils; die generische ETSI-Tabelle belegt den Anwendungsbereich mit SDS-TL, nicht allein die Zuordnung „C1 = RUA“ |
| `[8, 16)` | `0x00` | Passt zu SDS-TRANSFER mit den folgenden SDS-TL-Steuerbits auf null; kein belegtes proprietäres Versionsbyte |
| `[16, 24)` | Fortlaufende Referenz im beobachteten Ablauf | SDS-TL Message reference; nicht automatisch RUA-eigener Transaktionsschlüssel |
| `[24, 32)` | `0x01` bei Logon-bezogenen Nachrichten | Beobachtetes Anwendungskennbyte; Name „Logon Request“ ist empirisch, genaue Opcode-Feldbreite und Varianten benötigen die TIP |
| `[32, 42)` | Über alle Logons konstant | Historisch als `0x86` und `10` gruppiert; alternative Aufteilung siehe Abschnitt 7.2 |
| `[42, 50)` | Als acht Bit gelesener Wert ergibt jeweils `16 × n` | Gut passender Kandidat für die Bitlänge der Namensdaten; keine allgemeine Festlegung sämtlicher RUI-Varianten |
| `[50, 50 + 16n)` | Jeweils die bekannten Namenszeichen in 16-Bit-Big-Endian-Darstellung | Bei allen dreizehn Beispielen reproduzierbar; erweiterte Unicode-Eingaben nicht getestet |
| `[50 + 16n, 70 + 16n)` | Numerischer Wert aus 20 Bit | Bei zwölf von dreizehn Eingaben passend; Feldsemantik und Randwerte weiterhin zu bestätigen |
| Bit `70 + 16n` | Null | In der Arbeitshypothese ein abschließendes Steuer-/Optionsbit; seine Bedeutung ist nicht ermittelt |
| Danach | In den vorhandenen Speicherbytes verbleibt ein weiteres Nullbit | Bei ausdrücklich genannten 119 Bit außerhalb der Nutzlänge; bei der Längenserie nur unter der abgeleiteten Gesamtlänge als Padding einzuordnen |

Diese Tabelle ist ein **beobachtetes eingeschränktes Profil**, kein normativer RUA-Encoder. Insbesondere beweisen konstante Bits nicht ihre Feldnamen oder ihre Bedeutung bei anderen Nachrichtentypen.

### 6.4 Längenserie

Die ursprünglichen synthetischen Kennungen waren die aufeinanderfolgenden Präfixe `A`, `AB`, `ABC` bis `ABCDEFGH`. Die PIN-Werte und vollständigen Telegramme werden nicht übernommen.

| Zahl der 16-Bit-Einheiten `n` | Kandidat Namenslänge in Bit | Byte an Index 5 | Erhaltene Rohbytes | Abgeleitete gültige Bitlänge, nicht separat geloggt |
|---:|---:|---|---:|---:|
| 1 | 16 | `0x84` | 11 | 87 |
| 2 | 32 | `0x88` | 13 | 103 |
| 3 | 48 | `0x8C` | 15 | 119 |
| 4 | 64 | `0x90` | 17 | 135 |
| 5 | 80 | `0x94` | 19 | 151 |
| 6 | 96 | `0x98` | 21 | 167 |
| 7 | 112 | `0x9C` | 23 | 183 |
| 8 | 128 | `0xA0` | 25 | 199 |

Für die Arbeitshypothese ergeben sich:

```text
name_start = 50
name_length_bits = 16 * n
pin_start = name_start + name_length_bits
candidate_pin_width = 20
candidate_final_bit = pin_start + candidate_pin_width
candidate_total_bits = 71 + 16 * n
storage_bytes = ceil(candidate_total_bits / 8)
```

Die Rohbytezahlen passen zu dieser Formel. Trotzdem dürfen zukünftige Parser die tatsächliche SDS-Längenangabe nicht durch diese Formel ersetzen. Bei neuen Varianten, anderen Identitätstypen oder Erweiterungen kann die Struktur anders aussehen.

### 6.5 Zeichencodierung und zulässige Länge

Die gelieferten lateinischen Namen lassen sich exakt als 16-Bit-Big-Endian-Codeeinheiten lesen. Das ist kompatibel mit UCS-2/UTF-16BE. Die frühere Behauptung, allein diese ASCII-nahen Beispiele bewiesen bereits die komplette UTF-16-Unterstützung einschließlich Surrogaten und Nicht-BMP-Zeichen, war zu weitgehend.

Zusätzliche Standardbefunde zur Textcodierung stehen in Abschnitt 7.2. Noch ungeprüft sind unter anderem Umlaute, nichtlateinische Eingaben, Surrogatpaare, ungültige Codeeinheiten, Groß-/Kleinschreibung und Normalisierung. Eine WebUI darf solche Eingaben erst nach festgelegter Policy und Endgerätetest zusagen.

Wenn die vermutete Bitlänge tatsächlich in acht Bit steht, passen bei 16-Bit-Einheiten höchstens 15 Einheiten in das Feld. Das ist eine **Grenzfolgerung für dieses Profil**, kein bewiesenes Motorola-Eingabelimit. Ob ein 16-stelliger Name abgelehnt, gekürzt oder über eine andere Variante übertragen wird, wurde nicht getestet.

### 6.6 Numerische PIN-Interpretation und offener Widerspruch

Im zweiten und dritten Testblock liefert die Extraktion des 20-Bit-Fensters unmittelbar nach dem Namen den ausdrücklich genannten Zahlenwert. Das spricht bei diesen Beispielen für eine reversible numerische Darstellung, nicht für einen kryptografischen PIN-Hash im sichtbaren Anwendungsinhalt.

Für das ursprüngliche lange Telegramm ist der extrahierte Wert jedoch **um drei kleiner** als die separat genannte Eingabe. Beide konkreten Zahlen und sämtliche dazugehörigen Hex-Endungen sind hier aus Geheimnisschutzgründen entfernt. Eingabeabweichung, Zuordnung eines anderen Versuchs oder eine noch nicht erfasste Strukturvariante bleiben offen. Es gibt keinen Nachweis eines Funkfehlers und keinen Grund, im Decoder pauschal eine Korrektur zu addieren.

Auch die genaue Abgrenzung von 20 PIN-Bits und dem folgenden konstanten Nullbit muss durch die Protokollbeschreibung beziehungsweise weitere Variationen bestätigt werden. Eine passende Extraktion allein beweist nicht sämtliche semantischen Feldgrenzen.

**Führende Nullen:** Eine reine Zahlenrepräsentation erhält die ursprüngliche Textlänge nicht. Vor dem Speichern und Prüfen von Hashes muss daher eine eindeutige Kanonisierung festgelegt werden. Sonst können die WebUI-Eingabe als Zeichenfolge und die RF-seitig gelesene Zahl trotz inhaltlich gleicher Eingabe unterschiedliche Hash-Eingaben erzeugen. Zeichenfolge, zulässige Ziffernzahl und eventuell führende Nullen nicht ungeprüft aus der alten Skizze übernehmen.

Ein reversibler Inhalt nach der Decodierung sagt allein nichts darüber aus, ob die ursprüngliche Funkstrecke verschlüsselt war. Der tatsächliche AIE-/E2EE-Zustand der Mitschnitte wurde nicht festgestellt.

### 6.7 Abmeldebezogene Nachricht

Die kurze, keine PIN enthaltende Nachricht darf vollständig festgehalten werden:

```text
33 gültige Bits
C1 00 04 04 00

C1       beobachteter Anwendungs-PID mit SDS-TL
00       passender SDS-TRANSFER-Steuerteil
04       SDS-TL Message reference
04       beobachtetes Anwendungskennbyte des Abmeldeereignisses
0        ein verbleibendes gültiges Bit mit unbestimmter Semantik
```

Die übrigen sieben Bits des fünften Speicherbytes liegen außerhalb der angegebenen Nutzlänge. Ein generisches „jedes RUA-Paket hat genau ein Paddingbit“ wäre somit falsch: Das galt nur für die betrachtete Logon-Arbeitshypothese. Die Semantik „Logoff Request“ ist durch den Bedienkontext plausibel, aber keine Bestätigung eines vollständigen Logoff-/Acknowledge-Ablaufs.

## 7. Standardsabgleich vom 03.10.2026

Dieser Abschnitt enthält **neu geprüfte Quellenbefunde**, nicht nachträglich behauptete Ergebnisse der ursprünglichen Unterhaltung.

### 7.1 SDS-TL statt unbekannter proprietärer Transportheader

Die bereitgestellte EN 300 392-2 V3.8.1 beschreibt in Abschnitt 29.4.2.4, Tabelle 29.14, Seite 1203, die Struktur von SDS-TRANSFER. Relevant sind zunächst PID, vier Bit Nachrichtentyp, zwei Bit Delivery-report request, ein Bit Service selection/short form report, ein Bit Storage/forward control und acht Bit Message reference. Zusätzliche Weiterleitungsfelder sind bedingt vorhanden.

Tabelle 29.21 auf Seiten 1208–1209 ordnet `0xC0` bis `0xFE` dem Bereich benutzerdefinierter Anwendungen **mit zu verwendendem SDS-TL-Datentransferdienst** zu. `0xC1` liegt in diesem Bereich. Der Nachrichtentyp `0` bezeichnet SDS-TRANSFER; die Message reference ist ein separates acht Bit breites Transportfeld. Die Tabellen wurden anhand gerenderter PDF-Seiten gegengeprüft.

Damit ist für die vorhandenen Nachrichten folgende Schichtung sinnvoll:

```text
U-SDS-DATA / SDS Type 4
    -> SDS-TL: PID, Transfer-Steuerfelder, Message reference
        -> Anwendungsdaten des beobachteten RUA-Profils
            -> Anforderung, Identitätsdaten, vertrauliche Authentifizierungsdaten
```

**Korrektur:** Eine SDS-TL-Quittung bestätigt Transport beziehungsweise Verarbeitung einer SDS-Nachricht; sie ersetzt kein nachgewiesenes RUA-Accept. Umgekehrt ist aus einem inkrementierenden Transportzähler keine umfassende RUA-Transaktions-, Replay- oder Sitzungslogik abzuleiten.

**Grenze:** Der allgemeine PID-Bereich beweist nicht die konkrete PID-Inhaberschaft oder universelle Gültigkeit von `0xC1` für alle Hersteller. Auch ein Paket mit diesem PID muss auf Typ, Länge, Richtung und konfigurierten Kontext geprüft werden.

### 7.2 Wahrscheinliche Aufteilung der bisher festen Logon-Metadaten

Die beobachtete Konstante in Bits 32–41 kann alternativ aufgeteilt werden:

| Kandidat | Offset / Breite | Wert in allen Logon-Beispielen | Nachweisgrad |
|---|---|---:|---|
| Identitätstyp | Bit 32, 3 Bit | 4 | Hypothese; könnte zur zuvor genannten Alpha-Tag-Variante passen, aber PEI-Werte sind nicht automatisch Luftschnittstellenwerte |
| Text coding scheme | Bit 35, 7 Bit | 26 / `0x1A` | Sehr gut gestützter Strukturkandidat: ETSI definiert diesen Wert als UCS-2 mit UTF-16BE-Erweiterung |
| Length of character string | Bit 42, 8 Bit | `16 × n` | Passt zu den Messwerten und zum generischen Mnemonic-Name-Aufbau |
| Character string | Ab Bit 50 | Beobachtete Namen | Für die vorhandenen Beispiele rechnerisch bestätigt |

Quellen: EN 300 392-2 V3.8.1, Abschnitt 29.5.4.1, Tabelle 29.29, Seite 1214; EN 300 392-9 V1.7.1, Abschnitt 8.4.2, Tabellen 15 und 17, Seite 24. Letztere beschreibt für Mnemonic Names Textcodierung, Bitlänge und Zeichenfolge.

**Wichtig:** Die Übertragung eines generischen SS-Stringformats auf diese konkrete RUA-Anwendung bleibt ein begründeter Abgleich, keine in den Anhängen gefundene RUA-PDU-Tabelle. Das Archiv ersetzt die frühere starre Interpretation von `0x86` nicht stillschweigend durch eine neue Gewissheit.

### 7.3 RUA-Anforderung während der Registrierung

In den Arbeitsnotizen wurde anhand angeblicher Motorola-Dokumentation eine RUA-Anforderung innerhalb von `D-LOCATION-UPDATE-ACCEPT` diskutiert. Der geprüfte Code besitzt einen generischen Platz für ein proprietäres Informationselement, setzt ihn aber im untersuchten Erzeugungspfad auf `None`.

Die EN 300 392-2 V3.8.1 führt auf Seite 1437 in ihrer Änderungshistorie CR 053 auf: eine Ergänzung zur Unterstützung von RUA, mit den betroffenen Abschnitten 16.9.2.7, 16.10.51 und 16.10.41. Dieser Eintrag ist mit **REJ / zurückgezogen** gekennzeichnet. Er ist keine angenommene normative RUA-Felddefinition.

Daraus folgen zwei Grenzen: Weder dürfen aus der bloßen Existenz von `proprietary` passende RUA-Bits geraten werden, noch bedeutet der zurückgezogene Einzel-CR, dass RUA grundsätzlich nicht spezifiziert oder nicht implementierbar wäre. Die passende TIP-/Herstellerbeschreibung bleibt zu beschaffen und zu prüfen.

### 7.4 Was die Anhänge nicht geliefert haben

Die gezielte Volltextsuche in allen bereitgestellten PDFs fand keine vollständige RUA-Accept-, Reject-, Cancel- oder Book-On-Codierung und keine enthaltene TTR 001-17. Die relevanten RUA-Nennungen in der Air-Interface-Datei waren die Abkürzungsliste und der zurückgezogene CR; die Sammeldatei enthält entsprechende Fundstellen erneut.

Die generische PEI-Norm `en_30039205v020701p.pdf` lieferte bei dieser Suche keinen `+CTRUA`-Befehl. Das widerlegt keine herstellerspezifische Erweiterung, belegt sie aber auch nicht. Ebenso ist **EN 300 392-11-17 „Include Call“ nicht TTR 001-17 „Radio User Assignment“**; die übereinstimmende Endnummer darf nicht zu einer Verwechslung führen.

## 8. Historischer Repository-Befund und Codevergleich vom 03.10.2026

### 8.1 Historisch geprüfter Stand

Die früheren Abrufe aus Commit `41d925457bed3cc0f1207d509642f3e003303010` zeigten:

- `sds_bs.rs`: U-SDS-DATA parsen, Rohbytes loggen, lokale Sonderdienste behandeln, danach lokale Einzel-/Gruppenzustellung oder Brew-Weiterleitung.
- `SdsUserData::Type4(u16, Vec<u8>)`: exakte Länge und Bytes vorhanden; Type-Identifier `3`.
- `SubscriberRegistry`: Geräte und Gruppen, keine Benutzer-Assignments; `register()` erzeugt den Eintrag neu.
- `DLocationUpdateAccept`: generisches optionales proprietäres Feld; Erzeugung mit `proprietary: None`.
- Dashboard: Gerätestatus und WebSocket-Telemetrie; Cookie-Sessions vorhanden; keine belegte RUA-Benutzeroberfläche.

Das erklärte die ursprünglichen Betriebslogs. Es war keine Bestätigung, dass Brew die Benutzeranmeldung bearbeitet oder dass eine RUA-Serverantwort bereits existiert.

### 8.2 Am 03.10.2026 geprüfter Stand auf `Archiving`

Alle Links in der folgenden Tabelle sind auf den geprüften Commit gepinnt. Die Prüfung war eine gezielte Quelltextsichtung, kein vollständiger Build oder Testlauf des gesamten Repositories.

| Bereich | Geprüfte Datei / Symbol | Befund | Auswirkung auf die Fortsetzung |
|---|---|---|---|
| SDS-Datentyp | [sds_user_data.rs](https://github.com/JanHG98/netcore-tetra/blob/27996d2f494add5ae16837acaaba4f6f22f9f49d/crates/tetra-saps/src/control/enums/sds_user_data.rs) | `Type4(u16, Vec<u8>)`, Identifier 3 unverändert | Bitlänge bis in jeden neuen Decoder erhalten |
| RF-Eingang | [sds_bs.rs](https://github.com/JanHG98/netcore-tetra/blob/27996d2f494add5ae16837acaaba4f6f22f9f49d/crates/tetra-entities/src/cmce/subentities/sds_bs.rs), `route_rf_deliver()` | Roh-PDU-Debug und vollständige INFO-Hexausgabe weiterhin vor Sonderbehandlung; im geprüften Pfad kein RUA-Handler | Redaktion muss vor diesen Ausgaben ansetzen, nicht erst nach einem neuen Decoder |
| Zentrales SDS-Routing | Dieselbe Datei, `central_sds_routing_enabled()`, `central_sds_routing_configured()`, `emit_sds_edge_data()` | Zusätzlich zentraler Handoff, lokale Ausfallzustellung und Weitergabe für Warteschlangen; Legacy-Brew-Pfad bleibt | Früherer Einbauplan allein vor Brew reicht nicht mehr |
| Rückweg aus Brew | Dieselbe Datei, `rx_sds_from_brew()` | Kann ebenfalls an die zentrale SDS-Verarbeitung übergeben | Richtung und Eigentümer der RUA-Verarbeitung festlegen; keine Doppelauswertung |
| Subscriber-Lebenszyklus | [state.rs](https://github.com/JanHG98/netcore-tetra/blob/27996d2f494add5ae16837acaaba4f6f22f9f49d/crates/tetra-config/src/bluestation/state.rs), `SubscriberRegistry::register()` | Weiterhin Neuaufbau über `deregister()`; `Subscriber` enthält ISSI, Gruppen und Duplexfähigkeit | Assignment nicht ausschließlich in diesen flüchtigen Eintrag legen |
| Registrierungsantwort | [mm_bs.rs](https://github.com/JanHG98/netcore-tetra/blob/27996d2f494add5ae16837acaaba4f6f22f9f49d/crates/tetra-entities/src/mm/mm_bs.rs) | Untersuchte `DLocationUpdateAccept`-Erzeugung mit `proprietary: None` | Kein nachgewiesener RUA-Registrierungstrigger |
| WebUI-Assets | [html.rs](https://github.com/JanHG98/netcore-tetra/blob/27996d2f494add5ae16837acaaba4f6f22f9f49d/crates/tetra-entities/src/net_dashboard/html.rs) | Bindet `ui/dashboard.html`, `ui/login.html`, CSS und JavaScript über `include_str!` ein | Neue Bedienung in vorhandene Asset-Struktur integrieren, keine alte Monolithdatei ersetzen |
| Dashboard-Gerätestatus | [net_dashboard/state.rs](https://github.com/JanHG98/netcore-tetra/blob/27996d2f494add5ae16837acaaba4f6f22f9f49d/crates/tetra-entities/src/net_dashboard/state.rs), `MsState` | ISSI, Gruppen, Auswahl-Inferenz, RSSI, Zeiten, Energiesparmodus; keine RUA-Felder im geprüften Modell | Benutzeransicht und Assignment-Snapshot sind noch zu entwickeln |
| Transport-Telemetrie | [events.rs](https://github.com/JanHG98/netcore-tetra/blob/27996d2f494add5ae16837acaaba4f6f22f9f49d/crates/tetra-entities/src/net_telemetry/events.rs) | `SdsEdgeIngress` enthält unter anderem PID, `len_bits` und vollständigen `payload` | Sicherheitsprüfung muss Transport, zentrale Verarbeitung und Spool einschließen |
| Web-Authentifizierung | [server.rs](https://github.com/JanHG98/netcore-tetra/blob/27996d2f494add5ae16837acaaba4f6f22f9f49d/crates/tetra-entities/src/net_dashboard/server.rs), `SessionStore` | Lokale Cookie-Sessions, sieben Tage Inaktivitätsfrist, prozessinterner Speicher | Noch kein Nachweis fein abgestufter RUA-Administrationsrechte |
| Web-Konfiguration | [sec_dashboard.rs](https://github.com/JanHG98/netcore-tetra/blob/27996d2f494add5ae16837acaaba4f6f22f9f49d/crates/tetra-config/src/bluestation/sec_dashboard.rs) | Quelltext-Defaults: Bind `0.0.0.0`, Port 8080, optionales Zugangsdatenpaar | Defaults sind kein Live-Port-/Auth-Nachweis; keine offenen Benutzerverwaltungs-Endpunkte hinzufügen |
| Aktuelle IAM-Planung | [CENTRAL_IDENTITY_RBAC_ROADMAP.md](https://github.com/JanHG98/netcore-tetra/blob/27996d2f494add5ae16837acaaba4f6f22f9f49d/Docs/CENTRAL_IDENTITY_RBAC_ROADMAP.md) | NETCORE-IAM-01, ausdrücklich geplant; zentraler Identity-Dienst als Empfehlung, Produkt-/Betriebsdetails offen | RUA-Fachverwaltung mit Web-IAM abstimmen, aber Funk-PIN und Web-Passwort nicht verschmelzen |

Der rekursiv abgerufene Repository-Baum enthielt keine RUA-benannten Pfade. Zusammen mit den geprüften Eingangs-, Zustands- und UI-Punkten wurde damit **keine fertige RUA-Implementierung nachgewiesen**. Das ist bewusst keine Behauptung, jede Zeile sämtlicher Dateien oder anderer Branches geprüft zu haben.

### 8.3 Wichtige zusätzliche Sicherheitsbefunde

Im vorhandenen `generate_session_token()` wird bei erfolgreichem Öffnen von `/dev/urandom` ein möglicher Fehler von `read_exact()` ignoriert; bei fehlendem Zugriff existiert ein aus Zeit und Prozess-ID abgeleiteter Fallback. Für eine Oberfläche mit PIN- und Benutzeradministration sollte diese Konstruktion nicht als ausreichende Absicherung übernommen werden. Ein belastbarer Zufallsquellenfehler muss zu einem definierten Fehler führen. Eine Korrektur ist noch offen.

Zusätzlich fiel ein unmittelbar SDS-relevanter Widerspruch auf: `SDS_TL_STATUS_UNDELIVERABLE` ist im geprüften `sds_bs.rs` auf `0x02` gesetzt und wird für einen Fehlerreport verwendet. EN 300 392-2 V3.8.1, Tabelle 29.16, Seite 1204, bezeichnet `0x02` jedoch als erfolgreiches „vom Ziel konsumiert“. Das ist ein **offener Prüf-/Fehlerkandidat**, kein hier getesteter Defekt am Endgerät. Für RUA dürfen weder dieser Wert noch der dort benutzte Text-PID unbesehen übernommen werden.

### 8.4 Geprüfte Blob-Identitäten

| Datei | Blob-SHA am Prüfstand |
|---|---|
| `crates/tetra-entities/src/cmce/subentities/sds_bs.rs` | `09533926ec7272a46813c295c0f3f67628dd750c` |
| `crates/tetra-saps/src/control/enums/sds_user_data.rs` | `39df4f708260cbdc9b2f28177cdea244c5ef6ca7` |
| `crates/tetra-config/src/bluestation/state.rs` | `da9fd0257fe09077f95d05d214fcf3a21159e5b9` |
| `crates/tetra-entities/src/mm/mm_bs.rs` | `17b290cbeb097b255ff78b5e8b8fe2f4537cd37f` |
| `crates/tetra-entities/src/net_dashboard/html.rs` | `caf9cd6932346141021ed2f4ef62fbc79562fece` |
| `crates/tetra-entities/src/net_dashboard/state.rs` | `8254ae2b69a03eebd0b70fc3cd90422919fe827a` |
| `crates/tetra-entities/src/net_dashboard/server.rs` | `a331ef1343013320077dd6975e450b55796cfb22` |
| `crates/tetra-entities/src/net_telemetry/events.rs` | `e98ce67c8690fad54bf81c36b87b0511527ad18a` |
| `crates/tetra-config/src/bluestation/sec_dashboard.rs` | `b0ace018ab8c3fdf9eba6c2f0001176237846de3` |
| `Docs/CENTRAL_IDENTITY_RBAC_ROADMAP.md` | `5ec152513f4b0c8127669c694ec9b804ff078c9a` |

Diese Werte dokumentieren gelesene Git-Objekte, keine von dieser Entwicklungsphase erzeugten Implementierungs-Commits. Ein einschlägiger RUA-PR wurde nicht nachgewiesen.

## 9. Vorgeschlagene Architektur für die Umsetzung

### 9.1 Verarbeitung und Zuständigkeiten

Die in den Arbeitsnotizen vorgeschlagene Trennung bleibt sinnvoll, muss aber mit der inzwischen vorhandenen zentralen SDS-Verarbeitung abgeglichen werden:

```text
Funkgerät / physische ISSI
    -> U-SDS-DATA / exakte Bitlänge
    -> frühe sichere Klassifizierung und redigierte Diagnose
    -> SDS-TL-Parser
    -> RUA-Parser des explizit unterstützten Profils
    -> asynchrone Benutzerprüfung / Policy / persistenter Store
    -> Assignment-Zustandsautomat
    -> erst bei bestätigter Codierung: RUA-Antwort über D-SDS-DATA
    -> redigierte Fachereignisse und Snapshot für WebUI / Control Room
```

Zu entscheiden ist, ob die RUA-Serverlogik lokal in der TBS läuft oder zentral hinter der TBS. **Eine im lokalen WebUI sichtbare Verwaltung schreibt nicht vor, wo die maßgebliche Benutzer- und Sitzungsdatenbank liegen muss.** Bei mehreren Zellen darf es nicht pro Station unkoordiniert konkurrierende Assignments geben.

Für jedes unterstützte Betriebsprofil braucht es genau einen zuständigen RUA-Verarbeiter. Lokaler Handler, zentraler SDS Router und Brew dürfen nicht dieselbe Anfrage mehrfach akzeptieren. Im Beobachtungsmodus sind normaler Betriebsverkehr und reine Anzeige zu trennen; ein lokal „PIN passend“ bewerteter Versuch darf nicht als am Funkgerät bestätigter Login erscheinen.

### 9.2 Modulskizzen

Es wurden zwei Ablagevarianten vorgeschlagen, aber keine beschlossen oder angelegt:

```text
Frühe Skizze:
  crates/tetra-pdus/src/sds/apps/rua.rs
  crates/tetra-entities/src/cmce/subentities/rua_bs.rs

Spätere Skizze:
  crates/tetra-entities/src/rua/
    mod.rs
    protocol.rs
    service.rs
    store.rs
    state.rs
    policy.rs
```

Die konkrete Umsetzung soll reine Bitcodierung, Transport-SDS-TL, Fachlogik und Speicherung trennen. Bestehende `BitBuffer`-APIs und PDU-Fehlertypen sind zu verwenden; die früher in den Arbeitsnotizen erfundenen Hilfsmethoden sind keine unmittelbar kompilierbaren Projekt-APIs.

Bei der Weiterarbeit außerdem relevant: `crates/tetra-pdus/src/mm/pdus/d_location_update_accept.rs`, `crates/tetra-entities/src/cmce/cmce_bs.rs`, `crates/tetra-entities/src/net_brew/entity.rs`, die Control-Commands sowie `bins/netcore-control-room/`. Ihre Änderung wäre ein späterer Entwicklungsauftrag außerhalb des Archivs.

### 9.3 Datenmodell

| Objekt | Geplante Inhalte | Grenzen |
|---|---|---|
| Funkbenutzer | Stabile interne ID, Anzeigename, RUI einschließlich Typ, Aktiv-/Sperrstatus, Credential-Version, optional Profil/RUN/P-ISSI | Keine Klartext-PIN und kein automatisch serialisiertes Geheimnis |
| Assignment | Benutzer-ID, Geräte-ISSI, Netz-/Stationskontext, Zustand, Anforderungs-/Bestätigungszeit, Ablauf, letzte Aktivität, Widerrufsgrund | Nicht nur im flüchtigen MM-Subscriber ablegen |
| Gerätepräsenz | Registriert/erreichbar, letzte Aktivität, zugehörige Zelle | Nicht mit erfolgreicher Benutzerauthentifizierung gleichsetzen |
| Anmeldeversuch | Zeit, Referenz, Geräteadresse, bekannte/zulässige Benutzerreferenz, Ergebnis, sicherer Fehlercode | Kein Rohtelegramm und keine PIN im Audit |
| Richtlinie | Mehrfachanmeldung, Ablauf, Sperren, Offline-Verhalten, Berechtigungen | Werte und Prioritäten müssen ausdrücklich entschieden werden |

Für Netzverbünde ist eine nackte ISSI gegebenenfalls nicht eindeutig genug. MNI/Netzkontext und Zellenzuständigkeit gehören deshalb als Roadmap-Kandidat in das Modell, ohne die bestehende Einzelzellenannahme stillschweigend zu erweitern.

### 9.4 Speicherung und Laufzeit

SQLite mit Tabellen für `users`, `assignments`, `login_attempts` und `audit_log` war die vorgeschlagene lokale Lösung. `rua.db` ist ein **Planungsname**, keine nachgewiesene Betriebsdatei. Ein dauerhafter Pfad außerhalb des Git-Arbeitsverzeichnisses, Zugriffsrechte, Migrationen, Backup, Restore und Aufbewahrung sind festzulegen.

Transaktionen und Eindeutigkeitsbedingungen sollen verhindern, dass parallele Requests mehrere ungewollte aktive Bindungen erzeugen. Datenbankwahl allein garantiert jedoch weder beliebige Stromausfallsicherheit noch einen korrekten Wiederanlauf. Insbesondere müssen tatsächlich konfigurierte Durability-Einstellungen und Restore-Verfahren getestet werden.

Passwort-/PIN-Hashprüfung und Datenbankzugriffe gehören nicht blockierend in den zeitkritischen Funk-/TDMA-Pfad. Vorzusehen sind eine begrenzte Worker-/Anfragewarteschlange, Deadlines und redigierte Ergebnisse zurück in den Zustandsautomaten. Keine unbeschränkte Thread-Erzeugung pro empfangener SDS.

### 9.5 Assignment- und Ausfalllogik

Im Gespräch wurden unter anderem die Zustände `Unassigned`, `Requested`, `Authenticating`, `Assigned`, `Limited`, `PseudoLoggedOn`, `ForceOffPending` und `Suspended` genannt. Das sind Entwurfsbegriffe; die endgültige Abbildung auf Protokollzustände fehlt.

Für das erste beobachtende Release sind ehrlichere UI-Zustände etwa „Anfrage beobachtet“, „lokal geprüft“, „Antwort nicht implementiert“ und „Gerät nicht erreichbar“. Erst ein belegter aktiver Ablauf darf „zugewiesen/angemeldet“ im protokollbezogenen Sinn setzen.

Zu definieren sind insbesondere: Behandlung verspäteter Nachrichten, Wiederholungen und Referenzüberlauf; expliziter Logoff gegenüber Funkverlust; Zeitablauf; Sperre während einer laufenden Prüfung; Neustart von TBS oder Backend; Gerätewechsel; DMO-/LST-Rückkehr; Rücknahme eines angeforderten Force Off bei fehlender Bestätigung. Persistierte absolute Zeiten und monotone Laufzeittimer erfüllen unterschiedliche Aufgaben.

## 10. WebUI, API und Rollen

### 10.1 Geplante Bedienoberfläche

Ein eigener Bereich „Funkbenutzer / RUA“ im bestehenden Basisstations-WebUI ist das bevorzugte Bedienkonzept. Unterseiten wurden für Benutzer, aktive Anmeldungen, Anmeldeversuche und Einstellungen vorgeschlagen.

| Ansicht | Felder / Funktionen |
|---|---|
| Benutzerliste | Anzeigename, RUI/Typ, Aktiv-/Sperrstatus, optional Profil, aktuell zugeordnetes Gerät, letzte erfolgreiche Anmeldung, Aktionen |
| Benutzerformular | Anzeigename, RUI, neue PIN mit Wiederholung, Status; spätere optionale Profil-/Nummern-/Gruppenfelder |
| Aktive Zuordnungen | Nutzer, physische Geräte-ISSI, separate Erreichbarkeit, tatsächlicher RUA-Zustand, seit wann, Ablauf, Quelle der Bestätigung |
| Gerätedetails | Bestehender Gerätename plus angemeldeter/angefragter Funkbenutzer; physische ISSI bleibt sichtbar |
| Anmeldeversuche | Zeitpunkt, Gerät, redigierte Benutzerreferenz, Ergebnis/Fehlerkategorie, Rate-Limit-/Sperrereignis |
| Einstellungen | Betriebsmodus, Serverzuständigkeit, Lebensdauern, Mehrfachanmeldung, Offline-Regeln, unterstütztes Protokollprofil |

Zusatzaktionen waren PIN setzen/ändern, Sperren/Entsperren, sofortige Abmeldung, Historie, Verlängerung, Book On und Löschung beziehungsweise Archivierung. Nicht implementierbare Downlink-Aktionen müssen deaktiviert oder klar als nicht unterstützt markiert bleiben; kein Button darf nur den UI-Eintrag löschen und dies als Funkgeräteabmeldung verkaufen.

Die neue Oberfläche soll die vorhandenen Design-Tokens, Größen-/Touch-Regeln und Live-Updates weiterverwenden. Für die aktuelle Struktur sind `ui/dashboard.html`, `ui/netcore.js` und `ui/netcore.css` die naheliegenden Integrationspunkte, nicht eine historische vollständig eingebettete HTML-Zeichenkette.

### 10.2 Vorgeschlagene HTTP-Endpunkte

Die folgenden Endpunkte waren **Entwurf, nicht vorhandene API**:

```text
GET    /api/rua/users
POST   /api/rua/users
PATCH  /api/rua/users/{id}
DELETE /api/rua/users/{id}
POST   /api/rua/users/{id}/pin
POST   /api/rua/users/{id}/lock
POST   /api/rua/users/{id}/unlock
GET    /api/rua/assignments
POST   /api/rua/assignments/{issi}/logoff
POST   /api/rua/assignments/{issi}/book-on
GET    /api/rua/audit
```

Die PIN ist ein ausschließlich schreibbares Eingabefeld. Lesende Antworten sollen höchstens anzeigen, ob eine PIN gesetzt ist, aber weder PIN noch Hash oder Rohtelegramm liefern. Für jede schreibende Aktion sind serverseitige Berechtigungsprüfung, Validierung, Audit und Schutz gegen unbeabsichtigte browserübergreifende Aufrufe vorzusehen. Versteckte Schaltflächen allein sind keine Autorisierung.

Es ist kein eigener RUA-Web-Port beschlossen. Eine Einbindung in den existierenden Dashboard-Server ist möglich; dessen Quelltext-Default 8080 ist nicht automatisch der tatsächlich verwendete Anlagenport. Ein separater zentraler Dienst wäre eine zusätzliche Architekturentscheidung.

### 10.3 Live-Daten und Control Room

Vorgeschlagen wurden `RuaLogonRequested`, `RuaLogonAccepted`, `RuaLogonRejected`, `RuaLoggedOff` beziehungsweise ein vereinheitlichtes `RuaAssignmentChanged`. Fachereignisse enthalten sichere IDs, Zustand, Ablauf und Grund, keine Authentifizierungsgeheimnisse.

Der bestehende WebSocket-Ansatz kann Snapshot und spätere Änderungen verteilen. Ein Reconnect benötigt einen vollständigen aktuellen Snapshot, damit fehlende Delta-Ereignisse keine falschen Zuordnungen hinterlassen. Gerätepräsenz und Benutzerzuordnung sollen getrennt aktualisierbar sein.

Das Anhängen neuer Varianten an `TelemetryEvent` vermeidet zwar eine Verschiebung bestehender Variantennummern, garantiert aber nicht, dass alte Empfänger unbekannte Varianten verarbeiten können. Versionierung, gemischte Softwarestände und die konkret eingesetzte Bitcode-/JSON-Übertragung müssen getestet werden.

### 10.4 Abgleich mit zentralem IAM

Die am 03.10.2026 vorhandene `Docs/CENTRAL_IDENTITY_RBAC_ROADMAP.md` ist selbst ein Planungsdokument. Sie empfiehlt einen zentralen Identity-Dienst und eine gemeinsame Integration, legt aber weder ein bereits installiertes Produkt noch den Betriebsort endgültig fest.

Für RUA ergibt sich daraus als neue Integrationsaufgabe: Die WebUI-Administration soll die künftigen Web-IAM-Rechte nutzen können, während RUA-Identitäten und Funk-PINs getrennte Fachobjekte bleiben. Ein Benutzer kann organisatorisch beiden Welten zugeordnet sein, ohne dass dieselben Geheimnisse oder automatisch dieselben Rechte verwendet werden. Die RUA-Integration bleibt ein eigener Entwicklungspunkt.

## 11. Sicherheits- und Policy-Anforderungen

### 11.1 Geheimnisschutz entlang des ganzen Datenwegs

Der frühere Code schrieb vollständige U-SDS-DATA-Bytes mit INFO ins Log; die Betriebslogs bestätigen diese Ausgabe. Der am 03.10.2026 geprüfte Pfad tut dies weiterhin. Zusätzlich sind Debug-Ausgaben des geparsten PDU, Binärdumps bei Parserfehlern, Dashboard-SDS-Logs und die neue vollständige Payload in `SdsEdgeIngress` zu berücksichtigen.

Die nötige Maßnahme ist nicht lediglich eine Logzeile „PIN redacted“ hinter dem bestehenden Dump. Vor jedem diagnostischen Export muss zuverlässig entschieden werden, ob vertrauliche Anwendungsdaten vorliegen. Bei einer noch nicht verständlichen RUA-Variante ist eine grobe Unterdrückung des Inhalts besser als eine fehlerhafte feldweise Maskierung. Gleichzeitig muss der geschützte Parser den Originalinhalt intern noch verarbeiten können.

Mögliche Diagnosefelder: Richtung, Geräteadresse, konfiguriertes Ziel, PID, tatsächliche Bitlänge, Nachrichtentyp soweit bekannt, Transportreferenz, Parser-/Policy-Ergebnis. Eine Laborschaltung für Rohcaptures war eine Idee; sie darf nicht standardmäßig aktiviert sein und ist kein Freibrief für Weiterleitung in normale Logs oder ein öffentliches Git-Repository.

### 11.2 PIN-Prüfung

Vorgeschlagen bleiben individuelle Salts, geeignete langsame Hashprüfung, gegebenenfalls ein separat verwalteter Pepper und begrenzte Fehlversuche. Dazu gehören Limits pro Benutzeridentität und Gerät sowie ein Gesamtlimit, weil wechselnde Kennungen sonst eine einzelne Sperre umgehen können. Hashingparameter müssen zum Zielsystem und zum begrenzten Worker-Pool passen.

Ein `SensitivePin`-/Secret-Wrapper soll unabsichtliches `Debug`, `Display`, Serialisieren und unnötiges Kopieren verhindern. Die früheren Codebeispiele mit `pub pin: u32` und abgeleitetem `Debug` sind ausdrücklich **nicht als sicherer Implementierungsvorschlag zu übernehmen**.

Hashspeicherung allein löst weder die schwache Entropie kurzer numerischer PINs noch Replay, sichere Transportwege, Schutz vor Rohdumps oder Zugriff auf Datenbanksicherungen. Diese Punkte sind eigenständige Abnahmekriterien.

### 11.3 Sperren, Ändern, Löschen und Doppelanmeldungen

| Aktion | Vor Umsetzung zu entscheidende Semantik |
|---|---|
| Benutzer sperren | Nur neue Anmeldungen verhindern oder zusätzlich bestehende Zuordnung widerrufen? Wie wird eine fehlende Endgerätebestätigung dargestellt? |
| PIN ändern | Bestehende Sitzung weiter gültig lassen oder neue Anmeldung erzwingen? Wie werden parallel laufende Prüfungen ungültig? |
| Benutzer löschen | Sofortiges Hard Delete oder Archivierung mit minimaler Audit-Referenz? Welche Löschfristen gelten? |
| Gerätewechsel | Neue Anmeldung ablehnen, bestätigte alte Bindung ersetzen oder Mehrfachanmeldung erlauben? |
| Force Off | Lokale Entscheidung, Nachricht versendet und Endgerät bestätigt sind getrennte Zustände |
| Wiederanlauf | Welche Zuordnungen sind nur historische Information, welche müssen erneut bestätigt werden? |

Ein Gerätewechsel darf die bisherige Sitzung nicht schon vor erfolgreicher Benutzerprüfung zerstören. Auch bei einer späteren Takeover-Policy müssen beide Seiten und verspätete Antworten konsistent behandelt werden.

### 11.4 Funkrechte und logische Erreichbarkeit

Ein späterer vollständiger RUA-Dienst soll Kommunikationsrechte aus Gerät, Benutzer, Profil und Betriebszustand kontrolliert ermitteln. In den Arbeitsnotizen wurden Gruppenruf, Einzelruf, SDS, Gruppenanmeldung, Paketdaten, Priorität und externe Gateways genannt. Das ist ein größerer Ausbau, nicht eine bereits vorhandene Funktion des Benutzerformulars.

Limited Service darf nicht nur eine Anzeige am Motorola sein. Umgekehrt darf eine nicht fertig implementierte RUA-Prüfung den bestehenden Funkbetrieb nicht ungeplant sperren. Notruf-/Sicherheitsverkehr und ausfallbedingte Grundrechte benötigen eine explizite, getestete Policy.

Die spätere Auflösung RUN/P-ISSI zu aktueller Geräte-ISSI erfordert ein konsistentes Nummern- und Routingmodell einschließlich Brew und Control Room. In Logs sollen physische Geräteidentität und logische Benutzeridentität nebeneinander erhalten bleiben. Eine reine Umbenennung der physischen ISSI genügt nicht.

## 12. Konfiguration, Dienste und technische Parameter

### 12.1 Bestätigte beziehungsweise aus dem Code gelesene Parameter

| Parameter | Wert | Nachweisgrenze |
|---|---|---|
| Beobachtetes Endgerät | ISSI `2020004` | Betriebslog; keine geprüfte Erreichbarkeitsprüfung |
| Beobachtetes SDS-Ziel | `16777213` / `0xFFFFFD` | Für diesen Codeplug/Ablauf belegt, nicht als universelle RUA-Adresse festgelegt |
| Beobachteter PID | `193` / `0xC1` | Für dieses Profil belegt; allgemeine Norm weist nur den benutzerdefinierten SDS-TL-Bereich aus |
| Brew-Transport | WebSocket, historisch `ws://10.0.1.22:8081` | Versandlog belegt; keine geprüfte Verbindung hergestellt |
| SDS-Datenart | SDS Type 4; im RF-Log Identifier 3 | Code und Betriebslogs |
| Zentrale Telemetrie-Datenart | `SdsEdgeIngress.sds_type` verwendet 1 bis 4; 0 ist Status | Nicht dieselbe Zählweise wie der historische RF-Type-Identifier |
| Dashboard-Default | TCP 8080, Bind `0.0.0.0` | Quelltext-Default, keine laufende Konfiguration ausgelesen |
| Lokale Control-/Dashboard-ISSI im SDS-Modul | `4010001` | Vorhandene Konstante; nicht ungeprüft als RUA-Absender verwenden |
| Generische SDS-Deferral-Deadline | 10 Sekunden | Vorhandener Code; keine RUA-spezifische Timeout-Abnahme |
| Web-Session-Inaktivitätsfrist | 7 Tage | Lokaler SessionStore im geprüften Source, kein zentrales IAM |
| Bisherige persistierte SDS-Historie | `sds_log.json` neben aktiver Konfiguration | Codebeschreibung; nicht als RUA-Datenbank geeignet oder geprüft |

### 12.2 Diskutierte, noch nicht implementierte RUA-Einstellungen

Im Verlauf wechselten einzelne Schlüsselnamen. Die folgende Zusammenfassung ist deshalb ein **Konfigurationsentwurf und kein kopierfertiger gültiger TOML-Block**:

| Entwurfsparameter | Diskutierte Bedeutung |
|---|---|
| `enabled` / `mode` | Deaktiviert, beobachtend oder aktiv; aktive Antwort erst nach bestätigter Codierung |
| `protocol_id` bzw. `protocol_identifier` | Beobachtetes Profil zunächst 193; nicht an mehreren Stellen hardcoden |
| `service_ssis` bzw. `service_ssi` | Konfigurierte lokale/zentral terminierte Diensteadresse; beobachtet 16777213 |
| `redact_payloads` | Standardmäßig aktiv |
| `unsafe_trace_payloads` | Standardmäßig aus; nur kontrollierter gesonderter Labormodus als Idee |
| `request_on_registration` | Bis zur bestätigten IE-Codierung aus |
| `request_identity_type` | Alpha wurde wegen der vorliegenden Namen diskutiert; Wertezuordnung noch zu prüfen |
| `required` / `limited_service_without_login` | Umfang der Funkrechte bei fehlendem Login; Entscheidung und Regressionstest erforderlich |
| `default_assignment_secs` | Beispiel 28800 Sekunden, keine vereinbarte Dauer |
| Offline-Grace | Beispiel 60 Sekunden, keine implementierte oder beschlossene Frist |
| `duplicate_login_policy` | Ablehnen, Takeover oder Mehrfachanmeldung; keine endgültige Wahl |

Die frühe TOML-Skizze `[[rua.users]]` wurde später zugunsten einer verwalteten Datenbank als weniger geeigneter Ansatz eingeordnet. Die DB-Empfehlung wurde aber nicht als fertig implementierte Migration beschlossen.

## 13. Tests, Befehle und tatsächlicher Ausführungsstand

### 13.1 Belegte Tests

Die gelieferten SDS-Ereignisse belegen, dass Bedienhandlungen am Funkgerät zu empfangenen Nachrichten führten und dass der damalige Code zumindest die ersten Nachrichten in Richtung Brew weitergab. Die Vergleichsserien isolieren Namens- und PIN-Änderungen sowie die Namenslänge.

Nicht belegt sind eine positive oder negative RUA-Serverantwort, deren Anzeige am Endgerät, eine akzeptierte Assignment-Dauer, ein Profilwechsel oder funktionierende RUA-Rechtesperren. Ein ausdrücklich als „falsche PIN“ eingegebener Wert wäre ohne aktive Gegenstelle noch kein getesteter Reject.

### 13.2 Während der Archivierung ausgeführte Offline-Prüfung

Ein lokaler Python-Prüflauf verwendete die Originaldaten ausschließlich im Arbeitsspeicher. Exportiert wurde nur ein geheimnisfreier Ergebnisbericht. Er prüfte die angegebenen MSB-first-Fenster, die Namenslänge, die Namensextraktion, die numerische PIN-Interpretation sowie die Wiederzusammensetzung unter derselben Arbeitshypothese.

| Prüfung | Ergebnis |
|---|---|
| Namen aus dreizehn Logon-Beispielen | 13/13 entsprechen der jeweiligen Angabe |
| Numerisches PIN-Fenster | 12/13 entsprechen der jeweiligen Angabe; erster Datensatz mit Differenz minus drei offen |
| Wiederzusammensetzung aus den extrahierten Feldern | 13/13 reproduzieren die Speicherbits unter dem gewählten Schema |
| Kandidaten Typ-/Codierungswerte | In allen Logon-Beispielen drei Bit mit Wert 4, danach sieben Bit mit Wert 26 |
| Logon-Bitlängen | Fünf Originalangaben geprüft; acht Längen nur aus dem Schema abgeleitet |
| Logoff-Muster | Kurzes 33-Bit-Format nachvollzogen; kein vollständiger Protokollablauf getestet |
| Rust-Unit-/Integrationstests | Nicht ausgeführt |
| RF-/Motorola-Interoperabilitätstest | Nicht ausgeführt |

Ein Roundtrip mit demselben hypothetischen Layout kann eine falsche gemeinsame Annahme nicht widerlegen. Er ist ein Reproduzierbarkeitsnachweis, kein Konformitätszertifikat. Ebenso ist die Zählung passender Feldwerte nicht gleichbedeutend mit einer korrekten Authentifizierungsimplementierung.

### 13.3 Repository-, Datei- und Quellenoperationen

| Operation | Tatsächlicher Status |
|---|---|
| Lesen von `Archiving`, Archivindex, Baum und ausgewählten Quelldateien über die Repository-API | Erfolgreich; Prüfstand in Abschnitt 1 und Blob-Liste in Abschnitt 8 |
| Lokaler Cloneversuch mit `git clone --single-branch --branch Archiving --depth 1 ...` | Fehlgeschlagen: DNS-Auflösung von `github.com` im Container nicht möglich; kein lokaler Repo-Clone als Prüfgrundlage verwendet |
| PDF-Metadaten und Volltext über Files/PyMuPDF | Erfolgreich für 25 Dateien; kein OCR eingesetzt |
| Gerenderte Tabellenprüfung | Ausgewählte einschlägige PDF-Seiten geprüft; bei nicht bereitgestellten Files-Bildern lokal gerendert |
| Erster Versuch, den lokalen Ergebnisbericht zu speichern | Dateisystemberechtigung verhinderte das Schreiben; Arbeitsverzeichnisrechte angepasst und der geheimnisfreie Prüfbericht anschließend erfolgreich geschrieben |
| Öffnen der ETSI TR 102 300-5 über WWW | Erfolgreich; Annex-B-Text erneut geprüft |
| Zugriff auf eine versuchte TCCA-Webseite | Fehlgeschlagen; daraus keine Aussage über aktuelle Mitgliedschaftspflicht oder generelle Dokumentverfügbarkeit ableiten |
| Compiler, Installation, `systemctl`, Datenbankmigration, Funkkonfiguration | Nicht ausgeführt |
| Downlink-Testtelegramme / Force Off / Book On | Nicht ausgesendet |

### 13.4 Reproduzierbare spätere Entwicklerprüfungen

Die folgenden Befehle sind **Prüfvorschläge für eine vorhandene, berechtigte Entwicklerkopie**, keine hier erfolgreich ausgeführten Installationsschritte:

```bash
git fetch origin Archiving
git show origin/Archiving:Docs/archive/README.md
git grep -n -E 'RUA|RUI|rua|SdsEdgeIngress' origin/Archiving -- crates bins system-backend
git show origin/Archiving:crates/tetra-entities/src/net_dashboard/html.rs
git show origin/Archiving:crates/tetra-entities/src/cmce/subentities/sds_bs.rs
```

Ein späterer Rust-Testaufruf ist anhand der dann tatsächlich angelegten Crates und Testnamen festzulegen. Die früheren Parser-Skizzen verwenden nicht belegte Methoden wie `read_u8_at` und sind nicht als bereits kompilierter Rust-Code zu archivieren. Neue öffentliche Regressionstests sollen eigenständig erzeugte, eindeutig fiktive Fixtures verwenden, keine früheren Zugangsdaten der Projektplanung.

## 14. Fehler, überholte Aussagen und verworfene Ansätze

| Frühere Aussage / Ansatz | Am 03.10.2026 festzuhaltende Korrektur oder Grenze |
|---|---|
| `type=3` bedeute SDS Type 3 | Falsch: Im konkreten Enum ist Identifier 3 SDS Type 4. |
| Byte an Index 2 könnte RUA-PDU-Typ sein | Die Veränderung gehört zur SDS-TL Message reference; das Anwendungskennbyte folgt danach. |
| Name/PIN seien nicht direkt binär enthalten | Für die später ausgewerteten Beispiele widerlegt beziehungsweise überholt: bitverschobene Namensdaten und ein passendes numerisches Fenster sind lesbar. |
| „Uplink vollständig geknackt“ | Zu stark: nur eingeschränktes Profil, ein PIN-Widerspruch, keine nichtlateinischen Daten, keine vollständige Normzuordnung und kein Downlink-Test. |
| `0x86` und `10` seien sicher zwei feste Metadatenfelder | Nur beobachtete Bitfolge; der Abgleich legt eine 3+7-Bit-Aufteilung nahe. |
| Letztes gültiges Nullbit sei sicher Extension-/Abschlussbit | Semantik nicht bestätigt. |
| Neue Längenserie habe nachweislich 87 bis 199 gültige Bits | Nur Hexbytes geliefert; diese Bitlängen sind eine passende Ableitung. |
| Alle Pakete hätten ein Paddingbit | Nicht die 33-Bit-Abmeldenachricht: fünf Speicherbytes enthalten dort sieben unbenutzte Bits. |
| Maximal 15 Zeichen seien Motorola-Grenze | Nur plausible Profil-/Stringformatgrenze, kein Endgerätetest mit 15/16 Zeichen. |
| RUI sei eventuell nur ein numerischer Wert hinter einem lokalen Namen | Die vorhandenen Alpha-Beispiele enthalten die eingegebenen Namen reproduzierbar; andere Identitätstypen bleiben möglich. |
| `0xFFFFFD` sei eine universelle, fest standardisierte RUA-Serveradresse | Für den konkreten Ablauf beobachtet; generische Adressdarstellungen belegen keine universelle RUA-Zuordnung. Konfigurierbar halten. |
| Jede Nachricht an `0xFFFFFD` sei RUA | Unsicher und fachlich falsch als Erkennungsregel: PID, SDS-TL, Struktur, Richtung und Kontext zusätzlich prüfen. |
| Ein Transport-ACK oder lokaler Datenbanktreffer mache den Login erfolgreich | Keine protokollbestätigte Benutzerzuweisung; UI muss den Unterschied anzeigen. |
| User/PIN direkt im `Subscriber` speichern | Verworfen zugunsten getrennter Benutzer-/Assignment-Objekte; Registry wird bei Registrierung neu aufgebaut. |
| Benutzerverwaltung in Klartext-TOML / einfache JSON-Datei | Im Gespräch zugunsten transaktionaler Speicherung weniger geeignet beurteilt; kein fertiger Migrationsbeschluss. |
| SQLite verhindere jede Beschädigung bei Stromausfall | Unzulässige Garantie; Durability, Dateisystem, Speichermedium und Wiederherstellung müssen getestet werden. |
| RUA-Nachrichten im Beobachtungsmodus beliebig zu Brew spiegeln | Ohne klare Zuständigkeit und geschützten vertraulichen Transport nicht als Standard übernehmen. |
| `#[derive(Debug, Clone)]` auf einer Logon-Struktur mit offenem PIN-Feld | Würde Geheimnisse wieder exponieren; keine sichere Referenzimplementierung. |
| Für das Funkgerät fehle „nur noch eine Antwort“ | Zusätzlich fehlen Zustandsautomat, Timeout-/Duplikatlogik, sichere Administration, Rechtepolitik und Interoperabilitätsabnahme. |
| Aktuelle Oberfläche weiterhin vollständig in `html.rs` | Überholt: separater `ui/`-Assetaufbau. |
| Aktueller SDS-Routingpfad bestehe nur aus Lokalzustellung und Brew | Überholt: zusätzlicher zentraler Handoff mit Ausfallpfaden. |
| Keine öffentliche RUA-Codierung gefunden bedeute, sie existiere ausschließlich hinter einer Bezahlschranke | Nicht bewiesen. Bisher nicht gefunden ist ein Suchstand, keine globale Verfügbarkeitsaussage. |

## 15. Roadmap-Kandidaten und nächste Schritte

Die Prioritäten sind **Empfehlungen für die Fortsetzung**, keine bereits ausgeführten oder zeitlich zugesagten Arbeiten. Sie werden ausschließlich in diesem Archiv erfasst; bestehende Roadmap-Dateien werden nicht geändert.

| ID | Priorität | Arbeitspaket | Abhängigkeiten / Abnahme |
|---|---|---|---|
| RUA-01 | P0 | Vertrauliche SDS früh klassifizieren; INFO-, DEBUG-, Fehler- und Weiterleitungs-/Spool-Pfade prüfen | Kein echter PIN und kein reversibles Originaltelegramm in Logs, Browserdaten oder Standardexporten; Regressionstest auch für beschädigte Pakete |
| RUA-02 | P0 | SDS-TL-Header sauber vom RUA-Anwendungsprofil trennen | Headerfelder, Referenz, optionale Felder und tatsächliche Bitlänge korrekt behandeln; Transportreport nicht als Login-Accept werten |
| RUA-03 | P0 | Eingeschränkten Uplink-Parser mit synthetischen Fixtures implementieren | Namenslänge, 16-Bit-Codierung, numerisches Feld, Bounds und unbekannte Varianten testen; PIN-Widerspruch nicht kaschieren |
| RUA-04 | P0 | TTR 001-17 / IOP 001-17 und passende Motorola-Unterlagen beschaffen oder öffentlich belastbare Implementierung/Decoder finden | Versionsstand und Nutzbarkeit prüfen; genaue Downlink-PDUs, Timer, Identitätstypen und Registrierungserweiterung belegen |
| RUA-05 | P0/P1 | Serverzuständigkeit lokal/zentral und Abgrenzung zu SDS Router, Brew und Web-IAM entscheiden | Ein zuständiger Assignment-Verarbeiter; keine doppelten Antworten, keine unkontrollierte Credential-Spool |
| RUA-06 | P1 | Benutzer-Store, PIN-Kanonisierung, Hashprüfung, Sperren und Audit implementieren | Persistenz, Konflikte, begrenzte Worker, Backup/Restore und Rechte prüfen; keine Geheimnisrückgabe |
| RUA-07 | P1 | WebUI-CRUD und beobachtete Gerätezuordnung in aktuelle Assets integrieren | Anforderung funktional erfüllt; Status „beobachtet/lokal geprüft“ korrekt; API und UI autorisiert |
| RUA-08 | P1 | Verifizierten aktiven Logon-/Reject-/Logoff-Ablauf ergänzen | Bestätigte Codierung aus RUA-04, Testfunkgerät, tatsächliche Anzeige-/Antwortprüfung, Latenz-/Retry-/Duplikatfälle |
| RUA-09 | P1 | Assignment-Lebenszyklus und Sperrwirkung abnehmen | Ablauf, Gerätewechsel, Neustart, Coverageverlust, Sperre während Prüfung und verspätete Antworten konsistent |
| RUA-10 | P2 | Book On, Force Off, Verlängerung und Profile | Erst nach bestätigtem Grundablauf; gesendet/bestätigt/fehlgeschlagen getrennt |
| RUA-11 | P2 | Kommunikationsrechte, Limited Service, RUN/P-ISSI-Routing, Control Room/Brew | Separates Rechte-/Nummernkonzept; Notruf- und Bestandsfunkregressionen prüfen |
| RUA-12 | P2 | Mehrzellenbetrieb, netzweite eindeutige Identitäten und Ausfallstrategie | Autoritative Bindung und Replikations-/Cache-Regeln; keine pro Zelle widersprüchlichen Benutzerzustände |
| RUA-13 | P1, angrenzend | Vorhandenen SDS-Fehlerreportstatus `0x02` gegen Norm und Endgeräte testen | Befund aus Abschnitt 8.3 bestätigen und in separatem Codeauftrag korrigieren; keine Änderung durch dieses Archiv |

### 15.1 Praktische Reihenfolge

Zuerst Geheimnisschutz und ein sauber begrenzter Parser. Parallel dazu dürfen Store, Verwaltungs-API und Oberfläche aufgebaut werden, solange sie den Beobachtungsstatus ehrlich kennzeichnen. Die aktive Funkantwort bleibt deaktiviert, bis ihre Codierung und ihr Verhalten bestätigt sind. Anschließend folgen Assignment-Lebenszyklus und erst danach die weitreichenden Rechte-/Routingfunktionen.

Die früher vorgeschlagene Zweiteilung in „RUI Presence / Benutzeranzeige“ und „vollständiger RUA Application Server“ bleibt als Release-Abgrenzung brauchbar. Sie ist keine Zusage, dass ein erster Release bereits existiert.

### 15.2 Noch erforderliche Testfälle

| Testbereich | Geplante Prüfungen |
|---|---|
| Eingaben | Gleiche Eingaben mehrfach mit wechselnder Transportreferenz; einzelne Namensstelle ändern; einzelne PIN-Stelle ändern; zulässige PIN-Längen und führende Nullen |
| Stringformat | Grenzen 15/16 Einheiten, leere Kennung soweit zulässig, Umlaute, weitere Alphabete, ungültige UTF-16-Einheiten, Groß-/Kleinschreibung und Normalisierung |
| Parser | Zu kurz, abgeschnitten, zusätzliche gültige Bits, falscher PID, unbekannter Transporttyp, unbekanntes Anwendungskennbyte, Erweiterungen und Padding |
| Authentifizierung | Unbekannter Benutzer, gesperrter Benutzer, falsche PIN, Rate-Limits, gleichzeitige Login-/PIN-Änderung, kein Geheimnis in Fehlerausgabe |
| Sitzungen | Doppelanmeldung, Geräteübernahme, expliziter Logoff, Ablauf, verspäteter Accept, Referenzüberlauf, idempotente Wiederholung |
| Betrieb | T351, Funkverlust und Rückkehr, TBS-/Backend-Neustart, DMO-/LST-Übergänge, isolierte Zelle, Zentraldienst-Ausfall |
| Luftschnittstelle | Reale Accept-/Reject-/Cancel-Anzeige, Idle und laufender Ruf, Energy Economy, Haupt-/Neben-Carrier soweit im System unterstützt |
| WebUI/API | Rollen und Ressourcen, CSRF-/Origin-Schutz, Eingabevalidierung, Reconnect-Snapshot, PIN nie lesbar, Sperraktion mit nachvollziehbarem Status |
| Regression | Normale Text-SDS, Status, lokale Sonderdienste, Brew-/Zentralrouting, Gruppen-/Einzelruf und Notruf dürfen nicht unbeabsichtigt verändert werden |

Eine zusätzliche PEI-Beobachtung über einen herstellerspezifisch unterstützten `+CTRUA`-Befehl bleibt eine Diagnoseidee. Befehl, Syntax und Verfügbarkeit am konkreten Endgerät sind vorher zu belegen. Ein kommerzieller RUA-Server ist keine Voraussetzung.

## 16. Offene Punkte und explizite Lücken

Es fehlen weiterhin:

- Kalendertag der ursprünglichen Funklogs.
- Exaktes Funkgerätmodell, Firmware, Codeplug-Einstellungen und beobachtete Displayzustände bei den einzelnen Versuchen.
- Unabhängige gültige SDS-Bitlängen der acht zuletzt nur als Hex gelieferten Längentests.
- Auflösung der Abweichung zwischen erstem numerischen Feld und genannter Eingabe.
- Vollständige normative beziehungsweise herstellerbestätigte RUA-Feldgrenzen, Bedeutung des letzten gültigen Nullbits und unterstützte Varianten.
- Bestätigte Accept-, Reject-, Cancel-/Force-Off-, Book-On- und gegebenenfalls Erneuerungs-PDUs samt Timer- und Referenzregeln.
- Bitgenaue RUA-Registrierungserweiterung; die generische Existenz eines proprietären Informationselements genügt nicht.
- Entscheidung über lokale oder zentrale RUA-Serverzuständigkeit, Speicherort, Löschfristen, Mehrfachanmeldung, Offline- und Rechtepolitik.
- Ein Build-/Installationsnachweis für RUA; weder lokale Rust-Tests noch eine produktive Softwareversion oder ein RUA-PR sind belegt.
- Am 03.10.2026 geprüfte Live-Konfiguration und Erreichbarkeit der TBS und des Brew-Servers; Repository-Source ist kein Betriebsabzug.
- Vollständige fachliche Prüfung aller Standards und aller Repository-Dateien; die dokumentierte Suche und die gezielten Lesefenster sind der tatsächliche Umfang.

**Offen bleibt die Protokollimplementierung.** Die vorhandenen Entwicklungsnotizen schließen keine der genannten RUA-Protokoll- oder Abnahmelücken.

## 17. Quellen und Anhänge

### Q1. Erneut geprüfte öffentliche Architekturquelle

[ETSI TR 102 300-5 V1.4.1 (2015-06), Designers’ guide, Part 5: Guidance on numbering and addressing](https://www.etsi.org/deliver/etsi_tr/102300_102399/10230005/01.04.01_60/tr_10230005v010401p.pdf), insbesondere Annex B, Seiten 34–37. Bei der Archivierung über die ETSI-Adresse erneut geöffnet und einschlägige Textpassagen gelesen. Maßgeblich ist die Versionsangabe im PDF, nicht ein abweichender Titel in Such-/Abrufmetadaten.

Diese Quelle stützt das Prinzip Benutzer-zu-Gerät-Zuordnung, SDS-Authentifizierung und Rückmeldung. Sie liefert keinen in dieser Prüfung verwendbaren bitgenauen RUA-Downlink-Encoder.

### Q2. Historische Recherchehinweise, nicht neu als vollständig verifiziert ausgeben

| Historischer Quellenhinweis | Bedeutung und verbleibende Prüfung |
|---|---|
| TTR 001-17, „TETRA Interoperability Profile; Radio User Assignment; Phase 1“ | Als maßgebliche Profilspur genannt. Die in den Arbeitsnotizen genannte Version 1.1.0 vom 17.11.2016 ist hier kein erneut bestätigter aktueller Veröffentlichungsstand. |
| IOP 001-17 | Genannter Interoperabilitäts-Testplan; Inhalt und Verfügbarkeit nicht vorliegend. |
| [TIP-Verzeichnis bei Oborne Consulting](https://www.oborneconsulting.co.uk/TETRA_Standards/TIP-TTR001.htm) | Historischer Kataloghinweis; kein Ersatz für den Spezifikationstext. |
| [Motorola-Handbuchspiegel](https://manuals.plus/m/473fbd9f73160dc68ce4499dd080689cb082e197b051ae9e4f65ba93a4b7132f) | In den Arbeitsnotizen als Quelle für RUA, Assignment-Dauer und `+CTRUA` genannt. Konkrete Ausgabe, Feldtabellen und Originalherkunft erneut prüfen. |
| [Airbus-Unterlage als Drittanbieter-Upload](https://www.scribd.com/document/721722061/00126593) | Historische Spur für Alias-/Profilverwaltung; keine verifizierte Funk-PDU-Tabelle. |
| [VIRVE-Unterlage](https://www.doria.fi/bitstream/handle/10024/129991/lo_2016-36_virve_network_web.pdf?isAllowed=y&sequence=2) | In den Arbeitsnotizen für RUA Accept/Reject/Cancel und systembezogene Adressen genannt; genaue Abschnitte und Übertragbarkeit sind weiter zu prüfen. |
| [RFE-MultiAnalyzer-Datenblatt](https://www.dmrassociation.org/femvenner/DS_MAS_555_MultiAnalyzer_rfe_en.pdf) | Historischer Hinweis auf einen kommerziellen TIP-Decoder, nicht auf verfügbaren Quellcode oder nutzbare Testvektoren. |

Zu den historischen, unbestätigten Detailangaben gehören `+CTRUA`-Werte 0 für keine neue Zuweisung, 1 RUN, 2 SSI, 3 MS-ISDN, 4 Alpha-Tag, 5 permanente Abmeldung sowie 6/7 reserviert; ebenso Profil-ID 0–255, mögliche Bedeutung von Profil 0, Pseudo-Logon und die Wirkung eines unaufgeforderten Accept als Book On. Diese Angaben bleiben **Recherchekandidaten**, keine implementierbaren Luftschnittstellenkonstanten.

Die früher berichtete Suche nach `RUA Accept PDU`, `RUA Reject PDU`, `RUA Cancel PDU`, `Motorola RUA SDS`, `TTR 001-17`, `IOP 001-17`, `CR199` und `CR286` lieferte in den Arbeitsnotizen keine dokumentierte vollständige Codierung. Für die beiden zuletzt genannten CR-Nummern liegt kein belastbarer Inhalt vor. Die Aussage, alle relevanten Tabellen seien ausschließlich im Mitgliederbereich erhältlich, wurde bei der Quellenprüfung vom 03.10.2026 nicht bestätigt.

### Q3. Bereitgestellte PDF-Dateien

Die Dateien wurden im Arbeitscontainer unter `/mnt/data/` bereitgestellt. Dieser Pfad ist eine temporäre Arbeitsumgebung, kein Installations- oder Repository-Pfad. Die PDFs selbst werden nicht in `Docs/archive/` kopiert; das Archiv enthält nur das Inventar und gezielte Quellenbezüge.

| Datei | Dokument / Ausgabe laut Inhalt | Seiten | Relevanz für das Vorhaben |
|---|---|---:|---|
| `en_3003920308v010401p.pdf` | EN 300 392-3-8 V1.4.1, 2020-04 | 22 | ISI Generic Speech Format; Randbezug, kein RUA-Decoder |
| `en_30039209v010701p.pdf` | EN 300 392-9 V1.7.1, 2020-04 | 46 | Allgemeine SS-Anforderungen; Stringcodierung auf Seite 24 gezielt geprüft |
| `ts_10081201v020205p.pdf` | TS 100 812-1 V2.2.5, 2003-10 | 8 | UICC-Physik/Logik; nicht RUA-Login über SDS |
| `en_3003921201v010202p.pdf` | EN 300 392-12-1 V1.2.2, 2007-08 | 56 | Call Identification, Stage 3; möglicher späterer Identitätsbezug |
| `en_3003920304v010301p.pdf` | EN 300 392-3-4 V1.3.1, 2010-08 | 28 | ISI-SDS-Transport, nicht die RUA-Anwendung |
| `en_3003921117v010102p.pdf` | EN 300 392-11-17 V1.1.2, 2002-01 | 18 | Include Call, Stage 2; ausdrücklich nicht TTR 001-17 |
| `en_3003921114v010101p.pdf` | EN 300 392-11-14 V1.1.1, 2002-07 | 23 | Late Entry, Stage 2; Randbezug |
| `es_20081202v020401m.pdf` | Final draft ES 200 812-2 V2.4.1, 2005-08 | 139 | TSIM-Anwendung; TSIM-PIN-Verfahren nicht mit RUA-PIN verwechseln |
| `es_20081201v020205p.pdf` | ES 200 812-1 V2.2.5, 2003-12 | 8 | UICC-Physik/Logik; Randbezug |
| `en_300812v020101p.pdf` | EN 300 812 V2.1.1, 2001-12 | 156 | SIM-ME-Schnittstelle und Security; nicht der gesuchte Serverdialog |
| `en_3003921101v010201p.pdf` | EN 300 392-11-1 V1.2.1, 2004-01 | 44 | Call Identification, Stage 2 |
| `en_3003921006v010401p.pdf` | EN 300 392-10-6 V1.4.1, 2006-08 | 20 | Call Authorized by Dispatcher; späterer Rechtebezug |
| `en_3003921018v010301p.pdf` | EN 300 392-10-18 V1.3.1, 2003-10 | 17 | Barring of Outgoing Calls; nicht gleichbedeutend mit Benutzer-Logoff |
| `en_3003921216v010400a.pdf` | DRAFT EN 300 392-12-16 V1.4.0, 2026-03 | 67 | Pre-emptive Priority Call; Entwurfsstatus nicht als verabschiedeten Standard ausgeben |
| `en_30039201v010601p.pdf` | EN 300 392-1 V1.6.1, 2020-04 | 182 | Allgemeines Netzdesign und Identitäten |
| `ets_30039214e01v.pdf` | Final draft prETS 300 392-14, 1997-09 | 61 | PICS-Proforma; keine absolvierte RUA-Konformitätsprüfung |
| `en_30039207v030501p.pdf` | EN 300 392-7 V3.5.1, 2019-07 | 216 | TETRA-Security; Geräteauthentifizierung und Schlüsselverwaltung getrennt von RUA behandeln |
| `en_30039401v030301p.pdf` | EN 300 394-1 V3.3.1, 2015-04 | 169 | Funk-Konformität; keine RUA-Accept-Feldtabelle gefunden |
| `en_3003920313v010201p.pdf` | EN 300 392-3-13 V1.2.1, 2020-04 | 191 | Transportunabhängiger ISI-Gruppenruf |
| `en_30039502v010303p.pdf` | EN 300 395-2 V1.3.3, 2025-02 | 94 | TETRA-Sprachcodec; kein unmittelbarer RUA-Bezug |
| `en_3003920303v010301p.pdf` | EN 300 392-3-3 V1.3.1, 2011-11 | 251 | ISI-Gruppenruf |
| `en_30039205v020701p.pdf` | EN 300 392-5 V2.7.1, 2020-04 | 320 | Generische PEI-Norm; kein `+CTRUA`-Treffer in der gezielten Volltextsuche |
| `en_3003920315v010500a.pdf` | Draft EN 300 392-3-15 V1.5.0, 2026-04 | 380 | Transportunabhängiges ISI-MM; Entwurf, kein Beleg einer RUA-Implementierung |
| `en_30039202v030801p.pdf` | EN 300 392-2 V3.8.1, 2016-08 | 1.445 | Hauptquelle für SDS-TL, Textcodierung und zurückgezogenen RUA-CR |
| `ETSI.pdf` | Sammeldatei, beginnt mit EN 300 812 V2.1.1 | 4.100 | Enthält Standards erneut; kein eigenständiger Beleg einer zusätzlichen RUA-Spezifikation |

Die 24 Einzeldateien ergeben 3.961 Seiten; zusammen mit der Sammeldatei wurden 8.061 Seiteninstanzen textuell durchsucht. Die Sammeldatei ist nicht als 4.100 neue, voneinander unabhängige Seiten Fachinformation zu zählen. Ihre vollständige Zusammenstellung wurde nicht rekonstruiert.

Gezielte Suchbegriffe waren `RUA`, `RUI`, `Radio User Assignment`, `CTRUA` und `TTR 001-17` einschließlich Varianten. Die bedeutenden RUA-Treffer lagen in der Air-Interface-Datei auf Seiten 63 und 1437 sowie in der Sammeldatei auf Seiten 2508 und 3882. Aus dem Fehlen eines Texttreffers folgt nicht automatisch die Abwesenheit jeder denkbaren bildbasierten oder anders benannten Information.

### Q4. Fingerprints der besonders relevanten Originalanhänge

Diese SHA-256-Werte beziehen sich auf öffentliches Standardmaterial, nicht auf PIN-Telegramme:

| Datei | SHA-256 |
|---|---|
| `en_30039202v030801p.pdf` | `3f07b1e4ad73fabc16277a900006fc19844b3a882bbc2bc1d74d78f6156daf28` |
| `en_30039209v010701p.pdf` | `cad45938bcdf2fa4e8063d4aa9e7d7c06c45f729d3c55dc033e00c27af9f2e06` |
| `en_30039205v020701p.pdf` | `10aaf78988ae4080c3dfe90165ee7f4f3fe8eda402e09fc1e6fa0a0ad890758d` |
| `ETSI.pdf` | `9434dad1e7bc80ca39b0edadd8e3b9995fda5ae5f5c3dd565af05708d5059e38` |

### Q5. Repository- und Archivbezüge

- [Historischer Codebezug](https://github.com/JanHG98/netcore-tetra/tree/41d925457bed3cc0f1207d509642f3e003303010).
- [Zusätzlich geprüfter Archiving-Snapshot](https://github.com/JanHG98/netcore-tetra/tree/27996d2f494add5ae16837acaaba4f6f22f9f49d).
- [Archivindex](README.md).
- [Vorhandene zentrale IAM-Roadmap](../CENTRAL_IDENTITY_RBAC_ROADMAP.md), als angrenzende Planung und nicht als fertige RUA-Lösung.
- [Angrenzendes SDS-/Gateway-Archiv](2026-10-03_flowstation-sds-services-gateways-und-bot-architektur.md), zur späteren Abstimmung der inzwischen zentralisierten SDS-Verarbeitung; seine Inhalte werden hier nicht als neue RUA-Abnahme übernommen.
