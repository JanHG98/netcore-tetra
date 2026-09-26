# Betrieb, Wartung und Reparatur

Diese Seite ist die kurze Betriebskarte. Die tatsächliche Anlage kann andere Units, Pfade und Retentionsregeln haben; `inventory.toml`, `/etc/netcore/` und `systemctl cat <unit>` sind vor einem Eingriff maßgeblich.

## Tägliche Sichtung

- TBS: Carrier/Slots, RF-Timing, registrierte Endgeräte, aktive Rufe, Queue, Health, freien Speicher und Temperatur prüfen.
- Backend: `/health/live` **und** `/health/ready`, Node-Gateway-Service-Matrix, Broker-/SIP-Verbindungen und Abhängigkeiten vergleichen. Ein laufender Prozess mit `ready=503` kann fachlich nicht bereit sein.
- Daten: SDS-/MQTT-Acks, Directory-Lookups, Aufzeichnungen, TTS-Cache und Archiv-Retry beobachten.
- Abweichungen mit Zeitstempel, Commit, lokalem Konfigurationsstand und Umfang dokumentieren.

## Periodische Arbeiten

| Intervall nach Betriebsplan | Handlung |
|---|---|
| wöchentlich | Log-/Speicherwachstum, Aufbewahrung, NFS-Nachläufe, MQTT-Outbox und Fehlerqueue prüfen |
| vor jeder Änderung | Backup und Rückweg verifizieren; Ports, Dependencies und RF-Parameter vergleichen |
| nach jedem Update | Register/Call/SDS/Release und betroffene Integration einmal vollständig testen |
| regelmäßig nach Standortvorgabe | Antennen-/Koax-/Duplexerzustand, Erdung, Versorgung, Lüftung und mechanische Verbindungen prüfen |

## Wartungsfenster und Update

1. Aktive Rufe beenden, Testumfang und betroffene Dienste festlegen.
2. Daten/Config/Units sichern, installierten Commit und lokale Abweichungen notieren. [[Backup-and-Fallback]]
3. Bei LXC-Änderungen Inventory `validate`, `plan`, `render`, `apply --dry-run`; bei TBS-Änderungen Build und Startparameter prüfen. [[Open-Lab-Deployment]] · [[Build-and-Update]]
4. Von fachlichen Abhängigkeiten zum abhängigen Dienst aktualisieren. SIP/Medienänderungen brauchen abgestimmte TBS-, Call-Control- und Media-Switch-Versionen.
5. Health, Readiness, echten Testablauf und Ausfallrückkehr prüfen. [[Abnahme]]

## Reparaturpfad

Bei RF-Störungen nicht zuerst an einer Statusanzeige „reparieren“: TX- und RX-Pfad, Kabel, Duplexer und SDR getrennt messen. Bei Softwarefehlern zuerst die konkrete Fehlermeldung, installierte TOML, Unit und jüngste Änderung lesen. Defekte Datenträger/SD-Karten aus einem **getesteten** Image plus gesichertem Konfigurations-/Schlüsselstand wiederherstellen; der gewünschte automatische Imagebuilder ist noch ein Ausbauziel. [[Hardware-und-RF]] · [[Troubleshooting]] · [[Roadmap]]

Konfigurations- und Datenbankmigrationen sind keine reinen Datei-Rollbacks. Den letzten kompatiblen Software- und Datenstand gemeinsam wiederherstellen und danach Funkbetrieb sowie die Fach-APIs neu abnehmen.
