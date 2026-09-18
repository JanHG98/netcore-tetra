# NINA/KATWARN installieren — was du auf welchem System machen musst

**Diese Reihenfolge abarbeiten: SDS-Router-LXC aktualisieren → Control-Room-LXC aktualisieren und mit Node Gateway verbinden → neuen Warn-LXC erstellen → TBS aktualisieren und prüfen → Versand einschalten.**

| System | Was du machen musst |
|---|---|
| Vorhandener **SDS-Router-LXC** | **Aktualisieren** — Schritt 1 |
| Vorhandener **Control-Room-LXC** | **Aktualisieren und Gateway-Telemetrie einschalten** — Schritt 1b |
| Neuer **alert-service-LXC** | **Erstellen und installieren** — Schritt 2 |
| Jede vorhandene **TBS** | **Aktualisieren, dann Anmeldung, GPS und SDS prüfen** — Schritt 3 |
| Node Gateway und alle übrigen LXCs | **Für diese Funktion kein Update nötig** |

Die Warnfunktion liegt im Branch **`feat/katwarn-nina-alerts`** aus [PR #49](https://github.com/JanHG98/netcore-tetra/pull/49). Die folgenden Befehle verwenden genau diesen Branch. Der PR ist noch nicht in `katwarn/nina` zusammengeführt.

**SDS Router oder Call Control wechseln ständig zwischen ausgefallen, Fallback und recovering?** Dafür gibt es eine separate [Reparaturanleitung mit Befehlen je LXC](SDS_CALL_CONTROL_FALLBACK_REPARATUR.md). Diese Fehlerkorrektur erfordert ein Update von **SDS Router und Call Control**; Node Gateway und TBS müssen dafür nicht aktualisiert werden.

**Vorher notieren:** IP deines SDS-Router-LXC, IP deines Control-Room-LXC, IP deines Node-Gateway-LXC und später die IP des neuen Warn-LXC. Die Bezeichnungen `SDS-ROUTER-IP`, `CONTROL-ROOM-IP`, `NODE-GATEWAY-IP` und `WARN-LXC-IP` unten durch diese echten Adressen ersetzen.

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

## 1b. Vorhandener Control-Room-LXC: Muss aktualisiert werden

**Dieser Schritt fehlte in der ersten Fassung der Anleitung.** Wenn deine TBS am Node Gateway hängen, bekommt der bisherige Control Room deren Geräte- und GPS-Telemetrie nicht automatisch. Die Warnzentrale erhielt deshalb leere Listen, auch wenn der SDS-Router korrekt verbunden war.

### Software auf dem Control-Room-LXC aktualisieren

Vorher in Proxmox ein Backup des Control-Room-LXC erstellen. **Diesen gesamten Block in seiner Konsole als root ausführen.** Er baut aus einer zusätzlichen Arbeitskopie und erhält den vorhandenen Projektordner.

```bash
(
  set -e
  test -s /etc/netcore-control-room/control-room.toml
  cd /opt
  update_dir=$(mktemp -d /opt/netcore-control-room-warn.XXXXXX)
  git clone --single-branch --branch feat/katwarn-nina-alerts https://github.com/JanHG98/netcore-tetra.git "$update_dir"
  cd "$update_dir"
  if [ -f /root/.cargo/env ]; then . /root/.cargo/env; fi
  bash system-backend/control-room/install/update.sh
  systemctl is-active netcore-control-room
)
```

Der Updater baut vor dem Dienststopp. Bei `active` mit der Konfiguration weitermachen.

### Node-Gateway-Verbindung auf dem Control-Room-LXC eintragen

Die bereits vom SDS-Router verwendete Gateway-Adresse lässt sich **vom Warn- oder Control-Room-LXC** aus anzeigen:

```bash
curl --noproxy '*' -fsS http://SDS-ROUTER-IP:8150/api/v1/config | python3 -c 'import json,sys; print(json.load(sys.stdin)["node_gateway"]["url"])'
```

**Auf dem Control-Room-LXC** die Konfiguration öffnen:

```bash
nano /etc/netcore-control-room/control-room.toml
```

Diesen Abschnitt ergänzen; wenn er schon existiert, seine Werte bearbeiten statt einen zweiten anzulegen. Für `url` die gerade angezeigte Adresse verwenden:

```toml
[node_gateway]
enabled = true
url = "ws://NODE-GATEWAY-IP:8080/ws/backend"
```

Speichern (Strg+O → Enter → Strg+X), dann **auf dem Control-Room-LXC**:

```bash
systemctl restart netcore-control-room
systemctl is-active netcore-control-room
journalctl -u netcore-control-room -n 30 --no-pager
```

Die Verbindung liest nur Telemetrie. TBS und SDS-Router bleiben am vorhandenen Node Gateway; deren URLs nicht umstellen.

### Neue Gerätedaten eintreffen lassen und prüfen

Nach dem ersten Einschalten dieser Verbindung **das Testfunkgerät neu anmelden und eine aktuelle GPS-Meldung senden lassen**. Das Node Gateway liefert keine vollständige Historie alter Gerätepositionen nach. Ein Neustart der TBS ist dafür nicht erforderlich.

**Auf dem Warn-LXC oder Control-Room-LXC prüfen:**

```bash
curl --noproxy '*' -fsS http://CONTROL-ROOM-IP:9010/api/nodes
curl --noproxy '*' -fsS 'http://CONTROL-ROOM-IP:9010/api/subscribers?online=true'
```

Erwartet: TBS in der ersten Antwort; Testgerät mit `online: true` und `last_location` in der zweiten. Bei aktivierter Control-Room-Anmeldung diese Ansicht im angemeldeten Browser prüfen. Danach erkennt die bereits laufende Warnzentrale das Gerät beim nächsten Abgleich.

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

## 3. Jede TBS: Aktualisieren und Funkversand prüfen

**Korrektur zur ersten Anleitung: Ein TBS-Softwareupdate ist erforderlich.** Der zentrale SDS-Befehl `DeliverSds` war zwar im Protokoll und im SDS-Untermodul vorhanden, wurde aber vom vorgeschalteten CMCE-Befehlshandler nicht weitergeleitet. Dadurch konnte die Warnzentrale das Gerät mit GPS erkennen und der SDS Router den Auftrag übernehmen, ohne dass eine Funk-SDS entstand. Auch `SendStatus` war von dieser fehlenden Weiterleitung betroffen.

### Auf jeder TBS die Software aktualisieren

**Bei manuellem Start ohne systemd-Dienst:** Den folgenden Service-Updater überspringen und den Abschnitt „Manuell gestartete TBS“ darunter verwenden.

**Diese Befehle direkt auf der Basisstation ausführen**, angemeldet mit dem Linux-Benutzer, mit dem du normalerweise Rust/Cargo verwendest. Der folgende Block erstellt eine zusätzliche Arbeitskopie, baut bei laufendem Dienst und verwendet den vorhandenen Updater. Dieser erkennt den Dienst und die tatsächlich gestartete ausführbare Datei, sichert sie und startet die TBS anschließend neu. Dabei wird der Funkbetrieb kurz unterbrochen.

```bash
(
  set -e
  update_dir=$(mktemp -d "$HOME/netcore-tbs-sds.XXXXXX")
  git clone --single-branch --branch feat/katwarn-nina-alerts https://github.com/JanHG98/netcore-tetra.git "$update_dir"
  cd "$update_dir"
  sudo env MIGRATE_LOCAL_TTS_CONFIG=0 DISABLE_LOCAL_PIPER=0 bash install/update-basisstation.sh
)
```

Die beiden gesetzten Optionen erhalten bestehende TTS-Einstellungen und den Piper-Dienst. Die Funkkonfiguration bleibt erhalten. Standard-Konfigurationspfad ist `/etc/netcore/config.toml`; bei einer abweichenden Installation `CONFIG_PATH=/dein/pfad/config.toml` zusätzlich hinter `sudo env` eintragen. Unterstützte automatisch erkannte Dienstnamen sind `tetra.service`, `bluestation.service`, `tetra-bluestation.service` und `bluestation-bs.service`; ein abweichender Name kann dort mit `UNIT=dein-dienst.service` angegeben werden. Der Updater zeigt den gewählten Dienst und Programmpfad an.

Bei fehlenden Build-Abhängigkeiten bricht der Updater vor dem Dienststopp ab. Er verwendet den Cargo-Benutzer des `sudo`-Aufrufs und die standardmäßig aktivierten TBS-Funktionen. Den Updateblock erst nach erfolgreichem Build und gemeldetem `Update erfolgreich` als abgeschlossen betrachten.

### Manuell gestartete TBS

**In einem zweiten Terminal auf der TBS**, mit demselben Benutzer wie beim bisherigen Build:

```bash
(
  set -e
  update_dir=$(mktemp -d "$HOME/netcore-tbs-sds.XXXXXX")
  git clone --single-branch --branch feat/katwarn-nina-alerts https://github.com/JanHG98/netcore-tetra.git "$update_dir"
  cd "$update_dir"
  if [ -f "$HOME/.cargo/env" ]; then . "$HOME/.cargo/env"; fi
  cargo build --release -p bluestation-bs
  printf '\nNeues Programm: %s/target/release/bluestation-bs\n' "$update_dir"
)
```

Der bisherige Funkprozess kann während des Builds weiterlaufen. Erst nach erfolgreichem Build im bisherigen TBS-Terminal mit **Strg+C** beenden. Anschließend **im bisherigen Arbeitsverzeichnis** denselben Startbefehl wie bisher verwenden, darin ausschließlich den Programmpfad durch den ausgegebenen vollständigen Pfad zum neuen `bluestation-bs` ersetzen. Alle Argumente und insbesondere den Konfigurationspfad beibehalten. Nicht gleichzeitig eine zweite TBS-Instanz starten. Die bisherige ausführbare Datei bleibt für einen Rückwechsel erhalten.

**Konkrete Variante für den manuellen Start als root aus `/opt/netcore-tetra` mit `./target/release/bluestation-bs ./config.toml`:** Statt des obigen manuellen Blocks diesen Block in einem zweiten TBS-Terminal ausführen. Er ersetzt nach erfolgreichem Build genau die bisher verwendete Programmdatei; der Startbefehl bleibt gleich.

```bash
(
  set -e
  test -s /opt/netcore-tetra/config.toml
  test -x /opt/netcore-tetra/target/release/bluestation-bs
  update_dir=$(mktemp -d /opt/netcore-tbs-sds.XXXXXX)
  git clone --single-branch --branch feat/katwarn-nina-alerts https://github.com/JanHG98/netcore-tetra.git "$update_dir"
  cd "$update_dir"
  if [ -f /root/.cargo/env ]; then . /root/.cargo/env; fi
  cargo build --release -p bluestation-bs
  binary=/opt/netcore-tetra/target/release/bluestation-bs
  cp -a "$binary" "${binary}.bak.$(date +%Y%m%d-%H%M%S)"
  install -m 0755 target/release/bluestation-bs "${binary}.new"
  mv -f "${binary}.new" "$binary"
  echo 'Build und Austausch erfolgreich. Jetzt die laufende TBS mit Strg+C beenden und neu starten.'
)
```

Die laufende Instanz verwendet bis zum Beenden noch das alte Programm. **Nur nach erfolgreichem Abschluss** im bisherigen TBS-Terminal Strg+C drücken und anschließend wieder starten:

```bash
cd /opt/netcore-tetra
./target/release/bluestation-bs ./config.toml
```

### Nach dem TBS-Update prüfen

An **jeder TBS**, deren Geräte Warnungen bekommen sollen:

1. Ein Testfunkgerät einschalten und anmelden.
2. Eine aktuelle GPS-Position übertragen lassen.
3. Im vorhandenen **Control Room** prüfen: Gerät online, Position richtig.
4. Im **SDS-Router-WebUI** unter `http://SDS-ROUTER-IP:8150/` auf **„Nachricht senden“** klicken: Quell-ISSI wie die oben gewählte `source_issi`, Ziel-ISSI des Testfunkgeräts, Adressart **„Einzel“**, SDS-Typ **„Type 4“**, Protocol-ID **`130`**, Text **`TEST Warnzentrale`**. Dann **„Einplanen“** anklicken.
5. Am Funkgerät prüfen, dass die SDS ankommt.
6. In der **neuen Warnzentrale** prüfen, dass das Gerät auf der Karte erscheint.

**Wenn das funktioniert, ist auf dieser TBS alles erledigt.** Keine Warnservice-URL eintragen und `central_sds_routing` nicht für diese Funktion ändern. Dieser Schalter betrifft eingehende Funk-SDS, nicht den Versand der Warnungen.

**Wurde bereits vor dem Update eine Testwarnung an diese TBS übergeben?** Nach dem Update eine **neue** Testwarnung erstellen. Die alte Warnung wird wegen der dauerhaften Duplikatsperre nicht automatisch noch einmal versendet. Die Empfängerhistorie deshalb nicht löschen. Im SDS Router bedeutet `delivered`, dass die TBS die Nachricht zum Funkversand angenommen hat; den tatsächlichen Empfang zusätzlich am Funkgerät prüfen.

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

### Wenn trotz aktivem Versand keine Nachricht ankommt

**In der Warn-WebUI zuerst „Geräte mit aktuellem GPS“ und „Geräteprüfung“ ansehen.** Ein Gerät muss dort für Warnungen verfügbar sein. Seine tatsächliche Position im Kreis allein reicht nicht, solange der Dienst diese Position nicht vom Control Room bekommt.

| Anzeige | Auf welchem System du was prüfen musst |
|---|---|
| Control Room meldet keine angemeldeten Geräte | **Warn-LXC:** `control_room_url` prüfen. Ist die Adresse korrekt und sind auch `/api/nodes` und `/api/subscribers` leer, auf dem **Control-Room-LXC Schritt 1b** durchführen. Danach Testgerät neu anmelden und GPS übertragen lassen. |
| GPS fehlt oder ist zu alt | **Testfunkgerät/TBS:** Eine aktuelle GPS-Meldung übertragen lassen. **Control Room:** Den Zeitstempel der Position prüfen; standardmäßig darf sie höchstens 3600 Sekunden alt sein. |
| TBS nicht verbunden oder Meldung zu alt | **Control Room/Node Gateway:** TBS-Verbindung prüfen. Standardmäßig muss die TBS-Meldung jünger als 120 Sekunden sein. |
| Beim SDS-Router | **SDS-Router-WebUI:** Den Auftrag und seine TBS-Zustellung prüfen. Die Nachricht wurde bereits an den Router übergeben. Bleibt sie bei älterer TBS-Software auf `in_flight` und kommt keine Funk-SDS an, **TBS nach Schritt 3 aktualisieren** und danach eine neue Testwarnung erstellen. |
| Fehlgeschlagen oder Unklar | Den Hinweis in der Zustellhistorie prüfen. Die Empfängerhistorie nicht löschen, um einen erneuten Versand zu erzwingen. |

**Auf dem Warn-LXC die verwendete Control-Room-Adresse anzeigen:**

```bash
python3 - <<'PY'
import tomllib
with open('/etc/netcore/alert-service.toml', 'rb') as f:
    print(tomllib.load(f)['netcore']['control_room_url'])
PY
```

Zeigt sie auf die falsche Instanz, **auf dem Warn-LXC** korrigieren:

```bash
nano /etc/netcore/alert-service.toml
```

Im vorhandenen Abschnitt `[netcore]` nur `control_room_url` auf die tatsächliche Control-Room-API setzen (normalerweise `http://CONTROL-ROOM-IP:9010`). Danach:

```bash
systemctl restart netcore-alert-service
```

Nach dem nächsten Geräteabgleich müssen Gerätezahl und Karte die angemeldeten Geräte mit aktueller Position zeigen. Eine noch aktive Testwarnung wird anschließend automatisch geprüft.

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

Für alle anderen vorhandenen LXCs ist im Rahmen dieser Warnfunktion nichts zu installieren oder zu aktualisieren. Die separate [Reparatur für wechselnden SDS-/Call-Control-Fallback](SDS_CALL_CONTROL_FALLBACK_REPARATUR.md) betrifft zusätzlich den Call-Control-LXC. Technische Hintergründe stehen in der [Dienstbeschreibung](../system-backend/alert-service/README.md).
