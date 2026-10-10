# Brainstorming: NetCore-Tetra-Gruppen und Zebra-Test

**Archivhinweis, 09.10.2026:** Diese Datei dokumentiert das im Titel beziehungsweise Quellenrahmen genannte Gespräch und dessen damalige Entscheidungen. Historische Kommandos, Planungen, Versionsangaben und Fehlerbilder bleiben erhalten; der heutige technische Stand steht in der [Gesamtroadmap](../roadmaps/gesamtroadmap.md) und in den [aktuellen Dienstanleitungen](../services/README.md). Die Archivaufnahme bestätigt keine spätere Implementierung oder Live-Abnahme.

**Stand der Notizen und ergänzenden Prüfungen: 2026-10-04.** Historische Entwürfe, nachgewiesene Umsetzung und ausgeführte Tests sind jeweils getrennt gekennzeichnet.

**Arbeitsrichtung:** Frühere Gruppenkandidaten einschließlich Zebra-Test konsolidieren. Namen und einige Zwecke sind erhalten; GSSIs, Mitgliedschaften, Berechtigungen und die vollständige Liste bleiben offen.

## 1. Kontext und Quellenlage

- Thema: Wiederherstellung der früheren Gruppenübersicht und Bedeutung von „Zebra test“.
- Erstellung: 2026-10-04, Zeitzone Europe/Berlin.
- Repository: https://github.com/JanHG98/netcore-tetra
- Geprüfter und ausschließlich beschriebener Zielbranch: `Archiving`.
- Geprüfter Ausgangscommit: `d0e9781c44f9c3e7418c67d67c30249d7444d8f6`.
- Ausgangsbaum: `e2f3455539a071ae81a85cdb606b73e6eb010e06`.

**Quellenlage:** Die frühere Gruppenplanung ist nur teilweise erhalten: Kategorien, ausgewählte Namen und drei Zweckbeschreibungen. Die Auszüge sind ergänzend dem 07.10.2025, 22:21–22:24 UTC zugeordnet; ein Originalexport zur unabhängigen Bestätigung fehlt.

Eine vollständige Gruppenliste liegt nicht vor. Fehlende Zwecke und Adressen werden nicht aus Namen abgeleitet.

Discovery, VPN-Automatik und HF-Aufbau gehören zu anderen Projektbereichen und werden hier nicht als Gruppenfestlegungen behandelt.

## 2. Ziel, Ausgangslage und behandelte Themen

Ziel ist die Wiederherstellung einer verständlichen Gruppenübersicht mit kurzen Zweckbeschreibungen. Besonderer Klärungspunkt ist die Bedeutung von „Zebra-Test“.

Geräteprogrammierung, Neuvergabe von GSSIs, Dienstinstallation und ausgelöste Frequenzwechsel sind hier nicht als umgesetzt belegt.

Die historischen Kandidaten werden mit dem Repository-Stand vom 2026-10-04 abgeglichen.

## 3. Statusbegriffe und endgültig belegte Anforderungen

| Status | Bedeutung in diesem Archiv |
|---|---|
| Idee | Historisch vorgeschlagener Zweck oder Einsatz; technische Umsetzung fehlt |
| beschlossen/geplant | Ausdrücklich vereinbarter Arbeitsschritt oder Arbeitsauftrag |
| implementiert | Zugehöriger Quellcode im geprüften Repository vorhanden; kein Betriebsnachweis |
| getestet | Test tatsächlich ausgeführt und Ergebnis dokumentiert |
| im Betrieb bestätigt | Nachweis aus realem Zielsystem oder Funkbetrieb; hier nicht vorhanden |

Eine vollständige Vergabeliste sowie verbindliche GSSIs, Mitglieder, Berechtigungen und Endgeräteprofile fehlen für die historischen Gruppen.

Spätere Korrekturen zur Gruppenliste sind in den verfügbaren Unterlagen nicht enthalten.

## 4. Historischer Inhalt: Zebra-Test und Gruppenübersicht

### 4.1 Zebra-Test

„Zebra-Test“ ist als **Hybrid-Testgruppe mit Frequenzwechsel** beziehungsweise „Handover-Playground“ in der Kategorie Sandbox/Debug/Spezial beschrieben. Die Testideen umfassen:

- wechselnde Frequenz- und Kanalparameter;
- TMO-/DMO-Übergänge;
- Frequenz-Handover;
- Messung von Umschaltzeiten.

