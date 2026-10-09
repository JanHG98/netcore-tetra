# MQTT Phase 5 – Home Assistant und Homematic IP

Phase 5 erweitert den IoT Gateway um:

- Home Assistant MQTT Discovery;
- erneute Discovery nach `homeassistant/status = online`;
- einfache HA-Command-Topics für virtuelle Lab-Geräte;
- normalisierten State-Ingress für ausgewählte Home-Assistant-/HmIP-Entitäten;
- optionales direktes CCU-/RaspberryMatic-Polling per XML-RPC;
- explizite, mehrfach gesperrte Vorbereitung direkter Schreibzugriffe;
- WebUI/API für Discovery, importierte Entitäten und Homematic-Datenpunkte.

Die Stufe bleibt OPEN LAB. Reale Aktionen sind standardmäßig deaktiviert.
