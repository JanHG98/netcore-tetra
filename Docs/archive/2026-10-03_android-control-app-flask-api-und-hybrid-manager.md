# Abschlussdokumentation: Android-Control-App, Flask-API und Hybrid-Manager

## 1. Metadaten und Geltungsbereich

| Feld | Wert |
|---|---|
| Projekt | NetCore-Tetra |
| Thema | Android-App zur Basisstationsverwaltung; bestehender Flask-Control-Server; JSON-Vertrag; Start/Stop; Statusanzeige; V2-Dateiansicht; Signatur- und `last_version`-Diagnose |
| Ursprünglicher Chattitel | Im zugänglichen Verlauf nicht eindeutig verfügbar; der Dokumenttitel ist eine nachträgliche Themenbeschreibung. |
| Ursprünglicher Chatlink | Nicht verfügbar; kein Link rekonstruiert oder erfunden. |
| Historischer Zeitraum | Nachweisbare Log- und State-Daten vor allem vom 7. Mai 2026; ältere Metadaten vom 28./30. April und 6. Mai. Nicht alle Nachrichten besitzen sichtbare Zeitstempel. |
| Erstellt | 2026-10-03, Europe/Berlin |
| Repository | `JanHG98/netcore-tetra` |
| Geprüfter Zielbranch | `Archiving` |
| Geprüfter Ausgangscommit | `855ec68e4379983b8a59725723d1ad99bc8fcb47` |
| Zugehöriger Root-Tree | `c3c3ddad768ac60a72f5e2138afe4e7fff50f9b2` |
| Änderungsumfang dieses Archivauftrags | Diese Markdown-Datei und der zugehörige Eintrag in `Docs/archive/README.md`; keine App-, Server-, Schlüssel-, Konfigurations- oder Betriebsänderungen. |
| Archivierungsstatus | Technische Übergabe mit ausdrücklich offenen Punkten; keine Erklärung einer vollständig fehlerfreien Anwendung. |

**Wichtigste Übergabeinformation:** Das gepostete Stationsplugin definiert Start/Stop als **`GET /stations/set/<node_id>/<desired_state>`** mit `RUN` oder `STOP`. Die belegte Stationsliste ist ein **nach Node-ID indiziertes Objekt mit `meta`, `state` und `urls`**, keine Liste flacher Stationsobjekte. Diese beiden Verträge dürfen bei einer Fortsetzung nicht erneut geraten oder durch Änderungen am Server passend gemacht werden. [C04, C05]

**Ebenso wichtig:** Für die Meldung `FEHLER: last_version-Datei ist defekt.` ist im zugänglichen Material **keine erfolgreich bestätigte Reparatur mit belegtem Dateipfad und Dienst** enthalten. Frühere Assistenzantworten behaupteten mehrfach, eine bereits bekannte Lösung wiederzugeben, ohne den Nachweis zu liefern. Diese Antworten sind ausdrücklich **keine belastbaren Betriebsanleitungen**. [C08]

### 1.1 Grenzen der Auswertung

Der bereitgestellte Chat enthält einen großen ausgelassenen Anfangsblock und zahlreiche weitere Kennzeichnungen ausgelassener Nachrichten. Viele vollständige Kotlin-Dateien, die im Verlauf angefordert oder offenbar ausgegeben wurden, liegen deshalb nicht als zugänglicher Quelltext vor. Sichtbar sind unter anderem Nutzerkorrekturen, Screenshots, Compiler- und Logcat-Ausgaben, zwei ursprüngliche Kotlin-Ausschnitte, der ursprüngliche Flask-Einstieg und Kontext sowie die vollständigen geposteten Stations- und Generatorplugins.

Zusätzlich wurden tatsächlich vorhandene Laufzeitdateien geprüft: ein älteres Server-Fix-ZIP mit zwei Python-Dateien, dessen bereits entpackte identische Dateien, sowie die verfügbaren Screenshots. Die Dateisuche in den Projektquellen lieferte überwiegend ETSI-Unterlagen, aber keine wiedergewonnenen vollständigen Kotlin-Endfassungen. Eine gezielte Rücksuche lieferte nochmals die Start-/Stop-Route und den Vorschlag zur Redirect-Behandlung, jedoch keine belegte `last_version`-Lösung und keinen Chatlink.

Die Dokumentation trennt daher vier Ebenen: **historischer Nutzernachweis**, **sichtbarer oder als Anhang vorhandener Code**, **heutiger Repository-Befund** und **neue technische Ableitung für die Fortsetzung**. Fehlende historische Inhalte werden nicht mit anderen Projektchats aufgefüllt.

### 1.2 Statusbegriffe

| Status | Bedeutung in dieser Dokumentation |
|---|---|
| Idee | Diskutiert, ohne eindeutigen Umsetzungsauftrag oder Nachweis. |
| Beschlossen/geplant | Vom Nutzer verlangt oder ausdrücklich festgelegt; Umsetzung nicht automatisch belegt. |
| Implementiert | Konkreter zugänglicher Code oder erzeugtes Artefakt vorhanden; Speicherort und Version werden genannt. |
| Getestet | Ein konkreter Test und sein Ergebnis sind sichtbar oder im Archivlauf tatsächlich durchgeführt worden. |
| Im Betrieb bestätigt | Nutzerbeobachtung oder Betriebsbeleg zeigt die Funktion im damaligen Aufbau; kein Nachweis des heutigen Deployments oder einer dauerhaften Fehlerfreiheit. |
| Unbestätigt/widersprochen | Behauptung ohne Nachweis beziehungsweise ausdrücklich korrigierter Ansatz. |

## 2. Ziel, Ausgangslage und Themen

Die Android-Anwendung `NetCoreTetraControl` soll den vorhandenen Control-Server bedienen, Basisstationen darstellen, Start und Stop auslösen sowie Stationsdetails und später Konfigurationsdateien innerhalb der App anzeigen. Der Control-Server verwaltet signierte Sollzustände und Konfigurationen; das BREW-Backend ist eine getrennte Komponente. Der vorhandene Webzugriff funktionierte bereits, während die App wiederholt falsche Modelle, falsche Aktionsrouten oder unzutreffende Statusannahmen verwendete. [C01–C06]

Ausgangsproblem war zunächst eine fest eincodierte Serveradresse. Danach entstanden mehrere aufeinanderfolgende Probleme: DNS-Erreichbarkeit, fehlende Stationslisten trotz grüner Serveranzeige, unpassende JSON-Auswertung, irreführende Aktivitätsfilter, Start-/Stop-404, abgebrochene HTTP-Antworten, Kotlin-/Compose-Buildfehler, ein Runtime-Absturz durch eine nicht implementierte `weight`-Funktion, zu große Kopfbereiche und ein zeitweise funktionsloser Details-Button. Schließlich wurde eine lokale V2-Kopie angelegt und eine rein interne Dateiansicht gewünscht. Am Ende kehrte die Diskussion zur Basisstation mit Signaturwarnung und defekter `last_version`-Datei zurück. [C06–C08]

## 3. Endgültige Anforderungen und Entscheidungen

| Anforderung | Letzte belastbare Festlegung | Status und Begründung |
|---|---|---|
| Serveradresse | In der App einstellbar; Wechsel zwischen lokalem Netz, VPN, Domain und Testserver ohne Neubau | Beschlossen; mehrere Screenshots mit wechselnden Adressen belegen eine vorhandene Serverauswahl. Dauerhafte Speicherung und Verhalten nach Prozessneustart nicht abgenommen. |
| DNS | Control: `CT-H-DEV-04`; BREW: `CT-H-DEV-01`; funktionierender Control-FQDN: `ct-h-dev-04.netcore-tetra.de` | Historisch im Betrieb bestätigt, nachdem der Nutzer die serverseitige BREW-Adresse selbst angepasst hatte. |
| Bestehendes JSON | Die aktuellen JSON-Daten bleiben unverändert | Ausdrückliche Randbedingung; App muss den vorhandenen Vertrag lesen. Frühere Server-Fix-Experimente sind keine Freigabe für spätere API-Umbauten. |
| Start/Stop | In der App reparieren; vorhandene funktionierende WebUI-Route verwenden | Ausdrücklich beschlossen; keine neue Serverroute, kein weiteres GET-/POST-Raten. |
| Statusfilter | Erreichbarkeit und tatsächlich aktive Sender nicht mit Soll-RUN oder bloßer Registrierung verwechseln | Doppel-Filter diskutiert/beauftragt; spätere Korrektur ersetzt „angemeldet“ durch „aktiv senden“. Korrekte Live-Datenversorgung nicht belegt. |
| Darstellung | Kopfbereich und Filter kompakter, ohne unleserlich kleine Bedienung; genügend Fläche für Stationsliste | Beschlossen; kompaktere Versionen sichtbar, aber wiederkehrende Kürzungen und Layoutregressionen. |
| Details | Details-Button muss einen tatsächlichen Dialog öffnen | Zunächst defekt; später durch Screenshot eines geöffneten Dialogs im Betrieb bestätigt. |
| Dateiansicht | Klick etwa auf `config.toml` öffnet eine Ansicht direkt in der App; kein externer Link, Browser oder Downloadablauf | Beschlossen/geplant für V2; finale Implementierung und Test nicht zugänglich. |
| V1/V2 | Bestehenden Stand sichern und separat weiterentwickeln | Nutzer bestätigte Kopie des Projektordners mit angehängtem `_V2`; kein belegter Git-Tag oder Entwicklungsbranch. |
| Lieferform bei weiterer Arbeit | Ganze betroffene Dateien, einschließlich Imports, Package, Models und tatsächlich verwendeter UI-Funktionen; keine Platzhalter, Dummys oder leeren Handler | Wiederholt ausdrücklich festgelegt. Keine ZIP-Pflicht und keine bloßen Austauschfragmente. Erläuterungen formatiert, nicht als ein unstrukturierter Gesamtblock. |
| Betriebsreparatur | Bereits funktionierende Lösung zu `last_version` wiederfinden statt neue Pfade/Dienste erfinden | Beschlossenes Anliegen, im zugänglichen Verlauf nicht erfüllt. |

