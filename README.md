### MQTT branch Phase 11b

`system-backend/sip-switch` ergänzt den zentralen SIP-Router zwischen dem vorhandenen PBX und allen TETRA-Basisstationen. PBX→TETRA-Rufe werden per Mobility Core zur aktuellen Serving-TBS geroutet; der TETRA-Codecpfad bleibt auf der jeweiligen TBS (`edge_media`).

Neu in Phase 11b: Jede TBS behält einen lokalen Asterisk als Edge-B2BUA. Die native TBS-Bridge spricht dauerhaft nur mit diesem lokalen Asterisk. Dieser nutzt den zentralen SIP-Switch als Primärweg und das vorhandene PBX als direkten Fallback. Dadurch ist kein TBS-Neustart nötig, wenn der zentrale SIP-Switch ausfällt oder zurückkommt.
