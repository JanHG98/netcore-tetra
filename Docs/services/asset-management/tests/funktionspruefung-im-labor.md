# Inventar und Materialverwaltung: Funktionsprüfung im Labor

Manuelle Funktionsprüfung über die WebUI auf Port 8290; Testobjekte verwenden und ihren Zustand nach jedem Schritt prüfen. Diese Checkliste ist kein automatisch ausgeführter Repositorytest. Subscriber-/Mobility-Abgleich und MQTT erfordern erreichbare, passend konfigurierte Upstreams.


1. Person und Funkgerät anlegen.
2. Funkgerät an Person ausgeben.
3. Rückgabe durchführen.
4. Wartung planen und abschließen.
5. Subscriber-/Mobility-Abgleich auslösen.
6. MQTT-Topics `netcore/v1/events/asset/#` und `netcore/v1/state/assets/#` prüfen.

**Quellabgleich: 9. Oktober 2026.** [Quellcode](../../../../system-backend/asset-management) · [Konfigurationsvorlagen](../../../../system-backend/asset-management/config).
