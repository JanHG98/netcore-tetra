# Leitstellenoberfläche v5.9.1 – Buildkorrektur

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: NetCore Control Room UI v5.9.1 – Buildfix. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Die aktuelle Desktop-Architektur und Bedienerprofile stehen in [Desktop-UI-Anleitung](../../services/control-room/desktop-bedienoberflaeche.md) und [Control Room](../../services/control-room/README.md); Implementierung: [UI-Quellen](../../../system-backend/control-room/ui/src). Versionsbezogene ZIP-Dateien sind durch den gepflegten Repository-Stand abgelöst.

Dieser Änderungsnachweis bewahrt den beschriebenen UI-/API-/Buildstand. Damalige Tokenregeln, Feldnamen, Überschreib- und Buildbefehle gelten für diese Revision und dürfen nicht als aktuelle Komplettanleitung übernommen werden.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### NetCore Control Room UI v5.9.1 – Buildfix

Dieses Paket behebt den Buildfehler aus v5.9.

## Fix

Der Button `Maske leer` verwendete versehentlich alte/falsche Feldnamen:

- `command_issi`
- `command_gssi`
- `emergency_issi`
- `command_detach`

In v5.x heißen die Felder tatsächlich:

- `kick_issi`
- `dgna_issi`
- `dgna_gssi`
- `emergency_clear_issi`
- `dgna_detach`

## Update Windows

```cmd
taskkill /IM netcore-control-room-ui.exe /F

cargo clean --manifest-path system-backend\control-room\ui\Cargo.toml
rmdir /S /Q system-backend\control-room\ui\target
del /F /Q target\release\netcore-control-room-ui.exe
del /F /Q system-backend\control-room\ui\target\release\netcore-control-room-ui.exe

powershell -NoProfile -Command "Get-ChildItem -Recurse -Filter netcore-control-room-ui.exe | Remove-Item -Force"

powershell -NoProfile -Command "Expand-Archive -Force '%USERPROFILE%\Downloads\netcore-control-room-v5-9-1-clean-workplace-buildfix-files.zip' -DestinationPath '%CD%'"

cargo build --release --manifest-path system-backend\control-room\ui\Cargo.toml

target\release\netcore-control-room-ui.exe --config "%APPDATA%\netcore\control-room\operator.toml" --profile default
```
