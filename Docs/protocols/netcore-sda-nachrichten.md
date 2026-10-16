# NetCore SDA: Nachrichten und lokale Basisstationsverarbeitung

Stand: 17.10.2026 · Roadmap **Z05 / NC1-SDA-01**.

Die Sepura-SDA-Vorlage sendet `NC1|...` als Text-SDS an die individuelle Service-ISSI **4010001**. Der SDA-Handler verarbeitet diese Nachrichten vor dem WX/METAR-Dienst. Wetterbefehle und andere Ziele behalten ihren bisherigen Weg. PID137 ist hierfür nicht erforderlich: Der gemeldete Datensatz verwendet PID130 (`0x82`).

## Nachrichten vom Funkgerät

`|` trennt Felder; jeder Auswahlwert steht für genau einen Wert. Befehle und Auswahlwerte beachten Groß-/Kleinschreibung.

| Nachricht | Verarbeitung / Grenzen |
| --- | --- |
| `NC1\|PING\|<Hinweis>` | Funkprobe lokal erfassen; Hinweis optional, höchstens60 Zeichen |
| `NC1\|RX_GOOD\|<Hinweis>` / `NC1\|RX_BAD\|<Hinweis>` | Gute / schlechte Empfangsmeldung; Hinweis optional, höchstens60 Zeichen |
| `NC1\|STATUS\|<Einheit>\|<Status>` | Einheit höchstens16 Zeichen; `READY`, `ENROUTE`, `ON_SCENE` oder `UNAVAILABLE` |
| `NC1\|ACK\|<Nummer>\|<Antwort>` | Nummer1–12 Ziffern, führende Nullen bleiben erhalten; `ACCEPT`, `BUSY` oder `REJECT` |
| `NC1\|MATERIAL\|<Ort>\|<Anzahl>\|<Material>` | Ort höchstens16 Zeichen, Anzahl1–999 in höchstens3 Ziffern, Material1–60 Zeichen |
| `NC1\|FAULT\|<Standort>\|<Beschreibung>` | Standort höchstens16 Zeichen, Beschreibung1–100 Zeichen |
| `NC1\|REQ\|SYSTEM` | Lokaler Stationsname, Stackversion, registrierte Teilnehmer und Core-Verbindung |
| `NC1\|REQ\|BS` | Lokale Zellkonfiguration: MCC/MNC, LA, Farbcode, Träger und Anzahl konfigurierter Nachbarn |
| `NC1\|REQ\|VPN` | Aktivierungszustand des NetCore-NetworkManager-Profils |
| `NC1\|REQ\|SIP` | Lokaler SIP-Laufzeitstatus und Anzahl aktiver Dialoge |

Einheit, Ort und Standort erlauben ASCII-Buchstaben, Ziffern, Leerzeichen sowie `_`, `-`, `/` und `.`. Leere Pflichtfelder und unbekannte Auswahlwerte werden abgewiesen. Im letzten Freitextfeld darf `|` vorkommen.

`BS` bestätigt keine Erreichbarkeit von Nachbarn. `VPN` prüft das über `vpn_uuid` in `/etc/netcore/image-network.json` zugeordnete aktive NetworkManager-Profil; dies bestätigt keinen Peer, keine Route und keinen entfernten Dienst. Ohne Zuordnung wird nur die Anzahl aktiver VPN-/WireGuard-Profile mit entsprechendem Hinweis gemeldet. Fehlender NetworkManager und Zeitlimit werden ausdrücklich benannt. `SIP` verwendet den vorhandenen Asterisk-Laufzeitstatus und meldet einen fehlenden Laufzeitstatus ausdrücklich.

## Antworten und Bedeutung der Quittungen

Antworten gehen von **4010001** über den lokalen SDS-Downlink an die anfragende ISSI:

```text
NC1|INFO|SIP lokal: registered; Dialoge 0
NC1|INFO|Status TRUPP1: ENROUTE; lokal erfasst #1
NC1|INFO|Fehler: Meldung nicht gespeichert; bitte erneut senden
```

Der Inhalt nach `NC1|INFO|` umfasst höchstens140 ASCII-Zeichen. Umlaute werden ausgeschrieben. Der vollständige SDS-TL-Datensatz ist `[82, 00, MR, 01, <Text>]`, ohne zweiten Header und ohne angeforderte Zustellrückmeldung.

Eine erfolgreiche Transportquittung bestätigt die Annahme in die lokale Arbeitswarteschlange. Erst `lokal erfasst #...` bestätigt das erfolgreiche Schreiben des Datensatzes. Bei voller oder fehlender Warteschlange erhält eine SDS-TL-Anfrage eine negative Transportquittung. Zustellberichte sowie die reservierten Servernachrichten `NC1|INFO|...` und `NC1|JOB|...` erzeugen keine weiteren Anwendungsantworten.

