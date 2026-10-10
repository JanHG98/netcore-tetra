# Paketdaten, IP Gateway und WAP

Der Paketdatenpfad besteht aus mehreren Zuständigkeiten: TBS/SNDCP auf der Luftschnittstelle, Packet Core für PDP-/NSAPI-Zustand und IP Gateway für TUN, Adressen, NAT, Firewall und Testdienste. Ein erfolgreiches `curl` von einem LXC zu einer WebUI belegt **keinen** Ende-zu-Ende-Paketdatenfluss vom Funkgerät.

## Testfolge

1. Endgerät fordert Packet Data an; TBS erkennt SNDCP-Aktivität und die zugehörige ISSI.
2. Packet Core erzeugt einen passenden PDP-Kontext/NSAPI; State-Machine, Fragmentierung und Flow Control prüfen.
3. IP Gateway erhält Lease und Flow, TUN-Gerät ist im LXC freigegeben, Route/NAT/Firewall passen.
4. IP-Datenpaket erreicht einen klar begrenzten Testdienst und der Rückweg trifft denselben Kontext.
5. Nach Deregistration/Timeout werden Kontext, Lease und Flow kontrolliert freigegeben.

Die [Packet-Core-Dokumentation](../services/packet-core/README.md) und die [IP-Gateway-Dokumentation](../services/ip-gateway/README.md) beschreiben Verträge, Testpfade und LXC-Voraussetzungen. Für TUN ist `/dev/net/tun`-Passthrough nötig. [Dienste und Pi-Images im Open Lab bereitstellen](dienste-und-pi-images-bereitstellen.md)

## WAP-Testdienste

Die IP-Gateway-Vorlage führt unter anderem `wap.netcore.test:8088/wap/` und `/wap/status.wml` als einfache WML-Testseiten. Das TBS-interne `[cell_info.wap_ip]` kennt separat einen beispielhaften Port `9200`. Diese beiden Einstellungen sind **verschiedene Komponenten** und müssen im konkreten Datenpfad zueinander passen; keinen der Ports mit dem IP-Gateway-Managementport `8170` verwechseln. [IP-Gateway-Testpfade](../services/ip-gateway/dns-wap-und-testdienste.md) · [WAP-Port-Spezifikation](../guides/wap/historischer-wap-portierungsvertrag.md).

## Fehler eingrenzen

- Kein PDP-Kontext: Funk-/SNDCP-Logs und Teilnehmerpolicy vor NAT untersuchen.
- PDP steht, kein Ping/HTTP: TUN, Adresspool, Route, NAT/Firewall, DNS und Rückweg prüfen.
- Nur WAP scheitert: Gerätetyp, APN/Proxy-Konfiguration, WML-URL, DNS und den **richtigen** WAP-Testserver prüfen.
- Daten bleiben nach Abbruch hängen: Context/Lease/Flow gemeinsam beobachten, bevor Timer verkürzt werden.

Mit Testadressen und begrenzten Diensten anfangen. [Netzwerk, Ports und Protokolle](netzwerk-und-ports.md) · [Inbetriebnahme und Abnahme](inbetriebnahme-und-abnahme.md)

## Lokaler und zentraler Datenpfad

Die TBS besitzt auch einen lokalen WAP-/SNDCP-Pfad. Ein funktionierender lokaler Test ist kein Nachweis der zentralen Packet-Core-/IP-Gateway-Integration. Z02.3 führt den Schutz vor unzulässigen TUN-Routen, Z02.4 den Abgleich der tatsächlich aktiven SAP-/SNDCP-/MLE-Pfade. [Gesamtroadmap](../roadmaps/gesamtroadmap.md)

## Quellen zur Pflege dieser Seite

[SNDCP-TBS-Pfad](../../crates/tetra-entities/src/sndcp/sndcp_bs.rs) · [Lokaler WAP-Pfad](../../crates/tetra-entities/src/sndcp/wap_ip.rs) · [IP-Gateway-Vorlage](../../system-backend/ip-gateway/config/ip-gateway.example.toml).
