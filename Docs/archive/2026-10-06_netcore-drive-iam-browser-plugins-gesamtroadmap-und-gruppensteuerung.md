# NetCore Drive, IAM, Browser-Plugins, Gesamtroadmap und zentrale Gruppensteuerung

## 1. Metadaten und Auswertungsgrenzen

| Feld | Wert |
| --- | --- |
| Dokumenttyp | Technische Abschlussdokumentation dieses Projektchats |
| Erstellt | 2026-10-06, Europe/Berlin |
| Thema | Eigene Dateicloud; zentrale und lokale Rechte; externe Ordnerfreigaben; Browser-Plugins; repositoryweite Gesamtroadmap; Ausführung zentraler Gruppenaufträge durch die TBS |
| Ursprünglicher Chattitel | Im zugänglichen Verlauf nicht übermittelt; die Überschrift ist ein beschreibender Archivtitel |
| Ursprünglicher Chatlink | Nicht verfügbar; keine URL rekonstruiert oder erfunden |
| Repository | `JanHG98/netcore-tetra` |
| Zielbranch / erlaubte Ablage | `Archiving`, ausschließlich `Docs/archive/` |
| Heutiger geprüfter Produktstand | `main@9116c15d645458f99e236712b67a1ad970432791` |
| Geprüfter Archiv-Ausgangsstand | `Archiving@abe69d05e0749ea45929e1d168899445907b6af1`; vor Veröffentlichung erneut auf zwischenzeitliche Änderungen prüfen |
| Historischer Gesamtroadmap-Ausgangsstand | `main@e5d825b33db2e1fce14bcb2cc23e73c241873a18`, `Archiving@9af2de92b120dc612d83a17f78eeee9a97f2f775` |
| Historische gezielte Gruppenprüfung | `main@07609fb56f412ebe6e36655323e8fc6359cf90ec`; am 2026-10-06 gegen heutigen main erneut geprüft |
| Bestehende Planungs-IDs | `NETCORE-MASTER-01`, `NETCORE-DRIVE-01`, `NETCORE-IAM-01`; Aufgaben Z01–Z11, D0–D9, M0–M8 |

Ausgewertet wurden die zugänglichen Nutzeraufträge, die verfügbare Fortsetzungszusammenfassung, die noch sichtbare Antwort zur Gruppensteuerung, ergänzend auffindbarer Gesprächskontext sowie die unten genannten Repository-Dateien und Commits. Die frühen vollständigen Assistant-Antworten und ursprünglichen Designpräsentationen liegen nicht als vollständige Rohtranskription vor. Die bestätigte Designkombination und die Anforderungen sind zusätzlich in der heutigen Drive-Fachroadmap dokumentiert. Ein Kontextabruf bestätigt die frühe Assistant-Auswahl vom 2026-10-04T23:50:34Z; dies entspricht dem 05.10.2026 in Europe/Berlin. Der Abruf liefert keinen exakten Chattitel, Chatlink oder vollständigen Bildnachweis.

Diese Datei ist keine behauptete vollständige Wort-für-Wort-Auswertung aller möglicherweise früheren Turns. Sie bewahrt den nachvollziehbaren fachlichen Endstand, die belegten Dokumentationsänderungen und die verbleibenden Lücken. Andere Projektchats werden nur als konkret verlinkte Quellen behandelt; daraus wird kein in diesem Chat durchgeführter Test oder Deployment abgeleitet.

### Statusbegriffe

| Begriff | Bedeutung in diesem Archiv |
| --- | --- |
| Idee | Erwogen, noch ohne verbindliche technische Auswahl |
| Beschlossen / geplant | Vom Nutzer verlangtes Ziel oder dokumentierter Arbeitsumfang; Funktion noch nicht allein dadurch vorhanden |
| Implementiert | Konkreter Code oder Dokumentationsinhalt im angegebenen Repository-Commit vorhanden; Dokumentation und Runtime getrennt benennen |
| Getestet | Ein ausgeführter Test mit Ergebnis ist nachgewiesen; Quelllesen ist nur statische Prüfung |
| Im Betrieb bestätigt | Konkreter erfolgreicher Lauf an der tatsächlichen Anlage / am Endgerät belegt |

Für diesen Chat sind **Roadmapdateien und Verweise implementiert und remote verifiziert**. Die neue Drive-Anwendung, zentrale IAM-Integration und Behebung der zentralen Gruppenhandler sind **geplant**, nicht durch diese Dokumentationsaufträge implementiert oder im Betrieb bestätigt.

## 2. Ziel, Ausgangslage und Verlauf

Jan schlug eine eigene Cloud für Dateien vor: funktional in Richtung Nextcloud, ohne MariaDB als zwingende Voraussetzung und mit eigenem NetCore-Design. Nach Annahme der Designrichtung wurde der Umfang konkretisiert: **„Funktionsumfang Nextcloud und Simplizität wie OneDrive“**. Dazu kamen gemeinsame Rechte im Ökosystem, Freigabe kompletter Ordner an externe Personen und ein eigenständiger Start, falls noch kein zentraler Identitäts-/Rechtedienst existiert.

Der Nutzer schrieb bei der Rückfallebene „EBAC“. Der weitere Dialog und die gespeicherten Fachroadmaps behandeln dies als die noch nicht vorhandene zentrale IAM-/RBAC-Ebene. Es wurde kein separates EBAC-Produkt, Schema oder Protokoll definiert; die originale Wortwahl darf nicht als eigenständige implementierte Komponente ausgegeben werden.

| Reihenfolge | Nutzerauftrag / Ergebnis |
| --- | --- |
| 1 | Eigene Dateicloud und Designvorschläge; bestätigte Kombination aus Core Console, Workspace und Archive Studio |
| 2 | Zentrale Rechte im gesamten Ökosystem und externe Freigabe ganzer Ordner; Produktleitlinie Nextcloud-Dateifunktionen / OneDrive-Bedienung |
| 3 | Drive-Plan in GitHub-Roadmaps aufnehmen; ausdrückliche Freigabe für main |
| 4 | Rückfallebene ohne schon vorhandenes zentrales IAM/RBAC einplanen |
| 5 | Browser-Plugin-Plattform für PDF, Office, 3D, Schaltpläne, Vektoren, Bilder, Video und Audio ergänzen |
| 6 | Alle Branches und das gesamte Repository für eine Gesamtfolge betrachten; zunächst ausdrücklich nur berichten, nichts ändern |
| 7 | Diese Gesamtroadmap anschließend auf main zentral ablegen, damit neue Chats sofort den ersten Schritt finden |
| 8 | Ergänzen, dass die Basisstation eingehende zentrale Gruppenzuweisungen umsetzt und nicht als unbekannten / nicht unterstützten Befehl ignoriert |
| 9 | Abschlussdokumentation und Index auf Archiving anlegen; dieser Auftrag erlaubt keine Änderungen außerhalb von Docs/archive |

Die Schreibfreigabe für main galt den vorherigen Roadmapänderungen. Der jetzige Archivauftrag hat den engeren, ausdrücklich festgelegten Zielbranch `Archiving`. Er löst keine Runtime-Implementierung, keinen Branch-Merge und kein Anlagenupdate aus.

## 3. Endgültige Drive-Anforderungen und offene Auswahl

### 3.1 Oberfläche und Produktumfang

| Ansicht | Bestätigte Aufgabe |
| --- | --- |
| Core Console | Hauptoberfläche mit kompakter Dateiliste und Details |
| Workspace | Startansicht mit angehefteten Ordnern und zuletzt verwendeten Dateien |
| Archive Studio | Bibliotheks-/Dokumentmodus mit Vorschau |

Die drei Ansichten bilden **eine Anwendung mit denselben Dateien, Rechten und Plugins**, keine drei getrennten Dateisysteme. Eigenes NetCore-Design und sinnvolle mobile Bedienung gehören zum Ziel. Exakte Originalmockups, Farben, Schriftgrößen oder Bildabmessungen sind aus dem derzeit zugänglichen Verlauf nicht belegbar und werden hier nicht ergänzt.

Der bestätigte Dateicloud-Umfang umfasst:

- Dateien und Ordner erstellen, hochladen, herunterladen, verschieben, umbenennen und löschen; Listen- und Bibliotheksansicht.
- Große Uploads in Teilen, Fortschritt und Wiederaufnahme; unvollständige Teilstände nicht als vollständige Datei veröffentlichen.
- Suche, Favoriten, zuletzt verwendet und geschützte Dokument-/Medienvorschau.
- Versionen, Papierkorb und Wiederherstellung mit definierten Aufbewahrungsregeln.
- Interne und externe Datei-/Ordnerfreigaben mit nachvollziehbarer Zugangsverwaltung.
- Desktop-/Mobilzugriff, Offline-Synchronisation und Konfliktbehandlung.
- Kommentare und passende Browser-Editoren; gemeinsames Bearbeiten als gesondert abgenommener Umfang.
- Installierbare Viewer, Player, Konverter und geeignete Editoren über eine Plugin-Plattform.
- Konsistente Sicherung / Wiederherstellung von Inhalten, Metadaten, Identitäten, Freigaben und Versionen; Quotas, Monitoring und Verwaltungsnachweise.

