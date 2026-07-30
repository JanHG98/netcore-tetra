#!/usr/bin/env python3
"""Migrate legacy Media Library archive folders to YYYY/MM/DD file layout.

Legacy layout:
    <root>/<asset-uuid>/<archive-version>/{original.*,preview.wav,audio.tacelp,manifest.json}

Current layout:
    <root>/YYYY/MM/DD/<descriptive-stem>_{original,preview,tetra,metadata}.*

The script is idempotent, updates state.json archive_path values, and applies
OPEN-LAB NFS/SMB modes (directories 0777, files 0666).
"""
from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import sys
import tempfile
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

try:
    import tomllib
except ModuleNotFoundError as exc:  # pragma: no cover - Python < 3.11
    raise SystemExit("Python 3.11+ with tomllib is required") from exc

UUID_RE = re.compile(r"^[0-9a-fA-F]{8}-[0-9a-fA-F-]{27}$")


def log(message: str) -> None:
    print(f"[Media Library Archivmigration] {message}")


def parse_time(value: Any) -> datetime | None:
    if not isinstance(value, str) or not value.strip():
        return None
    text = value.strip()
    if text.endswith("Z"):
        text = text[:-1] + "+00:00"
    try:
        parsed = datetime.fromisoformat(text)
    except ValueError:
        return None
    if parsed.tzinfo is None:
        parsed = parsed.replace(tzinfo=timezone.utc)
    return parsed


def safe_token(value: Any, fallback: str) -> str:
    text = str(value or "").strip()
    text = text.replace("→", "-nach-")
    text = re.sub(r"[^A-Za-z0-9ÄÖÜäöüß._ -]+", "_", text)
    text = text.strip(" ._-") or fallback
    text = re.sub(r"[ .]+", "-", text)
    return text[:80].strip("-") or fallback


def archive_time(asset: dict[str, Any]) -> datetime:
    source_meta = asset.get("source_metadata") or {}
    return (
        parse_time(source_meta.get("recorded_at"))
        or parse_time(asset.get("created_at"))
        or datetime.now(timezone.utc)
    )


def stem_for(asset: dict[str, Any], timestamp: datetime) -> str:
    asset_id = str(asset.get("asset_id") or "unknown")
    short_id = re.sub(r"[^0-9a-fA-F]", "", asset_id)[:8] or "unknown"
    hhmmss = timestamp.strftime("%H-%M-%S")
    kind = str(asset.get("kind") or "other")
    source_meta = asset.get("source_metadata") or {}
    if kind == "recording" and source_meta:
        destination_type = source_meta.get("destination_type")
        call_type = "Gruppenruf" if destination_type == "group" else "Einzelruf" if destination_type == "individual" else "Funkruf"
        destination_kind = "GSSI" if destination_type == "group" else "ISSI"
        destination = source_meta.get("destination_id")
        source = source_meta.get("source_issi")
        duration_ms = source_meta.get("duration_ms")
        destination_text = f"{destination_kind}-{destination}" if destination is not None else destination_kind
        source_text = f"von-ISSI-{source}" if source is not None else "Quelle-unbekannt"
        try:
            duration_text = f"{(int(duration_ms) + 999) // 1000}s"
        except (TypeError, ValueError):
            duration_text = "Dauer-unbekannt"
        return f"{hhmmss}_{call_type}_{destination_text}_{source_text}_{duration_text}_{short_id}"

    if kind == "recording":
        title_text = str(asset.get("title") or "")
        legacy = re.match(r"^(Gruppenruf|Einzelruf)\s+(\d+)\s+[^0-9]+\s*(\d+)", title_text)
        if legacy:
            call_type, source, destination = legacy.groups()
            destination_kind = "GSSI" if call_type == "Gruppenruf" else "ISSI"
            duration_ms = (asset.get("metadata") or {}).get("duration_ms")
            try:
                duration_text = f"{(int(duration_ms) + 999) // 1000}s"
            except (TypeError, ValueError):
                duration_text = "Dauer-unbekannt"
            return f"{hhmmss}_{call_type}_{destination_kind}-{destination}_von-ISSI-{source}_{duration_text}_{short_id}"

    title = safe_token(asset.get("title"), "Ohne-Titel")
    type_label = "TTS" if kind == "tts" else "Funkruf" if kind == "recording" else safe_token(kind, "Medium")
    return f"{hhmmss}_{type_label}_{title}_{short_id}"


def chmod_tree(root: Path) -> None:
    if not root.exists():
        return
    for current_root, directories, files in os.walk(root, followlinks=False):
        current = Path(current_root)
        try:
            current.chmod(0o777)
        except OSError as exc:
            log(f"WARNUNG: Rechte für {current} konnten nicht gesetzt werden: {exc}")
        for name in directories:
            path = current / name
            if path.is_symlink():
                continue
            try:
                path.chmod(0o777)
            except OSError as exc:
                log(f"WARNUNG: Rechte für {path} konnten nicht gesetzt werden: {exc}")
        for name in files:
            path = current / name
            if path.is_symlink():
                continue
            try:
                path.chmod(0o666)
            except OSError as exc:
                log(f"WARNUNG: Rechte für {path} konnten nicht gesetzt werden: {exc}")