Die historischen Farbvarianten wechselten von Gelb/Schwarz zu Dunkelblau/Cyan. Daraus ist keine abschließend freigegebene Farb- oder Theme-Spezifikation abzuleiten. Die dauerhafte Anforderung betrifft vor allem Lesbarkeit, Bedienbarkeit und den Erhalt funktionierender Abläufe. [C06]

## 4. Historische Architektur

### 4.1 Komponenten und Verantwortlichkeiten

| Komponente | Aufgabe | Beleg/Abgrenzung |
|---|---|---|
| Android-App | Stationsliste und Details anzeigen; Serveradresse auswählen; RUN/STOP anfordern; später Dateiinhalte intern anzeigen | Kotlin, Jetpack Compose, Retrofit/OkHttp/Gson in sichtbaren Imports und Logs. Keine vollständige letzte App-Quelle verfügbar. |
| Flask-Control-Server | Stationsregister, Sollzustände, Konfigurationsdateien, WebUI und JSON-Zugriff | Original-`app.py`, `control_context.py`, Stationsplugin und Generatorplugin. |
| BREW-Backend | Separater Backenddienst mit eigenem Health-Endpunkt und eigenen Zählern | `CT-H-DEV-01.netcore-tetra.de:8081`; nicht mit Control-Server oder Stations-Health gleichsetzen. |
| Stations-Health-Agent | Auf der Basisstation Dienst-, Prozess-, Konfigurations- und Signaturstatus liefern | Vom Control-Server unter Port `8088`, Pfad `/health`, abgefragt; vollständiger Agentcode/Unit fehlt. |
| `bluestation_hybrid_manager.sh` | Lokaler Manager auf der TBS; meldet den Fehler der Versionsdatei | Im Nutzerlog direkt nachgewiesen; Installationspfad, vollständiger Code und ausführende Unit unbekannt. |
| BlueStation-Sender | Tatsächlicher Funkprozess bzw. dessen Betrieb | Nur mittelbare Health-/Prozessindikatoren vorhanden; kein unabhängiger HF-Nachweis aus dem App-Chat. |

Die belegten Verbindungen sind: App → Control-HTTP; Browser → Control-WebUI; Control → BREW-Health; Control → DNS/Ping/Stations-Health. Die App-Anzeige darf nicht allein aus dem Erfolg des ersten Weges ableiten, dass alle übrigen Komponenten online sind.

### 4.2 Flask-Einstieg und Plugin-Laden

Der ursprüngliche Einstieg erzeugt `app = Flask(__name__)`, ermittelt `BASE_DIR` aus dem Dateipfad und erstellt `ControlContext(BASE_DIR)`. Aus `BASE_DIR / "plugins"` werden lexikografisch sortierte `*_plugin.py` geladen. Jedes gültige Modul benötigt `PLUGIN`-Metadaten und `register(app, ctx)`. Fehler beim Laden werden per Traceback ausgegeben; die übrigen Plugins werden weiter verarbeitet. `ctx.plugins` dient anschließend unter anderem der Navigation. [C02]

Die sichtbaren Plugins sind `10_stations_plugin.py` und `20_generator_plugin.py`. Der ursprüngliche Einstieg stellt `/`, `/dashboard` und `/health` bereit. `/` leitet zum Dashboard weiter. Dashboard und Stations-WebUI enthalten einen HTML-Meta-Refresh mit 15 Sekunden. Der Entwicklungsstart im Quelltext lautet `app.run(host="0.0.0.0", port=8080)`; daraus folgt kein Nachweis einer konkreten produktiven WSGI-/systemd-Konfiguration.

### 4.3 Datenablage auf dem Control-Server

Der Terminalprompt belegt `/opt/netcore-tetra-control/plugins`. Zusammen mit dem relativen Layout des Kontexts ergeben sich folgende serverseitige Pfade. Schlüsselpfade dienen nur der Architekturzuordnung; Schlüsselmaterial wird nicht archiviert. [C03, C05]

| Pfad | Inhalt/Zweck |
|---|---|
| `/opt/netcore-tetra-control/app.py` | Flask-Einstieg, aus dem geposteten Aufbau abgeleiteter Installationspfad |
| `/opt/netcore-tetra-control/control_context.py` | Gemeinsame Verwaltung, Signieren, Health und Layout |
| `/opt/netcore-tetra-control/plugins/10_stations_plugin.py` | Im Terminal tatsächlich ausgegebener Stationscode |
| `/opt/netcore-tetra-control/plugins/20_generator_plugin.py` | Im Terminal tatsächlich ausgegebener Generatorcode |
| `/opt/netcore-tetra-control/nodes.json` | Registry mit Node-ID als Schlüssel und Metadaten als Wert |
| `/opt/netcore-tetra-control/data/<node_id>/state.json` | Signierter Sollzustand als JSON |
| `/opt/netcore-tetra-control/data/<node_id>/state.json.sig` | Base64-kodierte State-Signatur |
| `/opt/netcore-tetra-control/static/configs/<node_id>/config.toml` | Aktive Konfiguration am Control-Server |
| Gleicher Ordner: `config.toml.sig` | Zugehörige aktive Konfigurationssignatur |
| Gleicher Ordner: `config.toml.pending` | Noch nicht signierter/aktivierter Entwurf |
| `/opt/netcore-tetra-control/keys/state_private_key.pem` | State-Signierschlüssel; Inhalt nicht enthalten |
| `/opt/netcore-tetra-control/templates/generator.html` | Vom Generatorplugin erwartete HTML-Basisdatei |

**Nicht verwechseln:** Die WebUI zeigt für TBS01 in einem zugänglichen Screenshot als tatsächlich geladene Konfiguration **`/run/bluestation/config.toml`** an. Dieser Pfad wurde bei der Archivierung im Bildausschnitt gelesen. Er widerspricht einer ungeprüften Gleichsetzung mit `config.toml` im Home-Projektordner, verrät aber **nicht** den Pfad von `last_version`. [A02]

### 4.4 State- und Konfigurationsversionierung

`ensure_node_files()` legt für bekannte Nodes fehlende State-/Config-Verzeichnisse an und erzeugt gegebenenfalls einen initialen State mit `desired_state = STOP`, Version `1`, Device-ID, Lock-Wert und UTC-Zeitstempel. `load_state()` ruft diese Vorbereitung auf; ein HTTP-GET auf die Registry ist deshalb im Quelltext nicht unter allen Umständen dateisystemseitig schreibfrei. [C03, C05]

`save_and_sign_state()` normalisiert den Wunsch auf Großbuchstaben, akzeptiert nur `RUN` oder `STOP` und erhöht die State-Version um eins. Bei einer gesperrten Node wird der Wunsch zu `STOP` korrigiert. `set_locked()` aktualisiert die Registry und erzeugt ebenfalls eine neue signierte State-Version; Sperren erzwingt STOP, Entsperren behält den bisherigen Sollzustand. Ein erneuter identischer Start-/Stop-Aufruf kann daher trotzdem eine neue Version erzeugen.

Der State wird als formatiertes JSON mit sortierten Schlüsseln und abschließendem Zeilenumbruch geschrieben. Anschließend signiert der Code die exakten Dateibytes mit einem als Ed25519 geprüften privaten Schlüssel und speichert die Signatur Base64-kodiert mit Zeilenumbruch. Die Implementierung des getrennten `config_signer.py` wurde nicht bereitgestellt: Das kryptographische Detail der Konfigurationssignatur darf nicht allein aus dem State-Signiercode abgeleitet werden.

Die nächste **Konfigurationsversion** wird dagegen aus der Kommentarzeile `# CONFIG_VERSION=` der aktiven `config.toml` gelesen und erhöht; fehlt die Datei oder scheitert die Ermittlung, liefert der sichtbare Code `1`. **State-Version, Config-Version und lokaler `last_version`-Merker sind nicht ohne Prüfung dasselbe.** Die frühere Assistenz setzte deren Bedeutung teilweise ungeprüft gleich. [C03, C08]

## 5. Verbindlicher historischer HTTP-/JSON-Vertrag

### 5.1 Hosts, Ports und beobachtete Adressen

| Verbindung | Adresse/Parameter | Nachweisgrenze |
|---|---|---|
| App → Control | `http://ct-h-dev-04.netcore-tetra.de:8080` | Historisch funktionierende vollständige DNS-Adresse in App und Nutzerprüfung |
| Control-Kurzname | `http://ct-h-dev-04:8080` | Ein App-Screenshot zeigt Offline; später liefern beide `/api/nodes`-Adressen laut Nutzer dasselbe JSON. Clientkontexte nicht gleichsetzen. |
| Frühere Control-IP | `10.0.1.186:8080` | Im Ausgangswunsch/festen Codekontext genannt; historisch, nicht heutige Konfiguration |
| Weitere beobachtete Control-IP | `10.0.1.31:8080` | In App-Screenshot erreichbar; keine abschließende IP-Zuordnung aus der Chronologie erzwingen |
| Control → BREW | `http://CT-H-DEV-01.netcore-tetra.de:8081/health` | Im ursprünglichen Kontext und im Fix-Artefakt gleich hinterlegt |
| BREW-WebUI | `http://CT-H-DEV-01.netcore-tetra.de:8081/` | Verlinkt im Dashboard |
| Control → Stations-Health | `http://<aufgeloester-stationshost>:8088/health` | Im geposteten Kontext implementiert |
| TBS01 / TBS02 | `SRV-M-RPi-TBS01` / `SRV-M-RPi-TBS02`; Screenshot-IP `10.0.1.20` / `10.0.1.21` | Historische WebUI-Beobachtungen, kein aktueller Netzscan |
| Stations-DNS-Fallback | `.netcore-tetra.de` | Wird bei Node-IDs ohne Punkt als zweiter Auflösungskandidat verwendet |

