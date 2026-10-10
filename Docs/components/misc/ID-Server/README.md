# NetCore Directory Server `0.2.0`

**Quellen:** [misc/ID-Server](../../../../misc/ID-Server) · [Repository-Root](../../../..). Bei Befehlen das in der Anleitung angegebene Arbeitsverzeichnis beachten.

Lokaler **RadioID-ähnlicher Server** für **NetCore-Tetra / FlowStation**.

---

## Lokaler Start

Die folgenden Befehle im Quellverzeichnis `misc/ID-Server/` ausführen. `APP_VERSION` im Python-Server ist `0.2.0`; SQLite und die eingebettete WebUI sind implementiert. Der Dienst besitzt keine eigene Anmeldung; die Bind-Adresse und Netzfreigabe bestimmen seine Erreichbarkeit.

```bash
python3 netcore_directory_server.py \
  --host 0.0.0.0 \
  --port 8095 \
  --db ./netcore_directory.db \
  --seed seed.json
```

---

## Web UI

```text
http://<SERVER-IP>:8095/
```

Beispiel lokal:

```text
http://127.0.0.1:8095/
```

---

## RadioID-kompatible Tests

### DMR User / Device abfragen

```bash
curl -s 'http://127.0.0.1:8095/api/dmr/user/?id=2020001' | jq .
```

### DMR Repeater / Basisstation abfragen

```bash
curl -s 'http://127.0.0.1:8095/api/dmr/repeater/?id=4010001' | jq .
```

---

## Native APIs

### Devices

```http
GET     /api/devices
POST    /api/devices
GET     /api/devices/<id>
PUT     /api/devices/<id>
DELETE  /api/devices/<id>
```

### Basisstationen

```http
GET     /api/basestations
POST    /api/basestations
GET     /api/basestations/<id>
PUT     /api/basestations/<id>
DELETE  /api/basestations/<id>
```

### Gruppen

```http
GET     /api/groups
POST    /api/groups
GET     /api/groups/<id>
PUT     /api/groups/<id>
DELETE  /api/groups/<id>
```

### Statusmeldungen

```http
GET     /api/status
POST    /api/status
GET     /api/status/<id>
PUT     /api/status/<id>
DELETE  /api/status/<id>
```

---

## Gerätegruppen und Statuszuordnung

Zusätzlich zu den oben aufgeführten CRUD-Ressourcen implementiert der Server `/api/device-groups` sowie `/api/status-group-members?issi=<ISSI>`. Die konkreten IDs und Payload-Felder aus dem Python-Handler beziehungsweise seiner WebUI übernehmen.

## Systemd-Installation

### Verzeichnis vorbereiten

```bash
sudo mkdir -p /opt/netcore-directory
```

### Dateien kopieren

```bash
sudo cp netcore_directory_server.py /opt/netcore-directory/
sudo cp seed.json /opt/netcore-directory/
```

### Einmalig manuell starten / Seed importieren

```bash
cd /opt/netcore-directory

sudo python3 netcore_directory_server.py \
  --db /opt/netcore-directory/netcore_directory.db \
  --seed seed.json
```

Mit `Strg+C` stoppen, danach den Dienst einrichten.

---

## Systemd-Service aktivieren

```bash
sudo cp netcore-directory.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now netcore-directory
```

---

## Dienst prüfen

```bash
systemctl status netcore-directory
```

Logs live ansehen:

```bash
journalctl -u netcore-directory -f
```
