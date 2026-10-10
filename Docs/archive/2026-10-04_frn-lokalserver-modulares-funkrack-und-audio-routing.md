# Brainstorming: FRN-Lokalserver, modulares Funkrack und Audio-Routing

**Archivhinweis, 09.10.2026:** Diese Datei dokumentiert das im Titel beziehungsweise Quellenrahmen genannte Gespräch und dessen damalige Entscheidungen. Historische Kommandos, Planungen, Versionsangaben und Fehlerbilder bleiben erhalten; der heutige technische Stand steht in der [Gesamtroadmap](../roadmaps/gesamtroadmap.md) und in den [aktuellen Dienstanleitungen](../services/README.md). Die Archivaufnahme bestätigt keine spätere Implementierung oder Live-Abnahme.

**Stand der Notizen und ergänzenden Prüfungen: 2026-10-04.** Historische Entwürfe, nachgewiesene Umsetzung und ausgeführte Tests sind jeweils getrennt gekennzeichnet.

**Zielbild:** Lokales FRN mit Windows-Server, physischen Pi-Funkgateways, modularen Funk-Einschüben und Raumrouting. Die Architektur ist eine Idee; Softwarewahl, Audio/PTT, Crosslinks und autarker Betrieb sind noch zu prüfen.

## 1. Kontext und Quellenlage

| Feld | Wert |
|---|---|
| Projekt | NetCore-Tetra |
| Thema | Lokales Free Radio Network (FRN), Windows-Server, Raspberry-Pi-Funkgateways und modulares Rack |
| Historische Datierung | Kernkonzept vom 12.10.2025; Zuordnung aus ergänzenden Unterlagen, vollständiger Originalexport fehlt. |
| Erstellung und Repository-Prüfung | 04.10.2026, Europe/Berlin |
| Geprüftes Repository | https://github.com/JanHG98/netcore-tetra |
| Geprüfter Repository-Branch | `Archiving` |
| Geprüfter Ausgangscommit | `83ebe448243ddf07fb9e3d31730a52680dd5d747` |
| Archivpfad | `Docs/archive/2026-10-04_frn-lokalserver-modulares-funkrack-und-audio-routing.md` |
| Umfang | Historische Konzeptdokumentation mit getrenntem Repository-Abgleich; keine FRN-Implementierung |

Der Ausgangscommit fixiert den Repository-Stand für den technischen Abgleich.

**Quellenlage:** Das frühe Konzept und ergänzende Raum-/Crosslink-Ideen sind nur in Auszügen erhalten. Vollständige FRN-Konfiguration und Installationsnachweise fehlen.

## 2. Ziel, Ausgangslage und rekonstruierter Verlauf

FRN bezeichnet hier **Free Radio Network**.

Ziel ist ein lokales System mit Router, Windows-Rechner als Server und mehreren Raspberry Pis als Relais-/Funkclients. Daraus entstand die Idee eines universellen modularen Racks mit Einschüben für **CB-Funk, TETRA, Amateurfunk (AFU), PMR, LPD und weitere Funkarten**. Mehrere Serverräume sollen die Bedienung und Audiozuordnung ermöglichen.

Das Konzept umfasst vier Themen:

1. Kenntnis und Einordnung von FRN.
2. Lokalen Router, Windows-Server und mehrere Raspberry-Pi-Clients für Relais.
3. Ein universelles Rack mit getrennten Funk-Einschüben.
4. Audio-Routing über mehrere Serverräume.

Als Erweiterung sind mehrere Räume/Channels und gezielte Crosslinks über einen zusätzlichen Client oder einen Pi mit zwei FRN-Instanzen vorgeschlagen. Beispielnamen: `CB-Lokal`, `PMR-Cluster`, `TETRA-Link`, `HAM-Bridge` und `CrossBridge`. Die Räume sind nicht nachgewiesen eingerichtet.

## 3. Status und endgültig erhaltene Anforderungen

