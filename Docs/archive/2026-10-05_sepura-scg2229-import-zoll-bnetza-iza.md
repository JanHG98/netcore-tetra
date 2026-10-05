# Sepura SCG2229 Import aus Australien – Zoll, BNetzA, Konformität und erfolgreiche Freigabe

## Metadaten

- **Thema:** Import eines Sepura SCG2229 TETRA-Mobilfunkgeräts aus Australien; DHL-/Zollprozess; Marktüberwachung durch die Bundesnetzagentur; EU-Konformitätsnachweis; deutschsprachige Dokumentation; Einspruch/Neubewertung; erneute Internetzollanmeldung (IZA); erfolgreiche Abfertigung und Abholung.
- **Ursprünglicher Chattitel:** im zugänglichen Verlauf nicht zuverlässig verfügbar.
- **Chatlink:** im zugänglichen Verlauf nicht verfügbar.
- **Zusammenfassung erstellt:** 2026-10-05.
- **Zielbranch des Archivs:** `Archiving`.
- **Beim Repository-Abgleich geprüfter `Archiving`-Stand vor dieser Archivierung:** `013244c5d8c29a0fefc665f3f50897340e9d0d90` (muss unmittelbar vor dem Schreiben wegen möglicher paralleler Archivierungen nochmals geprüft werden).
- **Beim heutigen Code-/Dokumentationsabgleich geprüfter `main`-Stand:** `9116c15d645458f99e236712b67a1ad970432791`.
- **Repository:** `JanHG98/netcore-tetra`.
- **Archivumfang:** gesamter in diesem Chat zugänglicher Verlauf, die hier verfügbaren Anhänge/Scans sowie später ausdrücklich korrigierte Festlegungen. Frühere Aussagen werden bei Widersprüchen durch spätere explizite Angaben ersetzt.

> **Statuslegende:** **Idee** = noch unverbindlich; **beschlossen/geplant** = ausdrücklich vorgesehen, aber nicht nachgewiesen umgesetzt; **implementiert** = im beschriebenen Ablauf tatsächlich durchgeführt; **getestet** = Ergebnis durch konkrete Prüfung/Abnahme belegt; **im Betrieb bestätigt** = im realen NetCore-Tetra-Betrieb nachgewiesen. Eine Chat-Aussage allein gilt nicht als Implementierungsnachweis.

---

## 1. Ziel, Ausgangslage und behandelte Themen

Der Chat begann als Kosten- und Importabschätzung für ein in Australien gekauftes professionelles TETRA-Mobilfunkgerät und entwickelte sich zu einer vollständigen Fallstudie für die Einfuhr professioneller Funktechnik nach Deutschland.

Gegenstand war ein **Sepura SCG2229** als umfangreiches Installations-/Zubehörset. Im Verlauf wurden Kaufpreis, Versand, Zolltarifierung, CE-/UKCA-Konformitätsunterlagen, deutschsprachige Dokumentation, DHL-/Zollstatus, die Marktüberwachungsentscheidung der Bundesnetzagentur, der Einspruch gegen den Zollbescheid, die erneute Prüfung durch die BNetzA, die zweite IZA und die tatsächliche Abholung dokumentiert.

Für NetCore-Tetra ist der Chat aus zwei Gründen relevant:

1. Das SCG2229 ist als reales TETRA-Endgerät/Testgerät für spätere NetCore-Tetra-Versuche vorgesehen.
2. Der Fall liefert einen wiederverwendbaren **Import-/Compliance-Runbook-Entwurf** für professionelle Funkgeräte aus Nicht-EU-Ländern.

---

## 2. Kauf, Gerät und Ausgangsdaten

### 2.1 Kaufdaten

Im Chat wurden folgende Beträge genannt:

- Warenwert: **842,60 AUD**
- Versand: **115,02 AUD**
- australische GST: **7,40 AUD**
- Gesamtbetrag: **965,02 AUD**

Die erste grobe Kostenabschätzung lag bei etwa 645–670 EUR all-in. Diese war nur eine Vorabschätzung und wurde später durch reale Abgaben ersetzt.

### 2.2 Gerät

**Beschlossen/geplant / physisch vorhanden:**

- Modell: **Sepura SCG2229**
- Gerätetyp: professionelles TETRA-Mobilfunkgerät / Mobile TETRA Terminal
- vollständiges Installationsset mit umfangreichem Zubehör
- mindestens Bedienteil(e), Kabelbäume, Halterungen und weitere Sepura-/RFi-Komponenten sind auf den Chatbildern erkennbar

Im Chat wurden als erwartete bzw. vermutete Lizenzmerkmale genannt:

- DMO Gateway
- DMO Repeater
- 10 W RF

**Wichtig:** Diese Lizenzmerkmale wurden in diesem Chat **nicht per Radio Manager oder am Gerät ausgelesen und damit nicht technisch verifiziert**. Sie bleiben ein offener Prüfschritt.

### 2.3 Sichtbare Kennzeichnung

