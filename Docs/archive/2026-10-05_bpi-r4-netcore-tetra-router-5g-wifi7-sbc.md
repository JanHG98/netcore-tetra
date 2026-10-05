# Technische Abschlussdokumentation: Router-SBC für NetCore-TETRA – Banana Pi BPI-R4, 5G und Wi-Fi 7

> **Ergebnis dieses Chats:** Für einen künftigen NetCore-TETRA-Edge-/Basisstationsknoten wurde die Router-SBC-Klasse als besonders passend bewertet. Die zuletzt bevorzugte Ausbaurichtung ist ein **Banana Pi BPI-R4 mit 8 GB RAM**, ergänzt um ein **BPI-R4-NIC-BE14** als Wi-Fi-7-Access-Point-Modul, ein **Quectel RM520N-GL** als 5G-/LTE-WWAN-Modem und optional eine NVMe-SSD. Diese Auswahl ist eine **Planungs- und Beschaffungsempfehlung**, kein Nachweis einer Bestellung, Installation oder Inbetriebnahme. Ein BPI-R4 Pro bleibt als Ausbaualternative erhalten, wurde im Verlauf aber nicht mehr als zwingend beste Erstwahl betrachtet.
>
> **Repository-Abgleich am 05.10.2026:** Im aktuellen Hauptbranch ist die Idee eines mobilen Routers mit 5G, M.2-Funkmodul und Access Point bereits dokumentarisch erhalten. Für BPI-R4/R4-Pro werden dort Strombudget, Treiber, Antennen, AP-Betrieb und VPN-Übergänge ausdrücklich noch als offene Konkretisierungen geführt. Für die konkreten Produktnamen **RM520N**, **BE14**, **OpenWrt** und **wwan** ergab die heutige Code-/Dokumentensuche im Default-Branch keinen belastbaren Implementierungsnachweis. Der hier archivierte Hardware-Stack ist daher **nicht implementiert und nicht getestet**.

## 1. Metadaten

| Feld | Inhalt |
|---|---|
| Projekt | NetCore-Tetra |
| Thema | Router-SBC für Basisstation/Edge-Node mit Ethernet, Wi-Fi-AP, 5G-SIM, VPN und optional NVMe |
| Ursprünglicher Chattitel | Nicht verfügbar; im zugänglichen Verlauf wurde kein belastbarer Originaltitel geliefert. |
| Ursprünglicher Chatlink | Nicht verfügbar; es wird kein Link konstruiert. |
| Historischer Gesprächsstand | Die Hardware-/Preisdiskussion ist im sichtbaren Verlauf mit Stand **10.09.2026** gekennzeichnet; der Archivauftrag erfolgte am **05.10.2026**. |
| Erstellungsdatum dieser Zusammenfassung | **2026-10-05** |
| Zielrepository | JanHG98/netcore-tetra |
| Geprüfter Zielbranch vor Archivierung | **Archiving** |
| Eingangs-Commit Archiving | **d37658d8588a367d11e979bfd29b07a8a50f61ef** |
| Eingangs-Tree Archiving | **011b61e59aabdf8f7acffa634a3cdb950a935c60** |
| Zusätzlich geprüfter Default-Branch | **main** |
| Geprüfter main-Commit | **9116c15d645458f99e236712b67a1ad970432791** |
| Ablage | Docs/archive/2026-10-05_bpi-r4-netcore-tetra-router-5g-wifi7-sbc.md |
| Archivindex | Docs/archive/README.md |
| Änderungsumfang | Ausschließlich diese Datei und der Archivindex unter Docs/archive/. Keine Änderung an Runtime-Code, Roadmap, Konfiguration oder Deployment. |

Der spätere Archivcommit ist vom oben genannten Eingangs-Commit zu unterscheiden. Ein Dokumentationscommit bestätigt keine technische Umsetzung der beschriebenen Hardware.

### 1.1 Quellenbasis und Auswertungslücken

Ausgewertet wurde der in diesem Chat zugängliche Verlauf von der ursprünglichen Suche nach einem geeigneten SBC über die Diskussion der BPI-R4-Familie, 5G-/SIM-Integration, WLAN-AP-Modul, Router-/Failover-Idee bis zur historischen Preissuche.

Es gibt in diesem Chat **keine eigenständigen vom Nutzer hochgeladenen Bilder oder Screenshots**, die zu diesem Thema gehören. Entsprechend wurden keine Chatbilder nach Docs/archive/ übernommen. Die im Projektkontext vorhandenen ETSI-PDFs sind allgemeines TETRA-Referenzmaterial und waren für die Auswahl eines Router-SBCs, eines 5G-Modems und eines Wi-Fi-Moduls nicht entscheidungsrelevant; sie wurden für diese Abschlussdokumentation nicht als Hardwarefreigabe interpretiert.

Nicht verfügbar beziehungsweise nicht nachgewiesen sind insbesondere:

