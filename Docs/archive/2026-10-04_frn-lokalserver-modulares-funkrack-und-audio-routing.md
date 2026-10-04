# FRN-Lokalserver, modulares Funkrack und Audio-Routing

## 1. Metadaten und Aussagegrenzen

| Feld | Wert |
|---|---|
| Projekt | NetCore-Tetra |
| Thema | Lokales Free Radio Network (FRN), Windows-Server, Raspberry-Pi-Funkgateways und modulares Rack |
| Ursprünglicher Chattitel | Nicht zugänglich; kein Titel wird aus der ersten Frage erfunden |
| Ursprünglicher Chatlink | Nicht zugänglich |
| Historische Datierung | Ergänzende Verlaufssuche ordnet die Kernnachrichten dem 12.10.2025 zu; kein vollständiger Originalexport vorhanden |
| Erstellung und Repository-Prüfung | 04.10.2026, Europe/Berlin |
| Geprüftes Repository | https://github.com/JanHG98/netcore-tetra |
| Geprüfter und ausschließlich beschriebener Zielbranch | `Archiving` |
| Geprüfter Ausgangscommit | `83ebe448243ddf07fb9e3d31730a52680dd5d747` |
| Archivpfad | `Docs/archive/2026-10-04_frn-lokalserver-modulares-funkrack-und-audio-routing.md` |
| Umfang | Historische Konzeptdokumentation mit getrenntem Repository-Abgleich; keine FRN-Implementierung |

Der Commit, der dieses Archiv ablegt, ist über die Git-Historie dieser Datei nachvollziehbar. Der obige Ausgangscommit bezeichnet den vor dem Schreiben geprüften Stand, nicht den späteren Archivcommit.

**Auswertungslücke:** Direkt verfügbar waren vier historische Nutzernachrichten. Die damaligen Assistentenantworten wurden nicht als Volltext übergeben. Eine ergänzende Personal-Context-Verlaufssuche lieferte passende Auszüge mit Raum- und Crosslink-Vorschlägen, aber keinen vollständigen Chat, keine vollständige Konfiguration und keinen Chatlink. Diese Dokumentation beansprucht daher keine lückenlose Vollauswertung. Andere Projektchats und allgemeine Projekt-Erinnerungen werden nicht als Entscheidungen dieses FRN-Chats ausgegeben.

## 2. Ziel, Ausgangslage und rekonstruierter Verlauf

Jan fragte zunächst nach Kenntnissen zu FRN. Im weiteren Verlauf bestätigte er, dass die Interpretation richtig war. Die ergänzende Verlaufssuche identifiziert FRN als **Free Radio Network**.

Die konkrete Ausgangsfrage lautete sinngemäß: Kann ein lokales System aus einem Router, einem Windows-Rechner mit laufendem Server und mehreren Raspberry Pis als Clients für Relais aufgebaut werden? Anschließend entwickelte Jan daraus die Idee eines universellen modularen Racks mit Einschüben für **CB-Funk, TETRA, Amateurfunk (AFU), PMR, LPD und weitere Funkarten**. Als Bedien-/Routingkonzept nannte er die verschiedenen Räume eines Servers.

Die vier direkt verfügbaren Kernnachrichten behandeln:

1. Kenntnis und Einordnung von FRN.
2. Lokalen Router, Windows-Server und mehrere Raspberry-Pi-Clients für Relais.
3. Ein universelles Rack mit getrennten Funk-Einschüben.
4. Audio-Routing über mehrere Serverräume.

Eine ergänzende historische Assistentenantwort schlug mehrere Räume/Channels und gezielte Crosslinks über einen zusätzlichen Client beziehungsweise einen Pi mit zwei FRN-Instanzen vor. Die erhaltenen Beispielnamen sind `CB-Lokal`, `PMR-Cluster`, `TETRA-Link`, `HAM-Bridge` und `CrossBridge`. Dies sind **Vorschläge**, keine nachgewiesen eingerichteten Räume.

## 3. Status und endgültig erhaltene Anforderungen