Alle genannten Webverbindungen verwenden im historischen Material HTTP. Daraus folgt keine Freigabe zum öffentlichen Exponieren der Verwaltungsrouten. VPN-/Domain-Wechsel ist ein App-Wunsch, kein in diesem Chat nachgewiesenes VPN- oder TLS-Deployment.

### 5.2 Registry: `GET /api/nodes`

Der Nutzer prüfte sowohl die Kurznamens- als auch die FQDN-Adresse. Beide lieferten ein Objekt mit fünf Schlüsseln `SRV-M-RPi-TBS01` bis `SRV-M-RPi-TBS05`. Jeder Wert enthält `meta`, `state` und `urls`. Der exakt passende Rückgabetyp ist auf Strukturebene **eine Map von String auf ControlNode**. Der sichtbare Serververtrag verlangt keinen äußeren Wrapper `stations` oder `nodes`. [C04, C05]

| JSON-Pfad je Node | Typ/Inhalt | Verwendung |
|---|---|---|
| `meta.name` | String | Anzeigename |
| `meta.device_id` | String | Gerätezuordnung |
| `meta.locked` | Boolean | Administrative Sperre |
| `meta.created_at`, `meta.updated_at` | ISO-8601-Strings | Registry-Zeitpunkte |
| `state.desired_state` | String `RUN` oder `STOP` | Sollzustand, nicht tatsächliche Funkaktivität |
| `state.device_id` | String | Device-ID des signierten States |
| `state.locked` | Boolean | Sperrstatus im State |
| `state.version` | Ganzzahl | State-Version |
| `state.updated_at` | ISO-8601-String | Zeitpunkt der State-Änderung |
| `urls.state`, `urls.state_sig` | Relative URL | State und Signatur |
| `urls.config`, `urls.config_sig` | Relative URL | Aktive Konfiguration und Signatur |
| `urls.pending_config` | Relative URL | Pending-Konfiguration; ihre Existenz wird damit nicht garantiert |

Das belegte JSON enthält **kein** `reachable`, `sender_running`, `health`, `bluestation_service`, `signature_ok` oder `registered`. Ein Defaultwert `false` für nicht gelieferte Telemetrie wäre keine Messung. Auch der erfolgreiche Abruf einer zentral gespeicherten `state.json` belegt nicht, dass der betreffende Raspberry Pi erreichbar ist.

Die fünf State-Versionen in einem vollständig geposteten Snapshot waren `76`, `7`, `1`, `1`, `3`; alle Sollzustände waren dort STOP. Andere Screenshots zeigen zu anderen Zeitpunkten TBS01 unter anderem mit `83`, `84`, `86` und `97`. Diese Werte dokumentieren verschiedene Beobachtungen, keine unveränderlichen Sollwerte und keine lückenlose Ereigniskette. [C04, C06]

### 5.3 Health: drei getrennte Aussagen

Der ursprüngliche Control-Endpunkt `/health` enthält `status: ok`, den Servicenamen `netcore-tetra-control`, Plugininformationen, die Anzahl registrierter Nodes, den verschachtelten BREW-Backendstatus und einen Zeitstempel. `status: ok` bedeutet dort nicht automatisch, dass BREW oder alle Stationen online sind. [C02, C03]

BREW meldet nach dem DNS-Fix beispielsweise `Nodes: 8 | erlaubte ISSIs: 4 | aktive Verbindungen: 0`, während die Control-Registry fünf Basisstationen enthält. Diese Zähler stammen aus verschiedenen Datenquellen; acht BREW-Nodes sind nicht acht Einträge der App-Registry. Der sichtbare Control-Code liest BREW-Felder `nodes_count`, `allowed_issis_count` und `active_connections_count` nur zur Textdarstellung aus.

Die Erreichbarkeitsprüfung einer Station arbeitet im ursprünglichen Kontext in dieser Reihenfolge: DNS-Auflösung, Ping, Health-HTTP. Verwendet werden `ping -c 1 -W 1`, ein Subprozesslimit von zwei Sekunden und anschließend ein HTTP-Timeout von 1,8 Sekunden. Für BREW gilt ein Timeout von zwei Sekunden. Beim Stations-Health werden maximal 16.384 Bytes gelesen. [C03; A01, `control_context.py`:285–408]

Bei fehlgeschlagener Auflösung oder fehlgeschlagenem Ping wird `offline` geliefert. Bei erfolgreichem Ping und fehlendem/degradiertem Agent wird `warning` geliefert. Nur `health.ok is True` führt in dieser Funktion zu `online`. Der Code prüft dabei nicht als zusätzliche Voraussetzung, ob jede Einzelbewertung für Signatur und Konfiguration positiv ist.

Die verwendeten Health-Daten sind `device_id`, `bluestation_service`, `status`, `last_config_version`, `loaded_config.path`, `loaded_config.config_version`, `signature.ok`, `sender_process.running`, `summary.config_found` und `summary.service_ok`. Die WebUI interpretiert `sender_process.running` als **„Sender erkannt“**, nicht als unabhängig gemessene HF-Abstrahlung. Fehlende Werte werden in mehreren Zweigen mit negativen oder unklaren Anzeigen zusammengefasst.

**Folgerung für die App, nicht bereits nachgewiesene Umsetzung:** Sollzustand, Dienstzustand, Erreichbarkeit, erkannter Senderprozess und tatsächliche Sendetelemetrie benötigen getrennte Anzeigen. „Unbekannt“ darf nicht stillschweigend „offline“ oder „sendet nicht“ werden. Der gewünschte Filter für aktiv sendende Stationen benötigt eine belegte Datenquelle zusätzlich zum unveränderten Registry-JSON.

### 5.4 Start/Stop: einzig belegte Aktionsroute

| Aktion | Methode | Pfad | Antwort des geposteten Plugins |
|---|---|---|---|
| Start | GET | `/stations/set/SRV-M-RPi-TBS01/RUN` | State schreiben/signieren, danach Redirect nach `/stations` |
| Stop | GET | `/stations/set/SRV-M-RPi-TBS01/STOP` | State schreiben/signieren, danach Redirect nach `/stations` |
| Andere Node | GET | `/stations/set/<node_id>/<desired_state>` | Node muss in der Registry vorhanden sein |

Die maßgebliche Funktion lautet `stations_set_state`. Der Handler prüft die Node, ruft `ctx.save_and_sign_state()` auf und erst danach `redirect(url_for("stations_index"))`. Die WebUI erzeugt ihre Start-/Stop-Links über genau diesen Handler. [C05]

Die früheren App-Pfade `/control/<node>/stop`, `/state/<node>/run`, `/api/nodes/<node>/state/RUN` und `/api/nodes/<node>/state` waren nicht durch dieses Plugin gedeckt. GET-/POST-/Formularvarianten gegen diese geratenen Pfade konnten die fehlende Route nicht beheben.

**Wichtige Ergebnisgrenze:** Ein erfolgreich gespeicherter Sollzustand bestätigt zunächst eine Änderung am Control-Server, nicht die Ausführung am Funkprozess. Zusätzlich kann der Lock-Mechanismus einen RUN-Wunsch in STOP umwandeln. Eine korrekte App muss deshalb das Ergebnis nachlesen und zwischen angefordertem, gespeichertem und tatsächlich beobachtetem Zustand unterscheiden.

### 5.5 Weitere belegte Stations- und Dateirouten

| Methode | Pfad | Aufgabe |
|---|---|---|
| GET | `/stations` | WebUI der Stationen |
| POST | `/stations/node/create` | Node anlegen: Formularfelder `node_id`, `name`, `device_id` |
| POST | `/stations/node/<node_id>/edit` | Name/Device-ID bearbeiten und State-Version erhöhen |
| POST | `/stations/node/<node_id>/delete` | Node und zugehörige State-/Config-Verzeichnisse entfernen |
| GET | `/stations/node/<node_id>/lock` | Sperren und STOP erzwingen |
| GET | `/stations/node/<node_id>/unlock` | Entsperren |
| POST | `/stations/node/<node_id>/upload-config` | Multipart-Upload aktiver Config/Signatur |
| POST | `/stations/node/<node_id>/sign-pending` | Pending validieren/signieren/aktivieren; Formularfeld `passphrase` |
| POST | `/stations/node/<node_id>/discard-pending` | Pending verwerfen |
| GET | `/state/<node_id>/state.json` | State-Datei ausliefern |
| GET | `/state/<node_id>/state.json.sig` | State-Signatur ausliefern |
| GET | `/configs/<node_id>/config.toml` | Aktive Config ausliefern |
| GET | `/configs/<node_id>/config.toml.sig` | Aktive Config-Signatur ausliefern |
| GET | `/configs/<node_id>/config.toml.pending` | Pending ausliefern, sofern vorhanden |

Node-IDs bei Neuanlage werden gegen `^[a-zA-Z0-9._-]+$` geprüft. Die Dateiauslieferung besitzt feste Dateinamen-Whitelists. Der Upload akzeptiert exakt `config.toml` bis 1 MiB und `config.toml.sig` bis 4096 Bytes; leere Dateien und andere Namen werden abgelehnt. Config und Signatur können getrennt hochgeladen werden. Im sichtbaren Upload-Handler ist keine Prüfung des vollständigen kryptographischen Dateipaars vor dem direkten Schreiben enthalten. Das ist ein Befund am geposteten Code, kein Nachweis einer konkret dadurch verursachten Störung. [C05]

## 6. Config-Generator und Signierablauf

`20_generator_plugin.py` lädt eine vorhandene `templates/generator.html`, entfernt bestimmte Cloudflare-Skriptinjektionen und fügt Stationsmetadaten sowie eine Schaltfläche „Als Pending speichern“ ein. Die eigentliche vollständige Generator-HTML-Datei und `config_signer.py` sind im zugänglichen Material nicht vorhanden. [C05]

Der Generator ergänzt `TBS_ID`, `CONFIG_VERSION` und einen Stationskommentar als Kopfkommentare der TOML-Datei. `TBS_ID` soll laut Hilfetext exakt zur `/etc/bluestation/device_id` der Basisstation passen. Die Felder `tbsId`, `configVersionMeta` und `stationComment` lösen bei `input`, `change` und `keyup` eine Neugenerierung aus. URL-Parameter `node`, `version` und `name` können die Werte setzen.

