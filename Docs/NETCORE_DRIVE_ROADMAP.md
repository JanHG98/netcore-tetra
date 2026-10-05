# Roadmap: NetCore Drive – Dateicloud und externe Ordnerfreigaben

| Feld | Wert |
| --- | --- |
| Roadmap-ID | NETCORE-DRIVE-01 |
| Erstellt / aktualisiert | 2026-10-05, Europe/Berlin |
| Gesamtstatus | **Geplant; Designrichtung bestätigt, Backend- und Betriebsentscheidung offen** |
| Produktziel | Dateicloud mit Nextcloud-ähnlichem Funktionsumfang und der einfachen Bedienung von OneDrive |
| Bestätigte Designrichtung | Core Console als Hauptoberfläche, Workspace als Startansicht, Archive Studio als Dokumentmodus; eigenes NetCore-Design |
| Datenhaltung | Dateien auf NAS / Dateispeicher; Verwaltungsdaten getrennt; MariaDB ist keine Voraussetzung |
| Anmeldung / RBAC | Eigenständiger Start mit lokalen Konten, Gruppen und Ressourcenrechten; spätere Anbindung an [NETCORE-IAM-01](CENTRAL_IDENTITY_RBAC_ROADMAP.md) in D6; zentraler IAM-Dienst ist keine Startvoraussetzung |
| Browser-Funktionen | Erweiterbare Plugin-Plattform für PDF, Office, 3D, Schaltpläne, Vektorgrafiken, Bilder, Video und Audio; Viewer- / Editorumfang je Format abnehmen |
| Architekturvorschlag | Eigene NetCore-Oberfläche vor Nextcloud Files mit PostgreSQL; vollständiger Eigenbau als zu bewertende Alternative |
| Nächster Schritt | Datei-Funktionsumfang, Backend, lokalen Betriebsmodus, Gastanmeldung, Plugin-Vertrag und spätere Identitätsmigration in einer Architekturentscheidung festlegen |
| Zeitplanung | Meilensteinfolge ohne verbindliche Termine; Prioritäten sind Vorschläge |

## 1. Bestätigtes Ziel und offene Architektur

NetCore Drive soll eine eigene Dateicloud im NetCore-Ökosystem werden. Die im Gespräch angenommenen Entwürfe bilden die Designrichtung: eine kompakte Dateiliste mit Details in Core Console, ein ruhiger Einstieg mit angehefteten Ordnern und zuletzt verwendeten Dateien im Workspace sowie eine Bibliotheksansicht mit Dokumentvorschau in Archive Studio. Die Ansichten sollen innerhalb einer Anwendung dieselben Dateien und Rechte verwenden und auf Mobilgeräten sinnvoll bedienbar bleiben.

Die Leitlinie lautet **„Funktionsumfang Nextcloud, Simplizität OneDrive“**. Für die Umsetzung wird daraus zunächst ein überprüfbarer Dateicloud-Umfang abgeleitet. Vollständige Funktionsparität mit sämtlichen Nextcloud-Hub-Apps ist damit noch keine Zusage für ein erstes Release. Die Entwürfe sind keine implementierte Cloud; dieser Roadmap-Auftrag installiert keine Software und verändert keinen bestehenden Funkdienst.

Für einen kleinen eigenen Dateidienst wurde zunächst ein einzelner LXC mit NAS-Dateien und einer lokal gespeicherten SQLite-Datenbank vorgeschlagen. Mit dem erweiterten Funktionsziel ist **Nextcloud Files mit PostgreSQL als bewährte Datei-/Synchronisationsbasis hinter unserer Oberfläche** die bevorzugte Empfehlung zur Prüfung. SQLite bleibt eine mögliche Option für einen begrenzten Eigenbau, ist aber keine festgelegte Produktionsarchitektur. Eine SQLite-Datei gehört in dieser Variante auf lokalen persistenten Speicher, nicht auf den NFS-/SMB-Dateimount.

Die endgültige Entscheidung bleibt offen: Backend, unterstützte Version, eigener UI-Integrationsweg, Client-Kompatibilität und Migrationsaufwand sind vor der Implementierung zu belegen. Auch eine vollständige Eigenentwicklung ist möglich; dann gehören Synchronisation, Konfliktauflösung, konsistente Versionierung und Rechteprüfung zum eigenen Entwicklungsumfang.

## 2. Geplanter Dateicloud-Funktionsumfang

