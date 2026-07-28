// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Nachrichten an den Schnittstellen zwischen TETRA-Protokollschichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! Shared, strongly typed service-access-point values used by TLMC and LTPD.
//!
//! The types in this module model local primitives from ETSI EN 300 392-2.
//! They are deliberately independent from any future network transport.  The
//! TLMC and LTPD SAPs remain in-process boundaries inside the TBS.

use core::fmt;
use std::time::Duration;

use tetra_core::{BitBuffer, EndpointId, LinkId, SsiType, TetraAddress};

/// Local identifier that correlates a request with a later report or cancel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
// Was: Bündelt die zusammengehörigen Werte für request handle in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct RequestHandle(pub u32);

/// Local identifier for a MAC channel-change decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
// Was: Bündelt die zusammengehörigen Werte für Kanal change handle in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ChannelChangeHandle(pub u32);

/// Decision returned to MAC for a channel allocation that requested a response.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// Was: Listet die möglichen Varianten für Kanal change decision auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum ChannelChangeDecision {
    Accept,
    Reject,
    #[default]
    Ignore,
}

/// Current availability of a lower-layer resource identified by an endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für lower layer resource availability auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum LowerLayerResourceAvailability {
    Available,
    Unavailable,
}

/// Reason carried by an MLE-CONFIGURE indication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für lower layer resource reason auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum LowerLayerResourceReason {
    ReceptionStopped,
    TransmissionStopped,
    UsageMarkerMismatch,
    LossOfRadioResources,
    RecoveryOfRadioResources,
    Other(u8),
}

/// Type of TETRA cell used by local mobility management.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// Was: Listet die möglichen Varianten für cell type auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum CellType {
    #[default]
    ConventionalAccess,
    DirectAccess,
}

/// Service level currently offered by a cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// Was: Listet die möglichen Varianten für cell Dienst level auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum CellServiceLevel {
    NoService,
    GracefulServiceDegradation,
    #[default]
    NormalService,
}

/// Stable identity and radio reference for a cell candidate.
#[derive(Debug, Clone, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für cell identity in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CellIdentity {
    pub mcc: u16,
    pub mnc: u16,
    pub location_area: Option<u16>,
    pub colour_code: Option<u8>,
    pub main_carrier: u16,
    pub cell_type: CellType,
}

/// Unit used by a local measurement value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für measurement unit auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum MeasurementUnit {
    Db,
    Dbm,
    Raw,
}

/// Measurement with an explicit unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für measurement value in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct MeasurementValue {
    pub value: i16,
    pub unit: MeasurementUnit,
}

// Was: Implementiert das zugehörige Verhalten für `MeasurementValue`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl MeasurementValue {
    // Was: Führt den Arbeitsschritt `db` für Datenbank aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub const fn db(value: i16) -> Self {
        Self {
            value,
            unit: MeasurementUnit::Db,
        }
    }

    // Was: Führt den Arbeitsschritt `dbm` für dbm aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub const fn dbm(value: i16) -> Self {
        Self {
            value,
            unit: MeasurementUnit::Dbm,
        }
    }

    // Was: Führt den Arbeitsschritt `raw` für raw aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub const fn raw(value: i16) -> Self {
        Self {
            value,
            unit: MeasurementUnit::Raw,
        }
    }
}

/// Consolidated local measurement result used by mobility selection logic.
#[derive(Debug, Clone, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für measurement report in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct MeasurementReport {
    pub endpoint_id: Option<EndpointId>,
    pub channel_number: Option<RfChannelNumber>,
    pub path_loss_c1: Option<MeasurementValue>,
    pub path_loss_c2: Option<MeasurementValue>,
    pub path_loss_c3: Option<MeasurementValue>,
    pub path_loss_c4: Option<MeasurementValue>,
    pub path_loss_c5: Option<MeasurementValue>,
    pub quality: Option<QualityIndication>,
}

/// Candidate returned by monitoring or scanning.
#[derive(Debug, Clone, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für cell candidate in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CellCandidate {
    pub identity: Option<CellIdentity>,
    pub channel_number: RfChannelNumber,
    pub service_level: CellServiceLevel,
    pub measurements: MeasurementReport,
}

/// Correlation identifier for a local scan operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
// Was: Bündelt die zusammengehörigen Werte für scan request Kennung in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ScanRequestId(pub u32);

/// Reason why MLE asks MAC to select a channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für selection cause auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum SelectionCause {
    InitialCellSelection,
    AnnouncedReselectionType1,
    AnnouncedReselectionType2,
    AnnouncedReselectionType3,
    UnannouncedReselection,
    UndeclaredReselection,
    BaseStationControlledChannelChange,
    CallRestoration,
    Other(u8),
}

