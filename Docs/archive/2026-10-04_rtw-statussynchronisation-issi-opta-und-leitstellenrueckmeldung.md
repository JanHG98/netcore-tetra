# TETRA-Statussynchronisation im Fahrzeug: ISSI, OPTA und Leitstellenrückmeldung

> Technische Abschlussdokumentation für NetCore-Tetra. Historischer Gesprächsstand und nachträglicher Quellen-/Repository-Abgleich sind getrennt. Die früheren Aussagen über eine gemeinsame Fahrzeug-ISSI und automatisches Routing „an die OPTA“ sind **keine belastbare technische Grundlage**. Im geprüften Repository existiert inzwischen ein konkreter Status-Sync-Pfad über Directory-Gerätegruppen und einzeln adressierte Display-SDS; eine erfolgreiche Abnahme auf den im Gespräch gemeinten RTW-Geräten ist nicht belegt.

## 1. Metadaten und Prüfgrenzen

| Merkmal | Feststellung |
|---|---|
| Projekt / Repository | `NetCore-Tetra` / `JanHG98/netcore-tetra` |
| Thema | Gemeinsame Statusanzeige mehrerer Funkgeräte eines Einsatzmittels; individuelle Teilnehmeridentitäten; Bedeutung von OPTA und Rückmeldungen |
| Ursprünglicher Chattitel | Nicht verlässlich verfügbar; die Überschrift dieses Dokuments ist ein nachträglicher Archivtitel. |
| Ursprünglicher Chatlink | Nicht verfügbar; kein Link rekonstruiert oder erfunden. |
| Historisches Gesprächsdatum | Aus dem unmittelbar sichtbaren Verlauf nicht sicher feststellbar; nicht mit dem Archivdatum gleichsetzen. |
| Erstellungsdatum dieser Dokumentation | **2026-10-04** |
| Geprüfter und ausschließlich für Archivänderungen verwendeter Branch | **`Archiving`** |
| Festgehaltener Quellcode-Prüfstand | **`2d4fbd627a1ecc9508f8bf52783bbc7fffb414c4`** |
| Root-Tree dieses Prüfstands | `b76a417cbdb490d09734c6923979e28073152bdb` |
| Prüfstand-Commitnachricht | `docs(archive): document radio inventory fields and asset-management gaps` |
| Ablage | `Docs/archive/2026-10-04_rtw-statussynchronisation-issi-opta-und-leitstellenrueckmeldung.md` |
| Archivindex | `Docs/archive/README.md` |
| Umfang des Schreibauftrags | Dieses Archivdokument und der erhalten/ergänzt geführte Archivindex; keine Quellcode-, Konfigurations-, Wiki- oder Roadmapänderungen außerhalb von `Docs/archive/`. |
| Archiv-Commit | Über die Git-Historie dieses Dokuments und die Abschlussmeldung festzustellen. Der oben genannte Commit ist der **geprüfte Ausgangsstand**, nicht der nachträglich erzeugte Archiv-Commit. |

### 1.1 Tatsächlich ausgewertete Grundlagen

Ausgewertet wurden die drei zugänglichen Frage-/Antwortpaare dieses Chats, der abschließende Archivauftrag, die angehängten ETSI-Dokumente in den thematisch relevanten Abschnitten sowie gezielt gelesene Repository-Dateien am festgehaltenen Commit. Der Repository-Abgleich ist eine **statische Quelltextprüfung**, keine Ausführung der Basisstation.

Alle 25 bereitgestellten PDF-Dateien waren als Dateien zugänglich. Dateinamen, Seitenzahlen, Titelseiten und SHA-256-Prüfsummen wurden inventarisiert. Zusammen ergeben sie 8.061 Dateiseiten, wobei `ETSI.pdf` allein 4.100 Seiten umfasst und Überschneidungen mit Einzeldokumenten enthält. Das ist **keine Aussage über 8.061 unterschiedliche Normseiten**. Die Sammlung wurde nicht vollständig Satz für Satz geprüft. Die vertieft ausgewerteten Stellen sind in Abschnitt 14 angegeben; die übrigen Dokumente wurden hinsichtlich Thema und Relevanz eingeordnet.

Weitere Projektchats sind kein Ersatz für den Originalverlauf. Bereits vorhandene Archive dienen nur als ausdrücklich benannte Querverweise. Eine zusätzliche Kontextsuche ergab keinen verlässlich verwendbaren Originaltitel oder Chatlink und keinen belegten späteren Implementierungsbeschluss innerhalb dieses konkreten Gesprächs.

### 1.2 Nicht verfügbare Nachweise

Es fehlen insbesondere Gerätehersteller/-modelle, Firmwarestände, Codeplugs, reale ISSI-/ITSI-Listen, zuständige Leitstelle, eingesetztes Einsatzleitsystem, regionale Betriebsregeln, Funk-/SDS-Mitschnitte und eine Aufzeichnung der tatsächlich umspringenden Anzeige. Auch ist nicht geklärt, ob wirklich mehrere unabhängige Funkgeräte oder teilweise mehrere Bedienteile eines einzelnen MRT gemeint waren. Letzteres wäre eine andere Architektur; es wird hier nicht als Erklärung angenommen.

Es gab keinen Zugriff auf produktive RTW-Geräte, laufende NetCore-Dienste oder deren Datenbanken. Aussagen über installierte Binaries, aktive Konfigurationen, tatsächliche Teilnehmerregistrierungen und Ende-zu-Ende-Zustellung bleiben deshalb offen.

## 2. Ziel, Ausgangslage und historischer Verlauf

### 2.1 Ausgangsbeobachtung

Der Nutzer beschrieb, dass mehrere Funkgeräte auf einem RTW denselben Status anzeigen: Wird an irgendeinem Gerät eine Statustaste gedrückt, springt die Anzeige auf den anderen Geräten ebenfalls um. Er wollte den technischen Mechanismus verstehen, insbesondere angesichts eigener SIM-/Sicherheitskarten der Geräte.

Die Beobachtung ist als **Nutzerbeobachtung eines nicht näher identifizierten Systems** erhalten. Sie ist kein Nachweis, dass diese Funktion damals bereits in NetCore implementiert oder dort getestet war.

### 2.2 Die drei Gesprächsschritte

| Schritt | Nutzerfrage / Gedanke | Damalige Assistentenantwort | Historischer Beweiswert |
|---|---|---|---|
| H1 | Warum springen die Statusanzeigen mehrerer Geräte eines RTW gemeinsam um? | Erklärung über gemeinsamen „ISSI/GSSI-Kontext“, eine logische Fahrzeug-ISSI und automatische Spiegelung durch das TETRA-Netz. Als mögliche Geräte wurden MRT, HRT und MDT/Tablet genannt. | Unbelegte und teilweise falsche Erklärung; keine konkrete Systemanalyse. |
| H2 | Jedes Gerät hat eine SIM; können drei Geräte gleichzeitig dieselbe ISSI nutzen? | Einerseits eigene ISSI pro Gerät, andererseits gemeinsame „operative Fahrzeug-ISSI“, OPTA beziehungsweise „Alias-ISSI“. Behauptet wurde, der Status werde nicht unter der echten ISSI, sondern der Fahrzeugidentität versendet. | Begriffe und Protokollschichten wurden vermischt; keine Karten-/Netzprüfung. |
| H3 | Könnte die Leitstelle „Status empfangen“ zurücksenden und dadurch die anderen Anzeigen aktualisieren? | Grundsätzlich bejaht, aber erneut als SDS „an die OPTA“ mit automatischem Empfang aller gleich bezeichneten Geräte dargestellt. Dazu Gleichsetzung von Quittungsarten und Behauptung, immer gewinne der zuletzt von der Leitstelle bestätigte Status. | Die Idee einer anwendungsseitigen Rückverteilung ist plausibel; Zieladressierung, tatsächlicher Rückmeldeinhalt und Konfliktregel wurden nicht belegt. |

Die Beispiele „Status 1“ und „Status 3 / Einsatz übernommen“ dienten nur der Veranschaulichung. Sie legen weder einen NetCore-Statusnummernplan noch die tatsächlichen 16-Bit-Werte auf der Luftschnittstelle fest. Eine Statustaste, ein Benutzerlabel und ein codierter TETRA-Statuswert sind nicht ungeprüft gleichzusetzen.

### 2.3 Historisches Ergebnis und Nicht-Entscheidungen

Der Gesprächsstand endete bei einer Erklärungshypothese, nicht bei einem Entwicklungsauftrag. Es wurden weder ein Statusserver beschlossen noch Dateien erstellt, Dienste installiert, Ports zugewiesen oder Tests durchgeführt. Die frühere Assistentensicherheit ist kein Nachweis.

Als Nebenideen angeboten, aber nicht ausdrücklich beauftragt oder entschieden, waren: die Zuordnung von Funkgeräten zu einem Fahrzeug erklären; die Kopplung ändern; zwischen persönlicher und Fahrzeugrolle wechseln; automatische Fahrzeuganmeldung prüfen; Rückmelde-SDS beziehungsweise Quittungen diagnostisch unterscheiden. Die dabei erwähnte Bezeichnung `SDS-OPTA-Set` war nicht mit einer Spezifikation belegt und darf nicht als vorhandener standardisierter Befehl übernommen werden.

