# MQTT, Home Assistant und Homematic

Im zentralen Aufbau sammelt **IoT Gateway** Ereignisse, veröffentlicht Zustände/Events per MQTT und erzeugt Home-Assistant-Discovery. Die reale Ausführung von HA-/Homematic-Aktionen ist in der [Open-Lab-Vorlage](https://github.com/JanHG98/netcore-tetra/blob/main/system-backend/iot-gateway/config/iot-gateway.example.toml) standardmäßig gesperrt. Eine verbundene Broker-Session allein bestätigt keinen SDS-Datenweg.

## Topic-Vertrag

| Muster | Zweck |
|---|---|
| `netcore/v1/events/<domain>/<action>` | Ereignisse; nicht retained |
| `netcore/v1/state/<subject-type>/<subject-id>` | zuletzt bekannter Zustand; retained |
| `netcore/v1/commands/#` | eingehende Befehle, Ledger/Policy |
| `netcore/v1/acks/<command-id>` | Lebenszyklusquittungen |
| `homeassistant/<component>/netcore_tetra/<object_id>/config` | HA MQTT Discovery; retained |
| `homeassistant/status` | HA-Online-Meldung löst erneute Discovery aus |
| `netcore/v1/integrations/homeassistant/state` | HA-Zustandsimport nach NetCore |
| `netcore/v1/integrations/homeassistant/command-egress` | vorbereiteter, standardmäßig deaktivierter HA-Aktionsausgang |

Vollständiger [MQTT-Vertrag](https://github.com/JanHG98/netcore-tetra/blob/main/system-backend/iot-gateway/docs/mqtt-contract.md) und [HA-Adapter](https://github.com/JanHG98/netcore-tetra/blob/main/system-backend/iot-gateway/docs/home-assistant.md). Die tatsächliche HA-`entity_id` kann von HA vergeben werden; für Discovery ist `unique_id` stabiler.

## SDS bis zur HA-Entität verfolgen

1. **Funk:** Eingehende SDS/U-STATUS mit Quelle, Ziel, Zeitpunkt und Inhalt im TBS-Log belegen. `4010001` ist im bestehenden TBS-Kontext eine System-ISSI; sie ist nicht automatisch ein MQTT-Ziel.
2. **Routing:** Prüfen, ob die Nachricht lokal verarbeitet oder an SDS Router/Backend exportiert wurde; Node Gateway und Service-Matrix müssen erreichbar sein.
3. **IoT Gateway:** Event-Quelle, Normalisierung, Outbox und Fehlerzähler prüfen. Eine HA-Discovery-Nachricht ist noch keine SDS-Nachricht.
4. **Broker:** Mit einem zugelassenen Testclient auf dem konkreten Topic abonnieren, z. B. `mosquitto_sub -h <BROKER> -t 'netcore/v1/#' -v`; Broker-ACL und tatsächlichen `topic_prefix` beachten.
5. **Home Assistant:** MQTT-Integration, Discovery-Entity, Sensor-/Automationstrigger und HA-Logs prüfen. Eine Automation verarbeitet nur das Topic/Payload, das sie tatsächlich abonniert.

Bei jedem Schritt dieselbe Test-Nachricht und Zeitreferenz benutzen. Wenn Schritt 1 funktioniert und am Broker nichts ankommt, liegt das Problem **vor** der HA-Automation. Wenn MQTT Daten zeigt, aber HA nichts, dort Topic, JSON-Schema, Trigger, Retained-Verhalten und Automation-Syntax prüfen. [[Troubleshooting]]

## Befehle und Homematic

`[commands]` arbeitet mit `default_deny = true`, TTL, dedupliziertem Ledger und Acks. Die Beispiel-Policies erlauben nur virtuelle Laborgeräte. Für reale HA-Aktionen braucht es zusätzlich `allow_command_egress = true`, eine aktive eng gefasste Policy **und** eine bewusst begrenzte HA-Automation. Bei CCU/RaspberryMatic setzt direkte Schreibbarkeit zusätzlich eine passende Homematic-Konfiguration und pro Datenpunkt `writable = true` voraus; für Homematic IP Access Point ist die HA-Brücke der dokumentierte Weg.

Das Aktivieren eines HTTP-Managementports oder das Senden einer SDS an eine System-ISSI umgeht diese Grenzen nicht. [[Security-and-Operations]]
