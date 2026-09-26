# Sicherheit und Betrieb

## Netzwerkgrenzen

Dashboard, Directory, Piper und Control Room sind Verwaltungsdienste. Ihre direkten Listener gehören in ein isoliertes Managementnetz. Für kontrollierten externen Zugriff kann ein abgesicherter Reverse Proxy hinzukommen; die Direktports müssen weiter geschützt bleiben. Ein bloßes Portforwarding ist keine sinnvolle Standardlösung.

Das aktuelle `open_lab`-Inventory bindet zahlreiche Fach-WebUIs ohne Login, Management-Token oder TLS. Hier reicht ein Reverse Proxy **allein** nicht, wenn die direkten LXC-Ports aus anderen Netzen erreichbar bleiben. Alle Management-Endpunkte im isolierten Testnetz halten und die konkrete Exposition mit Firewall/`ss` prüfen. Diese Vorlage ist nicht für ungeschützte produktive Netze geeignet. [[Open-Lab-Deployment]]

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

Security Core und KMF haben getrennte Zuständigkeiten für Policy/Authentisierung und Schlüssel-Lebenszyklus. Rohschlüssel gehören weder in UI-Screenshots noch in Logs oder Git. Bei Ausfall des KMF dürfen installierte Schlüssel lokal weitergelten, neue Rotation/OTAR ist damit nicht automatisch verfügbar. Physische Ausgänge und HA-/Homematic-Aktionen bleiben im Open-Lab-Beispiel gesondert freizugeben; `default_deny` und Zielbegrenzung nicht für bequeme Tests aushebeln. [[MQTT-und-Home-Assistant]] · [[Dienstkatalog]]
