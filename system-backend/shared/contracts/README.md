# NetCore Shared Contracts

`netcore-contracts` is the transport-neutral contract crate for backend-to-backend communication.
It owns validated 24-bit SSI types, the `netcore.v1` envelope, service descriptors, health documents,
problem details, events, audit records and pagination shapes.

## Compatibility rule

- `netcore.v1` and `netcore.v1.x` share one major wire contract.
- A major change requires a parallel endpoint or adapter; it is never deployed as an in-place silent change.
- Unknown JSON fields may be accepted by receivers where their parser permits it, but senders must not rely on them without a capability handshake.
- Commands that can be retried require an idempotency key and a stable message ID.
- Raw TETRA key material and unredacted connector secrets are never valid generic-envelope payloads.

JSON Schemas are documentation and integration-test assets. The Rust types remain the compile-time source of truth.

## Gemeinsames Ereignismodell

Das kanonische Runtime-Format ist `netcore-event-v1`. Rust-Typ, JSON-Schema, Katalog und Beispiel liegen in:

- `src/event.rs`
- `schemas/netcore-event-v1.schema.json`
- `examples/netcore-event-subscriber-route-changed.json`
- `EVENT_MODEL_V1.md`

Lokale Dienstereignisse dürfen aus Kompatibilitätsgründen Zusatzfelder behalten, müssen für neue Integrationen aber ein gültiges `NetCoreEvent` bereitstellen.

## Gemeinsames Command-/Ack-Modell

Phase 4 ergänzt die transportneutralen Verträge `netcore-command-v1` und `netcore-command-ack-v1`. Rust-Typen, JSON-Schemas, Beispiele und Regeln liegen in:

- `src/command.rs`
- `schemas/netcore-command-v1.schema.json`
- `schemas/netcore-command-ack-v1.schema.json`
- `examples/netcore-command-virtual-relay-set.json`
- `examples/netcore-command-ack-succeeded.json`
- `COMMAND_MODEL_V1.md`

## Gemeinsames Task-Modell

Phase 9 ergänzt `netcore-task-v1` für strukturierte Aufträge und WAP-Formulare:

- `schemas/netcore-task-v1.schema.json`
- `TASK_MODEL_V1.md`
- Ereignisse `task.*` in `src/event.rs`

## Asset-, Personen- und Zuordnungsverträge

Phase 10 ergänzt:

- `schemas/netcore-asset-v1.schema.json`
- `schemas/netcore-person-v1.schema.json`
- `schemas/netcore-assignment-v1.schema.json`
- `ASSET_MANAGEMENT_V1.md`
- Ereignisse `asset.*`, `person.*`, `assignment.*` und `maintenance.*`

## SIP-Routing-Ereignisse

Phase 11 ergänzt `sip.*` für zentrale PBX-/TBS-Routingentscheidungen, TBS-Kontaktzustände und den SIP-Call-Lifecycle. Die transportneutralen Ereignisse enthalten keine SIP-Kennwörter oder vollständigen Asterisk-Konfigurationen.
