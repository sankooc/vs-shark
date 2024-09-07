
use std::fmt::Display;
//https://www.rfc-editor.org/rfc/rfc1035
use pcap_derive::{Packet, NINFO};
use anyhow::Result;

use crate::common::{IPv4Address, Reader};
use crate::constants::{dns_class_mapper, dns_type_mapper};
use crate::files::{DomainService, Frame, Initer, MultiBlock, PacketContext, PacketOpt, Ref2, Visitor};

use super::ProtocolData;

type Questions = Ref2<MultiBlock<Question>>;
type Answers = Ref2<MultiBlock<RecordResource>>;
type Authority = Ref2<MultiBlock<RecordResource>>;
#[derive(Default, Packet, NINFO)]
pub struct DNS {
    
    transaction_id: u16,
    flag: u16,
    questions: u16,
    answer_rr: u16,
    authority_rr: u16,
    additional_rr: u16,
    opcode: u16,
    is_response: bool,
    questions_ref: Option<Questions>,
    answers_ref: Option<Answers>,
    authorities_ref: Option<Authority>,
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
}


#[derive(Default, Clone, Packet)]
pub struct Question {
    name: String,
    _type: u16,
    class: u16,
}
impl Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(
            format!(
                "{}: type: {}, class: {}",
                self.name,
                dns_type_mapper(self._type),
                dns_class_mapper(self.class)
            )
            .as_str(),
        )?;
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
}

#[derive(Debug, Default)]
pub enum ResourceType {
    A(IPv4Address),
    CNAME(String),
    PTR(String),
    SOA(String),
    #[default]
    EMPTY,
}

impl ResourceType {
    fn content(&self) -> String {
        match self {
            ResourceType::EMPTY => "".into(),
            ResourceType::A(addr) =>  addr.to_string(),
            ResourceType::CNAME(str) =>  str.into(),
            ResourceType::PTR(str) =>  str.into(),
            ResourceType::SOA(str) =>  str.into(),
        }
    }
}

#[derive(Default, Packet)]
pub struct RecordResource {
    name: String,
    _type: u16,
    class: u16,
    ttl: u32,
    len: u16,
    data: ResourceType,
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
    // fn address(&self) -> String {
    //     format!("Address: {}", p.len)
    // }
}
impl Display for RecordResource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(
            format!(
                "{}: type: {}, class: {}",
                self.name,
                dns_type_mapper(self._type),
                dns_class_mapper(self.class)
            )
            .as_str(),
        )?;
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

pub struct DNSVisitor;

impl DNSVisitor {
    fn read_question(reader: &Reader, _: Option<PacketOpt>) -> Result<PacketContext<Question>> {
        let packet: PacketContext<Question> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        p.name = packet.build_lazy(reader, Reader::_read_dns_query, Question::name)?;
        p._type = packet.build_lazy(reader, Reader::_read16_be, Question::_type)?;
        p.class = packet.build_lazy(reader, Reader::_read16_be, Question::class)?;
        drop(p);
        Ok(packet)
    }
    fn read_questions(reader: &Reader, opt: Option<u16>) -> Result<PacketContext<MultiBlock<Question>>> {
        let count = opt.unwrap();
        let packet: PacketContext<MultiBlock<Question>> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        for _ in 0..count {
            let item = packet.build_packet(reader, DNSVisitor::read_question,None, None)?;
            p.push(item);
        }
        drop(p);
        Ok(packet)
    }
    fn read_rr(reader: &Reader, opt: Option<PacketOpt>) -> Result<PacketContext<RecordResource>> {
        let archor = opt.unwrap();
        let packet: PacketContext<RecordResource> = Frame::create_packet();
        let mut p: std::cell::RefMut<RecordResource> = packet.get().borrow_mut();
        let name_ref = reader.read16(true)?;
        let _read = |reader: &Reader| reader.read_dns_compress_string(archor, "", name_ref);
        p.name = packet.build_lazy(reader, _read, RecordResource::name)?;
        p._type = packet.build_lazy(reader, Reader::_read16_be, RecordResource::_type)?;
        p.class = packet.build_lazy(reader, Reader::_read16_be, RecordResource::class)?;
        p.ttl = packet.build_lazy(reader, Reader::_read32_be, RecordResource::ttl)?;
        p.len = packet.build_lazy(reader, Reader::_read16_be, RecordResource::len)?;
        match p._type {
            1 => {
                if p.len == 4 {
                    p.data = ResourceType::A(packet.build_format(reader, Reader::_read_ipv4, "Address: {}")?);
                } else {
                    reader.slice(p.len as usize);
                }
            },
            5 => {
                let _read = |reader: &Reader| reader._read_compress(archor); 
                p.data = ResourceType::CNAME(packet.build_format(reader, _read, "CNAME: {}")?);
            }
            _ => {reader.slice(p.len as usize);},
        };
        // reader.slice(p.len as usize);
        drop(p);
        Ok(packet)
    }
    fn read_rrs(
        frame: &Frame,
        reader: &Reader,
        count: u16,
        archor: usize,
    ) -> Result<PacketContext<MultiBlock<RecordResource>>> {
        let packet: PacketContext<MultiBlock<RecordResource>> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        for _ in 0..count {
            let item: Ref2<RecordResource> = packet.build_packet(reader, DNSVisitor::read_rr, Some(archor), None)?;
            let ctx = frame.ctx.clone();
            ctx.add_dns_record(item.clone());
            p.push(item);
        }
        drop(p);
        Ok(packet)
    }
}

