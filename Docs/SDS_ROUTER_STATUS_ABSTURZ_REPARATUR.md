# SDS-Router: Absturz nach einer Statusnachricht reparieren

Der Dienst erscheint in `systemctl` als **active** und `/health/live` antwortet mit HTTP `200`. Trotzdem brechen `/health/ready` und `/api/v1/status` ohne Antwort ab; die WebUI zeigt einen Netzwerkfehler und die TBS meldet den SDS-Router als ausgefallen.

Kennzeichnend ist dieser erste Fehler im Dienstprotokoll:

```text
panicked at .../state.rs:...:
index out of bounds: the len is 2 but the index is 2
```

Danach folgen wiederholt `SDS router state poisoned: PoisonError`. Eine Statusnachricht mit zwei Nutzdatenbytes löste einen Zugriff auf ein nicht vorhandenes drittes Byte aus. Dadurch schlugen auch weitere Zugriffe auf den gemeinsamen Routerzustand fehl. Die Korrektur liest die Nachrichtenreferenz erst nach der passenden Typ- und Längenprüfung. Ein bloßer Neustart beseitigt die Fehlerursache nicht.

## Was auf welchem System zu tun ist

| System | Aktion |
|---|---|
| **Bestehender SDS-Router-LXC** | Schritt 1 und 2 als root ausführen: Software aktualisieren und Antworten prüfen |
| **Call-Control-LXC, Node-Gateway-LXC, Control-Room-LXC, Warn-LXC** | Für diese Reparatur kein Update und kein Neustart erforderlich |
| **Jede TBS** | Kein Update und kein Neustart erforderlich; nach der Reparatur die Dienstanzeige prüfen |
| **Neuer LXC** | Keiner erforderlich |

Die vorhandene SDS-Datenbank und die Warn-Datenbank behalten. Sie enthalten Nachrichten und die Zuordnung bereits behandelter Warnungen zu Geräten. **Keine Datenbank löschen, um den Fehler zu beheben.**

## 1. Nur den SDS-Router-LXC aktualisieren

**Diesen gesamten Block auf dem SDS-Router-LXC als root ausführen:**

```bash
(
  set -e
  test -s /etc/netcore/sds-router.toml
  if [ -f /root/.cargo/env ]; then . /root/.cargo/env; fi
  command -v cargo >/dev/null
  update_dir=$(mktemp -d /opt/netcore-sds-status-fix.XXXXXX)
  git clone --single-branch --branch katwarn/nina https://github.com/JanHG98/netcore-tetra.git "$update_dir"
  cd "$update_dir"
  git fetch origin 2249e3ccb93ec866647c47c64258ae0fd3d763bc
  git checkout --detach FETCH_HEAD
  test -s system-backend/sds-router/install/update.sh
  . system-backend/shared/install/lxc-network.sh
  netcore_detect_lxc_ipv4 >/dev/null
  cargo build --release -p netcore-sds-router
  cp -a /etc/netcore/sds-router.toml "/etc/netcore/sds-router.toml.before-status-fix.$(date +%Y%m%d-%H%M%S)"
  bash system-backend/sds-router/install/update.sh
  systemctl is-active netcore-sds-router
)
```

Der Block lädt eine feste Version mit der Korrektur. Er bleibt dadurch auch verwendbar, wenn der Arbeitsbranch des PR nach dem Merge gelöscht wird. Vorhandene Projektordner werden nicht verändert.

Der erste Build erfolgt vor dem Stoppen des Dienstes. Scheitert er, wird der Updater nicht aufgerufen. Der Updater baut anschließend nochmals mit den bereits gebauten Dateien, installiert das Programm und startet den Dienst. Die vorhandene Gateway-, Routing- und Speicherkonfiguration bleibt erhalten. Wie beim bisherigen Updater wird die Listenadresse auf die erkannte LXC-IPv4-Adresse und Port `8150` gesetzt; bei mehreren Netzanschlüssen kann vorher `NETCORE_LXC_IP` auf die gewünschte Adresse gesetzt werden. Die Konfiguration wird vorher mit einem Zeitstempel gesichert.

**Erwartet:** Der Build ist erfolgreich und am Ende steht `active`. Das bestätigt zunächst nur den laufenden Prozess. Deshalb anschließend Schritt 2 ausführen.

## 2. Auf dem SDS-Router-LXC die Antworten prüfen

Dieser Block liest die konfigurierte Listenadresse und wartet kurz auf die Gateway-Verbindung. Er sendet keine Funknachricht.

```bash
python3 - <<'PY'
import json
import time
import tomllib
import urllib.error
import urllib.request

with open('/etc/netcore/sds-router.toml', 'rb') as f:
    bind = tomllib.load(f)['server']['bind']
bind = bind.replace('0.0.0.0:', '127.0.0.1:', 1).replace('[::]:', '[::1]:', 1)
base = 'http://' + bind
client = urllib.request.build_opener(urllib.request.ProxyHandler({}))
last_error = ''
for attempt in range(20):
    try:
        with client.open(base + '/health/ready', timeout=2) as response:
            json.load(response)
            print('/health/ready: HTTP', response.status)
        break
    except urllib.error.HTTPError as error:
        last_error = 'HTTP ' + str(error.code)
        if error.code == 503:
            last_error += ': Router antwortet, aber die Node-Gateway-Verbindung ist noch nicht bereit.'
    except (OSError, ValueError) as error:
        last_error = str(error)
    time.sleep(1)
else:
    raise SystemExit('Bereitschaftsprüfung fehlgeschlagen: ' + last_error)

for path in ['/health/live', '/api/v1/status']:
    with client.open(base + path, timeout=2) as response:
        json.load(response)
        print(path + ': HTTP', response.status)
PY
```

**Erwartet:** Für alle drei Endpunkte erscheint HTTP `200`. Danach die SDS-Router-WebUI mit **Strg+F5** neu laden. Die TBS sollte den SDS-Router nach den nächsten Überwachungsprüfungen und ihrer Erholungszeit wieder als verfügbar anzeigen.

**HTTP `503`** ist eine reguläre Antwort des Routers: Seine Node-Gateway-Verbindung ist noch nicht hergestellt. Das unterscheidet sich vom beschriebenen Absturz ohne HTTP-Antwort. Bei `503`, einem Verbindungsabbruch oder einem fehlgeschlagenen Update auf dem SDS-Router-LXC auslesen:

```bash
systemctl status netcore-sds-router --no-pager -l
journalctl -u netcore-sds-router --since '10 minutes ago' -n 80 --no-pager
```

## 3. Funkversand nach der Reparatur prüfen

Eine **neue**, kurz gültige Testwarnung für ein angemeldetes Testgerät mit aktueller GPS-Position anlegen und den Empfang am Funkgerät prüfen. Alte Warnungen nicht als Test erneut versenden: Die dauerhafte Einmal-Zustellung bleibt wirksam. War ein früherer Versand beim Absturz bereits begonnen, aber noch nicht bestätigt, kann er nach dem Neustart als unklar beziehungsweise `dead_letter` erscheinen. Bei solchen Warnungen verhindert `at_most_once` eine automatische Wiederholung sowie `Retry` und `Requeue` im Router, damit das Gerät dieselbe Warnung nicht mehrfach erhält.

Eine Meldung „Von TBS angenommen“ bestätigt weiterhin die Annahme durch die Basisstation, nicht den Empfang am Funkgerät.