## 3. Nachträgliche technische Korrektur des Gesprächs

**Dieser Abschnitt entstand bei der Archivierung. Er ersetzt nicht stillschweigend den historischen Verlauf und wird nicht als damalige Nutzerentscheidung ausgegeben.**

### 3.1 Individuelle Identität ist keine Fahrzeug-Sammelanmeldung

ETSI EN 300 392-1 V1.6.1, Abschnitt 7.1, Seite 26, unterscheidet ausdrücklich: Eine individuelle Teilnehmeridentität bezieht sich auf einen mobilen oder festen Anschluss; eine Gruppenidentität kann mehrere Anschlüsse umfassen. Abschnitt 7.2 definiert TSI mit 48 Bit und SSI mit 24 Bit. Die vollständige ITSI enthält den Netzkontext aus MCC und MNC sowie die ISSI. Eine numerisch gleiche SSI in verschiedenen Netzen ist deshalb nicht automatisch dieselbe vollständige Identität. [N1]

Für drei unabhängige Geräte im selben relevanten Netz ist die korrekte Ausgangsannahme **drei individuelle Teilnehmeridentitäten**, die auf Anwendungsebene demselben Fahrzeug zugeordnet werden können. Ein gemeinsamer Fahrzeugstatus erfordert keine gemeinsame ISSI. Die frühere Behauptung einer regulären Dreifachanmeldung derselben Fahrzeug-ISSI ist zurückzunehmen.

Wie eine bestimmte Infrastruktur auf doppelt konfigurierte individuelle Identitäten reagiert, wurde nicht geprüft. Es wird weder ein bestimmter Ablehnungsfehler noch ein garantiertes gegenseitiges Abmelden behauptet. Auch sind normierte Aliasidentitäten ATSI/ASSI keine Begründung für die behauptete Fahrzeug-Sammelanmeldung: Abschnitt 7.2.2/7.2.3 beschreibt sie als netzseitige Aliaszuordnung zur individuellen Identität, nicht als OPTA. Die besondere Anmerkung zu räumlicher ASSI-Wiederverwendung unter kontrolliertem Roaming ist ebenfalls kein allgemeines Multi-SIM-Verfahren. [N1]

### 3.2 SIM/TSIM und OPTA sauber trennen

Die angehängte EN 300 812 V2.1.1 beschreibt in Abschnitt 10.3.2 das sechs Byte große `EF_ITSI` mit MCC, MNC und ISSI. Das belegt eine standardisierte Ablage der Teilnehmeridentität auf dieser SIM-Schnittstelle. Es belegt **nicht**, dass Karten der konkret gemeinten BOS-Geräte ausgelesen wurden oder deren gesamte Implementierung dieser historischen Schnittstellenedition entspricht. Die allgemein formulierte Aussage „jede SIM hat ihre eigene ISSI“ muss als Aussage über getrennt provisionierte Teilnehmer verstanden werden, nicht als dokumentierter Kartenbefund. [N3]

Die OPTA ist eine operativ-taktische alphanumerische Kennung. Eine offizielle THW-Darstellung beschreibt sie als 24-stelligen Klartextdatensatz zur Identifikation/Anzeige; sie ist nicht mit einer 24-Bit-ISSI zu verwechseln. Ob verschiedene Geräte eines Fahrzeugs identische oder durch Geräte-/Funktionszusätze unterschiedliche OPTAs haben, wurde für den Fall nicht ermittelt. [W1]

Eine Anwendung darf nach einer Fahrzeugkennung oder OPTA suchen und daraus passende Empfänger auflösen. Die tatsächliche Nachricht benötigt anschließend eine unterstützte technische Zieladressierung, etwa einzelne Teilnehmer oder eine ausdrücklich eingerichtete Gruppenadresse. **Ein identischer OPTA-Text bewirkt nicht von selbst eine TETRA-Multicastzustellung.** Dies ist die aus dem Identitätsmodell und dem unten geprüften konkreten Routing abgeleitete Korrektur; kein behaupteter Mitschnitt des RTW-Systems. [N1, R1, R3]

### 3.3 Rückmeldung ja – aber nicht jede Quittung bedeutet dasselbe

Die Nutzerhypothese lässt sich technisch so formulieren: Eine Leitstellen- oder Statusanwendung kann den empfangenen Status einem Einsatzmittel zuordnen und anschließend den übernommenen Zustand an dessen zugeordnete Geräte zurückverteilen. Ob dies im beobachteten System geschieht, bleibt ohne Systemunterlagen oder Mitschnitt offen.

Dabei sind vier Ebenen zu trennen:

| Ebene | Aussage | Was daraus nicht automatisch folgt |
|---|---|---|
| LLC-/Layer-2-Quittung | Ein Übertragungsschritt zwischen MS und Infrastruktur beziehungsweise umgekehrt wurde bestätigt. | Noch keine inhaltliche Auswertung durch eine Leitstelle. |
| SDS-TL `message received` | Empfang und Decodierung der betreffenden SDS-TL-Nachricht am Ziel wurden bestätigt. | Noch keine pauschale Einsatzmittel-Statusübernahme. |
| SDS-TL `message consumed` | Die Zielanwendung hat die Nachricht nach ihrer anwendungsspezifischen Definition genutzt. | Ohne Anwendungsvertrag keine universelle Bedeutung „Fahrzeugstatus verbindlich geändert“. |
| Fachliche Status-/Display-Rückmeldung | Eine Anwendung übermittelt einen angenommenen Status oder einen anzuzeigenden Text. | Nicht automatisch Beweis, dass alle weiteren Geräte ihn erhalten oder denselben internen Status gesetzt haben. |

Die ersten drei Ebenen sind in EN 300 392-2 V3.8.1, Abschnitt 29.3.2.2, Seiten 1186–1188 beschrieben. Die grafisch geprüften Abbildungen 29.4 und 29.5 auf Seite 1187 zeigen zusätzliche Ende-zu-Ende-Reports neben den separaten BL-ACKs. Daraus folgt gerade keine automatische Spiegelung an weitere Fahrzeugeinheiten. Abschnitt 14.5.5, Seiten 278–279, trennt die Übergabe eingehender `D-STATUS`/`D-SDS-DATA` an die Anwendung vom Senden entsprechender Uplink-Nachrichten. [N2]

Ein Text „Status empfangen“ allein benennt zudem nicht zwingend, welcher Zustand auf einem anderen Gerät dargestellt werden soll. Dafür braucht es beispielsweise den Statuswert/Statustext in der Rückmeldung oder einen passenden, bereits bekannten Anwendungskontext. Ebenso kann ein Displaytext geändert werden, ohne dass die zuletzt gedrückte lokale Statustaste oder ein herstellerspezifischer interner Statusautomat mitgeändert wird.

### 3.4 Kein allgemeines „zuletzt quittiert gewinnt“

Eine verbindliche Reihenfolge erfordert einen definierten Zustandsbesitzer und Konfliktregeln. Nachrichten können unterschiedliche Verzögerungen haben; Rückmeldungen allein legen keine netzweite Zustandsordnung fest. Die frühere Aussage, stets setze sich der zuletzt von der Leitstelle bestätigte Status durch, war unbelegt. Im geprüften lokalen NetCore-Sync-Pfad gibt es gerade keine vorgeschaltete Leitstellenentscheidung und keine globale Statusrevision. [R1]

### 3.5 Ergänzender öffentlicher Praxisbeleg – nicht das Nutzersystem

Die Leitstelle Lausitz beschreibt auf ihrer eigenen Digitalfunkseite Rückmeldungen des Einsatzleitsystems auf das Funkgerätedisplay. Dort wird außerdem die Umstellung dieser Statusrückmeldungen zum **01.10.2025 von PID 220 auf PID 204** und eine dafür erforderliche Geräte-Softwareversion MR2024.1a oder neuer genannt. Das belegt ein konkretes Betreiberbeispiel und die Bedeutung passender Endgeräteprofile, aber weder Fahrzeug-Fanout im hier beobachteten RTW noch eine allgemeine Migrationsanweisung für NetCore. [W2]

**Folgerung für NetCore:** Die im Code verwendete PID 220 nicht allein aufgrund dieses Beispiels blind ersetzen. Zunächst Hersteller, Firmware, Codeplug und tatsächlichen Nachrichtenvertrag der vorgesehenen Geräte feststellen.

## 4. Anforderungen, Entscheidungen und Reifegrad

