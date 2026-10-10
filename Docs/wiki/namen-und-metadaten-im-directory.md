# Namen und Metadaten im NetCore Directory

NetCore Directory ist der zentrale Namens- und Metadatendienst für die Basisstation. Der Dienst läuft als Python-Anwendung mit SQLite-Datenbank und stellt Weboberfläche sowie HTTP-API bereit.

Im verteilten Ausbau ist er **nicht identisch** mit Subscriber Core oder Group Core. Lesbare Namen/Statusgruppen aus Directory ersetzen keine zentrale Teilnehmerfreigabe, GSSI-Policy oder aktuelle RF-Affiliation. [Provisioning und Datenhoheit](teilnehmer-und-gruppen-anlegen.md)

## Datenbereiche

- Geräte
- Basisstationen
- Gruppen
- Gerätegruppen bzw. Statusgruppen
- Statusmeldungen

## Installation

```bash
sudo useradd --system --home /opt/netcore-directory --shell /usr/sbin/nologin netcore-directory
sudo install -d -o netcore-directory -g netcore-directory /opt/netcore-directory
sudo install -d -o netcore-directory -g netcore-directory /var/lib/netcore-directory
sudo install -m 0755 system-backend/directory/netcore-directory.py \
  /opt/netcore-directory/netcore-directory.py
```

Die eingecheckte Unit verweist noch auf `netcore_directory-server.py`; die vorhandene Quelldatei heißt `netcore-directory.py`. Für den oben gezeigten Installationspfad muss die lokale Unit deshalb ausdrücklich so aussehen:

```ini
[Unit]
Description=NetCore Directory
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=netcore-directory
Group=netcore-directory
WorkingDirectory=/opt/netcore-directory
ExecStart=/usr/bin/python3 /opt/netcore-directory/netcore-directory.py --host 0.0.0.0 --port 8095 --db /var/lib/netcore-directory/netcore-directory.db
Restart=on-failure
RestartSec=3

[Install]
WantedBy=multi-user.target
```

Als `/etc/systemd/system/netcore-directory.service` speichern; anschließend `sudo systemctl daemon-reload` und `sudo systemctl enable --now netcore-directory.service`. Bei bestehender Installation Benutzer und Datenbankpfad erhalten. Der direkte HTTP-Handler besitzt keine eigene Anmeldung; nur im kontrollierten Managementnetz binden.

Beispiel für die Basisstationsanbindung:

```toml
[netcore_directory]
enabled = true
base_url = "http://<DIRECTORY-IP>:8095"
timeout_ms = 2000
```

## Laufzeitverhalten

Die Basisstation fragt Directory-Daten für Anzeige und Statuslogik ab. Der Funkbetrieb ist nicht vollständig vom Directory abhängig. Bei Ausfall bleiben numerische IDs und lokal bekannte Zustände nutzbar.

Statusgruppen werden regelmäßig neu geladen. Änderungen an Mitgliedern können dadurch ohne Neustart der Basisstation wirksam werden. Bei erneuter Registrierung eines Gruppenmitglieds kann der zuletzt bekannte Status erneut ausgesendet werden.

## Optionale Exporte

Die Directory-Konfiguration unterstützt neben der reinen Namensauflösung optionale Laufzeit-Exporte, zum Beispiel:

- Präsenz
- Status
- CDR/Rufereignisse
- Notfälle
- Health
- SDS-Aktivität
- Positionen

Diese Optionen sollten nur aktiviert werden, wenn der Directory-Server die jeweilige Verarbeitung unterstützt.

## Sicherung

Die SQLite-Datenbank muss regelmäßig gesichert werden. Vor Importen oder größeren Strukturänderungen:

```bash
sudo systemctl stop netcore-directory.service
sudo cp /var/lib/netcore-directory/netcore-directory.db \
  /var/lib/netcore-directory/netcore-directory.db.$(date +%Y%m%d-%H%M%S).bak
sudo systemctl start netcore-directory.service
```

Pfad und Dateiname können je nach lokaler Unit abweichen.

## Health-Prüfung

```bash
curl -fsS http://127.0.0.1:8095/api/health
sudo systemctl status netcore-directory.service --no-pager
sudo journalctl -u netcore-directory.service -n 200 --no-pager
```

## Quellen zur Pflege dieser Seite

[Python-Dienst und Datenbankschema](../../system-backend/directory/netcore-directory.py) · [Eingecheckte Unit mit abweichendem Dateinamen](../../system-backend/directory/netcore-directory.service).
