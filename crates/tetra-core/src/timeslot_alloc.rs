// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für grundlegende TETRA-Datentypen und Hilfsfunktionen.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für timeslot owner auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum TimeslotOwner {
    Brew,
    Cmce,
    /// One-slot packet-data bearer used by the opt-in SNDCP/WAP profile.
    Sndcp,
}

#[derive(Debug, Clone, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für timeslot alloc err auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum TimeslotAllocErr {
    InvalidTimeslot(u8),
    InUse {
        ts: u8,
        owner: TimeslotOwner,
    },
    NotAllocated {
        ts: u8,
    },
    OwnerMismatch {
        ts: u8,
        owner: TimeslotOwner,
        actual: TimeslotOwner,
    },
}

/// Logical traffic timeslot allocator.
///
/// Historically the stack only had one carrier and therefore only TS2..=TS4 were
/// allocatable. For dual-carrier operation we keep the public "timeslot" value
/// as a compact logical bearer id:
///
/// - 2, 3, 4  => main carrier TS2, TS3, TS4
/// - 5, 6, 7  => secondary carrier TS2, TS3, TS4
///
/// Secondary-carrier TS1 is deliberately reserved for control/guard operation and
/// is not allocated as a traffic bearer.
///
/// That lets existing higher layers (Brew, Asterisk, EchoLink, CMCE call maps)
/// keep using `ts` as their bearer key without collisions when both carriers use
/// the same physical TETRA timeslot at the same time.
#[derive(Debug, Clone, Default)]
// Was: Bündelt die zusammengehörigen Werte für timeslot allocator in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TimeslotAllocator {
    // Index 0 = logical TS2, 1 = TS3, 2 = TS4, 3 = TS5, 4 = TS6, 5 = TS7
    owners: [Option<TimeslotOwner>; 6],
}

// Was: Implementiert das zugehörige Verhalten für `TimeslotAllocator`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl TimeslotAllocator {
    // Was: Legt den festen Wert `SINGLE_CARRIER_TRAFFIC_SLOTS` für single carrier Nutzdatenverkehr slots fest.
    // Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
    pub const SINGLE_CARRIER_TRAFFIC_SLOTS: usize = 3;
    // Was: Legt den festen Wert `DUAL_CARRIER_TRAFFIC_SLOTS` für dual carrier Nutzdatenverkehr slots fest.
    // Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
    pub const DUAL_CARRIER_TRAFFIC_SLOTS: usize = 6;

    // Was: Führt den Arbeitsschritt `clamp_capacity` für clamp capacity aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn clamp_capacity(capacity: usize) -> usize {
        capacity.clamp(Self::SINGLE_CARRIER_TRAFFIC_SLOTS, Self::DUAL_CARRIER_TRAFFIC_SLOTS)
    }

    // Was: Führt den Arbeitsschritt `idx` für idx aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn idx(ts: u8) -> Result<usize, TimeslotAllocErr> {
        if (2..=7).contains(&ts) {
            Ok((ts - 2) as usize)
        } else {
            Err(TimeslotAllocErr::InvalidTimeslot(ts))
        }
    }

    // Was: Diese Funktion weist any.
    // Warum: Knapp vorhandene Ressourcen werden dadurch nachvollziehbar und konfliktfrei vergeben.
    pub fn allocate_any(&mut self, owner: TimeslotOwner) -> Option<u8> {
        self.allocate_any_with_capacity(owner, Self::SINGLE_CARRIER_TRAFFIC_SLOTS)
    }

    // Was: Diese Funktion weist any with capacity.
    // Warum: Knapp vorhandene Ressourcen werden dadurch nachvollziehbar und konfliktfrei vergeben.
    pub fn allocate_any_with_capacity(&mut self, owner: TimeslotOwner, capacity: usize) -> Option<u8> {
        let capacity = Self::clamp_capacity(capacity);
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for (i, slot) in self.owners.iter_mut().take(capacity).enumerate() {
            if slot.is_none() {
                *slot = Some(owner);
                return Some(i as u8 + 2);
            }
        }
        None
    }

    /// Allocate the first free logical traffic slot from a caller supplied
    /// preference order. Entries outside the configured carrier capacity are
    /// ignored. This lets packet data prefer the secondary carrier while still
    /// sharing the same allocator with CMCE voice calls.
    // Was: Diese Funktion weist preferred with capacity.
    // Warum: Knapp vorhandene Ressourcen werden dadurch nachvollziehbar und konfliktfrei vergeben.
    pub fn allocate_preferred_with_capacity(
        &mut self,
        owner: TimeslotOwner,
        preferred: &[u8],
        capacity: usize,
    ) -> Option<u8> {
        let capacity = Self::clamp_capacity(capacity);
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for &ts in preferred {
            let Ok(idx) = Self::idx(ts) else { continue };
            if idx >= capacity || self.owners[idx].is_some() {
                continue;
            }
            self.owners[idx] = Some(owner);
            return Some(ts);
        }
        None
    }

    // Was: Führt den Arbeitsschritt `reserve` für reserve aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn reserve(&mut self, owner: TimeslotOwner, ts: u8) -> Result<(), TimeslotAllocErr> {
        self.reserve_with_capacity(owner, ts, Self::DUAL_CARRIER_TRAFFIC_SLOTS)
    }

    // Was: Führt den Arbeitsschritt `reserve_with_capacity` für reserve with capacity aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn reserve_with_capacity(&mut self, owner: TimeslotOwner, ts: u8, capacity: usize) -> Result<(), TimeslotAllocErr> {
        let idx = Self::idx(ts)?;
        let capacity = Self::clamp_capacity(capacity);
        if idx >= capacity {
            return Err(TimeslotAllocErr::InvalidTimeslot(ts));
        }
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self.owners[idx] {
            None => {
                self.owners[idx] = Some(owner);
                Ok(())
            }
            Some(existing) => Err(TimeslotAllocErr::InUse { ts, owner: existing }),
        }
    }

    // Was: Diese Funktion gibt den vorgesehenen Arbeitsschritt.
    // Warum: Ressourcen werden dadurch rechtzeitig freigegeben und blockieren keine weiteren Vorgänge.
    pub fn release(&mut self, owner: TimeslotOwner, ts: u8) -> Result<(), TimeslotAllocErr> {
        let idx = Self::idx(ts)?;
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self.owners[idx] {
            None => Err(TimeslotAllocErr::NotAllocated { ts }),
            Some(existing) if existing != owner => Err(TimeslotAllocErr::OwnerMismatch {
                ts,
                owner,
                actual: existing,
            }),
            Some(_) => {
                self.owners[idx] = None;
                Ok(())
            }
        }
    }

    // Was: Führt den Arbeitsschritt `owner` für owner aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn owner(&self, ts: u8) -> Option<TimeslotOwner> {
        Self::idx(ts).ok().and_then(|idx| self.owners[idx])
    }

    // Was: Prüft, ob free zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub fn is_free(&self, ts: u8) -> bool {
        self.owner(ts).is_none()
    }

    /// Number of currently unallocated single-carrier traffic bearers (logical TS2..=TS4).
    // Was: Führt den Arbeitsschritt `free_count` für free count aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn free_count(&self) -> usize {
        self.free_count_with_capacity(Self::SINGLE_CARRIER_TRAFFIC_SLOTS)
    }

    /// Number of currently unallocated traffic bearers for the configured carrier capacity.
    // Was: Führt den Arbeitsschritt `free_count_with_capacity` für free count with capacity aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn free_count_with_capacity(&self, capacity: usize) -> usize {
        let capacity = Self::clamp_capacity(capacity);
        self.owners.iter().take(capacity).filter(|o| o.is_none()).count()
    }
}


