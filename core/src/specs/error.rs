use std::fmt::Formatter;

use anyhow::Result;
use pcap_derive::Packet;

use crate::{
    common::Reader,
    files::{Frame, Initer, PacketContext},
};

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
impl crate::files::InfoPacket for Error {
    fn info(&self) -> String {
        self.to_string()
    }

    fn status(&self) -> String {
        "reset".into()
    }
}
impl Error {
}
pub struct ErrorVisitor;

impl ErrorVisitor {
    pub fn visit(&self, _: &Frame, reader: &Reader, proto: &'static str) -> Result<(ProtocolData, &'static str)> {
        let packet: PacketContext<Error> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        p.proto = proto;
        drop(p);        
        let start = reader.cursor();
        let left_size = reader.left()?;
        if left_size > 0 {
            packet._build(reader, start, left_size, format!("Packet length: {}", left_size));
        }
        Ok((super::ProtocolData::ERROR(packet), "none"))
    }
}
