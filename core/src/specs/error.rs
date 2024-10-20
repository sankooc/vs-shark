use std::fmt::Formatter;

use anyhow::Result;
use pcap_derive::Packet;

use crate::{
    common::{io::Reader, FIELDSTATUS}, common::base::{Frame, PacketBuilder, PacketContext}
};
use crate::common::io::AReader;

use super::ProtocolData;

#[derive(Default, Packet)]
pub struct Error {
    proto: &'static str,
}

impl std::fmt::Display for Error {
    fn fmt(&self, _fmt: &mut Formatter) -> std::fmt::Result {
        _fmt.write_fmt(format_args!("Parse failed [{}]", self.proto))
    }
}
impl crate::common::base::InfoPacket for Error {
    fn info(&self) -> String {
        self.to_string()
    }

    fn status(&self) -> FIELDSTATUS {
        FIELDSTATUS::ERROR
    }
}
impl Error {
}
pub struct ErrorVisitor;

impl ErrorVisitor {
    pub fn build(reader: &Reader, proto: &'static str) -> ProtocolData {
        let packet: PacketContext<Error> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        p.proto = proto;
        drop(p);
        let start = reader.cursor();
        let left_size = reader.left();
        packet._build(reader, 0, start + left_size, format!("Full Packet: {}", start + left_size));
        if left_size > 0 {
            packet._build(reader, start, left_size, format!("Packet length: {}", left_size));
        }
        super::ProtocolData::ERROR(packet)
    }
    pub fn visit(&self, _: &Frame, reader: &Reader, proto: &'static str) -> Result<(ProtocolData, &'static str)> {
        Ok((ErrorVisitor::build(reader, proto), "none"))
    }
    
    pub fn visit2(&self, reader: &Reader, proto: &'static str) -> Result<ProtocolData> {
        Ok(ErrorVisitor::build(reader, proto))
    }
}