- Kaufbelege, Bestellbestätigungen oder Inventarnummern für BPI-R4, BE14 oder RM520N-GL;
- reale Board-Revision und konkrete Speicherbestückung eines vorhandenen Gerätes;
- Fotos der Hardware, Antennen, Pigtails oder des späteren Gehäuses;
- OpenWrt-/Linux-Konfiguration eines solchen Routers;
- reale SIM-/Providerdaten;
- WWAN-, QMI-/MBIM-, AP-, VLAN-, VPN- oder Failover-Logs;
- Stromaufnahme-, Temperatur-, CPU-, Datendurchsatz- oder RF-Messungen;
- ein On-Air-Test zusammen mit der TETRA-Basisstation;
- ein belastbarer Original-Chattitel oder Chatlink.

Die im Chat genannten Händlerpreise und Verfügbarkeiten waren Momentaufnahmen der damaligen Websuche und wurden **bei dieser Archivierung nicht erneut als aktuelle Marktpreise verifiziert**.

### 1.2 Statusbegriffe

| Status | Bedeutung |
|---|---|
| **Idee** | Im Chat vorgeschlagene Erweiterung oder Architekturidee ohne verbindliche Umsetzungszusage. |
| **Beschlossen/geplant** | Als Ziel oder bevorzugte Richtung erkennbar festgehalten; weiterhin kein Implementierungsnachweis. |
| **Implementiert** | Im Repository als Code/Konfiguration nachweisbar. |
| **Getestet** | Durch einen konkreten dokumentierten Test belegt. |
| **Im Betrieb bestätigt** | Durch reale Laufzeit-/Hardwarebeobachtung nachgewiesen. |

Für den in diesem Chat diskutierten BPI-R4-/BE14-/RM520N-Aufbau wurde **weder Implementierung noch Test noch Betrieb** belegt.

## 2. Ziel, Ausgangslage und behandelte Themen

Ausgangspunkt war die Suche nach einem SBC für NetCore-TETRA, das nicht nur Rechenplattform für einen Basisstations- oder Edge-Knoten sein soll, sondern zugleich einen sinnvollen **Router- und Access-Point-Charakter** mitbringt.

Der Nutzer nannte ausdrücklich den **Banana Pi BPI-R4 und Varianten der R4-Familie** als Beispiel. Im weiteren Verlauf verschob sich die Frage von einem allgemeinen SBC-Vergleich zu einer konkreteren Architektur:

- NetCore-TETRA-Dienste beziehungsweise Edge-Funktionen lokal betreiben;
- mehrere Ethernet-Ports für WAN, Backbone, Management oder Service nutzen;
- ein eigenes WLAN als Access Point ausstrahlen;
- über M.2 ein 5G-Modem mit SIM integrieren;
- Ethernet als primären Backhaul und 5G als möglichen Fallback nutzen;
- optional NVMe für Logs, Recordings, Datenbank oder Medien verwenden;
- den TETRA-SDR/SXceiver weiterhin über eine geeignete lokale Schnittstelle anbinden;
- später portable beziehungsweise autarke NetCore-TETRA-Nodes ermöglichen.

Die Kosten sollten dabei praktisch bleiben: Beim späteren Preisvergleich galt ausdrücklich **„billigste gewinnt“**, solange die gefundene Variante technisch brauchbar bleibt.

## 3. Verlauf und endgültige Planungsrichtung

| Schritt | Inhalt | Status |
|---|---|---|
| 1 | Suche nach einem NetCore-TETRA-SBC mit eigenem WLAN/AP; BPI-R4-Familie als Ausgangsidee. | Nutzeranforderung |
| 2 | Vergleich BPI-R4 Pro, BPI-R4, R4 Mini, R3 Mini sowie grobe Einordnung gegenüber Raspberry Pi 5. | Analyse/Empfehlung |
| 3 | BPI-R4 Pro 8X zunächst als maximal ausbaufähige Variante genannt. | Frühe Empfehlung |
| 4 | Standard-BPI-R4 später als pragmatischere Hauptwahl bewertet, insbesondere wegen reiferem Software-/OpenWrt-Umfeld. | Spätere Empfehlung; ersetzt die frühere Priorisierung als Erstwahl |
| 5 | Nutzer hebt den Router-Ansatz hervor und fragt nach 5G-SIM über M.2/PCIe. | Konkretisierung |
| 6 | Quectel RM520N-GL als bevorzugtes 5G-Modem vorgeschlagen; SIM soll über den Board-SIM-Slot genutzt werden. | Hardwareempfehlung |
| 7 | BPI-R4-NIC-BE14 als bevorzugtes Wi-Fi-7-AP-Modul vorgeschlagen. | Hardwareempfehlung |
| 8 | Konkreter Stack wird zu BPI-R4 8 GB + BE14 + RM520N-GL + optional 512 GB/1 TB NVMe verdichtet. | Letzte technische Empfehlung |
| 9 | Router-Architektur mit Ethernet-WAN, 5G-Fallback, WireGuard und mehreren WLAN-/VLAN-Zonen wird entworfen. | Idee/Planung |
| 10 | Island-/Degraded-Mode für lokale TETRA-Dienste bei WAN-Ausfall wird als spätere Erweiterung skizziert. | Idee/Roadmap-Kandidat |
| 11 | Historische Preissuche für BPI-R4, BE14 und RM520N-GL. | Markt-Snapshot, kein Kaufnachweis |

### 3.1 Spätere Präzisierung gegenüber der ersten Empfehlung

