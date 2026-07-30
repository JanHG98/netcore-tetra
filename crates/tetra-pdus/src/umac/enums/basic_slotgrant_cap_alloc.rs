// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

/// Clause 21.5.6 Basic slot granting, Capacity Allocation element
/// Bits: 4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// Was: Listet die möglichen Varianten für basic slotgrant cap alloc auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum BasicSlotgrantCapAlloc {
    FirstSubslotGranted = 0,
    Grant1Slot = 1,
    Grant2Slots = 2,
    Grant3Slots = 3,
    Grant4Slots = 4,
    Grant5Slots = 5,
    Grant6Slots = 6,
    Grant8Slots = 7,
    Grant10Slots = 8,
    Grant13Slots = 9,
    Grant17Slots = 10,
    Grant24Slots = 11,
    Grant34Slots = 12,
    Grant51Slots = 13,
    Grant68Slots = 14,
    SecondSubslotGranted = 15,
}

// Was: Implementiert das zugehörige Verhalten für `std::convert::TryFrom<u64> for BasicSlotgrantCapAlloc`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::convert::TryFrom<u64> for BasicSlotgrantCapAlloc {
    // Was: Vergibt für error einen fachlich verständlichen Typnamen.
    // Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
    type Error = ();
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn try_from(x: u64) -> Result<Self, Self::Error> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match x {
            0 => Ok(BasicSlotgrantCapAlloc::FirstSubslotGranted),
            1 => Ok(BasicSlotgrantCapAlloc::Grant1Slot),
            2 => Ok(BasicSlotgrantCapAlloc::Grant2Slots),
            3 => Ok(BasicSlotgrantCapAlloc::Grant3Slots),
            4 => Ok(BasicSlotgrantCapAlloc::Grant4Slots),
            5 => Ok(BasicSlotgrantCapAlloc::Grant5Slots),
            6 => Ok(BasicSlotgrantCapAlloc::Grant6Slots),
            7 => Ok(BasicSlotgrantCapAlloc::Grant8Slots),
            8 => Ok(BasicSlotgrantCapAlloc::Grant10Slots),
            9 => Ok(BasicSlotgrantCapAlloc::Grant13Slots),
            10 => Ok(BasicSlotgrantCapAlloc::Grant17Slots),
            11 => Ok(BasicSlotgrantCapAlloc::Grant24Slots),
            12 => Ok(BasicSlotgrantCapAlloc::Grant34Slots),
            13 => Ok(BasicSlotgrantCapAlloc::Grant51Slots),
            14 => Ok(BasicSlotgrantCapAlloc::Grant68Slots),
            15 => Ok(BasicSlotgrantCapAlloc::SecondSubslotGranted),
            _ => Err(()),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `BasicSlotgrantCapAlloc`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl BasicSlotgrantCapAlloc {
    /// Convert this enum back into the raw integer value
    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn into_raw(self) -> u64 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            BasicSlotgrantCapAlloc::FirstSubslotGranted => 0,
            BasicSlotgrantCapAlloc::Grant1Slot => 1,
            BasicSlotgrantCapAlloc::Grant2Slots => 2,
            BasicSlotgrantCapAlloc::Grant3Slots => 3,
            BasicSlotgrantCapAlloc::Grant4Slots => 4,
            BasicSlotgrantCapAlloc::Grant5Slots => 5,
            BasicSlotgrantCapAlloc::Grant6Slots => 6,
            BasicSlotgrantCapAlloc::Grant8Slots => 7,
            BasicSlotgrantCapAlloc::Grant10Slots => 8,
            BasicSlotgrantCapAlloc::Grant13Slots => 9,
            BasicSlotgrantCapAlloc::Grant17Slots => 10,
            BasicSlotgrantCapAlloc::Grant24Slots => 11,
            BasicSlotgrantCapAlloc::Grant34Slots => 12,
            BasicSlotgrantCapAlloc::Grant51Slots => 13,
            BasicSlotgrantCapAlloc::Grant68Slots => 14,
            BasicSlotgrantCapAlloc::SecondSubslotGranted => 15,
        }
    }

    /// Pass 0 when the first subslot should be granted
    /// Pass 99 when the second subslot should be granted
    // Was: Wandelt Eingangsdaten in req slotcount um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn from_req_slotcount(req: usize) -> Self {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match req {
            0 => BasicSlotgrantCapAlloc::FirstSubslotGranted,
            1 => BasicSlotgrantCapAlloc::Grant1Slot,
            2 => BasicSlotgrantCapAlloc::Grant2Slots,
            3 => BasicSlotgrantCapAlloc::Grant3Slots,
            4 => BasicSlotgrantCapAlloc::Grant4Slots,
            5 => BasicSlotgrantCapAlloc::Grant5Slots,
            6 => BasicSlotgrantCapAlloc::Grant6Slots,
            7..=8 => BasicSlotgrantCapAlloc::Grant8Slots,
            9..=10 => BasicSlotgrantCapAlloc::Grant10Slots,
            11..=13 => BasicSlotgrantCapAlloc::Grant13Slots,
            14..=17 => BasicSlotgrantCapAlloc::Grant17Slots,
            18..=24 => BasicSlotgrantCapAlloc::Grant24Slots,
            25..=34 => BasicSlotgrantCapAlloc::Grant34Slots,
            35..=51 => BasicSlotgrantCapAlloc::Grant51Slots,
            52..=68 => BasicSlotgrantCapAlloc::Grant68Slots,
            99 => BasicSlotgrantCapAlloc::SecondSubslotGranted,
            _ => panic!(),
        }
    }

    /// Returns 0 when the first subslot is granted
    /// Returns 99 when the second subslot is granted
    // Was: Wandelt den vorhandenen Wert in req slotcount um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn to_req_slotcount(&self) -> usize {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            BasicSlotgrantCapAlloc::FirstSubslotGranted => {
                unimplemented!();
                // 0
            }
            BasicSlotgrantCapAlloc::Grant1Slot => 1,
            BasicSlotgrantCapAlloc::Grant2Slots => 2,
            BasicSlotgrantCapAlloc::Grant3Slots => 3,
            BasicSlotgrantCapAlloc::Grant4Slots => 4,
            BasicSlotgrantCapAlloc::Grant5Slots => 5,
            BasicSlotgrantCapAlloc::Grant6Slots => 6,
            BasicSlotgrantCapAlloc::Grant8Slots => 8,
            BasicSlotgrantCapAlloc::Grant10Slots => 10,
            BasicSlotgrantCapAlloc::Grant13Slots => 13,
            BasicSlotgrantCapAlloc::Grant17Slots => 17,
            BasicSlotgrantCapAlloc::Grant24Slots => 24,
            BasicSlotgrantCapAlloc::Grant34Slots => 34,
            BasicSlotgrantCapAlloc::Grant51Slots => 51,
            BasicSlotgrantCapAlloc::Grant68Slots => 68,
            BasicSlotgrantCapAlloc::SecondSubslotGranted => {
                unimplemented!();
                // 99
            }
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `From<BasicSlotgrantCapAlloc> for u64`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl From<BasicSlotgrantCapAlloc> for u64 {
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from(e: BasicSlotgrantCapAlloc) -> Self {
        e.into_raw()
    }
}

