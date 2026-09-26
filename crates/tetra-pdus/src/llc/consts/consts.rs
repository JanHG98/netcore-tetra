// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

// Numbers as defined in Annex A.2 LLC constants

///  This is the maximum length of one TL-SDU if the optional Frame Check Sequence (FCS) is used.
///  Default value = 2 595 bits (i.e. approximately 324 octets).
///  The FCS is optional. If the FCS is not used, the TL-SDU part may be larger by four octets.
// Was: Legt den festen Wert `N251_BL_MAX_TLSDU_LEN_BITS` für n251 bl max tlsdu len bits fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const N251_BL_MAX_TLSDU_LEN_BITS: u32 = 2595;

/// MS designer choice from range 1 to 5 if the stealing repeats flag is not set.
// Was: Legt den festen Wert `N252_BL_MAX_TLSDU_RETRANSMITS_ACKED` für n252 bl max tlsdu retransmits acked fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const N252_BL_MAX_TLSDU_RETRANSMITS_ACKED: u8 = 3;

/// MS designer choice from range 3 to 5 if the stealing repeats flag is set.
// Was: Legt den festen Wert `N252_BL_MAX_TLSDU_RETRANSMITS_ACKED_STEALING_REPEATS` für n252 bl max tlsdu retransmits acked stealing und weitere Angaben fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const N252_BL_MAX_TLSDU_RETRANSMITS_ACKED_STEALING_REPEATS: u32 = 3;

/// MS designer choice from range 0 to 5.
/// NOTE 1: The service user may indicate the required number of TL-SDU repetitions for a particular TL-SDU in the
/// unacknowledged basic link service. The value of N.253 chosen by the MS designer applies when the
/// service user does not indicate the required number of repetitions.
// Was: Legt den festen Wert `N253_BL_MAX_TLSDU_REPETITIONS_UNACKED` für n253 bl max tlsdu repetitions unacked fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const N253_BL_MAX_TLSDU_REPETITIONS_UNACKED: u32 = 3;

/// MS designer choice from range 1 to 5.
// Was: Legt den festen Wert `N262_AL_MAX_CONNECTION_SETUP_RETRIES` für n262 al max connection setup retries fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const N262_AL_MAX_CONNECTION_SETUP_RETRIES: u32 = 3;

/// MS designer choice from range 3 to 5.
// Was: Legt den festen Wert `N263_AL_MAX_DISCONNECTION_RETRIES` für n263 al max disconnection retries fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const N263_AL_MAX_DISCONNECTION_RETRIES: u32 = 3;

/// This value may be defined during the set-up of the advanced link (see AL-SETUP definition). Range: 1 to 4.
// Was: Legt den festen Wert `N264_AL_NUM_DQPSK_TIMESLOTS` für n264 al num dqpsk timeslots fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const N264_AL_NUM_DQPSK_TIMESLOTS: u32 = 4;

/// MS designer choice from range 0 to 5.
// Was: Legt den festen Wert `N265_AL_MAX_RECONNECTION_RETRIES` für n265 al max reconnection retries fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const N265_AL_MAX_RECONNECTION_RETRIES: u32 = 3;

/// This is the maximum length of one TL-SDU including the FCS, it is defined during the set-up of the advanced
/// link (see AL-SETUP PDU definition), Range: (32, 4 096) octets.
// Was: Legt den festen Wert `N271_AL_MAX_TLSDU_LEN` für n271 al max tlsdu len fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const N271_AL_MAX_TLSDU_LEN: u32 = 4096;

/// This value is defined during the set-up of the advanced link, (see AL-SETUP definition).
///  Range: (1;3) for the original advanced link.
///  Range: (1;15) for an extended advanced link.
// Was: Legt den festen Wert `N272_AL_WINDOW_SIZE_TLSDU_ACKED` für n272 al window size tlsdu acked fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const N272_AL_WINDOW_SIZE_TLSDU_ACKED: u32 = 3;

/// This value is defined during the set-up of the advanced link (see AL-SETUP definition). Range: (0;7).
// Was: Legt den festen Wert `N273_AL_MAX_TLSDU_RETRANSMISSIONS` für n273 al max tlsdu retransmissions fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const N273_AL_MAX_TLSDU_RETRANSMISSIONS: u32 = 3;

/// This value is defined during the set-up of the advanced link, (see AL-SETUP definition). Range: (0;15).
// Was: Legt den festen Wert `N274_AL_MAX_SEGMENT_RETRANSMISSIONS` für n274 al max segment retransmissions fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const N274_AL_MAX_SEGMENT_RETRANSMISSIONS: u32 = 3;

/// This value is defined during the set-up of the advanced link (see AL-SETUP definition).
/// Range: (1;3) for the original advanced link.
/// Range: (1;15) for an extended advanced link.
// Was: Legt den festen Wert `N281_AL_WINDOW_SIZE_TLSDU_UNACKED` für n281 al window size tlsdu unacked fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const N281_AL_WINDOW_SIZE_TLSDU_UNACKED: u32 = 3;

/// This value is defined during the set-up of the advanced link (see AL-SETUP definition). Range: (0;7).
// Was: Legt den festen Wert `N282_AL_NUM_REPETITIONS_UNACKED` für n282 al num repetitions unacked fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const N282_AL_NUM_REPETITIONS_UNACKED: u32 = 3;

/// MS designer choice from range 0 to 5.
/// NOTE 2: The MAC may indicate the required number of repetitions of a particular layer 2 signalling PDU. The
/// value of N.293 chosen by the MS designer applies when the MAC does not indicate the required number
/// of repetitions.
/// NOTE 3: It is recommended that N.293 is set to 0 in most cases.
// Was: Legt den festen Wert `N293_AL_NUM_REPETITIONS_LAYER2_SIGNALLING_PDU` für n293 al num repetitions layer2 signalling Protokollnachricht (PDU) fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const N293_AL_NUM_REPETITIONS_LAYER2_SIGNALLING_PDU: u32 = 3;
