use std::{cell::Cell, cmp, rc::Rc, str::from_utf8};

use anyhow::{bail, Result};

use crate::common::DataError;

use super::{IPv4Address, IPv6Address, MacAddress};


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

#[derive(Clone)]
pub struct SliceReader<'a> {
    _data: Option<&'a [u8]>,
    cursor: Cell<usize>,
}
impl SliceReader<'_> {
    pub fn new(data: &[u8]) -> SliceReader {
        SliceReader { _data: Some(data), cursor: Cell::new(0) }
    }
}
impl AReader for SliceReader<'_> {
    fn _get_data(&self) -> &[u8] {
        return self._data.unwrap();
    }
    fn cursor(&self) -> usize {
        return self.cursor.get();
    }
    fn _set(&self, cursor: usize) {
        let len = self._get_data().len();
        let min = cmp::min(len, cursor);
        self.cursor.set(min);
    }
}

#[derive(Clone)]
pub struct Reader {
    _raw: Rc<Vec<u8>>,
    cursor: Cell<usize>,
}

impl AReader for Reader {
    fn _get_data(&self) -> &[u8] {
        return self._raw.as_ref();
    }
    fn cursor(&self) -> usize {
        return self.cursor.get();
    }
    fn _set(&self, cursor: usize) {
        let len = self._get_data().len();
        let min = cmp::min(len, cursor);
        self.cursor.set(min);
    }
}
impl Reader {
    // pub fn clone_one(&self) -> Self {
    //     self.clone()
    // }
    pub fn get_raw(&self) -> Rc<Vec<u8>> {
        self._raw.clone()
    }
    pub fn new_raw(raw: Rc<Vec<u8>>) -> Reader {
        Reader { _raw: raw, cursor: Cell::new(0) }
    }
    pub fn _read_enter(reader: &Reader) -> Result<String> {
        reader.read_enter()
    }
    pub fn _read_mac(reader: &Reader) -> Result<MacAddress> {
        reader.read_mac()
    }

    pub fn _read_ipv4(reader: &Reader) -> Result<std::net::Ipv4Addr> {
        reader.read_ipv4()
    }
    pub fn _read_ipv6(reader: &Reader) -> Result<std::net::Ipv6Addr> {
        reader.read_ipv6()
    }
    pub fn _read8(reader: &Reader) -> Result<u8> {
        reader.read8()
    }
    pub fn _read16_be(reader: &Reader) -> Result<u16> {
        reader.read16(true)
    }

    pub fn _read16_ne(reader: &Reader) -> Result<u16> {
        reader.read16(false)
    }

    pub fn _read32_be(reader: &Reader) -> Result<u32> {
        reader.read32(true)
    }
    pub fn _read32_ne(reader: &Reader) -> Result<u32> {
        reader.read32(false)
    }
    pub fn _read_dns_query(reader: &Reader) -> Result<String> {
        reader.read_dns_query()
    }
    pub fn _read_compress_string(reader: &Reader) -> Result<(String, bool)> {
        reader.read_compress_string()
    }
    pub fn _read_netbios_string(reader: &Reader) -> Result<String> {
        reader.read_netbios_string()
    }
}

