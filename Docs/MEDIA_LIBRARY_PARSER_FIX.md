# Fix: `[media_library]` wird als unbekannt abgewiesen

## Fehlerbild

```text
Primary config '/etc/netcore/config.toml' failed to load:
Unrecognized top-level fields: ["media_library"].
Running on fallback '/etc/netcore/config.toml.fallback'.
```

Der Abschnitt ist im neuen Quellstand gültig. Diese Meldung bedeutet daher in der Praxis,
dass systemd noch eine ältere `bluestation-bs`-Binary ausführt. Besonders leicht passiert das,
wenn eine neue Binary nach `/usr/local/bin/bluestation-bs` kopiert wird, die Unit aber weiterhin
eine andere Kopie unter `/home/.../target/release/`, `/opt/...` oder einem abweichenden Pfad startet.

## Korrektur

Im Wurzelverzeichnis des entpackten Pakets:

```bash
cd /PFAD/ZU/netcore-tetra-swmi
sudo ./install/update-basisstation.sh
```

Das Skript:

1. erkennt die konfigurierte bzw. vorhandene systemd-Unit,
2. ermittelt über den laufenden Hauptprozess die tatsächlich verwendete Binary,
3. führt die Parser-Regressionstests aus,
4. baut `bluestation-bs` aus genau diesem Paket,
5. sichert die alte Binary,
6. ersetzt den aktiven Binary-Pfad,
7. startet die Unit neu und prüft, dass `[media_library]` nicht mehr als unbekannt gilt.

## Abweichende Unit oder Binary

```bash
sudo -E env \
  UNIT=bluestation.service \
  BINARY_PATH=/usr/local/bin/bluestation-bs \
  ./install/update-basisstation.sh
```

Optional können explizite Cargo-Features gesetzt werden:

```bash
sudo -E env \
  CARGO_FEATURES="asterisk,recording,audio-player" \
  ./install/update-basisstation.sh
```

Die Default-Features von `bluestation-bs` enthalten diese Funktionen bereits; die explizite
Angabe ist normalerweise nicht nötig.

## Kontrolle

```bash
systemctl status tetra.service --no-pager
journalctl -u tetra.service -n 120 --no-pager
```

In den neuen Logs darf nicht mehr stehen:

```text
Unrecognized top-level fields: ["media_library"]
Running on fallback
```

Die Datei `/etc/netcore/config.toml.fallback` wird absichtlich nicht automatisch überschrieben.
Sie bleibt die bekannte Rückfallebene für echte Konfigurationsfehler.
