use std::fmt::Formatter;

use super::ProtocolData;
use crate::common::io::Reader;
use crate::{
    common::base::{Frame, PacketBuilder, PacketContext, PacketOpt},
    common::FIELDSTATUS,
};
use anyhow::Result;
use pcap_derive::{Packet2, Visitor3};


// enum LineType{
//     Request,
// }

// #[derive(Default, Packet2)]
// struct Line;
// impl std::fmt::Display for Line {
//     fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
//         fmt.write_fmt(format_args!("Session Initiation Protocol"))
//     }
// }
// impl Line {
//     fn _create(_reader: &Reader, _packet: &PacketContext<Self>, _p: &mut std::cell::RefMut<Self>, _: Option<PacketOpt>) -> Result<()> {
//         // reader.
//         Ok(())
//     }
// }



#[derive(Default, Packet2)]
pub struct SIP {
}


impl std::fmt::Display for SIP {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_fmt(format_args!("Session Initiation Protocol"))
    }
}

impl crate::common::base::InfoPacket for SIP {
    fn info(&self) -> String {
        "".into()
    }

    fn status(&self) -> FIELDSTATUS {
        FIELDSTATUS::INFO
    }
}

impl SIP {
    fn _create(_reader: &Reader, _packet: &PacketContext<Self>, _p: &mut std::cell::RefMut<Self>, _: Option<PacketOpt>) -> Result<()> {
        
        Ok(())
    }
}

#[derive(Visitor3)]
pub struct SIPVisitor;

impl SIPVisitor {
    pub fn visit2(&self, reader: &Reader) -> Result<(ProtocolData, &'static str)> {
        let packet = SIP::create(reader, None)?;
        Ok((ProtocolData::SIP(packet), "none"))
    }
}
