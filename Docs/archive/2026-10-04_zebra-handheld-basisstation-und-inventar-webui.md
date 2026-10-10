# Brainstorming: Zebra-Handheld, Basisstationszugriff und Inventar im WebUI

**Archivhinweis, 09.10.2026:** Diese Datei dokumentiert das im Titel beziehungsweise Quellenrahmen genannte Gespräch und dessen damalige Entscheidungen. Historische Kommandos, Planungen, Versionsangaben und Fehlerbilder bleiben erhalten; der heutige technische Stand steht in der [Gesamtroadmap](../roadmaps/gesamtroadmap.md) und in den [aktuellen Dienstanleitungen](../services/README.md). Die Archivaufnahme bestätigt keine spätere Implementierung oder Live-Abnahme.

**Arbeitsstand:** 2026-10-04. Historische Betriebsbeobachtungen und der an diesem Datum geprüfte Repository-Stand sind getrennt ausgewiesen.

Gesucht sind realistische Einsatzmöglichkeiten für ein Zebra-Handheld mit einer einzelnen Basisstation. Ein späteres Lighthouse-Inventar bleibt als Ausbauidee erhalten; der am 4. Oktober 2026 vorhandene Asset-Dienst bietet dafür einen anderen technischen Ausgangspunkt.

## 1. Arbeitsstand und Bezugsquellen

