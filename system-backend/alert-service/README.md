# NetCore Warnzentrale — NINA / KATWARN

Die Warnzentrale vergleicht aktuelle Gerätepositionen aus dem Control Room mit aktiven Warngebieten und verschickt individuelle SDS über den SDS Router. Sie läuft als eigener Python-3.11+-Dienst in einem LXC und benötigt keine pip-Pakete. WebUI: `http://<WARN-LXC-IP>:8310/`.

Die [Schritt-für-Schritt-Anleitung pro LXC und TBS](../../Docs/KATWARN_NINA_INSTALL_UPDATE.md) nennt für jedes betroffene System die erforderlichen Befehle und Prüfungen. **Kommen Warnungen bereits am Funkgerät an, braucht die neue Geräteübersicht nur ein [Update des Warn-LXC](../../Docs/KATWARN_NINA_INSTALL_UPDATE.md#5-bestehenden-warn-lxc-aktualisieren).** Konfiguration, Token, eigene Warnungen und dauerhafte Empfängerhistorie bleiben erhalten; weitere LXC- oder TBS-Updates sind dafür nicht erforderlich.

Bei der Ersteinrichtung müssen der neue Warn-LXC installiert sowie SDS Router, Control Room und die TBS aktualisiert werden. Die bereits in `katwarn/nina` enthaltene TBS-Korrektur ergänzt die fehlende CMCE-Weiterleitung der zentralen Befehle `DeliverSds` und `SendStatus` samt Rückmeldung. Der Control Room benötigt die aktivierte, nur lesende Verbindung zu Node Gateway `/ws/backend`, damit seine Geräte- und GPS-Ansichten mit Telemetrie gefüllt werden. Die TBS bleiben mit dem Node Gateway verbunden. Die ebenfalls enthaltene [Reparatur langsamer Bereitschaftsprüfungen](../../Docs/SDS_CALL_CONTROL_FALLBACK_REPARATUR.md) betrifft SDS Router und Call Control.

## Verhalten

- Geräteabgleich alle 5 Sekunden: Warnungen bei Anmeldung, neu eingegangenen Meldungen und Einfahrt in ein Warngebiet.
- Nur angemeldete Geräte mit gültigem, ausreichend aktuellem GPS und verbundener, frischer TBS-Telemetrie. Die Geräteidentität ist die ISSI innerhalb dieses NetCore-Netzes.
- BBK-Abgleich alle 60 Sekunden; standardmäßig `mowas`, `katwarn`, `biwapp`, `dwd` und `lhp`. Nur `Actual`/`Public`; Übungen und private Meldungen werden nicht ausgesendet. Quellen sind in `[nina].sources` konfigurierbar, `police` ist zusätzlich unterstützt.
- Gültigkeit, Entwarnung, CAP-Referenzen, Polygone, mehrere Teilgebiete, Löcher und Kreise. Updates derselben Warnung behalten ihre Empfängerhistorie; bereits benachrichtigte Geräte erhalten kein weiteres Update derselben Warnung. Neue Geräte erhalten den aktuellen Text.
- Eigene Meldungen per Kartenklick, Koordinaten, Radius von 50 m bis 200 km, Warnstufe, Text und Ablaufzeit erstellen. Löschen beendet weitere Aussendungen; Verlauf und Empfängerhistorie bleiben erhalten.
- WebUI mit Karte, vollständigen Meldungstexten, Funktext-Vorschau, Geräten, Zustellstatus und Archiv. Leaflet 1.9.4 ist lokal enthalten; nur Hintergrundkacheln werden vom Browser bei OpenStreetMap geladen.
- **„Geräte & Warnstatus“** zeigt alle aktuell vom Control Room gemeldeten Geräte mit ISSI, TBS, Position, Prüfung des Warngebiets und Versandstatus je passender Warnung. Filter: **„Alle“**, **„Im Warngebiet“** und **„Prüfung nötig“**, jeweils mit Anzahl. Der letzte Filter umfasst ausgeschlossene Geräte sowie fehlgeschlagene oder unklare Zustellungen. Auch Geräte ohne verwendbares GPS oder frische TBS-Verbindung bleiben mit dem Ausschlussgrund sichtbar. Die Übersicht ist kein Verzeichnis bereits abgemeldeter Geräte.
- Eine leere Teilnehmerantwort oder ein Fehler beim Control-Room-Abruf wird ausdrücklich angezeigt. Geräte ohne verwendbare Position werden beim Warngebiet als **„Nicht geprüft“** geführt. Die Geräteübersicht ist nur eine zusätzliche Anzeige und ändert weder die Versandvoraussetzungen noch die Duplikatsperren.
- Zugang mit lokal erzeugtem Token. Das Token wird als HTTP-Header gesendet, ausschließlich in der Browsersitzung gespeichert und in normalen Antworten nicht ausgegeben.

BBK-Daten stammen aus den öffentlichen [Warnungsfeeds](https://warnung.bund.de/api31/mowas/mapData.json) und dem [KATWARN-Feed](https://warnung.bund.de/api31/katwarn/mapData.json). KATWARN ist enthalten, soweit Meldungen dort bereitgestellt werden; der Dienst besitzt keinen unabhängigen KATWARN-Partnerzugang. Das BBK beschreibt die Verbindung der Warnsysteme auf seiner [MoWaS-Seite](https://www.bbk.bund.de/DE/Warnung-Vorsorge/Warnung-in-Deutschland/MoWaS/mowas_node.html). Die öffentlichen App-Endpunkte sind eine externe Abhängigkeit und können sich ändern.

## Einmaliger Versand und Fehlerfälle

Die SQLite-Datenbank reserviert `(Warnung, ISSI)` samt vollständigem SDS-Auftrag **vor** dem ersten HTTP-Aufruf. Der aktualisierte Router speichert zusätzlich einen dauerhaften `idempotency_key`. Derselbe Auftrag kann nach einem HTTP-Timeout abgefragt oder erneut übergeben werden, ohne eine zweite Nachricht anzulegen. Die Router-Sperre bleibt auch nach dem Löschen oder Bereinigen eines Nachrichtenobjekts erhalten.

Warnaufträge setzen `at_most_once=true`: eine individuelle Ziel-TBS, keine Routenvervielfachung und keine automatische oder manuelle erneute Funkübergabe nach einem gestarteten Versuch. Die absolute `expires_at`-Frist verhindert, dass ein verspäteter HTTP-Auftrag die Warnung länger gültig macht. Wurde ein Auftrag nachweislich noch nie beim Router angenommen, darf der Dienst eine abgelaufene Reservierung mit demselben Schlüssel erneuern, solange Warnung und Position weiterhin passen.

**Eine exakt einmalige Anzeige am Funkgerät ist ohne entsprechende Endgerätebestätigung nicht beweisbar.** `accepted` bedeutet, dass die TBS den Auftrag zum Senden angenommen hat. Bei verlorenem Ergebnis nach Übergabe an die Funkstrecke wird nicht automatisch erneut gesendet; dadurch kann eine Meldung unbestätigt bleiben. Diese Entscheidung priorisiert den gewünschten Schutz vor wiederholten Warnungen.

| WebUI-Zustand | Bedeutung |
|---|---|
| Ausstehend | Dauerhaft reserviert, Übergabe ausstehend oder nach API-Fehler in Klärung |
| Beim SDS-Router | Nachrichten-ID bekannt; Router verarbeitet den Auftrag |
| Von TBS angenommen | TBS hat die Nachricht zum Senden angenommen; keine Lesebestätigung |
| Unklar · keine Wiederholung | Ausgang unklar oder Router-Historie bereits bereinigt; Sperre bleibt |
| Fehlgeschlagen | Auftrag abgewiesen; Fehler im Verlauf |
| Gestoppt | Warnung gelöscht/zurückgenommen oder doppelte Referenzkette zusammengeführt |
| Abgelaufen | Gültigkeit beendet |

Entwarnungen sind in dieser Version Rücknahmen der ursprünglichen Warnung, keine zusätzliche SDS. Neue Geräte werden danach nicht mehr benachrichtigt. Bereits am Funkgerät angezeigte Nachrichten können nicht zurückgerufen werden. Bei Feed-Ausfall wird der letzte Stand sichtbar gehalten; nach `max_stale_seconds` (Standard 300) entstehen daraus keine neuen SDS. Unvollständige Abfragen gelten nie als erfolgreicher leerer Feed. Eigene Meldungen funktionieren unabhängig vom BBK-Abruf.

SQLite und die SDS-Router-Datei `storage.database_path` enthalten dauerhaften Betriebszustand. Beide regelmäßig konsistent sichern und nicht durch alte oder leere Datenbanken ersetzen. Beim Programmrollback die aktuellen Zustellinformationen behalten. Kein zweites aktives Exemplar mit einer getrennten Historie betreiben. Bei Einsatz mehrerer Netze mit wiederverwendeten ISSIs getrennte Warnzentralen/Router verwenden.

Der Funktext wird für die vorhandene SDS-Zeichenkodierung in ASCII umgewandelt (z. B. `ü` → `ue`) und standardmäßig auf 120 Zeichen gekürzt. Der vollständige Text bleibt in der WebUI. Die vorhandenen TBS-/Funkgerät-Einstellungen müssen bei der Abnahme überprüft werden.

## Schnittstellen

```text
GET    /health/live                Prozessprüfung, ohne Anmeldung
GET    /health/ready               503 bei fehlendem Erstabgleich oder Abhängigkeitsfehler
GET    /api/v1/status              Gesamte Übersicht einschließlich Geräteprüfung und Warnstatus
GET    /api/v1/alerts              Warnungen einschließlich Archiv
GET    /api/v1/devices             Online-Geräte mit verwendbarer Position
GET    /api/v1/deliveries           Dauerhafte Empfängerhistorie
POST   /api/v1/alerts              Eigene Kreiswarnung erstellen
DELETE /api/v1/alerts/{id}         Eigene Warnung zurückziehen, Historie behalten
```

Alle `/api/`-Anfragen benötigen `Authorization: Bearer <NETCORE_ALERT_TOKEN>`. JSON-Beispiel für eine eigene Meldung (Ablauf in die Zukunft setzen):

```json
{
  "title": "Testwarnung",
  "description": "Zufahrt zum Einsatzbereich freihalten.",
  "severity": "Moderate",
  "latitude": 52.3759,
  "longitude": 9.732,
  "radius_m": 1000,
  "expires_at": "2030-01-01T16:00:00+01:00"
}
```

Eine eigene Warnung darf höchstens 366 Tage gültig sein. Der Beispielzeitpunkt muss daher passend ersetzt werden. Der Dienst nutzt beim Control Room nur lesende Endpunkte `/api/subscribers?online=true` und `/api/nodes`. Diese werden bei TBS am Node Gateway erst durch die eingeschaltete `[node_gateway]`-Verbindung des Control Rooms gefüllt. Nach erstmaligem Einschalten müssen aktuelle Anmeldung und GPS-Telemetrie eintreffen; der Gateway-Snapshot enthält keine alten Gerätepositionen. Für geschützte Control Rooms sind Benutzername und `NETCORE_CONTROL_ROOM_PASSWORD` vorgesehen.

`GET /api/v1/status` ergänzt die bisherigen Felder `devices`, `device_diagnostics` und `subscribers_seen` um:

| Feld | Bedeutung |
|---|---|
| `device_overview` | Eine Zeile je aktuell gemeldeter ISSI, einschließlich ausgeschlossener Geräte; enthält `available`, `reason_code`, `reason`, GPS-/TBS-Alter, `area_status` und `matching_alerts` |
| `device_summary.total` | Anzahl der Gerätezeilen, ohne doppelte ISSIs |
| `device_summary.available` | Geräte mit für Warnungen verwendbarer Position und TBS-Verbindung |
| `device_summary.affected` | Verfügbare Geräte in mindestens einem aktiven Warngebiet |
| `device_summary.blocked` | Geräte, deren GPS-/TBS-Prüfung den Warnversand verhindert |

`area_status` ist `affected` (im Warngebiet), `outside` (außerhalb), `no_alerts` (keine aktiven Warnungen) oder `unknown` (Gerät nicht prüfbar). `matching_alerts` enthält je geografisch passender aktiver Warnung Titel, Versandfreigabe und den gespeicherten Zustellstatus für diese ISSI. Ein vorhandener Zustellstatus wird auch nach zusammengeführten Warnungsreferenzen korrekt zugeordnet. Ohne gespeicherten Auftrag bleibt `delivery_state` leer; die Anzeige allein legt keinen Auftrag an. Ein Gerät in einem Warngebiet erhält bei pausiertem Versand oder nicht ausreichend aktuellen NINA-Daten keine neue Zustellung. Die bestehenden Endpunkte `/api/v1/devices` und das Statusfeld `devices` enthalten weiterhin ausschließlich verwendbare Geräte. Der UI-Filter „Prüfung nötig“ berücksichtigt zusätzlich Zustellprobleme und kann deshalb mehr Geräte enthalten als `device_summary.blocked`.

## Entwicklung und Prüfung

```bash
cd system-backend/alert-service
python3 -m unittest discover -s tests -v
python3 main.py --config /etc/netcore/alert-service.toml --check-config
```

Rust und optionaler echter HTTP-Integrationstest, vom Repository-Hauptverzeichnis:

```bash
cargo test -p netcore-sds-router
cargo build -p netcore-sds-router
python3 system-backend/alert-service/tests/integration_router.py
```

Die Geräteanzeige mit dem eingebauten Node.js-Testläufer prüfen, ebenfalls vom Repository-Hauptverzeichnis (Node.js nur für die Entwicklung erforderlich):

```bash
node --test system-backend/alert-service/tests/test_device_overview_ui.cjs
```

Der Integrationstest startet ausschließlich seinen eigenen Router auf Loopback mit temporärer Datenbank und einem reservierten, nicht erreichbaren Gateway-Port. Er sendet keine Funknachrichten und stoppt keine vorhandenen Dienste. Ein abweichender Binary-Pfad ist über `--router-binary` möglich.

Die Tests prüfen unter anderem Anmeldung, Gebietswechsel, Referenzketten, Neustarts, Persistenzfehler, verlorene HTTP-Antworten, absolute Fristen, Entwarnung, Token-Schutz und Eingabevalidierung. Für die Geräteübersicht werden verfügbare und ausgeschlossene Geräte, überlappende Warngebiete, gespeicherte Zustellzustände und zusammengeführte Warnungsreferenzen geprüft. Die Abnahme mit echten Funkgeräten ist in der Rollout-Anleitung beschrieben.

## Abhängigkeiten und Quellen

- Python 3.11+ mit SQLite und `tomllib`, systemd im LXC.
- [Leaflet 1.9.4](https://leafletjs.com/download.html), BSD-Lizenz in `static/vendor/LEAFLET-LICENSE.txt`. Lokale JS/CSS-Dateien entsprechen den vom Projekt veröffentlichten SHA-256-Integritätswerten.
- [OpenStreetMap-Nutzungsbedingungen für Kartenkacheln](https://operations.osmfoundation.org/policies/tiles/): normale Browserabfragen mit Attribution und Referer, keine Vorabdownloads. Ohne Kachelzugriff bleiben Warngeometrien und Koordinaten bedienbar.
- [CAP 1.2](https://docs.oasis-open.org/emergency/cap/v1.2/CAP-v1.2-os.html) für Warnungsreferenzen, Zeitangaben und Geometrien.
