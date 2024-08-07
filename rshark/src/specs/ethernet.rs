use pcap_derive::Packet;

use crate::common::{ContainProtocol, Description, MacAddress, MacPacket, PtypePacket, DEF_EMPTY_MAC};
use crate::constants::{etype_mapper, link_type_mapper, ssl_type_mapper};
use crate::files::Visitor;
use crate::{
    common::{Protocol, Reader},
    files::{Frame, Initer, PacketContext},
};
use std::fmt::Display;

#[derive(Default, Packet)]
pub struct Ethernet {
    protocol: Protocol,
    source_mac: Option<MacAddress>,
    target_mac: Option<MacAddress>,
    len: u16,
    ptype: u16,
}
impl Ethernet {
    fn _info(&self) -> String {
        return self.to_string()
    }
    fn _summary(&self) -> String {
        return self.to_string()
    }
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
        f.write_str(format!("Ethernet II, Src: {}, Dst: {}", source, target).as_str())?;
        Ok(())
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
    fn visit(&self, frame: &Frame, reader: &Reader) {
        let packet: PacketContext<Ethernet> = Frame::create_packet(Protocol::ETHERNET);

        let mut p = packet.get().borrow_mut();
        p.source_mac = packet.read_with_string(reader, Reader::_read_mac, Description::source_mac);
        p.target_mac = packet.read_with_string(reader, Reader::_read_mac, Description::target_mac);
        let ptype = packet.read_with_string(reader, Reader::_read16_be, Description::ptype);
        if reader.left() == ptype as usize {
            p.len = ptype;
            // info!("{}", ptype); // IEEE 802.3
            return;
        }
        p.ptype = ptype;
        drop(p);
        frame.add_element(Box::new(packet));
        excute(ptype, frame, reader);
    }
}
#[derive(Default, Packet)]
pub struct PPPoESS {
    protocol: Protocol,
    version: u8,
    _type: u8,
    code: u8,
    session_id: u16,
    payload: u16,
    ptype: u16,
}
impl Display for PPPoESS {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}

impl PPPoESS{
    fn _info(&self) -> String {
        return self.to_string()
    }
    fn _summary(&self) -> String {
        return self.to_string()
    }
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
    fn visit(&self, frame: &Frame, reader: &Reader) {
        let packet: PacketContext<PPPoESS> = Frame::create_packet(Protocol::PPPoESS);
        let mut p = packet.get().borrow_mut();
        let head = reader.read8();
        p.version = head >> 4;
        p._type = head & 0x0f;
        p.code = packet.read_with_string(reader, Reader::_read8, PPPoESS::code);
        p.session_id = packet.read_with_string(reader, Reader::_read16_be, PPPoESS::session_id);
        p.payload = packet.read_with_string(reader, Reader::_read16_be, PPPoESS::payload);
        p.ptype = packet.read_with_string(reader, Reader::_read16_be, PPPoESS::ptype);
        let code = p.code;
        let ptype = p.ptype;
        drop(p);
        frame.add_element(Box::new(packet));
        if code == 0 {
            match ptype {
                33 => super::network::IP4Visitor.visit(frame, reader),
                _ =>(),
            }
        }
    }
}
#[derive(Default, Packet)]
pub struct SSL {
    protocol: Protocol,
    _type: u16,
    ltype: u16,
    len: u16,
    source: Option<MacAddress>,
    ptype: u16,
}

impl SSL {
    fn _summary(&self) -> String{
        "Linux cooked capture v1".into()
    }
    fn _info(&self) -> String{
        self._summary()
    }
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
    fn visit(&self, frame: &Frame, reader: &Reader) {
        let packet:PacketContext<SSL> = Frame::create_packet(Protocol::SSL);
        let mut p = packet.get().borrow_mut();
        p._type = packet.read_with_string(reader, Reader::_read16_be, SSL::_type);
        p.ltype = packet.read_with_string(reader, Reader::_read16_be, SSL::ltype);
        p.len = packet.read_with_string(reader, Reader::_read16_be, SSL::len_str);
        p.source = packet.read_with_string(reader, Reader::_read_mac, SSL::source_str);
        reader._move(2);
        p.ptype = packet.read_with_string(reader, Reader::_read16_be, SSL::ptype_str);
        let ptype = p.ptype;
        drop(p);
        frame.add_element(Box::new(packet));
        excute(ptype, frame, reader);
    }
}

pub fn excute(etype: u16, frame: &Frame, reader: &Reader) {
    match etype {
        2048 => {
            super::network::IP4Visitor.visit(frame, reader);
        }
        34916 => {
            PPPoESSVisitor.visit(frame, reader);
        }
        _ => (),
    }
}
