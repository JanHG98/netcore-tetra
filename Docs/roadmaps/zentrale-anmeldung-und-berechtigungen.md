# Roadmap: Zentrale Anmeldung und RBAC für NetCore-Tetra

**Standprüfung am 09.10.2026:** Die Roadmap beschreibt weiterhin Planung. Im geprüften `main@c3ccdb4` ist kein eigenständiger zentraler IAM- beziehungsweise Drive-Dienst im Workspace oder Betriebsinventar implementiert. Lokale Benutzer, Tokens und vorhandene Dateispeicher ersetzen diese geplanten Dienste nicht. Die unten angegebenen frühen Code-/Entwurfsstände bleiben historische Ausgangspunkte; Priorität und Folge stehen in der [Gesamtroadmap](gesamtroadmap.md).

**Gesamtpriorität / nächster Schritt:** [NETCORE-MASTER-01](gesamtroadmap.md). Diese Fachroadmap führt IAM-Umfang und Abnahme; für die aktuelle Reihenfolge im gesamten Projekt zuerst die zentrale Gesamtroadmap prüfen.

| Feld | Wert |
| --- | --- |
| Roadmap-ID | NETCORE-IAM-01 |
| Erstellt / aktualisiert | Erstellt 2026-10-03; aktualisiert 2026-10-05, Europe/Berlin |
| Gesamtstatus | **Geplant; bisher nur dokumentiert, keine zentrale IAM-Implementierung durch diesen Auftrag** |
| Nutzerauftrag | Zentrale Anmeldung und Rechteverwaltung statt regulärer Basisstations-Zugangsdaten in config.toml; Roadmap im Docs-Ordner für die Projektstatusläufe |
| Geltungsbereich | Basisstations-Dashboard, Control Room, Verwaltungsoberflächen / Management-APIs der Zentraldienste und NetCore Drive einschließlich begrenzter Gastzugänge |
| Architekturstand | **Empfehlung:** eigener Identity-LXC, Keycloak als Identity Provider, gemeinsame Rust-Integration; technische Festlegung noch offen |
| Geprüfter Code-Ausgangsstand | main, Commit f4fd490f577c1315279ecc89f0c6b1808ee7733d |
| Nächster Schritt | Zugänge vollständig inventarisieren und Architekturentscheidung dokumentieren |
| Zeitplanung | Meilensteinfolge ohne verbindliche Termine oder erfundene Fertigstellungsprozente |

## 1. Ziel und Umfang

Benutzer sollen zentral angelegt, gesperrt und mit Rollen sowie Ressourcenbereichen versehen werden. Eine zentrale Anmeldung soll den Wechsel zwischen Basisstations-Dashboards, Control Room und angebundenen Zentraldiensten ermöglichen. Reguläre menschliche Benutzernamen und Passwörter sollen aus den einzelnen Basisstations-Konfigurationen verschwinden.

Die Berechtigungsprüfung bleibt im jeweiligen Backend: Identität zentral verwalten, jede Aktion lokal durchsetzen. Ein Benutzer mit Zugriff auf eine Station erhält dadurch keinen automatischen Zugriff auf andere Stationen. Die laufende Funkvermittlung darf nicht von einer erreichbaren Web-Anmeldung oder einem geöffneten Browser abhängen.

Diese Roadmap ist ein Planungsartefakt. Der Auftrag bestätigt das Ziel und die Dokumentation, aber weder eine bereits erfolgte Installation noch die endgültige Wahl von Keycloak, LXC oder AD-Anbindung. Die technische Umsetzung erfolgt später in eigenen Entwicklungsbranches und überprüfbaren PRs.

## 2. Belegter Ausgangsstand

Die folgenden Befunde stammen aus einer gezielten Code- und Dokumentationssichtung. Sie sind keine vollständige Sicherheitsprüfung und kein Nachweis des Live-Zustands der installierten Anlagen.

