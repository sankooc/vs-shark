use std::fmt::Display;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
//https://www.rfc-editor.org/rfc/rfc1035
use anyhow::Result;
use pcap_derive::Visitor3;
use pcap_derive::{Packet, Packet2, NINFO};

use crate::common::base::{DomainService, Frame, PacketBuilder, PacketContext, PacketOpt};
use crate::common::io::AReader;
use crate::common::io::Reader;
use crate::common::MultiBlock;
use crate::common::Ref2;
use crate::common::FIELDSTATUS;
use crate::constants::{dns_class_mapper, dns_type_mapper};

use super::ProtocolData;

type Answers = Ref2<MultiBlock<RecordResource>>;
type Authority = Ref2<MultiBlock<RecordResource>>;
#[derive(Default, Packet2, NINFO)]
pub struct DNS {
    transaction_id: u16,
    flag: u16,
    pub questions: u16,
    pub answer_rr: u16,
    pub authority_rr: u16,
    pub additional_rr: u16,
    opcode: u16,
    is_response: bool,
    questions_ref: MultiBlock<Questions>,
    pub answers_ref: Option<Answers>,
    pub authorities_ref: Option<Authority>,
}
impl Display for DNS {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_response {
            f.write_str("Domain Name System (response)")?;
        } else {
            f.write_str("Domain Name System (query)")?;
        }
        Ok(())
    }
}

impl DNS {
    fn transaction_id(packet: &DNS) -> String {
        format!("Transaction ID: {:#06x}", packet.transaction_id)
    }
    fn flag(packet: &DNS) -> String {
        format!("Flags: {:#06x}", packet.flag)
    }
    fn questions(packet: &DNS) -> String {
        format!("Questions: {}", packet.questions)
    }
    fn answer_rr(packet: &DNS) -> String {
        format!("Answer RRs: {}", packet.answer_rr)
    }
    fn authority_rr(packet: &DNS) -> String {
        format!("Authority RRs: {}", packet.authority_rr)
    }
    fn additional_rr(packet: &DNS) -> String {
        format!("Additional RRs: {}", packet.additional_rr)
    }
    fn _create<PacketOpt>(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _: Option<PacketOpt>) -> Result<()> {
        let _cur = reader.cursor();
        p.transaction_id = packet.build_lazy(reader, Reader::_read16_be, Some("dns.transaction.id"), DNS::transaction_id)?;
        let flag = packet.build_lazy(reader, Reader::_read16_be, None,DNS::flag)?;
        let questions = packet.build_lazy(reader, Reader::_read16_be, Some("dns.question.count"), DNS::questions)?;
        let answer_rr = packet.build_lazy(reader, Reader::_read16_be, Some("dns.answer.count"),DNS::answer_rr)?;
        let authority_rr = packet.build_lazy(reader, Reader::_read16_be, Some("dns.authority.count"),DNS::authority_rr)?;
        let additional_rr = packet.build_lazy(reader, Reader::_read16_be, Some("dns.additional.count"),DNS::additional_rr)?;
        p.is_response = (flag >> 15) > 0;
        p.opcode = (flag >> 11) & 0xf;
        let qs = packet.build_packet(reader, Questions::create, Some((questions, _cur)), None)?;
        p.questions_ref.push(qs);
        if answer_rr > 0 {
            let _read = |reader: &Reader, _: Option<()>| DNSVisitor::read_rrs(reader, answer_rr, _cur);
            let qs: Ref2<Vec<Ref2<RecordResource>>> = packet.build_packet(reader, _read, None, Some("Answers".into()))?;
            p.answers_ref = Some(qs);
        }
        if authority_rr > 0 {
            let _read = |reader: &Reader, _: Option<()>| DNSVisitor::read_rrs(reader, authority_rr, _cur);
            p.authorities_ref = packet.build_packet(reader, _read, None, Some("Authorities".into())).ok();
        }
        p.flag = flag;
        p.questions = questions;
        p.answer_rr = answer_rr;
        p.authority_rr = authority_rr;
        p.additional_rr = additional_rr;
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
        f.write_str(format!("{}: type: {}, class: {}", self.name, dns_type_mapper(self._type), dns_class_mapper(self.class)).as_str())?;
        Ok(())
    }
}

