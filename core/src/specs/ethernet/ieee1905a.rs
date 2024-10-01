use pcap_derive::{Packet2, Visitor3, NINFO};

use crate::common::base::PacketOpt;
use crate::specs::ProtocolData;
use crate::{
    common::io::Reader,
    common::base::{Frame, PacketBuilder, PacketContext},
};
use crate::common::io::AReader;
use anyhow::{Ok, Result};
use std::fmt::Display;
use crate::common::FIELDSTATUS;

#[derive(Default, Packet2, NINFO)]
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
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _: Option<PacketOpt>) -> Result<()> {
        p.version = packet.build_format(reader, Reader::_read8, "Message version: {}")?;
        reader.read8()?; //Message type: Topology response (0x0003)
        p.message_type = packet.build_format(reader, Reader::_read16_be, "Message type: ({})")?;
        p.message_id = packet.build_format(reader, Reader::_read16_be, "Message id: {}")?;
        p.flagment = packet.build_format(reader, Reader::_read8, "Fragment id: {}")?;
        Ok(())
    }
}
#[derive(Visitor3)]
pub struct IEEE1905AVisitor;
impl IEEE1905AVisitor {
    fn visit2(&self, reader: &Reader) -> Result<(ProtocolData, &'static str)> {
        let packet = IEEE1905A::create(reader, None)?;
        Ok((ProtocolData::IEEE1905A(packet), "none"))
    }
}
