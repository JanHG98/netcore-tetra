# SDS Router – Nachrichtenweiterleitung – historische Einspielung

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: SWMI Core 1 / Paket F. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Für sds-router und seine Dienstgrenzen sind [sds-router](../../services/sds-router/README.md) und [Quellcode](../../../system-backend/sds-router) die aktuellen Einstiege. Die damaligen Paketlisten beschreiben keine heute vollständig installierte Anlage.

Die damaligen ZIP-Namen, vollständigen Quellbaum-Ersetzungen und Rückbaubefehle gehören zum beschriebenen Lieferstand. Für ein heutiges Update den aktuellen Komponenten-Installer und die bestehende Standortkonfiguration verwenden; diese Einspielsequenz nicht ungeprüft erneut ausführen.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### Package F anwenden

## SDS Router installieren

```bash
sudo system-backend/sds-router/install/install.sh
```

Konfiguration prüfen:

```bash
sudo editor /etc/netcore/sds-router.toml
sudo systemctl restart netcore-sds-router
```

WebUI:

```text
http://<SDS-Router-IP>:8150/
```

## TBS schrittweise umstellen

In der TBS-`config.toml`:

```toml
[control_room]
central_sds_routing = true
```

Danach die TBS neu starten. Ohne diesen Schalter bleibt das bisherige lokale SDS-Routing aktiv.

## Abnahme

1. TBS erscheint unter `/api/v1/nodes`.
2. Registrierung eines Endgeräts erscheint unter `/api/v1/subscribers`.
3. Eine Uplink-SDS erzeugt einen Datensatz unter `/api/v1/messages`.
4. Eine Downlink-SDS über die WebUI wird an die zuständige TBS eingeplant.
5. Bei ausgeschaltetem Zielgerät bleibt die Nachricht offline/queued und läuft nicht vor TTL ab.
6. Nach erneuter Registrierung wird die Nachricht neu eingeplant.
7. Eine Protocol-ID-Tap-Regel erzeugt einen Eintrag in der Application-Outbox.
