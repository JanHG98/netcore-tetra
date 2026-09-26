// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für grundlegende TETRA-Datentypen und Hilfsfunktionen.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für physical Kanal auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum PhysicalChannel {
    Tp,
    Cp,
    Unallocated,
}

/// The endpoint identifiers between the MLE and LLC, and between the LLC and MAC, refer to the MAC resource that is
/// currently used for that service. These identifiers may be local. There shall be a unique correspondence between the
/// endpoint identifier and the physical allocation (timeslot or timeslots) used in the MAC. (This correspondence is known
/// only within the MAC.) More than one advanced link may use one MAC resource.
/// In the current implementation, the endpoint_id is just the timeslot number used by the MAC.
// Was: Vergibt für endpoint Kennung einen fachlich verständlichen Typnamen.
// Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
pub type EndpointId = u32;

// Was: Vergibt für link Kennung einen fachlich verständlichen Typnamen.
// Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
pub type LinkId = u32;

/// Handle assigned by MLE to primitives for MM/CMCE/SNDCP
// Was: Vergibt für MLE-Verbindungssteuerung handle einen fachlich verständlichen Typnamen.
// Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
pub type MleHandle = u32;

#[derive(Debug, Clone, Copy, PartialEq)]
// Was: Listet die möglichen Varianten für layer2 Dienst auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum Layer2Service {
    /// TODO FIXME, remove this option once all Layer2Service uses have been checked to have the right type
    /// Behavior defaults to Acknowledged type
    Todo,
    /// Use acknowledged BL-DATA (or BL-ADATA) service
    Acknowledged,
    /// Use unacknowledged BL-UDATA service
    Unacknowledged,
}
