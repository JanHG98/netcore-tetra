# Betriebsüberwachung: Installation im LXC

Empfohlen: Debian-LXC mit eigener Management-IP, mindestens 2 vCPU, 4 GiB RAM und ausreichend Speicher für die gewünschte Prometheus-/Loki-Retention.

Aus dem Repository-Hauptverzeichnis:

```bash
sudo bash system-backend/observability/install/install.sh
sudo bash system-backend/observability/install/install-stack.sh
```

Vor dem Start sind in `/etc/netcore/observability.toml` alle `base_url`-Werte auf die echten LXC-Adressen zu setzen. Die Stack-Installation aktiviert nur bereits installierte Binaries. Grafana, Loki und Prometheus sollten mit eigenen Systemnutzern und Schreibverzeichnissen betrieben werden.

Der gemeinsame Installer setzt `server.bind` auf die erkannte Management-IP. Die
Prüf-URL muss deshalb diese IP verwenden; `127.0.0.1` ist nur bei einem entsprechend
konfigurierten Listener erreichbar. Bei mehreren Interfaces kann
`NETCORE_LXC_IP` beim Installieren die Adresse festlegen.

**Quellabgleich: 9. Oktober 2026.** [Quellcode](../../../system-backend/observability) · [Konfigurationsvorlagen](../../../system-backend/observability/config).