Die **frühe Empfehlung „R4 Pro 8X als Favorit“ ist nicht der letzte Stand des Chats**. Später wurde der **normale BPI-R4 in 8-GB-Ausführung** als vernünftigere Erstwahl bewertet, weil für eine NetCore-Basisstation nicht nur maximale I/O-Zahl, sondern auch ein möglichst gut gereiftes OpenWrt-/Treiberumfeld zählt.

Der BPI-R4 Pro bleibt relevant, wenn später deutlich mehr PCIe-/M.2-/Ethernet-Ressourcen benötigt werden. Er ist damit **Ausbaualternative**, nicht verworfene Hardware.

## 4. Endgültige Anforderungen und Entscheidungen

### 4.1 Zielanforderungen

Der geplante NetCore-Knoten soll möglichst viele der folgenden Rollen in einem Gerät bündeln:

- Linux-/OpenWrt-Router;
- Ethernet-WAN und lokales LAN/Backbone;
- Wi-Fi-Access-Point;
- 5G-/LTE-WWAN mit physischer SIM;
- VPN-Gateway zurück zur NetCore-Infrastruktur;
- lokaler Host für ausgewählte NetCore-Dienste;
- Anbindung des TETRA-RF-Pfads/SXceiver;
- optional NVMe für Logs, Recording, TTS/Media und Datenhaltung;
- später automatisches Failover und optionaler lokaler Weiterbetrieb ohne Zentrale.

### 4.2 Bevorzugter Hardware-Stack

| Komponente | Letzte Empfehlung im Chat | Einordnung |
|---|---|---|
| SBC/Router | **Banana Pi BPI-R4, bevorzugt 8 GB** | Bevorzugte Erstwahl, nicht gekauft/nicht getestet |
| WLAN | **BPI-R4-NIC-BE14** | Bevorzugtes AP-Modul für 2,4/5/6 GHz Wi-Fi 7; im Chat als native R4-Lösung gewählt |
| 5G | **Quectel RM520N-GL** | Bevorzugtes M.2-WWAN-Modul mit 5G/LTE-Fallback |
| Storage | 512 GB oder 1 TB NVMe | Option/Empfehlung |
| Router-OS | OpenWrt als naheliegende Routerplattform | Planung; keine NetCore-spezifische Installation belegt |
| VPN | WireGuard als naheliegender Backhaul-Tunnel | Architekturidee; keine Konfiguration belegt |

**Wichtig:** Der Nutzer hat keine Bestellung oder finale Stücklistenfreigabe dokumentiert. Die drei Hauptkomponenten wurden im Gespräch als konkrete Kaufkandidaten verdichtet, nicht als bereits vorhandene Hardware.

## 5. Betrachtete Boardvarianten

### 5.1 Banana Pi BPI-R4

Im Chat wurde der Standard-R4 als Sweet Spot für die Basisstation bewertet:

- vier Cortex-A73-Kerne im MT7988-Umfeld;
- 4/8-GB-Varianten, wobei für NetCore **8 GB** bevorzugt wurden;
- mehrere Ethernet-Schnittstellen;
- M.2 Key-M für NVMe;
- M.2 Key-B für WWAN/4G/5G;
- Nano-SIM am Board;
- zusätzliche PCIe-/miniPCIe-Anbindung für das BE14-WLAN-Modul;
- USB für lokale Peripherie beziehungsweise den TETRA-SDR/SXceiver.

Diese Produktmerkmale stammen aus der damaligen Hardware-/Webanalyse im Chat. **Vor Bestellung müssen Board-Revision, Pin-/Lane-Zuordnung und aktuelle Herstellerdokumentation erneut geprüft werden.**

### 5.2 Banana Pi BPI-R4 Pro / 8X

Der Pro wurde als besonders ausbaufähige Option diskutiert:

- mehr Ethernet-/M.2-Ressourcen;
- potenziell zwei NVMe-Laufwerke beziehungsweise zusätzliche WWAN-Erweiterungen;
- geeignet, wenn die Basisstation später noch deutlich mehr Edge-Dienste oder lokale Storage-/Netzfunktionen übernimmt.

Als Nachteil wurde die komplexere Lane-/Overlay-Situation einzelner Key-M-/Key-B-Ressourcen angesprochen. Zudem wurde im Chat der Software-/Mainline-Stand gegenüber dem normalen R4 als weniger konservative Wahl betrachtet.

**Status:** Alternative; nicht als zwingende Erstwahl beibehalten.

### 5.3 BPI-R4 Mini

Der R4 Mini wurde wegen integriertem WLAN als attraktiv für kleine Geräte betrachtet, aber für einen vollwertigen NetCore-Knoten als zu knapp eingeordnet:

- deutlich weniger RAM, im Gespräch 2 GB;
- schwächere A53-CPU-Klasse;
- interessant für kleine portable Nodes, aber nicht für den kompletten Dienstestapel.

### 5.4 BPI-R3 Mini

Der R3 Mini wurde als möglicher **BlueStation-/Portable-Node** eingeordnet:

- kleines Format;
- Wi-Fi 6 onboard;
- 2.5-GbE;
- NVMe-Möglichkeit;
- M.2 für Mobilfunk.

Er bleibt ein interessanter Spezialkandidat, ist aber nicht die favorisierte Hauptplattform dieses Chats.

### 5.5 Raspberry Pi 5 als Vergleich

