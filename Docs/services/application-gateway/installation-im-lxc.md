# Anwendungsanbindung: Installation im LXC

## Mindestanforderungen

- Debian/Ubuntu-LXC
- Rust-Toolchain für Installation aus dem Repo
- ausgehender HTTP/HTTPS-Zugriff zu aktivierten Connectoren
- Managementnetz-Zugriff auf TCP 8220
- lokaler oder erreichbarer Piper-Dienst für TTS

## Installation

Aus dem Repository-Hauptverzeichnis:

```bash
sudo bash system-backend/application-gateway/install/install.sh
```

Danach:

```bash
systemctl status netcore-application-gateway --no-pager
curl -fsS http://APPLICATION-GATEWAY-IP:8220/health/ready
```

## Netzgrenze

Die systemd-Unit benötigt nur `AF_UNIX`, `AF_INET` und `AF_INET6`. Keine TUN-/TAP-, GPIO-, SDR- oder Raw-Socket-Capabilities sind erforderlich.

## Daten

```text
/etc/netcore/application-gateway.toml
/var/lib/netcore-application-gateway/state.json
/var/lib/netcore-application-gateway/secrets.json
/var/lib/netcore-application-gateway/spool/
/var/lib/netcore-application-gateway/backups/
```

`secrets.json` muss getrennt von normalen State-Backups behandelt werden.

Der gemeinsame Installer setzt `server.bind` auf die erkannte Management-IP. Die
Prüf-URL muss deshalb diese IP verwenden; `127.0.0.1` ist nur bei einem entsprechend
konfigurierten Listener erreichbar. Bei mehreren Interfaces kann
`NETCORE_LXC_IP` beim Installieren die Adresse festlegen.

**Quellabgleich: 9. Oktober 2026.** [Quellcode](../../../system-backend/application-gateway) · [Konfigurationsvorlagen](../../../system-backend/application-gateway/config).