`GET /generator` zeigt die Auswahl. `GET /generator/<node_id>` ermittelt den nächsten Config-Zähler und patcht die HTML-Vorlage. `POST /generator/<node_id>/save-pending` erwartet JSON mit einem nichtleeren String `config`, prüft die Node und die Maximalgröße von 1 MiB und schreibt `config.toml.pending`. Es wird noch keine Signatur dadurch erzeugt. Anschließend soll die WebUI-Route zum Signieren/Aktivieren verwendet werden; sie ruft `sign_and_activate_pending(ctx, node_id, passphrase, client_ip=...)` auf und zeigt eine Erfolgs- oder Fehlermeldung. Zugangsdaten oder Passphrasen sind hier nicht enthalten.

**Neu erkannte Prüfkandidaten aus dem sichtbaren Code, nicht historische Fehlernachweise:** Das HTML-Patching hängt von exakten Zeichenketten der Vorlage ab. Außerdem wird an `patch_generator()` als `node_id` die `device_id` übergeben; dieselbe Variable landet im Save-Pending-URL-Pfad. Wenn Registry-Node-ID und Device-ID künftig voneinander abweichen, muss diese Zuordnung getestet werden. Bei den geposteten Beispielnodes waren beide gleich. [C05]

## 7. Android-Projekt, Modelle und UI

### 7.1 Bekannte Dateien und Abhängigkeiten

Der lokale Projektpfad in den Compilerfehlern lautet `C:/Users/janho/AndroidStudioProjects/NetCoreTetraControl/`. Das Anwendungspackage ist `de.netcore.tetra.mobile`. Die Java-/Kotlin-Quellwurzel wird als `app/src/main/java/` gezeigt. [C06]

| Datei/Symbol | Sichtbarer Stand |
|---|---|
| `de/netcore/tetra/mobile/MainActivity.kt` | Oft vollständig angefordert; mehrere inkompatible Zwischenstände durch Logs belegt; keine vollständige finale Kotlin-Datei zugänglich |
| `api/ApiService.kt` | Früher Nutzerstand mit dynamischem `@Url` und `getHealth`, `getStationState`, `getText`, `getAction`; spätere vollständige Endfassung fehlt |
| `api/RetrofitClient.kt` | Früher vollständiger Nutzerstand: feste `BASE_URL`, OkHttp mit BODY-Logging, Scalars-Converter vor Gson-Converter, lazy `ApiService` |
| `models/ControlNode.kt` | Für Map-Werte mit `meta`, `state`, `urls` benötigt; wiederholte Redeclarations zeigen inkonsistente Modellverteilung |
| `models/HealthResponse`, `models/StationState` | Bereits vorhandene Modelle, deren erneute Definition später Compilerkonflikte erzeugte |
| `LoadedConfig`, `SignatureInfo`, `SenderProcess`, `HealthSummary` | Ebenfalls mehrfach deklariert; endgültige Dateiaufteilung nicht belegt |
| `NetCoreTetraTheme`, `NetCoreControlScreen` | In einer Zwischenfassung referenziert, aber nicht auflösbar |
| `StationListPanel`, Compose `Scaffold` | Im Stacktrace der UI-Komposition nachgewiesen |

Die belegten Bibliotheken sind AndroidX Activity/Compose, Material 3, Kotlin-Coroutines in Suspend-Schnittstellen, Retrofit 2, OkHttp, `HttpLoggingInterceptor`, `ScalarsConverterFactory` und `GsonConverterFactory`. Genaue Gradle-, Kotlin-, Compose-, AGP-, SDK- und Bibliotheksversionen sind nicht vollständig überliefert; keine Versionskombination wird nachträglich als damaliger Buildstand behauptet.

Der frühe Retrofit-Client enthält `BASE_URL = "http://ct-h-dev-04:8080/"`. Die Reihenfolge Scalars vor Gson passt zur gleichzeitig angebotenen Abfrage von Rohtext und JSON. Das Vorhandensein dieses Ausschnitts belegt aber nicht, dass die letzte App-Version denselben Client oder dieselbe Redirect-/Timeout-Konfiguration verwendet.

### 7.2 Bedeutung der Filter

Die Filter entwickelten sich von „Inaktive Stationen anzeigen“ über RUN-/STOP-Auswahl zu „nur erreichbare“ und „nur aktiv sendende“. Der Nutzer widersprach ausdrücklich der Gleichsetzung von „aktiv“ mit Soll-RUN und ersetzte später „angemeldet“ durch aktive Sender.

Mehrere sichtbare Zwischenstände waren semantisch nicht abgenommen: „Nur RUN“ und „STOP anzeigen“ sind keine zwei unabhängigen Achsen für Erreichbarkeit und Senden; fünf bekannte Nodes sind keine fünf aktiven Anmeldungen; ein fehlendes Telemetriefeld beweist keinen ausgeschalteten Sender. Ein späterer Screenshot zeigt fünf erreichbare Stationen, während ein früherer WebUI-/App-Vergleich nur zwei erreichbare Stationen zeigt. Ohne zugänglichen endgültigen Auswertecode ist daraus weder ein realer Ausfall noch eine tatsächlich vollständige Erreichbarkeit abzuleiten. [C04, C06]

### 7.3 Layout und Details

Zu große Server-/Filterkarten ließen zu wenig Höhe zum Scrollen. Andere Versionen quetschten den Titel auf schmale Zeilen oder kürzten Stationsnamen stark. Ein zwischenzeitlicher Versuch mit einer eigenen `weight()`-Funktion führte sogar zu einem Laufzeitabsturz. Der Auftrag war eine kompakte, aber weiterhin ausreichend große Bedienoberfläche, nicht bloß kleinere Schrift.

Der Details-Button war zunächst wirkungslos. Ein späterer Screenshot zeigt einen funktionierenden Dialog für TBS01 mit Node-ID, Name, Device-ID, Sollzustand STOP, Version 97, Lock „nein“, State-/Meta-Zeitpunkten sowie State-, Config-, Config-Signatur- und Pending-URLs. Damit ist der **Detaildialog** historisch bestätigt. Eine funktionierende Dateiinhaltsansicht ist dadurch noch nicht bestätigt. [C06, C07; A02]

### 7.4 V2 und interne Dateiansicht

Der Nutzer kopierte den bestehenden Projektordner und hängte `_V2` an. Diese Sicherung ist eine ausdrücklich bestätigte lokale Aktion; eine echte Git-Baseline, ein Versions-Tag, ein separater Application-ID-Suffix oder eine parallele Installation auf dem Gerät wurden nicht nachgewiesen.

Für V2 soll ein Klick etwa auf `urls.config` die Datei in einer eigenen App-Ansicht öffnen. Verbindlich sind der Verbleib in der App und der Verzicht auf externen Browser oder Downloadablauf. Die Anwendung muss dazu zwar HTTP-Inhalte abrufen, darf daraus aber keinen externen Dateihandler-/Download-Workflow machen.

**Fortsetzungsentwurf, nicht als implementiert markiert:** Relative URLs gegen die ausgewählte Control-Adresse auflösen; Text mit Lade-/Fehlerzustand und scrollbar lesbarer Darstellung anzeigen; Zurück/Schließen vorsehen; HTTP-404 bei fehlender Pending-Datei sachlich darstellen; keinen TOML-Inhalt als HTML ausführen. View-only ist der belegte Auftrag, keine implizite Freigabe zum Bearbeiten, Hochladen oder Signieren.

## 8. Fehlerchronik und belastbare Diagnose

### 8.1 DNS und getrennte Backend-Verbindungen

Anfangs war der Control-Server über eine IP erreichbar, während `BREW Backend nicht erreichbar · offline · <urlopen error timed out>` angezeigt wurde. Der Kurzname des Control-Servers erschien in einer App-Aufnahme offline; der vollständige Name erreichte den Control-Server. Nachdem der Nutzer auf dem Control-Server die IP durch den BREW-DNS-Namen ersetzt hatte, meldeten WebUI und App das Backend wieder online. [C01, C06]

**Belegt gelöst:** Der damalige Backendzugriff funktionierte nach dieser Nutzeränderung wieder. **Nicht belegt:** DNS-Suchsuffix-/VPN-/Android-Resolverkonfiguration im Detail. Die spätere Nutzerprüfung beider `/api/nodes`-Namen erfolgte nicht nachweislich unter denselben Netzwerkbedingungen wie der frühere App-Fehler.

### 8.2 Online-Anzeige ohne Stationen und falsche Modellierung

Die WebUI zeigte Basisstationen, während die App trotz „Online“ null Stationen meldete. Andere Zwischenstände zeigten fünf Nodes, aber Roh-JSON im Feld „Soll“, `Service inactive`, falsche Offline-/Signaturanzeigen oder den Hinweis „Aus WebUI gelesen“. [C04, C06]

**Belegte Ursache auf Vertragsebene:** Die Stationsdaten liegen verschachtelt in einer Node-ID-Map. Einzelne Felder, ganze State-Objekte und HTML-Ausweichpfade wurden in Zwischenversionen erkennbar nicht konsistent behandelt. **Grenze:** Ohne alle Kotlin-Versionen lässt sich nicht jeder Screenshot exakt einem Parserzweig zuordnen. Ein grün dargestellter Health-Abruf ist kein Parser- oder Listen-Abnahmetest.

### 8.3 Start/Stop mit HTTP 404

Nacheinander sichtbare Fehler betrafen unter anderem:

- HTTP 404 bei `/control/SRV-M-RPi-TBS01/stop`.
- HTTP 404 bei GET `state/SRV-M-RPi-TBS01/run`.
- HTTP 404 bei GET und später POST `/api/nodes/SRV-M-RPi-TBS01/state/RUN`.
- HTTP 404 bei POST FORM `/api/nodes/SRV-M-RPi-TBS01/state` mit RUN als Formularinhalt.

