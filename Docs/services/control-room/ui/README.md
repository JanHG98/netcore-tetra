# Leitstelle

**Quellen:** [system-backend/control-room/ui](../../../../system-backend/control-room/ui) · [Repository-Root](../../../..). Bei Befehlen das in der Anleitung angegebene Arbeitsverzeichnis beachten.

Stand: **9. Oktober 2026**. Die Anleitung beschreibt die im Hauptzweig vorhandene Umsetzung; Anlagen- und Funkabnahmen stehen in der [Gesamtroadmap](../../../roadmaps/gesamtroadmap.md).

Native Desktop UI für den NetCore Control Room.

## Vorhandene Funktionen

- echte OS-Fenster pro Modul
- Multi-Monitor-tauglich
- echte Live-Karte mit Kartenkacheln
- interaktive Karte: ziehen, Mausrad-Zoom, Doppelklick-Zentrierung
- flüssigere Karte durch nicht-blockierendes Tile-Laden im Hintergrund
- Klick auf GPS-/LIP-Punkt zeigt Geräteinfos direkt in der Karte
- lokale Tile-Cache-Ablage
- Standortpunkte aus `/api/locations`
- kein Browser, keine Web-App
- lokale Verbindung und Bedienplatzprofil über `operator.toml`; keine Passwörter in der Datei

## Windows Update/Build

Im Repo-Root:

```cmd
taskkill /IM netcore-control-room-ui.exe /F
cargo build --release --manifest-path system-backend\control-room\ui\Cargo.toml
```

Start:

```cmd
system-backend\control-room\ui\target\release\netcore-control-room-ui.exe --config "%APPDATA%\netcore\control-room\operator.toml" --profile default
```

Die Desktop-Crate ist ein eigener Cargo-Workspace. Ohne `CARGO_TARGET_DIR` liegt
ihre EXE deshalb im oben genannten Unterverzeichnis. Für einen absichtlich
abweichenden Buildpfad die dort erzeugte EXE verwenden. `cargo clean` ist für ein
normales Update nicht nötig.

Erzeugte EXE bei abweichendem Zielverzeichnis suchen:

```cmd
powershell -NoProfile -Command "Get-ChildItem -Recurse -Filter netcore-control-room-ui.exe | Sort-Object LastWriteTime -Descending | Select-Object LastWriteTime,FullName"
```

## Kartenkonfiguration

In `%APPDATA%\netcore\control-room\operator.toml` kann zusätzlich stehen:

```toml
[ui.map]
online_tiles = true
tile_url = "https://tile.openstreetmap.org/{z}/{x}/{y}.png"
tile_attribution = "© OpenStreetMap contributors"
default_lat = 52.3759
default_lon = 9.7320
default_zoom = 13
min_zoom = 3
max_zoom = 18
```

Die UI cached Kacheln lokal. Wenn `online_tiles = false` gesetzt ist, nutzt sie nur den lokalen Cache/Fallback.

## Bedienung

Links:

- `OS-Fenster-Modus`
- `↗` Modul als echtes OS-Fenster öffnen
- `▣` Modulfenster offen
- `Alle Module als OS-Fenster öffnen`
- `Alle OS-Fenster schließen`

Im Kartenmodul:

- Ziehen mit linker Maustaste: Karte verschieben
- Mausrad: rein-/rauszoomen, mit Mausposition als Anker
- Doppelklick: Karte auf Mausposition zentrieren
- `Positionen folgen`: automatisch auf vorhandene LIP-Punkte zoomen
- `Ansicht reset`: wieder auf Live-/Folgemodus zurücksetzen
- Online-Kartenkacheln ein/aus
- Standortpunkte live aus `/api/locations`
- Klick auf Standortpunkt: Geräte-/ISSI-Details anzeigen


## Aktuelle Positionsansicht

- Standorte und Karte zeigen pro ISSI nur noch den aktuellsten Standort.
- Alte historische Positionsmeldungen werden in der UI als Zombie-Positionen ausgeblendet.

Der Login-Dialog ist im Client vorhanden. Der ausgelieferte Server läuft mit
`--no-auth`; dessen Login-Antwort lautet dann `auth-disabled` mit Adminrechten.
Der Dialog stellt in diesem Modus keine Zugangskontrolle her. Im geschützten
Serverprofil werden Benutzername und Passwort geprüft. Siehe
[Anmeldung und Rollen](../anmeldung-und-rollen.md).
