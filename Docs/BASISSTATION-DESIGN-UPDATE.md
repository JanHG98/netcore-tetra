# Neues Basisstations-Dashboard installieren

Das freigegebene Design liegt auf `feat/netcore-dashboard-design`. Dieser Branch enthält auch die neuen Dienstoberflächen. Diese Anleitung aktualisiert eine **bereits installierte Basisstation** aus einem Git-Checkout; für die zentralen Dienste gilt [DIENST-WEBUI-DESIGN-UPDATE.md](DIENST-WEBUI-DESIGN-UPDATE.md). Das Dashboard wird in `bluestation-bs` eingebettet; ein zusätzlicher Webserver oder ein Node-/npm-Build ist nicht erforderlich.

**Hell und Dunkel** stehen sowohl auf der Anmeldeseite als auch im Dashboard zur Verfügung. Das Dashboard behält zusätzlich sein blaues Theme. Die Auswahl bleibt im jeweiligen Browser für diese Webadresse gespeichert und wird beim Neuladen wiederhergestellt. Nach dem Update auch Tabellen, Dialoge, öffentliche Übersicht und RF-Anzeigen im dunklen Design prüfen.

Die folgenden Befehle in einer Bash-Sitzung auf der Basisstation als bisheriger Build-Benutzer ausführen. Für den Dienstneustart entsteht eine kurze Unterbrechung. Den bestehenden Quellcodeordner verwenden, beispielsweise `/opt/netcore-tetra`, falls dort tatsächlich dein Checkout liegt.

## 1. Dienst, Konfiguration und bisherigen Stand feststellen

Im Quellcodeordner beginnen:

```bash
git status --short --branch
git rev-parse HEAD
command -v cargo
cargo --version
rustc --version
```

Bei lokalen Änderungen zuerst deren Inhalt sichern und bewusst committen oder stashen. Erst mit sauberem Arbeitsbaum fortfahren; kein `reset --hard` verwenden. Rust muss die Edition 2024 unterstützen. SoapySDR, der passende SDR-Treiber und die bisherigen nativen Codec-Abhängigkeiten müssen weiterhin vorhanden sein.

Der Updater verwendet standardmäßig `/etc/netcore/config.toml`. **Wenn deine Unit eine andere Konfiguration startet, den tatsächlichen Pfad unten einsetzen.** Die Unit-Erkennung entspricht der Erkennung im Updater:

```bash
CONFIG_PATH='/etc/netcore/config.toml'
UNIT="$(sudo sed -nE 's/^[[:space:]]*service_name[[:space:]]*=[[:space:]]*"([^"]+)".*/\1/p' "$CONFIG_PATH" | head -n1)"
if [[ -n "$UNIT" && "$UNIT" != *.service ]]; then
    UNIT="${UNIT}.service"
fi
if [[ -z "$UNIT" ]] || ! systemctl cat "$UNIT" >/dev/null 2>&1; then
    UNIT=''
    for candidate in tetra.service bluestation.service tetra-bluestation.service bluestation-bs.service; do
        if systemctl cat "$candidate" >/dev/null 2>&1; then
            UNIT="$candidate"
            break
        fi
    done
fi
printf 'Basisstations-Unit: %s\n' "$UNIT"
test -n "$UNIT" && systemctl show "$UNIT" -p ExecStart -p User -p MainPID
```

Die Ausgabe prüfen: `ExecStart` muss deine Basisstation und die richtige Konfigurationsdatei nennen. Wurde keine Unit gefunden oder gibt es mehrere Installationen, `UNIT` ausdrücklich auf die richtige vorhandene Unit setzen. Nicht mit leerer oder ungeprüfter Unit fortfahren.

Bei laufendem Dienst den tatsächlichen Binärpfad erfassen:

