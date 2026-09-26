# Paketdaten, IP Gateway und WAP

Der Paketdatenpfad besteht aus mehreren Zuständigkeiten: TBS/SNDCP auf der Luftschnittstelle, Packet Core für PDP-/NSAPI-Zustand und IP Gateway für TUN, Adressen, NAT, Firewall und Testdienste. Ein erfolgreiches `curl` von einem LXC zu einer WebUI belegt **keinen** Ende-zu-Ende-Paketdatenfluss vom Funkgerät.

## Testfolge

1. Endgerät fordert Packet Data an; TBS erkennt SNDCP-Aktivität und die zugehörige ISSI.
2. Packet Core erzeugt einen passenden PDP-Kontext/NSAPI; State-Machine, Fragmentierung und Flow Control prüfen.
3. IP Gateway erhält Lease und Flow, TUN-Gerät ist im LXC freigegeben, Route/NAT/Firewall passen.
4. IP-Datenpaket erreicht einen klar begrenzten Testdienst und der Rückweg trifft denselben Kontext.
5. Nach Deregistration/Timeout werden Kontext, Lease und Flow kontrolliert freigegeben.

Die [Packet-Core-Dokumentation](https://github.com/JanHG98/netcore-tetra/tree/main/system-backend/packet-core/docs) und die [IP-Gateway-Dokumentation](https://github.com/JanHG98/netcore-tetra/tree/main/system-backend/ip-gateway/docs) beschreiben Verträge, Testpfade und LXC-Voraussetzungen. Für TUN ist `/dev/net/tun`-Passthrough nötig. [[Open-Lab-Deployment]]

## WAP-Testdienste

Die IP-Gateway-Vorlage führt unter anderem `wap.netcore.test:8088/wap/` und `/wap/status.wml` als einfache WML-Testseiten. Das TBS-interne `[cell_info.wap_ip]` kennt separat einen beispielhaften Port `9200`. Diese beiden Einstellungen sind **verschiedene Komponenten** und müssen im konkreten Datenpfad zueinander passen; keinen der Ports mit dem IP-Gateway-Managementport `8170` verwechseln. [IP-Gateway-Testpfade](https://github.com/JanHG98/netcore-tetra/blob/main/system-backend/ip-gateway/docs/dns-wap-test.md) · [WAP-Port-Spezifikation](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/wap-port-spec.md).

## Fehler eingrenzen

- Kein PDP-Kontext: Funk-/SNDCP-Logs und Teilnehmerpolicy vor NAT untersuchen.
- PDP steht, kein Ping/HTTP: TUN, Adresspool, Route, NAT/Firewall, DNS und Rückweg prüfen.
- Nur WAP scheitert: Gerätetyp, APN/Proxy-Konfiguration, WML-URL, DNS und den **richtigen** WAP-Testserver prüfen.
- Daten bleiben nach Abbruch hängen: Context/Lease/Flow gemeinsam beobachten, bevor Timer verkürzt werden.

Mit Testadressen und begrenzten Diensten anfangen. [[Netzwerk-und-Ports]] · [[Abnahme]]
