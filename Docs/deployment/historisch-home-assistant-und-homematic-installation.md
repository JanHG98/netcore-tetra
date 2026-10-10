# Historisches Installationsprotokoll – Phase 5: Home Assistant und Homematic

**Einordnung: historisches Arbeitsprotokoll.** Geprüft gegen `main` (`c3ccdb4`, 09.10.2026). Die Funktionen sind im aktuellen `main` integriert. Folge der [aktuellen IoT-Dienstanleitung](../services/iot-gateway/README.md), [LXC-Installation](../services/iot-gateway/installation-im-lxc.md) und dem [Command-Vertrag](../contracts/kommandos-und-bestaetigungen-v1.md). Die Beispielkonfiguration aktiviert die virtuelle Sandbox; reale Home-Assistant-/CCU-Aktionen bleiben separat deaktiviert. Aktualisiere den vorhandenen Git-Checkout und verwende den dienstspezifischen Installer oder Updater, statt ein ZIP über den laufenden Quellbaum zu kopieren.

Die ursprünglichen Phasen, Branches, Adressen und Befehle bleiben im folgenden Original erhalten. Sie beschreiben den damaligen Stand; insbesondere ZIP-Austausch, Repository-Löschung und Feature-Branch-Wechsel sind keine aktuelle Update-Anweisung. Ein überprüfter Quellstand belegt keine Installation oder Funkabnahme auf deinem System.

<details>
<summary>Historische Originalanleitung anzeigen</summary>

# Installation – MQTT Phase 5

## 1. Repository austauschen

```bash
cd /opt
mv netcore-tetra "netcore-tetra-backup-$(date +%Y%m%d-%H%M%S)"
unzip netcore-tetra-mqtt-phase5-homeassistant-homematic-openlab.zip
mv netcore-tetra-mqtt netcore-tetra
```

Die Konfiguration unter `/etc/netcore/` und die Laufzeitdaten unter `/var/lib/netcore-iot-gateway/` bleiben erhalten.

## 2. IoT Gateway aktualisieren

```bash
cd /opt/netcore-tetra/system-backend/iot-gateway
find install -type f -name '*.sh' -exec chmod 755 {} +
./install/update.sh
```

## 3. Status prüfen

```bash
systemctl status netcore-iot-gateway --no-pager --full
journalctl -u netcore-iot-gateway -n 100 --no-pager
ss -ltnp | grep -E ':8240|:1883'
```

## 4. Home Assistant verbinden

Home Assistant verwendet den bestehenden MQTT-Broker des IoT-Gateway-LXC:

```text
Broker: IP-DES-IOT-GATEWAY
Port: 1883
Benutzer: leer
Passwort: leer
TLS: aus
```

Nur im isolierten OPEN-LAB-Netz verwenden.

Discovery erneut auslösen:

```bash
curl -fsS -X POST http://IP-DES-IOT-GATEWAY:8240/api/v1/actions/home-assistant-discovery \
  | python3 -m json.tool
```

MQTT beobachten:

```bash
mosquitto_sub -h 127.0.0.1 -p 1883 -v -t 'homeassistant/#'
```

## 5. Homematic IP Access Point

Der Access Point bleibt in Home Assistant. Die Datei

```text
system-backend/iot-gateway/examples/home-assistant/state-bridge.yaml
```

in Home Assistant übernehmen, die drei Beispiel-Entity-IDs durch eigene Homematic-Entitäten ersetzen und Automationen neu laden.

Importierte Zustände prüfen:

```bash
curl -fsS http://IP-DES-IOT-GATEWAY:8240/api/v1/home-assistant/entities \
  | python3 -m json.tool
```

## 6. CCU3 oder RaspberryMatic – optional

`/etc/netcore/iot-gateway.toml` bearbeiten:

```toml
[homematic]
enabled = true
mode = "ccu_xml_rpc"
ccu_host = "IP-DER-CCU"
ccu_port = 2010
poll_interval_ms = 2000
request_timeout_ms = 2500
allow_writes = false
```

Datenpunkte aus

```text
system-backend/iot-gateway/examples/homematic/datapoints.example.toml
```

übernehmen und Adressen anpassen. Danach:

```bash
systemctl restart netcore-iot-gateway
curl -fsS -X POST http://IP-DES-IOT-GATEWAY:8240/api/v1/actions/homematic-poll-now \
  | python3 -m json.tool
curl -fsS http://IP-DES-IOT-GATEWAY:8240/api/v1/homematic/datapoints \
  | python3 -m json.tool
```

## 7. OPEN-LAB-Testgeräte

In Home Assistant sollten über MQTT Discovery erscheinen:

```text
NetCore Relais lab-relay-01
NetCore Licht lab-light-01
NetCore Taster lab-button-01
IoT Gateway Verbindung
Quelle node-gateway / mobility-core / call-control / sds-router
```

Die Schaltbefehle laufen durch `netcore-command-v1`, Default Deny, persistentes Ledger und `netcore-command-ack-v1`.

## 8. Reale Schreibwege

Standardzustand:

```toml
home_assistant.allow_command_egress = false
homematic.allow_writes = false
```

Diese Sperren nicht für den ersten Test öffnen. Direkte CCU-Schreibzugriffe benötigen zusätzlich einen `writable = true` Datenpunkt und eine aktivierte Allow-Policy.

</details>