Der Satz „Funktionsumfang Nextcloud“ wurde in der Roadmap auf überprüfbare **Dateicloud-Funktionen** konkretisiert. Die vollständige Parität mit allen Nextcloud-Hub-Apps ist keine zugesagte erste Version. Der erste Releaseumfang wird in D0 festgelegt.

### 3.2 Dateiinhalt und Datenbank

Dateiinhalt soll auf NAS / Dateispeicher liegen. Verwaltungsdaten, Rechte und Versionsinformationen werden getrennt gehalten. MariaDB ist **keine Voraussetzung**; daraus folgt nicht „ohne Metadatenbank“. Ein konkreter Drive-Mount, LXC, Port, DNS-Name, Client oder Datenbankschema wurde noch nicht ausgewählt.

| Ansatz | Historische Einordnung / heutiger Status |
| --- | --- |
| Kleiner eigener Dateidienst im LXC mit NAS und lokalem SQLite | Früher Vorschlag für begrenzten Umfang; keine bestätigte Produktionsarchitektur |
| Eigene NetCore-Oberfläche vor Nextcloud Files mit PostgreSQL | Bevorzugte Empfehlung zur Prüfung, nachdem der Umfang auf umfassende Datei-/Sync-Funktionen erweitert wurde |
| Vollständiger Eigenbau | Weiter zu bewertende Alternative; Sync, Konflikte, Versionierung und Rechte werden dann eigene Entwicklungsaufgaben |

Bei einer begrenzten SQLite-Eigenbauvariante gehört die Datenbank auf lokalen persistenten Speicher und nicht auf den NFS-/SMB-Dateimount. Backend, unterstützte Version, Integrationsweg der eigenen Oberfläche, Clients und Migrationsaufwand bleiben offen. Die Empfehlung „Nextcloud Files + PostgreSQL“ ist weder ein beschlossener Installationsbefehl noch eine belegte schon laufende Cloud.

Vorhandene Recorder-/Media-Library-Dateien werden nicht ungeprüft automatisch als Drive-Bestände importiert. Funkbetrieb und bestehende Mediendienste dürfen nicht von der Drive-Anwendung oder einer erreichbaren Web-Anmeldung abhängig werden.

## 4. Rechte, lokale Anfangsebene und spätere IAM-Migration

### 4.1 Ökosystemweite Rechte

Das Ziel ist zentrale Verwaltung menschlicher Identitäten, Gruppen und Dienstrollen für Basisstations-Dashboard, Control Room, Zentraldienste und Drive. **Jedes Fachbackend setzt seine Rechte weiterhin selbst durch.** Identität, Rolle / Einzelrecht und Ressourcenbereich bilden zusammen die Berechtigung; Zugriff auf eine TBS berechtigt nicht automatisch zu allen Stationen, Dateien oder Aufnahmen.

Menschliche Benutzer, Gäste, Maschinenkonten und TETRA-Teilnehmer sind unterschiedliche Identitätsklassen. Web-IAM ersetzt weder TETRA-Authentisierung noch KMF / Funk-Security. Eine Dienstrolle ist kein pauschaler Zugriff auf Dateiobjekte; Drive-Datei-/Ordnerrechte bleiben im zuständigen Backend verbindlich.

Der Control Room besitzt bereits eigene lokale Benutzer und Rollen `Node`, `Viewer`, `Operator`, `Admin`. Vorschläge für gemeinsame Rollen lauten `viewer`, `operator`, `technician`, `administrator`, `auditor`, `owner`. Ihr Mapping, insbesondere zum bestehenden `Admin`, ist noch festzulegen; keine automatische Rechteerhöhung aus ähnlichen Namen ableiten.

### 4.2 Was die verlangte Rückfallebene bedeutet

**Beschlossen/geplant:** Drive kann zuerst vollständig eigenständig mit lokalen Konten, Gruppen, Rollen, Dienstidentitäten und Ressourcenrechten laufen. D0–D5 warten nicht auf einen zentralen IAM-Dienst. Dieser lokale Betrieb ist eine abgesicherte reguläre Anfangsebene mit vollständigen Freigaberechten; kein offener Open-Lab-Zugang.

Die spätere zentrale Anbindung wird separat in D6 entwickelt und abgenommen:

- Eigentum, ACLs und Freigaben an stabile interne Drive-Identitäts- und Objekt-IDs binden.
- Zentrale Konten erst nach bestätigter Zuordnung über `issuer` + `subject` verbinden; gleicher Name oder gleiche E-Mail allein genügt nicht.
- Gruppen und Dienstrollen ausdrücklich abgleichen; Dateien, Versionen, Papierkorb, Gäste und Freigaben erhalten, keine zusätzlichen Rechte erzeugen.
- Sicherung, Probemigration und geprüften Rückweg vorsehen.
- Reguläre lokale Passwortlogins migrierter Konten deaktivieren.

**Nach zentraler Migration kein automatischer Wechsel auf regulären lokalen Login bei IAM-Ausfall.** Bestehende zentrale Sessions / Tokens gelten nur innerhalb noch festzulegender Gültigkeits- und Sperrfristen. Ohne IdP keine neuen Logins oder Tokenverlängerung. Ein gezielt provisionierter, gehashter, begrenzter und protokollierter lokaler Notzugang ist ein eigener Mechanismus; kein stilles Standardpasswort und kein offener Zugriff.

JWKS-Caching dient der Prüfung bereits vorhandener Tokens, nicht der Neuanmeldung. Es ermöglicht bei AD-Ausfall keinen Offline-AD-Passwortlogin. Numerische Token-/Sitzungs-/Sperrfristen wurden noch nicht festgelegt.

### 4.3 Vorgeschlagene zentrale Komponenten

Eigener Identity-LXC, Keycloak, zunächst PostgreSQL im selben LXC und eine gemeinsame Rust-Integration `netcore-auth` sind **Empfehlungen**, nicht endgültig ausgewählte oder installierte Komponenten. Authentik bleibt Alternative; bestehende Deployment-VM ist eine Pilotoption; AD-Anbindung optional. Der genaue Rust-Paketpfad ist offen. Kopplung an Control Room oder Security Core ist nicht das empfohlene Ziel; Hochverfügbarkeit folgt als eigene Betriebsaufgabe.

Noch zu entscheiden sind Produkt / Version, Betriebsort, DB / Backup / Verfügbarkeit, DNS / HTTPS / vertrauenswürdiger Issuer, AD-/Gruppenmapping, Delegation, Ressourcenbereiche, Fristen, MFA, letzter Administrator, isolierter Neustart und Notzugang. Die allgemeine Open-Lab-Nutzung vorhandener Managementdienste ersetzt keinen sicheren externen Drive-Betrieb.

## 5. Freigabe kompletter Ordner an externe Personen

### 5.1 Geplanter Bedienablauf

Ordner auswählen → **Freigeben** → Person / Gruppe oder **Jeder mit dem Link** → Recht → optional Ablaufdatum → Einladung / Link. **Zugriff verwalten** zeigt bestehende Zugänge samt Herkunft und Laufzeit. Komplexere Einstellungen gehören unter **Weitere Optionen**. Dieser OneDrive-nahe Ablauf ist ein UI-Ziel, keine schon implementierte Oberfläche.

| Freigabeart | Verbindliche Unterscheidung |
| --- | --- |
| Bestimmte externe Personen | Standard für personenbezogene Freigabe; bestätigte Gastidentität. E-Mail-Eintrag allein ist keine Identitätsprüfung; weitergereichter Link allein berechtigt keine weitere Person |
| Interne Personen / Gruppen | Zunächst lokale Drive-Identitäten; später geprüft auf zentrale Identitäten / Gruppen abbilden |
| Jeder mit dem Link | Langer zufälliger Linktoken; optional Passwort und Ablauf. Weitergebbar, deshalb keine bestätigte Personenidentität |

Ein Gast benötigt kein internes AD-Konto und keinen schon vorhandenen zentralen IAM-Dienst. E-Mail-Codes wurden als mögliches Prüfverfahren vorgeschlagen, aber nicht als technische Lösung ausgewählt oder implementiert. Gastprüfung, Mailzustellung und Sessions bleiben Architekturarbeit.

| Recht | Zielverhalten |
| --- | --- |
| Ansehen | Anzeigen und herunterladen |
| Bearbeiten | Zusätzlich hochladen, ändern, umbenennen und löschen; Löschrecht gegebenenfalls separat abschaltbar |
| Nur hochladen | Uploadbriefkasten ohne Einsicht in vorhandene Dateien |
| Weiterfreigeben | Eigenständiges Recht; extern nicht automatisch vergeben |

### 5.2 Vererbung und Widerruf

Die Ordnerfreigabe gilt für Unterordner und später hinzugefügte Inhalte. Übergeordnete / benachbarte Ordner bleiben verborgen, auch bei direkter Objekt-ID oder manipuliertem Pfad. Umbenennen erhält die stabile Identität. Hineinverschieben erteilt geerbten Zugang, Hinausverschieben entzieht ihn; eigenständige weitere Freigaben bleiben separat nachvollziehbar.

Ablauf und Widerruf müssen Sessions, Caches, Download-URLs und Sync innerhalb einer noch festzulegenden Durchsetzungsfrist erreichen. Bereits heruntergeladene Offlinekopien lassen sich nicht zurückholen. Identische serverseitige Rechte gelten für Anzeige, Vorschau, Suche, Versionen, Papierkorb, Download, ZIP, WebDAV / Sync und direkte API-Aufrufe.