Auf einem im Chat archivierten Produktetikett sind am Karton/Gerät unter anderem **CE** und **UKCA** sichtbar. Die zuständige Zollbeamtin erinnerte sich beim späteren Abholtermin ausdrücklich daran, dass am Gerät das CE-Zeichen vorhanden war.

Gerätebezogene Serien-/TEI-/Bluetooth-IDs werden in dieser Markdown-Datei bewusst nicht noch einmal textuell vervielfältigt. Sie sind, soweit im Bild sichtbar, nur im archivierten Originalbild enthalten.

---

## 3. Chronologie des Import- und Behördenverfahrens

### 3.1 Versand und DHL

**Implementiert / real erfolgt:**

- Versand aus Australien im Juni 2026.
- DHL-Tracking zeigte zunächst normale Importbearbeitung.
- Später erfolgte die Weiterleitung an das zuständige Zollamt.
- Die Ware wurde dem Zoll übergeben und blieb dort während der Prüfung verwahrt.

Die archivierten DHL-Screenshots zeigen unter anderem:

- 17.06.2026: australisches Exportzentrum / Weitergabe ins Zielland
- 24.06.2026: Importprozess in Frankfurt
- 07.07.2026: Region des Empfängers
- 08.07.2026: Weiterleitung an Zoll / Übergabe an zuständiges Zollamt

### 3.2 Erste Vorführung beim Zoll

**Implementiert / real erfolgt:**

Der Nutzer erschien persönlich beim Zoll und legte umfangreiche Kauf-/Zahlungsunterlagen vor, darunter:

- eBay-Angebot/Kaufbestätigung
- PayPal-Nachweis
- Rechnung
- Kontoauszug
- DHL-Unterlagen

Die Sendung wurde vor Ort geöffnet. Später bestätigte der Nutzer, dass der Inhalt vollständig war.

Eine Zollunterlage beschrieb die Ware zeitweise als **„Tetrapol-Funkgerät (Digitalfunk)“**. Diese Bezeichnung war sachlich falsch: das SCG2229 ist ein **TETRA**-Gerät, kein Tetrapol-Gerät. Die fehlerhafte Bezeichnung wurde im Chat als redaktionelle/administrative Fehlklassifizierung erkannt.

### 3.3 Erste Marktüberwachungsentscheidung: Einfuhr abgelehnt

**Historischer Stand – später überholt:**

Nach Prüfung durch die Bundesnetzagentur wurde die Ware zunächst als nicht einfuhrfähig bewertet. Der Zoll nahm daraufhin die Annahme der Zollanmeldung zurück.

Als maßgebliche Beanstandungen wurden im Verlauf genannt:

- fehlende **EU-/EG-Konformitätserklärung bzw. deren Fundstelle**
- fehlende **deutschsprachige Gebrauchsanleitung / Produktdokumentation**

Die Bundesnetzagentur wurde im Zollverfahren als zuständige Fach-/Marktüberwachungsbehörde behandelt; der Zoll konnte deren produktsicherheitsrechtliche Bewertung nicht selbst ersetzen.

### 3.4 UKCA-Konformitätserklärung: echt, aber für EU-Zweck nicht die richtige Hauptunterlage

Zunächst wurde eine Sepura-Erklärung mit der Kennung **`249176-01_UKCA`** geprüft.

- Produktbezeichnung darin: `SCG2221 Mobile TETRA Terminal`
- Datum: 19.12.2024
- Bezug: britische `Radio Equipment Regulations 2017` und weitere UK-Regelungen
- Kennzeichnung: UKCA

**Bewertung im Chat:** Das Dokument ist eine echte Hersteller-Konformitätserklärung, aber eine **UKCA Declaration of Conformity**. Für die EU-/RED-Fragestellung war sie nicht die entscheidende Unterlage.

### 3.5 EU-Konformitätserklärung gefunden

Anschließend wurde die öffentlich verfügbare Sepura-EU-Erklärung gefunden und im Chat als zentraler Nachweis verwendet:

- **EU Declaration of Conformity**
- DoC-Nr. **`25176-07_EN`**
- Produktbezeichnung im Dokument: **`SCG2221 Mobile TETRA Terminal`**
- Hersteller: Sepura Limited
- Datum: **24.10.2025**
- Richtlinie **2014/53/EU (RED)**
- Richtlinie **2011/65/EU (RoHS)**
- diverse harmonisierte EN-Normen, unter anderem EN 301 489-Reihe, EN 303 758, EN 303 413, EN 300 328 (bei Wi-Fi/Bluetooth), EN IEC 62311 und EN IEC 63000

Öffentliche Fundstelle, die der Nutzer selbst recherchiert hat:

- https://sepura.com/wp-content/uploads/2025/04/5-07-scg2221-series-radio-eu-doc.pdf

#### Modellbezeichnungs-Hinweis

Das importierte Gerät ist **SCG2229**, während die EU-DoC in der vorliegenden Fassung **SCG2221** als Produktnamen trägt. Im Chat wurde dies als Erklärung für die SCG22-Serie behandelt; eine formale Hersteller-Mapping-Tabelle `SCG2221 ↔ SCG2229` wurde jedoch **nicht separat nachgewiesen**. Praktisch ist relevant, dass die BNetzA nach Vorlage der Unterlagen ihre Einfuhrbewertung später änderte. Für künftige Fälle wäre eine explizite Herstellerbestätigung der Variantenabdeckung dennoch die sauberste Dokumentation.