/// Outcome returned by TL-SELECT or related local selection procedures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für selection result auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum SelectionResult {
    Success,
    RandomAccessFailure,
    ReconnectionFailure,
    Reject,
    Other(u8),
}

/// Instruction to release a locally configured circuit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// Was: Listet die möglichen Varianten für Ruf release instruction auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum CallReleaseInstruction {
    #[default]
    Keep,
    Release,
}

/// Whether the U-plane is active for a circuit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// Was: Listet die möglichen Varianten für uplane switch auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum UPlaneSwitch {
    #[default]
    Off,
    On,
}

/// Current transmit grant associated with an operating-mode instruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// Was: Listet die möglichen Varianten für tx grant Zustand auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum TxGrantState {
    #[default]
    NotGranted,
    Granted,
}

/// Directionality of a circuit-mode resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// Was: Listet die möglichen Varianten für duplex mode auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum DuplexMode {
    #[default]
    Simplex,
    Duplex,
}

/// Circuit type selected for the local U-plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für circuit mode auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum CircuitMode {
    Speech,
    UnprotectedData72,
    LowProtectionData48,
    HighProtectionData24,
    Other(u8),
}

/// Complete local operating-mode instruction passed down to MAC.
#[derive(Debug, Clone, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für operating mode in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct OperatingMode {
    pub u_plane: UPlaneSwitch,
    pub tx_grant: TxGrantState,
    pub duplex: DuplexMode,
    pub circuit_mode: CircuitMode,
    pub interleaving_depth: Option<u8>,
    pub end_to_end_encrypted: bool,
    pub user_device: Option<u8>,
    pub endpoint_id: EndpointId,
}

/// Packet-data priority, including the ETSI "undefined" value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// Was: Listet die möglichen Varianten für data Priorität auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum DataPriority {
    Priority0,
    Priority1,
    Priority2,
    Priority3,
    Priority4,
    Priority5,
    Priority6,
    Priority7,
    #[default]
    Undefined,
}

// Was: Implementiert das zugehörige Verhalten für `DataPriority`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl DataPriority {
    // Was: Wandelt Eingangsdaten in raw um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn from_raw(value: u8) -> Option<Self> {
        Some(match value {
            0 => Self::Priority0,
            1 => Self::Priority1,
            2 => Self::Priority2,
            3 => Self::Priority3,
            4 => Self::Priority4,
            5 => Self::Priority5,
            6 => Self::Priority6,
            7 => Self::Priority7,
            _ => return None,
        })
    }

    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub const fn as_raw(self) -> Option<u8> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            Self::Priority0 => Some(0),
            Self::Priority1 => Some(1),
            Self::Priority2 => Some(2),
            Self::Priority3 => Some(3),
            Self::Priority4 => Some(4),
            Self::Priority5 => Some(5),
            Self::Priority6 => Some(6),
            Self::Priority7 => Some(7),
            Self::Undefined => None,
        }
    }
}

/// Local PDU priority (ETSI range 0 to 7).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// Was: Bündelt die zusammengehörigen Werte für Protokollnachricht (PDU) Priorität in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct PduPriority(u8);

// Was: Implementiert das zugehörige Verhalten für `PduPriority`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl PduPriority {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new(value: u8) -> Option<Self> {
        (value <= 7).then_some(Self(value))
    }

    // Was: Diese Funktion liest den vorgesehenen Arbeitsschritt.
    // Warum: Der Zugriff auf den Wert bleibt dadurch gekapselt und kann später zentral angepasst werden.
    pub const fn get(self) -> u8 {
        self.0
    }
}

// Was: Implementiert das zugehörige Verhalten für `Default for PduPriority`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for PduPriority {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self(0)
    }
}

/// SNDCP NSAPI (ETSI range 1 to 14).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// Was: Bündelt die zusammengehörigen Werte für SNDCP-Kontextkennung (NSAPI) in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct Nsapi(u8);

// Was: Implementiert das zugehörige Verhalten für `Nsapi`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Nsapi {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new(value: u8) -> Option<Self> {
        (1..=14).contains(&value).then_some(Self(value))
    }

    // Was: Diese Funktion liest den vorgesehenen Arbeitsschritt.
    // Warum: Der Zugriff auf den Wert bleibt dadurch gekapselt und kann später zentral angepasst werden.
    pub const fn get(self) -> u8 {
        self.0
    }
}

