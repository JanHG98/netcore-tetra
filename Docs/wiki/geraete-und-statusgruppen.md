# Gerätegruppen und Statusgruppen

Gerätegruppen fassen Geräte organisatorisch zusammen. Mit aktivem `status_sync` dienen sie zusätzlich als Statusgruppen: Ein U-STATUS eines Mitglieds kann an die übrigen Mitglieder verteilt werden.

## Felder

| Feld | Bedeutung |
|---|---|
| `group_id` | interne eindeutige Gruppen-ID |
| `opta` | optionale taktische/organisatorische Kennung |
| `name` / `short` | Bezeichnungen |
| `type` | Gruppe oder Fahrzeugtyp |
| `owner` | organisatorische Zuordnung |
| `color` | Darstellungsfarbe |
| `status_sync` | Statusverteilung aktiv |
| `visible` | Sichtbarkeit |
| `notes` | interne Hinweise |

Mitglieder werden in einer eigenen Zuordnungstabelle gehalten.

## Statusverteilung im lokalen TBS-Pfad

1. Ein Mitglied sendet U-STATUS.
2. Die Basisstation ordnet den Status über Directory einem Label zu.
3. Sie ermittelt die Statusgruppen des Absenders.
4. Der Status wird an weitere Mitglieder weitergegeben.
5. Der letzte Status wird zwischengespeichert.
6. Ein später wieder registriertes Mitglied erhält den aktuellen Stand erneut.

## Live-Aktualisierung

Die Mitgliedschaften werden periodisch aus Directory aktualisiert. Änderungen können damit ohne kompletten Basisstationsneustart greifen. Trotzdem sollte nach größeren Umbauten ein Funktionstest mit allen betroffenen Geräten erfolgen.

## Grenzen

- Nicht erreichbare Mitglieder können erst nach erneuter Registrierung versorgt werden.
- Die Statusgruppe ersetzt keine Sprachgruppe und keine GSSI-Affiliation.
- Falsche oder zyklische Zuordnungen können unnötigen SDS-Verkehr erzeugen.

## Weiterführend

[Statusmeldungen](statusmeldungen-im-directory.md) · [SDS und U-STATUS](kurznachrichten-und-status.md) · [Namen und Metadaten im NetCore Directory](namen-und-metadaten-im-directory.md)

`group_id` ist die interne Directory-ID und keine GSSI. Eine geänderte Mitgliedschaft beeinflusst Statusverteilung; sie programmiert weder Sprachgruppen im Funkgerät noch Teilnehmerrechte im Group Core.

## Quellen zur Pflege dieser Seite

[Mitgliedschaft und Alias-Routen](../../system-backend/directory/netcore-directory.py) · [Statusverteilung und Replay](../../crates/tetra-entities/src/cmce/subentities/sds_bs.rs).
