# NetCore Provisioning Core – vollständige Installation und Inbetriebnahme

## 1. Zweck und Name

Der neue Verwaltungs-LXC heißt **NetCore Provisioning Core**.

- technischer Dienstname: `provisioning-core`
- systemd-Unit: `netcore-provisioning-core.service`
- empfohlener LXC-Hostname: `CT-M-PROV-01`
- WebUI/API: TCP `8125`

Der Provisioning Core ersetzt Subscriber Core und Group Core nicht. Er ist die gemeinsame Verwaltungsoberfläche vor beiden autoritativen Diensten.

Funktionen:

- Geräte/ISSIs anlegen, bearbeiten, sperren, freigeben und löschen
- Gruppen/GSSIs anlegen, bearbeiten und löschen
- Attach, Gruppenruf, SDS, Notruf und DGNA je Gruppe freigeben
- Mitgliedschaften als Geräte-×-Gruppen-Matrix verwalten
- beim Löschen eines Gerätes oder einer Gruppe abhängige Mitgliedschaften entfernen
- Subscriber Core und Group Core gemeinsam auf alle verbundenen TBS synchronisieren

## 2. Beispieltopologie

| Komponente | Beispiel-IP | Port |
|---|---:|---:|
| Node Gateway | `10.0.1.179` | 8080 |
| Subscriber Core | `10.0.1.181` | 8100 |
| Group Core | `10.0.1.182` | 8110 |
| Provisioning Core (`CT-M-PROV-01`) | `10.0.1.183` | 8125 |
| Call Control | nach eigener Belegung | 8120 |
| Media Switch | nach eigener Belegung | 8130 |
| Basisstation | nach eigener Belegung | 8080 |

Die Beispieladressen müssen an das eigene Netz angepasst werden.

## 3. Änderungen zuerst in GitHub übernehmen

Die Implementierung wird als Git-Patch `netcore-provisioning-core-and-fallback.patch` bereitgestellt. Der Patch wird einmal in einen eigenen Branch übernommen; danach installieren und aktualisieren alle LXCs wieder normal über GitHub.

Auf einem Rechner mit Git und Rust:

```bash
rm -rf netcore-tetra-provisioning

git clone   --branch swmi   --single-branch   https://github.com/JanHG98/netcore-tetra.git   netcore-tetra-provisioning

cd netcore-tetra-provisioning
git checkout -b feature/provisioning-core

git apply --check /PFAD/netcore-provisioning-core-and-fallback.patch
git apply /PFAD/netcore-provisioning-core-and-fallback.patch

cargo fmt --all
cargo check --locked --package netcore-provisioning-core
cargo test --locked --package netcore-provisioning-core
cargo test --locked --package tetra-config --package tetra-entities

git add -A
git commit -m "feat: add Provisioning Core and unrestricted local fallback"
git push -u origin feature/provisioning-core
```

Erst nach erfolgreichem Build wird der Branch auf Basisstation und LXCs verwendet. Nach dem späteren Merge in `swmi` können die Clone-/Update-Befehle wieder auf `swmi` umgestellt werden.

## 4. Proxmox-LXC anlegen

Empfohlene Werte:

- Debian 13
- unprivilegierter Container
- 1–2 vCPU
- 1 GiB RAM
- 8 GiB Storage
- feste IP oder DHCP-Reservation
- Start beim Host-Boot aktiv
- keine Hardware-Durchreichung notwendig

Beispielname:

```text
CT-M-PROV-01
```

Nach dem ersten Start anmelden und aktualisieren:

```bash
apt update
apt full-upgrade -y
apt install -y \
  git curl ca-certificates \
  build-essential pkg-config libssl-dev \
  jq netcat-openbsd
reboot
```

## 5. Rust installieren

Als `root` im separaten LXC:

```bash
curl --proto '=https' --tlsv1.2 -sSf \
  https://sh.rustup.rs \
  | sh -s -- -y

source /root/.cargo/env
rustup default stable
cargo --version
```

## 6. Repository klonen

```bash
rm -rf /opt/netcore-tetra

git clone \
  --branch feature/provisioning-core \
  --single-branch \
  https://github.com/JanHG98/netcore-tetra.git \
  /opt/netcore-tetra

cd /opt/netcore-tetra
git branch --show-current
git log -1 --oneline
```

Der Branch muss für den Test `feature/provisioning-core` sein und den Ordner `system-backend/provisioning-core/` enthalten.