/// Data-priority random-access delay factor (ETSI range 0 to 7).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für data Priorität random access delay factor in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct DataPriorityRandomAccessDelayFactor(u8);

// Was: Implementiert das zugehörige Verhalten für `DataPriorityRandomAccessDelayFactor`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl DataPriorityRandomAccessDelayFactor {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new(value: u8) -> Option<Self> {
        (value <= 7).then_some(Self(value))
    }

    // Was: Diese Funktion liest den vorgesehenen Arbeitsschritt.
    // Warum: Der Zugriff auf den Wert bleibt dadurch gekapselt und kann später zentral angepasst werden.
    pub const fn get(self) -> u8 {
        self.0
    }
}

/// Data class visible to LLC/MAC.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// Was: Listet die möglichen Varianten für data class auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum DataClass {
    Background,
    Telemetry,
    RealTime,
    #[default]
    NonClassified,
    Other(u8),
}

/// Data-category reliability level used by lower-layer adaptation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für data category in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct DataCategory {
    pub class: DataClass,
    pub reliability_level: u8,
}

/// Original or extended advanced-link format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// Was: Listet die möglichen Varianten für advanced link format auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum AdvancedLinkFormat {
    #[default]
    Original,
    Extended,
}

/// Throughput information used during advanced-link QoS negotiation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// Was: Bündelt die zusammengehörigen Werte für throughput information in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ThroughputInformation {
    pub bits_per_second: Option<u32>,
    pub timeslots: Option<u8>,
}

/// Layer-2 QoS negotiated for an advanced link.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für layer2 Dienstgüte (QoS) in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct Layer2Qos {
    pub throughput: ThroughputInformation,
    pub link_format: AdvancedLinkFormat,
    pub acknowledged_window_size: u8,
    pub max_tl_sdu_retransmissions: u8,
    pub max_segment_retransmissions: u8,
}

// Was: Implementiert das zugehörige Verhalten für `Layer2Qos`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Layer2Qos {
    // Was: Diese Funktion prüft den vorgesehenen Arbeitsschritt.
    // Warum: Unzulässige Werte werden dadurch erkannt, bevor sie im Betrieb Schaden anrichten.
    pub fn validate(&self) -> Result<(), &'static str> {
        if !(1..=15).contains(&self.acknowledged_window_size) {
            return Err("acknowledged window size must be in 1..=15");
        }
        if self.max_tl_sdu_retransmissions > 7 {
            return Err("TL-SDU retransmissions must be in 0..=7");
        }
        if self.max_segment_retransmissions > 15 {
            return Err("segment retransmissions must be in 0..=15");
        }
        Ok(())
    }
}

// Was: Implementiert das zugehörige Verhalten für `Default for Layer2Qos`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for Layer2Qos {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            throughput: ThroughputInformation::default(),
            link_format: AdvancedLinkFormat::Original,
            acknowledged_window_size: 1,
            max_tl_sdu_retransmissions: 0,
            max_segment_retransmissions: 0,
        }
    }
}

/// Amount of data currently available for an advanced-link reservation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// Was: Bündelt die zusammengehörigen Werte für reservation info in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ReservationInfo {
    pub octets_available: u32,
}

/// Result of an MLE-UNITDATA transfer request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für transfer result auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum TransferResult {
    SuccessMoreDataBuffered,
    SuccessBufferEmpty,
    DelayedByGracefulDegradation,
    FailedRemovedFromBuffer,
    RejectedByEmergencyCall,
    Other(u8),
}

/// Result reported during advanced-link setup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für setup report auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum SetupReport {
    Success,
    ServiceChange,
    ParametersAcceptable,
    ParametersNotAcceptable,
    Other(u8),
}

/// SNDCP service state visible to MLE and lower layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// Was: Listet die möglichen Varianten für SNDCP-Paketdaten Status auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum SndcpStatus {
    #[default]
    Idle,
    Standby,
    Ready,
}

/// Sleep permission passed from a layer-3 user to MLE.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// Was: Listet die möglichen Varianten für sleep mode auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum SleepMode {
    StayAlive,
    #[default]
    SleepPermitted,
}

/// Channel-advice request flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// Was: Listet die möglichen Varianten für Kanal advice auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum ChannelAdvice {
    #[default]
    NotRequested,
    Requested,
}

/// Stealing urgency for a signalling SDU.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// Was: Listet die möglichen Varianten für stealing permission auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum StealingPermission {
    StealImmediately,
    StealWithinT214,
    StealWhenConvenient,
    #[default]
    NotRequired,
}

