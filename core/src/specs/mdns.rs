        //https://datatracker.ietf.org/doc/html/rfc1001
        //https://datatracker.ietf.org/doc/html/rfc1002
        //https://blog.csdn.net/CodingMen/article/details/105056639
        use std::fmt::{Display, Formatter};

use pcap_derive::{Packet, Packet2, NINFO};
use anyhow::Result;

use crate::{
    common::{IPPacket, IPv4Address, MacAddress, Reader}, constants::etype_mapper, files::{Frame, Initer, MultiBlock, PacketContext, PacketOpt, Ref2}
};

use super::ProtocolData;

#[derive(Default, Packet2, NINFO)]
pub struct MDNS {
    transaction_id: u16,
    flag: u16,
    questions: u16,
    answer_rr: u16,
    authority_rr: u16,
    additional_rr: u16,
    // questions_ref: Option<Questions>,
    // answers_ref: Option<Answers>,
    // authorities_ref: Option<Authority>,
}

impl std::fmt::Display for MDNS {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_fmt(format_args!("Multicast Domain Name System ({})", "query"))
    }
}
impl MDNS {
    fn _create(
        reader: &Reader,
        packet: &PacketContext<Self>,
        p: &mut std::cell::RefMut<Self>,
        _: Option<PacketOpt>,
    ) -> Result<()> {
        p.transaction_id = packet.build_format(reader, Reader::_read16_be, "Transaction ID: {}")?;
        p.flag = packet.build_format(reader, Reader::_read16_be, "Flags: {}")?;
        p.questions = packet.build_format(reader, Reader::_read16_be, "Questions: {}")?;
        p.answer_rr = packet.build_format(reader, Reader::_read16_be, "Answer RRs: {}")?;
        p.authority_rr = packet.build_format(reader, Reader::_read16_be, "Authority RRs: {}")?;
        p.additional_rr = packet.build_format(reader, Reader::_read16_be, "Additional RRs: {}")?;
        // p.hardware_type = packet.build_lazy(reader, Reader::_read16_be, MDNS::hardware_type_desc)?;
        // p.protocol_type = packet.build_lazy(reader, Reader::_read16_be, MDNS::protocol_type_desc)?;
        // p.hardware_size = packet.build_format(reader, Reader::_read8, "Hardware size: {}")?;
        // p.protocol_size = packet.build_format(reader, Reader::_read8, "Protocol size: {}")?;
        // p.operation = packet.build_lazy(reader, Reader::_read16_be, MDNS::operation_type_desc)?;
        // p.sender_mac = packet.build_format(reader, Reader::_read_mac, "Sender MAC address: ({})").ok();
        // p.sender_ip = packet.build_format(reader, Reader::_read_ipv4, "Sender IP address: {}").ok();
        // p.target_mac = packet.build_format(reader, Reader::_read_mac, "Target MAC address: ({})").ok();
        // p.target_ip = packet.build_format(reader, Reader::_read_ipv4, "Target IP address: {}").ok();
    Ok(())
    }
}

#[derive(Default, Clone, Packet)]
pub struct Question {
    name: String,
    _type: u16,
    class: u16,
}
impl Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("todo")
    }
}
// impl Question {
//     fn name(q: &Question) -> String {
//         format!("Name: {}", q.name)
//     }
//     fn _type(q: &Question) -> String {
//         todo!()
//     }
//     fn class(q: &Question) -> String {
//         todo!()
//     }
// }
type Questions = Ref2<MultiBlock<Question>>;


pub struct MDNSVisitor;

impl crate::files::Visitor for MDNSVisitor {
    fn visit(&self, frame: &Frame, reader: &Reader) -> Result<()> {
        let packet = MDNS::create(reader, None)?;
        // frame.add_element(ProtocolData::MDNS(packet));
        Ok(())
    }
}
