# Roadmap: NetCore Drive – Dateicloud und externe Ordnerfreigaben

| Feld | Wert |
| --- | --- |
| Roadmap-ID | NETCORE-DRIVE-01 |
| Erstellt / aktualisiert | 2026-10-05, Europe/Berlin |
| Gesamtstatus | **Geplant; Designrichtung bestätigt, Backend- und Betriebsentscheidung offen** |
| Produktziel | Dateicloud mit Nextcloud-ähnlichem Funktionsumfang und der einfachen Bedienung von OneDrive |
| Bestätigte Designrichtung | Core Console als Hauptoberfläche, Workspace als Startansicht, Archive Studio als Dokumentmodus; eigenes NetCore-Design |
| Datenhaltung | Dateien auf NAS / Dateispeicher; Verwaltungsdaten getrennt; MariaDB ist keine Voraussetzung |
| Zentrale Abhängigkeit | [NETCORE-IAM-01: zentrale Anmeldung und RBAC](CENTRAL_IDENTITY_RBAC_ROADMAP.md) |
| Architekturvorschlag | Eigene NetCore-Oberfläche vor Nextcloud Files mit PostgreSQL; vollständiger Eigenbau als zu bewertende Alternative |
| Nächster Schritt | Datei-Funktionsumfang, Backend, Gastanmeldung und Rechteabbildung in einer Architekturentscheidung festlegen |
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
| Vorschau und Suche | Dokument-/Medienvorschau, Favoriten, zuletzt verwendet und Suche | Treffer, Vorschauen und Vorschaudateien nur für berechtigte Personen |
| Versionen und Papierkorb | Frühere Dateiversionen wiederherstellen; versehentlich gelöschte Inhalte zurückholen | Rechte, Aufbewahrung, Speicherbedarf und Restore-Verhalten festlegen und prüfen |
| Freigaben | Einzeldateien und ganze Ordner intern oder extern freigeben; Zugang verwalten | Personen, Gruppen und Links unterscheiden; Vererbung, Ablauf und Widerruf prüfen |
| Synchronisation | Desktop-/Mobilzugriff, Offline-Nutzung und nachvollziehbare Konfliktbehandlung | Backend und Clients auswählen; echte Client-Tests für Änderungen, Konflikte und Rechteentzug |
| Zusammenarbeit | Kommentare und bei gewünschtem Ausbau gemeinsames Bearbeiten von Office-Dokumenten | Viewer / Office-Komponente und deren Anmeldung gesondert festlegen; folgt nicht automatisch aus einer neuen Oberfläche |
| Betrieb | Sicherung, Wiederherstellung, Quotas, Aufbewahrung, Monitoring und nachvollziehbare Verwaltungsereignisse | Dateiinhalt, Metadaten, Freigaben und Versionen gemeinsam wiederherstellen |

Welche dieser Ziele das erste Release umfasst und welche als Ausbau folgen, wird in D0 entschieden. Bestätigte Designrichtung und Produktziel werden dadurch nicht als bereits entwickelte Funktion ausgegeben.

## 3. Gemeinsame Anmeldung und Ressourcenrechte

Drive wird an [NETCORE-IAM-01](CENTRAL_IDENTITY_RBAC_ROADMAP.md) angebunden. Zentrale Identitäten, Gruppen und Dienstrollen ermöglichen einen gemeinsamen Login zwischen den angebundenen NetCore-Oberflächen. Die bestehende IAM-Planung lässt lokale Konten und eine optionale AD-Anbindung zu; Produktwahl und Betrieb des Identity-Dienstes bleiben deren Architekturentscheidung.

Die Zuständigkeiten sind verbindlich zu trennen:

- **IAM:** Wer ist angemeldet, welchen Dienst darf die Identität nutzen und welche Verwaltungsaktionen darf sie ausführen?
- **Drive-Backend:** Auf welche konkreten Dateien / Ordner darf die Identität zugreifen und mit welchen Einzelrechten?
- **Gemeinsame NetCore-Verwaltung:** Benutzer, Gruppen, Dienstrollen, Freigaben und deren Herkunft verständlich anzeigen; die fachliche Berechtigungsautorität bleibt eindeutig.

Ein Basisstations-Techniker erhält durch seine Dienstrolle keinen pauschalen Zugriff auf private Drive-Dateien. Eine externe Gastidentität erhält durch ihre Ordnerfreigabe keine internen NetCore-Dienstrollen. Ein bestehendes Dateibackend darf nicht durch ein zweites, widersprüchliches ACL-System in der Oberfläche umgangen werden.

Für automatische Ablagen aus Recording, TBS oder Discovery werden eigene Dienstidentitäten mit begrenzten Aufgaben und Zielordnern vorgesehen. Menschliche Konten, Gastidentitäten, Maschinenidentitäten und TETRA-Teilnehmer bleiben getrennte Klassen. Die laufende Funkvermittlung ist unabhängig von Drive und von einer erreichbaren Web-Anmeldung.

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
| Interne Personen / Gruppen | Bestehende zentrale Identitäten und Gruppen verwenden | Dienstzugang ersetzt keine konkrete Datei- / Ordnerfreigabe |
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

## 5. Meilensteine und Abnahme

Alle technischen Meilensteine sind **geplant**. Die Priorität P1 ist ein Vorschlag für dieses Vorhaben; bestehende Funk- und IAM-Prioritäten werden damit nicht umgeordnet.

