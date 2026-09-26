# Open-Lab-Deployment der Backend-LXCs

Die aktuelle Vorlage in [`deploy/open-lab/`](https://github.com/JanHG98/netcore-tetra/tree/main/deploy/open-lab) plant **einen Dienst pro LXC**. Der Deployment-Host rendert aus einem lokalen Inventory Konfigurationen und führt Installer in Abhängigkeitsreihenfolge per SSH aus. Er speichert nach der [README](https://github.com/JanHG98/netcore-tetra/blob/main/deploy/open-lab/README.md) keine Passwörter, Tokens oder KMF-Schlüssel.

> Die Vorlage ist `mode = "open_lab"`. Alle erreichbaren Clients im Managementnetz können bei vielen Diensten Verwaltungsaktionen auslösen. Vor dem ersten `apply` Netzisolation und die zugewiesenen LXC-Adressen prüfen. [[Security-and-Operations]]

## Voraussetzungen

- Debian 13 oder kompatible LXC mit `systemd`, Rust-Toolchain und C-Build-Werkzeugen;
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

Die zweite Zeile verändert die Ziel-LXCs. Der Deployer baut die betroffenen Binärdateien auf den Zielhosts, überträgt gerenderte Konfigurationen und startet Dienste in Abhängigkeitsreihenfolge. **Kein blindes Wiederholen** bei Teilfehlern: zuerst betreffenden LXC, Installer-Log, TOML und Journal prüfen. Das lokale `inventory.toml` enthält Standortdaten und gehört nicht als unveränderte Vorlage ins Wiki.

## Abnahme in Stufen

```bash
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml test --profile full --validate-only
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml test --profile smoke
```

`validate-only` prüft lokal, `smoke` liest Dienstverträge und nutzt einen Mock-TBS-Pfad. Die Profile `full --allow-mutations` und `fault --allow-mutations --allow-restarts` verändern Testdaten bzw. stoppen Dienste absichtlich; nur in einem dafür vorgesehenen Labornetz durchführen. Ausgaben liegen unter `tests/e2e/artifacts/<run-id>/`. Das ältere [E2E-Runbook](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/OPEN_LAB_E2E_RUNBOOK.md) erwähnt noch 17 Dienste: Befehle und Dienstzahl stets am aktuellen Inventory prüfen.

## Ergänzende Dienste

Die lokale TBS, Provisioning Core, Directory, Piper, lokaler TBS-Asterisk und Brew-Server sind **keine stillschweigend vom 24er-Inventory installierten Komponenten**. Sie haben eigene Installations- und Freigabeschritte: [[Installation]] · [[Provisioning]] · [[NetCore-Directory]] · [[SIP-und-Brew]].

## Update und Rückweg

1. `git status`, Branch/Commit und Ist-Konfiguration auf Deployment- und Zielhosts dokumentieren.
2. Konfigurationen, Persistenzdaten und Keys nach passendem Verfahren sichern. [[Backup-and-Fallback]]
3. Änderungen per `validate` → `plan` → `render` → `apply --dry-run` prüfen.
4. Abhängigkeitsreihenfolge beachten; erst danach `apply` und `status`.
5. Bei Fehler: Zielhost auf letzten bekannten Build/Konfigurationsstand zurücksetzen und Datenmigration gesondert behandeln.

Ein fertiges Pi-Image und automatische Multicast-/Broadcast-Discovery aller Dienste gehören zur [[Roadmap]], nicht zum beschriebenen Deployer.
