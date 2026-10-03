# Abschlussdokumentation: Support-Mailadressen L0–L4 und Abgrenzung zu SLA-Klassen

## 1. Metadaten und Geltungsbereich

| Feld | Wert |
|---|---|
| Projekt / Repository | NetCore-Tetra / `JanHG98/netcore-tetra` |
| Thema | Wiederfinden und Absichern der festgelegten Support-Mailadressen je fachlichem Dokumentations-/Support-Level |
| Ursprünglicher Titel dieses Chats | Im zugänglichen Verlauf nicht übermittelt; der Dokumenttitel oben ist ein Archiv-Arbeitstitel. |
| Ursprünglicher Chatlink | Nicht verfügbar; kein Link rekonstruiert oder erfunden. |
| Identifizierende Ausgangsfrage | „wir hatten mal in einem Chat, ich weiß nicht mehr welcher, festgelegt welche Mail Adresse welcher Support Level bekommt. Glaube bei dem Service Levels. Welche waren das?“ |
| Erstellungsdatum | **2026-10-04**, Bezugszeitzone `Europe/Berlin` |
| Zeitpunkt der ersten Branchprüfung | Archivierungslauf am 2026-10-04; lokale Zeit beim Zeitabruf 01:23:36 CEST |
| Geprüfter Zielbranch | **`Archiving`**, Groß-/Kleinschreibung beibehalten |
| Geprüfter Ausgangscommit | **`03f6e65a19a9a642fdd5f896de563707dff2d015`** |
| Root-Tree des Ausgangscommits | `734b45f479168b7e355100370807eaa81575d702` |
| Schreibbereich dieses Auftrags | Ausschließlich `Docs/archive/` |
| Archivdatei | `Docs/archive/2026-10-04_support-mailadressen-level-l0-l4-und-sla-abgrenzung.md` |
| Zugehöriger Index | `Docs/archive/README.md` |
| Historisches Ergebnis | Namensschema ausdrücklich beschlossen; Postfächer, Mailrouting und produktiver Betrieb dadurch nicht nachgewiesen. |

Der Ausgangscommit wurde vor dem Schreiben erneut als Branchspitze bestätigt. Seine Commit-Zeit lautet `2026-10-03T23:16:08Z`; das ist kein Widerspruch zum lokalen Erstellungsdatum 2026-10-04. Der Archivcommit selbst ist über die Git-Historie dieser Datei und die Abschlussmeldung identifizierbar; der oben genannte Commit ist ausdrücklich der **Prüfstand vor der Archivänderung**, nicht eine vorab erfundene Speicher-Commitnummer. [R1]

Diese Dokumentation behandelt den vorliegenden kurzen Wiederauffindungs-Chat einschließlich seiner erhaltenen Quellenbelege. Sie ist keine Abschlussdokumentation aller NetCore-Tetra-Chats und kein Gesamtbetriebsnachweis.

## 2. Quellenlage, Auswertungsumfang und Grenzen

Ausgewertet wurden die sichtbare Ausgangsfrage, die damalige Antwort, der aktuelle Archivierungsauftrag sowie die im Verlauf erhaltenen Suchauszüge aus **`Chat BackUp Nr. 2.docx`**. Besonders wichtig ist der Auszug mit Jans ausdrücklicher Wahl **„wir nehmen variante D“** und der anschließend wiederholten L0–L4-Zuordnung. Diese Entscheidung ist damit nicht nur aus der unmittelbar vorangegangenen Assistentenantwort abgeleitet. [H1–H3]

Die alte DOCX erscheint in den erhaltenen Ergebnissen unter zwei Datei-IDs: `file-3XkD3ZRCEZBGLB5PPYhnPr` und `file-U4xdR52iMnuhXuC7bq4b9e`. Die entsprechenden Ausschnitte sind inhaltlich doppelt vorhanden; eine vollständige Byte-Identität der beiden Originaldateien wurde nicht geprüft.

**Auswertungslücken:** Ein erneuter Zugriff auf das vollständige alte Chat-Backup gelang in diesem Lauf nicht. Die Files-Suche fand keine passende aktuelle Datei; der direkte Zugriff meldete, dass die alte Referenz derzeit nicht sichtbar sei. Eine ergänzende Suche nach Chatmetadaten lieferte einen Suchfehler. Deshalb bleiben Originaltitel, Original-Chatlink, ursprüngliches Entscheidungsdatum und möglicherweise außerhalb der Ausschnitte vorhandene spätere Korrekturen offen. Die erhaltenen Ausschnitte wurden ausgewertet, nicht ein angeblich vollständig erneut geöffnetes DOCX.

Die 25 aktuell bereitgestellten ETSI-PDFs wurden als Anlagen inventarisiert und auf ihre Relevanz für dieses Thema eingeordnet. Ihre vollständigen fachlichen Inhalte und Abbildungen wurden **nicht** durchgearbeitet: Sie dokumentieren TETRA-Standards, nicht Jans Support-Mailadressen. Aus ihnen werden hier weder Mailkonfigurationen noch Implementierungs- oder Konformitätsnachweise abgeleitet. Inventar und Bildstatus stehen in Abschnitt 13.

## 3. Ziel, Ausgangslage und behandelte Themen