## 7. Provisioning Core installieren

```bash
cd /opt/netcore-tetra
chmod +x system-backend/provisioning-core/install/*.sh
sudo bash system-backend/provisioning-core/install/install.sh
```

Danach die Konfiguration öffnen:

```bash
nano /etc/netcore/provisioning-core.toml
```

Beispiel:

```toml
[server]
bind = "0.0.0.0:8125"

[upstream]
subscriber_core = "http://10.0.1.181:8100"
group_core = "http://10.0.1.182:8110"
timeout_secs = 5

[security]
mode = "open_lab"
allow_remote_management = true

[limits]
max_body_bytes = 2097152
```

Dienst neu starten:

```bash
systemctl restart netcore-provisioning-core.service
systemctl status netcore-provisioning-core.service --no-pager
journalctl -u netcore-provisioning-core.service -n 100 --no-pager
```

## 8. Netzwerkverbindungen prüfen

Vom Provisioning-LXC:

```bash
nc -vz 10.0.1.181 8100
nc -vz 10.0.1.182 8110

curl -fsS http://10.0.1.181:8100/api/v1/status | jq .
curl -fsS http://10.0.1.182:8110/api/v1/status | jq .
```

Provisioning Core selbst:

```bash
curl -fsS http://127.0.0.1:8125/health/live | jq .
curl -fsS http://127.0.0.1:8125/health/ready | jq .
curl -fsS http://127.0.0.1:8125/api/v1/dashboard | jq .
```

WebUI:

```text
http://10.0.1.183:8125/
```

## 9. Welche anderen LXCs müssen den Provisioning Core kennen?

### 9.1 Subscriber Core

Subscriber Core muss **keine** Provisioning-Core-Adresse erhalten. Der Datenfluss läuft andersherum:

```text
Provisioning Core → Subscriber Core → Node Gateway → TBS
```

Ist der Dienst noch nicht installiert, im Subscriber-Core-LXC:

```bash
apt update
apt install -y git curl ca-certificates build-essential pkg-config libssl-dev jq
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source /root/.cargo/env

git clone --branch feature/provisioning-core --single-branch \
  https://github.com/JanHG98/netcore-tetra.git /opt/netcore-tetra
cd /opt/netcore-tetra
bash system-backend/subscriber-core/install/install.sh
```

In `/etc/netcore/subscriber-core.toml` mindestens prüfen:

```toml
[server]
bind = "0.0.0.0:8100"

[node_gateway]
url = "ws://10.0.1.179:8080/ws/backend"
reconnect_secs = 5

[access_policy]
mode = "allow_list"
auto_sync = true
disconnect_unauthorized = true
sync_timeout_secs = 30

[security]
mode = "open_lab"
allow_remote_management = true
```

Danach:

```bash
systemctl restart netcore-subscriber-core.service
systemctl status netcore-subscriber-core.service --no-pager
curl -fsS http://127.0.0.1:8100/api/v1/status | jq .
```

`mode = "allow_list"` bedeutet online: Nur im Provisioning Core angelegte und freigegebene ISSIs dürfen sich registrieren. Für einen Einzelruf müssen deshalb Anrufer **und** Zielgerät vorhanden und freigegeben sein.

### 9.2 Group Core

Auch Group Core benötigt keine Provisioning-Core-Adresse:

```text
Provisioning Core → Group Core → Node Gateway → TBS
```

Ist der Dienst noch nicht installiert, im Group-Core-LXC:

```bash
apt update
apt install -y git curl ca-certificates build-essential pkg-config libssl-dev jq
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source /root/.cargo/env

git clone --branch feature/provisioning-core --single-branch \
  https://github.com/JanHG98/netcore-tetra.git /opt/netcore-tetra
cd /opt/netcore-tetra
bash system-backend/group-core/install/install.sh
```

In `/etc/netcore/group-core.toml` mindestens prüfen:

```toml
[server]
bind = "0.0.0.0:8110"

[node_gateway]
url = "ws://10.0.1.179:8080/ws/backend"
reconnect_secs = 5

[policy]
allow_unlisted_groups = false
enforce_memberships = true
reconcile_registered = true
auto_sync = true
sync_timeout_secs = 30
dgna_timeout_secs = 30

[security]
mode = "open_lab"
allow_remote_management = true
```

Danach:

```bash
systemctl restart netcore-group-core.service
systemctl status netcore-group-core.service --no-pager
curl -fsS http://127.0.0.1:8110/api/v1/status | jq .
```

