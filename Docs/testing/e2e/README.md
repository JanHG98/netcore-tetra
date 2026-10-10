# NetCore-Tetra Open-Lab E2E Tests

**Quellstand:** `main` (`c3ccdb4`, 09.10.2026), `deploy/open-lab/inventory.example.toml` und `tests/e2e/netcore_e2e/scenarios.py`. Das deklarierte Inventory enthält **26 Dienste** und der Runner **13 Szenarien**. Diese Anleitung ist kein Ergebnisbericht eines ausgeführten Live- oder Funklaufs.

**Quellen:** [tests/e2e](../../../tests/e2e) · [Repository-Root](../../..). Bei Befehlen das in der Anleitung angegebene Arbeitsverzeichnis beachten.

This package validates the interaction of all 26 services declared in the current isolated Open-Lab deployment inventory. It deliberately uses only the Python standard library so the runner can execute from the deployment host without an additional test framework.

## Safety model

The live tests target an explicitly configured `open_lab` inventory. They honor the declared endpoint/security contracts; the alert-service status path expects a token challenge and Deployment Core is a VM service. Other lab endpoints may be open and the runner does not add authentication to them. The management network must therefore remain isolated.

Read-only checks run without extra flags. Fixture creation, traffic injection and service restarts require explicit opt-in:

```bash
# Inventory and scenario validation only; no network access.
python3 tests/e2e/netcore_open_lab_e2e.py --validate-only --profile full

# Smoke test without application fixtures; mock-TBS sessions are transient runtime traffic.
python3 deploy/open-lab/netcore-deploy.py test --profile smoke

# Cross-service functional flow. Creates and removes E2E fixtures.
python3 deploy/open-lab/netcore-deploy.py test --profile full --allow-mutations

# Persistence and dependency-outage tests. Stops/restarts selected services over SSH.
python3 deploy/open-lab/netcore-deploy.py test --profile fault --allow-mutations --allow-restarts
```

Use `--inventory deploy/open-lab/inventory.toml` when the real lab addresses differ from the example inventory.

## Scenarios

| Scenario | Purpose | Mutating | Restart/stop |
| --- | --- | ---: | ---: |
| `contracts` | Liveness, readiness, status, OpenAPI, metrics and independent WebUI for every service | no | no |
| `node-gateway` | RFC6455 mock TBS handshake, capabilities, presence and ping | no | no |
| `edge-fallback-contract` | Mock-TBS fallback advertisement, availability and recovery contract | no | no |
| `subscriber-group` | Subscriber/group fixtures, registration and group-affiliation propagation | yes | no |
| `call-media-recorder` | Group-call telemetry, 35-byte TACELP frames, Media Switch and Recorder finalization | yes | no |
| `sds` | Individual delivery and offline store-and-forward | yes | no |
| `packet-data` | PDP activation, N-PDU downlink and IP Gateway context synchronization | yes | no |
| `control-room-federation` | Immediate federation poll and non-authoritative overview across the active configured federation targets | no | no |
| `platform-services` | Metadata-only management views for Security, KMF, Transit, Application Gateway and Media Library | no | no |
| `observability` | Central scrape plus searchable structured E2E marker | yes | no |
| `restart-restore` | Subscriber and group persistence after systemd restart | yes | yes |
| `fault-matrix` | Dependency degradation and recovery for Call/Media, Packet/IP and Media/Recorder paths | no | yes |
| `edge-service-outages` | Stop each eligible remote backend declared in the inventory in turn and verify Gateway + TBS fallback/recovery state | no | yes |

A custom subset can be selected repeatedly or as a comma-separated list:

```bash
python3 deploy/open-lab/netcore-deploy.py test \
  --scenario contracts,node-gateway \
  --scenario sds \
  --allow-mutations
```

## Mock TBS

Even non-fixture smoke scenarios establish temporary WebSocket sessions and emit telemetry. Use `--no-mock-tbs` to skip that peer. The thirteen scenario names are the current runner selection, not proof that a live run passed.

`netcore_e2e/mock_tbs.py` connects to the Node Gateway using `netcore-control-room-node-v1`. It advertises the currently required capabilities, emits node/registration/group/call/media telemetry, acknowledges control commands and returns correlated control responses. It is a deterministic Core integration peer, not an RF or ETSI conformance simulator.

## Reports

Each run creates a directory below `tests/e2e/artifacts/<run-id>/` containing:

- `report.json` – complete structured evidence;
- `junit.xml` – CI-compatible result;
- `summary.txt` – compact counters.

The generated artifact directory is excluded from repository packages.

## On-air evidence

Real radio tests are documented separately from the mock-TBS run:

```bash
cp tests/e2e/on_air_template.json tests/e2e/on_air_evidence.json
$EDITOR tests/e2e/on_air_evidence.json
python3 tests/e2e/validate_on_air_evidence.py tests/e2e/on_air_evidence.json
```

For a complete evidence set, use:

```bash
python3 tests/e2e/validate_on_air_evidence.py \
  tests/e2e/on_air_evidence.json \
  --require-complete \
  --require-two-vendors
```

The evidence format records firmware, MCC/MNC, RF parameters, terminal vendor/model, result, timestamps and references to logs or captures. It does not turn a passing lab test into a conformance claim.
