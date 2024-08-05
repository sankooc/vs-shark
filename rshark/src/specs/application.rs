use std::cell::RefCell;
use std::fmt::{Display, Write};
use std::rc::Rc;

// use log::info;

use crate::common::{ContainProtocol, Protocol, Reader};
use crate::constants::{dns_class_mapper, dns_type_mapper};
use crate::files::{Frame, Initer, MultiBlock, PacketContext, Visitor};

type Questions = Rc<RefCell<MultiBlock<Question>>>;
type Answers = Rc<RefCell<MultiBlock<RecordResource>>>;
#[derive(Default)]
pub struct DNS {
    protocol: Protocol,
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
impl ContainProtocol for DNS {
    fn get_protocol(&self) -> Protocol {
        self.protocol.clone()
    }
}
impl Initer<DNS> for DNS {
    fn new() -> DNS {
        DNS {
            protocol: Protocol::DNS,
            ..Default::default()
        }
    }

    fn info(&self) -> String {
        self.to_string().clone()
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
#[derive(Default, Clone)]

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
impl Initer<Question> for Question {
    fn new() -> Question {
        Question {
            ..Default::default()
        }
    }
    fn info(&self) -> String {
        self.to_string()
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

#[derive(Default, Clone)]
pub struct RecordResource {
    owner: String,
    _type: u16,
    class: u16,
    ttl: u32,
    len: u16,
}
impl RecordResource {
    fn owner(p: &RecordResource) -> String {
        format!("Name: {}", p.owner)
    }
    fn _type(p: &RecordResource) -> String {
        format!("Type: {} ({})", dns_type_mapper(p._type), p._type)
    }
    fn class(p: &RecordResource) -> String {
        format!("Class: {} ({:#06x})", dns_type_mapper(p.class), p.class)
    }
    fn ttl(p: &RecordResource) -> String {
        format!("Time to live: {} ({} seconds)", p.ttl, p.ttl)
    }
    fn len(p: &RecordResource) -> String {
        format!("Data length: {}", p.len)
    }
}
impl Initer<RecordResource> for RecordResource {
    fn new() -> RecordResource {
        RecordResource {
            ..Default::default()
        }
    }

    fn info(&self) -> String {
        self.to_string()
    }
}
impl Display for RecordResource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(
            format!(
                "{}: type: {}, class: {}",
                self.owner,
                dns_type_mapper(self._type),
                dns_class_mapper(self.class)
            )
            .as_str(),
        )?;
        Ok(())
    }
}

// pub enum RecordResourceType {
//     NONE,
//     A,
//     CNAME,
//     SOA,
//     RPT,
// }

pub struct DNSVisitor;

impl DNSVisitor {
    fn read_question(reader: &Reader) -> PacketContext<Question> {
        let packet: PacketContext<Question> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        p.name = packet.read_with_string(reader, Reader::_read_dns_query, Question::name);
        p._type = packet.read_with_string(reader, Reader::_read16_be, Question::_type);
        p.class = packet.read_with_string(reader, Reader::_read16_be, Question::class);
        drop(p);
        packet
    }
    fn read_questions(reader: &Reader, count: u16) -> PacketContext<MultiBlock<Question>> {
        let packet: PacketContext<MultiBlock<Question>> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        for _ in 0..count {
            let item = packet.read_with_field(reader, DNSVisitor::read_question, None);
            p.push(item);
        }
        drop(p);
        packet
    }
    fn read_rr(reader: &Reader, archor: usize) -> PacketContext<RecordResource> {
        let packet: PacketContext<RecordResource> = Frame::create_packet();
        let mut p: std::cell::RefMut<RecordResource> = packet.get().borrow_mut();
        let name_ref = reader.read16(true);
        let _read = |reader: &Reader| reader.read_dns_compress_string(archor, "", name_ref);
        p.owner = packet.read_with_string(reader, _read, RecordResource::owner);
        p._type = packet.read_with_string(reader, Reader::_read16_be, RecordResource::_type);
        p.class = packet.read_with_string(reader, Reader::_read16_be, RecordResource::class);
        p.ttl = packet.read_with_string(reader, Reader::_read32_be, RecordResource::ttl);
        p.len = packet.read_with_string(reader, Reader::_read16_be, RecordResource::len);
        reader.slice(p.len as usize);
        drop(p);
        packet
    }
    fn read_rrs(
        reader: &Reader,
        count: u16,
        archor: usize,
    ) -> PacketContext<MultiBlock<RecordResource>> {
        let packet: PacketContext<MultiBlock<RecordResource>> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        let _resource = |reader: &Reader| DNSVisitor::read_rr(reader, archor);
        for _ in 0..count {
            let item = packet.read_with_field(reader, _resource, None);
            p.push(item);
        }
        drop(p);
        packet
    }
}

impl Visitor for DNSVisitor {
    fn visit(&self, frame: &Frame, reader: &Reader) {
        let packet: PacketContext<DNS> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        let _cur = reader.cursor();
        let transaction_id =
            packet.read_with_string(reader, Reader::_read16_be, DNS::transaction_id);
        let flag = packet.read_with_string(reader, Reader::_read16_be, DNS::flag);
        let questions = packet.read_with_string(reader, Reader::_read16_be, DNS::questions);
        let answer_rr = packet.read_with_string(reader, Reader::_read16_be, DNS::answer_rr);
        let authority_rr = packet.read_with_string(reader, Reader::_read16_be, DNS::authority_rr);
        let additional_rr = packet.read_with_string(reader, Reader::_read16_be, DNS::additional_rr);
        p.is_response = (flag >> 15) > 0;
        p.opcode = (flag >> 11) & 0xf;
        if questions > 0 {
            let read_question = |reader: &Reader| DNSVisitor::read_questions(reader, questions);
            p.questions_ref =
                Some(packet.read_with_field(reader, read_question, Some("Questions".into())));
        }
        if answer_rr > 0 {
            let _read = |reader: &Reader| DNSVisitor::read_rrs(reader, answer_rr, _cur);
            p.answers_ref = Some(packet.read_with_field(reader, _read, Some("Answers".into())));
        }
        p.transaction_id = transaction_id;
        p.flag = flag;
        p.questions = questions;
        p.answer_rr = answer_rr;
        p.authority_rr = authority_rr;
        p.additional_rr = additional_rr;
        drop(p);
        frame.add_element(Box::new(packet));
    }
}