Status: **Historische Idee, in Auszügen belegt.** Funktionierende Frequenzsteuerung, TMO-/DMO-Übergänge oder seamless Handover sind nicht nachgewiesen.

Technische Präzisierung für die Fortsetzung: Ein Gruppenname erzeugt keine Handover-Funktion. Zu unterscheiden sind Gruppenadressierung, Zell-/Trägerwechsel und ein Betriebsartwechsel zwischen TMO und DMO. Der historische Sammelbegriff ist für eine Testplanung zu ungenau und muss vor einer Umsetzung in getrennte Szenarien aufgelöst werden. „Zebra-Test“ allein belegt auch keinen Bezug zu einem Zebra-Handheld.

### 4.2 Wiedergefundene Namen

| Historischer Name | Belastbar wiedergefundene Erklärung | Status |
|---|---|---|
| Core-Group01 | Zentrale Steuer-/Admin-Gruppe | Historische Idee |
| Zebra-Test | Frequenzwechsel-/Hybridtests und Handover-Playground | Historische Idee |
| Echo-Loop | Voll-Loopback-Test | Historische Idee |
| Node-Maint01 | Zweck im verfügbaren Auszug nicht enthalten | Name wiedergefunden |
| Net-Control | Zweck im verfügbaren Auszug nicht enthalten | Name wiedergefunden |
| OTA-Update | Zweck im verfügbaren Auszug nicht enthalten | Name wiedergefunden |
| GSSI-Master | Zweck im verfügbaren Auszug nicht enthalten | Name wiedergefunden |
| Cluster-TMO1/DMO1 | Genau diese verkürzte Schreibweise zurückgegeben; getrennte Originalzeilen fehlen | Name/Schreibweise unvollständig |
| Fallback-TX | Zweck im verfügbaren Auszug nicht enthalten | Name wiedergefunden |
| Echo-Test | Zweck im verfügbaren Auszug nicht enthalten | Name wiedergefunden |
| VPN-Check | Zweck im verfügbaren Auszug nicht enthalten | Name wiedergefunden |
| Sys-Monitor | Zweck im verfügbaren Auszug nicht enthalten | Name wiedergefunden |
| Audio-Demo1 | Zweck im verfügbaren Auszug nicht enthalten | Name wiedergefunden |
| Safe-Mode | Zweck im verfügbaren Auszug nicht enthalten | Name wiedergefunden |
| Lighthouse | Zweck im verfügbaren Auszug nicht enthalten | Name wiedergefunden |
| Dev-Test01 | Zweck im verfügbaren Auszug nicht enthalten | Name wiedergefunden |

Der Suchauszug enthielt zusätzlich „und weitere“. Die Tabelle ist deshalb ausdrücklich unvollständig. Für keine dieser historischen Gruppen wurden GSSI, Netzkennung, Mitglieder, Priorität, Berechtigungen, konkrete Frequenzen, Timeouts oder Zuordnung zu Endgeräten zurückgegeben.

Historische Kategorien: System & Steuerung; Netzbetrieb & Tests; Monitoring & Analyse; Audio/Demo/Trigger; Utility & Dynamik; Sicherheit & Events; Spezialrollen/Cluster; Sandbox/Debug/Fun. Eine zuverlässige Zuordnung jedes Namens zu einer Kategorie fehlt.

## 5. Zusätzlich überprüfter Repository-Stand vom 04.10.2026

Die folgenden Angaben stammen aus Dateien am oben genannten Ausgangscommit, nicht aus einer beobachteten Installation.

### 5.1 Gruppenbegriffe

`wiki/Groups.md` beschreibt TETRA-Gruppen mit den Feldern `gssi`, `name`, `short`, `type`, `owner`, `color`, `visible` und `notes`. Ein Directory-Eintrag allein affiliiert kein Funkgerät. Bei Aufzeichnung ausgewählter Gruppen ist die lokale Liste für `recording.mode = "selected_groups"` zusätzlich erforderlich.

`wiki/Device-Groups.md` beschreibt dagegen organisatorische Gerätegruppen mit `group_id`, optionaler `opta`, Namen, Typ, Eigentümer, Farbe, `status_sync`, Sichtbarkeit und Hinweisen. Diese Gruppen können U-STATUS zwischen Mitgliedern verteilen; sie ersetzen keine Sprachgruppe und keine GSSI-Affiliation.

