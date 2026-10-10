# IoT-Anbindung

**Quellen:** [system-backend/iot-gateway/examples/home-assistant](../../../../../system-backend/iot-gateway/examples/home-assistant) · [Repository-Root](../../../../..). Bei Befehlen das in der Anleitung angegebene Arbeitsverzeichnis beachten.

Stand: **9. Oktober 2026**. Die Anleitung beschreibt die im Hauptzweig vorhandene Umsetzung; Anlagen- und Funkabnahmen stehen in der [Gesamtroadmap](../../../../roadmaps/gesamtroadmap.md).

`state-bridge.yaml` spiegelt nur explizit ausgewählte Entitäten zum IoT Gateway. Dadurch wird nicht der komplette Home-Assistant-Zustandsbus in MQTT gekippt.

`limited-command-egress.yaml` ist eine bewusst eng begrenzte OPEN-LAB-Demo. Sie ist nicht erforderlich, um die von NetCore per MQTT Discovery angelegten virtuellen Geräte zu bedienen.
