# Leitstellenoberfläche v5.9 – Clean Leitstellenarbeitsplatz

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: NetCore Control Room UI v5.9 – Clean Leitstellenarbeitsplatz. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Die aktuelle Desktop-Architektur und Bedienerprofile stehen in [Desktop-UI-Anleitung](../../services/control-room/desktop-bedienoberflaeche.md) und [Control Room](../../services/control-room/README.md); Implementierung: [UI-Quellen](../../../system-backend/control-room/ui/src). Versionsbezogene ZIP-Dateien sind durch den gepflegten Repository-Stand abgelöst.

Dieser Änderungsnachweis bewahrt den beschriebenen UI-/API-/Buildstand. Damalige Tokenregeln, Feldnamen, Überschreib- und Buildbefehle gelten für diese Revision und dürfen nicht als aktuelle Komplettanleitung übernommen werden.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### NetCore Control Room UI v5.9 – Clean Leitstellenarbeitsplatz

Dieses Update räumt die v5.8-Oberfläche grundlegend auf.

## Änderungen

- Navigation links driftet nicht mehr nach rechts
- Modulnamen sind linksbündig und sauber ausgerichtet
- Kopfbereich ist clean und kompakt
- API-/Profil-/Directory-Technikdetails sind aus dem sichtbaren Kopf entfernt
- Auto-Refresh ist fest auf 1 Sekunde gesetzt
- Refresh-Slider ist entfernt
- Toolbar oben ist funktional:
  - Übersicht
  - Teilnehmer
  - Gruppen
  - Rufe
  - SDS
  - Karte
  - Befehle, falls operator/admin
  - Benutzer, falls admin
  - Maske leer
  - Aktualisieren
- RBAC-Gating bleibt erhalten
- User+Passwort Login bleibt erhalten
- LXC/TBS müssen für dieses UI-Update nicht geändert werden

## Windows Update

```cmd
taskkill /IM netcore-control-room-ui.exe /F

cargo clean --manifest-path system-backend\control-room\ui\Cargo.toml
rmdir /S /Q system-backend\control-room\ui\target
del /F /Q target\release\netcore-control-room-ui.exe
del /F /Q system-backend\control-room\ui\target\release\netcore-control-room-ui.exe

powershell -NoProfile -Command "Get-ChildItem -Recurse -Filter netcore-control-room-ui.exe | Remove-Item -Force"

powershell -NoProfile -Command "Expand-Archive -Force '%USERPROFILE%\Downloads\netcore-control-room-v5-9-clean-workplace-ui-files.zip' -DestinationPath '%CD%'"

cargo build --release --manifest-path system-backend\control-room\ui\Cargo.toml

target\release\netcore-control-room-ui.exe --config "%APPDATA%\netcore\control-room\operator.toml" --profile default
```
