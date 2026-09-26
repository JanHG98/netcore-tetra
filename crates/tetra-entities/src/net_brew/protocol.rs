// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! Brew protocol binary message parsing and serialization (2-byte [kind, type] prefix, little-endian)

use uuid::Uuid;

// ─── Message classes ───────────────────────────────────────────────

// Was: Legt den festen Wert `BREW_CLASS_SUBSCRIBER` für Brew-Verbindung class Teilnehmer fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const BREW_CLASS_SUBSCRIBER: u8 = 0xf0;
// Was: Legt den festen Wert `BREW_CLASS_CALL_CONTROL` für Brew-Verbindung class Ruf Steuerung fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const BREW_CLASS_CALL_CONTROL: u8 = 0xf1;
// Was: Legt den festen Wert `BREW_CLASS_FRAME` für Brew-Verbindung class Funkrahmen fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const BREW_CLASS_FRAME: u8 = 0xf2;
// Was: Legt den festen Wert `BREW_CLASS_ERROR` für Brew-Verbindung class error fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const BREW_CLASS_ERROR: u8 = 0xf3;
// Was: Legt den festen Wert `BREW_CLASS_SERVICE` für Brew-Verbindung class Dienst fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const BREW_CLASS_SERVICE: u8 = 0xf4;

// ─── Subscriber control types (0xf0) ──────────────────────────────

// Was: Legt den festen Wert `BREW_SUBSCRIBER_DEREGISTER` für Brew-Verbindung Teilnehmer deregister fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const BREW_SUBSCRIBER_DEREGISTER: u8 = 0;
// Was: Legt den festen Wert `BREW_SUBSCRIBER_REGISTER` für Brew-Verbindung Teilnehmer register fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const BREW_SUBSCRIBER_REGISTER: u8 = 1;
// Was: Legt den festen Wert `BREW_SUBSCRIBER_REREGISTER` für Brew-Verbindung Teilnehmer reregister fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const BREW_SUBSCRIBER_REREGISTER: u8 = 2;
// Was: Legt den festen Wert `BREW_SUBSCRIBER_AFFILIATE` für Brew-Verbindung Teilnehmer affiliate fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const BREW_SUBSCRIBER_AFFILIATE: u8 = 8;
// Was: Legt den festen Wert `BREW_SUBSCRIBER_DEAFFILIATE` für Brew-Verbindung Teilnehmer deaffiliate fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const BREW_SUBSCRIBER_DEAFFILIATE: u8 = 9;

// ─── Call control types (0xf1) ────────────────────────────────────

// Was: Legt den festen Wert `CALL_STATE_GROUP_TX` für Ruf Zustand Gruppe tx fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const CALL_STATE_GROUP_TX: u8 = 2;
// Was: Legt den festen Wert `CALL_STATE_GROUP_IDLE` für Ruf Zustand Gruppe idle fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const CALL_STATE_GROUP_IDLE: u8 = 3;
// Was: Legt den festen Wert `CALL_STATE_SETUP_REQUEST` für Ruf Zustand setup request fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const CALL_STATE_SETUP_REQUEST: u8 = 4;
// Was: Legt den festen Wert `CALL_STATE_SETUP_ACCEPT` für Ruf Zustand setup accept fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const CALL_STATE_SETUP_ACCEPT: u8 = 5;
// Was: Legt den festen Wert `CALL_STATE_SETUP_REJECT` für Ruf Zustand setup reject fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const CALL_STATE_SETUP_REJECT: u8 = 6;
// Was: Legt den festen Wert `CALL_STATE_CALL_ALERT` für Ruf Zustand Ruf alert fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const CALL_STATE_CALL_ALERT: u8 = 7;
// Was: Legt den festen Wert `CALL_STATE_CONNECT_REQUEST` für Ruf Zustand connect request fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const CALL_STATE_CONNECT_REQUEST: u8 = 8;
// Was: Legt den festen Wert `CALL_STATE_CONNECT_CONFIRM` für Ruf Zustand connect confirm fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const CALL_STATE_CONNECT_CONFIRM: u8 = 9;
// Was: Legt den festen Wert `CALL_STATE_CALL_RELEASE` für Ruf Zustand Ruf release fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const CALL_STATE_CALL_RELEASE: u8 = 10;
// Was: Legt den festen Wert `CALL_STATE_SHORT_TRANSFER` für Ruf Zustand short transfer fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const CALL_STATE_SHORT_TRANSFER: u8 = 11;
// Was: Legt den festen Wert `CALL_STATE_SIMPLEX_GRANTED` für Ruf Zustand simplex granted fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const CALL_STATE_SIMPLEX_GRANTED: u8 = 12;
// Was: Legt den festen Wert `CALL_STATE_SIMPLEX_IDLE` für Ruf Zustand simplex idle fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const CALL_STATE_SIMPLEX_IDLE: u8 = 13;

// ─── Frame types (0xf2) ──────────────────────────────────────────

// Was: Legt den festen Wert `FRAME_TYPE_TRAFFIC_CHANNEL` für Funkrahmen type Nutzdatenverkehr Kanal fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const FRAME_TYPE_TRAFFIC_CHANNEL: u8 = 0;
// Was: Legt den festen Wert `FRAME_TYPE_SDS_TRANSFER` für Funkrahmen type TETRA-Kurznachricht (SDS) transfer fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const FRAME_TYPE_SDS_TRANSFER: u8 = 1;
// Was: Legt den festen Wert `FRAME_TYPE_SDS_REPORT` für Funkrahmen type TETRA-Kurznachricht (SDS) report fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const FRAME_TYPE_SDS_REPORT: u8 = 2;
// Was: Legt den festen Wert `FRAME_TYPE_DTMF_DATA` für Funkrahmen type dtmf data fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const FRAME_TYPE_DTMF_DATA: u8 = 3;
// Was: Legt den festen Wert `FRAME_TYPE_PACKET_DATA` für Funkrahmen type Datenpaket data fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const FRAME_TYPE_PACKET_DATA: u8 = 4;

// ─── Circuit/individual call wire format ─────────────────────────
// Was: Legt den festen Wert `CIRCULAR_NUMBER_LEN` für circular number len fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const CIRCULAR_NUMBER_LEN: usize = 32;
/// Total wire size of BrewCircularCall payload: source(4)+dest(4)+number(32)+11 single-byte fields
// Was: Legt den festen Wert `CIRCULAR_CALL_LEN` für circular Ruf len fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const CIRCULAR_CALL_LEN: usize = 4 + 4 + CIRCULAR_NUMBER_LEN + 11;

