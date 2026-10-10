# Lokales Dashboard der Basisstation

Das Web-Dashboard ist die lokale Bedien- und Diagnoseoberfläche der Basisstation. Die aktuelle Oberfläche ist vollständig deutschsprachig; eine Sprachauswahl wird nicht angezeigt.

## Reguläre Navigation

### Funkbetrieb

- **Funkgeräte** – registrierte Endgeräte, Namen, Status und Aktionen
- **Rufe** – aktive und vergangene Rufzustände
- **Zuletzt gehört** – letzte RF-/Signalisierungsaktivität
- **RF** – Carrier, Timeslots und Funkparameter
- **Health** – zusammengefasster Zustand der Komponenten
- **Log** – Laufzeitmeldungen
- **SDS-Log** – Text-, Status- und Datenmeldungen
- **Karte** – bekannte Positionsdaten

### Integrationen

Je nach Build und Konfiguration:

- Asterisk SIP
- Audio-Zentrale
- Telegram
- WLAN-Verwaltung, wenn NetworkManager verfügbar ist

Labor- und Sonderintegrationen wie DAPNET, EchoLink, MeshCom und GeoAlarm sind in der normalen Navigation und Health-Ansicht bewusst nicht sichtbar. Der technische Code bleibt für interne Tests erhalten.

### Administration

- **Konfiguration** – TOML bearbeiten und sichern
- **System** – Dienstzustand, Neustart, Update und Diagnose

## Anmeldung

In `[dashboard]` können Benutzername und Passwort gesetzt werden. Die Anmeldung erfolgt über eine Cookie-Sitzung. Ohne Authentifizierung ist das Dashboard vollständig offen und sollte dann nur in einem isolierten Managementnetz gebunden werden.

```toml
[dashboard]
bind = "0.0.0.0"
port = 8080
username = "<BENUTZER>"
password = "<LANGES-PASSWORT>"
public_overview = false
```

Mit `public_overview = true` kann trotz aktivierter Anmeldung eine eingeschränkte öffentliche Übersicht bereitgestellt werden. Das ersetzt keine Netzwerksegmentierung.

## Konfigurationseditor

Vor dem Überschreiben der Konfiguration wird eine `.bak`-Sicherung erzeugt. Nach strukturellen Änderungen ist normalerweise ein Neustart erforderlich. RF-relevante Werte sollten nicht während laufender Gespräche geändert werden.

## Systemaktionen

Mögliche Aktionen umfassen:

- Basisstationsdienst neu starten
- System neu starten oder herunterfahren, sofern zugelassen
- Update aus dem Quellverzeichnis anstoßen
- Dual Carrier umschalten
- Diagnose- und Zustandsdaten abrufen

Diese Funktionen sind administrativ. Dashboard-Port nicht direkt ins Internet weiterleiten.

## Directory-Anbindung

Das Dashboard lädt Namen, Kurzbezeichnungen, Farben, Icons, Gruppen und Statuslabels aus NetCore Directory. Bei Ausfall werden numerische IDs weiterhin angezeigt; je nach Funktion kann ein gecachter Stand sichtbar bleiben.

## Audio-Zentrale

Die Audio-Zentrale vereint:

- Sprachaufzeichnungen
- lokale Medienbibliothek
- NFS-Dateibrowser
- Vorschau
- Aussendung an Gruppe oder Einzelgerät
- TTS-Erzeugung und Vorlagen

Details stehen unter [Audio-Zentrale](audio-aufnahmen-und-tts.md).

## Browser-Cache

Nach einem Dashboard-Update kann alter JavaScript-/CSS-Code im Browser verbleiben. Dann hart neu laden:

- Windows/Linux: `Ctrl` + `F5`
- alternativ Cache für die Dashboard-Seite löschen
- bei installierter Web-App zusätzlich Service-Worker prüfen

Das lokale TBS-Dashboard und die 26 Fach-WebUIs sind eigenständige Oberflächen mit unterschiedlichen Authentisierungs- und Netzgrenzen. [Bedienoberflächen und Rollen](bedienoberflaechen-und-zustaendigkeiten.md) · [Sicherheit und Betrieb](sicherheit-im-betrieb.md)

## Update-Quelle und Dienstsuche

Wenn die laufende Binärdatei außerhalb des Git-Checkouts liegt, `[dashboard].source_dir` ausdrücklich setzen. Der eingeblendete Discovery-Verweis öffnet den vorhandenen Controller beziehungsweise Agenten; er verändert nicht automatisch den lokalen RF- oder Rufpfad. Die dortige Open-Lab-Oberfläche besitzt ihre eigene Netzgrenze.

## Quellen zur Pflege dieser Seite

[HTTP-Server und Cookie-Sitzung](../../crates/tetra-entities/src/net_dashboard/server.rs) · [Aktuelle Navigation](../../crates/tetra-entities/src/net_dashboard/ui/dashboard.html).