Die Unterscheidung ist für die historische Liste wesentlich: Ein technischer oder organisatorischer Name ist noch keine eindeutig spezifizierte TETRA-Sprachgruppe.

### 5.2 Vorhandener Group Core

`system-backend/group-core/` enthält einen zentralen Dienst für Gruppenprofile, Teilnehmermitgliedschaften, Affiliationen und DGNA. Geprüft wurden README, Beispielkonfiguration, HTTP-Quellcode, Zustandsquellcode, Installationsskript und systemd-Unit.

Im Quellcode sind Gruppenanlage, Änderung und Löschung sowie Mitgliedschafts- und DGNA-Routen vorhanden. Damit sind diese Verwaltungspfade **implementiert auf Quellcodeebene**. Die README beschreibt zusätzlich Richtlinienverteilung, lokale Zulassung auf der TBS sowie Mindestpriorität und Class of Usage. Diese vollständigen Ende-zu-Ende-Funktionen wurden bei dieser Archivierung nicht durch Funk- oder Integrationstests nachgewiesen.

### 5.3 Konfiguration, Schnittstellen und Abhängigkeiten

| Parameter/Pfad | Geprüfter Wert oder Bedeutung |
|---|---|
| HTTP/WebUI | Beispielbindung `0.0.0.0:8110` |
| Node Gateway | WebSocket-Verbindung auf `/ws/backend`; Beispieladresse enthält Platzhalter `10.0.1.XX:8080` |
| Wiederverbindung | `reconnect_secs = 5` |
| Daten | `/var/lib/netcore-group-core/groups.json` |
| Backup | `/var/lib/netcore-group-core/groups.json.bak` |
| Dienst | `netcore-group-core.service` |
| Binärdatei | `/usr/local/bin/netcore-group-core` |
| Laufzeitkonfiguration | `/etc/netcore/group-core.toml` |
| Dienstkonto | `netcore:netcore` |
| Gruppenpolicy | `allow_unlisted_groups = false`, `enforce_memberships = true` |
| Abgleich | `reconcile_registered = true`, `auto_sync = true` |
| Timeouts | Sync und DGNA jeweils 30 Sekunden |
| Sicherheit | Beispielmodus `open_lab`, Remote Management aktiviert |
| Limits | 2.097.152 Byte Requestbody; 65.536 Gruppen; 1.000.000 Mitgliedschaften; History 2.000 |

Die HTTP-Implementierung enthält unter anderem `/api/v1/groups`, `/api/v1/groups/{gssi}`, `/api/v1/memberships`, `/api/v1/affiliations`, `/api/v1/dgna`, `/api/v1/sync`, `/api/v1/export.json`, `/health/live`, `/health/ready` und `/metrics`.

Die Dokumentation verlangt für den beschriebenen Open-Lab-Ausbau ein isoliertes Testnetz und nennt fehlende Anmeldung/Tokens/TLS. Dies ist eine Eigenschaft des geprüften Repository-Stands, keine neue Sicherheitsentscheidung dieser Planung. Kompatible TBS benötigen laut README `group_policy`- und `dgna`-Capabilities; Subscriber Core, Call Control und SDS Router werden als spätere Abhängigkeiten genannt.

### 5.4 Tatsächlich vorhandener Beispielgruppeneintrag

`system-backend/directory/seed.json` enthält:

| GSSI | Name | Kurzname | Typ | Eigentümer |
|---|---|---|---|---|
| 15201 | NetCore Testgruppe | TEST | TMO Group | Jan |

Status: **Repository-Beispieldaten vorhanden**. Dieser Eintrag ist kein Nachweis für eine produktive Programmierung und keine belegte Zuordnung zu „Zebra-Test“.

In den gezielt gelesenen Group-Core-Dateien und dieser Seed-Datei wurde keine historische Zebra-Test-/Core-Group01-/Echo-Loop-Konfiguration nachgewiesen. Es erfolgte keine vollständige inhaltliche Suche über sämtliche Repository-Blobs. Deshalb wird keine globale Abwesenheit behauptet.

## 6. Befehle, Installation und Reparatur

Ausgeführte Installations- oder Reparaturbefehle für die historische Gruppenplanung sind nicht dokumentiert.

