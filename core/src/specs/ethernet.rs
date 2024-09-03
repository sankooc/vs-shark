use pcap_derive::{Packet, NINFO};

use crate::common::{Description, MacAddress, MacPacket, PtypePacket, DEF_EMPTY_MAC};
use crate::constants::{etype_mapper, link_type_mapper, ssl_type_mapper};
use crate::files::{InfoPacket, Visitor};
use crate::{
    common::Reader,
    files::{Frame, Initer, PacketContext},
};
use std::fmt::{Display, Write};
use anyhow::{Ok, Result};

use super::ProtocolData;

#[derive(Default, Packet, NINFO)]
pub struct Ethernet {
    source_mac: Option<MacAddress>,
    target_mac: Option<MacAddress>,
    len: u16,
    ptype: u16,
}

impl Display for Ethernet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let source = self
            .source_mac
            .as_ref()
            .unwrap_or(&DEF_EMPTY_MAC)
            .to_string();
        let target = self
            .target_mac
            .as_ref()
            .unwrap_or(&DEF_EMPTY_MAC)
            .to_string();
        f.write_str(format!("Ethernet II, Src: {}, Dst: {}", source, target).as_str())
    }
}

impl MacPacket for Ethernet {
    fn source_mac(&self) -> String {
        self.source_mac
            .as_ref()
            .unwrap_or(&DEF_EMPTY_MAC)
            .to_string()
    }

    fn target_mac(&self) -> String {
        self.target_mac
            .as_ref()
            .unwrap_or(&DEF_EMPTY_MAC)
            .to_string()
    }
}
impl PtypePacket for Ethernet {
    fn protocol_type(&self) -> u16 {
        self.ptype
    }
}

pub struct EthernetVisitor;

impl Visitor for EthernetVisitor {
    fn visit(&self, frame: &Frame, reader: &Reader) -> Result<()> {
        let packet: PacketContext<Ethernet> = Frame::create_packet();

        let mut p = packet.get().borrow_mut();
        p.source_mac = packet.build_lazy(reader, Reader::_read_mac, Description::source_mac).ok();
        p.target_mac = packet.build_lazy(reader, Reader::_read_mac, Description::target_mac).ok();
        let ptype = packet.build_lazy(reader, Reader::_read16_be, Description::ptype)?;
        if reader.left()? == ptype as usize {
            p.len = ptype;
            // info!("{}", ptype); // IEEE 802.3
            return Ok(());
        }
        p.ptype = ptype;
        drop(p);
        frame.add_element(ProtocolData::ETHERNET(packet));
        excute(ptype, frame, reader)
    }
}
#[derive(Default, Packet, NINFO)]
pub struct PPPoESS {
    version: u8,
    _type: u8,
    code: u8,
    session_id: u16,
    payload: u16,
    ptype: u16,
}
impl Display for PPPoESS {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        _f.write_str("PPP-over-Ethernet Session")
    }
}

impl PPPoESS{
    fn code(&self) -> String{
        format!("Code: Session Data ({:#04x})", self.code)
    }
    fn session_id(&self) -> String{
        format!("Session ID: {:#06x}", self.session_id)
    }
    fn payload(&self) -> String{
        format!("Payload Length: {}", self.payload)
    }
    fn ptype(&self) -> String{
        match self.ptype {
            33 => "Protocol: Internet Protocol version 4 (0x0021)".into(),
            87 => "Protocol: Internet Protocol version 6 (0x0057)".into(),
            _ => format!("Unknown: {}", self.ptype),
        }
    }
}
struct PPPoESSVisitor;
impl Visitor for PPPoESSVisitor {
    fn visit(&self, frame: &Frame, reader: &Reader) -> Result<()> {
        let packet: PacketContext<PPPoESS> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        let head = reader.read8()?;
        p.version = head >> 4;
        p._type = head & 0x0f;
        p.code = packet.build_lazy(reader, Reader::_read8, PPPoESS::code)?;
        p.session_id = packet.build_lazy(reader, Reader::_read16_be, PPPoESS::session_id)?;
        p.payload = packet.build_lazy(reader, Reader::_read16_be, PPPoESS::payload)?;
        p.ptype = packet.build_lazy(reader, Reader::_read16_be, PPPoESS::ptype)?;
        let code = p.code;
        let ptype = p.ptype;
        drop(p);
        frame.add_element(ProtocolData::PPPoESS(packet));
        if code == 0 {
            return match ptype {
                33 => super::ip4::IP4Visitor.visit(frame, reader),
                87 => super::ip6::IP6Visitor.visit(frame, reader),
                _ => Ok(()),
            }
        }
        Ok(())
    }
}
#[derive(Default, Packet,NINFO)]
pub struct SSL {
    _type: u16,
    ltype: u16,
    len: u16,
    source: Option<MacAddress>,
    ptype: u16,
}
impl Display for SSL {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        _f.write_str("Linux cooked capture v1")
    }
}
impl SSL {
    fn _type(&self) -> String{
        format!("Packet Type: {}", ssl_type_mapper(self._type))
    }
    fn ltype(&self) -> String{
        format!("Link-layer address type: {} ({})", link_type_mapper(self.ltype), self.ltype)
    }
    fn len_str(&self) -> String{
        format!("Link-layer address length: {}", self._type)
    }
    fn source_str(&self) -> String{
        let add = self.source
        .as_ref()
        .unwrap_or(&DEF_EMPTY_MAC)
        .to_string();
        format!("Source: {}", add)
    }
    fn ptype_str(&self) -> String{
        format!("Protocol: {} ({:#06x})", etype_mapper(self.ptype), self.ptype)
    }
}

pub struct SSLVisitor;
impl Visitor for SSLVisitor {
    fn visit(&self, frame: &Frame, reader: &Reader) -> Result<()>{
        let packet:PacketContext<SSL> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        p._type = packet.build_lazy(reader, Reader::_read16_be, SSL::_type)?;
        p.ltype = packet.build_lazy(reader, Reader::_read16_be, SSL::ltype)?;
        p.len = packet.build_lazy(reader, Reader::_read16_be, SSL::len_str)?;
        p.source = packet.build_lazy(reader, Reader::_read_mac, SSL::source_str).ok();
        reader._move(2);
        p.ptype = packet.build_lazy(reader, Reader::_read16_be, SSL::ptype_str)?;
        let ptype = p.ptype;
        drop(p);
        frame.add_element(ProtocolData::SSL(packet));
        excute(ptype, frame, reader)
    }
}

pub fn excute(etype: u16, frame: &Frame, reader: &Reader) -> Result<()>{
    match etype {
        2048 => {
            return super::ip4::IP4Visitor.visit(frame, reader);
        }
        34525 => {
            return super::ip6::IP6Visitor.visit(frame, reader);
        }
        0x0806 => {
            return super::arp::ARPVisitor.visit(frame, reader);
        }
        34916 => {
            return PPPoESSVisitor.visit(frame, reader);
        }
        _ => Ok(()),
    }
}
