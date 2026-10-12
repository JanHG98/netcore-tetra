//! Shared TOML editing for dashboard changes. Image-generated configs quote
//! table names, so table identity must come from the parser, not a line scan.

use std::io::{self, Write};
use std::path::Path;
use std::sync::{
    Mutex,
    atomic::{AtomicU64, Ordering},
};
use toml_edit::{DocumentMut, Item, Table, TableLike, Value};

/// Serializes read/validate/write for the dashboard's restarting config edits.
pub static CONFIG_EDIT_LOCK: Mutex<()> = Mutex::new(());
static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

pub fn document(original: &str) -> Result<DocumentMut, String> {
    original
        .parse::<DocumentMut>()
        .map_err(|e| format!("configuration does not parse: {e}"))
}

pub fn cell_table(doc: &mut DocumentMut) -> Result<&mut dyn TableLike, String> {
    if !doc.contains_key("cell_info") {
        doc.insert("cell_info", Item::Table(Table::new()));
    }
    doc.get_mut("cell_info")
        .and_then(Item::as_table_like_mut)
        .ok_or_else(|| "cell_info must be a TOML table".to_string())
}

/// Preserve a scalar's inline comments/spacing when replacing its value.
pub fn set_scalar(table: &mut dyn TableLike, key: &str, mut value: Value) {
    if let Some(existing) = table.get(key).and_then(Item::as_value) {
        *value.decor_mut() = existing.decor().clone();
    }
    table.insert(key, Item::Value(value));
}

/// The compare immediately before publication rejects stale browser edits.
/// Temp files live alongside the writable runtime config; rename publishes a
/// complete file and retains its permissions. No backup/cache trees are touched.
pub fn write_checked(config_path: &str, original: &str, replacement: &str) -> io::Result<()> {
    let path = Path::new(config_path);
    let metadata = std::fs::symlink_metadata(path)?;
    if !metadata.file_type().is_file() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "configuration must be a regular file"));
    }
    if std::fs::read_to_string(path)? != original {
        return Err(io::Error::new(
            io::ErrorKind::WouldBlock,
            "configuration changed; reload before saving",
        ));
    }
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let temp = parent.join(format!(".netcore-config-{}-{sequence}.tmp", std::process::id()));
    let result = (|| {
        let mut file = std::fs::OpenOptions::new().write(true).create_new(true).open(&temp)?;
        file.set_permissions(metadata.permissions())?;
        #[cfg(unix)]
        {
            use std::os::fd::AsRawFd;
            use std::os::unix::fs::MetadataExt;
            if unsafe { libc::fchown(file.as_raw_fd(), metadata.uid(), metadata.gid()) } != 0 {
                return Err(io::Error::last_os_error());
            }
        }
        file.write_all(replacement.as_bytes())?;
        file.sync_all()?;
        drop(file);
        if std::fs::read_to_string(path)? != original {
            return Err(io::Error::new(
                io::ErrorKind::WouldBlock,
                "configuration changed; reload before saving",
            ));
        }
        std::fs::rename(&temp, path)?;
        #[cfg(unix)]
        std::fs::File::open(parent)?.sync_all()?;
        Ok(())
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&temp);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atomic_write_rejects_stale_content_without_touching_config() {
        let path = std::env::temp_dir().join(format!("netcore-stale-config-{}.toml", std::process::id()));
        std::fs::write(&path, "version = 2\n").unwrap();
        let err = write_checked(path.to_str().unwrap(), "version = 1\n", "version = 3\n").unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::WouldBlock);
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "version = 2\n");
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn atomic_write_publishes_complete_content_and_preserves_permissions() {
        let path = std::env::temp_dir().join(format!("netcore-write-config-{}.toml", std::process::id()));
        std::fs::write(&path, "version = 1\n").unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o640)).unwrap();
        }
        write_checked(path.to_str().unwrap(), "version = 1\n", "version = 2\n").unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "version = 2\n");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(std::fs::metadata(&path).unwrap().permissions().mode() & 0o777, 0o640);
        }
        std::fs::remove_file(path).unwrap();
    }
}