| Gegenstand | Status | Beleg und Grenze |
|---|---|---|
| Lokales FRN-System | Idee / gewünschtes Zielbild | Nutzer stellt Machbarkeitsfrage; keine Installation belegt |
| Router als lokales Netz | Beschlossen/geplant als Konzeptbestandteil | Vom Nutzer im Zielbild genannt; Modell, Adressen und Konfiguration fehlen |
| Windows-Rechner als FRN-Server | Beschlossen/geplant als Zielaufbau | Keine Auswahl von Serverprodukt, Version oder Windows-Version belegt |
| Mehrere Raspberry Pis als Funk-/Relaisclients | Beschlossen/geplant als Zielaufbau | Anzahl, Modelle, Betriebssystem und Software offen |
| Modulare Rack-Einschübe | Idee / ausdrücklicher Wunsch | CB, TETRA, AFU, PMR, LPD und weitere Funkarten genannt |
| Trennung und Zuordnung über Räume | Gewünschtes Routingkonzept | Keine eingerichtete Raummatrix oder Abnahme belegt |
| Crosslinks / zwei FRN-Instanzen auf einem Pi | Idee, historischer Assistentenvorschlag | Technische Machbarkeit mit konkretem Client nicht geprüft |
| FRN-Code in NetCore-Tetra | Nicht nachgewiesen implementiert | Repository-Treffer betreffen Ausbauplanung und Handbücher |
| FRN-Hardware und Audio/PTT | Nicht nachgewiesen getestet | Keine Messwerte, Fotos, Logs oder Erfolgsberichte verfügbar |
| Produktiver lokaler FRN-Betrieb | Nicht im Betrieb bestätigt | Keine Betriebsbestätigung verfügbar |

Erhaltene Begründungen: Der Nutzer bewertet die Modularität positiv, weil unterschiedliche Funkarten in einem gemeinsamen universellen Rack untergebracht werden könnten. Räume sollen die logische Audiozuordnung ermöglichen. Eine Kosten-, Verfügbarkeits- oder Latenzentscheidung ist nicht überliefert.

Es gibt keine erhaltene ausdrückliche Korrektur, die Windows oder Pis aus diesem Konzept entfernt. Ein späterer Hinweis aus einem anderen Chat nennt die Raspberry Pis als physische Schnittstellen zwischen Funkgerät und Netzwerk, nicht als Container, und enthält die **Namensidee** `SRV-H-RPi-FRN01` (08.12.2025). Dies ist ergänzender Projektkontext und kein Nachweis einer real eingerichteten Maschine oder einer Namensfestlegung in diesem Chat.

## 4. Architektur und Komponenten

### Historisches Zielbild

| Ebene | Vorgesehene Rolle | Noch zu spezifizieren |
|---|---|---|
| Router / lokales Netzwerk | Verbindet Server und Pi-Clients | Switch/AP, DHCP oder feste Adressen, Erreichbarkeit ohne WAN |
| Windows-Rechner | Zentraler FRN-Server mit mehreren Räumen | Produkt, Version, Start als Dienst, Konten und Offline-Verhalten |
| Raspberry Pi je Funkmodul | FRN-Client und Verbindung zum Funkgerät/Relais | Pi-Modell, Client, Audiohardware, PTT und Empfangserkennung |
| Funk-Einschub | Funktechnik für CB/TETRA/AFU/PMR/LPD | Gerätemodell, Antennenpfad, Versorgung und Anschlüsse |
| Räume | Logische Zuordnung von Audioteilnehmern | Raumliste, Zuordnung, Bedienung und Berechtigungen |
| Optionale Crosslink-Komponente | Kontrollierter Übergang zwischen Räumen | Clientunterstützung, Richtungen, Schleifenschutz und Sprechrecht |

Die Aussage „Räume ermöglichen Audio-Routing“ ist als gewünschtes Konzept zu bewahren. Sie belegt keine beliebige Audio-Matrix: Raumzugehörigkeit, Wechsel zwischen Räumen und eine aktive Brücke zwischen zwei Räumen sind unterschiedliche Funktionen. Ob ein konkreter FRN-Client gleichzeitig mehrere Instanzen und unabhängige Audio-/PTT-Pfade unterstützt, bleibt zu prüfen. Die historischen Crosslink-Vorschläge dürfen nicht ungeprüft als fertige Serverfunktion beschrieben werden.

