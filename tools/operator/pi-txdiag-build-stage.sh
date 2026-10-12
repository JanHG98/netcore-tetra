#!/usr/bin/env bash
# Run as the ordinary SSH user on srv-m-tbs-03; stages only the reviewed diagnostic binary.
# Usage: bash pi-txdiag-build-stage.sh FULL_DIAGNOSTIC_COMMIT FULL_OLD_BINARY_SHA256 [EXISTING_SOURCE]
# Builds and replaces only the TBS executable. It never starts/enables RF.
# The diagnostic/recovery commit must be published, reviewed and tested before use.
# Native Soapy/codec build dependencies are those already installed in the image.
set -Eeuo pipefail
umask 0022

NC_DIAG_SHA=${1:?Full reviewed diagnostic commit required}
NC_OLD_SHA=${2:?Full current bluestation-bs SHA-256 required}
NC_RESUME_SOURCE=${3:-}
NC_BASE_SHA=1595259a2a76abfc9eff08842409156473b585a7
NC_BIN=/usr/local/bin/bluestation-bs
[[ $EUID -ne 0 ]] || { echo 'Run as the ordinary SSH user, not root.' >&2; exit 1; }
[[ $NC_DIAG_SHA =~ ^[0-9a-f]{40}$ && $NC_OLD_SHA =~ ^[0-9a-f]{64}$ ]] || exit 1
[[ $(hostname -s) == srv-m-tbs-03 && $(dpkg --print-architecture) == arm64 ]] || exit 1

nc_require_stopped() {
    [[ $(systemctl --no-pager show tetra.service -p ActiveState --value) == inactive &&
       $(systemctl --no-pager show tetra.service -p MainPID --value) == 0 &&
       $(systemctl --no-pager show tetra.service -p UnitFileState --value) == disabled ]] || {
        echo 'tetra.service must remain inactive, MainPID=0 and disabled.' >&2
        return 1
    }
}
nc_require_stopped
sudo -v
for NC_TOOL in git python3 pkg-config cmake readelf ldd; do
    command -v "$NC_TOOL" >/dev/null || { echo "Missing build tool: $NC_TOOL" >&2; exit 1; }
done
pkg-config --exists SoapySDR || { echo 'SoapySDR development package missing.' >&2; exit 1; }
[[ $(sudo sha256sum "$NC_BIN" | awk '{print $1}') == "$NC_OLD_SHA" ]] || {
    echo 'Installed executable differs from the recorded operator SHA-256.' >&2
    exit 1
}

# The image retains its source checkout, but removes the target and root Rustup
# caches. Use a separate user-owned clone; leave both agent-managed source and
# VM119 recipe/cache untouched. No APT, installer, native-codec or driver update.
if [[ -n $NC_RESUME_SOURCE ]]; then
    NC_SRC=$(readlink -e "$NC_RESUME_SOURCE")
    [[ -d $NC_SRC && $NC_SRC == "$HOME/netcore-txdiag-${NC_DIAG_SHA:0:12}."*/source ]] || {
        echo 'Resume source must be an existing isolated diagnostic checkout.' >&2
        exit 1
    }
    NC_GIT_STATUS=$(git -C "$NC_SRC" status --porcelain --untracked-files=all)
    [[ -z $NC_GIT_STATUS ]] || {
        echo 'Resume checkout has local changes; refusing to build.' >&2
        exit 1
    }
    NC_WORK=${NC_SRC%/source}
else
    NC_WORK=$(mktemp -d "$HOME/netcore-txdiag-${NC_DIAG_SHA:0:12}.XXXXXX")
    NC_SRC=$NC_WORK/source
    git clone --filter=blob:none --no-checkout https://github.com/JanHG98/netcore-tetra.git "$NC_SRC"
    git -C "$NC_SRC" fetch origin "$NC_DIAG_SHA"
    git -C "$NC_SRC" checkout --detach "$NC_DIAG_SHA"
fi
[[ $(git -C "$NC_SRC" rev-parse HEAD) == "$NC_DIAG_SHA" ]] || {
    echo 'Checkout does not match the requested diagnostic commit.' >&2
    exit 1
}
git -C "$NC_SRC" cat-file -e "$NC_BASE_SHA^{commit}"
NC_TARGET="$HOME/.cache/netcore-tetra-txdiag-target"
mkdir -p "$NC_TARGET"