Das ergänzende `system-backend/group-core/install/install.sh` sieht einen Root-Aufruf vor, stoppt den bestehenden Dienst, entfernt das installierte Binary sowie ausgewählte Buildartefakte, baut mit `cargo build --release -p netcore-group-core`, installiert Binary/Konfiguration/Dienstkonto/Unit, ruft den gemeinsamen LXC-Netzwerkhelfer auf und aktiviert den Dienst mit `systemctl enable --now netcore-group-core.service`. Eine vorhandene Laufzeitkonfiguration wird nicht durch die Beispielkonfiguration ersetzt.

**Status: Skript gelesen, nicht ausgeführt.** Kein Build- oder Installationserfolg wird behauptet. Die Unit startet:

```text
/usr/local/bin/netcore-group-core --config /etc/netcore/group-core.toml
```

Vorgesehene spätere Diagnosebefehle, hier **nicht ausgeführt**:

```bash
systemctl status netcore-group-core.service
journalctl -u netcore-group-core.service --since today
curl http://127.0.0.1:8110/health/ready
curl http://127.0.0.1:8110/api/v1/groups
```

Sie gehören zu einer zukünftigen Abnahme auf dem Zielsystem, nicht zu einer Wiederherstellung des historischen Gruppenplans.

## 7. Fehler, Diagnose und Tests

Das Hauptproblem ist die **unvollständige Quellenlage**: Die ursprüngliche Gruppenliste konnte nur teilweise wiederhergestellt werden. Funktionsfehler im Funkbetrieb sind nicht belegt.

Eine mögliche Fehlinterpretation wäre, aus dem Namen Zebra-Test eine bereits funktionierende Handover-Automatik oder aus OTA-Update einen implementierten Updatekanal abzuleiten. Das ist nicht zulässig; die Namen beschreiben höchstens frühere Ideen.

Durchgeführt: Branch-/Commit-/Baumprüfung, gezielte Repository-Dateilesung und Deckblattinventarisierung aller 25 bereitgestellten PDFs. Nicht durchgeführt: Rust-Build, Unit-/Integrationstests, API-Aufrufe auf einem laufenden Group Core, TBS-Registrierung, Endgeräteprogrammierung, Gruppenruf, Loopback, Frequenzwechsel, TMO-/DMO-Wechsel und Handover-Messung. Vorhandene Testdateien im Repository sind kein ausgeführtes Testergebnis.

## 8. Verworfene oder ersetzte Ansätze

Keine ausdrückliche historische Verwerfung ist zugänglich. Für diese Archivierung wurde die freie Rekonstruktion unbekannter Gruppenzwecke verworfen, weil sie die ursprüngliche Planung verfälschen würde.

Es wurde auch keine GSSI aus einem Gruppennamen abgeleitet. Die ergänzende Wiki-Empfehlung fordert ein eigenständiges GSSI-Primärfeld. Die historische Testidee wird nicht als Ersatz für am 2026-10-04 vorhandene Group-Core-/Directory-Strukturen behandelt.

## 9. Offene Aufgaben und Roadmap-Kandidaten

Die folgende Reihenfolge ist eine **Empfehlung dieses Archivs**, keine wiedergefundene frühere Prioritätsvereinbarung.

1. **Gruppenbestand wiederherstellen:** Vollständige Liste mit Zwecken und eventuellen Korrekturen beschaffen.
2. **Gruppenregister konsolidieren:** Pro Gruppe Name, Kurzname, GSSI und Netzkontext, Zweck, Eigentümer, Mitglieder, zulässige Dienste, Priorität und Lebenszyklus festlegen. Historische Namen zunächst als Kandidaten führen.
3. **Begriffe trennen:** TETRA-Sprachgruppen, organisatorische Gerätegruppen und reine Diagnose-/Automationsfunktionen unterscheiden.
4. **Bestehende Dienste nutzen:** Directory-Darstellung und Group-Core-Policy abgleichen; keine zweite unabhängige Gruppendatenbank ohne geklärte Zuständigkeit einführen.
5. **Zebra-Test präzisieren:** Zellwechsel, Trägerwechsel und TMO-/DMO-Betriebsartwechsel getrennt planen. Für jedes Szenario Ausgangszustand, Trigger, erwartetes Verhalten, Messpunkte und Erfolgskriterien definieren.
6. **Abnahme dokumentieren:** Konfigurationsabgleich, tatsächliche Affiliation, Gruppenruf, DGNA und Mitgliedschaftspolicy zuerst belegen; erst danach behauptete Umschalt-/Handover-Funktionen bewerten.
7. **Echo-Loop abgrenzen:** Loopback-Pfad und Audio-/PTT-Abbruchbedingungen spezifizieren; die historische Bezeichnung enthält noch keinen ausführbaren Test.
8. **Bildbestand ergänzen:** Zugehörige Originalbilder nur mit eindeutiger Zuordnung aufnehmen.

