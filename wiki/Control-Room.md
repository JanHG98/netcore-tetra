# Control Room und Node Gateway

Im verteilten Aufbau sind **Node Gateway** und **Control Room** verschiedene Dienste. Die TBS verbindet sich über `[control_room]` per WebSocket mit dem Node Gateway (`/ws/node`, Managementport im Beispiel `8080`). Control Room (`9010` im Inventory) zeigt netzweite Zustände und verarbeitet Operatoraktionen. Ein TBS-WebSocket direkt zur Control-Room-WebUI ist in dieser Topologie der falsche Zielpfad.

## Daten- und Befehlsweg

```text
TBS ── /ws/node ──> Node Gateway ── Fach-APIs/Backend-Sessions ──> Control Room
  <─────────────── Health-Matrix und autorisierte Kommandos ───────────────
```

`node_id` bezeichnet die TBS stabil im Backend. Control Room besitzt Operatorprofile, Rollen und Arbeitsplatzansichten; die lokale TBS bleibt RF- und CMCE-Instanz. Der vollständige [Control-Room-Quellort](https://github.com/JanHG98/netcore-tetra/tree/main/system-backend/control-room) enthält Core, UI, Auth-/Profil- und Deploymentunterlagen. Daneben gibt es [`bins/netcore-control-room`](https://github.com/JanHG98/netcore-tetra/tree/main/bins/netcore-control-room); beim Deployment nicht ältere Standalone-Beispiele mit dem 24-Dienst-Inventory vermischen.

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

Nur diesen Block in eine **vollständige und passende** TBS-TOML übernehmen. Die [sanitisierte Vorlage](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/basisstation.config.sanitized.example.toml) deaktiviert die Anbindung zunächst. Bei aktivem Open-Lab-Modus sind Node- und Managementverbindungen im isolierten Testnetz zu halten. Bei gesicherter Bereitstellung müssen Authentisierung und TLS für alle beteiligten Endpunkte gemeinsam geplant werden. [[Security-and-Operations]]

## Bedienung und Grenzen

Operatorrollen und Profile des Control Room steuern menschliche Zugriffe. Ein Node-Token (falls konfiguriert) dient dagegen der **TBS-Identität**. Der UI-Arbeitsplatz ersetzt weder die Teilnehmerdaten des Subscriber Core noch die Gruppenpolicy des Group Core. Bei Ausfall des Control Room kann die TBS lokal weiterarbeiten; bei Node-Gateway-Ausfall folgt sie dem [[Mehrzellenbetrieb]]- und [[Fallback|Backup-and-Fallback]]-Verhalten.

Für einen Fehler zuerst Gateway-Listener, `/ws/node`, Service-Matrix und TBS-Logs prüfen, anschließend Control Room und dessen Fach-APIs. [[Netzwerk-und-Ports]] · [[Troubleshooting]]
