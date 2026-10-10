# Architektur und Datenfluss

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/state.rs](../../../system-backend/sds-router/src/state.rs) · [src/http.rs](../../../system-backend/sds-router/src/http.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

## Uplink

Das Funkgerät sendet U-SDS-DATA oder U-STATUS an die lokale CMCE/SDS-Edge der TBS. Diese erzeugt `TelemetryEvent::SdsEdgeIngress` und übergibt es über den Node-Transport an den Backend-WebSocket des Node Gateway. Der SDS Router übernimmt Routing, Persistenz, TTL und Duplikaterkennung.

`SdsEdgeIngress` enthält eine stabile Message-ID, Ingress-Art, Source/Destination, Group-Flag, SDS-Typ, Protocol-ID, exakte Bitlänge, Payload und Priorität. Der Router verändert die Nutzdaten nicht.

## Downlink

Der SDS Router sendet `ControlCommand::DeliverSds` beziehungsweise `SendStatus` über den Node Gateway an die zuständige TBS. Deren CMCE/SDS-Edge rekonstruiert D-SDS-DATA beziehungsweise D-STATUS für das lokale Air Interface.

Die TBS beantwortet die Annahme mit `ControlResponse::SdsDeliveryResponse`. Das ist eine **Edge-Annahme**, noch kein garantierter Terminal-Zustellbericht. Ein später empfangener SDS-TL-Report wird separat mit der Nachricht korreliert.

## Routingreihenfolge

1. Passende Anwendungsregeln erzeugen Application Legs. `intercept` unterdrückt sämtliche Funkziele, auch explizit erzwungene TBS.
2. Ohne Intercept werden explizite `force_nodes` und passende Node-Routen gemeinsam in die Zielmenge aufgenommen.
3. Nur wenn diese Menge leer ist, wird auf beobachtete Teilnehmerlage beziehungsweise Gruppenaffiliationen zurückgegriffen.
4. Bei lokal bereits zugestelltem Ingress wird die Quell-TBS aus der zentralen Zielmenge entfernt.

`at_most_once` verwendet einen eigenen Individualpfad: genau eine bekannte Serving-TBS oder ein einzelnes explizites Ziel; normale Anwendungs- und Gruppenrouten sind dabei ausgeschlossen.

Ohne verfügbares Ziel bleibt die Nachricht im Zustand `offline` gespeichert und wird bei neuer Präsenz erneut geplant.
