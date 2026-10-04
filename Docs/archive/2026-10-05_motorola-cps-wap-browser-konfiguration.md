# Motorola CPS: WAP-Browser über NetCore-Tetra SNDCP

Technische Abschlussdokumentation zum in diesem Chat behandelten Motorola-CPS-Setup für den NetCore-Tetra-WAP-Browser. Historischer Gesprächsstand, hochgeladener Repository-Snapshot und am 5. Oktober 2026 erneut geprüfter Repository-Stand werden getrennt behandelt. Aussagen im Chat werden nicht automatisch als implementiert oder im Funkbetrieb bestätigt gewertet.

## 1. Metadaten und Quellenumfang

| Merkmal | Wert |
|---|---|
| Repository | [JanHG98/netcore-tetra](https://github.com/JanHG98/netcore-tetra) |
| Zielbranch dieses Archivs | `Archiving` |
| Zusammenfassung erstellt | **2026-10-05**, Europe/Berlin |
| Ursprünglicher Chattitel | Im zugänglichen Gesprächskontext **nicht verfügbar**; nicht erfunden |
| Chatlink / Chat-ID | Im zugänglichen Gesprächskontext **nicht verfügbar** |
| Hauptthema | Motorola-CPS-Parameter für TETRA-Paketdaten, CHAP und WAP/WSP gegen NetCore-Tetra |
| Vom Nutzer im Chat referenzierter Entwicklungsbranch | `swmi`; dieser Branch war bei der heutigen GitHub-Prüfung **nicht mehr vorhanden** |
| Zielbranch vor Beginn der Archivänderung | [`a76f0f5ae264e42aac2a244f3c2e32c09e7886bb`](https://github.com/JanHG98/netcore-tetra/commit/a76f0f5ae264e42aac2a244f3c2e32c09e7886bb) |
| Zusätzlich geprüfter heutiger Default-Branch | `main` bei [`7137e0dd69877e1b604bf89148fd8b6b590c1a97`](https://github.com/JanHG98/netcore-tetra/commit/7137e0dd69877e1b604bf89148fd8b6b590c1a97) |
| Hochgeladener Quell-Snapshot | `netcore-tetra-swmi(7).zip`, 31.391.861 Byte, 1.672 ZIP-Einträge |
| SHA-256 des ZIP | `665511c63b163cd85690d454fadf0c234b4cdcf9163722904e7bfaa9bd5b5031` |
| Archivdatei | `Docs/archive/2026-10-05_motorola-cps-wap-browser-konfiguration.md` |
| Chatbild-Archiv | `Docs/archive/assets/2026-10-05_motorola-cps-wap-browser/motorola-cps-wap-screenshots.svg` |

### 1.1 Evidenzklassen

Diese Dokumentation verwendet folgende Statusbegriffe bewusst streng:

- **Idee**: im Gespräch vorgeschlagene Möglichkeit ohne Beschluss oder Umsetzungsnachweis.
- **Beschlossen/geplant**: im Chat als gewünschter Zielzustand festgelegt; noch kein Implementierungsnachweis.
- **Implementiert**: durch aktuell gelesenen Repository-Quelltext oder Konfiguration belegt.
- **Getestet**: ein konkreter Test wurde nachweislich ausgeführt und ein Ergebnis liegt vor.
- **Im Betrieb bestätigt**: Verhalten wurde am realen Funkgerät beziehungsweise On-Air mit der konkreten CPS-Konfiguration nachgewiesen.
- **Historisch/überholt**: frühere Dokumentation oder Aussage, die gegenüber dem heutigen Quellstand eingeschränkt oder ersetzt ist.

Für diesen Chat ist besonders wichtig: **Es gibt keinen im Chat dokumentierten erfolgreichen Browser-On-Air-Test nach dem CPS-Setup.** Die Konfiguration wurde hergeleitet und vorgeschlagen, aber nicht durch einen anschließenden Live-Log oder einen sichtbaren Seitenabruf bestätigt.

### 1.2 Zugängliche Quellen und Lücken

Vollständig zugänglich waren:

1. die aktuelle Nutzerfrage mit sechs Motorola-CPS-Screenshots;
2. die darauf folgende technische Antwort mit vorgeschlagenem CPS-Mapping;
3. der aktuelle Archivierungsauftrag;
4. der hochgeladene ZIP-Snapshot `netcore-tetra-swmi(7).zip`;
5. die über GitHub zugänglichen Dateien des heutigen `main`-Standes;
6. die bereits vorhandene Archivindexdatei `Docs/archive/README.md`.

Nicht verfügbar waren ein ursprünglicher Chattitel, eine Chat-ID/ein Chatlink sowie ein nachfolgender On-Air-Test. Der historisch vom Nutzer genannte GitHub-Branch `swmi` existierte bei der Archivprüfung nicht mehr und konnte daher nicht als Remote-Branch erneut geprüft werden. Der hochgeladene ZIP-Snapshot trägt jedoch den entsprechenden Namen und enthält die relevanten WAP/SNDCP-Dateien.

Der ZIP-Inhalt wurde **themenbezogen** auf WAP/SNDCP, Paketdaten und Konfiguration untersucht; es wird kein vollständiges Audit aller 1.672 Einträge behauptet.

### 1.3 Chatbilder

Sechs originale PNG-Screenshots waren im Chat sichtbar. Die vorhandene GitHub-Schreibschnittstelle konnte für diesen Auftrag UTF-8-Dateien direkt committen, aber die lokalen PNG-Binärdateien nicht als Repository-Dateien übernehmen. Deshalb wurde eine textnative SVG-Archivdarstellung mit allen sechs sichtbaren CPS-Panels unter `Docs/archive/assets/...` angelegt. Sie ist **kein pixelidentisches Original**, sondern eine nachvollziehbare visuelle Transkription.

Zur Integritätsreferenz der tatsächlich sichtbaren Original-PNGs wurden lokal folgende Werte ermittelt:

| Nr. | Inhalt | Originalgröße | Bytegröße | SHA-256 |
|---|---|---:|---:|---|
| 1 | Paketdaten/IP | 579×286 | 20.661 | `4aa2a1a91f705090b8b0a96eb00eb78b4a9273686b63186eefec2d4c545dcf2c` |
| 2 | interne Authentisierung | 414×126 | 6.592 | `00f82339a22f5ad2aa618f2bb84a39630ce739b1d5f46d2c4c8b45b64cdad720` |
| 3 | WAP allgemein | 612×198 | 13.736 | `62d6e7bb216197575a7330000f254e3f9e9797944595d957fc6ea02dced91c3d` |
| 4 | WAP Proxy | 471×154 | 11.699 | `862f2ea0660c810d333c4270c188ab471509f68c9cbed704878b8ce313cbc167` |
| 5 | WAP Träger | 528×133 | 9.077 | `5ba4999752a06cd6bca86b18dc91689dde6e1b102c52d6a94d2bd1da22fc6cf9` |
| 6 | WAP Access Point | 499×198 | 12.438 | `ccc0361a77c3633d12be70e506fe404102b6d1badac72938c017020c5e5cb0de` |

Archivdarstellung: [Motorola-CPS-WAP-Screenshots](assets/2026-10-05_motorola-cps-wap-browser/motorola-cps-wap-screenshots.svg).

---

## 2. Ziel und Ausgangslage

Der Nutzer hatte in NetCore-Tetra bereits WAP-Unterstützung integriert und wollte wissen, **welche Werte in den einzelnen Motorola-CPS-Masken einzutragen sind, damit der integrierte Funkgerätebrowser über TETRA-Paketdaten funktioniert**.

Die sichtbaren CPS-Seiten deckten sechs Bereiche ab:

1. Paketdaten-Grundeinstellungen und IP-Adressierung;
2. interne Benutzerauthentisierung;
3. allgemeine WAP-Browserparameter;
4. WAP-Proxy/Gateway;
5. WAP-Träger;
6. Access-Point-Tabelle.

Auf NetCore-Seite war im historischen WAP-Profil bereits ein lokaler WAP-Endpunkt vorgesehen:

- IPv4-Adresse: `10.0.0.1`
- UDP-Port: `9200`
- dynamischer Teilnehmerpool: `10.0.0.2` bis `10.0.0.254`
- WML: `/status.wml`
- XHTML: `/status.xhtml`
- Root: `/`
- WTP/WSP über IPv4/UDP
- SNDCP/PDP-Aktivierung mit Motorola-/DIMETRA-kompatiblem CHAP-Erfolgspfad.

Die Aufgabe dieses Chats war **keine neue Implementierung des WAP-Stacks**, sondern die Zuordnung der vorhandenen NetCore-Eigenschaften zu den Motorola-CPS-Feldern.

---

## 3. Historischer Chatstand: vorgeschlagenes Motorola-CPS-Mapping

### 3.1 Paketdaten / IP

Der erste Screenshot zeigte bereits folgende Werte:

| CPS-Feld | Sichtbarer Ausgangswert | Im Chat empfohlener Zielwert | Status |
|---|---|---|---|
| Benutzerauthentisierung | aktiviert | aktiviert lassen | **beschlossen/geplant**, nicht On-Air bestätigt |
| Authentifikatorname | `DIMETRA_P` | `DIMETRA_P` | **beschlossen/geplant** |
| Nur Daten | aus | aus | **beschlossen/geplant** |
| Sprache & Daten | an | an | **beschlossen/geplant** |
| Nur Sprache | aus | aus | **beschlossen/geplant** |
| Standard-Paketdatenmodus | Sprache & Daten | Sprache & Daten | **beschlossen/geplant** |
| PD-Seitenperioden-Aktualisierungen | 1000 ms | 1000 ms | vorhandener CPS-Wert; **nicht separat validiert** |
| IP-Warteschlangen-Timeout | aus | zunächst aus | **beschlossen/geplant** |
| Max. IP-Warteschlangenzeit | 5 s | 5 s | vorhandener CPS-Wert; **nicht separat validiert** |
| Dynamische IP-Adresse anfordern | an | **an** | passt zum NetCore-Pool; **beschlossen/geplant** |
| Statische IP-Adresse | `0.0.0.0` | `0.0.0.0` bei dynamischer Adressierung | **beschlossen/geplant** |

Die zentrale Entscheidung ist die **dynamische IPv4-Anforderung**. NetCore stellt einen dynamischen SNDCP-Pool bereit; DHCP ist auf diesem Roh-IP-Bearer nicht der Mechanismus der Adressvergabe. Die IP wird im PDP-Kontext ausgehandelt/zugewiesen.

### 3.2 Interne Benutzerauthentisierung

Der zweite Screenshot zeigte:

- Protokolltyp: **CHAP**
- Benutzername: leer
- Kennwort: leer

Der Chat legte CHAP als kompatiblen Weg fest. NetCore besitzt einen CHAP-Success-Pfad für die Motorola-/DIMETRA-Kompatibilität.

Eine frühere Antwort in diesem Chat nannte zusätzlich beispielhafte Zugangsdaten. Diese Werte waren **nicht durch Repository oder Screenshot belegt** und werden in dieser Abschlussdokumentation bewusst **nicht als Zugangsdaten übernommen**. Sie sind weder als Produktiv-Credential noch als technischer Standard zu behandeln. Der Nutzer hat für das Archiv ausdrücklich vorgegeben, keine Passwörter oder Tokens zu übernehmen.

Belastbare Aussage:

- **CHAP als Protokolltyp:** geplant und durch NetCore-Kompatibilitätscode plausibel.
- **konkrete CPS-Benutzerkennung/Kennwort:** in diesem Chat nicht belastbar festgelegt; nicht archiviert.
- Historische `Docs/WAP_INTEGRATION.md` beschreibt CHAP als Kompatibilitätspfad und sagt, dass der Hash dort nicht als eigene Zugangskontrolle validiert wird.
- Der heutige Quellstand enthält weiterhin `PPP_PROTO_CHAP = 0xC223`, `CHAP_CODE_SUCCESS = 3` und erzeugt bei vorhandenem CHAP-Identifier ein CHAP-Success-PCO-Element. Das bestätigt den Mechanismus, ersetzt aber keinen vollständigen Security-Audit der aktuellen PCO-Verarbeitung.

### 3.3 WAP allgemein

Empfohlen wurde:

| CPS-Feld | Empfehlung | Status / Begründung |
|---|---|---|
| Cache aktiviert | an | unkritische Browseroption; **nicht On-Air getestet** |
| Max. Zeit ununterbrochene WAP-Datensitzung | `0` | Screenshotwert beibehalten; Semantik nicht im NetCore-Code definiert |
| Homepage-Adresse | `http://10.0.0.1:9200/status.wml` | **beschlossen/geplant**, nicht Live bestätigt |
| Alternative Homepage | `http://10.0.0.1:9200/status.xhtml` | **Idee/Alternative** |
| UAProf-Adresse | leer | **beschlossen/geplant** |
| vertrauenswürdige Domäne | leer | **beschlossen/geplant** |
| Initiator Client ID | leer | **beschlossen/geplant** |
| Client-ID-Stil | OpenWave | Screenshotwert beibehalten; passt zum historischen Openwave-Zielprofil |

Wichtige Präzisierung gegenüber einer leicht missverständlichen Lesart der URL: Der NetCore-WAP-Dienst ist **kein normaler HTTP/TCP-Webserver auf Port 9200**. Die URL ist ein CPS-/Browser-Zielstring. Der aktuelle `wap_ip.rs` beschreibt ausdrücklich einen **WAP 1.x/2.0 WTP/WSP adapter over IPv4/UDP**. Die historische WAP-Dokumentation schließt einen TCP/HTTP-Kompatibilitätsendpunkt ausdrücklich aus.

Damit bedeutet die Homepage-Zeile nicht: „Browser öffnet TCP/HTTP zu Port 9200“. Der Datenpfad ist WAP/WSP/WTP über UDP, eingebettet in IPv4 und SNDCP.

### 3.4 WAP Proxy/Gateway

Der vierte Screenshot stand initial auf:

- Proxy-IP-Adresse `0.0.0.0`
- Remote-Anschluss `0`
- SAR erzwingen: an
- SAR-Gruppengröße: `3`
- Trägerindex: `0`

Vorgeschlagen wurde:

| Feld | Zielwert |
|---|---|
| Proxy-IP-Adresse | **`10.0.0.1`** |
| Remote-Anschluss | **`9200`** |
| SAR erzwingen | zunächst **aktiviert** |
| SAR-Gruppengröße | `3` |
| Trägerindex | `0` |

Die technisch wichtigste Korrektur war der Port: **NetCore verwendet 9200**, nicht einen eventuell aus anderen WAP-Umgebungen bekannten Standardwert. Der Endpunkt und der Zielport werden durch NetCore selbst definiert; falscher Port würde am WAP-Endpunkt als falsches Ziel verworfen.

Der SAR-Hinweis war **nur ein Startwert/Troubleshooting-Vorschlag**. Es gibt in diesem Chat keinen On-Air-Nachweis, dass die konkrete Motorola-Firmware mit „SAR erzwingen = an, Gruppengröße 3“ optimal funktioniert. Falls Connect/Reply trotz erfolgreichem PDP-Kontext hängen bleibt, sollte diese Option gezielt gegengeprüft werden.

### 3.5 WAP-Träger

Der sichtbare Träger war:

- Trägername: `TETRA_PACKET`
- B-Typ: `1`
- WDP B-Typ: `0`
- Max. Anforderungszeit: `45` s

Der Chat empfahl, diese Werte unverändert zu lassen und den Proxy mit **Trägerindex 0** auf diesen Eintrag zu verweisen.

Status: **beschlossen/geplant**, aber nicht durch einen realen erfolgreichen Browseraufruf in diesem Chat bestätigt.

### 3.6 Access Point

Der sechste Screenshot zeigte eine leere AP-Tabelle:

- AP-Zugriffstyp
- AP-Adresse
- AP-Authentisierungstyp
- AP-Benutzername
- AP-Kennwort
- AP-Skript

Der Chat entschied: **für den lokalen NetCore-WAP/SNDCP-Pfad zunächst leer lassen**. Die Adresse `10.0.0.1` gehört nicht als klassischer APN/Access-Point in diese Tabelle, sondern in das WAP-Gateway/Proxy-Ziel.

Status: **beschlossen/geplant**, nicht im Livebetrieb bestätigt.

---

## 4. NetCore-Architektur und Datenpfad

### 4.1 Lokaler WAP-Pfad

Der angestrebte Datenweg ist:

```text
Motorola MS
  |
  | TETRA TMO
  | SNDCP / PDP context
  v
NetCore-Tetra BS / SwMI
  |
  | dynamische IPv4-Zuweisung 10.0.0.x
  | PDCH / Packet Data
  v
IPv4 / UDP
  |
  | Ziel 10.0.0.1:9200
  v
WTP / WSP Adapter
  |
  +--> /status.wml
  +--> /status.xhtml
  +--> /
```

Der WAP-Pfad hängt von mehreren Ebenen gleichzeitig ab:

1. Die Zelle muss SNDCP-/Packet-Data-Fähigkeit korrekt ankündigen.
2. Das Funkgerät muss Packet Data und Browser/WAP zulassen.
3. PDP-Aktivierung muss akzeptiert werden.
4. Es muss ein PDCH/Bearer bereitgestellt werden.
5. IP-Paket muss mit korrekter Teilnehmeradresse zum lokalen WAP-Endpunkt gelangen.
6. UDP-Ziel muss `10.0.0.1:9200` sein.
7. WTP/WSP muss von NetCore erkannt und beantwortet werden.
8. Das Funkgerät muss die zurückkommende WSP-Antwort rendern.

Ein Fehler auf einer Ebene kann sich am Gerät nur als „Browser geht nicht“ äußern. Deshalb ist eine stufenweise Logdiagnose zwingend sinnvoll.

### 4.2 Historisches NetCore-WAP-Profil

Die historische `Docs/WAP_INTEGRATION.md` im ZIP und im heutigen Repository nennt:

- `sndcp_service = true`
- `advanced_link = true`
- `[cell_info.wap_ip] enabled = true`
- Adresse `10.0.0.1`
- Port `9200`
- Poolpräfix `10.0.0`
- Hosts `2..254`
- statische IPv4 optional erlaubt
- Root-/Status-/WML-Pfade
- Request-Limit `1024` Byte
- TTL `32`
- MTU-Profil historisch 576 Byte
- WTP Invoke/Result/ACK/Abort
- WSP Connect/Connect-Reply, Resume/Resume-OK und GET
- Openwave-taugliche WML-/XHTML-Ausgabe.

Diese Dokumentation ist als historische WAP-Baseline weiterhin sehr nützlich.

### 4.3 Heutiger Quellstand ist weiter als die alte WAP-Dokumentation

Die WAP-Dokumentation vom 21.07.2026 enthält inzwischen **überholte Einschränkungen**. Dort steht unter anderem:

- nur lokaler Statusdienst, kein allgemeines Internet-Gateway;
- genau ein aktiver PDCH auf Hauptcarrier TS2;
- noch kein PDCH auf Secondary Carrier;
- keine vollständige SNDCP-Suite mit RECONNECT/MODIFY;
- keine Fragmentierung.

Der am 05.10.2026 geprüfte `main`-Stand zeigt dagegen bereits zusätzliche Infrastruktur:

- `[cell_info.packet_data_gateway]` ist in `config.toml` vorhanden und aktiviert;
- Linux-TUN/Forwarding/NAT-Konfiguration ist vorhanden;
- `max_pdch_bearers = 0` steht für dynamische Kapazitätswahl;
- `reserved_voice_slots = 1`;
- `prefer_secondary_carrier = true`;
- DNS-Server werden konfiguriert;
- `sndcp_bs.rs` importiert und verwaltet `PacketGateway`/`PacketGatewayConfig`;
- `SECONDARY_CARRIER_HINT` ist vorhanden;
- `Reconnect`, `Modify`, QoS, Fragmentierung/Reassembly und weitere Paketdatenbausteine sind im aktuellen SNDCP-Code sichtbar.

Daraus folgt:

> Die alte Datei `Docs/WAP_INTEGRATION.md` beschreibt zuverlässig die ursprüngliche lokale WAP-Integration, aber **nicht mehr den gesamten heutigen Packet-Data-Funktionsumfang**.

Diese heutige Erweiterung ändert die für den lokalen Motorola-Browser wesentlichen Werte `10.0.0.1:9200` nicht. Sie bedeutet lediglich, dass die alte Aussage „NetCore hat keinen allgemeinen IP-Gateway-Pfad“ nicht mehr als aktueller Gesamtzustand verwendet werden darf.

Ob sämtliche neueren Packet-Data-Funktionen am konkreten Motorola-Zielgerät bereits On-Air abgenommen wurden, wurde für diese Archivierung **nicht** nachgewiesen.

---

## 5. Relevante Konfigurationen und technische Parameter

### 5.1 Heutiger `main`-Stand

Am geprüften Commit `7137e0dd69877e1b604bf89148fd8b6b590c1a97` enthält `config.toml` unter anderem:

```toml
[cell_info]
sndcp_service = true
advanced_link = true

[cell_info.wap_ip]
enabled = true
address = "10.0.0.1"
port = 9200
response_ttl = 32
dynamic_pool_prefix = "10.0.0"
dynamic_pool_first_host = 2
dynamic_pool_last_host = 254
allow_static_ipv4 = true
accept_empty_probe = true
accept_root_path = true
accept_status_path = true
accept_status_wml_path = true
max_request_payload_bytes = 1024
assume_pdch_ready_after_data_transmit = false

pdu_priority_max = 4
ready_timer_code = 8
standby_timer_code = 4
response_wait_timer_code = 7
mtu_code = 2
network_default_data_priority = 4
max_contexts_per_issi = 4
max_total_contexts = 64
strict_source_address = true
```

Zusätzlich ist heute ein allgemeiner Packet-Data-Gateway-Block vorhanden:

```toml
[cell_info.packet_data_gateway]
enabled = true
interface_name = "ntetra0"
prefix_len = 24
auto_configure = true
enable_ipv4_forwarding = true
managed_forwarding = true
allow_unsolicited_inbound = false
nat_mode = "masquerade"
firewall_backend = "auto"
dns_servers = ["1.1.1.1", "9.9.9.9"]
channel_capacity = 256
max_pdch_bearers = 0
reserved_voice_slots = 1
prefer_secondary_carrier = true
```

Die DNS-Adressen sind hier reine Repository-Konfiguration; für den lokalen WAP-Statuspfad auf `10.0.0.1` sind sie nicht erforderlich.

### 5.2 WAP/WSP-Protokollbausteine

Der heutige `crates/tetra-entities/src/sndcp/wap_ip.rs` enthält unter anderem:

- WTP Invoke, ACK und Abort;
- WSP Connect, Reply, Resume und GET;
- WML-Content-Type `0x88`;
- XHTML-Content-Type `0xC5`;
- eine WAP-Endpunktstruktur aus IPv4-Adresse, Port und TTL;
- Pfad-/Payload-Policy;
- UIntVar-Verarbeitung und Connect-Capability-Verhandlung.

Damit ist der WAP-Stack nicht nur in Dokumentation beschrieben, sondern **im aktuellen Quellcode implementiert**.

### 5.3 CHAP/PCO

Der heutige `sndcp_bs.rs` enthält:

- `PPP_PROTO_CHAP = 0xC223`
- `CHAP_CODE_SUCCESS = 3`
- PCO-Verarbeitung
- Erzeugung eines CHAP-Success-Elements bei erkanntem CHAP-Identifier.

Das ist ein **Implementierungsnachweis** für den Kompatibilitätspfad. Es ist kein Beleg dafür, dass beliebige CPS-Credentials als echte Zugangskontrolle ausgewertet werden. Die alte WAP-Dokumentation sagt ausdrücklich, dass der Hash im ursprünglichen Profil nicht als eigene Zugangskontrolle validiert wurde. Diese Aussage sollte für den heutigen erweiterten Packet-Data-Code bei Bedarf noch separat vollständig geprüft werden.

---

## 6. Hochgeladener ZIP-Snapshot `netcore-tetra-swmi(7).zip`

Der Snapshot wurde lokal geöffnet und enthält die für diesen Chat relevanten Dateien:

- `netcore-tetra-swmi/Docs/WAP_INTEGRATION.md`
- `netcore-tetra-swmi/Docs/wap-port-spec.md`
- `netcore-tetra-swmi/config.toml`
- `netcore-tetra-swmi/config.toml.fallback`
- `netcore-tetra-swmi/crates/tetra-entities/src/sndcp/sndcp_bs.rs`
- `netcore-tetra-swmi/crates/tetra-entities/src/sndcp/wap_ip.rs`

Einige lokale SHA-256-Werte des Snapshots:

| Datei | SHA-256 |
|---|---|
| `Docs/WAP_INTEGRATION.md` | `9b5ce9385ae40617a99c518100e2818a450f3c5aa8c8caf810253934a48d2bdf` |
| `config.toml` | `cdd8e334f89118542edfd37bd426b6c72502a04dd9e970f6c398696460769879` |
| `sndcp/wap_ip.rs` | `3c43ac55057eb6d70dd11cb72c5345900c2eaa71d5bd57600da12a0c7590e7bb` |
| `sndcp/sndcp_bs.rs` | `fe774677902ac1283c96d5ef86ba2ec538cb4848338d0e3d2440a8e3931b363c` |

Der ZIP-Snapshot ist bereits **weiter entwickelt als seine eigene alte WAP-Dokumentation**: Seine `config.toml` enthält ebenfalls `packet_data_gateway`, `max_pdch_bearers = 0` und `prefer_secondary_carrier = true`; sein `sndcp_bs.rs` enthält `PacketGateway`, `SECONDARY_CARRIER_HINT` und CHAP-Success. Das ist ein weiterer Hinweis darauf, dass `Docs/WAP_INTEGRATION.md` als historische Baseline zu lesen ist.

Der ZIP-Snapshot wurde in diesem Archiv **nicht vollständig in das Repository kopiert**, da der Auftrag Änderungen ausschließlich unter `Docs/archive/` erlaubt und eine technische Abschlussdokumentation, nicht die Aufnahme eines kompletten Quellbaums, verlangt.

---

## 7. Diagnostik und erwartete Zustandsfolge

Die historische WAP-Dokumentation erwartet grundsätzlich eine Sequenz wie:

```text
SYSINFO advertising: sndcp_service=true advanced_link=true ...
SNDCP: PDP context accepted ISSI=... NSAPI=... IPv4=... CHAP=true
SNDCP: <- type=6 ...
SNDCP: <- type=4 ...
```

In der damaligen Interpretation bedeuten diese Stufen grob:

1. **SYSINFO**: die Zelle kündigt Packet Data / Advanced Link an.
2. **PDP accepted**: SNDCP-Kontext und IPv4-Adresse wurden ausgehandelt.
3. **type 6**: Data-Transmit-Anforderung/Bearer-Aufbau.
4. **type 4**: SN-UNITDATA mit IP-/WAP-Nutzdaten.

Im Chat wurde als Diagnosebefehl vorgeschlagen:

```bash
sudo journalctl -u tetra -f | grep -Ei 'SNDCP|PDP|PDCH|WAP|CHAP'
```

Status dieses Befehls: **nur vorgeschlagen**, nicht im zugänglichen Chat ausgeführt.

### 7.1 Empfohlene Diagnose-Reihenfolge für die Fortsetzung

Nicht sofort „WAP kaputt“ diagnostizieren, sondern stufenweise prüfen:

1. Funkgerät registriert/campt normal?
2. kündigt die TBS `sndcp_service` und `advanced_link` an?
3. kommt eine PDP-ACTIVATE-Anforderung?
4. wird der PDP-Kontext akzeptiert und welche IPv4 bekommt das MS?
5. ist CHAP im PCO sichtbar und wird Success zurückgegeben?
6. wird ein PDCH/Bearer zugewiesen?
7. kommt IPv4/UDP vom MS?
8. ist Ziel wirklich `10.0.0.1:9200`?
9. klassifiziert `wap_ip.rs` die Nutzlast als WTP/WSP?
10. wird Connect-Reply beziehungsweise GET-Reply erzeugt?
11. wird Downlink-SN-UNITDATA an das richtige MS zurückgegeben?
12. rendert der Motorola-Browser WML/XHTML?

Diese Reihenfolge trennt CPS-, SNDCP-, Radio-Ressourcen-, IP- und WAP-Probleme sauber voneinander.

---

## 8. Tests und Ergebnisse

### 8.1 Im Chat tatsächlich ausgeführt

Für die konkrete Motorola-CPS-Konfiguration: **keine nachgewiesenen Live-Tests**.

Es liegt kein Log vor, das nach dem Eintragen der Werte:

- einen erfolgreichen PDP-Kontext,
- einen erfolgreichen WSP Connect,
- einen GET auf `/status.wml`,
- oder die gerenderte Statusseite auf dem Motorola-Gerät

bestätigt.

### 8.2 Repository-/Dokumentationsstand

Die historische WAP-Dokumentation nennt folgende vorgesehene Tests:

```bash
cargo test -p tetra-core timeslot_alloc
cargo test -p tetra-entities sndcp --features runtime
cargo test -p tetra-config
```

sowie einen vollständigen Release-Build.

Diese Befehle sind im Repository dokumentiert, wurden **für diesen Archivauftrag nicht erneut ausgeführt**. Es gibt daher hier keinen neuen Build-/Testnachweis.

Der heutige Quellstand wurde über GitHub gelesen, aber nicht lokal kompiliert oder auf RF-Hardware ausgeführt.

### 8.3 Grenzen der Aussagekraft

- Eine syntaktisch plausible CPS-Konfiguration ist kein On-Air-Nachweis.
- Die Existenz von Quellcode ist kein Beweis dafür, dass die aktuell auf der Basisstation laufende Binary denselben Commit enthält.
- Ein historischer Testvektor ist kein Nachweis für eine konkrete Motorola-Firmware/CPS-Version.
- Die Motorola-CPS-Feldsemantik wurde in diesem Archiv nicht gegen eine exakt passende Motorola-Programmierhandbuch-Version neu validiert.

---

## 9. Frühere oder inzwischen überholte Aussagen

### 9.1 „Nur lokaler WAP-Statusdienst, kein Internet-Gateway“

**Historisch korrekt für die erste WAP-Integration, heute als Gesamtaussage überholt.**

Heute existiert zusätzlich ein allgemeiner Packet-Data-Gateway-Pfad mit TUN, Forwarding und optional NAT. Für den lokalen Browser-Endpunkt bleibt `10.0.0.1:9200` dennoch gültig.

### 9.2 „Genau ein PDCH auf Main-Carrier TS2“

**Historische WAP-MVP-Grenze, nicht mehr als allgemeiner heutiger Paketdatenstand verwenden.**

Die aktuelle Konfiguration enthält dynamische PDCH-Kapazität und bevorzugten Secondary Carrier. Eine vollständige On-Air-Abnahme dieses erweiterten Modells wurde hier nicht durchgeführt.

### 9.3 „Keine RECONNECT/MODIFY/Fragmentierung“

**Historische Einschränkung.**

Im heutigen SNDCP-Quellstand sind entsprechende Protokoll-/Fragmentierungsbausteine sichtbar. Diese Archivprüfung bewertet nicht deren vollständige Conformance.

### 9.4 Konkrete CHAP-Benutzerkennung und Kennwort

Die frühere Chatantwort nannte exemplarische Werte. Sie waren nicht aus dem Repository abgeleitet und sind **verworfen als technische Festlegung**. Keine Passwörter werden archiviert.

### 9.5 WAP-Port 9201/9203 als möglicher Motorola-Standard

Für dieses Projekt ist entscheidend: **NetCore erwartet 9200.** Allgemeine WAP-Defaultports dürfen den projektspezifischen Endpunkt nicht überschreiben.

---

## 10. Fehlerbilder, Diagnose und mögliche Ursachen

### 10.1 Browser startet, aber keine Seite erscheint

Mögliche Ursachen:

- Paketdaten-/Browserfeature am Gerät nicht freigeschaltet;
- falscher Packet-Data-Modus;
- SNDCP/Advanced-Link nicht angekündigt;
- kein PDP-Kontext;
- falsche Proxy-IP;
- falscher Remote-Port;
- falscher Bearer-Index;
- WSP-Verhandlung scheitert;
- SAR-Verhalten der Firmware passt nicht;
- Downlink-Paket erreicht das MS nicht.

**Noch offen**, bis Live-Logs vorliegen.

### 10.2 PDP-Kontext kommt nicht zustande

Prüfen:

- `sndcp_service = true`
- `advanced_link = true`
- `wap_ip.enabled = true`
- dynamische IPv4-Anforderung am MS
- CHAP/PCO-Auswertung
- Poolkapazität
- Kontextlimits
- statische/dynamische Adressart.

### 10.3 PDP erfolgreich, aber keine IP-/WAP-Nutzdaten

Prüfen:

- SN-DATA TRANSMIT / PDCH-Zuteilung;
- Timeslot-/Carrier-Ressourcen;
- aktuelle Multi-PDCH-/Voice-Reservation;
- Bearerindex des Motorola-WAP-Profils;
- ob der Browser wirklich den eingestellten WAP-Proxy verwendet.

### 10.4 UDP erreicht NetCore, aber WAP wird verworfen

Der aktuelle WAP-Code kennt explizit Fehler wie:

- WrongDestination
- WrongPort
- PayloadTooLarge
- UnsupportedPayload
- UnsupportedPath
- NoResponseRequired.

Besonders relevant sind Zieladresse und **Port 9200**.

---

## 11. Betriebs- und Sicherheitsbetrachtung

Der WAP-Browserpfad ist ein Datenpfad aus dem Funknetz in die IP-Verarbeitung der Basisstation. Deshalb sollten die historischen Open-Lab-/Kompatibilitätsannahmen nicht automatisch als Produktionssicherheit interpretiert werden.

Wichtige Punkte:

- CHAP-Success-Kompatibilität ist nicht gleich starke Zugangskontrolle.
- Der WAP-Endpunkt sollte nur von registrierten/zulässigen SNDCP-Kontexten erreichbar sein.
- `strict_source_address = true` ist im heutigen Profil sinnvoll und aktiv.
- Das allgemeine Packet-Data-Gateway hat eine eigene Firewall-/Forwarding-Verantwortung.
- `allow_unsolicited_inbound = false` ist im aktuellen Beispiel eine wichtige Default-Grenze.
- WAP-Statusdaten sollten keine Geheimnisse, Schlüssel oder unnötige Betriebsdetails offenlegen.
- Diese Archivdatei enthält absichtlich keine Passwörter, Tokens oder privaten Schlüssel.

---

## 12. Erreichter Stand nach diesem Chat

### Implementiert im Repository

- SNDCP/PDP-Stack mit IPv4-Profil.
- WAP WTP/WSP über IPv4/UDP.
- lokaler WAP-Endpunkt `10.0.0.1:9200`.
- WML-/XHTML-Statuspfade.
- dynamischer Adresspool.
- CHAP-Success-Kompatibilitätspfad.
- SNDCP-/Advanced-Link-Konfiguration.
- heute zusätzlich allgemeines Packet-Data-Gateway und erweiterte PDCH-/SNDCP-Bausteine.

### Beschlossen/geplant im Chat

- Motorola auf dynamische IP-Anforderung.
- Sprache & Daten als Standard-Paketdatenmodus.
- CHAP als internes Authentisierungsprotokoll.
- WAP-Gateway `10.0.0.1`.
- Remote-Port `9200`.
- Startseite bevorzugt `/status.wml`, XHTML als Alternative.
- Proxy nutzt Trägerindex 0 / `TETRA_PACKET`.
- AP-Tabelle für diesen lokalen Pfad zunächst leer.
- SAR zunächst eingeschaltet lassen und bei Problemen gezielt gegentesten.

### Getestet

- **Nicht in diesem Chat:** die konkrete CPS-Konfiguration am realen Motorola-Gerät.
- **Nicht in diesem Archivauftrag:** Rust-Build, Cargo-Test oder RF-Test.
- Repository und ZIP wurden statisch gelesen und gegeneinander eingeordnet.

### Im Betrieb bestätigt

- **Nichts aus dem konkreten Motorola-Browser-Setup ist durch diesen Chat als im Betrieb bestätigt belegt.**

---

## 13. Offene Aufgaben und Roadmap-Kandidaten

### Priorität A – direkt als Nächstes

1. **CPS-Werte am Zielgerät setzen und Codeplug schreiben.**
2. Funkgerät komplett neu starten.
3. Registrierung und Packet-Data-Service prüfen.
4. Parallel TBS-Logs für `SNDCP|PDP|PDCH|WAP|CHAP` beobachten.
5. Zugewiesene `10.0.0.x`-Adresse dokumentieren.
6. WSP-Connect und GET auf `/status.wml` nachweisen.
7. Screenshot des erfolgreich gerenderten Browsers und passenden Logausschnitt archivieren.

### Priorität B – bei Fehlschlag

8. SAR an/aus gegentesten.
9. Homepage `/status.wml` gegen `/status.xhtml` vergleichen.
10. Bearerindex und CPS-Featurefreigabe prüfen.
11. CHAP-PCO mitsamt Identifier und Success im Log instrumentieren, ohne Geheimnisse zu loggen.
12. WrongDestination/WrongPort/UnsupportedPayload explizit in der Diagnose sichtbar machen.

### Priorität C – Dokumentation/Codehygiene

13. `Docs/WAP_INTEGRATION.md` aktualisieren, damit die inzwischen vorhandene Packet-Data-Gateway-/Multi-PDCH-Entwicklung nicht mehr durch alte MVP-Grenzen missverständlich beschrieben wird.
14. Motorola-CPS-Profil als **separate, modell-/firmwarebezogene Dokumentation** anlegen, sobald ein Gerät erfolgreich getestet ist.
15. Die genaue CPS-/Firmware-/Browser-Version des Zielgeräts erfassen.
16. Für CHAP klar dokumentieren, ob und wo aktuelle Credentials tatsächlich validiert werden oder ob der Pfad weiterhin reine Kompatibilitätsantwort ist.
17. WAP und allgemeines IP-Gateway in der Dokumentation sauber trennen.

### Kleine Nebenideen aus dem Chat

- WML als konservative Defaultseite für ältere OpenWave-Clients beibehalten.
- XHTML als zweites Profil testen.
- Bei späterem Internetzugang nicht den lokalen WAP-Proxy mit dem allgemeinen NAT/TUN-Gateway verwechseln.
- Browser-/Packet-Data-Featureflags und Menüfreigaben in eine künftige Motorola-Checkliste aufnehmen.

---

## 14. Konkreter Fortsetzungsplan

Empfohlene Reihenfolge für den nächsten technischen Chat:

```text
1. Motorola-Modell + Firmware + CPS-Version notieren
2. Codeplug mit den hier beschriebenen Werten schreiben
3. TBS-Log starten
4. MS neu starten und registrieren
5. Browser öffnen
6. Log ab PDP ACTIVATE sichern
7. bei Erfolg: WSP GET/Reply + Browser-Screenshot
8. bei Fehler: letzte erreichte Schicht bestimmen
9. nur diese Schicht korrigieren
10. erfolgreichen Parametersatz als getestetes Geräteprofil dokumentieren
```

Ziel ist, den nächsten Stand von **„beschlossen/geplant“** auf **„getestet“** und anschließend **„im Betrieb bestätigt“** zu heben.

---

## 15. Relevante Repository-Dateien und stabile Links

Heutiger geprüfter `main`-Commit: [`7137e0dd69877e1b604bf89148fd8b6b590c1a97`](https://github.com/JanHG98/netcore-tetra/commit/7137e0dd69877e1b604bf89148fd8b6b590c1a97)

- [`Docs/WAP_INTEGRATION.md`](https://github.com/JanHG98/netcore-tetra/blob/7137e0dd69877e1b604bf89148fd8b6b590c1a97/Docs/WAP_INTEGRATION.md) – historische WAP-MVP-Dokumentation.
- [`Docs/wap-port-spec.md`](https://github.com/JanHG98/netcore-tetra/blob/7137e0dd69877e1b604bf89148fd8b6b590c1a97/Docs/wap-port-spec.md) – Wire-/Port-Spezifikation.
- [`config.toml`](https://github.com/JanHG98/netcore-tetra/blob/7137e0dd69877e1b604bf89148fd8b6b590c1a97/config.toml) – aktuelles Beispielprofil für SNDCP, WAP und Packet-Data-Gateway.
- [`crates/tetra-entities/src/sndcp/wap_ip.rs`](https://github.com/JanHG98/netcore-tetra/blob/7137e0dd69877e1b604bf89148fd8b6b590c1a97/crates/tetra-entities/src/sndcp/wap_ip.rs) – WTP/WSP-Adapter.
- [`crates/tetra-entities/src/sndcp/sndcp_bs.rs`](https://github.com/JanHG98/netcore-tetra/blob/7137e0dd69877e1b604bf89148fd8b6b590c1a97/crates/tetra-entities/src/sndcp/sndcp_bs.rs) – PDP-Kontexte, CHAP/PCO, Gateway- und Bearerlogik.
- [Frühere thematisch breitere WAP-/SNDCP-Archivdokumentation](2026-10-03_wap-sndcp-ip-gateway-multi-pdch-und-control-room.md) – anderer Chat, daher **nicht überschrieben**.

### Relevante ETSI-Quellen aus dem Projektkontext

Im Projekt sind unter anderem ETSI EN 300 392-2 (Air Interface) und EN 300 392-5 (PEI) als Referenzen vorhanden. Für SNDCP/WAP ist insbesondere der Air-Interface-/SNDCP-Kontext relevant. Dieser Archivauftrag führt keine vollständige Neuauswertung der 1.445-seitigen Air-Interface-Norm durch.

---

## 16. Abschlussbewertung

Der Chat hat das **Motorola-CPS-seitige Anschlussstück** zwischen dem vorhandenen NetCore-SNDCP/WAP-Stack und einem realen Browserprofil festgelegt. Die zentrale technische Zuordnung ist belastbar:

```text
MS-Adresse:     dynamisch aus 10.0.0.2..254
WAP-Ziel:       10.0.0.1
Transport:      IPv4 / UDP / WTP / WSP
UDP-Port:       9200
Startpfad:      /status.wml
Alternative:    /status.xhtml
SNDCP:          aktiviert
Advanced Link:  aktiviert
Auth-Protokoll: CHAP-Kompatibilitätspfad
Bearer:         TETRA_PACKET / Index 0
AP-Tabelle:     für lokalen Pfad zunächst leer
```

Die **wichtigste verbleibende Lücke** ist nicht mehr die theoretische CPS-Zuordnung, sondern der reale End-to-End-Nachweis mit genau dem Motorola-Gerät und seiner Firmware. Erst danach sollten SAR, Browserpfad und etwaige CPS-Sonderfelder als endgültig „getestet“ dokumentiert werden.

Die heutige Repository-Prüfung zeigt außerdem, dass NetCore-Tetra beim Packet-Data-Unterbau inzwischen deutlich weiter ist als die ursprüngliche WAP-MVP-Dokumentation. Eine spätere Fortsetzung sollte deshalb nie pauschal die alten Grenzen „nur TS2 / kein Internet-Gateway / kein RECONNECT / keine Fragmentierung“ wieder einführen.