Jan wollte eine bereits früher getroffene Adressentscheidung wiederfinden, ohne den damaligen Chat noch benennen zu können. Die Vermutung lautete, dass sie im Zusammenhang mit „Service Levels“ getroffen worden sei.

Das unmittelbare Ziel war daher **Rekonstruktion einer bestehenden Festlegung**, nicht die Auswahl eines neuen Mailservers, die Einführung eines Ticketsystems oder das Erteilen neuer Supportrechte. Behandelt wurden:

- die fünf Support-Adressen und ihre fachlichen Zuständigkeiten;
- die Entscheidung für ein erkennbares Support-Namensschema ohne sichtbare Levelnummern;
- die Verwendung im Kontaktbereich der levelbezogenen Booklets;
- als mögliche Weiterentwicklung: Aliase, Ticket-Routing, SLA-Zuordnung und interne/öffentliche Darstellung.

In diesem Chat wurden weder Postfächer eingerichtet noch DNS-Einträge geändert, Konfigurationen ausgerollt oder Testmails versendet.

## 4. Endgültige Anforderungen und historische Entscheidung

### 4.1 Verbindlich rekonstruierte Adresstabelle: Variante D

| Fachliches Level | Bezeichnung | Historisch festgelegte Adresse | Fachlicher Zweck |
|---|---|---|---|
| **L0** | Quickstart / Einstieg | **`support@netcore-tetra.local`** | Allgemeine Hilfe und Einsteigerfragen |
| **L1** | Benutzer / User | **`support.user@netcore-tetra.local`** | Bedienung, Geräteeinstellungen und Benutzerfragen |
| **L2** | Operator | **`support.ops@netcore-tetra.local`** | Technischer Betrieb, Netzverhalten, Logs und Gruppenmanagement |
| **L3** | Admin | **`support.admin@netcore-tetra.local`** | Administration, API, Updates und Systemfragen |
| **L4** | Core / Deep System | **`support.core@netcore-tetra.local`** | Quellcode, SDR, Build-System und Architektur |

Die Domäne wurde in den historischen Tabellen als `netcore-tetra.local` verwendet. Abgekürzte Folgezellen mit `@...` sind oben zur eindeutigen Wiedergabe entsprechend ausgeschrieben. Das ist keine neue Domänenentscheidung. Eine abweichende produktive oder öffentliche Zieldomäne ist im zugänglichen Verlauf nicht beschlossen und ihre Zustellbarkeit wurde nicht geprüft. [H2–H3]

### 4.2 Anforderungen und Begründungen

**Support muss im Namen erkennbar sein.** Jan präzisierte den ersten Vorschlag mit „erstmal mails. aber es soll schon als support erkennbar sein“. Das gemeinsame Präfix `support` erfüllt diese sprachliche Anforderung. [H2]

**Keine unmittelbar sichtbare L0-/L1-/L2-Adressfolge.** Der Zwischenvorschlag `support.l0@…` bis `support.l4@…` wurde mit dem Einwand zurückgewiesen, dass das Schema schnell zu erraten sei; Jan fragte nach einer eleganteren Lösung. Die gewählte Variante verwendet stattdessen fachliche Bezeichnungen. Das dokumentiert die Namenspräferenz, aber keine Sicherheitskontrolle. [H2–H3]

**Explizite Auswahl von Variante D.** Nach mehreren Alternativen erfolgte die klare Nutzerentscheidung „wir nehmen variante D“. Andere Varianten bleiben verworfene Vorschläge. [H3]

**Geplante Booklet-Verwendung.** Als Standard war auf Seite 6 unter **„Kontakt & Support“** jeweils die zum Booklet-Level passende Adresse vorgesehen. Die damalige Assistentenaussage, dies künftig einzubauen, belegt die dokumentarische Absicht, nicht bereits geänderte Booklet-Dateien. Eine erfolgreiche Prüfung aller Booklets liegt nicht vor. [H3]

### 4.3 Was ausdrücklich nicht entschieden wurde

Nicht festgelegt sind Mailanbieter oder Mailserver, Anzahl realer Postfächer, Aliasziele, zuständige Personen, Vertretungen, Ticketsoftware, Kundenzuordnung, konkrete Eskalationsregeln, Zugangsschutz, Betriebszeiten für diese Adressen oder eine technische Bindung an SLA-Verträge. Ebenso fehlt eine Entscheidung für die spätere Alternative `support.level1@…` oder `l1-support@…`; diese ersetzt Variante D nicht.

## 5. Fachliches Support-Level ist nicht gleich SLA-Vertragsklasse

### 5.1 Einordnung der früheren Antwort

Die frühere Antwort in diesem Chat stellte die richtige Adresstabelle unter die Überschrift **„Support-Mailadressen pro Service Level“**. Das war begrifflich zu pauschal: Die belegte Tabelle ordnet Adressen den **Dokumentations-/Fachlevels L0–L4** zu. An anderer Stelle des erhaltenen Backups stehen zusätzlich **SLA-Klassen 0–4** für vertragliche Verfügbarkeit und Reaktion. Gleiche Ziffern belegen keine identische Bedeutung oder automatische Zuordnung. [H3–H4]

Diese Präzisierung ist eine quellenbasierte Einordnung bei der Archivierung, **keine nachträglich behauptete Nutzeränderung** der Adressen.

### 5.2 Separat erhaltenes historisches SLA-Konzept