# Independently enforce the full runtime baseline comparison. Exact Markdown
# moves were verified against both Git trees; every other runtime source change
# must be one of the reviewed Soapy diagnostic or RX-time recovery files.
# No extension wildcard.
python3 -B - "$NC_SRC" "$NC_BASE_SHA" "$NC_DIAG_SHA" <<'PY'
import subprocess, sys
source, baseline, diagnostic = sys.argv[1:]
changed = set(subprocess.check_output([
    'git', '-C', source, 'diff', '--name-only', baseline, diagnostic, '--',
    'crates', 'bins', 'Cargo.toml', 'Cargo.lock', 'tetra-codec-master',
    'system-backend/shared'
], text=True).splitlines())
patch = {
    'crates/tetra-entities/src/phy/components/soapy_dev.rs',
    'crates/tetra-entities/src/phy/components/soapy_dev_tests.rs',
    'crates/tetra-entities/src/messagerouter.rs',
    'crates/tetra-entities/src/phy/phy_bs.rs',
}
allowed = patch | {
    'bins/netcore-control-room/README.md',
    'tetra-codec-master/README.md',
    'system-backend/shared/README.md',
    'system-backend/shared/contracts/README.md',
    'system-backend/shared/contracts/ASSET_MANAGEMENT_V1.md',
    'system-backend/shared/contracts/COMMAND_MODEL_V1.md',
    'system-backend/shared/contracts/EVENT_MODEL_V1.md',
    'system-backend/shared/contracts/TASK_MODEL_V1.md',
    'system-backend/shared/web-ui/README.md',
}
if not patch <= changed or not changed <= allowed:
    raise SystemExit('Unexpected runtime source changes; refusing this build: ' + repr(sorted(changed)))
print('Verified runtime changes:', ', '.join(sorted(changed)))
PY

# Keep the image's recorded compiler version rather than selecting a newer
# stable toolchain. Debian 12's distro Cargo is too old for edition 2024.
NC_RUST_VERSION=$(sudo python3 -B - <<'PY'
import json, re
from pathlib import Path
data = json.loads(Path('/usr/share/netcore-image/versions.json').read_text())
match = re.match(r'^rustc ([0-9]+\.[0-9]+\.[0-9]+) ', data['rustc'])
if not match:
    raise SystemExit('Image compiler version is unavailable or not a stable version.')
print(match.group(1))
PY
)
if [[ ! -x "$HOME/.cargo/bin/rustup" ]]; then
    curl -q --fail --show-error --proto '=https' --tlsv1.2 \
        https://sh.rustup.rs -o "$NC_WORK/rustup-init.sh"
    sh "$NC_WORK/rustup-init.sh" -y --profile minimal \
        --no-modify-path --default-toolchain none
fi
"$HOME/.cargo/bin/rustup" toolchain install "$NC_RUST_VERSION" --profile minimal
"$HOME/.cargo/bin/cargo" +"$NC_RUST_VERSION" --version
"$HOME/.cargo/bin/rustc" +"$NC_RUST_VERSION" --version

cd "$NC_SRC"
CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR="$NC_TARGET" \
    "$HOME/.cargo/bin/cargo" +"$NC_RUST_VERSION" test --locked \
        -p tetra-entities --lib phy::components::soapy_dev::tests
CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR="$NC_TARGET" \
    "$HOME/.cargo/bin/cargo" +"$NC_RUST_VERSION" test --locked \
        -p tetra-entities --lib messagerouter::tests
CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR="$NC_TARGET" \
    "$HOME/.cargo/bin/cargo" +"$NC_RUST_VERSION" test --locked \
        -p tetra-entities --lib phy::phy_bs::tests
CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR="$NC_TARGET" \
    "$HOME/.cargo/bin/cargo" +"$NC_RUST_VERSION" build --locked --release -p bluestation-bs
NC_NEW=$NC_TARGET/release/bluestation-bs
[[ -f $NC_NEW && -x $NC_NEW ]]
readelf -h "$NC_NEW" | tee "$NC_WORK/elf-header.txt"
grep -Eq 'Machine:[[:space:]]+AArch64' "$NC_WORK/elf-header.txt"
ldd "$NC_NEW" | tee "$NC_WORK/ldd.txt"
if grep -q 'not found' "$NC_WORK/ldd.txt"; then
    echo 'Unresolved dynamic dependency; refusing installation.' >&2
    exit 1
fi
grep -a -Fq 'tx_written_blocks' "$NC_NEW"
grep -a -Fq 'tx_slot_not_ready_returns' "$NC_NEW"
grep -a -Fq 'PHY RX clock advanced' "$NC_NEW"
NC_NEW_SHA=$(sha256sum "$NC_NEW" | awk '{print $1}')
sha256sum "$NC_NEW"
nc_require_stopped