| Gegenstand | Status | Beleg und Grenze |
|---|---|---|
| Lokales FRN-System | Idee / gewünschtes Zielbild | Machbarkeit noch zu prüfen; keine Installation belegt |
| Router als lokales Netz | Beschlossen/geplant als Konzeptbestandteil | Im Zielbild festgelegt; Modell, Adressen und Konfiguration fehlen |
| Windows-Rechner als FRN-Server | Beschlossen/geplant als Zielaufbau | Keine Auswahl von Serverprodukt, Version oder Windows-Version belegt |
| Mehrere Raspberry Pis als Funk-/Relaisclients | Beschlossen/geplant als Zielaufbau | Anzahl, Modelle, Betriebssystem und Software offen |
| Modulare Rack-Einschübe | Idee / ausdrücklicher Wunsch | CB, TETRA, AFU, PMR, LPD und weitere Funkarten genannt |
| Trennung und Zuordnung über Räume | Gewünschtes Routingkonzept | Keine eingerichtete Raummatrix oder Abnahme belegt |
| Crosslinks / zwei FRN-Instanzen auf einem Pi | Historische Idee | Technische Machbarkeit mit konkretem Client nicht geprüft |
| FRN-Code in NetCore-Tetra | Nicht nachgewiesen implementiert | Repository-Treffer betreffen Ausbauplanung und Handbücher |
| FRN-Hardware und Audio/PTT | Nicht nachgewiesen getestet | Keine Messwerte, Fotos, Logs oder Erfolgsberichte verfügbar |
| Produktiver lokaler FRN-Betrieb | Nicht im Betrieb bestätigt | Keine Betriebsbestätigung verfügbar |

Die Modularität soll unterschiedliche Funkarten in einem universellen Rack zusammenführen. Räume sollen die logische Audiozuordnung ermöglichen. Kosten-, Verfügbarkeits- und Latenzvorgaben sind noch offen.

Windows-Server und Pi-Clients bleiben das Zielbild. Ein ergänzender Stand vom 08.12.2025 beschreibt die Raspberry Pis als physische Schnittstellen zwischen Funkgerät und Netzwerk und enthält die **Namensidee** `SRV-H-RPi-FRN01`. Weder eine eingerichtete Maschine noch eine verbindliche Namensfestlegung ist damit belegt.

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

Der historische Stand bleibt ein Konzept. Erfolgreiche FRN-Installation, angelegte Räume, eingerichtete Pi-Clients und Funkabnahme sind nicht dokumentiert. Bestehende NetCore-TETRA-Installationen bestätigen dieses FRN-Rack nicht.

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

### Historie gegenüber geprüftem Befund

Es ist kein Widerspruch zwischen dem frühen modularen Rackwunsch und der geprüften FRN-Ausbauplanung sichtbar. Der ergänzende Backend-Medientransport ist gegenüber dem frühen Konzept konkreter, löst aber die FRN-Kopplung nicht nachweislich. Alte Raum-/Crosslink-Vorschläge sind weiterhin unbestätigte Ideen. Eine inzwischen erfolgreich behobene FRN-Störung ist weder im Entwurf noch in der Prüfung belegt.

## 7. Technische Parameter, Konfigurationen und Dienste

| Parameter | Dokumentierter Stand |
|---|---|
| FRN-Serverport / Transport | Nicht festgelegt oder verifiziert; keine Standardwerte erfunden |
| FRN-Server-IP, Hostname, Bind-Adresse | Nicht festgelegt |
| Clientsoftware / Serverdistribution / Versionsstände | Nicht erhalten |
| Pi-Betriebssystem | Nicht festgelegt; RaspiOS aus anderen Themen ist keine Entscheidung dieser Planung |
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

Konkrete FRN-Fehlerbilder, Logs, Diagnosen und erfolgreiche Reparaturen fehlen. Ein störungsfreier Betrieb ist nicht bestätigt.

Durchgeführte Prüfungen für dieses Archiv: Zielbranch und Commit bestimmt, vorhandenen Archivindex und Dateinamen geprüft, FRN-Texttreffer ausgewertet, vorhandenen Media-Switch-Code und Dokumentation gelesen, 25 PDF-Titelseiten inventarisiert. Das sind Dokumentationsprüfungen, keine Funk-, Audio-, Last- oder Sicherheitsabnahme.

Nicht durchgeführt: Start des FRN-Servers, Pi-Enrollment, Raumwechsel, Crosslinks, Audiopegel-/Latenzmessung, PTT/COR-Test, Neustart-/WAN-Ausfalltest, Mehrteilnehmerbetrieb und TETRA-FRN-Ende-zu-Ende-Test.

## 10. Verworfene und ersetzte Ansätze

Keine ausdrücklich verworfene Architektur ist erhalten. Windows-Server, Pi-Clients und modulares Rack bleiben das historische Zielbild. Die später vorhandene NetCore-Backendarchitektur ersetzt den FRN-Server nicht automatisch. Crosslinks und Raumnamen bleiben Vorschläge, nicht veraltete produktive Konfigurationen.

