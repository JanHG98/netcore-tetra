# NetCore SIP Switch – Phase 11c

Der zentrale SIP-Switch ist im Normalbetrieb die **einzige Vermittlungsstelle zwischen NetCore-TETRA und dem vorhandenen PBX**. Die lokale TBS behält trotzdem ihren eigenen Asterisk als Edge-B2BUA und Notvermittlung.

## Normalbetrieb – „Dame vom Amt“

```text
TETRA-Funkgerät
      │
      ▼
Native TBS-SIP-/Codec-Bridge
      │ 127.0.0.1:5060
      ▼
Lokaler TBS-Asterisk
      │ einzige aktive externe Registrierung
      ▼
Zentraler NetCore SIP-Switch
      │ einziger normaler PBX-Trunk
      ▼
vorhandenes PBX
```

Die direkte Registrierung der TBS am PBX ist im Normalbetrieb **nicht geladen**. Das PBX sieht daher nicht dieselbe TBS gleichzeitig über den Switch und direkt.

## Bestätigter Zentral-Ausfall

Eine lokale State Machine überwacht den zentralen SIP-Switch. Standardmäßig müssen drei Prüfungen fehlschlagen. Erst danach wird atomar umgeschaltet:

```text
CENTRAL_ACTIVE
  → FAILOVER_PENDING
  → PBX_DIRECT_ACTIVE
```

Dabei wird die zentrale Registrierung entfernt und erst anschließend die direkte PBX-Registrierung geladen. Der direkte Dialplan ist zusätzlich über `AstDB(netcore/failover_mode)` gesperrt und wird nur in `pbx_direct` freigegeben.

## Rückkehr des zentralen Switches

Der zentrale Switch muss standardmäßig 30 Sekunden stabil erreichbar sein. Danach:

```text
PBX_DIRECT_ACTIVE
  → RECOVERY_PENDING
  → direkte PBX-Registrierung abmelden
  → zentrale Registrierung laden
  → CENTRAL_ACTIVE
```

So gibt es immer nur **eine absichtlich aktive externe Registrierung pro TBS**. Nach einem harten Ausfall kann ein alter SIP-Kontakt im PBX noch bis zum Ablauf seiner kurzen Registrierungszeit sichtbar sein; er wird aber nicht mehr von der TBS aktiv gehalten.

## Kein TBS-Neustart beim Failover

Die native TBS-Bridge bleibt dauerhaft auf `127.0.0.1:5060`. Nur der lokale Asterisk schaltet seine externe Registrierung um. Laufende Gespräche werden nicht mitten im Dialog verschoben; der neue Weg gilt für neue Rufe.

## OPEN LAB

Keine WebUI-Anmeldung, keine API-Tokens und kein TLS. SIP-Zugangsdaten liegen im Klartext in der Laborkonfiguration. Ausschließlich im isolierten Testnetz verwenden.

## Installation

- Zentraler LXC: [`docs/installation-openlab.md`](docs/installation-openlab.md)
- Lokale TBS: [`tbs-fallback/docs/installation-openlab.md`](tbs-fallback/docs/installation-openlab.md)

## SIP-Routing: T-Marker und AGI-Skriptpfad

Eingehende PBX-Ziele `5102`, `T5102` und `t5102` werden als ISSI `5102`
vermittelt. Der zentrale Dialplan und der direkte PBX-Fallback nehmen den
optionalen T-Marker an und entfernen ihn vor den bestehenden numerischen
Prefix-/Mapping-Regeln. `tetra_number_prefix` und `fallback_tetra_prefix`
bleiben numerische Wahlpräfixe; für `T5102` allein ist kein Präfix einzutragen.
Ungültige Ziele wie `T51x02` werden nicht durch Entfernen beliebiger Buchstaben
in eine andere Teilnehmernummer umgewandelt.

Wenn Asterisk `extension 'T5102' ... not found in context 'netcore-from-pbx'`
meldet, wurde der Ruf vor dem Routing-Skript abgewiesen. Deshalb können die
Routen-/Rufzähler trotz funktionierender SIP-Registrierungen bei null bleiben.