### TETRA-Schnittstelle

Ein TETRA-Einschub ist gewünscht. Nicht festgelegt ist, ob die Kopplung über ein physisches Funkgerät mit Audio/PTT, über eine Geräteschnittstelle oder über ein Softwaregateway im NetCore-Backend erfolgt. Eine Audio-Brücke überträgt nicht automatisch TETRA-Signalisierung, Teilnehmeridentitäten, Gruppenverwaltung, SDS, Prioritäten oder Mobility. Für eine native Kopplung müssen Medienformat, Rufzuordnung und Sprechrecht ausdrücklich definiert werden.

## 5. Historischer Entwicklungs- und Betriebsstand

Der verfügbare Chat endet auf der Konzeptstufe. Es liegen keine erfolgreich ausgeführten FRN-Installationen, keine angelegten Räume, keine eingerichteten Pi-Clients und keine Funkabnahme vor. Eine vorhandene NetCore-TETRA-Anlage aus anderen Chats ist kein Beleg für dieses FRN-Rack.

Es fehlen insbesondere Stückliste, Rackmaße, Einschubhöhe, Verkabelungsplan, Strombudget, Audiopegel, PTT-/GPIO-Belegung und eine Liste verwendeter Funkgeräte. Für das Wort „Relais“ ist nicht abschließend geklärt, ob ein vorhandenes Funkrelais angebunden oder ein Gateway mit angeschlossenem Funkgerät gemeint war.

## 6. Zusätzlich überprüfter Repository-Stand am 04.10.2026

Prüfung am Ausgangscommit `83ebe448243ddf07fb9e3d31730a52680dd5d747` auf `Archiving`. Es wurde nicht behauptet, damit den aktuellsten Stand aller Entwicklungsbranches oder laufender Anlagen geprüft zu haben.

| Datei / Bereich | Tatsächlich festgestellter Inhalt | Bedeutung für FRN |
|---|---|---|
| `system-backend/roadmap.md`, Abschnitt „MQTT-Branch – aktuelle Integrationsreihenfolge“ | Punkt 9: „Zello, FRN und weitere Voice-Gateways“; Punkt 8: zentraler SIP-Switch nach hinten gestellt | FRN bleibt ein Roadmap-Kandidat; keine Umsetzungszusage mit Termin |
| `Docs/NetCore-Tetra-Komplettguide-2026-09-28.md` | Weitere Voice-Gateways wie Zello/FRN noch zu konkretisieren; Connectoren belegen keine vollständige Ende-zu-Ende-Funktion | Deckt sich mit unbestätigtem FRN-Status |
| `Docs/NetCore-Tetra-Systemhandbuch-2026-09-28.md` | FRN/Zello als erhaltene Ausbauideen | Kein FRN-Betriebsnachweis |
| `system-backend/media-switch/README.md` | Zentraler Transport gepackter 35-Byte-TETRA-ACELP-Sprachframes; HTTP/WebUI 8130; Node-Gateway-/Call-Control-WebSockets | Relevanter vorhandener Medienbaustein, aber keine FRN-Anbindung |
| `system-backend/media-switch/src/gateway.rs` | WebSocket-Verbindung zum Node Gateway, Protokollheader, Subscription `media_frames`, Reconnect-Schleife | Codebeleg für NetCore-Medientransport; kein FRN-Protokollbeleg |
| `system-backend/README.md` | Node Gateway 8080; Application Gateway 8220; Media Library 8230; IoT Gateway 8240; Hardware Gateway 8250 | Kontext für spätere Integration, keine FRN-Portzuweisung |

Die textuelle Suche nach `FRN` beziehungsweise `Free Radio Network` im ausgecheckten Repository unter Ausschluss von PDF-Inhalten und Cargo.lock fand nur Roadmap-/Handbuchverweise. Kein eigener FRN-Adapter oder FRN-Installationspfad wurde gefunden. Diese Negativaussage ist auf die Suchbegriffe und diesen Branch begrenzt; anders benannter oder externer Code wird damit nicht ausgeschlossen.

