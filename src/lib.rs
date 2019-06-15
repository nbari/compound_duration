pub const SECOND: usize = 1;
pub const MINUTE: usize = 60;
pub const HOUR: usize = 3_600;
pub const DAY: usize = 86_400;
pub const WEEK: usize = 604_800;

pub fn format_dhms(seconds: usize) -> String {
    let mut compound_duration = String::new();
    if seconds <= 0 {
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
    let ss = sec / SECOND;
    if ss != 0 {
        compound_duration.push_str(format!("{}s", ss).as_str());
    }

    return compound_duration;
}

pub fn format_wdhms(seconds: usize) -> String {
    let mut compound_duration = String::new();
    if seconds <= 0 {
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
    let ss = sec / SECOND;
    if ss != 0 {
        compound_duration.push_str(format!("{}s", ss).as_str());
    }

    return compound_duration;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_dhms() {
        assert_eq!(format_dhms(0), "0s");
        assert_eq!(format_dhms(30), "30s");
        assert_eq!(format_dhms(61), "1m1s");
        assert_eq!(format_dhms(3600), "1h");
        assert_eq!(format_dhms(86400), "1d");
        assert_eq!(format_dhms(86401), "1d1s");
        assert_eq!(format_dhms(7259), "2h59s");
        assert_eq!(format_dhms(604800), "7d");
        assert_eq!(format_dhms(6000000), "69d10h40m");
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
        assert_eq!(format_wdhms(604800), "1w");
        assert_eq!(format_wdhms(6000000), "9w6d10h40m");
    }
}
