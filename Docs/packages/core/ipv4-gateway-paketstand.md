# IP Gateway – IPv4-Anbindung – SWMI-Paketstand

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: SWMI Core 1 / Paket H. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Für ip-gateway und seine Dienstgrenzen sind [ip-gateway](../../services/ip-gateway/README.md) und [Quellcode](../../../system-backend/ip-gateway) die aktuellen Einstiege. Die damaligen Paketlisten beschreiben keine heute vollständig installierte Anlage.

Die Beschreibungen „umgesetzt“, „noch nicht enthalten“ und „nächster Baustein“ gelten innerhalb der damaligen Paketfolge. Sie ersetzen keinen Abgleich mit dem heutigen Quellcode oder mit der installierten Konfiguration.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### SWMI Core 1 – Paket H: IP Gateway

## Ergebnis

Der neue LXC-Dienst `system-backend/ip-gateway/` schließt den allgemeinen IPv4-Datenweg oberhalb des Packet Core. Er verwendet ein Linux-TUN-Interface, weil SNDCP rohe IP-N-PDUs und keine Ethernet-Frames transportiert.

## Enthalten

- WebUI und REST-API auf Port 8170
- Shadow-/Authoritative-Modus
- Packet-Core-Polling für Kontexte und Uplink-Outbox
- Delete/ACK erst nach erfolgreichem TUN-Write
- IPv4-Zieladressauflösung auf ISSI und NSAPI
- Downlink-Retry-Queue vor Übergabe an den Packet Core
- IPv4-Forwarding und verwaltete Routen
- eigene nftables-Tabellen für Filter und NAT
- Masquerading, SNAT, DNAT und Flow-Block
- DNS-Forwarder und statische A-Records
- WML/WAP-, HTTP- und UDP-Testdienste
- Flow-Tabelle und PCAP Classic mit `DLT_RAW`
- persistente Regeln, Blocklisten, Captures, Events und Backups
- systemd-Härtung plus gezielte Linux-Capabilities

## Nicht verschoben

PHY, MAC, LLC, PDCH, SNDCP-PDU-Coding, Fragmentierung und Mobility Anchors bleiben in TBS beziehungsweise Packet Core. Der IP Gateway ist ein Layer-3-Gateway und keine zweite SNDCP-State-Machine.

## Betriebsgrenze

`shadow` verändert den Kernel nicht. `authoritative` öffnet `/dev/net/tun`, setzt Interface/Adresse/MTU, aktiviert IPv4-Forwarding und reconciliert die eigenen nftables-Tabellen. Wegen Open-Lab ohne Authentisierung nur im isolierten Testnetz einsetzen.
