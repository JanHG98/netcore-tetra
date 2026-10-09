# MQTT-Vertrag Phase 5

```text
netcore/v1/events/<domain>/<action>
netcore/v1/state/<subject-type>/<subject-id>
netcore/v1/commands/#
netcore/v1/acks/<command-id>
netcore/v1/integrations/homeassistant/state
netcore/v1/integrations/homeassistant/commands/#
netcore/v1/integrations/homeassistant/command-egress
netcore/v1/state/homematic/<datapoint-id>
homeassistant/<component>/netcore_tetra/<object-id>/config
homeassistant/status
```

Discovery- und Zustandsnachrichten sind retained. Aktionsbefehle sind nicht retained und laufen durch das persistente Command-Ledger.