| Bereich | Zielverhalten | Abnahme / Abhängigkeit |
| --- | --- | --- |
| Dateiverwaltung | Dateien und Ordner erstellen, hochladen, herunterladen, verschieben, umbenennen und löschen; Listen- und Bibliotheksansicht | Rechte und stabile Objektidentität bleiben nachvollziehbar; direkte API-Aufrufe entsprechen der Oberfläche |
| Große Uploads | Upload in Teilen, Fortschritt, Wiederaufnahme nach Unterbrechung und nachvollziehbare Fehler | Unterbrochene Uploads fortsetzen; fehlerhafte Teilstände nicht als vollständige Datei veröffentlichen |
| Vorschau und Suche | Dokument-/Medienvorschau, Favoriten, zuletzt verwendet und Suche; spezialisierte Browser-Viewer über Plugins | Treffer, Vorschauen und Vorschaudateien nur für berechtigte Personen; unterstützte Formate und Grenzen pro Plugin dokumentieren |
| Versionen und Papierkorb | Frühere Dateiversionen wiederherstellen; versehentlich gelöschte Inhalte zurückholen | Rechte, Aufbewahrung, Speicherbedarf und Restore-Verhalten festlegen und prüfen |
| Freigaben | Einzeldateien und ganze Ordner intern oder extern freigeben; Zugang verwalten | Personen, Gruppen und Links unterscheiden; Vererbung, Ablauf und Widerruf prüfen |
| Synchronisation | Desktop-/Mobilzugriff, Offline-Nutzung und nachvollziehbare Konfliktbehandlung | Backend und Clients auswählen; echte Client-Tests für Änderungen, Konflikte und Rechteentzug |
| Zusammenarbeit | Kommentare sowie Browser-Bearbeitung geeigneter Office-Dokumente; gemeinsames Bearbeiten als gesondert abgenommene Ausbaustufe | Office- / Editor-Plugin, Anmeldung, Versionen und Konfliktbehandlung festlegen; folgt nicht automatisch aus einer neuen Oberfläche |
| Plugins | Zusätzliche Viewer, Player, Konverter und geeignete Editoren installierbar; einheitliche Browser-Bedienung | Erweiterungsvertrag, Verwaltung, Isolation, Rechte und Formatmatrix aus Abschnitt 5; im lokalen und später zentralen Betrieb nutzbar |
| Betrieb | Sicherung, Wiederherstellung, Quotas, Aufbewahrung, Monitoring und nachvollziehbare Verwaltungsereignisse | Dateiinhalt, Metadaten, Freigaben und Versionen gemeinsam wiederherstellen |

Welche dieser Ziele das erste Release umfasst und welche als Ausbau folgen, wird in D0 entschieden. Bestätigte Designrichtung und Produktziel werden dadurch nicht als bereits entwickelte Funktion ausgegeben.

## 3. Gemeinsame Anmeldung und Ressourcenrechte

Drive kann zunächst ohne zentralen IAM-Dienst mit eigener Benutzer- und Rechteverwaltung starten. Die spätere Anbindung an [NETCORE-IAM-01](CENTRAL_IDENTITY_RBAC_ROADMAP.md) ermöglicht über zentrale Identitäten, Gruppen und Dienstrollen einen gemeinsamen Login zwischen den angebundenen NetCore-Oberflächen. Die bestehende IAM-Planung lässt lokale Konten im Identity-Dienst und eine optionale AD-Anbindung zu; diese sind vom eigenständigen lokalen Drive-Betrieb zu unterscheiden. Produktwahl und Betrieb des Identity-Dienstes bleiben deren Architekturentscheidung.

Die Zuständigkeiten sind verbindlich zu trennen:

- **Identitäts- und Rollenverwaltung:** Zunächst lokal in Drive, später über IAM: Wer ist angemeldet, welchen Dienst darf die Identität nutzen und welche Verwaltungsaktionen darf sie ausführen?
- **Drive-Backend:** Auf welche konkreten Dateien / Ordner darf die Identität zugreifen und mit welchen Einzelrechten?
- **Gemeinsame NetCore-Verwaltung:** Benutzer, Gruppen, Dienstrollen, Freigaben und deren Herkunft verständlich anzeigen; die fachliche Berechtigungsautorität bleibt eindeutig.

Ein Basisstations-Techniker erhält durch seine Dienstrolle keinen pauschalen Zugriff auf private Drive-Dateien. Eine externe Gastidentität erhält durch ihre Ordnerfreigabe keine internen NetCore-Dienstrollen. Ein bestehendes Dateibackend darf nicht durch ein zweites, widersprüchliches ACL-System in der Oberfläche umgangen werden.

