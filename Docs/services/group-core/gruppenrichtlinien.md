# Gruppenrichtlinie

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/state.rs](../../../system-backend/group-core/src/state.rs) · [src/http.rs](../../../system-backend/group-core/src/http.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

Die Richtlinie wird versioniert über den Node Gateway an jede kompatible TBS übertragen.

Sie enthält:

- GSSI und Aktivierungszustand
- Freigaben für Affiliation, DGNA, Gruppenruf, Gruppen-SDS und Notruf
- Mindestpriorität für Gruppenrufe
- Class of Usage für zentral ausgelöste DGNA-Anhänge
- Teilnehmermitgliedschaften
- Auto-Attach-Markierungen
- optionale TBS-Bereiche pro Gruppe

## Lokale TBS-Durchsetzung

Nach erfolgreicher Synchronisation prüft die TBS neue Gruppenaffiliationen und DGNA lokal. Bei aktivierter Mitgliedschaftsdurchsetzung darf ein Teilnehmer einen Gruppenruf nur starten, wenn er lokal zu dieser Gruppe affiliiert ist. Gruppenruf- und Notruffreigabe sowie die konfigurierte Mindestpriorität werden in CMCE geprüft.

Bei einer neuen Policy können bestehende Affiliationen bereinigt und Auto-Attach-Mitgliedschaften per DGNA gesetzt werden. Ohne zentrale Policy bleibt das bisherige lokale Gruppenverhalten bestehen.

## Aktuelle Grenze

`sds_allowed` wird bereits als Teil der Richtlinie verteilt. Der SDS Router ist inzwischen implementiert, liest aber in diesem Quellstand keine Group-Core-Gruppenrichtlinie. Aus dem Feld allein folgt daher keine verbindliche netzweite Sperre für Gruppen-SDS. Die Integration der gemeinsamen Gruppenrechte in den SDS-Pfad bleibt offen.
