use std::fmt::Formatter;

use anyhow::Result;
use pcap_derive::Packet;

use crate::{
    common::io::Reader, constants::igmp_type_mapper, files::{Frame, Initer, PacketContext}
};
use crate::common::io::AReader;

use super::ProtocolData;

#[derive(Default, Packet)]
pub struct IGMP {
    _type: u8,
    resp: u8,
    checksum: u16,
}

impl std::fmt::Display for IGMP {
    fn fmt(&self, _fmt: &mut Formatter) -> std::fmt::Result {
        _fmt.write_str("Internet Group Management Protocol")
    }
}
impl crate::files::InfoPacket for IGMP {
    fn info(&self) -> String {
        self._type()
    }

    fn status(&self) -> String {
        "info".into()
    }
}
impl IGMP {
    fn _type(&self) -> String {
        format!("Type: {} ({})", igmp_type_mapper(self._type), self._type)
    }
}
pub struct IGMPVisitor;

impl crate::files::Visitor for IGMPVisitor {
    fn visit(&self, _: &Frame, reader: &Reader) -> Result<(ProtocolData, &'static str)> {
        let packet: PacketContext<IGMP> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        p._type = packet.build_lazy(reader, Reader::_read8, IGMP::_type)?;
        p.resp = packet.build_format(reader, Reader::_read8, "Max Resp Time: {} sec)")?;
        p.checksum = packet.build_format(reader, Reader::_read16_be, "Checksum: {}")?;
        //TODO ADD
        drop(p);
        Ok((super::ProtocolData::IGMP(packet), "none"))
    }
}