Erst der vollständig gepostete Stationscode stellt den Vertrag eindeutig klar: `/stations/set/<node_id>/<desired_state>`. Änderungen am Server wurden mehrfach ausdrücklich abgelehnt. Die falschen Endpunkte sind verworfene App-Ansätze und dürfen nicht in eine zukünftige Fehlersuch-Fallbackliste übernommen werden. [C05, C06]

### 8.4 `unexpected end of stream`: Wirkung und Rückmeldung getrennt

Der Fehler trat beim Laden der Stationsliste sowie nach Start/Stop auf. Der Nutzer beobachtete nach einem solchen Aktionsfehler, dass manuelles Aktualisieren den scheinbar bereits angenommenen neuen Zustand zeigte; Stop verhielt sich entsprechend. Später erschien eine RUN-Erfolgsmeldung gleichzeitig mit einer gescheiterten Listenaktualisierung. [C06]

**Direkt aus dem Plugin ableitbar:** Schreiben/Signieren findet vor dem Redirect statt. Ein Fehler beim anschließenden Abruf oder Lesen der Antwort kann deshalb auftreten, obwohl der Sollzustand bereits geändert wurde. Der Redirect führt zur aufwendigeren `/stations`-WebUI mit seriellen Health-Prüfungen.

**Historisch vorgeschlagener App-Fix:** Aktionsaufruf als direkter OkHttp-GET ohne automatisches Folgen des Redirects, anschließend separater Refresh; Detailsdialog ergänzen. Diese Absicht ist aus der Rücksuche verfügbar. Die vollständige letzte Implementierung fehlt und der spätere Nutzerbericht zeigt weiterhin sporadische Lesefehler. Daher keine Behauptung einer abschließenden Fehlerbehebung.

**Nicht bewiesene eigentliche Stream-Ursache:** Abbruchstelle in HTTP-Headern oder Body, Verbindungspool-Wiederverwendung, Proxy, Timeout, Längen-/Encodingproblem oder Parserfehler lassen sich aus der knappen UI-Meldung allein nicht unterscheiden. Es fehlen die vollständige Exception-Kette, Request-/Response-Header und korrelierte Serverlogs. Ein pauschales „Gson ist schuld“ oder „mehr Timeout löst es“ wäre nicht belegt.

**Offener App-Fix:** Letzte erfolgreich geladene Stationsliste bei einem temporären Fehler erhalten und als veraltet markieren, nicht durch null Stationen ersetzen. Schreibwirkung, Transportergebnis und Refresh getrennt behandeln. Einen möglicherweise bereits wirksamen Aktionsaufruf nicht blind wiederholen: Der gezeigte Server erhöht dabei jeweils die State-Version.

### 8.5 Kotlin-/Compose-Fehler

| Fehlermeldung | Sichtbarer Befund | Richtige Einordnung für die Fortsetzung |
|---|---|---|
| `Missing return statement` in `MainActivity.kt:276:1` | Kompilerfehler | Betroffene vollständige Funktion prüfen; keine belegte finale Reparaturfassung vorhanden. |
| `ClassNotFoundException: de.netcore.tetra.mobile.MainActivity` | App scheitert bereits bei Activity-Instanziierung | Klasse, Package, Manifest und APK-Inhalt zusammen prüfen; kein DNS-/API-Fehler. Spätere startende Screenshots zeigen nur, dass dieser konkrete Zustand nicht dauerhaft bestand. |
| `Unresolved reference 'DarkColorScheme'` | Falscher/nicht vorhandener Symbolbezug | Material-3-Funktions-/Importbezug prüfen; die historische letzte Korrekturdatei fehlt. |
| `Unresolved reference 'NetCoreTetraTheme'` und `'NetCoreControlScreen'` | Nicht vollständig mitgelieferte oder falsch benannte Symbole | Vollständige UI-/Theme-Definitionen samt Imports und Aufrufstellen zusammenhalten. |
| `Redeclaration` bei `HealthResponse`, `StationState` | Mehrfachdefinition im Model-Package | Bereits vorhandene Klassen nicht erneut in `ControlNode.kt` definieren. |
| `Redeclaration` bei `LoadedConfig`, `SignatureInfo`, `SenderProcess`, `HealthSummary` | Erneute Modellkollisionen | Eindeutige Dateiverantwortung herstellen; vollständigen Modellbestand prüfen. |
| `NotImplementedError: Provide the return value` | Stacktrace: `MainActivityKt.weight(MainActivity.kt:523)` → `StationListPanel(...:442)` | Eine eigene nicht implementierte `weight()` wurde zur Laufzeit aufgerufen. Keine TODO-Ersatzfunktion für Compose-Layout-APIs anlegen; Layout-Scope und Modifier korrekt verwenden. |

Logcat-Meldungen zu Debugger-Wartezustand, Emulatorgrafik, Play-Store-RPC und Eingabekanälen wurden ebenfalls gepostet. Die entscheidenden Absturzstellen waren jedoch ausdrücklich die fehlende Activity-Klasse beziehungsweise der `NotImplementedError`, nicht diese Begleitmeldungen. [C06]

### 8.6 Signaturwarnung

Die Anzeige „Signatur nicht OK / fehlt“ stammt in der WebUI aus einer zusammenfassenden Bewertung von `signature.ok`; im Kontext wird nur der exakte Booleanwert `True` als erfolgreich gewertet. Die sichtbare Oberfläche kann daher zwischen fehlender Bewertung, fehlender Signatur und tatsächlichem Validierungsfehler nicht zuverlässig unterscheiden. In einer App mit unpassendem Datenmodell kann zusätzlich eine falsch interpretierte oder nicht gelieferte Bewertung angezeigt werden. [C03, C05]

Die frühere Assistenzbehauptung, die Warnung beweise zwingend ein ungültiges lokales `config.toml`-/`.sig`-Paar, war zu weitgehend. Ebenso unbestätigt sind die vorgeschlagenen Downloads direkt in `/home/jan/netcore-tetra`, der passende lokale Eigentümer und der vorgeschlagene Neustartdienst. Der Screenshotpfad `/run/bluestation/config.toml` ist ein zusätzlicher Grund, nicht ungeprüft eine andere lokale Datei zu überschreiben. Es gibt keinen belegten erfolgreichen Abschluss der Signaturdiagnose. [C08; A02]

### 8.7 `last_version-Datei ist defekt`

Der maßgebliche Nutzerlog lautet:

```text
May 07 20:16:45 SRV-M-RPi-TBS01 bluestation_hybrid_manager.sh[2042361]: FEHLER: last_version-Datei ist defekt.
```

Dieser Log belegt Host, Managername, Zeitpunkt und Fehlermeldung. Er belegt **nicht** den Dateipfad, das akzeptierte Format, die aktuelle Datei, deren Rechte oder die systemd-Unit. [C08]

| Früherer Vorschlag | Quellenlage | Archivbewertung |
|---|---|---|
| `/var/lib/netcore-tetra/last_version` auf 1 setzen | Vom Nutzer mit Verweis auf eine andere frühere Lösung zurückgewiesen | Nicht als Lösung übernehmen. |
| `/home/jan/netcore-tetra/last_version` auf 1 setzen | Danach von der Assistenz behauptet, ohne Ausgabe der tatsächlichen Managerkonfiguration | Unbestätigt; nicht als richtigen Pfad etablieren. |
| `bluestation-config-agent.service` neu starten | Nutzer: „haben wir doch gar nicht“ | Ausdrücklich widersprochen. |
| Stattdessen `bluestation.service` neu starten | Anschließender Assistenzvorschlag ohne Unit-Beleg | Unbestätigt, kein Ersatz für Diensterkennung. |
| `printf "1\n"`, `chown jan:jan`, `chmod 644`, `cat -A` | Als Reparatur vorgeschlagen | Keine sichtbare erfolgreiche Ausführung oder passende Formatprüfung. |
| Datei löschen und neu erzeugen | „Holzhammer“-Vorschlag der Assistenz | Nicht bestätigt; keine Standardreparatur aus diesem Archiv. |

Die Vermutung „kein sauberer Integer, eventuell CRLF“ ist eine mögliche, aber **nicht durch den Managerquelltext oder Dateibytes bewiesene** Erklärung. Auch die Aussage, dieser Merker sei sicher die zuletzt akzeptierte Server-State-Version, ist ungeprüft.

**Fortsetzungsvoraussetzung:** Zunächst den tatsächlich gestarteten Manager und dessen Unit, Environment/Arbeitsverzeichnis sowie die im Script ausgewertete Datei feststellen; erst danach Format, Eigentümer, mögliche konkurrierende Schreibprozesse und Rücksetzsemantik prüfen. Die Sicherheitsbedeutung eines möglichen Versions-/Replay-Schutzes muss aus dem wirklichen Code abgeleitet werden. Ein pauschaler Reset auf 1 ist hier nicht als freigegebene Reparatur dokumentiert.

## 9. Wiedergefundenes Server-Fix-Artefakt

### 9.1 Identität und Integrität

Im Laufzeitbereich war `netcore_tetra_control_server_fix.zip` tatsächlich verfügbar. Es enthält genau `app.py` und `control_context.py`. Bereits entpackte Kopien unter `netcore_tetra_control_server_fix/` wurden byteweise mit den ZIP-Mitgliedern verglichen und stimmen überein. [A01]

| Datei | Umfang | SHA-256 |
|---|---|---|
| `netcore_tetra_control_server_fix.zip` | ZIP mit zwei Python-Dateien | `32f0fe7a271b43dd0c9902fc5a71f6d15e8b2fc02242b1f3da716829293922a2` |
| ZIP-Mitglied `app.py` | 4258 Bytes, 175 Zeilen | `9048bb3f84e4b99e944461a0455ecc9dc7d561cb0ae1103667348ec21a37c8f7` |
| ZIP-Mitglied `control_context.py` | 24031 Bytes, 815 Zeilen | `f3b00edd2ec15caed21b40be1ce501b8fbfc4c06fb5e0f7ff715e33b471e6347` |

