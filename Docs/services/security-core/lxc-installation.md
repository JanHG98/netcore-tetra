# Security Core – Installation im LXC

Diese Anleitung beschreibt den vorhandenen systemd-Installer. Ein Debian-LXC mit eigenem isoliertem Managementnetz ist ein geeigneter Lab-Aufbau; CPU-, RAM- und Plattengröße müssen nach Build- und Lastbedarf gewählt werden. Eine verifizierte Mindestgröße ist im Paket nicht hinterlegt.

## Installation

Voraussetzungen: Linux mit systemd, `iproute2`, Root-Rechte und eine zur Workspace-Konfiguration passende Rust-/Cargo-Toolchain. Aus dem Repository-Hauptverzeichnis:

```bash
sudo system-backend/security-core/install/install.sh
systemctl status netcore-security-core
source /etc/netcore/lxc-network.env
curl --fail "${NETCORE_WEBUI_URL}health/ready"
```

Der Installer baut mit `CARGO_BUILD_JOBS=1`, soweit nicht vorgegeben, stoppt eine vorhandene Instanz vor dem Build und aktiviert den Dienst anschließend. Eine vorhandene Konfiguration wird nicht durch die Beispieldatei ersetzt; der gemeinsame LXC-Netzwerkhelfer setzt den Listener jedoch auf die erkannte LXC-IPv4-Adresse und schreibt die tatsächliche WebUI-Adresse nach `/etc/netcore/lxc-network.env`.

## Installierte Pfade

| Pfad | Verwendung |
| --- | --- |
| `/usr/local/bin/netcore-security-core` | Release-Binärdatei |
| `/etc/netcore/security-core.toml` | Konfiguration, `root:netcore-security`, `0640` bei Anlage |
| `/var/lib/netcore-security-core/state.json` | Persistente Metadaten |
| `/var/lib/netcore-security-core/state.json.bak` | Metadatenbackup |
| `/var/lib/netcore-security-core/lab-auth.seed` | Lab-Seed, Dienstkonto `netcore-security`, `0600` bei Anlage |

Das Datenverzeichnis gehört `netcore-security:netcore-security` und wird mit `0700` angelegt. systemd nutzt `UMask=0077`, `ProtectSystem=strict` und erlaubt Schreiben im Datenverzeichnis.

## Erstprüfung

Die Beispieldatei beginnt in `shadow` und bindet an `0.0.0.0:8180`. Den konfigurierten Gateway-Endpunkt für einen separaten Gateway-LXC prüfen; `ws://127.0.0.1:8080/ws/backend` funktioniert nur bei lokaler Gateway-Instanz. Beim authoritative-Betrieb kann eine fehlende Gateway-Verbindung die Readiness auf 503 setzen.

**Quellabgleich vom 9. Oktober 2026:** [Installer](../../../system-backend/security-core/install/install.sh), [Unit](../../../system-backend/security-core/systemd/netcore-security-core.service) und [Konfiguration](../../../system-backend/security-core/config/security-core.example.toml). Weiter: [Open Lab](offene-testumgebung.md).