impl Question {
    fn name(q: &Question) -> String {
        format!("Name: {}", q.name)
    }
    fn _type(q: &Question) -> String {
        format!("Type: {} ({})", dns_type_mapper(q._type), q._type)
    }
    fn class(q: &Question) -> String {
        format!("Class: {} ({:#06x})", dns_class_mapper(q.class), q.class)
    }
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, opt: Option<usize>) -> Result<()> {
        let archor = opt.unwrap();

        let _read = |reader: &Reader| {
            return reader.read_dns_compress_string(archor, "");
        };
        p.name = packet.build_lazy(reader, _read, Some("dns.question.name"),Question::name)?;
        if p.name == "" {
            p.name = "<Root>".into();
        }
        p._type = packet.build_lazy(reader, Reader::_read16_be, Some("dns.question.type"), Question::_type)?;
        p.class = packet.build_lazy(reader, Reader::_read16_be, Some("dns.question.class"), Question::class)?;
        Ok(())
    }
}

#[derive(Default, Packet)]
pub struct Questions {
    items: MultiBlock<Question>,
}

impl Display for Questions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("Questions")
    }
}
impl Questions {
    fn create(reader: &Reader, opt: Option<(u16, usize)>) -> Result<PacketContext<Self>> {
        let packet: PacketContext<Self> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        let rs = Self::_create(reader, &packet, &mut p, opt);
        drop(p);
        rs?;
        Ok(packet)
    }
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _count: Option<(u16, usize)>) -> Result<()> {
        let (count, archor) = _count.unwrap();
        for _ in 0..count {
            let item = packet.build_packet(reader, Question::create, Some(archor), None)?;
            p.items.push(item);
        }
        Ok(())
    }
}

#[derive(Default)]
pub enum ResourceType {
    A(Ipv4Addr),
    AAAA(Ipv6Addr),
    CNAME(String),
    PTR(String),
    SOA(String),
    SRV(u16, u16, u16, String),
    #[default]
    EMPTY,
}

impl ResourceType {
    fn content(&self) -> String {
        match self {
            ResourceType::EMPTY => "".into(),
            ResourceType::A(addr) => addr.to_string(),
            ResourceType::CNAME(str) => str.into(),
            ResourceType::PTR(str) => str.into(),
            ResourceType::SOA(str) => str.into(),
            ResourceType::AAAA(str) => str.to_string(),
            ResourceType::SRV(_, _, port, target) => format!("{}:{}", target, port),
        }
    }
}

#[derive(Default, Packet2)]
pub struct RecordResources {
    items: MultiBlock<RecordResource>,
}

impl Display for RecordResources {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("RecordResources")
    }
}
impl RecordResources {
    fn _create(reader: &Reader, packet: &PacketContext<Self>, p: &mut std::cell::RefMut<Self>, _count: Option<usize>) -> Result<()> {
        let count = _count.unwrap();
        for _ in 0..count {
            let item = packet.build_packet(reader, RecordResource::create, None, None)?;
            p.items.push(item);
        }
        Ok(())
    }
}
#[derive(Default, Packet2)]
pub struct RecordResource {
    name: String,
    _type: u16,
    class: u16,
    ttl: u32,
    len: u16,
    pub data: ResourceType,
}
impl RecordResource {
    fn name(p: &RecordResource) -> String {
        format!("Name: {}", p.name)
    }
    fn _type(p: &RecordResource) -> String {
        format!("Type: {} ({})", p._type(), p._type)
    }
    fn class(p: &RecordResource) -> String {
        format!("Class: {} ({:#06x})", p.class(), p.class)
    }
    fn ttl(p: &RecordResource) -> String {
        format!("Time to live: {} ({} seconds)", p.ttl(), p.ttl)
    }
    fn len(p: &RecordResource) -> String {
        format!("Data length: {}", p.len)
    }
    fn _create(_: &Reader, _: &PacketContext<Self>, _: &mut std::cell::RefMut<Self>, _count: Option<usize>) -> Result<()> {
        // let count = _count.unwrap();
        // for _ in 0..count {
        //    let item = packet.build_packet(reader, Question::create, None, None)?;
        //    p.items.push(item);
        // }
        Ok(())
    }
}
impl Display for RecordResource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(format!("{}: type: {}, class: {}", self.name, dns_type_mapper(self._type), dns_class_mapper(self.class)).as_str())?;
        Ok(())
    }
}

