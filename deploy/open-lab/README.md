# NetCore Open-Lab Deployment

Inventory, rendering and integration deployment for 26 declared runtime services. Most services use LXCs; Deployment Core on 8320 is the Ubuntu VM/imagebuilder. The inventory count is source configuration, not live-installation evidence. It does not turn the management plane into a production system: existing backend WebUIs generally remain reachable without login, token or TLS and therefore belong on an isolated management VLAN only. The new `alert-service` (port 8310) requires a locally generated API token by default. Its delivery switch starts disabled; see [installation and update guide](../../Docs/deployment/warnzentrale-installation-und-update.md).

[Vollständige Dokumentation](../../Docs/deployment/open-lab/README.md) · [Zentraler Dokumentationsindex](../../Docs/README.md)

[Online lesen](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/deployment/open-lab/README.md).
