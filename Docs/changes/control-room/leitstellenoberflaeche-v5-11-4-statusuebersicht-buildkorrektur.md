# Leitstellenoberfläche v5.11.4 – Status-Tableau Buildkorrektur

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: NetCore Control Room UI v5.11.4 – Status-Tableau Buildfix. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Für heutige Namen, Standorte und Statusdarstellung [Control Room](../../services/control-room/README.md), [Directory-Stammdaten](../../services/directory/README.md) und [Desktop-UI-Quellen](../../../system-backend/control-room/ui/src) verwenden. Die vielen Versionsschritte dokumentieren Fehler und Zwischenlösungen; ältere Import-/Override-Regeln nicht gleichzeitig kombinieren.

Dieser Änderungsnachweis bewahrt den beschriebenen UI-/API-/Buildstand. Damalige Tokenregeln, Feldnamen, Überschreib- und Buildbefehle gelten für diese Revision und dürfen nicht als aktuelle Komplettanleitung übernommen werden.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### NetCore Control Room UI v5.11.4 – Status-Tableau Buildfix

Dieses Paket behebt den Rust-Buildfehler aus v5.11.3.

## Ursache

In `infer_status_code_from_text()` war diese Mapping-Liste als normales Array geschrieben:

```rust
(1, ["frei auf wache", "frei"])
(2, ["bereit"])
(4, ["einsatz", "alarm", "unterwegs"])
```

Rust verlangt bei Arrays aber überall die gleiche Länge. Deshalb kam:

```text
expected an array with a size of 2, found one with a size of 1/3
```

## Fix

Die Mapping-Liste ist jetzt als Slice-Liste typisiert:

```rust
let checks: &[(u64, &[&str])] = &[
    (1, &["frei auf wache", "frei"]),
    (2, &["bereit"]),
    (4, &["einsatz", "alarm", "unterwegs"]),
];
```

## Windows Update

```cmd
taskkill /IM netcore-control-room-ui.exe /F

cargo clean --manifest-path system-backend\control-room\ui\Cargo.toml
rmdir /S /Q system-backend\control-room\ui\target
del /F /Q target\release\netcore-control-room-ui.exe
del /F /Q system-backend\control-room\ui\target\release\netcore-control-room-ui.exe

powershell -NoProfile -Command "Get-ChildItem -Recurse -Filter netcore-control-room-ui.exe | Remove-Item -Force"

powershell -NoProfile -Command "Expand-Archive -Force '%USERPROFILE%\Downloads\netcore-control-room-v5-11-4-status-tableau-buildfix-files.zip' -DestinationPath '%CD%'"

cargo build --release --manifest-path system-backend\control-room\ui\Cargo.toml

target\release\netcore-control-room-ui.exe --config "%APPDATA%\netcore\control-room\operator.toml" --profile default
```
