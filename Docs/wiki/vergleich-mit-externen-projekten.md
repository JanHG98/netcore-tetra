# Vergleich mit externen TETRA Projekten

**Stand 13.10.2026.** Verglichen wurde `NetCore-Tetra main@e1617b28` mit sechs eigenstaendigen Quellen. Ein Commit oder eine Funktion in einem Fremdprojekt ist kein Nachweis für NetCore und keine automatische Uebernahmeentscheidung. Die Einordnung unterscheidet vorhandene Gleichwertigkeit, Teilintegration, gezielt portierbare Aenderung, groesseren Umbau und derzeit fehlende Relevanz.

| Quelle | Gepruefter Stand | Aenderung seit 07.10.2026 | Einordnung für NetCore |
| --- | --- | --- | --- |
| [FlowStation](https://github.com/razvanzeces/flowstation) | `0f4faa98` | unverändert | Circuit-Reopen, fruehe RTP-Pakete, SIP-CANCEL und SDS-Deduplizierung bleiben gezielte Vergleichspunkte; NetCore besitzt Teilpfade, aber keine pauschal belegte Gleichwertigkeit |
| [TETRA BlueStation](https://github.com/MidnightBlueLabs/tetra-bluestation) | `09d4e0d9`, Release `v0.5.10-09d4e0d` | unverändert | MAC-END-/Fragmentierungs- und Kanalallokationslogik nur gemeinsam in Encoder, Parser und Scheduler bewerten |
| [Nexus BS](https://github.com/invictus737/nexus-bs) | `6a4e8f58`, Release `v0.1.80` | unverändert; Projekt als 1.x archiviert | Auth-Grenzen sind vergleichbar; RF-Kalibrierung/Smart-Tune erfordert Hardwaremessung und groesseren Umbau |
| [Nexus BS2](https://github.com/invictus737/nexus-bs2) | `ebfd0b51` | unverändert | Oeffentliche Konfigurations-, Touch- und Kalibrierungsdokumentation; kein veroeffentlichter Runtime-Quellcode oder GitHub-Release, daher keine pruefbare Codeintegration ableitbar |
| [Bost FlowStation](https://github.com/Aitorrio/bost-flowstation) | `ccae4632`, Codeangabe `0.5.4`; letzter GitHub-Release weiter `v0.4.0` | 12 Commits seit `96671561`: Ambient Listening, LIP-Anforderung/Korrelation sowie Stationsposition und MCC/MNC in Telemetrie | LIP-Eingang, Kartenpositionen und Node-Identitaet existieren in NetCore teilweise. SS-AL ist nur als Faehigkeit/Planung sichtbar; eine Portierung benoetigt CMCE-, Berechtigungs-, Audit-, Medien- und Endgeräteabnahme |
| [Tetra Dispatch](https://github.com/ysamouhos/tetra-dispatch) | `f04012ee`, keine Releases | unverändert | Browser-Audio/PTT, SDS, LIP und serverseitig geprüftes Ambient-Listen-Kommando bleiben Referenzen für den Dispatch-Ausbau; ein einzelnes Passwort ersetzt kein Ressourcen-RBAC |

## Neuer Bost Befund

Der Vergleich `96671561...ccae4632` aendert unter anderem `crates/tetra-entities/src/cmce/subentities/cc_bs/procedures/ambience.rs`, `sds_bs.rs`, `net_telemetry/events.rs`, `net_brew/entity.rs`, `net_control/*` und die Telemetriekonfiguration. Die Aenderungen verbinden damit Funkprozedur, Steuerkommando, Brew und Anzeige. Ein isoliertes Kopieren des Dashboardteils waere fachlich unvollstaendig.

NetCore dekodiert LIP-Positionen bereits in `crates/tetra-entities/src/cmce/subentities/sds_bs.rs`; der eigene Brew-Server und der Control Room fuehren Karten-/Positionsdaten. Node Gateway zeigt MCC, MNC, LA, CC und Traeger aus der TBS-Identitaet. Nicht belegt ist dagegen ein gleichwertiger aktiver LIP-Anforderungspfad mit durchgaengiger Korrelation oder eine vollständige Ambient-Listening-Prozedur. `PermittedTemporaryServices.ambience_listening` in `crates/tetra-saps` ist nur ein Datenfeld, kein Betriebsnachweis.

**Empfehlung:** Zunaechst aktive LIP-Anforderung als begrenzten Interoperabilitaetstest spezifizieren. Ambient Listening bleibt nachgelagert, bis NETCORE-IAM-01 beziehungsweise ein ausdruecklicher lokaler Ressourcen- und Auditvertrag festlegt, wer welche Ziel-ISSI ausloesen darf. Danach sind negative Berechtigungstests, Ruf-/Floor-/Medienabbruch, Endgeräteanzeige und On-Air-Verhalten gemeinsam abzunehmen.

## Dauerhafte Vergleichsregeln

- `nexus-bs` und `nexus-bs2` bleiben getrennte Quellen.
- Dokumentation oder Binaerdownloads ohne Quellcode werden als Beleggrenze gekennzeichnet.
- Upstream-Features werden nicht nur nach Namen, sondern gegen die betroffenen NetCore-Subsysteme und vorhandenen Tests verglichen.
- Ein bestandener Unit-/Browser-Test ersetzt keine installierte Anlage, kein RF-Messbild und keine Endgeräte-Interoperabilitaet.
- Sicherheitsrelevante Funktionen wie Ambient Listening, Authentifizierung und Updates werden nicht aus Open-Lab-Beispielen als produktionsreif abgeleitet.

## Quellen

[NetCore Projektstand](projektstand-und-nachweise.md) · [Gesamtroadmap](../roadmaps/gesamtroadmap.md) · [Zentrale Anmeldung und RBAC](../roadmaps/zentrale-anmeldung-und-berechtigungen.md)
