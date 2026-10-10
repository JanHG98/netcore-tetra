# Phase 6 – Hardware-I/O und Racküberwachung

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: Phase 6 – Hardware-I/O und Racküberwachung. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Die gepflegte Implementierung und die aktuelle Betriebsanleitung stehen unter [hardware-gateway](../../services/hardware-gateway/README.md) und [Quellcode](../../../system-backend/hardware-gateway). DSP-/Telemetriewerte, kalibrierte Antennenmessung und physische Aktorfreigabe bleiben getrennte Aussagen.

Phasennummern, damalige Ports, Testidentitäten und Befehle dokumentieren die Einführung. Neue Installationen folgen der heutigen Komponentenbeschreibung und dem tatsächlich gewählten Inventory.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### Phase 6 – Hardware-I/O und Racküberwachung

Neuer Dienst: `system-backend/hardware-gateway`.

Er sammelt MQTT-/HTTP-Telemetrie von Edge-Nodes, überwacht Heartbeats und Grenzwerte, veröffentlicht retained Zustände und erzeugt normalisierte `netcore-event-v1` Ereignisse. OPEN LAB bleibt aktiv; physische Ausgänge sind standardmäßig deaktiviert.