| Gegenstand | Einordnung zum historischen Chat | Zusätzlich geprüfter Stand am 2026-10-04 |
|---|---|---|
| Ursache der gemeinsamen Anzeige verstehen | Nutzerziel | Technisches Modell korrigiert; reales RTW-System weiterhin nicht identifiziert. |
| LST-/Statusserver verteilt Rückmeldung | **Idee / Hypothese** des Nutzers | Als mögliche Anwendungsarchitektur sinnvoll; nicht als konkreter BOS-Ablauf nachgewiesen. |
| Gemeinsame Fahrzeug-ISSI / Routing an OPTA | Frühere falsche beziehungsweise unbelegte Assistentenerklärung | Als Grundlage verworfen; individuelle Adressierung und separate Zuordnung verwenden. |
| Directory-Gerätegruppen mit `status_sync` | Kein historischer Implementierungsnachweis | **Implementiert**: Datenmodell und Auflösung im gelesenen Repository vorhanden. |
| U-STATUS → Label → einzelne HMD-Display-SDS | Kein damaliger Entwicklungsauftrag | **Implementiert**: konkreter BS-Code und Aufruf aus CMCE vorhanden. |
| Wiederholung bei Wiederanmeldung und Gruppenänderung | Im Ursprungsgespräch nicht ausgearbeitet | **Implementiert** als lokale Cache-/Replay-Funktionen; keine hier nachgewiesene Funkabnahme. |
| Drei reale RTW-Geräte synchron, auch bei Ausfällen | Nutzerbeobachtung in fremdem/unklarem System | Für NetCore weder **getestet** noch **im Betrieb bestätigt**. |
| Persistenter, zellübergreifender, leitstellenverbindlicher Fahrzeugstatus | Kein Beschluss | **Roadmap-Kandidat / zu klärendes Ziel**, nicht aus lokalen Funktionen ableitbar. |
| Diese Dokumentation samt Index in `Archiving` | **Beschlossen / autorisiert** durch Archivauftrag | Dokumentationsänderung; keine Freigabe zur Änderung der Funk-/Backendlogik. |

„Implementiert“ bedeutet hier, dass ausführbarer Quelltext für den beschriebenen Teil vorliegt. Es bedeutet nicht automatisch „gebaut“, „installiert“, „normkonform abgenommen“ oder „betrieblich bewährt“.

## 5. Separater heutiger Repository-Befund

### 5.1 Quellen und Prüfmethode

Branch und Commit wurden über die GitHub-Schnittstelle gelesen. Pfade wurden teilweise über die GitHub-Codesuche gefunden; deren Suchtreffer verwiesen auf einen anderen Suchindexstand. Für die nachstehenden Kernaussagen wurden die Dateien deshalb **erneut ausdrücklich am Commit `2d4fbd627a1ecc9508f8bf52783bbc7fffb414c4` gelesen**. Der Suchindexstand ist nicht der geprüfte Branchstand.

Hauptquelle ist `crates/tetra-entities/src/cmce/subentities/sds_bs.rs`, Blob `09533926ec7272a46813c295c0f3f67628dd750c`. Ergänzend wurden die CMCE-Einbindung, das Python-Directory-Datenmodell und die passende Wiki-Beschreibung geprüft. Die genaue laufende Directory-Variante wurde nicht festgestellt. [R1–R4]

### 5.2 Vorhandener Datenfluss

```text
Unabhängiges Funkgerät A, eigene Teilnehmeridentität
    │ U-STATUS: Ziel-SSI und codierter Status
    │ Empfangskontext liefert die Quellidentität
    ▼
CmceBs → SdsBsSubentity::route_status_deliver
    │
    ├─ kein SDS-TL-Kurzreport?
    │    └─ handle_directory_status_label
    │         ├─ GET Directory /api/status → Statuslabel
    │         ├─ GET /api/status-group-members?issi=A → Mitglieder
    │         ├─ lokaler Statuscache + Dashboard-Ereignis je Mitglied
    │         └─ einzeln adressierte Type-4-HMD-SDS an A, B, C ...
    │              Quelle im Code: 4010001
    │              Ziel: jeweilige individuelle ISSI
    │              PID: 220; Text: "Status: <Label>"
    │
    └─ danach reguläre weitere Verarbeitung:
         lokale Kommandobehandlung / zentraler SDS-Handoff /
         lokale Zustellung / Brew-Pfad, abhängig von der Konfiguration
```

Die Pfeile beschreiben Quellcode, nicht einen aufgezeichneten Funkversuch. Vor allem ist die lokale Display-Rückmeldung **vor** dem zentralen SDS-Handoff angeordnet. Ein geänderter HMD-Text bedeutet in diesem Pfad daher zunächst lokale Verarbeitung – nicht zwingend fachliche Annahme durch die Leitstelle. [R1: Zeilen 1240–1410]

### 5.3 Identitäts- und Gruppenzuordnung

Die geprüfte Python-Directory-Implementierung unterscheidet Tabellen `devices`, `groups`, `device_groups` und `device_group_members`:

- `devices.issi` identifiziert ein Gerät im vorgesehenen Nummernraum.
- `groups.gssi` bezeichnet eine TETRA-Sprech-/Funkgruppe.
- `device_groups.group_id` bezeichnet eine organisatorische Gruppe, optional mit `opta`, `name`, `short`, `type`, `owner`, `color`, `status_sync`, `visible` und `notes`.
- `device_group_members` verknüpft `group_id` und `issi`; dies ist keine zusätzliche Funkanmeldung.

`groups_for_issi` findet sichtbare Gruppen des Absenders. Der HTTP-Handler `/api/status-group-members` vereinigt die Mitglieder der Gruppen mit aktivem `status_sync`, entfernt Dubletten und liefert `issi`, `count`, `groups` und `status_sync_members`. Die OPTA ist dabei ein Datenfeld, **nicht** der technische Schlüssel der Downlinkzustellung. [R3: Zeilen 54–120, 338–358 und 1014–1032]

Die BS akzeptiert primär `status_sync_members`, ergänzend `members` oder die passende verschachtelte `groups`-Struktur. Sie filtert ungültige Werte, schließt 0 aus, dedupliziert und nimmt den Absender in die Empfängerliste auf. Fehlen Konfiguration oder verwertbare Daten, fällt die Gruppenauflösung auf den Absender allein zurück. [R1: Zeilen 2355–2530]

### 5.4 Statusannahme und Displayübertragung

`route_status_deliver` liest die Quelle aus `received_tetra_address`, nicht aus einer angenommenen OPTA. Der Zielwert kommt aus `UStatus.called_party_ssi`. `PreCodedStatus::SdsTl(_)` wird von der Directory-Statuslabel-Verarbeitung ausgeschlossen: Ein SDS-TL-Kurzreport soll nicht als neuer Fahrzeugstatus gespiegelt werden. [R1: Zeilen 1240–1330]

Ein Statuslabel muss über `status_directory_lookup` gefunden werden; ohne Eintrag endet dieser Sync-Pfad. Mit Eintrag werden `last_status_by_issi` und Dashboard für die Mitglieder aktualisiert. Die HMD-SDS wird an den ursprünglichen Sender und an weitere **lokal registrierte** Mitglieder ausgelöst. Für andere Mitglieder wird in diesem Funktionspfad zunächst nur der lokale Cache aktualisiert. Eine netzweite Zustellung an in fremden Zellen registrierte Geräte ist damit nicht nachgewiesen. [R1: Zeilen 2210–2295]

`send_home_mode_display_text` erzeugt Type-4-Nutzdaten nach dem im Code verwendeten Schema:

```text
DC 00 MR 01 <Textbytes>
│  │  │  └─ im Code Latin-1-Textcodierung
│  │  └──── laufende ein Byte große Nachrichtenreferenz
│  └─────── TRANSFER ohne angeforderten Zustellreport / Store-and-forward
└────────── PID 220, im Projekt als Home Mode Display verwendet
```

Das ist eine **Beschreibung der vorhandenen Implementierung**, keine aus den angehängten ETSI-Normen abgeleitete herstellerübergreifende Garantie für diesen Displaydienst. Der aktuelle Statustext wird in diesem Pfad als Text versendet, nicht als numerischer Fahrzeugstatus in einem gesonderten Anwendungsfeld. [R1: Zeilen 2298–2352]

Die Zustellung erfolgt hier unmittelbar und mit erzwungenem MCCH-Pfad durch `deliver_d_sds_data_now(..., SsiType::Issi, ..., true)`. Das unterscheidet sich von allgemeinen SDS-Pfaden mit Warteschlangen. Die Annahme, der Sender habe gerade den MCCH benutzt, lässt sich nicht ungeprüft auf jedes andere Gruppenmitglied übertragen. [R1]

### 5.5 Cache, Replay, Anzeige und Nebenwirkungen

`last_status_by_issi` enthält pro ISSI Statuscode sowie Label/Schweregrad/Beschreibung. `handle_subscriber_update` wiederholt bei `Register` den im Cache vorhandenen Zustand. Die CMCE-Einbindung ruft den SDS-Handler beim entsprechenden Subscriber-Update tatsächlich auf. Das ist ein belegter Codepfad, aber keine Bestätigung eines gelungenen Wiedereinbuchungstests. [R1: Zeilen 2150–2200; R2: Zeilen 269–348]

`refresh_status_groups_from_directory` ist als periodische Funktion vorhanden: Sie betrachtet gespeicherte Statuswerte als Ausgangspunkte, löst Mitgliedschaften erneut auf und aktualisiert abweichende Einträge; bei lokal registrierten Mitgliedern kann sie die Displayantwort erzwingen. Die Konfiguration nennt dafür fünf Sekunden. Die Wirkung im laufenden System wurde nicht gemessen. [R1: Zeilen 2061–2150]