| Bereich | Befund am geprüften main-Stand | Folgerung |
| --- | --- | --- |
| Basisstations-Konfiguration | CfgDashboard führt optional username und password; ohne beide ist der Zugang offen | Beim Umstieg darf ein fehlender Identity-Dienst keinen offenen Zugriff erzeugen |
| Basisstations-Webserver | Formularlogin mit lokalem, prozessinternem Cookie-Sitzungsspeicher; Prüfung gegen das konfigurierte Zugangsdatenpaar | Bestehende Sitzungen und Loginlogik kontrolliert ersetzen; keinen zweiten parallelen Dauerlogin schaffen |
| Control Room | Eigene Benutzerverwaltung; Rollen Node, Viewer, Operator und Admin; getrennte Maschinenzugänge | Identitäten und Rollen migrieren; Maschinenidentitäten getrennt behandeln |
| Backend-WebUI-Standard | Zentrale Hauptanmeldung, gemeinsame Rollen und deaktivierbarer lokaler Break-Glass-Zugang vorgesehen | Vorhandene Architekturregel konkret umsetzen |
| Security Core | TETRA-Teilnehmerauthentisierung, Security-Class-Policy und DCK-Kontexte; Management derzeit als Open Lab dokumentiert | Menschliches Web-IAM als getrennten Aufgabenbereich behandeln |
| Gemeinsame Plattform | Shared-Bibliotheken und WebUI-Bausteine vorhanden; shared ist selbst kein Runtime-Dienst | Authentifizierungsbibliothek dort einordnen, ohne zusätzlichen Shared-LXC |
| Übrige Dienste | Dienstmatrix beschreibt Verwaltungsbereiche und geschützte Aktionen; mehrere Oberflächen noch Open Lab | Tatsächliche Login-, API- und Transportwege in M0 einzeln prüfen |

Quellen: [Dashboard-Konfiguration](../../crates/tetra-config/src/bluestation/sec_dashboard.rs), [Dashboard-Webserver](../../crates/tetra-entities/src/net_dashboard/server.rs), [Control-Room-Authentifizierung](../../bins/netcore-control-room/src/auth.rs), [Backend-WebUI-Standard](../design/dienstoberflaechen-gestaltungsstandard.md), [Dienstmatrix](../design/dienstoberflaechen-funktionsmatrix.md), [Security Core](../../system-backend/security-core/README.md), [Shared-Komponenten](../../system-backend/shared/README.md).

## 3. Empfohlene Zielarchitektur

| Baustein | Vorgeschlagene Verantwortung | Status |
| --- | --- | --- |
| Eigener Identity-LXC | Keycloak, zunächst eigene PostgreSQL-Datenbank im gleichen LXC, Administration, Backups und Monitoring | Empfehlung, nicht installiert |
| Identity Provider | Lokale Benutzer, optional AD über abgesicherte LDAP-Anbindung, SSO, MFA, zentrale Gruppen und Rollenzuweisung | Produktwahl offen; Keycloak bevorzugter Kandidat |
| Gemeinsamer Rust-Baustein netcore-auth | OIDC, Sessions, Tokenprüfung, Berechtigungen, Ressourcenprüfung und Audit-Kontext | Neu zu entwickeln; genauer Paketpfad in M0/M1 festlegen |
| Fachbackends | Verbindliche Prüfung jeder geschützten Aktion einschließlich API, Exporten und WebSocket-Befehlen | Schrittweise Migration |
| NetCore-Verwaltungsoberfläche | Rollen, Ressourcenzuordnung und Benutzerverwaltung im gemeinsamen NetCore-Design | Ausbauziel; für den ersten Pilot genügt die Identity-Administration |
| Discovery / Deployment | Vertrauenswürdige Client-Provisionierung, Stationseinbindung und getrennte Maschinenzugänge | Integration nach dem Pilot |
| Observability | Identity-Erreichbarkeit, Authentifizierungsfehler, RBAC-Ablehnungen und Audit | Integration zu planen |

Eine Unterbringung in der bestehenden Deployment-VM bleibt als Pilotoption möglich. Sie koppelt allerdings deren Wartung an neue Logins. Die Kopplung an Control Room oder Security Core wird nicht empfohlen. Ein separates hochverfügbares Identity-/Datenbank-Setup ist eine spätere Ausbauentscheidung; ein einzelner LXC ist noch keine Hochverfügbarkeit.

Authentik bleibt eine Alternative. Ein eigener vollständiger Passwort-, MFA- und Tokenserver wird für das erste Ziel nicht empfohlen. NetCore-spezifische Rechte und Oberflächen entwickeln wir innerhalb unseres Projekts; die eigentliche Anmeldung verwendet ein etabliertes Protokoll.

