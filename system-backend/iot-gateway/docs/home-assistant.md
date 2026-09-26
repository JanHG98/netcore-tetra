# Home Assistant MQTT Adapter

## Discovery

Der Gateway veröffentlicht retained Discovery-Nachrichten unter:

```text
homeassistant/<component>/netcore_tetra/<object_id>/config
```

Nach MQTT-Verbindung und nach einer `online`-Meldung auf `homeassistant/status` wird die Discovery erneut in die Outbox gelegt.

## Testgeräte

```text
switch.netcore_virtual_relay_lab_relay_01
light.netcore_virtual_light_lab_light_01
button.netcore_virtual_button_lab_button_01
```

Die tatsächlichen Entity-IDs werden von Home Assistant vergeben; stabil sind die `unique_id`-Werte.

## Zustände aus Home Assistant importieren

Topic:

```text
netcore/v1/integrations/homeassistant/state
```

Payload:

```json
{
  "entity_id": "binary_sensor.wassermelder_technikraum",
  "state": "off",
  "attributes": {
    "device_class": "moisture",
    "friendly_name": "Wassermelder Technikraum"
  },
  "device": {
    "manufacturer": "eQ-3"
  },
  "observed_at": "2026-08-01T13:00:00Z"
}
```

Der normalisierte retained Zustand erscheint unter:

```text
netcore/v1/state/integrations/homeassistant/binary_sensor.wassermelder_technikraum
```

## Aktionen in Home Assistant

Der Egress ist vorbereitet, aber standardmäßig deaktiviert. Erst bei `allow_command_egress = true` erzeugt ein policy-erlaubter `homeassistant.entity.command` eine Nachricht auf dem konfigurierten Command-Egress-Topic. Die Ausführung muss zusätzlich durch eine bewusst begrenzte Home-Assistant-Automation erfolgen.
