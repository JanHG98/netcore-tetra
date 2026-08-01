# NetCore SIP Switch – Phase 11

Der SIP Switch zentralisiert die SIP-Verwaltung für mehrere TETRA-Basisstationen, ohne eine zweite Telefonanlage aufzubauen. Das vorhandene PBX bleibt zuständig für Nebenstellen, DECT, Rufgruppen, Rufnummernplan und Telefoniefunktionen. Der SIP-Switch-LXC verwendet Asterisk ausschließlich als SIP-B2BUA, Registrar und Routing-/Media-Anker.

## Datenweg der ersten Ausbaustufe

```text
vorhandenes PBX
      │ ein Trunk
      ▼
NetCore SIP Switch
      │ Mobility Core: Serving-TBS der Ziel-ISSI
      ├──────────────► TBS-01 SIP-Connector
      ├──────────────► TBS-02 SIP-Connector
      └──────────────► TBS-03 SIP-Connector
```

Jede TBS registriert ihren bereits vorhandenen lokalen Asterisk-/RTP-Connector nicht mehr direkt am PBX, sondern am SIP Switch. Ein eingehender PBX-Ruf wird anhand der ISSI über den Mobility Core aufgelöst und nur an den SIP-Endpunkt der aktuellen Serving-TBS geschickt. Von der TBS kommende Telefonierufe werden über den einzigen PBX-Trunk weitergereicht.

## Bewusste Grenze dieser Phase

Der Medienmodus heißt `edge_media`: TETRA-ACELP↔PCMU bleibt zunächst im vorhandenen TBS-Connector. Der SIP Switch verankert SIP und RTP zentral in Asterisk, aber ein bereits laufender SIP-Ruf wird bei einem Zellwechsel noch nicht unterbrechungsfrei auf eine andere TBS verschoben. Dafür werden später ein echtes externes Call-Leg im Call Control und ein zentraler Codecpfad am Media Switch benötigt.

Diese Grenze ist sichtbar in `/api/v1/status`:

```json
{
  "media_mode": "edge_media",
  "central_media_ready": false
}
```

## OPEN LAB

- WebUI/API ohne Anmeldung, Token oder TLS auf Port `8300`
- SIP auf Port `5060`
- RTP standardmäßig `10000-20000/udp`
- TBS-SIP-Kennwörter und ein mögliches PBX-Kennwort liegen im Klartext in der Laborkonfiguration
- ausschließlich in einem isolierten Testnetz einsetzen

## API

- `GET /health/live`
- `GET /health/ready`
- `GET /api/v1/status`
- `GET /api/v1/tbs`
- `GET /api/v1/mappings`
- `GET /api/v1/calls`
- `GET /api/v1/decisions`
- `GET /api/v1/events`
- `GET|POST /api/v1/resolve`
- `POST /api/v1/calls/{token}/state`
- `POST /api/v1/actions/render-asterisk`
- `POST /api/v1/actions/reload-asterisk`
- `GET /metrics`

## Ereignisse

- `sip.route_resolved`
- `sip.route_failed`
- `sip.call_started`
- `sip.call_answered`
- `sip.call_ended`
- `sip.call_failed`
- `sip.tbs_contact_up`
- `sip.tbs_contact_down`
- `sip.asterisk_config_rendered`
- `sip.asterisk_reloaded`
- `sip.asterisk_reload_failed`
