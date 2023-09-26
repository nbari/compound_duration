//! Convert seconds to compound duration (week, days, hours, minutes, seconds)

use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;
use std::ops::BitAnd;

pub const NS: u64 = 1;
pub const US: u64 = 1_000;
pub const MS: u64 = 1_000_000;
pub const NANOS: u64 = 1_000_000_000;
pub const SECOND: u64 = 1;
pub const MINUTE: u64 = 60;
pub const HOUR: u64 = 3_600;
pub const DAY: u64 = 86_400;
pub const WEEK: u64 = 604_800;

/// Convert seconds to compound duration (days, hours, minutes, seconds)
///
/// Example:
///```
/// use compound_duration::format_dhms;
/// use std::time::Instant;
///
/// let now = Instant::now();
/// // do something ...
/// println!("{}", format_dhms(now.elapsed().as_secs() as u64));
///
/// // 69d10h40m
/// println!("{}", format_dhms(6000000));
///```
#[must_use]
pub fn format_dhms<T: TryInto<u64> + TryFrom<u64> + BitAnd<Output = T>>(seconds: T) -> String
where
    <T as TryFrom<u64>>::Error: Debug,
    <T as TryInto<u64>>::Error: Debug,
{
    let seconds: u64 = if std::mem::size_of::<T>() <= std::mem::size_of::<u64>() {
        seconds.try_into().unwrap()
    } else {
        (seconds & u64::MAX.try_into().unwrap())
            .try_into()
            .unwrap()
    };
    let mut compound_duration = String::new();
    if seconds == 0 {
        compound_duration.push_str("0s");
        return compound_duration;
    }

    let mut sec = seconds % DAY;
    let ds = seconds / DAY;
    // days
    if ds != 0 {
        compound_duration.push_str(format!("{ds}d").as_str());
    }

    // hours
    let hs = sec / HOUR;
    sec %= HOUR;
    if hs != 0 {
        compound_duration.push_str(format!("{hs}h").as_str());
    }

    // minutes
    let ms = sec / MINUTE;
    sec %= MINUTE;
    if ms != 0 {
        compound_duration.push_str(format!("{ms}m").as_str());
    }

    // seconds
    if sec != 0 {
        compound_duration.push_str(format!("{sec}s").as_str());
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
pub fn format_wdhms<T: TryInto<u64> + TryFrom<u64> + BitAnd<Output = T>>(seconds: T) -> String
where
    <T as TryFrom<u64>>::Error: Debug,
    <T as TryInto<u64>>::Error: Debug,
{
    let seconds: u64 = if std::mem::size_of::<T>() <= std::mem::size_of::<u64>() {
        seconds.try_into().unwrap()
    } else {
        (seconds & u64::MAX.try_into().unwrap())
            .try_into()
            .unwrap()
    };
    let mut compound_duration = String::new();
    if seconds == 0 {
        compound_duration.push_str("0s");
        return compound_duration;
    }

    let mut sec = seconds % WEEK;
    // weeks
    let ws = seconds / WEEK;
    if ws != 0 {
        compound_duration.push_str(format!("{ws}w").as_str());
    }

    // days
    let ds = sec / DAY;
    sec %= DAY;
    if ds != 0 {
        compound_duration.push_str(format!("{ds}d").as_str());
    }

    // hours
    let hs = sec / HOUR;
    sec %= HOUR;
    if hs != 0 {
        compound_duration.push_str(format!("{hs}h").as_str());
    }

    // minutes
    let ms = sec / MINUTE;
    sec %= MINUTE;
    if ms != 0 {
        compound_duration.push_str(format!("{ms}m").as_str());
    }

    // seconds
    if sec != 0 {
        compound_duration.push_str(format!("{sec}s").as_str());
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
/// println!("{}", format_ns(now.elapsed().as_nanos() as u64));
///```
#[must_use]
pub fn format_ns<T: TryInto<u64> + TryFrom<u64> + BitAnd<Output = T>>(nanos: T) -> String
where
    <T as TryFrom<u64>>::Error: Debug,
    <T as TryInto<u64>>::Error: Debug,
{
    let nanos: u64 = if std::mem::size_of::<T>() <= std::mem::size_of::<u64>() {
        nanos.try_into().unwrap()
    } else {
        (nanos & u64::MAX.try_into().unwrap()).try_into().unwrap()
    };
    let mut compound_duration = String::new();
    if nanos == 0 {
        compound_duration.push_str("0ns");
        return compound_duration;
    }

    let mut ns = nanos % (DAY * NANOS);
    let d_ns = nanos / (DAY * NANOS);
    // days
    if d_ns != 0 {
        compound_duration.push_str(format!("{d_ns}d").as_str());
    }

    // hours
    let h_ns = ns / (HOUR * NANOS);
    ns %= HOUR * NANOS;
    if h_ns != 0 {
        compound_duration.push_str(format!("{h_ns}h").as_str());
    }

    // minutes
    let minutes_ns = ns / (MINUTE * NANOS);
    ns %= MINUTE * NANOS;
    if minutes_ns != 0 {
        compound_duration.push_str(format!("{minutes_ns}m").as_str());
    }

    // seconds
    let sec_ns = ns / (SECOND * NANOS);
    ns %= SECOND * NANOS;
    if sec_ns != 0 {
        compound_duration.push_str(format!("{sec_ns}s").as_str());
    }

    // milliseconds
    let ms_ns = ns / MS;
    ns %= MS;
    if ms_ns != 0 {
        compound_duration.push_str(format!("{ms_ns}ms").as_str());
    }

    // microseconds
    let micro_ns = ns / US;
    ns %= US;
    if micro_ns != 0 {
        compound_duration.push_str(format!("{micro_ns}\u{b5}s").as_str());
    }

    // nanoseconds
    if ns != 0 {
        compound_duration.push_str(format!("{ns}ns").as_str());
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
        assert_eq!(format_dhms(4_294_967_295_u64), "49710d6h28m15s");
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
        assert_eq!(format_wdhms(4_294_967_295_u64), "7101w3d6h28m15s");
    }

    #[test]
    fn test_format_ns() {
        assert_eq!(format_ns(3_000_129_723_u64), "3s129\u{b5}s723ns");
        assert_eq!(format_ns(100_000_000_000_000_000_u64), "1157d9h46m40s");
        assert_eq!(format_ns(100_000_000_000_000_001_u64), "1157d9h46m40s1ns");
        assert_eq!(
            format_ns(100_000_000_000_001_001_u64),
            "1157d9h46m40s1\u{b5}s1ns"
        );
        assert_eq!(
            format_ns(100_000_000_000_100_001_u64),
            "1157d9h46m40s100\u{b5}s1ns"
        );
        assert_eq!(
            format_ns(100_000_000_010_100_001_u64),
            "1157d9h46m40s10ms100\u{b5}s1ns"
        );
        assert_eq!(
            format_ns(100_000_000_010_000_001_u64),
            "1157d9h46m40s10ms1ns"
        );
        assert_eq!(format_ns(10_000_000_000_000_000_u64), "115d17h46m40s");
        assert_eq!(format_ns(1_000_000_000_000_000_u64), "11d13h46m40s");
        assert_eq!(format_ns(100_000_000_000_000_u64), "1d3h46m40s");
        assert_eq!(format_ns(10_000_000_000_000_u64), "2h46m40s");
        assert_eq!(format_ns(1_000_000_000_000_u64), "16m40s");
        assert_eq!(format_ns(100_000_000_000_u64), "1m40s");
        assert_eq!(format_ns(100_000_000_010_u64), "1m40s10ns");
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
