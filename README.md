### MQTT branch Phase 11b

Die GPS-basierte Warnzentrale für NINA/KATWARN und eigene Kartenwarnungen liegt in `system-backend/alert-service`. Die [Schritt-für-Schritt-Anleitung pro LXC und TBS](Docs/KATWARN_NINA_INSTALL_UPDATE.md) zeigt, welcher LXC aktualisiert oder neu erstellt werden muss, welche Befehle dort auszuführen sind und was an jeder TBS zu prüfen ist.

`system-backend/sip-switch` ergänzt den zentralen SIP-Router zwischen dem vorhandenen PBX und allen TETRA-Basisstationen. PBX→TETRA-Rufe werden per Mobility Core zur aktuellen Serving-TBS geroutet; der TETRA-Codecpfad bleibt auf der jeweiligen TBS (`edge_media`).

Neu in Phase 11b: Jede TBS behält einen lokalen Asterisk als Edge-B2BUA. Die native TBS-Bridge spricht dauerhaft nur mit diesem lokalen Asterisk. Dieser nutzt den zentralen SIP-Switch als Primärweg und das vorhandene PBX als direkten Fallback. Dadurch ist kein TBS-Neustart nötig, wenn der zentrale SIP-Switch ausfällt oder zurückkommt.
