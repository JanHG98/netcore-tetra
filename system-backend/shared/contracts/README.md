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
