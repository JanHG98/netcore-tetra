# Leitstellenoberfläche v5.12.0 – Status-Tableau Drag + Directory

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: NetCore Control Room UI v5.12.0 – Status-Tableau Drag + Directory. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Für heutige Namen, Standorte und Statusdarstellung [Control Room](../../services/control-room/README.md), [Directory-Stammdaten](../../services/directory/README.md) und [Desktop-UI-Quellen](../../../system-backend/control-room/ui/src) verwenden. Die vielen Versionsschritte dokumentieren Fehler und Zwischenlösungen; ältere Import-/Override-Regeln nicht gleichzeitig kombinieren.

Dieser Änderungsnachweis bewahrt den beschriebenen UI-/API-/Buildstand. Damalige Tokenregeln, Feldnamen, Überschreib- und Buildbefehle gelten für diese Revision und dürfen nicht als aktuelle Komplettanleitung übernommen werden.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### NetCore Control Room UI v5.12.0 – Status-Tableau Drag + Directory

Dieses Paket behebt die offenen Status-Tableau-/Directory-/Kartenpunkte.

## Neu/Fix

- Status-Tableau-Karten können per Maus gezogen und frei angeordnet werden.
- Button `Tableau-Positionen zurücksetzen` setzt die manuelle Anordnung zurück.
- Geräte ohne Statusgruppe bleiben eigene Karten.
- Geräte mit Statusgruppe bleiben gruppiert.
- Statusnummer/Farbe wird final auch aus dem sichtbaren Statustext abgeleitet.
  - `Frei Auf Funk`
  - `Frei Auf Wache`
  - `Nicht bereit`
  - `Sprechwunsch`
  - etc.
- `/api/directory` wird robuster gelesen:
  - akzeptiert `{ "directory": { ... } }`
  - akzeptiert `subscribers/groups/status_groups/statuses` als Objekt
  - akzeptiert diese Felder auch als Array und mappt sie automatisch per ISSI/GSSI/ID
- Directory-first Namen bleiben in Status-Tableau und Karte aktiv.
- Karten-ISSI wird robust aus `issi`, `individual_issi`, `source_issi`, `address` gelesen.

## Windows Update

```cmd
taskkill /IM netcore-control-room-ui.exe /F

cargo clean --manifest-path system-backend\control-room\ui\Cargo.toml
rmdir /S /Q system-backend\control-room\ui\target
del /F /Q target\release\netcore-control-room-ui.exe
del /F /Q system-backend\control-room\ui\target\release\netcore-control-room-ui.exe

powershell -NoProfile -Command "Get-ChildItem -Recurse -Filter netcore-control-room-ui.exe | Remove-Item -Force"

powershell -NoProfile -Command "Expand-Archive -Force '%USERPROFILE%\Downloads\netcore-control-room-v5-12-0-status-tableau-drag-directory-files.zip' -DestinationPath '%CD%'"

cargo build --release --manifest-path system-backend\control-room\ui\Cargo.toml
```

## Start

```cmd
target\release\netcore-control-room-ui.exe --config "%APPDATA%\netcore\control-room\operator.toml" --profile default
```
