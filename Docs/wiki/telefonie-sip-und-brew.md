# SIP, lokaler Asterisk und Brew

NetCore besitzt zwei getrennte Integrationswege: **SIP** koppelt TETRA-Einzelrufe an die vorhandene PBX; **Brew** verbindet die TBS optional mit einem Brew-Peer. Beide dürfen dieselben Nummern, Rufzustände oder Audioflüsse nur nach bewusst festgelegtem Routing anfassen.

## SIP-Rollen

| Rolle | Zweck |
|---|---|
| vorhandene PBX | Nebenstellen, externer Wählplan, DECT/Telefonie |
| zentraler SIP Switch | TBS-Routing anhand aktueller Mobility-Daten; HTTP-Management z. B. 8300 |
| lokaler Asterisk auf **jeder TBS** | Edge-B2BUA und direkter PBX-Fallback |
| native TBS-Bridge | lokale CMCE-Rufsteuerung und PCMU↔TETRA-Codec |

Normalweg: **Funkgerät ⇄ native TBS-Bridge ⇄ lokaler Asterisk ⇄ zentraler SIP Switch ⇄ PBX.** Die native Bridge registriert sich am lokalen Asterisk; der lokale Asterisk am zentralen Switch und hält den direkten PBX-Trunk bereit. [Architekturquelle](../services/sip-switch/architektur.md).

### Fallback und Grenzen

- TETRA→PBX verwendet den direkten PBX-Weg nur bei `CHANUNAVAIL`/`CONGESTION`, nicht bei `BUSY` oder `NOANSWER`.
- Bei PBX→TETRA löst **die PBX** den Failover auf direkte TBS-Trunks aus; eine ausgefallene zentrale Instanz kann keinen neuen Fallbackbefehl senden.
- Bestehende SIP-Dialoge wechseln nicht mitten im Gespräch auf einen anderen Pfad. Fallback gilt für neue Rufe.
- Ein zentraler Asterisk-Installer und der lokale TBS-Fallback-Installer verwalten ähnliche Asterisk-Dateien und SIP-Ports; **nicht auf demselben Host** installieren. Siehe [Rollout und Reparatur](../roadmaps/zentraler-netzbetrieb.md).

## SIP-Diagnose

Prüfreihenfolge: native Bridge am lokalen Asterisk → lokaler Asterisk am Switch → Switch an PBX → Mobility meldet erreichbare Serving-TBS → PJSIP-Kontakte `Avail` → Ruf-Setup/U-CONNECT → RTP in beide Richtungen → sauberer Release. Ein `180 Ringing` belegt nur den Signalisierungsschritt. Relevante Ports sind getrennt unter [Netzwerk, Ports und Protokolle](netzwerk-und-ports.md) dokumentiert.

```bash
asterisk -rx 'pjsip show registrations'
asterisk -rx 'pjsip show contacts'
asterisk -rx 'pjsip show endpoints'
```

Die tatsächlichen Peer-Namen, Auth-Daten und RTP-Bereiche aus den installierten Dateien lesen. Bei einem alten direkten PBX-Setup den [TBS-Fallback-Reparaturpfad](../roadmaps/zentraler-netzbetrieb.md) prüfen; die lokale Installation nicht mit dem zentralen Switch verwechseln.

## Brew

Die TBS-`[brew]`-Sektion öffnet eine optionale Verbindung zu einem externen Brew-Peer (Vorlagenport `8081`, WebSocket). Je nach Gegenstelle werden Gruppenaffiliation, Sprache, SDS und weitere Nachrichten verarbeitet. Der eigene [brew-server](../../misc/brew-server) ist ein **separater Dienst**, keine implizite Ersetzung von Call Control oder SIP Switch.

Vor Aktivierung klären: Welche GSSI/ISSI bleibt lokal? Welche wird nach Brew geroutet? Wer besitzt die Rufsteuerung, Aufzeichnung und das Release? Gibt es eine Rückkopplung von aus- und eingehenden Rufen/SDS? Die TBS-Konfiguration unterscheidet ausgehende Whitelist und eingehende Bereichsgrenze; diese Regeln sind nicht gleich. Erst mit einer isolierten Testgruppe und eindeutigen Routen testen. [Gruppen- und Einzelrufe](gruppen-und-einzelrufe.md) · [SDS und U-STATUS](kurznachrichten-und-status.md)

Brew ist eine optionale Gegenstellenintegration. Einseitige Einspeisung fremder TETRA-Empfangsgruppen mit netzseitiger Sendesperre ist ein eigenes, noch offenes Vorhaben Z11.1; sie entsteht nicht automatisch durch Aktivierung des vorhandenen Brew-Clients. [Gesamtroadmap](../roadmaps/gesamtroadmap.md)

## Quellen zur Pflege dieser Seite

[SIP-Architektur](../services/sip-switch/architektur.md) · [Brew-Protokoll](../../crates/tetra-entities/src/net_brew/protocol.rs).
