//! Convert seconds to compound duration (week, days, hours, minutes, seconds)

pub const NS: usize = 1;
pub const US: usize = 1_000;
pub const MS: usize = 1_000_000;
pub const NANOS: usize = 1_000_000_000;
pub const SECOND: usize = 1;
pub const MINUTE: usize = 60;
pub const HOUR: usize = 3_600;
pub const DAY: usize = 86_400;
pub const WEEK: usize = 604_800;

/// Convert seconds to compound duration (days, hours, minutes, seconds)
///
/// Example:
///```
/// use compound_duration::format_dhms;
/// use std::time::Instant;
///
/// let now = Instant::now();
/// // do something ...
/// println!("{}", format_dhms(now.elapsed().as_secs() as usize));
///
/// // 69d10h40m
/// println!("{}", format_dhms(6000000));
///```
#[must_use]
pub fn format_dhms(seconds: usize) -> String {
    let mut compound_duration = String::new();
    if seconds == 0 {
        compound_duration.push_str("0s");
        return compound_duration;
    }

    let mut sec = seconds % DAY;
    let ds = seconds / DAY;
    // days
    if ds != 0 {
        compound_duration.push_str(format!("{}d", ds).as_str());
    }

    // hours
    let hs = sec / HOUR;
    sec %= HOUR;
    if hs != 0 {
        compound_duration.push_str(format!("{}h", hs).as_str());
    }

    // minutes
    let ms = sec / MINUTE;
    sec %= MINUTE;
    if ms != 0 {
        compound_duration.push_str(format!("{}m", ms).as_str());
    }

    // seconds
    if sec != 0 {
        compound_duration.push_str(format!("{}s", sec).as_str());
    }

    compound_duration
}

/// Convert seconds to compound duration (week, days, hours, minutes, seconds)
///
/// Example:
///```
/// use compound_duration::format_wdhms;
/// // 9w6d10h40m
/// println!("{}", format_wdhms(6000000));
///```
#[must_use]
pub fn format_wdhms(seconds: usize) -> String {
    let mut compound_duration = String::new();
    if seconds == 0 {
        compound_duration.push_str("0s");
        return compound_duration;
    }

    let mut sec = seconds % WEEK;
    // weeks
    let ws = seconds / WEEK;
    if ws != 0 {
        compound_duration.push_str(format!("{}w", ws).as_str());
    }

    // days
    let ds = sec / DAY;
    sec %= DAY;
    if ds != 0 {
        compound_duration.push_str(format!("{}d", ds).as_str());
    }

    // hours
    let hs = sec / HOUR;
    sec %= HOUR;
    if hs != 0 {
        compound_duration.push_str(format!("{}h", hs).as_str());
    }

    // minutes
    let ms = sec / MINUTE;
    sec %= MINUTE;
    if ms != 0 {
        compound_duration.push_str(format!("{}m", ms).as_str());
    }

    // seconds
    if sec != 0 {
        compound_duration.push_str(format!("{}s", sec).as_str());
    }

    compound_duration
}

/// Convert seconds to compound duration (days, hours, minutes, seconds, ms, µs, ns)
///
/// Example:
///```
/// use compound_duration::format_ns;
/// use std::time::Instant;
///
/// let now = Instant::now();
/// println!("{}", format_ns(now.elapsed().as_nanos() as usize));
///```
#[must_use]
pub fn format_ns(nanos: usize) -> String {
    let mut compound_duration = String::new();
    if nanos == 0 {
        compound_duration.push_str("0ns");
        return compound_duration;
    }

    let mut ns = nanos % (DAY * NANOS);
    let d_ns = nanos / (DAY * NANOS);
    // days
    if d_ns != 0 {
        compound_duration.push_str(format!("{}d", d_ns).as_str());
    }

    // hours
    let h_ns = ns / (HOUR * NANOS);
    ns %= HOUR * NANOS;
    if h_ns != 0 {
        compound_duration.push_str(format!("{}h", h_ns).as_str());
    }

    // minutes
    let minutes_ns = ns / (MINUTE * NANOS);
    ns %= MINUTE * NANOS;
    if minutes_ns != 0 {
        compound_duration.push_str(format!("{}m", minutes_ns).as_str());
    }

    // seconds
    let sec_ns = ns / (SECOND * NANOS);
    ns %= SECOND * NANOS;
    if sec_ns != 0 {
        compound_duration.push_str(format!("{}s", sec_ns).as_str());
    }

    // milliseconds
    let ms_ns = ns / MS;
    ns %= MS;
    if ms_ns != 0 {
        compound_duration.push_str(format!("{}ms", ms_ns).as_str());
    }

    // microseconds
    let micro_ns = ns / US;
    ns %= US;
    if micro_ns != 0 {
        compound_duration.push_str(format!("{}\u{b5}s", micro_ns).as_str());
    }

    // nanoseconds
    if ns != 0 {
        compound_duration.push_str(format!("{}ns", ns).as_str());
    }

    compound_duration
}