// ─── Error types (0xf3) ──────────────────────────────────────────

// Was: Legt den festen Wert `BREW_TYPE_MALFORMED` für Brew-Verbindung type malformed fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const BREW_TYPE_MALFORMED: u8 = 0;
// Was: Legt den festen Wert `BREW_TYPE_RESTRICTED` für Brew-Verbindung type restricted fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const BREW_TYPE_RESTRICTED: u8 = 1;

// ─── Parsed message types ─────────────────────────────────────────

/// Top-level parsed Brew message
#[derive(Debug, Clone)]
// Was: Listet die möglichen Varianten für Brew-Verbindung Nachricht auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum BrewMessage {
    Subscriber(BrewSubscriberMessage),
    CallControl(BrewCallControlMessage),
    Frame(BrewFrameMessage),
    Error(BrewErrorMessage),
    Service(BrewServiceMessage),
}

/// Subscriber control (0xf0)
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für Brew-Verbindung Teilnehmer Nachricht in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct BrewSubscriberMessage {
    pub msg_type: u8,
    pub number: u32,      // ISSI
    pub time: u64,        // UNIX timestamp
    pub fraction: u32,    // Nanoseconds
    pub groups: Vec<u32>, // GSSIs (for affiliate/deaffiliate)
}

/// Group transmission data, part of CALL_STATE_GROUP_TX
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für Brew-Verbindung Gruppe transmission in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct BrewGroupTransmission {
    pub source: u32,      // ISSI of caller
    pub destination: u32, // GSSI of group
    pub priority: u8,
    pub access: u8,
    pub service: u16, // Speech service
    /// SS-TPI mnemonic name (Brew v1+, optional). Raw 34-byte field:
    /// byte 0: text coding scheme (7-bit), bytes 1+: encoded name.
    /// None when server is v0 or mnemonic not present in message.
    pub mnemonic: Option<[u8; 34]>,
}

/// Circuit/PBX/phone call data, part of SETUP_REQUEST / CONNECT_REQUEST
/// (ETSI EN 300 392-2 §14 individual call fields mapped to Brew wire format)
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für Brew-Verbindung circular Ruf in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct BrewCircularCall {
    pub source: u32,
    pub destination: u32,
    pub number: String, // ASCII, up to 32 bytes, NUL-padded
    pub priority: u8,
    pub service: u8,
    pub mode: u8,
    pub duplex: u8,
    pub method: u8,
    pub communication: u8,
    pub grant: u8,
    pub permission: u8,
    pub timeout: u8,
    pub ownership: u8,
    pub queued: u8,
    /// SS-TPI mnemonic name (Brew v1+, optional). Present in SETUP_REQUEST only.
    /// None when server is v0 or mnemonic not present.
    pub mnemonic: Option<[u8; 34]>,
}

/// Circuit grant payload, part of CONNECT_CONFIRM / SIMPLEX_* states
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für Brew-Verbindung circular grant in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct BrewCircularGrant {
    pub grant: u8,
    pub permission: u8,
}

/// Call control (0xf1)
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für Brew-Verbindung Ruf Steuerung Nachricht in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct BrewCallControlMessage {
    pub call_state: u8,
    pub identifier: Uuid, // Call session UUID (16 bytes)
    pub payload: BrewCallPayload,
}

/// Union-like payload for call control messages
#[derive(Debug, Clone)]
// Was: Listet die möglichen Varianten für Brew-Verbindung Ruf payload auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum BrewCallPayload {
    /// CALL_STATE_GROUP_TX
    GroupTransmission(BrewGroupTransmission),
    /// CALL_STATE_GROUP_IDLE, CALL_STATE_SETUP_REJECT, CALL_STATE_CALL_RELEASE
    Cause(u8),
    /// CALL_STATE_SETUP_ACCEPT, CALL_STATE_CALL_ALERT — no extra payload
    Empty,
    /// CALL_STATE_SHORT_TRANSFER (SDS header)
    ShortTransfer { source: u32, destination: u32 },
    /// CALL_STATE_SETUP_REQUEST, CALL_STATE_CONNECT_REQUEST (individual/circuit call)
    CircularCall(BrewCircularCall),
    /// CALL_STATE_CONNECT_CONFIRM, CALL_STATE_SIMPLEX_GRANTED, CALL_STATE_SIMPLEX_IDLE
    CircularGrant(BrewCircularGrant),
    /// Unknown/unhandled call state
    Raw(Vec<u8>),
}

/// Voice and data frames (0xf2)
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für Brew-Verbindung Funkrahmen Nachricht in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct BrewFrameMessage {
    pub frame_type: u8,
    pub identifier: Uuid, // Call session UUID
    pub length_bits: u16, // Length of data in bits
    pub data: Vec<u8>,
}

/// Error messages (0xf3)
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für Brew-Verbindung error Nachricht in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct BrewErrorMessage {
    pub error_type: u8,
    pub data: Vec<u8>,
}

/// Service messages (0xf4)
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für Brew-Verbindung Dienst Nachricht in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct BrewServiceMessage {
    pub service_type: u8,
    pub json_data: String,
}

// ─── Parsing ──────────────────────────────────────────────────────

/// Parse error
#[derive(Debug)]
// Was: Listet die möglichen Varianten für Brew-Verbindung parse error auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum BrewParseError {
    TooShort(usize),
    UnknownClass(u8),
    InvalidUtf8,
    InvalidUuid,
}

// Was: Implementiert das zugehörige Verhalten für `std::fmt::Display for BrewParseError`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::fmt::Display for BrewParseError {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            Self::TooShort(n) => write!(f, "message too short: {} bytes", n),
            Self::UnknownClass(c) => write!(f, "unknown message class: 0x{:02x}", c),
            Self::InvalidUtf8 => write!(f, "invalid UTF-8 in service message"),
            Self::InvalidUuid => write!(f, "invalid UUID in call control message"),
        }
    }
}

