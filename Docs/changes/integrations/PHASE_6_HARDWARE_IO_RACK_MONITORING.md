# Phase 6 – Hardware-I/O und Racküberwachung

Neuer Dienst: `system-backend/hardware-gateway`.

Er sammelt MQTT-/HTTP-Telemetrie von Edge-Nodes, überwacht Heartbeats und Grenzwerte, veröffentlicht retained Zustände und erzeugt normalisierte `netcore-event-v1` Ereignisse. OPEN LAB bleibt aktiv; physische Ausgänge sind standardmäßig deaktiviert.
