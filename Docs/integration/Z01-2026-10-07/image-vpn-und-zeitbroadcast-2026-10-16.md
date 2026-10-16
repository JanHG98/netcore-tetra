# Image: VPN fertig einrichten und Zeitbroadcast korrigieren

Stand: 16.10.2026. Aufgabe Z01.4, [PR #98](https://github.com/JanHG98/netcore-tetra/pull/98).

## Befund und Korrektur

Jan bestätigt KDE auf dem Image `5ea8d3be`. Die in Deploy eingetragene
OpenVPN-Verbindung fehlt in KDEs Verbindungsliste; das Funkgerät zeigt
14:40 statt 16:40. Eine Offsetänderung im RadioManager behebt dies nicht.

Der alte Imagepfad erzeugte einen OpenVPN-Systemdienst außerhalb von
NetworkManager. Jetzt entsteht bereits während der Personalisierung das
systemweite Profil **NetCore VPN**. Der echte OpenVPN-Importer und libnm
erzeugen und prüfen es offline, einschließlich der in Deploy eingegebenen
Zugangsdaten. Ein Importfehler beendet den Build.

Zertifikate und Schlüssel liegen unter `/etc/openvpn/client/netcore.d/`
mit Verzeichnismodus 0700 und Dateimodus 0600. Das NetworkManager-Keyfile ist
ebenfalls 0600; gespeicherte Zugangsdaten ermöglichen die Verbindung vor einer
Desktopanmeldung. Private Inhalte werden nicht in öffentliche Jobdaten geschrieben.

Die Heimnetzregel steuert dieselbe sichtbare Verbindung über ihre UUID.
In einem eingetragenen vertrauenswürdigen WLAN oder über Ethernet im
eingetragenen Heimnetz bleibt sie aus beziehungsweise wird getrennt.
Außerhalb verbindet sie automatisch, sobald ein Netzwerkzugang besteht.
Laufende oder verbindende Profile werden nicht nochmals gestartet.
Der alte OpenVPN-Instanzautostart wird deaktiviert.

Der Zeitencoder verwendete fälschlich 23 Bit für den Offset in
Zweiskundenschritten. ETSI EN 300 392-2 V3.8.1, 18.5.24, Tabelle 18.100,
Seite 562, legt diese 48 Bit fest:

| Feld | Bits | Kodierung |
| --- | ---: | --- |
| UTC-Zeit | 24 | Sekunden seit Jahresbeginn UTC, geteilt durch 2 |
| Offset-Vorzeichen | 1 | 0 positiv, 1 negativ |
| Lokaler Offset | 6 | 15-Minuten-Schritte einschließlich Sommerzeit |
| Jahr | 6 | Jahr seit 2000, gültig 0–62 |
| Reserviert | 11 | alle Bits 1 |

Für 16.10.2026 14:40:00 UTC / 16:40:00 Europe/Berlin ergibt sich
`0xBE3F2010D7FF`: UTC-Zähler 12468000, Vorzeichen 0, Offset 8, Jahr 26,
reserviert `0x7FF`. Die UTC-Zeit wird nicht künstlich verschoben.
Die Zeitzone aus Deploy gilt für Betriebssystem und `cell_info.timezone`;
Sommer-/Winterzeit wird aus der IANA-Zeitzone abgeleitet.

## Deployment-VM aktualisieren

Auf **VM-H-DEPLOY-01** als `jhoffmeister` ausführen. main wird einmal auf
einen vollständigen Commit festgelegt. Der bestehende Updater aktualisiert
Controller, Builder, Rezept und UI. Er erhält Konfiguration, Profile, Artefakte
und Caches und startet selbst keinen Imagebuild. Laufende oder ungeklärte
Aufträge blockieren den Austausch; einen aktiven Auftrag zuerst abschließen lassen.

```bash
(
  set -Eeuo pipefail
  NC_SRC=$(mktemp -d /tmp/netcore-image-vpn-time.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" remote add origin https://github.com/JanHG98/netcore-tetra.git
  git -C "$NC_SRC" config http.version HTTP/1.1
  git -C "$NC_SRC" config remote.origin.promisor true
  git -C "$NC_SRC" config remote.origin.partialclonefilter blob:none
  git -C "$NC_SRC" fetch --depth 1 --filter=blob:none origin refs/heads/main
  NC_REF=$(git -C "$NC_SRC" rev-parse FETCH_HEAD)
  printf 'Image-Rezept Zielcommit: %s\n' "$NC_REF"
  git -C "$NC_SRC" sparse-checkout init --cone
  git -C "$NC_SRC" sparse-checkout set tools/operator system-backend/deployment-core
  git -C "$NC_SRC" checkout --detach "$NC_REF"
  [[ "$(git -C "$NC_SRC" rev-parse HEAD)" == "$NC_REF" ]]
  sudo python3 -B "$NC_SRC/tools/operator/update-vm119-image-features.py" \
    --commit "$NC_REF" --apply
)
```

Nach `phase=passed` die WebUI neu laden und unter **Images** einen neuen
Build anlegen: gewünschter Desktop, `.ovpn`, VPN-Benutzername und
VPN-Passwort, Zeitzone `Europe/Berlin` sowie die gewünschten
Heimnetzausnahmen eintragen. Neue Jobs lösen main frisch auf.
Beide Korrekturen benötigen das neue Rezept und TBS-Binary. Ein bereits
heruntergeladenes Image ändert sich durch das VM-Update nicht.
Der Rezeptfingerprint verhindert einen alten Softwarecache.

## Prüfung am Pi

**NetCore VPN** muss in der Verbindungsliste erscheinen. Im Heimnetz ist
ein sichtbares, ausgeschaltetes Profil korrekt. Die automatische Verbindung
in einem fremden WLAN oder Ethernetnetz außerhalb der Ausnahmen prüfen;
Benutzername und Passwort müssen nicht erneut eingegeben werden.

Die folgenden Befehle geben keine VPN-Passwörter oder Schlüssel aus:

```bash
nmcli -f NAME,UUID,TYPE connection show
nmcli -f NAME,TYPE,STATE connection show --active
systemctl status netcore-vpn-policy.timer --no-pager
sudo journalctl -b -u netcore-vpn-policy.service --no-pager -n 25
date --iso-8601=seconds
timedatectl show -p Timezone -p NTPSynchronized
sudo journalctl -b -u tetra.service --no-pager -n 150 \
  | sed -n '/D-NWRK-BROADCAST/p'
```

Bei synchronisierter Systemzeit die Ortszeit am Funkgerät vergleichen.
Die Verbindung mit Jans Server und die tatsächliche Funkgeräteanzeige
bleiben Betreiberabnahmen.

## Nachweise

VPN-/Zeit-Prüfstand: `151368a6c9bd732d59757ae34bb9d4ffd96d1391`.
ARM64-Personalisierung: `fbc538acc258ff3841bf63b464d1728991142876`.

- [Native VPN-/Zeitprüfung](https://github.com/JanHG98/netcore-tetra/actions/runs/38924137691):
  Ubuntu 24.04, Ubuntu 26.04 und Debian Bookworm jeweils 24 VPN- und 14
  Policy-/Materialfälle sowie ein tatsächlicher zertifikat-/passwortgeschützter
  OpenVPN-Tunnel mit tls-auth, Datenverkehr zwischen getrennten Netz-Namespaces,
  Heimnetzabschaltung, Schutz vor Doppelaktivierung und Wiederverbindung nach
  NetworkManager-Neustart ohne Desktop oder Secret-Agent; echte nmcli-Abfrage eines WLAN-Profils mit Umlauten und Doppelpunkt ebenfalls bestanden.
- Neun Rust-Fälle: der gemeldete Berliner Oktoberzeitpunkt, UTC, positive und
  negative Viertelstundenoffsets, Sommer-/Winterzeitwechsel, Schaltjahr,
  Jahreswechsel, ungültige Werte und vollständige Broadcast-PDU-Serialisierung.
- [Tatsächliche ARM64-Personalisierung](https://github.com/JanHG98/netcore-tetra/actions/runs/38922557349/job/117045870983):
  offizielles Pi-OS-Image/SHA256, echtes ARM64-chroot, vollständiges
  CA-Clientprofil, privates NetworkManager-Keyfile samt Zugangsdaten/UUID,
  Stationszeitzone, SSH-Konfiguration, ext4-Prüfung, Verkleinerung und Cleanup
  bestanden. Ebenso reale VM-Installation/Wiederholungsinstallation,
  Update-/Erhalt-/Busy-/Rückwegprüfung und systemd-Verzeichnistest bestanden.
- [Zentraler Quellabgleich](https://github.com/JanHG98/netcore-tetra/actions/runs/38922557519),
  Deployment-/Browser-Prüfung und Defaultfeature-TBS-Link ebenfalls bestanden.

Die ARM64-Personalisierung verwendet die Lite-Variante; sie ist kein neuer
vollständiger KDE-Softwarebuild oder Pi-/On-Air-Beleg. Die separate Desktop-
Matrix auf Ubuntu 24.04 bleibt am bereits bekannten fehlenden
`qemu-aarch64`-binfmt der Runner blockiert. Der Betreiber bestätigt KDE
auf `5ea8d3be`; den neuen Build und beide Funktionen am Pi gesondert abnehmen.

Normgrundlage: bereitgestellte ETSI EN 300 392-2 V3.8.1, Tabelle 18.100.
[NetworkManager-Beispiel für den nativen VPN-Import](https://github.com/NetworkManager/NetworkManager/blob/main/examples/python/gi/vpn-import.py).
