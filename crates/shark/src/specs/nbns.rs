//https://datatracker.ietf.org/doc/html/rfc1001
//https://datatracker.ietf.org/doc/html/rfc1002
//https://blog.csdn.net/CodingMen/article/details/105056639
use std::fmt::{Display, Formatter};

use anyhow::Result;
use pcap_derive::{Packet2, Visitor3, NINFO};

use crate::{
    common::base::{PacketContext, PacketOpt},
    common::{io::Reader, MultiBlock},
    constants::{dns_class_mapper, nbns_type_mapper},
};

use super::ProtocolData;

#[derive(Default, Packet2, NINFO)]
pub struct NBNS {
    transaction_id: u16,
    flag: u16,
    questions: u16,
    answer_rr: u16,
    authority_rr: u16,
    additional_rr: u16,
    // questions_ref: MultiBlock<Questions>,
    // answers_ref: Option<Answers>,
    // authorities_ref: Option<Authority>,
}

impl std::fmt::Display for NBNS {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_str("NetBIOS Name Service")
    }
}
impl NBNS {
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _: Option<PacketOpt>) -> Result<()> {
        p.transaction_id = packet.build_format(reader, Reader::_read16_be, Some("nbns.transaction.id"), "Transaction ID: {}")?;
        p.flag = packet.build_format(reader, Reader::_read16_be, None, "Flags: {}")?;
        p.questions = packet.build_format(reader, Reader::_read16_be, Some("nbns.question.count"), "Questions: {}")?;
        p.answer_rr = packet.build_format(reader, Reader::_read16_be, Some("nbns.answer.count"), "Answer RRs: {}")?;
        p.authority_rr = packet.build_format(reader, Reader::_read16_be, Some("nbns.authority.count"), "Authority RRs: {}")?;
        p.additional_rr = packet.build_format(reader, Reader::_read16_be, Some("nbns.additional.count"), "Additional RRs: {}")?;
        Ok(())
    }
}

#[derive(Default, Packet2)]
pub struct Question {
    name: String,
    _type: u16,
    class: u16,
}
impl Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(format!("{}: type: {}, class: {}", self.name, nbns_type_mapper(self._type), dns_class_mapper(self.class)).as_str())?;
        Ok(())
    }
}

impl Question {
    fn name(q: &Question) -> String {
        format!("Name: {}", q.name)
    }
    fn _type(q: &Question) -> String {
        format!("Type: {} ({})", nbns_type_mapper(q._type), q._type)
    }
    fn class(q: &Question) -> String {
        format!("Class: {} ({:#06x})", dns_class_mapper(q.class), q.class)
    }
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _: Option<PacketOpt>) -> Result<()> {
        p.name = packet.build_lazy(reader, Reader::_read_netbios_string, Some("nbns.question.name"), Question::name)?;
        p._type = packet.build_lazy(reader, Reader::_read16_be, Some("nbns.question.type"), Question::_type)?;
        p.class = packet.build_lazy(reader, Reader::_read16_be, Some("nbns.question.class"), Question::class)?;
        Ok(())
    }
}

#[derive(Default, Packet2)]
pub struct Questions {
    items: MultiBlock<Question>,
}

impl Display for Questions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("Questions")
    }
}
impl Questions {
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _count: Option<PacketOpt>) -> Result<()> {
        let count = _count.unwrap();
        for _ in 0..count {
            let item = packet.build_packet(reader, Question::create, None, None)?;
            p.items.push(item);
        }
        Ok(())
    }
}
#[derive(Visitor3)]
pub struct NBNSVisitor;

impl NBNSVisitor {
    fn visit2(&self, reader: &Reader) -> Result<(ProtocolData, &'static str)> {
        let packet = NBNS::create(reader, None)?;
        Ok((ProtocolData::NBNS(packet), "none"))
    }
}
