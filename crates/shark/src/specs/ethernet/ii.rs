use pcap_derive::{Packet2, Visitor3, NINFO};

use crate::common::base::PacketOpt;
use crate::common::io::AReader;
use crate::common::{Description, MacAddress, MacPacket, PtypePacket, DEF_EMPTY_MAC};
use crate::specs::ProtocolData;
use crate::{
    common::base::PacketContext,
    common::io::Reader,
};
use anyhow::{Ok, Result};
use std::cell::RefCell;
use std::fmt::Display;

use super::get_next_from_type;

#[derive(Default, Packet2, NINFO)]
pub struct Ethernet {
    source_mac: Option<MacAddress>,
    target_mac: Option<MacAddress>,
    len: u16,
    pub ptype: u16,
}
impl Ethernet {
    fn _create<PacketOpt>(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _: Option<PacketOpt>) -> Result<()> {
        p.source_mac = packet.build_lazy(reader, Reader::_read_mac, Some("ethernet.source.mac"), Description::source_mac).ok();
        p.target_mac = packet.build_lazy(reader, Reader::_read_mac, Some("ethernet.target.mac"), Description::target_mac).ok();
        let ptype = reader.read16(true)?;
        if reader.left() == ptype as usize {
            p.len = ptype;
            p.ptype = 1010; // IEEE 802.3
            packet._build(reader, reader.cursor() - 2, 2, None, format!("Length: {}", ptype));
            return Ok(());
        } else {
            p.ptype = ptype;
            packet._build_lazy(reader, reader.cursor() - 2, 2, Some(("ethernet.protocol.type", ptype.to_string().leak())), Description::ptype);
        }
        Ok(())
    }
}

impl Display for Ethernet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let source = self.source_mac.as_ref().unwrap_or(&DEF_EMPTY_MAC).to_string();
        let target = self.target_mac.as_ref().unwrap_or(&DEF_EMPTY_MAC).to_string();
        if self.ptype == 1010 {
            return f.write_str("IEEE 802.3 Ethernet");
        }
        f.write_str(format!("Ethernet II, Src: {}, Dst: {}", source, target).as_str())
    }
}

impl MacPacket for Ethernet {
    fn source_mac(&self) -> String {
        self.source_mac.as_ref().unwrap_or(&DEF_EMPTY_MAC).to_string()
    }

    fn target_mac(&self) -> String {
        self.target_mac.as_ref().unwrap_or(&DEF_EMPTY_MAC).to_string()
    }
}
impl PtypePacket for Ethernet {
    fn protocol_type(&self) -> u16 {
        self.ptype
    }
}
#[derive(Visitor3)]
pub struct EthernetVisitor;

impl EthernetVisitor {
    pub fn visit2(&self, reader: &Reader) -> Result<(ProtocolData, &'static str)> {
        let packet = Ethernet::create(reader, None)?;
        let val: &RefCell<Ethernet> = packet.get();
        let ptype = val.borrow().ptype;
        Ok((ProtocolData::ETHERNET(packet), get_next_from_type(ptype)))
    }
}
