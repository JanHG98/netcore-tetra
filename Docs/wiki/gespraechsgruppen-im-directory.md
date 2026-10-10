# Gesprächsgruppen und ihre Anzeigenamen

Gruppen beschreiben TETRA-Gruppenadressen mit GSSI. Sie werden für Gruppenrufe, Audioaussendungen, Darstellung und organisatorische Zuordnung verwendet.

## Felder

| Feld | Bedeutung |
|---|---|
| `gssi` | Group Short Subscriber Identity |
| `name` | vollständige Gruppenbezeichnung |
| `short` | kurze Anzeige |
| `type` | fachliche Kategorie |
| `owner` | organisatorischer Eigentümer |
| `color` | Darstellungsfarbe |
| `visible` | Sichtbarkeit |
| `notes` | interne Hinweise |

## Funkseitige Bedeutung

Ein Directory-Eintrag allein affiliiert kein Funkgerät. Die Gruppenbindung entsteht durch Endgeräte-Signalisierung und lokale Ruflogik. Directory ergänzt Namen und Struktur.

## Aufzeichnung

Bei `recording.mode = "selected_groups"` müssen die aufzuzeichnenden GSSIs zusätzlich in der lokalen Basisstationskonfiguration stehen. Directory-Sichtbarkeit ersetzt diese Liste nicht.

## Benennungsempfehlung

- volle Bezeichnung für Bedienoberflächen
- kurze, eindeutige Abkürzung für kompakte Ansichten
- GSSI nicht aus dem Namen ableiten, sondern als eigenes Primärfeld pflegen

## Weiterführend

[Gerätegruppen und Statusgruppen](geraete-und-statusgruppen.md) · [Provisioning und Datenhoheit](teilnehmer-und-gruppen-anlegen.md) · [Gruppen- und Einzelrufe](gruppen-und-einzelrufe.md)

Zentrale Freigaben und Mitgliedschaften liegen im Group Core. Der vollständige zentrale Policy-/DGNA-Pfad zur MM-Instanz und zum Endgerät ist als Z02.5 noch offen. Ein gespeicherter Gruppenauftrag ist deshalb kein Nachweis einer übernommenen Endgerätezuweisung. [Gesamtroadmap](../roadmaps/gesamtroadmap.md)

## Quellen zur Pflege dieser Seite

[Directory-Gruppenschema](../../system-backend/directory/netcore-directory.py) · [Group-Core-Verarbeitung](../../system-backend/group-core/src/state.rs) · [MM-Gruppenverarbeitung](../../crates/tetra-entities/src/mm/mm_bs.rs).