```bash
TBS_PID="$(systemctl show "$UNIT" -p MainPID --value)"
BINARY_PATH="$(sudo readlink -f "/proc/$TBS_PID/exe")"
BINARY_PATH="${BINARY_PATH% (deleted)}"
printf 'Aktive Binary: %s\n' "$BINARY_PATH"
test -f "$BINARY_PATH" && test "$(basename -- "$BINARY_PATH")" = bluestation-bs
```

Ist der Dienst gestoppt, den Binärpfad aus dem zuvor geprüften `ExecStart` übernehmen: `BINARY_PATH='/tatsaechlicher/pfad/bluestation-bs'`. Bei fehlgeschlagener Pfadprüfung erst die Ursache klären. Nicht automatisch eine eventuell andere Kopie aus `/usr/local/bin` verwenden.

Den bisherigen Git-Stand, die Pfade und eine geschützte Konfigurationskopie sichern:

```bash
TBS_STAMP="$(date +%Y%m%d-%H%M%S)"
TBS_STATE_DIR="$HOME/netcore-design-update-$TBS_STAMP"
TBS_CONFIG_BACKUP_DIR="/var/backups/netcore-tetra/dashboard-design-$TBS_STAMP"
mkdir -m 0700 -- "$TBS_STATE_DIR"
git rev-parse HEAD > "$TBS_STATE_DIR/commit-before.txt"
git branch --show-current > "$TBS_STATE_DIR/branch-before.txt"
printf '%s\n' "$UNIT" > "$TBS_STATE_DIR/unit.txt"
printf '%s\n' "$BINARY_PATH" > "$TBS_STATE_DIR/binary-path.txt"
printf '%s\n' "$CONFIG_PATH" > "$TBS_STATE_DIR/config-path.txt"
sudo install -d -m 0700 -- "$TBS_CONFIG_BACKUP_DIR"
sudo cp -aL -- "$CONFIG_PATH" "$TBS_CONFIG_BACKUP_DIR/config.toml"
printf 'Update-Protokoll und Rückweg: %s\n' "$TBS_STATE_DIR"
printf 'Geschützte Konfigurationssicherung: %s\n' "$TBS_CONFIG_BACKUP_DIR"
```

Die beiden Verzeichnisnamen aufbewahren. Die Konfigurationssicherung enthält Zugangsdaten und gehört nicht in Git oder in einen öffentlichen Fehlerbericht.

## 2. Auf den Design-Branch wechseln

```bash
git fetch origin --prune
if git show-ref --verify --quiet refs/heads/feat/netcore-dashboard-design; then
    git switch feat/netcore-dashboard-design
    git merge --ff-only origin/feat/netcore-dashboard-design
else
    git switch --track -c feat/netcore-dashboard-design origin/feat/netcore-dashboard-design
fi
git status --short --branch
git rev-parse HEAD
cargo check --locked -p bluestation-bs
```

Ein fehlgeschlagener Branchwechsel, nicht möglicher Fast-forward oder Buildfehler muss vor der Installation geklärt werden. Der gezielte Build erhält die Default-Features `asterisk`, `recording` und `audio-player`. Nicht zur Fehlerumgehung `--no-default-features` hinzufügen; das würde die Ausstattung ändern.

## 3. Bauen und die aktive Binary aktualisieren

Der vorhandene Updater baut als der Benutzer, der `sudo` aufruft, und ersetzt die tatsächlich verwendete Binary. **Die beiden TTS-Schalter sind für dieses reine Design-Update ausdrücklich `0`:** Sonst würde der allgemeine Updater standardmäßig lokale TTS-Konfiguration migrieren und einen vorhandenen lokalen Piper-Dienst deaktivieren.

