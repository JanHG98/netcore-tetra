# SIP, lokaler Asterisk und Brew

NetCore besitzt zwei getrennte Integrationswege: **SIP** koppelt TETRA-Einzelrufe an die vorhandene PBX; **Brew** verbindet die TBS optional mit einem Brew-Peer. Beide dürfen dieselben Nummern, Rufzustände oder Audioflüsse nur nach bewusst festgelegtem Routing anfassen.

## SIP-Rollen

| Rolle | Zweck |
|---|---|
| vorhandene PBX | Nebenstellen, externer Wählplan, DECT/Telefonie |
| zentraler SIP Switch | TBS-Routing anhand aktueller Mobility-Daten; HTTP-Management z. B. 8300 |
| lokaler Asterisk auf **jeder TBS** | Edge-B2BUA und direkter PBX-Fallback |
| native TBS-Bridge | lokale CMCE-Rufsteuerung und PCMU↔TETRA-Codec |

Normalweg: **Funkgerät ⇄ native TBS-Bridge ⇄ lokaler Asterisk ⇄ zentraler SIP Switch ⇄ PBX.** Die native Bridge registriert sich am lokalen Asterisk; der lokale Asterisk am zentralen Switch und hält den direkten PBX-Trunk bereit. [Architekturquelle](https://github.com/JanHG98/netcore-tetra/blob/main/system-backend/sip-switch/docs/architecture.md).

### Fallback und Grenzen

- TETRA→PBX verwendet den direkten PBX-Weg nur bei `CHANUNAVAIL`/`CONGESTION`, nicht bei `BUSY` oder `NOANSWER`.
- Bei PBX→TETRA löst **die PBX** den Failover auf direkte TBS-Trunks aus; eine ausgefallene zentrale Instanz kann keinen neuen Fallbackbefehl senden.
- Bestehende SIP-Dialoge wechseln nicht mitten im Gespräch auf einen anderen Pfad. Fallback gilt für neue Rufe.
- Ein zentraler Asterisk-Installer und der lokale TBS-Fallback-Installer verwalten ähnliche Asterisk-Dateien und SIP-Ports; **nicht auf demselben Host** installieren. Siehe [Rollout und Reparatur](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/CENTRAL_NETWORK_ROLLOUT.md).

## SIP-Diagnose

Prüfreihenfolge: native Bridge am lokalen Asterisk → lokaler Asterisk am Switch → Switch an PBX → Mobility meldet erreichbare Serving-TBS → PJSIP-Kontakte `Avail` → Ruf-Setup/U-CONNECT → RTP in beide Richtungen → sauberer Release. Ein `180 Ringing` belegt nur den Signalisierungsschritt. Relevante Ports sind getrennt unter [[Netzwerk-und-Ports]] dokumentiert.

```bash
asterisk -rx 'pjsip show registrations'
asterisk -rx 'pjsip show contacts'
asterisk -rx 'pjsip show endpoints'
```

Die tatsächlichen Peer-Namen, Auth-Daten und RTP-Bereiche aus den installierten Dateien lesen. Bei einem alten direkten PBX-Setup den [TBS-Fallback-Reparaturpfad](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/CENTRAL_NETWORK_ROLLOUT.md) prüfen; die lokale Installation nicht mit dem zentralen Switch verwechseln.

## Brew

Die TBS-`[brew]`-Sektion öffnet eine optionale Verbindung zu einem externen Brew-Peer (Vorlagenport `8081`, WebSocket). Je nach Gegenstelle werden Gruppenaffiliation, Sprache, SDS und weitere Nachrichten verarbeitet. Der eigene [`misc/brew-server`](https://github.com/JanHG98/netcore-tetra/tree/main/misc/brew-server) ist ein **separater Dienst**, keine implizite Ersetzung von Call Control oder SIP Switch.

Vor Aktivierung klären: Welche GSSI/ISSI bleibt lokal? Welche wird nach Brew geroutet? Wer besitzt die Rufsteuerung, Aufzeichnung und das Release? Gibt es eine Rückkopplung von aus- und eingehenden Rufen/SDS? Die TBS-Konfiguration unterscheidet ausgehende Whitelist und eingehende Bereichsgrenze; diese Regeln sind nicht gleich. Erst mit einer isolierten Testgruppe und eindeutigen Routen testen. [[Calls]] · [[SDS-and-U-STATUS]]
