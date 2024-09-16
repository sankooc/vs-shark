use pcap_derive::{Packet2, NINFO};

use crate::common::MacAddress;
use crate::constants::etype_mapper;
use crate::files::{PacketOpt, Visitor};
use crate::specs::ProtocolData;
use crate::{
    common::Reader,
    files::{Frame, Initer, PacketContext},
};
use anyhow::{Ok, Result};
use std::fmt::Display;

use super::get_next_from_type;

#[derive(Default, Packet2, NINFO)]
pub struct IEE80211 {
    version: u8,
    len: u16,
    present: u32,
    // mac_ts: [u8; 8],
    // flag: u8,
    // channel_frequency: u16,
    // channel_flag: u16,
    // antenna_signal: u8,
    // antenna_noise: u8,
    // antenna: u8,
    head: u16,
    duration: u16,
    receiver: Option<MacAddress>,
    transmitter: Option<MacAddress>,
    destination: Option<MacAddress>,
    sequence: u16,
    qos: u16,
    dsap: u8,
    ssap: u8,
    control_field: u8,
    ptype: u16,
    // organization_code: [] //Organization Code: 00:00:00 (Officially Xerox, but
}
impl IEE80211 {
    fn ptype_str(&self) -> String {
        format!("Protocol: {} ({:#06x})", etype_mapper(self.ptype), self.ptype)
    }
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _: Option<PacketOpt>) -> Result<()> {
        p.version = packet.build_format(reader, Reader::_read8, "Header revision: {}")?;
        packet.build_format(reader, Reader::_read8, "Header pad: {}")?;
        p.len = packet.build_format(reader, Reader::_read16_ne, "Header length: {}")?;
        p.present = packet.build_format(reader, Reader::_read32_ne, "Header length: {}")?;
        let _len = p.len - 8;
        packet.build_skip(reader, _len as usize);
        let left = reader.left()?;
        if left < 34 {
            return Ok(());
        }
        p.head = reader.read16(true)?;
        p.duration = reader.read16(true)?;
        p.receiver = Some(packet.build_format(reader, Reader::_read_mac, "Receiver address: {}")?);
        p.transmitter = Some(packet.build_format(reader, Reader::_read_mac, "Transmitter address: {}")?);
        p.destination = Some(packet.build_format(reader, Reader::_read_mac, "Destination address: {}")?);
        let _sq = packet.build_format(reader, Reader::_read16_ne, "Sequence No: {}")?;
        p.sequence = _sq >> 4;
        p.qos = packet.build_format(reader, Reader::_read16_ne, "Qos Control: {}")?;

        p.dsap = reader.read8()?;
        p.ssap = reader.read8()?;
        p.control_field = reader.read8()?;
        reader._move(3);
        p.ptype = packet.build_lazy(reader, Reader::_read16_be, IEE80211::ptype_str)?;
        Ok(())
    }
}

impl Display for IEE80211 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("IEEE 802.11")
    }
}

pub struct IEE80211Visitor;
impl Visitor for IEE80211Visitor {
    fn visit(&self, _f: &Frame, reader: &Reader) -> Result<(ProtocolData, &'static str)>{
        let packet = IEE80211::create(reader, None)?;
        let p = packet.get();
        let ptype = p.borrow().ptype;
        Ok((ProtocolData::IEE80211(packet), get_next_from_type(ptype)))
    }
}