impl Visitor for DNSVisitor {
    fn visit(&self, frame: &Frame, reader: &Reader)  -> Result<()>{
        let packet: PacketContext<DNS> = Frame::create_packet();
        // info!("# index:{}", frame.summary.borrow().index);
        let mut p = packet.get().borrow_mut();
        let _cur = reader.cursor();
        // if frame.summary.borrow().index == 86 {
        //     println!("--")
        // }
        p.transaction_id =
            packet.build_lazy(reader, Reader::_read16_be, DNS::transaction_id)?;
        let flag = packet.build_lazy(reader, Reader::_read16_be, DNS::flag)?;
        let questions = packet.build_lazy(reader, Reader::_read16_be, DNS::questions)?;
        let answer_rr = packet.build_lazy(reader, Reader::_read16_be, DNS::answer_rr)?;
        let authority_rr = packet.build_lazy(reader, Reader::_read16_be, DNS::authority_rr)?;
        let additional_rr = packet.build_lazy(reader, Reader::_read16_be, DNS::additional_rr)?;
        p.is_response = (flag >> 15) > 0;
        p.opcode = (flag >> 11) & 0xf;
        if questions > 0 {
            let qs = packet.build_packet(reader, DNSVisitor::read_questions, Some(questions), Some("Questions".into()))?;
            p.questions_ref = Some(qs);
        }
        if answer_rr > 0 {
            let _read = |reader: &Reader, _: Option<()>| DNSVisitor::read_rrs(frame, reader, answer_rr, _cur);
            let qs: Ref2<Vec<Ref2<RecordResource>>> = packet.build_packet(reader, _read, None, Some("Answers".into()))?;
            for r in qs.as_ref().borrow().iter() {
                frame.ctx.add_dns_record(r.clone());
            }
            p.answers_ref = Some(qs);
        }
        if authority_rr > 0 {
            let _read = |reader: &Reader, _: Option<()>| DNSVisitor::read_rrs(frame,reader, authority_rr, _cur);
            p.authorities_ref = packet.build_packet(reader, _read, None, Some("Authorities".into())).ok();
        }
        // if additional_rr > 0 {
        //     let _read = |reader: &Reader| DNSVisitor::read_rrs(frame,reader, additional_rr, _cur);
        //     p.authorities_ref = Some(packet.build_packet(reader, _read, Some("Additionals".into())));
        // }

        // p.transaction_id = transaction_id;
        p.flag = flag;
        p.questions = questions;
        p.answer_rr = answer_rr;
        p.authority_rr = authority_rr;
        p.additional_rr = additional_rr;
        drop(p);
        frame.add_element(ProtocolData::DNS(packet));
        Ok(())
    }
}
