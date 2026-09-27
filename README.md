# NetCore-TETRA

**v1.9.0 · NINA/KATWARN, eigene Warnmeldungen und Funk-/SDS-Korrekturen**

Die Warnfunktionen aus `katwarn/nina` sind in `main` integriert. Eigene Meldungen bleiben nach dem Senden auffindbar und können vor Ablauf gelöscht werden. Die neuere separate Brew-Server-Version bleibt enthalten; bestehende Python-Brew-Installationen können weiterverwendet werden.

[Release-Hinweise](Docs/RELEASE_v1.9.0.md) · [NINA/KATWARN installieren und aktualisieren](Docs/KATWARN_NINA_INSTALL_UPDATE.md)

### Zentrale SIP-Anbindung und lokaler Fallback

Die GPS-basierte Warnzentrale für NINA/KATWARN und eigene Kartenwarnungen liegt in `system-backend/alert-service`. Die [Schritt-für-Schritt-Anleitung pro LXC und TBS](Docs/KATWARN_NINA_INSTALL_UPDATE.md) zeigt, welcher LXC aktualisiert oder neu erstellt werden muss, welche Befehle dort auszuführen sind und was an jeder TBS zu prüfen ist.

`system-backend/sip-switch` ergänzt den zentralen SIP-Router zwischen dem vorhandenen PBX und allen TETRA-Basisstationen. PBX→TETRA-Rufe werden per Mobility Core zur aktuellen Serving-TBS geroutet; der TETRA-Codecpfad bleibt auf der jeweiligen TBS (`edge_media`).

Neu in Phase 11b: Jede TBS behält einen lokalen Asterisk als Edge-B2BUA. Die native TBS-Bridge spricht dauerhaft nur mit diesem lokalen Asterisk. Dieser nutzt den zentralen SIP-Switch als Primärweg und das vorhandene PBX als direkten Fallback. Dadurch ist kein TBS-Neustart nötig, wenn der zentrale SIP-Switch ausfällt oder zurückkommt.
### Auto Discovery & Deployment (Feature-Branch)

Der Branch `feature/openlab-discovery-deployment` ergänzt eine zentrale
Ubuntu-VM mit HTTP-WebUI auf Port 8320, Pi-Imagebuilder, Discovery-Agenten, Commit-Abgleich
und TBS-Provisionierung. Installation und Betriebsgrenzen:
[Deployment & Auto Discovery](system-backend/deployment-core/README.md).
