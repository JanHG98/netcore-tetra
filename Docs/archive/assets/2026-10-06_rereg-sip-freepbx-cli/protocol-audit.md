# Gezielte Prüfung der zugänglichen Protokollunterlagen am 2026-10-06

Diese Prüfung betrifft die CLI-Diagnose im Chat. Sie ist keine vollständige Durchsicht sämtlicher PDF-Seiten und kein Endgeräte-/RF-Test. Alle 25 lokal vorhandenen ETSI-PDFs sowie zwei wiederhergestellte Sepura/SELECTRIC-Unterlagen sind mit Größe, Seitenzahl und SHA-256 in `pdf-inventory.json` erfasst (27 PDFs, 58.550.221 Bytes). Die Original-PDFs werden durch dieses Inventar nicht in das Repository übernommen.

## ETSI-Fundstellen

- `24-en_30039202v030801p.pdf`: ETSI EN 300 392-2 V3.8.1 (2016-08), S. 291, §14.7.1.12, Tabelle 14.15: D-SETUP enthält optional Calling Party Type Identifier, bedingt Calling Party SSI bzw. SSI + Calling Party Extension sowie optional External Subscriber Number. Eine fehlende Facility ist für sich kein Nachweis für einen fehlerhaften CLI-Aufbau.
- Dasselbe Dokument, S. 303, §14.8.9–14.8.11, Tabellen 14.43–14.45: CPTI `01` bezeichnet SSI; `10` bezeichnet TSI. Die SSI hat 24 Bit, die optionale Extension setzt sich aus 10 Bit Country Code und 14 Bit Network Code zusammen.
- S. 309, §14.8.20 und Tabelle 14.59: External Subscriber Number überträgt die externe Teilnehmernummer zwischen Funkteilnehmer und Gateway. Bis zu 24 Ziffern; Ziffernzahl = vorangestellte Type-3-Länge / 4. Jede Ziffer ist ein 4-Bit-Symbol in normaler Wählreihenfolge. Zusätzlich zu 0–9 sind `*`, `#` und `+` definiert.
- S. 317, §14.8.48, Tabelle 14.86: Type-3-ID `0010` (dezimal 2) steht für External Subscriber Number.
- S. 1301–1303, Annex E, Tabelle E.1: Nach den Type-2-Feldern zeigt M=1 ein Type-3-Feld an; anschließend folgen 4 Bit Feld-ID, 11 Bit Nutzlänge in Bits und Nutzdaten. Nach dem letzten Feld folgt M=0. Bedingte Felder erhalten kein zusätzliches Presence-Bit.
- `15-en_30039201v010601p.pdf`: ETSI EN 300 392-1 V1.6.1 (2020-04), S. 30, §7.2.6 sowie S. 39, §7.8.2.2.2: SSI/TSI fungieren als Routingidentitäten; externe Rufe enthalten Gateway-ITSI/ISSI und externe Teilnehmeradresse. Diese Stellen rechtfertigen die Trennung einer Gateway-Identität von den eigentlichen Telefonziffern. Sie erklären die im Test benutzte SSI `16777184` nicht zur universellen reservierten PABX-Adresse.

## Heute wiederholte Prüfung des konkreten Log-Bitvektors

Der 99-Bit-D-SETUP-Vektor aus dem TBS-Log vom Chat wurde unabhängig nach den dokumentierten Feldbreiten zerlegt. Ergebnis: Call ID 4, optionaler Bereich ab Bit 40, CPTI `01`, Calling Party SSI `16777184`, Type-3-ID 2, Länge 12 Bit, Nutzdaten `0001 0000 0011`, finales M=0. Alle 99 Bit wurden genau verbraucht.

Damit ist `Type3FieldGeneric { field_id: 2, len: 12, data: 259 }` mit der externen Nummer **103** vereinbar: dezimal 259 entspricht hexadezimal `0x103`; die Nibbles bilden die drei Ziffern 1, 0, 3. Daraus darf weder eine übertragene Rufnummer „259“ noch ein Dezimal-/BCD-Konvertierungsfehler behauptet werden.

Grenzen: Geprüft wurde die im Chat sichtbare Bitdarstellung. Das ersetzt weder einen empfangsseitigen Luftschnittstellenmitschnitt noch den internen Parser des Sepura SC20. Es beweist nicht die endgültige Konfiguration nach dem erfolgreichen Outbound-CID-Versuch und nicht alle längeren Nummern/Präfixe.

## SELECTRIC-/Sepura-Unterlagen

- `17_04_2024_SELECTRIC-Netzwerk-VO-6.pdf`, S. 28/30: Parameter 9200.1.3/.4 betreffen minimale/maximale Zeichen der Rufnummerneingabe; .5 die führende Ziffer bei der Rufnummernwahl. .7/.8 sind Systemadressen für ein- bzw. ausgehende Nicht-TETRA-Gespräche. 9210.1.1 betrifft Halb-/Vollduplexfreigabe. Diese Beschreibungen belegen **keine allgemeine Pflicht**, einer eingehenden PABX-CLI eine `9` voranzustellen.
- S. 29/30 erläutert eine gekürzte Anzeige empfangener **TETRA-ISSIs** abhängig vom Modus „Verkürzte Wahl in das TETRA Netzwerk“. Diese Aussage lässt sich nicht ohne weitere Belege auf externe PABX-Nummern übertragen.
- S. 30/30: 9000.1.1 aktiviert den jeweiligen Wahlmodus; 9000.1.4 betrifft Rufannahme/Direktaufbau. Das ist mit den im Chat besprochenen Wahlmoduseinstellungen vereinbar, aber kein Nachweis eines konkreten CLI-Firmwarefehlers.
- `Sepura Software V 10.24-SC 2.0-SALT 2.pdf`: Titel nennt V10.24-003 und SC2.0-003, Änderungsstand 1.31 vom 09.11.2020. Gezielte Textsuche und Durchsicht der Funktionsübersicht ergeben keinen belastbaren Hinweis auf den konkreten Fehler „cannot display cli“ oder eine vorgeschriebene eingehende PABX-Präfix-9-Regel. Das Dokument ist keine vollständige Aussage über alle Firmwarefehler oder alle Displayregeln.

## Schlussfolgerung für das Archiv

Die protokollseitige Prüfung und die spätere Betriebsaussage müssen getrennt bleiben. Das ursprüngliche 103-D-SETUP zeigt eine nachvollziehbare CLI-Codierung. Die endgültige positive Aussage des Nutzers nach dem Auffinden/Setzen von **FreePBX → Nebenstelle 103 → General → Outbound CID** hebt den früheren Status „CLI ungelöst“ auf. Eine universelle Geräteinkompatibilität oder Präfixpflicht ist damit nicht nachgewiesen.