### Noch zu entscheiden

- [ ] Identity-Produkt und unterstützte Version auswählen.
- [ ] Betriebsort, Datenbank, Backupziel und späteren Verfügbarkeitsbedarf festlegen.
- [ ] Vertrauenswürdigen DNS-Namen, HTTPS-Zertifikate und festen Issuer festlegen.
- [ ] Lokale Konten und optionale AD-Anbindung einschließlich Gruppenzuordnung beschließen.
- [ ] Rollen, Einzelrechte, Ressourcenbereiche und Trennung von Account-Administration / Fachadministration beschließen.
- [ ] Token- und Sitzungsdauer sowie gewünschte maximale Verzögerung einer Sperre festlegen.
- [ ] Regeln für isolierte Stationen, Neustart ohne Zentrale und lokale Notzugänge beschließen.
- [ ] Besitzerrolle, Schutz des letzten administrativen Zugangs und MFA für privilegierte Aktionen festlegen.

## 4. Berechtigungsmodell

Eine Zuweisung besteht aus **Identität + Rolle/Einzelrecht + Ressourcenbereich**. Benutzer können mehrere Zuweisungen erhalten. Rechte sind nicht allein aus einem globalen Rang abzuleiten.

| Vorgeschlagene Rolle | Zweck | Wichtige Grenze |
| --- | --- | --- |
| viewer | Freigegebene Zustände und Diagnosedaten lesen | Kein Zugriff auf Geheimnisse oder automatisch auf alle Aufnahmen |
| operator | Freigegebene Betriebsaktionen, beispielsweise SDS oder Durchsagen | Keine impliziten Rechte für RF-Konfiguration oder Benutzerverwaltung |
| technician | Technische Diagnose, Konfiguration, Updates und Wartung | Nur zugewiesene Stationen / Dienste; kritische Aktionen gesondert |
| administrator | Benutzer- und Rechteverwaltung im delegierten Bereich | Keine automatische Möglichkeit, eigene Rechte über diesen Bereich hinaus zu erhöhen |
| auditor | Audit und ausdrücklich freigegebene Exporte | Keine Betriebsänderungen; Aufzeichnungsinhalte separat berechtigen |
| owner | Netzweite Administration / Godmode | Audit und Regeln zum Umgang mit Rohschlüsseln bleiben wirksam |

Die Rollennamen sind ein Vorschlag. Der vorhandene Standard verwendet unter anderem administrator, der Control Room intern Admin; ein einheitliches Mapping ist Teil der Migration.

Beispiele für einzelne Berechtigungen: Station ansehen, Station konfigurieren, Station aktualisieren, Station neu starten, SDS senden, Audio einspeisen, Aufnahmen lesen/exportieren/löschen, Audit lesen, Benutzer verwalten und Rollen zuweisen. Ihre endgültigen technischen Bezeichner werden in M1 definiert.

Beispielzuweisung: Ein Techniker kann TBS-A konfigurieren und TBS-B ausschließlich ansehen. Ein Operator darf nur für bestimmte Gruppen Durchsagen auslösen. Die Zielressource muss bei jeder Aktion aus serverseitig verifizierten Daten geprüft werden.

Menschliche Konten, Dienste-/Stationsidentitäten und TETRA-Teilnehmeridentitäten sind getrennte Identitätsklassen. Rollen im Web-IAM ändern nicht automatisch TETRA-Authentisierung oder Schlüsselmaterial.

### 4.1 Ökosystemweite Anbindung und NetCore Drive

Mit der Erweiterung vom 2026-10-05 wird [NETCORE-DRIVE-01](dateicloud-und-ordnerfreigaben.md) in die zentrale Anmeldung und Dienstrollenplanung aufgenommen. Gemeinsame Identitäten, Gruppen und anwendungsspezifische Rollen gelten für die angebundenen NetCore-Oberflächen; Datei- und Ordnerrechte bleiben fachliche Ressourcenrechte des Drive-Backends.

