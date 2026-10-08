#!/usr/bin/env bash
# Z01.4: replace the Observability binary on the verified CT136 only.
set -Eeuo pipefail

[[ ${EUID} -eq 0 && $(hostname) == Observability ]] || {
  echo 'Dieser Block ist für root im CT136 Observability.' >&2; exit 1;
}
NC_REPO=$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)
git -C "$NC_REPO" diff --quiet
git -C "$NC_REPO" diff --cached --quiet
NC_SOURCE=$(git -C "$NC_REPO" rev-parse HEAD)
NC_BINARY=/opt/netcore-observability/bin/netcore-observability
[[ -f "$NC_BINARY" && ! -L "$NC_BINARY" && -x "$NC_BINARY" ]]
[[ $(systemctl show netcore-observability.service -p User --value) == netcore-observability ]]
systemctl is-active --quiet netcore-observability.service
systemctl show netcore-observability.service -p ExecStart --value |
  grep -F -- "$NC_BINARY --config /etc/netcore/observability.toml" >/dev/null
export PATH="/root/.cargo/bin:$PATH"
cargo --version
rustc --version

NC_RUN=$(mktemp -d /var/tmp/netcore-obs-update.XXXXXX)
NC_MAIN_PID=$BASHPID
NC_CHANGED=0
NC_STAGE=''
NC_rollback() {
  local code=$1
  [[ $BASHPID -eq $NC_MAIN_PID ]] || exit "$code"
  trap - ERR INT TERM HUP
  set +e
  if [[ $NC_CHANGED -eq 1 ]]; then
    journalctl -u netcore-observability.service --no-pager -n 30 >&2
    if systemctl stop netcore-observability.service &&
       NC_STAGE=$(mktemp "${NC_BINARY}.rollback.XXXXXX") &&
       cp --preserve=mode,ownership -- "$NC_RUN/previous-binary" "$NC_STAGE" &&
       mv -fT -- "$NC_STAGE" "$NC_BINARY" &&
       systemctl restart netcore-observability.service &&
       python3 - <<'PY_ROLLBACK'
import json, time, urllib.request
http = urllib.request.build_opener(urllib.request.ProxyHandler({}))
deadline = time.monotonic() + 15
while True:
    try:
        with http.open('http://10.0.1.143:8210/health/ready', timeout=3) as response:
            assert json.load(response)['ready'] is True
        break
    except (OSError, AssertionError, KeyError):
        if time.monotonic() >= deadline:
            raise RuntimeError('Vorheriger Build wird nicht bereit')
        time.sleep(1)
PY_ROLLBACK
    then
      echo 'Vorheriger Binarybuild wieder gestartet; Update fehlgeschlagen.' >&2
    else
      echo "Rücknahme nicht vollständig. Vorheriges Binary: $NC_RUN/previous-binary" >&2
    fi
  fi
  [[ -z $NC_STAGE ]] || rm -f -- "$NC_STAGE"
  echo "Prüf- und Rückwegdateien: $NC_RUN" >&2
  exit "$code"
}
trap 'NC_rollback $?' ERR
trap 'NC_rollback 130' INT
trap 'NC_rollback 143' TERM
trap 'NC_rollback 129' HUP

NC_preflight() {
  python3 - <<'PY'
import json
from pathlib import Path
import subprocess
import tomllib
import urllib.request

cfg = tomllib.loads(Path('/etc/netcore/observability.toml').read_text())
assert cfg['server']['bind'] == '10.0.1.143:8210'
logs = json.loads(Path('/etc/netcore/syslog.json').read_text())
assert logs['nms_url'] == 'http://127.0.0.1:8210'
http = urllib.request.build_opener(urllib.request.ProxyHandler({}))
for unit, port in [('netcore-discovery.service', 8321),
                   ('netcore-deployment.service', 8320)]:
    result = subprocess.run(['systemctl', 'show', unit, '-p', 'LoadState',
                             '-p', 'ActiveState'], capture_output=True, text=True)
    properties = dict(line.split('=', 1) for line in result.stdout.splitlines()
                      if '=' in line)
    if properties.get('LoadState') == 'not-found':
        continue
    assert result.returncode == 0, (unit, result.stderr)
    state = properties['ActiveState']
    assert state not in ('activating', 'deactivating', 'reloading'), (unit, state)
    if state == 'active':
        with http.open(f'http://127.0.0.1:{port}/api/v1/jobs', timeout=10) as response:
            jobs = json.load(response)
        assert isinstance(jobs, list)
        for job in jobs:
            assert job.get('status') not in ('queued', 'running'), job.get('id')
            assert not job.get('result', {}).get('remote_uncertain'), job.get('id')
print('Standortkonfiguration und Auftragsvorprüfung bestanden.')
PY
}

NC_preflight
sha256sum /etc/netcore/observability.toml /etc/netcore/syslog.json > "$NC_RUN/config.sha256"
sha256sum "$NC_BINARY" > "$NC_RUN/binary.sha256"
stat -c '%u:%g:%a' "$NC_BINARY" > "$NC_RUN/binary.stat"
CARGO_BUILD_JOBS=1 cargo build --locked --release \
  --manifest-path "$NC_REPO/Cargo.toml" \
  --target-dir "$NC_RUN/target" -p netcore-observability

NC_preflight
sha256sum --check "$NC_RUN/config.sha256"
sha256sum --check "$NC_RUN/binary.sha256"
[[ $(stat -c '%u:%g:%a' "$NC_BINARY") == $(cat "$NC_RUN/binary.stat") ]]
cp -a -- "$NC_BINARY" "$NC_RUN/previous-binary"
cmp -- "$NC_BINARY" "$NC_RUN/previous-binary"
NC_STAGE=$(mktemp "${NC_BINARY}.update.XXXXXX")
install -m 0755 "$NC_RUN/target/release/netcore-observability" "$NC_STAGE"
chown --reference="$NC_BINARY" "$NC_STAGE"
chmod --reference="$NC_BINARY" "$NC_STAGE"
NC_CHANGED=1
mv -fT -- "$NC_STAGE" "$NC_BINARY"
NC_STAGE=''
systemctl restart netcore-observability.service

python3 - <<'PY'
import json
import time
import urllib.request

http = urllib.request.build_opener(urllib.request.ProxyHandler({}))
def get(base, path):
    with http.open(base + path, timeout=3) as response:
        return json.load(response)
bases = ('http://127.0.0.1:8210', 'http://10.0.1.143:8210')
deadline = time.monotonic() + 30
while True:
    try:
        for base in bases:
            assert get(base, '/health/ready')['ready'] is True
            assert get(base, '/api/v1/config')['server']['bind'] == '10.0.1.143:8210'
        break
    except (OSError, AssertionError, KeyError) as error:
        if time.monotonic() >= deadline:
            raise RuntimeError('Beide HTTP-Adressen werden nicht bereit') from error
        time.sleep(1)
print('Beide HTTP-Adressen bereit; Management-Bind erhalten.')
try:
    receiver = get(bases[0], '/api/v1/syslog').get('receiver') or {}
    print('Preview-Puffer:', json.dumps({key: receiver.get(key) for key in
          ('preview_pending', 'preview_error', 'updated_at')}, ensure_ascii=False))
except OSError as error:
    print('Separater Syslog-Status noch nicht lesbar:', error)
PY
sha256sum --check "$NC_RUN/config.sha256"
printf '%s\n' "PASS: Observability-Binary aktualisiert, beide APIs bereit, Konfiguration erhalten." \
  "Quellcommit: $NC_SOURCE" \
  "Vorheriges Binary und Prüfsummen: $NC_RUN"
trap - ERR INT TERM HUP
