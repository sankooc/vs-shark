use std::fmt::Formatter;
use std::net::Ipv4Addr;

use crate::common::base::{BitFlag, BitType, FlagData};
use crate::common::FIELDSTATUS;
use anyhow::{bail, Result};
use pcap_derive::{Packet2, Visitor3, NINFO};

use crate::common::io::AReader;
use crate::{
    common::base::{Frame, PacketBuilder, PacketContext, PacketOpt},
    common::{io::Reader, Description, IPPacket, TtypePacket},
};

use super::ProtocolData;

pub fn excute(ipprototype: u8) -> &'static str {
    match ipprototype {
        1 => "icmp",
        2 => "igmp",
        6 => "tcp",
        17 => "udp",
        _ => "none",
    }
}


pub struct TOSFlag;

impl FlagData<u8> for TOSFlag {
    fn bits(inx: usize) -> Option<(u8, BitType<u8>)> {
        match inx {
            0 => Some((0x03, BitType::ABSENT("Explicit Congestion Notification: ECN-Capable Transport", "Explicit Congestion Notification: Not ECN-Capable Transport"))),
            1 => Some((0xfc, BitType::VAL("Differentiated Services Codepoint", 2, 0x3f))),
            _ => None,
        }
    }

    fn summary(title: &mut String, value: u8) {
        title.push_str(format!("Differentiated Services Field: {:#04x}", value).as_str());
    }

    fn summary_ext(_: &mut String, _: &str, _: bool) {}
}

pub struct Flag;

impl FlagData<u16> for Flag {
    fn bits(inx: usize) -> Option<(u16, BitType<u16>)> {
        match inx {
            0 => Some((0x8000, BitType::ABSENT("Reserved bit: set", "Reserved bit: Not set"))),
            1 => Some((0x4000, BitType::ABSENT("Don't fragment: Set", "Don't fragment: Not Set"))),
            2 => Some((0x2000, BitType::ABSENT("More fragments: set", "More fragments: Not set"))),
            3 => Some((0x1fff, BitType::VAL("Fragment offset", 0, 0x1fff))),
            _ => None,
        }
    }

    fn summary(title: &mut String, value: u16) {
        title.push_str(format!("Flags: {:#04x}", value).as_str());
    }

    fn summary_ext(_: &mut String, _: &str, _: bool) {}
}


#[derive(Default, Packet2, NINFO)]
pub struct IPv4 {
    pub source_ip: Option<Ipv4Addr>,
    pub target_ip: Option<Ipv4Addr>,
    tos: u8,
    total_len: u16,
    payload_len: Option<u16>,
    identification: u16,
    flag: u16,
    ttl: u8,
    t_protocol: u8,
    crc: u16,
}

impl IPv4 {
    fn tos(&self) -> Option<PacketContext<BitFlag<u8>>> {
        BitFlag::make::<TOSFlag>(self.tos)
    }
    fn flag(&self) -> Option<PacketContext<BitFlag<u16>>> {
        BitFlag::make::<Flag>(self.flag)
    }

    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _: Option<PacketOpt>) -> Result<()> {
        let _start = reader.left();
        let head = reader.read8()?;
        let head_len = head & 0x0f;
        packet.build_backward(reader, 1, "0100 .... = Version: 4".into());
        packet.build_backward(reader, 1, format!(".... {:04b} = Header Length: {} bytes ({})", head_len, head_len*4, head_len));
        p.tos = packet.build_packet_lazy(reader, Reader::_read8, None, Self::tos)?;
        let total_len = packet.build_lazy(reader, Reader::_read16_be, Some("ipv4.total.len"), |val| format!("Total Length: {}", val.total_len))?;
        let identification = packet.build_lazy(reader, Reader::_read16_be, Some("ipv4.identification"), |val| format!("Identification: {:#06x}", val.identification))?;
        let flag = packet.build_packet_lazy(reader, Reader::_read16_be, None, Self::flag)?;
        let ttl = packet.build_lazy(reader, Reader::_read8, Some("ipv4.ttl"), |val| format!("Time To Live: {}", val.ttl))?;
        let ipproto = packet.build_lazy(reader, Reader::_read8, Some("ipv4.protocol.type"), Description::t_protocol)?;
        let crc = packet.build_format(reader, Reader::_read16_be, None, "Header Checksum: {}")?;
        let source = packet.build_lazy(reader, Reader::_read_ipv4, Some("ipv4.source.ip"), Description::source_ip).ok();
        let target = packet.build_lazy(reader, Reader::_read_ipv4, Some("ipv4.target.ip"), Description::target_ip).ok();
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
        let _stop = reader.left();
        if total_len == 0 {
            p.payload_len = None;
        } else {
            if total_len < (_start - _stop) as u16 {
                bail!("error_len");
            }
            p.payload_len = Some(total_len - (_start - _stop) as u16);
        }
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
    fn payload_len(&self) -> Option<u16> {
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
#[derive(Visitor3)]
pub struct IP4Visitor;

impl IP4Visitor {
    fn visit2(&self, reader: &Reader) -> Result<(ProtocolData, &'static str)> {
        let packet = IPv4::create(reader, None)?;
        let p = packet.get();
        let ipproto = p.borrow().t_protocol;
        Ok((ProtocolData::IPV4(packet), excute(ipproto)))
    }
}
