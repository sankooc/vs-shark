use std::time::{Duration, UNIX_EPOCH};

use chrono::{DateTime, Utc};

/// Convert a byte slice to a hexadecimal string.
///
pub fn hexlize(data: &[u8]) -> String {
    data.iter().map(|f| format!("{:02x}", f)).collect::<String>()
}
/// Format a timestamp (in microseconds) as a string in the format
/// `"%Y-%m-%d %H:%M:%S"`.
///
/// # Examples
///
/// 
pub fn date_str(ts: u64) -> String {
    let d = UNIX_EPOCH + Duration::from_micros(ts);
    let datetime = DateTime::<Utc>::from(d);
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}
