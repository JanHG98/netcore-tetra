# Provisioning und Datenhoheit

Provisioning Core bündelt die Pflege von Teilnehmern und Gruppen. Der [Dienst im Repository](https://github.com/JanHG98/netcore-tetra/tree/main/system-backend/provisioning-core) ist **zusätzlich** zu den 24 Open-Lab-Inventory-Diensten vorhanden; der Beispiel-HTTP-Port ist `8125`. Seine Vorlagen enthalten teils historische Branch- oder IP-Beispiele. Für eine Einrichtung die **aktuelle `main`-Version** und reale Subscriber-/Group-Core-Adressen einsetzen.

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
2. GSSI und Mitgliedschaft im Group Core anlegen, soweit benötigt.
3. Lesbare Bezeichnung im Directory ergänzen; IMSI/ISSI/GSSI und SIP-Rufnummer nicht verwechseln.
4. TBS- und Gateway-Verbindungen prüfen, Gerät registrieren und Affiliation im **Laufzeitstand** nachweisen.
5. SDS, Gruppenruf, Einzelruf und Release mit genau diesem Gerät testen.

Ein grüner Datensatz in der Verwaltung sagt noch nichts über Funkempfang, Zeitbasis oder aktuelle Erreichbarkeit. [[Registration-and-Affiliation]] · [[Devices]] · [[Groups]]

## Installation

Der Dienst hat einen [eigenen LXC-Installer](https://github.com/JanHG98/netcore-tetra/tree/main/system-backend/provisioning-core/install) und eine [Beispiel-TOML](https://github.com/JanHG98/netcore-tetra/blob/main/system-backend/provisioning-core/config/provisioning-core.example.toml). Die Upstreams sind Subscriber Core `8100` und Group Core `8110` im Beispiel; lokale Installationswerte ersetzen. Die ältere [LXC-Anleitung](https://github.com/JanHG98/netcore-tetra/blob/main/system-backend/provisioning-core/docs/lxc-deployment.md) referenziert einen früheren Feature-Branch: Build-Pfad und Branch vor Ausführung gegen `main` prüfen. [[Open-Lab-Deployment]]
