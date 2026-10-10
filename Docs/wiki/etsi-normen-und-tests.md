# ETSI-Quellen, Tests und Konformität

Die dem Projekt beigelegten ETSI-PDFs sind **Referenzen für Protokoll und Testplanung**. Ein PDF im Repository, ein PDU-Parser oder ein erfolgreicher Mock-Test sind keine ETSI-Zertifizierung. Die [generierte PDU-Bestandsmatrix](../protocols/etsi-pdu-konformitaetsmatrix.md) zählt am dokumentierten Stand 77 erfasste PDU-Implementierungen, darunter unvollständige Pfade und viele Dateien ohne nachgewiesenen Testverweis. Änderungen am Code können diese Zahlen überholen.

| Themenfeld | Bereitgestellte Ausgabe | Bezug im Projekt |
|---|---|---|
| Netzdesign | [EN 300 392-1 V1.6.1](../en_30039201v010601p.pdf) | Systemaufbau, Begriffe und Netzfunktionen |
| Luftschnittstelle | [EN 300 392-2 V3.8.1](../en_30039202v030801p.pdf) | PHY/MAC/MLE/MM/CMCE, Nachrichten und Timing |
| Sicherheitsfunktionen | [EN 300 392-7 V3.5.1](../en_30039207v030501p.pdf) | Authentisierung und Schutzmechanismen |
| Konformitätstest | [EN 300 394-1 V3.3.1](../en_30039401v030301p.pdf) | Interoperabilitäts-/Testfälle |
| Peripherieschnittstelle | [EN 300 392-5 V2.7.1](../en_30039205v020701p.pdf) | Peripherieschnittstelle und zugehörige Protokolle |
| Sprachcodec | [EN 300 395-2 V1.3.3](../en_30039502v010303p.pdf) | Sprachformat und Codec-Pfad |
| SIM/TSIM | [EN 300 812 V2.1.1](../en_300812v020101p.pdf), [ES 200 812-2 V2.4.1](../es_20081202v020401m.pdf) | Security-/UICC-Schnittstellen; kein Nachweis einer vorhandenen TSIM-Integration |
| ISI | [EN 300 392-3-3 V1.3.1](../en_3003920303v010301p.pdf), [3-4 V1.3.1](../en_3003920304v010301p.pdf), [3-15 V1.5.0](../en_3003920315v010500a.pdf) | Gruppenruf, SDS und Mobility an Inter-System-Grenzen |
| Zusatzdienste | [EN 300 392-9 V1.7.1](../en_30039209v010701p.pdf) und Teil 10/11/12 | Funktionen wie Call Identification, Late Entry oder Priorität |

## Testkette

1. Normstelle samt **Version und Abschnitt** einer Funktion zuordnen.
2. PDU-Parser/Encoder, Runtime-State-Machine und Fehlerpfad im Code finden.
3. Golden Vectors und Negativtests mit reproduzierbarem Build durchführen.
4. Mehrere Endgeräte/Softwarestände mit Air-Logs und SDR-/Timing-Messungen prüfen.
5. Integration mit Backend, Ausfall und Release abnehmen; Ergebnis mit Commit und Messbelegen archivieren.

Die [statische Matrix](../protocols/etsi-pdu-konformitaetsmatrix.md) bewertet Codec-Einstiegspunkte, nicht die gesamte Funk- oder Security-Laufzeit. [Inbetriebnahme und Abnahme](inbetriebnahme-und-abnahme.md) beschreibt einen Testplan für eine konkrete Anlage. Die mitgelieferten ETSI-Versionen sind projektbezogene Arbeitsstände; ihre Versionsnummer allein behauptet keine Aktualität aller TETRA-Normen.

Die hier genannten Ausgaben sind die eingecheckten Normreferenzen. Vor einer gezielten Protokolländerung Normstelle, konkrete PDF-Ausgabe und tatsächlich verwendeten Runtimepfad zusammen lesen. Die Zahl 77 stammt aus der verlinkten generierten Matrix und ist kein Fortschrittsprozentsatz.

## Quellen zur Pflege dieser Seite

[Generierte PDU-Matrix](../protocols/etsi-pdu-konformitaetsmatrix.md) · [Inventurgenerator](../../tools/protocol_inventory.py).
