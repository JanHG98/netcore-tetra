# Phase 9 – WAP-Formulare und strukturierte Aufträge

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: Phase 9 – WAP-Formulare und strukturierte Aufträge. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Die gepflegte Implementierung und die aktuelle Betriebsanleitung stehen unter [task-workflow](../../services/task-workflow/README.md) und [Quellcode](../../../system-backend/task-workflow). Die `?issi`-Testidentität in WAP-Beispielen ist keine nachgewiesene Benutzeranmeldung.

Phasennummern, damalige Ports, Testidentitäten und Befehle dokumentieren die Einführung. Neue Installationen folgen der heutigen Komponentenbeschreibung und dem tatsächlich gewählten Inventory.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### Phase 9 – WAP-Formulare und strukturierte Aufträge

## Ziel

Phase 9 ergänzt einen eigenen `task-workflow`-LXC. Er verwaltet Aufgaben als `netcore-task-v1`, veröffentlicht `task.*`-Ereignisse und stellt dieselben Vorgänge über REST, MQTT, SDS, XHTML Basic und WML 1.1 bereit.

## Dienst

```text
system-backend/task-workflow/
Port 8280
OPEN LAB: kein Login, kein Token, kein TLS
```

## Datenwege

```text
Control Room / API / WAP
        -> Task Workflow
        -> SDS Router -> TETRA
        -> MQTT -> Home Assistant / Automationen

TETRA SDS oder Status
        -> SDS Router event
        -> MQTT
        -> Task Workflow state transition
```

## WAP-Einstiege

- `/x`: XHTML Basic
- `/w`: WML 1.1
- `?issi=4010001`: Testidentität im Open Lab

Der Dienst ist direkt über TETRA-Paketdaten erreichbar. Der lokale kompakte TBS-WSP-Portalpfad wird um Informationsseiten für Aufgaben/Formulare ergänzt, ist aber noch kein Reverse Proxy zum zentralen LXC.

## SDS-Kommandos

`TAKE`, `START`, `BLOCK`, `DONE`, `CANCEL`, `REOPEN`, `INFO` gefolgt vom kurzen Task-Token.

## Pre-coded Status

5301 annehmen, 5302 starten, 5303 blockieren, 5304 erledigen, 5305 abbrechen.