#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;

    #[test]
    // Was: Führt den Arbeitsschritt `sndcp_reservation_blocks_voice_until_release` für SNDCP-Paketdaten reservation blocks voice until release aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn sndcp_reservation_blocks_voice_until_release() {
        let mut alloc = TimeslotAllocator::default();
        alloc.reserve(TimeslotOwner::Sndcp, 2).unwrap();
        assert_eq!(
            alloc.reserve(TimeslotOwner::Cmce, 2),
            Err(TimeslotAllocErr::InUse { ts: 2, owner: TimeslotOwner::Sndcp })
        );
        alloc.release(TimeslotOwner::Sndcp, 2).unwrap();
        assert!(alloc.reserve(TimeslotOwner::Cmce, 2).is_ok());
    }
    #[test]
    // Was: Führt den Arbeitsschritt `preferred_allocation_can_keep_main_carrier_free` für preferred allocation can keep main carrier free aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn preferred_allocation_can_keep_main_carrier_free() {
        let mut alloc = TimeslotAllocator::default();
        let order = [5, 6, 7, 2, 3, 4];
        assert_eq!(
            alloc.allocate_preferred_with_capacity(
                TimeslotOwner::Sndcp,
                &order,
                TimeslotAllocator::DUAL_CARRIER_TRAFFIC_SLOTS,
            ),
            Some(5),
        );
        assert!(alloc.is_free(2));
        assert_eq!(alloc.owner(5), Some(TimeslotOwner::Sndcp));
    }

    #[test]
    // Was: Führt den Arbeitsschritt `multiple_sndcp_bearers_share_allocator_with_voice` für multiple SNDCP-Paketdaten bearers share allocator with voice aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn multiple_sndcp_bearers_share_allocator_with_voice() {
        let mut alloc = TimeslotAllocator::default();
        alloc.reserve(TimeslotOwner::Cmce, 2).unwrap();
        let order = [5, 6, 7, 3, 4, 2];
        assert_eq!(
            alloc.allocate_preferred_with_capacity(
                TimeslotOwner::Sndcp,
                &order,
                TimeslotAllocator::DUAL_CARRIER_TRAFFIC_SLOTS,
            ),
            Some(5),
        );
        assert_eq!(
            alloc.allocate_preferred_with_capacity(
                TimeslotOwner::Sndcp,
                &order,
                TimeslotAllocator::DUAL_CARRIER_TRAFFIC_SLOTS,
            ),
            Some(6),
        );
        assert_eq!(alloc.owner(2), Some(TimeslotOwner::Cmce));
    }

}
