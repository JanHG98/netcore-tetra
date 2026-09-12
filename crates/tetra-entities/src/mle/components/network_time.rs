use chrono::{Datelike, Offset, TimeZone, Utc};
use std::str::FromStr;

/// Encode current system time as a 48-bit TETRA network time value
/// per ETSI EN 300 392-2 clause 18.5.24.
///
/// Field layout (MSB first, 48 bits total):
///   - UTC time (24 bits): seconds since Jan 1 00:00 UTC of the current year, divided by 2
///   - Local time offset sign (1 bit): 0 = positive (east of UTC), 1 = negative (west of UTC)
///   - Local time offset (23 bits): magnitude in 2-second increments
///
/// Returns `None` if the timezone name is invalid or the resulting values are not encodable.
pub fn encode_tetra_network_time(tz_name: &str) -> Option<u64> {
    let tz: chrono_tz::Tz = chrono_tz::Tz::from_str(tz_name).ok()?;
    let now_utc = Utc::now();

    encode_tetra_network_time_inner(now_utc, tz)
}

fn encode_tetra_network_time_inner(now_utc: chrono::DateTime<Utc>, tz: chrono_tz::Tz) -> Option<u64> {
    // Seconds since Jan 1 00:00:00 UTC of the current year, divided by 2
    let year = now_utc.year();
    let year_start = Utc.with_ymd_and_hms(year, 1, 1, 0, 0, 0).earliest()?;
    let secs_since_year_start = (now_utc - year_start).num_seconds().max(0);
    let utc_time = (secs_since_year_start / 2) as u64;
    // Values from F142FF to FFFFFF are reserved by clause 18.5.24. A valid
    // calendar year, including a leap year, remains below that range.
    if utc_time >= 0xF1_42FF {
        return None;
    }

    // Compute local time offset from UTC
    let now_local = now_utc.with_timezone(&tz);
    let offset_secs = now_local.offset().fix().local_minus_utc(); // seconds east of UTC
    let offset_sign: u64 = if offset_secs < 0 { 1 } else { 0 };
    let offset_secs_abs = offset_secs.unsigned_abs();
    if offset_secs_abs > 24 * 60 * 60 {
        return None;
    }
    let offset_magnitude = (offset_secs_abs / 2) as u64;

    // Pack into 48-bit value (MSB first):
    //   [23..0] network time | [0] offset sign | [22..0] offset magnitude
    let value = (utc_time << 24) | (offset_sign << 23) | offset_magnitude;

    Some(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_encode_known_time() {
        // 2026-02-15 12:00:00 UTC
        let dt = Utc.with_ymd_and_hms(2026, 2, 15, 12, 0, 0).unwrap();
        let tz: chrono_tz::Tz = "Europe/Amsterdam".parse().unwrap();

        let value = encode_tetra_network_time_inner(dt, tz).unwrap();

        // Seconds since 2026-01-01 00:00 UTC:
        // Jan=31 days + 14 days (Feb 1-14) + 12 hours = 45 days + 12h
        let year_start = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
        let expected_secs = (dt - year_start).num_seconds();
        let expected_utc_time = (expected_secs / 2) as u64;

        // Europe/Amsterdam in February = CET = UTC+1. Both the network time and
        // local offset use two-second increments.
        let expected_sign: u64 = 0;
        let expected_offset: u64 = 60 * 60 / 2;

        let expected = (expected_utc_time << 24) | (expected_sign << 23) | expected_offset;

        assert_eq!(value, expected);

        // Verify individual fields by extraction
        assert_eq!((value >> 24) & 0xFF_FFFF, expected_utc_time);
        assert_eq!((value >> 23) & 1, 0); // positive offset
        assert_eq!(value & 0x7F_FFFF, 1_800); // +1 hour in two-second units
    }

    #[test]
    fn test_encode_negative_offset() {
        // 2026-01-15 12:00:00 UTC, New York (EST = UTC-5)
        let dt = Utc.with_ymd_and_hms(2026, 1, 15, 12, 0, 0).unwrap();
        let tz: chrono_tz::Tz = "America/New_York".parse().unwrap();

        let value = encode_tetra_network_time_inner(dt, tz).unwrap();

        assert_eq!((value >> 23) & 1, 1); // negative offset
        assert_eq!(value & 0x7F_FFFF, 9_000); // 5 hours in two-second units
    }

    #[test]
    fn test_encode_berlin_summer_offset() {
        let dt = Utc.with_ymd_and_hms(2026, 9, 12, 15, 0, 0).unwrap();
        let tz: chrono_tz::Tz = "Europe/Berlin".parse().unwrap();

        let value = encode_tetra_network_time_inner(dt, tz).unwrap();

        assert_eq!((value >> 23) & 1, 0);
        assert_eq!(value & 0x7F_FFFF, 3_600); // CEST = UTC+2h in two-second units
    }

    #[test]
    fn test_encode_utc_timezone() {
        let dt = Utc.with_ymd_and_hms(2026, 2, 1, 0, 0, 0).unwrap();
        let tz: chrono_tz::Tz = "UTC".parse().unwrap();

        let value = encode_tetra_network_time_inner(dt, tz).unwrap();

        assert_eq!((value >> 23) & 1, 0); // positive
        assert_eq!(value & 0x7F_FFFF, 0); // zero offset
    }

    #[test]
    fn test_invalid_timezone() {
        assert!(encode_tetra_network_time("Invalid/Timezone").is_none());
    }
}
