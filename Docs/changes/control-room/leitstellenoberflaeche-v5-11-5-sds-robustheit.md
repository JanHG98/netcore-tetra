# Leitstellenoberfläche v5.11.5 – Status-Tableau SDS robust

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: NetCore Control Room UI v5.11.5 – Status-Tableau SDS robust. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Für heutige Namen, Standorte und Statusdarstellung [Control Room](../../services/control-room/README.md), [Directory-Stammdaten](../../services/directory/README.md) und [Desktop-UI-Quellen](../../../system-backend/control-room/ui/src) verwenden. Die vielen Versionsschritte dokumentieren Fehler und Zwischenlösungen; ältere Import-/Override-Regeln nicht gleichzeitig kombinieren.

Dieser Änderungsnachweis bewahrt den beschriebenen UI-/API-/Buildstand. Damalige Tokenregeln, Feldnamen, Überschreib- und Buildbefehle gelten für diese Revision und dürfen nicht als aktuelle Komplettanleitung übernommen werden.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### NetCore Control Room UI v5.11.5 – Status-Tableau SDS robust

Dieses Paket behebt, dass Status-SDS zwar in der SDS-Liste sichtbar sind, aber im Status-Tableau nicht ankommen.

## Ursache

Die SDS-Tabelle zeigt Werte unabhängig davon an, ob sie intern als JSON-Zahl oder JSON-String kommen.

Der SDS-Fallback im Status-Tableau hat bisher aber nur echte JSON-Zahlen gelesen:

- `source`
- `proto`
- `status`

Wenn `/api/sds` also z. B. `"source": "2020001"` oder `"proto": "218"` liefert, wurde der Eintrag nicht gefunden.

## Fix

- neues `u64_any_at(...)`
  - liest JSON-Zahlen
  - liest JSON-Strings mit Zahlen
- SDS-Status-Fallback nutzt `u64_any_at(...)`
- Geräte, die nur in SDS auftauchen, werden jetzt als Kandidaten ins Status-Tableau aufgenommen
- Proto 218 wird robust erkannt
- Status-Texte wie `Status: Frei Auf Wache — Frei Auf Wache` werden gemappt
- Legende bleibt sauber auf einer Linie

## Windows Update

```cmd
taskkill /IM netcore-control-room-ui.exe /F

cargo clean --manifest-path system-backend\control-room\ui\Cargo.toml
rmdir /S /Q system-backend\control-room\ui\target
del /F /Q target\release\netcore-control-room-ui.exe
del /F /Q system-backend\control-room\ui\target\release\netcore-control-room-ui.exe

powershell -NoProfile -Command "Get-ChildItem -Recurse -Filter netcore-control-room-ui.exe | Remove-Item -Force"

powershell -NoProfile -Command "Expand-Archive -Force '%USERPROFILE%\Downloads\netcore-control-room-v5-11-5-status-tableau-sds-robust-files.zip' -DestinationPath '%CD%'"

cargo build --release --manifest-path system-backend\control-room\ui\Cargo.toml
```

## Start

```cmd
target\release\netcore-control-room-ui.exe --config "%APPDATA%\netcore\control-room\operator.toml" --profile default
```