### 9.3 Node Gateway

Im Node Gateway müssen Subscriber Core und Group Core als kritische Dienste überwacht werden. Bereits vorhandene Blöcke nicht doppelt anlegen; nur deren URLs korrigieren:

```toml
[service_monitor]
enabled = true
interval_secs = 5
timeout_ms = 1500
failure_threshold = 2
recovery_threshold = 2

[[service_monitor.targets]]
name = "subscriber-core"
url = "http://10.0.1.181:8100/health/ready"
critical_for_edge = true
fallback_mode = "cached_policy_then_static_config"

[[service_monitor.targets]]
name = "group-core"
url = "http://10.0.1.182:8110/health/ready"
critical_for_edge = true
fallback_mode = "cached_policy_then_local_affiliations"
```

Der Provisioning Core kann zusätzlich angezeigt werden. Er ist **nicht funkbetriebskritisch** und darf daher keinen lokalen Fallback auslösen:

```toml
[[service_monitor.targets]]
name = "provisioning-core"
url = "http://10.0.1.183:8125/health/ready"
critical_for_edge = false
fallback_mode = "management_only_no_radio_impact"
```

Danach:

```bash
systemctl restart netcore-node-gateway.service
systemctl status netcore-node-gateway.service --no-pager
curl -fsS http://127.0.0.1:8080/api/v1/core-services | jq .
```

### 9.4 Basisstation

Die TBS verbindet sich weiterhin nur mit dem Node Gateway. **Provisioning Core wird nicht direkt in `config.toml` eingetragen.**

Der Provisioning Core gehört außerdem nicht in:

```toml
[edge_fallback]
required_services = [...]
```

Er ist nur die Verwaltungsoberfläche. Fällt er aus, behalten Subscriber Core und Group Core ihre Daten und der Funkbetrieb läuft weiter.

### 9.5 Call Control, Media Switch und Control Room

Diese Dienste benötigen ebenfalls keine Provisioning-Core-Adresse. Teilnehmer- und Gruppenrichtlinien werden über Node Gateway verteilt. Eine spätere reine UI-Verlinkung kann ergänzt werden, ist für den Betrieb jedoch nicht erforderlich.

## 10. Teilnehmer vollständig anlegen

Für einen lokalen Einzelruf müssen **beide** Funkgeräte im Subscriber Core angelegt und freigegeben sein. Nur den Anrufer anzulegen reicht nicht.

In der Provisioning-WebUI unter **Geräte** für jedes Funkgerät:

- ISSI korrekt
- Aktiv = an
- Registrierung erlaubt = an
- Home MCC/MNC passend zum Netz
- optional Notruf, SDS und Paketdaten

Beispiel:

```text
5102  HRT Jan
5103  HRT Test 2
```

Nach dem Speichern **Jetzt synchronisieren** ausführen.

Kontrolle auf Subscriber Core:

```bash
curl -fsS http://10.0.1.181:8100/api/v1/subscribers | jq .
curl -fsS -X POST http://10.0.1.181:8100/api/v1/sync | jq .
```

Kontrolle auf der TBS:

```bash
journalctl -u tetra.service -n 300 --no-pager \
  | grep -E 'subscriber access policy|allowed_count|registered|ISSI'
```

`allowed_count` muss mindestens der Zahl der freigegebenen Funkgeräte entsprechen.

## 11. Gruppen und Matrix anlegen

Unter **Gruppen** die Sprachgruppe erstellen, beispielsweise:

```text
GSSI 15201
Aktiv: ja
Attach: ja
Gruppenruf: ja
SDS: ja
Notruf: nach Bedarf
```

Eine reine SDS-/Statusgruppe, beispielsweise `15501`, kann so definiert werden:

```text
Aktiv: ja
Attach: ja
Gruppenruf: nein
SDS: ja
```

Unter **Mitgliedschaftsmatrix**:

- Zeile = Gerät/ISSI
- Spalte = Gruppe/GSSI
- Haken = Mitgliedschaft erlaubt

Danach erneut synchronisieren.

API-Kontrolle:

```bash
curl -fsS http://10.0.1.182:8110/api/v1/groups | jq .
curl -fsS http://10.0.1.182:8110/api/v1/memberships | jq .
curl -fsS -X POST http://10.0.1.182:8110/api/v1/sync | jq .
```

## 12. TBS für zentrale Richtlinien und lokalen Fallback konfigurieren

