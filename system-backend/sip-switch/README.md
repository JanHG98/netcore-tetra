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

## MAIN-COMPAT-Netzanbindung

Installations- und Update-Reihenfolge pro LXC/TBS, aktuelle Funktionsgrenzen und Tests:
[Zentraler Netzbetrieb](../../Docs/CENTRAL_NETWORK_ROLLOUT.md).
