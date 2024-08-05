use crate::common::{ContainProtocol, Description, MacAddress, MacPacket, PtypePacket, DEF_EMPTY_MAC};
use crate::files::Visitor;
use crate::{
    common::{Protocol, Reader},
    files::{Frame, Initer, PacketContext},
};
use std::fmt::Display;
pub struct Ethernet {
    protocol: Protocol,
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
        f.write_str(format!("Ethernet II, Src: {}, Dst: {}", source, target).as_str())?;
        Ok(())
    }
}
impl Initer<Ethernet> for Ethernet {
    fn new() -> Ethernet {
        Ethernet {
            source_mac: None,
            target_mac: None,
            ptype: 0,
            len: 0,
            protocol: Protocol::ETHERNET,
        }
    }

    fn info(&self) -> String {
        self.to_string().clone()
    }
}
impl ContainProtocol for Ethernet {
    fn get_protocol(&self) -> Protocol {
      self.protocol.clone()
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
        let packet: PacketContext<Ethernet> = Frame::create_packet();

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
#[derive(Default)]
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
impl Initer<PPPoESS> for PPPoESS {
    fn new() -> PPPoESS {
        PPPoESS {
            protocol: Protocol::PPPoESS,
            ..Default::default()
        }
    }

    fn info(&self) -> String {
        self.to_string().clone()
    }
}
impl ContainProtocol for PPPoESS {
    fn get_protocol(&self) -> Protocol {
      self.protocol.clone()
    }
}
impl PPPoESS{
    fn code(p:&PPPoESS) -> String{
        format!("Code: Session Data ({:#04x})", p.code)
    }
    fn session_id(p:&PPPoESS) -> String{
        format!("Session ID: {:#06x}", p.session_id)
    }
    fn payload(p:&PPPoESS) -> String{
        format!("Payload Length: {}", p.payload)
    }
    fn ptype(p:&PPPoESS) -> String{
        match p.ptype {
            33 => "Protocol: Internet Protocol version 4 (0x0021)".into(),
            87 => "Protocol: Internet Protocol version 6 (0x0057)".into(),
            _ => format!("Unknown: {}", p.ptype),
        }
    }
}
struct PPPoESSVisitor;
impl Visitor for PPPoESSVisitor {
    fn visit(&self, frame: &Frame, reader: &Reader) {
        let packet: PacketContext<PPPoESS> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        let head = reader.read8();
        p.version = head >> 4;
        p._type = head & 0x0f;
        p.code = packet.read_with_string(reader, Reader::_read8, PPPoESS::code);
        p.session_id = packet.read_with_string(reader, Reader::_read16_be, PPPoESS::session_id);
        p.payload = packet.read_with_string(reader, Reader::_read16_be, PPPoESS::payload);
        p.ptype = packet.read_with_string(reader, Reader::_read16_be, PPPoESS::ptype);
        if p.code == 0 {
            match p.ptype {
                33 => super::network::IP4Visitor.visit(frame, reader),
                _ =>(),
            }
        }
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
