# Leitstellenoberfläche v5.11.9 – Buildkorrektur

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: NetCore Control Room UI v5.11.9 – Buildfix. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Die aktuelle Desktop-Architektur und Bedienerprofile stehen in [Desktop-UI-Anleitung](../../services/control-room/desktop-bedienoberflaeche.md) und [Control Room](../../services/control-room/README.md); Implementierung: [UI-Quellen](../../../system-backend/control-room/ui/src). Versionsbezogene ZIP-Dateien sind durch den gepflegten Repository-Stand abgelöst.

Dieser Änderungsnachweis bewahrt den beschriebenen UI-/API-/Buildstand. Damalige Tokenregeln, Feldnamen, Überschreib- und Buildbefehle gelten für diese Revision und dürfen nicht als aktuelle Komplettanleitung übernommen werden.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### NetCore Control Room UI v5.11.9 – Buildfix

Dieses Paket behebt die Buildfehler aus v5.11.8.

## Fixes

- `id_key_variants(...)` wiederhergestellt
  - wurde beim Entfernen von `nearest_marker(...)` versehentlich mit entfernt
- verbliebene Initializer-Zeilen entfernt:
  - `config_path,`
  - `username_source,`
- v5.11.8-Fixes bleiben enthalten:
  - Directory-first Namen
  - eigene Karten für gruppenlose Einzelgeräte
  - kein `2020001 2020001`
  - Statusfarbe/-nummer aus Textstatus
  - Kartenlabels Directory-first
  - alte Warning-Funktionen entfernt

## Windows Update

```cmd
taskkill /IM netcore-control-room-ui.exe /F

cargo clean --manifest-path system-backend\control-room\ui\Cargo.toml
rmdir /S /Q system-backend\control-room\ui\target
del /F /Q target\release\netcore-control-room-ui.exe
del /F /Q system-backend\control-room\ui\target\release\netcore-control-room-ui.exe

powershell -NoProfile -Command "Get-ChildItem -Recurse -Filter netcore-control-room-ui.exe | Remove-Item -Force"

powershell -NoProfile -Command "Expand-Archive -Force '%USERPROFILE%\Downloads\netcore-control-room-v5-11-9-status-tableau-buildfix-files.zip' -DestinationPath '%CD%'"

cargo build --release --manifest-path system-backend\control-room\ui\Cargo.toml
```

## Start

```cmd
target\release\netcore-control-room-ui.exe --config "%APPDATA%\netcore\control-room\operator.toml" --profile default
```