Externe Nutzung benötigt einen erreichbaren HTTPS-Endpunkt. NAS-, TBS- oder sonstige Managementoberflächen werden dadurch nicht mit veröffentlicht. Konkretes Hosting, URL, Zertifikatsverfahren und Maildienst sind noch offen.

## 6. Browser-Plugins und Formatgrenzen

Der Nutzer verlangt Viewer / Player und geeignete Bearbeitung direkt im Browser. Dafür ist eine allgemeine Plugin-Plattform geplant; sie soll auch im lokalen Drive-Betrieb funktionieren und nicht auf D6 / zentrales IAM warten.

| Familie | Geplante Fähigkeiten / Kandidaten | Eigene Abnahmegrenzen |
| --- | --- | --- |
| PDF | Seitenübersicht, Zoom, Suche, berechtigter Druck | Annotationen separat; große / geschützte Dokumente prüfen |
| Word / Text | DOCX / ODT anzeigen; geeigneter Editor | Layout, Schriften, Kommentare, Speichern und Rückexport |
| Excel / Tabellen | XLSX / ODS / CSV | Formeln, Diagramme, Zellformate und Rückexport |
| Präsentationen | PPTX / ODP anzeigen / präsentieren | Editor abhängig von Office-Integration |
| 3D / CAD | STL / OBJ / glTF / GLB, Drehen und Zoom | STEP / native CAD, Messung, Baugruppen und Bearbeitung gesondert |
| Schaltpläne / EDA | PDF-/SVG-Exports und native Projekte, etwa KiCad | Seiten, Ebenen, Symbole und Referenzen; native Darstellung / Bearbeitung speziell prüfen |
| Vektor | SVG als Kandidat; weitere Formate gesondert | Ebenen, Schriften, Export und aktive Inhalte |
| Bilder | JPEG / PNG / WebP, Galerie, Zoom, Metadaten | Bearbeitung versioniert; RAW / Spezialformate separat |
| Video | Player, Spulen, Untertitel | Container und Codecs einzeln deklarieren; Range, Browser, Originalstream / isoliertes Transcoding |
| Audio | Player, Spulen, Metadaten | Lange Aufnahmen / Widerruf; Playlists und Wellenform später |

Diese Tabelle ist eine **geplante Abnahmematrix**, keine getestete universelle Format- oder Bearbeitungskompatibilität. Viewer, Editor, Konverter und Zusammenarbeit werden pro Integration getrennt angegeben.

### Plugin-Vertrag und Lebenszyklus

- Versioniertes Manifest mit ID, Version, kompatibler Drive-Schnittstelle, MIME / Endungen, Fähigkeiten, Ressourcen, Dateizugriffen und Serverkomponenten. Noch keine konkrete Manifestdatei oder fertiges Schema festgelegt.
- Gemeinsame Erweiterungspunkte für Öffnen, Vorschau, Bearbeiten, Player und Export; Thumbnail / Suchindex optional.
- Geprüfte Quellen / Integrität, Installation, Update, Aktivierung, Deaktivierung und Versionsrückweg. Updates erweitern Rechte nicht automatisch; Deinstallation löscht keine Dokumente.
- Benutzer wählen eine Standardanwendung aus freigegebenen kompatiblen Plugins.
- Datei-, aktions- und zeitbegrenzte Zugriffe. Kein allgemeiner Drive-/IAM-Sitzungstoken und kein direkter NAS-/beliebiger Dateipfadzugriff für Plugins.
- Referenzierte Modelle, Symbole und Bilder jeweils gesondert autorisieren. Browsermodule und Konverter isolieren; Zeit, Speicher und Netzwerk begrenzen.
- Ableitungen / Vorschaudateien an Dateiversion und Rechte binden; temporäre URLs und Caches bei Widerruf / Löschung berücksichtigen.
- Originaldatei erhalten. Editoren speichern versioniert mit Konflikt-/Sperrverfahren; verlustbehafteten Rückexport kenntlich machen.

Konkrete Office-Suite, 3D-Engine, EDA-Viewer, Konverter, Codecs und Plugin-Verteilquelle wurden nicht ausgewählt. Es wurden in diesem Chat keine Pakete installiert und keine Format-Roundtrips ausgeführt.

## 7. Drive- und IAM-Meilensteine

Alle nachfolgenden technischen Meilensteine sind im heute geprüften Stand **geplant / offen**. Die IDs sind Arbeitsreferenzen, keine Fertigstellungsnachweise und keine verbindlichen Kalendertermine.

### 7.1 Drive D0–D9

| ID | Umfang | Eintritt / Abnahme |
| --- | --- | --- |
| D0 | Umfang und Architektur | Backend / Daten / UI, lokaler Login und stabile IDs, Gäste / Freigaben, Clients, Plugins / Formate, erster Release und späterer IAM-Adapter in ADR festlegen; kein zentraler IAM-Dienst nötig |
| D1 | Backend und lokaler Pilot | D0; Konten / Gruppen / Dienste anlegen und sperren, ACL, Upload / Download, Neustart und gemeinsamen Restore prüfen |
| D2 | NetCore-UI und lokaler Login | D1; drei Ansichten und mobile Bedienung auf demselben Bestand, erlaubte / verbotene Zugriffe |
| D3 | Ordnerfreigaben | D2 und Gast-/Rechtevertrag D0; Personen / Gruppen / Links, drei Rechte, Vererbung / Ablauf / Widerruf positiv und negativ |
| D4 | Upload, Vorschau, Versionen, Papierkorb | D1 / D2 plus D3-Rechte; Unterbrechung, Paralleländerungen, Restore und alle Zugriffspfade |
| D5 | Sync und Betriebsabnahme | D3 / D4 und Clients; reale Konflikt-/Rechteentzugs-/Quota-/Restoretests ohne IAM, Monitoring und Rückweg |
| D6 | Zentraler Login und Migration | Abgenommener lokaler Betrieb, IAM M1 / M2 und verfügbarer OIDC-Adapter; M3 nur bei Nutzung des gemeinsamen Rust-Bausteins; Rechteerhalt, SSO / Clients, lokale Logins migrierter Konten abschalten, Ausfall / Rückweg |
| D7 | Plugin-Plattform und Basismodule | D0 / D2 / D3 / D4, kein D6; PDF / Bild / Audio / Video, APIs / Verwaltung / Isolation / Streaming, Gäste / Ableitungen und Update / Rückweg |
| D8 | Office | D7 plus gewählte Integration; Anzeige / Bearbeitung im vereinbarten Umfang, Layout / Formeln / Export / Versionen / Konflikte; Zusammenarbeit separat |
| D9 | Technische Module | D7 plus Formate / Konverter; 3D / Schaltpläne / Vektor, große Modelle / Referenzen / aktive Inhalte / Rechte / Isolation |

**D0–D5 und D7–D9 warten nicht auf D6.** D8 und D9 können nach D7 parallel bearbeitet werden. Die Gesamtpriorität ergibt sich aus NETCORE-MASTER-01; eine Fachroadmap-Priorität ersetzt diese Reihenfolge nicht.

### 7.2 IAM M0–M8

| ID | Geplantes Ergebnis |
| --- | --- |
| M0 | Tatsächliche Logins, APIs, Maschinenidentitäten und geschützte Aktionen vollständig inventarisieren |
| M1 | Architekturentscheidung, Rollen-/Ressourcenvertrag, Fristen, Migration / Ausfall / Notzugang |
| M2 | Identity-Pilot mit ausgewähltem Anbieter |
| M3 | Gemeinsamer Adapter: Authorization Code / PKCE, state / nonce, sichere Sessions / CSRF, Signatur / Algorithmus / Issuer / Audience / Ablauf / Tokenart, JWKS / Rotation und Ressourcenprüfung |
| M4 | Eine TBS als isolierter Pilot |
| M5 | Control Room und weitere Zentraldienste migrieren |
| M6 | Discovery / Deployment integrieren und reguläre lokale TOML-Logins ablösen |
| M7 | Betriebsabnahme einschließlich negativer Rechtefälle, Sperren, Ausfall, Rotation, Restore und Rückweg |
| M8 | Optionale NetCore-IAM-UI, AD-Ausbau, NFC / RFID und höhere Verfügbarkeit |

## 8. Repositoryweite Gesamtroadmap

### 8.1 Zentrale Ablage und Fortsetzungsregeln

Beschlossen und als Dokumentation umgesetzt ist **`ROADMAP.md` im Repository-Root auf main**. Sie führt `NETCORE-MASTER-01`, dauerhaft benannte Aufgaben, Abhängigkeiten, Nachweisstufen und einen konkreten nächsten Schritt. `AGENTS.md` sowie Verweise aus README, Backend-, Wiki-, IAM- und Drive-Roadmap führen dorthin.

Neue Chats sollen zuerst die aktuelle main-Roadmap und relevante neue Änderungen lesen, gültige Belege bei unveränderten Inhalten wiederverwenden und die höchstpriorisierte offene, abhängigkeitsbereite Aufgabe im aktuellen Nutzerauftrag nennen. Eine Frage „Was zuerst?“ ist zunächst Auskunft, kein automatischer Runtime-Auftrag. Bereits ausdrücklich autorisierte Arbeit benötigt keine künstliche zusätzliche allgemeine Bestätigung. Dokumentationsfortschritt ist kein Implementierungsfortschritt.

