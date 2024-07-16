use std::{borrow::BorrowMut, marker::Copy};

#[warn(dead_code)]
pub trait Context {
    fn get_file_type(&self) -> FileInfo;
}
#[derive(Default, Clone)]
pub struct FileInfo {
    pub link_type: u16,
    pub file_type: FileType,
    pub start_time: u64,
    pub version: String,
}


// impl FileInfo {
//   fn new() -> FileInfo{

//   }
// }

// enum IpAddrKind {
//   V4,
//   V6,
// }

pub struct IO;

impl IO {
    pub fn read32(data: &[u8], endian: bool) -> u32 {
        if endian {
            return u32::from_be_bytes(data.try_into().unwrap());
        }
        u32::from_ne_bytes(data.try_into().unwrap())
    }
}
pub struct Reader<'a> {
    data: &'a [u8],
    cursor: usize,
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
        if endian {
            return u16::from_be_bytes(data.try_into().unwrap());
        }
        u16::from_ne_bytes(data.try_into().unwrap())
    }

    pub fn read32(&mut self, endian: bool) -> u32 {
        let len = 4;
        let data: &[u8] = &self.data[self.cursor..self.cursor + len];
        self._move(len);
        if endian {
            return u32::from_be_bytes(data.try_into().unwrap());
        }
        u32::from_ne_bytes(data.try_into().unwrap())
    }
    pub fn has(&self) -> bool{
      return self.cursor < self.data.len() 
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