/// Read a little-endian u16 from a byte slice
// Was: Diese Funktion liest u16 le.
// Warum: Der Datenzugriff wird dadurch einheitlich behandelt und Fehler können zentral gemeldet werden.
fn read_u16_le(data: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([data[offset], data[offset + 1]])
}

/// Read a little-endian u32 from a byte slice
// Was: Diese Funktion liest u32 le.
// Warum: Der Datenzugriff wird dadurch einheitlich behandelt und Fehler können zentral gemeldet werden.
fn read_u32_le(data: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]])
}

/// Read a little-endian u64 from a byte slice
// Was: Diese Funktion liest u64 le.
// Warum: Der Datenzugriff wird dadurch einheitlich behandelt und Fehler können zentral gemeldet werden.
fn read_u64_le(data: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes([
        data[offset],
        data[offset + 1],
        data[offset + 2],
        data[offset + 3],
        data[offset + 4],
        data[offset + 5],
        data[offset + 6],
        data[offset + 7],
    ])
}

/// Write a little-endian u16 to a byte vec
// Was: Diese Funktion schreibt u16 le.
// Warum: Die Ausgabe wird dadurch einheitlich erzeugt und Schreibfehler können behandelt werden.
fn write_u16_le(buf: &mut Vec<u8>, val: u16) {
    buf.extend_from_slice(&val.to_le_bytes());
}

/// Write a little-endian u32 to a byte vec
// Was: Diese Funktion schreibt u32 le.
// Warum: Die Ausgabe wird dadurch einheitlich erzeugt und Schreibfehler können behandelt werden.
fn write_u32_le(buf: &mut Vec<u8>, val: u32) {
    buf.extend_from_slice(&val.to_le_bytes());
}

/// Write a little-endian u64 to a byte vec
// Was: Diese Funktion schreibt u64 le.
// Warum: Die Ausgabe wird dadurch einheitlich erzeugt und Schreibfehler können behandelt werden.
fn write_u64_le(buf: &mut Vec<u8>, val: u64) {
    buf.extend_from_slice(&val.to_le_bytes());
}

/// Parse a raw binary Brew message into a typed BrewMessage
// Was: Diese Funktion liest und prüft Brew-Verbindung Nachricht.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
pub fn parse_brew_message(data: &[u8]) -> Result<BrewMessage, BrewParseError> {
    if data.len() < 2 {
        return Err(BrewParseError::TooShort(data.len()));
    }

    let kind = data[0];
    let msg_type = data[1];

    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match kind {
        BREW_CLASS_SUBSCRIBER => parse_subscriber(msg_type, data),
        BREW_CLASS_CALL_CONTROL => parse_call_control(msg_type, data),
        BREW_CLASS_FRAME => parse_frame(msg_type, data),
        BREW_CLASS_ERROR => parse_error(msg_type, data),
        BREW_CLASS_SERVICE => parse_service(msg_type, data),
        _ => Err(BrewParseError::UnknownClass(kind)),
    }
}

// Was: Diese Funktion liest und prüft Teilnehmer.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
fn parse_subscriber(msg_type: u8, data: &[u8]) -> Result<BrewMessage, BrewParseError> {
    // Minimum: kind(1) + type(1) + number(4) + time(8) + fraction(4) = 18 bytes
    if data.len() < 18 {
        return Err(BrewParseError::TooShort(data.len()));
    }

    let number = read_u32_le(data, 2);
    let time = read_u64_le(data, 6);
    let fraction = read_u32_le(data, 14);

    // Remaining bytes are GSSIs (4 bytes each) for affiliate/deaffiliate
    let mut groups = Vec::new();
    let mut offset = 18;
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    while offset + 4 <= data.len() {
        groups.push(read_u32_le(data, offset));
        offset += 4;
    }

    Ok(BrewMessage::Subscriber(BrewSubscriberMessage {
        msg_type,
        number,
        time,
        fraction,
        groups,
    }))
}

// Was: Diese Funktion liest und prüft fixed ascii.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
fn parse_fixed_ascii(bytes: &[u8]) -> String {
    let end = bytes.iter().position(|b| *b == 0).unwrap_or(bytes.len());
    bytes[..end].iter().copied().filter(|b| b.is_ascii()).map(char::from).collect()
}

// Was: Diese Funktion schreibt fixed ascii.
// Warum: Die Ausgabe wird dadurch einheitlich erzeugt und Schreibfehler können behandelt werden.
fn write_fixed_ascii(buf: &mut Vec<u8>, value: &str, width: usize) {
    let bytes = value.as_bytes();
    let copy_len = bytes.len().min(width);
    buf.extend_from_slice(&bytes[..copy_len]);
    if width > copy_len {
        buf.resize(buf.len() + (width - copy_len), 0);
    }
}

