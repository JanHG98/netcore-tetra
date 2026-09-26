#!/usr/bin/env bash
set -Eeuo pipefail

# NetCore-TETRA Basisstation updater
# Builds the Basisstation from THIS extracted source tree and replaces the
# executable that systemd is actually running. This avoids updating a harmless
# copy in /usr/local/bin while the unit still starts an older binary elsewhere.
#
# The script itself needs root for systemd and binary installation, but Cargo is
# deliberately run as the user who invoked sudo. Rustup installations normally
# live in ~/.cargo and ~/.rustup and are hidden by sudo's secure_path.

if [[ ${EUID} -ne 0 ]]; then
    echo "FEHLER: Bitte als root ausführen (sudo $0)." >&2
    exit 1
fi

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "$SCRIPT_DIR/.." && pwd)"
CONFIG_PATH="${CONFIG_PATH:-/etc/netcore/config.toml}"
UNIT="${UNIT:-}"
BINARY_PATH="${BINARY_PATH:-}"
CARGO_FEATURES="${CARGO_FEATURES:-}"
BUILD_USER="${BUILD_USER:-${SUDO_USER:-root}}"
DISABLE_LOCAL_PIPER="${DISABLE_LOCAL_PIPER:-1}"
MIGRATE_LOCAL_TTS_CONFIG="${MIGRATE_LOCAL_TTS_CONFIG:-1}"

log() { printf '[NetCore Basisstation Update] %s\n' "$*"; }
die() { printf '[NetCore Basisstation Update] FEHLER: %s\n' "$*" >&2; exit 1; }

command -v systemctl >/dev/null 2>&1 || die "systemctl wurde nicht gefunden."
command -v getent >/dev/null 2>&1 || die "getent wurde nicht gefunden."
command -v runuser >/dev/null 2>&1 || die "runuser wurde nicht gefunden (Paket util-linux)."
[[ -f "$REPO_ROOT/Cargo.toml" ]] || die "Cargo.toml fehlt unter $REPO_ROOT."
[[ -f "$REPO_ROOT/crates/tetra-config/src/bluestation/sec_media_library.rs" ]] \
    || die "Diese Quelle enthält die Media-Library-Konfiguration nicht. Falsches/älteres Paket?"

grep -q 'media_library: Option<CfgMediaLibraryDto>' \
    "$REPO_ROOT/crates/tetra-config/src/bluestation/parsing.rs" \
    || die "Der Root-Konfigurationsparser kennt [media_library] in dieser Quelle nicht."

id "$BUILD_USER" >/dev/null 2>&1 \
    || die "Build-Benutzer '$BUILD_USER' existiert nicht. Optional explizit setzen: BUILD_USER=jan sudo -E $0"

BUILD_HOME="$(getent passwd "$BUILD_USER" | awk -F: '{print $6}')"
[[ -n "$BUILD_HOME" && -d "$BUILD_HOME" ]] \
    || die "Home-Verzeichnis für Build-Benutzer '$BUILD_USER' konnte nicht ermittelt werden."
BUILD_GROUP="$(id -gn "$BUILD_USER")"

# Locate Cargo in the invoking user's environment first. A rustup installation
# normally lives at /home/<user>/.cargo/bin/cargo and is not in sudo secure_path.
CARGO_BIN="${CARGO_BIN:-}"
if [[ -z "$CARGO_BIN" && -x "$BUILD_HOME/.cargo/bin/cargo" ]]; then
    CARGO_BIN="$BUILD_HOME/.cargo/bin/cargo"
fi
if [[ -z "$CARGO_BIN" ]]; then
    CARGO_BIN="$(
        runuser -u "$BUILD_USER" -- env HOME="$BUILD_HOME" \
            sh -lc 'command -v cargo 2>/dev/null || true'
    )"
fi
if [[ -z "$CARGO_BIN" ]] && command -v cargo >/dev/null 2>&1; then
    CARGO_BIN="$(command -v cargo)"
fi
[[ -n "$CARGO_BIN" && -x "$CARGO_BIN" ]] || die \
    "cargo wurde weder systemweit noch für '$BUILD_USER' gefunden. Prüfen: sudo -u $BUILD_USER -H bash -lc 'cargo --version'"

CARGO_DIR="$(dirname -- "$CARGO_BIN")"
BUILD_PATH="$CARGO_DIR:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
BUILD_CARGO_HOME="${BUILD_CARGO_HOME:-$BUILD_HOME/.cargo}"
BUILD_RUSTUP_HOME="${BUILD_RUSTUP_HOME:-$BUILD_HOME/.rustup}"

