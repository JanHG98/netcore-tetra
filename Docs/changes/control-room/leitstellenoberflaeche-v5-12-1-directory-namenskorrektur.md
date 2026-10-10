# Leitstellenoberfläche v5.12.1 – Directory-Namen Fix

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: NetCore Control Room UI v5.12.1 – Directory-Namen Fix. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Für heutige Namen, Standorte und Statusdarstellung [Control Room](../../services/control-room/README.md), [Directory-Stammdaten](../../services/directory/README.md) und [Desktop-UI-Quellen](../../../system-backend/control-room/ui/src) verwenden. Die vielen Versionsschritte dokumentieren Fehler und Zwischenlösungen; ältere Import-/Override-Regeln nicht gleichzeitig kombinieren.

Dieser Änderungsnachweis bewahrt den beschriebenen UI-/API-/Buildstand. Damalige Tokenregeln, Feldnamen, Überschreib- und Buildbefehle gelten für diese Revision und dürfen nicht als aktuelle Komplettanleitung übernommen werden.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### NetCore Control Room UI v5.12.1 – Directory-Namen Fix

Dieses Paket behebt, dass im Status-Tableau und auf der Karte weiterhin ISSIs statt Namen angezeigt wurden.

## Fixes

- `/api/directory` ist jetzt authoritative.
- `operator.toml` überschreibt vorhandene LXC-Directory-Einträge nicht mehr, sondern füllt nur noch fehlende Werte auf.
- Rohes `/api/directory` wird zusätzlich behalten und rekursiv durchsucht.
- Namen werden aus deutlich mehr Feldnamen erkannt:
  - `name`, `display_name`, `displayName`, `label`, `alias`
  - `rufname`, `callsign`, `radioAlias`, `shortName`, `terminalName`
  - `bezeichnung`, `description`, `title`
- Directory darf auch verschachtelt sein oder `{ "directory": ... }` liefern.
- Status-Tableau zeigt oben die Directory-Quelle inklusive erkannter Namensanzahl.
