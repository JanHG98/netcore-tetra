# WAP-Portal: Referenzseiten und Laufzeitgrenze

**Quellstand:** `main`, `c3ccdb4`, 09.10.2026. Quellen: [statisches Portal](../../../contrib/wap-portal), [Referenzrenderer](../../../crates/tetra-entities/src/sndcp/wap_portal.rs), [aktiver WAP-Adapter](../../../crates/tetra-entities/src/sndcp/wap_ip.rs), [SNDCP-Module](../../../crates/tetra-entities/src/sndcp/mod.rs).

## Was heute aktiv ist

Der eingebundene `wap_ip.rs` bedient **`/`, `/status`, `/status.xhtml` und `/status.wml`**. Ein Query mit `?s=` wählt die kompakte Sektor-/Health-Darstellung. Er verwendet die vorhandenen Schalter `accept_root_path`, `accept_status_path` und `accept_status_wml_path`; diese schalten heute keine vollständige Portalsammlung frei. XHTML bleibt auf 104 Byte für die Statusansicht beziehungsweise 144 Byte für die Sektorseite begrenzt; WML auf 144 Byte.

Die Datei `wap_portal.rs` enthält einen Parser und Renderer für 21 Seiten. Sie ist im aktuellen `sndcp/mod.rs` **nicht eingebunden**, und der aktive WAP-Adapter ruft sie nicht auf. Die kurzen Portalpfade `/x/...` und `/w/...` sowie `/index.xhtml` oder `/tasks.wml` sind deshalb kein bestätigter aktiver Funkdienst. Die dort abgelegten Tests sind ohne Moduleinbindung auch kein Nachweis eines ausgeführten Workspace-Tests.

## Vorhandene Referenzseiten

Unter `contrib/wap-portal/` liegen tatsächlich **21 XHTML-Basic- und 21 WML-Dateien**. Sie sind statische Muster für einen separat bereitgestellten HTTP/WAP-Webserver; die Dateien allein koppeln keinen Backend-Dienst an den SNDCP-Funkpfad.

| Seite | Lesbarer Referenzname | Kurzer Pfad im nicht eingebundenen Renderer |
| --- | --- | --- |
| Start | `index` | `/x`, `/w` |
| Status | `status` | `/x/st`, `/w/st` |
| Teilnehmer | `subscribers` | `/x/ms`, `/w/ms` |
| Gruppen | `groups` | `/x/gr`, `/w/gr` |
| Rufe | `calls` | `/x/ca`, `/w/ca` |
| SDS | `sds` | `/x/sd`, `/w/sd` |
| Aufträge | `tasks` | `/x/tk`, `/w/tk` |
| Auftragsformular | `task-form` | `/x/fm`, `/w/fm` |
| Control Room | `control-room` | `/x/cr`, `/w/cr` |
| Health | `health` | `/x/he`, `/w/he` |
| Funkzelle | `radio` | `/x/ra`, `/w/ra` |
| Paketdaten | `packet-data` | `/x/pd`, `/w/pd` |
| IP Gateway | `gateway` | `/x/gw`, `/w/gw` |
| Dienste | `services` | `/x/sv`, `/w/sv` |
| Diagnose | `diagnostics` | `/x/dg`, `/w/dg` |
| Media Library | `media-library` | `/x/me`, `/w/me` |
| Recorder | `recorder` | `/x/re`, `/w/re` |
| TTS | `tts` | `/x/tt`, `/w/tt` |
| Tests | `tests` | `/x/te`, `/w/te` |
| Hilfe | `help` | `/x/hl`, `/w/hl` |
| Projekt | `about` | `/x/ab`, `/w/ab` |

Für jede Referenzseite existieren die Endungen `.xhtml` und `.wml`. Die jeweilige Startdatei ist `xhtml/index.xhtml` beziehungsweise `wml/index.wml`. Die statischen Links lassen sich mit `python3 contrib/wap-portal/validate.py` prüfen. Das validiert Dateien und Navigation, keine Funkübertragung.

Die dynamischen XHTML-/WML-Auftragsformulare des **Task Workflow** auf Port 8280 sind ein eigener HTTP-Dienst. Ihre Existenz bindet diese Formulare nicht automatisch in den TBS-WAP-Adapter ein. Der [aktive SNDCP-/WAP-Statusdienst](wap-statusdienst-ueber-sndcp.md) beschreibt die heutigen Laufzeitgrenzen.
