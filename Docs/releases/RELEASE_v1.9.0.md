# NetCore-TETRA v1.9.0

Dieser Release führt die NINA/KATWARN-Warnzentrale und die Funk-/SDS-Korrekturen aus `katwarn/nina` mit dem aktuellen `main` zusammen. Er enthält außerdem die im laufenden Testbetrieb ergänzte Verwaltung eigener Meldungen, die korrigierten Asterisk-Dateirechte und die reparierte Cargo-Abhängigkeitsdatei.

## Warnzentrale

- Standortbasierte Warnungen aus NINA/KATWARN und eigene Kreiswarnungen mit Titel, Text, Warnstufe, Radius und Ablaufzeit.
- Individuelle SDS an angemeldete Funkgeräte mit verwendbarer GPS-Position im Warngebiet. Die dauerhafte Zuordnung von Warnung und Gerät verhindert Wiederholungen derselben Warnung auch nach einem Neustart.
- Geräteübersicht mit Standort, GPS-Alter, TBS-Verbindung, passendem Warngebiet und Zustellzustand. Fehlende oder veraltete Daten bleiben als Prüfbedarf sichtbar.
- Ein eigener Verwaltungsbereich hält selbst erstellte Meldungen nach dem Senden auffindbar. Suche und die Filter **Alle / Aktiv / Beendet** unterstützen das Wiederfinden.
- **Löschen** beendet weitere Zustellungen auch vor Ablauf. Bereits gesendete Nachrichten und die Zustellhistorie bleiben erhalten. Eine zusätzliche Entwarnungs-SDS wird nicht erzeugt.
- Ein veralteter Statusabruf kann neu erstellte oder gelöschte Meldungen nicht mehr kurzzeitig zurücksetzen.

## Funkbetrieb und Backend

- Ergänzte CMCE-Weiterleitung zentraler SDS- und Statusaufträge durch die Basisstation.
- Control-Room-Anbindung an die Geräte- und GPS-Telemetrie des Node Gateways.
- Dauerhafte Idempotenz und Rücknahme ausstehender Aufträge im SDS Router; Korrekturen für kurze SDS-/Status-Nutzdaten und für die Behandlung von Persistenz und langsamen Abhängigkeiten in SDS Router und Call Control.
- Korrekte normale Downlink-Bursts für obligatorische BNCH-Übertragungen auf belegten Kanälen. Die Änderung allein belegt keine vollständige Behebung aller möglichen Reregistrierungen oder Funkunterbrechungen.

## Installation und Betrieb

- Der SIP Switch erzeugt seine drei Asterisk-Includes ausdrücklich mit Modus `0640` und bei Ausführung als root mit der Gruppe `asterisk`, sofern diese vorhanden ist. Ein restriktives `umask 077` beim Update macht die Dateien dadurch nicht mehr für Asterisk unlesbar.
- Die Root-`Cargo.lock` enthält die fehlenden IoT-Gateway-Abhängigkeiten sowie die notwendigen Feature-Abhängigkeiten von `chrono` und `uuid`. Es werden dafür keine Paketversionen angehoben. Die relevanten CI-Builds verwenden `--locked`.
- Der neuere separate Brew-Server und die überarbeitete Dokumentation aus `main` bleiben erhalten. Eine bestehende Python-Brew-Installation wird durch diesen Release nicht automatisch auf den Rust-Brew-Server umgestellt.
- Die Warn- und Reparaturanleitungen verwenden jetzt `main` beziehungsweise diesen Release. Bestehende Checkouts aus der älteren Git-Historie können mit einem expliziten Fetch und einem detached Checkout umgeschaltet werden; lokale Änderungen müssen vorher gesichert werden.
- Die neu angelegte `main`-Historie wird fortgeführt. Ältere Commits bleiben über den Archivbranch und vorhandene Release-Tags erreichbar.

## Aktualisierung

Die [Installations- und Update-Anleitung je LXC/TBS](../deployment/KATWARN_NINA_INSTALL_UPDATE.md) beschreibt die Reihenfolge und die jeweiligen Hostrollen. Konfigurationen und Datenbanken erhalten, insbesondere die Warn- und SDS-Empfängerhistorie. Für einen reproduzierbaren Checkout:

```bash
git clone --branch v1.9.0 --single-branch https://github.com/JanHG98/netcore-tetra.git netcore-tetra-v1.9.0
```

Bei bereits funktionierender Warnzustellung genügt für die neue Meldungsverwaltung das Update des Warn-LXC und anschließendes Neuladen der WebUI. Eine laufende, manuell gestartete TBS wird weiterhin mit ihrem bisherigen Startbefehl und ihrer bestehenden Konfiguration betrieben.

## Prüfungen und Grenzen

Der Testbetrieb hat die Aussendung einer eigenen Warnmeldung und eine anschließende Empfangsmeldung des Funkgeräts bestätigt. Automatisierte Tests decken Warnzustellung, Persistenz, Rücknahme, Web-API, UI, zentrale SDS-/Statusweiterleitung, SIP-Dateirechte und den Funk-Scheduler ab. Die Linux-CI prüft außerdem die beteiligten Rust-Dienste und den Build der Basisstation.

„Von TBS angenommen“ im Warn-UI bestätigt weiterhin die Annahme zum Senden und ist keine Empfangs- oder Lesebestätigung. Bereits empfangene SDS lassen sich auf Funkgeräten nicht zurückholen. Vollständiges Handover laufender Gespräche zwischen Funkzellen und die Übertragung laufender SIP-Dialoge bei einer Fallback-Umschaltung sind weiterhin nicht enthalten. Die Backend-Dienste bleiben für den dokumentierten OPEN-LAB-Betrieb vorgesehen.
