# NetCore Open-Lab LXC Deployment

This directory is the final cross-LXC integration layer for the current lab phase. It does not turn the management plane into a production system: every backend WebUI remains reachable without login, token or TLS and therefore belongs on an isolated management VLAN only.

## Offline workflow

```bash
cp deploy/open-lab/inventory.example.toml deploy/open-lab/inventory.toml
$EDITOR deploy/open-lab/inventory.toml
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml validate
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml plan
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml render
```

`render` rewrites service-to-service URLs by management port, creates the service catalog, `/etc/hosts` example, CSV port list and Graphviz dependency graph.

## Deployment

```bash
# Shows every scp/ssh action but changes nothing.
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml apply --dry-run

# Explicit real deployment after reviewing the plan and rendered configs.
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml apply
```

The deployer creates a deterministic source archive without PDFs, `.git`, `target`, caches or Node modules. Each LXC builds its own binary through the service's existing installer, receives its rendered config and is restarted in dependency order.

When services are installed manually, every installer detects the IPv4 address currently assigned to its LXC (including DHCP static leases), binds the WebUI to that address and prints the resulting URL. `NETCORE_LXC_IP` can override the automatic choice on multi-homed containers. Cross-LXC dependency addresses still come from this inventory or from local DNS; one container cannot infer every other lease by itself.

## Requirements

- Debian 13 or compatible LXC with systemd,
- Rust toolchain and C build dependencies on each build LXC,
- root SSH key access from the deployment host,
- isolated management network,
- `/dev/net/tun` passthrough for the IP Gateway,
- NFS mount prepared separately for Recorder/Media Library when archive features are used.

The tool intentionally does not store passwords, tokens, TLS keys, KMF master material or connector secrets.

## Cross-LXC E2E validation

The same inventory is also the source of truth for the integration runner:

```bash
# No network access; validate inventory and scenario selection.
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml test --profile full --validate-only

# Read-only service contract and mock-TBS smoke checks.
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml test --profile smoke

# Functional call/SDS/packet-data flow with temporary fixtures.
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml test --profile full --allow-mutations

# Persistence plus deliberate systemd dependency outages.
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml test --profile fault --allow-mutations --allow-restarts
```

The runner writes JSON, JUnit XML and a compact summary below `tests/e2e/artifacts/<run-id>/`. See `Docs/OPEN_LAB_E2E_RUNBOOK.md` before enabling restarts.
