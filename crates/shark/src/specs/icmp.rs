use std::fmt::Formatter;

use anyhow::Result;
use pcap_derive::{Packet, Packet2, Visitor3, NINFO};

use crate::common::io::AReader;
use crate::{
    common::base::{Frame, PacketContext, PacketOpt},
    common::io::Reader,
    constants::icmpv6_type_mapper,
};

use super::ProtocolData;
//https://datatracker.ietf.org/doc/html/rfc792
#[derive(Default, Packet2, NINFO)]
pub struct ICMP {
    _type: u8,
    code: u8,
    checksum: u16,
    header: ICMPHeader,
}

#[derive(Default)]
pub enum ICMPHeader {
    #[default]
    UNKOWN,
    ECHO(Echo),
}
pub struct Echo {
    pub identifier: u16,
    pub sequence: u16,
}

impl std::fmt::Display for ICMP {
    fn fmt(&self, _fmt: &mut Formatter) -> std::fmt::Result {
        _fmt.write_str("Internet Control Message Protocol")
    }
}
impl ICMP {
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _: Option<PacketOpt>) -> Result<()> {
        p._type = packet.build_lazy(reader, Reader::_read8, Some("icmp.type"), ICMP::type_desc)?;
        p.code = packet.build_format(reader, Reader::_read8, Some("icmp.code"), "Code {}")?;
        p.checksum = reader.read16(false)?;
        packet._build(reader, reader.cursor() - 2, 2, None, format!("Checksum: {:#06x}", p.checksum));
        match p._type {
            0x00 | 0x08 => {
                let identifier = packet.build_format(reader, Reader::_read16_be, Some("icmp.identifier"), "Identifier: {}")?;
                let sequence = packet.build_format(reader, Reader::_read16_be, Some("icmp.sequence.no"), "Sequence Number: {}")?;
                p.header = ICMPHeader::ECHO(Echo { identifier, sequence });
            }
            _ => {}
        }
        Ok(())
    }
    fn _type(&self) -> String {
        let _t = self._type;
        let code = self.code;
        let def = format!("type:{}", _t);
        match _t {
            0 => "Echo reply".into(),
            3 => match code {
                0 => "Destination network unreachableunknow".into(),
                1 => "Destination host unreachable".into(),
                2 => "Destination protocol unreachable".into(),
                3 => "Destination port unreachable".into(),
                4 => "Fragmentation required, and DF flag set".into(),
                5 => "Source route failed".into(),
                6 => "Destination network unknown".into(),
                7 => "Destination host unknown".into(),
                8 => "Source host isolated".into(),
                9 => "Network administratively prohibited".into(),
                10 => "Host administratively prohibited".into(),
                11 => "Network unreachable for ToS".into(),
                12 => "Host unreachable for ToS".into(),
                13 => "Communication administratively prohibited".into(),
                14 => "Host Precedence Violation".into(),
                15 => "Precedence cutoff in effect".into(),
                _ => def,
            },
            4 => "Source quench".into(),
            5 => match code {
                0 => "Redirect datagrams for the Network".into(),
                1 => "Redirect datagrams for the Host".into(),
                2 => "Redirect datagrams for the Type of Service and Network".into(),
                3 => "Redirect datagrams for the Type of Service and Host".into(),
                _ => def,
            },
            8 => "Echo request".into(),
            9 => "Router Advertisement".into(),
            10 => "Router discovery/selection/solicitation".into(),
            11 => match code {
                0 => "TTL expired in transit".into(),
                1 => "Fragment reassembly time exceeded".into(),
                _ => def,
            },
            12 => match code {
                0 => "pointer indicates the error".into(),
                _ => def,
            },
            13 => "timestamp message".into(),
            14 => "timestamp reply message".into(),
            15 => "Information Request".into(),
            16 => "Information Reply".into(),
            43 => match code {
                0 => "No Error".into(),
                1 => "Malformed Query".into(),
                2 => "No Such Interface".into(),
                3 => "No Such Table Entry".into(),
                4 => "Multiple Interfaces Satisfy Query".into(),
                _ => def,
            },
            _ => def,
        }
    }
    fn type_desc(&self) -> String {
        format!("Type: {} ({})", self.code, self._type())
    }
}
#[derive(Visitor3)]
pub struct ICMPVisitor;

impl ICMPVisitor {
    pub fn visit2(&self, reader: &Reader) -> Result<(ProtocolData, &'static str)> {
        //https://book.huihoo.com/iptables-tutorial/x1078.htm
        let packet = ICMP::create(reader, None)?;
        Ok((super::ProtocolData::ICMP(packet), "none"))
    }
}

#[derive(Default, Packet, NINFO)]
pub struct ICMP6 {
    _type: u8,
    code: u8,
    checksum: u16,
}
impl std::fmt::Display for ICMP6 {
    fn fmt(&self, _fmt: &mut Formatter) -> std::fmt::Result {
        _fmt.write_str("Internet Control Message Protocol v6")
    }
}
impl ICMP6 {
    fn _type(&self) -> &'static str {
        icmpv6_type_mapper(self._type as u16)
    }
    fn type_desc(&self) -> String {
        format!("Type: {} ({})", self.code, self._type())
    }
    fn checksum(&self) -> String {
        format!("Checksum: {:#06x}", self.checksum)
    }
}
#[derive(Visitor3)]
pub struct ICMPv6Visitor;

impl ICMPv6Visitor {
    fn visit2(&self, reader: &Reader) -> Result<(ProtocolData, &'static str)> {
        let packet: PacketContext<ICMP6> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        p._type = packet.build_lazy(reader, Reader::_read8, Some("icmpv6.type"), ICMP6::type_desc)?;
        p.code = packet.build_format(reader, Reader::_read8, Some("icmpv6.sequence.no"), "Code {}")?;
        p.checksum = packet.build_lazy(reader, Reader::_read16_be, None, ICMP6::checksum)?;
        drop(p);
        Ok((super::ProtocolData::ICMPv6(packet), "none"))
    }
}