- Ein Recht zum Nutzen oder Administrieren eines Dienstes vermittelt keinen pauschalen Zugriff auf Dateien, Aufnahmen oder andere Dienste. Gemeinsame Rollen / Gruppen werden ausdrücklich auf die jeweilige fachliche Autorisierung abgebildet.
- Personenbezogene externe Freigaben verwenden eine bestätigte Gastidentität mit ausschließlich zugewiesenem Datei- / Ordnerzugang. Gäste benötigen kein Konto im internen AD und erhalten keine internen Dienstrollen. E-Mail-Codes sind ein Vorschlag für die Gastanmeldung, kein bereits vorhandener Bestandteil der OIDC-Integration.
- Linkfreigaben sind eigene begrenzte Zugangsberechtigungen; Linkbesitz ist kein Nachweis einer benannten Benutzeridentität. Passwort, Ablauf und Widerruf sind in Drive durchzusetzen.
- Dienste wie Recording, TBS und Discovery erhalten getrennte Maschinenidentitäten mit passenden Aufgaben und Zielordnern. Dateispeicher und IAM dürfen keine konkurrierenden Rechteautoritäten erzeugen.
- Stabile Identitäts- / Objekt-IDs, Gruppenänderungen und Entzug von Rechten müssen im Drive-Backend nachvollziehbar ankommen. OIDC-Gruppenprovisionierung bei Anmeldung allein belegt keine sofortige Sperre aller bereits laufenden Zugriffe.
- Drive, Gäste und Freigaben werden in M0 inventarisiert und in M1 bei Identitätsklassen, Rollen, Ressourcenrechten und Sperrfristen berücksichtigt. Ihre konkrete Implementierung / Abnahme wird in der Drive-Roadmap geführt; bestehende Funk- und IAM-Meilensteine werden nicht als erledigt umgestuft.

Die laufende Funkvermittlung bleibt von Drive und der Erreichbarkeit des Web-IAM unabhängig. Freigaben, Suchtreffer, Vorschau, Downloads, ZIP, Versionen, Papierkorb und Sync verwenden dieselbe fachliche Berechtigungsgrundlage. Details zum Produktumfang, zur externen Ordnervererbung und zur noch offenen Dateicloud-Backend-Auswahl stehen in [NETCORE-DRIVE-01](dateicloud-und-ordnerfreigaben.md).

### 4.2 Drive-Übergang ohne zentralen IAM-Dienst

NetCore Drive darf vor dem zentralen IAM-Dienst in einem ausdrücklich gewählten lokalen Betriebsmodus starten. Lokale Benutzer, Gruppen, Verwaltungsrollen und getrennte Dienstkonten erhalten vollständig durchgesetzte Datei- / Ordnerrechte; bestätigte Gäste, Personen- / Linkfreigaben und Uploadbriefkästen funktionieren mit denselben Ablauf- und Widerrufsregeln. Die Drive-Meilensteine D0–D5 warten nicht auf M1–M3; die gemeinsame Anmeldung und Migration folgen separat in D6. Lokale Konten direkt in Drive sind von lokalen Konten im späteren zentralen Identity-Dienst zu unterscheiden.

Bei der Migration bleiben interne Drive-Identitäts- / Objekt-IDs die Referenz für Eigentum und Freigaben. Eine bestätigte Zuordnung verbindet bestehende Konten mit zentralen `issuer` / `subject`-Identitäten; Namen oder E-Mail-Adressen allein berechtigen nicht zur Kontenverknüpfung. Gruppen / Rollen werden geprüft, bestehende Freigaben erhalten und reguläre lokale Passwortlogins migrierter Konten deaktiviert. Abnahme, Sicherung und Rückweg stehen in D6 der Drive-Roadmap.

Der lokale Anfangsbetrieb ist keine automatische Ausfallumschaltung: Nach zentraler Anbindung führt ein nicht erreichbarer Identity-Dienst weder zu offenem Zugriff noch zur Reaktivierung regulärer lokaler Logins. Bestehende Zugänge unterliegen den definierten Gültigkeits- / Sperrfristen; ein separat eingerichteter lokaler Notfallzugang bleibt begrenzt und protokolliert. Die bestehenden IAM-Ausfallregeln für Basisstationen und Zentraldienste gelten weiterhin.

Die geplanten Browser-Plugins in Drive verwenden denselben Ressourcenvertrag im lokalen und zentralen Betrieb. Viewer, Editoren, Konverter und Player erhalten nur begrenzte Dateizugriffe und keine allgemeinen IAM-Sitzungstokens; ihre D7–D9-Meilensteine warten nicht auf die zentrale Migration D6.

