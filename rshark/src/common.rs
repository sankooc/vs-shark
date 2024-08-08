use crate::constants::{etype_mapper, ip_protocol_type_mapper};
use std::cell::Cell;
use std::fmt;
use std::rc::Rc;
use std::str;
use std::str::from_utf8;

pub trait ContainProtocol {
    fn get_protocol(&self) -> Protocol;
    // fn info(&self) -> String;
}

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
    // pub fn swap<T>(getter: fn(&T) -> String) -> impl Fn(usize, usize, &T) -> Field {
    //     move |start: usize, size: usize, t: &T| {
    //         return Field::new(start, size, getter(t));
    //     }
    // }
    pub fn source_mac(packet: &impl MacPacket) -> String {
        format!("Source: {}", packet.source_mac())
    }
    pub fn target_mac(packet: &impl MacPacket) -> String {
        format!("Destination: {}", packet.target_mac())
    }
    pub fn ptype(packet: &impl PtypePacket) -> String {
        format!(
            "Type: {} ({:#06x})",
            etype_mapper(packet.protocol_type()),
            packet.protocol_type()
        )
    }
    pub fn source_ip(packet: &impl IPPacket) -> String {
        format!("Source Address: {}", packet.source_ip_address())
    }
    pub fn target_ip(packet: &impl IPPacket) -> String {
        format!("Destination Address: {}", packet.target_ip_address())
    }
    pub fn t_protocol(packet: &impl TtypePacket) -> String {
        let ttype = packet.t_protocol_type();
        format!("Protocol: {} ({:#06x})", ip_protocol_type_mapper(ttype), ttype)
    }
    pub fn source_port(packet: &impl PortablePacket) -> String {
        format!("Source Port: {}", packet.source_port())
    }
    pub fn target_port(packet: &impl PortablePacket) -> String {
        format!("Destination Port: {}", packet.target_port())
    }
    pub fn packet_length(packet: &impl PlayloadPacket) -> String {
        format!("Length: {}", packet.len())
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
#[derive(Clone)]
pub struct Reader<'a> {
    _data: Option<&'a [u8]>,
    _raw: Rc<Vec<u8>>,
    cursor: Cell<usize>,
}
impl Reader<'_> {
    pub fn _get_data(&self) -> &[u8] {
        match self._data {
            Some(data) => return data,
            _ => (),
        }
        return self._raw.as_ref();
    }
    pub fn cursor(&self) -> usize {
        return self.cursor.get();
    }
    pub fn get_raw(&self) -> Rc<Vec<u8>> {
        self._raw.clone()
    }
}

impl Reader<'_> {
    pub fn new(data: &[u8]) -> Reader {
        Reader {
            _data: Some(data),
            _raw: Rc::new(Vec::new()),
            cursor: Cell::new(0),
        }
    }
    pub fn new_raw(raw: Rc<Vec<u8>>) -> Reader<'static> {
        Reader {
            _data: None,
            _raw: raw,
            cursor: Cell::new(0),
        }
    }

    pub fn _set(&self, cursor: usize){
        self.cursor.set(cursor);
    }
    pub fn _move(&self, len: usize) {
        self.cursor.set(self.cursor.get() + len);
    }
    pub fn _back(&self, len: usize) {
        self.cursor.set(self.cursor.get() - len);
    }
    pub fn _slice(&self, len: usize) -> &[u8] {
        &self._get_data()[self.cursor.get()..self.cursor.get() + len]
    }
    pub fn slice(&self, len: usize) -> &[u8] {
        // let c = self.cursor;
        let _tmp = self._slice(len);
        self._move(len);
        _tmp
    }
    pub fn read8(&self) -> u8 {
        let a = self._get_data()[self.cursor.get()];
        self._move(1);
        u8::from_be_bytes([a])
    }

    pub fn read_string(&self, size: usize) -> String{
        let _data = self.slice(size);
        from_utf8(_data).unwrap().into()
    }
    pub fn read_dns_query(&self) -> String {
        let mut list = Vec::new();
        loop {
            let len = self.read8() as usize;
            if len > 0 {
                let st = str::from_utf8(self.slice(len)).unwrap();
                list.push(st);
            } else {
                break;
            }
        }
        list.into_iter().collect::<Vec<_>>().join(".")
    }
    pub fn read_compress_string(&self) -> (String, u16){
        let mut list:Vec<String> = Vec::new();
        // let _join = || list.into_iter().collect::<Vec<_>>().join(".");
        loop {
            if self.left() == 2 {
                return (list.into_iter().collect::<Vec<_>>().join("."), self.read16(true));
            }
            let _size = self.read8();
            if _size > 0 {
                let str = self.read_string(_size as usize);
                list.push(str);
            }
            let next = self._get_data()[self.cursor.get()];
            if next == 0 {
                return (list.into_iter().collect::<Vec<_>>().join("."), 0);
            }
            if next >= 0xc0 {
                return (list.into_iter().collect::<Vec<_>>().join("."), self.read16(true));
            }
            if next > self.left() as u8 {
                return (list.into_iter().collect::<Vec<_>>().join("."), 0);
            }
        }
    }
    pub fn read_dns_compress_string(&self, archor: usize, def: &str, refer: u16) -> String {
        let inx = (refer & 0x3fff) as usize;
        let from = archor + inx;
        let _reader = self.clone();
        _reader._set(from);
        let mut rs = String::from(def);
        if def.len() > 0 {
            rs.push_str(".");
        }
        let (str, refer2) = _reader.read_compress_string();
        rs.push_str(str.as_str());
        if refer2 > 0 {
            self.read_dns_compress_string(archor, rs.as_str(), refer2)
        } else {
            rs
        }
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
        self._get_data().len() - self.cursor.get()
    }
    pub fn has(&self) -> bool {
        return self.cursor.get() < self._get_data().len();
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
    
    pub fn _read32_be(reader: &Reader) -> u32 {
        reader.read32(true)
    }

    pub fn _read32_ne(reader: &Reader) -> u32 {
        reader.read32(false)
    }

    pub fn _read_dns_query(reader: &Reader) -> String {
        reader.read_dns_query()
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
        if str == "255.255.255.255"{
            fmt.write_str("Boardcast")?;
        } else {
            fmt.write_str(str.as_str())?;
        }
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
    PPPoESS,
    SSL,
    IPV4,
    // IPV6,
    // ARP,
    // TCP,
    UDP,
    // ICMP,
    // ICMPV6,
    // IGMP,
    DNS,
    // DHCP,
    // TLS,
    // HTTP,
    #[default]
    UNKNOWN,
}