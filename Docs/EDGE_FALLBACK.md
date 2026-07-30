# TBS Edge Autonomy and Core-Service Fallback

NetCore-Tetra does not treat loss of Internet, VPN or Core reachability as a
reason to stop the radio cell. The base station keeps the Air Interface and the
locally installed functions authoritative until the central service plane has
recovered consistently.

## Decision path

```text
Core LXCs ── /health/ready ──> Node Gateway
                                   │
                                   │ CoreServicesSnapshot (revisioned)
                                   ▼
                               TBS WebSocket
                                   │
                 ┌─────────────────┴──────────────────┐
                 │ healthy                            │ unavailable
                 ▼                                    ▼
          central authority                    local edge authority
          + multi-cell routing                 + durable replay spool
```

The TBS does **not** test a public Internet host. Reachability is determined by
its Node Gateway connection and the health of the configured NetCore services.
An Internet outage, broken VPN or routing failure therefore has the same clear
result: central functions are withdrawn and the cell switches locally.

## State machine

- `online`: Node Gateway and all required services are healthy.
- `degraded`: one or more individual Core services are unavailable and their
  service-specific local fallbacks are active; remaining healthy services stay
  usable. It is also the short grace phase before full isolation.
- `isolated`: the Node Gateway itself is unreachable or the complete health
  matrix lease expired, so the TBS is fully locally authoritative.
- `recovering`: all required services are healthy again, but the recovery
  hysteresis and durable event replay are still running.

`enter_after_secs` prevents a short Gateway/network interruption from causing
an immediate full-isolation transition. A single backend outage remains
`degraded` and activates only that backend's documented fallback.
`recover_after_secs` prevents flapping and avoids central/local split authority.
Unknown service state is unavailable by default.

## Local behaviour by service

| Service | Behaviour while unreachable |
|---|---|
| Node Gateway | Local edge autonomy; reconnect loop continues |
| Subscriber Core | Last-known policy cache, then static configuration |
| Group Core | Last-known group policy and local affiliations |
| Mobility Core | Local registration and Location Area operation |
| Call Control | Calls within the local cell remain possible |
| Media Switch | Local Air-Interface media remains active; central frames are ignored |
| Recorder | Local recorder continues independently |
| SDS Router | Local delivery; non-local traffic is durably queued |
| Packet Core | Local SNDCP/PDP contexts remain active |
| IP Gateway | Local TUN/routing remains available when configured locally |
| Security Core | Last-known policy; never silently downgrade security |
| KMF | Already installed keys remain usable; no new OTAR is invented |
| Transit | No inter-region routing |
| Control Room | Local TBS dashboard and audit remain available |
| Observability | Local logs and health continue; central collection may be absent |
| Application Gateway | Only locally reachable integrations remain available |
| Media Library | Locally cached/approved media remains usable |

## Subscriber and group policy cache

Subscriber admission and group policy are persisted atomically in
`edge-policy-cache.json`. The cache is loaded before the TBS entities start.
By default a stale policy remains effective because reverting silently to an
open network would be less safe than retaining the last explicit operator
policy. This is configurable with `keep_last_known_policy`.

The cache contains policy metadata only, never KMF master material or raw keys.

## Durable SDS and telemetry replay

Control-plane events that are safe to replay are written as bounded JSONL to
`edge-event-spool.jsonl`. The spool has entry and byte limits, is fsynced, and is
replayed in bounded batches after recovery.

High-rate speech and RF frames are deliberately **not** spooled. They are
real-time traffic; replaying stale audio would be wrong. Local recording remains
separate and continues where enabled.

For group SDS delivered locally during isolation, the replay record is marked
`air_fallback_local_delivered`. The SDS Router excludes the originating node
when it later creates remote delivery legs, preventing duplicate local delivery.

## Required configuration

The TBS must connect its `[control_room]` endpoint to the **Node Gateway**
(`/ws/node`, default port `8080`) when the distributed LXC backend is used.
The Node Gateway monitors the other 16 Runtime LXCs through `/health/ready` and
sends the complete service matrix over that existing WebSocket.

The repository defaults remain Open Lab: no login, no tokens and no TLS. That is
appropriate only in an isolated test network.

## Validation

Run the component and cross-system checks:

```bash
python3 tools/check_full_system_integration.py
python3 deploy/open-lab/netcore-deploy.py \
  --inventory deploy/open-lab/inventory.example.toml validate
python3 deploy/open-lab/netcore-deploy.py \
  --inventory deploy/open-lab/inventory.example.toml test \
  --profile smoke --validate-only
```

A real acceptance test still needs running LXCs and at least one real TBS/radio
pair. Static checks and mocks prove contracts and orchestration, not RF or ETSI
on-air conformance.

## Health-matrix lease

The Node Gateway republishes the complete service matrix once per probe cycle,
even when no state changed. The TBS treats that matrix as a renewable lease
(`service_matrix_lease_secs`, default 60 seconds). If the WebSocket remains open
but the monitor thread or its scheduler stops, the lease expires and **every
central-service availability check fails closed** instead of trusting stale
`available` states. Older matrix revisions cannot renew this lease.

The TBS diagnostic response includes `service_matrix_fresh` and
`service_matrix_received_at`. The current TBS view is available at
`GET /api/edge-fallback`; the Node Gateway view is available at
`GET /api/v1/core-services`.

For a live fault acceptance run, `--profile fault --allow-restarts` stops each of
the 16 remote backend services in turn and verifies that the Node Gateway and a
connected TBS both see `unavailable`, then verifies the corresponding recovery.
The Node Gateway outage itself is exercised by disconnecting the TBS path: the
local lease/connection state machine handles that case because the health
matrix transport is then unavailable too.
