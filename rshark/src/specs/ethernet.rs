use crate::common::{MacAddress, DEF_EMPTY_MAC};
use crate::files::Visitor;
use crate::{
    common::{Protocol, Reader},
    files::{Field, Frame, Initer, PacketContext},
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
        let source = self.source_mac.as_ref().unwrap_or(&DEF_EMPTY_MAC).to_string();
        let target = self.target_mac.as_ref().unwrap_or(&DEF_EMPTY_MAC).to_string();
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

    fn get_protocol(&self) -> Protocol {
        self.protocol.clone()
    }
    
    fn info(&self) -> String {
        self.to_string().clone()
    }
}
impl Ethernet {
    pub fn _source_mac(start: usize, size: usize, p: &Ethernet) -> Field {
        let addr = p.source_mac.as_ref().unwrap_or(&DEF_EMPTY_MAC).to_string();
        let txt = format!("Destination: {}", addr);
        Field::new(start, size, txt)
    }
    pub fn _target_mac(start: usize, size: usize, p: &Ethernet) -> Field {
        let addr = p.target_mac.as_ref().unwrap_or(&DEF_EMPTY_MAC).to_string();
        let txt = format!("Source: {}", addr);
        Field::new(start, size, txt)
    }
    pub fn _ptype(start: usize, size: usize, p: &Ethernet) -> Field {
        let txt = format!("Type: IPv4 ({:#06x})", p.ptype);
        Field::new(start, size, txt)
    }
}
pub struct EthernetVisitor;

impl Visitor for EthernetVisitor {
    fn visit(&self, frame: &Frame, reader: &Reader) {
        let mut packet: PacketContext<Ethernet> = Frame::create_packet();

        let source: Option<MacAddress> =
            packet.read(reader, Reader::_read_mac, Some(Ethernet::_source_mac));
        let target = packet.read(reader, Reader::_read_mac, Some(Ethernet::_target_mac));
        let ptype = packet.read(reader, Reader::_read16_be, Some(Ethernet::_ptype));
        let p = &mut packet.val;
        p.source_mac = source;
        p.target_mac = target;
        if reader.left() == ptype as usize {
            p.len = ptype;
            // info!("{}", ptype); // IEEE 802.3
            return;
        }
        p.ptype = ptype;
        frame.add_element(Box::new(packet));
        excute(ptype, frame, reader);
    }
}

pub fn excute(etype: u16, frame: &Frame, reader: &Reader) {
    match etype {
        2048 => {
            super::network::IP4Visitor.visit(frame, reader);
        }
        _ => (),
    }
}
