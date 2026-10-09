# NetCore Open-Lab LXC Deployment

This directory is the final cross-LXC integration layer for the current lab phase. It does not turn the management plane into a production system: existing backend WebUIs generally remain reachable without login, token or TLS and therefore belong on an isolated management VLAN only. The new `alert-service` (port 8310) requires a locally generated API token by default. Its delivery switch starts disabled; see [installation and update guide](../../Docs/deployment/KATWARN_NINA_INSTALL_UPDATE.md).

[Vollständige Dokumentation](../../Docs/deployment/open-lab/README.md) · [Zentraler Dokumentationsindex](../../Docs/README.md)

[Online lesen](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/deployment/open-lab/README.md).
