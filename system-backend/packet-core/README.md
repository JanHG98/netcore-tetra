# NetCore Packet Core

Der Packet Core ist der zentrale SwMI-Dienst für **SNDCP-Kontexte, Packet-Data-Zustände und Mobility Anchoring**. Er übernimmt die langlebige Netzsicht oberhalb der lokalen TBS-SNDCP-Instanz; PHY, MAC, LLC, lokale PDCH-Zuteilung und die konkrete Air-PDU bleiben bewusst an der TBS.

Lokale Dateien: [src](src/) · [config](config/) · [install](install/) · [systemd](systemd/).

[Vollständige Dokumentation](../../Docs/services/packet-core/README.md) · [Zentraler Dokumentationsindex](../../Docs/README.md)

[Online lesen](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/services/packet-core/README.md).

Standardport `8160`; startet im Shadow-Modus. Zentrale Zustände und HTTP-Referenzaktionen ersetzen noch keinen vollständig angebundenen Funkdatenpfad.
