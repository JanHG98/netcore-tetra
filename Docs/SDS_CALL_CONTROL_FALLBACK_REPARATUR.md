# SDS Router und Call Control: wechselnden Fallback reparieren

Die TBS zeigt wiederholt **ausgefallen → Fallback → recovering** für SDS Router und Call Control. Bei der untersuchten Installation antwortete `/health/live` in wenigen Millisekunden, während `/health/ready` mehrere Sekunden benötigte oder nach sechs Sekunden noch nicht antwortete. Das Node Gateway wartete nur 1,5 Sekunden und meldete `read failed: Resource temporarily unavailable (os error 11)`.

Beide Dienste schrieben bei Gateway-Telemetrie ihre vollständige Datenbank, auch ohne Änderung an gespeicherten Nachrichten oder Rufen. Währenddessen war der gemeinsame Zustand für die Bereitschaftsprüfung gesperrt. Die Korrektur vermeidet diese unveränderten Schreibvorgänge. Echte Nachrichten-, Zustell-, Ruf- und Wiederherstellungsänderungen werden weiterhin gespeichert. Beim SDS Router bleibt die dauerhafte Speicherung vor dem Versand erhalten.

## Was auf welchem System zu tun ist

| System | Aktion |
|---|---|
| **SDS-Router-LXC** | Schritt 1 ausführen: Software aktualisieren und Dienst neu starten |
| **Call-Control-LXC** | Schritt 2 ausführen: Software aktualisieren und Dienst neu starten |
| **Warn-LXC oder ein anderer Rechner im selben Netz** | Schritt 3: Bereitschaft beider Dienste prüfen |
| **Node-Gateway-LXC, Control-Room-LXC, Warn-LXC** | Für diese Reparatur kein Softwareupdate oder Neustart nötig |
| **Jede TBS** | Kein Softwareupdate oder Neustart nötig; abschließend Dienstzustände und Funkbetrieb prüfen |
| **Neuer LXC** | Keiner erforderlich |

Die Befehle verwenden den Branch **`feat/katwarn-nina-alerts`** aus [PR #49](https://github.com/JanHG98/netcore-tetra/pull/49). Jeder Updateblock legt eine eigene Arbeitskopie unter `/opt` an und bricht bei einem Fehler ab. Vorhandene Projektordner bleiben erhalten.

Der Neustart unterbricht den jeweiligen Dienst kurz. Das Call-Control-Update bei ruhendem Rufbetrieb durchführen. Bestehende Konfigurationen und Datenbanken behalten; insbesondere die SDS-Datenbank enthält die dauerhaften Duplikatsperren.

## 1. SDS-Router-LXC aktualisieren

**Diesen gesamten Block in der Konsole des bestehenden SDS-Router-LXC als root ausführen:**

```bash
(
  set -e
  test -s /etc/netcore/sds-router.toml
  cd /opt
  update_dir=$(mktemp -d /opt/netcore-sds-fix.XXXXXX)
  git clone --single-branch --branch feat/katwarn-nina-alerts https://github.com/JanHG98/netcore-tetra.git "$update_dir"
  cd "$update_dir"
  test -s system-backend/sds-router/install/update.sh
  . system-backend/shared/install/lxc-network.sh
  netcore_detect_lxc_ipv4 >/dev/null
  if [ -f /root/.cargo/env ]; then . /root/.cargo/env; fi
  cargo build --release -p netcore-sds-router
  bash system-backend/sds-router/install/update.sh
  systemctl is-active netcore-sds-router
)
```

**Erwartet:** Der Build ist erfolgreich und abschließend erscheint `active`. Der erste Build läuft bei weiterhin gestartetem Dienst. Der Updater ruft den Build anschließend nochmals mit den bereits gebauten Dateien auf und wechselt den Dienst.

Bei Fehlern **auf dem SDS-Router-LXC**:

```bash
systemctl status netcore-sds-router --no-pager -l
journalctl -u netcore-sds-router -n 60 --no-pager
```

## 2. Call-Control-LXC aktualisieren

**Diesen gesamten Block in der Konsole des bestehenden Call-Control-LXC als root ausführen:**

```bash
(
  set -e
  test -s /etc/netcore/call-control.toml
  cd /opt
  update_dir=$(mktemp -d /opt/netcore-call-fix.XXXXXX)
  git clone --single-branch --branch feat/katwarn-nina-alerts https://github.com/JanHG98/netcore-tetra.git "$update_dir"
  cd "$update_dir"
  test -s system-backend/call-control/install/update.sh
  if [ -f /root/.cargo/env ]; then . /root/.cargo/env; fi
  bash system-backend/call-control/install/update.sh
  systemctl is-active netcore-call-control
)
```

**Erwartet:** Der Build ist erfolgreich und abschließend erscheint `active`. Dieser Updater baut vor dem Neustart und sichert die bisherige ausführbare Datei als `/usr/local/bin/netcore-call-control.previous`.

Bei Fehlern **auf dem Call-Control-LXC**:

```bash
systemctl status netcore-call-control --no-pager -l
journalctl -u netcore-call-control -n 60 --no-pager
```

## 3. Nach beiden Updates die Antwortzeiten prüfen

**Auf dem Warn-LXC oder einem anderen Rechner mit Zugriff auf beide Dienste:** In den ersten beiden Zeilen die tatsächlichen IP-Adressen einsetzen, dann den ganzen Block ausführen.

```bash
sds_ip='SDS-ROUTER-IP'
call_ip='CALL-CONTROL-IP'
for i in 1 2 3 4 5; do
  curl --noproxy '*' --max-time 2 --fail --silent --show-error \
    --output /dev/null --write-out 'SDS Router: HTTP %{http_code}, %{time_total}s\n' \
    "http://${sds_ip}:8150/health/ready"
  curl --noproxy '*' --max-time 2 --fail --silent --show-error \
    --output /dev/null --write-out 'Call Control: HTTP %{http_code}, %{time_total}s\n' \
    "http://${call_ip}:8120/health/ready"
  sleep 2
done
```

**Erwartet:** Jeweils HTTP `200`, zuverlässig unter dem eingestellten Timeout des Node Gateways (bei der untersuchten Installation 1,5 Sekunden). `active` allein reicht nicht: Es bestätigt nur, dass der Prozess läuft.

Danach die Basisstation beobachten: Beide Dienste müssen nach erfolgreichen Überwachungsprüfungen und der Erholungszeit wieder dauerhaft verfügbar sein. Andere ausgefallene Kerndienste können weiterhin ihren eigenen Fallback auslösen. Einen Testanruf und eine Test-SDS durchführen. Danach eine noch gültige Testwarnung für ein angemeldetes Gerät mit aktueller GPS-Position prüfen.

**Bleiben die Antwortzeiten hoch oder kommt HTTP `503`:** Die Ausgabe aus Schritt 3 und die beiden Dienstprotokolle aus Schritt 1/2 zur weiteren Diagnose verwenden. Die Bereitschaftsprüfung weiterhin auf `/health/ready` belassen, damit eine verlorene Gateway-Verbindung erkannt wird.
