# Transit – Vermittlung zwischen NetCore-Regionen

Transit vermittelt semantische Mobility-, Einzelruf-, Gruppenruf-, SDS-, Media- und Supplementary-Service-Ereignisse zwischen eigenständigen NetCore-Regionen. Das native Protokoll heißt `netcore-transit-v1`: Es ist **noch kein ETSI ISI** und bestätigt keine Interoperabilität mit fremden SwMIs.

**Stand: 9. Oktober 2026.** Abgeglichen mit [Konfiguration](../../../system-backend/transit/config/transit.example.toml), [Routing und Zuständen](../../../system-backend/transit/src/state.rs) und [HTTP-Transport](../../../system-backend/transit/src/transport.rs). Beschrieben ist der vorhandene Lab-Code, keine produktive WAN- oder On-Air-Abnahme.

## Vorhandene Funktionen

- Regionen und Peer-Links mit Heartbeat, Latenz, administrativem und Betriebszustand;
- Routen nach Dienst, Zielregion, ISSI/GSSI, Präfix oder Default sowie transitive Weiterleitung;
- Teilnehmerregionen und Gruppenreichweite, Sessions mit einem Leg je Zielregion;
- Path Vector (`trace`), Hop Limit, Deduplizierung, Retry, TTL und Failover;
- persistente Outbound- und Local-Delivery-Queues, Neustartbehandlung;
- Weboberfläche, REST-API, OpenAPI, Metrics, Export und Metadatenbackup;
- Installer und systemd-Unit.

## Start und Betriebsmodus

Aus dem Repository-Hauptverzeichnis auf einem Linux-Lab-Host mit systemd, `iproute2` und passender Rust-/Cargo-Toolchain:

```bash
sudo system-backend/transit/install/install.sh
systemctl status netcore-transit
source /etc/netcore/lxc-network.env
curl --fail "${NETCORE_WEBUI_URL}health/ready"
```

Die Beispielkonfiguration bindet zunächst an `0.0.0.0:8200` und beginnt in `shadow`; der Installer setzt `bind` und `advertised_endpoint` auf die erkannte LXC-IPv4-Adresse. Region-ID, SwMI-ID und den von anderen Regionen erreichbaren `advertised_endpoint` für jede Instanz passend einstellen. Der Installer verwendet `/etc/netcore/transit.toml`, `/opt/netcore-transit/bin/netcore-transit` und `/var/lib/netcore-transit/`.

`shadow` berechnet und persistiert den vorgesehenen Transit, sendet aber keine Heartbeats oder Envelopes. Ingress und Management bleiben erreichbar. `authoritative` aktiviert den HTTP-Peer-Versand. Der Betriebsmodus stammt aus der Startkonfiguration; Änderungen erfordern einen Neustart.

## Schnittstellen

| Schnittstelle | API-Pfad |
| --- | --- |
| Peer-Heartbeat / Envelope | `POST /api/v1/peer/heartbeat`, `POST /api/v1/peer/envelopes` |
| Lokaler Auftrag | `POST /api/v1/transit/submit` |
| Lokale Zustellungen / ACK | `GET /api/v1/local-deliveries`, `POST /api/v1/local-deliveries/{id}/ack` |
| Routingprüfung | `POST /api/v1/route/resolve` |
| Metadatenbackup | `POST /api/v1/maintenance/backup` |

Eine Zustellung in die Peer-Queue und die Anwendung durch den lokalen Core sind getrennte Schritte. Ein Peer-HTTP-Erfolg bestätigt keinen abgeschlossenen Funkruf.

## Lab-Grenze und weitere Anleitungen

Management und Peers teilen TCP **8200** ohne Anmeldung, Tokens, TLS, mTLS oder signierte Peer-Identitäten. Nur im isolierten Lab-Netz betreiben. Der vorhandene Dienst implementiert keinen ETSI-ISI-Stack, keine standardisierten ISI-Media-Profile und keine WAN-Bandbreitenreservierung.

- [Architektur und Zustellung](architektur-und-zustellung.md)
- [Routing und Failover](routing-und-failover.md)
- [Grenze zu ETSI ISI](grenze-zu-etsi-isi.md)
- [Offene Testumgebung](offene-testumgebung.md)
- [Zwei-Regionen-Labtest](tests/zwei-regionen-labortest.md)
