# Transit – Zwei Regionen im Labor testen

Bei installierten LXCs die tatsächlichen WebUI-/Listener-Adressen aus `/etc/netcore/lxc-network.env` verwenden; `127.0.0.1` im Curl-Beispiel setzt einen Loopback- oder Wildcard-Listener voraus.

Dieser Test prüft den nativen `netcore-transit-v1`-Pfad. Er ist **noch kein ETSI ISI**- oder On-Air-Test. Vorausgesetzt werden zwei isolierte Lab-Instanzen mit dem gleichen Code und gegenseitiger HTTP-Erreichbarkeit.

## Instanzen vorbereiten

- Region A: `region_id="region-a"`, eigene `swmi_id`, Port 8200.
- Region B: `region_id="region-b"`, eigene `swmi_id`, Port 8210 bei gleichem Host; auf getrennten LXCs kann ebenfalls 8200 verwendet werden.
- `server.bind` und `region.advertised_endpoint` pro Instanz anpassen. Ein `--bind`-Override allein ändert den advertised endpoint nicht.
- Bei zwei Prozessen auf demselben Host unterschiedliche `storage.database_path` und `backup_path` verwenden, damit keine gemeinsame Zustandsdatei überschrieben wird.
- Zu Beginn beide Instanzen in `shadow` starten.

## Peers und Routing

1. Auf beiden Seiten `POST /api/v1/peers` verwenden. Pflichtfelder: `peer_id`, fremde `region_id`, fremde `swmi_id`, `display_name`, erreichbarer `endpoint` und `capabilities`, zum Beispiel `["sds"]`.
2. Je Seite eine Default-Route über den fremden Peer mit `POST /api/v1/routes` anlegen: `service="sds"`, `selector_type="default"`, `selector_value=""`, `destination_region` und `peer_id`.
3. `POST /api/v1/route/resolve` mit Service, `destination_kind="issi"`, Ziel-ISSI als String, expliziter Zielregion und `trace=[]` prüfen. Direkte Peers sind standardmäßig zusätzlich Routingkandidaten.
4. Den Modus in beiden Startkonfigurationen auf `authoritative` ändern und beide Dienste neu starten. Es gibt keine Transit-Policy-API für den Laufzeitwechsel.
5. Heartbeats sowie Peer-Zustände über `/api/v1/peers` prüfen; die Beispiel-Peer-Timeouts betragen 20 Sekunden.

## SDS und lokale Anwendung

In Region A eine Lab-ISSI nach Region B mit `POST /api/v1/locations/subscribers` abbilden. Dazu `issi`, `home_region` und `current_region="region-b"` angeben. Die Registrierung ausschließlich auf Region B würde Region A ohne explizite Zielregion nicht über deren Aufenthaltsort informieren.

Danach den Auftrag in Region A senden; Adresse und Port auf die eigene Lab-Instanz anpassen:

```bash
curl --fail-with-body http://127.0.0.1:8200/api/v1/transit/submit \
  -H 'Content-Type: application/json' \
  -d '{"service":"sds","operation":"lab_test","source_kind":"issi","source":"4010001","destination_kind":"issi","destination":"4010002","target_region":"region-b","ttl_secs":60,"payload":{"text":"Transit-Labtest"}}'
```

Region B muss über `GET /api/v1/local-deliveries?service=sds` eine pending Zustellung liefern. Nach tatsächlicher Anwendung durch den Lab-Consumer `POST /api/v1/local-deliveries/{delivery_id}/ack` mit `{"success":true}` senden. Session, Outbound-Queue und Events auf beiden Seiten prüfen.

## Fehlerfälle und Nachweis

- Einen zweiten eigenständigen Pfad konfigurieren und den primären Peer deaktivieren beziehungsweise dessen Transport ausfallen lassen. Session-Leg und Queue müssen den vorgesehenen Failover oder einen klaren Fehler zeigen.
- Einen gültigen Peer-Envelope mit lokaler Region bereits im `trace` einspeisen: Loop-Ablehnung erwarten.
- Eine akzeptierte `dedupe_key` erneut senden: erfolgreiche Antwort mit `duplicate=true`, keine zweite Local Delivery erwarten.
- Version, Region-IDs, Konfiguration, ausgewählten Peer, Zustell-ID, ACK und beobachtete Fehler festhalten. Ein bloßer Modeltest ist kein Zweiregionen-Nachweis.

Aus dem Repository-Hauptverzeichnis ist der kleine Referenztest verfügbar:

```bash
python3 system-backend/transit/tests/transit_reference.py
```

**Quellabgleich vom 9. Oktober 2026:** [JSON-Felder](../../../../system-backend/transit/src/protocol.rs), [API-Routen](../../../../system-backend/transit/src/http.rs) und [Zielbestimmung](../../../../system-backend/transit/src/state.rs).
