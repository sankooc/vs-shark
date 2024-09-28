use std::fmt::Formatter;

use anyhow::{bail, Result};
use pcap_derive::{Packet2, NINFO};
use crate::common::FIELDSTATUS;

use crate::{
    common::{io::Reader, Description, IPPacket, IPv4Address, TtypePacket},
    files::{Frame, PacketBuilder, PacketContext, PacketOpt},
};
use crate::common::io::AReader;

use super::ProtocolData;

// pub fn excute(ipprototype: u8, frame: &Frame, reader: &Reader) -> Result<()> {
//     match ipprototype {
//         1 => super::icmp::ICMPVisitor.visit(frame, reader),
//         2 => super::igmp::IGMPVisitor.visit(frame, reader),
//         6 => super::tcp::TCPVisitor.visit(frame, reader),
//         17 => super::udp::UDPVisitor.visit(frame, reader),
//         _ => Ok(()),
//     }
// }
pub fn excute(ipprototype: u8) -> &'static str {
    match ipprototype {
        1 => "icmp",
        2 => "igmp",
        6 => "tcp",
        17 => "udp",
        _ => "none",
    }
}

#[derive(Default, Packet2, NINFO)]
pub struct IPv4 {
    source_ip: Option<IPv4Address>,
    target_ip: Option<IPv4Address>,
    total_len: u16,
    payload_len: u16,
    identification: u16,
    flag: u16,
    ttl: u8,
    t_protocol: u8,
    crc: u16,
}

impl IPv4 {
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _: Option<PacketOpt>) -> Result<()> {
        let _start = reader.left()?;
        let head = reader.read8()?;
        let head_len = head & 0x0f;
        let _ = reader.read8(); //tos
        let total_len = packet.build_lazy(reader, Reader::_read16_be, |val| format!("Total Length: {}", val.total_len))?;
        let identification = packet.build_lazy(reader, Reader::_read16_be, |val| format!("Identification: {:#06x}", val.identification))?;
        let flag = reader.read16(false)?;
        let ttl = packet.build_lazy(reader, Reader::_read8, |val| format!("Time To Live: {}", val.ttl))?;
        let ipproto = packet.build_lazy(reader, Reader::_read8, Description::t_protocol)?;
        let crc: u16 = reader.read16(false)?;
        let source = packet.build_lazy(reader, Reader::_read_ipv4, Description::source_ip).ok();
        let target = packet.build_lazy(reader, Reader::_read_ipv4, Description::target_ip).ok();
        p.total_len = total_len;
        p.identification = identification;
        p.flag = flag;
        p.ttl = ttl;
        p.t_protocol = ipproto;
        p.crc = crc;
        p.source_ip = source;
        p.target_ip = target;
        let ext = head_len - 5;
        if ext > 0 {
            reader.slice((ext * 4) as usize);
        }
        let _stop = reader.left()?;
        if total_len < (_start - _stop) as u16 {
            bail!("error_len");
        }
        p.payload_len = total_len - (_start - _stop) as u16;
        Ok(())
    }
}

impl IPPacket for IPv4 {
    fn source_ip_address(&self) -> String {
        self.source_ip.as_ref().unwrap().to_string()
    }
    fn target_ip_address(&self) -> String {
        self.target_ip.as_ref().unwrap().to_string()
    }
    fn payload_len(&self) -> u16 {
        self.payload_len
    }
}

impl TtypePacket for IPv4 {
    fn t_protocol_type(&self) -> u16 {
        self.t_protocol as u16
    }
}

impl std::fmt::Display for IPv4 {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        let source = match &self.source_ip {
            Some(ip) => ip.to_string(),
            _ => "".into(),
        };
        let target = match &self.target_ip {
            Some(ip) => ip.to_string(),
            _ => "".into(),
        };
        let mn = format!("Internet Protocol Version 4, Src: {}, Dst: {}", source, target);
        fmt.write_str(mn.as_str())
    }
}
impl IPv4 {
    fn _info(&self) -> String {
        return self.to_string();
    }
    fn _summary(&self) -> String {
        return self.to_string();
    }
}
pub struct IP4Visitor;

impl crate::files::Visitor for IP4Visitor {
    fn visit(&self, _: &Frame, reader: &Reader) -> Result<(ProtocolData, &'static str)> {
        let packet = IPv4::create(reader, None)?;
        let p = packet.get();
        let ipproto = p.borrow().t_protocol;
        Ok((ProtocolData::IPV4(packet), excute(ipproto)))
    }
}
