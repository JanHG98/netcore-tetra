// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

/// Keep a history of past N samples of type T
// Was: Bündelt die zusammengehörigen Werte für history in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct History<T: Copy, const N: usize> {
    buffer: [T; N],
    /// Index where latest sample has been written
    index: usize,
}

// Was: Implementiert das zugehörige Verhalten für `History<T, N>`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl<T: Copy, const N: usize> History<T, N> {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new(initial_values: T) -> Self {
        Self {
            buffer: [initial_values; N],
            index: 0,
        }
    }

    /// Write a sample to buffer
    // Was: Diese Funktion schreibt den vorgesehenen Arbeitsschritt.
    // Warum: Die Ausgabe wird dadurch einheitlich erzeugt und Schreibfehler können behandelt werden.
    pub fn write(&mut self, sample: T) {
        self.index = (self.index + 1) % N;
        self.buffer[self.index] = sample;
    }

    /// Get a sample with a given delay
    // Was: Führt den Arbeitsschritt `delayed` für delayed aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn delayed(&self, delay: usize) -> T {
        assert!(delay < N);
        self.buffer[(self.index + N - delay) % N]
    }
}
