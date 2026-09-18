# NINA/KATWARN installieren — was du auf welchem System machen musst

**Diese Reihenfolge abarbeiten: SDS-Router-LXC aktualisieren → neuen Warn-LXC erstellen → TBS prüfen → Versand einschalten.**

| System | Was du machen musst |
|---|---|
| Vorhandener **SDS-Router-LXC** | **Aktualisieren** — Schritt 1 |
| Neuer **alert-service-LXC** | **Erstellen und installieren** — Schritt 2 |
| Jede vorhandene **TBS** | **Anmeldung, GPS und SDS prüfen** — Schritt 3 |
| Control Room, Node Gateway und alle übrigen LXCs | **Für diese Funktion kein Update nötig** |

Die Warnfunktion liegt im Branch **`feat/katwarn-nina-alerts`** aus [PR #49](https://github.com/JanHG98/netcore-tetra/pull/49). Die folgenden Befehle verwenden genau diesen Branch. Der PR ist noch nicht in `katwarn/nina` zusammengeführt.

**Vorher notieren:** IP deines SDS-Router-LXC, IP deines Control-Room-LXC und später die IP des neuen Warn-LXC. Die Bezeichnungen `SDS-ROUTER-IP`, `CONTROL-ROOM-IP` und `WARN-LXC-IP` unten durch diese echten Adressen ersetzen.

## 1. Vorhandener SDS-Router-LXC: Muss aktualisiert werden

**Diese Befehle in der Konsole des SDS-Router-LXC als root ausführen.**

Vorher in Proxmox ein Backup dieses LXC erstellen. Seine bestehende Konfiguration und Datenbank müssen erhalten bleiben.

### Projekt aktualisieren

```bash
cd /opt/netcore-tetra
git status --short
```

Bei leerer Ausgabe weiter. Werden eigene Änderungen angezeigt, diese zuerst sichern/übernehmen. Liegt dein Repository woanders, nur die `cd`-Zeile anpassen.

```bash
git fetch origin feat/katwarn-nina-alerts:refs/remotes/origin/feat/katwarn-nina-alerts
git switch feat/katwarn-nina-alerts
git pull --ff-only origin feat/katwarn-nina-alerts
```

### Software bauen und Update ausführen

```bash
if [ -f /root/.cargo/env ]; then . /root/.cargo/env; fi
cargo build --release -p netcore-sds-router
```

**Nur wenn der Build erfolgreich war**, weiter:

```bash
bash system-backend/sds-router/install/update.sh
systemctl is-active netcore-sds-router
curl --fail --show-error http://SDS-ROUTER-IP:8150/api/v1/status
```

**Erwartetes Ergebnis:** Dienst `active`. In der Antwort müssen `durable_idempotency`, `at_most_once` und `node_gateway_connected` jeweils `true` sein.

Der Updater bindet an die echte LXC-IP. Deshalb hier **nicht `127.0.0.1`** verwenden. Die erkannte Adresse steht auch in `/etc/netcore/lxc-network.env`.

Bei einem Fehler auf diesem LXC prüfen:

```bash
journalctl -u netcore-sds-router -n 80 --no-pager
```

### Falls `update.sh` fehlt oder der Ordner `install` leer ist

Die Datei ist im oben genannten GitHub-Branch enthalten. Verwende in diesem Fall eine frische Arbeitskopie in einem eigenen Ordner. Der bisherige Projektordner bleibt erhalten.

**Diesen ganzen Block auf dem SDS-Router-LXC als root ausführen:**

```bash
(
  set -e
  cd /opt
  git clone --single-branch --branch feat/katwarn-nina-alerts https://github.com/JanHG98/netcore-tetra.git netcore-tetra-warn-update
  cd /opt/netcore-tetra-warn-update
  test -s system-backend/sds-router/install/update.sh
  test -s /etc/netcore/sds-router.toml
  . system-backend/shared/install/lxc-network.sh
  netcore_detect_lxc_ipv4 >/dev/null
  if [ -f /root/.cargo/env ]; then . /root/.cargo/env; fi
  cargo build --release -p netcore-sds-router
  bash system-backend/sds-router/install/update.sh
  systemctl is-active netcore-sds-router
)
```

Der Block bricht beim ersten Fehler ab. Der eigentliche Dienstwechsel erfolgt erst nach erfolgreichem Build. Existiert `/opt/netcore-tetra-warn-update` bereits, stoppt das Klonen; den vorhandenen Ordner nicht ungeprüft löschen. Anschließend den Status wie oben über die echte SDS-Router-IP prüfen.

## 2. Neuer LXC „alert-service“: Muss erstellt werden

### 2.1 In Proxmox den Container erstellen

**In der Proxmox-Weboberfläche „CT erstellen“ auswählen:**

| Einstellung | Wert |
|---|---|
| CT-ID | Eine freie CT-ID |
| Hostname | `alert-service` |
| Unprivilegierter Container | Ja |
| Template | Debian 13 Standard; Debian 12 funktioniert ebenfalls |
| Festplatte / CPU / RAM / Swap | 8 GB / 1 Kern / 1024 MB / 256 MB |
| Netzwerk-Bridge | Deine bestehende Management-Bridge |
| IP-Adresse | Freie feste IP oder DHCP mit fester Reservierung |
| Gateway und DNS | Wie bei deinen bestehenden Backend-LXCs |
| Autostart | Aktivieren |

Container erstellen und starten. Nesting und Geräte-Passthrough werden nicht benötigt.

### 2.2 Im neuen LXC installieren

**Ab jetzt alle Befehle in der Konsole des neuen alert-service-LXC als root ausführen.**

```bash
apt-get update
apt-get install -y git python3 ca-certificates curl nano
git clone --branch feat/katwarn-nina-alerts --single-branch https://github.com/JanHG98/netcore-tetra.git /opt/netcore-tetra
cd /opt/netcore-tetra
bash system-backend/alert-service/install/install.sh
```

**Falls eine ältere Installer-Version direkt nach dem Start `ConnectionRefusedError` ausgibt:** Die erste Prüfung kann kommen, bevor der Dienst bereit ist. Auf demselben Warn-LXC prüfen:

```bash
systemctl status netcore-alert-service --no-pager -l
curl --noproxy '*' --fail --show-error http://127.0.0.1:8310/health/live
```

Bei **`active (running)`** und **`{"status": "live"}`** war die Installation erfolgreich; direkt mit 2.3 weitermachen. Keine Neuinstallation nötig. Falls der Dienst nicht läuft oder die Abfrage weiter fehlschlägt:

```bash
journalctl -u netcore-alert-service -n 40 --no-pager
```

Der aktuelle Installer wartet bei kurzen Startverzögerungen ohne Python-Fehlerausgabe und zeigt bei einem dauerhaften Fehler das Dienstprotokoll an.

### 2.3 Im neuen LXC deine Adressen eintragen

```bash
nano /etc/netcore/alert-service.toml
```

Im **vorhandenen Abschnitt `[netcore]`** diese drei Zeilen bearbeiten:

```toml
control_room_url = "http://CONTROL-ROOM-IP:9010"
sds_router_url = "http://SDS-ROUTER-IP:8150"
source_issi = 9999
```

Die IP-Platzhalter ersetzen. Für `source_issi` eine freie, in deinem Netz erlaubte Absender-ISSI verwenden; `9999` nur behalten, wenn sie dafür frei ist. Im Abschnitt `[delivery]` bleibt zunächst **`enabled = false`**.

**Nur falls dein Control Room einen Login verlangt:** Im Abschnitt `[netcore]` auch `control_room_username = "DEIN-BENUTZERNAME"` eintragen. Anschließend:

```bash
nano /etc/netcore/alert-service.env
```

Dort ergänzen:

```text
NETCORE_CONTROL_ROOM_PASSWORD="DEIN-PASSWORT"
```

Die vorhandene Zeile `NETCORE_ALERT_TOKEN=...` behalten. Ohne Control-Room-Anmeldung diesen Passwortschritt überspringen.

**In nano speichern:** Strg+O → Enter → Strg+X.

### 2.4 Im neuen LXC neu starten und prüfen

```bash
systemctl restart netcore-alert-service
systemctl is-active netcore-alert-service
curl --fail --show-error http://127.0.0.1:8310/health/live
```

Erwartet: **`active`** und **`{"status": "live"}`**.

Nach dem ersten Datenabgleich, normalerweise innerhalb einer Minute:

```bash
curl --fail --show-error http://127.0.0.1:8310/health/ready
```

Erwartet: **`{"status": "ready"}`**. Falls nicht:

```bash
journalctl -u netcore-alert-service -n 80 --no-pager
```

Der neue LXC braucht Zugriff auf den **Control Room:9010**, **SDS Router:8150**, DNS und **warnung.bund.de:443**. Dein Browser muss den neuen LXC auf **Port 8310** erreichen können.

### 2.5 Weboberfläche öffnen

**Auf deinem PC im Browser öffnen:**

```text
http://WARN-LXC-IP:8310/
```

**In der Konsole des neuen LXC** den Zugriffsschlüssel anzeigen:

```bash
cat /etc/netcore/alert-service.env
```

Nur den Wert hinter **`NETCORE_ALERT_TOKEN=`** in das Feld „Zugriffsschlüssel“ kopieren und „Verbinden“ anklicken. Die Datei enthält Geheimnisse; nicht weitergeben.

## 3. Jede TBS: Diese Prüfungen durchführen

**Für diese Erweiterung ist kein TBS-Softwareupdate nötig.** Die vorhandene Anmeldung, GPS-Übertragung und zentrale SDS-Anbindung werden weiterverwendet.

An **jeder TBS**, deren Geräte Warnungen bekommen sollen:

1. Ein Testfunkgerät einschalten und anmelden.
2. Eine aktuelle GPS-Position übertragen lassen.
3. Im vorhandenen **Control Room** prüfen: Gerät online, Position richtig.
4. Im **SDS-Router-WebUI** unter `http://SDS-ROUTER-IP:8150/` auf **„Nachricht senden“** klicken: Quell-ISSI wie die oben gewählte `source_issi`, Ziel-ISSI des Testfunkgeräts, Adressart **„Einzel“**, SDS-Typ **„Type 4“**, Protocol-ID **`130`**, Text **`TEST Warnzentrale`**. Dann **„Einplanen“** anklicken.
5. Am Funkgerät prüfen, dass die SDS ankommt.
6. In der **neuen Warnzentrale** prüfen, dass das Gerät auf der Karte erscheint.

**Wenn das funktioniert, ist auf dieser TBS alles erledigt.** Keine Warnservice-URL eintragen und `central_sds_routing` nicht für diese Funktion ändern. Dieser Schalter betrifft eingehende Funk-SDS, nicht den Versand der Warnungen.

Falls Anmeldung, GPS oder die Test-SDS schon hier nicht funktionieren, auf der betreffenden TBS prüfen:

```bash
systemctl list-units --type=service --all | grep -E 'tetra|bluestation'
```

Dann mit dem angezeigten Dienstnamen, beispielsweise `tetra.service`:

```bash
systemctl status tetra.service --no-pager
journalctl -u tetra.service -n 80 --no-pager
```

Erst diese bestehende Verbindung wiederherstellen, danach Schritt 4 ausführen.

## 4. Neuer Warn-LXC: Versand einschalten und testen

**Zuerst in der WebUI:**

1. „Eigene Meldung“ anklicken.
2. Die Position des Testfunkgeräts **auf der Karte anklicken** oder dessen Breitengrad und Längengrad im Formular eintragen.
3. Radius **1000 Meter**, Titel **„TEST Warnzentrale“**, Ablaufzeit **in 15 Minuten**.
4. „Meldung erstellen“ anklicken. Der Versand steht noch auf „Pausiert“.

**Dann in der Konsole des neuen alert-service-LXC als root:**

```bash
nano /etc/netcore/alert-service.toml
```

Im **vorhandenen Abschnitt `[delivery]`** ändern:

```toml
enabled = true
```

Speichern, danach:

```bash
systemctl restart netcore-alert-service
systemctl is-active netcore-alert-service
```

**Jetzt am Funkgerät prüfen:**

1. Das angemeldete Gerät im Kreis erhält nach dem nächsten Abgleich eine SDS.
2. Dasselbe Gerät aus- und wieder einschalten: **Die gleiche Warnung darf nicht erneut kommen.**
3. Ein zweites Gerät im Kreis anmelden: Es erhält die aktive Warnung ebenfalls einmal.
4. Testwarnung in der WebUI über **„Löschen“ → „Jetzt löschen“** entfernen.

**Fertig.** Neue Warnungen werden automatisch verteilt. Der Dienst prüft Geräte normalerweise alle 5 Sekunden und NINA alle 60 Sekunden, jeweils zuzüglich Netzlaufzeit.

## 5. Spätere Updates: Nur auf dem Warn-LXC

**In der Konsole des alert-service-LXC als root:**

```bash
cd /opt/netcore-tetra
git status --short
```

Bei leerer Ausgabe:

```bash
git pull --ff-only origin feat/katwarn-nina-alerts
bash system-backend/alert-service/install/update.sh
systemctl is-active netcore-alert-service
curl --fail --show-error http://127.0.0.1:8310/health/live
```

Der Updater sichert und erhält Konfiguration, Token und Empfängerhistorie.

**Diese Datenbanken behalten:** Auf dem Warn-LXC `/var/lib/netcore-alert-service/alerts.sqlite3`, auf dem SDS-Router-LXC `/var/lib/netcore-sds-router/messages.json` bzw. dein abweichender `storage.database_path`. Sie enthalten die Duplikatsperren. Nicht löschen oder mit einem alten Stand überschreiben.

Für alle anderen vorhandenen LXCs ist im Rahmen dieser Warnfunktion nichts zu installieren oder zu aktualisieren. Technische Hintergründe stehen separat in der [Dienstbeschreibung](../system-backend/alert-service/README.md).
