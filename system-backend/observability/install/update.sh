#!/usr/bin/env bash
# One upgrade path for interactive installs and deployment-agent updates.
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec bash "${SCRIPT_DIR}/install.sh" "$@"
