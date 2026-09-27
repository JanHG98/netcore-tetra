# NetCore OpenLab Deployment & Auto Discovery

Feature-Branch: `feature/openlab-discovery-deployment` · Komponente `0.1.0`.
Ausgangspunkt: `main` / `20603a0` (v1.9.0 mit gemergtem PR #56).

Ein eigenständiger Deployment-LXC mit WebUI und ein Discovery-/Deployment-Agent
pro bestehendem LXC bzw. Pi. Python **3.11 oder neuer**, nur Standardbibliothek.
Die Oberfläche, API und Discovery verwenden OpenLab: **kein Login, keine Tokens,
kein TLS, keine Zertifikate, kein RBAC**. Erreichbare Teilnehmer im Lab können
Installationen und Neustarts auslösen. GitHub und Paketquellen werden weiterhin
über ihre normalen HTTPS-Adressen abgerufen.

## Enthalten

- Responsives WebUI: Hosts, Dienste, Healthchecks, installierte Commits,
  Versionsabweichungen, gezielte Installation/Updates/Neustarts und Auftragslogs.
- Multicast-Erkennung im gemeinsamen Netz, manuelle Suche, zusätzliche
  Unicast-Gegenstellen für VPN/geroutete Netze. Es ist kein eigenes VLAN nötig.
- Letzter bekannter guter Endpoint-Cache auf **jedem Host**. Der Controller ist
  keine Abhängigkeit von TBS, Node Gateway oder einem anderen Funkdienst.
- Rollenbasierte Auflösung der bestehenden Backend-Verbindungen. Node Gateway
  und TBS-Dashboard werden trotz gemeinsamem Port 8080 nicht verwechselt.
- TBS-Assistent: Name, MCC, MNC, ISSI, LA und CC. RF-/SDR-Einstellungen aus einer
  ausdrücklich importierten Standort-Konfiguration; Bootstrap für vorhandenes
  Debian/Raspberry Pi OS 64 Bit mit installiertem SDR-Treiber.
- Git-Branch-/Tag-/Commit-Auswahl, stündlicher Standabgleich (konfigurierbar),
  Anzeige der Tags. Ein Rollout wird vor Versand auf **einen vollständigen SHA**
  aufgelöst. Keine Quellcode-ZIPs, kein blindes `git pull` im laufenden Funkprozess.
- Persistente SQLite-Aufträge, begrenzte Logs, serialisierte Änderungen pro Host,
  Zeitlimits, Konfigurationssicherungen und Healthcheck nach dem Dienststart.
- „Auto Discovery“-Link in den bestehenden Management-WebUIs. Er öffnet den
  Agenten desselben Hosts und startet dort den Suchlauf.

## Deployment-LXC erstellen

Auf dem **Proxmox-Host** das Repo klonen und diesen Branch wählen:

```bash
git clone https://github.com/JanHG98/netcore-tetra.git
cd netcore-tetra
git switch feature/openlab-discovery-deployment
pveam available --section system
```

Ein Debian-12+-Template mit `pveam download <storage> <template>` herunterladen.
Dann freie CTID und den tatsächlichen Template-Namen einsetzen:

```bash
CTID=250 \
TEMPLATE=local:vztmpl/DEIN-DEBIAN-TEMPLATE.tar.zst \
STORAGE=local-lvm \
BRIDGE=vmbr0 \
bash system-backend/deployment-core/install/create-lxc.sh
```

Das Skript erstellt einen **unprivilegierten** LXC mit 2 Kernen, 2 GiB RAM,
512 MiB Swap, 16 GiB Disk, DHCP und Autostart. Bestehende CT-/VM-IDs werden
abgewiesen. Es gibt keine automatische Löschung bei einem Installationsfehler.
Optional: `IP=10.0.1.50/24 GATEWAY=10.0.1.1`, `VLAN_TAG=...`, `REF=...`.
Die Hardware-/Paket-Builds laufen auf den Zielhosts, nicht im Controller-LXC.

Danach: **`http://<LXC-IP>:8320`**.

In einem bereits vorhandenen Debian-LXC oder einer VM genügt:

```bash
bash system-backend/deployment-core/install/install.sh controller
```

## Vorhandene LXCs/Pis aufnehmen

Auf jedem Zielhost denselben Branch klonen/auschecken, dann:

```bash
sudo bash system-backend/deployment-core/install/install.sh agent
```

Im gemeinsamen Layer-2-Netz finden sich die Agenten selbst. Für ein VPN oder
blockiertes Multicast zusätzlich den Controller als feste Gegenstelle setzen:

```bash
sudo bash system-backend/deployment-core/install/install.sh agent \
  --seed http://10.0.1.50:8320
```

Alternativ bietet der Controller nach der ersten erfolgreichen Git-Prüfung
einen **Bootstrap-Download**. Das heruntergeladene Skript auf dem Zielhost als
root ausführen; es klont das Repo, checkt den geprüften Commit aus und installiert
den Agenten. Kein SSH-Zugang vom Controller aus erforderlich.

Der gemeinsame LXC-Netzwerkhelfer nimmt künftig auch regulär installierte/
aktualisierte Backend-LXCs auf. Die Originalinstallation bleibt bei einem Fehler
des Discovery-Installers nutzbar. `NETCORE_DISCOVERY_SKIP_INSTALL=1` überspringt
diesen Zusatz; `NETCORE_DEPLOYMENT_URL=http://...:8320` setzt eine Gegenstelle.

Vorhandene TBS werden anhand von `service_name` und ihrer systemd-Unit erkannt
(bekannte Konfigurationspfade `/etc/netcore/config.toml`, `/opt/tetra/config.toml`,
`/etc/flowstation/config.toml`). Bei abweichender Installation:

```bash
sudo bash system-backend/deployment-core/install/install.sh agent \
  --tbs-unit tetra.service \
  --tbs-config /opt/tetra/config.toml \
  --tbs-command '/opt/tetra/bluestation-bs /opt/tetra/config.toml'
```

`bluestation-bs` erwartet die Konfiguration **als Positionsargument**.
Die Backend-Binaries verwenden dagegen `--config`.

## Discovery und Übernahme von Verbindungen

Alle 15 Sekunden kündigt jeder Host seine Agent-Adresse an. Ein Empfänger prüft
Protokoll, Umgebung und Quellnetz und ruft das Manifest per HTTP von der tatsächlich
beobachteten Quelladresse ab. Das Manifest enthält Rollen, Ports, Bereitschaft
und bekannte Deployment-Commits, **keine Konfigurationen oder Secrets**.

Ein Seed liefert außerdem die ihm bekannten Agent-Adressen. Damit sind mehrere
Netze über ein bereits eingerichtetes, geroutetes VPN erreichbar. Das ist keine
VPN-Einrichtung und kein Multicast-Tunnel. Die Adressen müssen über die vorhandenen
Routen erreichbar sein. Nach 90 Sekunden ohne Antwort erscheint ein Host offline.

Mehrere erreichbare Instanzen einer Rolle ergeben einen sichtbaren Konflikt. Eine
neue Zuordnung wird dann nicht geraten. Unter Einstellungen zum Beispiel
`node-gateway=netcore-gateway-01` festlegen. Ein vorhandener guter Cache bleibt bei
Konflikten und Ausfällen erhalten. Ein stale Cache ist **kein** neuer Healthcheck.

Beim Dienststart erzeugt `launch.py` eine Konfiguration unter
`/var/lib/netcore-discovery-<dienst>/config.toml`. Der ursprüngliche TOML-Pfad und
ein vorhandener bekannter guter TBS-Fallback bleiben erhalten. Änderungen aus
einer Dienst-WebUI an der erzeugten Konfiguration überleben Neustarts; bei einem
Konflikt hat eine ausdrückliche Änderung der ursprünglichen TOML Vorrang. Die
automatisch aufgelösten Verbindungsfelder werden anschließend erneut eingesetzt.
Die Runtime-Datei liegt im privaten systemd-StateDirectory des jeweiligen Dienstes.

**Ein Suchlauf startet keine laufenden Dienste neu.** Neue Verbindungen werden
beim nächsten Start aktiv. Die Aktion „Verbindungen übernehmen / Neustart“ macht
das gezielt aus der UI. Bei der Installation aus dem Controller sind die bereits
gefundenen Abhängigkeiten vor dem ersten Dienststart verfügbar. Nachträglich
hinzugekommene/umgezogene Ziele benötigen bei bereits laufenden Diensten diese
gezielte Übernahme. Es gibt keine automatische Gesprächsunterbrechung.

## TBS-Assistent

1. Im Controller eine **geprüfte Standort-TOML** importieren. Sie bleibt lokal
   beim Controller und wird nicht im Discovery-Manifest veröffentlicht.
2. Name, MCC, MNC, ISSI, LA, CC eintragen. Die ISSI setzt die Brew-Identität und,
   sofern vorhanden, die lokale Audio-Quell-ISSI. Sie ersetzt keine Teilnehmer-ISSIs.
3. Den profilspezifischen Bootstrap herunterladen und auf dem Pi ausführen.
   Das Profiltemplate wird auf dem Pi abgelegt und der Agent meldet sich an.
4. Zielhost, Dienst `tbs`, Aktion `Installieren` und Profil auswählen; Plan prüfen
   und ausführen. Git-Stand, Build, Installer und Healthcheck erscheinen im Log.

Neue OpenLab-TBS übernehmen keine Dashboard-Benutzernamen/Passwörter aus der
Vorlage. Vorhandene TBS-Konfigurationen werden dadurch nicht nachträglich geändert.
Die Hardware, passende SoapySDR-/SXceiver-Treiber und Netzwerkverbindung müssen
vorhanden sein. Der Installer prüft auf ein SoapySDR-Gerät. Ein vollständiges
bootfähiges Pi-Imager-OS-Image, hardwareübergreifende Treiberinstallation und
VPN-Auto-Toggle sind **noch nicht Bestandteil dieses Branches**.

## Betrieb und Fehler

| Zweck | Protokoll / Port |
|---|---|
| Deployment-WebUI/API | HTTP TCP 8320 |
| Agent-WebUI/API pro Host | HTTP TCP 8321 |
| Lokale Discovery | UDP 48320, Multicast `239.192.84.82`, TTL 1 |
| Bestehende Dienste | Ihre bisherigen Managementports |

Konfiguration: `/etc/netcore/deployment.toml` bzw. `/etc/netcore/discovery.toml`.
Standardumgebung: `netcore-openlab`. Zulässige Netze: RFC1918 und Loopback;
andere VPN-Netze in `allowed_networks` aufnehmen. IPv4 ist implementiert.
Die feste Management-Schnittstelle kann über `interface` gewählt werden.

```bash
systemctl status netcore-deployment --no-pager
journalctl -u netcore-deployment -n 100 --no-pager
systemctl status netcore-discovery --no-pager
journalctl -u netcore-discovery -n 100 --no-pager
```

Die UI meldet Multicast-Fehler; feste Gegenstellen bleiben verwendbar.
`Unbekannt` beim Commit bedeutet: noch kein erfolgreiches, vom Agenten
aufgezeichnetes Deployment. Es wird kein Versionsstand aus dem Quellordner als
vermeintlich laufende Binary-Version übernommen.

Vor jedem bestehenden Deployment wird die ursprüngliche Konfiguration unter
`/var/lib/netcore-discovery/backups/` gesichert. Die Bestandsinstaller haben
unterschiedliche Build-/Migrationsabläufe; es gibt **keinen behaupteten atomaren
Rollout und keinen pauschalen automatischen Rollback** aller Dienste. Fehler
erscheinen im Auftrag, und nur ein erfolgreicher Healthcheck setzt den neuen
Commit-Marker. Die gesicherte Konfiguration und der letzte bekannte Commit
ermöglichen die gezielte Wiederherstellung. Der bestehende TBS-Updater behält
seinen eigenen Binary-Rollback.

Aufträge, die bei einem Agent-/Controller-Neustart offen waren, werden als
`interrupted` markiert und nicht erneut ausgeführt. Ein Remote-Auftrag kann nach
Verlust des Controllers auf seinem Zielhost weiterlaufen; die Remote-Auftrags-ID
steht im Controller-Log. Vor erneutem Ausrollen dort den Zustand prüfen.

Der Controller läuft als `netcore-deploy`; der Agent läuft für Paketinstallation
und systemd-Steuerung als root. Das sind Betriebssystemkonten, keine Web-Logins.

### Aktualisieren / Discovery rückgängig machen

```bash
git fetch origin
git switch feature/openlab-discovery-deployment
git pull --ff-only
sudo bash system-backend/deployment-core/install/update.sh controller
# Auf einem Zielhost stattdessen: .../update.sh agent
```

Zum Entfernen der Discovery-Auflösung für einen einzelnen Dienst dessen Datei
`/etc/systemd/system/<unit>.d/50-discovery.conf` entfernen, `systemctl daemon-reload`
ausführen und den Dienst im passenden Betriebsfenster neu starten. Er nutzt dann
wieder seine ursprüngliche TOML. Zuvor gewünschte WebUI-Änderungen aus der
Runtime-Datei übernehmen. Das bloße Stoppen des Agenten entfernt den Cache nicht
und stoppt keine Funkdienste.

Discovery unterscheidet Erreichbarkeit/Liveness von vollständiger Readiness.
Ein laufender Control Room mit noch fehlenden Abhängigkeiten kann bereits als
Ziel aufgelöst werden; sonst könnten sich gegenseitige Abhängigkeiten beim
Erstaufbau blockieren. Eingeschränkte Readiness bleibt in der Übersicht sichtbar.
Ein Deployment mit erfolgreichem Liveness-Check, aber noch eingeschränkter
Readiness wird entsprechend im Auftragsprotokoll vermerkt.

## API und Prüfung

Lesend: `GET /health/live`, `/health/ready`, `/api/v1/status`, `/api/v1/manifest`,
`/api/v1/peers`, `/api/v1/catalog`, `/api/v1/jobs`, `/api/v1/jobs/<id>`.
JSON-POST: `/api/v1/discovery/scan`, `/api/v1/settings`; Controller zusätzlich
`/api/v1/check`, `/api/v1/plan`, `/api/v1/deploy`, `/api/v1/template`,
`/api/v1/profiles`; Agent: `/api/v1/jobs`.

```bash
python3 -m unittest discover -s system-backend/deployment-core/tests -v
node --check system-backend/deployment-core/static/app.js
bash -n system-backend/deployment-core/install/create-lxc.sh
```

Tests decken u. a. echte HTTP-Verbindungen zwischen Controller/Agenten, Seed-Relay,
Commit-Pinning über ein lokales Git-Remote, konkurrierende Rollen, Ausfallcache,
TOML-Roundtrip aller Katalogvorlagen, Runtime-Edit-Erhalt, Fehlerzustände und
Auftragspersistenz ab. Proxmox-, SDR- und echte Zielhost-Installationen müssen im
Lab abgenommen werden; Tests ersetzen diesen Hardwarelauf nicht.

Implementationsreferenzen: [Proxmox pct](https://pve.proxmox.com/pve-docs/pct.1.html),
[systemd.service](https://www.freedesktop.org/software/systemd/man/latest/systemd.service.html),
[systemd.exec](https://www.freedesktop.org/software/systemd/man/latest/systemd.exec.html).