def role_and_extension(path: Path) -> tuple[str, str]:
    name = path.name.lower()
    extension = path.suffix.lstrip(".") or "bin"
    if name.startswith("preview"):
        return "preview", extension
    if name in {"audio.tacelp", "tetra.tacelp"} or extension == "tacelp":
        return "tetra", extension
    return "original", extension


def move_or_merge(source: Path, destination: Path) -> None:
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.parent.chmod(0o777)
    if destination.exists():
        if source.stat().st_size == destination.stat().st_size:
            source.unlink()
            return
        raise RuntimeError(f"Zieldatei existiert mit anderer Größe: {destination}")
    os.replace(source, destination)
    destination.chmod(0o666)


def find_legacy_manifests(root: Path) -> list[Path]:
    manifests: list[Path] = []
    if not root.is_dir():
        return manifests
    for asset_dir in root.iterdir():
        if not asset_dir.is_dir() or not UUID_RE.match(asset_dir.name):
            continue
        for version_dir in asset_dir.iterdir():
            if not version_dir.is_dir():
                continue
            manifest = version_dir / "manifest.json"
            if manifest.is_file():
                manifests.append(manifest)
    return manifests


def migrate_manifest(manifest_path: Path) -> tuple[str, Path] | None:
    data = json.loads(manifest_path.read_text(encoding="utf-8"))
    asset = data.get("asset") or {}
    asset_id = str(asset.get("asset_id") or manifest_path.parent.parent.name)
    timestamp = archive_time(asset)
    root = manifest_path.parent.parent.parent
    day = root / timestamp.strftime("%Y") / timestamp.strftime("%m") / timestamp.strftime("%d")
    day.mkdir(parents=True, exist_ok=True)
    for parent in [root, day.parent.parent, day.parent, day]:
        parent.chmod(0o777)
    stem = stem_for(asset, timestamp)

    files_meta: list[dict[str, Any]] = []
    for source in sorted(manifest_path.parent.iterdir()):
        if not source.is_file() or source == manifest_path:
            continue
        role, extension = role_and_extension(source)
        destination = day / f"{stem}_{role}.{extension}"
        move_or_merge(source, destination)
        files_meta.append({
            "filename": destination.name,
            "role": role,
            "size_bytes": destination.stat().st_size,
        })

    new_manifest = day / f"{stem}_metadata.json"
    asset["archive_path"] = str(new_manifest)
    data["schema"] = "netcore-media-library-archive-v2"
    data["layout"] = "year/month/day"
    data["asset"] = asset
    data["files"] = files_meta
    tmp = new_manifest.with_suffix(new_manifest.suffix + ".part")
    tmp.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    tmp.chmod(0o666)
    os.replace(tmp, new_manifest)
    new_manifest.chmod(0o666)
    manifest_path.unlink(missing_ok=True)
    try:
        manifest_path.parent.rmdir()
        manifest_path.parent.parent.rmdir()
    except OSError:
        pass
    return asset_id, new_manifest


def atomic_json(path: Path, data: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)

    # The migration runs as root from install/update.sh. Preserve the original
    # service ownership across the atomic replacement; otherwise state.json
    # becomes root:root 0640 and the netcore-media-library service immediately
    # fails with EACCES on its next start.
    original_stat = path.stat() if path.exists() else None
    fd, temp_name = tempfile.mkstemp(prefix=f".{path.name}.", suffix=".part", dir=path.parent)
    try:
        with os.fdopen(fd, "w", encoding="utf-8") as handle:
            json.dump(data, handle, ensure_ascii=False, indent=2)
            handle.write("\n")
            handle.flush()
            os.fsync(handle.fileno())

        if original_stat is not None:
            os.chown(temp_name, original_stat.st_uid, original_stat.st_gid)
            os.chmod(temp_name, original_stat.st_mode & 0o777)
        else:
            os.chmod(temp_name, 0o600)

        os.replace(temp_name, path)
    finally:
        try:
            os.unlink(temp_name)
        except FileNotFoundError:
            pass



def path_is_below(path: Path, root: Path) -> bool:
    try:
        path.resolve(strict=False).relative_to(root.resolve(strict=False))
        return True
    except ValueError:
        return False


def configured_root_for_asset(asset: dict[str, Any], storage: dict[str, Any]) -> Path | None:
    kind = str(asset.get("kind") or "other").strip().lower()
    key = (
        "recording_archive_root"
        if kind == "recording"
        else "tts_archive_root"
        if kind == "tts"
        else "archive_root"
    )
    value = storage.get(key)
    return Path(value) if value else None


def cleanup_empty_archive_parents(start: Path, archive_roots: list[Path]) -> None:
    roots = {root.resolve(strict=False) for root in archive_roots}
    current = start
    while current.exists() and current.resolve(strict=False) not in roots:
        try:
            current.rmdir()
        except OSError:
            break
        current = current.parent