Am geprüften Archiving-Ausgangsstand fehlen `ROADMAP.md`, `AGENTS.md` und `Docs/NETCORE_DRIVE_ROADMAP.md`. Deshalb verweisen die Quellen dieses Archivs für diese Dateien auf den geprüften **main-Commit**. Sie werden hier nicht in den Archivbranch gemergt oder außerhalb von Docs/archive nachkopiert.

### 8.2 Vereinbarte Gesamtfolge

| ID | Priorität | Arbeitsblock / Abhängigkeit / Abschluss |
| --- | --- | --- |
| Z01 | P0, erster Block | Repository / Installation konsolidieren: fehlende Deployment-, Discovery-, Imagebuilder-, VPN- und Syslog-Entwicklung kontrolliert übernehmen; Inventory, Readiness und CI; anschließend Installation / Upgrade / Recovery abnehmen |
| Z02 | P0, gezielt parallel zu Z01 | Zwei Restore-Fixes, Management-/Packet-Core-Routenschutz, aktive SAP-/Downlinkwege und zentrale Gruppenaufträge auf der TBS; positive / negative Regressionen |
| Z03 | P0, erstes Systemgate | Einzelzelle / Core, lokale Funkfunktionen, Dual Carrier, Matrix und Edge-Fallback nach Z01 / Z02; echte Labor-/On-Air-Nachweise mit Build, Konfiguration, Firmware und Logs |
| Z04 | P1, Architektur jetzt / Pilot danach | IAM M0 / M1 parallel vorbereiten, isolierter Identity-/TBS-/Control-Room-Pilot; breite Migration nach Pilotabnahme |
| Z05 | P1 nach Grundabnahme | SDS / Status / GPS, HA / MQTT, Warnzentrale, SIP / RTP, Recording, TTS / Media Library und Paketdaten als vollständige Alltagspfade; Zielwirkung, Rückmeldung und Ausfälle |
| Z06 | P1, eigener Produktstrang nach ersten P0-Fixes | Drive D0–D5 lokal unabhängig von zentralem IAM; Dateien, Freigaben, Versionen, Papierkorb, Backup und Sync; Plugin-Vertrag von Beginn an |
| Z07 | P1 nach stabiler Einzelzelle / Core-E2E | Zweite TBS unabhängig abnehmen; Registrierungen / neue Rufe, dann Kontexttransfer / Restore und laufende Medien; Seamless Handover erst nach gemessener Unterbrechung / Floor-/Fehlerabnahme |
| Z08 | P1, praktische Erweiterungen | Headset / Mikro / PTT und Leitstellen-Audio, echte HA-Aktoren, Rack-/RF-Sensorik, Asset-/Task-Rückweg; passende Fachpfade und echte Hardwarewirkung voraussetzen |
| Z09 | P1 nach Drive-Dateikern | D7 Plugin-Plattform / PDF / Bild / Audio / Video; D8 Office und D9 technische Module danach; kein IAM-/D6-Startgate |
| Z10 | P1 / P2 auf abgenommenem Grundbetrieb | Gesicherter Betrieb, Last / Dauerlauf, Upgrade / Restore; Backend-HA, produktive Funk-Security / OTAR, Regionen und ETSI ISI als gesonderte Ausbaustufen |
| Z11 | P2 nach praktischem Nutzen | Android / Hybrid, Zebra / QR-Inventur, WERMA, USB-Audio / PTT, FRN / Zello und weitere Anwendungen; vorhandene Autoritäten verwenden und archivierte Ideen aktuell prüfen |

Keine Termine oder Fertigstellungsprozente vereinbart. Z01 / Z02 fokussiert zuerst; IAM-Inventur und Drive-Architektur dürfen parallel vorbereitet werden. Die IDs bedeuten nicht, dass sämtliche großen Blöcke gleichzeitig gestartet werden.

### 8.3 Aktueller erster Schritt Z01.1

**Z01.1 bleibt die erste Gesamtaufgabe:** heutigen main-Tip vollständig mit dem historischen Feature-Tip `bbf039729b9b05f8d623b11195ca24a124f68d16` vergleichen und einen konkreten Integrationsplan erstellen. GitHub-Compare seit gemeinsamer Basis allein ist kein vollständiger Vergleich der beiden Tip-Bäume.

Zu erfassen sind fehlende Dateien, überlappende Änderungen, Konflikte, Konfigurationsübernahme, Tests und Rückweg für Deployment / Discovery, Imagebuilder / VPN und Observability / Syslog. Neue main-UI-/Fachänderungen erhalten. Repository-Inventar und zugängliche Installationsnachweise mit Hostrolle, Quell-SHA, Binaryversion und Konfiguration abgleichen; fehlende Live-Daten als unbekannt führen. Kein ungeprüftes Cherry-Pick eines alten Commitstapels.

**Abnahme:** Vergleichstabelle und prüfbarer Integrationsplan mit Quellen-SHAs, Konflikten, gültigen Nachweisen und Lücken. Live-Zugriff ist für den Quellvergleich kein zwingendes Gate. Danach Z01.2 Integration, Z01.3 Inventory / Ready-Schranke / CI, Z01.4 Installation / Upgrade / Recovery mit ARM64- und echtem Pi-/SXceiver-Nachweis.

