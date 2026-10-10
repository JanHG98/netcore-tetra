# Historisches Installationsprotokoll – Phase 3: IoT-Open-Lab

**Einordnung: historisches Arbeitsprotokoll.** Geprüft gegen `main` (`c3ccdb4`, 09.10.2026). Die Funktionen sind im aktuellen `main` integriert. Folge der [aktuellen IoT-Dienstanleitung](../services/iot-gateway/README.md), [LXC-Installation](../services/iot-gateway/installation-im-lxc.md) und dem [Command-Vertrag](../contracts/kommandos-und-bestaetigungen-v1.md). Die Beispielkonfiguration aktiviert die virtuelle Sandbox; reale Home-Assistant-/CCU-Aktionen bleiben separat deaktiviert. Aktualisiere den vorhandenen Git-Checkout und verwende den dienstspezifischen Installer oder Updater, statt ein ZIP über den laufenden Quellbaum zu kopieren.

Die ursprünglichen Phasen, Branches, Adressen und Befehle bleiben im folgenden Original erhalten. Sie beschreiben den damaligen Stand; insbesondere ZIP-Austausch, Repository-Löschung und Feature-Branch-Wechsel sind keine aktuelle Update-Anweisung. Ein überprüfter Quellstand belegt keine Installation oder Funkabnahme auf deinem System.

<details>
<summary>Historische Originalanleitung anzeigen</summary>

# Phase 3 installieren – IoT Gateway mit MQTT

1. Das vollständige Repository auf einen eigenen IoT-Gateway-LXC nach `/opt/netcore-tetra` kopieren.
2. Als `root` installieren:

```bash
cd /opt/netcore-tetra/system-backend/iot-gateway
chmod 755 install/*.sh
./install/install.sh
```

3. Adressen der vier Ereignisquellen setzen:

```bash
./install/configure-openlab.sh \
  NODE_GATEWAY_IP \
  MOBILITY_CORE_IP \
  CALL_CONTROL_IP \
  SDS_ROUTER_IP
```

4. Status und Ports prüfen:

```bash
systemctl status netcore-iot-gateway --no-pager --full
systemctl status mosquitto --no-pager --full
ss -ltnp | grep -E ':8240|:1883'
source /etc/netcore/lxc-network.env
curl -fsS "${NETCORE_WEBUI_URL}api/v1/status" | python3 -m json.tool
```

5. MQTT beobachten und Testnachricht senden:

```bash
mosquitto_sub -h 127.0.0.1 -p 1883 -v -t 'netcore/v1/#'
```

In einem zweiten Terminal:

```bash
source /etc/netcore/lxc-network.env
curl -fsS -X POST \
  -H 'Content-Type: application/json' \
  -d '{"payload":{"message":"Hallo aus Phase 3"}}' \
  "${NETCORE_WEBUI_URL}api/v1/test/publish" \
  | python3 -m json.tool
```

OPEN LAB bedeutet ausdrücklich: keine Anmeldung, keine Tokens, kein TLS und anonymer MQTT-Zugriff. Eingehende Commands werden gespeichert, aber in Phase 3 niemals ausgeführt.

</details>
