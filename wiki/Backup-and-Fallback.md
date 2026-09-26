# Backup und Fallback

## Primär- und Fallback-Konfiguration

Kann die angegebene Primärkonfiguration nicht geladen werden, versucht die Basisstation automatisch eine Datei mit angehängtem `.fallback`:

```text
config.toml
config.toml.fallback
```

Wird Fallback verwendet, zeigt das Dashboard dauerhaft eine rote Warnung. Das System kann damit weiterlaufen, aber die Ursache der fehlerhaften Primärdatei muss behoben werden.

## Dashboard-Sicherung

Vor dem Überschreiben durch den Konfigurationseditor wird zusätzlich eine `.bak`-Datei erzeugt. Diese ist eine kurzfristige Rückfallebene, ersetzt aber kein externes Backup.

## Zu sichernde Daten

- `config.toml`
- `config.toml.fallback`
- Konfigurations-`.bak`
- Directory-SQLite-Datenbank
- Directory-Exportdatei
- TTS-Vorlagen
- lokale Medienbibliothek
- Recovery-Cache
- Systemd-Units und Environment-Dateien
- bei Bedarf Aufzeichnungen und JSON-Metadaten
- im verteilten Aufbau die Persistenzdaten und Konfigurationen aller betroffenen Backend-LXCs
- IoT-Command-Ledger/Outbox, Ruf- und Teilnehmerzustände nach den Dienst-Runbooks
- SIP-/Asterisk-Konfiguration und Trunk-Zugangsdaten, getrennt vom Wiki
- Security-/KMF-Schlüsselmaterial ausschließlich mit dem dafür vorgesehenen geschützten Verfahren

## Backup-Beispiel

```bash
sudo systemctl stop tetra.service
sudo tar -C / -czf /var/backups/netcore-$(date +%Y%m%d-%H%M%S).tar.gz \
  etc/netcore \
  var/lib/netcore \
  var/lib/netcore-directory
sudo systemctl start tetra.service
```

Pfade an die reale Installation anpassen. Große Aufnahmeverzeichnisse gegebenenfalls separat sichern.

## Wiederherstellung

1. Dienst stoppen.
2. beschädigte Dateien separat wegkopieren.
3. Backup entpacken oder einzelne Dateien wiederherstellen.
4. Eigentümer und Dateirechte prüfen.
5. Basisstation manuell mit der wiederhergestellten Konfiguration starten.
6. erst danach Systemd wieder aktivieren.

## Nach einem Fallback-Start

```bash
sudo journalctl -u tetra.service -b --no-pager | grep -iE 'config|fallback|parse|error'
diff -u /etc/netcore/config.toml.fallback /etc/netcore/config.toml
```

Nicht die Fallback-Datei blind über die Primärdatei kopieren. Sie kann bewusst konservativ oder älter sein.

## Zentrale Ausfallgrenzen

Der lokale RF-Dienst kann bei einem Backend-Ausfall gemäß `[edge_fallback]` weiterlaufen. Das bedeutet **nicht**, dass netzweite Rufe, neue Schlüssel, SIP-Routing oder alle SDS-Zustellungen verfügbar bleiben. Der Zustand der Service-Matrix hat eine Lease; nach Wiederkehr Spool, Acks, Policies und tatsächliche Endgerätefunktion prüfen. [[Mehrzellenbetrieb]] · [[Betrieb-und-Wartung]]

Eine Datei-Kopie einer SQLite-/Zustandsdatenbank während laufender Schreibzugriffe ist nicht automatisch konsistent. Für jeden Dienst die zugehörige Backup- oder Exportfunktion, Dienststopp beziehungsweise transaktionale Sicherung verwenden. Rücksicherung mit passender Software- und Schema-Version erproben.
