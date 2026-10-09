# NetCore Command/Ack Model v1

## Zweck

`netcore-command-v1` und `netcore-command-ack-v1` bilden den transportneutralen Vertrag für steuernde Integrationen. MQTT ist nur ein Transport. Ein Command ist erst dann zulässig, wenn Schema, Zeitfenster, Deduplizierung und Policy geprüft wurden.

## Sicherheitsgrenze der OPEN-LAB-Stufe

Phase 4 besitzt weiterhin keine Anmeldung, keine Tokens und kein TLS. `source` und `actor` sind daher selbst deklarierte Metadaten und **keine authentisierte Identität**. Die mitgelieferte Konfiguration erlaubt ausschließlich virtuelle Testaktoren mit IDs unter `lab-`. Unbekannte oder reale Aktortypen werden standardmäßig abgewiesen.

## Verbindliche Regeln

- `command_id` ist stabil und global eindeutig.
- Wiederholungen mit derselben `command_id` dürfen nie erneut ausgeführt werden.
- `requested_at` und `expires_at` begrenzen die Gültigkeit.
- Retained Commands werden standardmäßig abgewiesen.
- Die Policy arbeitet standardmäßig nach `default deny`.
- Ein passendes Deny überstimmt jedes Allow.
- `dry_run` prüft Schema, Zeitfenster, Policy und Payload, verändert aber keinen Zustand.
- Jede Annahme, Ablehnung, Ausführung und Dublette erzeugt ein Ack.
- Terminale Acks sind `rejected`, `succeeded`, `failed` oder `duplicate`.
- Acks werden über eine persistente Outbox veröffentlicht.

## MQTT-Abbildung

```text
netcore/v1/commands/#
netcore/v1/acks/<command-id>
netcore/v1/state/virtual_relays/<target-id>
netcore/v1/state/virtual_lights/<target-id>
netcore/v1/state/virtual_buttons/<target-id>
```

## Mitgelieferte Sandbox-Executor

- `virtual.relay.set`
- `virtual.light.set`
- `virtual.button.press`

Diese Executor berühren ausschließlich die persistente virtuelle Zustandsdatei des IoT Gateways. Reale Homematic-, GPIO-, Modbus- oder Relaisaktionen folgen in den späteren Adapterphasen.
