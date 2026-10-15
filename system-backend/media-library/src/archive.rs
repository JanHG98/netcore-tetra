//! Verified NAS receipts and bounded local payload storage.
//! The private state is authoritative. An editable NAS manifest alone never
//! authorizes removing a local original.
use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

use crate::config::MediaLibraryConfig;
use crate::media;
use crate::model::{ArchiveFile, AssetRecord};

pub fn root_for<'a>(config: &'a MediaLibraryConfig, kind: &str) -> Result<&'a Path, String> {
    match kind {
        "recording" => config.storage.recording_archive_root.as_deref(),
        "tts" => config.storage.tts_archive_root.as_deref(),
        _ => config.storage.archive_root.as_deref(),
    }.ok_or_else(|| format!("dedicated archive root is not configured for kind {kind}"))
}

fn no_symlinks(path: &Path) -> Result<(), String> {
    for ancestor in path.ancestors() {
        match fs::symlink_metadata(ancestor) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err(format!("symlink in media storage path: {}", ancestor.display()));
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("cannot inspect {}: {error}", ancestor.display())),
        }
    }
    Ok(())
}

pub fn require_root(config: &MediaLibraryConfig, root: &Path) -> Result<(), String> {
    no_symlinks(root)?;
    if !root.is_dir() {
        return Err(format!("archive root {} is unavailable", root.display()));
    }
    if config.storage.archive_require_network_mount {
        let mounts = fs::read_to_string("/proc/self/mountinfo")
            .map_err(|error| format!("cannot inspect archive mount: {error}"))?;
        if !is_network_mount(&root.canonicalize().map_err(|error| error.to_string())?, &mounts) {
            return Err(format!("archive root {} is not on a mounted NFS/SMB filesystem", root.display()));
        }
    }
    Ok(())
}

fn unescape_mount(value: &str) -> String {
    value.replace("\\040", " ").replace("\\011", "\t")
        .replace("\\012", "\n").replace("\\134", "\\")
}

pub fn is_network_mount(path: &Path, mounts: &str) -> bool {
    mounts.lines().filter_map(|line| {
        let (left, right) = line.split_once(" - ")?;
        let point = PathBuf::from(unescape_mount(left.split_whitespace().nth(4)?));
        let kind = right.split_whitespace().next()?;
        path.starts_with(&point).then_some((point.components().count(), kind))
    }).max_by_key(|(depth, _)| *depth)
        .is_some_and(|(_, kind)| matches!(kind, "nfs" | "nfs4" | "cifs" | "smb3"))
}

pub fn free_bytes(root: &Path) -> Result<u64, String> {
    // coreutils reads only the named local filesystem. No shell or NAS inventory.
    let output = Command::new("df").env("LC_ALL", "C")
        .args(["--block-size=1", "--output=avail", "--"]).arg(root)
        .output().map_err(|error| format!("cannot sample local free space: {error}"))?;
    if !output.status.success() {
        return Err("cannot sample local media filesystem free space".to_string());
    }
    String::from_utf8_lossy(&output.stdout).split_whitespace().last()
        .and_then(|value| value.parse().ok())
        .ok_or_else(|| "invalid local filesystem free-space result".to_string())
}

pub fn automatic(config: &MediaLibraryConfig, asset: &AssetRecord) -> bool {
    let enabled = match asset.kind.as_str() {
        "recording" => config.runtime.auto_archive_recordings,
        "tts" => config.runtime.auto_archive_tts,
        _ => config.runtime.auto_archive_uploads,
    };
    enabled && asset.state == "ready" && !asset.local_offloaded
        && (!asset.archived || asset.archive_files.is_empty())
        && root_for(config, &asset.kind).is_ok()
}

fn role_paths(asset: &AssetRecord) -> Vec<(&'static str, &Path, Option<&str>)> {
    [("original", asset.original_path.as_deref(), asset.sha256.as_deref()),
     ("preview", asset.preview_path.as_deref(), asset.preview_sha256.as_deref()),
     ("tetra", asset.tetra_path.as_deref(), asset.tetra_sha256.as_deref())]
        .into_iter().filter_map(|(role, path, hash)| path.map(|path| (role, path, hash))).collect()
}

pub fn local_path(config: &MediaLibraryConfig, asset: &AssetRecord, path: &Path) -> Result<(), String> {
    if Path::new(&asset.asset_id).components().count() != 1
        || !matches!(Path::new(&asset.asset_id).components().next(), Some(Component::Normal(_)))
        || path.parent() != Some(config.storage.root.join(&asset.asset_id).as_path())
    {
        return Err("asset payload escaped its local directory".to_string());
    }
    no_symlinks(path)?;
    Ok(())
}

pub fn copy_payloads(
    config: &MediaLibraryConfig, asset: &AssetRecord, destination: &Path, stem: &str,
) -> Result<Vec<ArchiveFile>, String> {
    let paths = role_paths(asset);
    if paths.is_empty() || asset.original_path.is_none() {
        return Err("cannot archive an asset without an original".to_string());
    }
    let mut files = Vec::new();
    for (role, source, expected) in paths {
        local_path(config, asset, source)?;
        let metadata = fs::symlink_metadata(source)
            .map_err(|error| format!("cannot inspect local {role}: {error}"))?;
        if !metadata.is_file() {
            return Err(format!("local {role} is not a regular file"));
        }
        let hash = media::sha256_file(source)?;
        if expected.is_some_and(|expected| !expected.eq_ignore_ascii_case(&hash)) {
            return Err(format!("local {role} checksum does not match its receipt"));
        }
        let extension = source.extension().and_then(|value| value.to_str()).unwrap_or("bin");
        let filename = format!("{stem}_{role}.{extension}");
        let target = destination.join(&filename);
        no_symlinks(&target)?;
        let copied = media::copy_atomic(source, &target, true)?;
        if copied != metadata.len() || media::sha256_file(&target)? != hash
            || media::sha256_file(source)? != hash
        {
            return Err(format!("NAS {role} copy does not match the local payload"));
        }
        files.push(ArchiveFile { role: role.to_string(), filename, size_bytes: copied, sha256: hash });
    }
    Ok(files)
}