Für Node-Maint01, Net-Control, OTA-Update, GSSI-Master, Cluster-TMO1/DMO1, Fallback-TX, Echo-Test, VPN-Check, Sys-Monitor, Audio-Demo1, Safe-Mode, Lighthouse und Dev-Test01 sind Zwecke ausdrücklich noch zu rekonstruieren.

## 10. Quellen, Anhänge und Bilder

### 10.1 Repository-Quellen

Alle nachfolgend genannten Dateien wurden am Ausgangscommit gelesen:

- `wiki/Groups.md`
- `wiki/Device-Groups.md`
- `wiki/Device#U2010Groups.md.md` (älterer HTML-Inhalt; keine zusätzliche historische Gruppenliste)
- `system-backend/directory/seed.json`
- `system-backend/group-core/README.md`
- `system-backend/group-core/config/group-core.example.toml`
- `system-backend/group-core/src/http.rs`
- `system-backend/group-core/src/state.rs`
- `system-backend/group-core/install/install.sh`
- `system-backend/group-core/systemd/netcore-group-core.service`
- `Docs/archive/README.md`

Es wurden keine zu diesem Planungsstand gehörenden Implementierungscommits oder PRs zuverlässig identifiziert.

### 10.2 Bereitgestellte PDF-Anhänge

Alle 25 Dateien sind lokal zugänglich; ihre Deckblätter wurden gelesen. Sie sind allgemeine TETRA-Normenquellen und belegen keine projektspezifische Gruppe namens Zebra-Test. Eine vollständige Normenanalyse war für die Wiederherstellung dieser Gruppenliste nicht erforderlich und wurde nicht behauptet.

