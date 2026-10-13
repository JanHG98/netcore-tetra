# IoT-Anbindung: Installation im LXC

Für eine bestehende Installation aus einer sauberen, aktualisierten Arbeitskopie:

```bash
cd /opt/netcore-tetra
NC_REF=$(git rev-parse HEAD)
sudo python3 -B tools/operator/update-netcore-service.py \
  --service iot-gateway --commit "$NC_REF" --prepare --prepare-build-deps
sudo systemctl stop netcore-iot-gateway.service
sudo python3 -B tools/operator/update-netcore-service.py \
  --service iot-gateway --commit "$NC_REF" --apply
sudo systemctl reset-failed netcore-iot-gateway.service
sudo systemctl start netcore-iot-gateway.service
```

Für eine Neuinstallation aus derselben Arbeitskopie `sudo bash system-backend/iot-gateway/install/install.sh` verwenden. Das direkte Installerskript benötigt eine Rust-Toolchain für seinen Installationsbenutzer.

Der Dienst bleibt auf TCP 8240. Der Runtime-Updater erhält die installierte Unit, Konfiguration und Nutzdaten; ein vorhandener Mosquitto bleibt unverändert. Für Schemaergänzungen bietet der Installer `migrate-phase5-config.sh`, das nur fehlende Abschnitte, Storage-Schlüssel und Standard-Policies ergänzt.

Danach:

```bash
systemctl status netcore-iot-gateway --no-pager --full
journalctl -u netcore-iot-gateway -n 100 --no-pager
```

## Start bei voller MQTT-Outbox

**Betreiberbefund vom 14. Oktober 2026:** Auf `10.0.1.119` beendet sich der bisherige Gateway vor dem Start von MQTT-Worker und HTTP-Listener mit `outbox limit 20000 would be exceeded`. Er will beim Start zusätzliche virtuelle Gerätezustände einreihen; der bereits gefüllte persistente Ausgang verhindert dies. Die Wiederanlaufschleife kann den Rückstau dadurch selbst bei erreichbarem Broker nicht abbauen.

Die Korrektur stellt zunächst nur Gerätezustände mit einem freien Outbox-Platz ein und verschiebt den Rest. Der Gateway startet auch mit einer vollständig gefüllten Outbox oder einem Rückstau über dem aktuell konfigurierten Limit. Nach erfolgreich quittierten und lokal entfernten Nachrichten werden die ausstehenden Gerätezustände nachgereicht. Alte Nachrichten bleiben erhalten; das Limit für zusätzliche Nachrichten bleibt wirksam. Echte Fehler beim Lesen oder Schreiben des persistenten Zustands verhindern weiterhin den Start.

Nach der Übernahme dieser Korrektur den konfigurationserhaltenden Runtime-Updater oben verwenden. Ältere, auf einen festen Quellcommit gepinnte Updateblöcke enthalten diese Korrektur noch nicht. Für den beobachteten Bind-Wert ausdrücklich die eigene Hostadresse prüfen:

```bash
curl --noproxy '*' -i --max-time 5 http://10.0.1.119:8240/health/live
curl --noproxy '*' -sS --max-time 5 http://10.0.1.119:8240/api/v1/status
systemctl status mosquitto --no-pager --full
ss -lntp | grep ':1883'
```

`/health/live` muss antworten. Ein `updated_degraded` im Updatebericht bedeutet, dass die installierte Laufzeit geprüft wurde, ihre fachliche Readiness aber noch fehlt. `mqtt_connected=false` oder ein nicht fallender `outbox_pending` erfordern anschließend die Broker-/Verbindungsdiagnose. Der gemeldete Prozessabbruch belegt die volle Outbox, nicht die ursprüngliche Ursache des MQTT-Rückstaus. Bei genau dieser Bind-Adresse ist ein Test gegen `127.0.0.1:8240` kein Liveness-Nachweis.

Die Rust-Regressionen prüfen wiederholten Start mit voller, überfüllter und teilweise belegter Outbox, Erhalt der gespeicherten Nachrichten, Nachlieferung nach Quittierung sowie echte Speicherfehler. `python3 tools/test_iot_gateway_outbox.py` prüft das gebaute Programm mit HTTP und einem lokalen MQTT-Testbroker; die Linux-CI baut das Binary dafür selbst. Eine erfolgreiche lokale Prüfung ersetzt die tatsächliche Übernahme und Brokerabnahme auf dem Diensthost nicht.

## Git-Übertragung klein halten

