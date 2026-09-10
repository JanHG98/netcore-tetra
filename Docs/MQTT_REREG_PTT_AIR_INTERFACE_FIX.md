# MQTT REREG / PTT air-interface fix

## Symptom

A terminal can register and affiliate successfully, then repeatedly enter `RoamingLocationUpdating`. While it is trapped in MM re-registration, pressing PTT can show **service unavailable** on the radio and the base-station log contains no CMCE `U-SETUP`.

## Fix

This branch keeps the distributed Node Gateway / Core architecture and makes three narrow local-RF corrections:

1. A successful registration always carries an explicit energy-saving decision. If the MS requests no mode and no prior mode exists, the BS explicitly grants `StayAlive` in the first `D-LOCATION-UPDATE-ACCEPT`.
2. AIv2/common-SCCH compatibility preserves an `ItsiAttach` as `ItsiAttach` even when the BS retained an MM client record across RF absence/T351 recovery. `RoamingLocationUpdating`, however, is settled as `PeriodicLocationUpdating` whenever periodic registration is enabled, preventing a terminal-side REREG loop.
3. A Brew/backhaul reconnect no longer forces all healthy camped radios through `D-LOCATION-UPDATE-COMMAND`. `BrewEntity::resync_subscribers()` already restores REGISTER/AFFILIATE state to the remote side; local RF registration remains stable across backhaul churn.

No SNDCP/TUN behaviour is changed. No central call-control policy is bypassed.

## Expected log

```text
MM: ISSI 5102 initial AIv2/common-SCCH ITSI attach acknowledged as ITSI attach
MM: ISSI 5102 known roaming refresh settled as PeriodicLocationUpdating to prevent re-registration loop
```

After MM settles, PTT should reach CMCE and produce a `USetup` / `U-SETUP` trace.

A Brew reconnect should only log:

```text
MM: Brew backhaul reconnected; retaining local RF registrations (BrewEntity handles subscriber resync)
```

## TBS test

```bash
cd /opt/netcore-tetra
git fetch origin
git switch mqtt-fix/rereg-ptt-air-interface
git pull --ff-only origin mqtt-fix/rereg-ptt-air-interface

cargo build --release -p bluestation-bs
sudo install -m 0755 target/release/bluestation-bs /usr/local/bin/bluestation-bs
sudo systemctl restart tetra.service

sudo journalctl -u tetra.service -f | grep --line-buffered -Ei 'LocationUpdate|REREG|USetup|U-SETUP|Brew backhaul|5102'
```