| Dateiname | Deckblattkennung | SHA-256 |
|---|---|---|
| en_3003920308v010401p.pdf | ETSI EN 300 392-3-8 V1.4.1 (2020-04) | 4d993a25e35da7b1f2291ae281cb3ea5b3a7b702ab7eb3233bdf430a2c70909d |
| en_30039209v010701p.pdf | ETSI EN 300 392-9 V1.7.1 (2020-04) | cad45938bcdf2fa4e8063d4aa9e7d7c06c45f729d3c55dc033e00c27af9f2e06 |
| ts_10081201v020205p.pdf | ETSI TS 100 812-1 V2.2.5 (2003-10) | 96df75f5ddf1c3ae4ac75f796ee97babca68fa81aca22f39ba0e0658bd57eed1 |
| en_3003921201v010202p.pdf | ETSI EN 300 392-12-1 V1.2.2 (2007-08) | 4d5b56a188fb73c417d67b27da619cad2e49efb3b57306fd4ae51e140c018018 |
| en_3003920304v010301p.pdf | ETSI EN 300 392-3-4 V1.3.1 (2010-08) | 8a38cc6238c6da726e4f07d7d377b2c827447a1448a60ee5ef2a126cb3a0ef9d |
| en_3003921117v010102p.pdf | ETSI EN 300 392-11-17 V1.1.2 (2002-01) | 69ce800e352b1ecfc337416fd7b54c1bdf2d9270171af519ac2f5acc1ba85ca6 |
| en_3003921114v010101p.pdf | ETSI EN 300 392-11-14 V1.1.1 (2002-07) | ba1882df71dbb84a0c4f0f7dcea1dc8df968044457da03df9290a50e8821df32 |
| es_20081202v020401m.pdf | ETSI ES 200 812-2 V2.4.1 (2005-08) | 330f045a908eefd249fd7450e5b71bd08e8a7b20ce5b86dc9b87c9138a5c7268 |
| es_20081201v020205p.pdf | ETSI ES 200 812-1 V2.2.5 (2003-12) | 346dc545049a7397ca86831731bbc05e96f61ff047d6e7f6b7de56da3aa408e9 |
| en_300812v020101p.pdf | ETSI EN 300 812 V2.1.1 (2001-12) | 196effeaae473a98244c89538e1562daa58bb1d74cbfe7de78e58ceeafbad18b |
| en_3003921101v010201p.pdf | ETSI EN 300 392-11-1 V1.2.1 (2004-01) | 852c17ea566757e80ecf0d2718ae7839d9e218aa4384b63d1277ed7e25d86c69 |
| en_3003921006v010401p.pdf | ETSI EN 300 392-10-6 V1.4.1 (2006-08) | 32b6ee43602ef88a0b5c209b1c69ad35ba8b2e660502257c2dfaa6205a346523 |
| en_3003921018v010301p.pdf | ETSI EN 300 392-10-18 V1.3.1 (2003-10) | 4cc10bcd94d39201538639cefb529809729563f13f87cfe6ae52b44c253b0b8a |
| en_3003921216v010400a.pdf | ETSI EN 300 392-12-16 V1.4.0 (2026-03) | c0ee7859f448c4072befc25905c29b751b5151d3590a880a2d654309a3ba6c02 |
| en_30039201v010601p.pdf | ETSI EN 300 392-1 V1.6.1 (2020-04) | 788722e566098957bea1a9a5096e9dae1eace1da9f5d38c530c80230e5b9b9cb |
| ets_30039214e01v.pdf | pr ETS 300 392-14 | 2b703f3ebb883c90dab40e1b3d1a634a6acf9c9a20d468fd108ea02226c2bf2c |
| en_30039207v030501p.pdf | ETSI EN 300 392-7 V3.5.1 (2019-07) | df47af9a642b6eba7d7d5cf5fb7aa3f961e9e03bb912316bd2d189ce47344e08 |
| en_30039401v030301p.pdf | ETSI EN 300 394-1 V3.3.1 (2015-04) | 2d9c327e5ccf1476335e1b7a4f58a0ca7afc93911c30e8aad5ed7dc9ccf3276a |
| en_3003920313v010201p.pdf | ETSI EN 300 392-3-13 V1.2.1 (2020-04) | b47d63853f83a6a4adda08c0e325c8ca13d358e2f8840bfba4bce430e600b1bd |
| en_30039502v010303p.pdf | ETSI EN 300 395-2 V1.3.3 (2025-02) | ac716ca18082cc0fd2ee768a14f03deb48fa78a77e83751b1a6b319c7fc2106a |
| en_3003920303v010301p.pdf | ETSI EN 300 392-3-3 V1.3.1 (2011-11) | 94ca61038b3ff826e6adcfde56e00473dd996e1a47f9f84e7b9d2aa6c3133fd2 |
| en_30039205v020701p.pdf | ETSI EN 300 392-5 V2.7.1 (2020-04) | 10aaf78988ae4080c3dfe90165ee7f4f3fe8eda402e09fc1e6fa0a0ad890758d |
| en_3003920315v010500a.pdf | Draft ETSI EN 300 392-3-15 V1.5.0 (2026-04) | e959709667192e22d2dfeea51b48f9a579a4c107d952999056b7fad93a6d2100 |
| en_30039202v030801p.pdf | ETSI EN 300 392-2 V3.8.1 (2016-08) | 3f07b1e4ad73fabc16277a900006fc19844b3a882bbc2bc1d74d78f6156daf28 |
| ETSI.pdf | ETSI EN 300 812 V2.1.1 (2001-12) | 9434dad1e7bc80ca39b0edadd8e3b9995fda5ae5f5c3dd565af05708d5059e38 |

Die Titelblätter kennzeichnen insbesondere EN 300 392-12-16 V1.4.0 und EN 300 392-3-15 V1.5.0 als Entwürfe. Auch weitere ältere Dokumente tragen Draft-/Final-Draft-Kennzeichnungen. Die beigefügten Ausgaben wurden nicht auf neuere Veröffentlichungen geprüft. `ETSI.pdf` und `en_300812v020101p.pdf` nennen beide EN 300 812 V2.1.1; daraus wird ohne Binärvergleich keine Dateigleichheit behauptet.

### 10.3 Bildarchiv und übrige Lücken

Für die Gruppenplanung sind keine eigenständigen Bilddateien verfügbar. Die 25 Norm-PDFs ersetzen keine historischen Aufbau- oder Konfigurationsbilder.

Die bereitgestellten Norm-PDFs bleiben als Quellen inventarisiert; sie werden nicht pauschal als Bilder hochgeladen. Die Archivänderung enthält Dokumentation und Index, keine neu erzeugten technischen Implementierungen.

## 11. Archivierungsnachweis

Der technische Abgleich basiert auf dem Ausgangscommit aus Abschnitt 1. Dokumentänderungen sind über die Git-Historie dieser Datei nachvollziehbar.