| ID | Priorität | Meilenstein | Abhängigkeit | Abnahme |
| --- | --- | --- | --- | --- |
| D0 | P1 | Umfang und Architektur entscheiden | IAM M1 für gemeinsame Identitäts- / Rollenverträge | ADR für Backend, Datenhaltung, UI-Integration, Gastzugang, Freigabeverhalten, Clients und Release-Umfang |
| D1 | P1 | Datei-Backend und Betriebsgrundlage im Pilot | D0 | Upload / Download, Neustart und gemeinsamer Restore von Dateien / Metadaten funktionieren; keine Secrets in Git |
| D2 | P1 | NetCore-Oberfläche mit gemeinsamem Login | D1, IAM M2 / M3 beziehungsweise abgestimmter OIDC-Adapter | Core Console, Workspace und Dokumentmodus verwenden denselben berechtigten Datenbestand; mobile Bedienung und SSO geprüft |
| D3 | P1 | Interne und externe Ordnerfreigaben | D2, Gast- / Rechtevertrag aus D0 | Personen- / Gruppen- / Linkfreigaben, drei Rechtemodi, Vererbung, Ablauf und Widerruf mit positiven / negativen Tests |
| D4 | P1 | Große Uploads, Vorschau, Versionen und Papierkorb integrieren | D1 / D2; Freigaberechte aus D3 | Unterbrechung, parallele Änderungen, Wiederherstellung und Rechte bei allen Zugriffspfaden geprüft |
| D5 | P1 | Desktop- / Mobil-Synchronisation und Betriebsabnahme | D3 / D4, gewählte Clients | Reale Client- und Konflikttests, Entzug von Rechten, Quotas, Backup / Restore, Monitoring und dokumentierter Rückweg |

Der nächste konkrete Schritt ist D0. Für bestehende Open-Lab-Dienste erfolgt durch diese Roadmap kein automatischer Wechsel der Anmeldung. Eine externe Freigabeoberfläche darf nicht mit offenem Open-Lab-Zugriff gleichgesetzt werden.

## 6. Offene Entscheidungen und Statuspflege

- [ ] Gewünschten Dateicloud-Umfang für das erste Release gegenüber späteren Ausbaustufen festlegen.
- [ ] Nextcloud Files + PostgreSQL gegen vollständigen Eigenbau bewerten; Ergebnis und unterstützte Version dokumentieren.
- [ ] Integrationsweg für eigenes Design, Login, APIs und vorhandene Synchronisationsclients belegen.
- [ ] Dateien / Metadaten / Versionen, Mounts, Quotas und konsistente Backups festlegen; bestehende Recorder- / Media-Library-Daten nicht ungeprüft als Drive-Bestand behandeln.
- [ ] Zentrale Identitäten / Gruppen, Dienstrollen und fachliche Ressourcenrechte eindeutig abbilden.
- [ ] Gastidentitäten, Personenverifikation, E-Mail-Codes oder alternatives Loginverfahren sowie Mailzustellung festlegen.
- [ ] Freigabevererbung, Verschiebungen, Weiterfreigabe und maximal akzeptierte Widerrufs- / Sperrfrist entscheiden.
- [ ] DNS / HTTPS und Erreichbarkeit externer Freigaben festlegen; keine privaten Netzparameter oder Zugangsdaten veröffentlichen.
- [ ] Client-Kompatibilität, Offline-Verhalten und Konfliktauflösung abnehmen.

Dieses Vorhaben ist in [Backend-Roadmap](../system-backend/roadmap.md) und [Wiki-Roadmap im Repository](../wiki/Roadmap.md) verlinkt. Regelmäßige Projektstatusläufe sollen NETCORE-DRIVE-01 zusammen mit NETCORE-IAM-01 berücksichtigen und bestätigte Ziele, Empfehlungen, Entwicklungsstand und verifizierten Betrieb unterscheiden. Ein Dokumentationscommit ist kein Implementierungsnachweis; Termine und Fortschrittsprozente werden nicht erfunden.

## 7. Änderungsverlauf

| Datum | Änderung | Implementierungsnachweis |
| --- | --- | --- |
| 2026-10-05 | Angenommene Designrichtung und Dateicloud-Ziel aufgenommen; ökosystemweite IAM-Anbindung, externe Ordnerfreigaben, Backend-Empfehlung, Meilensteine und offene Entscheidungen dokumentiert | Dokumentation; keine technische Umsetzung |

## 8. Technische Referenzen

Die Primärquellen belegen Integrationsmöglichkeiten, keine bereits erfolgte Umsetzung in NetCore.

- [Nextcloud Datenbankkonfiguration](https://docs.nextcloud.com/server/stable/admin_manual/configuration_database/linux_database_configuration.html): PostgreSQL als unterstütztes und empfohlenes Backend; MariaDB ist nicht zwingend.
- [Nextcloud WebDAV APIs](https://docs.nextcloud.com/server/stable/developer_manual/client_apis/WebDAV/index.html): Dateioperationen und weiterführende Client-Schnittstellen.
- [Nextcloud OCS Share API](https://docs.nextcloud.com/server/stable/developer_manual/client_apis/OCS/ocs-share-api.html): Freigaben und deren Rechte, Änderung und Widerruf.
- [Nextcloud File Sharing](https://docs.nextcloud.com/server/stable/user_manual/en/files/sharing.html): Ordnerfreigaben, Uploadbriefkasten, Passwortschutz und Ablauf.
- [Nextcloud OIDC-Integration](https://github.com/nextcloud/user_oidc): Anbindung eines zentralen Identity Providers und Gruppenmapping.
- [OneDrive / SharePoint Freigabelinks](https://learn.microsoft.com/en-us/sharepoint/shareable-links-anyone-specific-people-organization): Unterschied zwischen personenbezogener Freigabe und weitergebbarem Link.
- [SQLite Einsatzgrenzen](https://www.sqlite.org/whentouse.html): lokaler Anwendungsbetrieb und Grenzen bei Netzwerk-Dateisystemen / Schreibkonkurrenz.
