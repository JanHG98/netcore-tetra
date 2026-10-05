# NetCore-Tetra: WAP-Portal in XHTML/WML und Rust-Testfehler

## 1. Metadaten und Geltungsbereich

| Merkmal | Stand |
|---|---|
| Thema | „Hallo Welt“ in mobilen Formaten; vollständig verlinktes NetCore-WAP-Portal; Aktualisierung der Basisstation; SNDCP- und Mehrzellen-Testfehler |
| Ursprünglicher Chattitel | Nicht im zugänglichen Verlauf enthalten. Der Dokumenttitel ist eine beschreibende Archivbezeichnung. |
| Ursprünglicher Chatlink | Nicht verfügbar; kein Link rekonstruiert oder erfunden. |
| Erstellungsdatum | 2026-10-05, Zeitzone Europe/Berlin |
| Historischer Arbeitsbranch | `swmi`, vom Benutzer als `https://github.com/JanHG98/netcore-tetra/tree/swmi` genannt |
| Historischer Quellcommit | Aus der hochgeladenen ZIP nicht zuverlässig ermittelbar; sie enthält keinen verwertbaren Git-Checkout mit Commitnachweis. |
| Heutiger geprüfter Hauptbranch | `main` bei [`e5d825b33db2e1fce14bcb2cc23e73c241873a18`](https://github.com/JanHG98/netcore-tetra/commit/e5d825b33db2e1fce14bcb2cc23e73c241873a18) |
| Geprüfter Archivbranch vor dieser Änderung | `Archiving` bei [`7a98acb0c7520beebbac47d907c01516daa0146b`](https://github.com/JanHG98/netcore-tetra/commit/7a98acb0c7520beebbac47d907c01516daa0146b) |
| Root-Tree des geprüften Archivstands | `4db14277f7c6d311bfe7488508f5f05ec1756892` |
| Archivdatei | `Docs/archive/2026-10-05_wap-portal-xhtml-wml-und-rust-testfehler.md` |
| Änderungsumfang dieses Archivauftrags | Diese Datei und der Eintrag in `Docs/archive/README.md`; keine Programm-, Konfigurations- oder Roadmapänderungen außerhalb des Archivs |
| Archivierung des Chats | Erfolgt nach Prüfung durch den Benutzer selbst. |

**Wichtige zeitliche Trennung:** Die Erstellung dieses Archivs am 05.10.2026 ist nicht das belegte Datum sämtlicher ursprünglicher Chatbeiträge. Die wiedergefundenen Portaldateien tragen Metadaten vom 30.07.2026; auch die historische Portalvalidierung heißt `WAP_PORTAL_VALIDATION_2026-07-30.md`. Die einzelnen Chatbeiträge liegen ohne vollständige ursprüngliche Zeitstempel vor.

### 1.1 Verfügbare Quellen und Grenzen

Ausgewertet wurden:

- der in diesen Arbeitskontext übergebene Gesprächsverlauf, vom ersten Formatvergleich bis zur letzten Fehlermeldung und der angekündigten, noch nicht abgeschlossenen Testreparatur;
- die hochgeladene `netcore-tetra-swmi(1).zip`;
- die wiedergefundenen Originalartefakte `netcore-tetra-swmi-wap-portal.zip`, `netcore-tetra-swmi-wap-portal-fixed.zip`, `netcore-wap-portal-pages.zip`, `netcore-tetra-wap-portal.patch`, `fragment-borrow-fix.patch` und `netcore-wap-portal-SHA256SUMS.txt`;
- die für WAP, Fragmentierung, Testharness, MLE/CMCE und Deployment maßgeblichen Dateien des oben genannten Repository-Stands;
- die 25 verfügbaren ETSI-PDF-Anhänge als Inventar anhand Titel-/Versionsseiten und Seitenzahlen; gezielt zusätzlich SNDCP-/WAP-Fundstellen in EN 300 392-2.

Nicht verfügbar beziehungsweise nicht durchgeführt:

- ein vollständiger Export mit ursprünglichem Chattitel, Chat-ID, Chatlink und Zeitstempel jedes Turns;
- eine spätere Antwort mit fertig repariertem Mehrzellen-Testbestand oder eine Erfolgsmeldung des Benutzers nach der letzten Fehlermeldung;
- ein nachgewiesener historischer Push der WAP-Portalpakete; die damalige Abschlussmeldung sagte ausdrücklich, dass nichts direkt nach GitHub gepusht wurde;
- Zugang zur laufenden Basisstation, zur tatsächlich installierten Binary, zum IP-Gateway-LXC oder zu einem Funkgerät;
- ein Rust-Build in der Archivierungsumgebung: `cargo` und `rustc` waren nicht verfügbar;
- eine vollständige fachliche Auswertung aller Seiten der ETSI-Sammlung. Insbesondere die 4.100-seitige `ETSI.pdf` wurde nicht vollständig gelesen.

Andere Projektchats und Profilinformationen wurden nicht als zusätzliche Beschlüsse dieses Chats behandelt. Die Kontextsuche lieferte keinen ursprünglichen Titel/Chatlink und keine eindeutig diesem Verlauf zugehörige spätere Reparaturbestätigung.

### 1.2 Statusbegriffe

| Bezeichnung | Bedeutung in dieser Dokumentation |
|---|---|
| Idee | Im Gespräch erwogen, ohne endgültigen Umsetzungsauftrag oder Nachweis. |
| Beschlossen/geplant | Vom Benutzer angefordert oder als Vorgehen festgelegt; Umsetzung noch gesondert nachzuweisen. |
| Implementiert im Artefakt | In einer geprüften ZIP oder einem Patch tatsächlich enthalten. |
| Implementiert im Repository | Im geprüften Commit vorhanden und, soweit angegeben, in den betreffenden Modulen eingebunden. Eine bloß abgelegte Datei ist keine aktive Integration. |
| Getestet | Eine konkret bezeichnete Prüfung wurde ausgeführt; ihre Reichweite wird genannt. |
| Im Betrieb bestätigt | Durch Logs, Geräteverhalten oder eine eindeutige Betriebsmeldung belegt. Für den neuen Portalbetrieb liegt dieser Nachweis hier nicht vor. |

## 2. Ziel, Ausgangslage und Verlauf

### 2.1 Ursprüngliche Anfrage

Der Benutzer wollte zunächst „Hallo Welt“ als:

- WML beziehungsweise „WML script“,
- XHTML,
- OMA,
- WBXML,
- OMA DRM.

Die Antwort stellte Minimalbeispiele bereit und unterschied Auszeichnungssprache, Organisation, Binärkodierung und Rechteobjekt. Anschließend beschränkte der Benutzer den eigentlichen Entwicklungsauftrag ausdrücklich auf **XHTML und WML**:

> Viele unterschiedliche NetCore-Seiten, jede in beiden Formaten, innerhalb des jeweiligen Formats vollständig miteinander verlinkt.

Der Kontext war der WAP-Browser eines Funkgeräts im NetCore-Tetra-Projekt und der damals angegebene Branch `swmi`.

### 2.2 Ausgangspunkt des eingebauten Diensts

Die hochgeladene Projekt-ZIP enthielt einen lokalen WAP-over-SNDCP-Dienst in der Basisstationssoftware. Der bisherige Router akzeptierte `/`, `/status`, `/status.xhtml` und `/status.wml`; die historische Kurzbeschreibung erwähnte nur die drei Einstiege `/`, `/status.xhtml` und `/status.wml`.

Der interne Zielendpunkt war `10.0.0.1:9200` über UDP/WDP/WTP/WSP. Das ist der eingebaut beantwortete Paketdatenendpunkt und kein Nachweis eines normalen TCP-HTTP-Servers auf dem IP-Gateway-LXC.

### 2.3 Wesentliche Gesprächsschritte

| Reihenfolge | Inhalt | Belegbarer Abschluss |
|---|---|---|
| 1 | Formatvergleich und „Hallo Welt“-Beispiele | Beispiele im Chat vorhanden; keine Geräteprüfung belegt. |
| 2 | Beschränkung auf XHTML und WML | Ausdrückliche Benutzerentscheidung. |
| 3 | Anforderung vieler vollständig verlinkter NetCore-Seiten | Verbindliche Portal-Anforderung. |
| 4 | Erstellung eines kompakten, eingebauten Portals plus statischer Referenzseiten | In den wiedergefundenen ZIPs/Patches nachprüfbar. |
| 5 | Frage nach Neuinstallation des IP-Gateways | Antwort: Basisstationssoftware bauen/aktualisieren; kein Neuaufsetzen des IP-Gateway-LXC für dieses lokale Portal. |
| 6 | Erster Testversuch scheitert mit E0502 in `fragment.rs` | Benutzer-Compilerlog vorhanden. |
| 7 | Längenwert vor dem veränderlichen Slicezugriff zwischenspeichern | Im korrigierten Paket und im heutigen Repository nachprüfbar; kein erfolgreicher Rust-Testlauf belegt. |
| 8 | Weitere Compilerfehler in Mehrzellen-/Restore-Integrationstests | Benutzerlog enthält E0599/E0308 und eine Warnung zu unbenutzten Reexports. |
| 9 | Ankündigung eines konsistenten Abgleichs zwischen Tests und Implementierungen | Letzter sichtbarer Entwicklungsschritt; eine fertige Reparatur fehlt. |
| 10 | Technische Archivierung dieses Verlaufs | Dieser Auftrag; ausschließlich Dokumentation unter `Docs/archive/`. |

## 3. Endgültige Anforderungen und Entscheidungen

### 3.1 Beschlossen

1. **Zwei Formate:** XHTML und WML.
2. **Thematische Vielfalt:** NetCore-Betrieb, Teilnehmer, Gruppen, Rufe, SDS, Control Room, Diagnose und Projektmodule.
3. **Gleiche Inhalte in beiden Formaten.**
4. **Formatreine Navigation:** XHTML verlinkt auf XHTML, WML auf WML.
5. **Vollständige Erreichbarkeit ab der Startseite.**
6. **Kompakte Seiten für den älteren Motorola/Openwave-Browserpfad.**
7. **Bestehende Statusadressen und Konfigurationsschalter kompatibel halten.**
8. **Portal im eingebauten WAP-Pfad der Basisstation ausliefern.**
9. **Für dieses lokale Portal die Basisstation aktualisieren; den IP-Gateway-LXC nicht neu installieren.**

Die konkrete Aufteilung auf 19 Seitenthemen, Kurzpfade und die Navigation mit `P/N/H` wurde im gelieferten Artefakt umgesetzt. Eine ausdrückliche spätere Benutzeränderung dieser Portalstruktur ist nicht sichtbar.

### 3.2 Nicht Bestandteil der endgültigen Portalfassung

OMA Push, generisches WBXML und OMA DRM wurden nach dem Formatvergleich nicht weiter beauftragt. Ein echter WMLScript-Interpreter oder WMLScript-Programm wurde nicht implementiert. Das erste Beispiel war **WML-Markup**, obwohl die Anfrage „WML script“ sagte; beide Begriffe sind technisch zu unterscheiden.

Die Seiten zu Control Room, Media Library, Recorder und TTS stellen überwiegend kurze Informationsansichten dar. Der Chat belegt keinen Auftrag, darüber vollständige Administrationsoberflächen, Audioübertragung, Recordersteuerung oder TTS-Ausführung auf dem Funkgerät zu realisieren.

## 4. Formatvergleich und wiederverwendbare Minimalbeispiele

### 4.1 WML 1.1

Historisches Beispiel für `hello.wml`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE wml PUBLIC "-//WAPFORUM//DTD WML 1.1//EN"
  "http://www.wapforum.org/DTD/wml_1.1.xml">
<wml>
  <card id="hello" title="Hallo">
    <p>Hallo Welt</p>
  </card>
</wml>
```

MIME-Typ: `text/vnd.wap.wml; charset=UTF-8`. Ein WML-Deck enthält Cards. Dieses Beispiel verwendet keine WMLScript-Datei.

### 4.2 XHTML Basic 1.1

Historisches Beispiel für `hello.xhtml`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML Basic 1.1//EN"
  "http://www.w3.org/TR/xhtml-basic/xhtml-basic11.dtd">
<html xmlns="http://www.w3.org/1999/xhtml">
  <head><title>Hallo</title></head>
  <body><p>Hallo Welt</p></body>
</html>
```

Im allgemeinen Vergleich wurde `application/xhtml+xml; charset=UTF-8` genannt. Für den konkreten mobilen Portalpfad wurde `application/vnd.wap.xhtml+xml; charset=UTF-8` verwendet.

Die vollständigen statischen Referenzdateien sind von den extrem kurzen WSP-Laufzeitdokumenten zu unterscheiden. Letztere verwenden kompakte Wrapper und verzichten aus Platzgründen auf die lange XML-/DOCTYPE-Deklaration der Referenzdateien.

### 4.3 OMA, WBXML und DRM: historische Nebeninformationen

| Begriff | Historisches Beispiel und technische Einordnung | Endgültiger Status |
|---|---|---|
| OMA | Organisation, keine eigene allgemeine Seitensprache. Als konkretes Beispiel wurde WAP Service Indication mit „Hallo Welt“ und Link `http://10.0.1.142/hello.wml` gezeigt. `10.0.1.142` war eine Beispieladresse, kein bestätigter Portalserver. MIME: `text/vnd.wap.si`, kompiliert `application/vnd.wap.sic`. | Nach Formatentscheidung nicht weiter verfolgt. |
| WBXML | Binäre XML-Kodierung. Generisches Beispiel `<hello>Hallo Welt</hello>` mit LITERAL-Tag und Stringtabelle, kein WML-spezifisches kompiliertes Deck. MIME: `application/vnd.wap.wbxml`. | Kein Bestandteil des Portalauftrags und keine bestätigte Terminalkompatibilität. |
| OMA DRM | Rights Object mit ODRL-EX/-DD-Namensräumen, Version 1.0, UID `cid:hello-world` und Berechtigung `display`. Der eigentliche Inhalt bleibt separat. MIME: `application/vnd.oma.drm.rights+xml`. | Kein DRM-Plugin oder Rechteverwaltungssystem implementiert. |

Der historische generische WBXML-Testvektor lautete:

```text
03016A0668656C6C6F0044000348616C6C6F2057656C740001
```

Aufschlüsselung: Versionbyte `03` für WBXML 1.3, Public Identifier `01`, UTF-8-Kennung `6A`, sechs Byte Stringtabelle `hello\0`, LITERAL-Tag mit Inhalt `44 00`, Inline-String `03`, „Hallo Welt“ mit Nullabschluss und END `01`.

Die im Chat genannte Dateierzeugung war nur vorgeschlagen:

```bash
printf '%s' '03016A0668656C6C6F0044000348616C6C6F2057656C740001'   | xxd -r -p > hello.wbxml
```

Für ein echtes WML-/SI-Dokument wären passende dokumenttypspezifische Token/Codepages und ein geeigneter Client erforderlich. Der Vektor ist kein Nachweis einer NetCore-WBXML-Integration.

## 5. Historisches Portal: Umfang, Pfade und Darstellung

### 5.1 Seitenmatrix

Im historischen Paket waren 19 Themen mit jeweils einer XHTML- und einer WML-Referenzdatei enthalten. Die beiden mit „heutige Ergänzung“ bezeichneten Zeilen gehören erst zum zusätzlich geprüften aktuellen Stand.

| Thema | XHTML-Alias | WML-Alias | XHTML-Kurzpfad | WML-Kurzpfad | Herkunft |
|---|---|---|---|---|---|
| Start | `/index.xhtml` | `/index.wml` | `/x` | `/w` | Historischer Chat |
| Status | `/status.xhtml` | `/status.wml` | `/x/st` | `/w/st` | Historischer Chat |
| Teilnehmer | `/subscribers.xhtml` | `/subscribers.wml` | `/x/ms` | `/w/ms` | Historischer Chat |
| Gruppen | `/groups.xhtml` | `/groups.wml` | `/x/gr` | `/w/gr` | Historischer Chat |
| Rufe | `/calls.xhtml` | `/calls.wml` | `/x/ca` | `/w/ca` | Historischer Chat |
| SDS | `/sds.xhtml` | `/sds.wml` | `/x/sd` | `/w/sd` | Historischer Chat |
| Control Room | `/control-room.xhtml` | `/control-room.wml` | `/x/cr` | `/w/cr` | Historischer Chat |
| Health | `/health.xhtml` | `/health.wml` | `/x/he` | `/w/he` | Historischer Chat |
| Funkzelle | `/radio.xhtml` | `/radio.wml` | `/x/ra` | `/w/ra` | Historischer Chat |
| Paketdaten | `/packet-data.xhtml` | `/packet-data.wml` | `/x/pd` | `/w/pd` | Historischer Chat |
| IP Gateway | `/gateway.xhtml` | `/gateway.wml` | `/x/gw` | `/w/gw` | Historischer Chat |
| Dienste | `/services.xhtml` | `/services.wml` | `/x/sv` | `/w/sv` | Historischer Chat |
| Diagnose | `/diagnostics.xhtml` | `/diagnostics.wml` | `/x/dg` | `/w/dg` | Historischer Chat |
| Media Library | `/media-library.xhtml` | `/media-library.wml` | `/x/me` | `/w/me` | Historischer Chat |
| Recorder | `/recorder.xhtml` | `/recorder.wml` | `/x/re` | `/w/re` | Historischer Chat |
| TTS Piper | `/tts.xhtml` | `/tts.wml` | `/x/tt` | `/w/tt` | Historischer Chat |
| Tests | `/tests.xhtml` | `/tests.wml` | `/x/te` | `/w/te` | Historischer Chat |
| Hilfe | `/help.xhtml` | `/help.wml` | `/x/hl` | `/w/hl` | Historischer Chat |
| Projektinfo | `/about.xhtml` | `/about.wml` | `/x/ab` | `/w/ab` | Historischer Chat |
| Aufgaben | `/tasks.xhtml` | `/tasks.wml` | `/x/tk` | `/w/tk` | Heutige Ergänzung, Phase 9 |
| Formulare | `/task-form.xhtml` | `/task-form.wml` | `/x/fm` | `/w/fm` | Heutige Ergänzung, Phase 9 |

Die aktuelle Existenz der Dateien/Parserzuordnungen in dieser Tabelle ist **nicht** mit ihrer Erreichbarkeit am aktiven eingebauten WSP-Handler gleichzusetzen. Abschnitt 10 beschreibt die heute fehlende Einbindung.

### 5.2 Navigation und Aliase

Historisch implementiert im Portalpaket:

- `P`: vorherige Seite;
- `N`: nächste Seite;
- `H`: Startseite;
- Startseite mit Einstiegen in Betrieb, Netz und Info;
- formatabhängige Kurzpfade unter `/x` beziehungsweise `/w`;
- lesbare Endungen `.xhtml` und `.wml`;
- `/`, `/index` und `/index.xhtml` als XHTML-Start;
- `/status` als XHTML-Statusalias;
- `/status.xhtml?s=1` und `/status.wml?s=1` als Health-Ansicht.

Die Kurzseiten sind keine Desktop-Webseiten. Kurze Beschriftungen, begrenzte Strings und abgestufte Darstellungsvarianten erhalten kleine Nutzlasten.

### 5.3 Bytebudgets

| Darstellung | Historischer dokumentierter Referenzwert | Grenzwert |
|---|---:|---:|
| XHTML-Status | 102 Byte | 104 Byte |
| Größte übrige XHTML-Seite beim Referenz-Snapshot | 126 Byte | 144 Byte |
| Größte WML-Seite beim Referenz-Snapshot | 131 Byte | 144 Byte |

Die Konstanten heißen `XHTML_INDEX_MAX_BYTES = 104`, `XHTML_PAGE_MAX_BYTES = 144` und `WML_PAGE_MAX_BYTES = 144`. Trotz des Namens `XHTML_INDEX_MAX_BYTES` wendet `render_portal_page` das 104-Byte-Limit auf **Status** an; die Startseite fällt unter das allgemeine XHTML-Limit.

Die Grenzwerte sind projekt-/terminalbezogene Kompatibilitätsvorgaben, keine allgemeinen XHTML-/WML- oder ETSI-Höchstgrößen. Die angegebenen Maxima wurden historisch mit einem modellierten Referenz-Snapshot dokumentiert. In dieser Archivierungsarbeit wurden sie nicht durch einen kompilierten Rust-Renderer erneut gemessen.

Der Renderer probiert:

1. Inhalt mit vollständiger verfügbarer Navigation;
2. Inhalt mit reduzierter Navigation ohne vorherigen Link;
3. kurzen Seitentitel mit Startlink.

Das begrenzt lange Live-Werte. `first_fitting` liefert, wenn kein Kandidat passt, den kürzesten Kandidaten; es ist keine mathematische Garantie für alle denkbaren Zählerwerte. Die Tests prüfen insbesondere den Referenz-Snapshot. UTF-8-/XML-Escaping und große Zähler sollten bei der späteren Abnahme mitgeprüft werden.

## 6. Architektur und technische Abhängigkeiten

### 6.1 Lokaler Paketdatenpfad

Das Portal gehört zur Basisstations-Binary `bluestation-bs`, zur Crate `tetra-entities` und zum Bereich `sndcp`.

Der relevante Weg lautet:

1. Das Funkgerät benutzt seinen WAP-Browser.
2. TETRA-Paketdaten werden über den LTPD-/SNDCP-Pfad und einen PDP-Kontext verarbeitet.
3. `SndcpBs::handle_user_data` prüft unter anderem Kontext, MTU, Quelladresse, Kompression, Verfügbarkeit und READY-Zustand.
4. IPv4-Fragmente werden gegebenenfalls zusammengesetzt.
5. Ein UDP-Paket an den konfigurierten lokalen WAP-Endpunkt wird an `wap_ip::build_response_npdu` übergeben.
6. Dort werden WTP/WSP beziehungsweise ein einfacher UDP-GET-Testpfad verarbeitet.
7. Die Antwort wird als IPv4/UDP-N-PDU aufgebaut, bei Bedarf auf die ausgehandelte MTU fragmentiert und zurück über SNDCP gesendet.

Im Portalpaket folgt zwischen WSP-Anfrage und Antwort zusätzlich der Portalparser/-renderer. Im heutigen Repository fehlt diese Einbindung, siehe Abschnitt 10.

Das weiterführende Paketdatenrouting zum Netzwerk/IP-Gateway ist ein eigener Weg. Die lokale WAP-Antwort wird von der Basisstation erzeugt; die bloße Erweiterung dieses lokalen Portals verlangt keine Neuinstallation des IP-Gateway-LXC.

### 6.2 Komponenten

| Komponente/Datei | Aufgabe und Prüfstatus |
|---|---|
| `bins/bluestation-bs/` | Basisstations-Binary; kein Build in dieser Archivierungsumgebung. |
| `crates/tetra-entities/src/sndcp/sndcp_bs.rs` | PDP-/SNDCP-Verarbeitung, Auswahl des lokalen WAP-Endpunkts, Snapshot und Antwortpfad; aktueller Quelltext geprüft. |
| `crates/tetra-entities/src/sndcp/wap_ip.rs` | IPv4/UDP, WTP/WSP, URI-/Policy-Behandlung und Antwortaufbau; historische und aktuelle Varianten unterscheiden sich wesentlich. |
| `crates/tetra-entities/src/sndcp/wap_portal.rs` | `WapMarkup`, `WapPage`, `WapPortalRoute`, `ALL_PAGES`, Parser, Renderer und Portaltests. Im Paket eingebunden; im heutigen Repository als Datei vorhanden, aber nicht im `sndcp/mod.rs` deklariert. |
| `crates/tetra-entities/src/sndcp/wap_status.rs` | `WapStatusSnapshot`, escaping und alte Statusrenderer; im heutigen aktiven Pfad verwendet. |
| `crates/tetra-entities/src/sndcp/fragment.rs` | IPv4-Fragmentierung/Reassembly und Regressionstests; historische E0502-Stelle und heutiger Fix geprüft. |
| `crates/tetra-config/src/bluestation/sec_cell.rs` | WAP-/Paketdatenkonfiguration, DTOs und Defaults. |
| `contrib/wap-portal/xhtml/`, `contrib/wap-portal/wml/` | Statische, vollständige Referenzseiten für einen geeigneten HTTP/WAP-Server und als lesbare Vorlagen. |
| `contrib/wap-portal/validate.py` | XML-Wohlgeformtheit, Dateizahl, Formatlinks und Erreichbarkeit. |
| `system-backend/ip-gateway/` | Separater Backend-/Paketdaten-Routingdienst; für den lokalen Portalpatch nicht neu installiert. |
| `install/update-basisstation.sh` | Baut und tauscht die tatsächlich benutzte Basisstations-Binary; enthält weitere Media-/TTS-Migrationslogik. |
| `MleBs`, `CmceBs`, `TwoCellHarness` | Für Mobilitäts-/Restore-Integrationstests relevant; nicht mit dem Seitendesign gleichzusetzen. |

Workspace-Version im geprüften Stand: `1.3.0`; Rust-Edition: `2024`. Daraus wurde keine tatsächlich installierte Compiler-Version abgeleitet.

### 6.3 Statusdaten und Informationsseiten

`WapStatusSnapshot` enthält:

- `title`, `state`, `version`;
- `registered_ms`, `attached_groups`, `active_calls`, `queued_sds`;
- `uptime_secs`, `last_activity`, `health`.

Die aktuelle Snapshot-Erzeugung nutzt lokale Shared-State-/Gatewaywerte. Der sichtbare Zustand wird unter anderem als `ONLINE` oder `STANDALONE` gesetzt. Die Health-Zeichenfolge kann PDP-, PDCH-, Gateway-, UL/DL- und Queuewerte enthalten.

Dynamisch darstellbar sind insbesondere Status, Teilnehmer-/Gruppen-/Ruf-/SDS-Zähler, Health/Uptime, Diagnose/letzte Aktivität und Version. Andere Portalthemen zeigen überwiegend kurze feste Beschreibungen. Ein Link auf „Media Library“ beweist daher keinen abrufbaren Medienkatalog oder Audio-Streamingbetrieb.

### 6.4 Protokolle, Adressen und Parameter

| Parameter | Wert beziehungsweise Einordnung |
|---|---|
| Lokale WAP-Adresse | `10.0.0.1` |
| WAP-Port | `9200/UDP` |
| Transportfolge | TETRA-Paketdaten → SNDCP/IPv4 → UDP/WDP → WTP/WSP |
| XHTML-MIME | `application/vnd.wap.xhtml+xml`, im statischen Portal mit UTF-8 |
| WML-MIME | `text/vnd.wap.wml`, im statischen Portal mit UTF-8 |
| WSP-Content-Type-Token | XHTML `0xc5`, WML `0x88` |
| WSP-PDU-Werte im geprüften Code | Connect `0x01`, Reply `0x04`, Resume `0x09`, GET `0x40` |
| WTP-Transaktionskennung | Antwortflag `0x8000`, Wertmaske `0x7fff` |
| WSP-SDU-Konstante | `545` Byte; nicht mit dem 104-/144-Byte-Seitenbudget gleichsetzen |
| IPv4-MTU-Referenz | Historische Kurzseite „MTU576“; reale Weiterverarbeitung benutzt die ausgehandelte Kontext-MTU |
| Default-TTL | `32` |
| Default-Pool | Präfix `10.0.0`, Hosts `2…254` |
| Maximaler Request-Payload | Default `1024` Byte |
| Default-Kontextgrenzen | `4` pro ISSI, `64` insgesamt |
| Strenge Quelladressprüfung | Default `true` |
| READY-Annahme nach Datensendung | Default `false` |
| Default SNDCP-Timer-/QoS-Codes | `pdu_priority_max=4`, `ready_timer_code=8`, `standby_timer_code=4`, `response_wait_timer_code=7`, `mtu_code=2`, `network_default_data_priority=4` |

Diese Defaults stammen aus `CfgWapIp::default`. Sie sind **keine ausgelesenen Betriebswerte** von Jans Basisstation. `wap_ip.enabled` ist per Default `false` und muss für den lokalen Dienst entsprechend konfiguriert werden.

### 6.5 Normenbezug

Die bereitgestellte EN 300 392-2 V3.8.1 enthält:

- SNDCP-Überblick in Abschnitt 28.1, gedruckte Seite 1023;
- U-CALL RESTORE in Abschnitt 14.7.2.2, gedruckte Seite 294;
- WAP im SDS-Kontext in Abschnitt 29.5.8, gedruckte Seite 1220.

Abschnitt 28.1 behandelt PDP-Kontextverwaltung und PDP-Datentransfer. Abschnitt 29.5.8 ist ein anderer Normkontext und kein Nachweis, dass das hier besprochene lokale IPv4/WSP-Portal über SDS transportiert wird. Die tatsächliche Portalzuordnung wurde am Projektcode geprüft. Eine allgemeine ETSI-/OMA-Konformitätszertifizierung wurde weder im Chat noch bei der Archivierung nachgewiesen.

## 7. Konfiguration und historisch vorgeschlagene Inbetriebnahme

### 7.1 Konfigurationsbeispiel

Im Chat wurde für `/etc/netcore/config.toml` vorgeschlagen:

```toml
[cell_info]
sndcp_service = true
advanced_link = true

[cell_info.wap_ip]
enabled = true
address = "10.0.0.1"
port = 9200
accept_root_path = true
accept_status_path = true
accept_status_wml_path = true
```

**Status:** vorgeschlagen; eine erfolgreich geladene Konfiguration dieses Benutzers ist nicht belegt.

Im historischen Portalpaket bedeuten die Schalter:

| Schalter | Portalpaket |
|---|---|
| `accept_root_path` | XHTML-Startseite |
| `accept_status_path` | Weitere XHTML-Portalseiten |
| `accept_status_wml_path` | Alle WML-Portalseiten einschließlich Start |

Im heutigen aktiven `wap_ip.rs` gelten dagegen weiterhin die alten, engeren Statuspfadprüfungen. Die Schalter allein aktivieren die neuen Portalseiten dort nicht.

### 7.2 Paket entpacken und Basisstation aktualisieren

Die ursprüngliche Anleitung verwendete einen separaten Entpackordner unter `/home/jan` und das Projekttoplevel `netcore-tetra-swmi`. Im Verlauf wurde der Entpackordner zuvor mit `rm -rf` neu erstellt. Das war lediglich eine vorgeschlagene Aufräumaktion und ist keine dauerhaft notwendige Updatevoraussetzung.

Für eine Fortsetzung ist ein neuer, leerer Entpackordner zweckmäßig; der folgende Ablauf ist ebenfalls **nur vorgeschlagen und hier nicht auf der Basisstation ausgeführt**:

```bash
cd /home/jan
mkdir netcore-tetra-swmi-wap-portal-review
unzip netcore-tetra-swmi-wap-portal-fixed.zip   -d netcore-tetra-swmi-wap-portal-review
cd netcore-tetra-swmi-wap-portal-review/netcore-tetra-swmi
python3 contrib/wap-portal/validate.py
```

Vor einem tatsächlichen Update müssen die in Abschnitt 8 und 10 beschriebenen Test-/Integrationsprobleme geklärt und der Quellstand festgehalten werden. Danach war im Chat vorgesehen:

```bash
sudo bash install/update-basisstation.sh
```

Die ZIP ist nicht automatisch ein geprüfter aktueller Checkout. Sie sollte insbesondere wegen ihres alten Projektstands und der vier ausgelassenen Wiki-Dateien nicht ungeprüft einen heutigen Checkout ersetzen.

### 7.3 Verhalten des vorhandenen Updateskripts

Am historischen Paket und am heutigen Quelltext nachvollziehbar:

1. Ermittelt Repositorypfad, Buildbenutzer und Cargo.
2. Prüft die Unterstützung der Top-Level-`[media_library]`-Konfiguration.
3. Führt zwei `tetra-config`-Parser-Regressionstests aus.
4. Baut `bluestation-bs` mit `cargo build --release -p bluestation-bs`, optional mit `CARGO_FEATURES`.
5. Ermittelt die systemd-Unit und den tatsächlich ausgeführten Binarypfad.
6. Sichert die alte Binary unter `/var/backups/netcore-tetra/`.
7. Stoppt die Unit, installiert die neue Binary und startet sie.
8. Prüft nach sechs Sekunden den aktiven Dienstzustand und bestimmte Media-Library-Konfigurationsfehler.
9. Rollt bei den dafür vorgesehenen Fehlerpfaden auf die gesicherte Binary zurück.

Zusätzliche, in der kurzen ursprünglichen Antwort nicht hervorgehobene Wirkungen:

- standardmäßig `MIGRATE_LOCAL_TTS_CONFIG=1`: Aufruf von `install/remove-local-tts-config.py` für die Konfiguration;
- standardmäßig `DISABLE_LOCAL_PIPER=1`: gegebenenfalls `netcore-piper.service` deaktivieren/stoppen;
- Dateieigentümer/-modus der Konfiguration werden berücksichtigt.

Das ist daher kein ausschließlich auf WAP beschränktes Kopierskript. Diese Wirkungen vor einem Update am konkreten System prüfen. Der konfigurierte `service_name` hat Vorrang; ansonsten werden nacheinander `tetra.service`, `bluestation.service`, `tetra-bluestation.service` und `bluestation-bs.service` gesucht. Eine vorhandene Unit ist nicht automatisch die gewünschte aktive Unit; bei mehreren Installationen `UNIT` und `BINARY_PATH` ausdrücklich verifizieren.

**Archivierungsprüfung:** `bash -n install/update-basisstation.sh` war erfolgreich. Das Skript wurde weder installiert noch auf einem laufenden Dienst ausgeführt; Rollback und Neustart wurden hier nicht praktisch getestet.

### 7.4 Historische Kontrollbefehle und Browserziele

Im Chat vorgeschlagen:

```bash
systemctl status bluestation.service --no-pager
systemctl status tetra.service --no-pager
journalctl -u bluestation.service -n 150 --no-pager
journalctl -u bluestation.service -f
```

Die richtige Unit ist installationsabhängig. Keiner dieser Befehle ist als erfolgreich auf Jans System ausgeführt belegt.

Historische Browserziele:

- `http://10.0.0.1/`
- `http://10.0.0.1/index.xhtml`
- `http://10.0.0.1/index.wml`
- `http://10.0.0.1/status.xhtml`
- `http://10.0.0.1/status.wml`

Die URL-Darstellung im WAP-Browser und der auf `9200/UDP` angesprochene WSP-Endpunkt sind unterschiedliche Ebenen. Diese Adressen sind kein Beleg eines TCP-Servers auf Port 80 und dürfen nicht allein mit einem gewöhnlichen Browser-/curl-GET als On-Air-WSP-Nachweis bewertet werden.

## 8. Fehler, Diagnose und Reparaturstand

### 8.1 E0502 in IPv4-Fragmentierungstest

Benutzerlog:

```text
error[E0502]: cannot borrow 'packet' as immutable because it is also borrowed as mutable
crates/tetra-entities/src/sndcp/fragment.rs:383:40
packet[2..4].copy_from_slice(&(packet.len() as u16).to_be_bytes());
```

Ursache: Während der veränderliche Slicezugriff `packet[2..4]` verwendet wird, leiht `packet.len()` denselben Vektor im selben Ausdruck unveränderlich aus.

Die betroffene Funktion ist der Test `only_copied_ipv4_options_are_present_after_first_fragment` unter `#[cfg(test)]`. Er fügt IPv4-Optionen ein, setzt IHL auf `0x47`, aktualisiert Gesamtlänge/Checksumme und prüft das Verhalten kopierter Optionen bei Fragmentierung/Reassembly. Es war an dieser Stelle kein nachgewiesener Laufzeitfehler des Fragmentierers.

Historisch gelieferter Fix:

```rust
packet.splice(20..20, [0x82, 4, 0xaa, 0xbb, 0x02, 4, 0xcc, 0xdd]);
packet[0] = 0x47;
let total_len = packet.len() as u16;
packet[2..4].copy_from_slice(&total_len.to_be_bytes());
packet[10..12].copy_from_slice(&0u16.to_be_bytes());
let checksum = internet_checksum(&packet[..28]);
packet[10..12].copy_from_slice(&checksum.to_be_bytes());
```

Der damalige automatische Python-Ersatz suchte genau die eingerückte alte Zeile und ersetzte sie einmal; bei fehlendem Treffer sollte er abbrechen. Er wurde als Reparaturmöglichkeit unter `~/netcore-tetra` vorgeschlagen, nicht als nachweislich dort ausgeführt protokolliert.

**Belegt:**

- Die korrigierte ZIP enthält diesen Fix.
- Original-Portal-ZIP und „fixed“-ZIP unterscheiden sich inhaltlich ausschließlich in `fragment.rs`.
- Im heutigen Repository steht derselbe Lösungsansatz mit dem Variablennamen `packet_len`.

**Nicht belegt:** ein erfolgreich kompilierter Testlauf nach diesem Fix. Die nächste Benutzerantwort enthält weitere Testkompilierungsfehler; sie ist kein Nachweis für einen komplett grünen Build.

### 8.2 E0599/E0308 in den Mehrzellen-/Restore-Tests

Nach dem E0502-Fix meldete der Benutzer:

| Fehler | Stelle im Benutzerlog | Erwartung des Tests | Befund |
|---|---|---|---|
| E0599 | `tests/common/two_cell.rs:312` | `MleBs::cell_change_snapshot()` | In der geprüften MLE-BS-Implementierung nicht bereitgestellt. |
| E0599 | `tests/common/two_cell.rs:327` | `MleBs::ltpd_snapshot()` | In der geprüften MLE-BS-Implementierung nicht bereitgestellt. Ein gleichnamiger SNDCP-Snapshot ersetzt diese MLE-API nicht automatisch. |
| E0308 | `tests/test_two_cell_call_restore.rs:43` | Zuweisung `location_area: u8` | Konfigurationsfeld ist `u16`; Testhelper verwendet weiterhin `u8`. |
| E0599 | `tests/test_two_cell_call_restore.rs:72` | `CmceBs::export_call_restore_context()` | In der geprüften CMCE-BS-Implementierung nicht bereitgestellt. |
| E0599 | Zahlreiche Zeilen im selben Test | `CmceBs::install_call_restore_context()` | Entsprechende CMCE-BS-Anbindung fehlt im geprüften Stand. |
| E0599 | Unter anderem Zeilen 228, 298, 426, 494 und 559 | `CmceBs::call_restore_snapshot()` | Erwartete Test-/Laufzeitsicht fehlt. |
| E0599 | Zeile 545 | `UCallRestore::clone()` | Typ ist mit `#[derive(Debug)]` deklariert; keine Clone-Implementierung in der geprüften Datei. |
| Warnung | `tests/common/mod.rs:21` | Reexport `TestCell`, `TwoCellHarness` | Unbenutzte Imports; Warnung, kein Ersatz für die eigentlichen Compilerfehler. |

Gemeldete betroffene Testtargets: `test_two_cell_foundation`, `test_mm_bs` und `test_two_cell_call_restore`. Für den letztgenannten Target wurden 19 vorherige Fehler gemeldet. Die vollständige Cargo-Ausgabe vor und nach diesem Ausschnitt liegt nicht vor.

Die damalige Diagnose „Versionsmischmasch im Testbestand“ beschreibt die beobachtete Inkonsistenz zwischen Testanforderungen und Implementierung. Die konkrete Entstehungsursache, ein bestimmter fehlerhafter Merge oder ein verursachender Commit wurde nicht nachgewiesen.

Die Dateien `two_cell.rs`, `test_two_cell_call_restore.rs`, `mle_bs.rs`, `cmce_bs.rs` und `u_call_restore.rs` sind zwischen der ursprünglichen hochgeladenen ZIP und der korrigierten Portal-ZIP unverändert. Das Portalpaket repariert diese API-Probleme folglich nicht.

### 8.3 Warum ein SNDCP-Filter auch Mehrzellentests blockieren kann

Die historischen Befehle

```bash
cargo test -p tetra-entities sndcp::wap_portal
cargo test -p tetra-entities sndcp::wap_ip
cargo test -p tetra-entities sndcp
```

begrenzen über den Namen, welche Tests laufen sollen. Ohne `--lib` kann Cargo zuvor trotzdem die Integrationstesttargets kompilieren. Außerdem wird `common::two_cell` im gemeinsamen Testmodul eingebunden; dadurch können fehlende APIs weitere Integrationstests blockieren, selbst wenn ein konkreter Test den Harness nicht nutzt.

Für einen abgegrenzten Bibliothekstest wurde bei der Archivierung als **künftiger Diagnosebefehl** festgehalten:

```bash
cargo test -p tetra-entities --lib sndcp::wap_ip
cargo test -p tetra-entities --lib sndcp::fragment
```

Nach Wiederherstellung der Portal-Moduldeklaration:

```bash
cargo test -p tetra-entities --lib sndcp::wap_portal
```

Wichtig: Im heutigen Stand kann der letzte Filter wegen der fehlenden Moduldeklaration null Portaltests finden. „0 tests“ ist keine erfolgreiche Portalabnahme. Die Zahl tatsächlich ausgeführter Tests muss im Ergebnis festgehalten werden.

### 8.4 Noch nicht funktionierend bestätigte Reparaturkandidaten

Für eine spätere Implementierungsarbeit zu prüfen:

- MLE-/CMCE-Laufzeitmodule und Testharness konsistent anbinden, einschließlich Kontextübergabe, Snapshot und Zustand; keine leeren Scheinmethoden nur zum Kompilieren hinzufügen.
- Den LA-Typ im Testhelper auf `u16` anpassen oder eine begründete verlustfreie Konvertierung verwenden. `u16::from(location_area)` löst nur die aktuelle Zuweisung, deckt aber im Helper weiterhin keine größeren LA-Werte ab.
- Bei wiederholtem U-CALL RESTORE entweder ein korrekt clonbares PDU-Modell mit geprüften Feldabhängigkeiten bereitstellen oder den Test-PDU erneut konstruieren. Ein unüberprüftes `derive(Clone)` ist hier kein bereits bestätigter Fix.
- Die Warnung zu Reexports bei Bedarf gezielt behandeln; die Integrationstestfehler bleiben unabhängig davon zu lösen.

Der letzte sichtbare Assistententurn kündigte einen konsistenten Abgleich an. Es folgt keine gelieferte Reparatur, kein weiterer Patch und kein grüner Testbericht. Dieser Arbeitsstand bleibt ausdrücklich **offen**.

## 9. Historische Artefakte und erneute Prüfung

### 9.1 Dateipakete

| Artefakt | Zweck | Prüfung bei Archivierung |
|---|---|---|
| `netcore-tetra-swmi(1).zip` | Benutzerseitiger Ausgangssnapshot | Lokale Dateien geprüft; historische E0502-Zeile und Testinkonsistenzen enthalten. |
| `netcore-tetra-swmi-wap-portal.zip` | Projekt mit Portal, vor E0502-Fix | ZIP-Integrität erfolgreich; 1.710 Einträge; Vergleich mit fixed-Version. |
| `netcore-tetra-swmi-wap-portal-fixed.zip` | Projekt mit zusätzlichem Borrow-Checker-Fix | ZIP-Integrität erfolgreich; 1.710 Einträge; nur `fragment.rs` gegenüber erster Portal-ZIP inhaltlich geändert. |
| `netcore-wap-portal-pages.zip` | Statische Seiten und Dokumentation | ZIP-Integrität erfolgreich; 48 Einträge; 19 Seiten je Format. |
| `netcore-tetra-wap-portal.patch` | Historischer Unified Diff der Portaländerung | Originaldatei wiedergefunden und identifiziert; ursprüngliche Prüfsumme stimmt. |
| `fragment-borrow-fix.patch` | Kleiner E0502-Fix | Inhalt geprüft; separate Längenvariable. |
| `netcore-wap-portal-SHA256SUMS.txt` | Originalprüfsummen der ersten Portal-ZIP, Seiten-ZIP und Portalpatch | Alle drei aufgeführten Prüfsummen stimmen mit den wiedergefundenen Dateien überein. Die fixed-ZIP ist darin nicht aufgeführt. |

Die damaligen `/mnt/data/`-Downloadlinks sind historische Bereitstellungspfade und keine Repository-Dateipfade. Die Dateien konnten für diese Archivierung als gespeicherte Originalartefakte wiedergefunden werden; ihr ursprünglicher lokaler Pfad war dafür nicht erforderlich.

### 9.2 SHA-256

```text
f55639a8536e781256a639c3571cd4462793b59608c52fa1e587278a8407833a  netcore-tetra-swmi(1).zip
de8931101e6ea85551da3d51615cf7dc1ea05356e660a6fea1529dadf4a02308  netcore-tetra-swmi-wap-portal.zip
9a3951911d0f8e46124d07b257a209a9d23e036cd0a186d30475ceeeefd34764  netcore-tetra-swmi-wap-portal-fixed.zip
19c852df16a7d347865798fa3a7a9b21b3a0514a1eca0f9ce32738580059b8aa  netcore-wap-portal-pages.zip
18c1cfb3f64fa689d1c99db93f17fbb897039c247ebe7ea95bb96ed5f68c6eb7  netcore-tetra-wap-portal.patch
f7a6f4950fab7903d9b25a06a6c6face9a22fba7328930efedfeecdfd24bead6  fragment-borrow-fix.patch
27b7c324968087266f7b90b7e40a21e084ca16e5330f13fa46b03f63d944a6a8  netcore-wap-portal-SHA256SUMS.txt
```

### 9.3 Festgestellte Paketabweichungen

Die fixed-ZIP fügt gegenüber dem Eingangssnapshot 43 Dateien hinzu und verändert vier vorhandene Dateien:

- `Docs/WAP_INTEGRATION.md`
- `crates/tetra-entities/src/sndcp/fragment.rs`
- `crates/tetra-entities/src/sndcp/mod.rs`
- `crates/tetra-entities/src/sndcp/wap_ip.rs`

Die wesentlichen zusätzlichen Dateien sind Portalrenderer, Portaldokumentation, Validierungsdokumentation sowie Referenzseiten/-validator.

Außerdem fehlen im gelieferten Projektpaket vier im Eingangssnapshot vorhandene Wiki-Dateien mit Unicode-Bindestrich:

- `wiki/Device‐Groups.md.md`
- `wiki/NetCore‐Directory.md.md`
- `wiki/Status‐Messages.md.md`
- `wiki/Systemd‐Service.md.md`

Dies ist eine tatsächlich beobachtete Packaging-Abweichung, keine im Chat beschlossene Löschung. Die genaue Ursache wurde nicht ermittelt. Bei einer späteren Paketübernahme diese Inhalte erhalten; im Archivauftrag wurden sie nicht im Repository geändert.

## 10. Zusätzlich geprüfter Repository-Stand vom 05.10.2026

### 10.1 Branches und Gleichstand der relevanten Dateien

`main` und `Archiving` wurden an den in Abschnitt 1 genannten Commits getrennt gelesen. Die entscheidenden WAP-, Fragment-, MLE-/CMCE-BS-, Test- und Konfigurationsdateien haben dort gleiche Blob-SHAs. Die nachstehenden Aussagen gelten für diese geprüften Dateien in beiden Ständen.

Der historische Ref `refs/heads/swmi` lieferte beim Prüfen HTTP 404; die Branchsuche nach `swmi` fand keinen Branch. Der heutige Vergleich erfolgt daher nicht gegen einen vermeintlich noch vorhandenen `swmi`-Head.

Die aktuelle pfadbezogene Commit-Historie für `fragment.rs` und `wap_portal.rs` führte zum Importcommit [`45cd9b6c3f001c99a1516ab86db7f767be806a91`](https://github.com/JanHG98/netcore-tetra/commit/45cd9b6c3f001c99a1516ab86db7f767be806a91) vom 26.09.2026. Dessen Nachricht nennt einen importierten früheren Snapshot; der genaue historische Fix-/Portalcommit wurde darüber nicht separat nachgewiesen.

### 10.2 Wesentlicher Integrationsbefund

**Die aktuelle Repository-Ablage enthält das Portal, aber der eingebaute WAP-Router bindet es nicht ein.**

Nachprüfbare Unterschiede:

| Prüfung | Historische Portal-ZIP | Heutiger Repository-Stand |
|---|---|---|
| `sndcp/mod.rs` | Enthält `pub mod wap_portal;` | Enthält diese Deklaration nicht. |
| `wap_ip.rs` | Importiert `WapMarkup`, `WapPage`, Parser und Renderer; benutzt `portal_route`. | Importiert die alten `wap_status`-Renderer; kein Portalimport/-aufruf. |
| Pfadauswahl | Kurzrouten und alle lesbaren Aliase über `parse_portal_path` | `path_allowed` prüft lediglich `/`, `/status`, `/status.xhtml` und `/status.wml`. |
| Health-Query | Status mit `?s=1` wird auf Portal-Health umgebogen. | Alter Sektorrenderer bei `?s=`; keine Portal-Health-Routenauswahl. |
| Formatnavigation | Formatrein im Portalrenderer und in Referenzseiten. | Alter XHTML-Statusrenderer verlinkt auf `/status.wml?s=1`; WML-Sektorseite verlinkt zurück auf `/`. |
| Kompilierbare Portaltests | Modul im Paket deklariert; Ausführung dennoch nicht belegt. | Datei mit Tests abgelegt, aber über diesen Modulbaum nicht eingebunden. |
| Statische Seiten | 19 je Format | 21 je Format, geprüft. |

Damit sind Aussagen wie „alle Portalseiten sind im heutigen TBS-WSP-Pfad erreichbar“ oder „alle Navigation bleibt im aktiven Dienst formatrein“ am geprüften heutigen Code **nicht bestätigt und durch die direkte Handlerprüfung widerlegt**.

Das ist eine Archivierungsfeststellung. Die fehlende Einbindung wurde in diesem Auftrag nicht repariert, weil Änderungen außerhalb `Docs/archive/` ausdrücklich nicht autorisiert sind.

### 10.3 Heutige Ergänzung um Aufgaben/Formulare

Aktuell vorhanden:

- `ALL_PAGES: [WapPage; 21]`;
- zusätzliche Parser-/Rendererwerte `Tasks` und `TaskForms`;
- zusätzliche statische Referenzseiten `tasks` und `task-form`;
- Validator mit `EXPECTED_PAGES = 21`;
- aktualisierte `contrib/wap-portal/README.md`.

Die [Phase-9-Dokumentation](../PHASE_9_WAP_FORMS_STRUCTURED_TASKS.md) beschreibt einen separaten `system-backend/task-workflow/`-Dienst auf Port `8280`, Einstiegspfade `/x` und `/w`, eine Open-Lab-Testidentität per `?issi=4010001`, strukturierte Aufgaben `netcore-task-v1`, REST/MQTT/SDS und Statusübergänge.

Wichtige dokumentierte Trennung: Der kompakte TBS-WSP-Pfad ist **kein Reverse Proxy** zu diesem Dienst. Seine Aufgaben-/Formularseiten sind kurze Hinweise auf Port 8280. Ein Formularsystem im zentralen LXC ist keine durch diesen Chat bestätigte interaktive TBS-Portalumsetzung.

Das ist ein späterer Repository-Befund, kein nachträglich erfundener Beschluss im historischen Chat. Der Dienst wurde in diesem Archivauftrag nicht gebaut oder live geprüft.

### 10.4 Dokumentationsabweichungen

`Docs/WAP_PORTAL.md` und `Docs/WAP_PORTAL_VALIDATION_2026-07-30.md` beschreiben noch 19 Seiten je Format und eine aktive Portalintegration. Dem stehen aktuell 21 Referenzseiten sowie die fehlende WSP-Einbindung gegenüber.

Die Dokumente sind relevante historische Quellen, aber keine allein ausreichenden Belege für den heutigen Dienst. Eine spätere technische Änderung sollte Code, aktive Modulbindung, Tests und diese Dokumente gemeinsam konsolidieren.

### 10.5 Heutiger Stand der Compilerprobleme

- **E0502:** Die fehlerhafte Fragmenttestzeile ist durch `let packet_len = packet.len() as u16;` vor dem Slicezugriff ersetzt. Lösung im Repository vorhanden; hier nicht mit Rust ausgeführt.
- **MLE-/CMCE-Test-APIs:** Die in den Logs erwarteten öffentlichen Methoden fehlen weiterhin in den geprüften BS-Implementierungen.
- **LA-Typ:** Der Restore-Testhelper verwendet weiterhin `location_area: u8`; das Konfigurationsfeld ist `u16`.
- **Clone:** `UCallRestore` ist weiterhin nur mit `Debug` abgeleitet; der Test verwendet weiterhin `restore.clone()`.
- **Reexportwarnung:** `common/mod.rs` unterdrückt Dead-Code-Warnungen, exportiert `TestCell`/`TwoCellHarness` aber weiter. Das ist keine Reparatur fehlender Methoden.

Diese Befunde ergeben sich aus Quelltextprüfung. Sie ersetzen keinen vollständigen Cargo-Lauf und behaupten keine vollständige Fehlerliste sämtlicher Targets/Features.

## 11. Tests und Beweisgrenzen

### 11.1 Prüfmatrix

| Prüfung | Historisch im Chat | Erneut bei Archivierung | Grenze |
|---|---|---|---|
| Integrität der ersten Portal-ZIP | Als erfolgreich gemeldet | Erfolgreich mit ZIP-CRC-Prüfung | Kein Build-/Funktionsnachweis. |
| Integrität der fixed-ZIP und Seiten-ZIP | Paket bereitgestellt | Erfolgreich | Keine Geräteabnahme. |
| Original-SHA-256-Liste | Bereitgestellt | Alle drei enthaltenen Einträge stimmen | fixed-ZIP separat berechnet. |
| Statisches historisches Portal | 19 XHTML + 19 WML als gültig/verlinkt gemeldet | Validator erfolgreich: 19 + 19 | XML-Wohlgeformtheit; keine vollständige DTD-/Browser-Konformitätsprüfung. |
| Statisches aktuelles Portal | Nicht Teil des ursprünglichen Chatabschlusses | Validator erfolgreich: 21 + 21 | Keine aktive WSP-Routenprüfung durch diesen Python-Test. |
| Formatlinks und Erreichbarkeit | Als geprüft gemeldet | Beide statischen Portalstände erfolgreich | Erreichbarkeit im Dateigraphen, nicht über Funk. |
| Dynamische Bytebudgets | Mit Referenz-Snapshot modelliert dokumentiert | Konstanten/Renderer/Tests im Quelltext geprüft | Keine erneut kompilierte Rust-Messung. |
| WTP/WSP-Testvektoren | Tests im Paket enthalten | Quelltext geprüft | Tests hier nicht kompiliert ausgeführt. |
| Borrow-Fix | Vorgeschlagen und in fixed-Paket bereitgestellt | In ZIP und Repository bestätigt | Kein grüner Rust-Lauf. |
| Mehrzellentests | Benutzer meldet Kompilierungsfehler | Inkonsistenzen im Code wiedergefunden | Kein erfolgreiches Ende-zu-Ende-/Handover-Ergebnis. |
| Updateskript | Vorgehen vorgeschlagen | `bash -n` erfolgreich | Keine tatsächliche Installation/Serviceänderung. |
| On-Air-WAP-Betrieb | Keine Erfolgsmeldung nach Portalupdate | Nicht verfügbar | Vollständig offen. |

### 11.2 Vorhandene, aber nicht hier ausgeführte Rust-Portaltests

Im historischen `wap_portal.rs`:

- `all_pages_have_both_readable_aliases`
- `rendered_pages_stay_inside_openwave_caps`
- `each_format_is_fully_navigable_without_cross_format_links`

Im historischen `wap_ip.rs` unter anderem:

- `connect_reply_matches_reference_vector`
- `full_connect_invoke_produces_reference_reply`
- `wsp_get_uses_uintvar_uri_and_returns_xhtml`
- `readable_wml_alias_returns_wml_content_type`
- `legacy_status_sector_query_maps_to_health_page`
- `endpoint_response_swaps_addresses_ports_and_increments_id`
- `three_octet_invoke_is_rejected_without_panicking`
- `ack_needs_no_response`

Diese Testnamen sind im Artefakt vorhanden. Ein enthaltenes `#[test]` ist kein ausgeführtes Testergebnis.

## 12. Überholte, verworfene und missverständliche Aussagen

| Frühere Aussage/Ansatz | Maßgeblicher Stand |
|---|---|
| OMA/WBXML/DRM als weitere „Seitenformate“ | Formatvergleich; Benutzer entscheidet anschließend ausschließlich für XHTML und WML. |
| „WML script“-Beispiel | Geliefert war WML-Markup, kein WMLScript. |
| Statischer Websiteordner allein genügt | Historisches Paket erweitert zusätzlich den eingebauten Router. Im heutigen Code ist genau diese Integration wieder abzugleichen. |
| „38 Seiten insgesamt“ als dauerhafte Projektzahl | Richtig für das historische Portal; heute 42 statische Referenzseiten durch Phase 9. |
| „Portal fertig“ | Für das erzeugte Artefakt und die statische Validierung zutreffend; Rust-Build, Deployment und Gerätebetrieb nicht bestätigt. |
| „fixed“-ZIP löst die Tests | Sie löst ausschließlich die E0502-Stelle, nicht den später gemeldeten Mehrzellen-Testbestand. |
| Neuen Inhalt durch Neuinstallation des IP-Gateways aktivieren | Für das lokal von der Basisstation beantwortete Portal nicht erforderlich. |
| Bestehende Accept-Schalter aktivieren automatisch alle heutigen Portalseiten | Gilt für den historischen Portalpatch; nicht für den heute aktiven alten Router. |
| XHTML/WML überall formatrein | Statische Portale sind geprüft formatrein; heutige alte Statusrenderer wechseln teilweise das Format. |
| Update ist nur Binarytausch ohne weitere Wirkung | Skript enthält auch TTS-/Piper-Migration und Media-Library-Prüfungen. |
| Erfolgsmeldung eines gefilterten Cargo-Laufs reicht | Testanzahl und Targets prüfen; null Portaltests wegen fehlender Modulbindung sind keine Abnahme. |

## 13. Offene Aufgaben, Roadmap-Kandidaten und Fortsetzung

### 13.1 Bereits aus diesem Chat offene Arbeit

| Priorität | Aufgabe | Status und Abhängigkeit |
|---|---|---|
| 1 | Quell-/Paketstand auf der echten Basisstation erfassen | Offen; Commit, Binarypfad, Unit und Konfiguration sichern, bevor ein neuer Updateversuch erfolgt. |
| 1 | Mehrzellen-Testbestand mit MLE-/CMCE-Implementierungen konsistent reparieren | Im letzten Chatturn angekündigt, nicht fertig geliefert; API-/Zustandsabgleich vor breiter Testabnahme. |
| 1 | LA-Typ und U-CALL-RESTORE-Wiederverwendung reparieren | Konkrete weiterhin relevante Compilerblocker; nicht durch den E0502-Fix erledigt. |
| 1 | Aktive Portalintegration im heutigen Code herstellen/prüfen | Bei Archivierung entdeckte Voraussetzung für tatsächliche Routen und Testeinbindung. |
| 2 | Rust-Bibliotheks-, Integrationstest- und Releasebuild ausführen | Rust-Toolchain und konsistenter Source erforderlich. |
| 2 | Basisstation kontrolliert aktualisieren | Erst nach geprüftem Build; Skript-/TTS-Wirkungen und echte Unit/Binary klären. |
| 2 | XHTML und WML am realen WAP-Terminal abnehmen | URL-/Proxy-/Paketdatenprofil, PDP/PDCH/WSP und Inhaltsanzeige gemeinsam prüfen. |
| 3 | Heutige 21-Seiten-Dokumentation konsolidieren | Alten 19-Seiten-Stand als historisch kennzeichnen; aktive Integration und Testergebnisse dokumentieren. |
| 3 | Packaging-Abweichungen der Wiki-Dateien beseitigen | Keine unbeabsichtigten Dokumentverluste bei Übernahme des historischen Pakets. |

Die numerische Priorisierung ist die bei der Archivierung abgeleitete technische Reihenfolge. Im historischen Verlauf war nur vereinbart, nach den Fehlermeldungen Tests und Implementierungen konsistent abzugleichen; ein vollständiger Prioritätenplan wurde dort nicht ausdrücklich beschlossen.

### 13.2 Konkrete spätere Prüfkommandos

Nach Reparatur der entsprechenden Modul-/Testanbindung, **hier nicht ausgeführt**:

```bash
python3 contrib/wap-portal/validate.py
cargo test -p tetra-entities --lib sndcp::fragment
cargo test -p tetra-entities --lib sndcp::wap_portal
cargo test -p tetra-entities --lib sndcp::wap_ip
cargo test -p tetra-entities --test test_two_cell_foundation
cargo test -p tetra-entities --test test_two_cell_call_restore
cargo test -p tetra-entities --test test_mm_bs
```

Historisch vorgeschlagener größerer Releasebuild:

```bash
cargo build --release   -p bluestation-bs   -p netcore-control-room   -p netcore-control-room-operator   --features "bluestation-bs/asterisk,bluestation-bs/recording,bluestation-bs/audio-player"
```

Dieser Befehl wurde im Chat empfohlen, aber nicht als erfolgreich ausgeführt bestätigt. Ein Releasebuild ohne Tests beweist keine erfolgreiche Integrationstestabnahme.

### 13.3 Geräteabnahme und kleine Nebenideen

Bei der späteren Fortsetzung bleiben relevant:

- beide Startseiten, Kurzrouten und lesbaren Aliase am selben Terminal testen;
- formatreine Navigation einschließlich `P`, `N`, `H` und Rückweg zum Start prüfen;
- alte Bookmarks `/status.xhtml?s=1` und `/status.wml?s=1` erhalten;
- sinnvolle Live-Zähler und abgeschnittene Health-/Aktivitätsstrings prüfen;
- Sonderzeichen, UTF-8, große Zähler und Bytebudgets kontrollieren;
- statische Referenzseiten auf einem geeigneten HTTP/WAP-Testserver nutzen, ohne diesen Test mit dem On-Air-WSP-Test gleichzusetzen;
- Gateway-/Paketdatenzustände als Informationsanzeige behalten;
- Aufgaben-/Formularseiten aus der späteren Phase 9 gegebenenfalls einbeziehen; zentrale Formulare und lokalen Kurzrenderer getrennt abnehmen.

OMA Push, WBXML und DRM bleiben lediglich historische Nebenideen. Ihre spätere Umsetzung wurde in diesem Chat ausdrücklich nicht weiterverfolgt.

## 14. Repository-Quellen, Anhänge und Bilder

### 14.1 Maßgebliche Repository-Dateien

Relative Links beziehen sich auf das Repository und damit auf den jeweiligen Branch beim Lesen. Für reproduzierbare Codeaussagen gelten die festgehaltenen Commitstände aus Abschnitt 1.

- [WAP-Integration](../WAP_INTEGRATION.md)
- [Historische Portaldokumentation](../WAP_PORTAL.md)
- [Historische Portalvalidierung vom 30.07.2026](../WAP_PORTAL_VALIDATION_2026-07-30.md)
- [Phase 9: Aufgaben und Formulare](../PHASE_9_WAP_FORMS_STRUCTURED_TASKS.md)
- [Statische Portalreferenz](../../contrib/wap-portal/README.md)
- [Statischer Validator](../../contrib/wap-portal/validate.py)
- [SNDCP-Modulbaum](../../crates/tetra-entities/src/sndcp/mod.rs)
- [WAP-IP-/WSP-Handler](../../crates/tetra-entities/src/sndcp/wap_ip.rs)
- [Abgelegter Portalrenderer](../../crates/tetra-entities/src/sndcp/wap_portal.rs)
- [Statusrenderer](../../crates/tetra-entities/src/sndcp/wap_status.rs)
- [SNDCP-Basisstation](../../crates/tetra-entities/src/sndcp/sndcp_bs.rs)
- [IPv4-Fragmentierung](../../crates/tetra-entities/src/sndcp/fragment.rs)
- [Zell-/WAP-Konfiguration](../../crates/tetra-config/src/bluestation/sec_cell.rs)
- [Gemeinsames Integrationstestmodul](../../crates/tetra-entities/tests/common/mod.rs)
- [Two-Cell-Harness](../../crates/tetra-entities/tests/common/two_cell.rs)
- [Two-Cell-Call-Restore-Test](../../crates/tetra-entities/tests/test_two_cell_call_restore.rs)
- [MLE-BS](../../crates/tetra-entities/src/mle/mle_bs.rs)
- [CMCE-BS](../../crates/tetra-entities/src/cmce/cmce_bs.rs)
- [Call-Restore-Laufzeitmodell](../../crates/tetra-entities/src/cmce/call_restore_runtime.rs)
- [U-CALL RESTORE](../../crates/tetra-pdus/src/cmce/pdus/u_call_restore.rs)
- [Basisstations-Update](../../install/update-basisstation.sh)

Für diesen Chat wurde keine zugehörige PR-Nummer oder ein einzelner historischer Portal-/Reparaturcommit nachgewiesen. Andere Projekt-PRs werden diesem Auftrag nicht ohne Beleg zugerechnet.

### 14.2 Verwandte, eigenständige Archive

Die folgenden Dateien betreffen Nachbarthemen und wurden nicht überschrieben oder mit diesem Chat gleichgesetzt:

- [WAP/SNDCP, IP-Gateway und Multi-PDCH](2026-10-03_wap-sndcp-ip-gateway-multi-pdch-und-control-room.md)
- [WAP/IP-Mehrgerätebetrieb und Logdiagnose](2026-10-04_codex-wap-ip-mehrgeraetebetrieb-und-logdiagnose.md)
- [Motorola-CPS-WAP-Browserprofil](2026-10-05_motorola-cps-wap-browser-konfiguration.md)

### 14.3 Bildbestand

Im zugänglichen Verlauf dieses Chats ist kein eigenständiges Bild oder Screenshot enthalten. Die vorhandenen Benutzer-Compilerlogs sind Text. Die drei historischen erzeugten ZIPs enthalten ebenfalls keine Bilddateien; der hochgeladene Quellsnapshot enthält keine einschlägigen Bildanhänge des Chats.

Die ETSI-PDFs enthalten Normseiten und Abbildungen, sind aber keine separat vom Benutzer im Gespräch geteilten Screenshots. Es wurden keine Normabbildungen als vermeintliche Chatbilder exportiert. Bilder aus anderen Projekt-/Medizinchats wurden nicht zugeordnet.

**Folge für den Auftrag „Bilder des Chats auch hochladen“:** Es konnte kein diesem Chat zugehöriges Originalbild zum Upload ermittelt werden. Falls im nicht übergebenen Originalchat weitere Bilder existierten, fehlen deren Originaldateien/Zuordnung hier. Diese Lücke wurde nicht durch erzeugte Ersatzbilder verdeckt.

### 14.4 PDF-Anhangsinventar

Die folgenden Dateien waren lokal verfügbar. Titel, Version und Seitenzahlen wurden geprüft; sie wurden nicht pauschal vollständig fachlich ausgewertet. Die Sammlung `ETSI.pdf` beginnt mit EN 300 812, enthält jedoch 4.100 Seiten und ist daher nicht als bloße Kopie der einzelnen 156-seitigen EN-300-812-Datei behandelt.

| Anhang | Norm/Version laut Titelseite | Seiten | Thema/Prüfumfang |
|---|---|---:|---|
| `en_3003920308v010401p.pdf` | ETSI EN 300 392-3-8 V1.4.1 (2020-04) | 22 | ISI: Generic Speech Format Implementation |
| `en_30039209v010701p.pdf` | ETSI EN 300 392-9 V1.7.1 (2020-04) | 46 | Allgemeine Anforderungen an Supplementary Services |
| `ts_10081201v020205p.pdf` | ETSI TS 100 812-1 V2.2.5 (2003-10) | 8 | SIM-ME/UICC: physische und logische Eigenschaften |
| `en_3003921201v010202p.pdf` | ETSI EN 300 392-12-1 V1.2.2 (2007-08) | 56 | Stage 3: Call Identification |
| `en_3003920304v010301p.pdf` | ETSI EN 300 392-3-4 V1.3.1 (2010-08) | 28 | ISI: Short Data Service (ANF-ISISDS) |
| `en_3003921117v010102p.pdf` | ETSI EN 300 392-11-17 V1.1.2 (2002-01) | 18 | Stage 2: Include Call |
| `en_3003921114v010101p.pdf` | ETSI EN 300 392-11-14 V1.1.1 (2002-07) | 23 | Stage 2: Late Entry |
| `es_20081202v020401m.pdf` | Final draft   ETSI ES 200 812-2 V2.4.1 (2005-08) | 139 | TSIM-Anwendung; Final Draft |
| `es_20081201v020205p.pdf` | ETSI ES 200 812-1 V2.2.5 (2003-12) | 8 | TSIM-ME/UICC: physische und logische Eigenschaften |
| `en_300812v020101p.pdf` | ETSI EN 300 812 V2.1.1 (2001-12) | 156 | Security: SIM-ME Interface |
| `en_3003921101v010201p.pdf` | ETSI EN 300 392-11-1 V1.2.1 (2004-01) | 44 | Stage 2: Call Identification |
| `en_3003921006v010401p.pdf` | ETSI EN 300 392-10-6 V1.4.1 (2006-08) | 20 | Stage 1: Call Authorized by Dispatcher |
| `en_3003921018v010301p.pdf` | ETSI EN 300 392-10-18 V1.3.1 (2003-10) | 17 | Stage 1: Barring of Outgoing Calls |
| `en_3003921216v010400a.pdf` | DRAFT   ETSI EN 300 392-12-16 V1.4.0 (2026-03) | 67 | Stage 3: Pre-emptive Priority Call; Draft |
| `en_30039201v010601p.pdf` | ETSI EN 300 392-1 V1.6.1 (2020-04) | 182 | General Network Design |
| `ets_30039214e01v.pdf` | pr ETS 300 392-14, Final Draft 1997-09 | 61 | PICS-Proforma; Final Draft September 1997 |
| `en_30039207v030501p.pdf` | ETSI EN 300 392-7 V3.5.1 (2019-07) | 216 | Security |
| `en_30039401v030301p.pdf` | ETSI EN 300 394-1 V3.3.1 (2015-04) | 169 | Conformance Testing: Radio |
| `en_3003920313v010201p.pdf` | ETSI EN 300 392-3-13 V1.2.1 (2020-04) | 191 | Transport Layer Independent ISI Group Call |
| `en_30039502v010303p.pdf` | ETSI EN 300 395-2 V1.3.3 (2025-02) | 94 | TETRA Speech Codec |
| `en_3003920303v010301p.pdf` | ETSI EN 300 392-3-3 V1.3.1 (2011-11) | 251 | ISI Group Call |
| `en_30039205v020701p.pdf` | ETSI EN 300 392-5 V2.7.1 (2020-04) | 320 | Peripheral Equipment Interface (PEI) |
| `en_3003920315v010500a.pdf` | Draft ETSI EN 300 392-3-15 V1.5.0 (2026-04) | 380 | Transport Layer Independent ISI Mobility Management; Draft |
| `en_30039202v030801p.pdf` | ETSI EN 300 392-2 V3.8.1 (2016-08) | 1445 | Air Interface; SNDCP/WAP/U-CALL-RESTORE-Fundstellen gezielt geprüft |
| `ETSI.pdf` | ETSI EN 300 812 V2.1.1 (2001-12) | 4100 | Sammlung; beginnt mit EN 300 812 V2.1.1, vollständiger Inhalt nicht ausgewertet |

### 14.5 Historisch genannte externe Formatquellen

Im ersten Formatvergleich wurden folgende öffentliche Quellen genannt. Sie wurden bei dieser Archivierung nicht erneut als aktueller Standardstand recherchiert; die finale Entwicklung beschränkte sich auf die geprüften XHTML-/WML-Artefakte.

- [W3C: Introduction to Mobile Web](https://www.w3.org/wiki/Introduction_to_mobile_web)
- [W3C: XHTML Basic](https://www.w3.org/TR/xhtml-basic/)
- [W3C: historisches WBXML-Dokument](https://www.w3.org/1999/06/NOTE-wbxml-19990624/)
- [OMA DRM 1.0 Rights Expression Language](https://www.openmobilealliance.org/release/DRM/V1_0-20040625-A/OMA-Download-DRMREL-V1_0-20040615-A.pdf)

Die damalige OMA-Push-Antwort verlinkte zusätzlich eine Notification-Channel-REST-Spezifikation; der übergebene Link ist abgeschnitten. Das ist kein ausreichend präziser normativer Nachweis für die Service-Indication-Beispielsyntax. Ein vollständiger entsprechender SI-Quellenbeleg bleibt im sichtbaren Verlauf offen.

## 15. Abschlussstand für einen neuen Chat

Der historische Entwicklungsauftrag erzeugte ein **in der ZIP integriertes 19-Themen-Portal in XHTML/WML** und einen **konkreten Borrow-Checker-Fix**. Die statischen Referenzseiten sind erneut erfolgreich geprüft. Der Benutzer erreichte anschließend weitere Compilerfehler; eine vollständige Testreparatur oder erfolgreiche Portalinstallation ist nicht übergeben.

Der zusätzlich geprüfte heutige Repository-Stand enthält **21 Themen/42 statische Seiten**, aber **keine aktive Anbindung des abgelegten Portalrenderers im eingebauten WSP-Handler**. Der E0502-Fix ist vorhanden; die genannten Mehrzellen-Testinkonsistenzen bleiben im geprüften Code bestehen.

Eine Fortsetzung sollte zuerst den real installierten Quellstand bestimmen und die fehlende Portal-/Testanbindung konsistent lösen. Erst danach sind Build, kontrolliertes TBS-Update und reale XHTML-/WML-WAP-Abnahme belastbar. Für das lokale Portal ist keine Neuinstallation des IP-Gateway-LXC vereinbart oder erforderlich.