Der vorhandene Media Switch verwendet laut README ausschließlich `security.mode = "open_lab"`. Seine Beschreibung ist kein Nachweis eines abgesicherten FRN-Gateways. Es wurden keine Builds und keine Live-Tests dieser Dienste durchgeführt.

### Historie gegenüber heutigem Befund

Es ist kein Widerspruch zwischen dem frühen modularen Rackwunsch und der heutigen FRN-Ausbauplanung sichtbar. Der heutige Backend-Medientransport ist gegenüber dem frühen Konzept konkreter, löst aber die FRN-Kopplung nicht nachweislich. Alte Raum-/Crosslink-Vorschläge sind weiterhin unbestätigte Ideen. Eine inzwischen erfolgreich behobene FRN-Störung ist weder im Chat noch in der Prüfung belegt.

## 7. Technische Parameter, Konfigurationen und Dienste

| Parameter | Dokumentierter Stand |
|---|---|
| FRN-Serverport / Transport | Nicht festgelegt oder verifiziert; keine Standardwerte erfunden |
| FRN-Server-IP, Hostname, Bind-Adresse | Nicht festgelegt |
| Clientsoftware / Serverdistribution / Versionsstände | Nicht erhalten |
| Pi-Betriebssystem | Nicht festgelegt; RaspiOS aus anderen Themen ist keine Entscheidung dieses Chats |
| Audioformat, Codec, Samplingrate | Für FRN offen; NetCore-Medienframes nicht damit gleichsetzen |
| Audio-Schnittstelle / PTT / COS / COR / VOX | Auswahl, elektrische Pegel und Logik offen |
| Funkfrequenzen, Leistung und Betriebsart | Für dieses Rack nicht festgelegt |
| Raumbeispiele | `CB-Lokal`, `PMR-Cluster`, `TETRA-Link`, `HAM-Bridge`, `CrossBridge`; nur vorgeschlagen |
| Konfigurationspfade / systemd-Dienste | Für FRN keine überliefert |
| Rackformat / Kühlung / Versorgung | Modularität gewünscht, konkrete Maße und Hardware offen |
| WAN-/Masterserver-Abhängigkeit | Lokaler Aufbau gewünscht; tatsächliche Autarkie der gewählten Software noch zu prüfen |

Keine Passwörter, Tokens, privaten Schlüssel oder Zugangsdaten wurden übernommen.

## 8. Befehle, Deployment und Reparatur

Im historischen FRN-Verlauf sind keine Shell-/PowerShell-Befehle oder Installationsabläufe erhalten. Daher gibt es keinen als erfolgreich ausführbar bestätigten FRN-Einzeiler und keine rekonstruierte Konfiguration.

**Bei der Archivprüfung tatsächlich erfolgreich ausgeführt:**

```bash
git ls-remote https://github.com/JanHG98/netcore-tetra.git refs/heads/Archiving
git clone --single-branch --branch Archiving https://github.com/JanHG98/netcore-tetra.git repo
git -C repo rev-parse HEAD
git -C repo fetch origin Archiving
git -C repo merge --ff-only origin/Archiving
rg -n -i '\bfrn\b|free radio network' repo --glob '!Cargo.lock' --glob '!*.pdf'
```

Diese Befehle prüfen Repository und Dokumentationsbasis. Sie installieren keinen FRN-Dienst. Die Fast-forward-Aktualisierung betrifft ausschließlich den vorhandenen Zielbranch; andere Branches wurden nicht gemergt.

## 9. Fehler, Diagnose und Tests

Im FRN-Chat sind keine konkreten Fehlerbilder, Logs, Diagnosen oder funktionierenden Reparaturen erhalten. Ein störungsfreier Betrieb kann daraus ebenfalls nicht abgeleitet werden.

Durchgeführte Prüfungen für dieses Archiv: Zielbranch und Commit bestimmt, vorhandenen Archivindex und Dateinamen geprüft, FRN-Texttreffer ausgewertet, vorhandenen Media-Switch-Code und Dokumentation gelesen, 25 PDF-Titelseiten inventarisiert. Das sind Dokumentationsprüfungen, keine Funk-, Audio-, Last- oder Sicherheitsabnahme.

