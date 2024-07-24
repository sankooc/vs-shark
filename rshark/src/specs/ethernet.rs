
// extern crate init;
// use std::collections::HashMap;

use crate::{common::{Protocol, Reader}, files::{Element, Initer, PacketContext}};

// pub struct  MacAddress {

// }
pub struct Ethernet {
    // pub packet: PacketContext<'a, Self>
    protocol: Protocol,
    source_mac: Option<[u8; 6]>,
    target_mac: Option<[u8; 6]>,
    ptype: u16,
}

impl Initer<Ethernet> for Ethernet {
    fn new() -> Ethernet {
        Ethernet{source_mac: None, target_mac: None, ptype: 0, protocol: Protocol::ETHERNET}
    }
    // fn new<'a, Ethernet>(packet: PacketContext<Ethernet>) -> Ethernet<'a> {
    //     Ethernet{packet, source_mac: None, target_mac: None, ptype: 0}
    // }
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
    pub fn source_mac(&self){

    }
}
pub struct Visitor;

impl crate::files::Visitor for Visitor {
    fn visit(&self, ele: &dyn Element, reader: &mut Reader) {
        let f = ele.get_frame();
        let mut packet:PacketContext<Ethernet> = f.create_packet();
        
        let source: Option<[u8; 6]> = packet.read(reader, Reader::_read_mac, Some(Ethernet::_source_mac));
        let target =  packet.read(reader, Reader::_read_mac, Some(Ethernet::_target_mac));
        let ptype = packet.read(reader, Reader::_read16_be, Some(Ethernet::_ptype));
        
        // packet.val.unwrap();
        let p = &mut packet.val;
        p.source_mac = source;
        p.target_mac = target;
        p.ptype = ptype;
    }
} 