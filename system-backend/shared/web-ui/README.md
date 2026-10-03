# NetCore Shared WebUI

Build-freie CSS- und ES-Modul-Bausteine für die Verwaltungsoberflächen der Backend-Dienste.
Die Assets benötigen weder Node.js noch einen zusätzlichen Frontend-Container in Produktion.

## Enthalten

- gemeinsames Layout, Karten, Tabellen, Formfelder und responsive Regeln,
- sichtbarer Open-Lab-Hinweis,
- Status-Badges,
- typisierter JSON-API-Client mit Timeout und Problem-Details-Fehlern,
- Bestätigungsdialoge und Toasts,
- deutsche und englische Basistexte,
- statische Demo unter `demo/index.html`.

## Einbindung

```html
<link rel="stylesheet" href="/assets/netcore.css">
<script type="module">
  import { NetCoreApiClient, statusBadge } from "/assets/netcore.js";
</script>
```

## Gemeinsames Dienstdesign

`assets/service-design.css` und `assets/service-design.js` ergänzen die vorhandenen
Dienstseiten um das helle NetCore-Design. Weiß, Hellblau, Navy und die blaue Akzentfarbe
entsprechen der Basisstation. Das originale PNG wird ohne Neuzeichnung eingebettet;
zwei CSS-Ausschnitte zeigen Symbol und Wortmarke nebeneinander. Ein optionales dunkles
Design bleibt über den Knopf in der Kopfzeile erreichbar.

Hell bleibt ohne gespeicherte Auswahl der Standard. Dunkel ist eine explizite
Browserauswahl und umfasst Kopfzeile, Navigation, Karten, Tabellen, Formulare,
Dialoge, Diagnosetexte, Statusfarben und die gemeinsamen Kartensteuerelemente.
Das unveränderte Original-Logo behält in beiden Modi seine helle Markenfläche.

`assets/service-theme-init.js` wird blockierend am Anfang des `<head>` eingebunden.
Dadurch gilt die gespeicherte Palette schon beim ersten Anzeigen. Die Auswahl liegt
unter `netcore-theme`; der bisherige Schlüssel `netcore-service-theme` dient als
Fallback und wird beim Umschalten ebenfalls aktualisiert. Die dritte Auswahl `blue`
der Basisstation entspricht bei Diensten dem hellen Design. Die Auswahl gilt je
Browser und Web-Origin, also nicht automatisch über unterschiedliche LXC-Adressen
oder Ports hinweg. Ohne verfügbaren Browser-Speicher funktioniert der Schalter
weiter; beim erneuten Laden beginnt die Seite wieder hell.

Beim Wechsel versendet die Shell auf `window` ein `netcore-theme-change`-Ereignis
mit `event.detail.theme` (`light` oder `dark`). Dienstgrafiken können ihre Farben
dann aus den CSS-Variablen lesen und neu zeichnen. Andere Tabs derselben Origin
übernehmen Änderungen über das `storage`-Ereignis.

Die Dienst-WebUIs bleiben fachlich eigenständig. Das Shell-Skript verschiebt vorhandene
Navigationsknoten in die horizontale Kopfzeile; IDs, Listener und Aktionsfunktionen
bleiben erhalten. Einseitige Oberflächen erhalten Anker zu ihren vorhandenen Bereichen.
Breite Tabellen scrollen im eigenen Bereich. Das Shell-Skript führt keine API-Abfragen
aus und ändert keine Anmeldung, Berechtigungen oder Dienstzustände.

### Rust-Dienste

```rust
#[path = "../../shared/web-ui/service-design.rs"]
mod service_design;

// Statt Html(INDEX_HTML); INDEX_HTML enthält die unveränderte Fachoberfläche.
Html(service_design::render(INDEX_HTML, "Subscriber Core", "open-lab"))
```

Der Renderer benötigt nur die Rust-Standardbibliothek. CSS, JavaScript und PNG werden
beim Kompilieren eingebunden und sind nach dem Rollout vollständig im Binary vorhanden.

### Python und statische Seiten

Das Werkzeug ist ausschließlich für die Entwicklung/Generierung vorgesehen:

```sh
python3 tools/embed_service_design.py path/to/index.html --name "Hardware Gateway" --access open-lab
python3 tools/embed_service_design.py path/to/index.html --refresh
```

Importierbar als `render(html, name, access)` und `refresh(html, name=None, access=None)`.
`render` ist idempotent. `refresh` ersetzt nur die markierten Shared-Blöcke und übernimmt
standardmäßig die eingebettete Dienstidentität. Änderungen an den Shared-Assets erfordern
bei statischen/Python-Seiten eine erneute Generierung. Produktionsinstallationen brauchen
das Werkzeug und die Shared-Dateien nicht, wenn die Assets im HTML eingebettet sind.
`--sync-logo` aktualisiert die Data-URI aus dem Original unter
`crates/tetra-entities/src/net_dashboard/ui/netcore-logo.png`.

### Zugangskennzeichnung

| Wert | Kennzeichnung | Bestehender Zugang |
| --- | --- | --- |
| `open-lab` | OPEN LAB | Keine Anmeldung in diesem Dienst |
| `session` | Sitzungszugang | Bestehende Benutzer-/Passwort-Sitzung |
| `access-key` | Zugangsschlüssel | Bestehende Schlüssel-Sitzung |
| `optional-basic` | HTTP-Basic optional | Konfigurierbarer Basic-Zugang |
| `http-basic` | HTTP-Basic | Aktivierter Basic-Zugang |

Die Kennzeichnung beschreibt ausschließlich den bestehenden Zugangsmodus. Sie ist keine
Zugangsprüfung. Benutzerkonten, zentrale Anmeldung und RBAC werden dadurch nicht eingeführt.

### Prüfung

`tests/service-design.mjs` prüft die Shell im Browser: unveränderte DOM-Knoten und
Aktionshandler, funktionierende Bestandsnavigation, Original-PNG, Zugangshinweise,
Theme-Schalter, eindeutige IDs und mobile Tabellen. Ausführung wie bei den Dashboardtests:

```sh
node system-backend/shared/web-ui/tests/service-design.mjs
```

Playwright und ein Chromium-Browser werden nur zum Testen benötigt. Ein vorhandener
Browser kann über `CHROMIUM_EXECUTABLE_PATH` gewählt werden. Die Rust-Modultests prüfen
idempotente Einbindung und eine gegen Script-Abschluss geschützte JSON-Konfiguration.
Die Browserprüfung deckt zusätzlich gespeicherte Auswahl und frühe Theme-Anwendung,
Reload, blockierten Browser-Speicher, die explizite Basisstationsauswahl `blue`,
mehrere Tabs und Textkontrast der gemeinsamen Oberflächen in beiden Modi ab.

Bei Diensten mit `script-src 'self'` (Warnzentrale) bleibt die Content Security Policy
bestehen: Der Initializer wird als blockierende lokale Datei am Anfang des `<head>`
geladen; die Shell als lokale Datei nach den Dienstskripten. Es wird kein ausführbares
Inline-Skript freigeschaltet. Die Workflow-Generierung kopiert diese Dateien mit.