Für automatische Ablagen aus Recording, TBS oder Discovery werden eigene Dienstidentitäten mit begrenzten Aufgaben und Zielordnern vorgesehen. Menschliche Konten, Gastidentitäten, Maschinenidentitäten und TETRA-Teilnehmer bleiben getrennte Klassen. Die laufende Funkvermittlung ist unabhängig von Drive und von einer erreichbaren Web-Anmeldung.

### 3.1 Rückfallebene: eigenständiger Start ohne zentrales RBAC

Solange der zentrale IAM-Dienst noch fehlt, erhält Drive einen **vollwertigen lokalen Betriebsmodus**. D0–D5 sind dadurch unabhängig vom Abschluss der IAM-Meilensteine. Fehlendes zentrales RBAC bedeutet keine fehlende Zugriffskontrolle: Jede Dateioperation bleibt durch das Drive-Backend autorisiert. Die konkrete Konfiguration und technische Umsetzung hängen vom in D0 gewählten Backend ab.

| Betriebsfall | Geplante Anmeldung und Rechte | Verhalten |
| --- | --- | --- |
| Lokaler Start ohne zentralen IAM-Dienst | Persistente lokale Benutzer, Gruppen, Verwaltungsrollen und getrennte Dienstkonten; Datei- / Ordnerrechte im Drive-Backend | Explizit gewählter Betriebsmodus; Konten anlegen / sperren, Gruppen zuweisen, Eigentum und Freigaben verwalten; Neustart und Restore erhalten Konten und Rechte |
| Spätere zentrale Anbindung | Zentraler Login über den vereinbarten OIDC-Vertrag; geprüfte Rollen- / Gruppenzuordnung; dieselbe fachliche Ressourcenprüfung | Kontrollierter Wechsel in D6; reguläre lokale Passwortanmeldung der migrierten Konten anschließend deaktivieren; keine widersprüchliche parallele Benutzerverwaltung |
| Zentraler Login bereits eingerichtet, IAM-Dienst fällt aus | Keine neuen zentralen Anmeldungen oder Token-Erneuerungen; bestehende Zugänge nur innerhalb der festgelegten Gültigkeits- und Sperrfristen | Kein automatischer Wechsel auf lokale Anmeldung und keine Erweiterung von Rechten; ein separat eingerichteter, begrenzter und protokollierter lokaler Notfallzugang folgt der IAM-Ausfallregel |

**Freigaben ohne IAM:** Interne lokale Benutzer / Gruppen, bestätigte externe Gastidentitäten, Linkfreigaben und Uploadbriefkästen verwenden dieselben Rechte, Vererbung, Ablauf- und Widerrufsregeln aus Abschnitt 4. Externe Gäste benötigen weder zentralen Login noch ein internes AD-Konto. Personenprüfung und Mailzustellung bleiben beim gewählten Gastverfahren erforderlich; fehlendes IAM ersetzt diese Prüfung nicht. Regulärer lokaler Login verwendet die abgesicherte Authentifizierung des gewählten Backends; Zugangsdaten gehören nicht in Git oder reguläre TOML-Konfigurationen. Erstes Administratorkonto, Sperren, Wiederherstellung und Protokollierung werden in D0 festgelegt.

**Späterer Umstieg ohne Verlust von Freigaben:** Eigentum, Gruppenrechte und Freigaben referenzieren stabile interne Drive-Identitäts- und Objekt-IDs. Zentralidentitäten werden nach bestätigter Zuordnung über das Paar `issuer` / `subject` mit bestehenden Konten verbunden; gleiche E-Mail-Adressen oder Namen reichen nicht für eine automatische Verknüpfung. Gruppen und Dienstrollen werden ausdrücklich abgeglichen. Dateien, Versionen, Papierkorb, Gäste und bestehende Freigaben bleiben erhalten; die Migration darf keine zusätzlichen Rechte erzeugen. Sicherung, probeweise Migration und geprüfter Rückweg gehören zur Abnahme von D6.

## 4. Externe Freigabe eines ganzen Ordners

### Bedienablauf

1. Ordner auswählen und **Freigeben** öffnen.
2. Person / Gruppe eingeben oder **Jeder mit dem Link** auswählen.
3. **Ansehen**, **Bearbeiten** oder **Nur hochladen** wählen.
4. Optional Ablaufdatum setzen; weitere Optionen bei Bedarf öffnen.
5. Einladung erstellen oder Link kopieren.