### 3.6 Deutschsprachiges SCG2229-Installationshandbuch gefunden

Als zweite zentrale Unterlage wurde das deutschsprachige **SELECTRIC-Installationshandbuch für das Sepura SCG2229**, Ausgabe **04/2023**, verwendet.

Öffentliche Fundstelle:

- https://digitalfunk.rlp.de/fileadmin/digitalfunk/Downloads/SCG2229_Installationshandbuch.pdf

Der im Chat geladene Stand umfasst 124 Seiten und behandelt unter anderem:

- Modellvarianten Dual-/Single-Console
- SCC3/HBC3
- bestimmungsgemäßen Gebrauch
- Sicherheits- und Warnhinweise
- Einhaltung gesetzlicher Bestimmungen
- HF-Energie und geprüfte Antennen
- Installation im Fahrzeug oder als Stand-Alone/Feststation
- Verkabelung und Stromversorgung
- Ethernet, USB, I/O, Lautsprecher
- TETRA-/GNSS-/Bluetooth-/Wi-Fi-Antennen
- SIM-Karte
- Programmierung/Konfiguration
- technische Daten

Damit lag eine gerätespezifische deutschsprachige Produkt-/Installationsdokumentation vor.

### 3.7 Einspruch beim Zoll und Zuständigkeitsklärung

Der Nutzer hatte gegen die Rücknahme der Annahme der Zollanmeldung Einspruch eingelegt.

Ein Schreiben des Hauptzollamtes Hannover vom **10.08.2026** stellte klar:

- die BNetzA Nürnberg hatte am **15.07.2026** die fehlende Einfuhrfähigkeit bestätigt
- die Zollbehörde ist nicht befugt, die produktsicherheitsrechtliche Fachentscheidung der BNetzA inhaltlich zu ersetzen
- für eine Änderung sollte sich der Nutzer direkt an die BNetzA wenden
- genannte BNetzA-Vorgangsnummer: **`VI 57086`**
- Kontaktweg im Schreiben: `poststelle@bnetza.de`
- der Nutzer sollte dem Zoll bis **15.09.2026** mitteilen, ob er den Einspruch aufrechterhält bzw. wie er weiter verfahren will

Das Zoll-Rechtsbehelfsverfahren hatte zusätzlich ein eigenes GZ (`S 0624 B - RL 446/26 - B 1006` im Scan).

### 3.8 Schreiben an die BNetzA

**Implementiert / versendet:**

An die Bundesnetzagentur wurde unter Bezug auf **VI 57086** ein strukturiertes Schreiben gesendet. Inhaltlich wurde beantragt:

- erneute Prüfung der produktsicherheitsrechtlichen Entscheidung vom 15.07.2026
- Berücksichtigung der EU-Konformitätserklärung
- Berücksichtigung des deutschsprachigen SCG2229-Handbuchs
- Eingangsbestätigung
- Mitteilung der neuen Entscheidung an den Nutzer **und** an das Hauptzollamt Hannover
- konkrete Benennung etwaiger verbleibender Mängel, falls die BNetzA bei Nichtkonformität bleiben sollte

Der Nutzer versendete die E-Mail tatsächlich.

### 3.9 Neubewertung durch die BNetzA: Einfuhrhindernis entfällt

**Implementiert / behördlich bestätigt:**

Im späteren Zollschreiben wurde mitgeteilt, dass seitens der Bundesnetzagentur **nun keine ausreichenden Anhaltspunkte mehr für ein Einfuhrverbot** vorlagen. Zusätzlich wurde sinngemäß/faktisch festgehalten, dass es sich um eine Einzelfallentscheidung handelt und **kein Risiko erkennbar** sei.

Damit war die ursprüngliche Negativbewertung materiell überholt.

Das neue Zollschreiben trug das vom Nutzer im Schriftverkehr verwendete GZ:

- **`SV0626 B-VVV65-HA110301`**

Der Scan ist typografisch nicht in jeder Stelle gleich gut lesbar; für das Archiv gilt die vom Nutzer mehrfach explizit eingegebene Schreibweise mit `VVV` als maßgeblich.

### 3.10 Neue IZA verlangt

Der Zoll verlangte nun eine **neue Internetzollanmeldung (IZA)**, da die erste Anmeldung zurückgenommen worden war.

Frist laut Schreiben:

- Abholung mit neuer IZA bis **07.09.2026**

Hinweis im Schreiben zu Lagerkosten:

- **0,50 EUR pro Tag**
- bestimmte Zeiträume, insbesondere Dauer der Prüfung der Einfuhrfähigkeit durch die Marktüberwachungsbehörde und Teile des Rechtsbehelfsverfahrens, sind ausgenommen
- Beträge unter 5 EUR werden nicht erhoben

Im Chat wurde daraus nur eine grobe theoretische Obergrenze abgeleitet; die tatsächlichen Lagerkosten wurden später **nicht separat ausgewiesen**.