Ein am 14. Oktober 2026 gemeldeter `RPC failed; curl 56 ... Connection reset by peer` beendet den vollständigen Fetch vor der Dienständerung. Der Quellbaum am geprüften Fixcommit enthält rund 326 MB unter `Docs/`; für diesen Runtime-Build sind diese Dateien nicht erforderlich. Ein Partial Clone mit Sparse Checkout lädt die Git-Metadaten sowie rund 20 MB Quellcode einschließlich aller benötigten Rust-Workspace-Pakete. HTTP/1.1 gilt nur für diese neue Arbeitskopie, auch bei nachgeladener Git-Dateiübertragung.

Der folgende vollständige Wiederanlaufblock verwendet den in PR #83 übernommenen und in allen drei Workflows geprüften Commit. Er übergibt den obligatorischen `--commit`, bereitet den Build und unveränderliche Snapshots vor dem Stoppen des Dienstes vor und übernimmt danach genau diesen Plan. Die Konfiguration, MQTT-Nachrichten und vorhandene Brokerinstallation bleiben erhalten.

```bash
(
  set -Eeuo pipefail
  NC_REF=b223ecc0f626e5dd8222d3edc0e969c774b8da21
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/iot-fix-small.XXXXXX)

  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" remote add origin https://github.com/JanHG98/netcore-tetra.git
  git -C "$NC_SRC" config http.version HTTP/1.1
  git -C "$NC_SRC" config remote.origin.promisor true
  git -C "$NC_SRC" config remote.origin.partialclonefilter blob:none
  git -C "$NC_SRC" fetch --depth 1 --filter=blob:none origin "$NC_REF"
  git -C "$NC_SRC" sparse-checkout init --cone
  git -C "$NC_SRC" sparse-checkout set system-backend crates bins tools/operator
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  [[ "$(git -C "$NC_SRC" rev-parse HEAD)" == "$NC_REF" ]]
  chmod 0755 "$NC_SRC"

  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service iot-gateway --commit "$NC_REF" --prepare --prepare-build-deps
  systemctl stop netcore-iot-gateway.service
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service iot-gateway --commit "$NC_REF" --apply
  systemctl reset-failed netcore-iot-gateway.service
  systemctl start netcore-iot-gateway.service

  curl --noproxy '*' --retry-connrefused --retry 30 --retry-delay 1 --max-time 5 -fsS http://10.0.1.119:8240/health/live
  curl --noproxy '*' --max-time 5 -fsS http://10.0.1.119:8240/api/v1/status
)
```

Der Sparse-Checkout-Ablauf wurde lokal gegen einen gefilterten Git-Transport geprüft; Cargo löst im entsprechenden reduzierten Quellbaum alle 30 Workspace-Pakete auf, einschließlich `netcore-iot-gateway`. Bash-Syntax und Updater-CLI wurden geprüft. Das bestätigt die Vorbereitung des Rückwegs, nicht dessen tatsächliche Ausführung auf dem Diensthost.

## Bestätigter Wiederanlauf auf dem Diensthost

Am 14. Oktober 2026 bestätigt Jan den reduzierten Checkout mit 128,80 KiB Git-Metadaten und 3,77 MiB Blob-Pack, den nativen Release-Build in 29,85 Sekunden und den verifizierten Austausch von `/usr/local/bin/netcore-iot-gateway` auf SHA-256 `32acbedd105aa1859db10f034af83fff6baf45c83a84512e99d6dbbb0bf3048d`. Nach dem ausdrücklichen Dienststart antwortet `/health/live` mit `live`; `/api/v1/status` meldet MQTT verbunden und keinen MQTT-Verbindungsfehler. Die zwei anfänglichen Verbindungsablehnungen liegen vor dem erfolgreichen HTTP-Antwortnachweis. [Strukturierter Betreiberbeleg](tests/evidence/wiederanlauf-2026-10-14.json).

Die erste Statusaufnahme zeigt noch 20.000 wartende Nachrichten, 0 Broker-Publish-Quittungen und 0 von 4 gesunden Ereignisquellen. Das ist kein Nachweis einer anhaltenden Zustellstörung; Quittierungs-/Outbox-Verlauf und Quellenfehler sind anschließend zu messen. `invalid_events=32` belegt ebenfalls keinen bestimmten Formatfehler: Der aktuelle Poller zählt auch Kapazitätsfehler beim Einreihen als ungültige Ereignisse. Vollständige MQTT-Zustellung, Quellen-Readiness, Home-Assistant-Discovery und die ursprüngliche Rückstauursache bleiben offen.

**Quellabgleich: 14. Oktober 2026.** [Quellcode](../../../system-backend/iot-gateway) · [Konfigurationsvorlagen](../../../system-backend/iot-gateway/config).