Unter **Zugriff verwalten** werden einzelne Personen-, Gruppen- und Linkfreigaben einschließlich ihrer Herkunft und Laufzeit angezeigt. Freigaben können geändert oder einzeln beendet werden. Komplexere Einstellungen stehen unter **Weitere Optionen**, damit die Standardbedienung einfach bleibt.

### Empfängerkreis und Identitätsprüfung

| Freigabeart | Geplantes Verhalten | Grenze |
| --- | --- | --- |
| Bestimmte Personen | Standard für personenbezogene externe Freigaben; Zugriff nach bestätigter Gastidentität ausschließlich auf den freigegebenen Inhalt | Ein weitergereichter Link allein berechtigt keine andere Person; bloße Eingabe einer E-Mail-Adresse ist keine Identitätsprüfung |
| Interne Personen / Gruppen | Zunächst lokale Drive-Konten und Gruppen; nach D6 geprüfte Zuordnung zentraler Identitäten und Gruppen | Dienstzugang ersetzt keine konkrete Datei- / Ordnerfreigabe |
| Jeder mit dem Link | Langer zufälliger Freigabetoken; optional Passwort und Ablaufdatum | Linkbesitz vermittelt Zugriff und kann weitergegeben werden; keine sichere Zuordnung zu einer benannten Person |

Gäste benötigen kein Konto im internen AD. E-Mail-Codes sind der vorgeschlagene komfortable Gastzugang; das tatsächliche Verfahren, Mailzustellung, Sitzungsregeln und Integration mit dem gewählten Backend müssen gesondert entwickelt oder ausgewählt werden. OIDC allein liefert diese Gastfunktion nicht automatisch. Externe Browser benötigen einen erreichbaren HTTPS-Endpunkt; die Veröffentlichung der Freigabeoberfläche ist eine eigene Betriebsentscheidung, keine Freigabe von NAS- oder Verwaltungsoberflächen.

### Einfache Rechteauswahl

| Auswahl | Geplante Rechte |
| --- | --- |
| Ansehen | Anzeigen und herunterladen |
| Bearbeiten | Zusätzlich hochladen, ändern, umbenennen und löschen; Löschen bei Bedarf in weiteren Optionen separat ausschalten |
| Nur hochladen | Dateien in einen Uploadbriefkasten abgeben, ohne vorhandene Dateien zu sehen |

Weiterfreigeben / Freigaben verwalten wird als eigenes Recht behandelt und extern nicht automatisch vergeben. Die Abbildung dieser Auswahl auf das gewählte Backend ist Bestandteil der Architektur- und Abnahmearbeit.

### Vererbung, Verschieben und Widerruf

- Die Freigabe umfasst Unterordner und später hinzugefügte Inhalte; dieser Umfang wird im Dialog deutlich angezeigt.
- Eltern- und Nachbarordner bleiben über diese Freigabe verborgen. Der Gast darf durch Pfadwechsel oder direkte Datei-IDs nicht aus dem freigegebenen Bereich ausbrechen.
- Freigaben und ACLs werden an stabile Objekt- und Identitäts-IDs gebunden. Umbenennen darf nicht allein durch einen veränderten Pfad Rechte verlieren oder neu erteilen; das konkrete Backend-Verhalten wird geprüft.
- Beim Verschieben werden geerbte Rechte anhand des neuen Ortes neu bewertet. Hineinverschobene Dateien werden über die Ordnerfreigabe sichtbar; hinausverschobene verlieren diesen geerbten Zugang. Etwaige eigenständige Freigaben sind separat sichtbar und zu prüfen.
- Widerruf und Ablauf verhindern weitere autorisierte Serverzugriffe entsprechend der definierten Durchsetzungsfrist. Bestehende Sitzungen, Caches, Download-URLs und Synchronisationszugänge sind dabei ausdrücklich zu behandeln.
- Bereits heruntergeladene oder offline gespeicherte Kopien können durch einen späteren Widerruf nicht zurückgeholt werden.
- Anzeige, Vorschau, Suchindex, Versionen, Papierkorb, Download, ZIP-Download, WebDAV / Sync und Verwaltungs-API verwenden dieselbe serverseitige Berechtigungsgrundlage.

## 5. Plugins und Dateinutzung im Browser