Die folgenden Werte stammen aus einem zusätzlichen NTOC-Quellenausschnitt. Sie werden zur Abgrenzung bewahrt, nicht als in diesem Chat verbindlich abgeschlossene Verträge oder implementierte Garantien übernommen. [H4]

| Historische SLA-Klasse | Vorgeschlagene Verfügbarkeit | Vorgeschlagene Reaktion | Kontext im damaligen Konzept |
|---|---|---|---|
| SLA 0 – Community | Mo–Fr, 08–17 Uhr | Nach Verfügbarkeit | Keine Garantie, keine Hotline, E-Mail/SDS |
| SLA 1 – Standard | Mo–Fr, 08–17 Uhr | Unter 24 Stunden | Ticket-Routing, kein Bereitschaftsdienst |
| SLA 2 – Plus | Mo–So, 06–22 Uhr | Unter 4 Stunden | Telefonannahme und erreichbare Bereitschaft |
| SLA 3 – 24/7 | Rund um die Uhr | Unter 1 Stunde | Sofortige Eskalation und Direktverbindung zum Operator |
| SLA 4 – Kritisch/Behörde | 24/7 plus SLA-Kanal | Unter 15 Minuten | Sonderkanal und gegebenenfalls kundenspezifische Zusatzfunktionen |

Der erhaltene Nutzerhinweis lautet, dass dies an Service-Level und Kunden **je Vereinbarung** angepasst sein soll. Zeitzone, Messbeginn, Geschäftszeitberechnung, Abgrenzung von Reaktions- und Lösungszeit sowie tatsächlich gültige Kundenvereinbarungen sind damit nicht vollständig spezifiziert.

Die damalige Antwort dieses Chats bot außerdem an, die Adressen „1:1 mit SLA-Klassen“ zu verknüpfen, beispielhaft „SLA 3 darf nur `support.ops`+“. **Das war ein unbestätigter Vorschlag des Assistenten, keine beschlossene Zugangsregel.** Er darf nicht als bestehende Konfiguration übernommen werden.

## 6. Architektur, Schnittstellen und Abhängigkeiten

### 6.1 Historischer Konzeptstand

Das beschlossene Element ist ein **Adress-/Kontaktverzeichnis**, noch keine technische Mailarchitektur:

```text
Booklet L0–L4 / Kontakt & Support
    -> fachlich passende Support-Mailadresse
    -> reales Postfach oder Aliasziel: offen
    -> Bearbeitung / Ticket-Routing: offen
    -> Kundenzuordnung und SLA-Behandlung: separat festzulegen
```

Bereits genannt war die Möglichkeit, alle Adressen als Aliase auf ein gemeinsames Postfach zu leiten oder getrennte Ziele zu verwenden. Das war eine Option, keine Auswahl. Weitere optionale Ausgabestellen waren Dokus, GUIs, SDS-Nachrichten und Systemmeldungen. [H2]

### 6.2 Heute belegte mögliche Integrationspunkte – kein fertiges Mailrouting

Der am Prüfcommit gelesene Dienstkatalog beschreibt unter anderem **Application Gateway**, **Alarm Workflow** und **Task Workflow**. Diese Komponenten sind mögliche spätere Anknüpfungspunkte, aber nicht im historischen Chat als Träger des Mail-Supports ausgewählt worden. [R2]

Die README des Application Gateway dokumentiert Connector Registry, Regeln, Vorlagen, Event-/Delivery-Queues und HTTP-/Webhook-Schnittstellen. In der gelesenen Standard-Connectorliste steht kein ausdrücklich benannter Mail-/SMTP-/IMAP-Supporteingang. Das ist ein begrenzter Dokumentationsbefund, **kein beweiskräftiger Ausschluss jeder Mailintegration im gesamten Repository oder außerhalb davon**. [R3]

Im tatsächlich gelesenen Rust-Ausschnitt von `system-backend/application-gateway/src/http.rs` sind unter anderem Handler für `GET /api/v1/connectors`, `GET /api/v1/rules`, `GET /api/v1/events`, `POST /api/v1/events`, `POST /api/v1/dispatch` und `GET /api/v1/deliveries` vorhanden. Das belegt entsprechenden Quellcode. Es belegt weder die fünf Support-Adressen noch ein laufendes Ticketsystem oder eine SLA-Zuordnung. [R4]

Für eine spätere Umsetzung sind daher mindestens produktive Domäne, empfangender Maildienst, Alias-/Postfachmodell, verantwortliche Teams und ein separat definiertes Kunden-/SLA-Modell erforderlich. Die technische Umsetzung und ihre Freigabe sind Gegenstand eines Folgeauftrags, nicht dieser Archivierung.

## 7. Erreichter Entwicklungs- und Betriebsstand