Bei `Failed to execute '/usr/share/asterisk/agi-bin/netcore-sip-route.py': File
does not exist` sucht der Debian-Asterisk den bisherigen relativen Skriptnamen
im falschen Verzeichnis. Der Installer legt das Skript unter
`/var/lib/asterisk/agi-bin/netcore-sip-route.py` ab. Der Renderer verwendet dafür
jetzt den absoluten Pfad, auch bei vorhandenen Konfigurationen mit
`agi_script = "netcore-sip-route.py"`. Individuell konfigurierte andere
Skriptpfade bleiben erhalten. Dieser Fehler verhindert sowohl ausgehendes
Routing als auch eingehendes Routing nach erfolgreicher Nummernerkennung.

Nach Übernahme des Fixes auf dem **SIP-LXC** ausführen:

```bash
cd /opt/netcore-tetra
git pull --ff-only origin mqtt
bash system-backend/sip-switch/install/update.sh
asterisk -rx 'dialplan show T5102@netcore-from-pbx'
asterisk -rx 'dialplan show netcore-from-tbs'
```

Für dieselbe Nummernunterstützung bei einem späteren zentralen Ausfall auf der
**TBS** aktualisieren:

```bash
cd /opt/netcore-tetra
git pull --ff-only origin mqtt
bash system-backend/sip-switch/install/update-tbs-local-fallback.sh
```

Es ist kein Rust-Neubau erforderlich. Die Funksoftware muss für den Rufversuch
laufen und als nativer Kontakt im lokalen Asterisk erscheinen. Die externe
TBS-Registrierung am zentralen Switch bestätigt diesen lokalen Kontakt nicht.
Bei einem erneuten Test müssen die AGI-Aufrufe den absoluten Pfad verwenden und
eine `NetCore route ...`-Antwort liefern. Erst ein Rufversuch mit Audio in beiden
Richtungen bestätigt die vollständige Telefonie; diese Regressionstests prüfen
Nummernauflösung, Dialplan und AGI-Anbindung ohne reale Funk-/SIP-Hardware.

## Teilnehmer registriert, aber `serving_tbs_not_configured`

Dieser Routingfehler bedeutet, dass der Mobility-Core eine bedienende Node meldet,
die keiner konfigurierten TBS zugeordnet ist. Eine SIP-Registrierung allein stellt
diese Zuordnung nicht her. Beispielsweise sind `SRV-M_TBS-01` und `SRV-M-TBS-01`
unterschiedliche IDs.

Auf dem **SIP-LXC** in `/etc/netcore/sip-switch.toml` beim bestehenden
`[[tbs]]`-Eintrag mit `node_id = "SRV-M-TBS-01"` den gemeldeten Namen in
`aliases` aufnehmen:

```toml
aliases = ["SRV-M_TBS-01"]
```

Vorhandene Aliase in derselben Liste erhalten; keinen zweiten `aliases`-Schlüssel
anlegen. `endpoint_id`, `username` und Zugangsdaten bleiben erhalten. Diese
explizite Zuordnung setzt voraus, dass beide IDs dieselbe Basisstation bezeichnen.
Es gibt bewusst keine pauschale Ersetzung von Unterstrichen durch Bindestriche.

Nach der Konfigurationsänderung nur den Routingdienst neu starten. Im Beispiel
lauscht der SIP-Switch auf `10.0.1.125:8300`; bei anderer Adresse diese ersetzen:

```bash
systemctl restart netcore-sip-switch.service
curl -sS --retry 3 --retry-connrefused --max-time 10 \
  'http://10.0.1.125:8300/api/v1/resolve?direction=inbound&number=5102&check_contact=true' \
  | python3 -m json.tool
```

Bei weiterhin registrierter ISSI und erreichbarer TBS muss `action` auf `tbs`
wechseln und `endpoint` den vorhandenen TBS-Endpunkt nennen. Asterisk und die
Funkbasis brauchen für diese Alias-Ergänzung keinen Neustart.

## Anruferkennung zeigt den TBS-Trunk statt der ISSI

Die native TBS-Bridge sendet die anrufende ISSI bereits als
`P-Asserted-Identity` (PAI). Der lokale Asterisk muss diese Identität annehmen und
über seinen zentralen beziehungsweise direkten PBX-Trunk weitergeben. Der
Fallback-Renderer setzt deshalb für die Endpunkte der nativen TBS, des zentralen
Switches und des PBX `trust_id_inbound=yes` und `send_pai=yes`. Damit können auch
eingehende Telefon-Anrufernummern bis zur nativen Bridge übertragen werden.
Die SIP-Kontonamen in `from_user` und den Authentifizierungs-/Registrierungsobjekten
bleiben erhalten; sie identifizieren den Trunk. Der zentrale SIP-Switch verwendet
diese PAI-Optionen bereits.

