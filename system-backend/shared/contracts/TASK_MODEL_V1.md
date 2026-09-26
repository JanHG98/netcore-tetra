# NetCore Task Model v1

`netcore-task-v1` beschreibt strukturierte, persistent bearbeitbare Aufträge. REST, MQTT, SDS, XHTML und WML bilden denselben Auftrag ab.

## Zustände

```text
open -> assigned -> accepted -> in_progress -> completed
  \-> cancelled
  \-> expired
accepted/in_progress -> blocked -> in_progress
completed/cancelled/expired -> open (reopen)
```

## Identitäten

- `task_id`: globale UUID für APIs und Korrelation.
- `token`: kurze Funkreferenz wie `T12AB34C`.
- `assigned_issi`/`assigned_gssi`: optionales Ziel.
- `accepted_by_issi`: Teilnehmer, der den Auftrag angenommen hat.

Im OPEN-LAB-Modus ist `?issi=` nur eine Testidentität und keine Authentisierung.

## Ereignisse und MQTT

Der Task Workflow veröffentlicht `task.*` als `netcore-event-v1`; retained Zustände liegen unter `netcore/v1/state/tasks/<task-id>`.

## WAP

- XHTML Basic: `/x`
- WML 1.1: `/w`

WML-Formulare verwenden `<go>` und `<postfield>` statt HTML-Formularen.
