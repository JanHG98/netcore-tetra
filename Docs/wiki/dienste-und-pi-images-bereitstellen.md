# Dienste und Pi-Images im Open Lab bereitstellen

Im Repository gibt es zwei ergänzende Bereitstellungswege. Der **Inventory-Deployer** rendert Dienstkonfigurationen und installiert sie per SSH in Abhängigkeitsreihenfolge. **Deployment Core** betreibt eine WebUI, Discovery und Agentenaufträge; sein Imagebuilder erstellt personalisierte Pi-Images auf einer vollständigen Ubuntu-VM. Diese Seite erklärt die Auswahl und den Inventory-Weg. Die [Deployment-Core-Anleitung](../services/deployment-core/README.md) führt durch VM, Agenten und Imageformular.

> Die Vorlage ist `mode = "open_lab"`. Alle erreichbaren Clients im Managementnetz können bei vielen Diensten Verwaltungsaktionen auslösen. Vor dem ersten `apply` Netzisolation und die zugewiesenen LXC-Adressen prüfen. [Sicherheit und Betrieb](sicherheit-im-betrieb.md)

## Voraussetzungen

- Dienstspezifisch unterstützte Debian-LXC mit `systemd`, Rust-Toolchain und C-Build-Werkzeugen;
- **Ausnahme Deployment Core:** vollständige Ubuntu-24.04-/26.04-VM auf AMD64 oder ARM64 mit separatem Imagebuilder, kein LXC;
- separater Deployment-Host mit Root-SSH-Schlüsselzugriff zu den Zielcontainern;
- auf jedem LXC echte IP-Adresse, DNS/Hosts-Auflösung und erreichbare Abhängigkeiten;
- `/dev/net/tun` für den IP Gateway, vorbereiteter NFS-Mount für Archivfunktionen;
- Quellstand, Wartungsfenster und Sicherung festhalten. Bei laufenden Rufen keine unkoordinierte Dienstwelle auslösen.

## Inventory vorbereiten und prüfen

Im Repositoryverzeichnis auf dem Deployment-Host:

```bash
cp deploy/open-lab/inventory.example.toml deploy/open-lab/inventory.toml
$EDITOR deploy/open-lab/inventory.toml
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml validate
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml plan
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml render
```

Alle Beispielhosts ersetzen. Insbesondere die TBS-`[control_room]`-Adresse auf **denselben Node Gateway** wie das Inventory setzen; die TBS verbindet sich im verteilten Aufbau über `/ws/node`, nicht direkt mit der Control-Room-WebUI. Gerenderte Dateien, Abhängigkeiten und Portliste vor der Installation lesen.

## Trockenlauf und Anwendung

```bash
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml apply --dry-run
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml apply
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml status
```

Die zweite Zeile verändert die Zielhosts. Der Deployer führt die jeweiligen Installer aus, überträgt gerenderte Konfigurationen und wartet nach jedem Dienst begrenzt auf Readiness, bevor abhängige Installer starten. Rust-Dienste werden auf den Zielhosts gebaut; Deployment Core hat einen Python-/VM-Installer. Ein HTTP-Erfolg mit ausdrücklich negativem Ready-Zustand gilt als Fehler. **Kein blindes Wiederholen** bei Teilfehlern: zuerst betreffenden LXC, Installer-Log, TOML und Journal prüfen. Das lokale `inventory.toml` enthält Standortdaten und gehört nicht als unveränderte Vorlage ins Wiki.

## Abnahme in Stufen

```bash
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml test --profile full --validate-only
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml test --profile smoke
```

`validate-only` prüft lokal, `smoke` liest Dienstverträge und nutzt einen Mock-TBS-Pfad. Die Profile `full --allow-mutations` und `fault --allow-mutations --allow-restarts` verändern Testdaten bzw. stoppen Dienste absichtlich; nur in einem dafür vorgesehenen Labornetz durchführen. Ausgaben liegen unter `tests/e2e/artifacts/<run-id>/`. Die [E2E-Anleitung](../testing/e2e/README.md) beschreibt die Artefakte und Grenzen. Befehle und Dienstzahl stets am aktuellen Inventory prüfen.

## Ergänzende Dienste

Die lokale TBS, Provisioning Core, Directory, Piper, lokaler TBS-Asterisk und Brew-Server sind **nicht Teil der 26 Backend-Rollen im Inventory**. Sie haben eigene Installations- und Freigabeschritte: [Installation der lokalen Basisstation](basisstation-installieren.md) · [Provisioning und Datenhoheit](teilnehmer-und-gruppen-anlegen.md) · [Namen und Metadaten im NetCore Directory](namen-und-metadaten-im-directory.md) · [SIP, lokaler Asterisk und Brew](telefonie-sip-und-brew.md).

## Update und Rückweg

1. `git status`, Branch/Commit und Ist-Konfiguration auf Deployment- und Zielhosts dokumentieren.
2. Konfigurationen, Persistenzdaten und Keys nach passendem Verfahren sichern. [Backup und Fallback](datensicherung-und-fallback.md)
3. Änderungen per `validate` → `plan` → `render` → `apply --dry-run` prüfen.
4. Abhängigkeitsreihenfolge beachten; erst danach `apply` und `status`.
5. Bei Fehler: Zielhost auf letzten bekannten Build/Konfigurationsstand zurücksetzen und Datenmigration gesondert behandeln.

## Discovery, Agenten und Pi-Imagebuilder

Deployment Core ist im aktuellen Quellbestand implementiert. Der Controller hört beispielhaft auf TCP `8320`; Agenten auf TCP `8321`. Discovery verwendet IPv4-Multicast `239.192.84.82:48320` mit TTL 1 und optional konfigurierten Unicast-Seeds. Agenten werben ihre installierten Rollen; Discovery erkennt keine beliebigen fremden Dienste und ist keine VPN-Einrichtung.

Der Imagebuilder läuft als eigener Dienst über einen lokalen Unix-Socket. Das Imageformular übernimmt TBS-Profil, Hardware, Benutzer/SSH, WLAN und optionales Inline-OpenVPN. Der Build verwendet frisch abgerufenes `origin/main` und hält den tatsächlichen SHA im Manifest fest. Bei konfiguriertem VPN schaltet die Pi-Regel bei vertrauter WLAN-SSID oder einer Ethernetadresse im Heimnetz den Client aus, sonst ein; der TBS-Dienst wird dabei nicht neu gestartet.

Ein erfolgreicher Imagejob, abrufbares Artefakt und tatsächlicher Hardwareboot brauchen getrennte Belege. Die [Imagebuilder-Abnahme](../integration/Z01-2026-10-07/imagebuilder-vm119.md) und [zentrale Roadmap](../roadmaps/gesamtroadmap.md) führen offene Standortprüfungen. Installer und API-Beispiele stehen in der [ausführlichen Deployment-Core-Anleitung](../services/deployment-core/README.md).

## Quellen zur Pflege dieser Seite

[Inventory-Deployer und Ready-Schranke](../../deploy/open-lab/netcore-deploy.py) · [Controller und Image-API](../../system-backend/deployment-core/main.py) · [VM-Installer](../../system-backend/deployment-core/install/install-vm.sh).
