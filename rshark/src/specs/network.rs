use std::fmt::{Formatter, Result};

use crate::{
    common::{Description, IPPacket, IPv4Address, Protocol, Reader, TtypePacket}, files::{Field, Frame, Initer, PacketContext, Visitor}
};

pub fn excute(ipprototype: u8, frame: &Frame, reader: &Reader) {
    match ipprototype {
        17 => {
            super::transport::UDPVisitor.visit(frame, reader);
        }
        _ => (),
    }
}


#[derive(Default)]
pub struct IPv4 {
    protocol: Protocol,
    source_ip: Option<IPv4Address>,
    target_ip: Option<IPv4Address>,
    total_len: u16,
    identification: u16,
    flag: u16,
    ttl: u8,
    t_protocol: u8,
    crc: u16,
}

impl IPPacket for IPv4 {
    fn source_ip_address(&self) -> String {
        self.source_ip.as_ref().unwrap().to_string()
    }

    fn target_ip_address(&self) -> String {
        self.target_ip.as_ref().unwrap().to_string()
    }
}

impl TtypePacket for IPv4{
    fn t_protocol_type(&self) -> u16 {
        self.t_protocol as u16
    }
}

impl std::fmt::Display for IPv4 {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let source = match &self.source_ip {
            Some(ip) => ip.to_string(),
            _ => "".into(),
        };
        let target = match &self.target_ip {
            Some(ip) => ip.to_string(),
            _ => "".into(),
        };
        let mn = format!(
            "Internet Protocol Version 4, Src: {}, Dst: {}",
            source, target
        );
        fmt.write_str(mn.as_str())?;
        Ok(())
    }
}

impl Initer<IPv4> for IPv4 {
    fn new() -> IPv4 {
        IPv4 {
            protocol: Protocol::IPV4,
            ..Default::default()
        }
    }
    fn get_protocol(&self) -> Protocol {
        self.protocol.clone()
    }
    fn info(&self) -> String {
        self.to_string().clone()
    }
}

pub struct IP4Visitor;

impl crate::files::Visitor for IP4Visitor {
    fn visit(&self, frame: &Frame, reader: &Reader) {
        let mut packet: PacketContext<IPv4> = Frame::create_packet();
        let head = packet.read(reader, Reader::_read8, None); //head
        let head_len = head & 0x0f;
        packet.read(reader, Reader::_read8, None); //tos
        let total_len = packet.read(reader, Reader::_read16_be, Some(|start, size, val| Field::new(start, size, format!("Total Length: {}", val.total_len))));
        let identification = packet.read(reader, Reader::_read16_be, Some(|start, size, val| Field::new(start, size, format!("Identification: {:#06x}", val.identification))));
        let flag = packet.read(reader, Reader::_read16_be, None);
        let ttl = packet.read(reader, Reader::_read8, Some(|start, size, val| Field::new(start, size, format!("Time To Live: {}", val.ttl))));
        let ipproto = packet.read(reader, Reader::_read8, Some(Description::t_protocol));
        let crc: u16 = packet.read(reader, Reader::_read16_be, None);
        let source = packet.read(reader, Reader::_read_ipv4, Some(Description::source_ip));
        let target = packet.read(reader, Reader::_read_ipv4, Some(Description::target_ip));
        // let ptype = packet.read(reader, Reader::_read16_be, Some(IPv4::_ptype));
        let p = &mut packet.val;
        p.total_len = total_len;
        p.identification = identification;
        p.flag = flag;
        p.ttl = ttl;
        p.t_protocol = ipproto;
        p.crc = crc;
        p.source_ip = source;
        p.target_ip = target;
        frame.summary.borrow_mut().source = p.source_ip.as_ref().unwrap().to_string();
        frame.summary.borrow_mut().target = p.target_ip.as_ref().unwrap().to_string();
        frame.summary.borrow_mut().protocol = p.protocol;
        let ext = head_len - 5;
        if ext > 0 {
            reader.slice((ext * 4) as usize);
        }
        frame.add_element(Box::new(packet));
        excute(ipproto,frame, reader);
    }
}