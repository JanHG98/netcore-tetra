# Leitstellenoberfläche v5.6 – ELP-Leitstellenlayout

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: NetCore Control Room UI v5.6 – ELP-Leitstellenlayout. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Die aktuelle Desktop-Architektur und Bedienerprofile stehen in [Desktop-UI-Anleitung](../../services/control-room/desktop-bedienoberflaeche.md) und [Control Room](../../services/control-room/README.md); Implementierung: [UI-Quellen](../../../system-backend/control-room/ui/src). Versionsbezogene ZIP-Dateien sind durch den gepflegten Repository-Stand abgelöst.

Dieser Änderungsnachweis bewahrt den beschriebenen UI-/API-/Buildstand. Damalige Tokenregeln, Feldnamen, Überschreib- und Buildbefehle gelten für diese Revision und dürfen nicht als aktuelle Komplettanleitung übernommen werden.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### NetCore Control Room UI v5.6 – ELP-Leitstellenlayout

Dieser Stand gestaltet die Windows-Operator-UI stärker wie einen klassischen Einsatzleitplatz:

- blaue Kopfzeile mit Systemstatus
- Ribbon-/Toolbar-Leiste
- rollenbasierte Module bleiben erhalten
- links kompakte Modulleiste mit OS-Fenster-Buttons
- Übersicht als Funklage-/ELP-Cockpit
- Login als zentrierte Karte
- responsive Tabellen und größere Bedienfelder

LXC und TBS müssen für diesen UI-only-Stand nicht geändert werden, sofern v5.3+ dort bereits läuft.
