#!/usr/bin/env bash
set -u
VENV="${PIPER_VENV:-/opt/netcore-piper}"
PORT="${PIPER_PORT:-5005}"
VOICE_DIR="${PIPER_VOICE_DIR:-/var/lib/netcore-media-library/piper}"

echo '=== systemd status ==='
systemctl --no-pager --full status netcore-piper.service || true

echo
echo '=== recent journal ==='
journalctl -u netcore-piper.service -n 100 --no-pager || true

echo
echo '=== Python/Package ==='
if [[ -x "${VENV}/bin/python" ]]; then
  "${VENV}/bin/python" --version || true
  "${VENV}/bin/python" -m pip show piper-tts || true
  "${VENV}/bin/python" - <<'PY' || true
import importlib
for name in ("piper", "piper.http_server", "piper.download_voices"):
    try:
        module = importlib.import_module(name)
        print(f"OK import {name}: {getattr(module, '__file__', '<namespace>')}")
    except Exception as exc:
        print(f"ERROR import {name}: {exc!r}")
PY
else
  echo "Missing virtualenv Python: ${VENV}/bin/python"
fi

echo
echo '=== Models ==='
find "${VOICE_DIR}" -maxdepth 1 -type f \( -name '*.onnx' -o -name '*.onnx.json' \) \
  -printf '%m %u:%g %s %f\n' 2>/dev/null | sort || true

echo
echo '=== Listener/API ==='
ss -ltnp 2>/dev/null | grep -E ":${PORT}[[:space:]]" || true
curl -v --max-time 5 "http://127.0.0.1:${PORT}/voices" || true