// Was: Diese Funktion liest und prüft Ruf Steuerung.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
fn parse_call_control(call_state: u8, data: &[u8]) -> Result<BrewMessage, BrewParseError> {
    // Minimum: kind(1) + type(1) + uuid(16) = 18 bytes
    if data.len() < 18 {
        return Err(BrewParseError::TooShort(data.len()));
    }

    let uuid_bytes: [u8; 16] = data[2..18].try_into().map_err(|_| BrewParseError::InvalidUuid)?;
    let identifier = Uuid::from_bytes(uuid_bytes);

    let payload_data = &data[18..];

    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    let payload = match call_state {
        CALL_STATE_GROUP_TX => {
            // v0: source(4)+dest(4)+priority(1)+access(1)+service(2) = 12 bytes
            // v1: same + mnemonic(34) = 46 bytes
            if payload_data.len() < 12 {
                return Err(BrewParseError::TooShort(data.len()));
            }
            let mnemonic = if payload_data.len() >= 46 && payload_data[13] > 0 {
                // byte 12 = coding scheme, byte 13 = length in bits; 0 bits = no mnemonic
                let mut m = [0u8; 34];
                m.copy_from_slice(&payload_data[12..46]);
                Some(m)
            } else {
                None
            };
            BrewCallPayload::GroupTransmission(BrewGroupTransmission {
                source: read_u32_le(payload_data, 0),
                destination: read_u32_le(payload_data, 4),
                priority: payload_data[8],
                access: payload_data[9],
                service: read_u16_le(payload_data, 10),
                mnemonic,
            })
        }

        CALL_STATE_GROUP_IDLE | CALL_STATE_SETUP_REJECT | CALL_STATE_CALL_RELEASE => {
            // Single byte cause
            if payload_data.is_empty() {
                return Err(BrewParseError::TooShort(data.len()));
            }
            BrewCallPayload::Cause(payload_data[0])
        }

        CALL_STATE_SETUP_REQUEST | CALL_STATE_CONNECT_REQUEST => {
            // v0: source(4)+dest(4)+number(32)+11 single-byte fields = 51 bytes
            // v1 SETUP_REQUEST: same + mnemonic(34) = 85 bytes (CONNECT_REQUEST has no mnemonic)
            if payload_data.len() < CIRCULAR_CALL_LEN {
                return Err(BrewParseError::TooShort(data.len()));
            }
            let mnemonic = if call_state == CALL_STATE_SETUP_REQUEST
                && payload_data.len() >= CIRCULAR_CALL_LEN + 34
                && payload_data[CIRCULAR_CALL_LEN + 1] > 0
            {
                // byte 0 = coding scheme, byte 1 = length in bits; 0 bits = no mnemonic
                let mut m = [0u8; 34];
                m.copy_from_slice(&payload_data[CIRCULAR_CALL_LEN..CIRCULAR_CALL_LEN + 34]);
                Some(m)
            } else {
                None
            };
            BrewCallPayload::CircularCall(BrewCircularCall {
                source: read_u32_le(payload_data, 0),
                destination: read_u32_le(payload_data, 4),
                number: parse_fixed_ascii(&payload_data[8..8 + CIRCULAR_NUMBER_LEN]),
                priority: payload_data[40],
                service: payload_data[41],
                mode: payload_data[42],
                duplex: payload_data[43],
                method: payload_data[44],
                communication: payload_data[45],
                grant: payload_data[46],
                permission: payload_data[47],
                timeout: payload_data[48],
                ownership: payload_data[49],
                queued: payload_data[50],
                mnemonic,
            })
        }

        CALL_STATE_SETUP_ACCEPT | CALL_STATE_CALL_ALERT => {
            // No extra payload
            BrewCallPayload::Empty
        }

        CALL_STATE_CONNECT_CONFIRM | CALL_STATE_SIMPLEX_GRANTED | CALL_STATE_SIMPLEX_IDLE => {
            if payload_data.len() < 2 {
                return Err(BrewParseError::TooShort(data.len()));
            }
            BrewCallPayload::CircularGrant(BrewCircularGrant {
                grant: payload_data[0],
                permission: payload_data[1],
            })
        }

        CALL_STATE_SHORT_TRANSFER => {
            // BrewShortData: source(4) + destination(4) + number[32](char) = 40 bytes
            if payload_data.len() < 8 {
                return Err(BrewParseError::TooShort(data.len()));
            }
            BrewCallPayload::ShortTransfer {
                source: read_u32_le(payload_data, 0),
                destination: read_u32_le(payload_data, 4),
            }
        }

        _ => {
            // Store raw for unhandled types
            BrewCallPayload::Raw(payload_data.to_vec())
        }
    };

    Ok(BrewMessage::CallControl(BrewCallControlMessage {
        call_state,
        identifier,
        payload,
    }))
}

// Was: Diese Funktion liest und prüft Funkrahmen.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
fn parse_frame(frame_type: u8, data: &[u8]) -> Result<BrewMessage, BrewParseError> {
    // kind(1) + type(1) + uuid(16) + length(2) = 20 bytes minimum
    if data.len() < 20 {
        return Err(BrewParseError::TooShort(data.len()));
    }

    let uuid_bytes: [u8; 16] = data[2..18].try_into().map_err(|_| BrewParseError::InvalidUuid)?;
    let identifier = Uuid::from_bytes(uuid_bytes);

    let length_bits = read_u16_le(data, 18);
    let frame_data = data[20..].to_vec();

    Ok(BrewMessage::Frame(BrewFrameMessage {
        frame_type,
        identifier,
        length_bits,
        data: frame_data,
    }))
}

// Was: Diese Funktion liest und prüft error.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
fn parse_error(error_type: u8, data: &[u8]) -> Result<BrewMessage, BrewParseError> {
    Ok(BrewMessage::Error(BrewErrorMessage {
        error_type,
        data: data[2..].to_vec(),
    }))
}

// Was: Diese Funktion liest und prüft Dienst.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
fn parse_service(service_type: u8, data: &[u8]) -> Result<BrewMessage, BrewParseError> {
    // Data is NULL-terminated JSON
    let json_bytes = &data[2..];
    // Find NULL terminator or use full length
    let end = json_bytes.iter().position(|&b| b == 0).unwrap_or(json_bytes.len());
    let json_str = std::str::from_utf8(&json_bytes[..end]).map_err(|_| BrewParseError::InvalidUtf8)?;

    Ok(BrewMessage::Service(BrewServiceMessage {
        service_type,
        json_data: json_str.to_string(),
    }))
}

// ─── Building (outgoing messages) ─────────────────────────────────

/// Build a subscriber registration message

// ─── Circuit / individual call serializers ────────────────────────────────

// Was: Diese Funktion erstellt circular Ruf.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
fn build_circular_call(call_state: u8, session_uuid: &Uuid, call: &BrewCircularCall) -> Vec<u8> {
    // v1 mnemonic is only sent in SETUP_REQUEST, not CONNECT_REQUEST
    let include_mnemonic = call_state == CALL_STATE_SETUP_REQUEST && call.mnemonic.is_some();
    let cap = 2 + 16 + CIRCULAR_CALL_LEN + if include_mnemonic { 34 } else { 0 };
    let mut buf = Vec::with_capacity(cap);
    buf.push(BREW_CLASS_CALL_CONTROL);
    buf.push(call_state);
    buf.extend_from_slice(session_uuid.as_bytes());
    write_u32_le(&mut buf, call.source);
    write_u32_le(&mut buf, call.destination);
    write_fixed_ascii(&mut buf, &call.number, CIRCULAR_NUMBER_LEN);
    buf.push(call.priority);
    buf.push(call.service);
    buf.push(call.mode);
    buf.push(call.duplex);
    buf.push(call.method);
    buf.push(call.communication);
    buf.push(call.grant);
    buf.push(call.permission);
    buf.push(call.timeout);
    buf.push(call.ownership);
    buf.push(call.queued);
    if include_mnemonic {
        if let Some(m) = &call.mnemonic {
            buf.extend_from_slice(m);
        }
    }
    buf
}

