// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

// Was: Legt den festen Wert `DQPSK4_BITS_PER_SYM` für dqpsk4 bits per sym fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const DQPSK4_BITS_PER_SYM: usize = 2;

// Was: Legt den festen Wert `SB_BITS` für sb bits fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SB_BITS: usize = (6 + 1 + 40 + 60 + 19 + 15 + 108 + 1 + 5) * DQPSK4_BITS_PER_SYM;
// Was: Legt den festen Wert `SB_BLK1_OFFSET` für sb blk1 offset fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SB_BLK1_OFFSET: usize = (6 + 1 + 40) * DQPSK4_BITS_PER_SYM;
// Was: Legt den festen Wert `SB_BBK_OFFSET` für sb bbk offset fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SB_BBK_OFFSET: usize = (6 + 1 + 40 + 60 + 19) * DQPSK4_BITS_PER_SYM;
// Was: Legt den festen Wert `SB_BLK2_OFFSET` für sb blk2 offset fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SB_BLK2_OFFSET: usize = (6 + 1 + 40 + 60 + 19 + 15) * DQPSK4_BITS_PER_SYM;

// Was: Legt den festen Wert `SB_BLK1_BITS` für sb blk1 bits fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SB_BLK1_BITS: usize = 60 * DQPSK4_BITS_PER_SYM;
// Was: Legt den festen Wert `SB_BBK_BITS` für sb bbk bits fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SB_BBK_BITS: usize = 15 * DQPSK4_BITS_PER_SYM;
// Was: Legt den festen Wert `SB_BLK2_BITS` für sb blk2 bits fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SB_BLK2_BITS: usize = 108 * DQPSK4_BITS_PER_SYM;

// Was: Legt den festen Wert `NDB_BITS` für ndb bits fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const NDB_BITS: usize = (5 + 1 + 1 + 108 + 7 + 11 + 8 + 108 + 1 + 5) * DQPSK4_BITS_PER_SYM;
// Was: Legt den festen Wert `NDB_BLK1_OFFSET` für ndb blk1 offset fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const NDB_BLK1_OFFSET: usize = (5 + 1 + 1) * DQPSK4_BITS_PER_SYM;
// Was: Legt den festen Wert `NDB_BBK1_OFFSET` für ndb bbk1 offset fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const NDB_BBK1_OFFSET: usize = (5 + 1 + 1 + 108) * DQPSK4_BITS_PER_SYM;
// Was: Legt den festen Wert `NDB_BBK2_OFFSET` für ndb bbk2 offset fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const NDB_BBK2_OFFSET: usize = (5 + 1 + 1 + 108 + 7 + 11) * DQPSK4_BITS_PER_SYM;
// Was: Legt den festen Wert `NDB_BLK2_OFFSET` für ndb blk2 offset fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const NDB_BLK2_OFFSET: usize = (5 + 1 + 1 + 108 + 7 + 11 + 8) * DQPSK4_BITS_PER_SYM;

// Was: Legt den festen Wert `NDB_BBK1_BITS` für ndb bbk1 bits fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const NDB_BBK1_BITS: usize = 7 * DQPSK4_BITS_PER_SYM;
// Was: Legt den festen Wert `NDB_BBK2_BITS` für ndb bbk2 bits fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const NDB_BBK2_BITS: usize = 8 * DQPSK4_BITS_PER_SYM;
// Was: Legt den festen Wert `NDB_BLK_BITS` für ndb blk bits fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const NDB_BLK_BITS: usize = 108 * DQPSK4_BITS_PER_SYM;
// Was: Legt den festen Wert `NDB_BBK_BITS` für ndb bbk bits fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const NDB_BBK_BITS: usize = SB_BBK_BITS;

// Was: Legt den festen Wert `CUB_BITS` für cub bits fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const CUB_BITS: usize = 4 + 84 + 30 + 84 + 4;
// Was: Legt den festen Wert `CUB_BLK_BITS` für cub blk bits fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const CUB_BLK_BITS: usize = 84;
// Was: Legt den festen Wert `CUB_HEADBITS_OFFSET` für cub headbits offset fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const CUB_HEADBITS_OFFSET: usize = 34;
// Was: Legt den festen Wert `CUB_BLK1_OFFSET` für cub blk1 offset fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const CUB_BLK1_OFFSET: usize = 4;
// Was: Legt den festen Wert `CUB_TRAINING_OFFSET` für cub training offset fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const CUB_TRAINING_OFFSET: usize = 4 + 84;
// Was: Legt den festen Wert `CUB_BLK2_OFFSET` für cub blk2 offset fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const CUB_BLK2_OFFSET: usize = 4 + 84 + 30;
// Was: Legt den festen Wert `CUB_TAILBITS_OFFSET` für cub tailbits offset fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const CUB_TAILBITS_OFFSET: usize = 4 + 84 + 30 + 84;
// Was: Legt den festen Wert `CUB_BURST_BITS` für cub burst bits fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const CUB_BURST_BITS: usize = CUB_TAILBITS_OFFSET + 4;

// Was: Legt den festen Wert `NUB_BITS` für nub bits fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const NUB_BITS: usize = 4 + 216 + 22 + 216 + 4;
// Was: Legt den festen Wert `NUB_BLK_BITS` für nub blk bits fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const NUB_BLK_BITS: usize = 216;
// Was: Legt den festen Wert `NUB_HEADBITS_OFFSET` für nub headbits offset fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const NUB_HEADBITS_OFFSET: usize = 34;
// Was: Legt den festen Wert `NUB_BLK1_OFFSET` für nub blk1 offset fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const NUB_BLK1_OFFSET: usize = 4;
// Was: Legt den festen Wert `NUB_TRAINING_OFFSET` für nub training offset fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const NUB_TRAINING_OFFSET: usize = 4 + 216;
// Was: Legt den festen Wert `NUB_BLK2_OFFSET` für nub blk2 offset fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const NUB_BLK2_OFFSET: usize = 4 + 216 + 22;
// Was: Legt den festen Wert `NUB_TAILBITS_OFFSET` für nub tailbits offset fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const NUB_TAILBITS_OFFSET: usize = 4 + 216 + 22 + 216;
// Was: Legt den festen Wert `NUB_BURST_BITS` für nub burst bits fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const NUB_BURST_BITS: usize = NUB_TAILBITS_OFFSET + 4;
