# NetCore-Tetra WAP-Portal

Stand: 30.07.2026

## Ziel

Die Basisstation stellt ueber den vorhandenen SNDCP/WDP/WTP/WSP-Pfad ein kompaktes, vollstaendig verlinktes NetCore-Portal bereit. Jede Portalseite existiert in zwei Darstellungen:

- XHTML fuer WAP-2.0-/XHTML-Mobile-Browser
- WML fuer klassische WAP-1.x-Browser

Die Seiten bleiben absichtlich klein. Der Motorola/Openwave-Pfad nutzt weiterhin die erprobten Obergrenzen von 104 Byte fuer die XHTML-Statusseite und 144 Byte fuer weitere XHTML-/WML-Seiten.

## Startseiten

| Format | Lesbarer Pfad | Kurzer Openwave-Pfad |
|---|---|---|
| XHTML | `/index.xhtml` oder `/` | `/x` |
| WML | `/index.wml` | `/w` |

## Seiten

| Inhalt | XHTML | WML | Kurzpfade |
|---|---|---|---|
| Start | `/index.xhtml` | `/index.wml` | `/x`, `/w` |
| Status | `/status.xhtml` | `/status.wml` | `/x/st`, `/w/st` |
| Teilnehmer | `/subscribers.xhtml` | `/subscribers.wml` | `/x/ms`, `/w/ms` |
| Gruppen | `/groups.xhtml` | `/groups.wml` | `/x/gr`, `/w/gr` |
| Rufe | `/calls.xhtml` | `/calls.wml` | `/x/ca`, `/w/ca` |
| SDS | `/sds.xhtml` | `/sds.wml` | `/x/sd`, `/w/sd` |
| Control Room | `/control-room.xhtml` | `/control-room.wml` | `/x/cr`, `/w/cr` |
| Health | `/health.xhtml` | `/health.wml` | `/x/he`, `/w/he` |
| Funkzelle | `/radio.xhtml` | `/radio.wml` | `/x/ra`, `/w/ra` |
| Paketdaten | `/packet-data.xhtml` | `/packet-data.wml` | `/x/pd`, `/w/pd` |
| IP Gateway | `/gateway.xhtml` | `/gateway.wml` | `/x/gw`, `/w/gw` |
| Dienste | `/services.xhtml` | `/services.wml` | `/x/sv`, `/w/sv` |
| Diagnose | `/diagnostics.xhtml` | `/diagnostics.wml` | `/x/dg`, `/w/dg` |
| Media Library | `/media-library.xhtml` | `/media-library.wml` | `/x/me`, `/w/me` |
| Recorder | `/recorder.xhtml` | `/recorder.wml` | `/x/re`, `/w/re` |
| TTS Piper | `/tts.xhtml` | `/tts.wml` | `/x/tt`, `/w/tt` |
| Tests | `/tests.xhtml` | `/tests.wml` | `/x/te`, `/w/te` |
| Hilfe | `/help.xhtml` | `/help.wml` | `/x/hl`, `/w/hl` |
| Projekt | `/about.xhtml` | `/about.wml` | `/x/ab`, `/w/ab` |

## Navigation

Die kompakte Navigation verwendet:

- `P`: vorherige Seite
- `N`: naechste Seite
- `H`: Startseite

Alle XHTML-Seiten verlinken ausschliesslich auf XHTML-Seiten. Alle WML-Seiten verlinken ausschliesslich auf WML-Seiten. Von der Startseite aus sind alle 19 Seiten des jeweiligen Formats erreichbar.

## Kompatibilitaet

Die bisherigen Pfade bleiben gueltig:

- `/status`
- `/status.xhtml`
- `/status.wml`
- `/status.xhtml?s=1`
- `/status.wml?s=1`

Der alte Sektorparameter `?s=1` fuehrt nun auf die Health-Seite. Damit bleiben vorhandene Bookmarks und Openwave-Testvektoren nutzbar.

## Konfiguration

Die bestehenden Schalter werden aus Kompatibilitaetsgruenden weiterverwendet:

```toml
[cell_info.wap_ip]
accept_root_path = true
accept_status_path = true
accept_status_wml_path = true
```

Bedeutung nach der Portal-Erweiterung:

- `accept_root_path`: XHTML-Startseite `/` und `/index.xhtml`
- `accept_status_path`: alle weiteren XHTML-Portalseiten
- `accept_status_wml_path`: alle WML-Portalseiten inklusive `/index.wml`

## Statische Referenzseiten

Unter `contrib/wap-portal/` liegen zusaetzlich 19 vollstaendige XHTML-Basic-Dateien und 19 vollstaendige WML-1.1-Dateien. Diese dienen zum Testen ueber einen normalen HTTP/WAP-Webserver und als gut lesbare Referenz fuer die eingebetteten Kurzseiten.