/// Build SETUP_REQUEST for circuit/PBX/phone call (ETSI 14.7.1 BS→TetraPack).
// Was: Diese Funktion erstellt setup request.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_setup_request(session_uuid: &Uuid, call: &BrewCircularCall) -> Vec<u8> {
    build_circular_call(CALL_STATE_SETUP_REQUEST, session_uuid, call)
}

/// Build CONNECT_REQUEST for circuit/PBX/phone call (ETSI 14.7.5 BS→TetraPack).
// Was: Diese Funktion erstellt connect request.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_connect_request(session_uuid: &Uuid, call: &BrewCircularCall) -> Vec<u8> {
    build_circular_call(CALL_STATE_CONNECT_REQUEST, session_uuid, call)
}

/// Build SETUP_ACCEPT — no payload (ETSI 14.7.2 BS→TetraPack).
// Was: Diese Funktion erstellt setup accept.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_setup_accept(session_uuid: &Uuid) -> Vec<u8> {
    let mut buf = Vec::with_capacity(18);
    buf.push(BREW_CLASS_CALL_CONTROL);
    buf.push(CALL_STATE_SETUP_ACCEPT);
    buf.extend_from_slice(session_uuid.as_bytes());
    buf
}

/// Build CALL_ALERT — no payload (ETSI 14.7.3 BS→TetraPack, called MS ringing).
// Was: Diese Funktion erstellt Ruf alert.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_call_alert(session_uuid: &Uuid) -> Vec<u8> {
    let mut buf = Vec::with_capacity(18);
    buf.push(BREW_CLASS_CALL_CONTROL);
    buf.push(CALL_STATE_CALL_ALERT);
    buf.extend_from_slice(session_uuid.as_bytes());
    buf
}

/// Build SETUP_REJECT with disconnect cause (ETSI 14.7.2 BS→TetraPack).
// Was: Diese Funktion erstellt setup reject.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_setup_reject(session_uuid: &Uuid, cause: u8) -> Vec<u8> {
    let mut buf = Vec::with_capacity(19);
    buf.push(BREW_CLASS_CALL_CONTROL);
    buf.push(CALL_STATE_SETUP_REJECT);
    buf.extend_from_slice(session_uuid.as_bytes());
    buf.push(cause);
    buf
}

/// Build CONNECT_CONFIRM with grant/permission (ETSI 14.7.6 BS→TetraPack).
// Was: Diese Funktion erstellt connect confirm.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_connect_confirm(session_uuid: &Uuid, grant: u8, permission: u8) -> Vec<u8> {
    let mut buf = Vec::with_capacity(20);
    buf.push(BREW_CLASS_CALL_CONTROL);
    buf.push(CALL_STATE_CONNECT_CONFIRM);
    buf.extend_from_slice(session_uuid.as_bytes());
    buf.push(grant);
    buf.push(permission);
    buf
}

/// Build SIMPLEX_GRANTED with grant/permission.
// Was: Diese Funktion erstellt simplex granted.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_simplex_granted(session_uuid: &Uuid, grant: u8, permission: u8) -> Vec<u8> {
    let mut buf = Vec::with_capacity(20);
    buf.push(BREW_CLASS_CALL_CONTROL);
    buf.push(CALL_STATE_SIMPLEX_GRANTED);
    buf.extend_from_slice(session_uuid.as_bytes());
    buf.push(grant);
    buf.push(permission);
    buf
}

/// Build SIMPLEX_IDLE with grant/permission.
// Was: Diese Funktion erstellt simplex idle.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_simplex_idle(session_uuid: &Uuid, grant: u8, permission: u8) -> Vec<u8> {
    let mut buf = Vec::with_capacity(20);
    buf.push(BREW_CLASS_CALL_CONTROL);
    buf.push(CALL_STATE_SIMPLEX_IDLE);
    buf.extend_from_slice(session_uuid.as_bytes());
    buf.push(grant);
    buf.push(permission);
    buf
}

/// Build CALL_RELEASE with disconnect cause (ETSI 14.7.x both directions).
// Was: Diese Funktion erstellt Ruf release.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_call_release(session_uuid: &Uuid, cause: u8) -> Vec<u8> {
    let mut buf = Vec::with_capacity(19);
    buf.push(BREW_CLASS_CALL_CONTROL);
    buf.push(CALL_STATE_CALL_RELEASE);
    buf.extend_from_slice(session_uuid.as_bytes());
    buf.push(cause);
    buf
}

/// Build DTMF data frame message.
// Was: Diese Funktion erstellt dtmf Funkrahmen.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_dtmf_frame(session_uuid: &Uuid, length_bits: u16, data: &[u8]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(20 + data.len());
    buf.push(BREW_CLASS_FRAME);
    buf.push(FRAME_TYPE_DTMF_DATA);
    buf.extend_from_slice(session_uuid.as_bytes());
    write_u16_le(&mut buf, length_bits);
    buf.extend_from_slice(data);
    buf
}

// Was: Diese Funktion erstellt Teilnehmer register.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_subscriber_register(issi: u32, groups: &[u32]) -> Vec<u8> {
    build_subscriber_register_with_type(issi, groups, BREW_SUBSCRIBER_REGISTER)
}

// Was: Diese Funktion erstellt Teilnehmer register with type.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_subscriber_register_with_type(issi: u32, groups: &[u32], msg_type: u8) -> Vec<u8> {
    build_subscriber_message(issi, msg_type, groups)
}

// Was: Diese Funktion erstellt Teilnehmer Nachricht.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
fn build_subscriber_message(issi: u32, msg_type: u8, groups: &[u32]) -> Vec<u8> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();

    let mut buf = Vec::with_capacity(18 + groups.len() * 4);
    buf.push(BREW_CLASS_SUBSCRIBER);
    buf.push(msg_type);
    write_u32_le(&mut buf, issi);
    write_u64_le(&mut buf, now.as_secs());
    write_u32_le(&mut buf, now.subsec_nanos());
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for &gssi in groups {
        write_u32_le(&mut buf, gssi);
    }
    buf
}