---

## 4. Internetzollanmeldung – wiederverwendbare technische Daten

### 4.1 Erste IZA (historische Referenz)

Alte ATLAS-/IZA-Auftragsnummer:

- **`26/DE/5102/G/I/0/002XR/R/7`**

Diese Anmeldung war technisch die Vorlage für die zweite IZA, wurde aber aufgrund der damaligen Fachentscheidung zurückgenommen.

### 4.2 Zweite IZA – tatsächlich neu erstellt

**Implementiert:**

Neue Auftragsnummer:

- **`26/DE/5102/H/I/0/004SN/R/0`**

Datum:

- **26.08.2026**

Bearbeitende Dienststelle:

- **5102**

### 4.3 Allgemeine Felder

Verwendete Werte:

- Anmeldung: `IM`
- Anmeldeart: `A`
- Art des Geschäfts: `12`
- Statistikstatus: `04`
- Währung: `AUD`
- in Rechnung gestellter Gesamtbetrag: `965,02`
- Vorsteuerabzug: `N`
- Zahlungsart: `A`
- Anmelder ist Empfänger: `J`
- Vertretungsverhältnis: keine
- Steuerbeteiligter in anderem Mitgliedstaat: nein

Versender/Ausführer:

- Nepia Simeon
- Caboolture, Australien

Empfänger/Anmelder:

- Jan Hoffmeister
- Deutschland

### 4.4 Versand-/Transport-/Vorpapierdaten

Verwendete Werte:

- Versendungs-/Ausfuhrland: `AU`
- Bestimmungsland: `DE`
- Bestimmungsbundesland: `03`
- Art des grenzüberschreitenden aktiven Beförderungsmittels: `07`
- Kennzeichen Beförderungsmittel bei Ankunft: `unbekannt`
- Staatszugehörigkeit: `AU`
- Verkehrszweig an der Grenze: `5`
- Eingangszollstelle: `DE009901`
- Container: `N`
- Lieferbedingung: `CPT`
- Lieferort: `Hannover`
- Lieferbedingung Schlüssel: `1`
- Vorpapierart: **`PUEB`**
- Vorpapiernummer: **`03/139`**

**Wichtige Erkenntnis:** Das Geschäftszeichen des Behördenvorgangs ist **kein Ersatz** für die Vorpapiernummer. `PUEB 03/139` bezeichnet den Postübergabebogen der konkreten Sendung. Das Behörden-GZ gehört in Begleitkommunikation/Positionszusatz, nicht in das Vorpapierfeld.

### 4.5 Positionsdaten

Verwendete Warenbezeichnung:

> `Professionelles TETRA-Mobilfunkgerät (Sepura SCG2229), gebraucht, mit Zubehör`

Verwendete Warennummer:

- **`85176990000`**

Weitere Werte:

- Verfahren: `4000`
- beantragte Begünstigung: `100`
- Ursprungsland: `AU`
- Packstückart: `PK`
- Packstückanzahl: `1`
- Rohmasse: `5,0 kg`
- Eigenmasse: `5,0 kg`
- Artikelpreis: `965,02 AUD`
- Zollwert in der neuen IZA: `965,02` (Formularwert)
- statistischer Wert der neuen IZA: **565**

#### Historische Abweichung beim statistischen Wert

Die erste IZA zeigte als statistischen Wert **610**, die neue IZA **565**. Das ist ein gutes Beispiel dafür, dass solche Werte nicht blind aus alten Unterlagen kopiert werden sollten; der aktuelle IZA-/ATLAS-Lauf bzw. der jeweils zugrunde gelegte Kurs/Statistikwert ist maßgeblich.

### 4.6 Positionszusatz

In die zweite IZA wurde als Zusatz aufgenommen:

> `Erneute Zollanmeldung nach abgeschlossener Prüfung der Einfuhrfähigkeit durch die Bundesnetzagentur;`

Damit war für die Sachbearbeitung erkennbar, warum eine zweite Anmeldung zur gleichen Sendung existierte.

### 4.7 Unterlagencodes

In der alten und neuen IZA wurden insbesondere verwendet:

- `7HHD` – eBay Kaufbestätigung – 12.06.2026
- `7HHW` – Kontoauszug – 15.06.2026
- `7HHW` – PayPal-Ausdruck – 12.06.2026

Im neuen Ausdruck waren eBay-Kaufbestätigung und Kontoauszug als vorhanden (`J`) markiert, der PayPal-Ausdruck als nicht vorhanden (`N`).

Für das neue Freigabe-/Neubewertungsschreiben wurde **kein künstlicher Unterlagencode erfunden**. Es wurde stattdessen separat in der Kommunikation mit dem Zoll vorgelegt bzw. mitgeschickt.

---

## 5. Kommunikation mit dem Zoll nach der neuen IZA

Die neue IZA wurde per E-Mail an die Postabfertigung des Zollamtes Hannover-Nord gesendet.

Am **27.08.2026** antwortete die Zollstelle sinngemäß:

- keine weiteren Unterlagen erforderlich
- Rechnung liegt bereits vor
- Paket kann innerhalb der Öffnungszeiten im Zollamt abgefertigt werden

