use std::{cmp, hash::{Hash, Hasher}, ops::Range, ptr };

use ahash::AHasher;
use anyhow::{bail, Ok, Result};

use crate::{common::enum_def::DataError};

use super::{concept::ProgressStatus, NString};

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
    pub fn data(&self, cursor: usize) -> &[u8] {
        let start: usize = self.range.start;
        let mut _offset = 0;
        if cursor >= start {
            _offset = cursor - start;
        }
        &self.data[_offset..]
    }
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
    pub fn new_sub(data: &'a DataSource, range: Range<usize>) -> Result<Self> {
        let cursor = range.start;
        if !data.range.contains(&cursor) {
            bail!(DataError::BitSize)
        }
        if data.range.end < range.end {
            bail!(DataError::BitSize)

        }
        Ok(Self { data, range, cursor })
    }
    pub fn _slice(&self, range: Range<usize>) -> Result<&[u8]> {
        self.data.slice(range)
    }

    pub fn preview(&self, len: usize) -> Result<&[u8]> {
        self._slice( self.cursor..self.cursor + len)
    }
    pub fn ds(&self) -> &DataSource {
        self.data
    }
}

impl Reader<'_> {
    // pub fn create_child_reader(&mut self, len: usize) -> Result<Self> {
    //     if self.left() < len {
    //         bail!(DataError::BitSize)
    //     }
    //     let ds = self.data;
    //     let range = self.range.start..self.range.start + len;
    //     self.forward(len);
    //     Ok(Self { data: ds, range, cursor: self.range.start })
    // }

    
    pub fn slice_as_reader(&mut self, len: usize) -> Result<Self> {
        if self.forward(len) {
            let range = self.cursor-len..self.cursor;
            Ok(Self { data: self.data, range, cursor: self.cursor-len })
        } else {
            bail!(DataError::BitSize)
        }
    }
    pub fn refer(&self) -> Result<&[u8]>{
        self.data.slice(self.range.clone())
    }
    // pub fn create_range_reader(&mut self, range: Range<usize>)-> Result<Self> {
    //     Reader::new_sub(self.data, range)
    // }
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
        // let data = self.slice(len, true)?;
        // IO::read16(data, endian)
        if !self.forward(len) {
            bail!(DataError::BitSize)
        }
        let bytes = self.data.data(self.cursor - len);
        let mut _val = 0;
        if endian {
            _val = unsafe { ptr::read_unaligned(bytes.as_ptr() as *const u16).to_be() }
        } else {
            _val = unsafe { ptr::read_unaligned(bytes.as_ptr() as *const u16).to_le() }
        }
        Ok(_val)
    }
    pub fn read32(&mut self, endian: bool) -> Result<u32> {
        let len = 4;
        // let data: &[u8] = self.slice(len, true)?;
        // IO::read32(data, endian)

        if !self.forward(len) {
            bail!(DataError::BitSize)
        }
        let bytes = self.data.data(self.cursor - len);
        let mut _val = 0;
        if endian {
            _val = unsafe { ptr::read_unaligned(bytes.as_ptr() as *const u32).to_be() }
        } else {
            _val = unsafe { ptr::read_unaligned(bytes.as_ptr() as *const u32).to_le() }
        }
        Ok(_val)

    }
    pub fn read64(&mut self, endian: bool) -> Result<u64> {
        let len = 8;
        // let data: &[u8] = self.slice(len, true)?;
        // IO::_read64(data, endian)
        if !self.forward(len) {
            bail!(DataError::BitSize)
        }
        let bytes = self.data.data(self.cursor - len);
        let mut _val = 0;
        if endian {
            _val = unsafe { ptr::read_unaligned(bytes.as_ptr() as *const u64).to_be() }
        } else {
            _val = unsafe { ptr::read_unaligned(bytes.as_ptr() as *const u64).to_le() }
        }
        Ok(_val)
    }
    pub fn read128(&mut self, endian: bool) -> Result<u128> {
        let len = 16;
        // let data: &[u8] = self.slice(len, true)?;
        // IO::_read64(data, endian)
        if !self.forward(len) {
            bail!(DataError::BitSize)
        }
        let bytes = self.data.data(self.cursor - len);
        let mut _val = 0;
        if endian {
            _val = unsafe { ptr::read_unaligned(bytes.as_ptr() as *const u128).to_be() }
        } else {
            _val = unsafe { ptr::read_unaligned(bytes.as_ptr() as *const u128).to_le() }
        }
        Ok(_val)
    }
}


pub fn read_mac(data: &[u8]) -> String {
    format!("{:02x?}:{:02x?}:{:02x?}:{:02x?}:{:02x?}:{:02x?}", data[0],data[1],data[2],data[3],data[4],data[5])
}



pub struct IP6 {
    pub str: NString,
    pub loopback: bool,
    pub multicast: bool
}


impl std::fmt::Display for IP6 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.str)
    }
}

// impl From<Ipv6Addr> for IP6{
//     fn from(val: Ipv6Addr) -> Self {
//         let str = intern(val.to_string());
//         let loopback = val.is_loopback();
//         let multicast = val.is_multicast();
//         Self { str, loopback, multicast }
//     }
// }

pub struct MacAddress {
    pub data: [u8; 6],
}

impl From<[u8; 6]> for MacAddress {
    fn from(data: [u8; 6]) -> Self {
        Self{data}
    }
}

impl std::fmt::Display for MacAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = &self.data;
        // read_mac(data);
        f.write_str(&read_mac(data))
    }
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