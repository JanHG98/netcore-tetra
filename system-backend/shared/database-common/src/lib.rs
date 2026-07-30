// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für gemeinsame Datenbankzugriffe.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! Small, dependency-light persistence primitives for JSON-backed lab services.
//!
//! This does not replace a transactional database. It makes the existing file-backed services
//! use the same crash-safe write pattern and keeps backup behaviour predictable.

use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für atomic JSON-Daten store in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct AtomicJsonStore {
    path: PathBuf,
    fsync: bool,
}

// Was: Implementiert das zugehörige Verhalten für `AtomicJsonStore`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl AtomicJsonStore {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into(), fsync: true }
    }

    // Was: Führt den Arbeitsschritt `with_fsync` für with fsync aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn with_fsync(mut self, fsync: bool) -> Self {
        self.fsync = fsync;
        self
    }

    // Was: Führt den Arbeitsschritt `path` für path aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn path(&self) -> &Path {
        &self.path
    }

    // Was: Diese Funktion lädt den vorgesehenen Arbeitsschritt.
    // Warum: Einlesen und Fehlerbehandlung bleiben dadurch an einer zentralen Stelle.
    pub fn load<T: DeserializeOwned>(&self) -> io::Result<T> {
        let file = File::open(&self.path)?;
        serde_json::from_reader(BufReader::new(file)).map_err(invalid_data)
    }

    // Was: Diese Funktion lädt or default.
    // Warum: Einlesen und Fehlerbehandlung bleiben dadurch an einer zentralen Stelle.
    pub fn load_or_default<T: DeserializeOwned + Default>(&self) -> io::Result<T> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self.load() {
            Ok(value) => Ok(value),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(T::default()),
            Err(error) => Err(error),
        }
    }

    // Was: Diese Funktion speichert den vorgesehenen Arbeitsschritt.
    // Warum: Die Speicherung wird dadurch einheitlich durchgeführt und Fehler gehen nicht unbemerkt verloren.
    pub fn save<T: Serialize>(&self, value: &T) -> io::Result<()> {
        let parent = self.path.parent().unwrap_or_else(|| Path::new("."));
        fs::create_dir_all(parent)?;
        let tmp = self.path.with_extension(format!("{}.tmp", std::process::id()));
        let file = OpenOptions::new().create_new(true).write(true).open(&tmp)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, value).map_err(invalid_data)?;
        writer.write_all(b"\n")?;
        writer.flush()?;
        if self.fsync {
            writer.get_ref().sync_all()?;
        }
        drop(writer);
        fs::rename(&tmp, &self.path)?;
        if self.fsync {
            File::open(parent)?.sync_all()?;
        }
        Ok(())
    }

    // Was: Führt den Arbeitsschritt `backup_to` für backup to aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn backup_to(&self, destination: impl AsRef<Path>) -> io::Result<u64> {
        let destination = destination.as_ref();
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        let copied = fs::copy(&self.path, destination)?;
        File::open(destination)?.sync_all()?;
        Ok(copied)
    }
}

// Was: Führt den Arbeitsschritt `invalid_data` für invalid data aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn invalid_data(error: impl std::fmt::Display) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, error.to_string())
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    // Was: Führt den Arbeitsschritt `atomically_roundtrips_json` für atomically roundtrips JSON-Daten aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn atomically_roundtrips_json() {
        let root = std::env::temp_dir().join(format!("netcore-db-common-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let store = AtomicJsonStore::new(root.join("state.json"));
        let source = BTreeMap::from([("answer".to_owned(), 42_u64)]);
        store.save(&source).unwrap();
        assert_eq!(store.load::<BTreeMap<String, u64>>().unwrap(), source);
        assert!(!root.join(format!("state.{}.tmp", std::process::id())).exists());
        fs::remove_dir_all(root).unwrap();
    }
}
