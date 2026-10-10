# Brainstorming: NetCore-Tetra-/ZKN-Webseitenstruktur, Subdomain und Portale

**Archivhinweis, 09.10.2026:** Diese Datei dokumentiert das im Titel beziehungsweise Quellenrahmen genannte Gespräch und dessen damalige Entscheidungen. Historische Kommandos, Planungen, Versionsangaben und Fehlerbilder bleiben erhalten; der heutige technische Stand steht in der [Gesamtroadmap](../roadmaps/gesamtroadmap.md) und in den [aktuellen Dienstanleitungen](../services/README.md). Die Archivaufnahme bestätigt keine spätere Implementierung oder Live-Abnahme.

## Zielbild und Festlegungen

- Website X5 Pro, Hauptauftritt **netcore-tetra.de**, zusätzliche Domain **netcore-tetra.com**.
- ZKN erhält den eigenen Host **zkn.netcore-tetra.de**; die Pfadvariante `/zkn/` ist verworfen.
- Öffentliche Inhalte und geschützte Mitglieder-/Partner-/Adminbereiche sind als Informationsarchitektur geplant.
- Offen: Rolle der .com-Domain, X5-Projektaufteilung, Hostrouting/TLS, serverseitige Berechtigungen und Deployment.

## 1. Arbeitsstand

| Feld | Wert |
|---|---|
| Projekt | NetCore-Tetra |
| Thema | Öffentliche Website für NetCore-Tetra und ZKN, Informationsarchitektur, ZKN-Subdomain, geschützte Bereiche |
| Erstellungsdatum der Projektnotizen | 2026-10-04 |
| Repository | JanHG98/netcore-tetra |
| Zielbranch der Dokumentation | Archiving |
| Geprüfter Archiving-Stand unmittelbar vor dem Schreiben | ed84e9bf3922781e130f5adf3417afadec828175 |
| Geprüfter main-Stand | 7137e0dd69877e1b604bf89148fd8b6b590c1a97 |
| main-Commit zum Prüfzeitpunkt | Merge pull request #59 – „NetCore-Design mit Dark Mode für Basisstation und alle Dienst-WebUIs“ |
| Geplanter Archivpfad | Docs/archive/2026-10-04_netcore-tetra-zkn-webseitenstruktur-subdomain-und-portale.md |
| Relevante Anhänge | Keine themenspezifischen Anhänge wurden in diesem Verlauf verwendet. Die im Projektkontext verfügbaren ETSI-PDFs waren für die Webseiten-Informationsarchitektur nicht erforderlich und wurden für diese Dokumentation nicht geprüft. |

### Statuslegende

Diese Notizen unterscheiden strikt zwischen den folgenden Zuständen:

- **Idee** – diskutierter Vorschlag ohne verbindliche Festlegung.
- **Beschlossen/geplant** – in der Planung als Ziel oder Struktur festgelegt, aber nicht als technische Umsetzung nachgewiesen.
- **Implementiert** – im Repository oder in einem realen System nachweisbar umgesetzt.
- **Getestet** – durch einen dokumentierten Test überprüft.
- **Im Betrieb bestätigt** – in einer realen Betriebsumgebung nachweislich aktiv und erfolgreich genutzt.

Für die in dieser Entwicklungsphase behandelte öffentliche Website gilt: **Die Informationsarchitektur wurde geplant; eine Implementierung, ein Deployment oder ein Live-Test wurde in der Planung nicht nachgewiesen.**

---

## 2. Ziel und Ausgangslage

Ziel war die Planung eines öffentlichen Webauftritts für NetCore-Tetra und ZKN.

Ausdrücklich fest vorgegeben wurden:

- Verwendung von **Website X5 Pro** zur Erstellung der Website.
- Domains **netcore-tetra.de** und **netcore-tetra.com**.
- NetCore-Tetra als primärer Webauftritt.
- ZKN als eigener Bereich beziehungsweise eigener Auftritt.
- Ein öffentlicher und ein geschützter Bereich.
- Eine Seitenstruktur, die sich in Website X5 Pro direkt als „Seite / Ordner / Seite im Ordner“ abbilden lässt.

Im Verlauf wurde zunächst diskutiert, ob ZKN als Unterverzeichnis der Hauptdomain oder als Subdomain betrieben werden soll. Festgelegt wurde:

> ZKN soll als **Subdomain** betrieben werden.

Der in der Planung festgelegte Zielhost lautet:

**zkn.netcore-tetra.de**