| Gegenstand | Idee | Beschlossen/geplant | Implementiert | Getestet | Im Betrieb bestätigt |
|---|---|---|---|---|---|
| Variante-D-Adressen für L0–L4 | Über Vorschlagsphase hinaus | **Ja, ausdrückliche Auswahl** | Reale Mailanlage nicht nachgewiesen | Nur Quellenabgleich | Nein, kein Betriebsbeleg |
| Passende Adresse auf Booklet-Seite 6 | – | Historisch vorgesehen | Booklet-Änderungen nicht nachgewiesen | Keine Sichtprüfung fertiger Booklets | Nicht nachgewiesen |
| Gemeinsames Postfach versus getrennte Ziele | **Ja** | Keine Variante gewählt | Nicht nachgewiesen | Nicht getestet | Nicht nachgewiesen |
| Ticket-Routing / SLA-Automatik | **Ja** | Keine konkrete Regel beschlossen | Für diese Adressen nicht nachgewiesen | Nicht getestet | Nicht nachgewiesen |
| Öffentliche versus interne Adresssicht | **Ja** | Nicht entschieden | Nicht nachgewiesen | Nicht getestet | Nicht nachgewiesen |
| Support per SDS | **Ja** | Durch „erstmal mails“ zurückgestellt | Keine Supportnummern-Zuweisung nachgewiesen | Nicht getestet | Nicht nachgewiesen |
| Application-Gateway-HTTP-Handler | Nicht Ergebnis dieses Chats | Separater Repository-Kontext | Im gelesenen Quelltext vorhanden | In diesem Lauf nicht ausgeführt | In diesem Lauf nicht geprüft |

**Erreicht wurde eine belastbar rekonstruierte Namensentscheidung, nicht die Inbetriebnahme eines Supportsystems.** Auch eine erfolgreich gespeicherte Archivdatei ändert diese technischen Statuswerte nicht.

## 8. Relevante Parameter, Dateien und überprüfter Repository-Stand

### 8.1 Historische technische Parameter

| Parameter | Wert / Befund |
|---|---|
| Zahl fachlicher Kontaktstufen | 5: L0, L1, L2, L3, L4 |
| Gemeinsames Namenspräfix | `support` |
| Fachliche Suffixe | Kein Suffix bei L0; `.user`, `.ops`, `.admin`, `.core` bei L1–L4 |
| Historische Domäne | `netcore-tetra.local` |
| Booklet-Position | Seite 6, „Kontakt & Support“ |
| Mailserver / Host / Dienstname | Nicht festgelegt |
| Mail-Konfigurationspfad | Nicht festgelegt; kein erfundenes `aliases`-/TOML-/YAML-Ziel |
| SMTP-/IMAP-/Submission-Ports | Im Chat nicht festgelegt oder getestet |
| Mailprotokoll-/TLS-/Authentisierungsparameter | Nicht festgelegt |
| Aliasziele / Ticketqueues | Nicht festgelegt |
| DNS- und Zustellprüfung | Nicht durchgeführt |

### 8.2 Separater Repository-Abgleich am 2026-10-04

Die folgenden Datei-/Codebefunde beziehen sich auf `Archiving` bei `03f6e65a19a9a642fdd5f896de563707dff2d015`, nicht automatisch auf eine laufende Installation:

| Geprüfter Gegenstand | Ergebnis | Grenze |
|---|---|---|
| Branch und Root-Tree | Zielbranch erreichbar; Commit und Tree gelesen und später erneut bestätigt | Kein Deploymentbeleg |
| `Docs/archive/README.md` | Vorhandener Archivindex vollständig gelesen; kein Eintrag zu diesem Support-Adress-Chat | Index allein ersetzt keine Quellcodesuche |
| Geplanter Archivdateipfad | Abruf vor Anlage ergab HTTP 404 | Kein fremdes Dokument an diesem Pfad überschreiben |
| Verzeichnisbäume `Docs/` und `wiki/` | Abgerufen; in der großen `Docs/`-Antwort über Ressourcensuche kein Pfadtreffer für `support` oder `sla` | Namenssuche, keine Volltextprüfung aller Dateien; Anzeige großer Antworten gekürzt |
| `wiki/Dienstkatalog.md` | Dienstbeschreibungen und mögliche Integrationspunkte vorhanden | Dienstkatalog behauptet keine Einrichtung dieser fünf Postfächer |
| `system-backend/application-gateway/README.md` | Standard-Managementport `8220`, dokumentierte Connector-/Routing-/Webhook-Funktionen | Beispielport/README, nicht tatsächlich gemessener Listener |
| `system-backend/application-gateway/src/http.rs`, Zeilen 1–180 | Genannte HTTP-Handler im Code vorhanden | Nur dieser Ausschnitt geprüft; kein Build oder Ende-zu-Ende-Test |

Der Dienstkatalog nennt ferner `8270` für Alarm Workflow und `8280` für Task Workflow als TCP-Beispielports. **Diese Werte sind keine Mailports und keine historische Supportkonfiguration.** [R2]

Ergänzende GitHub-Suchen nach `support`, `"support.user"` und `SLA` wurden zur Orientierung verwendet. Der Suchindex arbeitet auf dem Default-Branch; er wurde nicht als beweiskräftige Volltextprüfung von `Archiving` behandelt. Die exakte `support.user`-Suche lieferte keinen Treffer; die `SLA`-Suche lieferte unter anderem irrelevante Teilworttreffer. Daraus wird kein pauschales „nirgends implementiert“ abgeleitet. Sämtliche oben als aktuelle Datei-/Codebefunde verwendeten Inhalte wurden zusätzlich mit dem angegebenen Prüfcommit gelesen.

Ein lokaler Download des vollständigen Repository-Snapshots scheiterte an DNS-Auflösung für `codeload.github.com`. Deshalb liegt keine lokale Vollrepository-Suche vor. Der GitHub-Connector blieb für die gezielten Lese- und Archivschreiboperationen verfügbar.

