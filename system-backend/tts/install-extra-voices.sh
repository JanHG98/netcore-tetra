#!/usr/bin/env bash
# Adds configured voices to an existing central Piper installation.

set -euo pipefail

SERVICE_USER="${SERVICE_USER:-netcore-media-library}"
VENV="${VENV:-/opt/netcore-piper}"
VOICE_DIR="${VOICE_DIR:-/var/lib/netcore-media-library/piper}"
PIPER_PORT="${PIPER_PORT:-5005}"
PIPER_READY_TIMEOUT="${PIPER_READY_TIMEOUT:-120}"
VOICE_LIST="${VOICE_LIST:-de_DE-thorsten-high de_DE-karlsson-low de_DE-pavoque-low de_DE-thorsten_emotional-medium}"

[[ ${EUID} -eq 0 ]] || { echo "Run this helper as root." >&2; exit 1; }
id "${SERVICE_USER}" >/dev/null 2>&1 || {
  echo "Service user '${SERVICE_USER}' does not exist." >&2
  exit 1
}
[[ -x "${VENV}/bin/python" ]] || {
  echo "Piper virtual environment not found at ${VENV}." >&2
  exit 1
}
"${VENV}/bin/python" -c 'import piper, piper.http_server, piper.download_voices' || {
  echo "Piper or its HTTP extra is missing from ${VENV}. Run install-piper.sh first." >&2
  exit 1
}

install -d -o "${SERVICE_USER}" -g "$(id -gn "${SERVICE_USER}")" -m 0750 "${VOICE_DIR}"
read -r -a voices <<< "${VOICE_LIST}"
for voice in "${voices[@]}"; do
  echo "Downloading/checking Piper voice: ${voice}"
  runuser -u "${SERVICE_USER}" -- \
    "${VENV}/bin/python" -m piper.download_voices \
      --data-dir "${VOICE_DIR}" "${voice}"
done

chown -R "${SERVICE_USER}:$(id -gn "${SERVICE_USER}")" "${VOICE_DIR}"
systemctl reset-failed netcore-piper.service 2>/dev/null || true
systemctl restart netcore-piper.service

for ((second=1; second<=PIPER_READY_TIMEOUT; second++)); do
  if curl -fsS --max-time 2 "http://127.0.0.1:${PIPER_PORT}/voices" \
      > /tmp/netcore-piper-voices.json 2>/dev/null; then
    echo
    echo "Available Piper voices:"
    "${VENV}/bin/python" - <<'PY'
import json
from pathlib import Path
payload = json.loads(Path("/tmp/netcore-piper-voices.json").read_text())
if isinstance(payload, dict):
    print("\n".join(sorted(payload.keys())))
else:
    print(json.dumps(payload, ensure_ascii=False, indent=2))
PY
    rm -f /tmp/netcore-piper-voices.json
    exit 0
  fi
  if ! systemctl is-active --quiet netcore-piper.service; then
    systemctl --no-pager --full status netcore-piper.service >&2 || true
    journalctl -u netcore-piper.service -n 100 --no-pager >&2 || true
    exit 1
  fi
  sleep 1
done

echo "Piper did not become ready within ${PIPER_READY_TIMEOUT}s." >&2
systemctl --no-pager --full status netcore-piper.service >&2 || true
journalctl -u netcore-piper.service -n 100 --no-pager >&2 || true
exit 1
