# Leitstellenoberfläche v5.11.2 – Status-Tableau Buildkorrektur

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: NetCore Control Room UI v5.11.2 – Status-Tableau Buildfix. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Für heutige Namen, Standorte und Statusdarstellung [Control Room](../../services/control-room/README.md), [Directory-Stammdaten](../../services/directory/README.md) und [Desktop-UI-Quellen](../../../system-backend/control-room/ui/src) verwenden. Die vielen Versionsschritte dokumentieren Fehler und Zwischenlösungen; ältere Import-/Override-Regeln nicht gleichzeitig kombinieren.

Dieser Änderungsnachweis bewahrt den beschriebenen UI-/API-/Buildstand. Damalige Tokenregeln, Feldnamen, Überschreib- und Buildbefehle gelten für diese Revision und dürfen nicht als aktuelle Komplettanleitung übernommen werden.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### NetCore Control Room UI v5.11.2 – Status-Tableau Buildfix

Dieses Paket behebt den v5.11.1-Buildfehler.

## Fix

Vor `StatusTableauCard` stand versehentlich doppelt:

```rust
#[derive(Debug, Clone)]
#[derive(Debug, Clone)]
```

Dadurch kam:

```text
conflicting implementations of trait Debug/Clone for StatusTableauCard
```

In v5.11.2 steht der Derive nur noch einmal.

## Windows Update

```cmd
taskkill /IM netcore-control-room-ui.exe /F

cargo clean --manifest-path system-backend\control-room\ui\Cargo.toml
rmdir /S /Q system-backend\control-room\ui\target
del /F /Q target\release\netcore-control-room-ui.exe
del /F /Q system-backend\control-room\ui\target\release\netcore-control-room-ui.exe

powershell -NoProfile -Command "Get-ChildItem -Recurse -Filter netcore-control-room-ui.exe | Remove-Item -Force"

powershell -NoProfile -Command "Expand-Archive -Force '%USERPROFILE%\Downloads\netcore-control-room-v5-11-2-status-tableau-buildfix-files.zip' -DestinationPath '%CD%'"

cargo build --release --manifest-path system-backend\control-room\ui\Cargo.toml

target\release\netcore-control-room-ui.exe --config "%APPDATA%\netcore\control-room\operator.toml" --profile default
```
