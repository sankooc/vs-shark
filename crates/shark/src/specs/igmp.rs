use std::fmt::Formatter;

use anyhow::Result;
use pcap_derive::{Packet, Visitor3};

use crate::{
    common::base::{Frame, PacketContext},
    common::{io::Reader, FIELDSTATUS},
    constants::igmp_type_mapper,
};

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
impl crate::common::base::InfoPacket for IGMP {
    fn info(&self) -> String {
        self._type()
    }

    fn status(&self) -> FIELDSTATUS {
        FIELDSTATUS::INFO
    }
}
impl IGMP {
    fn _type(&self) -> String {
        format!("Type: {} ({})", igmp_type_mapper(self._type), self._type)
    }
}
#[derive(Visitor3)]
pub struct IGMPVisitor;

impl IGMPVisitor {
    fn visit2(&self, reader: &Reader) -> Result<(ProtocolData, &'static str)> {
        let packet: PacketContext<IGMP> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        p._type = packet.build_lazy(reader, Reader::_read8, Some("igmp.type"), IGMP::_type)?;
        p.resp = packet.build_format(reader, Reader::_read8, Some("igmp.resp.time"), "Max Resp Time: {} sec)")?;
        p.checksum = packet.build_format(reader, Reader::_read16_be, None, "Checksum: {}")?;
        //TODO ADD
        drop(p);
        Ok((super::ProtocolData::IGMP(packet), "none"))
    }
}