/// Build a subscriber re-registration message (for already-registered subscribers)
// Was: Diese Funktion erstellt Teilnehmer reregister.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_subscriber_reregister(issi: u32) -> Vec<u8> {
    build_subscriber_reregister_with_type(issi, BREW_SUBSCRIBER_REREGISTER)
}

// Was: Diese Funktion erstellt Teilnehmer reregister with type.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_subscriber_reregister_with_type(issi: u32, msg_type: u8) -> Vec<u8> {
    build_subscriber_message(issi, msg_type, &[])
}

/// Build a subscriber affiliation message
// Was: Diese Funktion erstellt Teilnehmer affiliate.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_subscriber_affiliate(issi: u32, groups: &[u32]) -> Vec<u8> {
    build_subscriber_affiliate_with_type(issi, groups, BREW_SUBSCRIBER_AFFILIATE)
}

// Was: Diese Funktion erstellt Teilnehmer affiliate with type.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_subscriber_affiliate_with_type(issi: u32, groups: &[u32], msg_type: u8) -> Vec<u8> {
    build_subscriber_message(issi, msg_type, groups)
}

/// Build a subscriber deaffiliation message
// Was: Diese Funktion erstellt Teilnehmer deaffiliate.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_subscriber_deaffiliate(issi: u32, groups: &[u32]) -> Vec<u8> {
    build_subscriber_deaffiliate_with_type(issi, groups, BREW_SUBSCRIBER_DEAFFILIATE)
}

// Was: Diese Funktion erstellt Teilnehmer deaffiliate with type.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_subscriber_deaffiliate_with_type(issi: u32, groups: &[u32], msg_type: u8) -> Vec<u8> {
    build_subscriber_message(issi, msg_type, groups)
}

/// Build a subscriber deregistration message
// Was: Diese Funktion erstellt Teilnehmer deregister.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_subscriber_deregister(issi: u32) -> Vec<u8> {
    build_subscriber_deregister_with_type(issi, BREW_SUBSCRIBER_DEREGISTER)
}

// Was: Diese Funktion erstellt Teilnehmer deregister with type.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_subscriber_deregister_with_type(issi: u32, msg_type: u8) -> Vec<u8> {
    build_subscriber_message(issi, msg_type, &[])
}

/// Build a group call transmission start message (GROUP_TX)
/// Sent when a local radio starts transmitting on a subscribed group.
/// `mnemonic` is the optional SS-TPI talking party name (Brew v1, 34 bytes raw).
// Was: Diese Funktion erstellt Gruppe tx.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_group_tx(
    session_uuid: &Uuid,
    source_issi: u32,
    dest_gssi: u32,
    priority: u8,
    service: u16,
    mnemonic: Option<&[u8; 34]>,
) -> Vec<u8> {
    // v0: kind(1)+type(1)+uuid(16)+source(4)+dest(4)+priority(1)+access(1)+service(2) = 30
    // v1: same + mnemonic(34) = 64
    let cap = if mnemonic.is_some() { 64 } else { 30 };
    let mut buf = Vec::with_capacity(cap);
    buf.push(BREW_CLASS_CALL_CONTROL);
    buf.push(CALL_STATE_GROUP_TX);
    buf.extend_from_slice(session_uuid.as_bytes());
    write_u32_le(&mut buf, source_issi);
    write_u32_le(&mut buf, dest_gssi);
    buf.push(priority);
    buf.push(0); // access = 0 (normal)
    write_u16_le(&mut buf, service);
    if let Some(m) = mnemonic {
        buf.extend_from_slice(m);
    }
    buf
}

/// Build a voice frame message (ACELP traffic channel data)
/// `data` should be packed ACELP bits (1 bit per byte in STE format, with
/// a leading STE header byte prepended by the caller if needed)
// Was: Diese Funktion erstellt voice Funkrahmen.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_voice_frame(session_uuid: &Uuid, length_bits: u16, data: &[u8]) -> Vec<u8> {
    // kind(1) + type(1) + uuid(16) + length(2) + data = 20 + data.len()
    let mut buf = Vec::with_capacity(20 + data.len());
    buf.push(BREW_CLASS_FRAME);
    buf.push(FRAME_TYPE_TRAFFIC_CHANNEL);
    buf.extend_from_slice(session_uuid.as_bytes());
    write_u16_le(&mut buf, length_bits);
    buf.extend_from_slice(data);
    buf
}

/// Build a group call idle (hangup) message
// Was: Diese Funktion erstellt Gruppe idle.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_group_idle(session_uuid: &Uuid, cause: u8) -> Vec<u8> {
    let mut buf = Vec::with_capacity(19);
    buf.push(BREW_CLASS_CALL_CONTROL);
    buf.push(CALL_STATE_GROUP_IDLE);
    buf.extend_from_slice(session_uuid.as_bytes());
    buf.push(cause);
    buf
}

/// Build a CALL_STATE_SHORT_TRANSFER message (SDS header with source/dest/number)
// Was: Diese Funktion erstellt short transfer.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_short_transfer(session_uuid: &Uuid, source: u32, destination: u32) -> Vec<u8> {
    // kind(1) + type(1) + uuid(16) + source(4) + destination(4) + number[32](1 byte each) = 58
    let mut buf = Vec::with_capacity(58);
    buf.push(BREW_CLASS_CALL_CONTROL);
    buf.push(CALL_STATE_SHORT_TRANSFER);
    buf.extend_from_slice(session_uuid.as_bytes());
    write_u32_le(&mut buf, source);
    write_u32_le(&mut buf, destination);
    // number field: 32 bytes, zero-filled (external subscriber number not supported)
    buf.extend_from_slice(&[0u8; 32]);
    buf
}

/// Build a FRAME_TYPE_SDS_TRANSFER message (SDS Type 4 PDU payload)
// Was: Diese Funktion erstellt TETRA-Kurznachricht (SDS) Funkrahmen.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_sds_frame(session_uuid: &Uuid, length_bits: u16, data: &[u8]) -> Vec<u8> {
    // kind(1) + type(1) + uuid(16) + length(2) + data = 20 + data.len()
    let mut buf = Vec::with_capacity(20 + data.len());
    buf.push(BREW_CLASS_FRAME);
    buf.push(FRAME_TYPE_SDS_TRANSFER);
    buf.extend_from_slice(session_uuid.as_bytes());
    write_u16_le(&mut buf, length_bits);
    buf.extend_from_slice(data);
    buf
}

