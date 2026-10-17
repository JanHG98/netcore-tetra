# NetCore SDA: Nachrichten und lokale Basisstationsverarbeitung

Stand: 17.10.2026 · Roadmap **Z05 / NC1-SDA-01**.

Die Sepura-SDA-Vorlage sendet `NC1|...` als Text-SDS an die individuelle Service-ISSI **4010001**. Der SDA-Handler verarbeitet diese Nachrichten vor dem WX/METAR-Dienst. Wetterbefehle und andere Ziele behalten ihren bisherigen Weg. PID137 ist hierfür nicht erforderlich: Der gemeldete Datensatz verwendet PID130 (`0x82`).

## Nachrichten vom Funkgerät

`|` trennt Felder; jeder Auswahlwert steht für genau einen Wert. Befehle und Auswahlwerte beachten Groß-/Kleinschreibung.

| Nachricht | Verarbeitung / Grenzen |
| --- | --- |
| `NC1\|PING\|<Hinweis>` | Funkprobe lokal erfassen; Hinweis optional, höchstens 60 Zeichen |
| `NC1\|RX_GOOD\|<Hinweis>` / `NC1\|RX_BAD\|<Hinweis>` | Gute / schlechte Empfangsmeldung; Hinweis optional, höchstens 60 Zeichen |
| `NC1\|STATUS\|<Einheit>\|<Status>` | Einheit höchstens 16 Zeichen; `READY`, `ENROUTE`, `ON_SCENE` oder `UNAVAILABLE` |
| `NC1\|ACK\|<Nummer>\|<Antwort>` | Nummer 1–12 Ziffern, führende Nullen bleiben erhalten; `ACCEPT`, `BUSY` oder `REJECT` |
| `NC1\|MATERIAL\|<Ort>\|<Anzahl>\|<Material>` | Ort höchstens 16 Zeichen, Anzahl 1–999 in höchstens 3 Ziffern, Material 1–60 Zeichen |
| `NC1\|FAULT\|<Standort>\|<Beschreibung>` | Standort höchstens 16 Zeichen, Beschreibung 1–100 Zeichen |
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

Der Inhalt nach `NC1|INFO|` umfasst höchstens 140 ASCII-Zeichen. Umlaute werden ausgeschrieben. Der vollständige SDS-TL-Datensatz ist `[82, 00, MR, 01, <Text>]`, ohne zweiten Header und ohne angeforderte Zustellrückmeldung.

Eine erfolgreiche Transportquittung bestätigt die Annahme in die lokale Arbeitswarteschlange. Erst `lokal erfasst #...` bestätigt das erfolgreiche Schreiben des Datensatzes. Bei voller oder fehlender Warteschlange erhält eine SDS-TL-Anfrage eine negative Transportquittung. Zustellberichte sowie die reservierten Servernachrichten `NC1|INFO|...` und `NC1|JOB|...` erzeugen keine weiteren Anwendungsantworten.

### Leeres SDA-Fenster und Empfangsvorlage V1.9

Jan meldet nach dem Senden eine leere SDA-Seite. In Datei 12 / V1.8 steht vor
den NetCore-Empfangsregeln noch die Empfangsregel aus der Originalvorlage für
Formular 1. Außerdem verwendet die INFO-Regel das Abfrageformular, dessen
Textfeld erst unter den vier Abfrageknöpfen steht. Ein leeres Fenster allein
beweist deshalb keine leere Basisstationsantwort.

`13_NetCore_SDA_Rueckmeldung_V1_9.rmt` entfernt den alten Empfangsverweis und
behält ausschließlich die getrennten Regeln für `NC1|JOB|...` und
`NC1|INFO|...`. INFO öffnet das eigene Formular 9 **NetCore Antwort**: eine
Beschriftung und direkt darunter vier Textzeilen für das vorhandene Feld 69
mit höchstens 140 Zeichen. Das bestehende Menü, seine Sendefunktionen und die
kompakten Abstände werden nicht geändert.

Geprüft sind das verschlüsselte Archiv, alle 20.717 zurückgelesenen Parameter,
die unveränderten Sendedefinitionen und ein ASCII-Empfangsmodell für Status,
Funkprobe, Fehler, 140 Zeichen und Auftragsnummern mit führenden Nullen. Die
unveränderten ARM-Feld-/Layoutlader übernehmen alle neun Formulare ohne
Assertion und ohne zusätzlich gesetzte Feature-Lizenzanforderung; deren
Parameterzugriff, Speicherzuteilung und Stringpuffer sind nachgebildet. Das
beweist weder den vollständigen nativen Nachrichtendecoder noch die Anzeige
am Funkgerät. Radio-Manager-Import und physische Empfangsabnahme bleiben offen.

Der Worker protokolliert nach erfolgreichem Einreihen die vollständige
Anwendungsantwort, zum Beispiel
`SDA: reply queued for ISSI 5102: "NC1|INFO|SIP lokal: ..."`.
Dies bestätigt das Einreihen des Downlinks, keine Funkzustellung. Die Zeile
ermöglicht beim Gerätetest die Unterscheidung zwischen Transportquittung,
fehlender Anwendungsantwort und einem Anzeige-/Vorlagenfehler.