/// Validate the NAS manifest against the receipt in private local state and
/// independently read every payload before authorizing eviction or recall.
pub fn verified_payloads(
    config: &MediaLibraryConfig, asset: &AssetRecord, check_local: bool,
) -> Result<Vec<(PathBuf, PathBuf, u64, String)>, String> {
    let root = root_for(config, &asset.kind)?;
    require_root(config, root)?;
    let manifest_path = asset.archive_path.as_ref()
        .ok_or_else(|| "asset has no archive manifest".to_string())?;
    no_symlinks(manifest_path)?;
    if !media::file_is_within(manifest_path, root)
        || fs::metadata(manifest_path).map_err(|error| error.to_string())?.len() > 2 * 1024 * 1024
    {
        return Err("archive manifest escaped its root or exceeds its size limit".to_string());
    }
    let manifest: serde_json::Value = serde_json::from_slice(
        &fs::read(manifest_path).map_err(|error| error.to_string())?
    ).map_err(|error| format!("invalid archive manifest: {error}"))?;
    let files: Vec<ArchiveFile> = serde_json::from_value(manifest["files"].clone())
        .map_err(|error| format!("invalid archive payload receipt: {error}"))?;
    if manifest["schema"] != "netcore-media-library-archive-v2"
        || manifest["asset"]["asset_id"].as_str() != Some(asset.asset_id.as_str())
        || asset.archive_files.is_empty() || files != asset.archive_files
    {
        return Err("archive manifest does not match the private local receipt".to_string());
    }
    let paths = role_paths(asset);
    if paths.len() != files.len() {
        return Err("archive receipt does not cover every local payload".to_string());
    }
    let mut roles = BTreeSet::new();
    let mut result = Vec::new();
    for file in files {
        if !roles.insert(file.role.clone()) || Path::new(&file.filename).components().count() != 1
            || !matches!(Path::new(&file.filename).components().next(), Some(Component::Normal(_)))
        {
            return Err("invalid or duplicate archive payload name/role".to_string());
        }
        let (_, local, expected) = paths.iter().find(|(role, _, _)| *role == file.role)
            .ok_or_else(|| "archive receipt contains an unexpected role".to_string())?;
        local_path(config, asset, local)?;
        if expected.is_some_and(|expected| !expected.eq_ignore_ascii_case(&file.sha256)) {
            return Err("archive checksum does not match asset metadata".to_string());
        }
        let archived = manifest_path.parent().expect("manifest has parent").join(&file.filename);
        no_symlinks(&archived)?;
        if !media::file_is_within(&archived, root)
            || fs::metadata(&archived).map_err(|error| error.to_string())?.len() != file.size_bytes
            || media::sha256_file(&archived)? != file.sha256
        {
            return Err(format!("NAS {} payload is unavailable or corrupt", file.role));
        }
        if check_local && local.exists()
            && (fs::metadata(local).map_err(|error| error.to_string())?.len() != file.size_bytes
                || media::sha256_file(local)? != file.sha256)
        {
            return Err(format!("local {} payload changed after archiving", file.role));
        }
        if check_local && !asset.local_offloaded && !local.is_file() {
            return Err(format!("local {} payload is missing", file.role));
        }
        result.push((local.to_path_buf(), archived, file.size_bytes, file.sha256));
    }
    Ok(result)
}

pub fn restore(config: &MediaLibraryConfig, asset: &AssetRecord) -> Result<(), String> {
    let paths = verified_payloads(config, asset, false)?;
    let bytes = paths.iter().fold(0u64, |total, (_, _, size, _)| total.saturating_add(*size));
    if free_bytes(&config.storage.root)? < config.storage.min_free_bytes.saturating_add(bytes) {
        return Err("insufficient local free space to restore this asset; NAS originals retained".to_string());
    }
    for (local, archived, size, expected) in paths {
        let copied = media::copy_atomic(&archived, &local, true)?;
        if copied != size || media::sha256_file(&local)? != expected
            || media::sha256_file(&archived)? != expected {
            return Err("recalled local payload failed its checksum check".to_string());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_mount_wins_and_local_shadow_is_not_an_archive() {
        let mounts = "1 0 0:1 / / rw - ext4 /dev/root rw\n\
            2 1 0:2 / /mnt/nfs-share rw - nfs4 10.0.1.148:/archive rw\n\
            3 2 0:3 / /mnt/nfs-share/hidden rw - tmpfs tmpfs rw\n";
        assert!(is_network_mount(Path::new("/mnt/nfs-share/Recordings"), mounts));
        assert!(!is_network_mount(Path::new("/mnt/nfs-share/hidden"), mounts));
        assert!(!is_network_mount(Path::new("/mnt/nfs-share-not/Recordings"), mounts));
        assert!(!is_network_mount(Path::new("/var/lib/media"), mounts));
    }
}