## 11. Offene Aufgaben und Roadmap-Kandidaten

Die folgenden Schritte bilden einen **Vorschlag für die weitere Planung**; verbindliche Prioritäten und Fristen fehlen. Die Repository-Roadmap ordnet FRN nach Hardware-I/O, Workflows und dem zurückgestellten SIP-Switch ein.

1. **Konzept konkretisieren:** gewünschte Funkmodule, Zahl der Pi-Gateways, Bedeutung von „Relais“, getrennte Räume und erlaubte Brücken festlegen. Physische Funkgeräte gegenüber nativer TETRA-Backendkopplung entscheiden.
2. **Software auswählen:** konkrete Windows-Server- und Pi-Clientimplementierungen samt Version und Lizenz identifizieren. Vollständig lokalen Betrieb ohne WAN/Masterserver, mehrere Räume und zwei Instanzen auf einem Pi anhand der gewählten Software prüfen.
3. **Einmodul-Prototyp:** zunächst Server plus ein Pi plus ein Funkmodul. Audio-Ein-/Ausgänge, PTT und Empfangssignal spezifizieren; Verbindung, Pegel, Verzögerung und saubere Sendefreigabe messen.
4. **Raumkonzept abnehmen:** Beispielräume auf tatsächlichen Bedarf reduzieren, Zuordnung und Raumwechsel testen. Crosslinks separat prüfen; Schleifen, Rückkopplung und unbeabsichtigtes Dauersenden verhindern. Verhalten bei gleichzeitigem Empfang/Sprechen festlegen.
5. **Rack mechanisch und elektrisch planen:** Einschubformat, modularer Steckerstandard, Spannungen, Sicherungen, Kühlung, Antennenanschlüsse, Beschriftung und Wartungszugang festlegen. Maße anderer Rackentwürfe müssen zum konkreten Aufbau passen.
6. **NetCore-Integration entwerfen:** Medienkonvertierung, Ruf-/Gruppenzuordnung und Sprechrechtsvermittlung definieren. Vorhandene Media-/Hardware-/IoT-Dienste auf Wiederverwendung prüfen; keine direkte Kompatibilität mit FRN voraussetzen.
7. **Betrieb abnehmen:** Server-/Pi-Neustarts, Verbindungsabbruch, WAN-Ausfall, Recovery und Mehrmodulbetrieb prüfen. Ergebnisse mit Softwareständen, Konfiguration ohne Secrets, Messwerten und Logs sichern.

Erhaltene Nebenideen: Erweiterbarkeit auf weitere Funkarten („und Co“), gemeinsame Unterbringung in einem universellen Rack, gezielte CrossBridge statt nur isolierter Räume sowie ein Pi mit zwei Clientinstanzen als möglicher Brückenbaustein. Die spätere Namensidee `SRV-H-RPi-FRN01` bleibt separat gekennzeichnet.

## 12. Quellen, Anhänge und Bilder

### Historische Quellen

- Frühes Konzept vom 12.10.2025: FRN-Lokalnetz, modulares Rack und Raumrouting.
- Ergänzende Raum-/Crosslink-Ideen: fünf Raumbeispiele und möglicher Brückenclient; nur in Auszügen erhalten.
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

25 ETSI-PDFs wurden anhand ihrer Titelseiten mit `pdftotext` eingeordnet. Sie behandeln TETRA-Netzdesign, Air Interface, Security, PEI, ISI, Zusatzdienste, SIM/UICC und Codec; **keine ist eine FRN-Server-/Clientdokumentation**. Eine vollständige Normprüfung ist nicht erfolgt. Aussagen zur FRN-Machbarkeit lassen sich daraus nicht ableiten.

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

**Bilder:** Eigenständige historische FRN-Bilder sind nicht vorhanden. Falls Originalbilder wieder verfügbar werden, können sie die noch fehlende Aufbau- und Schnittstellendokumentation ergänzen.

## 13. Abschluss und Fortsetzungsgrenze

Arbeitsstand ist ein lokales FRN-/Funkrack-Konzept mit Windows-Server, physischen Pi-Gateways, modularen Funk-Einschüben und Raumrouting. Der Repository-Abgleich bestätigt FRN als Ausbauidee; ein implementiertes oder getestetes Gateway ist nicht nachgewiesen. Zuerst konkrete Software, Funkinterfaces und Routinganforderungen festlegen.
