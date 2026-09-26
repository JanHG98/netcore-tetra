// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

/// CRC-16 (ITU-T / X.25) over raw bits or byte streams.
// Was: Legt den festen Wert `GEN_POLY` für gen poly fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const GEN_POLY: u16 = 0x1021;
// Was: Legt den festen Wert `TETRA_CRC_OK` für TETRA crc ok fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const TETRA_CRC_OK: u16 = 0x1d0f;

#[inline]
// Was: Diese Funktion liest nth bit.
// Warum: Der Zugriff auf den Wert bleibt dadurch gekapselt und kann später zentral angepasst werden.
pub fn get_nth_bit(input: &[u8], bit: usize) -> u16 {
    let byte = bit / 8;
    let bit_in_byte = 7 - (bit % 8);
    ((input[byte] >> bit_in_byte) & 1) as u16
}

/// CRC-16 ITU-T over a byte stream, processing `number_bits` bits (MSB first).
/// `crc` is the initial CRC value.  
/// Returns the updated CRC.
// Was: Führt den Arbeitsschritt `crc16_itut_bytes` für crc16 itut bytes aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn crc16_itut_bytes(mut crc: u16, input: &[u8], number_bits: usize) -> u16 {
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for i in 0..number_bits {
        let bit = get_nth_bit(input, i);
        crc ^= bit << 15;
        if (crc & 0x8000) != 0 {
            crc = (crc << 1).wrapping_add(0) ^ GEN_POLY;
        } else {
            crc <<= 1;
        }
    }
    crc
}

/// CRC-16 ITU-T over a bit-per-byte slice: each `input[i] & 1` is one bit.
/// `crc` is the initial CRC value.  
/// Processes the first `number_bits` entries of `input`.
// Was: Führt den Arbeitsschritt `crc16_itut_bits` für crc16 itut bits aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn crc16_itut_bits(mut crc: u16, input: &[u8], number_bits: usize) -> u16 {
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for &b in input.iter().take(number_bits) {
        let bit = (b & 1) as u16;
        crc ^= bit << 15;
        if (crc & 0x8000) != 0 {
            crc = (crc << 1).wrapping_add(0) ^ GEN_POLY;
        } else {
            crc <<= 1;
        }
    }
    crc
}

/// Standard CRC-ITU-T (initial 0xffff) over a bit-per-byte slice, as it is used in TETRA.
// Was: Führt den Arbeitsschritt `crc16_ccitt_bits` für crc16 ccitt bits aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn crc16_ccitt_bits(input: &[u8], len: usize) -> u16 {
    crc16_itut_bits(0xffff, input, len)
}