## 5. Meilensteine und Abnahme

Prioritäten P0/P1/P2 sind Empfehlungen für die Reihenfolge. Alle technischen Meilensteine sind derzeit geplant.

| ID | Priorität | Meilenstein | Abhängigkeit | Status |
| --- | --- | --- | --- | --- |
| M0 | P0 | Zugänge und geschützte Aktionen inventarisieren | Keine | Geplant; erste Teilbefunde in Abschnitt 2 |
| M1 | P0 | Architektur, Rollen und Ausfallregeln entscheiden | M0 | Geplant |
| M2 | P0 | Identity-Pilot bereitstellen | M1 | Geplant |
| M3 | P0 | Gemeinsamen Rust-Auth-Baustein entwickeln | M1, M2 für Integration | Geplant |
| M4 | P0 | Eine Basisstation als Pilot migrieren | M2, M3 | Geplant |
| M5 | P1 | Control Room und Zentraldienste schrittweise migrieren | M4 | Geplant |
| M6 | P1 | Discovery / Deployment integrieren und reguläre TOML-Logins ablösen | M4, M5 je betroffener Komponente | Geplant |
| M7 | P1 | Betrieb, Ausfallverhalten und Wiederherstellung abschließend abnehmen | Frühe Prüffälle ab M2; Gesamtabschluss nach M6 | Geplant |
| M8 | P2 | NetCore-IAM-WebUI, AD-Ausbau, NFC/RFID und höhere Verfügbarkeit | Stabiler Grundbetrieb | Geplant / optionale Ausbaustufe |

### M0 — Bestandsaufnahme

Aufgaben: Alle Weboberflächen, schreibenden/lesenden APIs, WebSocket-/SSE-Verbindungen, Downloads und Maschinenzugänge erfassen. Aktuelle Login-Konfigurationen, Sitzungsregeln und Open-Lab-Ausnahmen ermitteln. Rollen aus dem Control Room und Anforderungen der Dienstmatrix zusammenführen. TBS-Webserver und Backend-HTTP-Stacks auf Integrationsmöglichkeiten prüfen.

**Abnahme:** Eine vollständige Komponenten-/Endpunktmatrix mit aktuellem Schutz, benötigtem Einzelrecht und Ressourcenbereich liegt vor. Ungeprüfte Komponenten sind ausdrücklich als ungeprüft markiert.

### M1 — Architekturentscheidung

Aufgaben: Offene Entscheidungen aus Abschnitt 3 als ADR dokumentieren; Anmeldung und Autorisierung getrennt beschreiben. Für das erste Release möglichst Rollen und Ressourcenbereiche über klar definierte Claims/Zuweisungen abbilden. Eine zusätzliche zentrale Policy-API nur einführen, wenn ihre Notwendigkeit und ihr Ausfallverhalten belegt sind.

**Abnahme:** Produkt, Betriebsort, Benutzerquelle, Vertrauensmodell, Client-/Audience-Modell, Rollen, Ausfallregeln, Sperrfrist, Notzugang und Migrationsverfahren sind entschieden. Es gibt ein Modell für delegierte Rechteverwaltung ohne ungewollte Selbsterhöhung.

### M2 — Identity-Pilot

Aufgaben: Identity-System mit HTTPS und stabilem DNS-Namen bereitstellen. Benutzergruppen, OIDC-Clients und Testkonten anlegen. Privilegierte Konten absichern. Wiederherstellbares Backup für Konfiguration, Datenbank und benötigtes Schlüsselmaterial planen und überprüfen.

**Abnahme:** Zwei Benutzer mit unterschiedlichen Rechten können sich anmelden; Datenbank-/Identity-Neustart und ein Restore im Test funktionieren. Zugangsdaten und Schlüssel sind nicht im Repository enthalten. Noch keine automatische Änderung produktiver Basisstationen.

### M3 — Gemeinsame Integration

Aufgaben: Authorization Code mit PKCE, state-/nonce-Prüfung, sichere lokale Browser-Sessions, Logout und CSRF-Schutz integrieren. Für APIs geeignete Access Tokens verwenden; Signatur, zugelassene Algorithmen, Issuer, Audience, Ablauf und Tokenart prüfen. JWKS-Cache und Rotation behandeln. Berechtigungen/Ressourcen und Audit-Kontext einheitlich prüfen.

