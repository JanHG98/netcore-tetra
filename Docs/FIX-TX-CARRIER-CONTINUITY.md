# TX-Carrier-Zuordnung und gemeinsame Blockverarbeitung

## Fehler und Korrektur

Der Soapy-Sendepfad hat bisher Modulatoren und eingehende Bursts nach ihrer
Listenposition verbunden. Wenn ein Carrier fehlte, konnte dessen Nachbar auf
der falschen Frequenz moduliert werden. Ein nicht berücksichtigter Modulator
behielt zudem alte Filter- und Puffersamples.

Bei 600 kSamples/s verarbeitet der Sendepfad 108 Modem-Samples pro SDR-Block.
Ein TDMA-Zeitschlitz umfasst 1020 Modem-Samples. Die meisten Slotgrenzen liegen
deshalb innerhalb eines Blocks. Bisher kehrte die Schleife beim ersten Carrier,
der neue Slotdaten benötigte, sofort zurück. Spätere Carrier konnten dabei die
verbleibenden Samples des alten Slots verlieren. Bereits beigemischte Carrier
konnten bei einem erneuten Aufruf doppelt im selben Ausgabeblock landen.

Die Korrektur:

- Ordnet Bursts anhand ihrer Carrier-Nummer dem passenden Modulator zu.
- Verarbeitet fehlende Carrier als zeitlich begrenzte Stille; PHY-Drops behalten
  Carrier-Nummer und Slotzeit mit `slot: None`.
- Füllt zuerst die Eingabepuffer aller Carrier. Erst wenn alle vollständig sind,
  werden sie genau einmal in den gemeinsamen Ausgabeblock gemischt.
- Verwirft veraltete Teilblöcke und Filterhistorie, wenn die SDR-Zeit Blöcke
  übersprungen hat. Alte Samples erhalten keinen neuen Zeitstempel.

## Aussage zum REREG

Dies ist ein nachweisbarer Fehler im gemeinsamen Funk-Sendepfad. Er kann den
Zweiträgerbetrieb beschädigen. Der Referenzlauf mit v1.7.0 vom 13.09.2026 zeigt
REREGs allerdings auch mit einem Carrier und ohne die optionalen NetCore-
Backend-Integrationen. Die Mehrträgerfehler erklären diesen Lauf allein nicht.
Ein erfolgreicher Funkversuch bleibt erforderlich, bevor REREG als behoben gilt.
Aus diesem Befund folgt keine pauschale Änderung der Netzwerkdienste.

## Prüfung ohne Funkhardware

```bash
cargo test -p tetra-entities --lib phy::components::soapy_dev::tests:: -- --nocapture
cargo test -p tetra-entities --lib phy::components::slotter::tests:: -- --nocapture
```

Die fünf TX-Tests prüfen IQ-Wellenformen: gemeinsame Ausgabe gegen die Summe
einzeln berechneter Carrier, wechselnde Reihenfolge, pausierende Carrier,
wiederholtes Polling und Neustart der Modulation nach einem übersprungenen Block.
Sie öffnen kein SDR-Gerät. `--lib` begrenzt den Lauf auf die Bibliothekstests.
Der GitHub-Workflow `PHY downlink regression tests` führt diese Tests und einen
Release-Build der Basisstation aus.

## Auf der manuell gestarteten Basisstation nach dem Merge in mqtt

Den laufenden Basisstationsprozess zuerst im bisherigen Terminal mit Strg+C
beenden. Danach auf `SRV-M-TBS-01`:

```bash
(
set -euo pipefail
umask 077
cd /opt/netcore-tetra
if pgrep -x bluestation-bs >/dev/null; then
    echo 'Die Basisstation läuft noch. Zuerst im bisherigen Terminal beenden.'
    exit 1
fi
git switch mqtt
git pull --ff-only origin mqtt
cargo test -p tetra-entities --lib phy::components::soapy_dev::tests:: -- --nocapture
cargo build --release -p bluestation-bs
test -s ./config.toml
install -d -m 700 /opt/netcore-rereg-tests
REREG_LOG="/opt/netcore-rereg-tests/mqtt-tx-$(date +%Y%m%d-%H%M%S).log"
git log -1 --format='%h %s'
echo "Logdatei: $REREG_LOG"
./target/release/bluestation-bs ./config.toml 2>&1 | tee "$REREG_LOG"
)
```

Dieser Aufruf baut und startet den aktuellen MQTT-Stand mit der vorhandenen
Konfiguration. Er verwendet weder die alte v1.7.0-Referenzbinary noch eine
systemd-Unit. Lokale Änderungen werden nicht zurückgesetzt. Bei einem Pull- oder
Buildfehler bricht der Block vor dem Start ab.

Für die RF-Prüfung mindestens fünf Minuten laufen lassen, Anmeldung und REREG-
Zeitpunkte beobachten und das vollständige Log aufbewahren. Ein erfolgreicher
Build und eine bestätigte Location-Update-Antwort belegen allein keine dauerhaft
fehlerfreie Übertragung des Kontrollkanals.
