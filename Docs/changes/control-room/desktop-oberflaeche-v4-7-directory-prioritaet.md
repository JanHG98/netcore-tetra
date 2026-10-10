# Leitstellen-Desktopoberfläche v4.7 – Directory-first Cleanup

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: NetCore Control Room Native UI v4.7 – Directory-first Cleanup. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Die aktuelle Desktop-Architektur und Bedienerprofile stehen in [Desktop-UI-Anleitung](../../services/control-room/desktop-bedienoberflaeche.md) und [Control Room](../../services/control-room/README.md); Implementierung: [UI-Quellen](../../../system-backend/control-room/ui/src). Versionsbezogene ZIP-Dateien sind durch den gepflegten Repository-Stand abgelöst.

Dieser Änderungsnachweis bewahrt den beschriebenen UI-/API-/Buildstand. Damalige Tokenregeln, Feldnamen, Überschreib- und Buildbefehle gelten für diese Revision und dürfen nicht als aktuelle Komplettanleitung übernommen werden.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### NetCore Control Room Native UI v4.7 – Directory-first Cleanup

Dieser Stand macht das Directory zur Stammdatenquelle:

- LXC-Core liefert `GET /api/directory` aus `[directory]` in `/etc/netcore-control-room/control-room.toml`.
- Windows-UI zieht dieses Directory automatisch und merged lokale `operator.toml`-Overrides darüber.
- Teilnehmer-Tab zeigt Live-Geräte plus bekannte Directory-Geräte, aber keine Infrastruktur/Gateways/Basisstationen.
- Gruppen-Tab zeigt Live-Gruppen plus Directory-Gruppen.
- Karte/Standorte/Markerdetails nutzen Directory-Namen, Typen, Statusgruppen und Gruppen auch dann, wenn kein Live-Teilnehmerobjekt vorhanden ist.
- Unbekannte Statusnummern werden nicht mehr roh angezeigt.

Keine Patch-Dateien, kompletter Dateistand.