**Abnahme:** Positive und negative Tests decken erlaubte/verbotene Aktionen, falsche Station, manipuliertes/abgelaufenes Token, falschen Issuer/Audience und Schlüsselwechsel ab. Fehler verweigern geschützten Zugriff. Authentifizierung verursacht keine blockierenden Identity-Anfragen im RF-/Echtzeitpfad.

### M4 — Basisstations-Pilot

Aufgaben: Zentrale Anmeldung im vorhandenen Dashboard anbinden; erlaubte Aktionen entsprechend dem neuen UI-Design anzeigen. API, Exporte und WebSocket-Kommandos serverseitig schützen. Sitzungsende und Entzug von Rechten auch für lang laufende Verbindungen behandeln. Ausdrücklich konfigurierte öffentliche Übersicht separat prüfen.

**Abnahme:** Zwei Rollen und zwei Ressourcenbereiche werden nachweisbar unterschieden; direkte API-Aufrufe umgehen die Rechte nicht. Identity-Ausfall, Ablauf, Neustart ohne Zentrale und Notzugang wurden nach der ADR-Regel geprüft. Bestehende Funkfunktionen zeigen im Pilot keine durch den Umbau verursachte Regression. Rückkehr zur letzten geprüften Version ist dokumentiert.

### M5 — Control Room und Zentraldienste

Aufgaben: Control-Room-Benutzer und Rollen auf stabile zentrale Identitäten abbilden; Dubletten und Namensänderungen berücksichtigen. Keine schwachen/inkompatiblen Passwortspeicher ungeprüft importieren; erforderliche Neuanmeldung/Passwortsetzung dokumentieren. Zentraldienste einzeln entsprechend Risiko und Nutzung migrieren. Maschinenidentitäten mit begrenzten Rechten einrichten; vorhandene Transportverfahren erst nach eigener Prüfung umstellen.

**Abnahme:** Jeder migrierte Dienst hat nachvollziehbare Rechte und Audit; SSO funktioniert zwischen den angebundenen Oberflächen. Control-Room-Ausfall verhindert nicht die eigenständige Verwaltung anderer Dienste. Offene Open-Lab-Komponenten bleiben in einer Restliste sichtbar; eine Teilmigration wird nicht als vollständiger Schutz ausgegeben.

### M6 — Provisionierung und Ablösung

Aufgaben: Neue Stationen erhalten beim Deployment ihre Identity-Client-Konfiguration und eindeutige Maschinenidentität. Discovery darf Endpunkte finden, aber keinen beliebigen gefundenen Identity-Server als vertrauenswürdig übernehmen. Geheimnisse über einen geschützten Provisionierungspfad bereitstellen, nicht in Git oder Logs. Reguläre menschliche TOML-Zugangsdaten je erfolgreich migrierter Komponente entfernen.

**Abnahme:** Neuer Stationsaufbau, erneute Provisionierung nach defektem Datenträger und Widerruf funktionieren nachweisbar. Fehlender Issuer, falsche Konfiguration oder ausgefallene Zentrale öffnen keine Verwaltungs-API. Übergangs- und Rücksetzverfahren sowie getrennte Notzugänge sind dokumentiert.

### M7 — Betriebsabnahme

Aufgaben: Prüffälle aus Abschnitt 6 vollständig durchführen; Monitoring, Secret-/Schlüsselrotation, Restore und Audit-Nachlieferung prüfen. Installations-, Bedienungs- und Reparaturdokumentation aktualisieren. Ressourcenbedarf des Identity-LXC unter tatsächlicher Nutzung messen.

**Abnahme:** Ausfall-/Wiederherstellungsnachweise, Rollen-/Dienstmatrix, dokumentierte Sperrfristen, Backups und Betriebsanweisungen liegen vor. Keine pauschale Produktivfreigabe allein anhand erfolgreicher Anmeldung oder Mock-Tests.

### M8 — Optionale Erweiterungen

NetCore-WebUI für Benutzer, Rollen und Ressourcenzuordnung; AD-Gruppenmapping; Arbeitsplätze und später NFC/RFID; delegierte Administration; bei Bedarf mehrere Identity-Instanzen und Datenbank-Hochverfügbarkeit.