impl DomainService for RecordResource {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn _type(&self) -> String {
        dns_type_mapper(self._type)
    }

    fn proto(&self) -> String {
        "dns".into()
    }

    fn content(&self) -> String {
        self.data.content()
    }

    fn ttl(&self) -> u32 {
        self.ttl
    }

    fn class(&self) -> String {
        dns_class_mapper(self.class)
    }
}

#[derive(Visitor3)]
pub struct DNSVisitor;

impl DNSVisitor {
    fn read_rr(reader: &Reader, opt: Option<PacketOpt>) -> Result<PacketContext<RecordResource>> {
        let archor = opt.unwrap();
        let packet: PacketContext<RecordResource> = Frame::create_packet();
        let mut p: std::cell::RefMut<RecordResource> = packet.get().borrow_mut();
        let _read = |reader: &Reader| {
            // let name_ref = reader.read16(true)?;
            return reader.read_dns_compress_string(archor, "");
        };
        p.name = packet.build_lazy(reader, _read, Some("dns.record.resource.name"), RecordResource::name)?;
        p._type = packet.build_lazy(reader, Reader::_read16_be, Some("dns.record.resource.type"),RecordResource::_type)?;
        p.class = packet.build_lazy(reader, Reader::_read16_be, Some("dns.record.resource.class"),RecordResource::class)? & 0x00ff;
        p.ttl = packet.build_lazy(reader, Reader::_read32_be, Some("dns.record.resource.ttl"),RecordResource::ttl)?;
        p.len = packet.build_lazy(reader, Reader::_read16_be, Some("dns.record.resource.len"),RecordResource::len)?;
        let _finish = p.len as usize + reader.cursor();
        match p._type {
            1 => {
                if p.len == 4 {
                    p.data = ResourceType::A(packet.build_format(reader, Reader::_read_ipv4, Some("dns.resource.address"), "Address: {}")?);
                } else {
                    reader.slice(p.len as usize);
                }
            }
            5 => {
                let _read = |reader: &Reader| reader._read_compress(archor);
                p.data = ResourceType::CNAME(packet.build_format(reader, _read, Some("dns.resource.cname"), "CNAME: {}")?);
            }
            28 => {
                if p.len == 16 {
                    p.data = ResourceType::AAAA(packet.build_format(reader, Reader::_read_ipv6, Some("dns.resource.address"), "Address: {}")?);
                } else {
                    reader.slice(p.len as usize);
                }
            }
            33 => {
                let priority = packet.build_format(reader, Reader::_read16_be, Some("dns.resource.priority"), "Priority: {}")?;
                let weight = packet.build_format(reader, Reader::_read16_be, Some("dns.resource.weight"), "Weight: {}")?;
                let port = packet.build_format(reader, Reader::_read16_be, Some("dns.resource.port"), "Port: {}")?;
                let _read = |reader: &Reader| reader._read_compress(archor);
                let target = packet.build_format(reader, _read, Some("dns.resource.target"), "Target: {}")?;
                p.data = ResourceType::SRV(priority, weight, port, target);
                reader._set(_finish);
            }
            _ => {
                reader.slice(p.len as usize);
            }
        };
        // reader.slice(p.len as usize);
        drop(p);
        Ok(packet)
    }
    fn read_rrs(reader: &Reader, count: u16, archor: usize) -> Result<PacketContext<MultiBlock<RecordResource>>> {
        let packet: PacketContext<MultiBlock<RecordResource>> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        for _ in 0..count {
            let item: Ref2<RecordResource> = packet.build_packet(reader, DNSVisitor::read_rr, Some(archor), None)?;
            p.push(item);
        }
        drop(p);
        Ok(packet)
    }
}

impl DNSVisitor {
    pub fn visit2(&self, reader: &Reader) -> Result<(ProtocolData, &'static str)> {
        let packet = DNS::create(reader, None)?;
        Ok((ProtocolData::DNS(packet), "none"))
    }
}
#[derive(Visitor3)]
pub struct MDNSVisitor;
impl MDNSVisitor {
    fn visit2(&self, reader: &Reader) -> Result<(ProtocolData, &'static str)> {
        let packet = DNS::create(reader, None)?;
        Ok((ProtocolData::DNS(packet), "none"))
    }
}
