# Group Core

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/state.rs](../../../system-backend/group-core/src/state.rs) · [src/http.rs](../../../system-backend/group-core/src/http.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

## Zweck

Der Group Core ist der zentrale, eigenständig deploybare Dienst für GSSI-Stammdaten, Teilnehmermitgliedschaften, aktuelle Affiliationen und DGNA.

## Kernaufgaben

- Gruppenprofile und Dienstfreigaben verwalten
- feste und dynamische Mitgliedschaften verwalten
- automatische Gruppenanbindung definieren
- Gruppenrichtlinien versioniert an alle TBS verteilen
- aktuelle Affiliationen aus TBS-Telemetrie darstellen
- DGNA-Operationen auslösen und bis zur TBS-Antwort verfolgen
- Gruppenruf-, Mitgliedschafts- und Notrufzulassung lokal auf der TBS anwenden
- Mindestpriorität und Class of Usage zentral vorgeben

## WebUI

Die eigene WebUI läuft standardmäßig unter `http://<LXC-IP>:8110/` und bleibt unabhängig vom Control Room erreichbar.

Sie enthält Gruppen-, Mitgliedschafts-, Affiliation-, DGNA-, TBS-Sync- und Ereignisansichten.

## Open-Lab-Modus

Diese Ausbaustufe besitzt absichtlich keine Tokens, Passwörter, Benutzeranmeldung oder TLS. Sie darf nur in einem isolierten Testnetz betrieben werden.

## Datenhaltung

- `/var/lib/netcore-group-core/groups.json`
- `/var/lib/netcore-group-core/groups.json.bak`

## Abhängigkeiten

- Node Gateway auf `/ws/backend`
- kompatible TBS mit `group_policy`- und `dgna`-Capability
- Subscriber Core, Call Control und SDS Router sind vorhandene Nachbardienste, aber keine direkten HTTP-Abhängigkeiten dieses Dienstes; ihre Existenz bedeutet keine automatische gemeinsame Policy-Durchsetzung
