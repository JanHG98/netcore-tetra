# Packet Capture

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/state.rs](../../../system-backend/ip-gateway/src/state.rs) · [src/dataplane.rs](../../../system-backend/ip-gateway/src/dataplane.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

Captures werden direkt am Übergang zwischen Packet Core und TUN erzeugt. Dadurch enthalten sie die vollständigen IPv4-N-PDUs vor beziehungsweise nach Linux-Routing und ohne künstliche Ethernet-Header.

## Format

- PCAP Classic
- Link-Type `DLT_RAW` (101)
- Mikrosekunden-Zeitstempel
- konfigurierbare Snaplen
- Größenlimit pro Datei

## Filter

Ein Capture kann filtern nach:

- `uplink`, `downlink` oder `both`
- IPv4-Host
- `tcp`, `udp` oder `icmp`
- Quell- oder Zielport

Der Download erfolgt über die WebUI oder:

```text
GET /api/v1/captures/{id}/download
```

Bei Neustart werden aktive Captures sauber als gestoppt markiert. Dateien werden nicht automatisch gelöscht.

## Beobachtungsgrenze

Diese Captures enthalten IP-Pakete am Anwendungsübergang, keine Air-PDUs und keine Kernel-/NIC-Vollmitschnitte. Ein Paket im Capture ist deshalb kein Beleg der späteren Funkzustellung. Die Snaplen kann Nutzdaten kürzen; das Größenlimit kann die Aufnahme stoppen. Shadow öffnet keinen TUN-Datenpfad und liefert ohne eingespeiste beziehungsweise tatsächlich transportierte Pakete keinen normalen Funkverkehrsmitschnitt.
