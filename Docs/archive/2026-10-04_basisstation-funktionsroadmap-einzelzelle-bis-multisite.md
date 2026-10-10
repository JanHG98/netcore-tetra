# Brainstorming: Basisstations-Funktionsroadmap von der Einzelzelle zum Multi-Site-Netz

**Archivhinweis, 09.10.2026:** Diese Datei dokumentiert das im Titel beziehungsweise Quellenrahmen genannte Gespräch und dessen damalige Entscheidungen. Historische Kommandos, Planungen, Versionsangaben und Fehlerbilder bleiben erhalten; der heutige technische Stand steht in der [Gesamtroadmap](../roadmaps/gesamtroadmap.md) und in den [aktuellen Dienstanleitungen](../services/README.md). Die Archivaufnahme bestätigt keine spätere Implementierung oder Live-Abnahme.

**Stand der Notizen und ergänzenden Prüfungen: 2026-10-04.** Historische Entwürfe, nachgewiesene Umsetzung und ausgeführte Tests sind jeweils getrennt gekennzeichnet.

> **Zielbild:** Schrittweise vom ersten sendenden Standort mit belegter Geräteanmeldung zum Multi-Site-Netz. Der historische Entwurf enthält acht Phasen und 40 Schritte; Einzelentscheidungen und praktische Abnahmen sind noch nicht belegt. Der zusätzliche Repository-Abgleich stammt vom 04.10.2026.

## 1. Kontext und Einordnung

| Feld | Inhalt |
|---|---|
| Projekt | NetCore-Tetra |
| Thema | Ausschließlich Funktionen und Betriebsfähigkeit der TETRA-Basisstation samt unmittelbar benötigter Netzinfrastruktur |
| Ausgangsanforderung | Vollständige Basisstationsroadmap mit erstem Meilenstein Aussendung und Geräteanmeldung. |
| Historische Datierung | Entwurf ergänzend dem 18.10.2025 zugeordnet; vollständiger datierter Originalbestand fehlt. |
| Erstellungsdatum dieser Zusammenfassung | **2026-10-04**, Zeitzone Europe/Berlin |
| Repository | `JanHG98/netcore-tetra` |
| Geprüfter Branch | **`Archiving`** |
| Geprüfter Code-/Dokumentationsstand | **`530170d2deda13d531e5e200d0bde7ab97acbdcd`** |
| Root-Tree des geprüften Standes | `d9fbd0ada40ad58e5ddbae7d5f60446603505dc3` |
| Archivdatei | `Docs/archive/2026-10-04_basisstation-funktionsroadmap-einzelzelle-bis-multisite.md` |
| Archivindex | `Docs/archive/README.md` |

Die Roadmap gehört zur Funktionsplanung der Basisstation. Erstinstallation und Dual-Carrier-Ausbau sind als separate Entwicklungsphasen dokumentiert.

## 2. Quellenumfang, Belegstufen und Auswertungslücken

### 2.1 Tatsächlich ausgewerteter Verlauf

Die Grundlage ist eine Roadmap-Anforderung mit achtphasigem Entwurf und ergänzenden PDF-Quellen. Einzelne Technologieentscheidungen, Installationsausgaben, Testberichte und Funktionspatches fehlen.

Der Entwurf wurde als Zusammenführung zweier älterer Backups beschrieben. Diese Quellen konnten nicht eindeutig wiederhergestellt werden; die Vollständigkeit gegenüber sämtlichen früheren Ideen ist deshalb **nicht überprüfbar**. Erhalten sind alle Ideen des zugänglichen Entwurfs.

Repository-Dokumentation und thematische Nachbarn dienen als ergänzende Fortsetzungsquellen; ihr Implementierungsstand darf nicht rückwirkend auf den frühen Entwurf übertragen werden.

### 2.2 Statusbegriffe

| Status | Verwendung in diesem Archiv |
|---|---|
| **Idee** | Vorgeschlagen, jedoch nicht als technische Entscheidung bestätigt. |
| **Beschlossen/geplant** | Vorgegebenes Ziel oder ausdrücklich als Planung ausgewiesener Repository-Stand; keine Umsetzungsaussage. |
| **Implementiert** | Ein konkret gelesener Codepfad beziehungsweise Build-Baustein ist im geprüften Commit vorhanden. Der jeweils nachgewiesene Umfang wird eingegrenzt. |
| **Getestet** | Nur bei belegter Testausführung mit Ergebnis. Eine vorhandene Testdatei ist lediglich Testcode, kein bestandener Test. |
| **Im Betrieb bestätigt** | Erfordert einen zuordenbaren Live-Nachweis an Station und Endgerät. Für die historischen Roadmap-Phasen liegt ein solcher Nachweis in diesem Planungsstand nicht vor. |
| **Repository-Dokumentation** | Eine Datei beschreibt eine Funktion oder bezeichnet sie als „umgesetzt“. Das allein wird nicht in „implementiert“, „getestet“ oder „im Betrieb bestätigt“ umgewandelt. |

### 2.3 Grenzen der geprüften Prüfung

Die Repository-Prüfung erfolgte lesend über den GitHub-Connector und auf den oben festgehaltenen Commit bezogen. Suchtreffer des Default-Branches dienten zur Pfadfindung; die tatsächlich verwendeten Dateien wurden danach ausdrücklich aus dem geprüften `Archiving`-Commit geladen. Es handelt sich um eine **gezielte statische Prüfung**, nicht um einen vollständigen Audit aller Dateien und aller Branches.

Ein lokaler Clone scheiterte in der Arbeitsumgebung an der DNS-Auflösung von `github.com`. Deshalb wurden weder ein lokaler Cargo-Build noch Rust-Tests ausgeführt. Der GitHub-Connector blieb für Lese- und Schreiboperationen nutzbar. Es gab keinen SSH-Zugriff auf eine reale TBS, keinen SDR-Zugriff, keine Endgeräteprüfung und keine Messung der Luftschnittstelle.

Die 25 verfügbaren PDFs wurden anhand der vorhandenen Titelblätter, Inhaltsübersichten und Scope-Auszüge eingeordnet sowie lokal nach Seitenzahl und SHA-256 inventarisiert. Eine vollständige technische Prüfung aller **8.061 Dateiseiten** wurde nicht durchgeführt. Diese Summe enthält sowohl Einzeldateien als auch eine große Sammeldatei und ist **keine Anzahl unterschiedlicher Normseiten**. Auch der aktuelle Veröffentlichungsstatus sämtlicher Normen wurde nicht systematisch überprüft.

## 3. Ziel, Ausgangslage und endgültige Anforderungen

Gesucht ist eine **realistische schrittweise Funktionsroadmap für die Basisstation**. Firmenwebseite und vergleichbare Firmenaufgaben bleiben außerhalb des Umfangs. Erster praktischer Erfolg: **Die Station sendet, und Funkgeräte verbinden sich.**

Diese Vorgaben sind die belastbaren Entscheidungen dieser Planung:

| Anforderung | Status | Begründung und Konsequenz |
|---|---|---|
| Nur basisstationsbezogene Funktionen aufnehmen | Beschlossen/geplant: ausdrücklicher Arbeitsauftrag | Funkbetrieb, Verwaltung, Diagnose und notwendige Netzdienste gehören hinein; Firmenwebseite und Marketing nicht. |
| Mit Aussendung und Geräteanmeldung beginnen | Beschlossen/geplant: ausdrücklicher Arbeitsauftrag | Eine nutzbare Einzelzelle ist der erste überprüfbare Meilenstein. |
| Funktionen nach und nach entwickeln | Beschlossen/geplant: ausdrücklicher Arbeitsauftrag | Abhängigkeiten und Abnahmekriterien müssen vor komplexer Skalierung stehen. |
| Alle früheren Ideen berücksichtigen | Gewünschter Umfang; Vollständigkeit nicht belegbar | Die referenzierten Backups fehlen. Der sichtbare Entwurf kann vollständig inventarisiert werden, der gesamte damalige Ideenbestand nicht. |
| Acht Phasen und Produkt-/Architekturnamen | Historische Gliederungs- und Lösungsansätze | Einzelbestätigungen fehlen. |
| Verbindliche Termine, Budget, Personalbedarf oder Fertigstellungsprozente | Nicht festgelegt | Werden in diesem Archiv nicht nachträglich erfunden. |

Ein vermessener Hardware-Ausgangszustand ist nicht dokumentiert. Der gewünschte erste Meilenstein ist kein Nachweis, dass Aussendung und Teilnehmerregistrierung bereits funktionierten.

## 4. Vollständiges Inventar des historischen Acht-Phasen-Entwurfs

**Alle nachfolgenden Einzelpunkte sind historische Vorschläge.** „Ergebnis“ bezeichnet jeweils das damals formulierte Ziel, nicht einen eingetretenen Zustand. Die IDs H1.1 bis H8.5 sind ausschließlich Archiv-Verweise, keine bestehenden GitHub-Issues.

### H1 – Grundfunktion / Mindestziel

Ziel: Die Station sendet stabil; Funkgeräte können sich verbinden.

| ID | Historisch vorgeschlagener Schritt |
|---|---|
| H1.1 | SDR-Hardware als **LimeSDR Mini** konfigurieren. |
| H1.2 | **HamTetra / OsmocomTETRA** auf einer VM oder einem **Raspberry Pi 5** aufsetzen. |
| H1.3 | Einen Steuerkanal, als **MCCH** bezeichnet, auf **438 MHz** aussenden. |
| H1.4 | TMO-Anmeldung von Funkgeräten mit **MCC 901 / MNC 999** erreichen. |
| H1.5 | Basis-Monitoring mit **RSSI, Logs und Terminal-Connect-Anzeige** bereitstellen. |

Damals formuliertes Ergebnis: erstes funktionierendes TMO-Signal mit Verbindungsaufbau. Ein konkreter RX-Kanal, Duplexplan, Treiberstand, Funkgeräte-Codeplug oder Logbeleg wurde nicht angegeben. Die Technologie- und Frequenzangaben sind nicht als ergänzende Installationsanweisung zu übernehmen; siehe Abschnitt 8.

