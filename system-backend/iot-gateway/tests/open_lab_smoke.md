# Phase 5 OPEN-LAB-Smoke-Test

1. IoT Gateway und Mosquitto laufen.
2. `homeassistant/#` zeigt retained Discovery-Nachrichten.
3. Home Assistant legt Gateway, Quellen und drei virtuelle Lab-Geräte an.
4. Schalten des virtuellen Relais erzeugt `accepted`, `executing`, `succeeded`.
5. Ein Zustand auf `netcore/v1/integrations/homeassistant/state` erscheint unter `/api/v1/home-assistant/entities`.
6. Ein retained Command wird abgewiesen.
7. Ein Ziel außerhalb `lab-*` wird per Default Deny abgewiesen.
8. Direkter Homematic-Write bleibt ohne drei bewusste Freigaben gesperrt.