In `/etc/netcore/config.toml`:

```toml
[control_room]
enabled = true
host = "10.0.1.179"
port = 8080
use_tls = false
endpoint_path = "/ws/node"
node_id = "tbs-04010001"
station_name = "SRV-M-TBS-01"
site = "Main"
central_sds_routing = false

[edge_fallback]
enabled = true
enter_after_secs = 15
recover_after_secs = 20
unknown_service_is_available = false
service_matrix_lease_secs = 60
policy_cache_path = "/var/lib/flowstation/edge-policy-cache.json"
policy_cache_max_age_secs = 604800
keep_last_known_policy = true
event_spool_path = "/var/lib/flowstation/edge-event-spool.jsonl"
event_spool_max_entries = 10000
event_spool_max_bytes = 16777216
replay_batch_size = 128
required_services = [
  "subscriber-core",
  "group-core",
  "mobility-core",
  "call-control",
  "media-switch",
  "sds-router"
]
```

Wichtig:

- `control_room.enabled = true`
- `edge_fallback.enabled = true`
- `provisioning-core` nicht in `required_services`

## 13. TBS-Binary mit Einzelruf- und Fallback-Fix aktualisieren

Auf der Basisstation:

```bash
sudo systemctl stop tetra.service
cd /opt/netcore-tetra

git fetch origin
git checkout feature/provisioning-core
git pull --ff-only origin feature/provisioning-core

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

Der Fix bewirkt:

- Onlinebetrieb: Subscriber- und Gruppenrichtlinien bleiben autoritativ.
- Lokaler Fallback: zentrale Gruppen-, Mitgliedschafts- und Rufrichtlinien werden nicht angewendet.
- Gruppenrufe werden im Fallback auch ohne frische Listener-/Affiliation-Cacheeinträge lokal ausgesendet.
- lokale Einzelrufe werden im Fallback an die Ziel-ISSI gepaged, auch wenn der zentrale Teilnehmerzustand veraltet ist.
- Simplex benötigt einen Traffic Slot; Duplex benötigt zwei freie Traffic Slots.

Physikalische und protokollbedingte Grenzen bleiben bestehen: Ein Funkgerät muss den gewünschten Dienst unterstützen, auf der Zelle eingebucht und auf der Luftschnittstelle erreichbar sein.

## 14. Simplex-Einzelruf testen

Voraussetzungen:

- beide ISSIs im Provisioning Core aktiv und registrierungsberechtigt
- beide Geräte im TBS-WebUI als registriert sichtbar
- Zielgerät nicht bereits in einem inkompatiblen Ruf

Live-Log:

```bash
journalctl -u tetra.service -f \
  | grep -E 'U-SETUP P2P|rx_u_setup_p2p|D-SETUP|U-ALERT|U-CONNECT|D-CONNECT|individual|rejecting'
```

Erwartete Richtung:

```text
U-SETUP P2P 5102 → 5103
D-CALL-PROCEEDING an 5102
D-SETUP an 5103
U-ALERT/U-CONNECT von 5103
D-CONNECT an 5102
```

## 15. Duplex-Einzelruf testen

Duplex benötigt zwei freie Traffic Slots. Bei Dual Carrier stehen hierfür mehrere Traffic Slots zur Verfügung, trotzdem dürfen keine anderen Rufe die notwendige Kapazität belegen.

Log:

```bash
journalctl -u tetra.service -f \
  | grep -E 'duplex|second circuit|rx_u_setup_p2p|D-SETUP|U-CONNECT|D-CONNECT|Congestion'
