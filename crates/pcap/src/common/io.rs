use anyhow::{bail, Result};

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
    start: usize,
    end: usize,
}

impl DataSource {
    pub fn new() -> Self {
        Self { data: Vec::new(), start: 0, end: 0 }
    }
    // pub fn create_reader(&self, start: usize,) -> Reader {
    //     Reader::new(&self.data, start)
    // }
    pub fn update(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }
}

pub struct Reader<'a> {
    data: &'a DataSource,
    start: usize,
    pub cursor: usize,
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a DataSource, start: usize) -> Self {
        Self { data, start, cursor: 0 }
    }
    pub fn _data(&self) -> &[u8] {
        &self.data.data
    }
}

impl Reader<'_> {
    pub fn _move(&mut self, len: usize) -> bool {
        // let t = self.len();
        // let c = self.cursor();
        // if c + len > t {
        //     return false;
        // }
        // self._set(self.cursor + len);
        self.cursor += len;
        true
    }
    pub fn _slice(&mut self, _len: usize, mv: bool) -> &[u8] {
        // let lef = self.left();
        // let len = cmp::min(lef, _len);
        if mv {
            self._move(_len);
        }
        &(self._data())[self.cursor-_len..self.cursor]
    }
    pub fn read16(&mut self, endian: bool) -> Result<u16> {
        let len = 2;
        let data = self._slice(len, true);
        IO::read16(data, endian)
    }
    pub fn read32(&mut self, endian: bool) -> Result<u32> {
        let len = 4;
        let data: &[u8] = self._slice(len, true);
        IO::read32(data, endian)
    }
    pub 
    fn read64(&mut self, endian: bool) -> Result<u64> {
        let len = 8;
        let data: &[u8] = self._slice(len, true);
        IO::_read64(data, endian)
    }
}
