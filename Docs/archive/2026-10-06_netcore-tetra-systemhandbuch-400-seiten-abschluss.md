# Brainstorming: NetCore-Tetra-Systemhandbuch: vom 93-Seiten-Entwurf zur 400-Seiten-Ausgabe

Stand der Repository- und Quellenprüfung: **06.10.2026**. Historische Ergebnisse beziehen sich auf die jeweils genannten Daten und Commits.

Entwicklungsnotizen zur 400-Seiten-Ausgabe des NetCore-Tetra-Systemhandbuchs: Umfang, redaktionelle Entscheidungen, Qualitätssicherung und gesonderter Repository-Abgleich.

## 1. Projektstand und Quellenlücke

| Merkmal | Befund |
|---|---|
| Thema | Vollständiges, modern gestaltetes NetCore-Tetra-Systemhandbuch; spätere Korrektur der Länge von 93 auf ungefähr 400 Seiten |
| Erstellungsdatum dieser Zusammenfassung | 6. Oktober 2026, Europe/Berlin |
| Repository | [`JanHG98/netcore-tetra`](https://github.com/JanHG98/netcore-tetra) |
| Für den geprüften Abgleich gelesener Zielbranch | `Archiving`, Ausgangs-HEAD [`41161df571e66e028b2984ff825212305f86606d`](https://github.com/JanHG98/netcore-tetra/commit/41161df571e66e028b2984ff825212305f86606d), vor dieser Archivänderung; dessen Commitzeit ist 6. Oktober 2026, 06:30:27 UTC |
| Zusätzlich beobachteter `main`-HEAD | `9116c15d645458f99e236712b67a1ad970432791` beim Abruf; der inhaltliche Abgleich unten bezieht sich auf `Archiving` |
| Historische Codebasis der Handbuchausgabe | `main` bei [`3768f964100bf99e37914b8414ed611fe3bdcd67`](https://github.com/JanHG98/netcore-tetra/commit/3768f964100bf99e37914b8414ed611fe3bdcd67), im Buch als Quellenstand vom 26. September 2026 bezeichnet |
| Ablage | `Docs/archive/2026-10-06_netcore-tetra-systemhandbuch-400-seiten-abschluss.md`; [Archivindex](README.md) |
| Historische PR / Erstellungskommit | Nicht nachgewiesen. Die am 06.10.2026 vorgefundenen Dateien belegen den Buchstand, aber nicht seinen ursprünglichen Übertragungsvorgang. |

**Quellenbasis:** Anforderungen an das Vollhandbuch, die Korrektur des 93-Seiten-Entwurfs und die dokumentierte 400-Seiten-Übergabe. Teile der Herstellung sind nur in verdichteten Arbeitsnotizen erhalten. Vollständige Rohlogs, lokale Buildskripte und sämtliche QA-Bilder fehlen. Historisch gemeldete Zahlen und der Dateiabgleich vom 06.10.2026 werden deshalb getrennt geführt.

**Statusschlüssel für diese Datei:** **Idee** = geäußerter Wunsch ohne Umsetzungsentscheidung; **beschlossen/geplant** = ausdrücklich gewünschtes Ziel oder beschriebener Ablauf; **implementiert** = Datei/Code im geprüften Repository bzw. erzeugtes Dokument im damaligen Arbeitsgang; **getestet** = konkreter Check mit Ergebnis; **im Betrieb bestätigt** = durch reale Anlage/Endgeräte und dokumentierten Lauf belegt. „Implementiert“ allein schließt die letzten beiden Stufen nicht ein. Bei allen technischen Bestandsangaben wird zwischen *damals* (`main`-Snapshot) und *zum Prüfstand vom 06.10.2026* (`Archiving`-Snapshot) unterschieden.

## 2. Ziel, Ausgangslage und Verlauf

Ziel war ein **vollständiges, modern gestaltetes und projektgerechtes Handbuch** für den damaligen NetCore-Tetra-Stand: Einleitung, Inhaltsverzeichnis, Funktionen, Systemaufbau, Installation, Konfiguration, Betrieb, Wartung, Reparatur, Fehler samt Abhilfe sowie Ports und Protokolle. 400 oder mehr Seiten waren als Umfang vorgesehen. Das Ergebnis sollte als Betriebs- und Nachschlagewerk nutzbar sein.

| Phase | Feststellung | Status und Evidenz |
|---|---|---|
| Erstauftrag | Vollständiges Handbuch in modernem, projektgerechtem Design | **beschlossen/geplant** als Projektvorgabe. |
| Erster Entwurf | Eine 93-seitige Ausgabe wurde im dokumentierten Arbeitsstand geliefert | **implementiert** als damaliges Dokument, nach Arbeitszusammenfassung mit PDF/DOCX/Markdown; Umfang als damalige Prüfung gemeldet. |
| Ausdrückliche Korrektur | „nur 93 seiten? ... an die 400 erwartet“ | **beschlossen/geplant**; ersetzt die frühere Akzeptanz eines 93-Seiten-Zwischenstands. Die 93 Seiten sind ein historischer Zwischenstand, keine endgültige Längenvorgabe. |
| Ausbau | Genau 400 Seiten, 29 Kapitel, PDF/DOCX/Markdown und umfangreiche Register/Arbeitsblätter wurden als Schlussstand gemeldet | **implementiert und damals layoutseitig getestet** nach der zugänglichen Arbeitszusammenfassung; zum Prüfstand vom 06.10.2026 ist eine entsprechende dreifache Buchausgabe im Repo vorhanden. Kein Betriebsnachweis. |
| Spätere Repository-Fortschreibung | Datierte Ausgaben vom 27. und 28. September sind zum Prüfstand vom 06.10.2026 vorhanden | **implementiert im geprüften Repository**, aber spätere, eigenständige Dokumentstände. Sie ändern den historischen Abschlussstand nicht rückwirkend. |

Der Arbeitsschwerpunkt lag bei der **Dokumentation** des vorhandenen Codes, der Beispiele und der bereitgestellten ETSI-Unterlagen. In diesem Handbuch-Erstellung wurde keine Produktfunktion, kein Deployment und kein Funkbetrieb als ausgeführt nachgewiesen. Der spätere Wunsch nach einem 400-Seiten-Werk ist die maßgebliche Korrektur; jede Aussage „93 Seiten waren fertig“ beschreibt nur den verworfenen Zwischenstand.

## 3. Endgültige Anforderungen und redaktionelle Entscheidungen

| Gegenstand | Endgültige Festlegung / Begründung | Status |
|---|---|---|
| Umfang | Deutlich über dem 93-Seiten-Entwurf; die finale Ausgabe wurde auf genau 400 PDF-Seiten gebracht. Eine obere Grenze war nicht festgelegt. | **beschlossen/geplant**, damals als **implementiert/getestet** gemeldet. |
| Formate | Lesbares PDF, editierbares DOCX und durchsuchbarer Markdown-Text. | **implementiert** laut damaliger Übergabe; zum Prüfstand vom 06.10.2026 liegen drei gleichnamige Dateitypen in `Docs/` vor. |
| Gestaltung | Modernes Cover, Kapitelhierarchie, Tabellen, Seiten- und Inhaltsnavigation, Betriebsblätter und gut lesbare Druckausgabe. | **implementiert/getestet** laut damaliger Layoutprüfung; keine zum Prüfstand vom 06.10.2026 neue visuelle PDF-Prüfung. |
| Gliederung | 29 Kapitel; Einstieg, Architektur, Funk/Hardware, Netz/Ports, Installation, Konfiguration, Betrieb, Wartung, Reparatur, Fehler, Tests, Dienst- und API-Referenz, Protokolle, Praxis- und Formularteile bis zur Fünf-Minuten-Einsatzreferenz. | **implementiert**; das geprüfte Markdown zeigt diese 29 Kapitel und ihre PDF-Seitenzuordnung. |
| Quelltreue | Konkrete Pfade und Beispielwerte auf den untersuchten Commit beziehen; Vorlage, Quellcode, statische Prüfung und On-Air-Nachweis unterschiedlich kennzeichnen. | **beschlossen/geplant und im Buchtext umgesetzt**. Eine ETSI-Quellenliste ist keine Zertifizierung. |
| Sicherheit | Zugangsdaten nicht ins Buch übernehmen; Open-Lab-Management wegen vielfach HTTP ohne Login/Token/TLS nur in isoliertem Managementnetz beschreiben. | **implementiert** als Dokumentationsregel. Geprüfter `alert-service` hat eine gesonderte Token-Vorgabe; pauschale Aussagen über alle Dienste wären inzwischen falsch. |
| Standortparameter | IPs, Frequenzen, Netzkennungen, ISSIs, Ports und Unit-Namen anhand realer Standortkonfiguration ersetzen bzw. prüfen. | **beschlossen/geplant**; Beispielwerte sind keine bestätigten Runtime-Werte. |
| Live-Abnahme | TBS, LXC, SDR und Funkgeräte mit nachvollziehbarem Build, Logs und Messdaten prüfen. | **geplant/offen**, im dokumentierten Arbeitsstand **nicht im Betrieb bestätigt**. |

Die finale Länge ist kein Beleg für sachliche Vollständigkeit jeder Schnittstelle. Die Buchausgabe ist eine auf `main` `3768f964…` eingefrorene Bestandsaufnahme; spätere Codeänderungen erfordern einen neuen Abgleich.

## 4. Erreichtes Dokumentergebnis und damalige Qualitätssicherung

### 4.1 Struktur und Zählwerte des Schlussstands

Die zugängliche frühere Arbeitszusammenfassung meldet rund **102.839 Wörter im Markdown**, 29 Kapitel und ein auf **400 Seiten** gerendertes PDF. Daneben nennt sie **426 deklarierte API-Methode/Pfad-Einträge**, **882 Backend-Konfigurationsfelder**, **208 aktive TBS-Felder** und **196 optionale/kommentierte TBS-Zeilen**. Es entstanden **25 Dienst-Arbeitsblätter** (im damaligen Bezugsrahmen 24 Open-Lab-Dienste plus ein gesonderter Provisioning-Core-Blick), **22 symptomorientierte Reparaturfälle**, eine statische Inventur von **77 PDU- und 130 SAP-Einträgen** sowie **18 Zustandsbereichen**. Das sind damalige Inventur- und Manuskriptzählungen, **keine zum Prüfstand vom 06.10.2026 neu erhobenen Code- oder Konformitätszahlen**.

Der damalige Aufbau führte von 13 Basis-Kapiteln über Dienst-/Konfigurations- und API-Atlanten zu Betriebsblättern, Praxisabläufen, Installationsbuch, technischen Datenblättern, kopierbaren Prüfbögen, dokumentierten Abweichungen und einer kurzen Einsatzreferenz. Im geprüften Repository-Text steht die Kapitelübersicht mit Kapitel 29 ab PDF-Seite 396, die Ausgabebezeichnung **2.0** und der historische Quellcommit. Die in `Docs/` vorhandenen [Markdown-](../NetCore-Tetra-Systemhandbuch.md), [PDF-](../NetCore-Tetra-Systemhandbuch.pdf) und [DOCX-Dateien](../NetCore-Tetra-Systemhandbuch.docx) stützen die Existenz dieses Buchstands; die aktuelle Binär-PDF-Seitenzahl wurde bei diesem Prüfdurchlauf vom 06.10.2026 nicht erneut ausgelesen.

### 4.2 Herstellungs- und QA-Spur

In der vorherigen Arbeitsumgebung lagen laut Arbeitszusammenfassung `manual/handbuch_basis.md`, `manual/build_content.py`, `manual/expand_manual.py`, `manual/generate_ops.py`, `manual/praxisfaelle.md`, `manual/installationsbuch.md`, `manual/technik.md`, `manual/formblaetter.md`, `manual/abweichungen.md`, `manual/einsatzreferenz.md`, `manual/toc_pages.json` und `manual/style_docx.py`. Die Ausgabe hieß `manual/output/NetCore-Tetra-Systemhandbuch.{pdf,docx,md}`. Diese Pfade waren **damalige lokale Arbeitsdateien**, keine für diesen Prüfdurchlauf vom 06.10.2026 zum Prüfstand vom 06.10.2026 vorliegenden oder hier ins Repository eingefügten Quelldateien.

Damals gemeldete Prüfschritte: PDF-Seitenzählung 400; 29 Kapitelanfänge gegen das Inhaltsverzeichnis geprüft; PDF-Lesezeichen (595 gemeldet); textueller Überlauf per PyMuPDF ohne Befund; Seiten als PNG gerendert und stichprobenartig/visuell geprüft. 65 fehlerhafte **QA-Renderer-PNGs** wurden aus dem gültigen PDF neu gerendert; diese Meldung ist kein Nachweis, dass 65 PDF-Seiten beschädigt waren. Einzelne dünn belegte Seiten wurden in der Dokumentausgabe ergänzt. Diese QA belegt Satz und Navigation in der damaligen Umgebung, **nicht** Funktionsfähigkeit einer realen TETRA-Anlage. Die Renderdateien sind hier nach Workspace-Bereinigung nicht erneut zugänglich.

Die PDF-, DOCX- und Markdown-Ausgaben wurden im früheren Arbeitsgang auch als Dokumentartefakte bereitgestellt; nach der aktuellen Repository-Prüfung liegen sie zusätzlich im Branch. Ob die Repo-Binärdateien bytegenau die zuletzt im dokumentierten Arbeitsstand verlinkten lokalen Ausgaben sind, ist ohne Hashvergleich nicht belegt. Ein nachträglicher Commit dieser Buchdateien ist für diesen Entwicklungsstand nicht sichtbar.

## 5. Systemarchitektur, Komponenten und Abhängigkeiten des dokumentierten Produkts

Dieser Abschnitt beschreibt die **im Buch behandelten Schichten**, keine neu implementierte Architekturentscheidung dieser Entwicklungsphase.

| Ebene | Aufgaben / belegbare Orte | Verbindung und Grenze |
|---|---|---|
| TBS / FlowStation-BlueStation | Rust-Workspace, etwa `bins/bluestation-bs`, `crates/tetra-entities`, `crates/tetra-config`; PHY, MAC, LLC, MLE, MM, CMCE, SDR-/RF-Ansteuerung, lokale Registrierung, Ruf-/SDS-Fallback. Beispiel in [`config.toml`](../../config.toml) und [bereinigter Vorlage](../basisstation.config.sanitized.example.toml). | Luftschnittstelle zum MS; lokale zeitkritische Abläufe. On-Air-Verhalten wurde im dokumentierten Arbeitsstand nicht gemessen. |
| Node Gateway und Core | [`deploy/open-lab/inventory.example.toml`](../../deploy/open-lab/inventory.example.toml), [`system-backend/services.toml`](../../system-backend/services.toml); Teilnehmer, Gruppen, Mobility, Call Control, SDS, Medien, Packet Data, Security/KMF, Transit und Fachanwendungen. | TBS-zu-Gateway-WebSocket `/ws/node` und dienstinterne HTTP-APIs; Fallback bei Core-Verlust ist als Code-/Konfigpfad dokumentiert, nicht in dieser Sitzung live abgenommen. |
| Bedienung und Beobachtung | Control Room, Dienst-WebUIs, Observability, Health-/Metrics-/OpenAPI-Endpunkte. | Liveness ist kein fachlicher Readiness- oder RF-Nachweis. Managementzugang hängt von Open-Lab-Netzgrenzen ab. |
| Integrationen | SIP Switch/Asterisk, separater Brew-Pfad, IoT Gateway/MQTT/Home Assistant, Application Gateway/TTS, Recorder/Media Library/NFS, Hardware- und RF-Monitor. | Für SDS→MQTT→HA, Telefonie, Aufzeichnung und Funk muss jede Übergabe einzeln getestet werden. Beispielhafte Integrationsbeschreibung im Buch ist kein Betriebsbeweis. |
| Deployment | Open-Lab-LXC-Inventar, Python-Deployer, Installer/systemd. | Je Dienst eigener LXC im Beispiel; `apply` ist eine reale Änderung und wurde in diesem Handbuch-Erstellung nicht ausgeführt. |

Die beabsichtigte Fehlergrenze ist lokale Funkzellen-Autonomie bei zentralem Ausfall. Der statische Prüfer verlangt hierzu Service-Health-Ziele, Fallback-Modi, eine Frische-Lease und einen TBS-Gateway-Endpunkt. Daraus folgt kein nachgewiesener Verhaltenstest. ETSI-Matrizen in [`Docs/ETSI_CONFORMANCE_MATRIX.md`](../ETSI_CONFORMANCE_MATRIX.md), [`Docs/SAP_PRIMITIVE_MATRIX.md`](../SAP_PRIMITIVE_MATRIX.md) und [`Docs/IMPLEMENTATION_GAPS.md`](../IMPLEMENTATION_GAPS.md) katalogisieren Teile des Protokollstands; sie sind keine pauschale Zertifizierung.

## 6. Technischer Referenzstand: damals gegenüber zum Prüfstand vom 06.10.2026

### 6.1 Dienstinventar und Ports

Im historischen Buchstand nennt das Open Lab **24** deploybare Dienste, während ein gesonderter Provisioning-Core-Bereich im Manuskript als 25. Arbeitsblatt auftaucht. **Zum Prüfstand vom 06.10.2026** enthält das Beispielinventar auf `Archiving` **25** `[[services]]`, darunter neu `alert-service`. `system-backend/services.toml` hat **26** `[[services]]`: diese 25 Namen plus den Eintrag `shared`; dort ist `provisioning-core` zum Prüfstand vom 06.10.2026 **kein** Listeneintrag, sein Quellverzeichnis `system-backend/provisioning-core/` existiert separat. „25 Arbeitsblätter“ der alten Ausgabe und „25 LXC“ des geprüften Inventars sind somit verschiedene Zählungen.

Die folgenden IPs und Ports sind **Beispielwerte aus dem geprüften Inventory**, keine bestätigten Standortadressen oder geöffneten produktiven Listener. Präfix aller Adressen: `10.0.20.`; Management- und Health-Zugriffe nutzen im Open Lab überwiegend HTTP.

| Dienst | Beispiel-IP | Port | Dienst | Beispiel-IP | Port |
|---|---:|---:|---|---:|---:|
| node-gateway | `.10` | 8080 | mobility-core | `.11` | 8090 |
| subscriber-core | `.12` | 8100 | group-core | `.13` | 8110 |
| call-control | `.14` | 8120 | media-switch | `.15` | 8130 |
| recorder | `.16` | 8140 | sds-router | `.17` | 8150 |
| packet-core | `.18` | 8160 | ip-gateway | `.19` | 8170 |
| security-core | `.20` | 8180 | kmf | `.21` | 8190 |
| transit | `.22` | 8200 | application-gateway | `.23` | 8220 |
| media-library | `.24` | 8230 | control-room | `.25` | 9010 |
| observability | `.26` | 8210 | iot-gateway | `.27` | 8240 |
| hardware-gateway | `.28` | 8250 | rf-monitor | `.29` | 8260 |
| alarm-workflow | `.30` | 8270 | task-workflow | `.31` | 8280 |
| asset-management | `.32` | 8290 | sip-switch | `.33` | 8300 |
| alert-service | `.34` | 8310 | — | — | — |

`system-backend/services.toml` nennt als abstrakten WebUI-Default `https`, Port `8443` und `/api/v1`, während die einzelnen Open-Lab-Diensteinträge typischerweise `http`, ihren Inventory-Port, `security_mode = "open_lab"`, `token_auth = false`, `tls = false` setzen. **Geprüfte Ausnahme:** Der neue `alert-service` verlangt laut [`deploy/open-lab/README.md`](../../deploy/open-lab/README.md) standardmäßig einen lokal erzeugten API-Token; die Zustellung ist anfangs deaktiviert. Für eine reale Firewall- oder TLS-Freigabe ist die jeweils aktive Konfiguration maßgeblich.

### 6.2 Protokolle, Endpunkte und technische Parameter

| Schnittstelle | Beleg und Aussagegrenze |
|---|---|
| TETRA-Luftschnittstelle, SDS, Gruppen-/Einzelruf, Packet Data | Im Quellcode und Handbuch behandelt; ETSI EN 300 392-2 dient als Referenz. Kein Funkmess- oder Zulassungsnachweis aus dem Entwurf. |
| TBS ↔ Node Gateway | Beispiel `config.toml` `[control_room]`, Port `8080`, WebSocket-Pfad `/ws/node`, `use_tls = false`; Host-Widerspruch siehe Abschnitt 9. Nicht als öffentlich erreichbaren Endpunkt verstehen. |
| Dienst-Management | `/health/live`, `/health/ready`, `/metrics`, `/openapi.json`, `/api/v1` aus Dienstkatalog und Prüfern; Health-/API-Schema je Dienst und laufender Version verifizieren. `ready=503` kann fachliche Abhängigkeit anzeigen. |
| IoT | MQTT-Broker/Topic-Pfad als Integrationsschicht; das Buch nennt unter anderem `netcore/v1/#` als Diagnose-Subscription. Die tatsächliche SDS→IoT→HA-Zustellung ist nicht aus einem bloß verbundenen Broker ableitbar. |
| Telefonie/Audio | SIP-Signalisierung, SDP/RTP-Medien und Asterisk/Brew werden getrennt behandelt. Dynamische Medienports, Codec/Nummernplan und Fallback anhand der aktiven Anlage prüfen. |
| Paketdaten | SNDCP-/TUN-/IP-Gateway-Pfad; `/dev/net/tun` für den LXC im Deployment-README genannt. |
| Deployment/Archiv | SSH/SCP zu LXC; optional NFS für Recorder/Media Library. Verzeichnis- und Berechtigungszustand vor Ort prüfen. |
| SDR | `SoapySdr` in der bereinigten TBS-Vorlage, `config_version = "0.6"`, `stack_mode = "Bs"`; Frequenzen, Mittenfrequenzen, Sample Rate, Gains und Gerät sind Beispiel-/Standortparameter. Keine Genehmigung oder RF-Abnahme daraus ableiten. |

### 6.3 Pfade und Konfigurationsquellen

| Pfad / Datei | Zweck und zeitliche Einordnung |
|---|---|
| [`Docs/NetCore-Tetra-Systemhandbuch.md`](../NetCore-Tetra-Systemhandbuch.md) und `.pdf`/`.docx` | 26.-September-Buchausgabe, 29 Kapitel, historische `main`-Basis `3768f964…`. |
| [`Docs/NetCore-Tetra-Systemhandbuch-2026-09-27.md`](../NetCore-Tetra-Systemhandbuch-2026-09-27.md) und `.pdf`/`.docx` | Spätere Ausgabe 2.1; Quellenstand `main` `086a81fa8820ef579c475a65a38e3d23644c52f0`, im geprüften Branch vorhanden. |
| [`Docs/NetCore-Tetra-Systemhandbuch-2026-09-28.md`](../NetCore-Tetra-Systemhandbuch-2026-09-28.md) und `.pdf`/`.docx` | Spätere 32-Kapitel-Ausgabe; `main` `20603a042c97b60fb78599db3bc675993f3e74f9`, Entwicklungsvorschau `bbf039729b9b05f8d623b11195ca24a124f68d16`. Nicht das Ergebnis der ursprünglichen 400-Seiten-Korrektur. |
| [`deploy/open-lab/inventory.example.toml`](../../deploy/open-lab/inventory.example.toml) | Geprüfte 25 Beispiel-LXC mit Host, Port, Unit, Installer, Vorlage, Ziel und Abhängigkeiten. |
| [`system-backend/services.toml`](../../system-backend/services.toml) | Geprüfter WebUI-/Servicekatalog (25 deploybare Namen plus `shared`). |
| [`config.toml`](../../config.toml) und [`Docs/basisstation.config.sanitized.example.toml`](../basisstation.config.sanitized.example.toml) | TBS-Konfiguration und bereinigte Vorlage; tatsächliche Anlage und Geheimnisse sind hier nicht geprüft. Bei kaputtem Primär-TOML beschreibt `config.toml` eine separat anzulegende `.fallback`-Datei. |
| [`system-backend/control-room/install/install.sh`](../../system-backend/control-room/install/install.sh), [`systemd/netcore-control-room.service`](../../system-backend/control-room/systemd/netcore-control-room.service) | Control-Room-Build, systemd-Installation, Laufzeitpfad und Healthprobe. Zielpfad-Abweichung siehe Abschnitt 9. |
| [`Docs/generated/full-system-integration-audit.md`](../generated/full-system-integration-audit.md) | Eingecheckter Report `24`/`PASS`, zum geprüften 25er-Inventar veraltet. |
| [`tools/check_full_system_integration.py`](../../tools/check_full_system_integration.py) | Statischer Integrationsprüfer, derzeit selbst noch mit 24er-`EXPECTED`. Ausführen schreibt den generierten Report; bei dieser reinen Archivierung nicht ausgeführt. |
| [`Docs/ETSI_SOURCE_REGISTER.md`](../ETSI_SOURCE_REGISTER.md), weitere ETSI-/SAP-/Gap-Matrizen | Normreferenzen, Inventur und Grenzen; keine vollständige Konformitätsbescheinigung. |

## 7. Wichtige Abläufe und Befehle mit Ausführungsstatus

Die folgenden Kommandos sind **Dokumentations-/Betriebsreferenzen**. Winkelklammern und Beispielinventar sind zu ersetzen. Es fand bei der Quellenprüfung vom 06.10.2026 keine Installation, kein LXC-`apply`, kein Unit-Neustart und kein RF-Test statt.

| Befehl / Ablauf | Zweck | Tatsächlicher Nachweis für diesen Entwicklungsstand |
|---|---|---|
| `python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.example.toml validate` | Statische Prüfung des damaligen 24er-Inventars | **Damals ausgeführt und erfolgreich gemeldet** (`OK: 24 services`) auf historischer Quelle. Nicht zum Prüfstand vom 06.10.2026 für 25 Dienste wiederholt. |
| `python3 tools/check_full_system_integration.py` | Abgleich von Inventory, Port-/URL-Graph, Gateway-Health, TBS-Fallback und Konfiguration | **Damals ausgeführt und fehlgeschlagen** wegen abweichender Gateway-IP. Zum Prüfstand vom 06.10.2026 nicht ausgeführt; Skript und 25er-Inventar sind zusätzlich statisch gegeneinander widersprüchlich. |
| Pandoc/`manual/style_docx.py` und `render_docx.py`; PDF-Zählung, PyMuPDF-/PNG-Layoutkontrolle | Erstellung und Prüfung der Handbuchformate | **Damals als ausgeführt gemeldet**; Roh-Buildlogs/temporäre Skripte zum Prüfstand vom 06.10.2026 nicht verfügbar. Keine Code- oder Funkabnahme. |
| `cp deploy/open-lab/inventory.example.toml deploy/open-lab/inventory.toml` und lokale Bearbeitung | Standortinventar vorbereiten | **Nur im aktuellen Repo-README vorgeschlagen**, nicht hier ausgeführt. |
| `python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml validate`, danach `plan`, `render`, `apply --dry-run`, schließlich `apply` | Geordnete LXC-Installation; `apply` verändert reale Systeme und kann Konfigurationsvorlagen über bestehende Host-Dateien schreiben | **Vorgeschlagener Ablauf**, für diesen Entwicklungsstand kein Deployment. Für `alert-service` nennt das README ein gesondertes Updateverfahren zum Erhalt lokaler Einstellungen und Zugangsdaten. |
| `python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml test --profile full --validate-only` | Offline-Validierung der Szenarioauswahl | **Vorgeschlagen**, nicht ausgeführt. |
| Gleicher Befehl mit `test --profile smoke`, `full --allow-mutations` oder `fault --allow-mutations --allow-restarts` | Vertrag-/Mock-, Funktions- und Störungstests; Fault-Profil startet/stoppt Dienste | **Vorgeschlagen**, nicht ausgeführt. Testartefakte würden unter `tests/e2e/artifacts/<run-id>/` liegen. |
| `systemctl status <unit> --no-pager --full`; `journalctl -u <unit> -b -n 200 --no-pager`; `ss -lntup`; `curl` gegen `/health/live`, `/health/ready`, `/metrics` | Diagnoseleiter Prozess → Listener → Liveness → fachliche Readiness | **Handbuchanweisung**, keine hier ausgeführten Anlagenbefehle. |
| `cp config.toml config.toml.fallback` (am tatsächlichen TBS-Pfad) | Bekannte gute TBS-Konfiguration als Parser-Fallback | **In der geprüften Beispielkonfiguration empfohlen**; weder Datei noch Reparatur an einer realen TBS hier bestätigt. |
| `python3 tools/protocol_inventory.py` und `./tools/check_protocol_inventory.sh` | Statische PDU-/SAP-/Gap-Inventur aktualisieren und Drift erkennen | **Im Repo dokumentiert**, nicht zum Prüfstand vom 06.10.2026 durchgeführt; kann keine ETSI- oder On-Air-Konformität beweisen. |

Bei Reparatur und Rollback empfiehlt das Buch Sicherung von TBS-Konfiguration, systemd-Unit/Drop-ins, Policy-Cache, Spool, Medien und exaktem Build-Commit sowie je LXC Konfiguration und tatsächlichen State; danach gezielte Wiederherstellung und fachliche Tests. Das ist eine **beschriebene Prozedur**, keine für diesen Entwicklungsstand vollzogene Wiederherstellung. Normale Tages-/Wochen-/Monatskontrollen, Teilnehmeraufnahme, Gruppenruf, SDS, SIP, Recorder und Isolation/Recovery sind ebenfalls Buchabläufe, keine damals gemeldeten Live-Aktionen.

## 8. Fehlerbilder, Ursachen, Lösungen und Grenzen

### 8.1 Im damaligen Arbeitsgang konkret festgestellt

| Befund | Diagnose / Ursache | Lösung und Status |
|---|---|---|
| 93 statt erwarteter ~400 Seiten | Der erste Entwurf erfüllte die nachträglich ausdrücklich genannte Längenerwartung nicht. | Ausbau auf die gemeldete 400-Seiten-Ausgabe und 29 Kapitel: **als Dokument umgesetzt**, Layout damals geprüft. 93 Seiten sind überholt. |
| Full-System-Check `FAIL` | `[control_room].host` in Root-`config.toml` war `10.0.1.179`, Inventory-Node-Gateway `10.0.20.10`; Port 8080 und `/ws/node` stimmten. | Reale Gateway-Adresse je Standort konsistent setzen, dann den Check erneut ausführen. **Nicht für diesen Entwicklungsstand behoben**; zum Prüfstand vom 06.10.2026 in den Beispielquellen weiter sichtbar. Kein Beweis eines Fehlers der realen Anlage. |
| Report `PASS` gegenüber tatsächlich rotem Check | Eingechecktes generiertes Audit stammte aus einem anderen/konsistenteren Zustand. | Report erst nach korrigierter Quelle neu erzeugen und Ergebnis dokumentieren. **Offen**; zum Prüfstand vom 06.10.2026 durch 25er-Inventar zusätzlich veraltet. |
| QA-PNGs teilweise fehlerhaft | 65 Seitenbilder des Render-/QA-Schritts waren beschädigt; das PDF selbst blieb lesbar. | Aus PDF erneut rendern; **damals für QA-Artefakte behoben**. Kein Produktfehler. |

### 8.2 Zum Prüfstand vom 06.10.2026 aus dem Branch zusätzlich oder weiterhin erkennbar

| Befund bei `41161df…` | Evidenz | Status / nächster sinnvoller Schritt |
|---|---|---|
| Inventory 25, Full-System-Prüfer `EXPECTED` 24 und generierter Report `24`/`PASS` | `deploy/open-lab/inventory.example.toml` hat `alert-service` `.34:8310`; `tools/check_full_system_integration.py` erwartet 24 Namen; Report nennt 24. | **Implementierungs-/Dokudrift statisch bestätigt**, tatsächlicher geprüfter Skriptlauf **nicht getestet**. Prüfer, Fallback-/Health-Ziele und Report auf aktuelle Topologie abstimmen, danach ausführen. Den alten grünen Report nicht als geprüftes Testergebnis zitieren. |
| Beispiel-TBS-Gateway-IP weiterhin abweichend | Root-`config.toml` `10.0.1.179:8080` vs. Node Gateway `10.0.20.10:8080`. | **Statisch bestätigt, ungelöst in den Beispieldateien**; reale Zieladresse ermitteln und Quellen gemeinsam korrigieren. |
| Control-Room-Zielpfad uneinheitlich | Inventory `config_target = "/etc/netcore/control-room.toml"`; Installer und Unit nutzen `/etc/netcore-control-room/control-room.toml`. | **Statisch bestätigt**; Render-/Installationsziel und Runtime-Datei angleichen, danach Install-/Start- und Healthtest. Kein geprüfter LXC-Test. |
| Sicherheit des Managementnetzes | Die meisten Open-Lab-Dienste sind HTTP ohne Token/TLS; `alert-service` ist eine Ausnahme mit lokalem Token. | **Konfigurationsbefund**. Isoliertes Management-VLAN sowie konkrete Zugriffskontrolle vor echter Nutzung verifizieren; keine Aussage über tatsächlich exponierte Hosts. |

Das Handbuch listet weitere **symptomorientierte Diagnosefälle** (z. B. TBS-TOML-Parser/Fallback, MS findet Zelle nicht, Registrierung ohne Gruppen-PTT, SDS erreicht Home Assistant nicht, SIP klingelt ohne Audio, RTP/Codec-Probleme, Recorder/NFS, Paketdaten/TUN, VSWR-Sensorik, Dual-Carrier- und Release-Verhalten). Diese Fälle sind **Reparaturanleitungen und Testideen**, keine sämtlich für diesen Entwicklungsstand aufgetretenen, diagnostizierten und behobenen Vorfälle. Es fehlt insbesondere der reale Nachweis für SDS an eine Service-ISSI bis Home Assistant, durchgängige SIP-/Brew-Medienpfade und Funkgeräte-Interoperabilität.

## 9. Tests und Ergebnisse

| Zeitpunkt / Prüfart | Ergebnis | Aussagegrenze |
|---|---|---|
| Damalige Inventarvalidierung | `OK: 24 services` gemeldet | Nur historischer Quellstand; geprüftes Inventory hat 25 Dienste. |
| Damaliger Full-System-Integrationscheck | `FAIL` wegen Gateway-IP `10.0.1.179` vs. `10.0.20.10` | Statische Datei-/Vertragsprüfung, keine echte LXC-/RF-Strecke. Geprüfter Check wurde nicht gestartet. |
| Damalige Dokument-QA | 400 PDF-Seiten, 29 Kapitel-/Inhaltsverzeichnis-Starts, keine gemeldeten Textüberläufe; QA-Seitenbilder nachgerendert | Damals berichtete Renderprüfung; keine erneute Binär-PDF-Prüfung in dieser Archivierung und keine technische Produktabnahme. |
| Geprüfter Repository-Abgleich | Handbuch-Trio und zwei datierte Folgeausgaben vorhanden; 25er-Inventory, 26 Katalogeinträge inkl. `shared`, IP-/Pfad-Drift und veralteter Auditbericht anhand Dateien gelesen | **Read-only-Dateiprüfung** auf `Archiving` `41161df…`; keine Ausführung der Prüfer, kein Build und keine Live-Logs. |
| On-Air, E2E, Last, Fault, Security/ETSI | Kein hier zugänglicher Lauf mit realer TBS, LXC-Gesamtanlage, SDR, Funkgeräten und Messprotokoll | **Nicht getestet / nicht im Betrieb bestätigt** für diesen Entwicklungsstand. Andere Arbeitsphasen oder zukünftige Repo-Tests sind nicht automatisch dieser Arbeitsphase zurechenbar. |

Die Vorlage [`tests/e2e/on_air_template.json`](../../tests/e2e/on_air_template.json) und das im Buch genannte Validierungsverfahren strukturieren einen künftigen Abnahmebericht; eine ausgefüllte Vorlage allein ersetzt keinen Funkversuch. Im Repo beschriebene Testskripte oder bereits generierte Markdown-Reports sind keine Beweise, dass die geprüfte Konfiguration erfolgreich geprüft wurde.

## 10. Ersetzte Ansätze und zeitliche Abweichungen

1. **93-Seiten-Fassung:** durch die ausdrückliche 400-Seiten-Korrektur als Endlieferung ersetzt. Sie bleibt zur Chronologie relevant.
2. **Älterer 17-Dienste-Guide:** [`Docs/NetCore-Tetra-Komplettguide.md`](../NetCore-Tetra-Komplettguide.md) repräsentierte laut damaligem Handbuch einen älteren Ausschnitt. Die 24er-Inventur war für Ausgabe 2.0 näher am damaligen Quellstand; zum Prüfstand vom 06.10.2026 gilt für das Beispiel-Deployment die 25er-Inventur. Ein alter Guide darf nicht stillschweigend als aktueller Portplan verwendet werden.
3. **Historische 24/25-Zählung:** 24 Runtime-LXC plus gesondertes Provisioning-Arbeitsblatt im September sind nicht die geprüften 25 Runtime-LXC plus `shared`-Katalogeintrag. Der neue `alert-service` und die späteren Handbuchausgaben sind nachträgliche Entwicklungen.
4. **Grüner generierter Auditbericht:** als aktueller Erfolgsnachweis verworfen, solange Skript, Inventory, Fallback-Matrix und Konfiguration nicht zusammenpassen und erneut ausgeführt wurden.
5. **Statische Protokoll-/Konformitätsmatrizen:** als Orientierung und Gap-Liste brauchbar, als ETSI-Zertifizierung oder reale On-Air-Funktionsbestätigung ungeeignet.
6. **Beispielkonfiguration als Runtime-Wahrheit:** bewusst nicht übernommen. Standortdateien, Secret-Handhabung, Netz-/RF-Parameter und tatsächlich gestartete Units müssen vor Ort erhoben werden.

## 11. Offene Aufgaben und Roadmap-Kandidaten

Die folgende Reihenfolge orientiert sich an technischen Abhängigkeiten. Sie ist ein Vorschlag für die Fortsetzung; eine separate Implementierungs- oder Sprintplanung wurde für das Handbuch nicht festgelegt.

| Rang | Aufgabe | Abhängigkeit / Abnahmekriterium | Status |
|---|---|---|---|
| 1 | Handbuchausgabe 2.0 gegen aktuelle 25er-Topologie und Folgeausgaben vom 27./28. September abgleichen; Seiten-/Versionshinweise und die 24/25-/Provisioning-Zählung sauber fortführen. | Zuerst Referenzcommit, Inventar und tatsächliche Servicezuordnung festhalten. Nicht Teil dieses reinen Prüfdurchlaufs vom 06.10.2026. | **geplant/offen** |
| 1 | Gateway-Adresse in TBS-Beispiel und Inventory konsistent machen; Control-Room-Zielpfad mit Unit/Installer abstimmen. | Aktive Standortwerte ermitteln; danach Konfigurations-/Startchecks. | **offen**, statisch bestätigt |
| 1 | Full-System-Prüfer, Node-Gateway-Targets, TBS-Fallback-Matrix und generierten Report auf `alert-service` und die geprüfte Dienstmenge aktualisieren. | Erst alle vier Quellen abgleichen; anschließend Check wirklich ausführen und Ergebnis committen. | **offen**, alter `PASS` ist keine Freigabe |
| 2 | 25er-Open-Lab-Inventar in einem isolierten Labor mit `validate`/`plan`/`render`/Dry-run, danach kontrolliertem Install-/Updateprozess und E2E-Profilen prüfen. | Standortkonfiguration und sichere Token-/Secret-Verwaltung, besonders beim Alert-Update. | **geplant/offen** |
| 2 | Tatsächliche On-Air-Abnahme mit mindestens zwei realen Geräten/geeigneter Messtechnik: Registrierung, Einzel-SDS, Gruppenruf/Floor/Release, Audio, Packet Data, Dual Carrier, Ausfall und Wiederkehr. | Exakte Software-/Hardwareversion, zulässige RF-Parameter, Messdaten, UTC-Zeiten und Negativfälle. | **geplant/offen**, nicht im Betrieb bestätigt |
| 2 | SDS→SDS Router→IoT Gateway→MQTT→Home Assistant mit Service-ISSI und Payload-/Ack-Korrelation nachvollziehen; SIP/RTP/Asterisk/Brew und Recorder/NFS getrennt abnehmen. | Je Übergang Logs/Topics/Call-ID und reale Endgeräte bzw. Dienste. | **geplant/offen** |
| 3 | API-/Konfig-/PDU-/SAP-/State-Zählungen der 26.-September-Ausgabe neu inventarisieren; OpenAPI-Abdeckung und ETSI-Gaps anhand geprüfter Quellen prüfen. | Automatische Zähler plus manuelle Semantikprüfung; keine pauschale Konformitätsbehauptung. | **Roadmap-Kandidat** |
| 3 | Dokumentbuild reproduzierbar ins Repo oder einen separaten Buildprozess überführen, um PDF-Seitenzahl, Inhaltsverzeichnis und Formatgleichheit bei jedem Release erneut prüfen zu können. | Quelle und Lizenz-/Anhangsgrenzen klären; Buildskripte aus dem früheren Scratch stehen hier nicht zur Verfügung. | **Roadmap-Kandidat** |
| 3 | Die im Buch genannten Wartungs-, Backup-, Restore-, Formular- und Fünf-Minuten-Abläufe mit realen Betriebsdaten pilotieren; veraltete Beispielwerte und Lücken korrigieren. | Verantwortliche, echte Units, Speicherorte, SLO/Alarme und dokumentierter Restore-Test. | **Roadmap-Kandidat** |

Kleinere weiterhin relevante Ideen aus dem Buch: konsistente Dienst-/Katalogversionierung, klare Trennung zwischen `live` und `ready`, per Call-/SDS-ID korrelierbare Logs, sichere Default-Deny-Aktorbefehle bei MQTT/HA, lokale Policy-/SDS-Weiterarbeit in Isolation, Rollback mit Datenmigrationsprüfung, Mehrhersteller-Endgeräte und messbare Audio-Lead-in-/Release-Fälle. Das sind **dokumentierte Prüffelder oder Ideen**; ihr Live-Status ist hier nicht belegt.

## 12. Quellen, Anhänge, Bilder und Provenienz

### 12.1 Direkt zugeordnete Quellen

- Die ursprünglichen Anforderungen und ihre Längenkorrektur und die damals gemeldete 400-Seiten-Übergabe samt komprimierter Herstellungs-/QA-Spur; spätere Längenkorrektur hat Vorrang.
- Historischer Quellcommit `main` [`3768f964…`](https://github.com/JanHG98/netcore-tetra/commit/3768f964100bf99e37914b8414ed611fe3bdcd67); zum Prüfstand vom 06.10.2026 gelesener `Archiving`-Ausgangscommit [`41161df…`](https://github.com/JanHG98/netcore-tetra/commit/41161df571e66e028b2984ff825212305f86606d). Die datierten Folge-Handbücher nennen eigene, oben verlinkte Quellstände. Keine zu diesem Handbuch-Erstellung eindeutig belegte PR.
- Aktuelle Repository-Dateien aus den Abschnitten 5–9, insbesondere Inventar, Dienstkatalog, TBS-Beispiele, Prüfskript, Auditbericht, Handbuch-Trio, Open-Lab-README und ETSI-Quellenregister. Dieser Abgleich ist auf den genannten Commit beschränkt.
- Die im Arbeitskontext vorliegenden **25 PDF-Referenzdateien** unter `project_sources/`: 24 einzeln benannte ETSI-/TETRA-Dateien und `25-ETSI.pdf` als Sammeldatei. Sichtbar sind unter anderem EN 300 392-2 V3.8.1 (Air Interface), EN 300 392-5 V2.7.1 (PEI), EN 300 392-7 V3.5.1 (Security), EN 300 392-1 V1.6.1 (General Network Design), weitere ISI-/Supplementary-Services- und SIM/UICC-Unterlagen. Die Existenz der Anhänge wurde festgestellt; **welche einzelnen Normstellen im ursprünglichen Herstellungsdialog vollständig gelesen wurden, ist nicht rekonstruierbar**. Sie werden hier weder als konformitätsprüfender Test noch als zu kopierende Repo-Dateien behandelt. Das Projekt-[Quellenregister](../ETSI_SOURCE_REGISTER.md) gibt den normativen Verwendungsrahmen an.

Die 25 im aktuellen Arbeitskontext sichtbaren Anhangsdateinamen sind, zur späteren Zuordnung ohne Inhaltsexport: `01-en_3003920308v010401p.pdf`, `02-en_30039209v010701p.pdf`, `03-ts_10081201v020205p.pdf`, `04-en_3003921201v010202p.pdf`, `05-en_3003920304v010301p.pdf`, `06-en_3003921117v010102p.pdf`, `07-en_3003921114v010101p.pdf`, `08-es_20081202v020401m.pdf`, `09-es_20081201v020205p.pdf`, `10-en_300812v020101p.pdf`, `11-en_3003921101v010201p.pdf`, `12-en_3003921006v010401p.pdf`, `13-en_3003921018v010301p.pdf`, `14-en_3003921216v010400a.pdf`, `15-en_30039201v010601p.pdf`, `16-ets_30039214e01v.pdf`, `17-en_30039207v030501p.pdf`, `18-en_30039401v030301p.pdf`, `19-en_3003920313v010201p.pdf`, `20-en_30039502v010303p.pdf`, `21-en_3003920303v010301p.pdf`, `22-en_30039205v020701p.pdf`, `23-en_3003920315v010500a.pdf`, `24-en_30039202v030801p.pdf` und `25-ETSI.pdf`. Dateinamen sind keine Aussage, dass alle Normtexte vollständig ausgewertet wurden.

### 12.2 Originalbilder

Für das Handbuch sind keine separaten Originalbilddateien erhalten. Die 25 Anhänge sind PDFs; die früheren Seiten-PNGs waren interne QA-Renderings und fehlen. Originale QA-Bilder können bei späterer Verfügbarkeit mit eindeutiger Herkunft ergänzt werden.

### 12.3 Was diese Archivdatei bewusst offenlässt

Der genaue Erstellungsbeginn und Teile der Rohlogs sind nicht erhalten. Ein Hashvergleich der damaligen lokalen Ausgaben mit den Repository-Binärdateien sowie eine neue PDF-Seitenprüfung stehen aus. Reale Anlage, Standortkonfiguration und Funkbetrieb wurden nicht geprüft. Die ETSI-PDFs und Zugangsdaten wurden nicht zusätzlich ins Repository kopiert. Spätere Handbuchausgaben ersetzen fehlende Tests der historischen Ausgabe nicht.

## 13. Konkreter Fortsetzungspunkt

Für eine Wiederaufnahme zuerst den aktuellen `Archiving`-/`main`-Quellstand, das 25er-Inventory und die 28.-September-Buchausgabe gegen die 26.-September-Ausgabe diffen. Danach IP-/Pfad-Drift und Prüfer/Report korrigieren, statische Checks wirklich ausführen und erst auf dieser Basis das Lab- und Funk-Abnahmeprotokoll erheben. Bei jeder neuen Handbuchausgabe Quelle, Datum, Dienstzahl, PDF-Seiten und beobachtete Testbelege neu angeben. Diese Archivierung selbst verändert **ausschließlich `Docs/archive/`** und nimmt keine Produktkorrektur vor.