```bash
set -o pipefail
sudo env \
    UNIT="$UNIT" \
    BINARY_PATH="$BINARY_PATH" \
    CONFIG_PATH="$CONFIG_PATH" \
    MIGRATE_LOCAL_TTS_CONFIG=0 \
    DISABLE_LOCAL_PIPER=0 \
    bash install/update-basisstation.sh 2>&1 | tee "$TBS_STATE_DIR/update.log"
TBS_UPDATE_STATUS="${PIPESTATUS[0]}"
sed -n 's/^\[NetCore Basisstation Update\] Alte Binary gesichert: //p' \
    "$TBS_STATE_DIR/update.log" > "$TBS_STATE_DIR/binary-backup-path.txt"
printf 'Updater-Exitcode: %s\n' "$TBS_UPDATE_STATUS"
cat "$TBS_STATE_DIR/binary-backup-path.txt"
```

Nur Exitcode `0` bedeutet Erfolg. Die vorherige Binary liegt normalerweise unter `/var/backups/netcore-tetra/bluestation-bs.JJJJMMTT-HHMMSS.bak`; den **tatsächlich protokollierten Pfad** verwenden. Der Updater sichert die Binary vor dem Stoppen des Dienstes und versucht bei einem Startfehler einen automatischen Rückweg. Seine Startkontrolle ersetzt die folgende Funktionsprüfung nicht. Konfiguration, Profile und Medien werden durch die beiden oben deaktivierten TTS-Aktionen nicht migriert.

## 4. Das neue Dashboard prüfen

```bash
sudo systemctl is-active "$UNIT"
sudo systemctl status "$UNIT" --no-pager
sudo journalctl -u "$UNIT" --since '10 minutes ago' --no-pager -n 150
```

Die bisherige Dashboard-Adresse öffnen und vollständig neu laden (`Strg`+`F5` beziehungsweise `Cmd`+`Shift`+`R`). Bei einer alten Darstellung zunächst einen privaten Browser-Tab testen. Es müssen weiterhin echte Stationsdaten erscheinen; fehlende Messwerte werden als fehlend angezeigt.

- Anmeldung, Abmeldung sowie Haupt- und Unternavigation prüfen; anschließend eine schmale Fensterbreite oder das Mobilgerät testen.
- Hauptseite, Funkgeräte, Rufe, SDS, Paketdaten, Karte, Systemzustand, Log und Dienste mit dem tatsächlichen Stationszustand vergleichen. Konfiguration und Integrationen müssen ihre bisherigen Werte und Aktionen behalten.
- Auf der RF-Seite Spektrum, Wasserfall, Konstellation und DSP-/SDR-Werte prüfen. Die DSP-Anzeige betrachtet das **TX-Signal vor der Endstufe**. Sie ist keine kalibrierte Messung der Antennenleistung. Beim zweiten Träger bleibt der physische TS1 für Steuerung/Guard reserviert; dessen Verkehrsslots entsprechen den logischen TS5–TS7.
- Audio-Zentrale und getrennte Aufzeichnungen öffnen. Bei bestehender Nutzung auch eine Aufnahme abspielen sowie Registrierung, einen Ruf und SDS unter den bisherigen Betriebsbedingungen prüfen.
- Nachbarzellen zeigt die tatsächlich konfigurierten Zellen und Kennungen. Live-Verfügbarkeit und Handover werden dort noch nicht gemessen; die ausführlichen Hilfetexte folgen später.
- Die Seiten für DAPNET, EchoLink, MeshCom und GeoAlarm behalten ihre bisherige interne Sichtbarkeit. WLAN bleibt abhängig von der vorhandenen Netzwerkverwaltung.

Der Status vor der Anmeldung ist optional. Die vorhandene Einstellung `public_overview` im Abschnitt `[dashboard]` entscheidet, ob anonyme Besucher eine öffentliche Übersicht erhalten. Das Update aktiviert diese Einstellung nicht automatisch. Ohne Freigabe öffentlicher Daten bleibt die Anmeldung auch ohne diese Statuswerte benutzbar. Zugangsdaten und Teilnehmerdaten gehören nicht in den öffentlichen Status.

## 5. Bei Bedarf zurückwechseln

