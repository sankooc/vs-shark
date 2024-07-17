
// extern crate init;
// use std::collections::HashMap;

use crate::{common::Reader, files::{PacketContext}};

// pub struct  MacAddress {

// }
pub struct Ethernet<'a> {
    pub packet: PacketContext<'a, Self>,
    source_mac: Option<[u8; 6]>,
    target_mac: Option<[u8; 6]>,
    ptype: u16,
}

// impl Initer for Ethernet<'_> {
//     fn new<Self> (packet: PacketContext) -> Ethernet {
//         Ethernet{packet, source_mac: None, target_mac: None, ptype: 0}
//     }
//     // fn new<'a, Ethernet>(packet: PacketContext<Ethernet>) -> Ethernet<'a> {
//     //     Ethernet{packet, source_mac: None, target_mac: None, ptype: 0}
//     // }
// }
impl Ethernet<'_> {
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
    fn visit(&self, ele: &impl crate::files::Element) {
        let frame = ele.get_frame();
        let mut p = Ethernet{packet:frame.create_packet(), source_mac: None, target_mac: None, ptype: 0};

        // Ethernet{packet: _p, source_mac: None, target_mac: None, ptype: 0}
        // let p: Ethernet = frame.create_packet();
        // let mut packet = &_p;
        // let mut p = Ethernet::new(&packet);
        let packet = &mut p.packet;
        p.source_mac = packet.read(Reader::_read_mac, Some(Ethernet::_source_mac));
        p.target_mac = packet.read(Reader::_read_mac, Some(Ethernet::_target_mac));
        p.ptype = packet.read(Reader::_read16_be, Some(Ethernet::_ptype));
    }
} 