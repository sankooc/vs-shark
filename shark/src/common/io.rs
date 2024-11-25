
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
    fn len(&self) -> usize {
        match self._data {
            Some(data) => data.len(),
            _ => 0,
        }
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
    fn len(&self) -> usize {
        self._get_data().len()
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
    pub fn _read_i8(reader: &Reader) -> Result<i8> {
        let v = reader.read8()?;
        Ok(i8::from_be_bytes([v]))
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
    pub fn _read64_be(reader: &Reader) -> Result<u64> {
        reader.read64(true)
    }
    pub fn _read64_ne(reader: &Reader) -> Result<u64> {
        reader.read64(false)
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
    /// Cuts the data slice from the current position to the end of the slice.
    pub fn _cut(reader: &Reader)-> Result<Vec<u8>> {
        Ok(reader.cut())
    }

    // pub fn _read_string(reader: &Reader, size: usize) -> Result<String> {
    //     reader.read_string(size)
    // }
}

pub trait AReader: Clone {
    /// Return a slice of the internal data.
    ///
    /// The slice is a view into the internal data, and is not cloned or copied.
    /// The slice is not guaranteed to be valid for the lifetime of the `Reader`
    /// after any call to `set` or `move`.
    fn _get_data(&self) -> &[u8];
    fn cursor(&self) -> usize;
    fn len(&self) -> usize;
    fn _set(&self, cursor: usize);

    /// Move the reader forward by the given number of bytes, if possible.
    ///
    /// If the move is successful, true is returned. Otherwise, false is returned.
    /// The position of the reader is only changed if the move is successful.
    fn _move(&self, len: usize) -> bool {
        let t = self._get_data().len();
        let c = self.cursor();
        if c + len > t {
            return false;
        }
        self._set(self.cursor() + len);
        true
    }
    /// Move the reader backward by the given number of bytes, if possible.
    ///
    /// The position of the reader is only changed by the number of bytes that
    /// can be moved backward, which is limited by the current position. If the
    /// requested length exceeds the current cursor position, the cursor is set
    /// to the beginning.
    fn _back(&self, len: usize) {
        let _len = cmp::min(len, self.cursor());
        self._set(self.cursor() - _len);
    }
    /// Returns a slice of the reader's data, starting at the current cursor position.
    ///
    /// The slice length is the minimum of the given length and the remaining length of the reader.
    /// The slice is not moved, so the underlying data is not modified.
    ///
    /// The returned slice is not null-terminated, and does not include the null byte if the data
    /// ends with one.
    fn _slice(&self, _len: usize) -> &[u8] {
        let lef = self.left();
        let len = cmp::min(lef, _len);
        &self._get_data()[self.cursor()..self.cursor() + len]
    }
    /// Returns a slice of the reader's data starting at the current cursor position.
    ///
    /// The slice length is the minimum of the specified length and the remaining data length.
    /// The cursor position is advanced by the length of the returned slice.
    ///
    /// # Arguments
    ///
    /// * `_len` - The desired length of the slice.
    ///
    /// # Returns
    ///
    /// A slice of the reader's data of the specified length.
    fn slice(&self, _len: usize) -> &[u8] {
        let lef = self.left();
        let len = cmp::min(lef, _len);
        let _tmp = self._slice(len);
        self._move(len);
        _tmp
    }
    /// Returns the remaining data as a new vector.
    ///
    /// The returned vector owns the data and is not a reference to the underlying data.
    /// The cursor position is not changed.
    fn cut(&self) -> Vec<u8> {
        let lef = self.left();
        let data = self.slice(lef);
        data.to_vec()
    }
    /// Reads a single byte from the reader.
    ///
    /// The byte is read from the current cursor position, and the position is
    /// advanced by one byte after the read is complete.
    ///
    /// If there is no data left to read, an error is returned with the message
    /// "no_data".
    fn read8(&self) -> Result<u8> {
        if self.left() == 0 {
            bail!("no_data");
        }
        let a = self._get_data()[self.cursor()];
        self._move(1);
        Ok(a)
    }
    /// Reads a string of the specified byte size from the reader.
    ///
    /// The function reads the data at the current cursor position, attempting to interpret
    /// it as a UTF-8 string of the given size. The cursor position is advanced by the size
    /// of the data read. If the data cannot be interpreted as valid UTF-8, an error is returned.
    ///
    /// # Arguments
    ///
    /// * `size` - The number of bytes to read from the reader.
    ///
    /// # Returns
    ///
    /// A `Result` containing the read string if successful, or an error if the data cannot be
    /// interpreted as valid UTF-8.
    fn read_string(&self, size: usize) -> Result<String> {
        let _data = self.slice(size);
        let str = from_utf8(_data)?;
        Ok(str.into())
    }
    /// Reads a NBNS string of the specified byte size from the reader.
    ///
    /// The function reads the data at the current cursor position, attempting to interpret
    /// it as a NBNS string of the given size. The cursor position is advanced by the size
    /// of the data read. If the data cannot be interpreted as valid NBNS, an error is returned.
    ///
    /// # Arguments
    ///
    /// * `size` - The number of bytes to read from the reader.
    ///
    /// # Returns
    ///
    /// A `Result` containing the read string if successful, or an error if the data cannot be
    /// interpreted as valid NBNS.
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
    /// Reads a DNS-compressed string from the reader.
    ///
    /// This function attempts to read and decompress a DNS-compressed string starting at the current
    /// cursor position. The function handles pointers within the DNS message by resolving them
    /// recursively. If the string is empty or the cursor is at the end of the reader, the default
    /// value `def` is returned.
    ///
    /// # Arguments
    ///
    /// * `archor` - The anchor position in the reader, used as a base for resolving pointers.
    /// * `def` - A default string to return if the compressed string is empty or invalid.
    ///
    /// # Returns
    ///
    /// A `Result` containing the decompressed DNS string if successful, or an error if the
    /// reading or decompression fails.
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
            return self.read_dns_compress_string(archor, &pre);
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
    /// Read a MAC address from the byte stream.
    ///
    /// # Errors
    ///
    /// Returns a `DataError::BitSize` if there is not enough data left in the stream.
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
    /// Reads an IPv4 address from the byte stream.
    ///
    /// # Errors
    ///
    /// Returns a `DataError::BitSize` if there is not enough data left in the stream.
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
    /// Reads an IPv6 address from the byte stream.
    ///
    /// # Errors
    ///
    /// Returns a `DataError::BitSize` if there is not enough data left in the stream to
    /// read a complete IPv6 address.
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
    /// The number of bytes left to read from the stream.
    ///
    /// Returns 0 if the cursor is at or past the end of the stream.
    ///
    /// # Examples
    ///
    /// 
    fn left(&self) -> usize {
        if self._get_data().len() < self.cursor() {
            return 0;
        }
        self._get_data().len() - self.cursor()
    }
    /// Check if there are any bytes left to read from the stream.
    ///
    /// # Examples
    ///
    /// 
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
    /// Reads a Type-Length-Value (TLV) encoded data from the byte stream.
    ///
    /// The function determines the length of the TLV based on the byte
    /// following the current cursor position and reads the appropriate number
    /// of bytes to determine the length value. It supports lengths encoded
    /// in 1, 2, or 4 bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if there is insufficient data left in the stream
    /// to read the TLV length.
    ///
    /// # Returns
    ///
    /// The length of the TLV value as a `usize`.
    fn read_tlv(&self) -> Result<usize> {
        if self.left() <= 1 {
            bail!("no_data_for_tlv");
        }
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
        if self.left() < inx + 1 {
            return false;
        }
        let a = self._get_data()[self.cursor() + inx];
        let b = self._get_data()[self.cursor() + inx + 1];
        if a == 0x0d && b == 0x0a {
            return true;
        }
        false
    }
    /// Try to read a string ended by \\r\\n, return the string if succeed, return error if not.
    ///
    /// # Errors
    ///
    /// * `end of stream`: The stream is too short that there is not enough data to read.
    /// * `cannot find cf`: \\r\\n is not found in the given limit.
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
    /// Read a string ended by \\r\\n from the current position.
    ///
    /// # Errors
    ///
    /// * `end of stream`: The stream is too short that there is not enough data to read.
    /// * `cannot find cf`: \\r\\n is not found in the given limit.
    /// * `out_index`: The index is out of bounds.
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

const HEX_CHAR_LOOKUP: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];

    /// Convert a vector of u8 to a string of hexadecimal digits.
    ///
    /// Each pair of hexadecimal digits in the string corresponds to one byte in the input vector.
    ///
    /// # Examples
    ///
    /// 
pub fn vec_u8to_hex_string(array: &[u8]) -> String {
    let mut hex_string = String::new();
    for byte in array {
        hex_string.push(HEX_CHAR_LOOKUP[(byte >> 4) as usize]);
        hex_string.push(HEX_CHAR_LOOKUP[(byte & 0xF) as usize]);
    }
    hex_string
}
