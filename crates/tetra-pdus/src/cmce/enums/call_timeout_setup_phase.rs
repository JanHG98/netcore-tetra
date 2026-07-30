// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

/// Clause 14.8.17 Call time-out, set-up phase
/// Bits: 3
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// Was: Listet die möglichen Varianten für Ruf timeout setup phase auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum CallTimeoutSetupPhase {
    Predefined = 0,
    T1s = 1,
    T2s = 2,
    T5s = 3,
    T10s = 4,
    T20s = 5,
    T30s = 6,
    T60s = 7,
}

// Was: Implementiert das zugehörige Verhalten für `std::convert::TryFrom<u64> for CallTimeoutSetupPhase`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::convert::TryFrom<u64> for CallTimeoutSetupPhase {
    // Was: Vergibt für error einen fachlich verständlichen Typnamen.
    // Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
    type Error = ();
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn try_from(x: u64) -> Result<Self, Self::Error> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match x {
            0 => Ok(CallTimeoutSetupPhase::Predefined),
            1 => Ok(CallTimeoutSetupPhase::T1s),
            2 => Ok(CallTimeoutSetupPhase::T2s),
            3 => Ok(CallTimeoutSetupPhase::T5s),
            4 => Ok(CallTimeoutSetupPhase::T10s),
            5 => Ok(CallTimeoutSetupPhase::T20s),
            6 => Ok(CallTimeoutSetupPhase::T30s),
            7 => Ok(CallTimeoutSetupPhase::T60s),
            _ => Err(()),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `CallTimeoutSetupPhase`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl CallTimeoutSetupPhase {
    /// Convert this enum back into the raw integer value
    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn into_raw(self) -> u64 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            CallTimeoutSetupPhase::Predefined => 0,
            CallTimeoutSetupPhase::T1s => 1,
            CallTimeoutSetupPhase::T2s => 2,
            CallTimeoutSetupPhase::T5s => 3,
            CallTimeoutSetupPhase::T10s => 4,
            CallTimeoutSetupPhase::T20s => 5,
            CallTimeoutSetupPhase::T30s => 6,
            CallTimeoutSetupPhase::T60s => 7,
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `From<CallTimeoutSetupPhase> for u64`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl From<CallTimeoutSetupPhase> for u64 {
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from(e: CallTimeoutSetupPhase) -> Self {
        e.into_raw()
    }
}

// Was: Implementiert das zugehörige Verhalten für `core::fmt::Display for CallTimeoutSetupPhase`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl core::fmt::Display for CallTimeoutSetupPhase {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            CallTimeoutSetupPhase::Predefined => write!(f, "Predefined"),
            CallTimeoutSetupPhase::T1s => write!(f, "T1s"),
            CallTimeoutSetupPhase::T2s => write!(f, "T2s"),
            CallTimeoutSetupPhase::T5s => write!(f, "T5s"),
            CallTimeoutSetupPhase::T10s => write!(f, "T10s"),
            CallTimeoutSetupPhase::T20s => write!(f, "T20s"),
            CallTimeoutSetupPhase::T30s => write!(f, "T30s"),
            CallTimeoutSetupPhase::T60s => write!(f, "T60s"),
        }
    }
}
