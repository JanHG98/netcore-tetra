# KMF – Installation im LXC

Die KMF benötigt im Lab Linux mit systemd, `iproute2`, Root-Rechte für die Installation und eine zur Workspace-Konfiguration passende Rust-/Cargo-Toolchain. Ein eigener LXC und ein isoliertes Managementnetz erleichtern den Schutz von Vault, Bootstrap-Dateien und Backups. Der Paketcode definiert keine verifizierte Mindestgröße für CPU oder RAM.

## Installation und Prüfung

Der Installer ruft Cargo im aktuellen Arbeitsverzeichnis auf. Deshalb aus dem Repository-Hauptverzeichnis starten:

```bash
sudo system-backend/kmf/install/install.sh
systemctl status netcore-kmf
source /etc/netcore/lxc-network.env
curl --fail "${NETCORE_WEBUI_URL}health/live"
curl --fail "${NETCORE_WEBUI_URL}health/ready"
```

Die Unit startet `/usr/local/bin/netcore-kmf --config /etc/netcore/kmf.toml`. Eine vorhandene Konfiguration wird nicht durch die Vorlage ersetzt; der gemeinsame LXC-Netzwerkhelfer setzt den Listener jedoch auf die erkannte LXC-IPv4-Adresse und schreibt die tatsächliche WebUI-Adresse nach `/etc/netcore/lxc-network.env`. TCP **8190** führt WebUI, Management- und Edge-API gemeinsam ohne TLS oder Anmeldung.

## Rechte und Daten

| Pfad | Rechte bei Anlage / Zweck |
| --- | --- |
| `/etc/netcore/kmf.toml` | `root:netcore-kmf`, `0640` |
| `/var/lib/netcore-kmf` | `netcore-kmf:netcore-kmf`, `0700` |
| `master.key` | `0600`, lokale Vault-Wurzel |
| `vault.json` | `0600`, Lab-Secret-Blobs |
| `state.json` | Metadaten, Jobs und Audit |
| `bootstrap/` und `backups/` | Unterhalb des Datenverzeichnisses; private Dateien mit `0600` |

systemd verwendet `UMask=0077`, `NoNewPrivileges`, leere Capability-Sets und `ProtectSystem=strict`. Schreiben ist unter `/var/lib/netcore-kmf` erlaubt. Eigene Pfade außerhalb davon erfordern eine passende Unit-Anpassung.

## Betriebsprüfung

Die Beispielpolicy beginnt in `shadow`; ein leerer Claim ist dabei erwartbar. Readiness bewertet `vault_ready` und ist kein Nachweis produktiver Kryptografie oder erfolgreichen Funk-OTARs. Backups benötigen eine separat gesicherte Schlüsselwurzel; [Details](vault-backups-und-hsm.md).

**Quellabgleich vom 9. Oktober 2026:** [Installer](../../../system-backend/kmf/install/install.sh), [Unit](../../../system-backend/kmf/systemd/netcore-kmf.service) und [API](../../../system-backend/kmf/src/http.rs).
