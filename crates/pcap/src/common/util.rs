use std::time::{Duration, UNIX_EPOCH};
use chrono::{DateTime, Utc};

pub fn date_str(ts: u64) -> String {
    let d = UNIX_EPOCH + Duration::from_micros(ts);
    let datetime = DateTime::<Utc>::from(d);
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn date_sim_str(ts: u64) -> String {
    let d = UNIX_EPOCH + Duration::from_micros(ts);
    let datetime = DateTime::<Utc>::from(d);
    datetime.format("%H:%M:%S").to_string()
}

pub fn parse_tuple<T: std::str::FromStr>(s: &str) -> Option<(T, T)> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return None;
    }
    let a = parts[0].parse::<T>().ok()?;
    let b = parts[1].parse::<T>().ok()?;
    Some((a, b))
}

pub fn tuple_to_str<T: std::fmt::Display>(t: (T, T)) -> String {
    format!("{},{}", t.0, t.1)
}

const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];

pub fn format_bytes_single_unit_int(bytes: usize) -> String {
    let mut size = bytes;
    let mut low = 0;
    let mut unit_index = 0;

    while size >= 1024 && unit_index < UNITS.len() - 1 {
        low = size % 1024;
        size /= 1024;
        unit_index += 1;
    }

    format!("{}.{} {}", size, low, UNITS[unit_index])
}