# Root rechecks the unit and expected executable immediately before replacing
# it. Backup and candidate are verified; os.replace performs a same-filesystem
# atomic replacement. No updater, TTS migration, GPIO probe or service start.
sudo python3 -B - "$NC_BIN" "$NC_NEW" "$NC_OLD_SHA" "$NC_NEW_SHA" \
    "$NC_BASE_SHA" "$NC_DIAG_SHA" "$NC_RUST_VERSION" <<'PY'
import hashlib, json, os, shutil, stat, subprocess, sys, tempfile
from pathlib import Path

binary_arg, source_arg, old_sha, new_sha, baseline, diagnostic, rust = sys.argv[1:]
binary, source = Path(binary_arg), Path(source_arg)
def digest(path):
    with path.open('rb') as handle:
        return hashlib.file_digest(handle, 'sha256').hexdigest()
def require_stopped():
    expected = {'ActiveState': 'inactive', 'MainPID': '0', 'UnitFileState': 'disabled'}
    for key, value in expected.items():
        actual = subprocess.check_output([
            'systemctl', '--no-pager', 'show', 'tetra.service', '-p', key, '--value'
        ], text=True).strip()
        if actual != value:
            raise SystemExit(f'tetra.service {key}={actual}; refusing replacement.')
require_stopped()
if not stat.S_ISREG(binary.lstat().st_mode):
    raise SystemExit('Installed executable must be a regular non-symlink file.')
if digest(binary) != old_sha or digest(source) != new_sha:
    raise SystemExit('Executable checksum changed before staging.')

backups = Path('/var/backups/netcore-tetra')
backups.mkdir(mode=0o700, parents=True, exist_ok=True)
backup_dir = Path(tempfile.mkdtemp(prefix='txdiag-' + diagnostic[:12] + '-', dir=backups))
backup = backup_dir / 'bluestation-bs.before'
shutil.copy2(binary, backup)
if digest(backup) != old_sha:
    raise SystemExit('Backup checksum differs; installed executable remains unchanged.')
fd, name = tempfile.mkstemp(prefix='.bluestation-bs-txdiag-', dir=binary.parent)
candidate = Path(name)
try:
    with os.fdopen(fd, 'wb') as output, source.open('rb') as input_file:
        shutil.copyfileobj(input_file, output)
        os.fchmod(output.fileno(), 0o755)
        os.fchown(output.fileno(), 0, 0)
        output.flush()
        os.fsync(output.fileno())
    if digest(candidate) != new_sha:
        raise SystemExit('Staged executable checksum differs.')
    require_stopped()
    if digest(binary) != old_sha:
        raise SystemExit('Installed executable changed during staging.')
    manifest = {
        'image_runtime_base': baseline, 'diagnostic_commit': diagnostic,
        'rustc_version': rust, 'old_binary_sha256': old_sha,
        'new_binary_sha256': new_sha, 'binary_path': str(binary),
        'backup_path': str(backup), 'rf_start_requested': False,
    }
    manifest_path = backup_dir / 'manifest.json'
    with backup.open('rb') as handle:
        os.fsync(handle.fileno())
    with manifest_path.open('w') as handle:
        handle.write(json.dumps(manifest, indent=2) + '\n')
        handle.flush()
        os.fsync(handle.fileno())
    directory_flags = os.O_RDONLY | getattr(os, 'O_DIRECTORY', 0)
    backup_fd = os.open(backup_dir, directory_flags)
    try:
        os.fsync(backup_fd)
    finally:
        os.close(backup_fd)
    os.replace(candidate, binary)
    binary_directory_fd = os.open(binary.parent, directory_flags)
    try:
        os.fsync(binary_directory_fd)
    finally:
        os.close(binary_directory_fd)
    print(json.dumps(manifest, indent=2))
finally:
    if candidate.exists():
        candidate.unlink()
PY

systemctl --no-pager show tetra.service \
    -p ActiveState -p MainPID -p UnitFileState -p ExecStart
sudo sha256sum "$NC_BIN"
printf 'Diagnostic binary installed; TBS remains inactive/disabled. Build tree: %s\n' "$NC_WORK"
# A separate, explicitly described manual RF run collects the new diagnostics.
# Rollback: stop tetra.service, restore the recorded backup, verify old SHA-256.
# Rollback does not start or enable tetra.service automatically.