Für das Dashboard erzeugt `emit_status_dashboard` synthetische `SdsLog`-Ereignisse mit **PID 218** und Text `Status: <Label>` beziehungsweise zusätzlicher Beschreibung. PID 218 ist hier eine interne Kennzeichnung des Darstellungswegs, nicht die PID der HMD-SDS. Außerdem setzt das erzeugte Ereignis die jeweilige Mitglieds-ISSI als Quelle; die ursprüngliche auslösende ISSI wird in diesem einzelnen Anzeigeereignis nicht separat mitgeführt. Das ist bei späterer Auditierung zu berücksichtigen. [R1: Zeilen 2180–2230]

## 6. Relevante Dateien, Schnittstellen und Parameter

### 6.1 Dateien und Verantwortlichkeiten

| Pfad | Verantwortung / Prüfstand |
|---|---|
| `crates/tetra-entities/src/cmce/subentities/sds_bs.rs` | Gelesener Status-/SDS-Hauptpfad, Directory-Abfragen, HMD-Sendepfad, Caches und Replay. |
| `crates/tetra-entities/src/cmce/cmce_bs.rs` | Geprüfte Weiterleitung der Status-PDUs und Subscriber-Updates an SDS; Tick-Einbindung der SDS-Subentity. |
| `misc/ID-Server/netcore_directory_server.py` | Geprüfte Python-Referenz für SQLite-Datenmodell, Gerätegruppen und Statusgruppen-API; nicht als tatsächlich laufender Dienst identifiziert. |
| `wiki/Device-Groups.md` | Vorhandene Beschreibung organisatorischer Gruppen und `status_sync`; Statusgruppe ausdrücklich nicht gleich Sprechgruppe/GSSI-Affiliation. |
| `system-backend/directory/` | Im Repository vorhandener Backend-Bereich; konkrete produktive Variante und Deployment müssen separat abgeglichen werden. |
| `system-backend/sds-router/` | Vorhandener zentraler SDS-Baustein; kein im Ursprungschat neu beschlossener Statusserver. Die globale Fahrzeugstatuslogik wurde dort nicht vollständig geprüft. |
| `Docs/archive/README.md` | Bestehender Archivindex, nur um diesen Chat ergänzt. |

### 6.2 Parameter des geprüften BS-Sync-Pfads

| Element | Wert / Verhalten | Grenze |
|---|---|---|
| Individualadressierung | `SsiType::Issi`; Parserbereich bis `0xFFFFFF`, 0 wird als Mitglied ausgeschlossen | Keine Vergaberichtlinie für reale Teilnehmer; Netzkontext weiterhin wichtig. |
| Statuscode | `u16` / 16 Bit | Nicht automatisch identisch mit Beschriftung der Statustaste. |
| Reply-Quelle | `DASHBOARD_ISSI = 4010001` | Quellcodekonstante, kein Nachweis der realen BOS-Leitstellenadresse. |
| HMD-Protokollkennung | `220` / `0xDC` | Geräte-/Codeplugkompatibilität offen. |
| Dashboard-Statuskennung | `218` | Synthetisch; nicht als Funkpayload übernehmen. |
| HMD-Text | `Status: <Label>`, ungefähr auf 64 Zeichen begrenzt, Latin-1-Ersatz | Darstellungs-/Sonderzeichentests fehlen. |
| Wiederholungsdrossel | `STATUS_HMD_REPLY_THROTTLE = 30 s` | Schlüssel ist `(ISSI, Statuscode)`, nicht nur der letzte angezeigte Zustand. |
| Label-Cache | `STATUS_DIRECTORY_REFRESH = 30 s` | Kein zugesichertes Ende-zu-Ende-Latenzziel. |
| Mitglieder-Cache / Poll-Konstante | jeweils `5 s` | Quelle sind die genannten Konstanten und Funktionen, keine gemessene Aktualisierungszeit. |
| Directory-Standardziel | `http://127.0.0.1:8095` | Prozesslokaler Default, nicht automatisch separater Directory-LXC. |
| Directory-Standardaktivierung | `enabled = false` in der betrachteten Runtime-Konfiguration | Tatsächliche TOML-/Environment-Werte nicht gelesen. |
| HTTP-Timeout | Default `1000 ms`, Begrenzung `250..10000 ms` | Synchrone Netzwerkoperation; keine Messwerte. |
| Gruppenabfrage | `GET /api/status-group-members?issi=<ISSI>` | Mitgliedschaften, nicht verbindlicher globaler Fahrzeugstatus. |
| Statuskatalog | `GET /api/status` | Labels/Metadaten, nicht automatisch ein Journal aktueller Fahrzeugzustände. |
| Transport zur Directory-API | HTTP/JSON; Beispieldefault TCP 8095 | HTTPS/Proxy/Authentifizierung der realen Umgebung nicht nachgewiesen. |
| Funkseite | TETRA CMCE, U-STATUS, D-STATUS, D-SDS-DATA / SDS Type 4 | Keine für diesen Chat festgelegten Funkfrequenzen oder HF-Parameter. |

`[netcore_directory]` wird im untersuchten Pfad mit `enabled`, `base_url`, `timeout_ms` verarbeitet. Relevante Environment-Overrides sind `NETCORE_DIRECTORY_URL`, `NETCORE_DIRECTORY_ENABLED` und `NETCORE_DIRECTORY_TIMEOUT_MS`. Für Konfigurationspfade werden `FLOWSTATION_CONFIG`, `TETRA_CONFIG`, `BLUESTATION_CONFIG`, CLI-Argumente wie `--config`/`-c` sowie lokale Standardpfade berücksichtigt. Gelesene Kandidaten sind unter anderem `config.toml`, `/opt/tetra/config.toml`, `/opt/flowstation/config.toml`, `/opt/tetra-bluestation/config.toml` und `/etc/flowstation/config.toml`. [R1: Zeilen 2653–2800]

Diese Aufzählung enthält keine Zugangsdaten und keine Behauptung, welcher Pfad produktiv verwendet wird. MQTT-, SIP-, VPN-, HF- oder LXC-Parameter aus anderen Projektchats werden hier nicht als historische Festlegungen übernommen.

## 7. Fehler, Ursachen und noch offene technische Risiken

### 7.1 Tatsächlicher Fehler dieses Gesprächs: falsche Erklärung

Die wichtigste belegte Fehlleistung ist keine Funkstörung, sondern die frühere Assistentenantwort. Teilnehmeridentität, Gruppenidentität, taktische Kennung, Leitstellenressource und Displayzustand wurden vermischt. Aus einer sichtbaren Synchronisation wurde ohne Nachweis auf automatisches Netzrouting geschlossen. Außerdem wurden unterschiedliche Quittungsebenen gleichgesetzt.

Die funktionierende Korrektur besteht in der begrifflichen Trennung und dem belegten konkreten Codepfad. Für das ursprünglich beobachtete RTW-System bleibt der Mechanismus weiterhin zu ermitteln; eine Dokumentationskorrektur ist kein dort durchgeführter Reparatureingriff.

### 7.2 Statisch erkennbare Prüfpunkte im aktuellen Code

Die folgenden Punkte sind **neu bei der Archivprüfung abgeleitete Risiken**, keine im Chat beobachteten Störungen und keine in diesem Auftrag behobenen Bugs. [R1]

