#!/usr/bin/env bash
# Installs the central Piper provider used by the NetCore Media Library.

set -euo pipefail

SERVICE_USER="${SERVICE_USER:-netcore-media-library}"
SERVICE_GROUP="${SERVICE_GROUP:-$SERVICE_USER}"
DEFAULT_VOICE="${DEFAULT_VOICE:-${VOICE:-de_DE-thorsten-medium}}"
VOICE_LIST="${VOICE_LIST:-de_DE-thorsten-medium de_DE-thorsten-high de_DE-karlsson-low de_DE-pavoque-low de_DE-thorsten_emotional-medium}"
VENV="${VENV:-/opt/netcore-piper}"
VOICE_DIR="${VOICE_DIR:-/var/lib/netcore-media-library/piper}"
TTS_CACHE="${TTS_CACHE:-/var/lib/netcore-media-library/tts/cache}"
TTS_TEMPLATES="${TTS_TEMPLATES:-/var/lib/netcore-media-library/tts/templates}"
PIPER_PORT="${PIPER_PORT:-5005}"
PIPER_READY_TIMEOUT="${PIPER_READY_TIMEOUT:-120}"
UNIT_PATH="/etc/systemd/system/netcore-piper.service"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

[[ ${EUID} -eq 0 ]] || { echo "Run this installer as root." >&2; exit 1; }
id "${SERVICE_USER}" >/dev/null 2>&1 || {
  echo "Service user '${SERVICE_USER}' does not exist." >&2
  exit 1
}

apt-get update
apt-get install -y --no-install-recommends \
  python3 python3-venv ca-certificates curl

# --upgrade-deps repairs an existing but incomplete venv as well. This matters
# after an interrupted first installation where bin/python already exists but
# piper-tts or its HTTP extra is still missing.
python3 -m venv --upgrade-deps "${VENV}"
"${VENV}/bin/python" -m pip install --upgrade 'piper-tts[http]'

# Fail here with a useful message instead of installing a unit that can never
# start. Importing http_server also verifies that the [http] extra is present.
"${VENV}/bin/python" - <<'PY'
import importlib
for module in ("piper", "piper.http_server", "piper.download_voices"):
    importlib.import_module(module)
print("Piper Python package and HTTP server module are installed.")
PY

install -d -o "${SERVICE_USER}" -g "${SERVICE_GROUP}" -m 0750 \
  "${VOICE_DIR}" "${TTS_CACHE}" "${TTS_TEMPLATES}"

read -r -a voices <<< "${VOICE_LIST}"
if [[ ! " ${voices[*]} " =~ " ${DEFAULT_VOICE} " ]]; then
  voices+=("${DEFAULT_VOICE}")
fi
for voice in "${voices[@]}"; do
  echo "Downloading/checking Piper voice: ${voice}"
  runuser -u "${SERVICE_USER}" -- \
    "${VENV}/bin/python" -m piper.download_voices \
      --data-dir "${VOICE_DIR}" "${voice}"
done

sed \
  -e "s|^User=.*|User=${SERVICE_USER}|" \
  -e "s|^Group=.*|Group=${SERVICE_GROUP}|" \
  -e "s|^WorkingDirectory=.*|WorkingDirectory=${VOICE_DIR}|" \
  -e "s|^Environment=HOME=.*|Environment=HOME=${VOICE_DIR}|" \
  -e "s|^ExecStart=.*|ExecStart=${VENV}/bin/python -m piper.http_server -m ${DEFAULT_VOICE} --data-dir ${VOICE_DIR} --host 127.0.0.1 --port ${PIPER_PORT}|" \
  -e "s|^ReadWritePaths=.*|ReadWritePaths=${VOICE_DIR} ${TTS_CACHE}|" \
  "${SCRIPT_DIR}/netcore-piper.service" > "${UNIT_PATH}"
chmod 0644 "${UNIT_PATH}"
chown -R "${SERVICE_USER}:${SERVICE_GROUP}" \
  "${VOICE_DIR}" "${TTS_CACHE}" "${TTS_TEMPLATES}"

systemctl daemon-reload
systemctl reset-failed netcore-piper.service 2>/dev/null || true
systemctl enable netcore-piper.service
systemctl restart netcore-piper.service

# Loading the first ONNX model is not instantaneous. The old installer queried
# /voices immediately after systemctl restart and falsely reported a failed
# installation while Piper was still starting.
ready=0
for ((second=1; second<=PIPER_READY_TIMEOUT; second++)); do
  if curl -fsS --max-time 2 \
      "http://127.0.0.1:${PIPER_PORT}/voices" \
      > /tmp/netcore-piper-voices.json 2>/dev/null; then
    ready=1
    break
  fi

  if ! systemctl is-active --quiet netcore-piper.service; then
    echo "Piper exited before opening port ${PIPER_PORT}." >&2
    systemctl --no-pager --full status netcore-piper.service >&2 || true
    journalctl -u netcore-piper.service -n 100 --no-pager >&2 || true
    exit 1
  fi
  sleep 1
done

if [[ ${ready} -ne 1 ]]; then
  echo "Piper did not become ready on 127.0.0.1:${PIPER_PORT} within ${PIPER_READY_TIMEOUT}s." >&2
  systemctl --no-pager --full status netcore-piper.service >&2 || true
  journalctl -u netcore-piper.service -n 100 --no-pager >&2 || true
  exit 1
fi

echo
echo "Piper voices available on port ${PIPER_PORT}:"
"${VENV}/bin/python" - <<'PY'
import json
from pathlib import Path
payload = json.loads(Path("/tmp/netcore-piper-voices.json").read_text())
if isinstance(payload, dict):
    print("\n".join(sorted(payload.keys())))
elif isinstance(payload, list):
    for item in payload:
        if isinstance(item, dict):
            print(item.get("key") or item.get("name") or json.dumps(item, ensure_ascii=False))
        else:
            print(item)
else:
    print(json.dumps(payload, ensure_ascii=False, indent=2))
PY
rm -f /tmp/netcore-piper-voices.json