Auftragsantworten und Materialanforderungen sind **lokale Einträge**. Automatische Übernahme in zentrale Task-/Materialdienste und automatische Auftragserzeugung sind noch offen. Eine lokale Quittung bestätigt keine Bearbeitung durch eine Leitstelle. Das Downlinkformat `NC1|JOB|<Nummer>|<Text>` bleibt reserviert.

## Lokales Protokoll und Grenzen

`sda_log.json` liegt neben der geladenen Stations-TOML. Der Dienstbenutzer braucht Schreibrecht in diesem Verzeichnis. Erfasst werden Zeit, Absender-ISSI, laufende lokale Nummer und strukturierte Befehlsfelder; Abfragen werden nicht gespeichert. Die letzten 500 Meldungen bleiben über Prozessneustarts erhalten. Neue Stände werden vollständig geschrieben und atomar ersetzt. Ungültige vorhandene Dateien werden nicht still überschrieben.

`GET /api/sda-log` am vorhandenen Dashboard liefert `scope: "local"`, die neuesten Einträge zuerst und den letzten noch enthaltenen Status je Absender-ISSI und Einheit. Die vorhandene Dashboard-Zugangskontrolle gilt auch hier. Ein defektes Journal ergibt HTTP 500. Die bestehende SDS-Ansicht zeigt weiterhin den Nachrichtenverkehr.

Die Arbeitswarteschlange fasst 32 Anfragen. Diskzugriffe und die auf 3 Sekunden begrenzte NetworkManager-Abfrage laufen außerhalb des RF-Threads. Gleiche Absender-ISSI, SDS-TL-Referenz und gleicher Text werden bis 60 Sekunden aus einem auf 256 Antworten begrenzten Prozesscache beantwortet, ohne einen weiteren Journaleintrag. Nach Neustart oder Cacheablauf besteht dieser Wiederholungsschutz nicht fort. Nackte Text-SDS ohne TL-Referenz haben keine solche Erkennung.

## Übernahme und Funkabnahme

Nach Übernahme des PR den [bestehenden TBS-Build- und Updateweg](../wiki/software-bauen-und-aktualisieren.md#tbs-bauen-und-installieren) verwenden. Das Binary muss neu gebaut, installiert und neu gestartet werden; Git-Update allein aktiviert den Handler nicht. Die geladene TOML beibehalten. Es ist kein neuer Konfigurationsschlüssel erforderlich. Die feste Service-ISSI 4010001 entspricht der SDA-Vorlage.

Bei abweichender systemd-Unit oder anderem Binarypfad die tatsächlich installierte Unit aus `systemctl cat` verwenden. Das Wiki zeigt `tetra.service` und `/usr/local/bin/bluestation-bs`.

Abnahme mit dem bereits beobachteten Funkgerät **5102**:

1. `NC1|REQ|SIP` an 4010001 senden. Das Stationsjournal muss `SDA: request from ISSI 5102` zeigen und eine `NC1|INFO|SIP lokal: ...`-Antwort muss an 5102 zurückgehen. Der genaue SIP-Zustand hängt von der laufenden Station ab.
2. `NC1|STATUS|TRUPP1|ENROUTE` senden. Antwort mit `lokal erfasst #...` und Eintrag unter `/api/sda-log` kontrollieren.
3. `NC1|RX_GOOD|Ad` senden; lokalen Eintrag und Antwort prüfen.
4. Basisstationsprozess neu starten und erhaltene Einträge kontrollieren.
5. Normale Wetterabfrage und SDS zwischen zwei Funkgeräten prüfen.

Physischer Funk-/SDA-Anzeigetest und Übernahme auf Jans Station bleiben offen. Der unveränderte gemeldete 120-Bit-Datensatz ist als Testeingabe hinterlegt: `82 06 06 01 4E 43 31 7C 52 45 51 7C 53 49 50`.

## Quell- und Testnachweise

- [Parser, Journal und Worker](../../crates/tetra-entities/src/net_sda/).
- [RF-Routingtests mit Originaldatensatz, voller Warteschlange und vollständigem Antwortpaket](../../crates/tetra-entities/tests/test_sda_service.rs).
- [Linux-CI](../../.github/workflows/dashboard-ui-tests.yml): Protokolltests ohne native SDR-Abhängigkeiten, Basisstations-Buildcheck, Runtime-/HTTP-Tests und bestehende SDS-Regression.

Der lokale Windows-Testlauf ist vor der Projektkompilierung am fehlenden MSVC-Linker blockiert. `rustfmt` und `git diff --check` sind lokal ausführbar. Der verbindliche Linux-Build- und Teststatus ist in [PR #106](https://github.com/JanHG98/netcore-tetra/pull/106) verlinkt. Ein erfolgreicher Softwaretest ersetzt keine physische Funk-/SDA-Abnahme.