## 9. Befehle, Abläufe, Fehler und Lösungen

### 9.1 Historischer Chat

Für den Mail-Support wurden **keine Shellbefehle, Installationsschritte, Deploymentaktionen oder Reparaturen ausgeführt**. Es existieren hier keine erfolgreichen SMTP-Tests, Postfachanlagen oder Logausgaben. Die frühere Aussage „ist gespeichert“ im Quellenausschnitt war ohne technische Repository-/Mailserver-Nachweise und wird nicht als Umsetzung verbucht.

Das historische Problem war das Wiederfinden der Entscheidung. Die funktionierende Lösung war der erhaltene Quellenausschnitt mit der ausdrücklichen Variante-D-Auswahl, ergänzt um die Adresstabelle.

### 9.2 Tatsächlich ausgeführte Arbeiten des Archivierungslaufs

- Branch-/Ref- und Tree-Abfragen sowie gezielte Dateiabrufe über den GitHub-Connector, einschließlich erneuter Head-Prüfung vor dem Schreiben.
- Files-Suche und Wiederzugriffsversuch auf das alte Backup; kein erneuter vollständiger Zugriff möglich.
- Prüfung auf eigenständige Bilder über Files und im Container; Ergebnis jeweils keine verfügbaren eigenständigen Bilddateien.
- Öffnen der 25 bereitgestellten PDFs zur Bestands-/Seitenzahlprüfung; keine umfassende Normenauswertung.
- Versuch eines lokalen Snapshot-Downloads; **fehlgeschlagen** mit `Temporary failure in name resolution` / `NameResolutionError` für `codeload.github.com`.

Die DNS-Störung ist eine Grenze dieser Arbeitsumgebung, kein diagnostizierter Fehler der NetCore-Tetra-Basisstation, des Repositories oder eines Mailservers. Für die Archivierung wird stattdessen die verfügbare GitHub-Git-Daten-API verwendet. Ein fehlendes Chat-Backup wird nicht durch fachfremde ETSI-Unterlagen oder Vermutungen ersetzt.

### 9.3 Nur vorgeschlagene spätere Nachprüfung

Die folgenden Befehle wurden **nicht** als Git-CLI-Ablauf dieses Chats ausgeführt. Sie zeigen eine lesende Prüfung in einem vorhandenen lokalen Clone:

```bash
# Repository-Verzeichnis vorausgesetzt; keine Mailkonfiguration ändern.
git fetch origin Archiving
git rev-parse origin/Archiving
git show origin/Archiving:Docs/archive/2026-10-04_support-mailadressen-level-l0-l4-und-sla-abgrenzung.md
git show origin/Archiving:Docs/archive/README.md

# Ein späterer Negativbefund wäre nur für die durchsuchten getrackten Textdateien gültig.
# Exitcode 1 von git grep bedeutet normalerweise: kein Treffer.
git grep -n -I -F 'support.user@netcore-tetra.local' origin/Archiving -- . ':!Docs/archive'
```

Es wird absichtlich keine hypothetische Mailserver-Installationsanleitung beigefügt: Produkt, Zielsystem und Betriebsmodell wurden nicht ausgewählt.

## 10. Tests, Prüfergebnisse und verbleibende Probleme

| Prüfung | Ergebnis | Aussagegrenze |
|---|---|---|
| Adresstabelle gegen erhaltene Originalausschnitte | Variante D und L0–L4-Zuordnung konsistent | Nicht das gesamte alte DOCX erneut geprüft |
| Unterscheidung Support-Level / SLA | Zwei unterschiedliche Tabellen im Quellenmaterial erkennbar | Keine abgeschlossenen Kundenverträge geprüft |
| Zielbranch / Archivindex / Dateikollision | Branch und Index gelesen, Zielpfad vor Neuanlage nicht vorhanden | Abschluss der Speicherung ist separat am Remote zu kontrollieren |
| Sichtprüfung eigenständiger Bildverfügbarkeit | Kein eigenständiges Chatbild verfügbar | PDF-Vorschaubilder sind keine separat hochgeladenen Chatbilder |
| PDF-Bestand | 25 Dateien technisch geöffnet und Seitenzahlen erfasst | Keine umfassende fachliche oder visuelle Prüfung aller Seiten |
| Application-Gateway-Quellcode | Benannte Handler im gelesenen Ausschnitt vorhanden | Nicht gestartet, nicht gebaut und nicht auf Support-E-Mails getestet |
| SMTP-/IMAP-, Alias-, Ticket- und SLA-Tests | Nicht durchgeführt | Keine Aussage zur Erreichbarkeit oder Funktion |
| Hardware-/Funk-/On-Air-Tests | Nicht durchgeführt; für diese Adressrekonstruktion nicht relevant | Kein TETRA-Betriebs- oder Konformitätsnachweis |

Offen bleiben vor allem produktive Zieladressen, tatsächliche Mailanlage, Zuständigkeiten, verbindliche SLA-Logik und die konsistente Übernahme in Booklets. Es gibt in diesem Chat keinen dokumentierten Mail-Ausfall, dessen Ursache oder Behebung behauptet werden könnte.

## 11. Verworfene Ansätze und bewahrte Nebenideen

