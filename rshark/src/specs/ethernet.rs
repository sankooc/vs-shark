
use std::fmt::Display;
use crate::{common::{Protocol, Reader}, files::{Element, Frame, Initer, PacketContext}};
use crate::files::Visitor;
pub struct Ethernet {
    protocol: Protocol,
    source_mac: Option<[u8; 6]>,
    target_mac: Option<[u8; 6]>,
    len: u16,
    ptype: u16,
}
impl Display for Ethernet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("Ethernet II, Src: f4:2a:7d:13:4a:9e, Dst: 44:56:e2:60:2b:f8")?;
        Ok(())
    }
}
impl Initer<Ethernet> for Ethernet {
    fn new() -> Ethernet {
        Ethernet{source_mac: None, target_mac: None, ptype: 0, len: 0, protocol: Protocol::ETHERNET}
    }
    
    fn get_protocol(&self) -> Protocol {
        self.protocol.clone()
    }
}
impl Ethernet {
    pub fn _source_mac(p: &Ethernet) -> String{
        String::from("sourc-asd")
    }
    pub fn _target_mac(p: &Ethernet) -> String{
        String::from("sourc-asd")
    }
    pub fn _ptype(p: &Ethernet) -> String{
        String::from("sourc-asd")
    }
}
pub struct EthernetVisitor;

impl Visitor for EthernetVisitor {
    fn visit(&self, frame: &Frame, reader: &Reader){
        let mut packet:PacketContext<Ethernet> = Frame::create_packet();
        
        let source: Option<[u8; 6]> = packet.read(reader, Reader::_read_mac, Some(Ethernet::_source_mac));
        let target =  packet.read(reader, Reader::_read_mac, Some(Ethernet::_target_mac));
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

pub fn excute(etype: u16, frame: &Frame, reader: &Reader){
    match etype {
        2048 => {
            let visitor = super::network::IP4Visitor{};
            visitor.visit(frame, reader);
        },
        _ => (),
    }
}