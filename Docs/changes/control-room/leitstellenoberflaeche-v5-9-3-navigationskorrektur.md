# Leitstellenoberfläche v5.9.3 – Navigation Alignment Fix

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: NetCore Control Room UI v5.9.3 – Navigation Alignment Fix. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Die aktuelle Desktop-Architektur und Bedienerprofile stehen in [Desktop-UI-Anleitung](../../services/control-room/desktop-bedienoberflaeche.md) und [Control Room](../../services/control-room/README.md); Implementierung: [UI-Quellen](../../../system-backend/control-room/ui/src). Versionsbezogene ZIP-Dateien sind durch den gepflegten Repository-Stand abgelöst.

Dieser Änderungsnachweis bewahrt den beschriebenen UI-/API-/Buildstand. Damalige Tokenregeln, Feldnamen, Überschreib- und Buildbefehle gelten für diese Revision und dürfen nicht als aktuelle Komplettanleitung übernommen werden.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### NetCore Control Room UI v5.9.3 – Navigation Alignment Fix

Dieses Paket behebt den linken Menü-Drift aus v5.9.x.

## Ursache

Die Modulzeilen bestanden vorher aus zwei normalen egui-Widgets in einem `horizontal`-Layout.
Dadurch wurde die verfügbare Breite pro Zeile leicht anders berechnet und die OS-Fenster-Pfeile wanderten nach rechts.

## Fix

Die Navigation wird jetzt als eine feste Zeile gezeichnet:

- eine volle Zeile pro Modul
- Modulname links fest ausgerichtet
- OS-Fenster-Pfeil rechts fest ausgerichtet
- kein Layout-Drift mehr
- Klick auf Text/Zeile öffnet Modul
- Klick auf Pfeil öffnet/schließt OS-Fenster

## Windows Update

```cmd
taskkill /IM netcore-control-room-ui.exe /F

cargo clean --manifest-path system-backend\control-room\ui\Cargo.toml
rmdir /S /Q system-backend\control-room\ui\target
del /F /Q target\release\netcore-control-room-ui.exe
del /F /Q system-backend\control-room\ui\target\release\netcore-control-room-ui.exe

powershell -NoProfile -Command "Get-ChildItem -Recurse -Filter netcore-control-room-ui.exe | Remove-Item -Force"

powershell -NoProfile -Command "Expand-Archive -Force '%USERPROFILE%\Downloads\netcore-control-room-v5-9-3-clean-workplace-navfix-files.zip' -DestinationPath '%CD%'"

cargo build --release --manifest-path system-backend\control-room\ui\Cargo.toml

target\release\netcore-control-room-ui.exe --config "%APPDATA%\netcore\control-room\operator.toml" --profile default
```
