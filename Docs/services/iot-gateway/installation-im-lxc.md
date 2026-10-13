# IoT-Anbindung: Installation im LXC

Für eine bestehende Installation aus einer sauberen, aktualisierten Arbeitskopie:

```bash
cd /opt/netcore-tetra
sudo python3 -B tools/operator/update-netcore-service.py --service iot-gateway --apply
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

**Quellabgleich: 14. Oktober 2026.** [Quellcode](../../../system-backend/iot-gateway) · [Konfigurationsvorlagen](../../../system-backend/iot-gateway/config).