/// Build a FRAME_TYPE_SDS_REPORT message (delivery acknowledgement)
/// Wire: kind(1) + type(1) + uuid(16) + length_bits(2) + status(1) = 21 bytes
// Was: Diese Funktion erstellt TETRA-Kurznachricht (SDS) report.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_sds_report(session_uuid: &Uuid, status: u8) -> Vec<u8> {
    let mut buf = Vec::with_capacity(21);
    buf.push(BREW_CLASS_FRAME);
    buf.push(FRAME_TYPE_SDS_REPORT);
    buf.extend_from_slice(session_uuid.as_bytes());
    write_u16_le(&mut buf, 8); // length_bits = 8 (1 byte status)
    buf.push(status);
    buf
}

/// Service type for RSSI measurements
// Was: Legt den festen Wert `BREW_SERVICE_RSSI` für Brew-Verbindung Dienst rssi fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const BREW_SERVICE_RSSI: u8 = 0x10;

/// Build a Service (0xf4) RSSI update message.
///
/// Sends the current RSSI reading for an MS to the Brew server as JSON:
/// `{"issi": 2260570, "rssi_dbfs": -42.3}`
///
/// Service type 0x10 is used to distinguish RSSI messages from subscriber
/// query messages (type 0x01). The JSON is NULL-terminated per SmartConnect convention.
// Was: Diese Funktion erstellt Dienst rssi.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_service_rssi(issi: u32, rssi_dbfs: f32) -> Vec<u8> {
    let json = format!("{{\"issi\":{},\"rssi_dbfs\":{:.1}}}", issi, rssi_dbfs);
    let mut buf = Vec::with_capacity(3 + json.len());
    buf.push(BREW_CLASS_SERVICE);
    buf.push(BREW_SERVICE_RSSI);
    buf.extend_from_slice(json.as_bytes());
    buf.push(0); // NULL terminator
    buf
}