### 11.1 Durch Variante D ersetzte Adressvorschläge

Alle hier abgekürzten Namen beziehen sich auf die historische Domäne; sie sind **keine zusätzlichen aktiven Adressen**. [H2–H3]

| Vorschlag | Namen für L0 → L4 | Einordnung / Grund |
|---|---|---|
| Erster Vorschlag | `support`, `hilfe`, `ops`, `admin`, `core` | Durch Forderung nach durchgängig erkennbarem Support-Bezug ersetzt |
| Explizite Levelnummern | `support.l0` bis `support.l4` | Nutzer wollte ein eleganteres, nicht unmittelbar nummeriertes Schema |
| Variante A | `hello`, `user.support`, `ops.support`, `admin.desk`, `core.lab` | Nicht gewählt; ausdrückliche Entscheidung für D |
| Variante B | `start`, `field`, `ops`, `sys`, `kernel` oder `deep` | Nicht gewählt |
| Variante C | `welcome`, `talk`, `flow`, `access`, `stack` | Nicht gewählt |
| Weitere Formulierungsoptionen | `help.level0`, `techsupport.l3`, `support.level1`, `l1-support` | Nur angebotene Beispiele, nicht angenommen |

Die historische Überschrift „Variante D – mit Support-Endung“ war sprachlich unpräzise: Die gewählten Adressen haben `support` als **Präfix**. Maßgeblich ist die Adresstabelle, nicht diese Überschrift.

### 11.2 Noch relevante Ideen ohne Umsetzungsauftrag

**Aliase und Skalierung:** gemeinsames Postfach oder getrennte Instanzen; spätere Delegation an Teams; Ticketzuordnung und Priorisierung. Keine Option wurde abschließend ausgewählt.

**Interne/öffentliche Darstellung:** Nach außen einheitliche Kontakte und intern differenziertes Routing waren als möglicher nächster Schritt angeboten. Es gab keine darauf folgende Entscheidung.

**Weitere Kontaktoberflächen:** Dokus, GUIs, SDS-Nachrichten und Systemmeldungen könnten die passende Kontaktadresse ausgeben. Das ist eine Integrationsidee, kein belegter Einbau.

**SDS-Kontakte:** Der erste Vorschlag enthielt L0 → `1000001`, L1 → `1000002`, L2 → `1000003`, L3 → `1000004`, L4 → `1000005`. Jan begrenzte den Gegenstand anschließend auf **„erstmal mails“**. Diese Werte werden daher ausschließlich als zurückgestellte Idee bewahrt, nicht als freigegebener aktueller ISSI-/SDS-Nummernplan. Eine spätere Umsetzung müsste Kollisionen mit dem dann gültigen Plan prüfen. [H2]

**Farbsystem der Booklets:** Im angrenzenden Quellenausschnitt folgt die Bitte nach Farben. Sichtbar sind Vorschläge für L0/Quickstart `#7ED957`, L1/Benutzer `#4DA6FF` und L2/Operator `#FFA94D`. Der Ausschnitt endet danach; L3/L4 und eine abschließende Freigabe sind daraus nicht vollständig rekonstruierbar. Diese Nebenspur ist kein neues Farbschema dieses Chats. [H3]

**NTOC-/SLA-Routing:** Im erhaltenen Konzept sollte die Anfrage um Kunde, zugehörige Nodes und SLA-Klasse ergänzt und an Operator/Dispatcher, Bereitschaft oder Eskalationsteam geleitet werden. Priorisierte Kennzeichnung und Benachrichtigung über weitere Kanäle waren beschrieben. Dies bleibt separater Konzeptkontext; daraus folgt keine automatisch umgesetzte Mailadresse-zu-Vertrag-Zuordnung. [H4]

## 12. Roadmap-Kandidaten und konkrete Fortsetzung

Die folgende Reihenfolge ist ein **bei der Archivierung abgeleiteter Arbeitsvorschlag**, keine nachträglich erfundene historische Prioritätenvereinbarung. Für diesen Chat sind keine Termine, Aufwandsschätzungen oder Implementierungsprioritäten beschlossen.

| ID | Nächster Schritt / Kandidat | Voraussetzung | Abschlusskriterium / Status |
|---|---|---|---|
| SUP-01 | Variante-D-Tabelle als einheitliche Kontaktreferenz übernehmen; fachliche Level und SLA sauber benennen | Prüfung dieses Archivs | Beschlossene Namen bleiben konsistent; kanonischer Ablageort in separatem Auftrag bestimmen |
| SUP-02 | Produktive Domäne und intern/öffentlich sichtbare Adressen festlegen | Domänen-/Betriebsentscheidung | Freigegebene vollständige Zieladressliste; historische `.local`-Werte nicht stillschweigend austauschen |
| SUP-03 | Maildienst, Alias-/Postfachmodell, Teams und Vertretungen definieren und einrichten | SUP-02 | Konfigurationsnachweis ohne Geheimnisse; Zuständigkeiten dokumentiert |
| SUP-04 | Optionales Ticket-/Gateway-Routing und separates Kunden-/SLA-Modell entwerfen | SUP-03 und bestätigte Kundenvereinbarungen | Keine Privilegien oder SLA-Garantien allein aus der angeschriebenen Aliasadresse ableiten; explizite Regeln freigeben |
| SUP-05 | Booklets, Kontaktseiten und gegebenenfalls GUI-/Systemmeldungen angleichen | Freigegebene Kontakte | Tatsächliche Dateien geprüft; je Level korrekte Adresse, insbesondere historischer Kontaktbereich auf Seite 6 |
| SUP-06 | Ende-zu-Ende-Abnahme durchführen | Eingerichtete Test-/Produktivumgebung | Für jede Adresse Eingang, richtige Zuordnung, Rückantwort und Fehlerfall protokollieren; SLA-Eskalationen separat testen |
| SUP-07 | Fehlende Originalquelle/Metadaten nachtragen; SDS-/Farbideen nur bei Wiederaufnahme vervollständigen | Wieder zugänglicher Originalchat / Backup | Quellenlücken geschlossen; keine erfundenen Links oder Farbcodes; SDS-Plan geprüft |

