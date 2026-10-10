# ISSI und GSSI

## ISSI

Die ISSI identifiziert ein einzelnes TETRA-Endgerät oder einen Systemteilnehmer im jeweiligen Netzkontext. Der codierte SSI-Wert hat 24 Bit, also 0 bis 16.777.215; reservierte Werte und lokale Validierung machen diesen gesamten Zahlenraum nicht automatisch zu zulässigen Teilnehmeradressen.

Für NetCore wird ein planbares Nummernschema empfohlen. Ein mögliches Schema ist:

```text
D K EE NNNN
```

- `D` – Domäne
- `K` – Kategorie
- `EE` – Einheit/Eigentümer
- `NNNN` – laufende Nummer

Das Schema ist organisatorisch; auf der Luftschnittstelle bleibt es eine numerische ISSI. Es ergibt acht Dezimalstellen und darf nur Werte im tatsächlich erlaubten SSI-Bereich erzeugen. Für netzübergreifende Zuordnung MCC/MNC und ISSI/GSSI gemeinsam dokumentieren.

## GSSI

Die GSSI identifiziert eine Gesprächsgruppe. Sie ist unabhängig von der ISSI eines Endgeräts. Eine Gruppe kann vielen Geräten zugeordnet sein; ein Gerät kann mehrere Gruppen affiliieren.

## System-ISSIs

Systemteilnehmer wie Basisstation, Audioaussendung, Asterisk-Gateway oder Statussteuerung sollten eigene, dokumentierte ISSIs erhalten. Keine System-ISSI gleichzeitig als normales Handfunkgerät verwenden.

## Regeln

- IDs nicht spontan wiederverwenden.
- Produktions-, Test- und Simulationsbereiche trennen.
- Eigentümer und Verwendungszweck im Directory dokumentieren.
- lokale SSI-Bereiche in `[cell_info].local_ssi_ranges` bewusst setzen.
- Routing über Brew oder PBX getrennt von rein lokalen IDs planen.

## Weiterführend

[Provisioning und Datenhoheit](teilnehmer-und-gruppen-anlegen.md) · [Gesprächsgruppen und ihre Anzeigenamen](gespraechsgruppen-im-directory.md) · [Registrierung und Gruppenbindung](registrierung-und-gruppenbindung.md)

## Quellen zur Pflege dieser Seite

[SSI-Modell und Bereiche](../../crates/tetra-core/src/ranges.rs) · [Lokale SSI-Konfiguration](../../crates/tetra-config/src/bluestation/sec_cell.rs).
