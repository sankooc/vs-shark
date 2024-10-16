use std::fmt::Formatter;

use pcap_derive::{Packet2, Visitor3};

use crate::{
    common::{io::{AReader, Reader}, FIELDSTATUS}, common::base::{Frame, PacketBuilder, PacketContext, PacketOpt}
};
use anyhow::Result;

use super::ProtocolData;



#[derive(Default, Packet2)]
pub struct SSDP {
    header: Vec<String>,
}
impl crate::common::base::InfoPacket for SSDP {
    fn info(&self) -> String {
        self.to_string()
    }

    fn status(&self) -> FIELDSTATUS {
        FIELDSTATUS::INFO
    }
}
impl std::fmt::Display for SSDP {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_str("Simple Service Discovery Protocol")
    }
}

impl SSDP {
    
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _: Option<PacketOpt>) -> Result<()> {
        loop {
            if reader.left() == 0 {
                break;
            }
            if reader.enter_flag(0) {
                break;
            }
            let header = packet.build_format(reader, Reader::_read_enter, "{}")?;
            p.header.push(header);
        }
        let dlen = reader.left();
        packet._build(reader, reader.cursor(), dlen, format!("File Data: {} bytes",dlen));
        Ok(())
    }
}
#[derive(Visitor3)]
pub struct SSDPVisitor;

impl SSDPVisitor {
    pub fn check(reader: &Reader) -> bool {
        let method = reader._read_space(10);
        match method {
            Some(_method) => {
                return match _method.as_str() {
                    "M-SEARCH" => true,
                    _ => false,
                }
            }
            _ => false,
        }
    }
}

impl SSDPVisitor {
    fn visit2(&self, reader: &Reader) -> Result<(ProtocolData, &'static str)> {
        let packet = SSDP::create(reader, None)?;
        Ok((super::ProtocolData::SSDP(packet), "none"))
    }
}