Damit war klar, dass der Nutzer ohne zusätzlichen Papierstapel zur Abfertigung erscheinen konnte.

**Im Betrieb des Zollverfahrens bestätigt:** Die Behörde akzeptierte die neue IZA als ausreichende Grundlage für die abschließende Abfertigung.

---

## 6. Erfolgreiche Abfertigung und Kosten

### 6.1 Tatsächliche Abgaben

Bei der finalen Abfertigung wurden **112,40 EUR** gezahlt.

Der Chat nennt diesen Betrag als finalen an der Zollstelle gezahlten Betrag. Eine Trennung in EUSt, eventuellen Zoll und Lagerkosten wurde im Chat nicht weiter dokumentiert.

### 6.2 Korrektur einer früheren Chat-Aussage zum Gesamtpreis

Eine frühere Assistenzantwort addierte **565 EUR** (statistischer Wert der IZA) und **112,40 EUR** und nannte daraus **677,40 EUR** Gesamtpreis.

**Diese Rechnung ist nicht belastbar und wird hier ausdrücklich verworfen.**

Grund:

- `565` ist der **statistische Wert** der neuen IZA und nicht nachgewiesenermaßen der tatsächlich vom Konto abgebuchte EUR-Kaufpreis.
- Der reale Kauf wurde mit **965,02 AUD** bezahlt.
- Der tatsächlich verwendete PayPal-/Bank-Euro-Wechselkurs bzw. der reale EUR-Abbuchungsbetrag wurde im Chat nicht abschließend genannt.

Daher gilt als korrekter Abschlussstand:

> **Gesamtkosten = tatsächlicher EUR-Abbuchungsbetrag für 965,02 AUD + 112,40 EUR Abgaben + ggf. tatsächlich erhobene separate Lagerkosten.**

Der exakte All-in-EUR-Betrag bleibt offen, bis der damalige EUR-Abbuchungsbetrag aus PayPal/Konto nachgetragen wird.

### 6.3 Ware physisch übergeben

**Implementiert / real bestätigt:**

- Paket wurde abgefertigt und ausgehändigt.
- Ein Chatfoto zeigt den Zollkarton im Fahrzeug mit Markierungen wie `Produktsicherheit`, `Einspruch` und `Freigabe!`.
- Der Nutzer bestätigte, dass die Ware bereits bei der ersten Prüfung geöffnet worden war und vollständig war.
- Ein späteres Foto zeigt den geöffneten Karton, dicht gefüllt mit SCG2229 und Zubehör.

---

## 7. Technischer Stand des SCG2229 nach dem Import

### 7.1 Physischer Zustand

**Getestet im Sinne der Sicht-/Vollständigkeitsprüfung:**

- Hauptgerät vorhanden
- Zubehörkarton umfangreich und augenscheinlich vollständig
- mehrere Kabel, Bedienelemente/Control-Hardware, Halterungen und weitere Installationskomponenten sichtbar
- CE-/UKCA-Kennzeichnung sichtbar

### 7.2 Noch nicht technisch getestet

Nicht Bestandteil dieses Chats waren:

- Einschalten und vollständiger Funktionstest des SCG2229 nach Abholung
- Radio-Manager-Auslesung
- Firmwarestand
- Lizenzbestand
- Bestätigung DMO Gateway
- Bestätigung DMO Repeater
- Bestätigung 10-W-RF-Lizenz
- PEI-/USB-/Ethernet-Test
- Registrierung am NetCore-Tetra-Netz
- Gruppenruf / Einzelruf / SDS
- DMO/TMO-Test
- Audio/PTT
- Langzeit-/Thermaltest

Diese Punkte sind **offen**.

---

## 8. Bezug zum aktuellen NetCore-Tetra-Repository

### 8.1 Heutiger `main`-Stand

Geprüft wurde `main` bei Commit:

- **`9116c15d645458f99e236712b67a1ad970432791`**
- Commit-Message: `docs: prioritize TBS execution of central group assignments`

Für den Begriff `SCG2229` ergab die GitHub-Code-Suche am geprüften Stand **keinen direkten Treffer**.

Für `Sepura` existieren dagegen mehrere aktuelle Repository-Dokumente und Testbezüge. Beispiel:

- `Docs/SEPURA_REREG_FRAME18_BNCH.md`

Dieses Dokument beschreibt reale Sepura-REREG-Beobachtungen an `SRV-M-TBS-01`, zwei Carriern und eine Frame-18/BNCH-Korrektur. Damit ist **Sepura als Endgerätefamilie im aktuellen NetCore-Tetra-Testkontext klar relevant**, aber das in diesem Chat importierte SCG2229 ist im geprüften Repository noch nicht als eigenes inventarisiertes/abgenommenes Gerät dokumentiert.

### 8.2 Kein Code-Implementierungsnachweis aus diesem Chat

Dieser Chat implementierte **keinen neuen NetCore-Tetra-Code**. Der erfolgreiche Import ist Hardware-/Compliance-/Betriebslogistik, kein Software-Feature.