### H2 – Stabilisierung und Netzwerk-Backbone

Ziel: lokaler Dauerbetrieb, VPN-Anbindung und zentrale Steuerung.

| ID | Historisch vorgeschlagener Schritt |
|---|---|
| H2.1 | **OpenVPN** und **VLAN-Trennung** einrichten. |
| H2.2 | Eine zentrale Admin-VM namens **„Lighthouse“** im Heimnetz aufsetzen. |
| H2.3 | Logging und **ISSI-Monitor**, mit **Web-GUI und Telegram-Bot**, aktivieren. |
| H2.4 | Automatische Dienstüberwachung mit **Watchdog und Neustart** vorsehen. |
| H2.5 | Webinterface für Status und Grundfunktionen anbieten. |

Damals formuliertes Ergebnis: eine remote erreichbare, stabile Einzelzelle mit Telemetrie. VPN-Endpunkte, VLAN-IDs, IP-Adressen, systemd-Units, Alarmregeln und Zugriffsrechte wurden nicht festgelegt.

### H3 – Mehr-Node-Betrieb und Synchronisation

Ziel: mehrere Standorte über VPN koppeln; dafür wurden die Namen **„Terralink“ / „NetCore-Tetra Mesh“** benutzt.

| ID | Historisch vorgeschlagener Schritt |
|---|---|
| H3.1 | Eine zweite Node mit **Pi + LimeSDR** über VPN anbinden. |
| H3.2 | **Audio- und SDS-Routing über das Internet** testen. |
| H3.3 | **NTP / GPS** zur Zeitsynchronisation für „synchrone Aussendungen“ einsetzen. |
| H3.4 | **Shadow-GSSI** und einen **Meta-Sync-Kanal** einführen. |
| H3.5 | **Lastmanagement und dynamisches GSSI-Mapping** aktivieren. |

Damals formuliertes Ergebnis: ein dezentrales TETRA-Netz mit redundanter Struktur. Ein tatsächliches Routingprotokoll, ein Zellwechselablauf, eine Floor-Control-Verantwortung, ein Timingbudget oder ein Redundanzverfahren wurde nicht spezifiziert. „Mesh“, Mehrzellenbetrieb, Rufwiederherstellung und synchrones HF-Senden wurden im Entwurf nicht sauber gegeneinander abgegrenzt.

### H4 – Webpanel und mobile App-Integration

Ziel: zentrale rollenbasierte Verwaltung.

| ID | Historisch vorgeschlagener Schritt |
|---|---|
| H4.1 | **Lighthouse-Webpanel** mit Node-Übersicht, GSSI-Mapping und **OTA** bauen. |
| H4.2 | **REST-API und JWT-Authentifizierung** implementieren. |
| H4.3 | **Android- und iOS-App** für Status, Logs und Trigger anbinden. |
| H4.4 | **Push-Benachrichtigungen** bei Node-Ausfall oder Überhitzung vorsehen. |
| H4.5 | API-Tests und Rechteverwaltung für **Admin / Operator / Viewer** erstellen. |

Damals formuliertes Ergebnis: vollständige zentrale oder mobile Kontrolle. App-Technologie, Plattformumfang, Session-Lebenszyklus, Ressourcenrechte, Push-Anbieter und OTA-Rollback blieben offen. Die bloße Nennung von JWT ist kein fertiges Autorisierungsmodell.

### H5 – Virtueller Sprecher und Audio-Management

Ziel: Sprache aus Dateien oder einer App ins Funknetz einspeisen.

| ID | Historisch vorgeschlagener Schritt |
|---|---|
| H5.1 | **Web-Mediathek** mit Upload, Tags und Wiedergabe anbieten. |
| H5.2 | **PCM-Konvertierung und RF-Ausspielung** über eine **virtuelle ISSI** ermöglichen. |
| H5.3 | **Multi-Node-Sync-Senden**, erneut als NTP-/GPS-basiert beschrieben, ergänzen. |
| H5.4 | **Signierung, Logging und Rechte** für diese Funktionen vorsehen. |
| H5.5 | **Text-to-Speech** sowie **Sensor-Trigger**, beispielsweise eine Temperaturwarnung, ergänzen. |

Damals formuliertes Ergebnis: automatische Sprachrufe und Alarme. Codec, Rufart, Zieladressierung, Queue-Verhalten, PTT-/Floor-Regeln, Prioritäten, Abbruch und konkrete Sicherheitsmechanismen wurden nicht festgelegt. Es wurde keine numerische virtuelle ISSI zugeteilt.

### H6 – Sensorik, Aktorik und KI-Automatisierung

Ziel: reaktive, möglichst autonome Nodes.

| ID | Historisch vorgeschlagener Schritt |
|---|---|
| H6.1 | Sensoren für **Temperatur, Feuchte, Spannung und Bewegung** einbinden. |
| H6.2 | **Relais, Lüfter, Buzzer und Displays** ansteuern. |
| H6.3 | Eine **If-This-Then-That-Logik** für Automatisierungen vorsehen. |
| H6.4 | **Watchdog-Logik und Fail-Safe-Shutdowns** ergänzen. |
| H6.5 | Ein **KI-Modul für Predictive Maintenance und Anomalieerkennung** entwickeln. |

Damals formuliertes Ergebnis: wartungsarme, autonom reagierende Funkzellen. Es gab keine Sensorstückliste, Pinbelegung, Schwellwerte, Hysterese, definierte sichere Ausgangslage oder Trainings-/Validierungsdaten für KI.

### H7 – Skalierung und Edge-Intelligenz

Ziel: Ausbau zu einem Multi-Cluster-Netz.

| ID | Historisch vorgeschlagener Schritt |
|---|---|
| H7.1 | Eine **Lighthouse ↔ Watchtower-Architektur** aufbauen. |
| H7.2 | Ein **OTA-RolloutCenter** für Firmware- und Konfigurationssynchronisation entwickeln. |
| H7.3 | **KI-basierte GSSI-Optimierung und Loadbalancing** einsetzen. |
| H7.4 | **Shadow-Cluster-Kommunikation / Inter-Region-Sync** ergänzen. |
| H7.5 | **Redundanz-Failover mit automatischer Master-Umschaltung** bereitstellen. |

Damals formuliertes Ergebnis: ein „intelligentes, selbstheilendes TETRA-Mesh“. Der Entwurf enthielt dazu keine ausgearbeitete Steuerungs-, Replikations-, Quorum-, Split-Brain- oder Wiederanlaufstrategie. Die genannten Namen bleiben historische Begriffe, nicht nachgewiesene Dienste.

### H8 – Abschluss und Langzeittest

Ziel: Dauerstabilität, Dokumentation und Veröffentlichung.

| ID | Historisch vorgeschlagener Schritt |
|---|---|
| H8.1 | **Dauerbetrieb über 72 Stunden** mit Thermal-, Netzwerk- und Lasttests durchführen. |
| H8.2 | Logging optimieren und **automatische Backup-Rotation** ergänzen. |
| H8.3 | Dokumentation und **Wiki.js im Lighthouse** bereitstellen. |
| H8.4 | Ein **Git-Repository NetCore-Tetra** für Quellcode und Konfigurationen verwenden. |
| H8.5 | Ein **Beta-Release beziehungsweise eine Club-Verteilung** vorbereiten. |

Damals formuliertes Ergebnis: veröffentlichbarer Langzeitstand. Die 72 Stunden wurden nur als Testziel genannt; es gibt in diesem Planungsstand keine belegte 72-Stunden-Messung.

### Übergreifende Vision und kleine Nebenidee

Gesamtvision ist ein offenes resilientes System ohne notwendige zentrale Cloud, skalierbar von Einzelzelle bis landesweitem Amateurfunk-Mesh. Dies ist ein **Zielbild**, keine belegte Kapazitäts-, Verfügbarkeits- oder Betriebszusage. Betriebsart und nutzbare Frequenzen sind dadurch nicht freigegeben.

Am Ende wurde ein **grafisches Roadmap-Diagramm als SVG/PNG**, etwa mit Zeitachse, Phasen und Icons, angeboten. Eine Beauftragung oder Erstellung ist im zugänglichen Verlauf nicht vorhanden. Es gibt daher kein historisches Diagramm, das diesem Planungsstand als erzeugtes Artefakt zugeschrieben werden darf.

## 5. Historisch erreichter Entwicklungs- und Betriebsstand

| Bereich | Historischer Nachweis dieser Planung |
|---|---|
| Roadmap-Text | Erstellt: acht Phasen mit insgesamt 40 vorgeschlagenen Schritten sowie Gesamtvision und Grafikangebot. |
| Auswahl einer endgültigen Hardware-/Softwareplattform | Nicht bestätigt. LimeSDR Mini, Pi 5, VM und HamTetra/OsmocomTETRA wurden vorgeschlagen. |
| Erfolgreiche Aussendung / Registrierung | Gewünscht; Messungen, Logs und Betreiberbestätigung fehlen. |
| Lokale oder standortübergreifende Sprache / SDS | Nicht getestet oder im Betrieb bestätigt. |
| VPN, Lighthouse, Watchtower, Apps, Sensorik oder KI | Kein Implementierungs- oder Installationsnachweis im Entwurf. |
| Konkrete Codeänderung, Commit oder PR | Dem historischen Austausch nicht zuordenbar. |
| Shell-Kommandos / Deployment / Reparatur | Keine tatsächlich ausgeführten fachlichen Abläufe dokumentiert. |
| Bilder / Architekturzeichnung | Keine eigenständigen Bilder verfügbar; Grafik nur angeboten. |

Der historische Abschluss ist damit ein **Planungsartefakt**. Geprüfter Quellcode im Repository ist davon getrennt zu betrachten und darf nicht rückwirkend als damaliges Arbeitsergebnis ausgegeben werden.

