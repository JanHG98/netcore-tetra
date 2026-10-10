# Statusmeldungen

Statusmeldungen ordnen numerischen U-STATUS-Codes eine verständliche Darstellung zu.

## Felder

| Feld | Bedeutung |
|---|---|
| `code` | numerischer Statuswert |
| `label` | sichtbare Bezeichnung |
| `severity` | fachliche Gewichtung |
| `description` | ausführliche Erklärung |
| `color` | Darstellungsfarbe |
| `visible` | Sichtbarkeit |

## Verarbeitung

Bei Eingang eines U-STATUS:

- wird der numerische Wert protokolliert,
- Directory liefert – sofern erreichbar – Label und Darstellung,
- das Dashboard aktualisiert den Gerätestatus,
- ein passendes Home-Mode-Display kann beantwortet werden,
- Statusgruppen können synchronisiert werden,
- Notfallstatus kann die lokale Alarmkette auslösen.

## Notfallstatus

Status `0` entspricht im vorhandenen PDU-Modell dem Notfallstatus; zusätzlich behandelt die lokale SDS-Logik bestimmte zugeordnete Werte und gerätespezifische Sepura-Meldungen. Notfälle bleiben standardmäßig lokal an der Basisstation sichtbar und können optional an Telegram oder die Leitstelle gemeldet werden. Eine externe Weiterleitung sollte bewusst konfiguriert werden.

## Pflegehinweise

- Codes eindeutig halten.
- Label kurz genug für kleine Anzeigen wählen.
- Beschreibung für Bediener verständlich formulieren.
- Farben nicht als einziges Unterscheidungsmerkmal verwenden.
- Unsichtbare Statuswerte weiterhin dokumentieren, falls Endgeräte sie senden können.

## Weiterführend

[Gerätegruppen und Statusgruppen](geraete-und-statusgruppen.md) · [SDS und U-STATUS](kurznachrichten-und-status.md) · [Statusrückmeldung und Home Mode Display](statusrueckmeldung-und-home-mode-display.md)

Eine Quittierung in der Oberfläche beweist bei gerätespezifischer Notfallrücknahme noch nicht das Verlassen des Notfallmodus am Endgerät. Eine vollständige Statusprüfung umfasst Eingang, Anzeige, Weiterleitung nach Konfiguration und die Rücknahme am realen Gerät.

## Quellen zur Pflege dieser Seite

[Status- und Notfallverarbeitung](../../crates/tetra-entities/src/cmce/subentities/sds_bs.rs) · [Numerische Statuscodierung](../../crates/tetra-pdus/src/cmce/enums/pre_coded_status.rs).
