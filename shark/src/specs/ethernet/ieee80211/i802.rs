use pcap_derive::{Packet2, Visitor3, NINFO};

use crate::common::base::BitFlag;
use crate::common::base::BitType;
use crate::common::base::FlagData;
use crate::common::base::PacketOpt;
use crate::common::io::AReader;
use crate::common::MacAddress;
use crate::common::FIELDSTATUS;
use crate::constants::etype_mapper;
use crate::constants::ieee802_subtype_mapper;
use crate::specs::ethernet::get_next_from_type;
use crate::specs::ProtocolData;
use crate::{
    common::base::{Frame, PacketBuilder, PacketContext},
    common::io::Reader,
};
use anyhow::{Ok, Result};
use std::fmt::Display;

pub struct Flag;

impl FlagData<u8> for Flag {
    fn bits(inx: usize) -> Option<(u8, BitType<u8>)> {
        match inx {
            0 => Some((0x03, BitType::ONEoF(vec![(0x00, "DS status: From DS: 0"), (0x01, "DS status: To DS: 1")]))),
            1 => Some((0x04, BitType::ABSENT("More Fragments: More fragments follow", "More Fragments: This is the last fragment"))),
            3 => Some((0x08, BitType::ABSENT("Retry: Frame is being retransmitted", "Retry: Frame is not being retransmitted"))),
            4 => Some((0x10, BitType::ABSENT("PWR MGT: STA will go to sleep", "PWR MGT: STA will stay up"))),
            5 => Some((0x20, BitType::ABSENT("More Data: Data is buffered for STA at AP", "More Data: No data buffered"))),
            6 => Some((0x40, BitType::ABSENT("Protected flag: Data is protected", "Protected flag: Data is not protected"))),
            7 => Some((0x80, BitType::ABSENT("+HTC/Order flag: strictly ordered", "+HTC/Order flag: Not strictly ordered"))),
            _ => None,
        }
    }

    fn summary(title: &mut String, value: u8) {
        title.push_str(format!("Frame Control Field: {:#06x}", value).as_str());
    }

    fn summary_ext(_: &mut String, _: &str, _: bool) {}
}

#[derive(Default, Packet2, NINFO)]
pub struct IEE80211 {
    // head: u16,
    version: u8,
    _type: u8,
    sub_type: u8,
    flag: u8,
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
    fn flag(&self) -> Option<PacketContext<BitFlag<u8>>> {
        BitFlag::make::<Flag>(self.flag)
    }
    fn ptype_str(&self) -> String {
        format!("Protocol: {} ({:#06x})", etype_mapper(self.ptype), self.ptype)
    }
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _: Option<PacketOpt>) -> Result<()> {
        let head = reader.read8()?;
        p.version = head & 0x03;
        p._type = (head >> 2) & 0x03;
        p.sub_type = head >> 4;
        match p._type {
            1 => {
                p.sub_type += 16;
            }
            2 => {
                p.sub_type += 32;
            }
            _ => {}
        }
        let _type_desc = match p._type {
            0 => "Management Frame (0)",
            1 => "Control Frame (1)",
            2 => "Data Frame (2)",
            _ => "Extension",
        };
        packet.build_backward(reader, 1, format!("Version: {}", p.version));
        packet.build_backward(reader, 1, format!("Type: {}", _type_desc));
        packet.build_backward(reader, 1, format!("Subtype: {}", ieee802_subtype_mapper(p.sub_type)));

        p.flag = packet.build_packet_lazy(reader, Reader::_read8, None, IEE80211::flag)?;
        // p.head = reader.read16(true)?;
        p.duration = reader.read16(true)?;
        p.receiver = Some(packet.build_format(reader, Reader::_read_mac, Some("80211.receiver.address"), "Receiver address: {}")?);
        p.transmitter = Some(packet.build_format(reader, Reader::_read_mac, Some("80211.transmitter.address"), "Transmitter address: {}")?);
        p.destination = Some(packet.build_format(reader, Reader::_read_mac, Some("80211.destination.address"), "Destination address: {}")?);
        let _sq = packet.build_format(reader, Reader::_read16_ne, Some("80211.sequence.no"), "Sequence No: {}")?;
        p.sequence = _sq >> 4;
        p.qos = packet.build_format(reader, Reader::_read16_ne, Some("80211.qos.control"), "Qos Control: {}")?;

        p.dsap = reader.read8()?;
        p.ssap = reader.read8()?;
        p.control_field = reader.read8()?;
        reader._move(3);
        p.ptype = packet.build_lazy(reader, Reader::_read16_be, Some("80211.prorocol.type"), IEE80211::ptype_str)?;
        Ok(())
    }
}

impl Display for IEE80211 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("IEEE 802.11")
    }
}
#[derive(Visitor3)]
pub struct IEE80211Visitor;
impl IEE80211Visitor {
    fn visit2(&self, reader: &Reader) -> Result<(ProtocolData, &'static str)> {
        let packet = IEE80211::create(reader, None)?;
        let p = packet.get();
        let ptype = p.borrow().ptype;
        Ok((ProtocolData::IEE80211(packet), get_next_from_type(ptype)))
    }
}