/// Build a query subscribers service message
// Was: Diese Funktion erstellt query subscribers.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_query_subscribers(issis: &[u32]) -> Vec<u8> {
    let json = serde_json::to_string(issis).unwrap_or_else(|_| "[]".to_string());
    let mut buf = Vec::with_capacity(3 + json.len());
    buf.push(BREW_CLASS_SERVICE);
    buf.push(1); // Query subscribers type
    buf.extend_from_slice(json.as_bytes());
    buf.push(0); // NULL terminator
    buf
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;

    #[test]
    // Was: Prüft automatisch den Fall parse Gruppe tx v0 no mnemonic.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_parse_group_tx_v0_no_mnemonic() {
        let uuid = Uuid::new_v4();
        let mut data = vec![BREW_CLASS_CALL_CONTROL, CALL_STATE_GROUP_TX];
        data.extend_from_slice(uuid.as_bytes());
        write_u32_le(&mut data, 1001);
        write_u32_le(&mut data, 26);
        data.push(3); // priority
        data.push(0); // access
        write_u16_le(&mut data, 0); // service

        let msg = parse_brew_message(&data).unwrap();
        if let BrewMessage::CallControl(cc) = msg {
            if let BrewCallPayload::GroupTransmission(gt) = cc.payload {
                assert_eq!(gt.source, 1001);
                assert!(gt.mnemonic.is_none(), "v0 should have no mnemonic");
            } else {
                panic!("Expected GroupTransmission");
            }
        } else {
            panic!("Expected CallControl");
        }
    }

    #[test]
    // Was: Prüft automatisch den Fall parse Gruppe tx v1 with mnemonic.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_parse_group_tx_v1_with_mnemonic() {
        let uuid = Uuid::new_v4();
        let mut data = vec![BREW_CLASS_CALL_CONTROL, CALL_STATE_GROUP_TX];
        data.extend_from_slice(uuid.as_bytes());
        write_u32_le(&mut data, 1001);
        write_u32_le(&mut data, 26);
        data.push(3); // priority
        data.push(0); // access
        write_u16_le(&mut data, 0); // service
        // mnemonic: coding_scheme=0x01 (ISO-8859-1), length=32 bits (4 chars), "TEST"
        let mut mnemonic = [0u8; 34];
        mnemonic[0] = 0x01; // coding scheme
        mnemonic[1] = 32; // length in bits
        mnemonic[2..6].copy_from_slice(b"TEST");
        data.extend_from_slice(&mnemonic);

        let msg = parse_brew_message(&data).unwrap();
        if let BrewMessage::CallControl(cc) = msg {
            if let BrewCallPayload::GroupTransmission(gt) = cc.payload {
                assert_eq!(gt.source, 1001);
                let m = gt.mnemonic.expect("v1 should have mnemonic");
                assert_eq!(m[0], 0x01);
                assert_eq!(&m[2..6], b"TEST");
            } else {
                panic!("Expected GroupTransmission");
            }
        } else {
            panic!("Expected CallControl");
        }
    }

    #[test]
    // Was: Prüft automatisch den Fall parse setup request v1 with mnemonic.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_parse_setup_request_v1_with_mnemonic() {
        let uuid = Uuid::new_v4();
        let mut data = vec![BREW_CLASS_CALL_CONTROL, CALL_STATE_SETUP_REQUEST];
        data.extend_from_slice(uuid.as_bytes());
        // BrewCircularCall: source(4)+dest(4)+number(32)+11 bytes
        write_u32_le(&mut data, 2001); // source
        write_u32_le(&mut data, 3001); // destination
        let mut number = [0u8; 32];
        number[..3].copy_from_slice(b"600");
        data.extend_from_slice(&number);
        data.extend_from_slice(&[0u8; 11]); // 11 single-byte fields
        // mnemonic
        let mut mnemonic = [0u8; 34];
        mnemonic[0] = 0x01;
        mnemonic[1] = 40;
        mnemonic[2..7].copy_from_slice(b"RADIO");
        data.extend_from_slice(&mnemonic);

        let msg = parse_brew_message(&data).unwrap();
        if let BrewMessage::CallControl(cc) = msg {
            if let BrewCallPayload::CircularCall(c) = cc.payload {
                assert_eq!(c.source, 2001);
                let m = c.mnemonic.expect("v1 SETUP_REQUEST should have mnemonic");
                assert_eq!(&m[2..7], b"RADIO");
            } else {
                panic!("Expected CircularCall");
            }
        } else {
            panic!("Expected CallControl");
        }
    }

    #[test]
    // Was: Prüft automatisch den Fall build Gruppe tx v1 roundtrip.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_build_group_tx_v1_roundtrip() {
        let uuid = Uuid::new_v4();
        let mut mnemonic = [0u8; 34];
        mnemonic[0] = 0x01;
        mnemonic[1] = 24;
        mnemonic[2..5].copy_from_slice(b"YO6");

        let built = build_group_tx(&uuid, 9001, 26, 2, 0, Some(&mnemonic));
        assert_eq!(built.len(), 64, "v1 GROUP_TX should be 64 bytes");

        let msg = parse_brew_message(&built).unwrap();
        if let BrewMessage::CallControl(cc) = msg {
            if let BrewCallPayload::GroupTransmission(gt) = cc.payload {
                assert_eq!(gt.source, 9001);
                let m = gt.mnemonic.unwrap();
                assert_eq!(&m[2..5], b"YO6");
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }

    #[test]
    // Was: Prüft automatisch den Fall parse voice Funkrahmen.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_parse_voice_frame() {
        let uuid = Uuid::new_v4();
        let mut data = vec![BREW_CLASS_FRAME, FRAME_TYPE_TRAFFIC_CHANNEL];
        data.extend_from_slice(uuid.as_bytes());
        write_u16_le(&mut data, 274); // length in bits
        // 36 bytes of fake ACELP data
        let acelp = vec![0x80; 36];
        data.extend_from_slice(&acelp);

        let msg = parse_brew_message(&data).unwrap();
        if let BrewMessage::Frame(frame) = msg {
            assert_eq!(frame.frame_type, FRAME_TYPE_TRAFFIC_CHANNEL);
            assert_eq!(frame.identifier, uuid);
            assert_eq!(frame.length_bits, 274);
            assert_eq!(frame.data.len(), 36);
        } else {
            panic!("Expected Frame message");
        }
    }

    #[test]
    // Was: Prüft automatisch den Fall parse short transfer.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_parse_short_transfer() {
        let uuid = Uuid::new_v4();
        let mut data = vec![BREW_CLASS_CALL_CONTROL, CALL_STATE_SHORT_TRANSFER];
        data.extend_from_slice(uuid.as_bytes());
        write_u32_le(&mut data, 5001); // source
        write_u32_le(&mut data, 6001); // destination
        // number field (32 bytes)
        let number_str = b"6001";
        data.extend_from_slice(number_str);
        data.resize(data.len() + (32 - number_str.len()), 0);

        let msg = parse_brew_message(&data).unwrap();
        if let BrewMessage::CallControl(cc) = msg {
            assert_eq!(cc.call_state, CALL_STATE_SHORT_TRANSFER);
            assert_eq!(cc.identifier, uuid);
            if let BrewCallPayload::ShortTransfer { source, destination } = cc.payload {
                assert_eq!(source, 5001);
                assert_eq!(destination, 6001);
            } else {
                panic!("Expected ShortTransfer payload");
            }
        } else {
            panic!("Expected CallControl message");
        }
    }

    #[test]
    // Was: Prüft automatisch den Fall build parse TETRA-Kurznachricht (SDS) Funkrahmen.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_build_parse_sds_frame() {
        let uuid = Uuid::new_v4();
        let payload = vec![0xAA, 0xBB, 0xCC, 0xDD];
        let built = build_sds_frame(&uuid, 32, &payload);

        let msg = parse_brew_message(&built).unwrap();
        if let BrewMessage::Frame(frame) = msg {
            assert_eq!(frame.frame_type, FRAME_TYPE_SDS_TRANSFER);
            assert_eq!(frame.identifier, uuid);
            assert_eq!(frame.length_bits, 32);
            assert_eq!(frame.data, payload);
        } else {
            panic!("Expected Frame message");
        }
    }

    #[test]
    // Was: Prüft automatisch den Fall build parse short transfer.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_build_parse_short_transfer() {
        let uuid = Uuid::new_v4();
        let built = build_short_transfer(&uuid, 1001, 2002);

        let msg = parse_brew_message(&built).unwrap();
        if let BrewMessage::CallControl(cc) = msg {
            assert_eq!(cc.call_state, CALL_STATE_SHORT_TRANSFER);
            assert_eq!(cc.identifier, uuid);
            if let BrewCallPayload::ShortTransfer { source, destination } = cc.payload {
                assert_eq!(source, 1001);
                assert_eq!(destination, 2002);
            } else {
                panic!("Expected ShortTransfer payload");
            }
        } else {
            panic!("Expected CallControl message");
        }
    }

    #[test]
    // Was: Prüft automatisch den Fall parse Gruppe idle.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_parse_group_idle() {
        let uuid = Uuid::new_v4();
        let mut data = vec![BREW_CLASS_CALL_CONTROL, CALL_STATE_GROUP_IDLE];
        data.extend_from_slice(uuid.as_bytes());
        data.push(0); // cause = normal

        let msg = parse_brew_message(&data).unwrap();
        if let BrewMessage::CallControl(cc) = msg {
            assert_eq!(cc.call_state, CALL_STATE_GROUP_IDLE);
            if let BrewCallPayload::Cause(cause) = cc.payload {
                assert_eq!(cause, 0);
            } else {
                panic!("Expected Cause payload");
            }
        } else {
            panic!("Expected CallControl message");
        }
    }

    #[test]
    // Was: Prüft automatisch den Fall build parse TETRA-Kurznachricht (SDS) report.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_build_parse_sds_report() {
        let uuid = Uuid::new_v4();
        let built = build_sds_report(&uuid, 0);

        assert_eq!(built.len(), 21);
        assert_eq!(built[0], BREW_CLASS_FRAME);
        assert_eq!(built[1], FRAME_TYPE_SDS_REPORT);

        let msg = parse_brew_message(&built).unwrap();
        if let BrewMessage::Frame(frame) = msg {
            assert_eq!(frame.frame_type, FRAME_TYPE_SDS_REPORT);
            assert_eq!(frame.identifier, uuid);
            assert_eq!(frame.length_bits, 8);
            assert_eq!(frame.data, vec![0]);
        } else {
            panic!("Expected Frame message");
        }
    }
}
