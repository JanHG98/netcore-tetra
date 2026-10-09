# SWMI/Main Air-Interface Compatibility Fix

## Ziel

Der `swmi`-Stand behält Subscriber Core, Group Core, Mobility Core, Call Control,
Provisioning Core, Node Gateway und den offenen lokalen Fallback. Gleichzeitig wird
das am Funkgerät bewährte Air-Interface-Verhalten des letzten funktionierenden
`main`-Standes wiederhergestellt.

## Vergleich `main` gegen `swmi`

Der für Sepura relevante Unterschied lag in der MM-Antwort auf
`U-LOCATION-UPDATE-DEMAND`:

- `main` spiegelt bei AIv2-/Common-SCCH-Geräten den vom Funkgerät angeforderten
  Location-Update-Typ in `D-LOCATION-UPDATE-ACCEPT`.
- `swmi` hatte diese Kompatibilität auf den ersten ITSI-Attach begrenzt und spätere
  `RoamingLocationUpdating`- bzw. `ServiceRestorationRoamingLocationUpdating`-
  Vorgänge in `PeriodicLocationUpdating` umgewandelt.

Beim getesteten Sepura führte dies zu wiederholter MM-Behandlung, verzögerter
Gruppenauswahl und dazu, dass ein normaler `U-SETUP` erst nach einem weiteren
Location Update gesendet wurde.

Der Fix übernimmt deshalb die bewährte `main`-Semantik für die betroffene
Capability-Klasse, ohne die SWMI-Mobility- und Core-Funktionen zu entfernen:

- AIv2 + Common SCCH: angeforderten Update-Typ spiegeln.
- Andere Geräte: konfigurierte periodische Registrierung unverändert verwenden.
- Abgeschlossene Inter-SwMI-Migration: weiterhin `DemandLocationUpdating`.
- T351/Registrierungstimer laufen intern weiterhin unabhängig von der codierten
  Accept-Art.

## Simplex und Duplex

Simplex und Duplex sind **keine Provisioning-Berechtigungen**.

- Jedes registrierte TETRA-Funkgerät darf einen Simplex- oder Duplex-Einzelruf
  anfordern.
- `ClassOfMs.freq_simplex_duplex` wird nur als gemeldete Fähigkeit/Telemetrie
  gespeichert und nicht als Zugriffsregel verwendet.
- Maßgeblich sind die vom Funkgerät gesendete `U-SETUP`-Rufart, die Antwort des
  Zielgeräts und verfügbare Traffic Slots.
- Für einen lokalen Funkgerät-zu-Funkgerät-Ruf muss die Ziel-ISSI im Onlinebetrieb
  weiterhin als Teilnehmer bekannt sein. Das ist Erreichbarkeitsrouting, keine
  Simplex-/Duplex-Beschränkung.
- Im lokalen Edge-Fallback werden zentrale Teilnehmer-, Gruppen- und
  Rufrestriktionen weiterhin nicht erzwungen.

## SIP/Asterisk-Ziele

SIP-Nummern sind keine TETRA-Teilnehmer und werden nicht im Provisioning Core
angelegt.

Eine explizite Asterisk-Wählregel hat nun Vorrang vor der lokalen ISSI-Prüfung. So
wird beispielsweise `91385` bei `outbound_prefix = "91"` als SIP-Ziel `385`
geroutet, selbst wenn die Ziffernfolge zufällig auch als lokale ISSI vorkäme.

Unbegrenzte Ziele hinter einem Präfix:

```toml
[asterisk]
enabled = true
outbound_prefix = "91"
strip_outbound_prefix = true
service_numbers = ["*"]
```

Alternativ bedeutet auch eine leere Liste, dass jede Nummer hinter dem Präfix
zulässig ist:

```toml
service_numbers = []
```

Eine echte Allowlist bleibt möglich:

```toml
service_numbers = ["385", "600", "38*"]
```

## Betroffene Komponenten

Neu zu bauen ist nur die Basisstations-Binary `bluestation-bs`. Subscriber Core,
Group Core, Node Gateway und Provisioning Core benötigen für diesen Fix keinen
Neubau.

## Aktualisierung der Basisstation

```bash
sudo systemctl stop tetra.service
cd /opt/netcore-tetra
source "$HOME/.cargo/env"

CARGO_BUILD_JOBS=2 cargo build --release \
  -p bluestation-bs \
  --features "bluestation-bs/asterisk,bluestation-bs/recording,bluestation-bs/audio-player"

sudo install -m 0755 \
  target/release/bluestation-bs \
  /usr/local/bin/bluestation-bs

sudo systemctl restart tetra.service
sudo systemctl status tetra.service --no-pager
```

## Erwartete Logfolge beim Sepura

Nach Registrierung und Gruppen-Affiliation muss der erste normale PTT-Versuch ohne
vorherigen zusätzlichen Periodic-Refresh zu einem `U-SETUP` führen. Bei späteren
Roaming-Refreshes soll der Accept-Typ für AIv2/Common-SCCH gespiegelt werden.

```text
AIv2/common-SCCH compatibility mirrors RoamingLocationUpdating
<- U-SETUP
```