**Abnahme je Erweiterung:** Eigene Anforderungen und Prüffälle. Eine einfache Karten-UID gilt nicht automatisch als sicherer alleiniger Authentifizierungsnachweis. Ausbaustufen blockieren den ersten zentralen Login-Pilot nicht.

## 6. Ausfall-, Sicherheits- und Prüfkriterien

| Szenario | Zu definierendes / zu prüfendes Verhalten |
| --- | --- |
| Identity-LXC oder VPN nicht erreichbar | Funkbetrieb unabhängig; vorhandene Access Tokens nur innerhalb ihrer festgelegten Gültigkeit und mit verfügbaren vertrauenswürdigen Schlüsseln verwenden |
| Neuer Login / Token-Erneuerung bei Ausfall | Ohne erreichbaren Identity-Dienst nicht möglich; nur ausdrücklich eingerichteter lokaler Notzugang nach beschlossener Regel |
| AD ausgefallen | AD-Passwortlogin abhängig vom AD; keine Behauptung eines Offline-AD-Logins durch Benutzer-/JWKS-Cache |
| Station startet isoliert neu | Persistenz/Verfügbarkeit benötigter Vertrauensdaten und tatsächliches Sitzungs-/Notzugangsverhalten prüfen |
| Benutzer gesperrt / Rechte entzogen | Lokal geprüfte Tokens können alte Rechte bis zum Ablauf enthalten; maximal akzeptierte Sperrfrist festlegen; kritische Aktionen ggf. Onlineprüfung / aktuelle Policy |
| Token läuft während WebSocket-Verbindung ab | Reauthentifizierung, kontrolliertes Verbindungsende oder Verweigerung weiterer geschützter Aktionen; kein Dauerzugang durch offenen Socket |
| Signing-Key-Rotation | Übergang alter/neuer Schlüssel und JWKS-Cache kontrolliert; unbekannter Schlüssel ohne prüfbare Vertrauenskette wird abgelehnt |
| Zeitabweichung | Zeitsynchronisation und begrenzte Toleranz; abgelaufene Tokens nicht unbegrenzt weiter akzeptieren |
| Öffentliche Übersicht | Nur ausdrücklich freigegebene Daten; kein Zugriff auf Konfiguration, personenbezogene Details oder geschützte Dateien |
| Rollenverwaltung | Keine Selbsterhöhung außerhalb delegierter Rechte; Schutz des letzten administrativen / Notfallzugangs |
| Audit-Zentrale ausgefallen | Geschützte lokale Pufferung und Nachlieferung definieren; keine Passwörter, Tokens oder Rohschlüssel protokollieren |
| Notzugang | Individuell/gezielt provisioniert, gehasht gespeichert, eingeschränkt und auditierbar; kein stiller Standardpasswort-Fallback |
| Backups / Restore | Datenbank, Konfiguration und Schlüssel berücksichtigen; Gültigkeit alter Sitzungen und gewünschte Widerrufe nach Restore prüfen |
| Funk-/Betriebsregression | Laufende Vermittlung und betroffene Steuerfunktionen getrennt von UI-/Login-Tests überprüfen |

Ein gecachter öffentlicher Schlüssel ermöglicht Tokenprüfung, aber weder neue Anmeldung noch Verlängerung eines Tokens. Der initiale Vorschlag enthält keine unbeschränkte Offline-Anmeldung.

## 7. Risiken und Abhängigkeiten

- Unterschiedliche HTTP-/Transportimplementierungen können Adapter erforderlich machen; nicht allein einen Login-Proxy vor die Oberflächen setzen.
- Zentrale Anmeldung ohne Backend-RBAC würde direkte APIs und Langzeitverbindungen unzureichend schützen.
- Private lokale Netzparameter, AD-Daten und Zugangsdaten gehören nicht in öffentliche Roadmap- oder Wiki-Dateien.
- Ressourcenrechte, große Claim-Mengen und Rechteänderungen benötigen ein versioniertes, überprüfbares Modell.
- Identity- und Datenbankbetrieb erzeugen einen neuen Wartungsbedarf. Gemeinsamer Betrieb mit Deployment vergrößert den gemeinsamen Ausfallbereich.
- Automatisierte Client-/Secret-Provisionierung hängt vom tatsächlichen Discovery-/Deployment-Stand ab; derzeit nicht als fertig voraussetzen.
- NFC/RFID, MFA und AD sind getrennte Anforderungen; eine Anbindung bedeutet nicht automatisch sichere Kartenanmeldung.
- Die Umsetzung muss zur bestehenden WebUI-Vereinheitlichung passen. Eine eigene IAM-Oberfläche ist ein zusätzlicher Arbeitsschritt.

