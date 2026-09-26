// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use tetra_core::BitBuffer;

/// Scrambling/unscrambling functions type5 <-> type4
/// See Clause 8.3

/// Default scrambling code for BSCH channel
// Was: Legt den festen Wert `SCRAMB_INIT` für scramb init fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SCRAMB_INIT: u32 = 3;

/// Generate one LFSR bit (Fibonacci form, taps at 32,26,23,22,16,12,11,10,8,7,5,4,2,1).
#[inline]
// Was: Führt den Arbeitsschritt `next_lfsr_bit` für next lfsr bit aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn next_lfsr_bit(lfsr: &mut u32) -> u8 {
    let x = *lfsr;
    let bit = (x
        ^ (x >> (32 - 26))
        ^ (x >> (32 - 23))
        ^ (x >> (32 - 22))
        ^ (x >> (32 - 16))
        ^ (x >> (32 - 12))
        ^ (x >> (32 - 11))
        ^ (x >> (32 - 10))
        ^ (x >> (32 - 8))
        ^ (x >> (32 - 7))
        ^ (x >> (32 - 5))
        ^ (x >> (32 - 4))
        ^ (x >> (32 - 2))
        ^ (x >> (32 - 1)))
        & 1;
    *lfsr = (x >> 1) | (bit << 31);
    bit as u8
}

/// Fill `out[0..]` with `len = out.len()` raw scrambling bits
// Was: Führt den Arbeitsschritt `tetra_scramb_get_bits` für TETRA scramb get bits aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn tetra_scramb_get_bits(mut lfsr_init: u32, out: &mut [u8]) {
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for slot in out.iter_mut() {
        *slot = next_lfsr_bit(&mut lfsr_init);
    }
}

// /// XOR `out[0..]` in‐place with the scrambling bits
// pub fn tetra_scramb_bits(mut lfsr_init: u32, out: &mut [u8]) {

//     for byte in out.iter_mut() {
//         *byte ^= next_lfsr_bit(&mut lfsr_init);
//     }
// }

/// Scramble or unscramble the given BitBuffer.
/// Generate lfsr sequence for given lfsr initialization value
/// and XOR it with the given buffer, for all bits from the current
/// position to the end of the buffer. Resets position to the old
/// initial position when done.
// Was: Führt den Arbeitsschritt `tetra_scramb_bits` für TETRA scramb bits aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn tetra_scramb_bits(mut lfsr_init: u32, buf: &mut BitBuffer) {
    let num_bits = buf.get_len_remaining() as isize;
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for _ in 0..num_bits {
        let bit = next_lfsr_bit(&mut lfsr_init);
        buf.xor_bit(bit);
    }
    // Reset pos
    buf.seek_rel(-num_bits);
}

/// Compute the initial LFSR state from (mcc, mnc, colour).
// Was: Führt den Arbeitsschritt `tetra_scramb_get_init` für TETRA scramb get init aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn tetra_scramb_get_init(mcc: u16, mnc: u16, colour: u8) -> u32 {
    if colour == 0 {
        // See Clause 21.4.4.2, cc 0 means all 30 bits of scrambling code are 0
        return 0;
    }

    (((colour as u32) | ((mnc as u32) << 6) | ((mcc as u32) << 20)) << 2) | SCRAMB_INIT
}
