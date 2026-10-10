# Funkverzeichnis

Stand: **9. Oktober 2026**. Der Directory Server (`APP_VERSION = "0.2.0"`) verwaltet Geräte-, Basisstations-, Gruppen-, Geräteverbund- und Statusmetadaten. Er bietet eine eingebettete WebUI, RadioID-kompatible Abfragen und SQLite-Persistenz auf Port **8095**. Teilnehmerfreigaben und Dienstberechtigungen bleiben im Subscriber Core; das Verzeichnis ersetzt dessen autoritative Profile nicht.

Quellen: [netcore-directory.py](../../../system-backend/directory/netcore-directory.py), [Seed](../../../system-backend/directory/seed.json) und [systemd-Vorlage](../../../system-backend/directory/netcore-directory.service).

## Lokal starten

Vom Repository-Hauptverzeichnis:

```bash
cd system-backend/directory
python3 netcore-directory.py --host 127.0.0.1 --port 8095 --db ./netcore_directory.db --seed seed.json
```

WebUI: `http://127.0.0.1:8095/`. Für den Zugriff aus dem isolierten Managementnetz `--host 0.0.0.0` verwenden. Der Dienst implementiert keine Anmeldung und kein TLS. Der Seed ist ein optionaler Import; Beispielkennungen vor der Verwendung anpassen.

## Schnittstellen

| Zweck | Pfad |
|---|---|
| RadioID-kompatible Teilnehmerabfrage | `/api/dmr/user/?id=2020001` |
| RadioID-kompatible Stationsabfrage | `/api/dmr/repeater/?id=4010001` |
| Geräte und einzelne Geräte | `/api/devices`, `/api/devices/<ISSI>` |
| Basisstationen | `/api/basestations`, `/api/basestations/<ISSI>` |
| Gruppen | `/api/groups`, `/api/groups/<GSSI>` |
| Geräteverbünde | `/api/device-groups` |
| Zugehörige Statusgruppenmitglieder | `/api/status-group-members?issi=<ISSI>` |
| Statusmetadaten | `/api/status`, `/api/status/<ID>` |
| Vollständiger Export und Import | `GET /api/export`, `POST /api/import` |

Die nativen Objektpfade unterstützen je nach Ressource Auflisten, Anlegen, Bearbeiten und Löschen. Die RadioID-Pfade dienen nur zum Lesen.

```bash
curl -fsS 'http://127.0.0.1:8095/api/dmr/user/?id=2020001' | python3 -m json.tool
curl -fsS 'http://127.0.0.1:8095/api/devices' | python3 -m json.tool
```

Ein leeres Ergebnis kann bei nicht importierter Beispielkennung korrekt sein. Dieser ältere Dienst bietet keine `/health/live`-, `/health/ready`- oder `/metrics`-Verträge der neueren Backend-Dienste.

## Mit systemd installieren

Aus dem Repository-Hauptverzeichnis:

```bash
sudo install -d -m 0755 /opt/netcore-directory
sudo install -m 0755 system-backend/directory/netcore-directory.py /opt/netcore-directory/netcore_directory-server.py
sudo install -m 0644 system-backend/directory/seed.json /opt/netcore-directory/seed.json
sudo install -m 0644 system-backend/directory/netcore-directory.service /etc/systemd/system/netcore-directory.service
```

Der unterschiedliche Zieldateiname ist absichtlich gewählt: die ausgelieferte Unit erwartet `netcore_directory-server.py`, während die Quellcode-Datei `netcore-directory.py` heißt. Die Unit verwendet als Datenbank `/opt/netcore-directory/netcore-directory.db`.

Optional den Seed einmalig in genau diese Datenbank importieren:

```bash
cd /opt/netcore-directory
sudo python3 netcore_directory-server.py --host 127.0.0.1 --port 8095 --db /opt/netcore-directory/netcore-directory.db --seed seed.json
```

Mit `Strg+C` beenden und danach systemd starten:

```bash
sudo systemctl daemon-reload
sudo systemctl enable --now netcore-directory
systemctl status netcore-directory --no-pager
journalctl -u netcore-directory -n 50 --no-pager
```

## Daten und Betriebsgrenzen

Vor Updates Datenbank und vorhandene Exporte sichern. Die systemd-Vorlage besitzt keinen `User=`-Eintrag und läuft deshalb standardmäßig als root; ein angepasster Dienstnutzer benötigt Schreibrechte auf Datenbank und Verzeichnis. Ein Wechsel auf HTTPS, Rollen und geschützte Verwaltungszugriffe ist eine eigene Umsetzung; das gemeinsame WebUI-Design führt diese Funktionen nicht automatisch ein.
