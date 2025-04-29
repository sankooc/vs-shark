use std::{cmp, ops::Range};

use anyhow::{bail, Ok, Result};

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
    pub data: Vec<u8>,
    pub range: Range<usize>,
}

impl DataSource {
    pub fn new() -> Self {
        Self { data: Vec::new(), range: 0..0 }
    }
    pub fn update(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
        self.range.end = self.range.start + self.data.len();
    }
}

pub struct Reader<'a> {
    data: &'a DataSource,
    pub range: Range<usize>,
    pub cursor: usize,
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
    pub fn _data(&self) -> &[u8] {
        &self.data.data
    }
    pub fn offset(&self, range: Range<usize>) -> &[u8] {
        todo!("")
    }
}

impl Reader<'_> {
    pub fn set(&mut self, pos: usize) -> bool {
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
                Ok(&(self._data())[self.cursor - len..self.cursor])
            } else {
                self.back(len);
                Ok(&self._data()[self.cursor..self.cursor + len])
            }
        } else {
            bail!("todo: data length error");
        }
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
