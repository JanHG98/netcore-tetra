# Fragmentierung, Reassembly und Flow Control

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/state.rs](../../../system-backend/packet-core/src/state.rs) · [src/http.rs](../../../system-backend/packet-core/src/http.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

## Reassembly

Ein Datagramm wird über Node, ISSI, NSAPI, Richtung und Datagramm-ID identifiziert. Segmente werden nach Offset sortiert. Überlappende Fragmente werden standardmäßig abgewiesen, weil „last fragment wins“ bei Netzprotokollen eine wunderbare Quelle für sehr unschöne Überraschungen ist.

Grenzen:

- maximale parallele Datagramme,
- maximale Gesamtbytes,
- maximale Fragmente je Datagramm,
- Timeout je Reassembly.

Vollständige N-PDUs landen verlustfrei in der N-PDU-Outbox.

## Downlink

`POST /api/v1/downlink` ordnet das N-PDU einem Kontext zu, prüft Payload- und Queue-Limits und zerlegt es anhand der Kontext-MTU. Bei STANDBY oder QUIESCENT wird eine Page/Wake-Aktion erzeugt und der Kontext nach RESPONSE_WAITING überführt. Bereits wartende Wake-Aktionen werden nicht doppelt eingeplant; nicht verfügbare beziehungsweise suspendierte Kontexte sind gesondert zu behandeln.

## Flow Control

Jeder Kontext besitzt Paket-, Byte- und TTL-Grenzen. Aktionen haben Retry-Intervall, maximale Versuchszahl und eindeutige Sequenzen. Damit ist Rückstau sichtbar und nicht bloß ein schwarzes Loch mit optimistischem Logeintrag.

## Annahme und Verbrauch

Die N-PDU-Outbox bleibt bis zum expliziten DELETE durch den Verbraucher erhalten; der IP Gateway löscht einen Uplink-Eintrag erst nach erfolgreichem TUN-Write. Die Downlink-Fragmente stehen als versionierte Aktionen bereit. Die bestehende Gateway-Bridge transportiert diese Fragmente noch nicht vollständig zum TBS-Funkstack; Queue- und ACK-Tests an der HTTP-Schnittstelle sind deshalb von Funkzustelltests zu unterscheiden.