## 6. Repository-Abgleich am Dokumentdatum am geprüften Commit

### 6.1 Konkrete Befunde und ihre Grenzen

| Prüfbereich | Festgestellter Stand | Einordnung |
|---|---|---|
| Rust-/TBS-Struktur | Der Workspace enthält `tetra-core`, `tetra-saps`, `tetra-config`, `tetra-pdus`, `tetra-entities` und `bins/bluestation-bs`. [R2] | Build-Struktur vorhanden; der alte generische Erstaufbau entspricht nicht mehr der tatsächlichen Repository-Struktur. |
| TBS-Binary | Paket und Binary heißen `bluestation-bs`. Die Default-Features sind `asterisk`, `recording` und `audio-player`. [R3] | Build-Integration im Manifest implementiert; kein Nachweis des installierten Binarys oder eines erfolgreichen Builds. |
| RF-Konfiguration | `stack_mode = "Bs"`, Backend `SoapySdr`, TX 418.000.000 Hz und RX 408.000.000 Hz. [R4] | Eingecheckte Parameter, nicht live gemessene Frequenzen. |
| Netz-/Zellparameter | MCC 901, MNC 1510, LA 1, Colour Code 1; Main Carrier 720, Secondary Carrier 721. [R4] | Der alte Entwurfswert MNC 999 ist keine aktuelle Sollkonfiguration. |
| Dashboard | `CfgDashboard` enthält Port/Bind/Source-Verzeichnis, optionale lokale Zugangsfelder und `public_overview`. Defaults: Port 8080, Bind `0.0.0.0`, keine Zugangswerte. [R5] | Konfigurationslogik implementiert. Daraus folgt weder zentrale RBAC noch die genaue ergänzende Server-Authentifizierungsmechanik. |
| Zentrale Anmeldung/RBAC | `Docs/CENTRAL_IDENTITY_RBAC_ROADMAP.md` weist NETCORE-IAM-01 ausdrücklich als geplant aus; Identity-LXC/Keycloak sind Empfehlungen, keine endgültige Auswahl. [R11] | Planung. Der historische JWT-Stichpunkt darf nicht als bereits netzweit umgesetztes SSO/RBAC geführt werden. |
| Media Library | `src/main.rs` lädt Konfiguration, SharedLibrary und TTS, startet Worker sowie HTTP-Server und warnt ausdrücklich vor offenem Management ohne Login/Tokens/TLS. [R6] | Dienst-Einstieg und Verdrahtung im Code vorhanden. Keine Funktionsabnahme des gesamten Playout-Pfads. |
| Audio-/TTS-Playout | README beschreibt Asset-Verwaltung, TTS/Piper, TBS-basiertes Playout und alternativ Einspeisung in vorhandene Media-Switch-Sessions. [R7] | Konkreter dokumentierter Ausbau weit über H5 hinaus. Die dokumentierten Ende-zu-Ende-Funktionen wurden hier nicht live getestet. |
| Observability | README dokumentiert Collector, strukturierte Logs/Spans, Alarmregeln, Diagnose und einen begleitenden Prometheus/Grafana/Loki/Alertmanager-Stack. [R8] | Dokumentierte Komponenten, nicht Beleg ihrer Installation. Der Dienst grenzt Open Lab, fehlendes produktives RBAC/TLS und fehlende HA-Funktionen ausdrücklich ab. |
| IoT / Home Assistant | README dokumentiert MQTT-Ereignisse, Command/Ack, Default-Deny-Policies, HA-Discovery und optionale Homematic-Anbindung. Reale Schreibzugriffe sind standardmäßig aus. [R9] | Dokumentierter Integrationsstand; keine Bestätigung realer Sensoren/Aktoren oder KI. |
| Core-/Multi-Site-Architektur | Der Workspace enthält mehrere Core-/Gateway-Dienste. Die SwMI-Roadmap beschreibt lokale TBS vor Edge/Core-Aufteilung sowie Teilnehmer-/Mobility-Dienste, Call Control, Media, SDS, Packet Data, Security und Transit. [R2][R10] | Pfade und Planungs-/Dokumentationsstand vorhanden. Die dortigen „umgesetzt“-Markierungen wurden nicht pauschal als vollständig geprüft übernommen. |
| Call Restore | Ein konkreter Handler für `UCallRestore` ist vorhanden; aktive Einzelrufe, Gruppenrufe und Ablehnung unbekannter Rufe werden behandelt. [R12] | Codepfad implementiert, aber mit weiter zu prüfenden Risiken; siehe Abschnitt 9. |
| Zwei-Zellen-Testgerüst | Die gelesene Testdatei erzeugt `ComponentTest`-Zellen und transferiert Restore-Kontext direkt zwischen Instanzen. [R13] | Testcode vorhanden. Kein ausgeführter Test und kein Nachweis eines realen Zellwechsels über Funk. |
| Projektversion | Die Root-README trägt die Bezeichnung v1.9.0 und beschreibt unter anderem zentrale SIP-Anbindung mit lokalem Asterisk-Fallback. [R1] | Dokumentationsstand; weder installierte Version noch Nachweis des SIP-Failovers. |
| SXceiver-/Codec-Verzeichnisse | `sxxcvr-main` und `tetra-codec-master` sind im Root-Tree vorhanden. [R14] | Verzeichnisexistenz, keine Feststellung über angeschlossene Hardware oder erfolgreich gebaute Treiber. |

### 6.2 Nicht als am 2026-10-04 umgesetzt bestätigt

Für die historischen Namen **Lighthouse, Watchtower, Terralink, Shadow-GSSI, Meta-Sync-Kanal, Shadow-Cluster und OTA-RolloutCenter** wurde in dieser gezielten Prüfung kein eindeutig passender, vollständig geprüfter Implementierungsnachweis hergestellt. Das ist keine Behauptung, dass jede entsprechende Funktion im gesamten Repository fehlt. Es verhindert lediglich, historische Namen und ergänzende Dienste ohne Nachweis gleichzusetzen.

Auch eine fertige native iOS-App, Push-Zustellung, netzweit synchronisierte Audioaussendung, KI-gestützte Wartung, KI-Lastverteilung, automatische Masterwahl und landesweite Skalierbarkeit wurden hier nicht bestätigt.

## 7. Architektur, Komponenten und Abhängigkeiten

### 7.1 Historisch beabsichtigte Architektur

Der Entwurf sah eine SDR-basierte lokale TBS, eine zentrale Admin-VM, VPN-gekoppelte weitere Nodes, Web-/App-Bedienung, Mediathek/TTS, Hardware-I/O und später eine regionale Clusterhierarchie vor. Er spezifizierte jedoch keine implementierbaren Verträge zwischen diesen Ebenen.

Offen blieben insbesondere Zuständigkeiten für Teilnehmerregistrierung, Gruppenzuordnung, Rufaufbau, Floor Control, Audio-Routing, Updatefreigabe und die Reaktion auf Zentralenausfall. „NTP/GPS“ und „JWT“ waren Technologie-Stichworte, keine vollständigen Architekturentscheidungen.

### 7.2 Ergänzende Anschlussstellen statt Parallelneubau

Die gelesenen Quellen legen folgende Anschlussstellen für eine Fortsetzung nahe:

- **Lokale TBS:** vorhandenen Rust-Stack und `bluestation-bs` verwenden; Betriebsmodus, SoapySDR-Konfiguration, Dienst und Binary gemeinsam identifizieren. [R2][R3][R4]
- **Teilnehmer-/Netzbetrieb:** vorhandene Core-, Gateway-, Call-Control- und Media-Komponenten gegen den benötigten konkreten Ablauf prüfen, bevor ein neuer „Lighthouse“-Monolith gebaut wird. [R2][R10]
- **Bedienung und Identität:** bestehende Dashboard-Konfiguration und NETCORE-IAM-01 als Ausgangspunkt behandeln; menschliche Webidentitäten nicht mit TETRA-Teilnehmeridentitäten vermischen. [R5][R11]
- **Audio:** Mediathek/TTS vom zeitkritischen Funkpfad trennen. Der dokumentierte bevorzugte Pfad delegiert Ruf, Codec, Timeslot und Floor an die TBS. [R7]
- **Betrieb und Automatisierung:** vorhandene Observability- und IoT-Verträge nutzen; offene Lab-Verwaltung nicht als fertig abgesicherte Remote-Steuerung darstellen. [R8][R9]

Diese Anschlussstellen sind eine **Fortsetzungsempfehlung aus dem Repository-Abgleich**, keine nachträgliche historische Festlegung.

### 7.3 Zentrale fachliche Trennlinien

**Aussendung und Registrierung brauchen getrennte Nachweise.** Der erste Meilenstein erfordert beide. Trägerlinie, gestartete Unit und grüne Webseite allein genügen nicht.

**Registrierung ist nicht Sprache oder SDS.** Die nächste Ausbauentscheidung sollte einzelne überprüfbare Kommunikationsabläufe benennen statt alle unter „Geräte verbunden“ zusammenzufassen.

**IP-Kopplung ist nicht automatisch nahtloser Zellwechsel.** Standortübergreifendes Routing, Teilnehmerlage, Rufwiederherstellung, Floor-Besitz und HF-Timing müssen getrennt abgenommen werden. Der aktuelle Restore-Code und sein Komponenten-Testgerüst illustrieren zusätzliche Zustände, die im alten Entwurf fehlen. [R12][R13]

**Software-OTA ist nicht TETRA-OTAR.** Das historische OTA-Stichwort bezog sich auf Firmware/Konfigurationsrollout. Die Security-/TSIM-Anhänge sind keine Bestätigung einer implementierten Schlüsselverteilung oder eines Updateverfahrens.

**Zentrale Verwaltung ist nicht notwendige zentrale Cloud.** Der Wunsch nach cloudunabhängigem Betrieb bleibt als Ziel erhalten; welcher Funktionsumfang bei Ausfall eigener Zentraldienste lokal weiterläuft, muss ausdrücklich definiert werden.

