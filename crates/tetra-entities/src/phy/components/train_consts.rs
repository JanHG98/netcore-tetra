// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

// Was: Legt den festen Wert `HALFSLOT_TYPE4_BITS` für halfslot type4 bits fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const HALFSLOT_TYPE4_BITS: usize = 255; // TODO FIXME check if this is indeed type4
// Was: Legt den festen Wert `TIMESLOT_TYPE4_BITS` für timeslot type4 bits fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const TIMESLOT_TYPE4_BITS: usize = 255 * 2; // TODO FIXME check if this is indeed type4

// Was: Legt den festen Wert `SEQ_SYNC_OFFSET` für seq sync offset fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SEQ_SYNC_OFFSET: usize = 214;
// Was: Legt den festen Wert `SEQ_NORM_DL_OFFSET` für seq norm dl offset fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SEQ_NORM_DL_OFFSET: usize = 244;
// Was: Legt den festen Wert `SEQ_NORM_UL_OFFSET` für seq norm ul offset fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SEQ_NORM_UL_OFFSET: usize = 254;
// Was: Legt den festen Wert `SEQ_EXT_OFFSET_SSB1` für seq ext offset ssb1 fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SEQ_EXT_OFFSET_SSB1: usize = 122;
// Was: Legt den festen Wert `SEQ_EXT_OFFSET_SSB2` für seq ext offset ssb2 fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SEQ_EXT_OFFSET_SSB2: usize = 122 + HALFSLOT_TYPE4_BITS;

/* 9.4.4.3.2 Normal Training Sequence */
/// 22 n-bits
// Was: Legt den festen Wert `SEQ_NORM1_AS_ARR` für seq norm1 as arr fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SEQ_NORM1_AS_ARR: [u8; 22] = [1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0];
/// 22 p-bits
// Was: Legt den festen Wert `SEQ_NORM2_AS_ARR` für seq norm2 as arr fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SEQ_NORM2_AS_ARR: [u8; 22] = [0, 1, 1, 1, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0];
/// 22 q-bits
// Was: Legt den festen Wert `SEQ_NORM3_AS_ARR` für seq norm3 as arr fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SEQ_NORM3_AS_ARR: [u8; 22] = [1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1];
/// 30 x-bits
// Was: Legt den festen Wert `SEQ_EXT_AS_ARR` für seq ext as arr fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SEQ_EXT_AS_ARR: [u8; 30] = [
    1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1,
];
/// 38 y-bits
// Was: Legt den festen Wert `SEQ_SYNC_AS_ARR` für seq sync as arr fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SEQ_SYNC_AS_ARR: [u8; 38] = [
    1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1,
];

// Was: Legt den festen Wert `SEQ_NORM1` für seq norm1 fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SEQ_NORM1: u64 = 0b1101000011101001110100;
// Was: Legt den festen Wert `SEQ_NORM2` für seq norm2 fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SEQ_NORM2: u64 = 0b0111101001000011011110;
// Was: Legt den festen Wert `SEQ_NORM3` für seq norm3 fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SEQ_NORM3: u64 = 0b1011011100000110101101;
// Was: Legt den festen Wert `SEQ_NORM_LEN` für seq norm len fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SEQ_NORM_LEN: usize = 22;

// /* 9.4.4.3.3 Extended training sequence */
// Was: Legt den festen Wert `SEQ_EXT` für seq ext fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SEQ_EXT: u64 = 0b100111010000111010011101000011; // 30 bits
// Was: Legt den festen Wert `SEQ_EXT_LEN` für seq ext len fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SEQ_EXT_LEN: usize = 30;

// /* 9.4.4.3.4 Synchronization training sequence */
// Was: Legt den festen Wert `SEQ_SYNC` für seq sync fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SEQ_SYNC: u64 = 0b11000001100111001110100111000001100111; // 38 bits
// Was: Legt den festen Wert `SEQ_SYNC_LEN` für seq sync len fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SEQ_SYNC_LEN: usize = 38;

/* 9.4.4.3.5 Tail bits */
// Was: Legt den festen Wert `T_BITS` für t bits fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const T_BITS: u64 = 0b1100;
