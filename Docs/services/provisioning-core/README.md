# Teilnehmer- und Gruppenverwaltung

**Quellen:** [system-backend/provisioning-core](../../../system-backend/provisioning-core) · [Repository-Root](../../..). Bei Befehlen das in der Anleitung angegebene Arbeitsverzeichnis beachten.

Stand: **9. Oktober 2026**. Die Anleitung beschreibt die im Hauptzweig vorhandene Umsetzung; Anlagen- und Funkabnahmen stehen in der [Gesamtroadmap](../../roadmaps/gesamtroadmap.md).

Der **Provisioning Core** ist die zentrale Verwaltungsoberfläche für Teilnehmer, Geräte, Gruppen und Gruppenmitgliedschaften.

Er ersetzt Subscriber Core und Group Core nicht als autoritative Dienste, sondern bündelt deren APIs in einer gemeinsamen WebUI:

- Geräte/ISSIs anlegen, bearbeiten, sperren, freigeben und löschen
- Gruppen/GSSIs anlegen, bearbeiten und löschen
- Gruppenruf, SDS, Notruf, Attach und DGNA je Gruppe freigeben
- Mitgliedschaften als Geräte-×-Gruppen-Matrix verwalten
- beide Cores gemeinsam auf alle verbundenen Basisstationen synchronisieren
- beim Löschen eines Gerätes oder einer Gruppe zugehörige Mitgliedschaften automatisch bereinigen

Der Provisioning Core ist eine optionale Verwaltungsoberfläche. Er ist als Quellkomponente und Deployment-Agentrolle vorhanden, gehört aber nicht zu den 26 regulären Inventardiensten der aktuellen Registry. Seine Installation und Upstream-Adressen separat planen.

Standardport: `8125/tcp`

Der Dienst ist für die aktuelle Testphase bewusst **OPEN LAB**: kein Token, keine Anmeldung und kein TLS. Nur im isolierten Verwaltungsnetz betreiben.

## WebUI-Layout

Die Verwaltungsoberfläche verwendet getrennte, intern scrollende Tabellenbereiche mit feststehenden Kopfzeilen. Die Mitgliedschaftsmatrix besitzt feste, kompakte Gruppenspalten, eine beim horizontalen Scrollen sichtbare Gerätespalte sowie getrennte Filter für Geräte und Gruppen. Dadurch bleiben große Bestände auf Desktop, Tablet und kleineren Displays bedienbar, ohne dass Tabellenköpfe Datenzeilen überdecken.

## Dokumentation

- [Vollständige Installation](../../deployment/historisch-provisioning-core-erstinstallation.md)
- [Kurze LXC-Übersicht](installation-im-lxc.md)
- [API-Beispiele](tests/api-beispiele.md)

## Abhängigkeiten

- schreibt Teilnehmerprofile über Subscriber Core (`8100`)
- schreibt Gruppen und Mitgliedschaften über Group Core (`8110`)
- hat keine direkte TBS-Verbindung und gehört nicht zu den kritischen Fallback-Diensten

## Einzelrufe und SIP-Ziele

Der Provisioning Core verwaltet keine Freigabelisten für Simplex oder Duplex. Jedes
registrierte Funkgerät darf beide Einzelrufarten anfordern; die gemeldete
`ClassOfMs`-Duplexfähigkeit dient nur der Anzeige/Telemetrie und ist keine
Berechtigung.

SIP-/Asterisk-Nummern werden ebenfalls nicht als Geräte angelegt. Sie werden über
den Wählplan der Basisstation (`[asterisk]`, Präfix und `service_numbers`) geroutet.
Mit `service_numbers = ["*"]` oder einer leeren Liste sind beliebig viele Ziele
hinter dem konfigurierten Präfix möglich.
