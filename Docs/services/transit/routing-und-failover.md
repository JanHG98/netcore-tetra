# Transit – Routing und Failover

Transit ermittelt Zielregionen aus explizitem Ziel, Teilnehmerregion oder Gruppenreichweite und wertet passende Routen nach Dienst und Selector aus. Unterstützte Selectors sind `default`, `region`, `issi`, `gssi` und `prefix`.

## Auswahlreihenfolge

Die vorhandene Sortierung ist:

1. höhere Präferenz;
2. niedrigere Metrik;
3. niedrigere gemessene Latenz;
4. höhere Peer-Priorität.

Bei `prefer_direct_region_peer=true` kommen direkte Peers zusätzlich als Kandidaten mit Präferenz `10000 + peer.priority` und Metrik 0 hinzu. Sie sind damit normalerweise bevorzugt, aber **nicht** bedingungslos vor einer expliziten Route mit noch höherer Präferenz. Doppelte Peer-Kandidaten werden nach der Sortierung entfernt.

Ein Peer muss administrativ `enabled` sein und den Betriebszustand `up`, `degraded` oder `unknown` haben. Peers mit anderer Protokollversion, unpassenden nichtleeren Capabilities oder bereits im Trace enthaltener Region scheiden aus. Auch deaktivierte und abgelaufene Routen werden ausgeschlossen.

## Failover und Zustellung

Jedes Session-Leg hält den ausgewählten Peer und Backups für seine Zielregion. Ein Peer-Ausfall oder Transportfehler kann ausstehende Envelopes auf einen brauchbaren Backup-Peer umstellen. Ein kontrollierter Session-Failover benötigt ein noch failoverfähiges Outbound-Envelope; ein bereits abgearbeiteter Auftrag wird dadurch nicht nachträglich migriert.

Ohne passenden Backup-Pfad schlägt der betroffene Zustellpfad fehl. Ein Gruppenruf mit mehreren regionalen Legs kann teilweise erfolgreich sein. TTL, Versuchslimit und der Peer-Zustand begrenzen weitere Versuche; die persistente Queue bedeutet keine unbegrenzte Zustellgarantie.

## Loop- und Dedupe-Verhalten

Ingress lehnt abgelaufene Envelopes, überschrittenes Hop Limit und regionale Loops im `trace` ab. Eine erkannte `dedupe_key` wird hingegen erfolgreich mit `duplicate=true` bestätigt und ohne weitere Zustellung unterdrückt. Eine fehlende Route ist ein eigenständiger Routingfehler, kein Loopnachweis.

Die Beispielgrenzen sind acht Hops und 900 Sekunden Dedupe-Fenster. Der [Python-Referenztest](../../../system-backend/transit/tests/transit_reference.py) prüft ein kleines Routingmodell; er ersetzt keinen HTTP- oder LXC-Failovertest.

**Quellabgleich vom 9. Oktober 2026:** [resolve_route, peer_usable, ingest_envelope und Failover](../../../system-backend/transit/src/state.rs). Weiter: [Zwei-Regionen-Test](tests/zwei-regionen-labortest.md).
