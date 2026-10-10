# Sicherheit und Betrieb

## Netzwerkgrenzen

Die aktuelle Dienstregistry unterscheidet die folgenden Zugänge:

| Oberfläche / Dienst | Aktuelles Beispiel | Konsequenz |
|---|---|---|
| Backend-Fachdienste und Deployment Core | `open_lab`, HTTP ohne Management-Token oder TLS | erreichbare Clients können Verwaltungsaktionen auslösen; isoliertes Managementnetz |
| Control Room | Rollen/Profile vorhanden, Beispielauthentisierung deaktiviert | Rollenbestand ist kein Nachweis eines verpflichtenden Logins |
| Alert Service | `security_mode = "token"`, Verwaltung tokenpflichtig, kein TLS im Beispiel | Token vertraulich halten und Transport-/Netzgrenze separat schützen |
| Lokales TBS-Dashboard | optional Benutzername/Passwort, Loginformular und Cookie-Sitzung | ohne beide Werte offen; öffentliche Übersicht nur eingeschränkte Anzeige |
| Directory | direkte Python-HTTP-API ohne eigene Anmeldung im Handler | Schreib- und Import-API über Netzgrenze schützen |

Ein abgesicherter Reverse Proxy kann kontrollierten Zugriff bereitstellen, wenn die direkten Listener weiterhin geschützt sind. Allein die Proxy-Anmeldung schützt keinen aus anderen Netzen erreichbaren Direktport. Installierte TOMLs, Units, Firewall und `ss` gemeinsam prüfen. [Bereitstellung](dienste-und-pi-images-bereitstellen.md) · [Bedienoberflächen](bedienoberflaechen-und-zustaendigkeiten.md)

## Geheimnisse

Nicht in Git, Wiki, Screenshots oder Support-Logs veröffentlichen:

- Passwörter
- Tokens
- Bot-Schlüssel
- SIP-/Brew-Zugangsdaten
- interne Teilnehmerlisten
- private IP-Pläne, wenn sie nicht für die Fehlersuche notwendig sind

## Minimalrechte

- eigener Systembenutzer pro Dienst, soweit praktikabel
- Konfiguration `0600`
- Schreibrechte nur auf benötigte Verzeichnisse
- keine globale Schreibbarkeit von NFS- oder Medienpfaden
- Dashboard-Systemaktionen gezielt absichern

## Funkbetrieb

- nur zulässige Frequenzen und Leistungen verwenden
- Lasttests kontrolliert durchführen
- Notfall- und Systemstatus nicht mit realen Einsatznetzen vermischen
- Simulationsteilnehmer und produktive ISSIs trennen
- bei Änderungen an Antennen, Filtern oder Verstärkern Spektrum erneut prüfen

## Protokollierung

Logs können ISSIs, Gruppen, Texte und Standorte enthalten. Aufbewahrung und Weitergabe müssen zum Testzweck passen. Bei Support-Auszügen sensible Werte schwärzen, aber Zeitstempel und technische Fehlermeldungen erhalten.

## Notfalllogik

Notfallereignisse bleiben standardmäßig lokal. Externe Weiterleitung an Telegram, Brew oder Leitstelle nur aktivieren, wenn Empfänger, Eskalation und Rücknahme eindeutig definiert sind.

## Schlüssel und Aktoren

Security Core und KMF haben getrennte Zuständigkeiten für Policy/Authentisierung und Schlüssel-Lebenszyklus. Rohschlüssel gehören weder in UI-Screenshots noch in Logs oder Git. Bei Ausfall des KMF dürfen installierte Schlüssel lokal weitergelten, neue Rotation/OTAR ist damit nicht automatisch verfügbar. Physische Ausgänge und HA-/Homematic-Aktionen bleiben im Open-Lab-Beispiel gesondert freizugeben; `default_deny` und Zielbegrenzung nicht für bequeme Tests aushebeln. [MQTT, Home Assistant und Homematic](mqtt-home-assistant-und-homematic.md) · [Dienstkatalog](dienstkatalog.md)

## Quellen zur Pflege dieser Seite

[Dienstregistry mit Auth/TLS](../../system-backend/services.toml) · [Directory-HTTP-Handler](../../system-backend/directory/netcore-directory.py) · [TBS-Sitzungsanmeldung](../../crates/tetra-entities/src/net_dashboard/server.rs).
