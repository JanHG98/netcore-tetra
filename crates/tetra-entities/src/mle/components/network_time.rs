use chrono::{Datelike, Offset, TimeZone, Utc};
use std::str::FromStr;

/// Encode the 48-bit TETRA network time IE, ETSI EN 300 392-2
/// V3.8.1, 18.5.24, table 18.100 (MSB first):
/// UTC seconds since January 1 / 2 (24), offset sign (1),
/// local offset in 15-minute steps (6), year since 2000 (6),
/// reserved bits set to all ones (11).
///
/// UTC stays UTC; the offset includes the timezone's daylight saving rule.
/// Unsupported dates/offsets return None instead of reserved/invalid codes.
pub fn encode_tetra_network_time(tz_name: &str) -> Option<u64> {
    let tz = chrono_tz::Tz::from_str(tz_name).ok()?;
    encode_tetra_network_time_inner(Utc::now(), tz)
}

fn encode_tetra_network_time_inner(now_utc: chrono::DateTime<Utc>, tz: chrono_tz::Tz) -> Option<u64> {
    let year = now_utc.year();
    if !(2000..=2062).contains(&year) {
        return None;
    }
    let year_start = Utc.with_ymd_and_hms(year, 1, 1, 0, 0, 0).single()?;
    let utc_time = u64::try_from((now_utc - year_start).num_seconds() / 2).ok()?;
    if utc_time >= 0xF1_42FF {
        return None;
    }

    let offset_secs = now_utc.with_timezone(&tz).offset().fix().local_minus_utc();
    let magnitude = offset_secs.unsigned_abs();
    if magnitude > 14 * 60 * 60 || magnitude % (15 * 60) != 0 {
        return None;
    }
    let sign = u64::from(offset_secs < 0);
    let offset = u64::from(magnitude / (15 * 60));
    let encoded_year = u64::try_from(year - 2000).ok()?;
    Some((utc_time << 24) | (sign << 23) | (offset << 17) | (encoded_year << 11) | 0x7FF)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encode(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32, zone: &str) -> u64 {
        let at = Utc.with_ymd_and_hms(year, month, day, hour, minute, second).unwrap();
        encode_tetra_network_time_inner(at, zone.parse().unwrap()).unwrap()
    }

    // Decode the standard's actual wire positions, independently of the encoder.
    fn fields(value: u64) -> (u64, bool, u64, u64, u64) {
        (
            (value >> 24) & 0xFF_FFFF,
            (value >> 23) & 1 != 0,
            (value >> 17) & 0x3F,
            (value >> 11) & 0x3F,
            value & 0x7FF,
        )
    }

    #[test]
    fn reported_october_time_has_two_hour_offset_and_year() {
        // Jan's 16:40 Berlin / 14:40 UTC must carry +8 quarter hours.
        let value = encode(2026, 10, 16, 14, 40, 0, "Europe/Berlin");
        assert_eq!(fields(value), (12_468_000, false, 8, 26, 0x7FF));
    }

    #[test]
    fn utc_epoch_and_year_are_encoded_even_without_offset() {
        assert_eq!(encode(2000, 1, 1, 0, 0, 0, "UTC"), 0x7FF);
        assert_eq!(fields(encode(2026, 1, 1, 0, 0, 0, "UTC")), (0, false, 0, 26, 0x7FF));
    }

    #[test]
    fn berlin_winter_and_summer_keep_the_same_utc_field_as_utc_zone() {
        for (month, offset) in [(2, 4), (10, 8), (12, 4)] {
            let berlin = fields(encode(2026, month, 15, 12, 0, 0, "Europe/Berlin"));
            let utc = fields(encode(2026, month, 15, 12, 0, 0, "UTC"));
            assert_eq!(berlin.0, utc.0);
            assert_eq!((berlin.1, berlin.2, berlin.3, berlin.4), (false, offset, 26, 0x7FF));
        }
    }

    #[test]
    fn signed_quarter_hour_offsets_and_extremes() {
        for (zone, negative, offset) in [
            ("America/New_York", true, 20),
            ("America/St_Johns", true, 14),
            ("Asia/Kathmandu", false, 23),
            ("Pacific/Chatham", false, 55),
            ("Pacific/Kiritimati", false, 56),
            ("Etc/GMT+12", true, 48),
        ] {
            let value = fields(encode(2026, 1, 15, 12, 0, 0, zone));
            assert_eq!((value.1, value.2), (negative, offset), "{zone}");
            assert_eq!((value.3, value.4), (26, 0x7FF));
        }
    }

    #[test]
    fn spring_dst_transition_uses_utc_instant_without_local_ambiguity() {
        let before = fields(encode(2026, 3, 29, 0, 59, 58, "Europe/Berlin"));
        let after = fields(encode(2026, 3, 29, 1, 0, 0, "Europe/Berlin"));
        assert_eq!((before.2, after.2, after.0 - before.0), (4, 8, 1));
    }

    #[test]
    fn autumn_dst_transition_uses_utc_instant_without_local_ambiguity() {
        let before = fields(encode(2026, 10, 25, 0, 59, 58, "Europe/Berlin"));
        let after = fields(encode(2026, 10, 25, 1, 0, 0, "Europe/Berlin"));
        assert_eq!((before.2, after.2, after.0 - before.0), (8, 4, 1));
    }

    #[test]
    fn leap_year_end_and_year_rollover() {
        let last = fields(encode(2024, 12, 31, 23, 59, 59, "UTC"));
        assert_eq!(last, (15_811_199, false, 0, 24, 0x7FF));
        assert!(last.0 < 0xF1_42FF);
        assert_eq!(fields(encode(2025, 1, 1, 0, 0, 0, "UTC")), (0, false, 0, 25, 0x7FF));
    }

    #[test]
    fn unsupported_years_and_invalid_timezone_have_no_wire_value() {
        for year in [1999, 2063, 2100] {
            let at = Utc.with_ymd_and_hms(year, 1, 1, 0, 0, 0).unwrap();
            assert!(encode_tetra_network_time_inner(at, chrono_tz::UTC).is_none());
        }
        assert_eq!(fields(encode(2062, 12, 31, 23, 59, 59, "UTC")).3, 62);
        assert!(encode_tetra_network_time("Invalid/Timezone").is_none());
    }

    #[test]
    fn network_time_survives_full_broadcast_serialization() {
        use tetra_core::BitBuffer;
        use tetra_pdus::mle::pdus::d_nwrk_broadcast::DNwrkBroadcast;
        let time = encode(2026, 10, 16, 14, 40, 0, "Europe/Berlin");
        let pdu = DNwrkBroadcast {
            cell_re_select_parameters: 0,
            cell_load_ca: 0,
            tetra_network_time: Some(time),
            number_of_ca_neighbour_cells: Some(0),
            neighbour_cell_information_for_ca: Vec::new(),
        };
        let mut bits = BitBuffer::new_autoexpand(128);
        pdu.to_bitbuf(&mut bits).unwrap();
        bits.seek(0);
        let decoded = DNwrkBroadcast::from_bitbuf(&mut bits).unwrap();
        assert_eq!(decoded.tetra_network_time, Some(time));
        assert_eq!(fields(decoded.tetra_network_time.unwrap()).2, 8);
    }
}
