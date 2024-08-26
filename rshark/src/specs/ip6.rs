use std::fmt::Formatter;

use pcap_derive::Packet;
use anyhow::Result;

use crate::{
    common::{ContainProtocol, Description, IPPacket, IPv6Address, Protocol, Reader, TtypePacket}, files::{Frame, Initer, PacketContext, Visitor}
};

pub fn excute(ipprototype: u8, frame: &Frame, reader: &Reader) -> Result<()> {
    match ipprototype {
        17 => super::udp::UDPVisitor.visit(frame, reader),
        6 => super::tcp::TCPVisitor.visit(frame, reader),
        58 => super::icmp::ICMPv6Visitor.visit(frame, reader),
        _ => Ok(()),
    }
}


#[derive(Default, Packet)]
pub struct IPv6 {
    protocol: Protocol,
    source_ip: Option<IPv6Address>,
    target_ip: Option<IPv6Address>,
    total_len: u16,
    hop_limit: u8,
    t_protocol: u8,
}

impl IPPacket for IPv6 {
    fn source_ip_address(&self) -> String {
        self.source_ip.as_ref().unwrap().to_string()
    }

    fn target_ip_address(&self) -> String {
        self.target_ip.as_ref().unwrap().to_string()
    }
}

impl TtypePacket for IPv6{
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
        let mn = format!(
            "Internet Protocol Version 6, Src: {}, Dst: {}",
            source, target
        );
        fmt.write_str(mn.as_str())?;
        Ok(())
    }
}
impl IPv6 {
    fn _info(&self) -> String {
        return self.to_string()
    }
    fn _summary(&self) -> String {
        return self.to_string()
    }
}
pub struct IP6Visitor;

impl crate::files::Visitor for IP6Visitor {
    fn visit(&self, frame: &Frame, reader: &Reader) -> Result<()> {
        let packet: PacketContext<IPv6> = Frame::create_packet(Protocol::IPV6);
        let _ = reader.read32(true);
        let plen = packet._read_with_format_string_rs(reader, Reader::_read16_be, "Payload Length: {}")?;
        let ipproto = packet.read_with_string(reader, Reader::_read8, Description::t_protocol)?;
        let hop_limit = packet._read_with_format_string_rs(reader, Reader::_read8, "Hop Limit: {}")?;

        let source = packet.read_with_string(reader, Reader::_read_ipv6, Description::source_ip);
        let target = packet.read_with_string(reader, Reader::_read_ipv6, Description::target_ip);
        let mut p = packet.get().borrow_mut();
        p.source_ip = source.ok();
        p.target_ip = target.ok();
        p.total_len = plen;
        p.hop_limit = hop_limit;
        drop(p);
        frame.update_host(packet.get().borrow());
        frame.add_element(Box::new(packet));
        excute(ipproto,frame, reader)
    }
}
