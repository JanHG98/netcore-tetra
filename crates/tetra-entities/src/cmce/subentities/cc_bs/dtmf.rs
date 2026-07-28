// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use super::*;

// TODO: This should probably be in U/D-Info
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für dtmf kind auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub(super) enum DtmfKind {
    /// ETSI EN 300 392-2 V3.x: DTMF type = 000 (digits present)
    ToneStart,
    /// ETSI EN 300 392-2 V3.x: DTMF type = 001
    ToneEnd,
    /// ETSI EN 300 392-2 V3.x: DTMF type = 010
    NotSupported,
    /// ETSI EN 300 392-2 V3.x: DTMF type = 011
    NotSubscribed,
    /// ETSI EN 300 392-2 V3.x: reserved values 100..111
    Reserved(u8),
    /// Legacy edition-1 style payload (length divisible by 4): digits only, no 3-bit type.
    LegacyDigits,
    /// Payload could not be interpreted according to either format.
    Invalid,
}

#[derive(Clone, Debug, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für dtmf decoded in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub(super) struct DtmfDecoded {
    pub(super) kind: DtmfKind,
    pub(super) digits: String,
    pub(super) parsed_bits: usize,
    pub(super) full_len_bits: usize,
    pub(super) malformed: bool,
}

#[inline]
// Was: Diese Funktion dekodiert dtmf digit.
// Warum: Empfangene Protokolldaten müssen vor der weiteren Nutzung eindeutig verstanden und geprüft werden.
fn decode_dtmf_digit(nibble: u8) -> Option<char> {
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match nibble {
        0..=9 => Some(char::from(b'0' + nibble)),
        0x0a => Some('*'),
        0x0b => Some('#'),
        0x0c => Some('A'),
        0x0d => Some('B'),
        0x0e => Some('C'),
        0x0f => Some('D'),
        _ => None,
    }
}

#[inline]
// Was: Führt den Arbeitsschritt `type3_read_bit` für type3 read bit aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn type3_read_bit(field: &Type3FieldGeneric, bit_idx: usize) -> Option<u8> {
    let len_bits = field.len.min(128);
    if bit_idx >= len_bits {
        return None;
    }
    let shift = len_bits - 1 - bit_idx;
    Some(((field.data >> shift) & 0x01) as u8)
}

#[inline]
// Was: Führt den Arbeitsschritt `type3_read_bits` für type3 read bits aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn type3_read_bits(field: &Type3FieldGeneric, start_bit: usize, num_bits: usize) -> Option<u64> {
    if num_bits > 64 || start_bit + num_bits > field.len {
        return None;
    }

    let mut value = 0u64;
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for i in 0..num_bits {
        value = (value << 1) | type3_read_bit(field, start_bit + i)? as u64;
    }
    Some(value)
}

// Was: Diese Funktion dekodiert dtmf.
// Warum: Empfangene Protokolldaten müssen vor der weiteren Nutzung eindeutig verstanden und geprüft werden.
pub(super) fn decode_dtmf(field: &Type3FieldGeneric) -> DtmfDecoded {
    let full_len_bits = field.len;
    let len_bits = full_len_bits.min(128);
    if len_bits == 0 {
        return DtmfDecoded {
            kind: DtmfKind::Invalid,
            digits: String::new(),
            parsed_bits: 0,
            full_len_bits,
            malformed: true,
        };
    }

    // Legacy mechanism (edition-1): payload is 4-bit digit nibbles only.
    // ETSI EN 300 392-2 V3.x note: new mechanism length is not divisible by 4.
    if len_bits % 4 == 0 {
        let nibble_count = len_bits / 4;
        let mut digits = String::with_capacity(nibble_count);
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for i in 0..nibble_count {
            let nibble = type3_read_bits(field, i * 4, 4).unwrap_or(0) as u8;
            if let Some(c) = decode_dtmf_digit(nibble) {
                digits.push(c);
            }
        }
        return DtmfDecoded {
            kind: DtmfKind::LegacyDigits,
            digits,
            parsed_bits: len_bits,
            full_len_bits,
            malformed: len_bits != full_len_bits,
        };
    }

    if len_bits < 3 {
        return DtmfDecoded {
            kind: DtmfKind::Invalid,
            digits: String::new(),
            parsed_bits: len_bits,
            full_len_bits,
            malformed: true,
        };
    }

    let dtmf_type = type3_read_bits(field, 0, 3).unwrap_or(0) as u8;
    let tail_bits = len_bits - 3;

    let mut digits = String::new();
    let mut malformed = len_bits != full_len_bits;
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    let kind = match dtmf_type {
        0 => {
            if tail_bits == 0 || tail_bits % 4 != 0 {
                malformed = true;
            } else {
                let nibble_count = tail_bits / 4;
                digits.reserve(nibble_count);
                // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
                // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
                for i in 0..nibble_count {
                    let nibble = type3_read_bits(field, 3 + i * 4, 4).unwrap_or(0) as u8;
                    if let Some(c) = decode_dtmf_digit(nibble) {
                        digits.push(c);
                    }
                }
            }
            DtmfKind::ToneStart
        }
        1 => {
            if tail_bits != 0 {
                malformed = true;
            }
            DtmfKind::ToneEnd
        }
        2 => {
            if tail_bits != 0 {
                malformed = true;
            }
            DtmfKind::NotSupported
        }
        3 => {
            if tail_bits != 0 {
                malformed = true;
            }
            DtmfKind::NotSubscribed
        }
        4..=7 => {
            if tail_bits != 0 {
                malformed = true;
            }
            DtmfKind::Reserved(dtmf_type)
        }
        _ => DtmfKind::Invalid,
    };

    DtmfDecoded {
        kind,
        digits,
        parsed_bits: len_bits,
        full_len_bits,
        malformed,
    }
}

// Was: Führt den Arbeitsschritt `pack_type3_bits_to_bytes` für pack type3 bits to bytes aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub(super) fn pack_type3_bits_to_bytes(field: &Type3FieldGeneric) -> (u16, Vec<u8>) {
    let len_bits = field.len.min(128);
    if len_bits == 0 {
        return (0, Vec::new());
    }

    let mut out = vec![0u8; len_bits.div_ceil(8)];
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for bit_idx in 0..len_bits {
        let bit = type3_read_bit(field, bit_idx).unwrap_or(0);
        let byte_idx = bit_idx / 8;
        let bit_pos = 7 - (bit_idx % 8);
        out[byte_idx] |= bit << bit_pos;
    }
    (len_bits as u16, out)
}
