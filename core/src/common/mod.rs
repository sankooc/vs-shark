use thiserror::Error;

use crate::constants::{etype_mapper, ip_protocol_type_mapper};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::fmt;

pub mod io;

#[derive(Error, Debug)]
pub enum DataError {
    #[error("unsupport file type")]
    UnsupportFileType,
    #[error("bit error")]
    BitSize,
    // #[error("unknown data store error")]
    // Unknown,
}

pub trait PortPacket {
    fn source_port(&self) -> u16;
    fn target_port(&self) -> u16;
}

pub trait PlayloadPacket {
    fn len(&self) -> u16;
}

pub trait IPPacket {
    fn source_ip_address(&self) -> String;
    fn target_ip_address(&self) -> String;
    fn payload_len(&self) -> u16;
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
    pub fn source_port(packet: &impl PortPacket) -> String {
        format!("Source Port: {}", packet.source_port())
    }
    pub fn target_port(packet: &impl PortPacket) -> String {
        format!("Destination Port: {}", packet.target_port())
    }
    pub fn packet_length(packet: &impl PlayloadPacket) -> String {
        format!("Length: {}", packet.len())
    }
}

#[derive(Default, Clone)]
pub struct FileInfo {
    pub link_type: u32,
    pub file_type: FileType,
    pub start_time: u64,
    pub version: String,
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
    _ins: Ipv4Addr,
}

impl IPv4Address {
    pub fn new(data: [u8; 4]) -> Self {
        let _ins = Ipv4Addr::new(data[0], data[1], data[2], data[3]);
        Self{_ins}
    }
}
impl fmt::Display for IPv4Address {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&self._ins.to_string())
    }
}

pub struct IPv6Address {
    _ins: Ipv6Addr
}

impl IPv6Address {
    fn new(data: [u8; 16]) -> Self {
        let mut args:[u16; 8] = [0; 8];
        for inx in 0..8 {
            let _inx = (inx * 2) as usize;
            args[inx] = ((data[_inx] as u16) * 0x0100) + (data[_inx + 1] as u16);
        }
        let _ins = Ipv6Addr::new(args[0], args[1],args[2],args[3],args[4],args[5],args[6],args[7]);
        Self{ _ins }
    }
}
impl std::fmt::Display for IPv6Address {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(&self._ins.to_string())?;
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
