// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für grundlegende TETRA-Datentypen und Hilfsfunktionen.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use core::fmt;

use bitcode::{Decode, Encode};
use serde::{Deserialize, Serialize};

/// Convert a number of hyperframes to timeslots.  
#[macro_export]
// Was: Definiert das Makro `hyperframes`, das wiederkehrenden Rust-Code erzeugt.
// Warum: Gleichartige Strukturen werden dadurch nur einmal beschrieben und können nicht unbemerkt auseinanderlaufen.
macro_rules! hyperframes {
    ($h:expr) => {
        $h * 60 * 18 * 4
    };
}

/// Convert a number of multiframes to timeslots.  
#[macro_export]
// Was: Definiert das Makro `multiframes`, das wiederkehrenden Rust-Code erzeugt.
// Warum: Gleichartige Strukturen werden dadurch nur einmal beschrieben und können nicht unbemerkt auseinanderlaufen.
macro_rules! multiframes {
    ($m:expr) => {
        $m * 18 * 4
    };
}

/// Convert a number of frames to timeslots.  
#[macro_export]
// Was: Definiert das Makro `frames`, das wiederkehrenden Rust-Code erzeugt.
// Warum: Gleichartige Strukturen werden dadurch nur einmal beschrieben und können nicht unbemerkt auseinanderlaufen.
macro_rules! frames {
    ($f:expr) => {
        $f * 4
    };
}

#[derive(Clone, Copy, PartialEq, Encode, Decode, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für tdma time in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TdmaTime {
    /// Timeslot, from 1 to 4
    pub t: u8,
    /// Frame number, from 1 to 18
    pub f: u8,
    /// Multiframe number, from 1 to 60
    pub m: u8,
    /// Hyperframe number, from 0 to 0xFFFF
    pub h: u16,
}

// Was: Implementiert das zugehörige Verhalten für `Default for TdmaTime`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for TdmaTime {
    /// Returns the default TdmaTime of 0/1/1/1
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> TdmaTime {
        TdmaTime { h: 0, m: 1, f: 1, t: 1 }
    }
}

/// Value of i32 time where it wraps back to 0.
// Was: Legt den festen Wert `TIME_INT_WRAP` für time int wrap fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const TIME_INT_WRAP: i32 = 4 * 18 * 60 * 65536;

/// Difference between two int times, handling wrap-around of hyperframe number.
// Was: Führt den Arbeitsschritt `time_int_diff` für time int diff aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn time_int_diff(a: i32, b: i32) -> i32 {
    let mut diff = a - b;
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    while diff < -TIME_INT_WRAP / 2 {
        diff += TIME_INT_WRAP;
    }
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    while diff >= TIME_INT_WRAP / 2 {
        diff -= TIME_INT_WRAP;
    }
    diff
}