Die bestätigte Erweiterung ist eine **Plugin-Plattform**, über die Dateien möglichst direkt in Drive geöffnet werden: Vorschau, vollständiger Viewer / Player und bei geeigneten Formaten ein Editor. PDF, Office, 3D, Schaltpläne, Vektorgrafiken, Bilder, Video und Audio gehören zum geplanten Gesamtumfang. Das umfasst weder die Zusage beliebiger proprietärer Dateiformate noch uneingeschränkte Bearbeitung jeder Datei. Unterstützte Formate / Versionen und die Fähigkeiten Anzeigen, Bearbeiten, Konvertieren oder Exportieren werden je Plugin festgelegt und geprüft. D0 bestimmt die Reihenfolge für das erste Release und den anschließenden Ausbau.

### Geplante Plugin-Familien

Die folgenden Formate sind Kandidaten für die Abnahme, keine bereits belegte Kompatibilitätsliste.

| Familie | Beispiele / Umfang | Geplante Browser-Funktion und Abnahme |
| --- | --- | --- |
| PDF | PDF-Dokumente | Lesen, Seitenübersicht, Zoom, Suche und Druck nach Berechtigung; Kommentare / Annotationen als eigene Fähigkeit; große und geschützte Dokumente prüfen |
| Word / Textdokumente | DOCX, ODT; weitere Importformate gesondert | Anzeigen und mit einem Office-Plugin bearbeiten; Layouttreue, Schriften, Kommentare, Speichern und Rückexport prüfen |
| Excel / Tabellen | XLSX, ODS, CSV | Tabellen anzeigen und geeignete Formate bearbeiten; Formeln, Diagramme, Zellformate und Rückexport mit repräsentativen Dateien prüfen |
| Präsentationen | PPTX, ODP | Folien im Browser anzeigen / präsentieren; Bearbeitung abhängig vom gewählten Office-Plugin, einschließlich Layout- und Rückexportprüfung |
| 3D / CAD | Mesh- / Austauschformate wie STL, OBJ, glTF / GLB; STEP und native CAD-Dateien gesondert | Modell drehen, zoomen und untersuchen; Messung, Baugruppen und CAD-Bearbeitung nur als ausdrücklich unterstützte Plugin-Fähigkeiten; große Modelle und erforderliche Konverter prüfen |
| Schaltpläne / EDA | PDF / SVG-Exports sowie native Projektformate, beispielsweise KiCad | Plan öffnen, zoomen und Seiten / Ebenen navigieren; Projektteile und Symbole kontrolliert auflösen; native Darstellung / Bearbeitung über spezialisierte Plugins abnehmen |
| Vektorgrafiken | SVG; weitere Grafikformate gesondert | Skalierbare Vorschau und geeigneter Editor; Schriften, Ebenen und Export prüfen; aktive Inhalte / externe Referenzen kontrollieren |
| Bilder | Beispielsweise JPEG, PNG, WebP; RAW / Spezialformate gesondert | Galerie, Zoom und Metadaten; einfache Bildbearbeitung als eigene Fähigkeit mit versioniertem Speichern; große Dateien / Orientierung prüfen |
| Video | Container und Codecs getrennt deklarieren | Player mit Spulen, Untertiteln und optionalen Wiedergabevarianten; Originalstream oder isoliertes Transcoding wählen; Range-Zugriffe, große Dateien und Browser-Kompatibilität prüfen |
| Audio | Formate / Codecs je Player | Player mit Wiedergabe, Spulen und Metadaten; Playlists / Wellenform als Ausbau; Streaming, lange Aufnahmen und Rechteentzug prüfen |

### Erweiterungsvertrag und Verwaltung

- Ein versioniertes Plugin-Manifest deklariert ID, Version, kompatible Drive-Schnittstelle, MIME-Typen / Endungen, Viewer- / Editorfähigkeiten, benötigte Ressourcen, erlaubte Dateizugriffe und etwaige Server-Komponenten. Inhalt und Typ einer Datei werden geprüft; die Endung allein entscheidet nicht über die sichere Verarbeitung.
- Gemeinsame Erweiterungspunkte für Öffnen, Vorschau, Bearbeiten, Player, Export und optionale Thumbnail- / Suchindex-Erzeugung verhindern Sonderwege je Oberfläche. Core Console, Workspace und Archive Studio verwenden denselben Pluginbestand und dieselben Dateirechte.
- Eine Plugin-Verwaltung ermöglicht Berechtigten die geprüfte Installation, Aktualisierung, Aktivierung, Deaktivierung und Rückkehr zur vorherigen Version. Quellen, Integrität, Kompatibilität, Betriebsbedarf und gegebenenfalls Lizenzbedingungen werden vor Freigabe geprüft; Updates erweitern Dateirechte nicht automatisch und Deinstallation löscht keine Nutzerdokumente. Benutzer können unter den freigegebenen kompatiblen Anwendungen einen Standard zum Öffnen wählen.
- Ein fehlendes oder deaktiviertes Plugin zeigt verständlich, warum die Datei nicht im Browser geöffnet werden kann. Download bleibt bei entsprechender Berechtigung möglich; ein Pluginfehler darf den Datei- oder Freigabedienst nicht mitreißen.

