# NetCore Open-Lab LXC Deployment

**Gültigkeit:** Dokumentation gegen `main` (`c3ccdb4`, 09.10.2026) geprüft. Installer, Konfiguration und API im aktuellen Quellbaum sind maßgeblich; Phasennamen und Beispieladressen sind keine Live-Abnahme. Für Updates den bestehenden Checkout und die tatsächlich gestartete Unit verwenden.

**Quellen:** [deploy/open-lab](../../../deploy/open-lab) · [Repository-Root](../../..). Bei Befehlen das in der Anleitung angegebene Arbeitsverzeichnis beachten.

This directory is the final cross-LXC integration layer for the current lab phase. It does not turn the management plane into a production system: existing backend WebUIs generally remain reachable without login, token or TLS and therefore belong on an isolated management VLAN only. The new `alert-service` (port 8310) requires a locally generated API token by default. Its delivery switch starts disabled; see [installation and update guide](../warnzentrale-installation-und-update.md).

## Offline workflow

```bash
cp deploy/open-lab/inventory.example.toml deploy/open-lab/inventory.toml
$EDITOR deploy/open-lab/inventory.toml
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml validate
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml plan
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml render
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml check-generated
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

`apply` retains existing host configuration and the alert installer's separate token EnvironmentFile. It snapshots an existing config before invoking its installer and restores that file afterwards or on installer failure. New installations receive the rendered dependency URLs. To intentionally replace a host config, use `apply --replace-config`; the old file is retained beside it as `<config>.pre-netcore-<UTC timestamp>-<pid>`. Restore that copy, restart the service and check `/health/ready` to undo a configuration replacement. Application/database recovery uses each service's documented backup procedure.

After each service restart, `apply` waits at most `ready_timeout_secs` (default 60) for `/health/ready`. A timeout stops the run with exit status 1 before dependent installers run. The `health_timeout_secs` setting bounds each individual request. `--dry-run` does not probe or change remote hosts.

The example declares 26 backend services, including the token-protected alert API and the Deployment/Discovery controller. The controller and Imagebuilder require a full Ubuntu VM; the other listed backend hosts use LXCs. These are source/configuration declarations, not a measured live service count. Example addresses must be adapted to the actual management network.

The repository's existing TBS `config.toml` uses Node Gateway `10.0.1.179:8080/ws/node`, while the backend inventory example uses `10.0.20.10:8080`. Inventory `[tbs_site]` explicitly records the retained site mapping, and the audit checks `config.toml` against that field separately from backend example URLs. Before a live deployment, adapt both mappings and verify that the TBS address reaches the intended Node Gateway and its service-health matrix. Passing the static audit does not establish that the two addresses refer to the same host or a reachable route.

Run `python3 tools/check_z01_integration.py` for the shared offline registry, generated-catalog, matrix, E2E-selection and static audit gate. `check-generated` fails on drift without rewriting the checked files. After changing examples deliberately, run `render` and then the gate. Adding the alert service does not enable warning delivery automatically.

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

The runner writes JSON, JUnit XML and a compact summary below `tests/e2e/artifacts/<run-id>/`. See `Docs/deployment/open-lab-integrationstest-anleitung.md` before enabling restarts.