run_as_builder() {
    if [[ "$BUILD_USER" == "root" ]]; then
        env \
            HOME="$BUILD_HOME" \
            USER="$BUILD_USER" \
            LOGNAME="$BUILD_USER" \
            PATH="$BUILD_PATH" \
            CARGO_HOME="$BUILD_CARGO_HOME" \
            RUSTUP_HOME="$BUILD_RUSTUP_HOME" \
            "$@"
    else
        runuser -u "$BUILD_USER" -- env \
            HOME="$BUILD_HOME" \
            USER="$BUILD_USER" \
            LOGNAME="$BUILD_USER" \
            PATH="$BUILD_PATH" \
            CARGO_HOME="$BUILD_CARGO_HOME" \
            RUSTUP_HOME="$BUILD_RUSTUP_HOME" \
            "$@"
    fi
}

log "Build-Benutzer: $BUILD_USER"
log "Cargo: $CARGO_BIN"
run_as_builder "$CARGO_BIN" --version \
    || die "Cargo ist vorhanden, kann für '$BUILD_USER' aber nicht ausgeführt werden."

# Cargo only needs a writable target directory. Repair leftovers from an older
# root build without changing ownership of the whole source tree.
mkdir -p "$REPO_ROOT/target"
chown -R "$BUILD_USER:$BUILD_GROUP" "$REPO_ROOT/target"

# Prefer the service_name configured by the user, then common historical names.
if [[ -z "$UNIT" && -r "$CONFIG_PATH" ]]; then
    configured_name="$({
        sed -nE 's/^[[:space:]]*service_name[[:space:]]*=[[:space:]]*"([^"]+)".*/\1/p' "$CONFIG_PATH" || true
    } | head -n1)"
    if [[ -n "$configured_name" ]]; then
        [[ "$configured_name" == *.service ]] || configured_name="${configured_name}.service"
        if systemctl cat "$configured_name" >/dev/null 2>&1; then
            UNIT="$configured_name"
        fi
    fi
fi

if [[ -z "$UNIT" ]]; then
    for candidate in \
        tetra.service \
        bluestation.service \
        tetra-bluestation.service \
        bluestation-bs.service
    do
        if systemctl cat "$candidate" >/dev/null 2>&1; then
            UNIT="$candidate"
            break
        fi
    done
fi

[[ -n "$UNIT" ]] || die "Keine Basisstations-Unit gefunden. Beispiel: UNIT=tetra.service sudo -E $0"
log "Systemd-Unit: $UNIT"

# Determine the executable that is REALLY running. This is the core of this fix.
if [[ -z "$BINARY_PATH" ]]; then
    main_pid="$(systemctl show "$UNIT" -p MainPID --value 2>/dev/null || true)"
    if [[ "$main_pid" =~ ^[1-9][0-9]*$ && -e "/proc/$main_pid/exe" ]]; then
        running_exe="$(readlink -f "/proc/$main_pid/exe" 2>/dev/null || true)"
        running_exe="${running_exe% (deleted)}"
        if [[ "$(basename -- "$running_exe")" == "bluestation-bs" ]]; then
            BINARY_PATH="$running_exe"
        fi
    fi
fi

if [[ -z "$BINARY_PATH" ]]; then
    exec_show="$(systemctl show "$UNIT" -p ExecStart --value 2>/dev/null || true)"
    BINARY_PATH="$(sed -nE 's/.*path=([^ ;}]\/bluestation-bs|[^ ;}]*bluestation-bs).*/\1/p' <<<"$exec_show" | head -n1)"
fi

if [[ -z "$BINARY_PATH" ]]; then
    exec_line="$(systemctl cat "$UNIT" 2>/dev/null | sed -nE 's/^[[:space:]]*ExecStart=[-@:+!]*([^[:space:]]*bluestation-bs).*/\1/p' | tail -n1)"
    [[ -n "$exec_line" ]] && BINARY_PATH="$exec_line"
fi

if [[ -z "$BINARY_PATH" ]]; then
    if command -v bluestation-bs >/dev/null 2>&1; then
        BINARY_PATH="$(readlink -f "$(command -v bluestation-bs)")"
    elif [[ -e /usr/local/bin/bluestation-bs ]]; then
        BINARY_PATH=/usr/local/bin/bluestation-bs
    fi
fi

[[ -n "$BINARY_PATH" ]] \
    || die "Aktive Binary nicht eindeutig gefunden. Beispiel: BINARY_PATH=/usr/local/bin/bluestation-bs UNIT=$UNIT sudo -E $0"

BINARY_PATH="$(readlink -m "$BINARY_PATH")"
log "Aktive Binary: $BINARY_PATH"

cd "$REPO_ROOT"

log "Prüfe den Parser-Regressionstest für [media_library] ..."
run_as_builder "$CARGO_BIN" test -p tetra-config --lib media_library_top_level_section_parses
run_as_builder "$CARGO_BIN" test -p tetra-config --lib media_library_unknown_field_is_rejected