Nicht durchgeführt: Start des FRN-Servers, Pi-Enrollment, Raumwechsel, Crosslinks, Audiopegel-/Latenzmessung, PTT/COR-Test, Neustart-/WAN-Ausfalltest, Mehrteilnehmerbetrieb und TETRA-FRN-Ende-zu-Ende-Test.

## 10. Verworfene und ersetzte Ansätze

Keine ausdrücklich verworfene Architektur ist erhalten. Windows-Server, Pi-Clients und modulares Rack bleiben das historische Zielbild. Die später vorhandene NetCore-Backendarchitektur ersetzt den FRN-Server nicht automatisch. Crosslinks und Raumnamen bleiben Vorschläge, nicht veraltete produktive Konfigurationen.

## 11. Offene Aufgaben und Roadmap-Kandidaten

Die folgenden Schritte sind eine aus den offenen Punkten abgeleitete Fortsetzungsplanung, **keine nachträglich behauptete Vereinbarung**. Im Chat wurde keine verbindliche Priorität oder Frist festgelegt. Die Repository-Roadmap ordnet FRN nach Hardware-I/O, Workflows und dem zurückgestellten SIP-Switch ein; dieser Eintrag wurde hier nicht geändert.

1. **Konzept konkretisieren:** gewünschte Funkmodule, Zahl der Pi-Gateways, Bedeutung von „Relais“, getrennte Räume und erlaubte Brücken festlegen. Physische Funkgeräte gegenüber nativer TETRA-Backendkopplung entscheiden.
2. **Software auswählen:** konkrete Windows-Server- und Pi-Clientimplementierungen samt Version und Lizenz identifizieren. Vollständig lokalen Betrieb ohne WAN/Masterserver, mehrere Räume und zwei Instanzen auf einem Pi anhand der gewählten Software prüfen.
3. **Einmodul-Prototyp:** zunächst Server plus ein Pi plus ein Funkmodul. Audio-Ein-/Ausgänge, PTT und Empfangssignal spezifizieren; Verbindung, Pegel, Verzögerung und saubere Sendefreigabe messen.
4. **Raumkonzept abnehmen:** Beispielräume auf tatsächlichen Bedarf reduzieren, Zuordnung und Raumwechsel testen. Crosslinks separat prüfen; Schleifen, Rückkopplung und unbeabsichtigtes Dauersenden verhindern. Verhalten bei gleichzeitigem Empfang/Sprechen festlegen.
5. **Rack mechanisch und elektrisch planen:** Einschubformat, modularer Steckerstandard, Spannungen, Sicherungen, Kühlung, Antennenanschlüsse, Beschriftung und Wartungszugang festlegen. Keine Maße aus anderen Rackchats ungeprüft übernehmen.
6. **NetCore-Integration entwerfen:** Medienkonvertierung, Ruf-/Gruppenzuordnung und Sprechrechtsvermittlung definieren. Vorhandene Media-/Hardware-/IoT-Dienste auf Wiederverwendung prüfen; keine direkte Kompatibilität mit FRN voraussetzen.
7. **Betrieb abnehmen:** Server-/Pi-Neustarts, Verbindungsabbruch, WAN-Ausfall, Recovery und Mehrmodulbetrieb prüfen. Ergebnisse mit Softwareständen, Konfiguration ohne Secrets, Messwerten und Logs sichern.

Erhaltene Nebenideen: Erweiterbarkeit auf weitere Funkarten („und Co“), gemeinsame Unterbringung in einem universellen Rack, gezielte CrossBridge statt nur isolierter Räume sowie ein Pi mit zwei Clientinstanzen als möglicher Brückenbaustein. Die spätere Namensidee `SRV-H-RPi-FRN01` bleibt separat gekennzeichnet.

## 12. Quellen, Anhänge und Bilder

### Historische Quellen

