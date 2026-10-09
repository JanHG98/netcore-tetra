# Provisioning Core im eigenen Proxmox-LXC

## Voraussetzungen

- Debian 13 oder Ubuntu 24.04 LXC
- mindestens 1 vCPU, 1 GiB RAM und 8 GiB Storage
- Netzwerkzugriff auf Subscriber Core Port 8100 und Group Core Port 8110
- Quellcode unter `/opt/netcore-tetra`

## Installation

```bash
apt update
apt install -y git curl build-essential pkg-config libssl-dev ca-certificates
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
git clone --branch feature/provisioning-core --single-branch https://github.com/JanHG98/netcore-tetra.git /opt/netcore-tetra
cd /opt/netcore-tetra
sudo bash system-backend/provisioning-core/install/install.sh
```

Danach `/etc/netcore/provisioning-core.toml` bearbeiten und die realen IP-Adressen eintragen.