log "Baue bluestation-bs aus dem entpackten Paket ..."
build_cmd=("$CARGO_BIN" build --release -p bluestation-bs)
if [[ -n "$CARGO_FEATURES" ]]; then
    build_cmd+=(--features "$CARGO_FEATURES")
fi
run_as_builder "${build_cmd[@]}"

NEW_BINARY="$REPO_ROOT/target/release/bluestation-bs"
[[ -x "$NEW_BINARY" ]] || die "Build erfolgreich gemeldet, aber $NEW_BINARY fehlt."

if [[ "$MIGRATE_LOCAL_TTS_CONFIG" != "0" && -f "$CONFIG_PATH" ]]; then
    command -v python3 >/dev/null 2>&1 || die "python3 wird für die TTS-Konfigurationsmigration benötigt."

    # Keep the exact owner/group/mode. The helper performs an atomic replace and
    # runs as root, so without this guard config.toml could become root:root 0640
    # and fail the unit's ExecStartPre read check before the new binary starts.
    read -r CONFIG_UID CONFIG_GID CONFIG_MODE < <(stat -c '%u %g %a' "$CONFIG_PATH")
    python3 "$REPO_ROOT/install/remove-local-tts-config.py" "$CONFIG_PATH"
    chown "$CONFIG_UID:$CONFIG_GID" "$CONFIG_PATH"
    chmod "$CONFIG_MODE" "$CONFIG_PATH"
fi

SERVICE_USER="$(systemctl show "$UNIT" -p User --value 2>/dev/null || true)"
[[ -n "$SERVICE_USER" ]] || SERVICE_USER=root
if [[ "$SERVICE_USER" == root ]]; then
    test -r "$CONFIG_PATH" || die "$CONFIG_PATH ist für root nicht lesbar."
else
    runuser -u "$SERVICE_USER" -- test -r "$CONFIG_PATH" \
        || die "$CONFIG_PATH ist für den Dienstbenutzer '$SERVICE_USER' nicht lesbar. Prüfe Eigentümer, Gruppe und Modus."
fi
log "Konfiguration ist für den Dienstbenutzer '$SERVICE_USER' lesbar."

backup_dir="/var/backups/netcore-tetra"
mkdir -p "$backup_dir"
stamp="$(date +%Y%m%d-%H%M%S)"
backup_path="$backup_dir/bluestation-bs.${stamp}.bak"

if [[ -e "$BINARY_PATH" ]]; then
    cp -a -- "$BINARY_PATH" "$backup_path"
    log "Alte Binary gesichert: $backup_path"
fi

rollback() {
    local reason="$1"
    printf '[NetCore Basisstation Update] FEHLER: %s\n' "$reason" >&2
    if [[ -f "$backup_path" ]]; then
        log "Spiele die vorherige Binary zurück ..."
        systemctl stop "$UNIT" || true
        install -m 0755 -- "$backup_path" "$BINARY_PATH"
        systemctl start "$UNIT" || true
    fi
    exit 1
}

log "Stoppe $UNIT und ersetze genau die aktive Binary ..."
systemctl stop "$UNIT"
install -D -m 0755 -- "$NEW_BINARY" "$BINARY_PATH"

start_epoch="$(date +%s)"
systemctl start "$UNIT" || rollback "$UNIT ließ sich mit der neuen Binary nicht starten."
sleep 6

if ! systemctl is-active --quiet "$UNIT"; then
    journalctl -u "$UNIT" --since "@$start_epoch" --no-pager -n 120 || true
    rollback "$UNIT ist nach dem Update nicht aktiv."
fi

recent_log="$(journalctl -u "$UNIT" --since "@$start_epoch" --no-pager 2>/dev/null || true)"
if grep -Eq 'Unrecognized top-level fields: \["media_library"\]|Primary config .*media_library.*Running on fallback' <<<"$recent_log"; then
    printf '%s\n' "$recent_log" >&2
    rollback "Die neue Instanz meldet [media_library] weiterhin als unbekannt."
fi

if [[ "$DISABLE_LOCAL_PIPER" != "0" ]] && systemctl cat netcore-piper.service >/dev/null 2>&1; then
    log "Deaktiviere den nicht mehr benötigten lokalen Piper-Dienst auf der Basisstation ..."
    systemctl disable --now netcore-piper.service ||         log "WARNUNG: netcore-piper.service konnte nicht automatisch deaktiviert werden."
fi

log "Update erfolgreich. Die laufende Binary akzeptiert [media_library] und verwendet zentrale TTS-Assets."
log "Status: $(systemctl is-active "$UNIT")"
log "Kontrolle: journalctl -u $UNIT -n 100 --no-pager"