| ID | Befund / nachvollziehbare Ursache | Nächste Prüfung oder mögliche Verbesserung |
|---|---|---|
| SYNC-01 | Display-Sync wird vor zentralem SDS-Handoff ausgelöst. Eine lokale Rückmeldung kann daher auch ohne bestätigte fachliche LST-Übernahme entstehen. | Semantik ausdrücklich festlegen: „lokal empfangen“ versus „von Leitstelle übernommen“. Ggf. getrennte Ereignisse/Anzeige. |
| SYNC-02 | Drossel speichert `(ISSI, Code)` für 30 s. Bei A → B → A innerhalb dieses Fensters kann die zweite A-Anzeige unterdrückt werden, obwohl B zwischenzeitlich angezeigt wurde. | Deterministischen Regressionstest ergänzen; nur echte Wiederholung des bereits aktuellen Zustands unterdrücken oder Zustandsrevision berücksichtigen. |
| SYNC-03 | Gruppenrefresh verwendet einen Snapshot aus einer HashMap mit Statuswerten verschiedener ISSIs. Unterschiedliche Altzustände bei Zusammenführung/überlappender Mitgliedschaft haben keine explizite globale Revision oder Priorität. | Szenarien mit widersprechenden Ausgangswerten testen; einen autoritativen Gruppen-/Fahrzeugzustand und klare Konfliktregel entwerfen. |
| SYNC-04 | HMD-Antwort wird unmittelbar auf MCCH geschickt. Weitere Gruppenmitglieder können gerade auf Verkehrskanälen oder außerhalb eines Energiespar-Empfangsfensters sein. | Empfang unter Gruppenruf, Einzelruf und Energiesparbetrieb messen; Sendestrategie/Replay passend zu realen Terminalfähigkeiten wählen. |
| SYNC-05 | Nicht lokal registrierte Mitglieder werden im betrachteten Fanout zunächst nur gecacht. | Tatsächliche Zuständigkeit zwischen Zellen und zentralem SDS-Router nachvollziehen; Remote-Zustellung separat abnehmen. |
| SYNC-06 | Letzter Status liegt im beschriebenen BS-Pfad in einer prozesslokalen Map. Kein Persistenz-/Wiederherstellungspfad dafür wurde nachgewiesen. | Neustarttest und Sollverhalten definieren; bei Bedarf vorhandene zentrale Zustands-/Persistenzbausteine nutzen. |
| SYNC-07 | Directory-Abfragen verwenden `reqwest::blocking`; Mitgliedercache hält während des Abrufs einen Mutex. | Stack-Timing bei langsamer/ausgefallener Directory-API prüfen; asynchron vorgeladene Snapshots erwägen. Keine Latenz wurde gemessen. |
| SYNC-08 | Unbekannter Statuscode hat kein Label und beendet den Display-Sync-Pfad. Gruppenabfragefehler reduzieren auf den Sender. | Verständliche Diagnose, Zustandsfrische und Fehleranzeige; definieren, ob unbekannte Codes gespiegelt werden dürfen. |
| SYNC-09 | Replizierte Dashboard-Ereignisse tragen die Mitglieds-ISSI, nicht zusätzlich die ursprüngliche Auslöser-ISSI. | `origin_issi`, Gruppen-/Fahrzeugbezug und Verarbeitungsphase für Audit ergänzen; nicht bloß Anzeigezeilen als echte Uplink-Ereignisse zählen. |
| SYNC-10 | PID 220 und Textformat sind konkret im Code festgelegt; passende Geräteunterstützung ist nicht belegt. | Hersteller-/Firmware-/Codeplugmatrix einschließlich ggf. PID 204 erstellen; keine pauschale PID-Ersetzung. |

Weitere Begrenzungen: Ein auf einem Gerät sichtbarer Text beweist keine Zustellung an alle anderen. Das HMD-Payload fordert im betrachteten Pfad keinen SDS-TL-Zustellreport an. Ein vorhandenes Directory-Gruppendatenmodell ist außerdem nicht gleichbedeutend mit einer bereits verbindlichen netzweiten Zustandsmaschine.

## 8. Befehle und Diagnoseabläufe

### 8.1 Was tatsächlich ausgeführt wurde

Im ursprünglichen technischen Gespräch wurde **kein** Installations-, Deployment-, Reparatur- oder Testbefehl ausgeführt. Bei der Archivierung erfolgten GitHub-Lesezugriffe, PDF-Inventarisierung inklusive SHA-256 und gezielte Text-/Bildprüfung.

Ein lokaler Git-Clone-Versuch für `Archiving` scheiterte in der Arbeitsumgebung an der DNS-Auflösung von GitHub, Rückgabecode 128. Deshalb wurden die Quelltextprüfung und die Archivspeicherung über die verfügbare GitHub-Schnittstelle durchgeführt; aus dem fehlgeschlagenen Clone folgt weder ein Repository-Fehler noch ein fehlender GitHub-Schreibzugang. Es wurden kein Cargo-Build, kein CI-Lauf und kein Funkgerätetest als erfolgreich ausgegeben.

### 8.2 Vorgeschlagene spätere Read-only-Diagnose – nicht ausgeführt

Die folgenden Aufrufe dienen einer späteren Prüfung auf einem autorisierten Testsystem. Beispiel-ISSIs sind **keine** Vergabe für den BOS-Betrieb. Die Directory-Adresse muss die tatsächliche Instanz bezeichnen; `127.0.0.1` meint immer den Rechner, auf dem der Aufruf läuft.

```bash
# Erst nach Prüfung der realen Directory-Adresse verwenden.
DIRECTORY_URL='http://127.0.0.1:8095'
ISSI='2020001'  # Nur Beispiel; durch das freigegebene Testgerät ersetzen.

curl --fail --silent --show-error --max-time 5 \
  "$DIRECTORY_URL/api/status-group-members?issi=$ISSI"

curl --fail --silent --show-error --max-time 5 \
  "$DIRECTORY_URL/api/status"
```

Die erste Antwort sollte die beabsichtigten individuellen Empfänger in `status_sync_members` erkennen lassen. Die zweite Antwort muss den tatsächlich gesendeten codierten Status enthalten. Ein HTTP-200 allein ist kein Beleg für richtige Gerätezuordnung oder Funkzustellung.

Vor dem Funkversuch sind Source-Commit, tatsächlich gestartetes Binary, aktive Konfigurationsdatei, Geräte-/Firmwarestände und Mitgliedschaften zu dokumentieren. Danach Uplink, lokale Verarbeitung, etwaige zentrale Annahme und die Downlinks getrennt korrelieren. Keine Passwörter, Schlüssel oder vollständigen unbereinigten Produktivmitschnitte ins Archiv übernehmen.

PEI kann für autorisierte Diagnose geeignet sein; die angehängte EN 300 392-5 beschreibt unter anderem `+CMGS` und `+CTSDSR`. Daraus wird hier bewusst kein ungetesteter Sende-Einzeiler für ein unbekanntes Funkgerät abgeleitet. [N4]

## 9. Tests, Ergebnisse und Grenzen

### 9.1 Bereits belegte Prüfungen

| Prüfung | Ergebnis | Grenze |
|---|---|---|
| Sichtbaren Gesprächsverlauf auswerten | Drei inhaltliche Frage-/Antwortpaare und Archivauftrag berücksichtigt. | Kein vollständiger Chat-Export und keine gesicherten Originalmetadaten. |
| Anhänge inventarisieren | 25 PDFs zugänglich; Seitenzahlen und SHA-256 ermittelt. | Keine Vollprüfung aller Norminhalte oder der gesamten Sammeldatei. |
| Identitäts- und Quittungsmodell prüfen | Relevante ETSI-Abschnitte einschließlich ausgewählter Abbildungen gelesen. | Kein Konformitätstest eines Geräts oder Netzes. |
| Repository statisch prüfen | Statusgruppen-, HMD- und Replay-Code sowie CMCE-Einbindung vorhanden. | Kein Build-/Installations-/Laufzeitnachweis. |
| Vorhandene Unit-Testsektion sichten | `sds_bs.rs` enthält am Dateiende LIP-/Textdecodierungstests. | Diese Tests sind keine Drei-Geräte-Status-Sync-Abnahme und wurden nicht ausgeführt. |
| Archiv speichern | Dokumentation und Index über GitHub im Zielbranch ablegen; Commit/Dateien nach dem Schreibvorgang prüfen. | Dokumentationsprüfung ist keine Produktabnahme. |

Die in `sds_bs.rs` sichtbaren Tests heißen `lip_short_report_decodes_to_position_text`, `incomplete_lip_payload_stays_unlabelled`, `sds_text_pid_09_decodes_plain_and_coded_text` und `sds_tl_text_pid_89_decodes_utf16_payload`. Eine vollständige Inventur aller Tests des Monorepos erfolgte nicht; aus dieser Liste darf nicht auf das Fehlen sämtlicher anderer SDS-Tests geschlossen werden. [R1: Zeilen 2960–Dateiende]

Kommentare anderer Codepfade mit Formulierungen wie „verified on-air“ wurden nicht als im Rahmen dieser Archivierung durchgeführte Tests übernommen.

### 9.2 Vorgeschlagene Abnahmematrix – sämtliche Fälle offen

| Fall | Prüfinhalt / gewünschter Nachweis |
|---|---|
| T1: drei Sender | Drei getrennte Testidentitäten in einer Statusgruppe; jedes Gerät nacheinander auslösen lassen; alle Anzeigen und echte Absenderzuordnung dokumentieren. |
| T2: kein Senderwechsel | Gleichen Code mehrfach senden; Drossel soll unnötige Wiederholungen verhindern, aber richtigen Zustand nicht verlieren. |
| T3: A → B → A | Innerhalb 30 s auslösen; prüfen, ob alle Displays wirklich auf den zweiten A-Zustand zurückkehren. |
| T4: nur ACK | LLC-ACK und SDS-TL-Report ohne neuen fachlichen Status; keine unerwünschte Status-Sync-Schleife. |
| T5: Directory-Ausfall | Nicht erreichbar, Timeout, ungültiges JSON, unbekannter Statuscode; definiertes und erkennbares Degradationsverhalten. |
| T6: Mitgliedschaft | Gerät hinzufügen, entfernen, `status_sync` abschalten; keine falsche Weitergabe an entfernte Geräte. |
| T7: Gruppen zusammenführen | Verschiedene Altzustände und überlappende Gruppen; stabile, dokumentierte Konfliktauflösung statt zufälliger Übernahme. |
| T8: Wiederanmeldung | Gerät aus-/einschalten bei laufender BS; Cache-Replay tatsächlich am Display belegen. |
| T9: BS-Neustart | Zustand vor/nach Prozessneustart vergleichen; Persistenz beziehungsweise bewusst verlorener Cache sichtbar machen. |
| T10: Zellenwechsel / Remote-Mitglied | Mitglieder auf verschiedenen TBS; Zuständigkeit und Rückverteilung Ende zu Ende dokumentieren. |
| T11: besetzt / Energiesparen | Status-Sync während Gruppen-/Einzelruf und bei Energiesparprofilen; MCCH-Annahme überprüfen. |
| T12: Anzeige versus fachlicher Zustand | HMD-Text, lokaler Gerätestatus, NetCore-Dashboard und Leitstellenstatus getrennt prüfen. |
| T13: Firmware-/PID-Matrix | Unterstützte Terminals, PID, Textcodierung, lange Labels und Sonderzeichen überprüfen. |
| T14: zentrale Nichtannahme | Zentralen Handoff verzögern/ablehnen oder im Test deaktivieren; lokale Rückmeldung darf nicht unbemerkt als verbindliche LST-Quittung erscheinen. |