### Rechte, Verarbeitung und Speichern

- Plugins entscheiden keine Datei-ACLs und erhalten keinen allgemeinen Drive- oder IAM-Sitzungstoken. Die Plattform vermittelt auf die erlaubte Datei, Aktion und Laufzeit begrenzte Zugriffe; direkter NAS-Zugriff oder ein beliebiger Server-Dateipfad gehört nicht zum Plugin-Vertrag.
- Vorschau, Suche, Player-Streams / Range-Requests, Downloads, Druck- / Exportpfade und Bearbeitung prüfen dieselbe serverseitige Berechtigungsgrundlage. Ein Gast mit **Ansehen** erhält keine Schreibfunktion, **Nur hochladen** keine Vorschau des Ordnerinhalts. Falls Projektdateien weitere Modelle, Symbole oder Bilder referenzieren, ist jeder Zugriff darauf gesondert zu autorisieren.
- Browser-Komponenten und serverseitige Konverter werden entsprechend ihrem Vertrauensniveau isoliert; Zeit, Speicher und erlaubte Netzwerkziele werden begrenzt. Makros, Skripte, externe Inhalte und aktive SVG- / Office-Inhalte werden nicht ungeprüft im Drive-Kontext ausgeführt. Drittanbieter dürfen Dateien nur nach ausdrücklich festgelegter Betriebsentscheidung außerhalb des eigenen Systems verarbeiten.
- Vorschaubilder, Suchinhalte, konvertierte Modelle und Transcodes sind geschützte Ableitungen der Datei und werden an deren Version gebunden. Caches, temporäre URLs, Ablauf, Widerruf, Löschung und Sicherung müssen deren Rechte und Aufbewahrung berücksichtigen; Änderungen ersetzen veraltete Vorschauen und Rechteentzug verhindert weitere autorisierte Abrufe.
- Konvertierung erhält die Originaldatei. Ein Editor speichert mit Schreibrecht in den versionierten Dateibestand; Sperren oder nachvollziehbare Konfliktauflösung verhindern stilles Überschreiben. Verlustbehaftete Rückexporte werden kenntlich gemacht. Gleichzeitiges Bearbeiten wird pro Editor separat abgenommen.

Alle Plugin-Funktionen müssen im lokalen Betrieb ohne zentralen IAM-Dienst nutzbar sein. D6 ergänzt später die Anmeldung; die Browser-Plugins behalten denselben Ressourcenvertrag.

## 6. Meilensteine und Abnahme

Alle technischen Meilensteine sind **geplant**. Die Priorität P1 ist ein Vorschlag für dieses Vorhaben; bestehende Funk- und IAM-Prioritäten werden damit nicht umgeordnet.