def relocate_archived_asset(
    asset: dict[str, Any],
    storage: dict[str, Any],
    archive_roots: list[Path],
) -> Path | None:
    current_value = asset.get("archive_path")
    if not current_value:
        return None
    current_manifest = Path(str(current_value))
    desired_root = configured_root_for_asset(asset, storage)
    if desired_root is None or not desired_root.is_dir():
        return None
    if path_is_below(current_manifest, desired_root):
        return None
    if not current_manifest.is_file():
        log(
            f"WARNUNG: Archivpfad für {asset.get('asset_id', 'unbekannt')} fehlt: "
            f"{current_manifest}"
        )
        return None

    timestamp = archive_time(asset)
    target_day = (
        desired_root
        / timestamp.strftime("%Y")
        / timestamp.strftime("%m")
        / timestamp.strftime("%d")
    )
    target_day.mkdir(parents=True, exist_ok=True)
    for parent in [desired_root, target_day.parent.parent, target_day.parent, target_day]:
        parent.chmod(0o777)

    manifest_data = json.loads(current_manifest.read_text(encoding="utf-8"))
    file_names: list[str] = []
    for item in manifest_data.get("files") or []:
        if isinstance(item, dict) and item.get("filename"):
            file_names.append(str(item["filename"]))

    # Old or partially generated manifests may not have a files array. In that
    # case, collect every sibling belonging to the same descriptive stem.
    stem = current_manifest.name
    if stem.endswith("_metadata.json"):
        stem = stem[: -len("_metadata.json")]
    elif stem.endswith(".json"):
        stem = stem[:-5]
    if not file_names:
        file_names = [
            candidate.name
            for candidate in current_manifest.parent.glob(f"{stem}_*")
            if candidate.is_file() and candidate != current_manifest
        ]

    for filename in sorted(set(file_names)):
        source = current_manifest.parent / filename
        if not source.is_file():
            continue
        move_or_merge(source, target_day / filename)

    target_manifest = target_day / current_manifest.name
    manifest_asset = manifest_data.setdefault("asset", {})
    manifest_asset["archive_path"] = str(target_manifest)
    manifest_data["schema"] = "netcore-media-library-archive-v2"
    manifest_data["layout"] = "kind/year/month/day"
    atomic_json(target_manifest, manifest_data)
    target_manifest.chmod(0o666)
    if current_manifest != target_manifest:
        current_manifest.unlink(missing_ok=True)
    cleanup_empty_archive_parents(current_manifest.parent, archive_roots)
    asset["archive_path"] = str(target_manifest)
    asset["archived"] = True
    return target_manifest


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--config", default="/etc/netcore/media-library.toml")
    args = parser.parse_args()
    config_path = Path(args.config)
    with config_path.open("rb") as handle:
        config = tomllib.load(handle)
    storage = config.get("storage") or {}
    roots = []
    for key in ("archive_root", "recording_archive_root", "tts_archive_root"):
        value = storage.get(key)
        if value:
            path = Path(value)
            if path not in roots:
                roots.append(path)

    state_path = Path(storage.get("state_file", "/var/lib/netcore-media-library/state.json"))
    state: dict[str, Any] = {}
    if state_path.is_file():
        state = json.loads(state_path.read_text(encoding="utf-8"))
        backup = state_path.with_name(state_path.name + ".pre-archive-layout-v2.bak")
        if not backup.exists():
            shutil.copy2(state_path, backup)
            backup.chmod(0o640)
            log(f"Zustandsbackup erstellt: {backup}")

    migrated: dict[str, str] = {}
    for root in roots:
        if not root.is_dir():
            log(f"Überspringe nicht gemountetes Archiv: {root}")
            continue
        manifests = find_legacy_manifests(root)
        log(f"{root}: {len(manifests)} Legacy-Archive gefunden")
        for manifest in manifests:
            result = migrate_manifest(manifest)
            if result:
                asset_id, new_path = result
                migrated[asset_id] = str(new_path)
                log(f"{asset_id} -> {new_path}")
        chmod_tree(root)

    relocated: dict[str, str] = {}
    if state:
        assets = state.get("assets") or {}
        for asset_id, path in migrated.items():
            asset = assets.get(asset_id)
            if isinstance(asset, dict):
                asset["archive_path"] = path
                asset["archived"] = True

        # Enforce the physical archive roots after the legacy-layout migration:
        # recordings -> Recordings, TTS -> TTS-Dateien, generic media -> Media-Library.
        for asset_id, asset in assets.items():
            if not isinstance(asset, dict):
                continue
            new_path = relocate_archived_asset(asset, storage, roots)
            if new_path is not None:
                relocated[str(asset_id)] = str(new_path)
                log(f"{asset_id}: nach {new_path} einsortiert")

        if migrated or relocated:
            atomic_json(state_path, state)
            log(
                f"state.json aktualisiert: {len(migrated)} Layout-Migrationen, "
                f"{len(relocated)} Kategoriekorrekturen"
            )

    for root in roots:
        chmod_tree(root)

    log(
        f"Fertig. Layout migriert: {len(migrated)}, "
        f"falsch einsortierte Archive verschoben: {len(relocated)}"
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as exc:
        print(f"[Media Library Archivmigration] FEHLER: {exc}", file=sys.stderr)
        raise