Der Pi 5 wurde nicht pauschal als schlechter Rechner bewertet. Im Gegenteil wurde auf die stärkere allgemeine CPU-Leistung der Cortex-A76-Kerne hingewiesen. Der BPI-R4 wurde für diesen Einsatzzweck trotzdem bevorzugt, weil die **Router-I/O-Kombination** aus mehreren Netzwerkports, WWAN, AP-Modul und NVMe besser zur Zielarchitektur passt.

Daraus folgt ein noch offener technischer Prüfpunkt: Wenn der TETRA-PHY sehr CPU-/DSP-lastig lokal auf dem SBC läuft, muss die tatsächliche Rechenlast benchmarked werden. Routerleistung und allgemeine CPU-Leistung sind nicht dasselbe.

## 6. Architektur, Komponenten, Schnittstellen und Abhängigkeiten

### 6.1 5G-/SIM-Integration

Im Chat wurde das **RM520N-GL** als bevorzugter Kandidat gewählt. Die damalige Begründung:

- M.2-Key-B-Formfaktor für WWAN;
- 5G NR Sub-6;
- SA/NSA;
- LTE-Fallback;
- für den R4 dokumentierte beziehungsweise verbreitete Nutzung;
- Linux-/Router-Einsatz naheliegend;
- mehrere Mobilfunkantennenanschlüsse;
- Nutzung des am R4 vorhandenen SIM-Slots.

Für den normalen BPI-R4 wurde außerdem darauf hingewiesen, dass der WWAN-Pfad des Key-B-Slots praktisch über USB angebunden sein kann, obwohl das Modem selbst zusätzliche PCIe-Fähigkeiten besitzt. Das muss für die konkrete Board-/Modemrevision verifiziert werden.

Vorgesehene Rolle:

~~~text
Ethernet / lokaler Backbone
        |
        +---- primärer NetCore-Uplink
        |
        X Ausfall
        |
        v
5G/LTE über RM520N-GL
        |
        v
WireGuard / NetCore VPN
        |
        v
zentrale NetCore-Infrastruktur
~~~

Offen bleiben: konkrete Modem-/Firmwarevariante, APN, Provider, QMI/MBIM, SIM-/PIN-Handling, Antennen, Pigtails, 4x4-MIMO, GNSS-Nutzung, Failover-Hysterese und Telemetrie.

### 6.2 WLAN-AP

Das **BPI-R4-NIC-BE14** wurde gegenüber einer experimentelleren neueren Alternative bevorzugt. Ziel:

- 2,4 GHz;
- 5 GHz;
- 6 GHz;
- Wi-Fi 7;
- native R4-PCIe-/miniPCIe-Anbindung statt USB-WLAN-Stick;
- mehrere SSIDs und VLAN-Zonen.

Beispielhafte, **nicht final beschlossene** Segmentierung:

~~~text
NetCore-Service
  -> WebUI / SSH / Wartung

NetCore-Field
  -> portable NetCore-Clients / mobile Nutzung

NetCore-Admin
  -> restriktives Management

NetCore-Internet
  -> Internet über WAN/5G
  -> kein direkter Zugriff auf TETRA-Infrastruktur
~~~

Konkrete VLAN-IDs, IP-Netze und Firewallregeln wurden nicht beschlossen.

### 6.3 Gesamtarchitektur

~~~text
                        NetCore-TETRA Edge / TBS
                                  |
          +-----------------------+-----------------------+
          |                       |                       |
          v                       v                       v
       TETRA RF                Wi-Fi AP                5G/LTE
     SDR/SXceiver                BE14                RM520N-GL
          |                       |                       |
          +-----------------------+-----------------------+
                                  |
                               BPI-R4
                                  |
        +-------------------------+-------------------------+
        |                         |                         |
        v                         v                         v
      NVMe                    Ethernet                   VPN
 logs/recording             WAN / LAN /              WireGuard
 TTS/media/DB               backbone/service
~~~

Die besondere Stärke des Ansatzes ist die Kombination aus **Router, AP, WWAN, Storage und Edge-Compute** auf einer Plattform.

### 6.4 Physische Schnittstellen – geplanter Einsatz

| Schnittstelle | Geplanter Einsatz |
|---|---|
| M.2 Key-B | RM520N-GL 5G/LTE-WWAN |
| Nano-SIM | Provider-SIM für WWAN |
| M.2 Key-M | NVMe für Logs/Recordings/Media/DB |
| miniPCIe/PCIe-WLAN-Anbindung | BPI-R4-NIC-BE14 |
| USB | TETRA-SDR/SXceiver beziehungsweise weitere Servicehardware |
| Ethernet | WAN, NetCore-Backbone, Management, Service/LAN |
| Antennenanschlüsse | getrennt für TETRA, Wi-Fi und 5G planen |

### 6.5 Netzprotokolle – geplante Rolle

- IPv4/IPv6 nach endgültigem NetCore-Netzplan;
- VLAN 802.1Q für Segmentierung;
- OpenWrt Firewall/NAT/Policy Routing;
- 5G/LTE über passenden Linux-WWAN-Stack, voraussichtlich QMI oder MBIM je nach finalem Modembetrieb;
- WireGuard als bevorzugte VPN-Idee;
- NetCore-interne Dienste wie MQTT, Management-APIs und Observability nur über definierte Trust-Zonen.