```

Erwartet sind unterschiedliche Timeslots für beide Rufseiten:

```text
ts(call)=...
ts(called)=...
```

Ein gemeldetes `ClassOfMs duplex:false` wird von NetCore nur als Telemetrie gespeichert und blockiert weder Simplex noch Duplex. Der Provisioning Core enthält deshalb bewusst keine Rufarten-Freigabe. Entscheidend ist die tatsächlich vom Funkgerät gesendete `U-SETUP`-Rufart; lehnt das Endgerät den Ruf bereits lokal ab und sendet keine passende `U-SETUP`, liegt die Einschränkung im Gerät beziehungsweise Codeplug, nicht in der zentralen Provisionierung.

### SIP-/Asterisk-Ziele ohne Provisioning

SIP-Nummern werden nicht als Geräte im Provisioning Core angelegt. Eine explizite Asterisk-Wählregel wird vor der lokalen ISSI-Routingentscheidung ausgewertet. Für beliebig viele Ziele hinter dem Präfix:

```toml
[asterisk]
enabled = true
outbound_prefix = "91"
strip_outbound_prefix = true
service_numbers = ["*"]
```

Auch `service_numbers = []` erlaubt jedes Ziel hinter dem Präfix. Ein Wählstring wie `91385` wird damit als SIP-Benutzer `385` geroutet, selbst wenn `91385` zufällig als numerische ISSI interpretierbar wäre.

## 16. Lokalen Fallback testen

Im Testnetz auf dem Node Gateway oder den kritischen Cores stoppen:

```bash
systemctl stop netcore-node-gateway.service
```

Auf der TBS beobachten:

```bash
journalctl -u tetra.service -f \
  | grep -E 'edge fallback transition|fallback local P2P|fallback broadcasting|central.*policy'
```

Nach `enter_after_secs` muss der Modus `Degraded` oder `Isolated` werden.

Im lokalen Fallback entfallen administrative zentrale Restriktionen für:

- Gruppenmitgliedschaft
- Attach an Gruppen
- Gruppenruf-Freigabe
- Notruf-Freigabe einer Gruppe
- zentrale Rufart-/Gesprächsrichtlinien
- zentralen Teilnehmer-Admission-Override
- Entscheidung „kein gecachter Listener"
- Entscheidung „Ziel-ISSI nicht mehr zentral als lokal registriert bekannt"

Eine bewusst lokal in `[security]` konfigurierte statische ISSI-Whitelist bleibt als Standortschutz wirksam. Soll der Standort im Fallback vollständig offen sein, darf dort keine statische Whitelist eingetragen sein.

Testablauf:

1. Node Gateway stoppen.
2. Fallback-Zustand abwarten.
3. eine zentral nicht freigegebene Gruppe am Funkgerät auswählen und Gruppenruf testen.
4. Simplex-Einzelruf zwischen zwei lokal eingebuchten Geräten testen.
5. Duplex-Einzelruf bei zwei freien Traffic Slots testen.
6. Node Gateway wieder starten.
7. `recover_after_secs` abwarten; danach greifen zentrale Richtlinien wieder.

## 17. Fehlerdiagnose

### Provisioning Core sieht Subscriber/Group Core nicht

```bash
journalctl -u netcore-provisioning-core.service -n 100 --no-pager
curl -v http://10.0.1.181:8100/api/v1/status
curl -v http://10.0.1.182:8110/api/v1/status
```

### Gerät registriert ständig neu

```bash
journalctl -u tetra.service -n 500 --no-pager \
  | grep -E 'LocationUpdate|re-registered|subscriber access policy|disconnect|whitelist'
```

Ein `RoamingLocationUpdating` kann ein normaler Mobility Refresh sein. Kritisch wird es erst bei Reject/Disconnect oder ständig verlorenem Downlink.

### Einzelruf meldet „Dienst nicht verfügbar“

```bash
journalctl -u tetra.service -n 500 --no-pager \
  | grep -E 'U-SETUP P2P|called ISSI|routing.*Brew|RequestedServiceNotAvailable|rejecting|collision|Congestion'
```

Online müssen beide ISSIs zentral freigegeben und aktuell registriert sein. Im Fallback wird die Ziel-ISSI lokal gepaged.

### Gruppe wird abgelehnt

```bash
journalctl -u tetra.service -n 500 --no-pager \
  | grep -E 'central group policy|rejected affiliation|groups=|GSSI'
```

## 18. Update des Provisioning Core

Im Provisioning-LXC:

```bash
cd /opt/netcore-tetra
git fetch origin
git checkout feature/provisioning-core
git pull --ff-only origin feature/provisioning-core
sudo bash system-backend/provisioning-core/install/update.sh
```

Status:

```bash
systemctl status netcore-provisioning-core.service --no-pager
curl -fsS http://127.0.0.1:8125/health/ready | jq .
```

## 19. Sicherheitsgrenze der aktuellen Testphase

Der Provisioning Core läuft derzeit bewusst im **OPEN-LAB-Modus**:

- keine Anmeldung
- keine Tokens
- kein TLS
- vollständiger Schreibzugriff auf Subscriber Core und Group Core

Port 8125 deshalb ausschließlich im isolierten Test-/Managementnetz bereitstellen und nicht aus dem Internet veröffentlichen.