Falls du eine neue SSH-Sitzung verwendest, zuerst `TBS_STATE_DIR` auf das zuvor ausgegebene Verzeichnis setzen. Dann den gespeicherten Rückweg prüfen:

```bash
UNIT="$(cat "$TBS_STATE_DIR/unit.txt")"
BINARY_PATH="$(cat "$TBS_STATE_DIR/binary-path.txt")"
TBS_BINARY_BACKUP="$(cat "$TBS_STATE_DIR/binary-backup-path.txt")"
TBS_PREVIOUS_COMMIT="$(cat "$TBS_STATE_DIR/commit-before.txt")"
TBS_PREVIOUS_BRANCH="$(cat "$TBS_STATE_DIR/branch-before.txt")"
printf 'Unit: %s\nBinary: %s\nBackup: %s\nCommit: %s\n' \
    "$UNIT" "$BINARY_PATH" "$TBS_BINARY_BACKUP" "$TBS_PREVIOUS_COMMIT"
sudo test -f "$TBS_BINARY_BACKUP"
```

Nur mit vorhandener, passender Sicherung fortfahren:

```bash
sudo systemctl stop "$UNIT"
sudo install -m 0755 -- "$TBS_BINARY_BACKUP" "$BINARY_PATH"
sudo systemctl start "$UNIT"
sudo systemctl is-active "$UNIT"
sudo journalctl -u "$UNIT" --since '5 minutes ago' --no-pager -n 100
```

Die Konfiguration bei diesem Design-Rollback nicht überschreiben. Wurde sie nach dem Update separat geändert, diese Änderungen gesondert beurteilen; die geschützte Kopie ist für die bewusste Wiederherstellung vorhanden.

Auch den Quellcode zurückstellen, damit ein späterer Build nicht versehentlich wieder das neue Design installiert. Vorher erneut `git status --short --branch` prüfen und neue lokale Änderungen sichern. Der gespeicherte Commit ist der genaue vorherige Quellstand:

```bash
git status --short --branch
git switch --detach "$TBS_PREVIOUS_COMMIT"
```

Der detached Stand ist für den exakten Rückweg beabsichtigt. Ist `TBS_PREVIOUS_BRANCH` nicht leer und soll wieder auf diesem Branch gearbeitet werden, zunächst dessen Commit mit `git rev-parse "$TBS_PREVIOUS_BRANCH"` vergleichen und nur bei passendem Stand `git switch "$TBS_PREVIOUS_BRANCH"` verwenden. Anschließend den Browser erneut vollständig laden.

## Wenn etwas scheitert

| Beobachtung | Nächster Schritt |
|---|---|
| Cargo wird unter `sudo` nicht gefunden | Den Updater als bisherigen Build-Benutzer mit `sudo` aufrufen. Er sucht Cargo in dessen Rustup-Installation. Falls nötig `BUILD_USER` und `CARGO_BIN` ausdrücklich über `sudo env` setzen. |
| Keine Unit oder falscher Binärpfad | `systemctl show`/`systemctl cat` der tatsächlichen Basisstations-Unit prüfen und `UNIT`, `CONFIG_PATH`, `BINARY_PATH` ausdrücklich setzen. |
| SoapySDR-/Codec-/Linkerfehler | Die auf dieser Installation verwendeten nativen Abhängigkeiten und deren Architektur prüfen; [Häufige Buildfehler](../wiki/Common-Build-Errors.md) beachten. Den Dienst noch nicht ersetzen. |
| Dienst startet mit einer Fallback-Konfiguration | Das Journal sowie Konfigurationspfad und Leserechte für den Dienstbenutzer prüfen. Kein neues Beispiel-TOML über die bestehende Konfiguration kopieren. |
| Dienst läuft, Oberfläche zeigt Fehler | Browser-Konsole und Netzwerkfehler sowie das Journal prüfen; bei Funktionsverlust die gesicherte Binary zurückspielen. Öffentliche Screenshots und Logs von Zugangsdaten bereinigen. |
