# Installation – NetCore SIP Switch (OPEN LAB)

Der SIP-Switch-LXC ersetzt **nicht** das vorhandene PBX. Das PBX behält Nebenstellen, DECT, Rufgruppen, Rufnummernplan und Telefoniefunktionen. Der neue LXC besitzt genau einen PBX-Trunk und nimmt die Registrierungen der **lokalen TBS-Asterisk-Instanzen** entgegen. Die native TBS-SIP-/Codec-Bridge registriert sich nur am lokalen Asterisk und behält damit einen direkten PBX-Rückfallweg.

## 1. LXC vorbereiten

Empfohlen ist ein eigener Debian-/Ubuntu-LXC mit erreichbarer Management-IP. Benötigte Ports:

- `8300/tcp` – WebUI/API
- `5060/udp` oder `5060/tcp` – SIP
- `10000-20000/udp` – RTP

Repository nach `/opt/netcore-tetra` übertragen und installieren:

```bash
cd /opt/netcore-tetra/system-backend/sip-switch
find install -type f -name '*.sh' -exec chmod 755 {} +
./install/install.sh
```

## 2. PBX, Mobility Core und MQTT eintragen

```bash
./install/configure-openlab.sh \
  PBX-IP \
  MOBILITY-CORE-IP \
  MQTT-BROKER-IP \
  registration
```

Phase 11c verwendet damit im Normalbetrieb eine einzelne PBX-Registrierung des zentralen SIP-Switches. `ip_trunk` bleibt nur als Legacy-Lab-Option erhalten.

Danach in `/etc/netcore/sip-switch.toml` unter `[pbx]` Benutzername, Auth-Benutzer und Kennwort ergänzen. Anschließend:

```bash
/opt/netcore-tetra/system-backend/sip-switch/install/render-asterisk.sh
```

## 3. Jede TBS im SIP Switch anlegen

```bash
./install/add-tbs-openlab.sh \
  SRV-M-TBS-01 \
  tbs-srv-m-tbs-01 \
  OPENLAB-KENNWORT
```

Der erste Parameter muss exakt dem `serving_node` entsprechen, den der Mobility Core für diese Basisstation zurückgibt. Aliase können später im jeweiligen `[[tbs]]`-Block ergänzt werden.

## 4. Lokalen TBS-Asterisk mit Fallback installieren

Der empfohlene Phase-11c-Weg ist:

```text
Native TBS-Bridge → lokaler Asterisk → zentraler SIP-Switch
                                  └→ direkter PBX-Fallback
```

Auf jeder TBS:

```bash
./install/install-tbs-local-fallback.sh \
  SRV-M-TBS-01 \
  TBS-LOKALE-IP \
  SIP-SWITCH-IP \
  tbs-srv-m-tbs-01 \
  OPENLAB-KENNWORT \
  PBX-IP \
  netcore-tbs-01
```

Danach die native TBS-Konfiguration mit Sicherung auf den lokalen Asterisk umstellen:

```bash
./install/apply-tbs-local-asterisk-config.sh \
  --config /etc/netcore/config.toml
```

Die Basisstation muss anschließend nur einmal neu gestartet werden. Bei späterem Ausfall oder Wiederkehr des zentralen SIP-Switches bleibt die TBS aktiv; der lokale Asterisk wechselt den Trunk für neue Rufe automatisch.

Der frühere Direktmodus über `print-tbs-config.sh` bleibt als Legacy-Testpfad erhalten, ist aber nicht mehr die empfohlene Architektur.

## 5. Einen Trunk im vorhandenen PBX anlegen

Das PBX benötigt einen Primärtrunk zum SIP-Switch-LXC:

```text
Ziel: SIP-SWITCH-IP
Port: 5060
Transport: UDP oder TCP entsprechend sip-switch.toml
Codec: PCMU / ulaw
```

Den gewünschten ISSI-Rufnummernbereich auf diesen Primärtrunk routen. Zusätzlich wird pro TBS ein Fallback-Endpunkt vorbereitet. Die lokale TBS registriert sich dort erst nach bestätigtem Ausfall des zentralen SIP-Switches; im Normalbetrieb existiert dieser direkte Kontakt nicht aktiv. Das Nummernformat wird unter `[routing]` und optional mit `[[number_mappings]]` festgelegt.

## 6. Prüfen

```bash
systemctl status asterisk --no-pager --full
systemctl status netcore-sip-switch --no-pager --full
ss -lntup | grep -E ':5060|:8300|:10000'
asterisk -rx 'pjsip show contacts'
asterisk -rx 'pjsip show endpoints'
```

Managementstatus:

```bash
curl -fsS http://SIP-SWITCH-IP:8300/api/v1/status | python3 -m json.tool
```

Route ohne Anruf testen:

```bash
curl -fsS \
  'http://SIP-SWITCH-IP:8300/api/v1/resolve?direction=inbound&number=4010001&check_contact=true' \
  | python3 -m json.tool
```

Erwartet sind unter anderem:

```json
{
  "action": "tbs",
  "issi": 4010001,
  "node_id": "SRV-M-TBS-01",
  "endpoint": "tbs-srv-m-tbs-01",
  "aor": "tbs-srv-m-tbs-01"
}
```

## 7. Fallback-Regel im PBX

Der direkte TBS-Weg bleibt dauerhaft erhalten, darf aber nicht parallel zum gesunden Zentraltrunk gewählt werden. Die PBX-Route muss deshalb eine echte Reihenfolge verwenden: **zentraler SIP-Switch zuerst**, direkte TBS-Fallbacktrunks erst bei `CHANUNAVAIL`/Trunkfehler. Ein paralleler Normalbetrieb würde doppelte Rufe erzeugen.

## Aktuelle Mediengrenze

Phase 11 verwendet `edge_media`: Asterisk verankert SIP/RTP zentral, die TETRA↔PCMU-Umsetzung bleibt aber im vorhandenen Connector der ausgewählten TBS. Ein neuer Ruf wird nach jedem Mobility-Core-Lookup zur aktuellen Serving-TBS geschickt. Ein **bereits laufender** Ruf wird bei einem späteren Zellwechsel noch nicht unterbrechungsfrei auf eine andere TBS verschoben.