- Vier direkt bereitgestellte Nutzernachrichten zum FRN-Lokalnetz, modularen Rack und Raumrouting.
- Ergänzende Personal-Context-Verlaufssuche: passende Kernnachrichten vom 12.10.2025 und Assistentenvorschlag mit fünf Raumbeispielen und Crosslink-Idee.
- Separater Kontext vom 08.12.2025 zu physischen Pi-Funkinterfaces und `SRV-H-RPi-FRN01`; ausdrücklich keine Implementierungsbestätigung.
- Keine spezifischen FRN-Commits oder FRN-PRs im zugänglichen Verlauf.

### Geprüfte Repository-Quellen

Alle folgenden relativen Links beziehen sich auf den Branchstand dieses Archivs:

- [Backend-Roadmap](../../system-backend/roadmap.md)
- [Backend-Übersicht](../../system-backend/README.md)
- [Media Switch](../../system-backend/media-switch/README.md)
- [Media-Switch-Gateway-Code](../../system-backend/media-switch/src/gateway.rs)
- [Komplettguide 28.09.2026](../NetCore-Tetra-Komplettguide-2026-09-28.md)
- [Systemhandbuch 28.09.2026](../NetCore-Tetra-Systemhandbuch-2026-09-28.md)

Die relativen Links wurden gegen den ausgecheckten Repository-Baum geprüft.

### Verfügbare Anhänge

Für diesen Turn wurden 25 ETSI-PDFs bereitgestellt. Die Titelseiten wurden per `pdftotext` auf Inhalt und Bezug geprüft. Sie behandeln TETRA-Netzdesign, Air Interface, Security, PEI, ISI, Zusatzdienste, SIM/UICC und Codec; **keine ist eine FRN-Server-/Clientdokumentation**. Eine vollständige Normprüfung ist nicht erfolgt. Normative Aussagen zur FRN-Machbarkeit werden daraus nicht abgeleitet. Die Dateien wurden nicht erneut ins Git kopiert; der Auftrag verlangt Chatbilder, und diese PDFs sind keine eigenständigen Chatbilder.

Anhanginventar:

- `en_3003920308v010401p.pdf`
- `en_30039209v010701p.pdf`
- `ts_10081201v020205p.pdf`
- `en_3003921201v010202p.pdf`
- `en_3003920304v010301p.pdf`
- `en_3003921117v010102p.pdf`
- `en_3003921114v010101p.pdf`
- `es_20081202v020401m.pdf`
- `es_20081201v020205p.pdf`
- `en_300812v020101p.pdf`
- `en_3003921101v010201p.pdf`
- `en_3003921006v010401p.pdf`
- `en_3003921018v010301p.pdf`
- `en_3003921216v010400a.pdf`
- `en_30039201v010601p.pdf`
- `ets_30039214e01v.pdf`
- `en_30039207v030501p.pdf`
- `en_30039401v030301p.pdf`
- `en_3003920313v010201p.pdf`
- `en_30039502v010303p.pdf`
- `en_3003920303v010301p.pdf`
- `en_30039205v020701p.pdf`
- `en_3003920315v010500a.pdf`
- `en_30039202v030801p.pdf`
- `ETSI.pdf`

`ETSI.pdf` und `en_300812v020101p.pdf` zeigen auf der Titelseite dieselbe Normkennung EN 300 812 V2.1.1; vollständige Dateigleichheit wurde nicht geprüft.

**Bilder:** Im bereitgestellten FRN-Verlauf und den verfügbaren Anhängen sind keine eigenständigen historischen Chatbilder vorhanden. Deshalb wurden keine fremden Projektbilder übernommen und keine Ersatzbilder erzeugt. Sollten im Originalchat Bilder existieren, sind sie eine verbleibende Auswertungslücke und können erst mit Zugriff auf die Originaldateien nacharchiviert werden.

## 13. Abschluss und Fortsetzungsgrenze

Das technische Ergebnis dieses Chats ist ein lokales FRN-/Funkrack-Konzept mit Windows-Server, physischen Pi-Gateways, modularen Funk-Einschüben und Raumrouting. Der Repository-Abgleich bestätigt FRN als Ausbauidee, nicht als implementiertes oder getestetes Gateway. Für eine Fortsetzung sind zuerst konkrete Software, Funkinterfaces und Routinganforderungen festzulegen. Änderungen dieses Archivauftrags bleiben ausschließlich in `Docs/archive/`.
