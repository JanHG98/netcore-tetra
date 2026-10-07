# Brainstorming: Standalone-TETRA-Basisstation – Konfiguration und PHY-Grundlagen

> **Ergebnis der Planung:** Für die selbst entwickelte NetCore-Tetra-Basisstation wurde geklärt, welche Angaben tatsächlich zur Konfiguration einer eigenständigen TETRA-Zelle gehören, welche Werte nur für Teilnehmer- oder Gruppendienste benötigt werden und welche Aussagen aus der frühen Planung technisch zu korrigieren sind. Die wichtigste PHY-Korrektur lautet **π/4-DQPSK**; für die Zellenkonfiguration sind insbesondere MCC/MNC, Frequenzband, Hauptträger, Frequenzoffset, Duplex-Spacing-Index, Reverse-Operation, Location Area und Colour Code relevant. Eine Nachbarzellenliste ist für den ausdrücklich gewünschten Standalone-Einzelzellenbetrieb nicht erforderlich.
>
> **Nachweisstand:** In dieser Entwicklungsphase wurde keine neue TBS-Funktion implementiert und kein Funk- oder Hardwaretest durchgeführt. Der zusätzliche Repository-Abgleich bestätigt jedoch, dass die genannten Netz-/Zellenparameter und eine optionale ISSI-Whitelist im am Prüfdatum vorliegenden NetCore-Tetra-Code vorhanden sind. Gleichzeitig wurde ein konkreter Dokumentationswiderspruch beim Duplex-Spacing im am Prüfdatum vorliegenden Fallback-Config gefunden, der außerhalb dieses Dokumentationslaufs korrigiert werden sollte.

## Zielbild und Festlegungen

- **Eigene Standalone-TBS** mit einer Zelle; Nachbarzellen, Handover und ISI gehören noch nicht zum ersten Ausbau.
- Checkliste: Netzidentität, Carrier/Duplex, Teilnehmerzulassung, Gruppen, Zellzugang/Systeminformationen und PHY-/Scheduler-Parameter.
- PHY-Grundlage: **π/4-DQPSK**, 25-kHz-Raster und vier TDMA-Timeslots.
- Der Duplex-Spacing-Kommentar der Fallback-Konfiguration widerspricht dem ausführbaren Code; Zahlenwerte und ausgesendete Parameter müssen abgeglichen werden.

## 1. Arbeitsstand und Quellenbasis

