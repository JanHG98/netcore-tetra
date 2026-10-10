# Leitstellenoberfläche v5.12.2 – Directory Deep-Index

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: NetCore Control Room UI v5.12.2 – Directory Deep-Index. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Für heutige Namen, Standorte und Statusdarstellung [Control Room](../../services/control-room/README.md), [Directory-Stammdaten](../../services/directory/README.md) und [Desktop-UI-Quellen](../../../system-backend/control-room/ui/src) verwenden. Die vielen Versionsschritte dokumentieren Fehler und Zwischenlösungen; ältere Import-/Override-Regeln nicht gleichzeitig kombinieren.

Dieser Änderungsnachweis bewahrt den beschriebenen UI-/API-/Buildstand. Damalige Tokenregeln, Feldnamen, Überschreib- und Buildbefehle gelten für diese Revision und dürfen nicht als aktuelle Komplettanleitung übernommen werden.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### NetCore Control Room UI v5.12.2 – Directory Deep-Index

Dieses Update baut einen universellen Namensindex aus dem kompletten `/api/directory`-JSON.

## Warum

Wenn weiterhin nur ISSIs statt Namen sichtbar sind, liegt das echte Directory-JSON offenbar nicht exakt in der bisher erwarteten Struktur. Deshalb durchsucht v5.12.2 das komplette JSON rekursiv.

## Erkannt werden ISSI-Felder

- `issi`
- `individual_issi`
- `source_issi`
- `address`
- `id`
- `subscriber_id`
- `terminal_id`
- `radio_id`

## Erkannt werden Namensfelder

- `name`
- `display_name`
- `displayName`
- `label`
- `alias`
- `rufname`
- `callsign`
- `radio_alias`
- `short_name`
- `terminal_name`
- `bezeichnung`
- `description`
- `title`

## Diagnose

Im Status-Tab steht jetzt:

```text
Directory-Index: LXC /api/directory · X Namen
```

Wenn `X` größer 0 ist, nutzt die UI diese Namen für Status-Tableau und Karte.
Wenn dort `0 Namen` steht, müssen wir den echten Raw-JSON-Auszug aus `Raw JSON` oder `curl /api/directory` ansehen.

## Windows Update

```cmd
taskkill /IM netcore-control-room-ui.exe /F

cargo clean --manifest-path system-backend\control-room\ui\Cargo.toml
rmdir /S /Q system-backend\control-room\ui\target
del /F /Q target\release\netcore-control-room-ui.exe
del /F /Q system-backend\control-room\ui\target\release\netcore-control-room-ui.exe

powershell -NoProfile -Command "Get-ChildItem -Recurse -Filter netcore-control-room-ui.exe | Remove-Item -Force"

powershell -NoProfile -Command "Expand-Archive -Force '%USERPROFILE%\Downloads\netcore-control-room-v5-12-2-directory-deep-index-files.zip' -DestinationPath '%CD%'"

cargo build --release --manifest-path system-backend\control-room\ui\Cargo.toml
```

## Start

```cmd
target\release\netcore-control-room-ui.exe --config "%APPDATA%\netcore\control-room\operator.toml" --profile default
```
