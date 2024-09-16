use std::fmt::Formatter;

use anyhow::Result;
use pcap_derive::{Packet2, NINFO};

use crate::{
    common::{Description, IPPacket, IPv6Address, Reader, TtypePacket},
    files::{Frame, Initer, PacketContext, PacketOpt, Visitor},
};

use super::ProtocolData;

pub fn excute(ipprototype: u8) -> &'static str {
    match ipprototype {
        17 => "udp",
        6 => "tcp",
        58 => "icmpv6",
        _ => "none",
    }
}

#[derive(Default, Packet2, NINFO)]
pub struct IPv6 {
    source_ip: Option<IPv6Address>,
    target_ip: Option<IPv6Address>,
    total_len: u16,
    hop_limit: u8,
    t_protocol: u8,
}

impl IPv6 {
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _: Option<PacketOpt>) -> Result<()> {
        let _ = reader.read32(true);
        let plen = packet.build_format(reader, Reader::_read16_be, "Payload Length: {}")?;
        let ipproto = packet.build_lazy(reader, Reader::_read8, Description::t_protocol)?;
        let hop_limit = packet.build_format(reader, Reader::_read8, "Hop Limit: {}")?;
        let source = packet.build_lazy(reader, Reader::_read_ipv6, Description::source_ip);
        let target = packet.build_lazy(reader, Reader::_read_ipv6, Description::target_ip);
        p.t_protocol = ipproto;
        p.source_ip = source.ok();
        p.target_ip = target.ok();
        p.total_len = plen;
        p.hop_limit = hop_limit;
        Ok(())
    }
}

impl IPPacket for IPv6 {
    fn source_ip_address(&self) -> String {
        self.source_ip.as_ref().unwrap().to_string()
    }

    fn target_ip_address(&self) -> String {
        self.target_ip.as_ref().unwrap().to_string()
    }
    fn payload_len(&self) -> u16 {
        self.total_len
    }
}

impl TtypePacket for IPv6 {
    fn t_protocol_type(&self) -> u16 {
        self.t_protocol as u16
    }
}

impl std::fmt::Display for IPv6 {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        let source = match &self.source_ip {
            Some(ip) => ip.to_string(),
            _ => "".into(),
        };
        let target = match &self.target_ip {
            Some(ip) => ip.to_string(),
            _ => "".into(),
        };
        let mn = format!("Internet Protocol Version 6, Src: {}, Dst: {}", source, target);
        fmt.write_str(mn.as_str())?;
        Ok(())
    }
}
pub struct IP6Visitor;

impl crate::files::Visitor for IP6Visitor {
    fn visit(&self, _: &Frame, reader: &Reader) -> Result<(ProtocolData, &'static str)> {
        let packet: PacketContext<IPv6> = IPv6::create(reader, None)?;
        let p = packet.get();
        let ipproto = p.borrow().t_protocol;
        Ok((ProtocolData::IPV6(packet), excute(ipproto)))
    }
}