Abnahmebelege sollten Zeitfolge, Quell-/Zielidentitäten, Statuscode, Payloadart, zentrale Verarbeitungsphase und die jeweils sichtbare Geräteanzeige enthalten. Vorgeschlagene Testfälle werden erst nach tatsächlicher Ausführung mit Datum, Umgebung und Ergebnis auf „getestet“ gesetzt.

## 10. Verworfene beziehungsweise ersetzte Ansätze

| Früherer Ansatz | Behandlung | Grund |
|---|---|---|
| Ein Fahrzeug hat für alle Geräte eine gemeinsam angemeldete ISSI. | Als Erklärung verworfen. | Individuelle Teilnehmeridentität und organisatorische Ressource wurden verwechselt. |
| OPTA sei eine „Alias-ISSI“ und ersetze beim Statussenden die wirkliche Identität. | Korrigiert. | OPTA und ATSI/ASSI gehören nicht in denselben Begriff; aktueller Code nutzt Quell-/Ziel-SSI und getrennte Directory-Zuordnung. |
| SDS „an die OPTA“ erreicht automatisch alle gleich benannten Geräte. | Als allgemeines Netzverhalten verworfen. | Eine Anwendung muss technische Empfänger auflösen oder eine ausdrücklich eingerichtete Gruppe adressieren. |
| Netz-ACK, SDS-TL-ACK und fachliche Statusbestätigung seien austauschbar. | Korrigiert. | Unterschiedliche Schichten und Bedeutungen nach EN 300 392-2. |
| Immer gewinnt der zuletzt durch die Leitstelle bestätigte Status. | Nicht als Eigenschaft übernehmen. | Kein historischer Nachweis; im lokalen Sync-Pfad keine derartige globale Ordnung. |
| Neue Status-Sync-Funktion vollständig von null bauen. | Nicht erforderlich als Ausgangspunkt der Fortsetzung. | Gegenüber dem damaligen Gespräch ist bereits konkreter Code vorhanden; zunächst Bestand abgleichen und abnehmen. |

Die Grundidee einer Rückverteilung des Fahrzeugzustands bleibt erhalten. Verworfen werden die unbelegten Mechanismusbehauptungen, nicht das fachliche Ziel einer konsistenten Fahrzeuganzeige.

## 11. Offene Aufgaben und Roadmap-Kandidaten

Die nachstehenden Prioritäten sind **Vorschläge aus der Abschlussprüfung**, keine rückwirkend behaupteten Nutzerbeschlüsse und keine Änderungen an einer zentralen Roadmapdatei.

| Priorität | Aufgabe | Abhängigkeit / Abschlusskriterium |
|---|---|---|
| P0 | Das ursprünglich gemeinte RTW-System identifizieren. | Modelle, Codeplugs, getrennte Geräte oder Bedienteile, Leitstelle und autorisierte technische Unterlagen; tatsächlichen Rückmeldepfad bestimmen. |
| P0 | Semantik der NetCore-Rückmeldung festlegen. | Soll „lokal empfangen“, „weitergeleitet“, „zentral angenommen“ oder nur ein Displaytext gezeigt werden? Keine irreführende LST-Bestätigung. |
| P1 | Bestehenden Directory-/BS-Sync mit drei Testgeräten abnehmen. | Aktiven Source-/Binary-/Konfigstand und individuelle IDs sichern; T1–T4 ausführen. |
| P1 | Drossel und Konfliktfälle absichern. | SYNC-02/SYNC-03 reproduzierbar testen; festgelegter Zustandsbesitzer und Reihenfolge. |
| P1 | Busy-/EE-/Offline-Verhalten und PID-Kompatibilität prüfen. | Geräteprofile und zulässiger Testbetrieb; T8, T11–T14. |
| P2 | Persistenz und zellübergreifende Zuordnung klären. | Vorhandene Directory-, Control-Room- und SDS-Router-Bausteine berücksichtigen; kein paralleles Schattenregister ohne Not. |
| P2 | Audit und Zustandsfrische verbessern. | Originalabsender, Ressource/Gruppe, Zeitpunkt/Revision, Verarbeitungsergebnis und angezeigten Zustand auseinanderhalten. |
| P2 | Directory-Abrufe vom zeitkritischen Pfad entkoppeln, falls Messung nötig macht. | Langsam-/Ausfalltests, aktualisierte Snapshotstrategie und definiertes Cache-Verhalten. |
| P3 | Dynamische Fahrzeug-/Rollenanmeldung und manuelle Entkopplung ausarbeiten. | Ursprünglich nur angebotene Nebenideen; gesonderte fachliche Freigabe erforderlich. |

Konkrete Fortsetzung: Zuerst Referenzsystem und gewünschte Semantik festhalten, danach den **vorhandenen** `status_sync`-Pfad mit eindeutigen Testidentitäten reproduzieren. Erst anhand der Ergebnisse entscheiden, welche Korrekturen implementiert werden sollen. Doppel-ISSIs oder gleiche OPTA-Texte sind kein sinnvoller Ersatz für das Zuordnungsmodell.

## 12. Anhänge und Bildarchiv

### 12.1 Bildstatus

Im verfügbaren ursprünglichen Gespräch gibt es **keine eigenständigen Chatfotos, Screenshots oder generierten Bildentwürfe**. Es wurden deshalb keine historischen Bilddateien nach GitHub hochgeladen oder aus anderen Chats übernommen. Die ETSI-PDFs enthalten Titelseiten, Diagramme und Tabellen; das macht sie nicht zu Fotos des beschriebenen RTW-Aufbaus.

Für die Quellenprüfung bei der Archivierung wurden einzelne PDF-Seiten dargestellt beziehungsweise lokal gerendert. Diese neu erzeugten Prüfansichten sind keine historischen Chatbilder und werden nicht als solche in das Repository aufgenommen. Die PDFs werden hier bibliografisch und über Prüfsummen dokumentiert, nicht als neue vollständige Normkopien hochgeladen. Es fehlt damit kein hier tatsächlich vorhandenes eigenständiges Chatbild.

### 12.2 Inventar und Auswertungstiefe

Legende: **V** = thematisch vertiefte Abschnittsprüfung; **E** = ergänzend gezielt eingesehen; **I** = Datei-/Titelseiteninventar und Relevanzeinordnung, keine inhaltliche Vollprüfung. „Draft“ beschreibt die **angehängte Ausgabe**, nicht einen recherchierten heutigen Gesamtstatus der Normenreihe.

