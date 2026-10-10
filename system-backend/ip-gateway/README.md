# NetCore IP Gateway

Der IP Gateway ist der **Layer-3-Übergang zwischen dem zentralen Packet Core und normalen IPv4-Netzen**. Er übernimmt vollständige IP-N-PDUs aus der Packet-Core-Outbox, speist sie über ein Linux-TUN-Interface in den Kernel ein und liefert zum TETRA-Adresspool geroutete Pakete wieder an den passenden PDP-Kontext zurück.

Lokale Dateien: [src](src/) · [config](config/) · [install](install/) · [systemd](systemd/).

[Vollständige Dokumentation](../../Docs/services/ip-gateway/README.md) · [Zentraler Dokumentationsindex](../../Docs/README.md)

[Online lesen](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/services/ip-gateway/README.md).

Standardport `8170`; benötigt den Packet Core. Shadow-Modus erstellt den Kernelplan, Authoritative-Modus aktiviert TUN und Pakettransport.
