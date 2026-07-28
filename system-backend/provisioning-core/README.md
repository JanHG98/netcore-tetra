# NetCore Provisioning Core

Der **Provisioning Core** ist die zentrale Verwaltungsoberfläche für Teilnehmer, Geräte, Gruppen und Gruppenmitgliedschaften.

Er ersetzt Subscriber Core und Group Core nicht als autoritative Dienste, sondern bündelt deren APIs in einer gemeinsamen WebUI:

- Geräte/ISSIs anlegen, bearbeiten, sperren, freigeben und löschen
- Gruppen/GSSIs anlegen, bearbeiten und löschen
- Gruppenruf, SDS, Notruf, Attach und DGNA je Gruppe freigeben
- Mitgliedschaften als Geräte-×-Gruppen-Matrix verwalten
- beide Cores gemeinsam auf alle verbundenen Basisstationen synchronisieren
- beim Löschen eines Gerätes oder einer Gruppe zugehörige Mitgliedschaften automatisch bereinigen

Standardport: `8125/tcp`

Der Dienst ist für die aktuelle Testphase bewusst **OPEN LAB**: kein Token, keine Anmeldung und kein TLS. Nur im isolierten Verwaltungsnetz betreiben.

## WebUI-Layout

Die Verwaltungsoberfläche verwendet getrennte, intern scrollende Tabellenbereiche mit feststehenden Kopfzeilen. Die Mitgliedschaftsmatrix besitzt feste, kompakte Gruppenspalten, eine beim horizontalen Scrollen sichtbare Gerätespalte sowie getrennte Filter für Geräte und Gruppen. Dadurch bleiben große Bestände auf Desktop, Tablet und kleineren Displays bedienbar, ohne dass Tabellenköpfe Datenzeilen überdecken.

## Dokumentation

- vollständige Installation: `Docs/PROVISIONING_CORE_COMPLETE_INSTALL.md`
- kurze LXC-Übersicht: `docs/lxc-deployment.md`

## Abhängigkeiten

- schreibt Teilnehmerprofile über Subscriber Core (`8100`)
- schreibt Gruppen und Mitgliedschaften über Group Core (`8110`)
- hat keine direkte TBS-Verbindung und gehört nicht zu den kritischen Fallback-Diensten