pub trait AReader:Clone {
    fn _get_data(&self) -> &[u8];
    fn cursor(&self) -> usize;
    fn _set(&self, cursor: usize);
    fn _move(&self, len: usize) -> bool {
        let t = self._get_data().len();
        let c = self.cursor();
        if c + len > t {
            return false;
        }
        self._set(self.cursor() + len);
        true
    }
    fn _back(&self, len: usize) {
        let _len = cmp::min(len, self.cursor());
        self._set(self.cursor() - _len);
    }
    fn _slice(&self, _len: usize) -> &[u8] {
        let lef = self.left();
        let len = cmp::min(lef, _len);
        &self._get_data()[self.cursor()..self.cursor() + len]
    }
    fn slice(&self, _len: usize) -> &[u8] {
        let lef = self.left();
        let len = cmp::min(lef, _len);
        let _tmp = self._slice(len);
        self._move(len);
        _tmp
    }
    fn read8(&self) -> Result<u8> {
        let a = self._get_data()[self.cursor()];
        self._move(1);
        Ok(a)
    }
    fn read_string(&self, size: usize) -> Result<String> {
        let _data = self.slice(size);
        let str = from_utf8(_data)?;
        Ok(str.into())
    }
    fn read_nbns_string(&self, size: usize) -> Result<String> {
        let words: usize = size / 2;
        let mut rs = Vec::new();
        for _ in 0..words {
            let h = self.read8()? - 65;
            let l = self.read8()? - 65;
            let v = h * 16 + l;
            match v {
                32 | 0 => {}
                _ => {
                    rs.push(v);
                }
            }
        }
        Ok(from_utf8(&rs)?.into())
    }
    fn read_dns_query(&self) -> Result<String> {
        let mut list = Vec::new();
        loop {
            let len = self.read8()? as usize;
            if len > 0 {
                let st = from_utf8(self.slice(len))?;
                list.push(st);
            } else {
                break;
            }
        }
        Ok(list.into_iter().collect::<Vec<_>>().join("."))
    }
    fn read_compress_string(&self) -> Result<(String, bool)> {
        let mut list: Vec<String> = Vec::new();
        loop {
            if self.left() == 2 {
                return Ok((list.into_iter().collect::<Vec<_>>().join("."), true));
            }
            if self.left() == 0 {
                return Ok((list.into_iter().collect::<Vec<_>>().join("."), false));
            }
            let next = self._get_data()[self.cursor()];
            if next == 0 {
                self._move(1);
                return Ok((list.into_iter().collect::<Vec<_>>().join("."), false));
            }
            if next >= 0xc0 {
                return Ok((list.into_iter().collect::<Vec<_>>().join("."), true));
            }
            let __left = self.left();
            if next as usize > __left {
                return Ok((list.into_iter().collect::<Vec<_>>().join("."), false));
            }
            let _size = self.read8()?;
            if _size > 0 {
                let str = self.read_string(_size as usize)?;
                list.push(str);
            }
        }
    }
    fn read_dns_compress_string(&self, archor: usize, def: &str) -> Result<String> {
        if self.left() == 0 {
            return Ok("".into());
        }
        let next = self._get_data()[self.cursor()];
        if next == 0 {
            self._move(1);
            return Ok(def.into());
        }
        let refer = self.read16(true)?;
        let pre = refer >> 8;
        if pre < 0xc0 {
            self._back(2);
            let (str, refer2) = self.read_compress_string()?;
            if refer2 {
                return Ok(self.read_dns_compress_string(archor, &str)?);
            } else {
                return Ok(str);
            }
        }
        let inx = (refer & 0x3fff) as usize;
        let from = archor + inx;
        let _reader = self.clone();
        _reader._set(from);
        let mut rs = String::from(def);
        if def.len() > 0 {
            rs.push_str(".");
        }
        let (str, refer2) = _reader.read_compress_string()?;
        rs.push_str(str.as_str());
        if refer2 {
            Ok(_reader.read_dns_compress_string(archor, rs.as_str())?)
        } else {
            Ok(rs)
        }
    }
    fn _read_compress(&self, archor: usize) -> Result<String> {
        let (pre, has_next) = self.read_compress_string()?;
        if has_next {
            return self.read_dns_compress_string(archor, &pre)
        }
        Ok(pre)
    }
    fn read_netbios_string(&self) -> Result<String> {
        let mut list: Vec<String> = Vec::new();
        loop {
            if self.left() < 1 {
                return Ok(list.join(""));
            }
            let next = self._get_data()[self.cursor()] as usize;
            if next == 0 {
                self._move(1);
                return Ok(list.join(""));
            }
            if next > self.left() {
                return Ok(list.join(""));
            }
            let _size = self.read8()? as usize;
            if _size > 0 {
                let str = self.read_nbns_string(_size)?;
                list.push(str);
            }
        }
    }
    fn read16(&self, endian: bool) -> Result<u16> {
        let len = 2;
        let data: &[u8] = self._slice(len);
        self._move(len);
        IO::read16(data, endian)
    }
    fn read32(&self, endian: bool) -> Result<u32> {
        let len = 4;
        let data: &[u8] = self._slice(len);
        self._move(len);
        IO::read32(data, endian)
    }
    fn read64(&self, endian: bool) -> Result<u64> {
        let len = 8;
        let data: &[u8] = self._slice(len);
        self._move(len);
        IO::_read64(data, endian)
    }
    fn read_mac(&self) -> Result<MacAddress> {
        let len = 6;
        if self.left() < len {
            bail!(DataError::BitSize)
        }
        let mut data: [u8; 6] = [0; 6];
        data.copy_from_slice(self._slice(len));
        self._move(len);
        Ok(MacAddress { data })
    }
    fn read_ipv4(&self) -> Result<std::net::Ipv4Addr> {
        let len = 4;
        if self.left() < len {
            bail!(DataError::BitSize)
        }
        let mut data: [u8; 4] = [0; 4];
        data.copy_from_slice(self._slice(len));
        self._move(len);
        Ok(IPv4Address::new(data))
    }
    fn read_ipv6(&self) -> Result<std::net::Ipv6Addr> {
        let len = 16;
        if self.left() < len {
            bail!(DataError::BitSize)
        }
        let mut data: [u8; 16] = [0; 16];
        data.copy_from_slice(self._slice(len));
        self._move(len);
        Ok(IPv6Address::new(data))
    }
    fn left(&self) -> usize {
        if self._get_data().len() < self.cursor() {
            return 0;
        }
        self._get_data().len() - self.cursor()
    }
    fn has(&self) -> bool {
        return self.cursor() < self._get_data().len();
    }
    fn _read_space(&self, limit: usize) -> Option<String> {
        if self.left() < limit {
            return None;
        }
        for inx in 0..limit {
            let a = self._get_data()[self.cursor() + inx];
            if a == 32 {
                return from_utf8(self._slice(inx)).ok().map(|f| f.into());
            }
        }
        None
    }
    fn read_tlv(&self) -> Result<usize> {
        let b = self._get_data()[self.cursor() + 1];
        let len: usize = match b {
            0x82 => {
                self._move(2);
                self.read16(true)? as usize
            }
            0x83 => {
                self._move(2);
                let a = self.read8()? as usize;
                let b = self.read16(true)? as usize;
                (a << 8) + b
            }
            0x84 => {
                self._move(2);
                self.read32(true)? as usize
            }
            _ => {
                self._move(1);
                let l = self.read8()? as usize;
                l
            }
        };
        Ok(len)
    }
    fn enter_flag(&self, inx: usize) -> bool {
        let a = self._get_data()[self.cursor() + inx];
        let b = self._get_data()[self.cursor() + inx + 1];
        if a == 0x0d && b == 0x0a {
            return true;
        }
        false
    }
    fn try_read_enter(&self, limit: usize) -> Result<String> {
        let end = self.left();
        if end <= 2 {
            bail!("end of stream");
        }
        let _end = cmp::min(limit, end - 2);
        for inx in 0.._end {
            if self.enter_flag(inx) {
                let rs = from_utf8(self.slice(inx))?;
                self._move(2);
                return Ok(rs.into());
            }
        }
        bail!("cannot find cf")
    }
    fn read_enter(&self) -> Result<String> {
        let end = self.left() - 2;
        for inx in 0..end {
            if self.enter_flag(inx) {
                let rs = from_utf8(self.slice(inx))?;
                self._move(2);
                return Ok(rs.into());
            }
        }
        bail!("out_index")
    }
}


const HEX_CHAR_LOOKUP: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];

pub fn vec_u8to_hex_string(array: &[u8]) -> String {
    let mut hex_string = String::new();
    for byte in array {
        hex_string.push(HEX_CHAR_LOOKUP[(byte >> 4) as usize]);
        hex_string.push(HEX_CHAR_LOOKUP[(byte & 0xF) as usize]);
    }
    hex_string
}