use pcap_derive::{Packet2, NINFO};

use crate::common::{MacAddress, DEF_EMPTY_MAC};
use crate::constants::{etype_mapper, link_type_mapper, ssl_type_mapper};
use crate::files::{PacketOpt, Visitor};
use crate::specs::ProtocolData;
use crate::{
    common::io::Reader,
    files::{Frame, PacketBuilder, PacketContext},
};
use crate::common::io::AReader;
use std::fmt::Display;
use anyhow::{Ok, Result};

use crate::common::FIELDSTATUS;
use super::get_next_from_type;

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
    fn visit(&self, _f: &Frame, reader: &Reader) -> Result<(ProtocolData, &'static str)>{
        let packet = SSL::create(reader, None)?;
        let p = packet.get();
        let ptype = p.borrow().ptype;
        Ok((ProtocolData::SSL(packet), get_next_from_type(ptype)))
    }
}