## 8. Dateien, Parameter, Ports, Protokolle und Pfade

### 8.1 Historische Werte versus eingecheckte Konfiguration

| Parameter | Historischer Entwurf | Geprüfter Repository-Stand / Grenze |
|---|---|---|
| Plattform | LimeSDR Mini, VM oder Pi 5 | Konfiguration verwendet SoapySDR; physisches Gerät nicht ermittelt. [R4] |
| Betriebsmodus | TMO als Ziel | `stack_mode = "Bs"`. [R4] |
| TX / Downlink | 438 MHz | `tx_freq = 418000000`. [R4] |
| RX / Uplink | Nicht angegeben | `rx_freq = 408000000`. [R4] |
| SDR-Abtastrate | Nicht angegeben | `sample_rate = 600000` Samples/s. [R4] |
| SDR-Tuningmitten | Nicht angegeben | TX 418.012.500 Hz; RX 408.012.500 Hz. Nicht mit dem Hauptträger verwechseln. [R4] |
| MCC / MNC | 901 / 999 | 901 / 1510. [R4] |
| Main / Secondary Carrier | Nicht angegeben | 720 / 721; `freq_band = 4`, `freq_offset = 0`. [R4] |
| Duplex-/Richtungsfelder | Nicht angegeben | `duplex_spacing = 0`, `reverse_operation = false`; tatsächliche Interpretation und Endgeräteprofil gesondert prüfen. [R4] |
| Location Area / Colour Code | Nicht angegeben | 1 / 1. [R4] |
| Zeitzone | NTP/GPS nur als Idee | `timezone = "Europe/Berlin"`; ein Zeitzonenfeld beweist keine HF-Synchronisation. [R4] |
| Lokale SSI-Bereiche | Nicht angegeben | `local_ssi_ranges = [[0, 90]]`. [R4] |
| Dienstzuordnung | Nicht angegeben | `service_name = "tetra"`; installierte Unit nicht überprüft. [R4] |
| Virtuelle Sprecher-ISSI | Nur Begriff, keine Nummer | In dieser Archivierung keine Nummer zugeteilt oder aktuelle Zuordnung festgestellt. |

Die Kommentare der eingecheckten RF-Konfiguration sind teilweise nicht selbsterklärend: neben Main Carrier 720 steht beispielsweise noch ein Kommentar mit einer anderen Carrier-Zahl; der Duplex-Kommentar nennt einen Abstand, der nicht unmittelbar zum ausgeschriebenen RX-/TX-Paar passt. Hier werden **Werte und Kommentare nicht stillschweigend gleichgesetzt oder repariert**. Ein späterer Konfigurations-/Codeplug-Abgleich muss die verwendete Kanalberechnung und den wirklich ausgestrahlten Systeminhalt prüfen. [R4]

Alle Frequenzwerte in dieser Tabelle sind historische Angaben beziehungsweise aus dem Repository gelesene Werte, **keine Genehmigung zum Senden**. Die Konfiguration selbst warnt davor, ohne Anpassung an die zulässigen Betriebsbedingungen zu starten. [R4]

### 8.2 Relevante Management- und Datenpfade

| Komponente | Port / Protokoll / Pfad | Nachweisart |
|---|---|---|
| TBS-Dashboard | Default TCP 8080, Bind `0.0.0.0` | Code in `sec_dashboard.rs`; kein Nachweis eines offenen Live-Ports. [R5] |
| Media Library | HTTP/WebUI/API, dokumentierter Port 8230 | README. [R7] |
| Media-Library-Konfiguration | `/etc/netcore/media-library.toml`; CLI-Optionen `--config`, `--no-config`, `--bind` | Dienst-Einstiegscode. [R6] |
| TBS-Audiosteuerung | `POST /api/audio/play`, `GET /api/audio/status`; bei Bedarf Anmeldung über `/api/login` | Dokumentierter Worker-/TBS-Vertrag, nicht live aufgerufen. [R7] |
| Direkte Media-Switch-Einspeisung | `/api/v1/sessions/{session_id}/inject` | Dokumentierter Alternativpfad; vorhandene Session erforderlich. [R7] |
| Observability | HTTP/WebUI/API TCP 8210; `/metrics`, `/api/v1/logs/ingest`, `/api/v1/traces/ingest` | README; Ingest zunächst NetCore-JSON-v1. [R8] |
| Begleitender Monitoring-Stack | Grafana 3000, Prometheus 9090, Alertmanager 9093, Loki 3100 | Dokumentierte Standardports, nicht installierte Dienste bestätigt. [R8] |
| IoT Gateway | WebUI/API/Metrics TCP 8240; optional MQTT/Mosquitto TCP 1883 | README. [R9] |
| HA-State-Ingress | MQTT-Topic `netcore/v1/integrations/homeassistant/state`; HA-Neustartsignal `homeassistant/status` | README. [R9] |
| IoT-Verträge | `netcore-event-v1`, `netcore-command-v1`, `netcore-command-ack-v1` | README. [R9] |
| Homematic | Optional XML-RPC; HmIP-Beispielport 2010 | Kein standardmäßig aktivierter realer Aktorzugriff. [R9] |
| IoT-Persistenz | `/var/lib/netcore-iot-gateway/`, unter anderem Outbox, `dedup.json`, `command-ledger.json`, Audit und Gerätezustände | Dokumentierte Pfade. [R9] |
| TBS-Fallbackkonfiguration | `config.toml.fallback`; Kommentar nennt beispielhaft `/opt/tetra/config.toml.fallback` | Dokumentierte Recovery-Idee in der Konfiguration, Parserpfad hier nicht geprüft. [R4] |
| OpenVPN / VLAN / NTP / Telegram / Push | Historisch vorgeschlagen, aber ohne konkrete Ports, Ziele, VLAN-IDs oder Konfigurationen | Keine Standardwerte als angeblichen Projektbestand ergänzt. |

### 8.3 Audioformate und Betriebsmodi

Die Media-Library-Dokumentation beschreibt eine kanonische Vorschau als **8 kHz, mono, signed PCM16 in RIFF/WAVE**. Im bevorzugten `playout.mode = "basisstation"` lädt die TBS die WAV-Datei und übernimmt den lokalen Codec-/Rufpfad. Der dokumentierte Alternativmodus `media_switch` benötigt einen validierten TACELP-Cache, dessen gepackte Frames mit **35 Byte pro 60 ms** beschrieben werden, sowie eine vorhandene Session. Diese Zahlen sind hier ausdrücklich **Repository-Vertragsangaben**, keine eigene vollständige Codec-Konformitätsprüfung. [R7]

`runtime.operating_mode = "shadow"` bedeutet laut README lediglich protokollierte Aufträge ohne Funkübertragung. Erst `authoritative` mit korrekt konfiguriertem Playout-Pfad ist dort als Weg zur tatsächlichen Aussendung beschrieben. Ein abgeschlossener Shadow-Job darf deshalb nicht als Funkabnahme zählen. Die reale Konfiguration wurde nicht umgeschaltet. [R7]

## 9. Fehler, Diagnose, überholte Annahmen und verbleibende Risiken

### 9.1 Historische Dokumentationsprobleme

| Problem | Diagnose / Einordnung | Behandlung im Archiv |
|---|---|---|
| Behauptete Vollständigkeit auf Basis zweier Backups | Die Backups sind nicht eindeutig zugänglich. | Vollständigkeitsbehauptung zurückgenommen; zugängliche Ideen vollständig inventarisiert. |
| HamTetra/OsmocomTETRA als pauschaler TMO-Startweg | Passende Implementierung und bidirektionale TMO-Anmeldung fehlen. Die offizielle osmo-tetra-README beschreibt das Sender-Testprogramm als Burst-Erzeuger ohne tatsächliche Modulation/Aussendung. [E1] | Keine bewährte Installationsanleitung. Der geprüfte NetCore-Workspace ist separat erfasst. |
| 438 MHz / MNC 999 als scheinbare Festlegung | Historische Entwurfswerte; abweichend von eingecheckter Konfiguration. [R4] | Als Beispiele erhalten, keine bestätigten Sollwerte. |
| NTP/GPS als vermeintlich ausreichendes Synchronisationskonzept | Kein Timingbudget, keine Hardware-Zeitreferenz, kein Verhalten bei Drift oder Ausfall beschrieben. | Synchronisation als eigene Spezifikations-/Messaufgabe führen, nicht als erledigten Unterpunkt. |
| Shadow-GSSI, Meta-Sync und Shadow-Cluster | Begriffe ohne definiertes Datenmodell, Protokoll und Fehlerverhalten. | Als Ideen erhalten; keine Gleichsetzung mit DGNA, Call Restore oder ISI behaupten. |
| „Dezentral“, „redundant“ und „selbstheilend“ | Keine belegten Ausfalltests oder Replikationsregeln. | Nur Zielbegriffe, keine Betriebsmerkmale. |
| Langzeittest, Git und Dokumentation erst am Ende | Im ursprünglichen Ablauf nachgelagert, obwohl frühere Phasen bereits komplex sind. | Für die Fortsetzung frühe Nachweisführung empfehlen; historisches H8 unverändert dokumentieren. |

Es gab in diesem Planungsstand keine gemeldete konkrete Laufzeitstörung mit Logs und erfolgreich erprobter Reparatur. Die vorstehenden Punkte sind **Planungs-/Belegprobleme**, keine erfundenen historischen Software-Bugs.

### 9.2 Zusätzlich am 2026-10-04 sichtbare technische Prüfpunkte

**Einzelruf-Restore und Sprechrecht:** Der gelesene Handler prüft aktiven Ruf und Teilnehmerzugehörigkeit, leitet die Erteilung einer Sprechfreigabe im Einzelrufzweig anschließend aber unmittelbar aus `request_to_transmit_send_data` ab. Eine Prüfung, ob der andere Teilnehmer gerade das Sprechrecht hält, ist in diesem Zweig nicht sichtbar. Das ist ein **statisch begründeter Prüf-/Fixkandidat**, kein hier reproduzierter On-Air-Fehler. [R12]