Alle Kandidaten verbleiben **innerhalb dieser Archivdatei**. Produktive Konfigurationen, zentrale Roadmap-Dateien, Wiki-Seiten außerhalb des Archivs, Dienste und Tickets werden durch diesen Auftrag nicht verändert.

## 13. Anhänge und Bilder

### 13.1 Für die Entscheidung relevantes altes Backup

`Chat BackUp Nr. 2.docx` ist über die im aktuellen Verlauf erhaltenen Textauszüge relevant. Die Originaldatei konnte nicht erneut vollständig bezogen werden und wird daher nicht als vollständig mitarchivierter Anhang ausgegeben. Die entscheidenden Aussagen und Adressen sind in dieser Markdown-Datei selbst enthalten.

### 13.2 Inventar der derzeit bereitgestellten PDFs

Die Seitenzahlen wurden an den tatsächlich bereitgestellten Dateien ermittelt. Die Titel-/Themenzuordnung stammt aus ihren sichtbaren Dokumentköpfen. Diese Liste ist **Anlageninventar**, keine behauptete vollständige Normenauswertung.

| Datei | Seiten | Einordnung |
|---|---:|---|
| `ETSI.pdf` | 4100 | Umfangreiche ETSI-Sammeldatei; beginnt mit EN 300 812; nicht vollständig ausgewertet |
| `en_30039201v010601p.pdf` | 182 | Allgemeiner TETRA-Netzentwurf |
| `en_30039202v030801p.pdf` | 1445 | TETRA Air Interface |
| `en_3003920303v010301p.pdf` | 251 | ISI Group Call |
| `en_3003920304v010301p.pdf` | 28 | ISI Short Data Service |
| `en_3003920308v010401p.pdf` | 22 | Generic Speech Format Implementation |
| `en_3003920313v010201p.pdf` | 191 | Transportunabhängiger ISI Group Call |
| `en_3003920315v010500a.pdf` | 380 | ISI Mobility Management; bereitgestellte Ausgabe als Draft gekennzeichnet |
| `en_30039205v020701p.pdf` | 320 | Peripheral Equipment Interface |
| `en_30039207v030501p.pdf` | 216 | TETRA Security |
| `en_30039209v010701p.pdf` | 46 | Allgemeine Anforderungen an Supplementary Services |
| `en_3003921006v010401p.pdf` | 20 | Call Authorized by Dispatcher, Stage 1 |
| `en_3003921018v010301p.pdf` | 17 | Barring of Outgoing Calls, Stage 1 |
| `en_3003921101v010201p.pdf` | 44 | Call Identification, Stage 2 |
| `en_3003921114v010101p.pdf` | 23 | Late Entry, Stage 2 |
| `en_3003921117v010102p.pdf` | 18 | Include Call, Stage 2 |
| `en_3003921201v010202p.pdf` | 56 | Call Identification, Stage 3 |
| `en_3003921216v010400a.pdf` | 67 | Pre-emptive Priority Call, Stage 3; bereitgestellte Ausgabe als Draft gekennzeichnet |
| `en_30039401v030301p.pdf` | 169 | Radio Conformance Testing |
| `en_30039502v010303p.pdf` | 94 | TETRA Speech Codec |
| `en_300812v020101p.pdf` | 156 | SIM-ME-Schnittstelle |
| `es_20081201v020205p.pdf` | 8 | UICC, physische und logische Eigenschaften |
| `es_20081202v020401m.pdf` | 139 | TSIM-Anwendung; bereitgestellte Ausgabe als Final draft gekennzeichnet |
| `ets_30039214e01v.pdf` | 61 | PICS-Proforma; bereitgestellte Ausgabe als Final draft gekennzeichnet |
| `ts_10081201v020205p.pdf` | 8 | UICC, physische und logische Eigenschaften |

Es wird keine Aussage zum heutigen Gültigkeits- oder Ablösestatus dieser Normenausgaben getroffen. Für die Support-Adressentscheidung wurde keine aktuelle Normenrecherche benötigt. Die PDFs werden im Rahmen dieses thematisch begrenzten Archivs nicht erneut als Binärdateien ins Repository kopiert.

### 13.3 Bildarchivierung

Im verfügbaren Verlauf dieses Chats sind **keine eigenständigen Nutzerbilder, Screenshots oder generierten Chatbilder** vorhanden. Die Files-Bildabfrage ergab null Ergebnisse; im bereitgestellten Containerbestand lagen ebenfalls keine eigenständigen Bilddateien vor.