| Anhang | Angefügte Ausgabe / Thema | Seiten | Tiefe |
|---|---|---:|---|
| `ETSI.pdf` | Sammel-PDF, 4.100 Seiten; beginnt mit EN 300 812 V2.1.1 (2001-12); keine einzelne Normedition | 4100 | I |
| `en_30039201v010601p.pdf` | EN 300 392-1 V1.6.1 (2020-04), General network design | 182 | V: Identitäten, Abschn. 7.1–7.2 |
| `en_30039202v030801p.pdf` | EN 300 392-2 V3.8.1 (2016-08), Air Interface | 1445 | V: Abschn. 14.5.5 und 29.3.2.2–29.3.2.4 |
| `en_3003920303v010301p.pdf` | EN 300 392-3-3 V1.3.1 (2011-11), ISI Group Call | 251 | I: kein Beleg für Fahrzeug-Status-Sync |
| `en_3003920304v010301p.pdf` | EN 300 392-3-4 V1.3.1 (2010-08), ISI Short Data Service | 28 | E: Scope und Themenzuordnung; kein Betreiberprofil |
| `en_3003920308v010401p.pdf` | EN 300 392-3-8 V1.4.1 (2020-04), Generic Speech Format Implementation | 22 | I |
| `en_3003920313v010201p.pdf` | EN 300 392-3-13 V1.2.1 (2020-04), transportunabhängiger ISI Group Call | 191 | I |
| `en_3003920315v010500a.pdf` | Draft EN 300 392-3-15 V1.5.0 (2026-04), ISI Mobility Management | 380 | I: Entwurf, nicht als finale Norm ausgegeben |
| `en_30039205v020701p.pdf` | EN 300 392-5 V2.7.1 (2020-04), Peripheral Equipment Interface | 320 | E: Abschn. 6.13 / direkte SDS-Kommandos |
| `en_30039207v030501p.pdf` | EN 300 392-7 V3.5.1 (2019-07), Security | 216 | I: keine konkrete Karten-/Authentifizierungsprüfung |
| `en_30039209v010701p.pdf` | EN 300 392-9 V1.7.1 (2020-04), allgemeine Supplementary-Service-Anforderungen | 46 | I |
| `en_3003921006v010401p.pdf` | EN 300 392-10-6 V1.4.1 (2006-08), Call Authorized by Dispatcher, Stage 1 | 20 | E: Scope; Rufautorisierung ist nicht Statusquittierung |
| `en_3003921018v010301p.pdf` | EN 300 392-10-18 V1.3.1 (2003-10), Barring of Outgoing Calls, Stage 1 | 17 | I |
| `en_3003921101v010201p.pdf` | EN 300 392-11-1 V1.2.1 (2004-01), Call Identification, Stage 2 | 44 | I |
| `en_3003921114v010101p.pdf` | EN 300 392-11-14 V1.1.1 (2002-07), Late Entry, Stage 2 | 23 | E: Scope; Gesprächseinstieg, kein Statusreplay-Beleg |
| `en_3003921117v010102p.pdf` | EN 300 392-11-17 V1.1.2 (2002-01), Include Call, Stage 2 | 18 | E: Scope; Gesprächsteilnehmer, keine Fahrzeugressource |
| `en_3003921201v010202p.pdf` | EN 300 392-12-1 V1.2.2 (2007-08), Call Identification, Stage 3 | 56 | I |
| `en_3003921216v010400a.pdf` | DRAFT EN 300 392-12-16 V1.4.0 (2026-03), Pre-emptive Priority Call | 67 | I: Entwurf |
| `en_30039401v030301p.pdf` | EN 300 394-1 V3.3.1 (2015-04), Conformance testing, Radio | 169 | I: Spezifikation, kein ausgefüllter Testbericht |
| `en_30039502v010303p.pdf` | EN 300 395-2 V1.3.3 (2025-02), TETRA speech codec | 94 | I: kein Schwerpunkt dieses Chats |
| `en_300812v020101p.pdf` | EN 300 812 V2.1.1 (2001-12), SIM-ME interface / Security aspects | 156 | V: EF_ITSI, Abschn. 10.3.2, S. 56–57 |
| `es_20081201v020205p.pdf` | ES 200 812-1 V2.2.5 (2003-12), UICC physical/logical characteristics | 8 | E: Scope und Abgrenzung zur TSIM-Anwendung |
| `es_20081202v020401m.pdf` | Final draft ES 200 812-2 V2.4.1 (2005-08), TSIM application characteristics | 139 | I: Entwurf; kein ergänzender Gerätebefund |
| `ets_30039214e01v.pdf` | Final draft prETS 300 392-14 (1997-09), PICS proforma | 61 | E: Scope/Proforma; kein Konformitätsnachweis |
| `ts_10081201v020205p.pdf` | TS 100 812-1 V2.2.5 (2003-10), UICC physical/logical characteristics | 8 | E: Scope; nicht identisch mit ES-Ausgabe trotz Versionsnummer |

### 12.3 Prüfsummen der tatsächlich verfügbaren Dateien

Diese SHA-256-Werte identifizieren die Anhänge dieses Archivdurchlaufs. Sie sind keine Hashes von Repository-Blobs und keine Konformitätsbescheinigungen. Die Rohdateien bleiben Anhänge; aus ihren Namen wird kein vorhandener Repository-Pfad abgeleitet.

```text
9434dad1e7bc80ca39b0edadd8e3b9995fda5ae5f5c3dd565af05708d5059e38  ETSI.pdf
788722e566098957bea1a9a5096e9dae1eace1da9f5d38c530c80230e5b9b9cb  en_30039201v010601p.pdf
3f07b1e4ad73fabc16277a900006fc19844b3a882bbc2bc1d74d78f6156daf28  en_30039202v030801p.pdf
94ca61038b3ff826e6adcfde56e00473dd996e1a47f9f84e7b9d2aa6c3133fd2  en_3003920303v010301p.pdf
8a38cc6238c6da726e4f07d7d377b2c827447a1448a60ee5ef2a126cb3a0ef9d  en_3003920304v010301p.pdf
4d993a25e35da7b1f2291ae281cb3ea5b3a7b702ab7eb3233bdf430a2c70909d  en_3003920308v010401p.pdf
b47d63853f83a6a4adda08c0e325c8ca13d358e2f8840bfba4bce430e600b1bd  en_3003920313v010201p.pdf
e959709667192e22d2dfeea51b48f9a579a4c107d952999056b7fad93a6d2100  en_3003920315v010500a.pdf
10aaf78988ae4080c3dfe90165ee7f4f3fe8eda402e09fc1e6fa0a0ad890758d  en_30039205v020701p.pdf
df47af9a642b6eba7d7d5cf5fb7aa3f961e9e03bb912316bd2d189ce47344e08  en_30039207v030501p.pdf
cad45938bcdf2fa4e8063d4aa9e7d7c06c45f729d3c55dc033e00c27af9f2e06  en_30039209v010701p.pdf
32b6ee43602ef88a0b5c209b1c69ad35ba8b2e660502257c2dfaa6205a346523  en_3003921006v010401p.pdf
4cc10bcd94d39201538639cefb529809729563f13f87cfe6ae52b44c253b0b8a  en_3003921018v010301p.pdf
852c17ea566757e80ecf0d2718ae7839d9e218aa4384b63d1277ed7e25d86c69  en_3003921101v010201p.pdf
ba1882df71dbb84a0c4f0f7dcea1dc8df968044457da03df9290a50e8821df32  en_3003921114v010101p.pdf
69ce800e352b1ecfc337416fd7b54c1bdf2d9270171af519ac2f5acc1ba85ca6  en_3003921117v010102p.pdf
4d5b56a188fb73c417d67b27da619cad2e49efb3b57306fd4ae51e140c018018  en_3003921201v010202p.pdf
c0ee7859f448c4072befc25905c29b751b5151d3590a880a2d654309a3ba6c02  en_3003921216v010400a.pdf
2d9c327e5ccf1476335e1b7a4f58a0ca7afc93911c30e8aad5ed7dc9ccf3276a  en_30039401v030301p.pdf
ac716ca18082cc0fd2ee768a14f03deb48fa78a77e83751b1a6b319c7fc2106a  en_30039502v010303p.pdf
196effeaae473a98244c89538e1562daa58bb1d74cbfe7de78e58ceeafbad18b  en_300812v020101p.pdf
346dc545049a7397ca86831731bbc05e96f61ff047d6e7f6b7de56da3aa408e9  es_20081201v020205p.pdf
330f045a908eefd249fd7450e5b71bd08e8a7b20ce5b86dc9b87c9138a5c7268  es_20081202v020401m.pdf
2b703f3ebb883c90dab40e1b3d1a634a6acf9c9a20d468fd108ea02226c2bf2c  ets_30039214e01v.pdf
96df75f5ddf1c3ae4ac75f796ee97babca68fa81aca22f39ba0e0658bd57eed1  ts_10081201v020205p.pdf
```

## 13. Archivierung, Änderungsgrenze und Fortsetzbarkeit

Der Auftrag verändert ausschließlich das vorliegende Markdown und `Docs/archive/README.md`. Bestehende Archive werden nicht umgeschrieben; die gleichzeitige Pflege durch andere Chats ist durch erneutes Lesen des Branchkopfs/Index vor der Speicherung und einen nicht erzwungenen Fast-forward zu berücksichtigen. Der geprüfte Quellcode bleibt an den in Abschnitt 14 verlinkten Commit gebunden, auch wenn weitere Archive zwischenzeitlich ergänzt wurden.

Die Speicherung erfolgt als zusammengehörige Änderung von Dokument und Index. Der erzeugte Archiv-Commit ist über GitHub nachvollziehbar; seine tatsächliche ID gehört in die Abschlussmeldung. Nach dem Speichern sind Branch, beide Dateien und die auf `Docs/archive/` begrenzte Änderung zu verifizieren. Ein Merge in andere Branches und ein Force-Push sind nicht Teil des Auftrags.

Nicht im Archiv enthalten sind Passwörter, Tokens, private Schlüssel oder sonstige Zugangsdaten. Quellcodeparameter wie PID, Port, Funktionsname oder die lokale Service-ISSI werden dokumentiert, nicht mit geheimen Zugangswerten verwechselt.

Für spätere Arbeiten ist dieses Dokument ein Einstieg: historische Erklärung nicht wiederverwenden, sondern Abschnitt 3 und den tatsächlichen vorhandenen Codepfad in Abschnitt 5 zugrunde legen. Neue Testergebnisse müssen mit Umgebung und Ausführungsdatum ergänzt werden; bloße Chatbehauptungen oder Codekommentare sind keine Betriebsbestätigung.

