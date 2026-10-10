# Mobility Core – zentrale Teilnehmerlage – SWMI-Paketstand

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: SWMI Mobility 1 / Paket E. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Die aktuellen Implementierungseinstiege sind [mobility-core](../../services/mobility-core/README.md) und [Quellcode](../../../system-backend/mobility-core). Lokale Zellwechsel-/Restore-Bausteine und ein Quelltest belegen kein unterbrechungsfreies Handover laufender Gespräche auf der Anlage.

Die Beschreibungen „umgesetzt“, „noch nicht enthalten“ und „nächster Baustein“ gelten innerhalb der damaligen Paketfolge. Sie ersetzen keinen Abgleich mit dem heutigen Quellcode oder mit der installierten Konfiguration.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### SWMI Mobility 1 – Paket E: Mobility Core

## Ziel

Dieses Paket führt den zweiten deploybaren Backend-LXC ein:

```text
system-backend/mobility-core/
```

Der Dienst verbindet sich mit dem offenen Backend-WebSocket des Node Gateway, baut aus TBS-Telemetrie eine zentrale Teilnehmerlage auf und koordiniert MM-Context-Transfers zwischen zwei Basisstationen.

## Offener Testmodus

In der Testumgebung werden bewusst noch keine Tokens, Benutzerkonten, Passwörter, Zertifikate oder TLS-Verbindungen verwendet.

- WebUI/API: `http://<LXC-IP>:8090/`
- Node Gateway: `ws://<NODE-GATEWAY-IP>:8080/ws/backend`
- `security.mode = "open_lab"`
- andere Security-Modi werden beim Start abgewiesen

## Transferprotokoll

Der Mobility Core verwendet drei neue typisierte TBS-Kommandos:

1. `MobilityExportContext`
2. `MobilityImportContext`
3. `MobilityRemoveContext`

Die Quelle wird erst entfernt, wenn die Ziel-TBS den Import bestätigt hat.

Der Node Gateway gibt für Backend-Kommandos nun eine strukturierte `request_id` und `command_id` zurück. Die anschließende `ControlResponse` der TBS wird über die `command_id` eindeutig dem Transfer zugeordnet.

## Übertragener MM-Kontext

- Home-ISSI
- Registrierungszustand
- Gruppenaffiliationen
- Energy-Saving-Mode
- Monitoring Frame und Multiframe
- Class of MS
- letzter Layer-2-Handle
- TEI

## WebUI

Die eigene WebUI bietet:

- Gateway- und TBS-Status
- zentrale Teilnehmerlage
- Serving Node
- Gruppen, Energy-Saving und RSSI
- Transferstart
- Transferphasen
- kontrollierten Abbruch vor abgeschlossenem Zielimport
- Fehler-, Timeout- und Ereignisanzeige

## Bewusste Grenzen

- In-Memory-State; PostgreSQL folgt mit dem späteren Datenbank-/Subscriber-Core-Ausbau.
- Kein automatischer RF-basierter Handover-Entscheider.
- Kein Token-/TLS-/RBAC-Betrieb.
- Aktive Calls werden weiterhin durch die lokale MLE-/CMCE-Call-Restore-State-Machine behandelt.