| Merkmal | Stand |
|---|---|
| Projekt | NetCore-Tetra |
| Repository | [JanHG98/netcore-tetra](https://github.com/JanHG98/netcore-tetra) |
| Thema | Realistische Nutzung des Zebra-Handhelds zunächst mit einer einzelnen Basisstation; spätere Inventarverwaltung im Lighthouse-WebUI |
| Historischer Planungszeitraum | Hinweise auf 17./18.10.2025; mangels Originalzeitstempeln nicht abschließend verifiziert. |
| Erstellungsdatum | **2026-10-04**, Zeitzone Europe/Berlin |
| Ausschließlicher Zielbranch | **`Archiving`** |
| Geprüfter Code-Commit | **`e4cdd99091385b3b32b438f8cf6f020ccf78fba1`** ([Commit](https://github.com/JanHG98/netcore-tetra/commit/e4cdd99091385b3b32b438f8cf6f020ccf78fba1)) |
| Prüfverfahren | Frisch abgerufener Branch, Quelldateien und Konfigurationsvorlagen gelesen, relevante Begriffe gesucht, isolierter lokaler HTTP-Smoke-Test des vorhandenen Asset-Dienstes |
| Gerät | **„Zebra TH57“** und **„TC57“** sind als unterschiedliche Bezeichnungen überliefert. Typenschild, Android-Geräteinformationen oder ein Foto müssen die Variante bestätigen. |
| Archivdatei | `Docs/archive/2026-10-04_zebra-handheld-basisstation-und-inventar-webui.md` |
| Archivindex | `Docs/archive/README.md` |
| Vor dem Schreiben geladener Branchkopf | `e4cdd99091385b3b32b438f8cf6f020ccf78fba1` |

### 1.1 Quellenbasis und offene Nachweise

**H1 – Ausgangsplanung:** Realistische Nutzung des Zebra-Handhelds, lokale Basisstation und spätere Inventaridee. Die zentrale Korrektur lautet: Lighthouse war nicht vorhanden; unmittelbar nutzbare Funktionen müssen mit der Einzelbasisstation auskommen.

**H2 – Ergänzende Ideensammlung:** Browser/SSH, lokale Erfassung, Audio, Automatisierungsbuttons, GPS-Logging, Mini-API, PWA und PTT. Diese Möglichkeiten sind nur zusammenfassend dokumentiert und haben keine Installations- oder Testbelege.

**R1 – Repository-Prüfung vom 4. Oktober 2026:** Branch `Archiving` am genannten Prüfcommit; Quelldateien, Konfigurationsvorlagen und ein lokaler HTTP-Smoke-Test des Asset-Dienstes. Andere Entwicklungsbranches und Zielinstallationen gehören nicht zum Prüfstand.

**A1 – Normenbestand:** 25 TETRA-PDFs; Dateimetadaten, Titelseiten, Seitenzahlen und SHA-256 geprüft. Die vollständige Normauswertung war für die Zebra-Idee nicht erforderlich.

Offen bleiben Gerätevariante, Android-/DataWedge-/Browserversionen, Scannerprofil, installierter TBS-Commit sowie ein tatsächlicher Scan-, API- oder Lighthouse-Rollout. Originalbilder und ein vollständiger Übergabetext für die Umsetzung liegen nicht vor.

### 1.3 Statusbegriffe

| Kennzeichnung | Bedeutung in diesem Archiv |
|---|---|
| **Idee** | Vorschlag oder denkbare Nutzung; keine angenommene Umsetzung |
| **beschlossen/geplant** | Ausdrückliche Projektanforderung oder als Arbeitsrichtung festgelegter Umfang; noch kein Implementierungsbeleg |
| **implementiert** | Konkreter Code oder Konfigurations-/Installationspfad am Prüfcommit vorhanden |
| **getestet** | Ein beschriebener Test wurde tatsächlich ausgeführt; seine Grenzen gehören zur Aussage |
| **im Betrieb bestätigt** | Durch konkrete Beobachtungen auf der Zielinstallation bestätigt; für die Zebra-Integration für diesen Arbeitsstand nicht erreicht |
| **nicht belegt / nicht zugänglich** | Fehlende Evidenz; keine Behauptung, dass etwas niemals existierte |

## 2. Ziel, Ausgangslage und Verlauf

### 2.1 Ziel

Das Zebra soll als mobiles Arbeitsgerät einen praktischen Beitrag zu NetCore-Tetra leisten. Unmittelbar geht es um Zugriff auf die vorhandene Basisstation und lokale Materialerfassung. Der integrierte Scanner macht eine spätere Inventarverwaltung interessant.

### 2.2 Festlegungen und Korrekturen

| Thema | Festgehaltene Richtung | Status |
|---|---|---|
| Ausgangssystem | Eine einzelne Basisstation; Lighthouse noch nicht vorhanden | Maßgebliche Ausgangslage |
| Sofortiger Nutzen | Funktionen auswählen, die mit diesem Einzelsystem tatsächlich möglich sind | Geplant |
| Inventar | Lighthouse-WebUI, Scanner, Datenmodell und APIs als spätere Erweiterung untersuchen | Idee |
| VPN | Keine zusätzliche VPN-Pflicht als Voraussetzung der Zebra-Nutzung festgelegt | Korrektur |
| Umsetzungshilfe | Kompakte Arbeitsübergabe; Englisch ist zulässig | Gewünscht, Originaltext nicht verfügbar |
| Gerätebezeichnung | TH57/TC57 erst am realen Gerät klären | Offen |

### 2.3 Entwicklungsstand

Die Inventaridee ist ein Erweiterungswunsch. App-Installation, Scan, API-Implementierung oder Deployment sind für die historische Ausgangslage nicht belegt. Ein Plan oder Zeitraster ersetzt keinen Ausführungsnachweis.

## 3. Endgültige Anforderungen und Entscheidungsregister

| ID | Anforderung / Entscheidung | Status | Begründung / Konsequenz |
|---|---|---|---|
| Z-01 | Zebra sinnvoll in NetCore-Tetra einsetzen | **beschlossen/geplant** | Realistische Nutzung statt unbelegter Integrationsversprechen |
| Z-02 | Sofortmöglichkeiten müssen mit nur einer Basisstation funktionieren | **beschlossen/geplant** | Spätere ausdrückliche Korrektur hat Vorrang vor der früheren Lighthouse-Annahme. |
| Z-03 | Lighthouse-Inventar als spätere Möglichkeit erhalten | **Idee** | Als spätere Richtung vorgeschlagen; kein verbindlicher Vollausbau festgelegt. |
| Z-04 | Keine neue VPN-Pflicht aus der Planung ableiten | **beschlossen/geplant** als Korrektur | Eine zusätzliche VPN-Voraussetzung wurde ausdrücklich zurückgewiesen. Die frühere Formulierung „Sync wenn Online/VPN“ ist kein festgelegtes Architekturmerkmal. |
| Z-05 | Umsetzungsauftrag zusammenfassen; Englisch ist zulässig | **beschlossen/geplant** | Gewünscht; vollständiger Originaltext fehlt |
| Z-06 | Modell TH57/TC57 nicht stillschweigend festlegen | **offen** | Geräte- und Entwurfsbezeichnung unterscheiden sich; geprüfte technische TC57-Quellen gelten nur unter dieser Modellannahme. |
| Z-07 | Vorhandenes Asset Management für eine Fortsetzung berücksichtigen | **neuer Roadmap-Kandidat aus R1** | Am Prüfdatum ist ein Asset-Dienst vorhanden. Das ist eine technische Folgerung dieser Archivprüfung, keine rückwirkende Festlegung im historischen Planungsstand. |

Nicht endgültig festgelegt wurden Datenbanktechnologie, native App gegenüber Browser/PWA, Asset-Nummernplan, Labelabmessungen, produktive Rollen, Exportberechtigungen, Offlinekonfliktregeln und konkrete Einführungstermine.

## 4. Historischer Lighthouse-Inventarvorschlag

Der Lighthouse-Ansatz ist ein **Konzeptvorschlag**. Datenmodell, APIs und Sprints beschreiben eine mögliche Erweiterung.

### 4.1 Zielumfang und Assetarten

- Zentral verwaltete Hardware: Nodes/Basisstationen, SDRs, Antennen, USVs beziehungsweise Netzteile, Kabel, Koffer, Fahrzeuge, SIMs und sonstiges Material.
- QR-/Barcodegestützte Erfassung und Suche mit dem Zebra.
- Optional offline erfassen und später synchronisieren.
- Physische Assets mit NetCore-Nodes und Statusinformationen verknüpfen.
- Lighthouse als gedachte Verwaltungsoberfläche; Watchtower als gedachte Statusquelle. Eine API oder implementierte Komponente dieser Namen wurde in der Planung nicht nachgewiesen.

Die frühere Formulierung „GSSI-Standorte“ ist fachlich unscharf. Eine GSSI ist eine Gruppenidentität, kein physischer Standort. Eine spätere Umsetzung muss Standort, Asset, Basisstation und Gruppenbezug separat modellieren.

### 4.2 Vorgeschlagenes Datenmodell

| Bereich | Historisch vorgeschlagene Felder |
|---|---|
| Identität | `id` als UUID, `name`, `serial`, `qr_code` oder `nasset:`-URI |
| Typisierung | `kind`: `node`, `sdr`, `antenna`, `psu`, `case`, `sim`, `misc` |
| Organisation | `owner`, `custodian`, `tags` |
| Zustand | `status`: `in_service`, `spare`, `repair`, `retired` |
| Ort | `location` als Text, optional `geo` mit Breite/Länge |
| Systembezug | `related_node_id` optional |
| Dokumentation | `attachments`, `notes`, `created_at`, `updated_at` |
| Lifecycle/Audit | `asset_id`, `event`, `meta` als JSON, `ts`, `user` |
| Ausgabe/Rückgabe, später | `asset_id`, `to_user` / `to_team`, `due_date`, `condition_in`, `condition_out` |

Als Pflichtfelder wurden später im Vorschlag Typ, Status, Standort und Seriennummer beziehungsweise QR-Kennung genannt. Ein verbindliches Schema und die Behandlung von Material ohne Seriennummer wurden nicht beschlossen.

### 4.3 QR- und Labelvorschlag

Historische QR-Nutzlast:

```text
nasset:9f5d6b2e-7c11-4b38-9b3e-8fbe2b0d2a41
```

Die UUID sollte stabil sein und keine Seriennummer im Klartext erfordern. Auf dem Label waren zusätzlich Name, Typ, Seriennummer und Support-URL vorgesehen. Optionale Farbcodes: grün für `in_service`, gelb für `spare`, rot für `repair`.

**Offen:** Ein `nasset:`-Schema benötigt eine explizite Auswertung im Client oder eine passende App-Verknüpfung. Ein beliebiger Browser oder die geprüfte Inventarsuche kann daraus nicht automatisch eine Asset-Detailansicht ableiten. Gedruckte Statusfarben veralten bei Zustandsänderungen; ein unveränderliches Label und ein live abgefragter Status müssen bei der Umsetzung voneinander getrennt werden. Beides sind Folgerungen dieser Archivprüfung.

### 4.4 Vorgeschlagene Arbeitsabläufe

| Ablauf | Historischer Vorschlag | Nachweisstatus |
|---|---|---|
| Erfassen | „+ Asset“, Scan, Vorbelegung von Seriennummer/Typ, Formular, Speichern | **Idee**; Scan allein liefert noch keine verlässliche Typzuordnung. |
| Finden | „Scan & Jump“ direkt zur Detailseite; Volltext, Status-/Standort-/Typ-/Tagfilter | **Idee** |
| Wartung | Service beginnen/abschließen, Notiz und Foto, Audit-Eintrag | **Idee** |
| Zuordnung | SDR/Netzteil/Antennen usw. einem Node zuordnen; Quick-Link im Node | **Idee** |
| Inventur | Geplante Inventurliste; Scans mit „gesehen“-Zeitstempel, eventuell GPS | **Idee** |
| Ausgabe/Rückgabe | Zuweisung an Person/Team, Fälligkeit, Zustandsdokumentation | **Idee**, in späterer Phase vorgeschlagen |

### 4.5 Gedachte WebUI

- Neuer Hauptbereich **Inventar**.
- Tabelle: Name, Typ, Status, Standort, Node, zuletzt gesehen.
- Filter und Tags, Massenaktionen, CSV-Export.
- Detailseite mit Stammdaten, Node-Verknüpfung, Audit, Anhängen und Aktionen.
- Schnellerfassungsformular für Serienanlage.
- Node-Detail mit Hardware-Tab.
- Gedachtes Watchtower-Badge bei abweichender SDR-Seriennummer.

### 4.6 Historische API-Beispiele

Die folgenden Routen waren **Entwurfsbeispiele**, keine in der Planung ausgeführten Requests:

| Methode | Vorgeschlagene Route | Zweck |
|---|---|---|
| `GET` | `/api/assets?query=&status=&kind=` | Suche und Filter |
| `POST` | `/api/assets` | Assetanlage, eventuell Bild-Upload-Token |
| `GET` | `/api/assets/{id}` | Detail |
| `PATCH` | `/api/assets/{id}` | Änderung |
| `POST` | `/api/assets/{id}/events` | Lifecycle-Ereignis |
| `POST` | `/api/assets/bulk` | CSV-/JSON-Serienimport |
| `GET` | `/api/assets/{id}/label` | QR-Label als PDF |

Am Prüfdatum haben die tatsächlich implementierten Asset-Routen ein **`/api/v1/`-Präfix**, ein anderes Feldschema und `PUT` für Änderungen. Die historische Liste darf nicht als geprüfte API-Dokumentation verwendet werden.

### 4.7 Vorgeschlagene Rollen und Rechte

| Rolle | Historischer Vorschlag |
|---|---|
| Viewer | Lesen |
| Operator | Erfassen und bearbeiten; kein Ausmustern oder Export |
| Admin | Alle Funktionen sowie Massenimport/-export |
| `sys.core.master` | API-/Automationszugang |

Zusätzlich wurden JWT, optional 2FA für Massenfunktionen sowie später Virenscan/Quarantäne für Anhänge genannt. Das sind **Ideen**, keine nachgewiesene IAM-Architektur. Insbesondere wurde `sys.core.master` weder als vorhandene Rolle noch als notwendige Berechtigung belegt.

### 4.8 Clientvarianten, Automatisierungen und Reporting

**Client A:** PWA, Kioskbetrieb, Kamerascan; der damalige Entwurf nennt zusätzlich „Zebra EMDK Intent“ als vermeintlichen Browserweg. Diese Formulierung ist zu korrigieren, siehe Abschnitt 9.

**Client B:** Native Android-App mit Zebra-Anbindung, Room-Datenbank, Sync-Worker und Scanprofilen. Vorgeschlagene Funktionen: Scan/Lookup, Quick-Create, Inventursession, Fotos, Offlinequeue.

**Automatisierungen:** Meldung bei Reparaturstatus an Technik/Operator-Kanal, Seriennummerabweichung markieren, Standortscan mit GPS in Karte übernehmen. Eine Nachrichtenauslösung wurde weder eingerichtet noch getestet.

**Reporting:** Bestand nach Typ/Status, Wartungshistorie/MTBF, Inventurfortschritt je Standort. MTBF setzt definierte Fehlerereignisse und Betriebszeiten voraus; eine Wartungsakte allein beweist noch keine belastbare MTBF.

**Praktische Hinweise aus dem alten Vorschlag:** wenige sinnvolle Pflichtfelder, klare Standort-Tags statt GPS für Regal/Rack, dauerhaft haftende industrielle Labels. Konkrete Labelprodukte, Maße und Tests wurden nicht festgelegt.

### 4.9 Historischer Sprintplan

| Phase | Ursprünglich vorgeschlagener Inhalt | Geprüfte Einordnung |
|---|---|---|
| Sprint 1 | DB, CRUD, Liste/Detail, QR-Label, PWA-Scanner, Quick-Create, Node-Bezug, CSV-Import | Vorschlag; damalige Aufwandsschätzung 1–2 Wochen abhängig vom Team, ohne bestätigte Voraussetzungen |
| Sprint 2 | Audit/Lifecycle, Fotos, Inventurmodus, Filter/Export, Watchtower-Abweichungsbadge | Vorschlag |
| Sprint 3 | Ausgabe/Rückgabe, Reporting, Offline-PWA oder native App, Push/Automatisierungen | Vorschlag |

Die Reihenfolge passt nicht unverändert zum geprüften Repository: Ausgabe/Rückgabe, Audit und CSV-Export sind bereits im Asset-Dienst vorhanden; QR-/Scan-/Offlinefunktionen sind dagegen noch nicht belegt. Die später bestätigte historische Ausgangslage ohne Lighthouse macht einen vollständigen damaligen Sprint-1-Rollout zudem nicht plausibel.

## 5. Ideen für die damalige Einzelbasisstation

Für die Einzelbasisstation sind folgende Möglichkeiten in der ergänzenden Ideensammlung H2 festgehalten. Ihre konkrete Ausgestaltung und Umsetzung sind offen.

| Idee | Gedachte Nutzung mit einer Basisstation | Voraussetzungen und Grenzen |
|---|---|---|
| Mobiler Browserzugriff | Lokales Dashboard ansehen und vorhandene Funktionen bedienen | Netzwerkerreichbarkeit und Dashboard auf der TBS; damals kein konkret geprüfter Port oder Gerätebrowser belegt |
| SSH/Logs | Pi/PC erreichen, Logs ansehen, Dienste/Skripte bei Bedarf bedienen | SSH-Dienst, passende Berechtigung und Android-SSH-Client; genannt wurden Termius und JuiceSSH, aber keine Installation bestätigt |
| Lokales Inventar | Barcodes/Seriennummern erfassen; JSON/CSV später übernehmen | Lokale Erfassungsdatei/App; keine belegte automatische NetCore-Synchronisierung |
| Audioaufnahme | Aufnahme erstellen, übertragen und später als TETRA-Durchsage abspielen | Übertragungsweg, unterstütztes Audioformat und vorhandener Playout-Pfad; kein Live-Audio-/PTT-Beleg |
| HTTP-/SSH-Buttons | Wiederkehrende Status-/Test-/Verwaltungsaktionen | Genannt: HTTP Shortcuts, Tasker, Automate, MacroDroid; konkrete kompatible Verträge fehlten |
| GPS-/Reichweitenlogging | Feldnotizen mit Ort/Zeit und beobachtetem Funkverhalten | Keine automatische Messung der TETRA-Empfangsqualität aus dem Zebra belegt; Endgerät beziehungsweise externe Messquelle erforderlich |
| Lokale HTML-/PWA-Oberfläche | JSON-Zustände in einer kleinen lokalen Bedienoberfläche anzeigen | Noch zu erstellende App/Serveranbindung; „PWA“ ist keine vorhandene Implementierung |
| PTT-Apps | IP-basierte Sprachkommunikation, eventuell später Gatewayanbindung | Keine native TETRA-Funktion und keine nachgewiesene Brücke zur Funkzelle |
| Offline-Wartungsformulare | Notizen und Checklisten im Feld erfassen | Spätere manuelle oder implementierte Übernahme; Konflikt- und Synchronisierungslogik fehlt |

H2 nennt zusätzlich eine **Flask-/FastAPI-Mini-API** mit `/status`, `/play/<file>` und `/log`, sowie beispielhafte Aktionen `/play alarm.wav`, `/reboot node`, `/status check` und SDS-Kommandos `/play`, `/reboot`, `/test`. Diese Namen bleiben historische Konzeptbeispiele. Es gibt für diese Planung **keinen Nachweis ihrer Implementierung, Installation oder erfolgreichen Ausführung**. Ein Zebra kann solche SDS-Kommandos auch nicht allein wegen seiner Android-Apps direkt als TETRA-Endgerät senden.

Für eine Fortsetzung muss zuerst der installierte TBS-Stand mit dem geprüften Repository abgeglichen werden. Funktionen am Prüfcommit sind kein Beweis, dass dieselben Funktionen bereits damals oder am Prüfdatum auf der Basisstation laufen.

## 6. Am Prüfdatum überprüfter Repository-Stand

### 6.1 Prüfumfang

Der Stand von `Archiving` am Prüfcommit R1 wurde untersucht. Insbesondere wurden gelesen:

- `system-backend/asset-management/README.md`
- `system-backend/asset-management/docs/architecture.md`
- `system-backend/asset-management/src/netcore_asset_management.py`
- `system-backend/asset-management/web-ui/index.html`
- Konfigurationsvorlage, generierte Open-Lab-Konfiguration, systemd-Unit und Installationsskript des Asset-Dienstes
- `system-backend/asset-management/tests/open_lab_smoke.md`
- `Docs/PHASE_10_ASSET_DEVICE_USER_MANAGEMENT.md`
- relevanter Serviceeintrag in `deploy/open-lab/inventory.example.toml`
- relevante Dashboard-/Audiohandler in `crates/tetra-entities/src/net_dashboard/server.rs`
- Featuredeklarationen in `bins/bluestation-bs/Cargo.toml` und ausgewählte Dashboardwerte aus `config.toml.fallback`

Eine Suche in textuellen Code-, Konfigurations- und Dokumentationsdateien außerhalb `Docs/archive/` ergab keine Treffer für `Lighthouse`, `Watchtower`, `TC57`, `TH57`, `DataWedge`, `nasset:` oder das historische Präfix `/api/assets`. Keine Android-Quellen (`.java`, `.kt`, `AndroidManifest.xml`) oder `.apk` wurden gefunden. Das ist ein begrenzter Suchbefund dieses Branchstands, kein Beweis gegen externe oder frühere Anwendungen.

### 6.2 Vorhandene Architektur

**Implementiert:** Ein eigenständiger Python-Dienst **Asset Management, Phase 10** mit eigener WebUI und HTTP-/JSON-API. Die Persistenz erfolgt in JSON beziehungsweise NDJSON, nicht in einer in der Planung vorgeschlagenen relationalen DB.

| Komponente | Zuständigkeit / Datenweg |
|---|---|
| Asset Management | Physische Assets, Inventar-/Seriennummer, Firmware/Codeplug, Personen, Ausgaben/Rückgaben und Wartungsakten |
| Subscriber Core | Autorität für ISSI-Freigaben und Dienstberechtigungen; vom Asset-Dienst lesend abgefragt |
| Mobility Core | Autorität für Registrierung/Serving-TBS; vom Asset-Dienst lesend abgefragt |
| Task Workflow | Wartungsauftrag per `POST /api/v1/tasks` aus Asset Management; Verknüpfung über `task_id` |
| MQTT | Ereignisse und Zustandsveröffentlichung über `mosquitto_pub` |
| WebUI | Deutsche Tabellen und Dialoge für Assets, Personen, Ausgaben, Wartungen, Ereignisse und Upstreamabgleich |

Ein Asset wird im aktuellen `reconcile()` über seine **ISSI** mit Subscriber-/Mobility-Daten verknüpft. Das ist eine Netzstatusaufnahme (`network_snapshot`), kein implementierter physischer Asset-Baum und kein Watchtower-Seriennummervergleich. Der Asset-Dienst schreibt bei diesem Abgleich keine Subscriberprofile oder Mobility-Routen zurück.

RUI-/RUA-Angaben der Personen sind Metadaten; das aktuelle Schema setzt `pin_stored = false`. Der Asset-Dienst ist kein Funk-Anmeldedienst. Zugangsdaten werden in dieses Archiv nicht übernommen.

### 6.3 Tatsächliches Asset-Schema

Schema: `netcore-asset-v1`; Zustandsschema: `netcore-asset-management-state-v1`.

| Feldgruppe | Tatsächlich vorhandene Felder |
|---|---|
| Identität | `asset_id`, `inventory_id`, `schema` |
| Typ/Zustand | `kind`, `status` |
| Geräteinformationen | `manufacturer`, `model`, `serial_number`, `firmware_version`, `codeplug_version` |
| Funkbezug | `device_tei`, `issi` |
| Organisation/Ort | `organization`, `location`, `tags` |
| Bearbeitung | `notes`, `created_at`, `updated_at` |
| Vorgänge | `current_assignment_id`, `network_snapshot` |

Aktuelle Typen: `tetra_radio`, `tbs`, `server`, `rack`, `rf_component`, `power`, `gateway`, `vehicle`, `accessory`, `tool`, `generic`.

Aktuelle Assetzustände: `in_stock`, `assigned`, `maintenance`, `repair`, `retired`, `lost`. Wartungszustände: `planned`, `in_progress`, `completed`, `cancelled`.

Eine ID kann frei vorgegeben, aus einer Inventarnummer abgeleitet oder als UUID erzeugt werden. Das Schema erzwingt keine ausschließliche UUID-Nutzung. `normalize_asset()` übernimmt keine dedizierten Felder `qr_code`, `geo`, `attachments` oder `related_node_id`.

### 6.4 Historische Begriffe auf das geprüfte Schema abbilden

Diese Tabelle beschreibt **zu prüfende Migrationsentscheidungen**, keinen bereits umgesetzten Import:

| Historischer Begriff | Geprüfter Bezug | Verbleibende Entscheidung |
|---|---|---|
| `id` | `asset_id` | Stabilität und Zulässigkeit vorhandener IDs festlegen |
| `serial` | `serial_number` | Dublettenregeln auch für Änderungen und Import durchsetzen |
| `owner` / `custodian` | `organization`, Personen, Ausgaben | Eigentümer, Verantwortlicher und aktueller Entleiher unterscheiden |
| `in_service` | kein exakt gleicher Status | Einsatzbereitschaft ist nicht identisch mit Ausgabezustand. |
| `spare` | eventuell `in_stock` | Nur passend, wenn Ersatzbestand und verfügbare Ausgabe gleich behandelt werden sollen |
| `repair` / `retired` | `repair` / `retired` | Ähnliche Zustände, aber Lebenszyklusregeln abnehmen |
| Node-/SDR-/Antennentyp | `tbs`, `rf_component`, gegebenenfalls Tags | Feinere Hardwaretypisierung festlegen |
| `related_node_id` | kein dediziertes Feld | Physische Zuordnung und Netzstatus nicht vermischen |
| `qr_code`, `geo`, `attachments` | kein dediziertes Feld | Schema/Storage und API müssten gezielt erweitert werden. |

### 6.5 Geprüfte HTTP-Schnittstellen

| Methode | Route | Implementierter Zweck |
|---|---|---|
| `GET` | `/` | Eigenständige Asset-WebUI |
| `GET` | `/api/v1/status` | Kennzahlen, MQTT-/Upstreamstatus |
| `GET` | `/api/v1/assets?q=&kind=&status=` | Assetliste; `q` durchsucht JSON-Repräsentation, zusätzliche Typ-/Statusfilter |
| `POST` | `/api/v1/assets` | Assetanlage |
| `GET`, `PUT`, `DELETE` | `/api/v1/assets/{asset_id}` | Detail, Änderung, Löschen; keine `PATCH`-Implementierung |
| `GET`, `POST` | `/api/v1/persons` | Personenliste und Anlage |
| `GET`, `PUT`, `DELETE` | `/api/v1/persons/{person_id}` | Personendetail, Änderung, Löschen |
| `GET`, `POST` | `/api/v1/assignments` | Ausgabeübersicht und Ausgabe |
| `POST` | `/api/v1/assignments/{assignment_id}/return` | Rückgabe |
| `GET`, `POST` | `/api/v1/maintenance` | Wartungsübersicht und Anlage |
| `PUT` | `/api/v1/maintenance/{record_id}` | Wartungsänderung/Abschluss |
| `POST` | `/api/v1/assets/{asset_id}/maintenance-task` | Task-Workflow-Auftrag erzeugen und Wartung verknüpfen |
| `GET` | `/api/v1/events?limit=` | Ereignisauszug; bis 1000 pro Antwort |
| `GET` | `/api/v1/upstreams` | Upstreamgesundheit und Snapshot |
| `POST` | `/api/v1/reconcile` | Lesenden Netzstatusabgleich auslösen |
| `GET` | `/api/v1/export.json` | Assets, Personen, Ausgaben und Wartungen als JSON |
| `GET` | `/api/v1/export/assets.csv` | CSV mit ausgewählten Assetspalten |
| `POST` | `/api/v1/import` | JSON-Import mit ergänzendem oder ersetzendem Verhalten; kein nativer CSV-Uploadhandler |
| `GET` | `/health/live`, `/health/ready` | Prozess-/Bereitschaftsstatus |
| `GET` | `/metrics` | Prometheus-Textmetriken |
| `GET` | `/openapi.json` | Grobe Pfadübersicht; enthält keine vollständigen Request-/Responseschemas |

Bei schreibenden JSON-Anfragen begrenzt `json_body()` den Body auf 2.000.000 Bytes. Das ist keine Foto-Uploadimplementierung.

`X-NetCore-Actor` wird als Audit-Akteur angenommen; es ist **keine Authentifizierung**. Die aktuelle README beschreibt den Dienst ausdrücklich als **OPEN LAB: kein Login, keine Tokens, kein TLS**. Die historische Rollen-/JWT-/2FA-Idee ist damit nicht umgesetzt. Dies ist eine konkret beobachtete Abweichung, kein Nachweis einer produktiven Zugriffsabsicherung.

### 6.6 Tatsächliche WebUI und Scanneranschluss

Die WebUI enthält ein Suchfeld `id="filter"` mit `oninput="render()"` sowie Eingabefelder unter anderem für `inventory_id`, `serial_number` und `location`. Sie lädt Asset-/Personen-/Vorgangslisten und aktualisiert sie alle zehn Sekunden. Die Filterung der angezeigten Assets findet clientseitig über ihre JSON-Darstellung statt.

**Technische Folgerung, noch nicht auf dem Zebra getestet:** Ein passend eingerichteter Scanner mit Tastaturausgabe kann eine Inventar- oder Seriennummer in ein fokussiertes Formular-/Suchfeld schreiben. Daraus folgt noch kein Scan-und-Sprung-Ablauf, keine Inventursession und keine sichere automatische Typzuordnung.

In dieser Asset-WebUI wurden weder Service Worker noch IndexedDB-Synchronisierung gefunden. Themeeinstellungen in `localStorage` sind keine Offline-Inventardatenbank. Es ist keine installierbare Zebra-PWA oder native Scanner-App am Prüfcommit belegt.

### 6.7 Basisstationsfunktionen als geprüfte Alternative zur Mini-API

Der geprüfte TBS-Code enthält bereits eine Dashboardoberfläche und Audio-HTTP-Handler. Im Beispiel `config.toml.fallback` stehen für das Dashboard `bind = "0.0.0.0"` und `port = 8080`; das sind **Repository-Beispielwerte**, keine aus dem historischen Planungsstand bestätigten Livewerte.

| Geprüfter Codepfad | Aussage |
|---|---|
| `GET /api/btsinfo` | Zell-/RF-Identität aus laufender Konfiguration; Code gelesen, hier nicht auf einer TBS getestet |
| `GET /api/edge-fallback` | Lokale Autonomie-/Backend-/Replayzustände; Code gelesen, kein Betriebsnachweis |
| `GET /api/audio/status` | Audio-Player-Status bei vorhandener Unterstützung |
| `GET /api/audio/sources`, `/api/audio/browse`, `/api/audio/preview` | Quellen, Verzeichnisse und Vorschau |
| `POST /api/audio/play`, `/api/audio/stop` | Medien-/Recording-Playout und Stop; keine Live-Mikrofon-PTT-Anbindung dadurch belegt |

Für `/api/audio/play` erwartet der gelesene Handler JSON mit `target_type` (`group` oder `individual`), numerischem `target_id`, optional `priority` sowie Quellangaben. Für Medien etwa `source_type = "media"`, optional `source_id` und `path`. Gruppen-/Teilnehmerwahl muss mit der realen Installation übereinstimmen.

Die Audiohandler hängen am Compile-Feature `audio-player` und an einem verfügbaren Handle. `bins/bluestation-bs/Cargo.toml` führt `asterisk`, `recording` und `audio-player` in seinen Defaultfeatures. Ein tatsächlich installiertes Binary kann abweichen. Der Code liefert auch Fehler für fehlenden Audio-Player beziehungsweise nicht einkompilierte Unterstützung.

Die Dashboardauthentifizierung nutzt bei entsprechender Konfiguration eine Anmeldung unter `POST /api/login` und ein Sessioncookie `fs_session`. Die OPEN-LAB-Aussage des Asset-Dienstes darf nicht pauschal auf das TBS-Dashboard übertragen werden. Zugangsdaten und Sessionwerte werden hier nicht dokumentiert.

**Folgerung für eine Fortsetzung:** Vor einem zusätzlichen Flask-/FastAPI-Server prüfen, ob das installierte TBS-Dashboard die gewünschte Funktion bereits abdeckt. Der alte Mini-API-Vorschlag bleibt historisch erhalten; ein Parallelneubau ist daraus nicht erforderlich.

## 7. Relevante Parameter, Pfade und Abhängigkeiten

| Parameter | Überprüfter Repository-Stand | Einordnung |
|---|---|---|
| Dienst | `netcore-asset-management.service` | Vorhandene Unit |
| Ausführbare Datei | `/usr/local/bin/netcore-asset-management` | Installationsziel des Python-Skripts |
| Konfiguration | `/etc/netcore/asset-management.toml` | Zielpfad |
| HTTP-Listener | `0.0.0.0:8290`, TCP | Vorlagenwert, kein Live-Socket am Prüfdatum erhoben |
| WebUI | `http://<LXC-IP>:8290/` | Dokumentiertes Zugriffsmuster |
| Open-Lab-Beispielhost | `10.0.20.32` | Inventory-Beispiel; nicht Jans tatsächlich gemessene IP |
| Zustand | `/var/lib/netcore-asset-management/state.json` | Atomarer Austausch einer JSON-Datei |
| Ereignisse | `/var/lib/netcore-asset-management/events.ndjson` | Append-Log |
| Audit | `/var/lib/netcore-asset-management/audit.ndjson` | Append-Log; nicht kryptografisch manipulationssicher belegt |
| Ereignispuffer | `event_history_limit = 3000` | In-Memory-Verlauf aus der Vorlage |
| Abgleich | `upstream_sync_interval_secs = 60` | Vorlage; Code begrenzt den Rhythmus nach unten auf zehn Sekunden |
| MQTT | `127.0.0.1:1883`, `topic_prefix = "netcore/v1"` | Vorlagenwert; kein Brokerzugriff in der Archivprüfung |
| MQTT-Clientwerkzeug | `mosquitto_pub` | Tatsächlicher subprocess-Aufruf; `mosquitto-clients` wird installiert. |
| MQTT-Ereignisse | `netcore/v1/events/<event_type mit / statt .>` | Beispiel `asset.created` → `netcore/v1/events/asset/created` |
| MQTT-Zustand | `netcore/v1/state/assets/{asset_id}` | Retained-Veröffentlichung; entsprechende Zustände auch für weitere Objekte |
| Subscriber Core | Port `8100`; `/api/v1/subscribers`, `/api/v1/observed` | Read-only-Reconcile |
| Mobility Core | Port `8090`; `/api/v1/subscribers` | Read-only-Reconcile |
| Task Workflow | Port `8280`; `/api/v1/tasks` | Optionaler schreibender Wartungsauftrag |
| Task-Gruppenwert | `default_gssi = 15201` | Nur Vorlagenwert; keine festgelegte Betriebsgruppe |
| Python | `tomllib`, Standardbibliothek-HTTP-Server | Aus dem Quelltext mindestens Python 3.11 erforderlich |
| systemd | `User=root`, `Group=root`, `Restart=on-failure`, `RestartSec=2` | Tatsächliche Unitwerte; nicht im Archiv geändert |
| Inventory-Abhängigkeiten | `iot-gateway`, `subscriber-core`, `mobility-core`, `task-workflow` | Deployment-Reihenfolge laut Serviceeintrag; im Code direkte Upstreams nur wie oben |

Die generierte Open-Lab-Konfiguration verweist für Subscriber/Mobility/Task auf `10.0.20.12:8100`, `10.0.20.11:8090` und `10.0.20.31:8280`. Die generische Vorlage verwendet dafür Loopback. Bei einem getrennten LXC sind Loopbackadressen kein automatisch gültiger Ersatz für die Diensthosts.

SSH, HTTP-/JSON und gegebenenfalls MQTT sind die denkbaren Verwaltungswege dieses Zebra-Themas. Eine TETRA-Luftschnittstelle, eine PEI-Verbindung oder ein direktes Zebra-SDS-Protokoll wurde nicht eingerichtet. Die verfügbaren ETSI-Normen ändern daran nichts.

## 8. Befehle und Abläufe: tatsächlich ausgeführt oder nur vorgesehen

### 8.1 Im historischen Arbeitsstand

Keine vollständig zugänglichen Shellausgaben, erfolgreichen Installationsbefehle oder Tests für Zebra/Lighthouse/Inventar sind enthalten. Die API-Liste und die H2-Kommandonamen waren Beispiele, keine Ausführungsnachweise. Ein belastbarer historischer Reparaturablauf ist nicht vorhanden.

### 8.2 In dieser Archivprüfung tatsächlich erfolgreich ausgeführt

In einer eigenen, sauberen Arbeitskopie des angegebenen Repositorys:

```bash
git fetch origin Archiving
git reset --keep origin/Archiving
git rev-parse HEAD
git status --short
```

Der erste Codeabgleich ergab den in Abschnitt 1 angegebenen Prüfcommit. Die bereitgestellten Anhänge wurden mit `pdfinfo` und `pdftotext -f 1 -l 1` inventarisiert; SHA-256 wurde aus den Originalbytes berechnet. Diese Prüfung ist keine vollständige Normeninterpretation.

Der vorhandene Asset-Quelltext wurde mit Python 3.12.14 syntaktisch geparst und in einer isolierten lokalen HTTP-Instanz mit temporärer Persistenz getestet. Es gab keinen Aufruf eines Projektinstallers, keinen systemd-Rollout und keinen Zugriff auf Jans Netzwerk.

### 8.3 Vorhandener Installationsweg, hier nicht ausgeführt

Der gelesene Installer ist:

```bash
sudo bash system-backend/asset-management/install/install.sh
```

Er installiert laut Code `python3`, `mosquitto-clients` und `ca-certificates`, kopiert Programm und Unit, legt eine Konfiguration nur an, falls diese noch fehlt, ruft den gemeinsamen LXC-Endpointkonfigurator auf und startet die Unit mit systemd. Das Skript wurde in dieser Arbeit **nur gelesen**. Es ist keine historische erfolgreiche Installation aus dieser Planung.

Quellen für spätere Update-/Rückbauprüfung: `install/update.sh`, `install/configure-openlab.sh` und `install/uninstall.sh` im Asset-Modul. Auch diese Pfade sind kein durchgeführter Rollout.

### 8.4 Mögliche spätere Diagnose, hier nicht auf Zielsystemen ausgeführt

```bash
systemctl status netcore-asset-management.service --no-pager
journalctl -u netcore-asset-management.service -b --no-pager
curl -fsS http://127.0.0.1:8290/health/live
curl -fsS http://127.0.0.1:8290/health/ready
curl -fsS http://127.0.0.1:8290/api/v1/status
```

Diese Befehle gelten für den **Asset-Host**, wenn der Listener entsprechend eingerichtet ist. Sie beweisen ohne Ausführung weder Erreichbarkeit vom Zebra noch Upstreamfunktion. Für das TBS-Dashboard muss dessen tatsächliche Adresse, Port- und Anmeldungskonfiguration ermittelt werden.

## 9. Fehler, Diagnose und ersetzte Ansätze

| Punkt | Diagnose / Ursache | Korrektur oder verbleibende Aufgabe |
|---|---|---|
| Lighthouse als bereits verfügbare Grundlage | Frühere früherer Entwurf baut auf einer nicht vorhandenen Komponente auf. | Durch ausdrückliche spätere Korrektur ersetzt: damals nur einzelne Basisstation. |
| Ungewünschter VPN-Abzweig | Zusätzliche VPN-Voraussetzung ausdrücklich zurückgewiesen; ursprünglicher Zusammenhang fehlt. | Keine VPN-Pflicht in die Zebra- oder Inventarplanung hineinlesen. |
| TH57 ungefragt zu TC57 geändert | Unterschiedliche Gerätebezeichnungen ohne Nachweis | Modell am realen Gerät prüfen; geprüfte Zebra-Dokumentation ist unter TC57-Annahme eingeordnet. |
| „EMDK Intent“ direkt in einer PWA | Native Scanner-/Intent-Anbindung und Browsermöglichkeiten wurden vermischt. | DataWedge-Tastaturausgabe kann in fokussierte Webfelder schreiben; Intent Output benötigt einen Android-Empfänger beziehungsweise eine bewusst gebaute App-Brücke. Kein EMDK-/Intent-Zugriff aus beliebigem Browser-JavaScript ableiten. |
| Typ-/Seriennummer-Autofill durch beliebigen Scan | Ein Scan liefert die Nutzlast, nicht automatisch verlässliche Inventarmetadaten. | Payloadformat, Lookup und Pflichtfelder definieren. |
| „GPS = Funkabdeckung“ | Zebra-Ortsdaten allein enthalten keine nachgewiesene TETRA-Signalqualität. | Messquelle und Zuordnung zu Ort/Zeit getrennt festlegen. |
| PTT-App als TETRA-Anbindung | IP-Audio und Funkzellenanbindung sind verschiedene Schnittstellen. | Gateway, Ruf-/Floorsteuerung und Audioformat müssten gesondert belegt werden. |
| Alter API-Entwurf als aktuelle Anleitung | `/api/assets`, `PATCH`, `/bulk` und Labelhandler passen nicht zur Implementierung. | Geprüfte `/api/v1`-Routen und `PUT` verwenden; fehlende Funktionen explizit planen. |
| Seriennummer-Dubletten beim Bearbeiten | `create_asset()` prüft Dubletten; `update_asset()` enthält diese Prüfung nicht. | In der Archivprüfung reproduziert: `POST` einer Dublette → 400; `PUT` auf identische Seriennummer eines anderen Assets → 200. Fix als Roadmap-Kandidat, hier nicht implementiert. |
| `/label` liefert scheinbar Erfolg | GET-Handler nimmt bei `/api/v1/assets/...` nur das ID-Segment und prüft überzählige Segmente nicht. | Reproduziert: `/api/v1/assets/{id}/label` → 200 mit normalem Asset-JSON. Kein PDF-Label. Routing strikt machen und Labelhandler gezielt ergänzen. |
| `qr_code` und `geo` verschwinden bei Anlage | Sie gehören nicht zum normalisierten aktuellen Schema. | Im lokalen Test nicht übernommen; Schema bewusst erweitern oder Import abbilden. |
| JSON-Import ist nicht der alte CSV-Serienimport | `/api/v1/import` nimmt JSON-Strukturen an. | CSV-Konvertierung/Feldmapping entwickeln; bestehender Import umgeht im gelesenen Code die normale Objektvalidierung, daher Konsistenzprüfung als Aufgabe aufnehmen. |
| OPEN LAB statt historischer RBAC-Idee | Der aktuelle Asset-Dienst besitzt keine authentifizierten Rollen; der Akteurheader ist frei setzbar. | Mit der zentralen IAM-Planung abstimmen, bevor produktive Rechte behauptet werden. |
| „MQTT verbunden“ im isolierten Test | Bei deaktiviertem MQTT setzt der Code das Flag `mqtt_connected` auf wahr, ohne Brokerkontakt. | Lokalen Smoke-Test nicht als MQTT-/Readiness-Abnahme ausgeben. |

Die beiden lokal reproduzierten Codeprobleme bleiben **offen**.

### 9.1 Primärquellen zur geprüften Scanner-Einordnung

Zebras DataWedge-Dokumentation beschreibt **Keystroke Output** als Tastatureingaben an die zugeordnete Vordergrundanwendung, einschließlich TAB/ENTER. Die Quick-Start-Anleitung des TC57 nennt ein fokussiertes Textfeld als Ziel eines Scans. Das stützt einen Browser-Formulartest, aber keinen nachgewiesenen NetCore-Workflow.

**Intent Output** übergibt Android-Intentobjekte an Activities, Services oder Broadcast-Receiver. Ein gewöhnliches PWA-Skript ist kein solcher Empfänger; eine native Brücke müsste eigens gebaut werden. Diese Folgerung korrigiert die frühere „EMDK Intent“-Kurzform.

Der TC57-Produktstand nennt Android, WLAN/WWAN, einen 1D-/2D-Imager, Kameras und NFC. Ein natives TETRA-Funkmodul ist dort nicht spezifiziert. Direkte TETRA-Teilnahme darf daraus nicht angenommen werden. Ob Jans Gerät genau diese Variante ist, bleibt offen.

Quellen, abgerufen am 04.10.2026:

- [Zebra: DataWedge 8.0 – Keystroke Output](https://techdocs.zebra.com/datawedge/8-0/guide/output/keystroke/)
- [Zebra: DataWedge 8.0 – Intent Output](https://techdocs.zebra.com/datawedge/8-0/guide/output/intent/)
- [Zebra: TC57 Quick Start – Scanning](https://docs.zebra.com/us/en/mobile-computers/handheld/tc5-series/tc57-quick-start-guide/scanning.html)
- [Zebra: TC52/TC57 Specification Sheet](https://www.zebra.com/us/en/products/spec-sheets/mobile-computers/handheld/tc52-tc57.html)

Die DataWedge-Seiten erklären die Schnittstelle; sie belegen nicht die auf Jans Gerät installierte Version.

## 10. Durchgeführte Tests und Grenzen

### 10.1 Historische Tests

Im zugänglichen Verlauf ist **kein erfolgreicher Zebra-/Inventar-/Lighthouse-Test** nachgewiesen. Es gibt keine belegte Scannerkonfiguration, kein On-Air-Ergebnis, keine Betriebszeit und keinen ausgeführten Rollout. Der Satz, dass nur die Basisstation vorhanden ist, ist eine Ausgangslage und kein bestandener Funktions-/Funkabnahmetest.

### 10.2 Lokale Prüfungen vom Prüfdatum

Umgebung: Python **3.12.14**, Code R1; kurzlebiger HTTP-Server ausschließlich auf **Loopback und zufälligem Port**, temporäre Dateien; **MQTT, Subscriber Core, Mobility Core und Task Workflow deaktiviert**. Der periodische Integrationsworker wurde nicht gestartet. Synthetische IDs und Seriennummern wurden verwendet, keine Echtdaten. Die Instanz wurde anschließend beendet und die temporäre Persistenz entfernt.

| Prüfung | Tatsächliches Ergebnis |
|---|---|
| Syntax und Laden des Python-Moduls | Erfolgreich |
| Asset-WebUI `GET /` | HTTP 200; Inventarfelder und `/api/v1/assets` im HTML |
| `GET /health/live` | HTTP 200 |
| Assetanlage | HTTP 201; `in_stock` persistiert |
| Person anlegen, Asset ausgeben | HTTP 201; Asset danach `assigned` |
| Rückgabe | HTTP 200; Ausgabe `returned`, Asset wieder `in_stock` |
| Wartung mit Außerbetriebsetzung | HTTP 201; Asset in `maintenance` |
| Wartungsabschluss | HTTP 200; `completed_at` vorhanden, Asset wieder `in_stock` |
| Inventarnummer per `q` suchen | HTTP 200; genau ein Testasset |
| JSON- und CSV-Export | HTTP 200; Testasset enthalten |
| Persistenz neu laden | Eine neue `AssetManagement`-Instanz lädt Asset und Wartung mit richtigem Zustand; Audit-/Eventdateien vorhanden. |
| Anlage mit doppelter Seriennummer | HTTP 400; Abweisung funktioniert. |
| Änderung auf bereits vorhandene Seriennummer | HTTP 200; danach zwei Assets mit gleicher Seriennummer. **Reproduzierbares Problem.** |
| GET auf `{asset_id}/label` | HTTP 200 mit Asset-JSON statt PDF. **Reproduzierbares Routingproblem / fehlende Labelfunktion.** |
| Zusätzliche Felder `qr_code` / `geo` bei Anlage | Nicht in normalisiertes Asset übernommen |
| Suche nach `nasset:dummy` | HTTP 200, null Treffer; kein URI-Lookup dadurch nachgewiesen |

**Diese Tests bestätigen nur den genannten lokalen Teilumfang.** Nicht getestet: Zebra-Hardware, DataWedge, Android-Browser/Touch, Scannerfocus und Suffixe, Kioskmodus, Live-TBS-Dashboard, Audio/Funk, MQTT-Broker, zentrale Upstreams, systemd-Installation, produktive Authentifizierung, CSV-Import, Foto-/Label-/Inventur-/Offlinefunktionen, vollständiger Rust-Build oder langfristiger Parallelbetrieb.

Die vorhandene Datei `tests/open_lab_smoke.md` ist ein Prüfplan für Person/Funkgerät, Ausgabe/Rückgabe, Wartung, Reconcile und MQTT. Ihre Existenz ist kein erfolgreich ausgeführter Gesamttest. Die Integrationsschritte dieses Plans wurden hier nicht auf Zielsystemen nachgeholt.

## 11. Entwicklungs- und Betriebsstand im direkten Vergleich

| Funktion | Historischer Planungsstand | Repository R1 | Tests / Betriebsnachweis |
|---|---|---|---|
| Zebra-Gerät vorhanden | Nach Angabe zum Aufbau | Kein projektspezifischer Client gefunden | Modell/Version nicht verifiziert |
| Einzelbasisstation | Maßgebliche Angabe zum Aufbau | Umfangreicher TBS-Code vorhanden | Kein Livezugriff in diesem Auftrag |
| Lighthouse | Ausdrücklich noch nicht vorhanden | Keine benannte Implementierung gefunden | Nicht getestet |
| Web-Inventar | Nutzeridee + Konzeptentwurf | Eigenständiger Asset-Dienst **implementiert** | Kernabläufe lokal **getestet**; kein Zielbetriebsnachweis |
| Ausgabe/Rückgabe | Als späterer Sprint vorgeschlagen | **implementiert** | Lokal **getestet** |
| Wartungsakte und Ereignisse | Vorschlag | **implementiert** | Lokaler Teilumfang **getestet** |
| CSV-Export / JSON-Import | Serienimport/Export vorgeschlagen | CSV-Export/JSON-Import **implementiert** | Export lokal getestet; Import nicht ausgeführt |
| QR-/Barcode-Eingabe | Idee | Textfelder vorhanden; keine Zebra-Integration | DataWedge-Verwendung technisch plausibel, am Gerät ungetestet |
| QR-Label/PDF | Entwurfsroute | Kein Labelhandler; Routingproblem | Fehlverhalten lokal reproduziert |
| Scan & Jump / Inventursession | Idee | Nicht gefunden | Nicht getestet |
| GPS/Fotos/Anhänge | Idee | Kein dediziertes Schema/Workflow belegt | Nicht getestet |
| Offlinequeue/Sync | Idee | Kein entsprechender Asset-Client gefunden | Nicht getestet |
| RBAC/JWT/2FA | Vorschlag | Asset-Dienst OPEN LAB | Nicht umgesetzt/belegt |
| Browser/SSH als TBS-Werkzeug | H2-Idee | Dashboard im Code | Auf Zebra nicht getestet |
| Audio-/PTT-Nutzung | H2-Idee | Audio-Playout-Handler im TBS-Code | Kein Live-Audio-/Funk-/PTT-Nachweis |

## 12. Offene Aufgaben und Roadmap-Kandidaten

Die Prioritäten unten sind eine **Fortsetzungsempfehlung aus dieser Archivprüfung**. In der ursprünglichen Planung wurden außer dem Vorrang sofort nutzbarer Funktionen mit einer Einzelbasisstation keine verbindlichen Prioritäten oder Termine beschlossen.

| Priorität | Aufgabe | Abhängigkeit / konkreter Abschlussnachweis |
|---|---|---|
| P0 | Modell, Android/DataWedge-Version und Browser des Zebra erfassen | Geräteinformationen/Typenschild; TH57/TC57-Unterschied auflösen |
| P0 | Installierten TBS-Commit und Dashboardadresse/-funktionen feststellen | Lokale Versions-/Configprüfung; tatsächlicher Browserzugriff vom Zebra |
| P0 | Sofortnutzung als mobiles Browser-/Diagnosegerät abnehmen | Dashboarddarstellung, Sessionverhalten, Bedienung; bei gewünschtem SSH lesende Logabfrage testen |
| P1 | Vorhandenen Asset-Dienst statt neuem Parallel-Inventar bewerten | Tatsächlicher Diensthost, Konfiguration, vorhandener Datenbestand und Zuständigkeiten klären |
| P1 | Einfachen DataWedge-Scan in Inventar-/Seriennummer-/Suchfeld testen | Passendes Vordergrundprofil; Fokus, führende Nullen, Sonderzeichen, Wiederholung und Suffixe kontrollieren |
| P1 | Seriennummer-Dublettenprüfung vereinheitlichen | Dieselben Regeln für Anlage, Änderung und Import; Regressionstest für den hier reproduzierten PUT-Fall |
| P1 | Asset-Routing strikt prüfen | Überzählige Pfadsegmente dürfen keinen vermeintlichen Labelerfolg liefern. |
| P1 | Identifier und Labelstandard beschließen | `asset_id`/Inventarnummer und `nasset:` versus Weblink; Netzwechsel, Labeltext und dauerhaft stabile Kennung berücksichtigen |
| P2 | Scan-Lookup und Quick-Create ergänzen | Mehrdeutige/unbekannte Codes, keine Treffer und Doppelscans klar behandeln; kein ungeprüftes automatisches Anlegen |
| P2 | CSV-/JSON-Migration verlustfrei planen | Historische Felder/Status/Typen abbilden; Importvalidierung und Dublettenbehandlung, Preview und Wiederholbarkeit |
| P2 | Authentifizierung und Rollen integrieren | Vorhandene zentrale IAM-Planung; Rechte für Export, Ausmustern, Massenaktionen und Audit-Akteure konkret belegen |
| P2 | QR-Label-Endpoint/Etikettenausgabe | API/Format, Maße, Druck und Haltbarkeit; bestehendes Routing zuerst berichtigen |
| P2 | Physische Asset-/Node-Zuordnungen festlegen | Node-Hardware, Standort und ISSI-Netzsnapshot voneinander trennen |
| P2 | Inventursession mit „gesehen“-Zeitstempel | Sollbestand, fehlende/doppelte/fremde Assets und Sessionabschluss definieren |
| P3 | Foto-/Anhangsverwaltung und GPS-Notizen | Storage, Referenzen, Versionierung und Felddatenqualität |
| P3 | Offlineerfassung/PWA oder native App | Bedarf nach Onlinepilot prüfen; lokale IDs, Idempotenz, Konflikte, Wiederaufnahme und Synchronisierungsanzeige definieren |
| P3 | Wartungsbenachrichtigungen, Hardware-Mismatch und Reporting | Tatsächliche Ereignisquelle/Serial-Telemetrie; Zustellweg; fachliche Kennzahlen und MTBF-Grunddaten |
| Nachrangig | Audio-Buttons / Playout | Installierte Audiofeature-/Quellenkonfiguration, tatsächliche Gruppen/Teilnehmer und On-Air-Abnahme |
| Nachrangig | Reichweitenlogging und PTT-Gatewayideen | Explizite TETRA-Messquelle beziehungsweise Audio-/Floor-/Rufgateway; keine native Zebra-Funkfähigkeit voraussetzen |

### 12.1 Konkreter nächster sinnvoller Ablauf

1. Gerät und installierte TBS-Software identifizieren, einen echten Browserzugriff nachweisen und damit die unmittelbare praktische Fragestellung auf realem Stand beantworten.
2. Falls der vorhandene Asset-Dienst zugänglich ist, einen einzelnen Testdatensatz im Onlinebetrieb per DataWedge in ein fokussiertes Feld erfassen; andernfalls lokale Erfassung als Übergang verwenden.
3. Erst danach Scan-Nutzlast und Workflow festlegen, vorhandenen Asset-Dienst abbilden und die reproduzierten Datenqualitäts-/Routingprobleme reparieren.
4. Offlinebetrieb, Inventur, Fotos, Reporting und Lighthouse-Einbettung nach Bedarf priorisieren. Für Lighthouse ist weiterhin keine vorhandene Komponente aus dieser Planung oder R1 belegt.

## 13. Quellen, Repository-Dateien und Anhänge

### 13.1 Unveränderlich referenzierter Code

Die folgenden Links sind an R1 gebunden:

- [Asset-Dienst: README](https://github.com/JanHG98/netcore-tetra/blob/e4cdd99091385b3b32b438f8cf6f020ccf78fba1/system-backend/asset-management/README.md)
- [Asset-Dienst: Python-Implementierung](https://github.com/JanHG98/netcore-tetra/blob/e4cdd99091385b3b32b438f8cf6f020ccf78fba1/system-backend/asset-management/src/netcore_asset_management.py)
- [Asset-WebUI](https://github.com/JanHG98/netcore-tetra/blob/e4cdd99091385b3b32b438f8cf6f020ccf78fba1/system-backend/asset-management/web-ui/index.html)
- [Asset-Konfigurationsvorlage](https://github.com/JanHG98/netcore-tetra/blob/e4cdd99091385b3b32b438f8cf6f020ccf78fba1/system-backend/asset-management/config/asset-management.example.toml)
- [Asset-Unit](https://github.com/JanHG98/netcore-tetra/blob/e4cdd99091385b3b32b438f8cf6f020ccf78fba1/system-backend/asset-management/systemd/netcore-asset-management.service)
- [Asset-Installer](https://github.com/JanHG98/netcore-tetra/blob/e4cdd99091385b3b32b438f8cf6f020ccf78fba1/system-backend/asset-management/install/install.sh)
- [Asset-Smoke-Prüfplan](https://github.com/JanHG98/netcore-tetra/blob/e4cdd99091385b3b32b438f8cf6f020ccf78fba1/system-backend/asset-management/tests/open_lab_smoke.md)
- [Phase-10-Dokumentation](https://github.com/JanHG98/netcore-tetra/blob/e4cdd99091385b3b32b438f8cf6f020ccf78fba1/Docs/PHASE_10_ASSET_DEVICE_USER_MANAGEMENT.md)
- [Open-Lab-Inventory](https://github.com/JanHG98/netcore-tetra/blob/e4cdd99091385b3b32b438f8cf6f020ccf78fba1/deploy/open-lab/inventory.example.toml)
- [Generierte Asset-Konfiguration](https://github.com/JanHG98/netcore-tetra/blob/e4cdd99091385b3b32b438f8cf6f020ccf78fba1/deploy/open-lab/generated/configs/asset-management/asset-management.toml)
- [TBS-Dashboard-/Audiohandler](https://github.com/JanHG98/netcore-tetra/blob/e4cdd99091385b3b32b438f8cf6f020ccf78fba1/crates/tetra-entities/src/net_dashboard/server.rs)
- [TBS-Cargo-Features](https://github.com/JanHG98/netcore-tetra/blob/e4cdd99091385b3b32b438f8cf6f020ccf78fba1/bins/bluestation-bs/Cargo.toml)

Die historische Planung nennt keinen belastbar zugehörigen Implementierungscommit und keine zugehörige PR. Solche Referenzen werden nicht ergänzt. Andere Android-/Inventarplanungen bleiben eigenständige Vorhaben.

### 13.2 Bereitgestellte PDF-Dateien

Alle 25 PDFs waren lesbar. Insgesamt wurden **8.061 PDF-Seiten** gezählt; das ist eine Dateiseitensumme einschließlich der 4.100 Seiten von `ETSI.pdf`, keine Anzahl unterschiedlicher oder vollständig geprüfter Normseiten. Inhaltlich erfolgten hier nur Metadaten-/Titelseitenprüfung und Einordnung der Relevanz. PDFs wurden nicht als neue Zebra-Dokumentation oder Originalbilder in Git importiert.

| Datei | Metadaten / Titelseite | Seiten | SHA-256 |
|---|---|---:|---|
| `en_3003920308v010401p.pdf` | EN 300 392-3-8 - V1.4.1 - Terrestrial Trunked Radio (TETRA); Voice plus Data (V+D); Part 3: Interworking at the Inter-System Interface (ISI); Sub-part 8: Generic Speech Format Implementation | 22 | `4d993a25e35da7b1f2291ae281cb3ea5b3a7b702ab7eb3233bdf430a2c70909d` |
| `en_30039209v010701p.pdf` | EN 300 392-9 - V1.7.1 - Terrestrial Trunked Radio (TETRA); Voice plus Data (V+D); Part 9: General requirements for supplementary services | 46 | `cad45938bcdf2fa4e8063d4aa9e7d7c06c45f729d3c55dc033e00c27af9f2e06` |
| `ts_10081201v020205p.pdf` | TS 100 812-1 - V2.2.5 - Terrestrial Trunked Radio (TETRA); Subscriber Identity Module to Mobile Equipment (SIM-ME) interface; Part 1: Universal Integrated Circuit Card (UICC); Physical and logical characteristics | 8 | `96df75f5ddf1c3ae4ac75f796ee97babca68fa81aca22f39ba0e0658bd57eed1` |
| `en_3003921201v010202p.pdf` | EN 300 392-12-1 - V1.2.2 - Terrestrial Trunked Radio (TETRA); Voice plus Data (V+D); Part 12: Supplementary services stage 3; Sub-part 1: Call Identification (CI) | 56 | `4d5b56a188fb73c417d67b27da619cad2e49efb3b57306fd4ae51e140c018018` |
| `en_3003920304v010301p.pdf` | EN 300 392-3-4 - V1.3.1 - Terrestrial Trunked Radio (TETRA); Voice plus Data (V+D); Part 3: Interworking at the Inter-System Interface (ISI); Sub-part 4: Additional Network Feature Short Data Service (ANF-ISISDS) | 28 | `8a38cc6238c6da726e4f07d7d377b2c827447a1448a60ee5ef2a126cb3a0ef9d` |
| `en_3003921117v010102p.pdf` | EN 300 392-11-17 - V1.1.2 - Terrestrial Trunked Radio (TETRA); Voice plus Data (V+D); Part 11: Supplementary services stage 2; Sub-part 17: Include Call (IC) | 18 | `69ce800e352b1ecfc337416fd7b54c1bdf2d9270171af519ac2f5acc1ba85ca6` |
| `en_3003921114v010101p.pdf` | EN 300 392-11-14 - V1.1.1 - Terrestrial Trunked Radio (TETRA); Voice plus Data (V+D); Part 11: Supplementary services stage 2; Sub-part 14: Late Entry (LE) | 23 | `ba1882df71dbb84a0c4f0f7dcea1dc8df968044457da03df9290a50e8821df32` |
| `es_20081202v020401m.pdf` | Final draft: ES 200 812-2 - V2.4.1 - Terrestrial Trunked Radio (TETRA); Subscriber Identity Module to Mobile Equipment (TSIM-ME) interface; Part 2: Universal Integrated Circuit Card (UICC); Characteristics of the TSIM application | 139 | `330f045a908eefd249fd7450e5b71bd08e8a7b20ce5b86dc9b87c9138a5c7268` |
| `es_20081201v020205p.pdf` | ES 200 812-1 - V2.2.5 - Terrestrial Trunked Radio (TETRA); Subscriber Identity Module to Mobile Equipment (TSIM-ME) interface; Part 1: Universal Integrated Circuit Card (UICC); Physical and logical characteristics | 8 | `346dc545049a7397ca86831731bbc05e96f61ff047d6e7f6b7de56da3aa408e9` |
| `en_300812v020101p.pdf` | EN 300 812 - V2.1.1 - Terrestrial Trunked Radio (TETRA); Security aspects; Subscriber Identity Module to Mobile Equipment (SIM-ME) interface | 156 | `196effeaae473a98244c89538e1562daa58bb1d74cbfe7de78e58ceeafbad18b` |
| `en_3003921101v010201p.pdf` | EN 300 392-11-1 - V1.2.1 - Terrestrial Trunked Radio (TETRA); Voice plus Data (V+D); Part 11: Supplementary services stage 2; Sub-part 1: Call Identification (CI) | 44 | `852c17ea566757e80ecf0d2718ae7839d9e218aa4384b63d1277ed7e25d86c69` |
| `en_3003921006v010401p.pdf` | EN 300 392-10-6 - V1.4.1 - Terrestrial Trunked Radio (TETRA); Voice plus Data (V+D); Part 10: Supplementary services stage 1; Sub-part 6: Call Authorized by Dispatcher (CAD) | 20 | `32b6ee43602ef88a0b5c209b1c69ad35ba8b2e660502257c2dfaa6205a346523` |
| `en_3003921018v010301p.pdf` | EN 300 392-10-18 - V1.3.1 - Terrestrial Trunked Radio (TETRA); Voice plus Data (V+D); Part 10: Supplementary services stage 1; Sub-part 18: Barring of Outgoing Calls (BOC) | 17 | `4cc10bcd94d39201538639cefb529809729563f13f87cfe6ae52b44c253b0b8a` |
| `en_3003921216v010400a.pdf` | DRAFT: EN 300 392-12-16 - V1.4.0 - Terrestrial Trunked Radio (TETRA); Voice plus Data (V+D); Part 12: Supplementary services stage 3; Sub-part 16: Pre-emptive Priority Call (PPC) | 67 | `c0ee7859f448c4072befc25905c29b751b5151d3590a880a2d654309a3ba6c02` |
| `en_30039201v010601p.pdf` | EN 300 392-1 - V1.6.1 - Terrestrial Trunked Radio (TETRA); Voice plus Data (V+D); Part 1: General network design | 182 | `788722e566098957bea1a9a5096e9dae1eace1da9f5d38c530c80230e5b9b9cb` |
| `ets_30039214e01v.pdf` | FINAL DRAFT: ETS 300 392-14 - Terrestrial Trunked Radio (TETRA); Voice plus Data (V + D) Part 14: Protocol Implementation Conformance Statement (PICS) proforma specification | 61 | `2b703f3ebb883c90dab40e1b3d1a634a6acf9c9a20d468fd108ea02226c2bf2c` |
| `en_30039207v030501p.pdf` | EN 300 392-7 - V3.5.1 - Terrestrial Trunked Radio (TETRA); Voice plus Data (V+D); Part 7: Security | 216 | `df47af9a642b6eba7d7d5cf5fb7aa3f961e9e03bb912316bd2d189ce47344e08` |
| `en_30039401v030301p.pdf` | ETSI EN 300 394-1 V3.3.1 | 169 | `2d9c327e5ccf1476335e1b7a4f58a0ca7afc93911c30e8aad5ed7dc9ccf3276a` |
| `en_3003920313v010201p.pdf` | EN 300 392-3-13 - V1.2.1 - Terrestrial Trunked Radio (TETRA); Voice plus Data (V+D); Part 3: Interworking at the Inter-System Interface (ISI); Sub-part 13: Transport layer independent Additional Network Feature Group Call (ANF-ISIGC) | 191 | `b47d63853f83a6a4adda08c0e325c8ca13d358e2f8840bfba4bce430e600b1bd` |
| `en_30039502v010303p.pdf` | EN 300 395-2 - V1.3.3 - TETRA and Critical Communications Evolution (TCCE); Speech codec for full-rate traffic channel; Part 2: TETRA codec | 94 | `ac716ca18082cc0fd2ee768a14f03deb48fa78a77e83751b1a6b319c7fc2106a` |
| `en_3003920303v010301p.pdf` | EN 300 392-3-3 - V1.3.1 - Terrestrial Trunked Radio (TETRA); Voice plus Data (V+D); Part 3: Interworking at the Inter-System Interface (ISI); Sub-part 3: Additional Network Feature Group Call (ANF-ISIGC) | 251 | `94ca61038b3ff826e6adcfde56e00473dd996e1a47f9f84e7b9d2aa6c3133fd2` |
| `en_30039205v020701p.pdf` | EN 300 392-5 - V2.7.1 - Terrestrial Trunked Radio (TETRA); Voice plus Data (V+D) and Direct Mode Operation (DMO); Part 5: Peripheral Equipment Interface (PEI) | 320 | `10aaf78988ae4080c3dfe90165ee7f4f3fe8eda402e09fc1e6fa0a0ad890758d` |
| `en_3003920315v010500a.pdf` | DRAFT: EN 300 392-3-15 - V1.5.0 - Terrestrial Trunked Radio (TETRA); Voice plus Data (V+D); Part 3: Interworking at the Inter-System Interface (ISI); Sub-part 15: Transport layer independent Additional Network Feature, Mobility Management (ANF-ISIMM) | 380 | `e959709667192e22d2dfeea51b48f9a579a4c107d952999056b7fad93a6d2100` |
| `en_30039202v030801p.pdf` | EN 300 392-2 - V3.8.1 - Terrestrial Trunked Radio (TETRA); Voice plus Data (V+D); Part 2: Air Interface (AI) | 1445 | `3f07b1e4ad73fabc16277a900006fc19844b3a882bbc2bc1d74d78f6156daf28` |
| `ETSI.pdf` | Zusammenstellung; erste Seite: ETSI EN 300 812 V2.1.1 (2001-12); Gesamtinhalt nicht vollständig ausgewertet | 4100 | `9434dad1e7bc80ca39b0edadd8e3b9995fda5ae5f5c3dd565af05708d5059e38` |

### 13.3 Bildquellen

Für diese Zebra-Planung liegen keine Originalbilder vor. Der verfügbare Normenbestand ersetzt weder Gerätefotos noch Scannerbelege.
