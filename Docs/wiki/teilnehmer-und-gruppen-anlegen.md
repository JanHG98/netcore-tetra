# Provisioning und Datenhoheit

Provisioning Core bündelt die Pflege von Teilnehmern und Gruppen. Der [Dienst im Repository](../../system-backend/provisioning-core) ist **zusätzlich** zu den 26 Open-Lab-Inventory-Diensten vorhanden; der Beispiel-HTTP-Port ist `8125`. Für eine Einrichtung den geprüften Quellcommit und die realen Subscriber-/Group-Core-Adressen einsetzen. Der Deployment-Agent führt Provisioning Core als optionale Rolle; daraus folgt keine zusätzliche Inventoryrolle.

## Was wo liegt

| Datenart | Maßgebliche Stelle | Nicht gleichsetzen mit |
|---|---|---|
| Teilnehmerprofil, Freigabe, Gerätepolicy | Subscriber Core | Directory-Anzeigename oder nur registrierte ISSI |
| GSSI, Mitgliedschaft, DGNA-/Gruppenpolicy | Group Core | bloßer Directory-Gruppenname |
| aktuelle Registrierung und Serving-TBS | TBS + Mobility Core | dauerhaftes Teilnehmerprofil |
| lesbare Namen, Farben, lokale Status-/Fahrzeugmetadaten | NetCore Directory | Policy und RF-Affiliation |
| SIP-Serviceziele und PBX-Nummern | TBS-`[asterisk]`, lokaler Asterisk, SIP Switch/PBX | Teilnehmer als „Telefon“ im Provisioning Core |

## Teilnehmer anlegen und nachweisen

1. Eine eindeutige ISSI und Berechtigung im Subscriber Core über Provisioning pflegen.
2. GSSI und Mitgliedschaft im Group Core anlegen, soweit benötigt. Zentrale DGNA-/Policyaufträge sind noch nicht vollständig im MM-Dispatcher umgesetzt; [Z02.5 in der Gesamtroadmap](../roadmaps/gesamtroadmap.md#z025--zentrale-gruppenzuweisungen-auf-der-tbs-umsetzen) führt Umsetzung und Funkabnahme.
3. Lesbare Bezeichnung im Directory ergänzen; IMSI/ISSI/GSSI und SIP-Rufnummer nicht verwechseln.
4. TBS- und Gateway-Verbindungen prüfen, Gerät registrieren und Affiliation im **Laufzeitstand** nachweisen.
5. SDS, Gruppenruf, Einzelruf und Release mit genau diesem Gerät testen.

Ein grüner Datensatz in der Verwaltung sagt noch nichts über Funkempfang, Zeitbasis oder aktuelle Erreichbarkeit. [Registrierung und Gruppenbindung](registrierung-und-gruppenbindung.md) · [Funkgeräte im Directory verwalten](funkgeraete-im-directory.md) · [Gesprächsgruppen und ihre Anzeigenamen](gespraechsgruppen-im-directory.md)

## Installation

Der Dienst hat einen [eigenen LXC-Installer](../../system-backend/provisioning-core/install) und eine [Beispiel-TOML](../../system-backend/provisioning-core/config/provisioning-core.example.toml). Die Upstreams sind Subscriber Core `8100` und Group Core `8110` im Beispiel; lokale Installationswerte ersetzen. Die [LXC-Anleitung](../services/provisioning-core/installation-im-lxc.md) beschreibt den separaten Betrieb; ein vorhandener Agent kann die optionale Rolle ebenfalls verwalten. Installierten Build, Upstreams und Ready-Zustand prüfen. [Dienste und Pi-Images im Open Lab bereitstellen](dienste-und-pi-images-bereitstellen.md)

## Quellen zur Pflege dieser Seite

[Dienstvorlage](../../system-backend/provisioning-core/config/provisioning-core.example.toml) · [Agentenkatalog](../../system-backend/deployment-core/catalog.json) · [MM-Befehlsdispatcher](../../crates/tetra-entities/src/mm/mm_bs.rs).