| ID | Priorität | Meilenstein | Abhängigkeit | Abnahme |
| --- | --- | --- | --- | --- |
| D0 | P1 | Umfang und Architektur entscheiden | Keine zentrale IAM-Implementierung erforderlich | ADR für Backend, Datenhaltung, UI-Integration, lokalen Login / Rollen / stabile IDs, Gastzugang, Freigabeverhalten, Clients, Plugin-Vertrag / Formatmatrix, Release-Umfang und späteren IAM-Adapter |
| D1 | P1 | Datei-Backend und lokalen Betrieb im Pilot bereitstellen | D0 | Lokale Konten / Gruppen / Dienstkonten anlegen und sperren; Login und Ressourcenrechte ohne IAM durchsetzen; Upload / Download, Neustart und gemeinsamer Restore von Dateien / Metadaten / Konten funktionieren; keine Secrets in Git |
| D2 | P1 | NetCore-Oberfläche mit lokalem Login | D1 | Core Console, Workspace und Dokumentmodus verwenden denselben berechtigten Datenbestand; mobile Bedienung sowie erlaubte / verbotene Zugriffe geprüft; zentraler SSO folgt in D6 |
| D3 | P1 | Interne und externe Ordnerfreigaben | D2, Gast- / Rechtevertrag aus D0 | Personen- / Gruppen- / Linkfreigaben, drei Rechtemodi, Vererbung, Ablauf und Widerruf mit positiven / negativen Tests |
| D4 | P1 | Große Uploads, Vorschau, Versionen und Papierkorb integrieren | D1 / D2; Freigaberechte aus D3 | Unterbrechung, parallele Änderungen, Wiederherstellung und Rechte bei allen Zugriffspfaden geprüft |
| D5 | P1 | Desktop- / Mobil-Synchronisation und Betriebsabnahme | D3 / D4, gewählte Clients | Reale Client- und Konflikttests im lokalen Betrieb ohne zentralen IAM-Dienst, Entzug von Rechten, Quotas, Backup / Restore, Monitoring und dokumentierter Rückweg |
| D6 | P1 | Zentralen Login anbinden und bestehende Identitäten migrieren | Abgenommener lokaler Betrieb; IAM M1 / M2 und verfügbarer OIDC-Adapter; IAM M3 nur bei Nutzung des gemeinsamen Rust-Bausteins | Geprüfte Konten- / Gruppenzuordnung, gemeinsamer Login und Clients; Eigentum, Dateien und Freigaben erhalten; keine ungewollte Rechteausweitung; regulären lokalen Login migrierter Konten abschalten; IAM-Ausfall und Migrationsrückweg prüfen |
| D7 | P1 | Plugin-Plattform und PDF- / Bild- / Audio- / Video-Basismodule | D0, D2 / D3 / D4; keine Abhängigkeit von D6 | Manifest / APIs, Plugin-Verwaltung, Isolation, Browser-Viewer / Player, unterstützte Formate, Streaming und Rechte einschließlich Gästen / Ableitungen geprüft; Update, Deaktivierung und Rückweg abgenommen |
| D8 | P1 | Office-Plugins für Dokumente, Tabellen und Präsentationen | D7 und ausgewählte Office-Integration; keine Abhängigkeit von D6 | Word- / Excel- / Präsentationsformate im Browser anzeigen und im vereinbarten Umfang bearbeiten; Layout, Formeln, Export, Versionen, Schreibrechte und Konflikte geprüft; gemeinsame Bearbeitung gesondert abgenommen |
| D9 | P1 | Technische Viewer / Editoren für 3D, Schaltpläne und Vektorgrafiken | D7 und gewählte Format- / Konvertermodule; keine Abhängigkeit von D6 | Festgelegte Beispieldateien einschließlich großer Modelle, Projektreferenzen und aktiver Inhalte öffnen; Darstellungsqualität, Rechte, Isolation und deklarierte Editor- / Exportfähigkeiten pro Format prüfen |

Der nächste konkrete Schritt ist D0; Drive muss dafür nicht auf ein fertiggestelltes zentrales RBAC warten. D6 ist die spätere Integration und blockiert weder die eigenständig abgenommenen D0–D5 noch die Plugin-Meilensteine D7–D9. Die IDs sind keine zwingend lineare Reihenfolge; Office- und technische Module können nach D7 parallel ausgebaut werden. Für bestehende Open-Lab-Dienste erfolgt durch diese Roadmap kein automatischer Wechsel der Anmeldung. Eine externe Freigabeoberfläche darf nicht mit offenem Open-Lab-Zugriff gleichgesetzt werden.

## 7. Offene Entscheidungen und Statuspflege

- [ ] Gewünschten Dateicloud-Umfang für das erste Release gegenüber späteren Ausbaustufen festlegen.
- [ ] Nextcloud Files + PostgreSQL gegen vollständigen Eigenbau bewerten; Ergebnis und unterstützte Version dokumentieren.
- [ ] Integrationsweg für eigenes Design, Login, APIs und vorhandene Synchronisationsclients belegen.
- [ ] Lokalen Betrieb mit Benutzer- / Gruppenverwaltung, Verwaltungsrollen, Dienstkonten, abgesicherter Anmeldung, Kontensperren und Wiederherstellung festlegen und ohne IAM abnehmen.
- [ ] Dateien / Metadaten / Versionen, Mounts, Quotas und konsistente Backups festlegen; bestehende Recorder- / Media-Library-Daten nicht ungeprüft als Drive-Bestand behandeln.
- [ ] Zentrale Identitäten / Gruppen, Dienstrollen und fachliche Ressourcenrechte eindeutig abbilden.
- [ ] Stabile lokale Identitäts-IDs, bestätigte OIDC-Kontenzuordnung, Gruppenmigration und Abschaltung regulärer lokaler Logins in D6 festlegen; Freigabeerhalt, IAM-Ausfall und Rückweg prüfen.
- [ ] Gastidentitäten, Personenverifikation, E-Mail-Codes oder alternatives Loginverfahren sowie Mailzustellung festlegen.
- [ ] Freigabevererbung, Verschiebungen, Weiterfreigabe und maximal akzeptierte Widerrufs- / Sperrfrist entscheiden.
- [ ] DNS / HTTPS und Erreichbarkeit externer Freigaben festlegen; keine privaten Netzparameter oder Zugangsdaten veröffentlichen.
- [ ] Client-Kompatibilität, Offline-Verhalten und Konfliktauflösung abnehmen.
- [ ] Plugin-Vertrag, vertrauenswürdige Quellen, Installation / Updates / Rückweg und Isolation für Browser- sowie Server-Komponenten festlegen.
- [ ] Format- / Fähigkeitsmatrix für PDF, Office, 3D / CAD, Schaltpläne, Vektorgrafiken, Bilder, Video und Audio mit Beispieldateien abnehmen; Viewer, Editor und Konverter klar ausweisen.
- [ ] Office-Integration, Codec- / Transcodingbedarf, technische Konverter, Ressourcenbedarf und geschützte Ableitungen einschließlich Widerruf und Löschen festlegen.