Daher sind folgende Aussagen strikt zu trennen:

- **Implementiert:** Behörden-/Importablauf, neue IZA, tatsächliche Abholung.
- **Nicht implementiert:** eine besondere SCG2229-Integration im Repository.
- **Nicht getestet:** das konkrete importierte SCG2229 gegen den aktuellen NetCore-Tetra-Stand.

---

## 9. Funktionsfähiger Import-/Compliance-Ablauf für zukünftige Geräte

Der wichtigste wiederverwendbare Prozess aus diesem Chat ist:

1. **Korrekte Warenbezeichnung** verwenden, z. B. `Professionelles TETRA-Mobilfunkgerät (Sepura SCG2229), gebraucht, mit Zubehör`.
2. Kaufbelege und Zahlungsnachweise bereithalten.
3. Bei professioneller Funktechnik aus einem Drittland direkt bereithalten bzw. bei Rückfrage mitsenden:
   - **EU Declaration of Conformity (RED)**
   - **deutschsprachige Anleitung / Installationsdokumentation**
   - ggf. Foto/Beleg der CE-Kennzeichnung
4. Keine UKCA-Erklärung mit einer EU-RED-Erklärung verwechseln.
5. Wenn Marktüberwachung eingeschaltet wird, deren Vorgangsnummer separat dokumentieren.
6. Behörden-Geschäftszeichen nicht als ATLAS-Vorpapiernummer missbrauchen.
7. Bei zurückgenommener erster Zollanmeldung eine **neue IZA** erstellen; kaufbezogene Daten bleiben gleich, verfahrens-/kursbezogene Daten sind neu zu prüfen.
8. Vorpapier der konkreten Postsendung beibehalten, wenn es weiterhin dieselbe Gestellung/Sendung ist; im Fall dieses Chats war das `PUEB 03/139`.
9. Im Positionszusatz den Zusammenhang zur erneuten Prüfung knapp erläutern.
10. Neue IZA vorab per E-Mail an die zuständige Postabfertigung schicken und Rückmeldung abwarten.

### Praxishinweis des Zolls

Die zuständige Zollbeamtin empfahl für einen nächsten vergleichbaren Import ausdrücklich sinngemäß:

> **DoC und Anleitung direkt an die Mail anhängen.**

Das ist die wichtigste operative Lehre des gesamten Falls.

---

## 10. Fehler, Diagnose, Ursachen und funktionierende Lösungen

### Fehler 1: falsche Warenbezeichnung „Tetrapol“

- **Status:** historischer Behörden-/Dokumentationsfehler
- **Ursache:** nicht abschließend bekannt; vermutlich administrative Fehlbezeichnung
- **Korrektur:** klare Warenbezeichnung TETRA / Sepura SCG2229 in der IZA

### Fehler 2: nur UKCA-Dokument betrachtet

- **Status:** überholter Ansatz
- **Problem:** UKCA ist nicht die maßgebliche EU-RED-Konformitätserklärung
- **Lösung:** offizielle Sepura EU Declaration of Conformity nachreichen

### Fehler 3: deutsche Anleitung angeblich fehlend

- **Status:** behoben
- **Lösung:** öffentlich verfügbares deutschsprachiges SELECTRIC-Installationshandbuch SCG2229, Ausgabe 04/2023, nachreichen

### Fehler 4: Zoll kann BNetzA-Fachentscheidung nicht selbst ändern

- **Status:** Zuständigkeit geklärt
- **Lösung:** direkte Neubewertung bei der BNetzA unter `VI 57086` beantragen; Ergebnis an Nutzer und Zoll verlangen

### Fehler 5: erste IZA konnte nach Fachentscheidung nicht weiterverwendet werden

- **Status:** behoben
- **Lösung:** neue IZA mit neuer Auftragsnummer und aktuellem Datum erstellen; gleiche Kaufdaten, aber aktuelle verfahrensbezogene Felder prüfen

### Fehler 6: Behörden-GZ als mögliches Vorpapier erwogen

- **Status:** verworfen
- **Grund:** Vorpapierfeld erwartet zollrechtliches Vorgängerdokument, nicht Aktenzeichen
- **Lösung:** `PUEB 03/139` beibehalten; GZ nur in Begleitkommunikation/Positionszusatz

### Fehler 7: falsche All-in-Kostenrechnung aus statistischem Wert

- **Status:** hier korrigiert
- **Lösung:** realen EUR-Abbuchungsbetrag der 965,02 AUD verwenden; statistischen Wert nicht als Kaufpreis interpretieren

---

## 11. Tests und Ergebnisse

### Durchgeführt und bestätigt

- **Sichtprüfung/Vollständigkeit:** Ware beim Zoll geöffnet; Inhalt laut Nutzer vollständig.
- **Dokumentenprüfung praktisch erfolgreich:** Nach Einreichung von EU-DoC und deutschem Handbuch änderte die BNetzA die Bewertung; kein ausreichender Anhaltspunkt für ein Einfuhrverbot mehr.
- **IZA-Neuanmeldung:** erfolgreich erstellt.
- **Zoll-Kommunikation:** Zoll bestätigte, dass keine weiteren Unterlagen benötigt werden.
- **Abfertigung:** erfolgreich.
- **Zahlung:** 112,40 EUR.
- **Abholung:** erfolgreich; Paket physisch beim Nutzer.

