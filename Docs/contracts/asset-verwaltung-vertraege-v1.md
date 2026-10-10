# Asset Management Contracts v1

**Quellbezug:** gemeinsamer Vertrag und Schemas unter `system-backend/shared/`, Stand `main` (`c3ccdb4`, 09.10.2026). Phasenangaben sind Einführungshistorie; Ausführungsberechtigungen und optionale Felder richten sich nach dem konkreten Dienst.

Phase 10 führt drei transportneutrale Stammdatenverträge ein:

- `netcore-asset-v1`: physisches Gerät beziehungsweise Infrastruktur-Asset
- `netcore-person-v1`: Person und optionale RUI-Metadaten
- `netcore-assignment-v1`: zeitlich nachvollziehbare Ausgabe eines Assets

## Autoritative Grenzen

Asset Management ist **nicht** autoritativ für Teilnehmerfreigabe, Gruppen oder die aktuelle Serving-TBS. Diese Zustände werden lesend aus Subscriber Core und Mobility Core gespiegelt.

RUI/RUA wird noch nicht ausgeführt. PINs sind ausdrücklich kein Feld des Vertrages; `pin_stored` muss immer `false` sein.
