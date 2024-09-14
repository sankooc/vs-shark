use pcap_derive::{Packet, Packet2, NINFO};

use crate::common::{Description, MacAddress, MacPacket, PtypePacket, DEF_EMPTY_MAC};
use crate::constants::{etype_mapper, link_type_mapper, ssl_type_mapper};
use crate::files::{PacketOpt, Visitor};
use crate::{
    common::Reader,
    files::{Frame, Initer, PacketContext},
};
use std::cell::RefCell;
use std::fmt::Display;
use anyhow::{Ok, Result};

pub mod radiotap;

use super::ProtocolData;

#[derive(Default,Packet2, NINFO)]
pub struct Ethernet {
    source_mac: Option<MacAddress>,
    target_mac: Option<MacAddress>,
    len: u16,
    ptype: u16,
}
impl Ethernet{
    fn _create<PacketOpt>(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _:Option<PacketOpt>) -> Result<()> {
        p.source_mac = packet.build_lazy(reader, Reader::_read_mac, Description::source_mac).ok();
        p.target_mac = packet.build_lazy(reader, Reader::_read_mac, Description::target_mac).ok();
        let ptype = packet.build_lazy(reader, Reader::_read16_be, Description::ptype)?;
        if reader.left()? == ptype as usize {
            p.len = ptype;
            // info!("{}", ptype); // IEEE 802.3
            return Ok(());
        }
        p.ptype = ptype;
        Ok(())
    }
    // pub fn create<PacketOpt>(reader: &Reader, opt: Option<PacketOpt>) -> Result<PacketContext<Self>> {
    //     let packet: PacketContext<Self> = Frame::create_packet();
    //     let mut p = packet.get().borrow_mut();
    //     let rs = Self::_create(reader, &packet, &mut p, opt);
    //     drop(p);
    //     rs?;
    //     Ok(packet)
    // }
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
        let packet = Ethernet::create(reader, None)?;
        let val:&RefCell<Ethernet> = packet.get();
        let ptype = val.borrow().ptype;
        frame.add_element(ProtocolData::ETHERNET(packet));
        _excute(ptype, frame, reader)
    }
}
#[derive(Default, Packet2, NINFO)]
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

impl PPPoESS {
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>,_:Option<usize>) -> Result<()> {
        let head = reader.read8()?;
        p.version = head >> 4;
        p._type = head & 0x0f;
        p.code = packet.build_lazy(reader, Reader::_read8, PPPoESS::code)?;
        p.session_id = packet.build_lazy(reader, Reader::_read16_be, PPPoESS::session_id)?;
        p.payload = packet.build_lazy(reader, Reader::_read16_be, PPPoESS::payload)?;
        p.ptype = packet.build_lazy(reader, Reader::_read16_be, PPPoESS::ptype)?;
        Ok(())
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
        let packet = PPPoESS::create(reader, None)?;
        let p = packet.get();
        let code = p.borrow().code;
        let ptype = p.borrow().ptype;
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
#[derive(Default, Packet2, NINFO)]
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
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _:Option<PacketOpt>) -> Result<()> {
        p._type = packet.build_lazy(reader, Reader::_read16_be, SSL::_type)?;
        p.ltype = packet.build_lazy(reader, Reader::_read16_be, SSL::ltype)?;
        p.len = packet.build_lazy(reader, Reader::_read16_be, SSL::len_str)?;
        p.source = packet.build_lazy(reader, Reader::_read_mac, SSL::source_str).ok();
        reader._move(2);
        p.ptype = packet.build_lazy(reader, Reader::_read16_be, SSL::ptype_str)?;
        Ok(())
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
        let packet = SSL::create(reader, None)?;
        let p = packet.get();
        let ptype = p.borrow().ptype;
        frame.add_element(ProtocolData::SSL(packet));
        _excute(ptype, frame, reader)
    }
}
#[derive(Clone, Default, Packet2, NINFO)]
pub struct IEEE1905A {
    version: u8,
    message_type: u16,
    message_id: u16,
    flagment: u8,

}
impl Display for IEEE1905A {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        _f.write_str("IEEE 1905.1a")
    }
}
impl IEEE1905A {
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _:Option<PacketOpt>) -> Result<()> {
        p.version = packet.build_format(reader, Reader::_read8, "Message version: {}")?;
        reader.read8()?;//Message type: Topology response (0x0003)
        p.message_type = packet.build_format(reader, Reader::_read16_be, "Message type: ({})")?;
        p.message_id = packet.build_format(reader, Reader::_read16_be, "Message id: {}")?;
        p.flagment = packet.build_format(reader, Reader::_read8, "Fragment id: {}")?;
        // p.ltype = packet.build_lazy(reader, Reader::_read16_be, SSL::ltype)?;
        // p.len = packet.build_lazy(reader, Reader::_read16_be, SSL::len_str)?;
        // p.source = packet.build_lazy(reader, Reader::_read_mac, SSL::source_str).ok();
        // reader._move(2);
        // p.ptype = packet.build_lazy(reader, Reader::_read16_be, SSL::ptype_str)?;
        Ok(())
    }
    
}

pub struct IEEE1905AVisitor;
impl Visitor for IEEE1905AVisitor {
    fn visit(&self, frame: &Frame, reader: &Reader) -> Result<()>{
        let packet = IEEE1905A::create(reader, None)?;
        frame.add_element(ProtocolData::IEEE1905A(packet));
        Ok(())
    }
}
pub fn excute(etype: u16, frame: &Frame, reader: &Reader) -> Result<()>{
    match etype {
        2048 => super::ip4::IP4Visitor.visit(frame, reader),
        34525 => super::ip6::IP6Visitor.visit(frame, reader),
        0x0806 => super::arp::ARPVisitor.visit(frame, reader),
        _ => Ok(()),
    }
}

pub fn _excute(etype: u16, frame: &Frame, reader: &Reader) -> Result<()>{
    match etype {
        0x893a => IEEE1905AVisitor.visit(frame, reader),
        34916 => PPPoESSVisitor.visit(frame, reader),
        _ => excute(etype, frame, reader),
    }
}