/// Scheduled-data classification for a TL-SDU.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// Was: Listet die möglichen Varianten für scheduled data Status auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum ScheduledDataStatus {
    #[default]
    NotScheduled,
    InitialScheduledData,
    ScheduledData,
}

/// Repetition request for an SNDCP schedule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für schedule repetition information in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ScheduleRepetitionInformation {
    pub nsapi: Nsapi,
    pub start: bool,
    pub repetition_period_slots: u16,
}

// Was: Implementiert das zugehörige Verhalten für `ScheduleRepetitionInformation`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl ScheduleRepetitionInformation {
    // Was: Diese Funktion prüft den vorgesehenen Arbeitsschritt.
    // Warum: Unzulässige Werte werden dadurch erkannt, bevor sie im Betrieb Schaden anrichten.
    pub fn validate(&self) -> Result<(), &'static str> {
        if !(4..=706).contains(&self.repetition_period_slots) {
            return Err("schedule repetition period must be in 4..=706 slots");
        }
        Ok(())
    }
}

/// Periodic reporting policy configured for the MS.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// Was: Listet die möglichen Varianten für periodic reporting Zeitüberwachung auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum PeriodicReportingTimer {
    #[default]
    Disabled,
    Interval(Duration),
    UseSwmiRequested,
}

/// Local indication whether any layer-3 entity or advanced link is active.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// Was: Listet die möglichen Varianten für MLE-Verbindungssteuerung activity indicator auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum MleActivityIndicator {
    #[default]
    Inactive,
    Active,
}

/// LLC timers measured in downlink signalling frames.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// Was: Bündelt die zusammengehörigen Werte für LLC-Verbindungsschicht Zeitüberwachung Status in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LlcTimerStatus {
    pub t251_running: bool,
    pub t252_running: bool,
    pub t261_running: bool,
    pub t263_running: bool,
    pub t265_running: bool,
}

/// Opaque link-performance score derived by LLC from acknowledgements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// Was: Bündelt die zusammengehörigen Werte für link performance information in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LinkPerformanceInformation {
    pub score: i16,
}

/// Control for graceful-service-degradation operation and repetitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für graceful Dienst degradation Steuerung in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct GracefulServiceDegradationControl {
    pub active: bool,
    pub repetition_count: u8,
    pub repetition_interval: Duration,
}

/// Energy-economy group 0 to 7.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für energy economy Gruppe in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct EnergyEconomyGroup(u8);

// Was: Implementiert das zugehörige Verhalten für `EnergyEconomyGroup`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl EnergyEconomyGroup {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new(value: u8) -> Option<Self> {
        (value <= 7).then_some(Self(value))
    }

    // Was: Diese Funktion liest den vorgesehenen Arbeitsschritt.
    // Warum: Der Zugriff auf den Wert bleibt dadurch gekapselt und kann später zentral angepasst werden.
    pub const fn get(self) -> u8 {
        self.0
    }
}

/// Absolute startpoint for an energy-economy or dual-watch cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für energy economy startpoint in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct EnergyEconomyStartpoint {
    pub frame: u8,
    pub multiframe: u8,
}

// Was: Implementiert das zugehörige Verhalten für `EnergyEconomyStartpoint`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl EnergyEconomyStartpoint {
    // Was: Diese Funktion prüft den vorgesehenen Arbeitsschritt.
    // Warum: Unzulässige Werte werden dadurch erkannt, bevor sie im Betrieb Schaden anrichten.
    pub fn validate(&self) -> Result<(), &'static str> {
        if !(1..=18).contains(&self.frame) {
            return Err("frame must be in 1..=18");
        }
        if !(1..=60).contains(&self.multiframe) {
            return Err("multiframe must be in 1..=60");
        }
        Ok(())
    }
}

/// Timeslot to monitor in frame 18 while minimum mode is active.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für frame18 distribution in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct Frame18Distribution {
    pub timeslot: u8,
}

/// SCCH selection information supplied by higher layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// Was: Bündelt die zusammengehörigen Werte für scch information in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ScchInformation {
    pub configuration: u8,
}

/// Threshold set used for monitoring and cell selection.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
// Was: Bündelt die zusammengehörigen Werte für threshold values in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ThresholdValues {
    pub cell_relinquishable: Option<MeasurementValue>,
    pub cell_improvable: Option<MeasurementValue>,
    pub cell_usable: Option<MeasurementValue>,
    pub channel_relinquishable: Option<MeasurementValue>,
    pub channel_improvable: Option<MeasurementValue>,
    pub channel_usable: Option<MeasurementValue>,
}

/// RF carrier number used by TLMC.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
// Was: Bündelt die zusammengehörigen Werte für Funkstrecke Kanal number in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct RfChannelNumber(pub u16);

