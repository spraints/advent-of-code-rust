use chrono::{DateTime, FixedOffset, Utc};

pub fn aoc_now() -> DateTime<FixedOffset> {
    const HOUR: i32 = 3600;
    let aoc_tz = FixedOffset::west_opt(5 * HOUR).unwrap();
    Utc::now().with_timezone(&aoc_tz)
}