// Was: Implementiert das zugehörige Verhalten für `TdmaTime`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl TdmaTime {
    // Was: Prüft, ob valid zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub fn is_valid(self) -> bool {
        self.t >= 1 && self.t <= 4 && self.f >= 1 && self.f <= 18 && self.m >= 1 && self.m <= 60
    }

    // Was: Wandelt den vorhandenen Wert in int um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn to_int(self) -> i32 {
        (self.t as i32 - 1) + ((self.f as i32 - 1) * 4) + ((self.m as i32 - 1) * 4 * 18) + (self.h as i32 * 4 * 18 * 60)
    }

    /// Converts a i32 time into a TdmaTime,
    /// truncating the hyperframe number if it exceeds 65535
    // Was: Wandelt Eingangsdaten in int um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn from_int(time: i32) -> TdmaTime {
        let t = (time.rem_euclid(4) + 1) as u8;
        let f = (time.div_euclid(4).rem_euclid(18) + 1) as u8;
        let m = (time.div_euclid(4 * 18).rem_euclid(60) + 1) as u8;
        let h = (time.div_euclid(4 * 18 * 60)) as u16;
        // TODO handle overflow of hyperframe number

        TdmaTime { t, f, m, h }
    }

    /// Add a number of timeslots to a TdmaTime
    // Was: Diese Funktion fügt timeslots.
    // Warum: Das Einfügen wird so einheitlich geprüft und verwaltet.
    pub fn add_timeslots(self, num_slots: i32) -> TdmaTime {
        TdmaTime::from_int(self.to_int() + num_slots)
    }

    /// Difference between two TdmaTimes in timeslots
    // Was: Führt den Arbeitsschritt `diff` für diff aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn diff(self, b: Self) -> i32 {
        time_int_diff(self.to_int(), b.to_int())
    }

    /// Age of this TdmaTime compared to now
    #[inline(always)]
    // Was: Führt den Arbeitsschritt `age` für age aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn age(self, now: TdmaTime) -> i32 {
        now.diff(self)
    }

    #[inline(always)]
    /// Round this time up to the next occurrence for the given timeslot
    /// If already the right timeslot, time remains unchanged
    // Was: Diese Funktion leitet to timeslot.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn forward_to_timeslot(self, ts: u8) -> TdmaTime {
        let slots_to_add = ((ts + 4 - self.t) % 4) as i32;
        self.add_timeslots(slots_to_add)
    }

    /// Returns true if this DL timeslot should contain a mandatory BSCH (SYNC) block
    // Was: Prüft, ob mandatory bsch zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub fn is_mandatory_bsch(&self) -> bool {
        self.f == 18 && self.t == 4 - ((self.m + 1) % 4)
    }

    /// Returns true if this DL timeslot should contain a mandatory BNCH (broadcast) block
    // Was: Prüft, ob mandatory bnch zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub fn is_mandatory_bnch(&self) -> bool {
        self.f == 18 && self.t == 4 - ((self.m + 3) % 4)
    }

    /// Returns true if this UL timeslot should contain a mandatory CLCH (Common Linearization) block
    // Was: Prüft, ob mandatory clch zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub fn is_mandatory_clch(&self) -> bool {
        self.f == 18 && self.t == 4 - ((self.m + 1) % 4)
        // self.f == 18 && (self.m + self.t) % 4 == 3
    }

    /// Monotonic multiframe index (0-based) across hyperframes: 60 multiframes per hyperframe.
    /// Used for energy-economy monitoring-window scheduling.
    // Was: Führt den Arbeitsschritt `multiframe_index` für multiframe index aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn multiframe_index(self) -> u32 {
        (self.m as u32).saturating_sub(1) + (self.h as u32) * 60
    }

    /// True if this slot falls inside an energy-economy MS's downlink monitoring window, i.e. the
    /// MS (granted `EnergySavingMode` Eg1..Eg7) is awake to receive on the MCCH at this instant.
    ///
    /// Per ETSI EN 300 392-2 clause 23.7.6 / Table 23.9 the energy-economy cycle is counted in
    /// **TDMA frames, not multiframes**: from the start point (`monitoring_frame`, `monitoring_
    /// multiframe`) the MS wakes for one frame, then sleeps `cycle_len - 1` frames, and repeats —
    /// EG1 = wake every 2 frames, EG2 every 3, EG3 every 6. The absolute TDMA-frame index within a
    /// hyperframe is `(m-1)*18 + (f-1)`; the MS is awake when that index is congruent (mod
    /// `cycle_len`) to the start point's index. Every supported cycle length (2, 3, 6) divides the
    /// 1080-frame hyperframe, so the phase is consistent across hyperframe rollover and the
    /// hyperframe number need not enter the comparison. The check is timeslot-independent so the
    /// whole frame (all 4 timeslots) is "open" — this absorbs the 1-slot TX-ahead skew and the
    /// CMCE→LLC→UMAC latency.
    ///
    /// Returns false for invalid parameters (`cycle_len == 0`, frame out of 1..=18, multiframe out
    /// of 1..=60) so a bad published window never silently gates everything.
    // Was: Führt den Arbeitsschritt `in_ee_monitoring_window` für in ee monitoring window aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn in_ee_monitoring_window(self, monitoring_frame: u8, monitoring_multiframe: u8, cycle_len: u8) -> bool {
        if cycle_len == 0 || !(1..=18).contains(&monitoring_frame) || !(1..=60).contains(&monitoring_multiframe) {
            return false;
        }
        let start_abs = (monitoring_multiframe as u32 - 1) * 18 + (monitoring_frame as u32 - 1);
        let cur_abs = (self.m as u32).saturating_sub(1) * 18 + (self.f as u32).saturating_sub(1);
        (cur_abs % cycle_len as u32) == (start_abs % cycle_len as u32)
    }
}

// Was: Implementiert das zugehörige Verhalten für `fmt::Display for TdmaTime`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl fmt::Display for TdmaTime {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:5}/{:02}/{:02}/{}", self.h, self.m, self.f, self.t)
    }
}

