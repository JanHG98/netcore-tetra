# Leitstellenoberfläche v5.11.8 – Status-Tableau Final-Fix

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: NetCore Control Room UI v5.11.8 – Status-Tableau Final-Fix. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Für heutige Namen, Standorte und Statusdarstellung [Control Room](../../services/control-room/README.md), [Directory-Stammdaten](../../services/directory/README.md) und [Desktop-UI-Quellen](../../../system-backend/control-room/ui/src) verwenden. Die vielen Versionsschritte dokumentieren Fehler und Zwischenlösungen; ältere Import-/Override-Regeln nicht gleichzeitig kombinieren.

Dieser Änderungsnachweis bewahrt den beschriebenen UI-/API-/Buildstand. Damalige Tokenregeln, Feldnamen, Überschreib- und Buildbefehle gelten für diese Revision und dürfen nicht als aktuelle Komplettanleitung übernommen werden.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### NetCore Control Room UI v5.11.8 – Status-Tableau Final-Fix

Dieses Paket behebt die offenen Punkte aus v5.11.6/v5.11.7.

## Fixes

- Status-Tableau:
  - Geräte ohne Statusgruppe werden nicht mehr in einem Sammelblock `Einzelgeräte` angezeigt.
  - Jedes gruppenlose Gerät bekommt eine eigene Karte.
  - Geräte mit Statusgruppe werden weiterhin pro Statusgruppe gesammelt.
  - Anzeige `2020001 2020001` ist entfernt.
  - Name ist Directory-first über `/api/directory`.

- Statuscode/Farbe:
  - Textstatus wie `Frei Auf Funk` / `Frei Auf Wache` wird auf Status 1 gemappt.
  - Statusnummer und Farbe werden dadurch passend gesetzt.
  - Negative Status wie `Nicht bereit` werden vor generischem `frei` geprüft.

- Karte:
  - Markerlabels nutzen Directory-first Namen.
  - Positions-ISSI wird robuster gelesen.

- Build-Warnings:
  - unbenutzte Felder `config_path`, `username_source`, `owner` entfernt
  - unbenutzte Funktion `device_label_for_location` entfernt
  - unbenutzte Funktion `nearest_marker` entfernt
  - alte unbenutzte Toolbar-/Ribbon-Helfer entfernt

## Windows Update

```cmd
taskkill /IM netcore-control-room-ui.exe /F

cargo clean --manifest-path system-backend\control-room\ui\Cargo.toml
rmdir /S /Q system-backend\control-room\ui\target
del /F /Q target\release\netcore-control-room-ui.exe
del /F /Q system-backend\control-room\ui\target\release\netcore-control-room-ui.exe

powershell -NoProfile -Command "Get-ChildItem -Recurse -Filter netcore-control-room-ui.exe | Remove-Item -Force"

powershell -NoProfile -Command "Expand-Archive -Force '%USERPROFILE%\Downloads\netcore-control-room-v5-11-8-status-tableau-final-fix-files.zip' -DestinationPath '%CD%'"

cargo build --release --manifest-path system-backend\control-room\ui\Cargo.toml
```

## Start

```cmd
target\release\netcore-control-room-ui.exe --config "%APPDATA%\netcore\control-room\operator.toml" --profile default
```