**Keine produktiven Ports, IP-Adressen oder VLAN-IDs wurden in diesem Chat festgelegt.**

## 7. Failover-, Degraded- und Island-Mode-Ideen

### 7.1 Automatisches WAN-Failover

Vorgeschlagen:

1. Ethernet/Backbone bevorzugen.
2. Bei Ausfall auf 5G umschalten.
3. VPN zum NetCore-Core automatisch über den aktiven Uplink wiederherstellen.
4. Management und zentrale Dienste weiter erreichbar halten.
5. Lokale TETRA-Funktion nicht unnötig vom Internetzugang abhängig machen.

Als spätere Option wurde ein weiterer Uplink über externes WLAN erwähnt.

### 7.2 Policy Routing

Als spätere Ausbaustufe:

- TETRA-Steuer-/Core-Verkehr bevorzugt über definierten Backhaul;
- Management getrennt behandeln;
- Service-/Gast-WLAN nur über Internet-WAN routen;
- keine versehentliche Querroute aus Gast-/Internetsegmenten in TETRA-Management;
- VPN- und Failover-Zustand in NetCore-Observability sichtbar machen.

### 7.3 Island Mode

Als zukünftiger lokaler Weiterbetrieb bei Totalausfall der Zentrale wurden skizziert:

- lokale Subscriber-Daten;
- lokale Gruppen;
- lokales Call-Control;
- lokale SDS-Funktionen;
- lokales Recording/Logging;
- spätere Synchronisation nach Wiederkehr der Verbindung.

**Status: Idee.** Eine BPI-R4-spezifische Island-Mode-Implementierung ist nicht nachgewiesen.

## 8. RF-, EMV-, Strom- und Thermikthemen

Ein Router-SBC mit 5G und Wi-Fi direkt neben TETRA-RF ist mechanisch praktisch, HF-technisch aber nicht automatisch harmlos.

Im Chat bereits hervorgehoben:

- 5G-/LTE- und Wi-Fi-Antennen nicht gedankenlos direkt neben TETRA-RF-Anschlüsse setzen;
- räumliche Trennung, saubere Masse-/Schirmführung und geeignete Pigtails berücksichtigen;
- 5G-Modem ausreichend kühlen;
- Strombudget inklusive Spitzenlast des WWAN-Modems dimensionieren.

Zusätzlich für die reale Abnahme sinnvoll:

- Desensibilisierung des TETRA-Empfangspfads bei 5G-/Wi-Fi-Sendeaktivität messen;
- S21/Isolation beziehungsweise reale Störabstände bewerten;
- Antennen- und Kabelwege im Gehäuse trennen;
- DC/DC-Wandler und digitale Störer auf Spurious/Noise prüfen;
- Temperatur bei Dauerlast aus Router + AP + 5G + NVMe + TETRA messen.

Diese Punkte sind **nicht getestet**.

## 9. Erreichter Entwicklungs- und Betriebsstand

| Punkt | Status |
|---|---|
| BPI-R4 als NetCore-Hardware gekauft | **Nicht belegt** |
| BE14 gekauft/eingebaut | **Nicht belegt** |
| RM520N-GL gekauft/eingebaut | **Nicht belegt** |
| SIM eingesetzt | **Nicht belegt** |
| OpenWrt installiert | **Nicht belegt** |
| WLAN-AP sendet | **Nicht belegt** |
| 5G registriert / Datenverbindung | **Nicht belegt** |
| Ethernet/5G-Failover | **Nicht implementiert/nicht getestet** |
| WireGuard-Backhaul | **Nicht BPI-spezifisch belegt** |
| NetCore-Dienste auf R4 | **Nicht belegt** |
| SXceiver/SDR am R4 | **Nicht belegt** |
| NVMe-Recording | **Nicht belegt** |
| Island Mode | **Idee** |
| RF-Koexistenztest | **Nicht durchgeführt** |
| Dauerlast/Temperatur | **Nicht durchgeführt** |

## 10. Repository-Abgleich vom 05.10.2026

### 10.1 Hauptbranch

Der geprüfte Default-Branch ist **main** bei Commit **9116c15d645458f99e236712b67a1ad970432791**.

Die Code-/Dokumentensuche ergab:

- **BPI-R4/BPI-R4 Pro** tauchen in Docs/NetCore-Tetra-Komplettguide-2026-09-28.md auf.
- Dort werden mobile Router mit **5G, M.2-Funkmodulen, Access Point und kleinem Switch** ausdrücklich als erhaltene Ausbauidee genannt.
- Für die diskutierten BPI-R4-/R4-Pro-Varianten werden **Strombudget, Treiber, Antennen, AP-Betrieb und VPN-Übergänge** noch als zu konkretisieren beschrieben.
- Für die konkreten Suchbegriffe **RM520N**, **BE14**, **OpenWrt** und **wwan** ergab die heutige Suche keine belastbaren Treffer, die eine Implementierung dieses Hardware-Stacks belegen.
- Andere Vorkommen von „Router“ im Repository, etwa SDS-Router oder interne Software-Router, sind nicht mit dem hier geplanten physischen BPI-Router gleichzusetzen.