#[cfg(test)]
mod tests {
    use super::{format_dhms, format_ns, format_wdhms};

    #[test]
    fn test_format_dhms() {
        assert_eq!(format_dhms(0), "0s");
        assert_eq!(format_dhms(30), "30s");
        assert_eq!(format_dhms(61), "1m1s");
        assert_eq!(format_dhms(3600), "1h");
        assert_eq!(format_dhms(86400), "1d");
        assert_eq!(format_dhms(86401), "1d1s");
        assert_eq!(format_dhms(7259), "2h59s");
        assert_eq!(format_dhms(604_800), "7d");
        assert_eq!(format_dhms(6_000_000), "69d10h40m");
        assert_eq!(format_dhms(4_294_967_295), "49710d6h28m15s");
    }

    #[test]
    fn test_format_wdhms() {
        assert_eq!(format_wdhms(0), "0s");
        assert_eq!(format_wdhms(30), "30s");
        assert_eq!(format_wdhms(61), "1m1s");
        assert_eq!(format_wdhms(3600), "1h");
        assert_eq!(format_wdhms(86400), "1d");
        assert_eq!(format_wdhms(86401), "1d1s");
        assert_eq!(format_wdhms(7259), "2h59s");
        assert_eq!(format_wdhms(604_800), "1w");
        assert_eq!(format_wdhms(6_000_000), "9w6d10h40m");
        assert_eq!(format_wdhms(4_294_967_295), "7101w3d6h28m15s");
    }

    #[test]
    fn test_format_ns() {
        assert_eq!(format_ns(3_000_129_723), "3s129\u{b5}s723ns");
        assert_eq!(format_ns(100_000_000_000_000_000), "1157d9h46m40s");
        assert_eq!(format_ns(100_000_000_000_000_001), "1157d9h46m40s1ns");
        assert_eq!(
            format_ns(100_000_000_000_001_001),
            "1157d9h46m40s1\u{b5}s1ns"
        );
        assert_eq!(
            format_ns(100_000_000_000_100_001),
            "1157d9h46m40s100\u{b5}s1ns"
        );
        assert_eq!(
            format_ns(100_000_000_010_100_001),
            "1157d9h46m40s10ms100\u{b5}s1ns"
        );
        assert_eq!(format_ns(100_000_000_010_000_001), "1157d9h46m40s10ms1ns");
        assert_eq!(format_ns(10_000_000_000_000_000), "115d17h46m40s");
        assert_eq!(format_ns(1_000_000_000_000_000), "11d13h46m40s");
        assert_eq!(format_ns(100_000_000_000_000), "1d3h46m40s");
        assert_eq!(format_ns(10_000_000_000_000), "2h46m40s");
        assert_eq!(format_ns(1_000_000_000_000), "16m40s");
        assert_eq!(format_ns(100_000_000_000), "1m40s");
        assert_eq!(format_ns(100_000_000_010), "1m40s10ns");
        assert_eq!(format_ns(1_000_000_000), "1s");
        assert_eq!(format_ns(100_000_000), "100ms");
        assert_eq!(format_ns(10_000_000), "10ms");
        assert_eq!(format_ns(1_000_001), "1ms1ns");
        assert_eq!(format_ns(1_000_000), "1ms");
        assert_eq!(format_ns(10_000), "10\u{b5}s");
        assert_eq!(format_ns(1000), "1\u{b5}s");
        assert_eq!(format_ns(100), "100ns");
        assert_eq!(format_ns(1), "1ns");
        assert_eq!(format_ns(0), "0ns");
    }
}
