use std::fmt::Formatter;

use pcap_derive::Packet;
use anyhow::Result;

use crate::{
    common::{ContainProtocol, Description, IPPacket, IPv4Address, Protocol, Reader, TtypePacket}, files::{Frame, Initer, PacketContext, Visitor}
};

pub fn excute(ipprototype: u8, frame: &Frame, reader: &Reader) -> Result<()> {
    match ipprototype {
        17 => {
            return super::udp::UDPVisitor.visit(frame, reader);
        },
        6 => {
            return super::tcp::TCPVisitor.visit(frame, reader);
        },
        _ => Ok(()),
    }
}


#[derive(Default, Packet)]
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
            "Internet Protocol Version 4, Src: {}, Dst: {}",
            source, target
        );
        fmt.write_str(mn.as_str())?;
        Ok(())
    }
}
impl IPv4 {
    fn _info(&self) -> String {
        return self.to_string()
    }
    fn _summary(&self) -> String {
        return self.to_string()
    }
}
pub struct IP4Visitor;

impl crate::files::Visitor for IP4Visitor {
    fn visit(&self, frame: &Frame, reader: &Reader) -> Result<()> {
        let packet: PacketContext<IPv4> = Frame::create_packet(Protocol::IPV4);
        let head = reader.read8()?;
        let head_len = head & 0x0f;
        let _ = reader.read8();//tos
        let total_len = packet.read_with_string(reader, Reader::_read16_be, | val| format!("Total Length: {}", val.total_len))?;
        let identification = packet.read_with_string(reader, Reader::_read16_be, | val| format!("Identification: {:#06x}", val.identification))?;
        let flag = reader.read16(false)?;
        let ttl = packet.read_with_string(reader, Reader::_read8, |val| format!("Time To Live: {}", val.ttl))?;
        let ipproto = packet.read_with_string(reader, Reader::_read8, Description::t_protocol)?;
        let crc: u16 = reader.read16(false)?;
        let source = packet.read_with_string(reader, Reader::_read_ipv4, Description::source_ip).ok();
        let target = packet.read_with_string(reader, Reader::_read_ipv4, Description::target_ip).ok();
        // let ptype = packet.read(reader, Reader::_read16_be, Some(IPv4::_ptype));
        let mut p = packet.get().borrow_mut();
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
        drop(p);
        frame.update_host(packet.get().borrow());
        frame.add_element(Box::new(packet));
        excute(ipproto,frame, reader)
    }
}
