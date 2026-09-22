# Sepura-REREG: obligatorischen BNCH-Burst korrigieren

## Befund aus dem Funkversuch

Der Lauf mit `v1.3.0-b92888e1` auf `SRV-M-TBS-01` zeigt sichtbare
REREGs des Sepura (ISSI 5102) und Unterbrechungen des Funkbetriebs.
Die Basisstation nutzt den SXceiver mit zwei Carriern (720/721),
600 kSamples/s und SoapySX `9705147`.

- Die 15 Software-TX-Skip-Ereignisse mit insgesamt 89 übersprungenen
  Blöcken entstehen beim Start um 20:49:11, vor der ersten Anmeldung.
  Diese kumulativen Zähler steigen während der späteren REREGs nicht weiter.
- Die erste Anmeldung um 20:49:37 führt zum einmaligen Gruppenreport-Command.
  Der anschließende Demand und der Gruppenbericht werden bestätigt.
- Spätere Roaming-Anmeldungen kommen vom Funkgerät: 20:50:12.852,
  20:50:29.172, 20:51:12.976, 20:51:28.503, 20:52:09.644 und
  20:52:28.684. Die Antworten der Basisstation werden jeweils nach etwa
  110 ms quittiert. Das belegt die erfolgreiche erneute Anmeldung,
  nicht die Ursache der vorherigen Unterbrechung.
- Zellkennung und Location Area bleiben gleich. Die Hyperframe-Nummer
  läuft weiter; die beiden SYSINFO-Optionen wechseln planmäßig.
- `hw_status_supported=false` bedeutet fehlende Hardware-Statusmeldungen.
  Nullwerte in diesen Zählern schließen Fehler auf dem tatsächlichen Funkweg
  nicht aus. Der Log belegt aber keine neuen hostseitigen TX-Verspätungen.

## Nachgewiesene Abweichung und Korrektur

Die bisherige Default-Belegung erzeugte in jedem freien Zeitschlitz von
Rahmen 18 `BSCH + SYSINFO`. LMAC machte daraus einen Synchronisationsburst.
Das erfasste auch den obligatorischen BNCH-Zeitschlitz auf dem Kontrollkanal.

[ETSI EN 300 392-2 V3.8.1](https://www.etsi.org/deliver/etsi_en/300300_300399/30039202/03.08.01_60/en_30039202v030801p.pdf),
Tabelle 9.27 und Abschnitt 9.5.3/Tabelle 9.29, unterscheidet diese Positionen:
Der obligatorische BSCH verwendet einen Synchronisationsburst; der
obligatorische BNCH auf Kontroll- und Verkehrskanälen gehört in den zweiten
Block eines normalen Downlink-Bursts. Auf dem Hauptkontrollkanal TS1 liegt
der obligatorische BNCH in Rahmen 18 der Multiframen 4, 8, …, 60.

Die Korrektur stellt an dieser Position `SCH/HD + BNCH` bereit. LMAC erzeugt
daraus den normalen Downlink-Burst mit zwei Halbblöcken. Die verpflichtenden
Synchronisationspositionen bleiben erhalten. Regressionstests prüfen die
rotierende Belegung und die Umsetzung bis zum PHY-Auftrag.

Der Umfang ist auf den obligatorischen BNCH auf dem Hauptkontrollkanal und
belegten Kanälen begrenzt. Andere bisherige Default-Belegungen in Rahmen 18
werden für diesen Vergleich nicht verändert. Insbesondere ist damit keine
vollständige Konformität sämtlicher Rahmen-18-Positionen nachgewiesen.

Dies korrigiert einen Fehler in der Luftschnittstelle. Ob dieser Fehler die
beobachteten REREGs verursacht, ist noch nicht nachgewiesen. Die Änderung
ist kein Beleg für bereits behobene Unterbrechungen am Sepura.

## Prüfung und Update: nur SRV-M-TBS-01

Nach dem Merge in `katwarn/nina` den manuell gestarteten Funkprozess im
bisherigen Terminal mit **Strg+C** beenden. Dann auf der Basisstation:

```bash
(
set -euo pipefail
umask 077
cd /opt/netcore-tetra
if pgrep -x bluestation-bs >/dev/null; then
    echo 'Die Basisstation läuft noch. Zuerst im bisherigen Terminal beenden.'
    exit 1
fi
test -s ./config.toml
git remote set-branches origin katwarn/nina
git fetch origin
if git show-ref --verify --quiet refs/heads/katwarn/nina; then
    git switch katwarn/nina
else
    git switch --track -c katwarn/nina origin/katwarn/nina
fi
git branch --set-upstream-to=origin/katwarn/nina katwarn/nina
git pull --ff-only origin katwarn/nina
if [ -f /root/.cargo/env ]; then . /root/.cargo/env; fi
cargo test -p tetra-entities --lib umac::subcomp::bs_sched::tests:: -- --nocapture
cargo test -p tetra-entities --lib lmac::lmac_bs::tests:: -- --nocapture
cargo build --release -p bluestation-bs
install -d -m 700 /opt/netcore-rereg-tests
REREG_LOG="/opt/netcore-rereg-tests/frame18-bnch-$(date +%Y%m%d-%H%M%S).log"
git log -1 --format='%h %s'
echo "Logdatei: $REREG_LOG"
./target/release/bluestation-bs ./config.toml 2>&1 | tee "$REREG_LOG"
)
```

Der Befehl verwendet die vorhandene `config.toml`. Er setzt lokale Änderungen
nicht zurück und bricht bei Pull-, Test- oder Buildfehlern ab.
Die Fetch-Zuordnung wird zuerst auf `katwarn/nina` umgestellt. Das behebt auch
`couldn't find remote ref refs/heads/mqtt` bei alten Single-Branch-Klonen.
Auf den LXC-Diensten ist für diese Korrektur kein Update nötig.

Für den Vergleich dieselbe Konfiguration, Antennenposition und dasselbe
Sepura verwenden. Mindestens zehn Minuten beobachten: zunächst Leerlauf,
dann Gruppenruf/PTT, Freigabe und erneuter Ruf. Sichtbare REREGs und
Unterbrechungen mit Uhrzeit festhalten. Ein erfolgreicher Build oder
Location-Update-ACK ersetzt diesen Funkversuch nicht.
