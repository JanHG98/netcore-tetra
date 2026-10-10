# Call Restore über mehrere TBS

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/state.rs](../../../system-backend/call-control/src/state.rs) · [src/http.rs](../../../system-backend/call-control/src/http.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

Der zentrale Ablauf besteht aus:

1. aktiven Restore Context auf der Quell-TBS exportieren,
2. Context auf der Ziel-TBS installieren,
3. auf den tatsächlichen Restore des Funkgeräts und die Ziel-Telemetrie warten,
4. das beobachtete lokale Ziel-Leg dem bestehenden logischen Call zuordnen und Restore als abgeschlossen markieren,
5. anschließend den temporären Restore Context auf der Ziel-TBS korreliert entfernen.

Der Restore-Vorgang kennt `ExportQueued`, `ExportRequested`, `ImportQueued`, `ImportRequested`, `Ready`, `Completed`, `Cancelled`, `Failed` und `TimedOut`.

Ein Platzhalter-Leg auf der Ziel-TBS verhindert, dass die spätere Restore-Telemetrie versehentlich einen zweiten logischen Call erzeugt. Ein Restore gilt erst als abgeschlossen, wenn ein echtes Ziel-Leg beobachtet wurde. Schlägt danach nur das Aufräumen des temporären Contexts fehl, bleibt der bereits restaurierte Call aktiv und der Fehler erscheint separat in Ereignis und Diagnose.

Call Control selbst transportiert keine Sprachframes. Das übernimmt der vorhandene Media Switch anhand des aktualisierten Call-/Leg-Graphen. Die erfolgreiche Context-Installation allein beweist noch keine wiederhergestellte Sprachverbindung; tatsächliches Ziel-Leg, RouteReady und bidirektionale Medienbeobachtung getrennt prüfen.