/// Local channel-class reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
// Was: Bündelt die zusammengehörigen Werte für Kanal class label in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ChannelClassLabel(pub u16);

/// Broad modulation family relevant to local scanning/monitoring.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für modulation mode auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum ModulationMode {
    PhaseModulation,
    Qam,
    Other(u8),
}

/// Supported RF-channel bandwidth.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für Kanal bandwidth auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum ChannelBandwidth {
    Khz25,
    Khz50,
    Khz100,
    Khz150,
    OtherKhz(u16),
}

/// Relation of a monitored channel to its cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für Kanal role auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum ChannelRole {
    ServingMainCarrier,
    NeighbourMainCarrier,
    IrregularCarrier,
    Unknown,
}

/// Conforming/channel-topology information passed with TL-SELECT.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für Kanal topology auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum ChannelTopology {
    Conforming,
    NonConformingConcentric,
    Sectored,
    SuperSectored,
    Eccentric,
    Unknown,
}

/// Characteristics of an RF channel used for scan, monitor or select.
#[derive(Debug, Clone, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für Funkstrecke Kanal characteristics in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct RfChannelCharacteristics {
    pub modulation: ModulationMode,
    pub bandwidth: ChannelBandwidth,
    pub max_ms_tx_power_dbm: Option<i16>,
    pub min_rx_access_level_dbm: Option<i16>,
    pub discontinuous: Option<bool>,
    pub role: ChannelRole,
    pub topology: ChannelTopology,
}

/// Information supplied with a selected or indicated RF channel.
#[derive(Debug, Clone, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für Kanal information in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ChannelInformation {
    pub modulation: ModulationMode,
    pub bandwidth: ChannelBandwidth,
    pub topology: ChannelTopology,
}

/// Characteristics used to assess one channel class.
#[derive(Debug, Clone, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für Kanal class characteristics in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ChannelClassCharacteristics {
    pub modulation: ModulationMode,
    pub max_ms_tx_power_dbm: i16,
    pub min_rx_access_level_dbm: i16,
    pub bs_power_relative_to_main_db: i16,
}

/// Request to assess one channel class.
#[derive(Debug, Clone, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für Kanal class assessment request in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ChannelClassAssessmentRequest {
    pub label: ChannelClassLabel,
    pub characteristics: ChannelClassCharacteristics,
}

/// Measured/assessed path loss for one channel class.
#[derive(Debug, Clone, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für Kanal class measurement in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ChannelClassMeasurement {
    pub label: ChannelClassLabel,
    pub path_loss: MeasurementValue,
}

/// Local reception-quality indication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für quality indication in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct QualityIndication {
    pub raw: i16,
}

/// Method MAC shall use while scanning a carrier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für scanning measurement method auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum ScanningMeasurementMethod {
    Interrupting,
    NonInterrupting,
    Other(u8),
}

/// General lower-layer report values relevant to TLMC and LTPD.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für layer2 report auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum Layer2Report {
    AbortedNotCompletelySent,
    AbortedSentAtLeastOnce,
    ChannelReplacementAdvisable,
    ChannelReplacementBeneficial,
    Close,
    CommonChannelDeallocated,
    CurrentChannelAcceptable,
    DisconnectionFailure,
    DownlinkFailure,
    FailedTransfer,
    FirstCompleteTransmission,
    IncomingDisconnection,
    Layer2TransmissionContinuing,
    LocalDisconnection,
    MaximumPathDelayExceeded,
    MaximumPathDelayAlmostExceeded,
    NetworkBroadcastNotReceived,
    NetworkBroadcastReceived,
    RandomAccessFailure,
    Reject,
    Reset,
    ScheduleTimingPrompt,
    ServiceChange,
    ServiceDefinition,
    ServiceNotSupported,
    ServiceTemporarilyUnavailable,
    SetupFailure,
    Success,
    UsageMarkerMismatch,
    UplinkFailure,
    Other(u16),
}

/// Result of an advanced-link reconnection attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für reconnection result auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum ReconnectionResult {
    Success,
    Reject,
    Other(u8),
}

/// Current cell permission conveyed to SNDCP.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für permitted cell information auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum PermittedCellInformation {
    Permitted,
    NotPermitted,
}

/// Address classification supplied with MLE-RECEIVE/UNITDATA indication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für received address type auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum ReceivedAddressType {
    IndividualAllocated,
    IndividualUnexchanged,
    Group,
    Other,
}

