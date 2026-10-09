# Asset Management Contracts v1

Phase 10 führt drei transportneutrale Stammdatenverträge ein:

- `netcore-asset-v1`: physisches Gerät beziehungsweise Infrastruktur-Asset
- `netcore-person-v1`: Person und optionale RUI-Metadaten
- `netcore-assignment-v1`: zeitlich nachvollziehbare Ausgabe eines Assets

## Autoritative Grenzen

Asset Management ist **nicht** autoritativ für Teilnehmerfreigabe, Gruppen oder die aktuelle Serving-TBS. Diese Zustände werden lesend aus Subscriber Core und Mobility Core gespiegelt.

RUI/RUA wird noch nicht ausgeführt. PINs sind ausdrücklich kein Feld des Vertrages; `pin_stored` muss immer `false` sein.