**Fazit:** Die Router-/5G-/AP-Idee ist dokumentarisch erhalten; der konkrete BPI-R4 + BE14 + RM520N-GL-Stack ist im geprüften main nicht als produktive Integration nachgewiesen.

### 10.2 Bereits vorhandene Archivdokumentation

Im Branch Archiving existiert bereits:

- [2026-10-03_werma-racksignalisierung-bpi-r4-pro-und-rack-architektur.md](2026-10-03_werma-racksignalisierung-bpi-r4-pro-und-rack-architektur.md)

Dort wird der **BPI-R4 Pro als Router-/Switch-Option eines Rack-Konzepts** behandelt. Das vorliegende Dokument konkretisiert den späteren eigenen Chat zu:

- Auswahl innerhalb der R4-Familie;
- Standard-R4 versus Pro;
- 5G-SIM-Integration;
- RM520N-GL;
- BE14;
- WAN-Failover;
- möglichem Island Mode;
- konkretem historischen Preisvergleich.

### 10.3 Eingangsstand des Archivbranches

Vor dieser Archivierung stand Archiving bei **d37658d8588a367d11e979bfd29b07a8a50f61ef**. Bestehende Archiveinträge wurden vor dem Schreiben gelesen und der neue Dateiname auf Kollision geprüft.

## 11. Relevante Dateien, Dienste, Konfigurationen, Ports und Pfade

Repository-seitig wurde in diesem Chat keine neue Runtime-Datei festgelegt. Relevant sind aktuell:

- Docs/NetCore-Tetra-Komplettguide-2026-09-28.md – enthält die allgemeine mobile Router-/5G-/BPI-R4-Ausbauidee.
- Docs/archive/2026-10-03_werma-racksignalisierung-bpi-r4-pro-und-rack-architektur.md – verwandtes Archiv zum BPI-R4 Pro als Rackrouter.
- Docs/archive/README.md – Archivindex.
- Diese Datei – Konkretisierung der Router-SBC-/5G-/Wi-Fi-Auswahl.

Es gibt **keine** in diesem Chat belegte produktive OpenWrt-Konfiguration, keine konkreten Ports für Routermanagement und keine feste VLAN-/IP-Tabelle.

## 12. Wichtige Befehle und Installations-/Deploymentabläufe

In diesem Chat wurden **keine Installations-, OpenWrt-, QMI-/MBIM-, WireGuard- oder AP-Befehle erfolgreich auf realer Hardware ausgeführt**.

Ein sinnvoller späterer reproduzierbarer Ablauf wäre:

1. Hardware-Revision inventarisieren.
2. Freigegebenes OpenWrt-/Linux-Image festlegen.
3. Boot-/Recoveryweg dokumentieren.
4. BE14-Treiber/Firmware prüfen.
5. RM520N-GL-Modemmodus und SIM testen.
6. WAN/LAN/VLAN/Firewall aufbauen.
7. WireGuard und Failover einrichten.
8. NetCore-Agent/Services integrieren.
9. Observability aktivieren.
10. Hardware-/RF-/Dauerlastabnahme durchführen.

**Status: vorgeschlagen, nicht ausgeführt.**

## 13. Fehler, Diagnose, Ursachen und verbleibende Probleme

Es gab keine Laufzeitfehler einer realen R4-Installation. Die relevanten Risiken sind Planungsrisiken:

- **Software-/Treiberreife:** Standard-R4 später als konservativere Wahl gegenüber Pro priorisiert.
- **CPU vs. Router-SoC:** MT7988 für Netzwerk stark, aber PHY-/DSP-Last muss real gemessen werden.
- **RAM:** 2-GB-Mini-Varianten für den kompletten NetCore-Stapel zu knapp eingeschätzt.
- **RF-Koexistenz:** 5G/Wi-Fi kann TETRA-Empfang beeinflussen; reale Desense-Messung nötig.
- **Strom/Thermik:** WWAN-Spitzenlast plus AP/NVMe/SDR berücksichtigen.
- **Preis-/Händlerrisiko:** Marketplace-Varianten, Lieferumfang, Zoll/MwSt. und Versand vor Kauf prüfen.
- **R4-Pro-Lanes/Overlays:** bei Vollausbau genaue Lane-Zuordnung und Supportstand prüfen.

## 14. Durchgeführte Tests und Ergebnisse einschließlich Grenzen

### Tatsächlich durchgeführt

- technische Bewertung der Kandidaten im Chat;
- historische Web-/Preisrecherche am 10.09.2026;
- aktueller GitHub-Abgleich am 05.10.2026:
  - Branch Archiving geprüft;
  - main geprüft;
  - vorhandener Archivindex gelesen;
  - bereits vorhandenes BPI-R4-Pro/Rack-Archiv berücksichtigt;
  - Code-/Dokumentensuche nach BPI-R4, RM520N, BE14, OpenWrt, wwan und 5G-/M.2-Routerbezug.

### Nicht durchgeführt

- Hardware-Boot;
- OpenWrt-Flash;
- 5G-Registration;
- SIM-Test;
- WLAN-AP-Test;
- VLAN-/Firewall-Test;
- WAN-Failover;
- VPN-Failover;
- Throughput-/Latency-Test;
- CPU-/RAM-Benchmark;
- NVMe-Test;
- NetCore-Dienstetest;
- TETRA-On-Air-Test;
- RF-Desense-/Spektrummessung;
- Dauerlast-/Temperaturtest.