// Was: Implementiert das zugehörige Verhalten für `ReceivedAddressType`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl ReceivedAddressType {
    // Was: Wandelt Eingangsdaten in TETRA address um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn from_tetra_address(address: TetraAddress) -> Self {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match address.ssi_type {
            SsiType::Issi | SsiType::Ssi => Self::IndividualAllocated,
            SsiType::Ussi => Self::IndividualUnexchanged,
            SsiType::Gssi => Self::Group,
            _ => Self::Other,
        }
    }
}

/// Services that remain available while a terminal is temporarily disabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// Was: Bündelt die zusammengehörigen Werte für permitted temporary services in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct PermittedTemporaryServices {
    pub ambience_listening: bool,
    pub lip: bool,
}

/// Snapshot of broadcast information relevant to SNDCP.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
// Was: Bündelt die zusammengehörigen Werte für MLE-Verbindungssteuerung broadcast parameters in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct MleBroadcastParameters {
    pub mcc: Option<u16>,
    pub mnc: Option<u16>,
    pub location_area: Option<u16>,
    pub colour_code: Option<u8>,
    pub main_carrier: Option<u16>,
    pub packet_data_supported: Option<bool>,
    pub data_priority_supported: Option<bool>,
}

/// Command carried by D-NEW-CELL (ETSI EN 300 392-2, clause 18.5.6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für MLE-Verbindungssteuerung Kanal command valid auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum MleChannelCommandValid {
    FollowMacChannelChange,
    ChangeChannelImmediately,
    NoChannelChange,
    Reserved,
}

// Was: Implementiert das zugehörige Verhalten für `MleChannelCommandValid`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl MleChannelCommandValid {
    // Was: Wandelt Eingangsdaten in raw um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub const fn from_raw(value: u8) -> Self {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match value & 0b11 {
            0 => Self::FollowMacChannelChange,
            1 => Self::ChangeChannelImmediately,
            2 => Self::NoChannelChange,
            _ => Self::Reserved,
        }
    }

    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub const fn into_raw(self) -> u8 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            Self::FollowMacChannelChange => 0,
            Self::ChangeChannelImmediately => 1,
            Self::NoChannelChange => 2,
            Self::Reserved => 3,
        }
    }
}

/// Failure cause shared by D-PREPARE-FAIL and D-RESTORE-FAIL (clause 18.5.7).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für MLE-Verbindungssteuerung fail cause auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum MleFailCause {
    NeighbourCellEnquiryUnavailableOrTemporaryBreak,
    CellReselectionTypeNotSupported,
    MsNotAllowedOnCell,
    RestorationCannotBeDoneOnCell,
}

// Was: Implementiert das zugehörige Verhalten für `MleFailCause`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl MleFailCause {
    // Was: Wandelt Eingangsdaten in raw um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub const fn from_raw(value: u8) -> Self {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match value & 0b11 {
            0 => Self::NeighbourCellEnquiryUnavailableOrTemporaryBreak,
            1 => Self::CellReselectionTypeNotSupported,
            2 => Self::MsNotAllowedOnCell,
            _ => Self::RestorationCannotBeDoneOnCell,
        }
    }

    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub const fn into_raw(self) -> u8 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            Self::NeighbourCellEnquiryUnavailableOrTemporaryBreak => 0,
            Self::CellReselectionTypeNotSupported => 1,
            Self::MsNotAllowedOnCell => 2,
            Self::RestorationCannotBeDoneOnCell => 3,
        }
    }
}

/// Acceptance result in D-CHANNEL-RESPONSE (clause 18.5.6c).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für MLE-Verbindungssteuerung Kanal response type auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum MleChannelResponseType {
    Accepted,
    Rejected,
}

// Was: Implementiert das zugehörige Verhalten für `MleChannelResponseType`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl MleChannelResponseType {
    // Was: Wandelt Eingangsdaten in raw um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub const fn from_raw(value: u8) -> Self {
        if value & 1 == 0 {
            Self::Accepted
        } else {
            Self::Rejected
        }
    }

    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub const fn into_raw(self) -> u8 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            Self::Accepted => 0,
            Self::Rejected => 1,
        }
    }
}

/// Reason supplied by U-CHANNEL-REQUEST and repeated in D-CHANNEL-RESPONSE.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für MLE-Verbindungssteuerung Kanal request reason auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum MleChannelRequestReason {
    Unspecified,
    CurrentChannelRadioRelinquishable,
    CurrentChannelRadioImprovable,
    HigherLevelOfServiceRequested,
    Reserved(u8),
}