## 14. Quellen und Querverweise

### 14.1 Angefügte Normen – maßgebliche Fundstellen

**[N1] ETSI EN 300 392-1 V1.6.1 (2020-04)**, Anhang `en_30039201v010601p.pdf`: Abschnitt 7.1, S. 25–26; 7.2.1–7.2.5, S. 27–29. Individuelle versus Gruppenidentitäten, 48-/24-Bit-Struktur, ITSI/ISSI sowie ATSI/ASSI. Abbildung 2 auf S. 27 und Abbildung 3 auf S. 29 wurden visuell einbezogen. Ergänzende Suchstellen zum Adressgebrauch in 7.8 ändern diese Unterscheidung nicht.

**[N2] ETSI EN 300 392-2 V3.8.1 (2016-08)**, Anhang `en_30039202v030801p.pdf`: Abschnitt 14.5.5, S. 278–279, Eingang/Ausgang der SDS-Prozeduren; Abschnitt 29.3.2.2, S. 1186–1188, Quittungsebenen; Abbildungen 29.4/29.5 auf S. 1187, Ende-zu-Ende-Reports; 29.3.2.3/29.3.2.4 auf S. 1188, Gruppenservice-Auswahl und Store-and-forward. Keine dieser Stellen ist eine Spezifikation des konkreten RTW-Einsatzleitsystems.

**[N3] ETSI EN 300 812 V2.1.1 (2001-12)**, Anhang `en_300812v020101p.pdf`: Abschnitt 10.3.2, S. 56–57, `EF_ITSI`; Tabelle und Byteaufteilung visuell eingesehen. Die Datei beschreibt eine Schnittstellenedition, nicht einen ausgelesenen Kartensatz des Nutzers.

**[N4] ETSI EN 300 392-5 V2.7.1 (2020-04)**, Anhang `en_30039205v020701p.pdf`: Abschnitt 6.13, S. 89 ff., SDS direct commands; insbesondere `+CMGS` und der zugehörige Abschnitt zu `+CTSDSR`. Als Diagnoseverweis, nicht als durchgeführte PEI-Sitzung verwendet.

Die komplette Anhangsliste einschließlich Draft-Kennzeichnungen und begrenzter Prüftiefe steht in Abschnitt 12. Keine dieser Dokumenteditionen wird ohne gesonderte Aktualitätsprüfung als neueste Fassung ihrer Normenreihe bezeichnet.

### 14.2 Repository-Primärquellen, auf den Prüfcommit fixiert

**[R0]** [Geprüfter Ausgangscommit `2d4fbd627a1ecc9508f8bf52783bbc7fffb414c4`](https://github.com/JanHG98/netcore-tetra/commit/2d4fbd627a1ecc9508f8bf52783bbc7fffb414c4). Dieser Commit ist der Prüfanker, kein in diesem Chat historisch entwickelter Feature-Commit.

**[R1]** [`sds_bs.rs` am Prüfcommit](https://github.com/JanHG98/netcore-tetra/blob/2d4fbd627a1ecc9508f8bf52783bbc7fffb414c4/crates/tetra-entities/src/cmce/subentities/sds_bs.rs):

| Quellbereich | Nachgewiesener Inhalt |
|---|---|
| [Zeilen 124–190](https://github.com/JanHG98/netcore-tetra/blob/2d4fbd627a1ecc9508f8bf52783bbc7fffb414c4/crates/tetra-entities/src/cmce/subentities/sds_bs.rs#L124-L190) | PID-/Service- und Cache-/Drosselkonstanten |
| [Zeilen 619–680](https://github.com/JanHG98/netcore-tetra/blob/2d4fbd627a1ecc9508f8bf52783bbc7fffb414c4/crates/tetra-entities/src/cmce/subentities/sds_bs.rs#L619-L680) | Prüfung zentraler SDS-Verfügbarkeit und Edge-Telemetrie |
| [Zeilen 1240–1440](https://github.com/JanHG98/netcore-tetra/blob/2d4fbd627a1ecc9508f8bf52783bbc7fffb414c4/crates/tetra-entities/src/cmce/subentities/sds_bs.rs#L1240-L1440) | U-STATUS, Absenderkontext, Ausschluss von SDS-TL-Kurzreports, Reihenfolge lokaler Rückmeldung und zentraler Weitergabe |
| [Zeilen 2061–2300](https://github.com/JanHG98/netcore-tetra/blob/2d4fbd627a1ecc9508f8bf52783bbc7fffb414c4/crates/tetra-entities/src/cmce/subentities/sds_bs.rs#L2061-L2300) | Gruppenrefresh, Wiederanmeldung, Dashboard-Ereignisse, Fanout und Drossel |
| [Zeilen 2298–2540](https://github.com/JanHG98/netcore-tetra/blob/2d4fbd627a1ecc9508f8bf52783bbc7fffb414c4/crates/tetra-entities/src/cmce/subentities/sds_bs.rs#L2298-L2540) | HMD-Payload, individuelle Downlinks, HTTP-Gruppenauflösung und Parser |
| [Zeilen 2540–2800](https://github.com/JanHG98/netcore-tetra/blob/2d4fbd627a1ecc9508f8bf52783bbc7fffb414c4/crates/tetra-entities/src/cmce/subentities/sds_bs.rs#L2540-L2800) | Statuslabels, Runtime-Konfiguration, Environment und Konfigurationskandidaten |
| [Ab Zeile 2960](https://github.com/JanHG98/netcore-tetra/blob/2d4fbd627a1ecc9508f8bf52783bbc7fffb414c4/crates/tetra-entities/src/cmce/subentities/sds_bs.rs#L2960) | Sichtbare LIP-/Textdecodierungs-Unit-Tests, nicht ausgeführt |

**[R2]** [`cmce_bs.rs`, Zeilen 250–348](https://github.com/JanHG98/netcore-tetra/blob/2d4fbd627a1ecc9508f8bf52783bbc7fffb414c4/crates/tetra-entities/src/cmce/cmce_bs.rs#L250-L348), Blob `9220a9e49be5d4b40af91660659bff84a608ba3e`: Einbindung von Status-PDUs, SDS-Ticks und Subscriber-Update/Replays.

**[R3]** [`misc/ID-Server/netcore_directory_server.py`](https://github.com/JanHG98/netcore-tetra/blob/2d4fbd627a1ecc9508f8bf52783bbc7fffb414c4/misc/ID-Server/netcore_directory_server.py), Blob `7a5245d053ee5906762d7bcf74a0ee056706c417`: Datenmodell S. beziehungsweise Quellzeilen 54–120; `groups_for_issi` um 338–358; Handler `/api/status-group-members` um 1014–1032. Python-Implementierung statisch gelesen; produktive Instanz nicht identifiziert.

**[R4]** [`wiki/Device-Groups.md`](https://github.com/JanHG98/netcore-tetra/blob/2d4fbd627a1ecc9508f8bf52783bbc7fffb414c4/wiki/Device-Groups.md), Blob `e765584d432c0efd03acfef9c6a01798415cfb6f`: dokumentierte Gerätegruppen-/Status-Sync-Idee; ergänzt den Codebefund, ersetzt aber keine Tests.

### 14.3 Ergänzende öffentliche Primärquellen

**[W1]** [THW Ortsverband Waldshut-Tiengen: Operativ-taktische Adresse](https://www.thw-waldshut-tiengen.de/unser-thw-ortsverband/einheiten-fahrzeuge/ausstattung/digitalfunk/opta), abgerufen im Archivdurchlauf am 2026-10-04. Verwendet für die Erklärung der alphanumerischen OPTA als übermittelten Klartextdatensatz; keine Aussage über das konkrete Fahrzeug des Nutzers daraus abgeleitet.

**[W2]** [Regionalleitstelle Lausitz: Digitalfunk](https://www.leitstelle-lausitz.de/leitstelle/digitalfunk/), abgerufen im Archivdurchlauf am 2026-10-04, Abschnitt zu Statusrückmeldungen. Verwendet als klar getrenntes Betreiberbeispiel für Displayquittungen und dessen genannten PID-Wechsel. Kein Nachweis identischer Technik beim Nutzer oder allgemeiner Fanout-Automatik.

### 14.4 Bereits vorhandene Archive und Abgrenzung

Der vorhandene [Control-Room-/Status-Tableau-Archivbericht](2026-10-03_control-room-windows-ui-rbac-status-tableau-directory-api.md) behandelt die Darstellung und Directory-Anbindung in einem anderen Projektchat. Er ist ein thematischer Querverweis, keine Quelle für damalige Tests dieses Gesprächs. Die ursprünglichen Feature-Commits oder PRs des hier gelesenen Status-Sync-Codes wurden nicht historisch zugeordnet; entsprechende Nummern werden nicht erfunden.

---

**Fortsetzungsanker:** eigene individuelle Identitäten beibehalten; OPTA/Vehicle-ID als organisatorische Zuordnung behandeln; vorhandenen Directory-Status-Sync weiterverwenden und prüfen; lokale Displayantwort, technische Zustellquittung und fachlich bestätigten Leitstellenstatus getrennt führen.
