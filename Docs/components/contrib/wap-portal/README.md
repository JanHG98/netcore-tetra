# NetCore-Tetra WAP-Portal

**Quellen:** [contrib/wap-portal](../../../../contrib/wap-portal) · [Repository-Root](../../../..). Bei Befehlen das in der Anleitung angegebene Arbeitsverzeichnis beachten.

Dieses Verzeichnis enthaelt ein statisches Referenzportal mit 21 Seiten in zwei Formaten:

- `xhtml/`: XHTML Basic 1.1
- `wml/`: WML 1.1

Alle Seiten sind innerhalb ihres Formats vollstaendig navigierbar. Die Startseiten sind:

- `xhtml/index.xhtml`
- `wml/index.wml`

Der aktive Basisstations-Adapter bedient derzeit nur `/`, `/status`, `/status.xhtml` und `/status.wml`. Ein 21-Seiten-Renderer mit kurzen `/x/...`-/`/w/...`-Routen liegt im Quellbaum, ist jedoch nicht als Modul eingebunden. Die statischen Referenzdateien sind daher kein Nachweis eines aktiven vollständigen Funkportals. Details: [Portal und Laufzeitgrenze](../../../guides/wap/wap-portalreferenz-und-laufzeitgrenze.md).

## MIME-Typen

```text
.xhtml  application/vnd.wap.xhtml+xml; charset=UTF-8
.wml    text/vnd.wap.wml; charset=UTF-8
```

Der heutige Referenzbestand enthält außerdem die Seiten `tasks` und `task-form`. Die dynamischen Formulare liegen im zentralen Task Workflow auf Port 8280.