| Feld | Inhalt |
|---|---|
| Projekt | NetCore-Tetra |
| Thema | Pflicht- und Betriebsparameter einer selbst entwickelten Standalone-TETRA-Basisstation; MCC/MNC, ISSI/GSSI, LA, Colour Code, Carrier/Duplex sowie π/4-DQPSK |
| Erstellungsdatum der Projektnotizen | **2026-10-04**, Europe/Berlin |
| Zielrepository | [JanHG98/netcore-tetra](https://github.com/JanHG98/netcore-tetra) |
| Geprüfter Eingangscommit von Archiving | **2a0eaa3da26f4240cbb08514420f5c1682b4f321** |
| Geprüfter Eingangscommit – Nachricht | docs(archive): document WERMA relay Pi API and early boot signalling |
| Archivdatei | **Docs/archive/2026-10-04_tetra-basisstation-konfiguration-und-phy-grundlagen.md** |
| Archivindex | **Docs/archive/README.md** |

### 1.1 Geprüfter Entwurfsentwicklung

Ausgangsliste für die eigene Basisstation: **MCC, MNC, GSSI, erlaubte ISSI, LAC, Colour Code, Uplink und Duplex-Tabellen-ID**. Daraus soll eine vollständige Parametercheckliste entstehen.

Festgelegt sind eine **Standalone-TBS** und eine **eigene Implementierung**. Nachbarzellen werden für die zunächst einzelne Zelle nicht benötigt; neben Konfigurationsfeldern sind ausgesendete Systeminformationen und lokale PHY-/Scheduler-Funktionen relevant. Die Modulation wird als **π/4-DQPSK** bezeichnet.

Messbilder, Funkmitschnitte, Installationsprotokolle und Shellausgaben dieser Planungsphase fehlen. Generische frühe Beispiele sind gegen den Repository- und Normenstand zu prüfen.

### 1.2 Verfügbare Anhänge

Im Projektkontext stehen 25 ETSI-PDFs zur Verfügung. Für diese Planung besonders relevant waren:

- **en_30039202v030801p.pdf** – ETSI EN 300 392-2 V3.8.1, TETRA Voice plus Data, Part 2: Air Interface.
- **en_30039201v010601p.pdf** – ETSI EN 300 392-1 V1.6.1, Part 1: General network design.
- **en_30039205v020701p.pdf** – ETSI EN 300 392-5 V2.7.1, Part 5: Peripheral Equipment Interface; enthält unter anderem explizite Feldlängen für GSSI/SSI/MCC/MNC/LA.
- **en_30039401v030301p.pdf** – ETSI EN 300 394-1 V3.3.1, Radio Conformance Testing.
- Weitere bereitgestellte Dateien behandeln ISI, Supplementary Services, Security, SIM/TSIM, Codec und Konformität und sind für die hier behandelte Grundkonfiguration nur nachrangig.

Die PDFs wurden gezielt nach den für diese Planung relevanten Parametern durchsucht. Es wurde **keine vollständige normative Review aller 25 Dokumente** behauptet. Das große Sammeldokument **ETSI.pdf** enthält mehrere Dokumente und wird nicht als ein einzelner Standard behandelt.

### 1.3 Bildarchiv

Für den am Prüfdatum vorliegenden Unterlagen wurden über die verfügbare Dateiansicht **0 eigenständige Bilddateien** gefunden. Die ETSI-PDFs enthalten eingebettete Deckblätter, Diagramme und Tabellen, diese sind jedoch Quellenmaterial und keine ausdrücklich in dieser Entwicklungsphase bereitgestellten Projektbilder. Sie wurden deshalb nicht als vermeintliche Originalbilder extrahiert oder dupliziert.

### 1.4 Statusbegriffe

| Status | Bedeutung in dieser Dokumentation |
|---|---|
| **Idee** | In der Planung vorgeschlagen oder als mögliche Erweiterung erwähnt. |
| **Beschlossen/geplant** | Ausdrücklich ausdrücklich als Ziel oder Randbedingung gesetzt. |
| **Implementiert** | Im überprüften Repository als Code/Konfiguration vorhanden. |
| **Getestet** | Ein konkreter Test wurde tatsächlich durchgeführt und das Ergebnis liegt vor. |
| **Im Betrieb bestätigt** | Erfolgreicher Betrieb auf realer Hardware/Funkstrecke ist belegt. |

Eine frühere Entwurfsfassung ist weder Implementierungs- noch Testnachweis.

---

## 2. Ziel und Ausgangslage

Ziel der Planung war keine allgemeine Einführung in TETRA, sondern eine belastbare Checkliste für die **eigene NetCore-Tetra-Basisstation**. Beim Aufbau der Standalone-Zelle sollen keine wesentlichen Konfigurationsparameter fehlen.

Die Ausgangsliste war:

- MCC
- MNC
- GSSI
- erlaubte ISSI
- LAC
- ColorCode
- Uplink
- Duplex-Tabellen-ID

Das Planung zeigte, dass diese Liste drei verschiedene Ebenen vermischt:

1. **Netz- und Zellidentität**
2. **HF-/Carrierparameter**
3. **Teilnehmer- und Gruppendienstparameter**

Für die weitere Entwicklung ist diese Trennung wichtig, weil beispielsweise eine GSSI nicht erforderlich ist, um überhaupt eine Zelle auszusenden und ein Endgerät registrieren zu lassen, während sie für einen Gruppenruf sehr wohl benötigt wird.

---

## 3. Endgültige Anforderungen und Entscheidungen

### 3.1 Standalone-Einzelzelle

**Status: beschlossen/geplant**

Die Basisstation soll zunächst als **Standalone-TBS beziehungsweise Einzelzelle** betrieben werden. Daraus folgt:

- keine verpflichtende Neighbor List;
- kein Seamless-Handover als Voraussetzung für die Erstinbetriebnahme;
- keine Inter-Cell-Synchronisation als Voraussetzung für diesen ersten Betriebsmodus;
- die Zelle muss ihre eigene Netz-/Zellinformation vollständig korrekt aussenden;
- Teilnehmerregistrierung, Affiliation, Rufe und SDS müssen lokal funktionieren können.

Die vorhandene NetCore-Nachbarzellenfunktion kann für diesen Stand deaktiviert beziehungsweise unkonfiguriert bleiben.

### 3.2 Eigene Implementierung statt Herstellerparametrierung

**Status: beschlossen/geplant**

Es handelt sich um eine selbst entwickelte Basisstation. Damit müssen nicht nur Bedienparameter, sondern auch die dahinterliegenden Protokollfelder, Carrierberechnung, Timing- und Modulationsanforderungen verstanden und korrekt umgesetzt werden.

Der Schwerpunkt dieser Planung liegt dennoch auf **welche Werte benötigt werden**, nicht auf einer vollständigen Neuimplementierung der gesamten ETSI-Luftschnittstelle.

### 3.3 Modulation

**Status: fachlich verifiziert**

Die gesuchte Bezeichnung ist:

**π/4-DQPSK**

ausgeschrieben:

**π/4-shifted Differential Quaternary Phase Shift Keying**

Für die klassische phasenmodulierte TETRA-Luftschnittstelle ist dies die zentrale Modulationsart. Die ETSI-Luftschnittstelle kennt daneben auch weitere Modi wie π/8-D8PSK und QAM; diese ändern aber nicht die in dieser Entwicklungsphase getroffene Aussage zur π/4-DQPSK-Basis.

Für π/4-DQPSK nennt ETSI eine Modulationsrate von **36 kbit/s**. Bei zwei Bits pro Symbol ergibt sich daraus **18 ksym/s**. Die erlaubten differentiellen Phasenübergänge sind ±π/4 und ±3π/4.

### 3.4 Keine pauschale direkte Frequenzeingabe als einziges Zellmodell

**Status: durch am Prüfdatum vorliegenden Repository-Stand präzisiert**

Die ursprüngliche Frage nach „Uplink“ ist sinnvoll aus Bedienersicht, aber der am Prüfdatum vorliegende NetCore-Tetra-Stack modelliert die Zelle primär über:

- Frequenzband
- Carrier Number
- Frequenzoffset
- Duplex-Spacing-Index
- Reverse-Operation

Daraus werden Downlink und Uplink berechnet. Absolute SDR-Centerfrequenzen sind zusätzliche PHY-Parameter, insbesondere bei Dual-Carrier, aber nicht identisch mit der eigentlichen TETRA-Carrierdefinition.

---

## 4. Vollständige Parameterliste für die geplante Standalone-TBS

### 4.1 Kategorie A – Netzidentität

| Parameter | Status | Feldbreite / Bereich | Rolle | Für Standalone-Erstbetrieb |
|---|---|---:|---|---|
| **MCC** | implementiert | 10 Bit | Mobile Country Code, Teil der MNI | erforderlich |
| **MNC** | implementiert | 14 Bit | Mobile Network Code, Teil der MNI | erforderlich |
| **Location Area / LA** | implementiert | 14 Bit | Location Area der Zelle | erforderlich |
| **System Code** | implementiert | 4 Bit | Bestandteil von SYNC | erforderlich als korrekt gesetztes Zellfeld |
| **Colour Code** | implementiert | 6 Bit, 0–63 | Zell-/Scrambling-Kontext | erforderlich |
| **Subscriber Class** | implementiert im PDU-Modell | 16 Bit | Zugangs-/Teilnehmerklassensicht | je nach gewünschter Policy relevant |

**Begriffspräzisierung:** In der Planung wurde „LAC“ verwendet. Im am Prüfdatum vorliegenden NetCore-Code und in den relevanten TETRA-PDUs wird das Feld als **Location Area / LA** beziehungsweise <code>location_area</code> geführt. Es ist sinnvoll, intern im Projekt einheitlich „LA“ zu verwenden und „LAC“ nur als verständliche Umgangsbezeichnung zu behandeln.

### 4.2 Kategorie B – Carrier- und Frequenzparameter

| Parameter | Status | Feldbreite / Werte | Rolle | Für Standalone-Erstbetrieb |
|---|---|---:|---|---|
| **Frequency Band** | implementiert | 4 Bit | Frequenzband in der Carrierberechnung | erforderlich |
| **Main Carrier Number** | implementiert | 12 Bit | Hauptträger | erforderlich |
| **Frequency Offset** | implementiert | 2-Bit-Index; Config in Hz | Offset zur 25-kHz-Rasterfrequenz | erforderlich |
| **Duplex Spacing** | implementiert | 3-Bit-Index, 0–7 | Auswahl aus Duplex-Spacing-Tabelle | erforderlich |
| **Custom Duplex Spacing** | implementiert, optional | Hz | nur für bewusst abweichende Endgeräte-/Tabellenkonfiguration | normalerweise nicht erforderlich |
| **Reverse Operation** | implementiert | 1 Bit | legt fest, ob UL unter oder über DL liegt | erforderlich |
| **Secondary Carrier** | implementiert, optional | Carrier Number | zweiter Träger | für Single-Carrier nicht erforderlich |
| **TX/RX SDR Center Frequency** | implementiert als PHY-Option | Hz | SDR-Passbandzentrum, besonders Dual-Carrier | Single-Carrier typischerweise nicht separat nötig |
| **Sample Rate** | PHY-Parameter | geräteabhängig | SDR-Abtastrate | erforderlich für den realen SDR-Betrieb, aber kein TETRA-Netzidentitätsfeld |
| **RX/TX Gain** | PHY-Parameter | geräteabhängig | SDR-Verstärkung | erforderlich abzustimmen, kein TETRA-SYSINFO-Feld |
| **Clock/Reference** | Hardware/PHY | geräteabhängig | Frequenz- und Timingstabilität | erforderlich technisch zu beherrschen; GPS ist nicht pauschal vorgeschrieben |

### 4.3 Frequenzberechnung im am Prüfdatum vorliegenden NetCore-Tetra-Code

Der am Prüfdatum vorliegende Code berechnet die Downlinkfrequenz als:

~~~text
downlink_hz =
    100_000_000 * frequency_band
  + 25_000 * main_carrier
  + frequency_offset_hz
~~~

Anschließend:

~~~text
wenn reverse_operation == false:
    uplink_hz = downlink_hz - duplex_spacing_hz

wenn reverse_operation == true:
    uplink_hz = downlink_hz + duplex_spacing_hz
~~~

Damit ist die ausdrücklich genannte Angabe „Uplink“ für Planung und Prüfung weiterhin wichtig, aber im derzeitigen Stack **eine abgeleitete Größe**, sofern kein bewusst abweichender PHY-Override verwendet wird.

### 4.4 Kategorie C – Teilnehmerzulassung

| Parameter | Status | Rolle | Pflicht? |
|---|---|---|---|
| **ISSI** | Standard-/Repo-Begriff | individuelle Short Subscriber Identity, 24 Bit | pro Teilnehmer erforderlich |
| **ISSI-Whitelist** | implementiert | Liste zugelassener ISSIs | optional |
| **offenes Netz** | implementiertes Verhalten | leere Whitelist akzeptiert jeden Teilnehmer, soweit keine andere Policy blockiert | mögliche Entwicklungs-/Laboreinstellung |
| **Teilnehmerprofil / Berechtigungen** | in zentralen NetCore-Diensten vorhanden | Name, Organisation, Dienste, Priorität usw. | für ausgebauten Betrieb, nicht zum bloßen Aussenden der Zelle zwingend |

Die Anforderung **„Erlaubte ISSI“** lässt sich im am Prüfdatum vorliegenden Stack direkt auf <code>[security].issi_whitelist</code> abbilden.

Wichtig: „ISSI erlaubt“ und „ISSI zum Prüfdatum registriert“ sind zwei verschiedene Zustände.

### 4.5 Kategorie D – Gruppen

| Parameter | Status | Rolle | Pflicht? |
|---|---|---|---|
| **GSSI** | implementiert / Standard | Group Short Subscriber Identity, 24 Bit | nicht zum bloßen Zellbetrieb; erforderlich für Gruppenkommunikation |
| **Gruppenmitgliedschaft** | Group Core vorhanden | legt organisatorische Zugehörigkeit fest | für zentral verwaltete Gruppen relevant |
| **am Prüfdatum vorliegende Affiliation** | MM/Group Core unterstützt | welche GSSI ein Gerät zum Prüfdatum auf der Luftschnittstelle empfängt | für Gruppenrufrouting relevant |
| **DGNA** | Repository-Bausteine vorhanden | dynamische Gruppenzuweisung | optionale höhere Funktion |

Eine GSSI ist daher **kein zwingender Basisstations-Identitätsparameter**. Sie wird benötigt, sobald die Standalone-TBS reale Gruppenrufe beziehungsweise Gruppenaffiliation unterstützen soll.

### 4.6 Kategorie E – Zellzugang und Service-Advertisement

Zusätzlich zur minimalistischen Ausgangsliste sind für eine saubere eigene Basisstation relevant:

- Maximum MS transmit power / entsprechende Zellzugangsparameter.
- Minimum RX access level.
- Access Parameter.
- Radio Downlink Timeout.
- BS service details beziehungsweise advertised capabilities.
- Anzahl und Art verwendeter Control Channels.
- gegebenenfalls Packet-Data-/SDS-/Call-Service-Flags.

Diese Werte gehören zur tatsächlichen Protokollimplementierung und können nicht durch eine einfache Liste aus MCC/MNC/LA/CC ersetzt werden.

### 4.7 Kategorie F – PHY-/Scheduler-Laufzeitwerte

Diese Größen sind wichtig, aber normalerweise **keine statischen Installationsdaten des Betreibers**:

- Timeslotnummer
- Frame Number
- Multiframe Number
- Hyperframe-/Timingzustand
- konkrete Burst-Typen
- laufende Kanalbelegung
- zufälliger Zugriff / Access Assignment
- am Prüfdatum vorliegende Traffic-Channel-Zuweisung

Der Scheduler erzeugt und verwaltet diese Werte zur Laufzeit.

---

## 5. TETRA-PHY-Grundlagen, die für den Eigenbau festzuhalten sind

### 5.1 Modulation

**Verifiziert:** π/4-DQPSK.

Die frühere Formulierung „Pi 4 DQPSK“ war inhaltlich klar gemeint; die korrekte Schreibweise ist **π/4-DQPSK**.

### 5.2 Kanalraster und Bandbreite

Für die hier relevante klassische phasenmodulierte TETRA-Luftschnittstelle wird ein **25-kHz-Kanal** verwendet. Der NetCore-Code rechnet Carrier entsprechend mit 25.000 Hz.

### 5.3 TDMA

TETRA verwendet in diesem Betriebsmodus **vier Timeslots pro TDMA-Frame**.

Wichtige Korrektur gegenüber einer frühen Entwurfsfassung:

- extern/protokollseitig: **Timeslot 1 bis 4**
- die zweibitige Codierung kann intern 0 bis 3 repräsentieren;
- der am Prüfdatum vorliegende NetCore-Parser liest zwei Bits und addiert 1.

Eine frühere Aussage „Timeslot-Nummerierung 0–3“ war deshalb als Protokollbeschreibung falsch beziehungsweise mindestens missverständlich.

Ein Timeslot dauert ungefähr **14,167 ms**, ein vollständiger Vier-Slot-Frame damit ungefähr **56,67 ms**.

### 5.4 Synchronisation

Für eine Standalone-Einzelzelle muss die Basisstation selbst:

- stabile Carrierfrequenz liefern;
- ihre TDMA-Zeitbasis korrekt erzeugen;
- SYNC/SYSINFO korrekt senden;
- ihre eigene Frame-/Multiframe-Zeit konsistent fortführen.

**Nicht beschlossen und nicht normativ aus dieser Entwicklungsphase ableitbar:** „GPS ist zwingend“. GPS, GNSS, 10-MHz-Referenz oder andere externe Referenzen können technische Designoptionen sein, aber eine einzelne Zelle benötigt nicht allein wegen TDMA zwingend GPS. Für spätere synchronisierte Mehrzellenarchitekturen wird die Referenzfrage wesentlich wichtiger.

---

## 6. Was für eine Standalone-Zelle ausdrücklich NICHT benötigt wird

### 6.1 Neighbor List

**Status: für den am Prüfdatum vorliegenden Zielbetrieb nicht erforderlich**

Wenn nur eine einzelne TBS existiert und kein Zellwechsel stattfinden soll, muss keine Nachbarzellenliste ausgesendet beziehungsweise gepflegt werden.

Der am Prüfdatum vorliegende NetCore-Code unterstützt CA-Nachbarzellen, aber diese Funktion kann im Standalone-Aufbau ungenutzt bleiben.

### 6.2 Seamless Handover

Nicht Teil des Minimalumfangs dieser Planung. Es bleibt ein späterer Ausbaupunkt des Gesamtprojekts.

### 6.3 ISI zwischen SwMIs

Für den hier beschriebenen lokalen Einzelzellenbetrieb nicht erforderlich.

### 6.4 Ein frei erfundener „Cell ID = 16 Bit“-Pflichtwert

Eine frühere Entwurfsfassung hatte einen generischen „Cell ID, 16 Bit“ als Pflichtfeld genannt. Diese Aussage wird **verworfen**:

- im am Prüfdatum vorliegenden top-level <code>CfgCellInfo</code> existiert kein solcher allgemeiner 16-Bit-Cell-ID-Parameter;
- Nachbarzellen besitzen im CA-Modell einen eigenen Cell Identifier mit anderer Feldbreite;
- daher darf kein nicht vorhandenes Pflichtfeld nur aus einer generischen Mobilfunkanalogie in NetCore-Tetra eingeführt werden.

---

## 7. Aktueller Repository-Stand am 2026-10-04

### 7.1 Geprüfter Branch

Alle für die Schlussfolgerungen entscheidenden Dateien wurden explizit auf **Archiving** gegen Eingangscommit:

**2a0eaa3da26f4240cbb08514420f5c1682b4f321**

gelesen.

Die GitHub-Code-Suche dient nur zum Auffinden von Pfaden; entscheidende Dateien wurden anschließend nochmals direkt mit Ref <code>Archiving</code> geladen.

### 7.2 Netzkonfiguration

Datei:

[crates/tetra-config/src/bluestation/sec_net.rs](https://github.com/JanHG98/netcore-tetra/blob/Archiving/crates/tetra-config/src/bluestation/sec_net.rs)

Geprüfter Blob:

**cf9062b069cad9b36dd8efe058cc1cd81cb1e146**

Der Code definiert:

- MCC: 10 Bit
- MNC: 14 Bit

**Status: implementiert.**

### 7.3 Zellkonfiguration

Datei:

[crates/tetra-config/src/bluestation/sec_cell.rs](https://github.com/JanHG98/netcore-tetra/blob/Archiving/crates/tetra-config/src/bluestation/sec_cell.rs)

Geprüfter Blob:

**dfaf4bbaa609da79506f5b4adf73b061698cd65e**

Aktuell vorhandene Kernfelder:

- <code>main_carrier</code>
- <code>secondary_carrier</code> optional
- <code>freq_band</code>
- <code>freq_offset_hz</code>
- <code>duplex_spacing_id</code>
- <code>custom_duplex_spacing</code> optional
- <code>reverse_operation</code>
- <code>location_area</code>
- <code>system_code</code>
- <code>colour_code</code>
- weitere Zell-/Serviceparameter.

**Status: implementiert.**

### 7.4 MAC-SYNC

Datei:

[crates/tetra-pdus/src/umac/pdus/mac_sync.rs](https://github.com/JanHG98/netcore-tetra/blob/Archiving/crates/tetra-pdus/src/umac/pdus/mac_sync.rs)

Geprüfter Blob:

**74f8538a8a6ffe6534f6af5284ad1ddf457b830d**

Bestätigt:

- System Code: 4 Bit.
- Colour Code: 6 Bit.
- Timeslot: zwei Bit codiert, im Datenmodell +1 → Slots 1–4.

**Status: implementiert.**

### 7.5 MAC-SYSINFO

Datei:

[crates/tetra-pdus/src/umac/pdus/mac_sysinfo.rs](https://github.com/JanHG98/netcore-tetra/blob/Archiving/crates/tetra-pdus/src/umac/pdus/mac_sysinfo.rs)

Geprüfter Blob:

**5cef98764573b6fd09016c354bebade048c610cc**

Bestätigt:

- Main Carrier: 12 Bit.
- Frequency Band: 4 Bit.
- Frequency Offset: 2 Bit.
- Duplex Spacing: 3 Bit.
- Reverse Operation: 1 Bit.

**Status: implementiert.**

### 7.6 D-MLE-SYSINFO

Datei:

[crates/tetra-pdus/src/mle/pdus/d_mle_sysinfo.rs](https://github.com/JanHG98/netcore-tetra/blob/Archiving/crates/tetra-pdus/src/mle/pdus/d_mle_sysinfo.rs)

Geprüfter Blob:

**bb7fd347497f58c25a069b6e9ac0560f09450da5**

Bestätigt:

- Location Area: 14 Bit.
- Subscriber Class: 16 Bit.

**Status: implementiert.**

### 7.7 Teilnehmer-Whitelist

Datei:

[crates/tetra-config/src/bluestation/sec_security.rs](https://github.com/JanHG98/netcore-tetra/blob/Archiving/crates/tetra-config/src/bluestation/sec_security.rs)

Geprüfter Blob:

**a87ee2bcaa548868b51b8eadafa1dc57bcd1bc0a**

Aktuelles Verhalten:

- nicht leere <code>issi_whitelist</code> → nur gelistete ISSIs zugelassen;
- leere Liste → keine Beschränkung durch diese Whitelist.

Die MM-Implementierung nutzt diese Policy unter anderem bei Registrierungs-/Recoverypfaden.

**Status: implementiert.**

### 7.8 GSSI und Group Core

Datei:

[system-backend/group-core/README.md](https://github.com/JanHG98/netcore-tetra/blob/Archiving/system-backend/group-core/README.md)

Geprüfter Blob:

**68ee1fac1f81cc5c9616b7d61bf6bc9134a7a1a4**

Der Group Core ist als zentraler Dienst für:

- GSSI-Stammdaten,
- Mitgliedschaften,
- am Prüfdatum vorliegende Affiliationen,
- DGNA

dokumentiert.

**Status: Dienst/Architektur im Repository vorhanden.**
Ein Live-Test des Group Core wurde in dieser Entwicklungsphase nicht durchgeführt.

### 7.9 Projektwiki zu ISSI/GSSI

Datei:

[wiki/ISSI-and-GSSI.md](https://github.com/JanHG98/netcore-tetra/blob/Archiving/wiki/ISSI-and-GSSI.md)

Geprüfter Blob:

**822089ca3b00cf5299d8e3789204999cc62f2bfe**

Dort wird die ISSI als individuelle Identität im 24-Bit-Raum und die GSSI als unabhängige Gruppenidentität geführt.

---

## 8. Kritischer Fund: Duplex-Spacing-Kommentar widerspricht dem ausführbaren Code

### 8.1 Fallback-Config

Datei:

[config.toml.fallback](https://github.com/JanHG98/netcore-tetra/blob/Archiving/config.toml.fallback)

Geprüfter Blob:

**92f501078008bf58dd7a9b0bf34ee69861ec0ae4**

Dort steht im am Prüfdatum vorliegenden Stand sinngemäß:

~~~toml
freq_band = 4
main_carrier = 720
duplex_spacing = 0   # Kommentar behauptet 5 MHz bei 400 MHz
~~~

### 8.2 Ausführbare Duplextabelle

Datei:

[crates/tetra-core/src/freqs.rs](https://github.com/JanHG98/netcore-tetra/blob/Archiving/crates/tetra-core/src/freqs.rs)

Geprüfter Blob:

**aa8469b09ef10f28929d2771f1796277c4ca4006**

Der ausführbare Code verwendet eine Tabelle gemäß ETSI TS 100 392-15. Für:

- Frequency Band = 4
- Duplex-Spacing-Index = 0

liefert die am Prüfdatum vorliegende Tabelle **10.000 kHz = 10 MHz**.

Zusätzlich enthält dieselbe Datei einen Rust-Test, der für Band 4 / Index 0 **10.000.000 Hz** erwartet.

### 8.3 Bewertung

**Status: am Prüfdatum vorliegender Repository-Widerspruch gefunden; nicht bei der dokumentierten Prüfung behoben.**

Der Kommentar im Fallback-Config ist damit irreführend. Das ist besonders relevant, weil die Checkliste die Duplex-Tabellen-ID ausdrücklich umfasst.

Folgerung:

- Duplex-Spacing niemals aus dem alten Kommentar ableiten.
- Der 3-Bit-Index muss mit der tatsächlich verwendeten Duplextabelle des Stacks und der Endgeräte übereinstimmen.
- Vor einer HF-Inbetriebnahme muss dieser Kommentar außerhalb des Archivs korrigiert beziehungsweise die beabsichtigte Frequenzplanung ausdrücklich validiert werden.
- Bei bewusst eigener Duplextabelle darf <code>custom_duplex_spacing</code> nur verwendet werden, wenn die Funkgeräte dieselbe Zuordnung kennen.

**Priorität: hoch**, weil ein falscher Duplexabstand unmittelbar dazu führt, dass Downlink und erwarteter Uplink nicht zusammenpassen.

---

## 9. Korrekturregister der frühen Entwurfsfassungen

Dieser Abschnitt ist wichtig, damit die Archivdatei nicht frühere Fehler konserviert.

| Frühere Aussage | Bewertung | Korrigierter Stand |
|---|---|---|
| Colour Code 0–15 | **verworfen / falsch** | Colour Code ist 6 Bit → **0–63**. |
| Timeslotnummerierung 0–3 | **verworfen als Protokollbeschreibung** | TETRA-Slots werden als **1–4** behandelt; zweibitige Codierung wird intern entsprechend umgesetzt. |
| „4 Sprach + 1 Daten“-Timeslots | **verworfen / falsch** | Ein TDMA-Frame besitzt insgesamt **4 Timeslots**. Control/Traffic werden innerhalb dieser Struktur abgebildet. |
| generische 16-Bit Cell ID als Pflichtparameter | **verworfen** | Kein solches top-level Pflichtfeld im am Prüfdatum vorliegenden NetCore-Zellmodell. |
| RACH pauschal „TS3 in Multiframe 18“ | **verworfen / nicht belegt** | Random Access folgt den Control-/Access-Prozeduren; keine solche universale feste Zuordnung hier übernehmen. |
| Guard Time pauschal 2,5 ms | **verworfen / nicht ausreichend belegt** | Burst-/Guard-Timing direkt aus EN 300 392-2 implementieren, nicht aus dieser alten Kurzantwort. |
| „255 Bit pro Burst“ als generische Burstlänge | **verworfen / zu pauschal** | Es existieren unterschiedliche Burst-/Kanalstrukturen. |
| GPS/NTP zwingend für TDMA | **verworfen als Zwangsaussage** | stabile Zeit-/Frequenzbasis erforderlich; GPS nur eine mögliche technische Quelle. |
| Duplex-ID 1 beziehungsweise pauschal 10 MHz | **verworfen als allgemeine Regel** | Frequency Band + 3-Bit Duplex-Spacing-Index + Tabelle bestimmen den Abstand. |
| „MNC 99 Test“ oder bestimmte BOS-Beispiele | **nur Beispiel, keine Projektfestlegung** | tatsächliche Netzwerte aus dem eigenen autorisierten Netzplan verwenden. |
| SDS-Center-Adresse 0xFFFE | **nicht als Pflichtparameter übernommen** | kein aus dieser Entwicklungsphase verifizierter allgemeiner TBS-Pflichtwert. |
| Authentifizierung „Off“ oder TEA1 als Default | **nicht beschlossen** | Security separat und nach tatsächlich implementierter/gewünschter Sicherheitsklasse planen. |
| RX Sensitivity –112 dBm, 10–40 W TX usw. | **nur generische Beispiele** | reale Grenzwerte hängen von Hardware, Zulassung und Konformitätsziel ab. |
| IQ-Sampling 288 ksps und Clock <0,1 ppm als feste Projektwerte | **nicht beschlossen** | konkrete SDR-/Clockparameter aus Hardware, Modulationsqualität und Messung ableiten. |

---

## 10. Historischer Planungsstand und Repository-Abgleich vom 04.10.2026

### 10.1 Historischer Erkenntnisstand

Die Planung endete inhaltlich bei der Erkenntnis:

- Standalone-TBS;
- keine Nachbarn nötig;
- Netz-/Teilnehmer-/Gruppenparameter werden benötigt;
- für den Eigenbau zusätzlich PHY/MAC/L3-Belange;
- Modulation heißt π/4-DQPSK.

Es wurde **keine konkrete finale Config** mit realen zu verwendenden Werten beschlossen.

### 10.2 Heutiger, zusätzlich geprüfter Repository-Stand

Heute ist im Repository bereits deutlich mehr konkret:

- strukturierte Netzconfig für MCC/MNC;
- strukturierte Zellconfig für Carrier/Band/Offset/Duplex/LA/Colour Code/System Code;
- Frequenzberechnung und Duplextabelle;
- PDU-Encoding/Parsing der relevanten SYSINFO-/SYNC-Felder;
- ISSI-Whitelist;
- Mobility-Management mit Teilnehmerzuständen;
- Gruppen-/Affiliationslogik;
- Group Core und weitere Backend-Dienste.

Diese Implementierungen sind **nicht durch diese Planung entstanden**. Sie werden hier nur als geprüfter Vergleichsstand dokumentiert.

### 10.3 Nicht verifiziert

Nicht geprüft beziehungsweise durch diese Planung nicht nachgewiesen:

- welche Konfiguration zum Prüfdatum auf der realen TBS geladen ist;
- ob das ausführbare Binary exakt dem geprüften Branch entspricht;
- welche MCC/MNC/LA/CC-Werte zum Prüfdatum on-air gesendet werden;
- ob die realen Uplink-/Downlinkfrequenzen korrekt gemessen wurden;
- ob Endgeräte zum Prüfdatum erfolgreich campen und registrieren;
- ob alle gewünschten GSSIs funktionieren;
- Modulationsqualität/EVM;
- Ausgangsleistung und Nebenwellen;
- RX-Sensitivität beziehungsweise Desensibilisierung bei eigenem TX;
- Langzeitstabilität;
- regulatorische Freigabe der konkreten Aussendung.

---

## 11. Entwicklungs-, Installations- und Betriebsabläufe aus dieser Entwicklungsphase

### 11.1 Tatsächlich ausgeführt

In der ursprünglichen Planungsphase wurden **keine Installations-, Deployment- oder Reparaturbefehle** ausgeführt.

Im Rahmen der Quellenprüfung wurden nur Repository-Dateien und Standards lesend geprüft sowie diese Notizen unter Docs/archive/ geschrieben. Dies ist kein TBS-Funktionstest.

### 11.2 Nur vorgeschlagen, nicht ausgeführt

Frühere Entwürfe nannten sinngemäß:

- Konfigurationschecklisten;
- SDR-/IQ-Abgleich;
- Logging;
- BER/FER-/RSSI-/SINR-Monitoring;
- GPS/10-MHz-Referenzen;
- Loopback-Testmodi.

Keines davon wurde in dieser Entwicklungsphase auf der realen Basisstation ausgeführt oder bestätigt.

---

## 12. Fehler, Diagnose und Lösungen

### 12.1 Falscher Colour-Code-Bereich

**Fehler:** 0–15 genannt.

**Diagnose:** ETSI-/Repository-Abgleich zeigt 6-Bit-Feld.

**Lösung:** NetCore-Dokumentation und Konfiguration mit **0–63** behandeln.

**Status:** Korrektur in dieser Archivdokumentation abgeschlossen; keine Codeänderung erforderlich, da der am Prüfdatum vorliegende Code bereits 6 Bit modelliert.

### 12.2 Falsche Timeslotdarstellung

**Fehler:** 0–3 als TETRA-Timeslotnummern genannt.

**Diagnose:** am Prüfdatum vorliegender MAC-SYNC-Parser liest 2 Bit und bildet sie durch +1 auf die externe Nummerierung ab.

**Lösung:** technische Dokumentation auf Slots **1–4** vereinheitlichen.

### 12.3 Nicht existierendes fünftes Timeslot-Konzept

**Fehler:** „4 Sprache + 1 Daten“.

**Diagnose:** widerspricht der Vier-Slot-TDMA-Struktur.

**Lösung:** Traffic-, Control- und Datenkanäle als Nutzung der vier vorhandenen Slots behandeln.

### 12.4 Duplex-Kommentar widerspricht Implementierung

**Fehler:** Fallback-Config-Kommentar nennt 5 MHz für Band 4 / Index 0.

**Diagnose:** ausführbare Tabelle und Test erwarten 10 MHz.

**Lösung:** außerhalb dieses Dokumentationslaufs Config-Kommentar beziehungsweise intendierte Indexwahl korrigieren und danach Frequenzpaar messtechnisch prüfen.

**Status:** offen, Roadmap-Kandidat mit hoher Priorität.

---

## 13. Tests und Ergebnisse

### 13.1 Statischer Repository-Abgleich

**Status: durchgeführt**

Geprüft wurden auf Branch Archiving:

- Struktur und Feldbreiten von Netz-/Zellconfig;
- Carrier-/Duplexberechnung;
- MAC-SYNC;
- MAC-SYSINFO;
- D-MLE-SYSINFO;
- ISSI-Whitelist;
- Group-Core-Beschreibung.

**Ergebnis:** Die wesentlichen in diesen Notizen genannten Parameter sind im geprüften Code abgebildet.

### 13.2 Standardabgleich

**Status: durchgeführt, thematisch begrenzt**

Gezielt geprüft wurden unter anderem:

- π/4-DQPSK;
- Modulationsrate;
- Colour-Code-Feldbreite;
- MCC/MNC-Feldbreiten;
- LA;
- SSI/GSSI;
- Vier-Slot-TDMA;
- Carrier-/Duplexfelder.

**Grenze:** keine vollständige Konformitätsprüfung der TBS gegen alle relevanten ETSI-Klauseln.

### 13.3 Rust-Unit-Test zur Duplextabelle

Im Repository existiert ein Test, der Band 4 / Duplex-Index 0 mit 10 MHz erwartet.

**Wichtig:** Dieser Test wurde im Rahmen des Projektarchiven **nicht selbst ausgeführt**. Es wurde nur sein Quelltext überprüft. Deshalb Status:

**implementierter Test vorhanden, zum Prüfdatum nicht ausgeführt.**

### 13.4 RF-/Hardwaretest

**Status: nicht durchgeführt**

Keine Aussage zu:

- Leistung,
- EVM,
- BER,
- Empfindlichkeit,
- Spektrum,
- Duplexisolation,
- Endgeräteinteroperabilität

darf aus dieser Entwicklungsphase als erfolgreich getestet abgeleitet werden.

---

## 14. Verworfene oder ersetzte Ansätze

1. **Nachbarzellen als Pflichtbestandteil**\
   Für den geplanten Standalone-Betrieb verworfen. Erst bei Multicell erneut relevant.

2. **Absolute Uplinkfrequenz als einziger Frequenzparameter**\
   Durch das tatsächliche NetCore-Modell ersetzt: Band + Carrier + Offset + Duplexindex + Reverse-Flag; daraus wird UL/DL berechnet.

3. **GSSI als Teil der minimalen Zellidentität**\
   Präzisiert: GSSI gehört zur Gruppenkommunikation, nicht zur elementaren RF-/Netzidentität der Zelle.

4. **generischer 16-Bit Cell-ID-Zwang**\
   Verworfen, da weder aus dem am Prüfdatum vorliegenden top-level NetCore-Zellmodell noch aus dem hier geprüften Kontext als Pflicht ableitbar.

5. **GPS als zwingende Voraussetzung der Einzelzelle**\
   Ersetzt durch die allgemeinere Anforderung einer ausreichend stabilen und konformen Zeit-/Frequenzbasis.

---

## 15. Noch relevante Ideen, Wünsche und offene Aufgaben

### 15.1 Roadmap-Kandidaten – Priorität hoch

1. **Duplex-Spacing-Widerspruch beheben.**\
   Den irreführenden 5-MHz-Kommentar in <code>config.toml.fallback</code> gegen die tatsächliche Tabelle und gewünschte Funkplanung korrigieren.

2. **Verbindlichen RF-/Netzplan dokumentieren.**\
   Pro TBS beziehungsweise Netz eindeutig festlegen:
   - MCC
   - MNC
   - Frequency Band
   - Main Carrier
   - Frequency Offset
   - Duplex-Spacing-Index
   - Reverse Operation
   - daraus resultierender Downlink
   - daraus resultierender Uplink
   - LA
   - Colour Code
   - System Code.

3. **Parameter validieren, statt nur TOML zu parsen.**\
   Beim Start klare Range-/Konsistenzchecks für alle Luftschnittstellenfelder durchführen und abgeleitete UL/DL-Frequenzen sichtbar loggen.

4. **RF-Abnahme mit Messmitteln.**\
   Vor normalem Antennenbetrieb mindestens Frequenz, Leistung, Spektrum, Modulationsqualität und TX/RX-Kopplung messen.

### 15.2 Priorität mittel

5. **ISSI-Zulassungsmodell festlegen.**\
   Entscheidung zwischen offenem Labornetz und expliziter Whitelist; spätere zentrale Subscriber-Core-Policy dabei nicht mit lokaler RF-Registrierung vermischen.

6. **GSSI-Basissatz definieren.**\
   Nur für Gruppen, die tatsächlich benötigt werden. Gruppenstammdaten, Mitgliedschaft und am Prüfdatum vorliegende Affiliation getrennt halten.

7. **Standalone-Abnahmeskript beziehungsweise Checkliste.**\
   Einen reproduzierbaren Ablauf für:
   - BS start;
   - SYNC/SYSINFO prüfen;
   - MS campen;
   - Location Update;
   - ISSI-Allow/Reject;
   - Affiliation;
   - Gruppenruf;
   - Einzelruf;
   - SDS;
   - Release;
   - Neustart/Recovery
   erstellen.

8. **Dashboard/Provisioning-Wizard auf dieselbe Parametersemantik bringen.**\
   Begriffe „LA“, „Colour Code“, „Duplex Spacing Index“ und „Carrier“ ohne GSM-/LTE-Analogien darstellen.

### 15.3 Priorität später

9. Neighbor List und Zellwechsel erst bei echtem Multicell-Betrieb aktivieren.
10. Synchronisationskonzept zwischen mehreren Zellen festlegen.
11. Handover/Call Restore/Serving-TBS-Wechsel separat abnehmen.
12. Erweiterte Security/AIE/OTAR nach eigenem Sicherheitskonzept und Implementierungsstand behandeln.

---

## 16. Empfohlene Single-Cell-Konfigurationscheckliste

Die folgende Liste ist als Entwicklungscheckliste zu verstehen, nicht als konkrete Funkfreigabe:

~~~text
NETZ
[ ] MCC festgelegt und zulässig
[ ] MNC festgelegt und zulässig
[ ] Location Area festgelegt
[ ] System Code festgelegt
[ ] Colour Code 0..63 festgelegt

CARRIER
[ ] Frequency Band festgelegt
[ ] Main Carrier festgelegt
[ ] Frequency Offset festgelegt
[ ] Duplex-Spacing-Index gegen tatsächliche Tabelle geprüft
[ ] Reverse Operation geprüft
[ ] resultierender Downlink in Hz dokumentiert
[ ] resultierender Uplink in Hz dokumentiert
[ ] Frequenzpaar gegen Endgeräte-Codeplug geprüft

PHY
[ ] SDR-Treiber/Device festgelegt
[ ] RX/TX-Kanal festgelegt
[ ] Sample Rate passend
[ ] Gain/Leistungsweg abgestimmt
[ ] Clock/Frequenzstabilität vermessen
[ ] TX/RX-Isolation beziehungsweise Desense geprüft

ZUGANG
[ ] offene Registrierung oder ISSI-Whitelist bewusst gewählt
[ ] zulässige ISSIs dokumentiert
[ ] Subscriber Class / Access-Parameter geprüft

GRUPPEN
[ ] nur falls benötigt: GSSIs definiert
[ ] Mitgliedschaften/Policies gepflegt
[ ] reale Affiliation im Funkbetrieb geprüft

STANDALONE
[ ] Neighbor Broadcast deaktiviert/unbelegt
[ ] keine Handover-Abhängigkeit
[ ] lokale Rufe/SDS ohne fremde Zelle möglich

ABNAHME
[ ] SYNC decodierbar
[ ] SYSINFO korrekt
[ ] Endgerät campt
[ ] Location Update erfolgreich
[ ] unerlaubte ISSI wird korrekt abgewiesen, falls Whitelist aktiv
[ ] Gruppenaffiliation sichtbar
[ ] Gruppenruf funktioniert
[ ] Einzelruf funktioniert, falls im Zielumfang
[ ] SDS funktioniert, falls im Zielumfang
[ ] Call Release gibt Ressourcen frei
[ ] Neustart/Recovery geprüft
[ ] Spektrum und Modulationsqualität vermessen
~~~

---

## 17. Relevante Repository-Dateien und Quellen

### 17.1 Repository

- [config.toml.fallback](https://github.com/JanHG98/netcore-tetra/blob/Archiving/config.toml.fallback) – Beispiel-/Fallback-Konfiguration; enthält zum Prüfdatum den zu korrigierenden Duplex-Kommentar.
- [crates/tetra-config/src/bluestation/sec_net.rs](https://github.com/JanHG98/netcore-tetra/blob/Archiving/crates/tetra-config/src/bluestation/sec_net.rs) – MCC/MNC.
- [crates/tetra-config/src/bluestation/sec_cell.rs](https://github.com/JanHG98/netcore-tetra/blob/Archiving/crates/tetra-config/src/bluestation/sec_cell.rs) – Zellparameter.
- [crates/tetra-core/src/freqs.rs](https://github.com/JanHG98/netcore-tetra/blob/Archiving/crates/tetra-core/src/freqs.rs) – Carrier-/Duplexberechnung.
- [crates/tetra-pdus/src/umac/pdus/mac_sync.rs](https://github.com/JanHG98/netcore-tetra/blob/Archiving/crates/tetra-pdus/src/umac/pdus/mac_sync.rs) – System/Colour Code und TDMA-Zeit.
- [crates/tetra-pdus/src/umac/pdus/mac_sysinfo.rs](https://github.com/JanHG98/netcore-tetra/blob/Archiving/crates/tetra-pdus/src/umac/pdus/mac_sysinfo.rs) – Carrier/Band/Offset/Duplex/Reverse.
- [crates/tetra-pdus/src/mle/pdus/d_mle_sysinfo.rs](https://github.com/JanHG98/netcore-tetra/blob/Archiving/crates/tetra-pdus/src/mle/pdus/d_mle_sysinfo.rs) – Location Area und Subscriber Class.
- [crates/tetra-config/src/bluestation/sec_security.rs](https://github.com/JanHG98/netcore-tetra/blob/Archiving/crates/tetra-config/src/bluestation/sec_security.rs) – ISSI-Whitelist.
- [crates/tetra-entities/src/mm/mm_bs.rs](https://github.com/JanHG98/netcore-tetra/blob/Archiving/crates/tetra-entities/src/mm/mm_bs.rs) – Mobility Management, Zulassungs-/Recovery-/Affiliationspfade.
- [system-backend/group-core/README.md](https://github.com/JanHG98/netcore-tetra/blob/Archiving/system-backend/group-core/README.md) – GSSI-/Gruppenverwaltung.
- [wiki/ISSI-and-GSSI.md](https://github.com/JanHG98/netcore-tetra/blob/Archiving/wiki/ISSI-and-GSSI.md) – Projektbegriffe ISSI/GSSI.

### 17.2 ETSI-Unterlagen

Primäre Referenzen aus den bereitgestellten Anhängen:

- ETSI EN 300 392-1 V1.6.1 – General network design.
- ETSI EN 300 392-2 V3.8.1 – Air Interface.
- ETSI EN 300 392-5 V2.7.1 – Peripheral Equipment Interface.
- ETSI EN 300 394-1 V3.3.1 – Radio conformance testing.
- ETSI TS 100 392-15 wird vom am Prüfdatum vorliegenden NetCore-Frequenzcode explizit für Duplex Spacing referenziert; die entsprechende Logik ist in <code>freqs.rs</code> hinterlegt.

---

## 18. Abhängigkeiten für die nächste Fortsetzung

Bevor an einer neuen TBS-Konfigurationsmaske oder einem Provisioning-Wizard weitergearbeitet wird, sollten folgende Entscheidungen belastbar vorliegen:

1. Welcher **konkrete zulässige Frequenzplan** soll für die Einzelzelle verwendet werden?
2. Welcher Duplex-Spacing-Index entspricht diesem Frequenzplan in der tatsächlich verwendeten Tabelle?
3. Welche MCC/MNC sollen im konkreten Netz verwendet werden?
4. Soll das Labornetz offen registrieren oder eine ISSI-Whitelist erzwingen?
5. Welche GSSIs sind für die erste reale Funktionsabnahme notwendig?
6. Welcher SDR/Clock-Aufbau wird als Referenzhardware verwendet?
7. Welche Messmittel und Akzeptanzgrenzen gelten für die erste RF-Abnahme?

Erst danach sollte eine „Neue TBS“-Maske Werte automatisch provisionieren, damit nicht nur syntaktisch gültige, sondern fachlich richtige Parameter ausgerollt werden.

---

## 19. Schlussstand

**Festlegung für die weitere Entwicklung:**

**Für eine selbst gebaute Standalone-TETRA-Basisstation reichen MCC/MNC/GSSI/ISSI/LA/Colour Code/Uplink/Duplex-ID als unsortierte Liste nicht aus.**

Die Zelle benötigt eine konsistente Kombination aus:

- Netzidentität,
- Carrier-/Duplexparametern,
- Zellbroadcastparametern,
- PHY-/SDR-Konfiguration,
- optionaler Teilnehmerzulassung,
- und erst darüber liegend Gruppen-/Ruf-/Datendiensten.

Für den am Prüfdatum vorliegenden Standalone-Betrieb sind **Nachbarzellen nicht erforderlich**.

Die korrekte Modulationsbezeichnung ist **π/4-DQPSK**.

Die wichtigsten Korrekturen gegenüber den frühen Entwurfsfassungen sind:

- Colour Code **6 Bit / 0–63**, nicht 0–15;
- Timeslots **1–4**, nicht protokollseitig 0–3;
- kein fünfter „Daten-Timeslot“;
- kein generischer 16-Bit-Cell-ID-Zwang;
- Duplexabstand über **Band + 3-Bit-Index + Tabelle**, nicht über eine pauschale Zahl;
- GPS ist kein universelles Muss für eine einzelne Zelle;
- GSSI ist ein Gruppenparameter, kein zwingender Bestandteil der minimalen Zellidentität.

Der am Prüfdatum vorliegende NetCore-Tetra-Code bildet diese Kernparameter bereits weitgehend ab. Offen bleibt insbesondere die korrekte, verbindliche Funkplanung und die reale RF-/Endgeräteabnahme.
