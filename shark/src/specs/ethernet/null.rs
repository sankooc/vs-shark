use pcap_derive::{Packet2, Visitor3, NINFO};

use crate::common::base::PacketOpt;
use crate::common::io::AReader;
use crate::specs::ProtocolData;
use crate::{
    common::io::Reader,
    common::base::{Frame, PacketBuilder, PacketContext},
};
use std::fmt::Display;
use anyhow::{Ok, Result};

use crate::common::FIELDSTATUS;

#[derive(Default, Packet2, NINFO)]
pub struct NULL {
}
impl Display for NULL {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        _f.write_str("Null/Loopback")
    }
}
impl NULL {
    fn _create(reader: &Reader, packet: &PacketContext<Self>, _p: &mut std::cell::RefMut<Self>, _:Option<PacketOpt>) -> Result<()> {
        let _type = reader.read32(false)?;
        let content = format!("Family: {}", _type);
        packet.build_backward(reader, 4, content);
        Ok(())
    }
    
}
impl NULL {
}

#[derive(Visitor3)]
pub struct NullVisitor;
impl NullVisitor {
    fn visit2(&self, reader: &Reader) -> Result<(ProtocolData, &'static str)>{
        let packet = NULL::create(reader, None)?;
        let mut next = "none";
        let head = reader.read8()?;
        reader._back(1);
        if head == 0x45 {
            next = "ipv4";
        }
        Ok((ProtocolData::NULL(packet), next))
    }
}