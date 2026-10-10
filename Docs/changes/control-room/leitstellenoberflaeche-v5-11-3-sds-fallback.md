# Leitstellenoberfläche v5.11.3 – Status-Tableau SDS-Fallback

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: NetCore Control Room UI v5.11.3 – Status-Tableau SDS-Fallback. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Für heutige Namen, Standorte und Statusdarstellung [Control Room](../../services/control-room/README.md), [Directory-Stammdaten](../../services/directory/README.md) und [Desktop-UI-Quellen](../../../system-backend/control-room/ui/src) verwenden. Die vielen Versionsschritte dokumentieren Fehler und Zwischenlösungen; ältere Import-/Override-Regeln nicht gleichzeitig kombinieren.

Dieser Änderungsnachweis bewahrt den beschriebenen UI-/API-/Buildstand. Damalige Tokenregeln, Feldnamen, Überschreib- und Buildbefehle gelten für diese Revision und dürfen nicht als aktuelle Komplettanleitung übernommen werden.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### NetCore Control Room UI v5.11.3 – Status-Tableau SDS-Fallback

## Enthalten

- Status-Tableau nutzt jetzt zusätzlich den letzten passenden SDS-Status als Fallback.
- Statusmeldungen mit Proto 218 werden pro ISSI ausgewertet.
- Statuslabel wird aus dem SDS-Text abgeleitet und auf Directory-Statuscodes gemappt.
- Legende im Status-Tableau steht nun sauber auf einer Linie.
- Kleine Optikverbesserung: Wenn kein Name vorhanden ist, wird direkt die ISSI sauber angezeigt.

## Windows Update

```cmd
taskkill /IM netcore-control-room-ui.exe /F
cargo clean --manifest-path system-backend\control-room\ui\Cargo.toml
rmdir /S /Q system-backend\control-room\ui\target
powershell -NoProfile -Command "Get-ChildItem -Recurse -Filter netcore-control-room-ui.exe | Remove-Item -Force"
powershell -NoProfile -Command "Expand-Archive -Force '%USERPROFILE%\Downloads\netcore-control-room-v5-11-3-status-tableau-sds-fallback-files.zip' -DestinationPath '%CD%'"
cargo build --release --manifest-path system-backend\control-room\ui\Cargo.toml
```

## Start

```cmd
target\release\netcore-control-room-ui.exe --config "%APPDATA%\netcore\control-room\operator.toml" --profile default
```
