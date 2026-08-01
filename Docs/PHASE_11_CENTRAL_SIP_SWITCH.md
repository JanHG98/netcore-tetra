# Phase 11 – Zentraler NetCore SIP Switch

Phase 11 ergänzt `system-backend/sip-switch/` auf Port `8300`. Das bestehende PBX bleibt unverändert die Telefonanlage. Der neue LXC bündelt einen PBX-Trunk und die SIP-Registrierungen aller TBS, fragt bei PBX→TETRA-Rufen den Mobility Core ab und wählt nur den Connector der aktuellen Serving-TBS.

Die erste Ausbaustufe verwendet bewusst `edge_media`: Der vorhandene SIP-/RTP-/TETRA-Codec der ausgewählten Basisstation bleibt Medienendpunkt. Damit ist die zentrale Mehr-TBS-Rufzustellung bereits nutzbar, ohne den Codec zu duplizieren. Unterbrechungsfreies Live-Handover eines bestehenden SIP-Rufs folgt erst mit externen Call-Legs und zentralem Media-Switch-Codec.

## Ports

- Management/WebUI: `8300/tcp`
- SIP: `5060/udp` oder `5060/tcp`
- RTP: `10000-20000/udp`

## Keine zweite PBX

Asterisk wird nur als Registrar, B2BUA und SIP-/RTP-Anker betrieben. Nebenstellen, Rufgruppen, DECT, IVR und klassische Telefoniefunktionen verbleiben vollständig im bereits vorhandenen PBX.
