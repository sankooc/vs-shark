use std::time::{Duration, UNIX_EPOCH};
use chrono::{DateTime, Utc};

pub fn date_str(ts: u64) -> String {
    let d = UNIX_EPOCH + Duration::from_micros(ts);
    let datetime = DateTime::<Utc>::from(d);
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}