**Gruppenruf-Restore und Uplink-Überwachung:** Der Gruppenrufzweig unterscheidet bereits zwischen freiem beziehungsweise eigenem Floor und einem anderen Sprecher. Nach `grant_floor` ist im gelesenen Handler kein separater Benachrichtigungsaufruf an die untere Funksteuerung sichtbar. Ob ein anderer Pfad die notwendige Überwachung abdeckt, wurde hier nicht verfolgt. Der gesamte Pfad bis zum Verhalten bei ausbleibenden Uplink-Frames ist deshalb gezielt zu prüfen. [R12]

**Offene Verwaltung:** Media-Library-Einstiegscode sowie Observability-/IoT-Dokumentation warnen vor offenem Lab-Betrieb. Ein VPN allein ist noch kein Nachweis der vorgeschlagenen rollen- und ressourcenbezogenen Berechtigungsprüfung. Vor Ausweitung der Erreichbarkeit sind geschützte Aktionen, Identitäten und Ausfallregeln abzugleichen. [R6][R8][R9][R11]

**Build-/Dokumentationswiderspruch:** Das TBS-Manifest aktiviert Asterisk standardmäßig, enthält im späteren Debian-Paket-Kommentar aber zugleich eine gegenteilige Aussage zur enthaltenen Asterisk-Stimme beziehungsweise zum nativen Codec. Ohne Prüfung des tatsächlichen Packaging-/CI-Pfads darf daraus keine sichere Aussage über ein ausgeliefertes Paket entstehen. [R3]

**Konfigurationsdrift:** Historische Roadmap, Repository-Konfiguration, Kommentare und reales Endgeräteprofil sind vier verschiedene Quellen. Keine davon ersetzt automatisch die andere. Der erste nächste Schritt muss den tatsächlich installierten Stand festhalten, nicht alte Werte blind zurückschreiben.

## 10. Befehle und Abläufe mit Ausführungsstatus

### 10.1 Historischer Planungsstand

Es wurden keine vollständigen Installations-, Deployment- oder Reparaturbefehle ausgeführt oder als erfolgreich bestätigt. Die Namen OpenVPN, NTP, Wiki.js und HamTetra/OsmocomTETRA waren Funktions-/Technologievorschläge. Aus ihnen wird nachträglich keine erprobte Kommandoabfolge konstruiert.

### 10.2 In dieser Archivierung ausgeführt

| Vorgang | Ergebnis / Grenze |
|---|---|
| GitHub-Branchref und Commit-/Tree-Metadaten lesen | Erfolgreich; geprüfter SHA und Root-Tree in Abschnitt 1. |
| Archivindex und ausgewählte Repository-Dateien lesen | Erfolgreich; Quellen und gelesene Bereiche in Abschnitt 14. |
| Neuen Archivpfad auf Existenz prüfen | HTTP 404 vor Anlage: Zielpfad war nicht vorhanden. |
| Lokalen Clone versuchen | Fehlgeschlagen: `Could not resolve host: github.com`. Kein lokaler Quellcode-Checkout entstanden. |
| PDF-Inventar mit PyMuPDF und SHA-256 erzeugen | Erfolgreich für 25 lokal vorhandene PDFs. Kein OCR verwendet. |
| Eigenständige Bilddateien suchen | Keine Treffer im zugänglichen Dateienbestand; ursprünglicher Mount enthält nur PDFs. |
| Bestehenden Index lokal gegen Git-Blob prüfen | Rekonstruierter Originalinhalt entsprach Blob `ee0ff8e0796549a90ef9d4fbda69a313bf2aaf90` bei 14.173 Byte. |
| TBS starten, Konfiguration verändern, Funk aussenden | Nicht ausgeführt. |
| Cargo-/Komponenten-/On-Air-Tests | Nicht ausgeführt. |

Der fehlgeschlagene Clone verwendete ausdrücklich den Zielbranch:

```bash
git clone --depth 1 --single-branch --branch Archiving \
  https://github.com/JanHG98/netcore-tetra.git /mnt/data/netcore-archive-work
```

Die Speicherung des Archivs erfolgt davon unabhängig über GitHub-Git-Datenobjekte und ein nicht erzwungenes Branch-Update. Eine erfolgreiche Speicherung ist anhand des tatsächlichen Archivcommits und der anschließenden Branch-Leseprüfung zu beurteilen, nicht anhand dieses Clone-Versuchs.

### 10.3 Für eine Fortsetzung vorgeschlagen, hier nicht am Zielsystem ausgeführt

Zuerst read-only den installierten Stand sammeln:

```bash
# Im tatsächlich verwendeten Quellcodeverzeichnis ausführen:
git branch --show-current
git rev-parse HEAD
git status --short

# Nur die nachweislich installierte TBS-Unit verwenden.
# 'tetra.service' ist aus service_name="tetra" abgeleitet, nicht live bestätigt:
systemctl status tetra.service --no-pager
journalctl -u tetra.service -b -n 200 --no-pager

# Hardware-/Treiberprobe; aus dem Konfigurationskommentar übernommen:
SoapySDRUtil --probe
```

Ein aus dem geprüften Paketnamen abgeleiteter Buildkandidat lautet:

```bash
cargo build --locked --release -p bluestation-bs
```

Dieser Befehl wurde hier **nicht ausgeführt**. Rust-Toolchain, native Codec-Abhängigkeiten, SoapySDR und Zielarchitektur müssen zuvor zum vorhandenen Installationsweg passen. Ein Build startet noch keine Station. Ebenso ist `cp config.toml config.toml.fallback` nur ein Kommentarvorschlag aus der Konfiguration; eine bereits vorhandene bekannte gute Fallbackdatei sollte nicht ohne Prüfung überschrieben werden. [R3][R4]

Vor Übernahme von Logs und Konfigurationen in spätere Berichte sind Zugangsdaten zu entfernen. Dieses Archiv übernimmt keine Passwörter, Tokens oder privaten Schlüssel.

## 11. Tests, Ergebnisse und fehlende Abnahme

Die historische Roadmap enthält **Testwünsche**, aber keine fachlichen Testergebnisse. Die ergänzende Prüfung ergänzt lediglich statische Befunde und die Dateiinventarisierung.

| Test-/Nachweisebene | Belegter Stand |
|---|---|
| Code-/Konfigurationssichtung | Ausgeführt, auf die angegebenen Dateien und Bereiche begrenzt. |
| Testcode vorhanden | Zwei-Zellen-Restore-Komponentengerüst eingesehen. [R13] |
| Tests tatsächlich bestanden | Nicht nachgewiesen; keine Testausführung in dieser Archivierung. |
| Träger / MCCH empfangen und dekodiert | Kein Messbericht im Entwurf. |
| Geräteanmeldung / Wiederanmeldung | Keine zuordenbaren Funkgeräte-/TBS-Logs. |
| Sprache / SDS lokal und über zwei Standorte | Kein Ende-zu-Ende-Test. |
| Restore bei konkurrierendem Sprecher / stummem Uplink | Offener gezielter Regressionstest. |
| Auth-/Ressourcenrechte / Zentralenausfall | Keine abgeschlossene Sicherheits-/Ausfallabnahme. |
| Mediathek / TTS / virtuelle ISSI | Dokumentierte und teilweise im Einstiegscode verdrahtete Bausteine; kein hörbarer RF-Nachweis. |
| Sensoren / Aktoren / Fail-Safe | Keine Hardwaretestreihe. |
| 72-Stunden-Dauerbetrieb | Historisches Ziel, nicht durchgeführt oder bestätigt. |

Für künftige Nachweise sollten mindestens Commit, Binary-/Buildzuordnung, Konfigurationsstand, SDR/Treiber, Endgerät/Firmware/Profil, Testschritte, erwartetes Verhalten, beobachtetes Verhalten und Zeitbezug dokumentiert werden. Geheimnisse gehören nicht in den Testbericht. Diese Nachweisliste ist eine neue Fortsetzungsempfehlung aus der Archivierung.

## 12. Realistisch geordnete Roadmap-Kandidaten zur Fortsetzung

Die folgende Reihenfolge ist eine **am 2026-10-04 abgeleitete Empfehlung innerhalb des Archivs**. Sie ersetzt keine verbindliche Projekt-Roadmap und verändert keine Datei außerhalb `Docs/archive/`. Vorhandene Funktionen sollen jeweils zuerst abgenommen und nur bei einer belegten Lücke weiterentwickelt werden.

### M0 – Eine Station sendet, ein Gerät registriert sich

**Priorität P0.** Das ist der ausdrücklich gewünschte Anfang. Zuerst den realen Source-/Binary-/Treiberstand und die zulässige Testkonfiguration festhalten. Einen überschaubaren Einzelzellen-Testzustand verwenden, ohne zusätzliche Abhängigkeit von Apps, KI oder einem neuen Management-Core.

**Abnahme:** Die Aussendung wird unabhängig von der Weboberfläche erkannt und der erwartete Steuerkanalinhalt überprüft. Ein vorgesehenes Funkgerät meldet sich an; Teilnehmerkennung und Registrierung sind auf Geräte- und Stationsseite nachvollziehbar. Aus-/Einschalten und erneute Anmeldung lassen sich wiederholen. Ein lediglich gestarteter Prozess genügt nicht.

**Herkunft:** H1.1–H1.5; alte Frequenz-/Softwarevorschläge nicht ungeprüft verwenden. [R2][R3][R4]

### M1 – Lokale Kommunikation und Recovery

**Priorität P0; abhängig von M0.** Lokale Sprache und SDS als eigene Abläufe prüfen. Gruppen-/Einzelrufumfang bewusst festlegen, PTT/Floor, Freigabe, Abbruch und erneuten Ruf testen. Die geprüften Restore-Risiken früh bearbeiten, bevor sie in einen Zwei-Standort-Test übernommen werden.