Auftragsantworten und Materialanforderungen sind **lokale Einträge**. Automatische Übernahme in zentrale Task-/Materialdienste und automatische Auftragserzeugung sind noch offen. Eine lokale Quittung bestätigt keine Bearbeitung durch eine Leitstelle. Das Downlinkformat `NC1|JOB|<Nummer>|<Text>` bleibt reserviert.

## Lokales Protokoll und Grenzen

`sda_log.json` liegt neben der geladenen Stations-TOML. Der Dienstbenutzer braucht Schreibrecht in diesem Verzeichnis. Erfasst werden Zeit, Absender-ISSI, laufende lokale Nummer und strukturierte Befehlsfelder; Abfragen werden nicht gespeichert. Die letzten500 Meldungen bleiben über Prozessneustarts erhalten. Neue Stände werden vollständig geschrieben und atomar ersetzt. Ungültige vorhandene Dateien werden nicht still überschrieben.

`GET /api/sda-log` am vorhandenen Dashboard liefert `scope: "local"`, die neuesten Einträge zuerst und den letzten noch enthaltenen Status je Absender-ISSI und Einheit. Die vorhandene Dashboard-Zugangskontrolle gilt auch hier. Ein defektes Journal ergibt HTTP500. Die bestehende SDS-Ansicht zeigt weiterhin den Nachrichtenverkehr.

Die Arbeitswarteschlange fasst32 Anfragen. Diskzugriffe und die auf3 Sekunden begrenzte NetworkManager-Abfrage laufen außerhalb des RF-Threads. Gleiche Absender-ISSI, SDS-TL-Referenz und gleicher Text werden bis60 Sekunden aus einem auf256 Antworten begrenzten Prozesscache beantwortet, ohne einen weiteren Journaleintrag. Nach Neustart oder Cacheablauf besteht dieser Wiederholungsschutz nicht fort. Nackte Text-SDS ohne TL-Referenz haben keine solche Erkennung.

## Übernahme und Funkabnahme

Nach Übernahme des PR den [bestehenden TBS-Build- und Updateweg](../wiki/software-bauen-und-aktualisieren.md#tbs-bauen-und-installieren) verwenden. Das Binary muss neu gebaut, installiert und neu gestartet werden; Git-Update allein aktiviert den Handler nicht. Die geladene TOML beibehalten. Es ist kein neuer Konfigurationsschlüssel erforderlich. Die feste Service-ISSI4010001 entspricht der SDA-Vorlage.

Bei abweichender systemd-Unit oder anderem Binarypfad die tatsächlich installierte Unit aus `systemctl cat` verwenden. Das Wiki zeigt `tetra.service` und `/usr/local/bin/bluestation-bs`.

Abnahme mit dem bereits beobachteten Funkgerät **5102**:

1. `NC1|REQ|SIP` an4010001 senden. Das Stationsjournal muss `SDA: request from ISSI 5102` zeigen und eine `NC1|INFO|SIP lokal: ...`-Antwort muss an5102 zurückgehen. Der genaue SIP-Zustand hängt von der laufenden Station ab.
2. `NC1|STATUS|TRUPP1|ENROUTE` senden. Antwort mit `lokal erfasst #...` und Eintrag unter `/api/sda-log` kontrollieren.
3. `NC1|RX_GOOD|Ad` senden; lokalen Eintrag und Antwort prüfen.
4. Basisstationsprozess neu starten und erhaltene Einträge kontrollieren.
5. Normale Wetterabfrage und SDS zwischen zwei Funkgeräten prüfen.

Physischer Funk-/SDA-Anzeigetest und Übernahme auf Jans Station bleiben offen. Der unveränderte gemeldete120-Bit-Datensatz ist als Testeingabe hinterlegt: `82 06 06 01 4E 43 31 7C 52 45 51 7C 53 49 50`.

## Quell- und Testnachweise

- [Parser, Journal und Worker](../../crates/tetra-entities/src/net_sda/).
- [RF-Routingtests mit Originaldatensatz, voller Warteschlange und vollständigem Antwortpaket](../../crates/tetra-entities/tests/test_sda_service.rs).
- [Linux-CI](../../.github/workflows/dashboard-ui-tests.yml): Protokolltests ohne native SDR-Abhängigkeiten, Basisstations-Buildcheck, Runtime-/HTTP-Tests und bestehende SDS-Regression.

Der lokale Windows-Testlauf ist vor der Projektkompilierung am fehlenden MSVC-Linker blockiert. `rustfmt` und `git diff --check` sind lokal ausführbar. Linux-Ergebnisse werden im PR und in der Gesamtroadmap nachgetragen.
