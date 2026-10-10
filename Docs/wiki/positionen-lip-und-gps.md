# Positionen über LIP und GPS anzeigen

Positionsdaten können aus TETRA-LIP-Meldungen übernommen, im Dashboard angezeigt und optional an Directory oder Leitstelle exportiert werden.

## Voraussetzungen

- Endgerät sendet LIP in einem unterstützten Format.
- GPS ist im Endgerät aktiviert und hat einen gültigen Fix.
- ISSI ist bekannt und möglichst im Directory benannt.
- Kartenansicht kann die Positionsquelle erreichen.

## Verarbeitung

Die SDS-Logik dekodiert unterstützte LIP-Positionen, wenn der Binärinhalt eine plausible WGS84-Koordinate ergibt; unvollständige oder nicht unterstützte Formen bleiben als LIP-Meldung ohne gesicherte Koordinate sichtbar. Quelle, Empfangszeit und bekannte Qualitätsangaben beim Export erhalten. Alte Positionen dürfen nicht als aktueller Standort missverstanden werden; eine vollständige Auswertung aller LIP-Formen und Qualitätsfelder ist hier nicht zugesagt.

## Datenschutz

Positionsdaten sind betriebliche und potenziell personenbezogene Daten. Zugriff, Aufbewahrung und Export müssen zum Einsatzzweck passen. Eine aktivierte Karte ist kein Grund, Positionsdaten unbegrenzt zu speichern.

## Fehlersuche

- SDS-Log auf LIP-Nachricht prüfen.
- Endgeräteprofil und Zieladresse kontrollieren.
- GPS-Fix direkt am Funkgerät prüfen.
- Zeitstempel und Zeitzone vergleichen.
- Directory-/Control-Room-Export getrennt vom lokalen Empfang testen.

## Weiterführend

[Lokales Dashboard der Basisstation](dashboard-der-basisstation.md) · [MQTT, Home Assistant und Homematic](mqtt-home-assistant-und-homematic.md) · [Sicherheit und Betrieb](sicherheit-im-betrieb.md)

## Quellen zur Pflege dieser Seite

[Unterstützte LIP-Dekodierung und Negativtests](../../crates/tetra-entities/src/cmce/subentities/sds_bs.rs) · [Positionsanzeige](../../crates/tetra-entities/src/net_dashboard/ui/dashboard.html).