**Abnahme:** Die vereinbarten lokalen Kommunikationsfälle funktionieren mit zugeordneten Logs und realen Endgeräten. Ein wiederhergestellter Einzelruf erzeugt keine ungewollte zweite Sprechfreigabe; ausbleibender Uplink wird entsprechend dem festgelegten Timeout-Verhalten behandelt. Falls ein Test scheitert, konkrete Regression statt pauschaler „Multi-Site noch offen“-Markierung dokumentieren.

**Herkunft:** Im alten Entwurf nur unzureichend als lokales Gate ausgeführt; aus H3.2, H5 und den geprüften Befunden abgeleiteter zusätzlicher Zwischenschritt. [R12][R13]

### M2 – Stabile Einzelzelle und sichere Betriebsgrundlage

**Priorität P0/P1; abhängig von M0, parallel zu M1 ausbaubar.** Dienststart, Neustart, Fehlerprotokolle, Diagnose, Backup und Wiederherstellung konkretisieren. OpenVPN/VLAN nur nach festgelegtem Netzplan anbinden. Temperatur-/Spannungsüberwachung zunächst als beobachtende Funktionen aufbauen. Dokumentation und Git-Zuordnung bereits hier pflegen.

**Abnahme:** Reboot- und Ausfallfälle sind nachvollziehbar; ein Backup lässt sich tatsächlich zurückspielen; Monitoring liefert verwertbare Zustände und belastet den Funkpfad nicht unkontrolliert. Remote-Erreichbarkeit bleibt auf den ausdrücklich vorgesehenen Verwaltungsbereich begrenzt.

**Herkunft:** H2, H6.1/H6.4 und die früh vorzuziehenden Teile von H8.2–H8.4. Bestehende Observability-Komponenten berücksichtigen. [R8]

### M3 – Bedienung, Rechte und kontrollierte Updates

**Priorität P1; abhängig von belastbarer Einzelzelle.** Bestehendes Dashboard und zentrale Identitätsplanung zusammenführen. Zuerst Ansichts- und Schreibrechte für eine Pilot-TBS, danach weitere Dienste. API-Verträge vor einer plattformübergreifenden App festlegen. OTA benötigt festgelegte Freigabe-, Fehler- und Rollbackregeln.

**Abnahme:** Erlaubte und abgewiesene Aktionen werden für Rollen und konkrete Ressourcen geprüft. Ein Ausfall der Anmeldung eröffnet keinen ungeschützten Zugriff und beendet nicht allein deshalb den laufenden Funkbetrieb. Update und Rückkehr zum bekannten guten Stand sind reproduzierbar. Push-/Telegram-Zustellung wird separat geprüft.

**Herkunft:** H2.3/H2.5, H4 und H7.2; NETCORE-IAM-01 ist eine vorhandene Planungsgrundlage, kein bereits abgeschlossenes Ergebnis. [R5][R11]

### M4 – Mediathek, virtueller Sprecher und TTS auf einer Station

**Priorität P1; abhängig von M1 sowie kontrollierter Bedienung.** Vorhandene Media Library und TBS-Audioplayer verwenden. Zielgruppe beziehungsweise Zielteilnehmer, Sprecheridentität, Freigabe, Reihenfolge, Priorität, Abbruch und Audit festlegen. Zunächst eine Station, noch kein synchrones Multi-Node-Playout.

**Abnahme:** Ein freigegebenes Asset beziehungsweise TTS wird über den vorgesehenen Ruf am echten Funkgerät gehört. Erfolg, Ablehnung, Abbruch, fehlende Quelle und Neustart während eines Jobs sind unterscheidbar. Shadow-Jobs bleiben ausdrücklich Nicht-Sendetests.

**Herkunft:** H5.1/H5.2/H5.4 und TTS-Anteil H5.5. [R6][R7]

### M5 – Sensorik und deterministische Aktorik

**Priorität P1/P2; abhängig von M2, für Durchsagen zusätzlich M4.** Sensoren und reale I/O-Adapter auswählen; Einheiten, Abfrageintervalle, Grenzwerte, Hysterese und sichere Zustände festlegen. MQTT-/Command-Ack-Policies zuerst mit virtuellen Lab-Aktoren, danach einzeln mit realen Ausgängen abnehmen. Temperaturgetriggerte TTS ist ein nachgelagerter Workflow, kein Einstieg in den HF-Betrieb.

**Abnahme:** Jede erlaubte Aktion ist einem Eingang, einer Policy und einer Quittierung zugeordnet. Sensorfehler, Verbindungsabbruch und Neustart führen in den definierten sicheren Zustand. Lüfter, Relais, Buzzer und Displays werden nicht allein aufgrund einer erfolgreichen MQTT-Verbindung als funktionsfähig markiert.

**Herkunft:** H5.5 und H6.1–H6.4. KI wird davon getrennt. [R9]

### M6 – Zwei Standorte: Routing vor nahtlosem Zellwechsel

**Priorität P1/P2; abhängig von lokalem Funktions- und Recovery-Nachweis.** Zweite TBS zunächst unabhängig abnehmen. Danach Teilnehmerlage, Gruppen, SDS-Routing, Audio-Routing und Floor-Verantwortung zwischen zwei Standorten prüfen. Erst anschließend Zellwechsel, Context Transfer und Rufwiederherstellung. Zeit- und Synchronisationsbedarf anhand des konkreten Szenarios spezifizieren.

**Abnahme:** Ein definierter Ruf-/SDS-Ablauf über beide Standorte ist belegt. Trennung und Wiederkehr des Backhauls erzeugen nachvollziehbare Zustände. Ein echter Zellwechsel wird getrennt von einem Komponenten-Test protokolliert; Audio-Unterbrechung und Rufzustand werden gemessen beziehungsweise nachvollzogen. „Nahtlos“ wird nur bei entsprechendem Nachweis verwendet.

**Herkunft:** H3.1–H3.3/H3.5 und der standortübergreifende Anteil H5.3; vorhandene Core-/Restore-Bausteine nicht durch unbestimmte Shadow-Konstrukte ersetzen. [R10][R12][R13]

### M7 – Regionen, Redundanz und experimentelle Optimierung

**Priorität P2; abhängig von M6 und belastbaren Last-/Ausfalldaten.** Master-/Leader-Verantwortung, Replikation, Wiederanlauf, Split-Brain-Vermeidung und regionale Vermittlung spezifizieren. Shadow-GSSI, Meta-Sync, Watchtower und Shadow-Cluster erst weiterführen, wenn ihr konkreter Nutzen gegenüber vorhandenen Komponenten beschrieben ist. KI-Wartung und KI-Loadbalancing benötigen Datenbasis, Vergleich gegen deterministische Regeln und begrenzte Eingriffsrechte.

**Abnahme:** Vorab definierte Fehlerfälle und Lastgrenzen sind nachgewiesen. Automatische Umschaltung wird mit Trennung, Rückkehr und widersprüchlichen Zuständen getestet. Ein KI-Vorschlag gilt nicht ohne Vergleich und Ausfallregeln als besser oder sicherer.

**Herkunft:** H3.4, H6.5 und H7.1/H7.3–H7.5. Bis dahin Ideen-/Forschungskandidaten, keine Zusagen.

### M8 – Freigabestand und Langzeitbetrieb

**Priorität releaseabhängig; Tests und Dokumentation begleiten bereits M0–M7.** Den historischen Testwunsch **über 72 Stunden** als dokumentierten Meilenstein beibehalten; Thermal-, Netz- und Lastfälle sowie Backup/Restore und Release-Reproduzierbarkeit gemeinsam betrachten. Beta-/Club-Verteilung erst für einen klar beschriebenen Funktionsumfang.

**Abnahme:** Bekannte Grenzen, Konfiguration, Installationsweg, Rollback, Fehlerbehebung und Testbericht passen zum ausgelieferten Stand. Die Wahl von Wiki.js versus vorhandener Repository-/Wiki-Dokumentation bleibt eine Betriebsentscheidung und kein Hindernis für den ersten Funkerfolg.

**Herkunft:** H8 vollständig; H8.2–H8.4 werden nicht bis zum Projektende aufgeschoben.

## 13. Konkrete nächste Schritte und noch offene Entscheidungen

**Unmittelbar:** Den tatsächlichen Stationsstand erfassen und M0 mit einem vorgesehenen Funkgerät belegen. Danach lokale Sprache/SDS und die gezielten Restore-/Recovery-Fälle abnehmen. Für bereits vorhandene Komponenten lautet der Auftrag zunächst „prüfen und belegen“, nicht „noch einmal neu bauen“.

**Vor Remote-Ausbau:** NETCORE-IAM-01, lokale Dashboard-Zugänge und Open-Lab-Verwaltung konsistent behandeln. Maschinenidentitäten, menschliche Rollen und Funkteilnehmerkennungen getrennt planen. [R5][R6][R8][R9][R11]

**Vor Multi-Site:** Konkrete Ruf-/Datenfälle, Routingzuständigkeiten, Floor-Besitz, Ausfallregeln und Zeitbedarf festlegen. Die vorhandenen Restore-Testgerüste um die fehlenden relevanten Fälle ergänzen und anschließend reale TBS/MS-Tests durchführen. [R12][R13]

**Offene Nebenentscheidungen:** Bedeutung und Nutzen von Shadow-GSSI/Meta-Sync/Watchtower klären; App-Plattformumfang einschließlich iOS bestimmen; Push-/Telegram-Ziele festlegen; virtuelle Sprecher-ISSI und Audioziele zuordnen; Signierungsgegenstand definieren; Sensor-/Aktorhardware und Fail-Safe-Zustände auswählen; OTA-Rollback festlegen; Datenbasis und Eingriffsgrenzen für KI bestimmen; Wiki-/Release-Verteilungsweg auswählen; gegebenenfalls das damals nur angebotene SVG-/PNG-Diagramm neu beauftragen.

