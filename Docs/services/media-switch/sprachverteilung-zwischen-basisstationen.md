# Media-Routing

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/state.rs](../../../system-backend/media-switch/src/state.rs) · [src/http.rs](../../../system-backend/media-switch/src/http.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

Der Media Switch übernimmt revisionsgebundene Call-Snapshots über `/ws/media` des Call Control. `GET /api/v1/calls` dient zusätzlich als langsamer Abgleich und Reconnect-Sicherungsweg; im Konfigurationsbeispiel beträgt `call_control.reconcile_secs` 15 Sekunden. Aus aktiven logischen Calls und deren aktiven TBS-Legs entsteht ein Routingindex:

```text
(Node-ID, logischer Timeslot) -> Logical Call ID
```

Mit dem Standard `media.allow_same_leg_loopback = false` wird ein Uplink-Frame nicht an dasselbe Quell-Leg zurückgesendet. Ein abweichender Wert ist eine bewusste Laborkonfiguration. Alle anderen aktiven Legs derselben Session erhalten eine eigene Downlink-Kopie mit ihrem jeweiligen logischen Ziel-Timeslot. Offline-, nicht mediafähige oder stummgeschaltete Legs werden übersprungen und in den Diagnosezählern erfasst.

Call Control bleibt Eigentümer von Call-IDs, Legs, Floor und Restore. Der Media Switch erzeugt keine Calls und sendet selbst keine Air-Interface-Signalisierung.

## Freigabe der Route

Der Routinggraph muss alle erwarteten nicht-terminalen Legs mit aktiver lokaler Call-ID und Timeslot enthalten; auch die benötigten Gateway-Medienwege müssen verbunden sein. Erst dann bestätigt der Switch RouteReady an Call Control. Fehlen Legs, hält der Kaltstartpuffer die ersten Frames zeitlich und mengenmäßig begrenzt zurück. Fehlgeschlagene Legs müssen vom Eigentümer Call Control aus dem erwarteten Graphen entfernt werden.