// Was: Implementiert das zugehörige Verhalten für `MleChannelRequestReason`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl MleChannelRequestReason {
    // Was: Wandelt Eingangsdaten in raw um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub const fn from_raw(value: u8) -> Self {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match value & 0b111 {
            0 => Self::Unspecified,
            1 => Self::CurrentChannelRadioRelinquishable,
            2 => Self::CurrentChannelRadioImprovable,
            3 => Self::HigherLevelOfServiceRequested,
            other => Self::Reserved(other),
        }
    }

    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub const fn into_raw(self) -> u8 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            Self::Unspecified => 0,
            Self::CurrentChannelRadioRelinquishable => 1,
            Self::CurrentChannelRadioImprovable => 2,
            Self::HigherLevelOfServiceRequested => 3,
            Self::Reserved(value) => value & 0b111,
        }
    }
}

/// Minimum retry delay encoded in D-CHANNEL-RESPONSE (clause 18.5.6b).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für MLE-Verbindungssteuerung Kanal request retry delay auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum MleChannelRequestRetryDelay {
    NoDelay,
    Seconds5,
    Seconds10,
    Seconds15,
    Seconds20,
    Seconds25,
    Seconds30,
    Seconds40,
    Seconds50,
    Seconds60,
    Seconds80,
    Seconds120,
    Seconds300,
    Reserved(u8),
    RetransmissionNotPermitted,
}

// Was: Implementiert das zugehörige Verhalten für `MleChannelRequestRetryDelay`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl MleChannelRequestRetryDelay {
    // Was: Wandelt Eingangsdaten in raw um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub const fn from_raw(value: u8) -> Self {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match value & 0b1111 {
            0 => Self::NoDelay,
            1 => Self::Seconds5,
            2 => Self::Seconds10,
            3 => Self::Seconds15,
            4 => Self::Seconds20,
            5 => Self::Seconds25,
            6 => Self::Seconds30,
            7 => Self::Seconds40,
            8 => Self::Seconds50,
            9 => Self::Seconds60,
            10 => Self::Seconds80,
            11 => Self::Seconds120,
            12 => Self::Seconds300,
            15 => Self::RetransmissionNotPermitted,
            other => Self::Reserved(other),
        }
    }

    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub const fn into_raw(self) -> u8 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            Self::NoDelay => 0,
            Self::Seconds5 => 1,
            Self::Seconds10 => 2,
            Self::Seconds15 => 3,
            Self::Seconds20 => 4,
            Self::Seconds25 => 5,
            Self::Seconds30 => 6,
            Self::Seconds40 => 7,
            Self::Seconds50 => 8,
            Self::Seconds60 => 9,
            Self::Seconds80 => 10,
            Self::Seconds120 => 11,
            Self::Seconds300 => 12,
            Self::Reserved(value) => value & 0b1111,
            Self::RetransmissionNotPermitted => 15,
        }
    }

    // Was: Führt den Arbeitsschritt `duration` für duration aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn duration(self) -> Option<Duration> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            Self::NoDelay => Some(Duration::from_secs(0)),
            Self::Seconds5 => Some(Duration::from_secs(5)),
            Self::Seconds10 => Some(Duration::from_secs(10)),
            Self::Seconds15 => Some(Duration::from_secs(15)),
            Self::Seconds20 => Some(Duration::from_secs(20)),
            Self::Seconds25 => Some(Duration::from_secs(25)),
            Self::Seconds30 => Some(Duration::from_secs(30)),
            Self::Seconds40 => Some(Duration::from_secs(40)),
            Self::Seconds50 => Some(Duration::from_secs(50)),
            Self::Seconds60 => Some(Duration::from_secs(60)),
            Self::Seconds80 => Some(Duration::from_secs(80)),
            Self::Seconds120 => Some(Duration::from_secs(120)),
            Self::Seconds300 => Some(Duration::from_secs(300)),
            Self::Reserved(_) | Self::RetransmissionNotPermitted => None,
        }
    }
}

/// Explicit MLE cell-selection lifecycle.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
// Was: Listet die möglichen Varianten für MLE-Verbindungssteuerung cell Zustand auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum MleCellState {
    #[default]
    Null,
    Serving(CellIdentity),
    Scanning,
    CandidateSelected(CellCandidate),
    Preparing(CellCandidate),
    WaitingForNewCell(CellCandidate),
    ChangingChannel(CellCandidate),
    Restoring(CellCandidate),
    Resuming(CellIdentity),
    Failed,
}