// Was: Implementiert das zugehörige Verhalten für `fmt::Debug for TdmaTime`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl fmt::Debug for TdmaTime {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:5}/{:02}/{:02}/{}", self.h, self.m, self.f, self.t)
    }
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;
    #[test]
    // Was: Prüft automatisch den Fall add timeslots and diff.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_add_timeslots_and_diff() {
        let initial_time = TdmaTime::default();

        let mut time = initial_time;
        // Repeat add_timeslots enough times that hyperframe number wraps
        let iterations = 100000;
        let increment = 12345;
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for _ in 0..iterations {
            let time2 = time.add_timeslots(increment);
            // Check that difference is computed correctly
            assert_eq!(time2.diff(time), increment);
            assert_eq!(time.diff(time2), -increment);
            time = time2;
        }
        eprintln!("{:?}", time);

        // Go backwards to test that adding a negative number of slots works.
        // It should end up back at initial_time.
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for _ in 0..iterations {
            let time2 = time.add_timeslots(-increment);
            // Check that difference is computed correctly
            assert_eq!(time2.diff(time), -increment);
            assert_eq!(time.diff(time2), increment);
            time = time2;
        }

        assert_eq!(time, initial_time);
    }

    #[test]
    // Was: Prüft automatisch den Fall from int.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_from_int() {
        // Test both negative and positive numbers
        assert_eq!(TdmaTime::from_int(0), TdmaTime { t: 1, f: 1, m: 1, h: 0 });
        assert_eq!(TdmaTime::from_int(1), TdmaTime { t: 2, f: 1, m: 1, h: 0 });
        assert_eq!(
            TdmaTime::from_int(-1),
            TdmaTime {
                t: 4,
                f: 18,
                m: 60,
                h: 65535
            }
        );
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for time_int in -10000..10000 {
            assert_eq!(TdmaTime::from_int(time_int).diff(TdmaTime::from_int(0)), time_int);
        }
    }

    #[test]
    // Was: Prüft automatisch den Fall ee monitoring window Funkrahmen based.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_ee_monitoring_window_frame_based() {
        // Frame-based (ETSI EN 300 392-2 Table 23.9): cycle counted in TDMA FRAMES. Absolute frame
        // index within a multiframe = (m-1)*18 + (f-1); open when (cur_abs % cycle) ==
        // (start_abs % cycle). Timeslot- and hyperframe-independent.

        // Eg1 (cycle 2), start point (frame 1, multiframe 1) → start_abs 0 → open on even abs frames.
        assert!(TdmaTime { h: 0, m: 1, f: 1, t: 1 }.in_ee_monitoring_window(1, 1, 2)); // abs 0
        assert!(TdmaTime { h: 0, m: 1, f: 1, t: 4 }.in_ee_monitoring_window(1, 1, 2)); // all 4 slots open
        assert!(!TdmaTime { h: 0, m: 1, f: 2, t: 1 }.in_ee_monitoring_window(1, 1, 2)); // abs 1 (odd)
        assert!(TdmaTime { h: 0, m: 1, f: 3, t: 1 }.in_ee_monitoring_window(1, 1, 2)); // abs 2
        assert!(!TdmaTime { h: 0, m: 1, f: 18, t: 1 }.in_ee_monitoring_window(1, 1, 2)); // abs 17 (odd)
        // Parity continues across the multiframe boundary: m=2,f=1 → abs 18 (even) → open.
        assert!(TdmaTime { h: 0, m: 2, f: 1, t: 1 }.in_ee_monitoring_window(1, 1, 2));

        // Opposite Eg1 phase (start frame 2) → open on odd abs frames.
        assert!(TdmaTime { h: 0, m: 1, f: 2, t: 1 }.in_ee_monitoring_window(2, 1, 2)); // abs 1
        assert!(!TdmaTime { h: 0, m: 1, f: 1, t: 1 }.in_ee_monitoring_window(2, 1, 2)); // abs 0

        // Eg3 (cycle 6), start (frame 1, multiframe 1) → open every 6 frames.
        assert!(TdmaTime { h: 0, m: 1, f: 1, t: 1 }.in_ee_monitoring_window(1, 1, 6)); // abs 0
        assert!(!TdmaTime { h: 0, m: 1, f: 4, t: 1 }.in_ee_monitoring_window(1, 1, 6)); // abs 3
        assert!(TdmaTime { h: 0, m: 1, f: 7, t: 1 }.in_ee_monitoring_window(1, 1, 6)); // abs 6 → 0 mod 6
        assert!(TdmaTime { h: 0, m: 1, f: 13, t: 1 }.in_ee_monitoring_window(1, 1, 6)); // abs 12 → 0 mod 6

        // Phase is consistent across hyperframe rollover (every cycle divides the 1080-frame
        // hyperframe), so the hyperframe number does not change the result.
        assert_eq!(
            TdmaTime { h: 0, m: 1, f: 1, t: 1 }.in_ee_monitoring_window(1, 1, 2),
            TdmaTime { h: 5, m: 1, f: 1, t: 1 }.in_ee_monitoring_window(1, 1, 2),
        );

        // Invalid params never gate (return false).
        assert!(!TdmaTime { h: 0, m: 1, f: 7, t: 1 }.in_ee_monitoring_window(7, 1, 0)); // cycle 0
        assert!(!TdmaTime { h: 0, m: 1, f: 7, t: 1 }.in_ee_monitoring_window(0, 1, 2)); // frame 0
        assert!(!TdmaTime { h: 0, m: 1, f: 7, t: 1 }.in_ee_monitoring_window(19, 1, 2)); // frame 19
        assert!(!TdmaTime { h: 0, m: 1, f: 7, t: 1 }.in_ee_monitoring_window(7, 0, 2)); // multiframe 0
        assert!(!TdmaTime { h: 0, m: 1, f: 7, t: 1 }.in_ee_monitoring_window(7, 61, 2)); // multiframe 61
    }
}