Keine dieser Entscheidungen wird durch diese Archivierung automatisch getroffen. Es wurden keine neuen Issues, PRs, Produktivkonfigurationen oder Automationen angelegt.

## 14. Nachvollziehbare Repository- und externe Quellen

Alle R-Verweise beziehen sich auf **denselben geprüften Commit**. Angegebene Zeilenbereiche sind die gelesenen Originaldateibereiche, keine Aussage über eine vollständige Prüfung des jeweiligen Moduls.

| Referenz | Datei / Umfang | Verwendung |
|---|---|---|
| [R1] | `README.md`, vollständig | Dokumentierter Versions-/SIP-Stand. |
| [R2] | `Cargo.toml`, Zeilen 1–170 | Workspace und enthaltene Bausteine. |
| [R3] | `bins/bluestation-bs/Cargo.toml`, Zeilen 1–100 | Paket, Binary, Default-Features, widersprüchlicher Packaging-Kommentar. |
| [R4] | `config.toml`, Zeilen 1–160 | Betriebsmodus, RF-/Netz-/Zellwerte, Fallback- und Diagnosekommentare. |
| [R5] | `crates/tetra-config/src/bluestation/sec_dashboard.rs`, vollständig | Dashboard-Defaults und Konfigurationsvalidierung. |
| [R6] | `system-backend/media-library/src/main.rs`, vollständig | Dienststart, TTS-/Worker-/HTTP-Verdrahtung und Open-Lab-Warnung. |
| [R7] | `system-backend/media-library/README.md`, Zeilen 1–145 | Audioformate, Betriebsmodi, Playout-Verträge. |
| [R8] | `system-backend/observability/README.md`, bis Zeile 145 / verfügbares Dateiende | Monitoring, Ports, Ingest und ausdrücklich fehlende Produktions-/HA-Funktionen. |
| [R9] | `system-backend/iot-gateway/README.md`, bis Zeile 135 / verfügbares Dateiende | MQTT, HA, Homematic, Policies und Persistenz. |
| [R10] | `system-backend/roadmap.md`, Zeilen 1–170 | Strategische Reihenfolge und als Dokumentationsbehauptungen behandelte Umsetzungsstände. |
| [R11] | `Docs/CENTRAL_IDENTITY_RBAC_ROADMAP.md`, Zeilen 1–115 | Geplante zentrale Anmeldung/RBAC und ausdrücklich offene Produktauswahl. |
| [R12] | `crates/tetra-entities/src/cmce/subentities/cc_bs/procedures/restoration.rs`, vollständig | Statische Restore-Befunde. |
| [R13] | `crates/tetra-entities/tests/test_two_cell_call_restore.rs`, Zeilen 1–100 | Art und Grenze des Komponenten-Testgerüsts. |
| [R14] | Root-Git-Tree des geprüften Commits | Existenz der genannten Verzeichnisse. |
| [E1] | Offizielle osmo-tetra-README, Abschnitt „Transmitter Program“, am 04.10.2026 über öffentliche Primärquelle eingesehen | Einschränkung der früheren pauschalen Installationsannahme; keine vollständige Upstream-Codeprüfung. |

Die direkte Osmocom-Wikiseite war bei der externen Abfrage durch eine Zugriffsschutzseite blockiert; für E1 wurde die offizielle GitHub-README verwendet. Nicht passende öffentliche Suchtreffer wurden nicht als technische Belege übernommen.

Für den frühen Roadmap-Entwurf sind **kein Implementierungscommit und keine PR** sicher zugeordnet. Der Abgleich verwendet gepinnte Quellen; `main` und `Archiving` sind nicht als identisch behandelt.

### Passende Nachbararchive zur späteren Fortsetzung

Die folgenden Dateien waren im geprüften Archivindex bereits eingetragen. Sie wurden für diese Zusammenfassung **nicht vollständig neu ausgewertet** und dienen als Navigationshinweise, nicht als zusätzliche historische Beweise:

- [SXceiver-Erstinstallation, HamTetra-DMO und BlueStation](2026-10-04_basisstation-erstinstallation-sxceiver-hamtetra-dmo-und-bluestation.md)
- [DualCarrier-Portierung und SXceiver-Hotfixes](2026-10-03_flowstation-dualcarrier-portierung-sxceiver-hotfixes.md)
- [Gesprächssimulator und lokale Mikrofon-/PTT-Sprechstelle](2026-10-03_gespraechssimulator-issi-und-lokale-mikrofon-ptt-sprechstelle.md)
- [Android-Control-App und Hybrid-Manager](2026-10-03_android-control-app-flask-api-und-hybrid-manager.md)

## 15. Anhänge und Bildinventar

### 15.1 Umgang mit den PDF-Quellen

Es stehen **24 Einzel-PDFs mit zusammen 3.961 Seiten** sowie **`ETSI.pdf` mit 4.100 Seiten** zur Verfügung. Der Anfang der Sammeldatei trägt denselben Titel wie EN 300 812 V2.1.1. Eine vollständige Abschnitts-/Duplikatzuordnung der Sammeldatei wurde nicht erstellt; insbesondere wird aus der ähnlichen Anfangsseite keine Byteidentität aller Inhalte abgeleitet.

Die PDFs sind technische Referenzen, **keine Belege dafür, dass NetCore die jeweiligen Funktionen implementiert**. Auch ihre Bereitstellung im geprüften Projektkontext beweist nicht, dass sie alle bereits beim ursprünglichen Roadmap-Austausch vorlagen. Die Dateien mit 2026-Draft-Titel werden nicht rückwirkend als endgültige historische Grundlage behandelt.

Die Original-PDFs wurden in diesem Auftrag nicht erneut als Binärdateien ins Repository kopiert. Bewahrt werden Dateinamen, Titel-/Versionszuordnung, Umfang und Prüfsummen. Die folgende thematische Zuordnung ist eine Archivierungshilfe, keine nachträgliche Beauftragung sämtlicher in den Normen genannten Dienste.

| ID | Verfügbare Datei | Titel / Version laut Titelblatt | Seiten | Relevanz für die Fortsetzung |
|---|---|---|---:|---|
| A01 | `en_30039201v010601p.pdf` | EN 300 392-1 V1.6.1, 2020-04: General network design | 182 | Netzarchitektur und Identitäten. |
| A02 | `en_30039202v030801p.pdf` | EN 300 392-2 V3.8.1, 2016-08: Air Interface | 1445 | Luftschnittstelle und lokale TBS-/MS-Verfahren. |
| A03 | `en_3003920303v010301p.pdf` | EN 300 392-3-3 V1.3.1, 2011-11: ANF-ISIGC | 251 | Gruppenrufe über ISI. |
| A04 | `en_3003920304v010301p.pdf` | EN 300 392-3-4 V1.3.1, 2010-08: ANF-ISISDS | 28 | SDS zwischen Systemen. |
| A05 | `en_3003920308v010401p.pdf` | EN 300 392-3-8 V1.4.1, 2020-04: Generic Speech Format Implementation | 22 | Sprachtransport am ISI. |
| A06 | `en_3003920313v010201p.pdf` | EN 300 392-3-13 V1.2.1, 2020-04: transport layer independent ANF-ISIGC | 191 | Transportunabhängige ISI-Gruppenrufverfahren. |
| A07 | `en_3003920315v010500a.pdf` | **Draft** EN 300 392-3-15 V1.5.0, 2026-04: transport layer independent ANF-ISIMM | 380 | Mobility zwischen Systemen; Draft-Status der gelieferten Datei beachten. |
| A08 | `en_30039205v020701p.pdf` | EN 300 392-5 V2.7.1, 2020-04: Peripheral Equipment Interface | 320 | Endgeräte-Peripherie, nicht mit der TBS-Web-API gleichsetzen. |
| A09 | `en_30039207v030501p.pdf` | EN 300 392-7 V3.5.1, 2019-07: Security | 216 | TETRA-Sicherheitsverfahren; nicht gleichbedeutend mit Web-IAM. |
| A10 | `en_30039209v010701p.pdf` | EN 300 392-9 V1.7.1, 2020-04: General requirements for supplementary services | 46 | Zusatzdienst-Rahmen. |
| A11 | `en_3003921006v010401p.pdf` | EN 300 392-10-6 V1.4.1, 2006-08: Call Authorized by Dispatcher, Stage 1 | 20 | Dispatcherfreigabe als mögliche spätere Referenz; kein hier beschlossener Zusatzumfang. |
| A12 | `en_3003921018v010301p.pdf` | EN 300 392-10-18 V1.3.1, 2003-10: Barring of Outgoing Calls, Stage 1 | 17 | Rufbeschränkungen als Referenz. |
| A13 | `en_3003921101v010201p.pdf` | EN 300 392-11-1 V1.2.1, 2004-01: Call Identification, Stage 2 | 44 | Rufidentifikation / Funktionsmodell. |
| A14 | `en_3003921114v010101p.pdf` | EN 300 392-11-14 V1.1.1, 2002-07: Late Entry, Stage 2 | 23 | Später Einstieg in laufende Gruppenkommunikation. |
| A15 | `en_3003921117v010102p.pdf` | EN 300 392-11-17 **V1.1.2**, 2002-01: Include Call, Stage 2 | 18 | Teilnehmerhinzunahme; Titelblatt maßgeblich, keine V1.1.1 erfinden. |
| A16 | `en_3003921201v010202p.pdf` | EN 300 392-12-1 V1.2.2, 2007-08: Call Identification, Stage 3 | 56 | Signalisierung der Rufidentifikation. |
| A17 | `en_3003921216v010400a.pdf` | **DRAFT** EN 300 392-12-16 V1.4.0, 2026-03: Pre-emptive Priority Call, Stage 3 | 67 | Priorität / Verdrängung; gelieferte Entwurfsfassung. |
| A18 | `en_30039401v030301p.pdf` | EN 300 394-1 V3.3.1, 2015-04: Conformance testing specification, Radio | 169 | Referenz für RF-Prüfplanung, keine vorhandene Zertifizierung. |
| A19 | `en_30039502v010303p.pdf` | EN 300 395-2 V1.3.3, 2025-02: Speech codec for full-rate traffic channel, TETRA codec | 94 | Codec- und Sprachkanalreferenz. |
| A20 | `en_300812v020101p.pdf` | EN 300 812 V2.1.1, 2001-12: Security aspects, SIM-ME interface | 156 | Karten-/Endgeräteschnittstelle; kein Nachweis eines TBS-Kartenlesers. |
| A21 | `es_20081201v020205p.pdf` | ES 200 812-1 V2.2.5, 2003-12: UICC physical and logical characteristics | 8 | UICC-/TSIM-ME-Referenz. |
| A22 | `es_20081202v020401m.pdf` | **Final draft** ES 200 812-2 V2.4.1, 2005-08: Characteristics of the TSIM application | 139 | TSIM-Anwendung; Entwurfskennzeichnung erhalten. |
| A23 | `ets_30039214e01v.pdf` | **Final draft prETS** 300 392-14, 1997-09: PICS proforma specification | 61 | Historische PICS-Vorlage, kein ausgefüllter NetCore-Konformitätsnachweis. |
| A24 | `ts_10081201v020205p.pdf` | TS 100 812-1 V2.2.5, 2003-10: UICC physical and logical characteristics | 8 | Von A21 getrennte Dokumentart/-datierung. |
| A25 | `ETSI.pdf` | Sammeldatei; erstes Titelblatt EN 300 812 V2.1.1, 2001-12 | 4100 | Größerer Referenzbestand; vollständige Zusammensetzung nicht kartiert. |

