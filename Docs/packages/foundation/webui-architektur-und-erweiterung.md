# WebUI-Architektur und Foundation-Erweiterung – SWMI-Paketstand

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: SWMI Foundation 1 / WebUI-Erweiterung. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Die funknahe Typ- und Runtimebasis bleibt Teil des lokalen TBS/MS-Stacks. Für den heutigen Bestand [Protokollinventur](../../protocols/swmi-protokollinventur.md), [lokale TLMC-Laufzeit](../../../crates/tetra-entities/src/umac/tlmc_runtime.rs) und [SAP-Typen](../../../crates/tetra-saps) und die [Facharbeitsliste](../../roadmaps/protokollfundament-arbeitsliste.md) heranziehen; daraus entsteht kein eigener Backend-Container.

Die Beschreibungen „umgesetzt“, „noch nicht enthalten“ und „nächster Baustein“ gelten innerhalb der damaligen Paketfolge. Sie ersetzen keinen Abgleich mit dem heutigen Quellcode oder mit der installierten Konfiguration.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### SWMI Foundation 1 – WebUI-Erweiterung

## Entscheidung

Jeder später eigenständig laufende Backend-Container erhält eine eigene WebUI zur Verwaltung. Diese Entscheidung ist ab Paket A verbindlicher Bestandteil der Architektur.

## Auswirkungen auf Foundation 1

- Die Backend-Service-README-Dateien beschreiben nun jeweils ihre vorgesehenen Verwaltungsansichten und kritischen Aktionen.
- `Docs/design/dienstoberflaechen-gestaltungsstandard.md` definiert gemeinsame Endpunkte, RBAC, Audit, Sicherheit und Definition of Done.
- `Docs/design/dienstoberflaechen-funktionsmatrix.md` ordnet jedem Dienst seine fachlichen UI-Bereiche zu.
- `system-backend/services.toml` hält die Pflicht maschinenlesbar fest.
- `system-backend/shared/web-ui/` ist für gemeinsame Komponenten reserviert.
- TLMC und TLPD bleiben Teil der TBS und erhalten keinen eigenen Container; ihre Diagnosezustände werden später über TBS und Node Gateway sichtbar gemacht.

## Kein zusätzlicher Container pro Oberfläche

Die WebUI wird vom jeweiligen Dienst selbst ausgeliefert. Ein Dienst benötigt daher nicht noch einen separaten Frontend-LXC.

## Zielbild

```text
Browser
  ├── Node Gateway WebUI
  ├── Subscriber Core WebUI
  ├── Mobility Core WebUI
  ├── Call Control WebUI
  └── weitere Service-WebUIs

Control Room
  └── aggregiert Status und verlinkt auf die einzelnen Oberflächen
```