Repository-Suche ist ein **statischer Nachweis über vorhandene Dateien**, kein Funktions- oder Live-Test.

## 15. Historischer Preisstand „billigste gewinnt“

Die folgende Tabelle ist ein **historischer Snapshot aus der damaligen Suche**, kein aktuelles Angebot vom 05.10.2026:

| Teil | Historischer Fund | Genannter Preis | Bewertung im Chat |
|---|---|---:|---|
| BPI-R4 4 GB | Joom | ca. **264,96 €** | brauchbarer, aber nicht besonders günstiger Fund |
| BPI-R4 4 GB | Micros, Polen | **182,97 € netto**, rechnerisch ca. **217,73 € brutto** vor Versand | möglicher günstigerer Board-Fund; Versand nicht verifiziert |
| BPI-R4-NIC-BE14 | Joom/Banana-Pi-Store | ca. **27,58 €** | auffällig günstig; Variante beim Checkout prüfen |
| Quectel RM520N-GL | Kaufland Marketplace / Lenovo-Ausführung | ca. **183,99 € inkl. Versand** | damals günstigster sauber identifizierter deutscher Neupreis |
| Gesamt, Joom-R4 + BE14 + RM520N | Rechenwert | ca. **476,53 €** | 4-GB-Board, nicht die bevorzugte 8-GB-Zielausführung |
| Gesamt, Micros-R4 + BE14 + RM520N | Rechenwert | ca. **429,30 € + R4-Versand** | theoretisch, Versand/Verfügbarkeit offen |

Für die **8-GB-Ausführung** wurde im damaligen Verlauf kein sauber verifizierter Bestpreis festgehalten. Die Preissuche muss vor Kauf wiederholt werden.

## 16. Verworfene oder ersetzte Ansätze

| Ansatz | Ergebnis |
|---|---|
| R4 Pro 8X automatisch als beste Wahl | **Relativiert/ersetzt als Erstempfehlung.** Standard-R4 später für den ersten Node bevorzugt; Pro bleibt Ausbauoption. |
| R4 Lite als Sweet Spot | **Nicht empfohlen.** Kein klarer Vorteil gegenüber dem normalen R4 für die Zielrolle. |
| R4 Mini als Haupt-BS | **Nicht bevorzugt.** Zu wenig RAM/CPU-Reserve für den angedachten Gesamtstack. |
| R3 Mini als Haupt-BS | **Nicht bevorzugt.** Eher portable BlueStation-/Spezialrolle. |
| Raspberry Pi 5 + USB-WLAN als Routerplattform | **Nicht bevorzugt für diesen Zweck.** CPU interessant, aber Router-I/O des BPI-R4 passt besser. |
| BE19 statt BE14 | **Nicht gewählt.** BE14 im Chat als konservativere R4-Lösung bevorzugt. |
| Nur „5G-Internet“ | **Erweitert.** 5G soll perspektivisch echter Backhaul/Fallback mit VPN sein. |

Der ursprünglich angebotene breitere Vergleich mit weiteren Router-SBCs wie Radxa/FriendlyElec/OpenWrt One/CM5/RK3588 wurde in diesem Chat nicht mehr durchgeführt, weil der Nutzer die Diskussion auf BPI-R4, 5G und WLAN konkretisierte.

## 17. Sämtliche noch relevanten Ideen, Wünsche und offenen Aufgaben

Roadmap-Kandidaten aus diesem Chat; außerhalb Docs/archive/ wird durch diesen Auftrag nichts geändert:

1. **BPI-R4-Referenzhardware festlegen:** Standard-R4 8 GB gegen R4 Pro anhand realer Anforderungen final entscheiden.
2. **BOM einfrieren:** BE14, exakte RM520N-GL-Variante, NVMe, Kühlung, Netzteil, Antennen, Pigtails, Gehäuse.
3. **OpenWrt-/Linux-Baseline:** reproduzierbares Image, Kernel-/Treiberstand und Recovery.
4. **WWAN-Service:** QMI/MBIM, APN, SIM/PIN, Providerprofil, Watchdog.
5. **AP-/VLAN-Design:** Management, Field/Service und Internet/Gast strikt trennen.
6. **Backhaul-Failover:** Ethernet primär, 5G sekundär, Zustandsautomat und Hysterese statt Flapping.
7. **WireGuard-Integration:** Tunnel automatisch über aktiven Uplink wiederherstellen.
8. **NetCore-Discovery/Provisioning:** Router-/Edge-Node in die vorhandene Deployment-/Discovery-Idee integrieren.
9. **Observability:** Mobilfunksignal, Registration, IP, RTT, Paketverlust, VPN, AP-Clients, CPU, RAM, SSD und Temperatur zentral erfassen.
10. **RF-Koexistenztest:** TETRA-Empfang unter Wi-Fi-/5G-Sendeaktivität prüfen.
11. **Leistungs-/Thermiktest:** Worst Case aus 5G-Uplink, Wi-Fi, NVMe und TETRA-PHY.
12. **CPU-Benchmark:** reale TETRA-PHY-/Codec-/Service-Last gegen Pi 5 beziehungsweise aktuelle ARM64-Baseline.
13. **Island Mode spezifizieren:** lokale Subscriber-/Gruppen-/SDS-/Call-Funktionen und Re-Sync.
14. **Security/RBAC:** Routermanagement, NetCore-Management und Gast-/Internetverkehr klar trennen; keine Secrets im Image.
15. **Portable Variante:** R3 Mini oder R4 Mini separat als BlueStation-Mini prüfen.

