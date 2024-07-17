use std::{borrow::BorrowMut, marker::Copy};

#[derive(Default, Clone)]
pub struct FileInfo {
    pub link_type: u16,
    pub file_type: FileType,
    pub start_time: u64,
    pub version: String,
}

pub struct IO;

impl IO {
    pub fn read64(data: &[u8], endian: bool) -> u64 {
        if endian {
            return u64::from_be_bytes(data.try_into().unwrap());
        }
        u64::from_ne_bytes(data.try_into().unwrap())
    }
    pub fn read32(data: &[u8], endian: bool) -> u32 {
        if endian {
            return u32::from_be_bytes(data.try_into().unwrap());
        }
        u32::from_ne_bytes(data.try_into().unwrap())
    }
    
    pub fn read16(data: &[u8], endian: bool) -> u16 {
        if endian {
            return u16::from_be_bytes(data.try_into().unwrap());
        }
        u16::from_ne_bytes(data.try_into().unwrap())
    }
}
pub struct Reader<'a> {
    data: &'a [u8],
    pub cursor: usize,
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> Reader<'a> {
        Reader { data, cursor: 0 }
    }

    pub fn _move(&mut self, len: usize) {
        self.cursor += len;
    }
    pub fn _slice(&mut self, len: usize) -> &[u8] {
        &self.data[self.cursor..self.cursor + len]
    }
    pub fn slice(&mut self, len: usize) -> &[u8] {
        let _tmp = &self.data[self.cursor..self.cursor + len];
        self.cursor += len;
        _tmp
    }
    pub fn read8(&mut self) -> u8 {
        let a = self.data[self.cursor];
        self.cursor += 1;
        u8::from_be_bytes([a])
    }

    pub fn read16(&mut self, endian: bool) -> u16 {
        let len = 2;
        let data: &[u8] = &self.data[self.cursor..self.cursor + len];
        self._move(len);
        IO::read16(data, endian)
    }

    pub fn read32(&mut self, endian: bool) -> u32 {
        let len = 4;
        let data: &[u8] = &self.data[self.cursor..self.cursor + len];
        self._move(len);
        IO::read32(data, endian)
    }

    pub fn read_mac(&mut self) -> Option<[u8; 6]>{
        let len = 6;
        if self.left() < len {
            return None;
        }
        let mut data: [u8; 6] = [0; 6];
        data.copy_from_slice(&self.data[self.cursor..self.cursor+6]);
        Some(data)
    }
    pub fn left(&self) -> usize {
        self.data.len() - self.cursor
    }
    pub fn has(&self) -> bool{
      return self.cursor < self.data.len() 
    }

    pub fn _read_mac(reader: &mut Reader) -> Option<[u8; 6]> {
        reader.read_mac()
    }
    pub fn _read8(reader: &mut Reader) -> u8 {
        reader.read8()
    }
    
    pub fn _read16_be(reader: &mut Reader) -> u16 {
        reader.read16(true)
    }
    
    pub fn _read16_ne(reader: &mut Reader) -> u16 {
        reader.read16(false)
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub enum FileType {
    PCAP,
    PCAPNG,
    #[default]
    NONE,
}

pub enum Protocol{
    ETHERNET,
    SSL,
    IPV4,
    IPV6,
    ARP,
    TCP,
    UDP,
    ICMP,
    ICMPV6,
    IGMP,
    DNS,
    DHCP,
    TLS,
    HTTP,
}