Damit wurde die frühere Variante **netcore-tetra.de/zkn/** als bevorzugter Ansatz verworfen.

---

## 3. Behandelte Themen

Arbeitsfelder der Website-Planung:

1. Grundstruktur der öffentlichen NetCore-Tetra-Website.
2. Trennung zwischen öffentlichem und geschütztem Inhalt.
3. Rollenmodell für geschützte Inhalte.
4. Eigenständige ZKN-Präsenz.
5. Entscheidung Subdomain versus Unterverzeichnis.
6. DNS-Grundidee mit CNAME für ZKN.
7. SSL/TLS für Hauptdomain und Subdomain.
8. Aufteilung in ein oder mehrere Website-X5-Projekte.
9. SEO-/Sitemap-/Search-Console-Grundlagen.
10. Konkrete Menüstrukturen für NetCore-Tetra.
11. Konkrete Menüstrukturen für ZKN.
12. Abgrenzung zwischen Website-Planung und bereits vorhandenen internen NetCore-WebUIs.

---

## 4. Endgültige Anforderungen und Entscheidungen

### 4.1 Domains

**Beschlossen/geplant**

- Primäre Projekt-Domain: **netcore-tetra.de**
- Zusätzlich vorhandene beziehungsweise vorgesehene Domain: **netcore-tetra.com**
- ZKN-Zielhost: **zkn.netcore-tetra.de**

Für die .com-Domain wurde in der Planung **noch keine endgültige Rolle festgelegt**.

Offene Varianten für netcore-tetra.com:

- identischer Inhalt wie .de,
- permanente Weiterleitung auf .de,
- spätere englischsprachige internationale Website,
- separate Landingpage.

**Empfehlung für die spätere Umsetzung:** einen kanonischen Host festlegen und Doppelindexierung vermeiden. Wenn .de die Hauptdomain bleibt, ist eine HTTP-301-Weiterleitung von .com auf die entsprechende .de-URL der einfachste konsistente Ansatz.

### 4.2 ZKN als Subdomain

**Beschlossen/geplant**

ZKN wird nicht als normaler Pfad unter NetCore-Tetra geplant, sondern als eigener Host:

**zkn.netcore-tetra.de**

Begründung aus den Arbeitsnotizen:

- stärkere organisatorische und optische Trennung,
- Möglichkeit eines eigenen Designs,
- Möglichkeit einer eigenständigen Navigation,
- spätere technische Trennung des Hostings möglich,
- ZKN kann als eigenständige organisatorische Einheit auftreten, bleibt aber erkennbar mit NetCore-Tetra verbunden.

### 4.3 DNS-Grundidee

**Beschlossen/geplant, nicht umgesetzt bestätigt**

In der Planung wurde ein CNAME vorgeschlagen:

- Name/Host: zkn
- Typ: CNAME
- Ziel: netcore-tetra.de beziehungsweise der vom Hoster vorgesehene kanonische Zielhost

Wichtige technische Präzisierung:

Ein CNAME erzeugt **nur die DNS-Zuordnung**. Er erzeugt weder automatisch eine zweite Website noch ein getrenntes Webroot. Der Webserver oder das Hosting-Panel muss den Hostnamen **zkn.netcore-tetra.de** ausdrücklich einem eigenen Projekt beziehungsweise Dokumentenstamm zuordnen.

Falls der Hostinganbieter einen speziellen CNAME-Zielhost vorgibt, ist dieser in der Regel gegenüber einem bloßen Alias auf netcore-tetra.de zu bevorzugen.

### 4.4 TLS

**Beschlossen/geplant, nicht umgesetzt bestätigt**

Für HTTPS wurde vorgeschlagen:

- Zertifikat mit netcore-tetra.de und zkn.netcore-tetra.de als SANs, oder
- Wildcard-Zertifikat für *.netcore-tetra.de.

Zu beachten:

- Ein Wildcard-Zertifikat für *.netcore-tetra.de deckt die Subdomain zkn.netcore-tetra.de ab.
- Die Apex-Domain netcore-tetra.de muss je nach Zertifikat zusätzlich explizit enthalten sein.
- netcore-tetra.com benötigt ein eigenes Zertifikat beziehungsweise muss ebenfalls als SAN enthalten sein, wenn es aktiv HTTPS ausliefert.

### 4.5 Website X5 Pro

**Beschlossen:** Website X5 Pro ist das verwendete Autorensystem.

**Idee/Empfehlung:** NetCore-Tetra und ZKN als zwei getrennte Website-X5-Projekte führen.

Dies wurde als sinnvoll bezeichnet, weil ZKN als Subdomain ein eigenes Layout und einen eigenen Uploadpfad erhalten kann. Die Subdomain ist festgelegt; **nicht ausdrücklich festgelegt**, dass zwingend zwei getrennte X5-Projektdateien verwendet werden müssen.

---

## 5. Endgültige Informationsarchitektur – NetCore-Tetra

Die folgende Struktur wurde in der Planung als direkte „Seite / Ordner / Seite im Ordner“-Struktur ausgearbeitet.

### 5.1 Öffentlicher Bereich

**Beschlossen/geplant als Website-Struktur**

Startseite

Plattform\
&nbsp;&nbsp;&nbsp;&nbsp;Überblick\
&nbsp;&nbsp;&nbsp;&nbsp;Funktionen\
&nbsp;&nbsp;&nbsp;&nbsp;Hardware\
&nbsp;&nbsp;&nbsp;&nbsp;Sicherheit & Betrieb\
&nbsp;&nbsp;&nbsp;&nbsp;Roadmap\

Lösungen\
&nbsp;&nbsp;&nbsp;&nbsp;Einsatzszenarien\
&nbsp;&nbsp;&nbsp;&nbsp;Use-Cases\
&nbsp;&nbsp;&nbsp;&nbsp;Branchenlösungen\
&nbsp;&nbsp;&nbsp;&nbsp;Partner-Integrationen\

Ressourcen\
&nbsp;&nbsp;&nbsp;&nbsp;Dokumentation (öffentlich)\
&nbsp;&nbsp;&nbsp;&nbsp;Presse & Brandmaterial\
&nbsp;&nbsp;&nbsp;&nbsp;Changelog & News\
&nbsp;&nbsp;&nbsp;&nbsp;FAQ\

Über uns\
&nbsp;&nbsp;&nbsp;&nbsp;Team / Vision / Mission\
&nbsp;&nbsp;&nbsp;&nbsp;Kontakt\
&nbsp;&nbsp;&nbsp;&nbsp;Karriere & Community\

Kontakt / Demo

Rechtliches\
&nbsp;&nbsp;&nbsp;&nbsp;Impressum\
&nbsp;&nbsp;&nbsp;&nbsp;Datenschutz\
&nbsp;&nbsp;&nbsp;&nbsp;AGB (optional)\
&nbsp;&nbsp;&nbsp;&nbsp;Cookie-Hinweis\

### 5.2 Geschützter Bereich NetCore-Tetra

**Beschlossen/geplant als Informationsarchitektur; Authentisierung und Berechtigungsmodell nicht implementiert oder getestet**

Mitgliederportal\
&nbsp;&nbsp;&nbsp;&nbsp;Downloads\
&nbsp;&nbsp;&nbsp;&nbsp;Detail-Dokumentation\
&nbsp;&nbsp;&nbsp;&nbsp;How-To Videos\
&nbsp;&nbsp;&nbsp;&nbsp;Technischer Changelog & Release-Notes\
&nbsp;&nbsp;&nbsp;&nbsp;Bug-Meldung\

Partnerportal\
&nbsp;&nbsp;&nbsp;&nbsp;Erweiterte Dokus\
&nbsp;&nbsp;&nbsp;&nbsp;Projektvorlagen & SOPs\
&nbsp;&nbsp;&nbsp;&nbsp;Lizenz-/Key-Bereich\
&nbsp;&nbsp;&nbsp;&nbsp;Marketing-Vorlagen\
&nbsp;&nbsp;&nbsp;&nbsp;Beta-Releases & Roadmap-Details\

Adminportal\
&nbsp;&nbsp;&nbsp;&nbsp;API-Detail & interne Protokolle\
&nbsp;&nbsp;&nbsp;&nbsp;RolloutCenter\
&nbsp;&nbsp;&nbsp;&nbsp;Watchtower-Dashboards & Playbooks\
&nbsp;&nbsp;&nbsp;&nbsp;Partner-Accountverwaltung\
&nbsp;&nbsp;&nbsp;&nbsp;Pitch/Docs Masterfiles\

### 5.3 Rollenmodell NetCore-Tetra

In der Planung vorgeschlagen:

- **Member** – registrierte Nutzer.
- **Partner** – erweiterte Partnerrechte.
- **Admin** – interne Nutzung.

**Status: beschlossen/geplant als Website-Idee, nicht implementiert.**

### 5.4 Sicherheitskorrektur zu geschützten Downloads

Ein früher Entwurf nannte unter anderem Lizenz-/Keys, Signaturen und interne Deployment-Inhalte als mögliche Portalinhalte. Das ist nur mit sauberer Sicherheitsarchitektur sinnvoll.

Für die spätere Umsetzung gilt:

- Keine privaten Signierschlüssel auf der Website ablegen.
- Keine Tokens, Passwörter, privaten Schlüssel oder produktiven Secrets über statische Downloadbereiche verteilen.
- Öffentliche Prüfsummen und öffentliche Signaturen können veröffentlicht werden.
- Sensible Artefakte benötigen echte serverseitige Authentisierung, Autorisierung, Logging und gegebenenfalls zeitlich begrenzte Downloads.
- Ein einfacher Verzeichnisschutz oder rein clientseitiges „Verstecken“ einer X5-Seite reicht für vertrauliche Daten nicht aus.
- Admin-Dashboards sollten bevorzugt auf interne Managementendpunkte verlinken beziehungsweise über SSO/VPN/Reverse-Proxy-Policies geschützt werden, statt sensible Betriebsdaten direkt in eine öffentliche Website einzubetten.

---

## 6. Endgültige Informationsarchitektur – ZKN

ZKN wird als eigener Auftritt auf **zkn.netcore-tetra.de** geplant.

### 6.1 Öffentlicher Bereich ZKN

**Beschlossen/geplant als Website-Struktur**

Startseite

Aufgaben & Mission\
&nbsp;&nbsp;&nbsp;&nbsp;Wer wir sind\
&nbsp;&nbsp;&nbsp;&nbsp;Was wir tun\
&nbsp;&nbsp;&nbsp;&nbsp;Werte & Arbeitsweise\

Lösungen & Module\
&nbsp;&nbsp;&nbsp;&nbsp;Leitstellenstruktur\
&nbsp;&nbsp;&nbsp;&nbsp;BOS-/KRITIS-Bridge\
&nbsp;&nbsp;&nbsp;&nbsp;Watchtower-Integration\
&nbsp;&nbsp;&nbsp;&nbsp;Spezialeinsätze\

Referenzen & Projekte

Kooperation & Partner

Ressourcen (öffentlich)\
&nbsp;&nbsp;&nbsp;&nbsp;Pressebereich\
&nbsp;&nbsp;&nbsp;&nbsp;Whitepaper „ZKN Operations“\
&nbsp;&nbsp;&nbsp;&nbsp;Changelog & News\

Kontakt & Support

Rechtliches\
&nbsp;&nbsp;&nbsp;&nbsp;Impressum\
&nbsp;&nbsp;&nbsp;&nbsp;Datenschutz\
&nbsp;&nbsp;&nbsp;&nbsp;Cookie-Hinweis\

### 6.2 Geschützter Bereich ZKN

Partner-Portal\
&nbsp;&nbsp;&nbsp;&nbsp;Einsatzhandbücher\
&nbsp;&nbsp;&nbsp;&nbsp;Schnittstellenbeschreibung BOS-/KRITIS-Bridge\
&nbsp;&nbsp;&nbsp;&nbsp;Notfallprotokolle (generisch)\
&nbsp;&nbsp;&nbsp;&nbsp;ZKN-Marketingmaterial\
&nbsp;&nbsp;&nbsp;&nbsp;Monitoring-Dashboards\

Intern-Portal\
&nbsp;&nbsp;&nbsp;&nbsp;Einsatz- & Notfallprotokolle (vollständig)\
&nbsp;&nbsp;&nbsp;&nbsp;Watchtower-Profile & Alarmmatrizen\
&nbsp;&nbsp;&nbsp;&nbsp;OTA-Management-Tools & Deployment-Pakete\
&nbsp;&nbsp;&nbsp;&nbsp;Checklisten & SOPs\
&nbsp;&nbsp;&nbsp;&nbsp;Incident-Datenbank\
&nbsp;&nbsp;&nbsp;&nbsp;Bereitschafts- & Wartungskalender\

### 6.3 Status der ZKN-Inhalte

Die Seitenstruktur wurde als Webkonzept akzeptiert. Die Produkt-/Betriebsbegriffe sind überwiegend Entwurfsvorschläge, keine geprüften Module.

Daher ist zu unterscheiden:

- **Beschlossen/geplant:** ZKN bekommt einen eigenständigen Subdomain-Auftritt und die oben genannte Menüstruktur als Arbeitsgrundlage.
- **Nicht als Implementierung bestätigt:** BOS-/KRITIS-Bridge, Watchtower-Integration, OTA-Management, Incident-Datenbank, Bereitschaftskalender und ähnliche Module.
- **Nicht als Repository-Nachweis bestätigt:** Die Bezeichnungen „Watchtower“, „RolloutCenter“ und „Lighthouse“ ergaben beim geprüften Repository-Abgleich keine exakten Treffer auf main.
- **Nicht bestätigt:** der im Entwurf verwendete Slogan „netzweit. sicher. bereit.“. Er wurde vorgeschlagen, aber noch nicht ausdrücklich als endgültiger Claim freigegeben.

---

## 7. Architektur und Abhängigkeiten

### 7.1 Logische Webarchitektur

Vorgesehene Trennung:

1. **NetCore-Tetra Public Site**
   - Host: netcore-tetra.de
   - Zweck: Produkt-/Projektinformation, Lösungen, Ressourcen, Kontakt.

2. **NetCore-Tetra Protected Portal**
   - gleicher Host oder eigener geschützter Bereich unter netcore-tetra.de,
   - Rollen: Member, Partner, Admin.

3. **ZKN Public Site**
   - Host: zkn.netcore-tetra.de,
   - eigener Informations- und Markenauftritt.

4. **ZKN Protected Portal**
   - Partner-Portal,
   - internes Portal.

### 7.2 DNS

Relevantes Protokoll:

- DNS
- CNAME für zkn wurde als bevorzugter Mechanismus genannt.

Konkrete DNS-Werte wurden in der Planung nicht real im DNS-Provider angelegt und nicht getestet.

### 7.3 HTTP/HTTPS

Erwartete Standardports:

- TCP 80 für HTTP, vorzugsweise nur Redirect auf HTTPS.
- TCP 443 für HTTPS.

Diese Ports ergeben sich aus dem Webhosting-Modell; eine konkrete Firewall- oder Reverse-Proxy-Konfiguration wurde in der Planung nicht eingerichtet.

### 7.4 TLS-Zertifikate

Abhängigkeiten:

- Zertifikatsausstellung durch den Hostinganbieter oder ACME/Let’s Encrypt.
- Hostname muss beim Webserver bekannt sein.
- DNS muss auf den korrekten Server zeigen.

### 7.5 Website-X5-Projektstruktur

Empfohlene, aber noch nicht abschließend bestätigte Betriebsform:

- Projekt A: NetCore-Tetra
- Projekt B: ZKN
- getrennte Export-/Uploadziele.

Ein einzelnes Projekt mit externem Link zur ZKN-Subdomain bleibt technisch möglich, wurde aber nicht als bevorzugter Weg weiterverfolgt.

---

## 8. Erreichter Entwicklungs- und Betriebsstand

### Website-Planung

**Beschlossen/geplant**

- NetCore-Tetra-Sitemap vorhanden.
- ZKN-Sitemap vorhanden.
- Trennung öffentlich/geschützt definiert.
- ZKN als Subdomain festgelegt.
- CNAME als DNS-Ansatz vorgesehen.
- SSL/TLS berücksichtigt.
- Website X5 Pro als Authoring-Tool festgelegt.

### Implementierung

**Nicht nachgewiesen**

In der Planung wurden keine der folgenden Tätigkeiten nachweislich durchgeführt:

- DNS-Eintrag erstellt,
- Webspace/Subdomain im Hosting angelegt,
- SSL-Zertifikat ausgestellt,
- Website-X5-Projektdatei erzeugt,
- Seiten in X5 angelegt,
- HTML/CSS/JS exportiert,
- Website auf Server hochgeladen,
- Login-/Rollenfunktion eingerichtet,
- Redirects eingerichtet,
- Sitemap veröffentlicht,
- Search Console konfiguriert.

### Getestet

**Nein.**

Es wurden keine DNS-, TLS-, HTTP-, Login-, Rollen-, SEO-, Redirect- oder Cross-Browser-Tests dokumentiert.

### Im Betrieb bestätigt

**Nein.**

Für die neue öffentliche Website beziehungsweise die ZKN-Subdomain liegt in dieser Entwicklungsphase keine Betriebsbestätigung vor.

---

## 9. Geprüfter Repository-Abgleich

### 9.1 main

Geprüfter Stand:

**7137e0dd69877e1b604bf89148fd8b6b590c1a97**

Commit:

**Merge pull request #59 from JanHG98/feat/netcore-dashboard-design – NetCore-Design mit Dark Mode für Basisstation und alle Dienst-WebUIs**

Das Repository enthält damit nachweislich interne beziehungsweise dienstbezogene WebUIs. Diese dürfen nicht automatisch mit der hier geplanten öffentlichen Marketing-/Projektwebsite gleichgesetzt werden.

### 9.2 Suche nach Website-spezifischer Umsetzung

Auf main wurden unter anderem folgende exakten Suchbegriffe geprüft:

- zkn.netcore-tetra.de
- netcore-tetra.com
- Website X5 Pro
- NetCore-Tetra website
- Partner-Portal
- netcore-tetra.de
- Watchtower
- RolloutCenter
- Lighthouse

Für diese exakten Website-/Produktbegriffe wurden keine einschlägigen Treffer gefunden.

Eine Suche nach „zkn“ lieferte Treffer, die tatsächlich Bestandteil deutscher Wörter wie „Netzknoten“ waren und **keinen Nachweis für eine ZKN-Webimplementierung** darstellen.

### 9.3 Archiving

Unmittelbar vor dem Schreiben dieser Datei lag der Branch Archiving auf:

**ed84e9bf3922781e130f5adf3417afadec828175**

Der Zielpfad dieser Notizen existierte zu diesem Zeitpunkt noch nicht.

### 9.4 Ergebnis des Abgleichs

Die Website-Planung aus dieser Entwicklungsphase ist als **Konzept und Roadmap-Kandidat** zu behandeln.

Es gibt im geprüften Repository-Stand keinen belastbaren Nachweis, dass:

- die öffentliche NetCore-Tetra-Website aus dieser Entwicklungsphase bereits implementiert ist,
- die ZKN-Subdomain bereits eingerichtet ist,
- die hier entworfenen Portalrollen technisch existieren,
- die konkreten Seitenstrukturen bereits als Website-X5-Projekt vorliegen.

---

## 10. Wichtige Konfigurationen und technische Parameter

| Bereich | Wert/Stand | Status |
|---|---|---|
| Hauptdomain | netcore-tetra.de | beschlossen/geplant |
| Zweitdomain | netcore-tetra.com | vorhanden/vorgesehen, Rolle offen |
| ZKN-Host | zkn.netcore-tetra.de | beschlossen/geplant |
| DNS für ZKN | CNAME | beschlossen/geplant, Zielhost providerabhängig |
| DNS TTL | 3600 s wurde vorgeschlagen | Idee, nicht festgelegt |
| HTTPS | erforderlich/empfohlen | beschlossen/geplant |
| TLS | Wildcard oder SAN-Zertifikat | Idee/Empfehlung |
| Authoring | Website X5 Pro | beschlossen |
| NetCore Rollen | Member / Partner / Admin | geplant |
| ZKN Rollen | Partner / Intern | geplant |
| HTTP | TCP 80, vorzugsweise Redirect | technisch zu planen |
| HTTPS | TCP 443 | technisch zu planen |
| Sitemap | separat je Host sinnvoll | geplant |
| Search Console | separate Property für ZKN vorgeschlagen | Idee/Empfehlung |

---

## 11. Wichtige Abläufe

### 11.1 ZKN-Subdomain einrichten

**Nur vorgeschlagener Ablauf – nicht in der Planung ausgeführt**

1. Subdomain zkn beim DNS-/Hostinganbieter anlegen.
2. Passenden CNAME setzen.
3. Hostnamen im Webhosting auf eigenes Webroot oder eigenes X5-Uploadziel mappen.
4. TLS-Zertifikat für zkn.netcore-tetra.de ausstellen.
5. HTTP auf HTTPS umleiten.
6. ZKN-Website aus dem vorgesehenen X5-Projekt veröffentlichen.
7. Erreichbarkeit und Zertifikatskette testen.
8. sitemap.xml und robots.txt prüfen.
9. ZKN als eigene Search-Console-Property eintragen, falls gewünscht.

### 11.2 Getrennte X5-Projekte

**Nur vorgeschlagen**

NetCore-Tetra und ZKN können als zwei X5-Projekte geführt werden.

Vorteile:

- getrennte Navigation,
- getrenntes Design,
- getrennte Veröffentlichungsziele,
- geringere Gefahr, versehentlich ZKN-Seiten in die Hauptnavigation zu mischen.

Nachteile:

- doppelte Pflege gemeinsamer Footer-/Legal-Inhalte,
- Branding-Komponenten müssen synchron gehalten werden,
- Sprachversionen erhöhen den Pflegeaufwand zusätzlich.

### 11.3 Geschützte Bereiche

**Nur vorgeschlagen**

Vor produktiver Nutzung muss festgelegt werden:

- wer Benutzer anlegt,
- wie Passwörter gespeichert werden,
- ob MFA erforderlich ist,
- ob Partner- und Adminbereiche getrennte Authentisierungswege erhalten,
- ob SSO/LDAP/AD/OIDC angebunden wird,
- wie Sitzungen invalidiert werden,
- wie Downloadberechtigungen serverseitig geprüft werden,
- wie Zugriffe protokolliert werden.

Die Planung hat diese Punkte nicht technisch umgesetzt.

---

## 12. Fehler, Diagnose und Lösungen

### 12.1 Technischer Umsetzungsstatus

Es traten keine konkreten Laufzeit-, Build-, DNS- oder Hostingfehler auf, weil keine technische Implementierung durchgeführt wurde.

### 12.2 Konzeptuelle Korrekturen

#### Unterverzeichnis versus Subdomain

Früher diskutiert:

- netcore-tetra.de/zkn/

Später ausdrücklich ersetzt durch:

- zkn.netcore-tetra.de

**Planungsstand: Subdomain ist die verbindliche Planungsrichtung.**

#### SEO-Begründung

Die frühere Aussage, eine Subdomain werde von Suchmaschinen grundsätzlich wie eine komplett eigene Domain behandelt, ist zu pauschal.

Korrekte Einordnung:

- Subdomains sind technisch eigenständige Hosts.
- Sie können getrennte Sitemaps und Search-Console-Properties besitzen.
- Suchmaschinen können Zusammenhänge zwischen Hauptdomain und Subdomain erkennen.
- Die Entscheidung für die Subdomain sollte deshalb primär auf Marken-, Organisations- und Betriebsgründen beruhen, nicht auf einem garantierten SEO-Effekt.

#### CNAME

Die frühere Formulierung „CNAME zkn auf netcore-tetra.de und fertig“ wäre unvollständig.

Zusätzlich erforderlich:

- Hostrouting im Webserver/Hosting,
- eigenes Webroot oder korrekte Site-Zuordnung,
- TLS für den neuen Host.

---

## 13. Verworfene oder ersetzte Ansätze

### 13.1 ZKN als Unterverzeichnis

**Verworfen**

Variante:

netcore-tetra.de/zkn/

Grund:

Festgelegt ist die eigenständigere Darstellung als Subdomain.

### 13.2 ZKN vollständig auf derselben Site ohne getrennten Host

**Nicht weiterverfolgt**

Die Option hätte die Pflege vereinfacht, aber weniger organisatorische Trennung geboten.

### 13.3 Ein einziges Website-X5-Projekt

**Nicht endgültig verworfen, aber nicht bevorzugt**

Die Subdomain macht zwei getrennte Projekte organisatorisch sinnvoll. Eine endgültige technische Entscheidung über die X5-Projektdateien steht noch aus.

---

## 14. Frühere Entwurfsvorschläge ohne belastbare Bestätigung

Im Verlauf wurden mehrere Begriffe beziehungsweise Funktionsnamen in Seitenentwürfen verwendet. Sie dürfen nicht ohne weiteren Abgleich als vorhandene NetCore-Tetra-Produkte oder produktive Dienste dargestellt werden.

Dazu gehören insbesondere:

- Mesh-VPN
- Shadow-GSSI
- Watchtower
- RolloutCenter
- Lighthouse
- BOS-/KRITIS-Bridge
- Predictive Monitoring
- OTA-Management als ZKN-Funktion
- bestimmte Support-Mailadressen
- der ZKN-Slogan „netzweit. sicher. bereit.“

Diese Begriffe sind für die Website **Ideen beziehungsweise redaktionelle Platzhalter**, sofern sie nicht in anderen Projektteilen separat bestätigt und implementiert sind.

Der zusätzliche Code-Suchabgleich auf main ergab für Watchtower, RolloutCenter und Lighthouse keine exakten Treffer.

---

## 15. Offene Aufgaben und Roadmap-Kandidaten

### Priorität A – Hosting- und Domainbasis

1. **Kanonische Domain festlegen**
   - Entscheidung, ob netcore-tetra.de der alleinige kanonische Hauptauftritt ist.
   - Rolle von netcore-tetra.com festlegen.
   - Redirectstrategie definieren.

2. **DNS-Provider und Hostingmodell dokumentieren**
   - konkreten CNAME-Zielhost bestimmen,
   - TTL festlegen,
   - Webroot-Zuordnung für zkn definieren.

3. **TLS-Strategie festlegen**
   - Wildcard oder SAN,
   - automatische Verlängerung,
   - .com berücksichtigen.

### Priorität A – Informationssicherheit

4. **Geschützte Inhalte klassifizieren**
   - öffentlich,
   - Member,
   - Partner,
   - intern,
   - streng intern/secrets – nicht über die Website verteilen.

5. **Authentisierungsmodell festlegen**
   - X5 Access Management nur dann verwenden, wenn das Sicherheitsniveau ausreicht,
   - alternativ Reverse Proxy, OIDC/SSO oder eigenes Portal.

6. **Private Schlüssel und Secrets explizit ausschließen**
   - keine privaten Signierschlüssel,
   - keine API-Tokens,
   - keine produktiven Passwörter,
   - keine internen Zertifikat-Private-Keys.

### Priorität B – Website X5

7. Zwei getrennte X5-Projekte anlegen oder die Ein-Projekt-Variante bewusst bestätigen.
8. NetCore-Tetra-Sitemap 1:1 anlegen.
9. ZKN-Sitemap 1:1 anlegen.
10. Header/Footer/Navigation zwischen beiden Projekten konsistent gestalten.
11. Rückverlinkung „ZKN ↔ NetCore-Tetra“ definieren.

### Priorität B – Inhalte

12. Elevator-Pitch und Startseitentexte finalisieren.
13. Produktbegriffe gegen den echten Repository-/Betriebsstand prüfen.
14. ZKN-Aufgaben und tatsächliche Zuständigkeiten fachlich verifizieren.
15. Öffentliche versus interne Dokumentation abgrenzen.
16. Presse-/Brandbereich mit freigegebenen Logos und Nutzungsregeln ausstatten.
17. Referenzen nur veröffentlichen, wenn Freigabe und Datenschutz geklärt sind.

### Priorität C – SEO und Betrieb

18. Titles und Meta-Descriptions erstellen.
19. sitemap.xml je Host prüfen.
20. robots.txt je Host prüfen.
21. Canonical-Tags und Redirects testen.
22. Search Console für Hauptdomain und Subdomain einrichten.
23. 404-/Fehlerseiten definieren.
24. Performance/Bildoptimierung, WebP/SVG und Caching prüfen.
25. Monitoring der Website-Erreichbarkeit einrichten.

### Priorität C – Rechtliches

26. Impressum mit realem Betreiber abgleichen.
27. Datenschutztext an echte Formulare, Logs, Cookies und Analytics anpassen.
28. Cookie-/Consent-Lösung nur für tatsächlich eingesetzte Dienste konfigurieren.
29. AGB nur aufnehmen, wenn sie für reale Leistungen tatsächlich benötigt werden.

---

## 16. Tests, die vor Livegang erforderlich sind

Noch nicht durchgeführt.

### DNS

- A/AAAA/CNAME-Auflösung.
- Keine unerwünschten CNAME-Ketten.
- www/non-www Verhalten.
- .com Redirect.

### TLS

- gültige Zertifikatskette,
- Hostname-Match,
- automatische Verlängerung,
- keine Mixed-Content-Ressourcen.

### HTTP

- HTTP → HTTPS,
- Canonical Redirect,
- 404-Verhalten,
- Weiterleitungen ohne Schleifen.

### Access Control

- anonymer Zugriff auf geschützte URL muss scheitern,
- Member darf keine Partner-/Adminseite öffnen,
- Partner darf keine Adminseite öffnen,
- Logout invalidiert Sitzung,
- Direktlink auf Download darf Berechtigung nicht umgehen.

### Website X5

- Desktop,
- Tablet,
- Mobilgerät,
- Touch,
- Tastaturnavigation,
- Formularvalidierung,
- Spam-Schutz,
- Uploadpfade,
- Mehrsprachigkeit, falls aktiviert.

### SEO

- Sitemap erreichbar,
- robots.txt korrekt,
- Canonical korrekt,
- keine doppelten Inhalte zwischen .de und .com.

---

## 17. Relevante Repository-Stände und Quellen

### Repository

- Repository: https://github.com/JanHG98/netcore-tetra
- Branch Archiving: https://github.com/JanHG98/netcore-tetra/tree/Archiving
- main-Commit geprüft: https://github.com/JanHG98/netcore-tetra/commit/7137e0dd69877e1b604bf89148fd8b6b590c1a97
- PR aus dem geprüften main-Head: https://github.com/JanHG98/netcore-tetra/pull/59

### Arbeitsgrundlagen

Die inhaltliche Grundlage dieser Datei ist der verfügbare Planungsstand:

- Wunsch nach Website für NetCore-Tetra und ZKN,
- Website X5 Pro,
- Domains .de/.com,
- Entwurf der Hauptseiten,
- öffentlicher und geschützter Bereich,
- Diskussion Subdomain versus Unterverzeichnis,
- Entscheidung für Subdomain,
- DNS-/TLS-Vorschlag,
- finale NetCore-Tetra-Struktur,
- finale ZKN-Struktur.

### Bilder und Anhänge

- In dieser Entwicklungsphase wurden keine eigenständigen Bilder hochgeladen.
- Daher wurde kein Bildasset speziell für diese Planung in Docs/archive/ ergänzt.
- Projektweit verfügbare ETSI-PDFs sind nicht Teil dieser Webseitenplanung und wurden nicht dupliziert.

---

## 18. Zusammenfassung des am Prüfdatum vorliegenden Stands

Die wesentliche Festlegung dieser Planung ist die Trennung des öffentlichen Webauftritts in zwei Hosts:

- **netcore-tetra.de** für NetCore-Tetra,
- **zkn.netcore-tetra.de** für ZKN.

Für beide wurden öffentliche und geschützte Seitenstrukturen entworfen. Die Strukturen können in Website X5 Pro direkt als Seiten und Ordner angelegt werden.

Die Planung hat dagegen **keine tatsächliche Webimplementierung** erzeugt. DNS, Hosting, TLS, Login, Rollen, X5-Projektdateien und Livebetrieb bleiben umzusetzen und zu testen.

Die wichtigsten nächsten Entscheidungen sind:

1. netcore-tetra.com fest zuordnen beziehungsweise auf .de weiterleiten,
2. konkreten DNS-/Hosting-Zielhost für zkn festlegen,
3. X5-Einprojekt- versus Zweiprojektmodell final entscheiden,
4. geschützte Inhalte sicherheitstechnisch klassifizieren,
5. echte Produktnamen und ZKN-Funktionen gegen Repository und Betrieb verifizieren,
6. danach erst Inhalte erstellen und veröffentlichen.

**Arbeitsstand:** Informationsarchitektur, Subdomain und Portalbereiche sind geplant. Die technische Umsetzung und Abnahme stehen noch aus.
