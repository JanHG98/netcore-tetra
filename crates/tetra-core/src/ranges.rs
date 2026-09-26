// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für grundlegende TETRA-Datentypen und Hilfsfunktionen.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für TETRA-Teilnehmerkennung (SSI) range in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct SsiRange {
    /// Inclusive start of the range
    pub start: u32,
    /// Inclusive end of the range. E.g. if end is 199, it is considered part of the range
    pub end: u32,
}

// Was: Implementiert das zugehörige Verhalten für `SsiRange`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl SsiRange {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }
}

/// A sorted, non-overlapping (disjoint) list of SSI ranges.
/// Can only be constructed via `sort_non_overlapping()`.
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für sorted disjoint TETRA-Teilnehmerkennung (SSI) ranges in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct SortedDisjointSsiRanges(Vec<SsiRange>);
// Was: Implementiert das zugehörige Verhalten für `SortedDisjointSsiRanges`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl SortedDisjointSsiRanges {
    /// Takes Vec<SsiRange> and sorts it by start address, for fast lookups.
    /// Also asserts that ranges are disjoint, e.g, do not overlap.
    /// Returns a SortedDisjointSsiRanges wrapper which can be used for efficient lookups. See `contains()`.
    // Was: Wandelt Eingangsdaten in vec ssirange um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn from_vec_ssirange(mut ranges: Vec<SsiRange>) -> Self {
        ranges.sort_by_key(|a| a.start);

        // Sanity check for overlapping ranges
        let mut lower_bound = 0;
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for range in &ranges {
            assert!(range.start <= range.end, "Invalid SSI range: {:?}", range);
            assert!(range.start >= lower_bound, "SSI ranges overlap: {:?}", range);
            lower_bound = range.end + 1;
        }
        Self(ranges)
    }

    /// Takes Vec<(start: u32, end: u32)> and sorts it by start address, for fast lookups.
    /// Also asserts that ranges are disjoint, e.g, do not overlap.
    /// Returns a SortedDisjointSsiRanges wrapper which can be used for efficient lookups. See `contains()`.
    // Was: Wandelt Eingangsdaten in vec tuple um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn from_vec_tuple(tuples: Vec<(u32, u32)>) -> Self {
        let ssi_ranges = tuples.into_iter().map(|(start, end)| SsiRange { start, end }).collect();
        Self::from_vec_ssirange(ssi_ranges)
    }

    // Was: Wandelt den vorhandenen Wert in slice um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn as_slice(&self) -> &[SsiRange] {
        &self.0
    }

    /// Checks if the given address falls within any of the ranges.
    /// Note that range.end is inclusive, so if an address is exactly equal to range.end, it is considered in the range.
    // Was: Führt den Arbeitsschritt `contains` für contains aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn contains(&self, addr: u32) -> bool {
        // TODO FIXME this could technically be even faster by starting mid-list and doing binary search
        // Probably fine until we encounter tens of ranges
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for range in self.as_slice() {
            if addr >= range.start && addr <= range.end {
                return true;
            }
            if range.end > addr {
                // Since ranges are sorted, we can stop checking once we've passed the address
                break;
            }
        }
        false
    }
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;

    #[test]
    // Was: Prüft automatisch den Fall range sorting.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_range_sorting() {
        let ssi_ranges = vec![
            SsiRange { start: 300, end: 400 },
            SsiRange { start: 100, end: 200 },
            SsiRange { start: 500, end: 600 },
        ];
        let sorted = SortedDisjointSsiRanges::from_vec_ssirange(ssi_ranges);
        let s = sorted.as_slice();
        assert_eq!(s[0].start, 100);
        assert_eq!(s[1].start, 300);
        assert_eq!(s[2].start, 500);
    }

    #[test]
    #[should_panic(expected = "SSI ranges overlap")]
    // Was: Prüft automatisch den Fall overlapping ranges.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_overlapping_ranges() {
        let ranges = vec![SsiRange { start: 100, end: 200 }, SsiRange { start: 150, end: 300 }];
        SortedDisjointSsiRanges::from_vec_ssirange(ranges);
    }

    #[test]
    // Was: Prüft automatisch den Fall adjacent ranges.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_adjacent_ranges() {
        let ssi_ranges = vec![SsiRange { start: 100, end: 199 }, SsiRange { start: 200, end: 300 }];
        SortedDisjointSsiRanges::from_vec_ssirange(ssi_ranges);
    }

    #[test]
    // Was: Prüft automatisch den Fall containment.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_containment() {
        let ranges = SortedDisjointSsiRanges::from_vec_ssirange(vec![SsiRange { start: 100, end: 200 }, SsiRange { start: 400, end: 500 }]);
        assert!(ranges.contains(100));
        assert!(ranges.contains(150));
        assert!(ranges.contains(200));
        assert!(!ranges.contains(201));
        assert!(!ranges.contains(250));
        assert!(ranges.contains(450));
    }

    #[test]
    #[should_panic(expected = "Invalid SSI range")]
    // Was: Prüft automatisch den Fall invalid range.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_invalid_range() {
        let ranges = vec![SsiRange { start: 200, end: 100 }];
        SortedDisjointSsiRanges::from_vec_ssirange(ranges);
    }
}