Dieses Vorhaben ist in [Backend-Roadmap](../system-backend/roadmap.md) und [Wiki-Roadmap im Repository](../wiki/Roadmap.md) verlinkt. Regelmäßige Projektstatusläufe sollen NETCORE-DRIVE-01 zusammen mit NETCORE-IAM-01 berücksichtigen und bestätigte Ziele, Empfehlungen, Entwicklungsstand und verifizierten Betrieb unterscheiden. Ein Dokumentationscommit ist kein Implementierungsnachweis; Termine und Fortschrittsprozente werden nicht erfunden.

## 8. Änderungsverlauf

| Datum | Änderung | Implementierungsnachweis |
| --- | --- | --- |
| 2026-10-05 | Angenommene Designrichtung und Dateicloud-Ziel aufgenommen; ökosystemweite IAM-Anbindung, externe Ordnerfreigaben, Backend-Empfehlung, Meilensteine und offene Entscheidungen dokumentiert | Dokumentation; keine technische Umsetzung |
| 2026-10-05 | Lokalen Betrieb als Rückfallebene ohne zentralen IAM-Dienst ergänzt; D0–D5 entkoppelt, Freigaben lokal abgesichert und spätere Identitätsmigration mit Ausfallregeln als D6 eingeplant | Dokumentation; keine technische Umsetzung |
| 2026-10-05 | Plugin-Plattform mit Browser-Viewern / geeigneten Editoren für PDF, Office, 3D, Schaltpläne, Vektorgrafiken, Bilder, Video und Audio ergänzt; Formatabnahme, Plugin-Verwaltung, Rechte / Isolation und D7–D9 eingeplant | Dokumentation; keine technische Umsetzung |

## 9. Technische Referenzen

Die Primärquellen belegen Integrationsmöglichkeiten, keine bereits erfolgte Umsetzung in NetCore.

- [Nextcloud Datenbankkonfiguration](https://docs.nextcloud.com/server/stable/admin_manual/configuration_database/linux_database_configuration.html): PostgreSQL als unterstütztes und empfohlenes Backend; MariaDB ist nicht zwingend.
- [Nextcloud WebDAV APIs](https://docs.nextcloud.com/server/stable/developer_manual/client_apis/WebDAV/index.html): Dateioperationen und weiterführende Client-Schnittstellen.
- [Nextcloud OCS Share API](https://docs.nextcloud.com/server/stable/developer_manual/client_apis/OCS/ocs-share-api.html): Freigaben und deren Rechte, Änderung und Widerruf.
- [Nextcloud File Sharing](https://docs.nextcloud.com/server/stable/user_manual/en/files/sharing.html): Ordnerfreigaben, Uploadbriefkasten, Passwortschutz und Ablauf.
- [Nextcloud OIDC-Integration](https://github.com/nextcloud/user_oidc): Anbindung eines zentralen Identity Providers und Gruppenmapping.
- [OneDrive / SharePoint Freigabelinks](https://learn.microsoft.com/en-us/sharepoint/shareable-links-anyone-specific-people-organization): Unterschied zwischen personenbezogener Freigabe und weitergebbarem Link.
- [SQLite Einsatzgrenzen](https://www.sqlite.org/whentouse.html): lokaler Anwendungsbetrieb und Grenzen bei Netzwerk-Dateisystemen / Schreibkonkurrenz.
