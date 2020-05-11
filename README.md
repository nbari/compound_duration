# compound_duration

[![crates.io](https://img.shields.io/crates/v/compound_duration.svg)](https://crates.io/crates/compound_duration)
[![Build Status](https://travis-ci.org/nbari/compound_duration.svg?branch=master)](https://travis-ci.org/nbari/compound_duration)

Convert seconds to compound duration (week, days, hours, minutes, seconds)

| input number | output string |
| ------------ | ------------- |
| 7259         | 2h59s |
| 86400        | 1d |
| 6000000      | 9w6d10h40m |
| 4294967295   | 7101w3d6h28m15s |


Example:

```rs
use compound_duration::format_dhms;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    // do something ...
    println!("{}", format_dhms(now.elapsed().as_secs() as usize));
}
```