/// Explicit lifecycle of one channel-change request.
#[derive(Debug, Clone, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für Kanal change Zustand auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum ChannelChangeState {
    Requested {
        handle: ChannelChangeHandle,
        candidate: CellCandidate,
    },
    AwaitingDecision {
        handle: ChannelChangeHandle,
        candidate: CellCandidate,
    },
    Accepted {
        handle: ChannelChangeHandle,
        candidate: CellCandidate,
    },
    Rejected {
        handle: ChannelChangeHandle,
        candidate: CellCandidate,
    },
    Committed {
        handle: ChannelChangeHandle,
        cell: CellIdentity,
    },
    Failed {
        handle: ChannelChangeHandle,
        result: SelectionResult,
    },
}

/// Local scan state used by the Package C TLMC runtime.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
// Was: Listet die möglichen Varianten für tlmc scan Zustand auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum TlmcScanState {
    #[default]
    Idle,
    Requested {
        request_id: ScanRequestId,
        channel: RfChannelNumber,
    },
    Running {
        request_id: ScanRequestId,
        channel: RfChannelNumber,
    },
    Completed {
        request_id: ScanRequestId,
        candidate: CellCandidate,
    },
    Failed {
        request_id: ScanRequestId,
        report: Layer2Report,
    },
}

/// Local selection state used by the Package C TLMC runtime.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
// Was: Listet die möglichen Varianten für tlmc selection Zustand auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum TlmcSelectionState {
    #[default]
    Idle,
    Requested {
        candidate: CellCandidate,
        cause: SelectionCause,
    },
    AwaitingResponse {
        candidate: CellCandidate,
    },
    Completed {
        cell: CellIdentity,
    },
    Failed {
        result: SelectionResult,
    },
}

/// Link lifecycle visible at the LTPD-SAP.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// Was: Listet die möglichen Varianten für TETRA-Paketdatentransport link Zustand auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum LtpdLinkState {
    #[default]
    Null,
    Open,
    Connecting,
    Connected,
    Busy,
    Broken,
    Reconnecting,
    Releasing,
    Closed,
    Disabled,
}

/// Stable key for one SNDCP/LTPD context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// Was: Bündelt die zusammengehörigen Werte für TETRA-Paketdatentransport Kontext key in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LtpdContextKey {
    pub subscriber_ssi: u32,
    pub nsapi: Nsapi,
    pub endpoint_id: EndpointId,
    pub link_id: LinkId,
}

/// Context passed between mobility and call-control during call restoration.
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für Wiederherstellung Kontext in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct RestoreContext {
    pub subscriber: TetraAddress,
    pub old_endpoint_id: EndpointId,
    pub old_link_id: LinkId,
    pub target_cell: Option<CellIdentity>,
    pub call_identifier: Option<u16>,
    pub cmce_restore_payload: Option<BitBuffer>,
}

// Was: Implementiert das zugehörige Verhalten für `fmt::Display for LtpdContextKey`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl fmt::Display for LtpdContextKey {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ssi={} nsapi={} endpoint={} link={}",
            self.subscriber_ssi,
            self.nsapi.get(),
            self.endpoint_id,
            self.link_id
        )
    }
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;

    #[test]
    // Was: Führt den Arbeitsschritt `constrained_values_reject_out_of_range_inputs` für constrained values reject out of range inputs aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn constrained_values_reject_out_of_range_inputs() {
        assert!(PduPriority::new(7).is_some());
        assert!(PduPriority::new(8).is_none());
        assert!(Nsapi::new(1).is_some());
        assert!(Nsapi::new(14).is_some());
        assert!(Nsapi::new(0).is_none());
        assert!(Nsapi::new(15).is_none());
        assert!(DataPriorityRandomAccessDelayFactor::new(7).is_some());
        assert!(DataPriorityRandomAccessDelayFactor::new(8).is_none());
    }

    #[test]
    // Was: Führt den Arbeitsschritt `layer2_qos_validation_matches_etsi_ranges` für layer2 Dienstgüte (QoS) validation matches etsi ranges aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn layer2_qos_validation_matches_etsi_ranges() {
        let valid = Layer2Qos::default();
        assert!(valid.validate().is_ok());

        let invalid = Layer2Qos {
            acknowledged_window_size: 16,
            ..valid
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    // Was: Führt den Arbeitsschritt `energy_economy_startpoint_has_explicit_ranges` für energy economy startpoint has explicit ranges aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn energy_economy_startpoint_has_explicit_ranges() {
        let valid = EnergyEconomyStartpoint {
            frame: 18,
            multiframe: 60,
        };
        assert!(valid.validate().is_ok());

        let invalid = EnergyEconomyStartpoint {
            frame: 19,
            multiframe: 60,
        };
        assert!(invalid.validate().is_err());
    }
}
