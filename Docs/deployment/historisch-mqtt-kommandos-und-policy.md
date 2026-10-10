# Historisches Installationsprotokoll – Phase 4: Command/Ack-Policy

**Einordnung: historisches Arbeitsprotokoll.** Geprüft gegen `main` (`c3ccdb4`, 09.10.2026). Die Funktionen sind im aktuellen `main` integriert. Folge der [aktuellen IoT-Dienstanleitung](../services/iot-gateway/README.md), [LXC-Installation](../services/iot-gateway/installation-im-lxc.md) und dem [Command-Vertrag](../contracts/kommandos-und-bestaetigungen-v1.md). Die Beispielkonfiguration aktiviert die virtuelle Sandbox; reale Home-Assistant-/CCU-Aktionen bleiben separat deaktiviert. Aktualisiere den vorhandenen Git-Checkout und verwende den dienstspezifischen Installer oder Updater, statt ein ZIP über den laufenden Quellbaum zu kopieren.

Die ursprünglichen Phasen, Branches, Adressen und Befehle bleiben im folgenden Original erhalten. Sie beschreiben den damaligen Stand; insbesondere ZIP-Austausch, Repository-Löschung und Feature-Branch-Wechsel sind keine aktuelle Update-Anweisung. Ein überprüfter Quellstand belegt keine Installation oder Funkabnahme auf deinem System.

<details>
<summary>Historische Originalanleitung anzeigen</summary>

# Installation – MQTT Phase 4

## 1. Komplettes Repository ersetzen

Das ZIP enthält das vollständige Repository ohne PDFs und ohne GitHub-Workflows. Konfiguration und Laufzeitdaten unter `/etc/netcore` beziehungsweise `/var/lib/netcore-iot-gateway` bleiben außerhalb des Repositorys erhalten.

## 2. IoT-Gateway aktualisieren

```bash
cd /opt/netcore-tetra/system-backend/iot-gateway
find install -type f -name '*.sh' -exec chmod 755 {} +
./install/update.sh
```

## 3. Status prüfen

```bash
systemctl status netcore-iot-gateway --no-pager --full
journalctl -u netcore-iot-gateway -n 100 --no-pager
```

## 4. Phase-4-Konfiguration prüfen

```bash
grep -nE '^\[commands\]|^\[\[command_policies\]\]|^(enabled|mode|default_deny|allow_retained|id|effect|command_types|target_types|target_prefixes)[[:space:]]*=' \
  /etc/netcore/iot-gateway.toml
```

Erwartet:

```text
[commands]
enabled = true
mode = "open_lab_sandbox"
default_deny = true
allow_retained = false
```

## 5. API prüfen

```bash
source /etc/netcore/lxc-network.env
curl -fsS "${NETCORE_WEBUI_URL}api/v1/status" | python3 -m json.tool
curl -fsS "${NETCORE_WEBUI_URL}api/v1/policies" | python3 -m json.tool
curl -fsS "${NETCORE_WEBUI_URL}api/v1/virtual-devices" | python3 -m json.tool
```

## 6. Funktionstest

Die WebUI enthält Schaltflächen für Lab-Relais, Lab-Licht und Dry-Run. Sie ruft den gleichen Command-/Policy-/Ack-Pfad auf wie MQTT, nur über den lokalen HTTP-Testendpunkt.

Für den vollständigen MQTT-Test die Anleitung in `Docs/changes/integrations/mqtt-phase-4-befehle-quittierungen-und-richtlinien.md` verwenden.

</details>
