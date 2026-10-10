# Leitstellenoberfläche v5.11.7 – Directory-First Status/Layout

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: NetCore Control Room UI v5.11.7 – Directory-First Status/Layout. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Für heutige Namen, Standorte und Statusdarstellung [Control Room](../../services/control-room/README.md), [Directory-Stammdaten](../../services/directory/README.md) und [Desktop-UI-Quellen](../../../system-backend/control-room/ui/src) verwenden. Die vielen Versionsschritte dokumentieren Fehler und Zwischenlösungen; ältere Import-/Override-Regeln nicht gleichzeitig kombinieren.

Dieser Änderungsnachweis bewahrt den beschriebenen UI-/API-/Buildstand. Damalige Tokenregeln, Feldnamen, Überschreib- und Buildbefehle gelten für diese Revision und dürfen nicht als aktuelle Komplettanleitung übernommen werden.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### NetCore Control Room UI v5.11.7 – Directory-First Status/Layout

Dieses Update behebt die offenen Punkte aus v5.11.6:

- Status-Tableau nutzt Statuscode jetzt robuster auch dann, wenn Live-/Directory-Daten nur Textlabels liefern.
- Geräte-Namen werden Directory-first aufgelöst und in Tableau + Karte bevorzugt aus `/api/directory` geholt.
- Kartenpunkte lesen die ISSI jetzt robuster (`issi`, `individual_issi`, `source_issi`, `address`).
- Einzelgeräte landen nicht mehr gesammelt in einem einzigen Sammelblock, sondern jeweils als eigene Karte.
- In Einzelgerätekarten wird die ISSI nur noch in der Detailzeile gezeigt; keine doppelte Anzeige `2020001 2020001` mehr.

## Erwartetes Verhalten

- Wenn für eine ISSI ein Directory-Name vorhanden ist, wird dieser im Status-Tableau und auf der Karte angezeigt.
- Wenn ein Status nur als Text (z. B. `Frei Auf Wache`) kommt, werden Farbfeld und Nummer trotzdem passend gesetzt.
- Geräte ohne Statusgruppe erscheinen als eigene Statuskarte.
- Geräte mit Statusgruppe erscheinen gesammelt pro Gruppe.
