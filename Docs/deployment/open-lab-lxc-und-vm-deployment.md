# Open-Lab-Deployment: LXC-Dienste und Deployment-VM

**Quellstand:** `main`, `c3ccdb4`, 09.10.2026. Maßgeblich sind [Inventory](../../deploy/open-lab/inventory.example.toml), [Deployer](../../deploy/open-lab/netcore-deploy.py) und [ausführliche Anleitung](open-lab/README.md). Das Inventory deklariert **26 Laufzeitdienste**; das ist keine Behauptung, dass 26 Hosts bereits laufen.

## Netz- und Hostmodell

Die fachlichen Backend-Dienste laufen vorzugsweise in eigenen Debian-LXCs mit fester Adresse oder DHCP-Static-Lease im isolierten Management-Netz. **Deployment Core** ist dagegen der Ubuntu-VM-/Imagebuilder-Dienst auf Port 8320. Warnzentrale/Alert Service läuft auf 8310 und verlangt standardmäßig ein lokal erzeugtes API-Token. Die übrigen offenen Lab-UIs besitzen je nach Dienst keine Anmeldung oder TLS; die tatsächlichen Zugangsmodi stehen im Inventory und in den Dienstanleitungen.

Die Beispiele enthalten verschiedene Netze und TBS-Adressen. Eigene IPs, DNS, SSH-Hosts, Units und Abhängigkeiten vor dem Einsatz anpassen. Provisioning Core (8125), Directory und Legacy-Brew sind zusätzliche Komponenten; sie erhöhen nicht stillschweigend den Inventory-Zähler.

## Ablauf

Der Deployer berechnet die Reihenfolge aus `depends_on` und ergänzt ausgewählte Dienste um ihre transitiven Abhängigkeiten. Im Repository-Root zuerst die eigene Inventardatei erstellen und prüfen:

```bash
cp deploy/open-lab/inventory.example.toml deploy/open-lab/inventory.toml
$EDITOR deploy/open-lab/inventory.toml
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml validate
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml plan
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml render
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml apply --dry-run
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml apply
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml status
```

`apply` erhält bestehende Konfigurationen und Tokens. Ein absichtliches `--replace-config` erzeugt eine Sicherung und ersetzt Konfigurationen; vorher die [Details](open-lab/README.md) lesen. Die Ready-Schranke und ihre tatsächliche Fehlermeldung vor nachgelagerten Deployments beachten.

## Hostvoraussetzungen und Rückweg

- IP Gateway benötigt `/dev/net/tun` und geeignete Rechte im eigenen Namespace; [LXC-Beispiel](../../deploy/open-lab/lxc/ip-gateway.conf.example).
- Recorder und Media Library behalten Live-State lokal; NFS-Archiv separat vorbereiten.
- Observability benötigt Zugriff auf die im Inventory angegebenen Dienstendpunkte.
- Deployment Core benötigt die in seiner eigenen Anleitung beschriebenen VM-/Imagebuilder-Werkzeuge.

Vor Updates dienstspezifische Backups von State, SQLite-Dateien, Konfigurationen und Zugangsdaten erstellen. Ein Quell-/Bundle-Rollback stellt persistente Fachdaten nicht automatisch auf einen früheren Stand zurück.

## Abnahme

```bash
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml test --profile smoke
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml test --profile full --allow-mutations
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml test --profile fault --allow-mutations --allow-restarts
```

Smoke prüft Verträge und erstellt bei aktivem Mock-TBS temporäre Sessions. Full erzeugt markierte Fachfixtures und entfernt sie standardmäßig; Fault stoppt/restartet ausgewählte Dienste. Die [E2E-Anleitung](open-lab-integrationstest-anleitung.md) trennt diese Nachweise von einer echten Funkabnahme.
