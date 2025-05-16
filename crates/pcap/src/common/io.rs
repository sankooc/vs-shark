use std::{cmp, hash::{Hash, Hasher}, net::Ipv6Addr, ops::Range };

use ahash::AHasher;
use anyhow::{bail, Ok, Result};

use crate::{cache::intern, common::enum_def::DataError};

use super::concept::ProgressStatus;

pub struct IO;

impl IO {
    pub fn _read64(data: &[u8], endian: bool) -> Result<u64> {
        let _data = data.try_into()?;
        if endian {
            return Ok(u64::from_be_bytes(_data));
        }
        Ok(u64::from_ne_bytes(_data))
    }
    pub fn read32(data: &[u8], endian: bool) -> Result<u32> {
        let _data = data.try_into()?;
        if endian {
            return Ok(u32::from_be_bytes(_data));
        }
        Ok(u32::from_ne_bytes(_data))
    }
    pub fn read16(data: &[u8], endian: bool) -> Result<u16> {
        let _data = data.try_into()?;
        if endian {
            return Ok(u16::from_be_bytes(_data));
        }
        Ok(u16::from_ne_bytes(_data))
    }
}

pub struct DataSource {
    data: Vec<u8>,
    range: Range<usize>,
}

impl DataSource {
    pub fn create(data: Vec<u8>, range: Range<usize>) -> Self {
        Self { data, range }
    }
    pub fn new() -> Self {
        Self { data: Vec::new(), range: 0..0 }
    }
    pub fn range(&self) -> Range<usize>{
        self.range.clone()
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn _data(&self, cursor: usize) -> Result<u8> {
        if self.range.contains(&cursor) {
            let start = self.range.start;
            return Ok(self.data[cursor - start])
        }
        bail!(DataError::BitSize);
    }
    // 追加数据
    #[inline(always)]
    pub fn update(&mut self, data: Vec<u8>) {
        self.data.extend(data);
        self.range.end = self.range.start + self.data.len();
    }
    pub fn trim(&mut self, cursor: usize) -> Result<()> {
        if cursor <= self.range.start {
            return Ok(());
        }
        let offset = cursor - self.range.start;
        if offset >= self.data.len() {
            self.data.clear();
            self.range = cursor..cursor;
            return Ok(());
        }
        self.data.drain(..offset);
        self.range = cursor..self.range.end;
        Ok(())
    }
    pub fn slice(&self, range: Range<usize>) -> Result<&[u8]> {
        if !self.range.contains(&range.start) {
            bail!(DataError::BitSize);
        }
        if !self.range.contains(&range.end) && self.range.end != range.end {
            bail!(DataError::BitSize);
        }
        let _start = self.range.start;
        let _range = (range.start - _start)..(range.end - _start);
        Ok(&self.data[_range])
    }
}

pub struct Reader<'a> {
    data: &'a DataSource,
    pub range: Range<usize>,
    pub cursor: usize,
}

impl Into<ProgressStatus> for &Reader<'_> {
    fn into(self) -> ProgressStatus {
        let total = self.data.len();
        let cursor = self.cursor;
        ProgressStatus{total, cursor, count: 0}
    }
}
impl<'a> Reader<'a> {
    pub fn new(data: &'a DataSource) -> Self {
        let range = data.range.clone();
        let cursor = range.start;
        Self { data, range, cursor }
    }
    pub fn new_sub(data: &'a DataSource, range: Range<usize>) -> Self {
        let cursor = range.start;
        Self { data, range, cursor }
    }
    pub fn _slice(&self, range: Range<usize>) -> Result<&[u8]> {
        self.data.slice(range)
    }
}

impl Reader<'_> {
    pub fn create_child_reader(&mut self, len: usize) -> Result<Self> {
        if self.left() < len {
            bail!(DataError::BitSize)
        }
        let ds = self.data;
        let range = self.range.start..self.range.start + len;
        self.forward(len);
        Ok(Self { data: ds, range, cursor: self.range.start })
    }
    pub fn hash(&self) -> u64 {
        let mut hasher = AHasher::default();
        let data = self.data.slice(self.range.clone()).unwrap();
        data.hash(&mut hasher);
        hasher.finish()
    }
}
impl Reader<'_> {
    pub fn set(&mut self, pos: usize) -> bool {
        if pos == self.range.end || pos == self.data.range.end {
            self.cursor = pos;
            return true
        }
        if !self.data.range.contains(&pos) {
            return false;
        }
        if !self.range.contains(&pos) {
            return false;
        }
        self.cursor = pos;
        return true;
    }
    pub fn left(&self) -> usize {
        let _cursor = cmp::min(self.range.end, self.cursor);
        return self.range.end - _cursor;
    }
    pub fn forward(&mut self, len: usize) -> bool {
        return self.set(self.cursor + len);
    }
    pub fn back(&mut self, len: usize) -> bool {
        if self.cursor < len {
            return false;
        }
        return self.set(self.cursor - len);
    }
    pub fn slice(&mut self, len: usize, mv: bool) -> Result<&[u8]> {
        if self.forward(len) {
            if mv {
                self._slice(self.cursor - len..self.cursor)
            } else {
                self.back(len);
                self._slice(self.cursor..self.cursor + len)
            }
        } else {
            bail!(DataError::BitSize)
        }
    }

    pub fn next(&self) -> Result<u8>{
        if self.left() > 0 {
            self.data._data(self.cursor)
        } else {
            bail!(DataError::BitSize)
        }
    }

    pub fn read8(&mut self) -> Result<u8> {
        let d = self.next()?;
        self.forward(1);
        Ok(d)
    }
    pub fn read16(&mut self, endian: bool) -> Result<u16> {
        let len = 2;
        let data = self.slice(len, true)?;
        IO::read16(data, endian)
    }
    pub fn read32(&mut self, endian: bool) -> Result<u32> {
        let len = 4;
        let data: &[u8] = self.slice(len, true)?;
        IO::read32(data, endian)
    }
    pub fn read64(&mut self, endian: bool) -> Result<u64> {
        let len = 8;
        let data: &[u8] = self.slice(len, true)?;
        IO::_read64(data, endian)
    }
}


pub fn read_mac(reader: &mut Reader) -> Result<&'static str> {
    let data = reader.slice(6, true)?;
    let str = (data)
            .iter()
            .map(|x| format!("{:02x?}", x))
            .collect::<Vec<String>>()
            .join(":");
    Ok(intern(str))
}



pub struct IP6 {
    pub str: &'static str,
    pub loopback: bool,
    pub multicast: bool
}


impl std::fmt::Display for IP6 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str)
    }
}

impl From<Ipv6Addr> for IP6{
    fn from(val: Ipv6Addr) -> Self {
        let str = intern(val.to_string());
        let loopback = val.is_loopback();
        let multicast = val.is_multicast();
        Self { str, loopback, multicast }
    }
}

pub struct MacAddress {
    pub data: [u8; 6],
}
// fn read_ipv4(reader: &mut Reader) -> Result<std::net::Ipv4Addr> {
//     let len = 4;
//     if reader.left() < len {
//         bail!(DataError::BitSize)
//     }
//     let mut data: [u8; 4] = [0; 4];
//     data.copy_from_slice(self._slice(len));
//     self._move(len);
//     Ok(IPv4Address::new(data))
// }