Nach Übernahme des Fixes auf der **TBS** ausführen:

```bash
cd /opt/netcore-tetra
git pull --ff-only origin mqtt
bash system-backend/sip-switch/tbs-fallback/install/update-tbs-local-fallback.sh
asterisk -rx 'module show like res_pjsip_caller_id.so'
```

Das Update rendert die lokale Konfiguration neu und lädt sie in Asterisk.
Ein Rust-Neubau oder Neustart der Funkbasis ist dafür nicht erforderlich.
Das Modul `res_pjsip_caller_id.so` muss geladen sein. Anschließend einen neuen
Ruf, etwa ISSI `5102` zur Telefon-Nebenstelle `103`, testen: Der zentrale
Routing-Log sollte nun `caller=5102` beziehungsweise das AGI-Argument `5102`
anstelle des TBS-Kontonamens zeigen. Die PBX muss PAI am NetCore-Trunk ebenfalls
annehmen, damit das Telefon die Nummer anzeigt. Bei korrekter ISSI im zentralen
Routing und weiterhin falscher Anzeige ist diese PBX-Einstellung zu prüfen.
Die automatischen Tests prüfen die gerenderte Konfiguration und die
Node-Zuordnung; den Ruf mit Anzeige und Audio in beiden Richtungen am Netz testen.

## Telefon ruft das Funkgerät: CLI kann nicht angezeigt werden

Die Funkbasis muss die Identität des lokalen Telefon-Gateways und die externe
Anrufernummer getrennt signalisieren. Bisher konnte eine SIP-Nebenstelle wie `103`
zusätzlich als TETRA-Anrufer-SSI `103` im D-SETUP erscheinen. Das passt nicht zu
einem PABX-Profil, dessen eingehende Gateway-SSI beispielsweise `16777184` ist.

Für Asterisk-Rufe verwendet die Funkbasis deshalb `asterisk.inbound_gateway_issi`
als Calling Party SSI. Der Standardwert ist `16777184`; er stammt aus dem
vorliegenden Codeplug und ist **kein universell vorgeschriebener SIP-Gatewaywert**.
Eine abweichende Gateway-SSI lässt sich im bestehenden `[asterisk]`-Abschnitt
der TBS-Konfiguration setzen:

```toml
inbound_gateway_issi = 16777184
```

Für einen Anruf von Telefon `103` nach ISSI `5102` ergibt sich:

| D-SETUP-Feld | Wert |
|---|---|
| Calling Party SSI | `16777184` |
| Calling Party Extension | nicht vorhanden: Gateway im lokalen TETRA-Netz |
| External Subscriber Number | `103` (12 Bit, `0x103`) |

Die interne Kennzeichnung des externen Rufursprungs bleibt erhalten. Die
Telefonnummer wird nicht als MCC/MNC-Erweiterung interpretiert. Bestehende
TETRA-Quellidentitäten aus Brew bleiben erhalten.

Der Codeplug unterscheidet **Incoming Identity** und **Outgoing Identity**.
Für die eingehende CLI ist die erste relevant; die ausgehende Identität und
Wahlpräfixe werden durch diesen Fix nicht geändert. Der im vorliegenden CPS
angezeigte MCC `1000` wird nicht auf die Luftschnittstelle übernommen:
EN 300 392-1 V1.6.1, Abschnitt 7.2.5, reserviert MCC 1000 bis 1023.
Die Bedeutung dieses Werts in der Programmiersoftware muss anhand der
Herstellerdokumentation geprüft werden. Der Fix verwendet eine lokale
Gateway-ISSI gemäß Abschnitt 7.2.6 und eine separate externe Nummer gemäß
Abschnitt 7.8.2.2.2. Die tatsächliche Anzeige am Gerät muss im Funkversuch
bestätigt werden.

Nach Übernahme des Fixes **nur auf der TBS** aktualisieren und neu bauen:

```bash
cd /opt/netcore-tetra
git pull --ff-only origin mqtt
cargo build --release -p bluestation-bs
```

Nach erfolgreichem Build die bisher manuell gestartete Basis beenden und mit
derselben Konfiguration neu starten:

```bash
./target/release/bluestation-bs ./config.toml
```

