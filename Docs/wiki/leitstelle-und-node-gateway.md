# Control Room und Node Gateway

Im verteilten Aufbau sind **Node Gateway** und **Control Room** verschiedene Dienste. Die TBS verbindet sich über `[control_room]` per WebSocket mit dem Node Gateway (`/ws/node`, Managementport im Beispiel `8080`). Control Room (`9010` im Inventory) zeigt netzweite Zustände und verarbeitet Operatoraktionen. Ein TBS-WebSocket direkt zur Control-Room-WebUI ist in dieser Topologie der falsche Zielpfad.

## Daten- und Befehlsweg

TBS-Ereignisse laufen über `/ws/node` zum Node Gateway. Die Fachkerne verarbeiten den Netzstand; Control Room nutzt ihre APIs für Ansicht und Befehle. Health-Matrix und Kommandos gelangen über das Gateway zur zuständigen TBS zurück.

`node_id` bezeichnet die TBS stabil im Backend. Control Room besitzt Operatorprofile, Rollen und Arbeitsplatzansichten; die lokale TBS bleibt RF- und CMCE-Instanz. Der vollständige [Control-Room-Quellort](../../system-backend/control-room) enthält Core, UI, Auth-/Profil- und Deploymentunterlagen. Daneben gibt es [netcore-control-room](../../bins/netcore-control-room); beim Deployment nicht ältere Standalone-Beispiele mit dem 26-Dienst-Inventory vermischen.

## TBS-Anbindung im Open Lab

```toml
[control_room]
enabled = true
host = "<NODE-GATEWAY-IP>"
port = 8080
use_tls = false
endpoint_path = "/ws/node"
node_id = "TBS-LAB-01"
station_name = "TBS-LAB-01"
site = "Lab"
```

Nur diesen Block in eine **vollständige und passende** TBS-TOML übernehmen. Die [sanitisierte Vorlage](../basisstation.config.sanitized.example.toml) deaktiviert die Anbindung zunächst. Bei aktivem Open-Lab-Modus sind Node- und Managementverbindungen im isolierten Testnetz zu halten. Bei gesicherter Bereitstellung müssen Authentisierung und TLS für alle beteiligten Endpunkte gemeinsam geplant werden. [Sicherheit und Betrieb](sicherheit-im-betrieb.md)

## Bedienung und Grenzen

Control Room enthält Operatorrollen und Profile. Die aktuelle Open-Lab-Vorlage setzt jedoch `[auth].enabled = false` und einen leeren `node_token_env`; daraus entsteht keine verpflichtende Benutzeranmeldung oder authentisierte TBS-Verbindung. Ein in einem gesonderten Sicherheitsmodell eingerichteter Node-Token betrifft die **TBS-Identität**, nicht den menschlichen Arbeitsplatzlogin. Der UI-Arbeitsplatz ersetzt weder die Teilnehmerdaten des Subscriber Core noch die Gruppenpolicy des Group Core. Bei Ausfall des Control Room kann die TBS lokal weiterarbeiten; bei Node-Gateway-Ausfall folgt sie dem [Mehrzellenbetrieb, Mobility und Edge-Fallback](mehrzellenbetrieb-und-ausfallverhalten.md)- und [Fallback](datensicherung-und-fallback.md)-Verhalten.

Für einen Fehler zuerst Gateway-Listener, `/ws/node`, Service-Matrix und TBS-Logs prüfen, anschließend Control Room und dessen Fach-APIs. [Netzwerk, Ports und Protokolle](netzwerk-und-ports.md) · [Fehlersuche](fehlersuche.md)

## Quellen zur Pflege dieser Seite

[Control-Room-Vorlage](../../system-backend/control-room/config/control-room.example.toml) · [TBS-/Gateway-Vertrag](../../crates/tetra-entities/src/net_control_room/protocol.rs).
