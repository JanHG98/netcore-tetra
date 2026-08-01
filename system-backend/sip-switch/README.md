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