// Was: Implementiert das zugehörige Verhalten für `core::fmt::Display for BasicSlotgrantCapAlloc`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl core::fmt::Display for BasicSlotgrantCapAlloc {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            BasicSlotgrantCapAlloc::FirstSubslotGranted => write!(f, "FirstSubslotGranted"),
            BasicSlotgrantCapAlloc::Grant1Slot => write!(f, "Grant1Slot"),
            BasicSlotgrantCapAlloc::Grant2Slots => write!(f, "Grant2Slots"),
            BasicSlotgrantCapAlloc::Grant3Slots => write!(f, "Grant3Slots"),
            BasicSlotgrantCapAlloc::Grant4Slots => write!(f, "Grant4Slots"),
            BasicSlotgrantCapAlloc::Grant5Slots => write!(f, "Grant5Slots"),
            BasicSlotgrantCapAlloc::Grant6Slots => write!(f, "Grant6Slots"),
            BasicSlotgrantCapAlloc::Grant8Slots => write!(f, "Grant8Slots"),
            BasicSlotgrantCapAlloc::Grant10Slots => write!(f, "Grant10Slots"),
            BasicSlotgrantCapAlloc::Grant13Slots => write!(f, "Grant13Slots"),
            BasicSlotgrantCapAlloc::Grant17Slots => write!(f, "Grant17Slots"),
            BasicSlotgrantCapAlloc::Grant24Slots => write!(f, "Grant24Slots"),
            BasicSlotgrantCapAlloc::Grant34Slots => write!(f, "Grant34Slots"),
            BasicSlotgrantCapAlloc::Grant51Slots => write!(f, "Grant51Slots"),
            BasicSlotgrantCapAlloc::Grant68Slots => write!(f, "Grant68Slots"),
            BasicSlotgrantCapAlloc::SecondSubslotGranted => write!(f, "SecondSubslotGranted"),
        }
    }
}