Das Artefakt beweist, dass ein konkreter Server-Fix erzeugt wurde, **nicht** dass diese Fassung vollständig installiert, erfolgreich getestet oder später weiterhin aktiv war. Eine exakte Erzeugungszeit oder Zuordnung zu jeder ausgelassenen Antwort wird nicht behauptet. Der Nutzer verlangte später ausdrücklich einzelne vollständige Dateien statt dieses ZIP-Workflows.

### 9.2 Unterschiede zum ursprünglichen Servercode

Der ZIP-Einstieg ergänzt CORS-Header, Health-Aliase `/api/health` und `/api/mobile/health`, Listenrouten `/api/stations`, `/api/nodes`, `/api/mobile/stations`, `/api/mobile/nodes` sowie Einzelrouten `/api/station/<path:node_id>` und `/api/mobile/station/<path:node_id>`. Die Liste verwendet nun einen Wrapper mit `ok`, `count`, `nodes_count`, `online_count`, `reachable_count`, `stations`, `nodes` und `updated_at`. Beide Listenfelder enthalten dieselbe Stationsliste. [A01, `app.py`:55–171; `control_context.py`:473–581]

Die erzeugten flachen Stationsobjekte enthalten Identitäten, Host/IP, Lock, Sollzustand, Versions-/Zeitfelder, Health und abgeleitete Booleans wie `sender_running`, `signature_ok`, `service_ok`, `config_found` sowie `set_run_url` und `set_stop_url`. Fehlende Agentwerte werden dort vielfach zu `False`, weshalb die Unterscheidung zwischen fehlender Telemetrie und negativem Ergebnis verloren gehen kann.

`load_nodes()` des Artefakts liefert bei Lese-/JSON-Fehlern still `{}`; `load_state_safe()` erzeugt bei Fehlern einen Ersatzstate STOP/Version 0 mit Fehlertext. Das kann eine Störung in eine scheinbar leere Registry oder einen scheinbaren STOP-Zustand umwandeln. Dieser Befund erklärt ein Risiko der erzeugten Fassung, nicht automatisch einen bestimmten App-Screenshot.

### 9.3 Doppelte Route und ungelöster Vertragskonflikt

Im ZIP wird `load_plugins()` **vor** den zusätzlichen App-Routen ausgeführt. Das später vollständig gepostete Stationsplugin registriert ebenfalls `GET /api/nodes`, dort jedoch mit dem ursprünglichen Map-Vertrag. Eine Kombination beider Quellstände registriert somit dieselbe URL/Methode unter zwei unterschiedlichen Flask-Endpunkten (`stations_api_nodes` und `api_nodes`) mit inkompatiblen Antwortformen. [C05; A01, `app.py`:53,149–151]

Das ist ein konkreter **statischer Integrationskonflikt**, der in der Abschlussprüfung erkannt wurde. Welche Regel in einer tatsächlich installierten Kombination bedient wurde und ob genau diese Kombination damals aktiv war, ist nicht nachgewiesen. Ein dafür versuchter isolierter Flask-Test konnte in der Archivumgebung mangels Flask nicht gestartet werden. Die vom Nutzer später gepostete Antwort bestätigt jedenfalls den ursprünglichen Map-Vertrag für den beobachteten Zugriff.

Die Zusatzrouten des ZIP sind deshalb **keine verbindliche neue API-Baseline**. Die spätere ausdrückliche Vorgabe lautet, das aktuelle JSON unverändert zu lassen und die App anzupassen. Das historische ZIP darf nicht ungeprüft erneut auf den Server kopiert werden.

## 10. Heute geprüfter Repository-Stand — getrennt vom Chat

### 10.1 Prüfverfahren und Reichweite

Am 2026-10-03 wurde `Archiving` über den GitHub-Connector gelesen und auf Commit `855ec68e4379983b8a59725723d1ad99bc8fcb47` fixiert. Root- und ausgewählte Teilbäume wurden geprüft, ebenso der vollständige vorhandene Archivindex sowie relevante aktuelle Dateien. Die rekursive Gesamtbaumantwort und einige große Baumantworten wurden in der Werkzeugausgabe gekürzt; daher wird keine vollständige Negativsuche über jeden Pfad behauptet. [R01–R04]

Die ergänzende GitHub-Codesuche nach `last_version`, `10_stations_plugin.py` und `MainActivity.kt` lieferte keine Treffer. Diese Suche deckt laut Werkzeugvertrag den **Defaultbranch** ab; die Repository-Metadaten nennen dafür `main`. Sie ist ausdrücklich kein vollständiger Suchnachweis für `Archiving`. Der direkte, auf den Prüfcommit fixierte Abruf von `app/src/main/java/de/netcore/tetra/mobile/MainActivity.kt` lieferte 404. [R05]

Ein zusätzlicher lokaler Git-Clone für einen vollständigen Offline-Abgleich scheiterte an der DNS-Auflösung der Archivumgebung. Ein ergänzender Snapshot-Download gelang ebenfalls nicht. **Der GitHub-Connector war dagegen lesbar und besitzt Schreibzugriff; diese Einschränkung ist kein behaupteter Verlust des Repository-Zugriffs.**

### 10.2 Ergebnisse

| Gegenstand | Heute verifiziert | Was daraus nicht folgt |
|---|---|---|
| Zielbranch und Archiv | `Archiving` und vorhandener `Docs/archive/README.md` lesbar; vorhandene fremde Chatarchive berücksichtigt | Kein vollständiger Export dieses historischen Chats vorhanden |
| Android-Quellstand | Kein Android-Projekt am geprüften Root erkennbar; direkte Standardpfadabfrage für `MainActivity.kt` 404 | Kein Beweis, dass es auf keinem anderen Pfad/Branch/Repository einen App-Stand gibt |
| Historischer Flask-Server/Hybrid-Manager | Gepostete Dateien bzw. Anhang verfügbar, aber kein zugehöriger aktueller Repositorypfad in der durchgeführten Prüfung eindeutig zugeordnet | Keine Aussage, dass die damalige Serverinstallation entfernt oder vollständig migriert wurde |
| Aktueller Basisstations-Updater | `install/update-basisstation.sh` tatsächlich gelesen | Kein Nachweis einer `last_version`-Reparatur; nicht mit dem historischen Hybrid-Manager gleichsetzen |
| Neuere zentrale Verwaltung | `system-backend/provisioning-core/README.md` tatsächlich gelesen | Kein Nachweis, dass die Android-App diesen Dienst nutzt oder den alten Flask-Server ersetzt |

Der aktuelle Updater verwendet standardmäßig `CONFIG_PATH=/etc/netcore/config.toml`. Er versucht zunächst den konfigurierten `service_name` und danach `tetra.service`, `bluestation.service`, `tetra-bluestation.service`, `bluestation-bs.service`. Außerdem ermittelt er das wirklich gestartete Binary über MainPID/`/proc` bzw. ExecStart. Das bestätigt gerade **keinen einzelnen pauschal richtigen Dienstnamen** für den historischen Aufbau. [R03, Zeilen 18–26 und 108–150]

Die heutige Provisioning-Core-README beschreibt eine zentrale Teilnehmer-/Geräte-/Gruppenverwaltung auf Standardport `8125/tcp`, die Subscriber Core (`8100`) und Group Core (`8110`) bündelt. Das ist ein benachbarter neuerer Verwaltungsbaustein und **nicht** der im Chat nachgewiesene Control-Server auf `8080`. Es wurde nur die README verifiziert, nicht dessen gesamter Code oder Livebetrieb. [R04]

### 10.3 Nicht nachweisbar als „inzwischen behoben“

Aus dem Repository-Abgleich lässt sich keiner der folgenden Punkte als heute abschließend behoben markieren: Android-Streamfehler, korrekte aktive-Sender-Telemetrie, finale Modellkonsistenz, interner Dateiviewer, Signaturwarnung oder `last_version`-Defekt. Der sichtbare spätere Fortschritt bei Liste, Actions und Details ist ein **historischer Betriebsbefund**, keine verifizierte aktuelle App-Version.

## 11. Tests, Betriebsbeobachtungen und Grenzen

| Prüfung | Ergebnis | Status/Grenze |
|---|---|---|
| Nutzer prüft beide Control-DNS-Varianten für `/api/nodes` | Gleiches gepostetes JSON mit fünf Nodes | Historisch getestet; keine heutige Remoteabfrage |
| BREW-DNS-Änderung durch Nutzer | Backend in App/WebUI wieder online | Historisch im Betrieb bestätigt |
| App zeigt Liste mit fünf Stationen | Mehrfach sichtbar | Anzeige bestätigt; Telemetriesemantik nicht damit abgenommen |
| Start/Stop über korrigierten Weg | Nutzer sieht nach manuellem Refresh offenbar angenommenen Befehl; weitere State-Versionen sichtbar | Sollzustandswirkung teilweise bestätigt, Rückmeldung/Refresh weiter fehlerhaft; tatsächliches Senden nicht unabhängig bewiesen |
| Details-Button | Später geöffneter Dialog mit vollständigen Daten sichtbar | Historisch im Betrieb bestätigt |
| Interner Config-Viewer | Nur Auftrag sichtbar | Nicht als getestet markieren |
| V2-Kopie | Nutzer bestätigt kopierten Projektordner mit `_V2` | Lokale Aktion bestätigt, kein Git-Baseline-Nachweis |
| Signaturreparatur / Reset auf 1 | Keine passende Erfolgsrückmeldung im zugänglichen Verlauf | Unbestätigt |
| Archivprüfung: ZIP-Mitglieder gegen entpackte Dateien | Beide byteidentisch, SHA-256 ermittelt | Tatsächlich am 2026-10-03 durchgeführt |
| Archivprüfung: Python-Quellen aus ZIP | `ast.parse()` und `compile(..., 'exec')` für beide Dateien ohne Syntaxfehler | Syntaxprüfung, kein Import-/Flask-/Betriebstest |
| Archivprüfung: isolierter Flask-Routenversuch | Vor Ausführung mit `ModuleNotFoundError: No module named 'flask'` abgebrochen | Nicht erfolgreich getestet; keine fingierte Routing-Abnahme |
| Android-Build, Emulatorlauf, echter TBS-/BREW-Zugriff im Archivlauf | Nicht durchgeführt | Kein lokaler vollständiger Android-Quellstand und kein Zugriff auf Jans Verwaltungsnetz |