Bestehende lokale Konfiguration erhalten. Fehlt `inbound_gateway_issi`, wird
automatisch `16777184` verwendet. Für diesen Rust-Fix ist kein Update oder
Neustart des zentralen SIP-Switch-LXC oder des lokalen Asterisk erforderlich.
Bei einem neuen Anruf von `103` muss das TBS-Log `number='103'` und
`display_ssi=Some(16777184)` zeigen; auf DEBUG zusätzlich D-SETUP mit der
externen Nummer. Danach Anzeige, Gesprächsannahme und Rückruf am Funkgerät prüfen.

### Diagnose: vollständige Gateway-TSI

Beim getesteten Sepura SC20 mit V10.24 funktioniert die interne Einzelruf-CLI,
während die externe Telefon-CLI trotz getrennter Gateway-SSI und Telefonnummer
noch nicht angezeigt wird. Die geprüften Herstellerunterlagen belegen weder
einen Firmwarefehler noch eine Pflicht zur vollständigen TSI. Die folgende
Option ist deshalb ein **unbestätigter Kompatibilitätstest**, kein nachgewiesener
Fix für die Meldung „cannot display CLI“.

Standardmäßig bleibt `inbound_gateway_full_tsi = false`: nur die lokale
Gateway-SSI wird übertragen (Calling Party Type Identifier 1). Mit `true`
überträgt die TBS dieselbe Identität als vollständige TSI (Typ 2) mit MCC und MNC
aus `[net_info]`. Für das Testnetz ist das:

| Feld | Standard | Testoption aktiv |
|---|---|---|
| Calling Party SSI | `16777184` | `16777184` |
| Calling Party Extension | keine | `901/1510`, kodiert als `14763494` (`0xE145E6`) |
| External Subscriber Number | `103` | `103` |

Die Option gilt nur für eingehende Asterisk-Rufe. Interner Rufbesitzer, Ziele,
Brew-/EchoLink-Identitäten, Rufnummer und Hook-Methode ändern sich nicht. Die
Konfiguration wird bei aktivierter Option abgewiesen, wenn MCC außerhalb 0–999
oder MNC außerhalb 0–16383 liegt; CPS-Platzhalter wie MCC 1000 werden nicht
gesendet. Hintergrund: EN 300 392-1 V1.6.1 §§7.2.5–7.2.6 und
EN 300 392-2 V3.8.1, D-SETUP Tabelle 14.15.

**Nur auf SRV-M-TBS-01 (10.0.1.20), nach Übernahme dieser Änderung in `mqtt`:**

1. Bestehende lokale Konfiguration sichern, Branch aktualisieren und bauen:

   ```bash
   cd /opt/netcore-tetra
   cp -p config.toml "config.toml.before-cli-tsi-$(date +%Y%m%d-%H%M%S)"
   git pull --ff-only origin mqtt
   cargo build --release -p bluestation-bs
   ```

   Lokale Änderungen bei einem Git-Konflikt erhalten; kein Reset der Konfiguration.

2. Im **bestehenden** `[asterisk]`-Abschnitt der tatsächlich gestarteten
   `config.toml` ergänzen (keinen zweiten Abschnitt anlegen):

   ```toml
   inbound_gateway_full_tsi = true
   ```

3. Die manuell laufende Basis beenden und mit derselben Konfiguration neu starten:

   ```bash
   ./target/release/bluestation-bs ./config.toml
   ```

4. Erneut von Telefon `103` zu HRT `5102` anrufen. Das INFO-Log muss
   `number='103' display_ssi=Some(16777184) display_extension=Some(14763494)`
   zeigen. CLI, Rufannahme und Sprache prüfen. Zeigt es `display_extension=None`,
   ist die Testoption im laufenden Prozess nicht aktiv. Codeplug und SIP-Switch
   für diesen Vergleich unverändert lassen.

5. **Rücksetzen:** `inbound_gateway_full_tsi = false` setzen oder die Zeile
   entfernen und die Basis erneut starten. Ein weiterer Build ist nicht nötig.

Für diesen Test sind keine Installation, kein Update und kein Neustart auf
SIP-Switch-, Mobility- oder anderen LXC-Systemen erforderlich. Auch der lokale
Asterisk muss nicht neu gestartet werden. Bleibt die Anzeige unverändert,
ist die vollständige Gateway-TSI als alleinige Abhilfe nicht bestätigt.

## MAIN-COMPAT-Netzanbindung

Installations- und Update-Reihenfolge pro LXC/TBS, aktuelle Funktionsgrenzen und Tests:
[Zentraler Netzbetrieb](../../Docs/CENTRAL_NETWORK_ROLLOUT.md).
