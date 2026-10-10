# Packet Core – Paketdatenkern – historische Einspielung

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: SWMI Core 1 / Paket G. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Für packet-core und seine Dienstgrenzen sind [packet-core](../../services/packet-core/README.md) und [Quellcode](../../../system-backend/packet-core) die aktuellen Einstiege. Die damaligen Paketlisten beschreiben keine heute vollständig installierte Anlage.

Die damaligen ZIP-Namen, vollständigen Quellbaum-Ersetzungen und Rückbaubefehle gehören zum beschriebenen Lieferstand. Für ein heutiges Update den aktuellen Komponenten-Installer und die bestehende Standortkonfiguration verwenden; diese Einspielsequenz nicht ungeprüft erneut ausführen.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### SWMI Core 1 – Paket G anwenden

## 1. Packet-Core-LXC vorbereiten

Empfohlen: Debian 13, 2 vCPU, 1–2 GiB RAM, feste Management-IP.

## 2. Installieren

```bash
cd /opt/netcore-tetra
sudo system-backend/packet-core/install/install.sh
```

## 3. Konfigurieren

```bash
sudo editor /etc/netcore/packet-core.toml
sudo systemctl restart netcore-packet-core
```

Node Gateway muss unter der in `[node_gateway].url` eingetragenen Adresse erreichbar sein.

## 4. Prüfen

```bash
curl http://127.0.0.1:8160/health/live
curl http://127.0.0.1:8160/health/ready
curl http://127.0.0.1:8160/api/v1/status
journalctl -u netcore-packet-core -f
```

WebUI:

```text
http://<Packet-Core-IP>:8160/
```

## 5. Betriebsmodus

Zuerst immer:

```toml
[packet]
mode = "shadow"
```

Erst nach stabilen Vergleichsläufen in einem isolierten Testnetz auf `authoritative` umstellen.

## 6. Rollback

Der Dienst ist nicht im zeitkritischen RF-Pfad. Ein Stop des Packet Core lässt die lokale TBS-SNDCP-Implementierung weiterarbeiten. Für einen vollständigen Rückbau:

```bash
sudo system-backend/packet-core/install/uninstall.sh
```