### Grenzen der Tests

- Keine technische Funkprüfung des importierten Geräts in diesem Chat.
- Keine Lizenz-/Firmwareauslesung.
- Keine Netzregistrierung oder RF-Abnahme.
- Keine formale Hersteller-Mapping-Unterlage SCG2221-DoC zu SCG2229 separat dokumentiert.

---

## 12. Verworfene oder ersetzte Ansätze

- **UKCA als ausreichende EU-Erklärung:** ersetzt durch EU-DoC nach RED.
- **Zoll-Einspruch allein als Weg zur Fachentscheidung:** ergänzt/ersetzt durch direkte BNetzA-Neubewertung, da nur die Fachbehörde ihre Produktsicherheitsentscheidung ändern konnte.
- **Geschäftszeichen als Vorpapiernummer:** verworfen; `PUEB 03/139` blieb das richtige zollrechtliche Vorpapier.
- **alte IZA nur mit neuem Datum wiederverwenden:** teilweise richtig, aber verfahrens-/kursbezogene Felder mussten neu geprüft werden; es wurde eine neue IZA mit neuer Auftragsnummer erstellt.
- **677,40 EUR als sicherer All-in-Preis:** verworfen; beruhte auf Verwechslung des statistischen Werts mit dem realen EUR-Kaufpreis.

---

## 13. Roadmap-Kandidaten und offene Aufgaben

### P0 – Gerät technisch inventarisieren

**Beschlossen/geplant:**

- SCG2229 einschalten und mit Sepura Radio Manager auslesen.
- Firmwarestand dokumentieren.
- vollständigen Lizenzbestand dokumentieren.
- DMO Gateway / DMO Repeater / 10 W RF konkret verifizieren.
- sichtbare Zubehörteile anhand Part Numbers inventarisieren.

### P1 – NetCore-Tetra-Gerätetest

**Idee / sinnvoller nächster technischer Schritt:**

- SCG2229 als eigenes Testgerät in die NetCore-Tetra-Testmatrix aufnehmen.
- TMO-Registrierung am aktuellen `main` testen.
- Gruppenruf / Einzelruf / SDS testen.
- PEI/USB/Ethernet-Fähigkeiten prüfen.
- REREG-/Frame-18-Verhalten mit diesem konkreten Gerät gegen aktuelle Sepura-Fixes testen.
- DMO-Funktionen getrennt und nur im zulässigen Testsetup prüfen.

### P1 – Hardware-/Asset-Dokumentation

**Roadmap-Kandidat:**

- Gerät mit Modell, Variante, Part Number und Zubehör in die NetCore-Tetra-Hardware-/Asset-Dokumentation aufnehmen.
- Serielle IDs nur dort ablegen, wo deren Veröffentlichung bewusst gewollt ist.

### P2 – Import-Runbook als allgemeine Projektdokumentation

**Roadmap-Kandidat, noch nicht außerhalb des Archivs umgesetzt:**

- allgemeines `Hardware Import / Compliance Runbook` erstellen.
- pro Hersteller Links zu EU-DoC und deutschsprachiger Anleitung pflegen.
- Checkliste: CE, RED-DoC, Handbuch, Rechnung, Zahlungsnachweis, korrekte Warenbezeichnung, Tarifierung, Vorpapier.

### P2 – DoC-Modellabdeckung klären

**Offen:**

- bei Bedarf explizite Hersteller-/Distributorbestätigung einholen, welche SCG2229-Varianten von der SCG2221/SCG22-EU-DoC abgedeckt sind.

### P3 – echte Gesamtkosten nachtragen

**Offen:**

- tatsächlichen EUR-Abbuchungsbetrag der 965,02 AUD aus PayPal/Konto erfassen.
- daraus finalen All-in-Preis inklusive 112,40 EUR Abgaben bestimmen.

---

## 14. Relevante Quellen und Anhänge

### Öffentlich verfügbare Hersteller-/Fachdokumente

- Sepura EU Declaration of Conformity: https://sepura.com/wp-content/uploads/2025/04/5-07-scg2221-series-radio-eu-doc.pdf
- SELECTRIC SCG2229 Installationshandbuch: https://digitalfunk.rlp.de/fileadmin/digitalfunk/Downloads/SCG2229_Installationshandbuch.pdf

### Im Chat verfügbare PDF-Anhänge

- `6-01-SCG2221-series-radio-UKCA.pdf`
- `Eu.pdf`
- `SCG2229_Installationshandbuch.pdf`
- `Scanned-image08-17-2026-155153.pdf`
- `Auftragsnummer (1).pdf` – alte IZA
- `Auftragsnummer.pdf` – neue IZA
- zusätzlich im lokalen Chat-Arbeitsbereich vorhandene frühere Zollscan-PDFs `Scanned-image07-18-2026-161644*.pdf`

