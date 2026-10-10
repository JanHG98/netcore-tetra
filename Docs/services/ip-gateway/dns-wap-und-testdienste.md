# DNS, WAP und Testdienste

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/dns.rs](../../../system-backend/ip-gateway/src/dns.rs) · [src/http.rs](../../../system-backend/ip-gateway/src/http.rs) · [src/runtime.rs](../../../system-backend/ip-gateway/src/runtime.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

## DNS

Der eingebaute UDP-DNS-Server beantwortet statische A-Records direkt und leitet unbekannte Anfragen an den konfigurierten Upstream weiter. Built-in-Einträge:

```text
netcore.test
wap.netcore.test
test.netcore.test
```

Alle zeigen auf die Gateway-Adresse des TUN-Netzes.

## WAP

Der Testserver stellt eine bewusst einfache WML-1.1-Seite bereit:

```text
http://wap.netcore.test:8088/wap/
http://wap.netcore.test:8088/wap/status.wml
```

Damit kann der bereits vorhandene Legacy-WAP-/SNDCP-Pfad ohne externen Webserver geprüft werden.

## Weitere Tests

```text
GET  http://test.netcore.test:8088/test/echo
GET  http://test.netcore.test:8088/test/info
UDP  test.netcore.test:7007
```

`/test/info` liefert die beobachtete Peer-Adresse und den aktuellen Gatewaystatus als JSON.

## Voraussetzungen

Der DNS-Listener wird nur im Authoritative-Modus am TUN-Gateway aktiviert. Im Beispiel bindet er an `10.0.0.1:53`; bei einem anderen Paketdatenpool mitändern. HTTP-Testserver (`test_server.bind`) und UDP-Echo besitzen eigene Listener und können auch ohne funktionierenden Funkpfad lokal erreichbar sein. Die erfolgreiche WML-URL beweist deshalb zunächst nur den Testserver, nicht automatisch SNDCP, Funktransport oder WAP-Kompatibilität eines bestimmten Geräts.
