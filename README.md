# compound_duration

[![Crates.io](https://img.shields.io/crates/v/compound_duration.svg)](https://crates.io/crates/compound_duration)
[![Documentation](https://docs.rs/compound_duration/badge.svg)](https://docs.rs/compound_duration)
[![CI](https://github.com/nbari/compound_duration/workflows/CI/badge.svg)](https://github.com/nbari/compound_duration/actions)
[![codecov](https://codecov.io/github/nbari/compound_duration/graph/badge.svg?token=YN9AP5OOLB)](https://codecov.io/github/nbari/compound_duration)
[![License](https://img.shields.io/badge/license-BSD--3--Clause-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2024%20edition-blue.svg?maxAge=3600)](https://www.rust-lang.org/)

Convert seconds/nanoseconds to human-readable compound duration format.

## Features

- **Zero dependencies** - Lightweight and fast
- **Simple API** - Just call the function with your duration
- **Flexible input** - Accepts any type convertible to `u64`
- **High performance** - No allocations except for the output string
- **Well tested** - Comprehensive test suite
- **32-bit safe** - Works correctly on both 32-bit and 64-bit architectures

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
compound_duration = "2.0"
```

## Usage

### Basic Examples

```rust
use compound_duration::{format_dhms, format_wdhms, format_ns};

// Format seconds to days/hours/minutes/seconds
assert_eq!(format_dhms(7259), "2h59s");
assert_eq!(format_dhms(86400), "1d");
assert_eq!(format_dhms(6_000_000), "69d10h40m");

// Format seconds with weeks
assert_eq!(format_wdhms(604_800), "1w");
assert_eq!(format_wdhms(6_000_000), "9w6d10h40m");

// Format nanoseconds with sub-second precision
assert_eq!(format_ns(3_000_129_723_u64), "3s129µs723ns");
assert_eq!(format_ns(1_000_000_000), "1s");
assert_eq!(format_ns(1_500_000), "1ms500µs");
```

### With std::time::Instant

```rust
use compound_duration::{format_dhms, format_ns};
use std::time::Instant;

let start = Instant::now();
// ... do some work ...
let elapsed = start.elapsed();

// Format elapsed time in seconds
println!("Elapsed: {}", format_dhms(elapsed.as_secs()));

// Format with nanosecond precision
println!("Precise: {}", format_ns(elapsed.as_nanos()));
```

### With std::time::Duration

```rust
use compound_duration::format_dhms;
use std::time::Duration;

let duration = Duration::from_secs(3661);
println!("{}", format_dhms(duration.as_secs())); // "1h1m1s"
```

### Using Time Constants

```rust
use compound_duration::{format_dhms, DAY, HOUR, MINUTE};

// Calculate duration programmatically
let uptime = 2 * DAY + 3 * HOUR + 45 * MINUTE + 30;
println!("Server uptime: {}", format_dhms(uptime)); // "2d3h45m30s"

// Using constants for clarity
let backup_interval = 7 * DAY;
println!("Backup every: {}", format_dhms(backup_interval)); // "7d"
```

### Real-World Examples

```rust
use compound_duration::{format_dhms, format_wdhms, format_ns};
use std::time::{Duration, Instant};

// 1. HTTP Request Timeout
let timeout = Duration::from_secs(30);
println!("Timeout: {}", format_dhms(timeout.as_secs())); // "30s"

// 2. Cache Expiry
let cache_ttl = 3600;
println!("Cache TTL: {}", format_dhms(cache_ttl)); // "1h"

// 3. Session Duration
let session = 24 * 3600;
println!("Session expires in: {}", format_dhms(session)); // "1d"

// 4. Benchmark Results
let start = Instant::now();
// ... expensive operation ...
let elapsed = start.elapsed();
println!("Operation took: {}", format_ns(elapsed.as_nanos()));

// 5. Uptime Display
let uptime_seconds = 1_234_567;
println!("System uptime: {}", format_wdhms(uptime_seconds)); // "2w14h56m7s"

// 6. Rate Limiting
let rate_window = 60;
println!("Rate limit window: {}", format_dhms(rate_window)); // "1m"
```

### Different Input Types

```rust
use compound_duration::format_dhms;

// Works with various integer types
let seconds_u32: u32 = 3600;
let seconds_u64: u64 = 7200;
let seconds_i32: i32 = 1800;
let seconds_usize: usize = 900;

println!("{}", format_dhms(seconds_u32));   // "1h"
println!("{}", format_dhms(seconds_u64));   // "2h"
println!("{}", format_dhms(seconds_i32));   // "30m"
println!("{}", format_dhms(seconds_usize)); // "15m"
```


## API Reference

### `format_dhms`

Convert seconds to compound duration (days, hours, minutes, seconds).

```rust
pub fn format_dhms<T>(seconds: T) -> String
where
    T: TryInto<u64>,
    T::Error: std::fmt::Debug,
```

**Examples:**
- `0` → `"0s"`
- `7259` → `"2h59s"`
- `86400` → `"1d"`
- `6_000_000` → `"69d10h40m"`

### `format_wdhms`

Convert seconds to compound duration (weeks, days, hours, minutes, seconds).

```rust
pub fn format_wdhms<T>(seconds: T) -> String
where
    T: TryInto<u64>,
    T::Error: std::fmt::Debug,
```

**Examples:**
- `604_800` → `"1w"`
- `6_000_000` → `"9w6d10h40m"`
- `4_294_967_295` → `"7101w3d6h28m15s"`

### `format_ns`

Convert nanoseconds to compound duration (days, hours, minutes, seconds, milliseconds, microseconds, nanoseconds).

```rust
pub fn format_ns<T>(nanos: T) -> String
where
    T: TryInto<u64>,
    T::Error: std::fmt::Debug,
```

**Examples:**
- `1_000_000_000` → `"1s"`
- `3_000_129_723` → `"3s129µs723ns"`
- `1_500_000` → `"1ms500µs"`

## Constants

The library exports useful time constants:

```rust
pub const NS: u64 = 1;                  // Nanosecond
pub const US: u64 = 1_000;              // Microsecond
pub const MS: u64 = 1_000_000;          // Millisecond
pub const NANOS: u64 = 1_000_000_000;   // Nanoseconds per second
pub const SECOND: u64 = 1;
pub const MINUTE: u64 = 60;
pub const HOUR: u64 = 3_600;
pub const DAY: u64 = 86_400;
pub const WEEK: u64 = 604_800;
```

## Format Examples

| Input (seconds) | `format_dhms` | `format_wdhms` |
|----------------|---------------|----------------|
| 0 | `0s` | `0s` |
| 30 | `30s` | `30s` |
| 61 | `1m1s` | `1m1s` |
| 3600 | `1h` | `1h` |
| 7259 | `2h59s` | `2h59s` |
| 86400 | `1d` | `1d` |
| 604800 | `7d` | `1w` |
| 6000000 | `69d10h40m` | `9w6d10h40m` |

## Version 2.0 Breaking Changes

Version 2.0 introduces breaking changes to fix critical issues on 32-bit architectures:

- Updated to Rust 2024 edition (latest, won't be deprecated soon)
- All internal constants changed from `usize` to `u64`
- Function signatures changed to use generic `TryInto<u64>` instead of complex trait bounds

### Migration Guide

Most code will work without changes. If you were explicitly casting to `usize`:

```rust
// Old (v1.x)
format_dhms(duration.as_secs() as usize);

// New (v2.0) - no cast needed!
format_dhms(duration.as_secs());
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