## 18. Konkrete nächste Schritte, Abhängigkeiten und Prioritäten

### P0 – vor Bestellung

- aktuelle BPI-R4-Boardrevision und 8-GB-Verfügbarkeit prüfen;
- BE14-Kompatibilität für genau diese Revision bestätigen;
- RM520N-GL-Variante und Linux-/OpenWrt-Betriebsart bestätigen;
- Antennen-/Pigtail-/Kühlungsbedarf in den echten Gesamtpreis aufnehmen;
- Netzteil mit Reserve für WWAN-Spitzenlast dimensionieren.

### P0 – Architekturentscheidung

Entscheiden, ob der R4:

- nur Router/AP/5G-Gateway wird und TETRA auf einem separaten Compute-Node bleibt, oder
- **Router + Edge-Compute + TETRA-Basisstation** auf einem Board vereint.

Diese Entscheidung beeinflusst CPU-Benchmark, Ausfallsicherheit, Wartung und RF-Aufbau.

### P1 – Lab-Prototype

- OpenWrt/Linux booten;
- Ethernet-Routing;
- BE14 als AP;
- RM520N-GL mit Test-SIM;
- NVMe;
- WireGuard;
- Ethernet→5G-Failover;
- Monitoring.

### P1 – NetCore-Anbindung

- Discovery/Provisioning;
- zentrale Logs und Health;
- definierte Management-VLANs;
- NetCore-Agent;
- erst danach zusätzliche Core-/Edge-Dienste auf denselben SBC verschieben.

### P2 – TETRA und Resilienz

- SXceiver/SDR anbinden;
- CPU-/Timing-/USB-Verhalten prüfen;
- RF-Koexistenz messen;
- Island Mode und Re-Synchronisation entwickeln;
- Dauerlast- und Ausfalltests durchführen.

## 19. Relevante Quellen, Dateien, Branches, Commits und PRs

### Repository

- **Branch:** Archiving
- **Eingangsstand Archivbranch:** d37658d8588a367d11e979bfd29b07a8a50f61ef
- **Default-Branch:** main
- **Geprüfter main-Stand:** 9116c15d645458f99e236712b67a1ad970432791
- **Zentrale Projektzusammenfassung:** Docs/NetCore-Tetra-Komplettguide-2026-09-28.md
- **Verwandtes Archiv:** [WERMA-Racksignalisierung, BPI-R4 Pro und Rack-Architektur](2026-10-03_werma-racksignalisierung-bpi-r4-pro-und-rack-architektur.md)
- **Archivindex:** [README.md](README.md)

Es wurde in diesem Chat kein eigener Implementierungs-PR für den BPI-R4-/5G-/Wi-Fi-Stack identifiziert oder erstellt.

### Historische externe Referenzen aus dem Chat

Die damalige Beratung verwies auf Hersteller-/Wiki-/OpenWrt-/Quectel-Seiten sowie Preisvergleichs-/Marketplace-Angebote. Diese Quellen dienten der damaligen Auswahl; ihre Verfügbarkeit, Revision und Preise wurden bei diesem Archivlauf nicht erneut vollständig verifiziert.

## 20. Bilder und Anhänge

**Keine eigenständigen Chatbilder vorhanden.** Daher wurde kein Bildasset für diesen Chat in Docs/archive/ angelegt.

Die im Projektkontext verfügbaren ETSI-PDFs sind fachliche TETRA-Referenzen, aber keine Anhänge dieses Hardware-Auswahlgesprächs im engeren Sinn und keine Freigabe für BPI-R4, BE14 oder RM520N-GL. Sie wurden nicht kopiert oder dupliziert.

## 21. Abschlussbewertung

Der Kerngewinn dieses Chats ist die Verschiebung von „irgendein SBC mit WLAN“ zu einem **routerzentrierten NetCore-Edge-Node**.

Die bevorzugte Richtung lautet:

**BPI-R4 8 GB + BE14 Wi-Fi 7 + RM520N-GL 5G + optional NVMe**

mit:

- Ethernet als primärem Backhaul;
- 5G als automatisierbarem Fallback;
- WireGuard zurück zur NetCore-Infrastruktur;
- mehreren sauber getrennten WLAN-/VLAN-Zonen;
- optional lokaler Datenhaltung;
- perspektivisch lokalem Degraded-/Island-Betrieb.

Der Repository-Abgleich bestätigt nur, dass die **allgemeine BPI-R4/5G/AP-Idee bereits als Ausbauidee dokumentiert** ist. Er bestätigt **keine konkrete Hardwareintegration**. Vor einem produktiven Einsatz fehlen Hardwareauswahl, aktuelle Preisprüfung, Treiber-/OpenWrt-Abnahme, Netzwerkdesign, Strom-/Thermiktest, RF-Koexistenzmessung und der reale NetCore-/TETRA-Ende-zu-Ende-Test.