Historischer Nachweis: Der am 2026-10-06 erneut abgerufene [CI-Lauf 36349402097](https://github.com/JanHG98/netcore-tetra/actions/runs/36349402097), „OpenLab deployment and discovery“, lief am 2026-09-27 für den Feature-Tip `bbf039729b9b05f8d623b11195ca24a124f68d16` mit Ergebnis `success`. Die frühere Sichtung ordnete die Imageprüfungen als Teilnachweise ein; ein vollständiger NetCore-Image-Build und physischer Pi-/SXceiver-Boot sind dadurch nicht nachgewiesen. Kein heute neu gestarteter CI-Lauf und kein Nachweis der Integration in main.

### 8.4 Übrige P0-Aufgaben und frühe Abnahmepfade

- **Z02.1 Einzelruf-Restore:** Floor nur vergeben, wenn frei oder bereits beim anfragenden Teilnehmer; bei Simplex passenden Owner setzen. Konkurrierenden Teilnehmer und wiederholten Restore prüfen.
- **Z02.2 Gruppenruf-Restore:** Nach Floor-Grant die untere Funksteuerung benachrichtigen und Uplink-Sprachframes überwachen. Stummen Restore, Timer, Release und Wiederverwendung prüfen.
- **Z02.3 IP-Routenschutz:** Managementadresse / -netz und Packet-Core-Abhängigkeiten beim Schreiben, Kernel-Reconcile und Restore vor schädlichen TUN-Routen schützen; gespeicherte Alt-Konflikte behandeln. Historisches Beispiel `10.0.1.0/24 → ntc-tun0` ist keine heute neu bestätigte Störung.
- **Z02.4 Runtimepfade:** Tatsächlich aktive SAP-/SNDCP-/MLE-Downlinkwege und Fähigkeiten prüfen; isolierte Two-Cell-/Restore-Bausteine nicht als integrierten Funkbetrieb melden.
- **Z02.5 zentrale Gruppensteuerung:** Details in Abschnitt 9; unabhängige P0-Arbeit parallel zu Z01 möglich.

Für Z03 / Z05 sind TBS-Hello, richtige Serving-TBS / aktuelle Service-Matrix, Registration, lokale Einzel-/Gruppenrufe, Dual-Carrier-Belegung, Floor / Hangtime / Release, SDS / Status / GPS bis Control Room / HA, Warn-/Alarm-/Task-Dedupe, hörbare TTS-Durchsage samt Archiv und Rufabbau, Recorder-/NAS-Ausfall, SIP / RTP sowie Paketdaten mit passendem Teilnehmerkontext durchgehend abzunehmen. LXC-, Gateway-, VPN-, Speicher- und Stromausfall mit Wiederkehr gehören dazu. MQTT-Verbindung, HTTP 200, vorhandener E2E-Runner oder zwei erreichbare TBS ersetzen keine tatsächliche Fachwirkung.

## 9. Zentrale Gruppenzuweisungen: Fehlerbild, Ursache und geplante Behebung

### 9.1 Präziser Scope

Der letzte fachliche Nutzerauftrag verlangte, dass eingehende Gruppenzuweisungen von der Basisstation umgesetzt und nicht als unbekannter Befehl ignoriert werden. Im Kontext wurde dies **als Roadmap-Ergänzung** auf main ausgeführt. Eine Runtime-Reparatur wurde dabei nicht vorgenommen. Der Nachweis bezieht sich auf die konkreten zentralen Typen `GroupAccessPolicyApply` und `GroupDgnaApply`; eine universelle Protokollnachricht `group_command` wurde nicht nachgewiesen.

### 9.2 Heute belegter Codepfad

| Schritt / Komponente | Vorhanden | Verbleibende Lücke |
| --- | --- | --- |
| Group Core, `src/state.rs` | Erstellt Policy-Aufträge und zentrale DGNA-Operationen; erwartet fachliche Antworten / führt Timeouts | Policy-Sync bei fehlender `group_policy`-Capability als Unsupported; vorhandener Produzent ist kein Endgerätenachweis |
| Node Gateway / TBS-Worker | Transport zum Node; Worker routet beide zentralen Typen an MM und korreliert Handles / Antworten | Positives Dispatcher-ACK sagt nur „dispatched to Mm“ |
| MM, `mm_bs.rs::tick_start` | Verarbeitet für Gruppensteuerung lokales `ControlCommand::Dgna` | Beide zentralen Typen landen weiterhin in `MM: ignoring unsupported control command` |
| Lokales `do_dgna` | Aktualisiert MM-Gruppen, gemeinsam genutzten Subscriberzustand, CMCE und Wiederherstellungszustand; reiht D-ATTACH/DETACH GROUP IDENTITY ein | Zentrale Policy-/DGNA-Handler fehlen; lokale Zustandsänderung ist optimistisch vor Terminalbestätigung |
| Capability-Ankündigung | `dgna=true`, `group_policy=false` | Muss nach tatsächlich implementierter zentraler Unterstützung abgestimmt werden |
| Terminal-ACK-Handler | Dekodiert / protokolliert U-ATTACH/DETACH GROUP IDENTITY ACKNOWLEDGEMENT | Noch keine korrelierte Abschlussantwort für zentralen Auftrag |

Group Core behandelt einen positiven Worker-ACK bereits nicht als endgültiges `Applied`; dafür ist die fachliche Antwort erforderlich. Die Ursache liegt somit nicht allein im Transport und nicht in einem komplett fehlenden lokalen DGNA-Pfad, sondern im **fehlenden Anschluss der zentralen Befehlstypen an den aktiven MM-Dispatcher und deren Ergebnisführung**.

Zusätzlicher statischer Befund: Der Sendehelfer `send_d_attach_detach_group_identity` liefert keinen Erfolgswert. Ein Serialisierungsfehler lässt sich deshalb derzeit nicht sauber in den `true`-Rückgabewert des Aufrufers abbilden. Dies wurde nicht durch eine ausgeführte Fehler-Injektion getestet.

### 9.3 Vorhandener interner Vertrag

| Typ / Felder | Geplante Verwendung |
| --- | --- |
| `GroupAccessPolicyApply` | `handle`, `revision`, `allow_unlisted_groups`, `enforce_memberships`, `reconcile_registered`, `groups`, `memberships` |
| `GroupPolicyDefinition` | GSSI, enabled, Attach-/DGNA-/Call-/SDS-/Emergency-Regeln, Call-Priorität und `class_of_usage` |
| `GroupMembershipPolicy` | ISSI, GSSI, `allowed`, `auto_attach`, `locked` |
| `GroupDgnaApply` | `handle`, `issi`, `gssi`, `attach`, `force` |
| `GroupAccessPolicyApplied` | Handle / Revision / Erfolg, Gruppen-/Mitgliedschaftszahlen, Attach-/Detach-Zahlen und Meldung |
| `GroupDgnaApplied` | Handle / ISSI / GSSI / Attach-Richtung / Erfolg und Meldung |

Diese Namen sind NetCore-interne Verträge, keine ETSI-PDU-Namen. Statische Provisionierung, dynamische Gruppen und lokale Attach-/Detach-Signalisierung müssen passend zur Gerätefähigkeit auseinandergehalten werden; nicht jede Konfigurationsänderung ist automatisch eine vollständige SS-DGNA-Implementierung.

### 9.4 Geplanter Umfang Z02.5

- Zentrale Typen im MM-Dispatcher validieren, tatsächlich ausführen und fachliche `...Applied`-Antworten korrelieren. Bestehenden lokalen DGNA-Pfad erhalten.
- Gruppenprofile / Mitgliedschaften revisioniert einschließlich `class_of_usage`, `auto_attach`, `locked`, Mitgliedschaftsregeln und `reconcile_registered` übernehmen.
- Zuweisung und Entziehung wirksam in MM / Subscriberzustand / CMCE und, soweit erforderlich / unterstützt, über Funk am registrierten Teilnehmer ausführen. Andere vorhandene Gruppen erhalten.
- Richtige Serving-TBS, gültige ISSI / GSSI und Gerätefähigkeiten prüfen. `force` ist bewusster Operator-Override gemäß vorhandenem Vertrag, kein Ersatz für Registrierung oder gültige Gruppenadressen.
- Capability-Anzeige, Schema / Version und Backend-Auswahl an wirkliche Unterstützung anpassen.
- Ungültige / unzulässige / unbekannte / nicht unterstützte Aufträge, Offline-Teilnehmer, Serialisierung / Sendefehler, Terminalablehnung und Timeout ausdrücklich mit Fehler oder ausstehendem Zustand zurückmelden.
- Wiederholungen, veraltete Revisionen, konkurrierende Add-/Remove-Aktionen, Reconnect, erneute Registrierung und Serving-TBS-Wechsel eindeutig ordnen. Aktuellen Sollzustand abgleichen; entzogene Rechte nicht durch alte Aufträge wiederherstellen; Wiederholungen begrenzen.

| Nachweisstufe | Was damit tatsächlich belegt ist |
| --- | --- |
| Core gespeichert / TBS angenommen | Auftrag und Ziel bekannt; ACK belegt keine Fachumsetzung |
| Lokal angewendet | Richtlinien / Mitgliedschaften übernommen; Ergebnis und Teilfehler sichtbar |
| Funkaktion eingereiht / ausgesendet | Queueing und echte Aussendung getrennt nachweisen; beides bestätigt allein keine Gerätewirkung |
| Am Endgerät bestätigt | Terminalantwort korrelieren, soweit möglich; andernfalls ausdrücklich unbestätigt führen und reale On-Air-Abnahme dokumentieren |

**Abnahme:** Core-Auftrag → Node Gateway → MM → lokaler Gruppenstand → Funkgerät → korrelierte Rückmeldung. Zuweisung und Entziehung, wirksames Gruppenruf-/SDS-Routing, bestehendes lokales DGNA sowie sämtliche negativen / Wiederkehrfälle prüfen. Core-Sollzustand, TBS-Istzustand und Endgerätewirkung getrennt festhalten. Gezielte Handler-/Vertragstests ersetzen keine reale Funkabnahme.

## 10. Erreichter Stand: Historie und heutiger Abgleich

### 10.1 Tatsächliche Dokumentationscommits dieses Chats

Die vier folgenden Commits wurden am 2026-10-06 erneut direkt über GitHub abgerufen. Dateilisten und Committexte bestätigen Dokumentationsänderungen; keine dieser Dateilisten enthält die fehlende MM-Runtime-Implementierung.

| Commit | Tatsächliche Änderung |
| --- | --- |
| [6d26ad87ef897fd684b55e88fb318efec291e6c7](https://github.com/JanHG98/netcore-tetra/commit/6d26ad87ef897fd684b55e88fb318efec291e6c7) | Neue Drive-Roadmap, IAM-Erweiterung, Backend- und Wiki-Verweise; Design / Dateicloud / Ordnerfreigaben dokumentiert |
| [e5d825b33db2e1fce14bcb2cc23e73c241873a18](https://github.com/JanHG98/netcore-tetra/commit/e5d825b33db2e1fce14bcb2cc23e73c241873a18) | Lokaler Drive-Betrieb ohne IAM, geprüfte spätere Migration und Browser-Plugins in Drive / IAM / Backend / Wiki ergänzt |
| [07609fb56f412ebe6e36655323e8fc6359cf90ec](https://github.com/JanHG98/netcore-tetra/commit/07609fb56f412ebe6e36655323e8fc6359cf90ec) | Zentrale ROADMAP.md und AGENTS.md erstellt; README / Backend / Wiki / IAM / Drive auf Masterroadmap verlinkt |
| [9116c15d645458f99e236712b67a1ad970432791](https://github.com/JanHG98/netcore-tetra/commit/9116c15d645458f99e236712b67a1ad970432791) | Z02.5 P0 und Gruppen-End-to-End-Abnahme in ROADMAP.md / system-backend/roadmap.md ergänzt |

Die letzte Änderung wurde im zugänglichen vorherigen Turn nach Ref-Update durch erneuten main-Read und exakten Inhaltsvergleich beider Dateien verifiziert. Die weiteren früheren Veröffentlichungen sind aus Fortsetzungsstand und jetzt erneut gelesenen Commits nachvollziehbar; deren damalige komplette Tooltranskription ist hier nicht vollständig vorhanden.

### 10.2 Heutige Prüfung am 2026-10-06

| Aussage / Bereich | Aktueller Befund | Nachweisgrenze |
| --- | --- | --- |
| Branches | main und Archiving vorhanden; gelesene Heads in Metadaten | Zeitpunktbezogene Branchliste; kein erfundener heute noch aktiver Featurebranch |
| Drive / IAM | Beide Fachroadmaps geplant; alle D0–D9 / M0–M8 offen, Design bestätigt, Backend / Betrieb offen | Planungsdateien vorhanden, kein Nachweis fertiger Anwendung / zentraler Auth |
| Gesamtroadmap | Master und AGENTS vorhanden auf main; Z01.1 erster Schritt, Z02.5 P0 offen / parallel möglich | Dokumentation umgesetzt; technische Aufgaben bleiben offen |
| Zentrale Gruppenhandler | Historischer Befund an 07609fb besteht an 9116c15 fort | Heute gezielt erneut Code gelesen; keine Runtime-Reparatur oder Live-Logbestätigung |
| Deployment / Syslog | Rekursiver main-Baum ohne system-backend/deployment-core und ohne Imagebuilder-/Syslog-/Discovery-/VPN-Policy-Dateien unter den gesuchten Namen | Unterstützt Z01-Übernahmelücke; kein neuer vollständiger inhaltlicher Vergleich mit historischem Feature-Tip |
| Dienstinventar | inventory.example.toml enthält 25 [[services]]; generierter Katalog enthält 24 Einträge | Heute Dateiinhalte gezählt; keine Aussage zur tatsächlich installierten Dienstzahl |
| UI / Dark Mode | PR #59 als gemergt in main bestätigt, Merge 7137e0dd69877e1b604bf89148fd8b6b590c1a97 | Kein neuer Hostrollout / Bedienungstest |
| Observability-PR | PR #57 gemergt, aber Ziel feature/openlab-discovery-deployment, Merge 8573b50a22287e62a30ad34beb2abd70b262d112 | Merge in Featurebranch belegt nicht Integration in heutigen main |
| Live-System | Keine aktuellen Anlagen-/Binary-/Konfigurations-/Gerätelogdaten für diese Archivprüfung | Keine Behauptung einer neuen aktuellen Störung, erfolgreichen Installation oder On-Air-Funktion |

Die weitere repositoryweite Beurteilung aus der früheren Gesamtroadmap-Sichtung bleibt als datierter Ausgangsbefund erhalten: zwei aktive Restore-Korrekturen, Routenguard, aktive SAP-/Downlink- und Mehrzellenintegration, lokale Control-Room-Rollen, zahlreiche schon vorhandene Fachbausteine und Open-Lab-/KMF-/Transit-Grenzen. Diese Archivierung ist **keine neue Vollprüfung des gesamten Funkstacks**. Vor technischer Umsetzung die relevanten Dateien am dann aktuellen Stand erneut prüfen.

## 11. Dateien, Dienste, Schnittstellen und Parameter

### 11.1 Fach- und Prüfdateien

Alle folgenden Links beziehen sich auf den geprüften main-Commit; sie bleiben damit vom älteren Archivbranch-Code unabhängig.

| Datei / Pfad | Rolle |
| --- | --- |
| [ROADMAP.md](https://github.com/JanHG98/netcore-tetra/blob/9116c15d645458f99e236712b67a1ad970432791/ROADMAP.md) | NETCORE-MASTER-01, Reihenfolge, Aufgaben und Statuspflege |
| [AGENTS.md](https://github.com/JanHG98/netcore-tetra/blob/9116c15d645458f99e236712b67a1ad970432791/AGENTS.md) | Fortsetzungshinweise für neue Arbeitschats |
| [Docs/NETCORE_DRIVE_ROADMAP.md](https://github.com/JanHG98/netcore-tetra/blob/9116c15d645458f99e236712b67a1ad970432791/Docs/NETCORE_DRIVE_ROADMAP.md) | Drive-Design, lokaler Betrieb, Freigaben, Plugins und D0–D9 |
| [Docs/CENTRAL_IDENTITY_RBAC_ROADMAP.md](https://github.com/JanHG98/netcore-tetra/blob/9116c15d645458f99e236712b67a1ad970432791/Docs/CENTRAL_IDENTITY_RBAC_ROADMAP.md) | IAM M0–M8, Ressourcenrechte / Ausfall / Migration |
| [system-backend/roadmap.md](https://github.com/JanHG98/netcore-tetra/blob/9116c15d645458f99e236712b67a1ad970432791/system-backend/roadmap.md) | Technische Phasenhistorie, Masterverweis, offene Z02.5-Ergänzung |
| [wiki/Roadmap.md](https://github.com/JanHG98/netcore-tetra/blob/9116c15d645458f99e236712b67a1ad970432791/wiki/Roadmap.md) | Weitere Einstiegspunkte |
| [group-core/src/state.rs](https://github.com/JanHG98/netcore-tetra/blob/9116c15d645458f99e236712b67a1ad970432791/system-backend/group-core/src/state.rs) | Zentrale Policy-/DGNA-Aufträge und Ergebnisse |
| [net_control/commands.rs](https://github.com/JanHG98/netcore-tetra/blob/9116c15d645458f99e236712b67a1ad970432791/crates/tetra-entities/src/net_control/commands.rs) | Typisierte interne Befehle / Antworten |
| [net_control_room/worker.rs](https://github.com/JanHG98/netcore-tetra/blob/9116c15d645458f99e236712b67a1ad970432791/crates/tetra-entities/src/net_control_room/worker.rs) | TBS-Routing / Korrelation / Dispatcher-ACK |
| [mm/mm_bs.rs](https://github.com/JanHG98/netcore-tetra/blob/9116c15d645458f99e236712b67a1ad970432791/crates/tetra-entities/src/mm/mm_bs.rs) | Fehlende zentrale Handler, lokales DGNA, Terminal-ACK |
| [net_control_room/protocol.rs](https://github.com/JanHG98/netcore-tetra/blob/9116c15d645458f99e236712b67a1ad970432791/crates/tetra-entities/src/net_control_room/protocol.rs) | Angekündigte Fähigkeiten |
| [group-core/docs/dgna.md](https://github.com/JanHG98/netcore-tetra/blob/9116c15d645458f99e236712b67a1ad970432791/system-backend/group-core/docs/dgna.md) | Dokumentierter zentraler DGNA-Vertrag, class_of_usage / force-Grenzen; aktiven MM-Anschluss gesondert prüfen |
| [inventory.example.toml](https://github.com/JanHG98/netcore-tetra/blob/9116c15d645458f99e236712b67a1ad970432791/deploy/open-lab/inventory.example.toml) / [service-catalog.json](https://github.com/JanHG98/netcore-tetra/blob/9116c15d645458f99e236712b67a1ad970432791/deploy/open-lab/generated/service-catalog.json) | 25/24-Drift und Beispielports |
| [tests/e2e/README.md](https://github.com/JanHG98/netcore-tetra/blob/9116c15d645458f99e236712b67a1ad970432791/tests/e2e/README.md) / [wiki/Abnahme.md](https://github.com/JanHG98/netcore-tetra/blob/9116c15d645458f99e236712b67a1ad970432791/wiki/Abnahme.md) | Vorbereitete Nachweisstufen / Labor- und On-Air-Abnahme |

Wichtige erneut belegte Blob-SHAs: ROADMAP `457ff1796cf94584e3f373495b11aed8aa54c897`; Drive `590a5ddf11a9346688bd8e08164a11f0b4c335a4`; IAM `81cda0df10bb21b266be6fa396201709ee3b78c1`; MM `17b290cbeb097b255ff78b5e8b8fe2f4537cd37f`; TBS-Worker `2f700bfff24e6469488d4a626ef8381f080598e0`; Group Core `a315b56e0cd453e31ddf31148258502642354512`; Capability-Protokoll `5572e1cee34ef73cf20077b6eaaf872a101f2147`.

### 11.2 Beispielports und Protokolle

| Dienst | Port im gelesenen generierten Open-Lab-Katalog | Beispielunit |
| --- | --- | --- |
| Node Gateway | 8080 | netcore-node-gateway.service |
| Group Core | 8110 | netcore-group-core.service |
| Recorder | 8140 | netcore-recorder.service |
| Packet Core / IP Gateway | 8160 / 8170 | netcore-packet-core.service / netcore-ip-gateway.service |
| Media Library | 8230 | netcore-media-library.service |
| IoT / Hardware Gateway / RF Monitor | 8240 / 8250 / 8260 | netcore-iot-gateway.service / netcore-hardware-gateway.service / netcore-rf-monitor.service |
| Control Room | 9010 | netcore-control-room.service |

Das sind Repository-Beispiele mit `mode=open_lab`, keine heute bestätigten Live-Binds und keine Drive-Portauswahl. Für Drive und Identity gibt es aus diesem Chat noch keine verbindlichen Ports, Hostnamen, LXC-IDs, Mountpfade oder TOML-Schlüssel. Geplante Protokolle sind HTTPS und OIDC für Web-/Identitätswege, gegebenenfalls WebDAV / Sync je Backendwahl; TBS-Gruppenaufträge verwenden den vorhandenen NetCore-Vertrag und MM-Funksignalisierung.

## 12. Befehle, Speicherung, Tests und ihre Grenzen

### 12.1 Tatsächlich ausgeführte Arbeiten

Historisch wurden die vier Dokumentationsänderungen per GitHub-Datei-/Git-Objekt-Werkzeugen erstellt, auf main veröffentlicht und geprüft. Der zugängliche letzte Turn zeigt: frischen Head laden, auf dessen Tree aufsetzen, nur ROADMAP.md und system-backend/roadmap.md ändern, Commit-Dateiliste kontrollieren, main ohne Force vorziehen und beide Inhalte erneut exakt vergleichen. Es wurden dabei keine Funk-Runtime-Dateien verändert.

Bei dieser Archivierung wurden Branch-Refs, Commitmetadaten, rekursive Trees, Archivindex, relevante Fachroadmaps / Codepfade, Inventar / Katalog und PR #57 / #59 lesend abgerufen. PDF-Cover / Metadaten sowie wenige relevante MM-Fundstellen wurden geprüft. Eine gezielte Bildsuche lieferte keine diesem Chat sicher zuzuordnenden Originalbilder; unpassende Treffer wurden nicht als Chatbilder übernommen.

Für die Veröffentlichung gilt: aktuelle Archiving-Ref erneut lesen; vorhandenen Index erhalten; nur diese Archivdatei und ihren Indexeintrag auf dem aktuellen Tree erstellen; geänderte Pfade prüfen; Commit mit genau diesem Parent erzeugen; ohne Force und mit erwarteter Ausgangs-Ref aktualisieren; Zusammenfassung / Index danach am Zielbranch lesen. Bei zwischenzeitlicher Ref-Änderung neu basieren und fremde Einträge erhalten. Der tatsächliche Archivcommit steht in der Git-Historie und Abschlussmeldung; keine Commitnummer vorab erfunden.

### 12.2 Reproduzierbare Lese-/Vergleichsbefehle, nicht hier als Shelllauf behauptet

Die folgenden Befehle sind **Vorschläge** für einen vorhandenen vollständigen lokalen Checkout. Sie wurden in dieser Archivierung nicht als Git-CLI-Lauf ausgeführt:

```bash
git fetch origin main Archiving
git rev-parse origin/main origin/Archiving
git show origin/main:ROADMAP.md
git show origin/main:Docs/NETCORE_DRIVE_ROADMAP.md
git show origin/main:Docs/CENTRAL_IDENTITY_RBAC_ROADMAP.md
git show origin/Archiving:Docs/archive/README.md
git diff --name-status e5d825b33db2e1fce14bcb2cc23e73c241873a18 bbf039729b9b05f8d623b11195ca24a124f68d16
git show origin/main:crates/tetra-entities/src/mm/mm_bs.rs
```

Der letzte Diff vergleicht die explizit angegebenen historischen Tip-Commits direkt; bei Z01.1 den linken Commit durch den dann aktuellen main-Tip ersetzen und betroffene Dateiinhalte ergänzend vergleichen. Historischen Featurecommit gegebenenfalls erst gezielt verfügbar machen. Keine unbekannten Installationskommandos oder bereits erfolgte Produktinstallation aus diesen Vorschlägen ableiten.

### 12.3 Nachweise und fehlende Tests

| Prüfung | Ergebnis / Grenze |
| --- | --- |
| Existenz der vier historischen Commits | Direkt abgerufen; konkrete IDs / Dateilisten bestätigt |
| Historischer Feature-CI-Lauf 36349402097 | Heute Metadaten erneut geprüft: success am 2026-09-27 auf bbf0397; keine neue Ausführung, kein vollständiger Image-/Pi-/Anlagenbeleg |
| Letzter Gruppen-Roadmap-Commit | Historisch nur zwei Dokumente; damaliger exakter Remotevergleich belegt |
| Aktuelle Gruppen-Codeprüfung | Lücke fortbestehend; kein ausgeführter Rust-Test / Gerätestest |
| Inventory / Katalog | 25 / 24 gezählt; kein Deploy- oder Flottencheck |
| PDF-Anhänge | 25 Dateien inventarisiert, zielgerichtete Fundstellen; keine vollständige Norm- oder Konformitätsprüfung |
| Drive / IAM / Plugins | Keine Installation, Build-, Login-, Rechte-, Sync-, Format-, Export- oder Restore-Abnahme in diesem Chat nachgewiesen |
| Funkbetrieb | Keine neue TBS-/Pi-/Endgeräte-/On-Air-Abnahme, kein Dauerlauf oder Mehrzellentest |

Aufwändige Runtime-Testläufe wären für den reinen Archivcommit kein Beleg der geplanten Features. Der angemessene Archivcheck ist Vollständigkeit / Quellen / Links / Änderungsumfang / fremde Indexeinträge / Secrets-Vermeidung und exakte Remote-Präsenz.

## 13. Ersetzte Ansätze und wichtige Abgrenzungen

| Frühere Annahme / mögliche Fehlinterpretation | Gültiger Endstand und Grund |
| --- | --- |
| Kleine NAS-/SQLite-Cloud als endgültige Wahl | Mit erweitertem Nextcloud-Dateiumfang neu bewerten; Nextcloud Files / PostgreSQL ist bevorzugter Prüfvorschlag, Eigenbau bleibt Alternative |
| Alles wartet auf zentrales RBAC | D0–D5 und D7–D9 funktionieren geplant lokal; IAM-D6 später separat, damit Dateien / Gäste / Plugins nicht blockiert werden |
| Rückfallebene heißt automatische lokale Anmeldung bei IdP-Ausfall | Eigenständiger Anfangsbetrieb und begrenzter späterer Notzugang getrennt; kein automatischer regulärer lokaler Fallback nach Migration |
| Dienstrolle oder Link mit E-Mail ergibt automatisch Datei-/Personenrecht | Ressourcen-ACL verbindlich; bestätigte Gastidentität erforderlich für personenbezogene Freigabe |
| Browser kann jedes Format komplett bearbeiten | Plugin-/Formatmatrix mit expliziten Viewer-/Editor-/Export- und Codecgrenzen; einzelne Integrationen abnehmen |
| PR #57 gemergt, daher Deployment / Syslog in main | PR wurde in historischen Featurebranch gemergt; heutiger Hauptzweig enthält die fehlenden Bausteine nicht unter den geprüften Pfaden |
| Neues Design zuerst nochmals bauen | PR #59 ist bereits in main; vorhandene UI ausrollen / abnehmen, keine pauschale neue Designrunde als erster Schritt |
| Gruppen-Worker-Routing oder ACK bedeutet umgesetzt | Zentrale MM-Handler fehlen; Annahme, lokaler Zustand, Aussendung und Terminalwirkung getrennt |
| Lokales DGNA beweist vollständige zentrale Policy-/SS-DGNA-Unterstützung | Konkrete zentrale Handler / Reconcile / Ergebnisse und passende Norm-/Endgeräteabnahme fehlen |
| Archivauftrag erlaubt neue Umsetzung im Hauptzweig | Jetzt ausschließlich Docs/archive auf Archiving; Roadmap-Kandidaten nur in dieser Zusammenfassung erfassen |

## 14. Relevante Anhänge und Bilder

### 14.1 ETSI-PDF-Inventar

Alle 25 bereitgestellten PDF-Dateien waren im aktuellen Workspace vorhanden und wurden lesend nach Metadaten / Cover inventarisiert. Die folgende Tabelle verwendet die **hochgeladenen Dateinamen**, nicht die lokalen numerischen Arbeitspräfixe. Die PDFs werden durch diesen Archivauftrag nicht nochmals in Git kopiert. Sie sind weder Designmockups noch eigenständige Chatbilder.

| Hochgeladene Datei | Thema / Norm | Edition / Coverdatum | Seiten / Status |
| --- | --- | --- | --- |
| en_3003920308v010401p.pdf | EN 300 392-3-8, ISI Generic Speech Format | V1.4.1, 2020-04 | 22, EN |
| en_30039209v010701p.pdf | EN 300 392-9, allgemeine Zusatzdienste | V1.7.1, 2020-04 | 46, EN |
| ts_10081201v020205p.pdf | TS 100 812-1, SIM-ME / UICC | V2.2.5, 2003-10 | 8, TS |
| en_3003921201v010202p.pdf | EN 300 392-12-1, Call Identification Stage 3 | V1.2.2, 2007-08 | 56, EN |
| en_3003920304v010301p.pdf | EN 300 392-3-4, ISI SDS | V1.3.1, 2010-08 | 28, EN |
| en_3003921117v010102p.pdf | EN 300 392-11-17, Include Call Stage 2 | V1.1.2, 2002-01 | 18, EN |
| en_3003921114v010101p.pdf | EN 300 392-11-14, Late Entry Stage 2 | V1.1.1, 2002-07 | 23, EN |
| es_20081202v020401m.pdf | ES 200 812-2, TSIM-ME / UICC-TSIM-Anwendung | V2.4.1, 2005-08 | 139, Final draft |
| es_20081201v020205p.pdf | ES 200 812-1, UICC-Eigenschaften | V2.2.5, 2003-12 | 8, ES |
| en_300812v020101p.pdf | EN 300 812, Security / SIM-ME | V2.1.1, 2001-12 | 156, EN |
| en_3003921101v010201p.pdf | EN 300 392-11-1, Call Identification Stage 2 | V1.2.1, 2004-01 | 44, EN |
| en_3003921006v010401p.pdf | EN 300 392-10-6, Call Authorized by Dispatcher Stage 1 | V1.4.1, 2006-08 | 20, EN |
| en_3003921018v010301p.pdf | EN 300 392-10-18, Barring of Outgoing Calls Stage 1 | V1.3.1, 2003-10 | 17, EN |
| en_3003921216v010400a.pdf | EN 300 392-12-16, Pre-emptive Priority Call Stage 3 | V1.4.0, 2026-03 | 67, DRAFT |
| en_30039201v010601p.pdf | EN 300 392-1, General network design | V1.6.1, 2020-04 | 182, EN |
| ets_30039214e01v.pdf | pr ETS 300 392-14, PICS | Final draft, 1997-09 | 61, FINAL DRAFT |
| en_30039207v030501p.pdf | EN 300 392-7, Security | V3.5.1, 2019-07 | 216, EN |
| en_30039401v030301p.pdf | EN 300 394-1, Radio conformance testing | V3.3.1, 2015-04 | 169, EN |
| en_3003920313v010201p.pdf | EN 300 392-3-13, Transport-independent ISI Group Call | V1.2.1, 2020-04 | 191, EN |
| en_30039502v010303p.pdf | EN 300 395-2, TETRA speech codec | V1.3.3, 2025-02 | 94, EN |
| en_3003920303v010301p.pdf | EN 300 392-3-3, ISI Group Call | V1.3.1, 2011-11 | 251, EN |
| en_30039205v020701p.pdf | EN 300 392-5, PEI | V2.7.1, 2020-04 | 320, EN |
| en_3003920315v010500a.pdf | EN 300 392-3-15, Transport-independent ISI Mobility | V1.5.0, 2026-04 | 380, Draft |
| en_30039202v030801p.pdf | EN 300 392-2, Air Interface | V3.8.1, 2016-08 | 1445, EN |
| ETSI.pdf | Sammlung mit 25 Dokumentblöcken, keine einzelne Norm | Gemischte Editionen, 1997–2026 | 4100, Sammlung |

Die 24 Einzel-PDFs haben zusammen 3961 Seiten. Die Sammlung enthält zusätzlich TS 100 812-2 V2.4.1 (2005-08), 139 Seiten, Cover auf Sammel-PDF-Seite 312. Sie beginnt mit EN 300 812 V2.1.1. Die strukturelle Zuordnung ist kein bytegenauer Duplikatvergleich; nicht als neue einheitliche „ETSI-Ausgabe“ oder bloße 139-Seiten-Doppelung ausgeben.

### 14.2 Bezug zur Gruppenabnahme

Die vorhandene Air-Interface-Ausgabe EN 300 392-2 V3.8.1 enthält gezielt geprüfte Referenzen: §16.8.1, S.369, infrastrukturseitiges Attach / Detach; §16.9.2.1 / Tabelle 16.1, S.375, D-ATTACH/DETACH GROUP IDENTITY; §16.9.2.2 / Tabelle 16.2, S.376, Downlink-ACK; §16.9.3.1 / Tabelle 16.15, S.382, U-ATTACH/DETACH; §16.9.3.2 / Tabelle 16.16, S.383, Uplink-ACK; §16.10.12–16, S.397–398, Accept/Reject, ACK-Anforderung / -Typ, Adresstyp und Lifetime; §16.10.28 / Tabelle 16.60, S.402, GSSI.

Diese Stellen eignen sich für späteren gezielten PDU-/ACK-/Lifetime-/Ablehnungsabgleich; sie beweisen weder aktuellen Funkbetrieb noch vollständiges SS-DGNA. **EN 300 392-12-22, die Stage-3-Spezifikation SS-DGNA, ist in den bereitgestellten Einzeldateien und identifizierten Sammlungsteilen nicht enthalten.** Die PPC-Datei 12-16 nennt DGNA lediglich als anderen Unterteil in der Übersicht; sie ist keine DGNA-Spezifikation und ausdrücklich Draft. Für eine vollständige SS-DGNA-Abnahme die benötigte Normbasis gesondert beschaffen / versionieren.

### 14.3 Chatbilder und fehlende Originale

Im derzeit sichtbaren Verlauf sind keine eigenständigen Bilder eingebettet. Die frühen Designantworten werden nur durch Fortsetzungsstand, Kontextabruf und gespeicherte Fachroadmap erschlossen. Die gezielte Suche nach Core Console / Archive Studio / NetCore Drive und passenden Zeiträumen identifizierte kein sicher zugehöriges Originalbild. Zeitnah erstellte generische Bildtreffer gehörten bei visueller Kontrolle zu einem anderen Thema und wurden ausdrücklich nicht als NetCore-Chatbilder übernommen.

Deshalb **keine Originalbilder dieses Chats hochgeladen**. Es wurden keine Ersatzmockups oder nachgezeichneten Bilder erzeugt und als historische Originale ausgegeben. Falls originale Designbilder oder ein vollständiger Chat-Export später verfügbar werden, diese unter Docs/archive zu genau diesem Archiv ergänzen und Herkunft / unveränderte Prüfsumme dokumentieren. Die 25 PDF-Anhänge ersetzen diese Bildlücke nicht.

## 15. Offene Aufgaben und konkrete Fortsetzung

| Priorität / Referenz | Noch nötige Arbeit | Abhängigkeit / Erfolgskriterium |
| --- | --- | --- |
| Erster Gesamtschritt Z01.1 | Direkter Tip-/Inhaltsvergleich und Integrationsplan für historischen Deployment-/Syslog-Stand | Keine neue Liveanlage nötig für Quellvergleich; Tabelle / Plan mit SHAs, Konflikten, Tests und Rückweg |
| P0 Z02.5 parallel | Zentrale Policy-/DGNA-Handler und Ergebnisführung schließen | Aktueller Gruppenvertrag; lokale DGNA-Regression und reale Attach-/Detach-/Endgeräteabnahme |
| P0 Z02.1–Z02.4 | Restore, Routenguard und aktive Runtimepfade | Gezielte positive / negative Nachweise; keine falsche Erfolgsanzeige |
| Frühe IAM-Arbeit Z04 / M0–M1 | Endpunkt-/Rollen-/Ressourcen-/Fristeninventur und ADR | Laufende Funkvermittlung unabhängig lassen; lokale Benutzer und Maschinen sauber migrieren |
| Drive D0 / Z06 | Releaseumfang, Backend / UI / NAS / Metadaten, lokaler Login, Gäste, Sync und Plugin-Vertrag auswählen | Eigenständiger abgesicherter Pilot ohne zentralen IAM-Dienst |
| D1–D5 | Dateikern, drei Ansichten, Ordnerfreigaben, Versionen / Papierkorb und Clients abnehmen | Rechte einschließlich direkter APIs, Vererbung, Widerruf, Konflikten, Neustart / Restore |
| D7, danach D8 / D9 | Plugin-Lifecycle und Basismodule, dann Office / technische Formate | Viewer-/Editor-/Exportumfang pro Format, Quellen / Isolation / Originalerhalt / Update / Rückweg; kein D6-Gate |
| Später D6 | Zentraler Login / Identitätsmigration | IAM-Pilot / OIDC-Adapter, bestätigte issuer+subject-Zuordnung und Verlust-/Rechteerhalt / Ausfallabnahme |
| Z03 / Z05 / Z07 und spätere Stränge | Fach-E2E und echte Funk-/Mehrzellen-/Hardwareabnahme | Masterreihenfolge und Eintrittskriterien verwenden, vorhandene Bausteine nicht pauschal neu bauen |
| Quellenlücke | Frühe vollständige Assistant-Antworten, Chattitel / Link und gegebenenfalls Original-Designbilder nachsichern | Dieses Archiv ergänzen; keine fremden Chatbilder oder erfundenen Designparameter übernehmen |

Bei einer neuen Frage nach „Was als Erstes?“ am dann aktuellen main die aktuelle Masterroadmap lesen und den tatsächlichen offenen Status prüfen. Am heute belegten Stand ist die Antwort **Z01.1**, mit **Z02.5 als unabhängig möglicher P0-Arbeit**. Eine neue ausdrückliche Nutzerentscheidung kann die Auswahl innerhalb des Auftrags ändern.

## 16. Weitere Quellen und Änderungsgrenzen

- [Historischer Feature-Tip bbf039729b9b05f8d623b11195ca24a124f68d16](https://github.com/JanHG98/netcore-tetra/tree/bbf039729b9b05f8d623b11195ca24a124f68d16): Ausgangsquelle für Z01, kein heute noch aktiver Branch behauptet.
- [PR #57](https://github.com/JanHG98/netcore-tetra/pull/57): Observability / Discovery / Syslog, Feature-Zielbranch wie oben getrennt.
- [PR #59](https://github.com/JanHG98/netcore-tetra/pull/59): NetCore-Design / Dark Mode, in main integriert.
- [Deployment-/Image-/Discovery-Archiv](2026-10-05_deployment-vm-tbs-imagebuilder-und-auto-discovery.md): historische Entwicklung / Z01-Quellen.
- [Roadmap-UI-/RBAC-/Restore-Archiv](2026-10-06_roadmap-ui-dark-mode-zentrale-rbac-und-cmce-restore.md): angrenzende frühere Fachroadmap, nicht derselbe Chat.
- [Media-Library-/TTS-/IP-Gateway-Archiv](2026-10-05_media-library-tts-archivierung-basisstations-playout-und-ip-gateway-routing.md): historische Betriebs-/Routingbefunde mit eigenen Nachweisgrenzen.

Dieser Auftrag ergänzt ausschließlich die eigene Datei und den Archivindex unter Docs/archive auf Archiving. Main, bestehende Fachroadmaps, AGENTS, Runtime, Konfiguration, Anlagen und andere Archivdateien werden durch die Archivierung nicht geändert. Keine Passwörter, Tokens, privaten Schlüssel oder sonstigen Zugangsdaten übernommen. Der Nutzer archiviert den Chat nach eigener Prüfung selbst.
