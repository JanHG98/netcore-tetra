# MQTT Phase 5 – Home Assistant und Homematic IP

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: MQTT Phase 5 – Home Assistant und Homematic IP. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Die gepflegte Implementierung und die aktuelle Betriebsanleitung stehen unter [iot-gateway](../../services/iot-gateway/README.md) und [Quellcode](../../../system-backend/iot-gateway). Gemeinsame Ereignis- und Befehlsnamen bleiben durch [Schnittstellenverträge](../../contracts/README.md) definiert; spätere Richtlinien-/HA-/HmIP-Erweiterungen dürfen nicht mit dem reinen Beobachtungsmodus der Phase 3 verwechselt werden.

Phasennummern, damalige Ports, Testidentitäten und Befehle dokumentieren die Einführung. Neue Installationen folgen der heutigen Komponentenbeschreibung und dem tatsächlich gewählten Inventory.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### MQTT Phase 5 – Home Assistant und Homematic IP

Phase 5 erweitert den IoT Gateway um:

- Home Assistant MQTT Discovery;
- erneute Discovery nach `homeassistant/status = online`;
- einfache HA-Command-Topics für virtuelle Lab-Geräte;
- normalisierten State-Ingress für ausgewählte Home-Assistant-/HmIP-Entitäten;
- optionales direktes CCU-/RaspberryMatic-Polling per XML-RPC;
- explizite, mehrfach gesperrte Vorbereitung direkter Schreibzugriffe;
- WebUI/API für Discovery, importierte Entitäten und Homematic-Datenpunkte.

Die Stufe bleibt OPEN LAB. Reale Aktionen sind standardmäßig deaktiviert.
