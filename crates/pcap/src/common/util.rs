// Copyright (c) 2025 sankooc
// 
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

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


pub fn bytes_to_hex(bytes: &[u8]) -> String {
    if bytes.len() == 0 {
        return String::new();
    }
    let mut rt = String::with_capacity(2 * bytes.len() + 2);
    rt.push_str("0x");
    for b in bytes {
        rt.push_str(format!("{:02x}", b).as_str());
    }
    rt
}
pub fn bytes_to_hex_limit(bytes: &[u8], max: usize) -> String {
    let len = bytes.len();
    // let cut = len > max;
    let size = std::cmp::min(len, max);
    let _data = &bytes[0..size];
    return bytes_to_hex(_data)
}


pub trait BitData: 
    std::ops::BitAnd<Output = Self> + 
    std::ops::Sub<Self, Output = Self> + 
    std::ops::Shr<usize, Output = Self> + 
    std::ops::Shl<usize, Output = Self> + 
    From<u8> + 
    Copy + 
    std::cmp::PartialEq + 
    std::cmp::PartialOrd +
    Sized {}

impl BitData for u8 {}
impl BitData for u16 {}
impl BitData for u32 {}
impl BitData for u64 {}

pub fn get_masked_value<T: BitData>(value: T, range: &std::ops::Range<usize>) -> T {
    let bits = std::mem::size_of::<T>() * 8;
    let start = range.start;
    let end = range.end;
    let mut v = value;
    if bits > end {
        let offset = bits - end;
        v = v >> offset;
    }
    let len = end - start;
    let mask = (T::from(1) << len) - T::from(1);
    v & mask
}
 
// get_binary_text(0xf0f0u16, 4..8);   .... 1111 .... ....
//
pub fn get_binary_text<T: BitData>(value: T, range: &std::ops::Range<usize>) -> String {
    let bits = std::mem::size_of::<T>() * 8;
    let mut rs = String::with_capacity(bits * 2);
    for i in 0..bits {
        if i % 4 == 0 && i != 0 {
            rs.push(' ');
        }
        if range.contains(&i) {
            let bit = T::from(1) << (bits - i - 1);
            if value & bit == bit {
                rs.push('1');
            } else {
                rs.push('0');
            }
        } else {
            rs.push('.');
        }
    }
    rs
}


pub fn read_bits<T: BitData>(value: T, range: std::ops::Range<usize>, f: impl Fn (T) -> String) -> String {
    let bits = get_binary_text(value, &range);
    let v = get_masked_value(value, &range);
    format!("{} = {}", bits, f(v))
}

pub fn read_bit<T: BitData>(value: T, start: usize, key: &str, sets: (&str, &str)) -> String {
    let cb = |v| if v > T::from(0) { format!("{}: {}", key, sets.0) } else { format!("{}: {}", key, sets.1) };
    read_bits(value, start..start + 1, cb)
}