## 8. Aufnahme in regelmäßige Projektstatusläufe

Diese Datei ist die kanonische Planung für NETCORE-IAM-01. Beim nächsten und bei späteren Projektstatusläufen:

1. Aktuelle Version auf main lesen und mit dem letzten erfolgreich berichteten Stand vergleichen.
2. Als Schwerpunkt **Zentrale Anmeldung / RBAC** in der kurzen Roadmap und der vollständigen Wiki-Roadmap aufnehmen; bei knapper Schwerpunktzahl nach belegter Priorität einordnen.
3. Status je Meilenstein am aktuellen Code, PRs und dokumentierten Tests / Betriebsergebnissen prüfen. Ein Dokumentationscommit allein ist keine Implementierung.
4. Empfehlungen, bestätigte Entscheidungen, Entwicklungsbranch, main-Implementierung und verifizierten Betrieb auseinanderhalten.
5. Nächsten konkreten Schritt, Abhängigkeiten, Blocker und offene Entscheidungen berichten.
6. Historie erhalten; erledigte Ziele nicht spurlos löschen. Keine erfundenen Termine, Releases oder Fertigstellungsprozente.
7. Handbücher bei tatsächlichem Aktualisierungsbedarf nach den bestehenden Ausgaberegeln ergänzen. Geplante Funktionen ausdrücklich als geplant beschreiben.
8. Produktivsoftware nicht allein aufgrund dieser Roadmap verändern; bestehende Freigabe- und Branchregeln des jeweiligen Arbeitsauftrags beachten.

**Startstatus für den nächsten Lauf:** Idee und Roadmap dokumentiert; Architekturentscheidung offen; erste Teilbefunde vorhanden; noch kein Identity-LXC, kein gemeinsames Auth-Modul und keine durch diesen Auftrag migrierte Station nachgewiesen.

## 9. Änderungsverlauf

| Datum | Änderung | Implementierungsnachweis |
| --- | --- | --- |
| 2026-10-03 | Nutzeridee in Roadmap überführt; Zielarchitektur als Empfehlung, Meilensteine, Rechte-, Ausfall- und Prüfkriterien dokumentiert | Dokumentation; keine technische Umsetzung |
| 2026-10-05 | Ökosystemweite Anbindung um NetCore Drive ergänzt; zentrale Dienstrollen, fachliche Datei- / Ordnerrechte, Gäste, Linkfreigaben und begrenzte Dienstidentitäten konkretisiert | Dokumentation; keine technische Umsetzung |
| 2026-10-05 | Lokalen Drive-Anfangsbetrieb ohne zentralen IAM-Dienst eingeplant; spätere geprüfte Migration in D6 von automatischer Ausfallumschaltung getrennt | Dokumentation; keine technische Umsetzung |

## 10. Technische Referenzen

Die folgenden Primärquellen begründen die empfohlenen Integrationsmöglichkeiten; sie belegen keine Umsetzung in NetCore.

- [Keycloak Server Administration](https://www.keycloak.org/docs/latest/server_admin/index.html): lokale Benutzer, LDAP/AD, Rollen, Gruppen, Sessions und Dienstkonten.
- [Keycloak OpenID Connect](https://www.keycloak.org/securing-apps/oidc-layers): Authorization Code, Client Credentials, JWKS und lokale Tokenprüfung.
- [OpenID Connect Discovery 1.0](https://openid.net/specs/openid-connect-discovery-1_0.html): vertrauenswürdiger Issuer und Metadaten.
- [NetCore Backend-WebUI-Standard](../design/dienstoberflaechen-gestaltungsstandard.md) und [Dienstmatrix](../design/dienstoberflaechen-funktionsmatrix.md): vorhandene Projektvorgaben.
