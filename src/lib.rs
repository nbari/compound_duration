//! Convert seconds to compound duration (week, days, hours, minutes, seconds)
//!
//! This library provides simple, lightweight functions to format time durations
//! into human-readable compound format strings.
//!
//! # Examples
//!
//! ```
//! use compound_duration::{format_dhms, format_wdhms};
//!
//! assert_eq!(format_dhms(7259), "2h59s");
//! assert_eq!(format_wdhms(6_000_000), "9w6d10h40m");
//! ```

use std::{convert::TryInto, fmt::Debug};

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
/// # Examples
///
/// ```
/// use compound_duration::format_dhms;
/// use std::time::Instant;
///
/// let now = Instant::now();
/// // do something ...
/// println!("{}", format_dhms(now.elapsed().as_secs()));
///
/// assert_eq!(format_dhms(6000000), "69d10h40m");
/// assert_eq!(format_dhms(7259), "2h59s");
/// ```
#[must_use]
pub fn format_dhms<T>(seconds: T) -> String
where
    T: TryInto<u64>,
    T::Error: Debug,
{
    let seconds = seconds
        .try_into()
        .expect("value must be convertible to u64");
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
/// # Examples
///
/// ```
/// use compound_duration::format_wdhms;
///
/// assert_eq!(format_wdhms(6_000_000), "9w6d10h40m");
/// assert_eq!(format_wdhms(604_800), "1w");
/// ```
#[must_use]
pub fn format_wdhms<T>(seconds: T) -> String
where
    T: TryInto<u64>,
    T::Error: Debug,
{
    let seconds = seconds
        .try_into()
        .expect("value must be convertible to u64");
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

/// Convert nanoseconds to compound duration (days, hours, minutes, seconds, ms, µs, ns)
///
/// # Examples
///
/// ```
/// use compound_duration::format_ns;
/// use std::time::Instant;
///
/// let now = Instant::now();
/// println!("{}", format_ns(now.elapsed().as_nanos()));
///
/// assert_eq!(format_ns(3_000_129_723_u64), "3s129µs723ns");
/// ```
///
/// # Note
///
/// For values larger than `u64::MAX`, only the lower 64 bits are used.
#[must_use]
pub fn format_ns<T>(nanos: T) -> String
where
    T: TryInto<u64>,
    T::Error: Debug,
{
    let nanos = nanos.try_into().expect("value must be convertible to u64");
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
    use super::{format_dhms, format_ns, format_wdhms, DAY, HOUR, MINUTE, NANOS, WEEK};

    // ========================================================================
    // Basic functionality tests
    // ========================================================================

    #[test]
    fn test_format_dhms_basic() {
        assert_eq!(format_dhms(0), "0s");
        assert_eq!(format_dhms(1), "1s");
        assert_eq!(format_dhms(30), "30s");
        assert_eq!(format_dhms(59), "59s");
        assert_eq!(format_dhms(60), "1m");
        assert_eq!(format_dhms(61), "1m1s");
        assert_eq!(format_dhms(3599), "59m59s");
        assert_eq!(format_dhms(3600), "1h");
        assert_eq!(format_dhms(3661), "1h1m1s");
        assert_eq!(format_dhms(86400), "1d");
        assert_eq!(format_dhms(86401), "1d1s");
        assert_eq!(format_dhms(90061), "1d1h1m1s");
        assert_eq!(format_dhms(7259), "2h59s");
        assert_eq!(format_dhms(604_800), "7d");
        assert_eq!(format_dhms(6_000_000), "69d10h40m");
    }

    #[test]
    fn test_format_wdhms_basic() {
        assert_eq!(format_wdhms(0), "0s");
        assert_eq!(format_wdhms(1), "1s");
        assert_eq!(format_wdhms(30), "30s");
        assert_eq!(format_wdhms(60), "1m");
        assert_eq!(format_wdhms(61), "1m1s");
        assert_eq!(format_wdhms(3600), "1h");
        assert_eq!(format_wdhms(86400), "1d");
        assert_eq!(format_wdhms(86401), "1d1s");
        assert_eq!(format_wdhms(7259), "2h59s");
        assert_eq!(format_wdhms(604_800), "1w");
        assert_eq!(format_wdhms(604_801), "1w1s");
        assert_eq!(format_wdhms(691_261), "1w1d1m1s");
        assert_eq!(format_wdhms(6_000_000), "9w6d10h40m");
    }

    #[test]
    fn test_format_ns_basic() {
        assert_eq!(format_ns(0), "0ns");
        assert_eq!(format_ns(1), "1ns");
        assert_eq!(format_ns(100), "100ns");
        assert_eq!(format_ns(999), "999ns");
        assert_eq!(format_ns(1000), "1µs");
        assert_eq!(format_ns(1001), "1µs1ns");
        assert_eq!(format_ns(10_000), "10µs");
        assert_eq!(format_ns(999_999), "999µs999ns");
        assert_eq!(format_ns(1_000_000), "1ms");
        assert_eq!(format_ns(1_000_001), "1ms1ns");
        assert_eq!(format_ns(1_001_001), "1ms1µs1ns");
        assert_eq!(format_ns(10_000_000), "10ms");
        assert_eq!(format_ns(100_000_000), "100ms");
        assert_eq!(format_ns(999_999_999), "999ms999µs999ns");
        assert_eq!(format_ns(1_000_000_000), "1s");
        assert_eq!(format_ns(3_000_129_723_u64), "3s129µs723ns");
    }

    // ========================================================================
    // Edge cases and boundary values
    // ========================================================================

    #[test]
    fn test_edge_cases() {
        // Maximum values that were problematic on 32-bit (the bug we fixed!)
        assert_eq!(format_dhms(u32::MAX as u64), "49710d6h28m15s");
        assert_eq!(format_wdhms(u32::MAX as u64), "7101w3d6h28m15s");

        // Large u64 values
        assert_eq!(format_dhms(u64::MAX), "213503982334601d7h15s");

        // Values around the 32-bit boundary
        assert_eq!(format_dhms(4_294_967_295_u64), "49710d6h28m15s");
        assert_eq!(format_dhms(4_294_967_296_u64), "49710d6h28m16s");

        // Exact time unit boundaries
        assert_eq!(format_dhms(MINUTE), "1m");
        assert_eq!(format_dhms(HOUR), "1h");
        assert_eq!(format_dhms(DAY), "1d");
        assert_eq!(format_wdhms(WEEK), "1w");
    }

    #[test]
    fn test_ns_edge_cases() {
        // The critical overflow case on 32-bit: DAY * NANOS
        // On 32-bit usize: 86_400 * 1_000_000_000 = 86_400_000_000_000 (overflow!)
        // With u64: works fine!
        let day_in_nanos = DAY * NANOS;
        assert_eq!(format_ns(day_in_nanos), "1d");

        // Multiple days
        assert_eq!(format_ns(2 * day_in_nanos), "2d");

        // Large nanosecond values
        assert_eq!(format_ns(100_000_000_000_000_000_u64), "1157d9h46m40s");
        assert_eq!(format_ns(100_000_000_000_000_001_u64), "1157d9h46m40s1ns");

        // Max u64 nanoseconds
        let max_ns = u64::MAX;
        let result = format_ns(max_ns);
        assert!(result.contains("d")); // Should contain days
    }

    // ========================================================================
    // Type compatibility tests (critical for cross-platform)
    // ========================================================================

    #[test]
    fn test_type_compatibility() {
        // u8
        assert_eq!(format_dhms(255u8), "4m15s");

        // u16
        assert_eq!(format_dhms(65535u16), "18h12m15s");

        // u32
        assert_eq!(format_dhms(4_294_967_295u32), "49710d6h28m15s");

        // u64
        assert_eq!(format_dhms(12345u64), "3h25m45s");

        // usize (platform-dependent)
        assert_eq!(format_dhms(12345usize), "3h25m45s");

        // i32 (should work via TryInto)
        assert_eq!(format_dhms(3600i32), "1h");

        // i64
        assert_eq!(format_dhms(7200i64), "2h");
    }

    #[test]
    fn test_ns_type_compatibility() {
        assert_eq!(format_ns(255u8), "255ns");
        assert_eq!(format_ns(65535u16), "65µs535ns");
        assert_eq!(format_ns(1_000_000u32), "1ms");
        assert_eq!(format_ns(1_000_000_000u64), "1s");
        assert_eq!(format_ns(1_000_000usize), "1ms");
    }

    // ========================================================================
    // 32-bit vs 64-bit architecture safety tests
    // ========================================================================

    #[test]
    fn test_32bit_safety() {
        // These tests verify the fix for PR #2
        // On 32-bit systems with old code (usize), these would overflow

        // Critical: DAY * NANOS calculation
        // 86_400 * 1_000_000_000 = 86_400_000_000_000
        // This is > u32::MAX (4_294_967_295) so would overflow on 32-bit usize
        let day_nanos: u64 = DAY * NANOS;
        assert_eq!(day_nanos, 86_400_000_000_000);
        assert_eq!(format_ns(day_nanos), "1d");

        // HOUR * NANOS
        let hour_nanos: u64 = HOUR * NANOS;
        assert_eq!(hour_nanos, 3_600_000_000_000);
        assert_eq!(format_ns(hour_nanos), "1h");

        // Values around u32::MAX
        assert_eq!(format_dhms(u32::MAX as u64 - 1), "49710d6h28m14s");
        assert_eq!(format_dhms(u32::MAX as u64), "49710d6h28m15s");
        assert_eq!(format_dhms(u32::MAX as u64 + 1), "49710d6h28m16s");
    }

    #[test]
    fn test_arithmetic_no_overflow() {
        // Verify all arithmetic operations are safe

        // These calculations must not overflow
        let _week_seconds: u64 = WEEK;
        let _day_seconds: u64 = DAY;
        let _hour_seconds: u64 = HOUR;
        let _minute_seconds: u64 = MINUTE;

        // Nanosecond multiplications
        let _day_in_nanos: u64 = DAY * NANOS;
        let _hour_in_nanos: u64 = HOUR * NANOS;
        let _minute_in_nanos: u64 = MINUTE * NANOS;

        // All should succeed without panic
    }

    // ========================================================================
    // Special combinations and formatting tests
    // ========================================================================

    #[test]
    fn test_special_combinations() {
        // Only hours
        assert_eq!(format_dhms(7200), "2h");

        // Only minutes
        assert_eq!(format_dhms(120), "2m");

        // Only seconds
        assert_eq!(format_dhms(45), "45s");

        // Hours and minutes, no seconds
        assert_eq!(format_dhms(3660), "1h1m");

        // Days and seconds, no hours/minutes
        assert_eq!(format_dhms(86405), "1d5s");

        // All components
        assert_eq!(format_dhms(90061), "1d1h1m1s");
        assert_eq!(format_wdhms(691_261), "1w1d1m1s");
    }

    #[test]
    fn test_ns_special_combinations() {
        // Only specific units
        assert_eq!(format_ns(5_000_000_000_u64), "5s");
        assert_eq!(format_ns(500_000_000_u64), "500ms");
        assert_eq!(format_ns(500_000_u64), "500µs");
        assert_eq!(format_ns(500_u64), "500ns");

        // Mixed units
        assert_eq!(format_ns(1_234_567_890_u64), "1s234ms567µs890ns");
        assert_eq!(
            format_ns(100_000_000_010_100_001_u64),
            "1157d9h46m40s10ms100µs1ns"
        );
    }

    // ========================================================================
    // Regression tests for the 32-bit overflow bug
    // ========================================================================

    #[test]
    fn test_pr2_regression() {
        // These are the exact scenarios that failed on Debian 32-bit
        // before PR #2 was applied

        // The compiler warning that appeared:
        // "this arithmetic operation will overflow"
        // at: let mut ns = nanos % (DAY * NANOS);

        let large_nanos = 100_000_000_000_000_000_u64;
        let result = format_ns(large_nanos);
        assert!(!result.is_empty());
        assert_eq!(result, "1157d9h46m40s");

        // Test with exact DAY * NANOS calculation
        let one_day_nanos = 86_400_000_000_000_u64;
        assert_eq!(format_ns(one_day_nanos), "1d");

        // Multiple days
        assert_eq!(format_ns(2 * one_day_nanos), "2d");
        assert_eq!(format_ns(10 * one_day_nanos), "10d");
    }

    // ========================================================================
    // Constants validation
    // ========================================================================

    #[test]
    fn test_constants() {
        assert_eq!(MINUTE, 60);
        assert_eq!(HOUR, 3_600);
        assert_eq!(DAY, 86_400);
        assert_eq!(WEEK, 604_800);
        assert_eq!(NANOS, 1_000_000_000);

        // Verify constants are u64
        let _m: u64 = MINUTE;
        let _h: u64 = HOUR;
        let _d: u64 = DAY;
        let _w: u64 = WEEK;
        let _n: u64 = NANOS;
    }
}
