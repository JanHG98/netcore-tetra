// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Nachrichten an den Schnittstellen zwischen TETRA-Protokollschichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

#[derive(Debug, Clone, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für TETRA-Kurznachricht (SDS) Benutzer data auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum SdsUserData {
    /// Type field 0, 16 bits, short_data_type_identifier == 0
    Type1(u16),
    /// Type field 1, 32 bits, short_data_type_identifier == 1
    Type2(u32),
    /// Type field 2, 64 bits, short_data_type_identifier == 2
    Type3(u64),
    /// Type field 3, variable length, short_data_type_identifier == 3
    Type4(u16, Vec<u8>),
}

// Was: Implementiert das zugehörige Verhalten für `SdsUserData`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl SdsUserData {
    // Was: Führt den Arbeitsschritt `type_identifier` für type identifier aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn type_identifier(&self) -> u8 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            SdsUserData::Type1(_) => 0,
            SdsUserData::Type2(_) => 1,
            SdsUserData::Type3(_) => 2,
            SdsUserData::Type4(_, _) => 3,
        }
    }

    // Was: Führt den Arbeitsschritt `length_bits` für length bits aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn length_bits(&self) -> u16 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            SdsUserData::Type1(_) => 16,
            SdsUserData::Type2(_) => 32,
            SdsUserData::Type3(_) => 64,
            SdsUserData::Type4(len_bits, _) => *len_bits,
        }
    }

    // Was: Wandelt den vorhandenen Wert in arr um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn to_arr(&self) -> Vec<u8> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            SdsUserData::Type1(value) => value.to_be_bytes().to_vec(),
            SdsUserData::Type2(value) => value.to_be_bytes().to_vec(),
            SdsUserData::Type3(value) => value.to_be_bytes().to_vec(),
            SdsUserData::Type4(_, data) => data.clone(),
        }
    }
}
