# Phase 11c – lokalen TBS-SIP-Fallback installieren

Der lokale Asterisk hält im Normalbetrieb ausschließlich die Registrierung zum zentralen SIP-Switch. Eine direkte PBX-Registrierung wird erst nach bestätigtem Ausfall aktiviert.

## 1. TBS im zentralen SIP-Switch anlegen

```bash
cd /opt/netcore-tetra/system-backend/sip-switch

./install/add-tbs-openlab.sh \
  SRV-M-TBS-01 \
  tbs-srv-m-tbs-01 \
  openlab-central
```

## 2. Lokalen Asterisk installieren

Ohne PBX-Authentisierung im OPEN LAB:

```bash
./install/install-tbs-local-fallback.sh \
  SRV-M-TBS-01 \
  IP-DER-TBS \
  IP-DES-SIP-SWITCHES \
  tbs-srv-m-tbs-01 \
  openlab-central \
  IP-DES-PBX \
  netcore-tbs-01
```

Mit PBX-Benutzer und Kennwort:

```bash
./install/install-tbs-local-fallback.sh \
  SRV-M-TBS-01 IP-DER-TBS IP-DES-SIP-SWITCHES \
  tbs-srv-m-tbs-01 openlab-central \
  IP-DES-PBX netcore-tbs-01 \
  PBX-AUTH-USER PBX-PASSWORT
```

Installiert werden unter anderem:

```text
/etc/netcore/tbs-sip-fallback.toml
/etc/asterisk/netcore-tbs-fallback-pjsip.conf
/etc/asterisk/netcore-registration-central.conf
/etc/asterisk/netcore-registration-pbx-direct.conf
/etc/asterisk/netcore-active-registration.conf
netcore-tbs-sip-failover.service
```

`netcore-active-registration.conf` enthält immer nur **einen** Include.

Normal:

```ini
#include netcore-registration-central.conf
```

Fallback:

```ini
#include netcore-registration-pbx-direct.conf
```

## 3. Native TBS-Bridge lokal anbinden

```bash
./install/apply-tbs-local-asterisk-config.sh \
  --config /etc/netcore/config.toml
```

Danach zeigt die TBS dauerhaft auf:

```toml
remote_host = "127.0.0.1"
remote_port = 5060
```

Die TBS einmalig neu starten. Spätere Umschaltungen benötigen keinen TBS-Neustart.

## 4. PBX vorbereiten

Normalerweise existiert im PBX nur der zentrale NetCore-Trunk. Zusätzlich wird pro TBS eine Fallback-Registrierung beziehungsweise ein Fallback-Endpunkt vorbereitet. Dieser Kontakt erscheint erst, wenn die TBS nach bestätigtem Ausfall direkt registriert.

Die direkte PBX-Route darf nur für den Fallbackkontakt verwendet werden. Sie ist nicht der normale parallele Rufweg.

## 5. Zustand prüfen

```bash
systemctl status netcore-tbs-sip-failover --no-pager --full
journalctl -u netcore-tbs-sip-failover -f
./install/tbs-fallback-status.sh | python3 -m json.tool
```

Im Normalbetrieb muss gelten:

```text
mode: central
central_registration: registered
pbx_direct_registration: absent
```

Im Fallback:

```text
mode: pbx_direct
central_registration: absent
pbx_direct_registration: registered
```

## 6. Failover testen

Zentralen SIP-Switch stoppen. Nach drei fehlgeschlagenen Prüfungen im Abstand von zwei Sekunden wechselt die TBS auf `PBX_DIRECT_ACTIVE`.

Nach Wiederkehr muss der Switch 30 Sekunden stabil erreichbar sein. Der Agent meldet anschließend die direkte PBX-Registrierung ab und aktiviert wieder ausschließlich die zentrale Registrierung.

Manuelle Lab-Umschaltung:

```bash
/usr/local/bin/netcore-tbs-sip-fallback --force-mode pbx_direct
/usr/local/bin/netcore-tbs-sip-fallback --force-mode central
```