## 12. Befehle und Abläufe

### 12.1 Tatsächlich belegte historische Aktionen

Der Nutzer gab auf dem Control-Server im Verzeichnis `/opt/netcore-tetra-control/plugins` den Inhalt von `10_stations_plugin.py` und `20_generator_plugin.py` mit `cat` aus. Die Ausgaben sind Primärbelege für Routen und Quelltext. Die JSON-Aufrufe über beide Control-DNS-Adressen sowie die lokale `_V2`-Kopie sind ebenfalls vom Nutzer bestätigt. [C04, C05, C07]

### 12.2 Lesende Anschlussdiagnose — neu formuliert, nicht im Betrieb ausgeführt

Die folgenden Befehle sind ausschließlich Prüfschritte für eine spätere Fortsetzung. Sie setzen keine Version zurück, schreiben keine Konfiguration und starten keinen Dienst neu:

```bash
curl --fail --show-error --silent --max-time 10 \
  'http://ct-h-dev-04.netcore-tetra.de:8080/api/nodes'

curl --fail --show-error --silent --max-time 10 \
  'http://ct-h-dev-04.netcore-tetra.de:8080/state/SRV-M-RPi-TBS01/state.json'
```

Diese Abfragen wurden im Archivlauf nicht gegen das reale System ausgeführt. Im ursprünglichen Servercode kann das Lesen bei fehlenden Node-Dateien die in Abschnitt 4 beschriebene Initialisierung auslösen.

Auf der Basisstation kann zunächst die tatsächliche Zuordnung zum Manager gesucht werden:

```bash
sudo journalctl --since '2026-05-07 20:15:00' \
  --until '2026-05-07 20:18:00' \
  --grep 'last_version-Datei ist defekt' -o verbose --no-pager

sudo grep -RIn --include='*.service' --include='*.conf' \
  'bluestation_hybrid_manager' \
  /etc/systemd/system /usr/lib/systemd/system /lib/systemd/system
```

Auch diese Befehle sind **neu vorgeschlagene, nicht ausgeführte Diagnose**. Alte Journale können fehlen und nicht jedes System muss dieselben Verzeichnisse oder Journaloptionen besitzen. Environment-/Unit-Ausgaben können Zugangsdaten enthalten und sind vor Weitergabe zu bereinigen. Ein Manager-Dateipfad oder Restart-Befehl wird bewusst erst nach tatsächlicher Zuordnung festgelegt.

### 12.3 Nicht als Runbook übernehmen

Die früher vorgeschlagenen Befehle zum direkten Download von `config.toml` und `.sig` in einen geratenen Home-Pfad, zum Löschen oder Zurücksetzen einer geratenen `last_version`-Datei und zum Neustart eines geratenen Dienstes sind in Abschnitt 8 dokumentiert, aber **nicht zur Wiederverwendung freigegeben**. Die Archivierung führt keinen dieser Eingriffe aus.

## 13. Verworfene, ersetzte und problematische Ansätze

Die endgültige Basis ist der vorhandene Serververtrag, nicht ein nachträglich erfundener. Deshalb sind geratene Aktionsrouten, POST-Fallbacks auf nicht vorhandene Pfade und weitere Serveränderungen für den App-Fix verworfen. Ebenso ersetzt die Map-Auswertung das Anzeigen ganzer JSON-Objekte im Sollfeld oder eine nicht verlässlich zugeordnete HTML-Ausweichlösung.

„Registriert“, „erreichbar“, „Soll-RUN“ und „Sender aktiv“ werden nicht mehr als Synonyme behandelt. Fehlende Health-Daten dürfen nicht als gemessenes Nein ausgegeben werden. Bei temporären Ladefehlern eine leere Liste als „keine Stationen“ darzustellen ist eine offene Regression, kein gewünschtes Verhalten.

Unvollständige Austauschstücke, doppelte Modelklassen, fehlende Theme-/Screen-Funktionen und TODO-Implementierungen sind ausdrücklich abgelehnt. Das historische Server-ZIP wird als Artefakt erhalten, aber nicht als aktuelle, ungeprüft erneut auszurollende Reparatur empfohlen. Nicht belegte `last_version`-Pfad- und Dienstbehauptungen sind zurückzunehmen, statt sie durch Wiederholung zu verfestigen.

## 14. Offene Aufgaben, Roadmap-Kandidaten und Prioritäten

Die folgende Reihenfolge ist eine **aus den Blockern abgeleitete Priorisierung für die Fortsetzung**, keine nachträglich erfundene terminierte Projektzusage. Verbindlich historisch vereinbart sind vor allem App-only, unverändertes JSON, vollständige Dateien, kompaktere Bedienung und interne Dateiansicht.

| Priorität | Aufgabe | Status | Abhängigkeit/Abnahmekriterium |
|---|---|---|---|
| P0 | Letzten tatsächlich verwendeten Android-V1-/V2-Quellstand sichern und eindeutig zuordnen | Offen | Alle Kotlin-/Model-/Gradle-/Manifest-Dateien zusammen; reproduzierbarer Build ohne Redeclarations, TODOs oder fehlende Symbole |
| P0 | App-Aktionsroute auf belegtem `/stations/set/...` halten | Vertrag geklärt, finale Umsetzung nicht verifiziert | App und WebUI bewirken denselben Sollzustand; kein Serverumbau; keine 404-Fallbackketten |
| P0 | Action-Ergebnis, Redirect und separaten Refresh konsistent behandeln | Teilweise historisch bearbeitet, weiter offen | Kein Fehlalarm nach bestätigtem State-Wechsel; Lock-Korrektur erkennbar; keine blinde doppelte Aktion |
| P0 | Sporadisches `unexpected end of stream` eingrenzen und Listenstand erhalten | Offen | Vollständiger Stacktrace/HTTP-Metadaten; reproduzierbare Transporttests; Fehler löscht keine zuvor geladene Liste |
| P0 | Richtigen `last_version`-Leser, Pfad, Format und Dienst nachweisen | Offen, bisherige Antwort unzuverlässig | Reale Unit/Managerquelle/Dateibytes; begründeter Reparaturschritt; anschließend belegtes Logergebnis |
| P1 | Senderaktivität und Erreichbarkeit aus tatsächlich vorhandener Telemetrie anzeigen | Beschlossen, korrekte Umsetzung unbestätigt | Datenquelle neben unveränderter Registry festlegen; unbekannt/degradiert korrekt; RUN/STOP nicht als Istzustand ausgeben |
| P1 | Doppel-Filter und Zähler sauber definieren | Beschlossen, Zwischenstände widersprüchlich | Filterzustände widerspruchsfrei; sichtbare und gesamte Anzahl korrekt; aktuelle Senderdaten statt Anmeldung |
| P1 | V2-Dateiansicht implementieren/abnehmen | Beschlossen/geplant | Config und State intern lesbar; keine externe App; fehlende Pending-Datei korrekt; Zurücknavigation funktioniert |
| P1 | Kompaktes responsives Layout fertigstellen | Teilweise sichtbar | Stationstitel lesbar, Aktionen bedienbar, ausreichend Scrollfläche auch bei größerer Systemschrift |
| P1 | Signaturdiagnose an tatsächlich geladener Datei durchführen | Offen | Health-Rohdaten, geladener Pfad, passende Signatur und Verifier-Zuordnung prüfen; kein Zugriff auf private Schlüssel nötig |
| P2 | Serverauswahl über Prozess-/Geräteneustart abnehmen | Funktion teilweise bestätigt | Lokales Netz/VPN/FQDN/Testserver; relative Datei-/Action-URLs verwenden dieselbe ausgewählte Basis |
| P2 | V1-Baseline und V2-Entwicklung nachvollziehbar versionieren | Ordnerkopie bestätigt; Git-Lösung Idee/Empfehlung | Nutzbares Restore-Artefakt, eindeutige Versionskennzeichnung; kein solcher Branch in diesem Archivauftrag angelegt |
| P2 | Wiedergefundenen Server-Fix nur inventarisieren und Installed-State-Abgleich durchführen | Artefakt geprüft; Deployment offen | Doppelte `/api/nodes`-Registrierung und Wrapper/Map nicht vermischen; keine automatische Rückinstallation |
| P2 | Generatorfälle mit abweichender Node-ID/Device-ID testen | Neuer Prüfkandidat | Save-Pending adressiert Registry-Node korrekt; Template-Patching tatsächlich wirksam |
| P2 | Fehler-/Zustandsmatrix automatisieren | Neue technische Empfehlung | Tests für Map, leere Antwort, 404, Redirect, abgebrochenen Body, Sperre, unbekannte Telemetrie und Dateiansicht |

### 14.1 Sinnvolle erste Fortsetzungssequenz

Zunächst den funktionierenden lokalen Quellstand und die tatsächlichen laufenden Komponenten identifizieren. Danach **ohne Serveränderung** Registry-Modell, exakte Aktionsroute und Ergebnisbehandlung zusammen prüfen. Erst auf dieser stabilen Grundlage Filter, Layout und Dateiviewer erweitern. Die `last_version`-/Signaturdiagnose benötigt separat den realen Manager-/Health-Code und darf nicht durch ein weiteres App-Layoutupdate verdeckt werden.

Für eine Abschlussabnahme fehlen insbesondere: wiederholte RUN-/STOP-Zyklen mit korrelierter State-Version, simulierte bzw. kontrollierte HTTP-Abbrüche ohne Datenverlust in der Liste, Offline-/Warning-/Unknown-Anzeigen, Filterkombinationen, gesperrte Station, Serverwechsel und Neustart, Detailnavigation sowie interne Ansicht vorhandener und fehlender Dateien. Das sind geplante Testfälle, keine bereits erfolgreichen Ergebnisse.

