#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
MODE="${INSTALL_PIPER:-auto}"
SERVICE_USER="${NETCORE_MEDIA_SERVICE_USER:-netcore-media-library}"
SERVICE_GROUP="${NETCORE_MEDIA_SERVICE_GROUP:-netcore-media-library}"
VENV="${PIPER_VENV:-/opt/netcore-piper}"
VOICE_DIR="${PIPER_VOICE_DIR:-/var/lib/netcore-media-library/piper}"
TTS_CACHE="${PIPER_TTS_CACHE:-/var/lib/netcore-media-library/tts/cache}"
TTS_TEMPLATES="${PIPER_TTS_TEMPLATES:-/var/lib/netcore-media-library/tts/templates}"
PIPER_PORT="${PIPER_PORT:-5005}"
DEFAULT_VOICE="${PIPER_DEFAULT_VOICE:-de_DE-thorsten-medium}"
VOICE_LIST="${PIPER_VOICE_LIST:-de_DE-thorsten-medium de_DE-thorsten-high de_DE-karlsson-low de_DE-pavoque-low de_DE-thorsten_emotional-medium}"
UNIT_PATH="/etc/systemd/system/netcore-piper.service"

[[ ${EUID} -eq 0 ]] || { echo "ensure-piper.sh must run as root" >&2; exit 1; }
if [[ "${MODE}" == "0" || "${MODE}" == "false" || "${MODE}" == "no" ]]; then
  echo "[Media Library] Piper installation skipped (INSTALL_PIPER=${MODE})."
  exit 0
fi

id -u "${SERVICE_USER}" >/dev/null 2>&1 || \
  useradd --system --home /var/lib/netcore-media-library --shell /usr/sbin/nologin "${SERVICE_USER}"
install -d -o "${SERVICE_USER}" -g "${SERVICE_GROUP}" -m 0750 \
  "${VOICE_DIR}" "${TTS_CACHE}" "${TTS_TEMPLATES}"

full_install=0
[[ -x "${VENV}/bin/python" ]] || full_install=1
# A venv directory alone is not proof of a completed installation. Repair
# interrupted installs where Python exists but piper-tts[http] is missing.
if [[ ${full_install} -eq 0 ]]; then
  "${VENV}/bin/python" -c 'import piper, piper.http_server, piper.download_voices' \
    >/dev/null 2>&1 || full_install=1
fi
if [[ "${MODE}" == "1" || "${MODE}" == "true" || "${MODE}" == "force" ]]; then
  full_install=1
fi

if [[ ${full_install} -eq 1 ]]; then
  echo "[Media Library] Installing central Piper provider and German voices …"
  SERVICE_USER="${SERVICE_USER}" \
  SERVICE_GROUP="${SERVICE_GROUP}" \
  VENV="${VENV}" \
  VOICE_DIR="${VOICE_DIR}" \
  TTS_CACHE="${TTS_CACHE}" \
  TTS_TEMPLATES="${TTS_TEMPLATES}" \
  PIPER_PORT="${PIPER_PORT}" \
  DEFAULT_VOICE="${DEFAULT_VOICE}" \
  VOICE_LIST="${VOICE_LIST}" \
  bash "${ROOT}/system-backend/tts/install-piper.sh"
else
  echo "[Media Library] Existing Piper virtualenv found; checking service unit and models."

  # Always rewrite the unit for the central Media-Library account and paths. An
  # older basis-station-local unit must not survive the move to this LXC.
  sed \
    -e "s|^User=.*|User=${SERVICE_USER}|" \
    -e "s|^Group=.*|Group=${SERVICE_GROUP}|" \
    -e "s|^WorkingDirectory=.*|WorkingDirectory=${VOICE_DIR}|" \
    -e "s|^Environment=HOME=.*|Environment=HOME=${VOICE_DIR}|" \
    -e "s|^ExecStart=.*|ExecStart=${VENV}/bin/python -m piper.http_server -m ${DEFAULT_VOICE} --data-dir ${VOICE_DIR} --host 127.0.0.1 --port ${PIPER_PORT}|" \
    -e "s|^ReadWritePaths=.*|ReadWritePaths=${VOICE_DIR} ${TTS_CACHE}|" \
    "${ROOT}/system-backend/tts/netcore-piper.service" > "${UNIT_PATH}"
  chmod 0644 "${UNIT_PATH}"
  systemctl daemon-reload

  missing_models=0
  read -r -a voices <<< "${VOICE_LIST}"
  for voice in "${voices[@]}"; do
    [[ -f "${VOICE_DIR}/${voice}.onnx" && -f "${VOICE_DIR}/${voice}.onnx.json" ]] || missing_models=1
  done
  if [[ ${missing_models} -eq 1 ]]; then
    echo "[Media Library] One or more configured Piper models are missing; downloading them …"
    SERVICE_USER="${SERVICE_USER}" \
    VENV="${VENV}" \
    VOICE_DIR="${VOICE_DIR}" \
    VOICE_LIST="${VOICE_LIST}" \
    bash "${ROOT}/system-backend/tts/install-extra-voices.sh"
  fi

  chown -R "${SERVICE_USER}:${SERVICE_GROUP}" "${VOICE_DIR}" "${TTS_CACHE}" "${TTS_TEMPLATES}"
  systemctl daemon-reload
  systemctl enable --now netcore-piper.service
  systemctl restart netcore-piper.service
fi

for _ in $(seq 1 30); do
  if curl -fsS "http://127.0.0.1:${PIPER_PORT}/voices" >/dev/null 2>&1; then
    echo "[Media Library] Piper is ready on http://127.0.0.1:${PIPER_PORT}."
    exit 0
  fi
  sleep 1
done

echo "[Media Library] ERROR: Piper did not become ready within 30 seconds." >&2
systemctl --no-pager --full status netcore-piper.service >&2 || true
journalctl -u netcore-piper.service -n 100 --no-pager >&2 || true
exit 1