Die angezeigten ETSI-Titelseiten und Dokumentabbildungen sind PDF-interne Inhalte beziehungsweise automatische Vorschauen. Sie sind nicht mit gesonderten Bildern dieses Support-Chats zu verwechseln und wurden wegen fehlender thematischer Relevanz nicht extrahiert oder als vermeintliche Chatbilder hochgeladen. Deshalb enthält dieser Archivauftrag keine Bilddatei. Es wurde kein fehlendes Bild erfunden oder neu erzeugt.

## 14. Quellenverzeichnis und Nachvollziehbarkeit

### Historische Quellen im zugänglichen Verlauf

- **H1:** Ausgangsfrage und unmittelbar folgende Assistentenantwort dieses Chats. Die Antwort liefert die Variante-D-Tabelle, enthält aber auch unbestätigte Folgeangebote und die zu pauschale Bezeichnung „Service Level“.
- **H2:** Erhaltene Suchauszüge aus `Chat BackUp Nr. 2.docx`: ursprüngliche Rollen-/Adresszuordnung, SDS-Vorschläge, „erstmal mails. aber es soll schon als support erkennbar sein“, danach `support.l0` bis `support.l4`. Fundstellen im bereitgestellten Verlauf: `turn1file1`, Zeilen 13–154, sowie `turn1file0`, Zeilen 56–137. Diese Bezeichner sind Quellenanker der erhaltenen Chatdarstellung, keine Repository-Dateipfade.
- **H3:** Dasselbe Backup, Alternativen und ausdrückliche Auswahl „wir nehmen variante D“; Zuordnung für Booklet-Kontaktseite 6. Fundstelle `turn1file7`, Zeilen 77–131; identischer sichtbarer Ausschnitt unter `turn1file6`. Varianten A–C in `turn1file4`, Zeilen 43–131; angeschnittene Farbfolge in `turn1file7`, Zeilen 139–177.
- **H4:** Zusätzliche erhaltene NTOC-/SLA-Auszüge desselben Backups: `turn1file8`, Zeilen 55–115, und `turn1file12`, Zeilen 1–17. Separater Konzeptkontext, keine in diesem Wiederauffindungs-Chat bestätigte Umsetzung.

### Aktuell gelesene Repository-Quellen

- **R1:** [Geprüfter Ausgangscommit](https://github.com/JanHG98/netcore-tetra/commit/03f6e65a19a9a642fdd5f896de563707dff2d015), Branch `Archiving`; GitHub-Branch-/Ref-Abfragen und Root-Tree-Prüfung im Archivierungslauf.
- **R2:** [`wiki/Dienstkatalog.md` am Prüfcommit](https://github.com/JanHG98/netcore-tetra/blob/03f6e65a19a9a642fdd5f896de563707dff2d015/wiki/Dienstkatalog.md), vollständig gelesen; Blob `60ac2752598dd780cb55152908d5439ed865d6af`.
- **R3:** [`system-backend/application-gateway/README.md` am Prüfcommit](https://github.com/JanHG98/netcore-tetra/blob/03f6e65a19a9a642fdd5f896de563707dff2d015/system-backend/application-gateway/README.md), vollständig gelesen; Blob `eaad92c695e2367868d22b42e7118e05511e99c9`.
- **R4:** [`system-backend/application-gateway/src/http.rs`, Zeilen 1–180](https://github.com/JanHG98/netcore-tetra/blob/03f6e65a19a9a642fdd5f896de563707dff2d015/system-backend/application-gateway/src/http.rs#L1-L180); Blob der Datei `04ce8b22d1b76c333551c1e8df36b3552e07f084`.
- **R5:** [`Docs/archive/README.md` vor dieser Änderung](https://github.com/JanHG98/netcore-tetra/blob/03f6e65a19a9a642fdd5f896de563707dff2d015/Docs/archive/README.md), vollständig gelesen; Ausgangsblob `44e332d3a786e2a2531275c13f0c1e6ce18d6a3b`.

Für die historische Mailentscheidung sind keine früheren Implementierungscommits, PRs oder erfolgreichen CI-/Betriebstests belegt. Deshalb werden dafür keine Nummern ergänzt.

## 15. Archivierungsgrenze und Übergabe

Dieser Auftrag ergänzt die vorliegende Abschlussdokumentation und ihren relativen Link im vorhandenen Archivindex. Bestehende Archivtexte und Indexeinträge bleiben erhalten. Alle Schreibpfade liegen unter `Docs/archive/`; es gibt keinen Merge und keinen Force-Push. Zugangsdaten werden nicht übernommen.

Vor einer späteren technischen Fortsetzung ist zuerst zwischen **beschlossener Adresse**, **tatsächlich eingerichtetem Empfänger**, **getestetem Zustellpfad** und **vertraglich bestätigter SLA-Behandlung** zu unterscheiden. Die nächste Bearbeitung soll von dieser Variante-D-Tabelle ausgehen, nicht erneut aus den verworfenen Namensvorschlägen wählen.

**Übergabeergebnis:** Die fünf fachlichen Support-Adressen sind rekonstruiert. Ihre reale Einrichtung und SLA-Anbindung bleiben offen. Die Archivdatei ist eine Entscheidungs- und Fortsetzungsgrundlage, keine Bestätigung eines bereits laufenden Mail-Supports.
