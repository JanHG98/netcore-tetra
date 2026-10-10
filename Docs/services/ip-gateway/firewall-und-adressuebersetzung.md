# Firewall und NAT

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/kernel.rs](../../../system-backend/ip-gateway/src/kernel.rs) · [src/state.rs](../../../system-backend/ip-gateway/src/state.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

## Baseline

Die generierte Forward-Chain prüft zuerst Operator-Blocklisten für Quell- und Zieladressen. Erst danach folgt – falls aktiviert – `established,related`, gefolgt von benutzerdefinierten Regeln nach aufsteigender Priorität. So kann eine neu gesetzte Blockadresse auch einen bereits bestehenden Flow treffen.

Im Open-Lab-Beispiel ist allgemeiner ausgehender IPv4-Verkehr erlaubt:

```toml
[firewall]
allow_general_internet = true
```

Für einen restriktiven Testserverbetrieb auf `false` setzen und gezielte Forward-Regeln anlegen.

## Lokale Dienste

Vom TUN-Interface zum Gateway werden standardmäßig DNS, HTTP/WAP, UDP-Echo und optional ICMP zugelassen. Das Management-WebUI auf Port 8170 ist nicht automatisch aus dem TETRA-Paketdatennetz freigegeben.

## NAT-Typen

- `masquerade`: dynamische Quelladressübersetzung am Egress-Interface
- `snat`: feste Quelladresse, optional mit Port
- `dnat`: feste Zieladresse, optional mit Port

Regeln werden vollständig aus der Datenbank gerendert. Freitext-nftables wird absichtlich nicht akzeptiert; die API validiert CIDRs, Protokolle, Ports und Interface-Namen.

## Geltungsbereich

Die eigenen nftables-Tabellen werden im Authoritative-Modus ersetzt; vorhandene Regeln anderer Tabellen bleiben bestehen und können den Verkehr zusätzlich sperren. Die Input-Chain sperrt nicht ausdrücklich freigegebenen Verkehr **vom TUN-Interface**. Sie ist keine vollständige Firewall für das Management-Interface. In Shadow werden nur Datenbank und Kernelplan geändert, keine Linux-Regeln angewendet.
