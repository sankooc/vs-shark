use std::cell::Cell;
use std::fmt;
use crate::constants::{etype_mapper,ip_protocol_type_mapper};

use crate::files::Field;

pub trait PortablePacket {
    fn source_port(&self) -> u16;
    fn target_port(&self) -> u16;
}

pub trait PlayloadPacket {
    fn len(&self) -> u16;
}

pub trait IPPacket {
    fn source_ip_address(&self) -> String;
    fn target_ip_address(&self) -> String;
}

pub trait MacPacket {
    fn source_mac(&self) -> String;
    fn target_mac(&self) -> String;
}

pub trait PtypePacket {
    fn protocol_type(&self) -> u16;
}

pub trait TtypePacket {
    fn t_protocol_type(&self) -> u16;
}


pub struct Description;

impl Description {
    pub fn source_mac(start: usize, size: usize, packet: &impl MacPacket) -> Field {
        Field::new(start, size, format!("Source: {}", packet.source_mac()))
    }
    pub fn target_mac(start: usize, size: usize, packet: &impl MacPacket) -> Field {
        Field::new(start, size, format!("Destination: {}", packet.target_mac()))
    }
    pub fn ptype(start: usize, size: usize, packet: &impl PtypePacket) -> Field {
        Field::new(start, size, format!("Type: {} ({:#06x})",etype_mapper(packet.protocol_type()), packet.protocol_type()))
    }
    pub fn source_ip(start: usize, size: usize, packet: &impl IPPacket) -> Field {
        Field::new(start, size, format!("Source Address: {}", packet.source_ip_address()))
    }
    pub fn target_ip(start: usize, size: usize, packet: &impl IPPacket) -> Field {
        Field::new(start, size, format!("Destination Address: {}", packet.target_ip_address()))
    }
    pub fn t_protocol(start: usize, size: usize, packet: &impl TtypePacket) -> Field {
        let ttype = packet.t_protocol_type();
        Field::new(start, size, format!("Protocol: {} ({})", ip_protocol_type_mapper(ttype), ttype))
    }
    pub fn source_port(start: usize, size: usize, packet: &impl PortablePacket) -> Field {
        Field::new(start, size, format!("Source Port: {}", packet.source_port()))
    }
    pub fn target_port(start: usize, size: usize, packet: &impl PortablePacket) -> Field {
        Field::new(start, size, format!("Destination Port: {}", packet.target_port()))
    }
    pub fn packet_length(start: usize, size: usize, packet: &impl PlayloadPacket) -> Field {
        Field::new(start, size, format!("Length: {}", packet.len()))
    }
}


#[derive(Default, Clone)]
pub struct FileInfo {
    pub link_type: u16,
    pub file_type: FileType,
    pub start_time: u64,
    pub version: String,
}

pub struct IO;

impl IO {
    pub fn _read64(data: &[u8], endian: bool) -> u64 {
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
    cursor: Cell<usize>,
}
impl Reader<'_> {
    pub fn _get_data(&self) -> &[u8] {
        &self.data
    }
    pub fn cursor(&self) -> usize {
        return self.cursor.get();
    }
}

impl Reader<'_> {
    pub fn new(data: &[u8]) -> Reader {
        Reader {
            data,
            cursor: Cell::new(0),
        }
    }

    pub fn _move(&self, len: usize) {
        self.cursor.set(self.cursor.get() + len);
    }
    pub fn _slice(&self, len: usize) -> &[u8] {
        &self.data[self.cursor.get()..self.cursor.get() + len]
    }
    pub fn slice(&self, len: usize) -> &[u8] {
        // let c = self.cursor;
        let _tmp = self._slice(len);
        self._move(len);
        _tmp
    }
    pub fn read8(&self) -> u8 {
        let a = self.data[self.cursor.get()];
        self._move(1);
        u8::from_be_bytes([a])
    }

    pub fn read16(&self, endian: bool) -> u16 {
        let len = 2;
        let data: &[u8] = self._slice(len);
        self._move(len);
        IO::read16(data, endian)
    }

    pub fn read32(&self, endian: bool) -> u32 {
        let len = 4;
        let data: &[u8] = self._slice(len);
        self._move(len);
        IO::read32(data, endian)
    }

    pub fn read_mac(&self) -> Option<MacAddress> {
        let len = 6;
        if self.left() < len {
            return None;
        }
        let mut data: [u8; 6] = [0; 6];
        data.copy_from_slice(self._slice(len));
        self._move(len);
        Some(MacAddress { data })
    }
    pub fn read_ipv4(&self) -> Option<IPv4Address> {
        let len = 4;
        if self.left() < len {
            return None;
        }
        let mut data: [u8; 4] = [0; 4];
        data.copy_from_slice(self._slice(len));
        self._move(len);
        Some(IPv4Address { data })
    }
    pub fn left(&self) -> usize {
        self.data.len() - self.cursor.get()
    }
    pub fn has(&self) -> bool {
        return self.cursor.get() < self.data.len();
    }

    pub fn _read_mac(reader: &Reader) -> Option<MacAddress> {
        reader.read_mac()
    }

    pub fn _read_ipv4(reader: &Reader) -> Option<IPv4Address> {
        reader.read_ipv4()
    }
    pub fn _read8(reader: &Reader) -> u8 {
        reader.read8()
    }

    pub fn _read16_be(reader: &Reader) -> u16 {
        reader.read16(true)
    }

    pub fn _read16_ne(reader: &Reader) -> u16 {
        reader.read16(false)
    }
}

#[derive(Debug)]
pub struct MacAddress {
    pub data: [u8; 6],
}

impl fmt::Display for MacAddress {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let str = (&self.data)
            .iter()
            .map(|x| format!("{:x?}", x))
            .collect::<Vec<String>>()
            .join(":");
        fmt.write_str(str.as_str())?;
        Ok(())
    }
}

pub const DEF_EMPTY_MAC: MacAddress = MacAddress { data: [0; 6] };

#[derive(Debug)]
pub struct IPv4Address {
    pub data: [u8; 4],
}

impl fmt::Display for IPv4Address {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let str = (&self.data)
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<String>>()
            .join(".");
        fmt.write_str(str.as_str())?;
        Ok(())
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub enum FileType {
    PCAP,
    PCAPNG,
    #[default]
    NONE,
}

#[derive(Default, Debug, Copy, Clone)]
pub enum Protocol {
    ETHERNET,
    // SSL,
    IPV4,
    // IPV6,
    // ARP,
    // TCP,
    UDP,
    // ICMP,
    // ICMPV6,
    // IGMP,
    // DNS,
    // DHCP,
    // TLS,
    // HTTP,
    #[default]
    UNKNOWN,
}