**Call Authorized by Dispatcher, BOC, Call Identification, Late Entry, Include Call, PPC, TSIM und OTAR** sind durch Normanhänge allein keine zusätzlichen historischen Anforderungen. Umfang und Priorität müssen separat entschieden werden.

### 15.2 SHA-256-Manifest der zugänglichen Originaldateien

```text
788722e566098957bea1a9a5096e9dae1eace1da9f5d38c530c80230e5b9b9cb  en_30039201v010601p.pdf
3f07b1e4ad73fabc16277a900006fc19844b3a882bbc2bc1d74d78f6156daf28  en_30039202v030801p.pdf
94ca61038b3ff826e6adcfde56e00473dd996e1a47f9f84e7b9d2aa6c3133fd2  en_3003920303v010301p.pdf
8a38cc6238c6da726e4f07d7d377b2c827447a1448a60ee5ef2a126cb3a0ef9d  en_3003920304v010301p.pdf
4d993a25e35da7b1f2291ae281cb3ea5b3a7b702ab7eb3233bdf430a2c70909d  en_3003920308v010401p.pdf
b47d63853f83a6a4adda08c0e325c8ca13d358e2f8840bfba4bce430e600b1bd  en_3003920313v010201p.pdf
e959709667192e22d2dfeea51b48f9a579a4c107d952999056b7fad93a6d2100  en_3003920315v010500a.pdf
10aaf78988ae4080c3dfe90165ee7f4f3fe8eda402e09fc1e6fa0a0ad890758d  en_30039205v020701p.pdf
df47af9a642b6eba7d7d5cf5fb7aa3f961e9e03bb912316bd2d189ce47344e08  en_30039207v030501p.pdf
cad45938bcdf2fa4e8063d4aa9e7d7c06c45f729d3c55dc033e00c27af9f2e06  en_30039209v010701p.pdf
32b6ee43602ef88a0b5c209b1c69ad35ba8b2e660502257c2dfaa6205a346523  en_3003921006v010401p.pdf
4cc10bcd94d39201538639cefb529809729563f13f87cfe6ae52b44c253b0b8a  en_3003921018v010301p.pdf
852c17ea566757e80ecf0d2718ae7839d9e218aa4384b63d1277ed7e25d86c69  en_3003921101v010201p.pdf
ba1882df71dbb84a0c4f0f7dcea1dc8df968044457da03df9290a50e8821df32  en_3003921114v010101p.pdf
69ce800e352b1ecfc337416fd7b54c1bdf2d9270171af519ac2f5acc1ba85ca6  en_3003921117v010102p.pdf
4d5b56a188fb73c417d67b27da619cad2e49efb3b57306fd4ae51e140c018018  en_3003921201v010202p.pdf
c0ee7859f448c4072befc25905c29b751b5151d3590a880a2d654309a3ba6c02  en_3003921216v010400a.pdf
2d9c327e5ccf1476335e1b7a4f58a0ca7afc93911c30e8aad5ed7dc9ccf3276a  en_30039401v030301p.pdf
ac716ca18082cc0fd2ee768a14f03deb48fa78a77e83751b1a6b319c7fc2106a  en_30039502v010303p.pdf
196effeaae473a98244c89538e1562daa58bb1d74cbfe7de78e58ceeafbad18b  en_300812v020101p.pdf
346dc545049a7397ca86831731bbc05e96f61ff047d6e7f6b7de56da3aa408e9  es_20081201v020205p.pdf
330f045a908eefd249fd7450e5b71bd08e8a7b20ce5b86dc9b87c9138a5c7268  es_20081202v020401m.pdf
2b703f3ebb883c90dab40e1b3d1a634a6acf9c9a20d468fd108ea02226c2bf2c  ets_30039214e01v.pdf
96df75f5ddf1c3ae4ac75f796ee97babca68fa81aca22f39ba0e0658bd57eed1  ts_10081201v020205p.pdf
9434dad1e7bc80ca39b0edadd8e3b9995fda5ae5f5c3dd565af05708d5059e38  ETSI.pdf
```

### 15.3 Bildbestand

**Keine eigenständigen historischen Bilder verfügbar.** Die Bilddatei-Abfrage lieferte keine Treffer; der ursprüngliche lokale Dateibestand enthielt nur die 25 PDFs. Sichtbare Titelblatt-/Seitenbilder sind Renderansichten dieser PDF-Dokumente und keine eigenständigen hochgeladenen Stationsfotos oder Roadmap-Grafiken.

Originalbilder fehlen. Das SVG-/PNG-Angebot bleibt eine nicht umgesetzte Nebenidee. Später verfügbare Bilder können mit Herkunft und eindeutigem Themenbezug ergänzt werden.

## 16. Abschluss und Übergabe

Erhalten sind sämtliche 40 Schritte des historischen Entwurfs, Gesamtvision und Grafikidee. Verbindliche Ausgangsrichtung: **mit einer tatsächlich sendenden Station und belegter Geräteanmeldung beginnen; anschließend Basisstationsfunktionen schrittweise ausbauen.**

Die ergänzende Prüfung zeigt vorhandene konkrete NetCore-Bausteine, ohne sie pauschal als betriebsfertig zu bestätigen. Hauptaufgaben sind die Live-Baseline, lokale Kommunikations-/Restore-Abnahme, abgesicherte Verwaltung und anschließend die kontrollierte Nutzung vorhandener Audio-, IoT- und Multi-Site-Pfade. Historische Namen, alte Funkparameter, fehlende Backups und noch nicht geprüfte Funktionen bleiben erkennbar von nachgewiesenem Code getrennt.

Für die Umsetzung zuerst Betriebsziel, vorhandene Hardware, tatsächliche Revision und Kriterien des ersten Meilensteins festlegen.

[R1]: https://github.com/JanHG98/netcore-tetra/blob/530170d2deda13d531e5e200d0bde7ab97acbdcd/README.md
[R2]: https://github.com/JanHG98/netcore-tetra/blob/530170d2deda13d531e5e200d0bde7ab97acbdcd/Cargo.toml#L1-L170
[R3]: https://github.com/JanHG98/netcore-tetra/blob/530170d2deda13d531e5e200d0bde7ab97acbdcd/bins/bluestation-bs/Cargo.toml#L1-L100
[R4]: https://github.com/JanHG98/netcore-tetra/blob/530170d2deda13d531e5e200d0bde7ab97acbdcd/config.toml#L1-L160
[R5]: https://github.com/JanHG98/netcore-tetra/blob/530170d2deda13d531e5e200d0bde7ab97acbdcd/crates/tetra-config/src/bluestation/sec_dashboard.rs
[R6]: https://github.com/JanHG98/netcore-tetra/blob/530170d2deda13d531e5e200d0bde7ab97acbdcd/system-backend/media-library/src/main.rs
[R7]: https://github.com/JanHG98/netcore-tetra/blob/530170d2deda13d531e5e200d0bde7ab97acbdcd/system-backend/media-library/README.md#L1-L145
[R8]: https://github.com/JanHG98/netcore-tetra/blob/530170d2deda13d531e5e200d0bde7ab97acbdcd/system-backend/observability/README.md
[R9]: https://github.com/JanHG98/netcore-tetra/blob/530170d2deda13d531e5e200d0bde7ab97acbdcd/system-backend/iot-gateway/README.md
[R10]: https://github.com/JanHG98/netcore-tetra/blob/530170d2deda13d531e5e200d0bde7ab97acbdcd/system-backend/roadmap.md#L1-L170
[R11]: https://github.com/JanHG98/netcore-tetra/blob/530170d2deda13d531e5e200d0bde7ab97acbdcd/Docs/CENTRAL_IDENTITY_RBAC_ROADMAP.md#L1-L115
[R12]: https://github.com/JanHG98/netcore-tetra/blob/530170d2deda13d531e5e200d0bde7ab97acbdcd/crates/tetra-entities/src/cmce/subentities/cc_bs/procedures/restoration.rs
[R13]: https://github.com/JanHG98/netcore-tetra/blob/530170d2deda13d531e5e200d0bde7ab97acbdcd/crates/tetra-entities/tests/test_two_cell_call_restore.rs#L1-L100
[R14]: https://github.com/JanHG98/netcore-tetra/tree/530170d2deda13d531e5e200d0bde7ab97acbdcd
[E1]: https://github.com/osmocom/osmo-tetra/blob/master/README.md