## 15. Quellen, Anhänge und Nachvollziehbarkeit

### 15.1 Chat-Primärquellen

| Kürzel | Zugängliche Quelle |
|---|---|
| C01 | Nutzerwunsch „zurück zur App“ mit einstellbarer Control-URL und DNS-Namen; spätere Nutzerbestätigung der serverseitigen DNS-Anpassung |
| C02 | Vom Nutzer gepostete ursprüngliche vollständige `app.py` mit Plugin-Lader, Dashboard und `/health` |
| C03 | Vom Nutzer gepostete ursprüngliche vollständige `control_context.py` mit State-/Signier-/Health-Funktionen |
| C04 | Vollständig gepostete Antwort beider `/api/nodes`-Adressen mit fünf Nodes und `meta/state/urls` |
| C05 | Terminalausgabe `cat 10_stations_plugin.py` und `cat 20_generator_plugin.py` aus `/opt/netcore-tetra-control/plugins` |
| C06 | App-/WebUI-Screenshots, Compilerfehler und Logcat vom 7. Mai 2026; wiederholte ausdrückliche Nutzerkorrekturen zu kompletten Dateien und App-only-Fixes |
| C07 | Nutzerbestätigung der `_V2`-Ordnerkopie und Auftrag zur rein internen Config-Dateiansicht; Screenshot des funktionierenden Detailsdialogs |
| C08 | Nutzerkorrekturen zu `last_version`, ausdrücklich nicht vorhandenem Config-Agent-Dienst, Signaturwarnung und Hybrid-Manager-Log; dazu die unbestätigten Assistenzvorschläge |

Die Kürzel bezeichnen die hier zugänglichen Nachrichten, nicht erfundene externe Chatlinks. Die nicht sichtbaren vollständigen Kotlin-Antworten können damit nicht nachträglich wiederhergestellt werden.

### 15.2 Relevante lokale Artefakte

**A01:** `netcore_tetra_control_server_fix.zip`, zwei Mitglieder und entpackte identische Kopien; Hashes und Umfang in Abschnitt 9. Wichtige Quellstellen: `app.py` Plugin-Laden vor Zusatzrouten, Zeilen 53–171; `control_context.py` Fehlertoleranz in Zeilen 55–70 und 151–162, Reachability 285–408, Mobile-Payloads 473–581. Die Dateien enthalten keinen `last_version`-Bezug.

**A02:** 38 ursprüngliche PNG-Screenshots waren im Laufzeitbereich zugänglich. Sie wurden als Übersicht gesichtet; relevante Ansichten wurden direkt bzw. vergrößert geprüft. Besonders nützlich sind:

| Screenshotdatei | Inhalt |
|---|---|
| `301ded6c-438a-4588-a81a-f789c4e4e8a6.png` | Stations-WebUI mit BREW-Status, TBS01/TBS02, Health-Chips, Dateien und geladenem Pfad `/run/bluestation/config.toml` |
| `b85e1aa5-efa7-4bda-a4e4-f349198f2387.png` | Browser-JSON-Ansicht mit nach Node-ID geschachtelten `meta`, `state`, `urls`; älterer Snapshot als die vollständig gepostete Antwort |
| `1e5d25df-c42f-45f9-8b50-45991b8ad9d1.png` | Leere App-Liste nach Streamfehler bei gleichzeitig angezeigtem RUN-Auftrag |
| `9c689fd9-a206-4ebb-9071-28bbd9e28efe.png` | Aktionsfehler `unexpected end of stream` in einer späteren App-Version |
| `c559ed4d-201f-4bc8-9f18-3b439188e4f4.png` | Geöffneter Detailsdialog TBS01 mit Version 97 und Dateipfaden |

Die Namen dienen der Wiedererkennung der Originalanhänge. Sie sind keine behaupteten Repository-Dateien. Screenshots und ZIP werden durch diesen Auftrag nicht zusätzlich nach Git kopiert; die technischen Ergebnisse sind im Markdown bewahrt.

### 15.3 ETSI-Projektunterlagen

Zusätzlich verfügbar sind 24 einzelne ETSI-PDFs und die Sammlung `ETSI.pdf` mit laut bereitgestelltem Dateitext 4100 Seiten. Ihre sichtbaren Titelseiten/Inhaltsangaben wurden für die Relevanzzuordnung berücksichtigt; es wurde **keine vollständige Normenauswertung** für dieses App-Archiv vorgenommen. Keine dieser Unterlagen belegt eine Retrofit-Route, ein lokales `last_version`-Format oder einen installierten Dienst. Die in diesem Chat verwendete State-/Config-Signierung wird nicht als Nachweis einer TETRA-Air-Interface-Sicherheitsfunktion dargestellt.

| Dateien | Zugeordnetes Thema laut bereitgestellten Dokumenttiteln |
|---|---|
| `en_30039201v010601p.pdf`, `en_30039202v030801p.pdf` | General network design; Air Interface |
| `en_3003920303v010301p.pdf`, `en_3003920313v010201p.pdf` | ISI Group Call, klassische bzw. transportunabhängige Beschreibung |
| `en_3003920304v010301p.pdf`, `en_3003920308v010401p.pdf` | ISI SDS; Generic Speech Format |
| `en_3003920315v010500a.pdf` | Draft V1.5.0 (2026-04), transportunabhängiges Mobility Management; nicht als endgültig verabschiedete Ausgabe umetikettiert |
| `en_30039205v020701p.pdf`, `en_30039207v030501p.pdf`, `en_30039209v010701p.pdf` | PEI, Security, allgemeine Supplementary Services |
| `en_3003921006v010401p.pdf`, `en_3003921018v010301p.pdf` | Call Authorized by Dispatcher; Barring of Outgoing Calls, Stage 1 |
| `en_3003921101v010201p.pdf`, `en_3003921114v010101p.pdf`, `en_3003921117v010102p.pdf` | Call Identification, Late Entry, Include Call, Stage 2 |
| `en_3003921201v010202p.pdf`, `en_3003921216v010400a.pdf` | Call Identification, Stage 3; PPC Draft V1.4.0 (2026-03) |
| `en_30039401v030301p.pdf`, `en_30039502v010303p.pdf` | Radio Conformance Testing; TETRA Codec |
| `en_300812v020101p.pdf`, `ts_10081201v020205p.pdf`, `es_20081201v020205p.pdf`, `es_20081202v020401m.pdf` | SIM-/TSIM-/UICC-Schnittstellen; letzte Datei ist als Final Draft gekennzeichnet |
| `ets_30039214e01v.pdf` | Final draft prETS 300 392-14, September 1997, PICS-Proforma |
| `ETSI.pdf` | Umfangreiche bereitgestellte Sammlung; keine vollständige Gleichheits-/Duplikatprüfung gegen die Einzeldateien |

### 15.4 Heute verifizierte Repository-Referenzen

Alle folgenden Quellverweise sind auf den gelesenen Ausgangscommit fixiert, nicht auf einen später beweglichen Defaultbranch:

- **R01:** [Geprüfter Commit](https://github.com/JanHG98/netcore-tetra/commit/855ec68e4379983b8a59725723d1ad99bc8fcb47) und [Repository-Baum](https://github.com/JanHG98/netcore-tetra/tree/855ec68e4379983b8a59725723d1ad99bc8fcb47).
- **R02:** [Archivindex vor dieser Ergänzung](https://github.com/JanHG98/netcore-tetra/blob/855ec68e4379983b8a59725723d1ad99bc8fcb47/Docs/archive/README.md), Blob-SHA `2bac8564bb37b10bd9ba6b212c263275e5799582`.
- **R03:** [Aktueller Basisstations-Updater, geprüfter Ausschnitt](https://github.com/JanHG98/netcore-tetra/blob/855ec68e4379983b8a59725723d1ad99bc8fcb47/install/update-basisstation.sh#L1-L150), Blob-SHA `16083c6868fd3b7f16e41f50b6c84704cd5b9a64`.
- **R04:** [Provisioning-Core-README](https://github.com/JanHG98/netcore-tetra/blob/855ec68e4379983b8a59725723d1ad99bc8fcb47/system-backend/provisioning-core/README.md), Blob-SHA `f44a48815437da922091f3c820274601b569a20a`.
- **R05:** GitHub-Contents-Abfrage für `app/src/main/java/de/netcore/tetra/mobile/MainActivity.kt` mit `ref=855ec68e4379983b8a59725723d1ad99bc8fcb47`: HTTP 404; ergänzende Defaultbranch-Suchen ohne Treffer, mit den in Abschnitt 10 genannten Grenzen.

Ein historischer App-Commit, Release-Tag oder zugehöriger PR wurde nicht identifiziert. Vorhandene Archive anderer Themen, insbesondere des Windows-Control-Rooms, sind nicht dieselbe Anwendung und wurden nicht mit diesem Chat zusammengeführt. Der Archivcommit selbst ist über die Git-Historie dieser Datei und die Abschlussmeldung des Archivauftrags nachvollziehbar.

## 16. Kompakte Übergabe

Der damalige Stand umfasst eine startende Android-App mit einstellbarer Control-Adresse, wieder sichtbaren Stationsdaten, teilweise wirksamen Start-/Stop-Anforderungen und später funktionierendem Detailsdialog. Er ist **keine nachgewiesene fehlerfreie Release-Baseline**: sporadische Streamfehler, widersprüchliche Statusinterpretationen und die nicht belegte finale Dateiansicht bleiben offen.

Für eine Fortsetzung sind die vollständigen lokalen App-Dateien wichtiger als ein weiteres Neuschreiben aus Screenshots. Serverseitig sind Map-Vertrag und `/stations/set/...` durch Originalcode geklärt. Die zuletzt behauptete `last_version`-Reparatur ist ausdrücklich nicht geklärt. Der geeignete nächste Schritt ist dort die tatsächliche Manager-/Datei-/Unit-Zuordnung, nicht der nächste geraten ausgeführte Reset.