Diese PDFs wurden für die Zusammenfassung ausgewertet bzw. im Chat diskutiert, aber in diesem Archivauftrag **nicht als Binärkopien in Git dupliziert**, weil die öffentlichen Herstellerunterlagen über stabile Fundstellen erreichbar sind und die Behördenunterlagen personenbezogene Korrespondenz darstellen. Die eigenständigen Chatbilder werden dagegen auf ausdrücklichen Wunsch unter `Docs/archive/assets/` archiviert.

### Relevante Repository-Dateien / heutiger Abgleich

- `Docs/SEPURA_REREG_FRAME18_BNCH.md` (`main`)
- weitere Sepura-bezogene Dokumente wurden über die Repository-Code-Suche gefunden; ein direkter `SCG2229`-Treffer war am geprüften Stand nicht vorhanden.

---

## 15. Archivierte Chatbilder

Im zugänglichen Verlauf wurden **15 eigenständige Chatbilder** identifiziert. Um sämtliche Bilder des Chats innerhalb des reinen Dokumentationsbereichs zu erhalten und die Repository-Größe handhabbar zu halten, werden sie verlustbehaftet optimiert und in **fünf hochauflösenden Kontaktbögen** unter `Docs/archive/assets/2026-10-05_scg2229-import/` archiviert. Jeder Kontaktbogen enthält drei Original-Chatbilder mit eingeblendeten Quelldateinamen. Damit bleibt der komplette visuelle Verlauf erhalten, ohne 15 große Einzeldateien zu duplizieren.

Enthaltene Motive in ursprünglicher Reihenfolge:

1. ursprüngliches Angebots-/Setfoto mit ausgebreitetem Zubehör,
2. ursprünglicher Kartoninhalt,
3. Kartonetikett SCG2229 mit CE/UKCA,
4. DHL-Tracking vor Zollweiterleitung,
5. DHL-Tracking „Weiterleitung an Zoll“,
6. Foto der vorübergehenden Verwahrung / VuB-Prüfung,
7. Schreiben zur neuen Einfuhrentscheidung, Seite 1,
8. Schreiben zur neuen Einfuhrentscheidung, Seite 2,
9. E-Mail der Postabfertigung: keine weiteren Unterlagen erforderlich,
10. Screenshot IZA allgemeine Angaben,
11. Screenshot Versender/Empfänger/Anmelder,
12. Screenshot Versand/Transport/Vorpapier,
13. Screenshot Positionsdaten,
14. freigegebener Zollkarton im Fahrzeug,
15. geöffneter, voll gepackter Karton nach Abholung.

Archivdateien:

- `01_chatbilder_01-03.webp`
- `02_chatbilder_04-06.webp`
- `03_chatbilder_07-09.webp`
- `04_chatbilder_10-12.webp`
- `05_chatbilder_13-15.webp`

### Bildvorschau

![Chatbilder 01 bis 03](assets/2026-10-05_scg2229-import/01_chatbilder_01-03.webp)

![Chatbilder 04 bis 06](assets/2026-10-05_scg2229-import/02_chatbilder_04-06.webp)

![Chatbilder 07 bis 09](assets/2026-10-05_scg2229-import/03_chatbilder_07-09.webp)

![Chatbilder 10 bis 12](assets/2026-10-05_scg2229-import/04_chatbilder_10-12.webp)

![Chatbilder 13 bis 15](assets/2026-10-05_scg2229-import/05_chatbilder_13-15.webp)

---

## 16. Auswertungslücken

- Der ursprüngliche Chattitel und ein direkter Chatlink stehen im zugänglichen Kontext nicht zur Verfügung.
- Der exakte reale EUR-Abbuchungsbetrag für die 965,02 AUD wurde nicht genannt; deshalb ist kein centgenauer All-in-EUR-Preis ableitbar.
- Eine formale Herstellerbestätigung der Abdeckung `SCG2221`-DoC für die konkrete `SCG2229`-Variante liegt im Chat nicht separat vor.
- Der finale BNetzA-interne Bescheid/Datensatz selbst liegt nicht als eigenes BNetzA-Schreiben vor; die geänderte BNetzA-Bewertung ist über das spätere Zollschreiben dokumentiert.
- Der Lizenz-/Firmwarestand des Geräts wurde im Chat nicht ausgelesen.
- Es wurde kein vollständiger technischer NetCore-Tetra-On-Air-Test mit dem importierten SCG2229 durchgeführt.

---

## 17. Abschlussstatus

**Import/Behördenverfahren:** **erfolgreich abgeschlossen**.

**Physische Ware:** **abgeholt und vollständig vorhanden**.

**Dokumentenstrategie für zukünftige Importe:** **praktisch bestätigt** – EU-DoC und deutschsprachige Anleitung direkt bereithalten bzw. an die erste relevante Zollkommunikation anhängen.

**NetCore-Tetra-Integration des konkreten SCG2229:** **noch offen**.

**Nächster technischer Schritt:** Gerät inventarisieren, Radio Manager auslesen, Lizenz-/Firmwarestand sichern und danach als definiertes Testendgerät gegen den aktuellen NetCore-Tetra-Stand abnehmen.