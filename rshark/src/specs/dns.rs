use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

// use log::info;
//https://www.rfc-editor.org/rfc/rfc1035
use pcap_derive::Packet;

use crate::common::{ContainProtocol, Protocol, Reader};
use crate::constants::{dns_class_mapper, dns_type_mapper};
use crate::files::{DomainService, Frame, Initer, MultiBlock, PacketContext, Visitor};

type Questions = Rc<RefCell<MultiBlock<Question>>>;
type Answers = Rc<RefCell<MultiBlock<RecordResource>>>;
type Authority = Rc<RefCell<MultiBlock<RecordResource>>>;
#[derive(Default, Packet)]
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
    fn _info(&self) -> String {
        return self.to_string()
    }
    fn _summary(&self) -> String {
        return self.to_string()
    }
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
impl Initer for Question {
    fn new(_p:Protocol) -> Question {
        Question {
            ..Default::default()
        }
    }
    fn summary(&self) -> String {
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
    name: String,
    _type: u16,
    class: u16,
    ttl: u32,
    len: u16,
    data: String,
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
}
impl Initer for RecordResource {
    fn new(_p:Protocol) -> RecordResource {
        RecordResource {
            data: "".into(),
            ..Default::default()
        }
    }

    fn summary(&self) -> String {
        self.to_string()
    }
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
        format!("{:?}", Protocol::DNS)
    }

    fn content(&self) -> String {
        self.data.clone()
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
    fn read_question(reader: &Reader) -> PacketContext<Question> {
        let packet: PacketContext<Question> = Frame::create_packet(Protocol::UNKNOWN);
        let mut p = packet.get().borrow_mut();
        p.name = packet.read_with_string(reader, Reader::_read_dns_query, Question::name);
        p._type = packet.read_with_string(reader, Reader::_read16_be, Question::_type);
        p.class = packet.read_with_string(reader, Reader::_read16_be, Question::class);
        drop(p);
        packet
    }
    fn read_questions(reader: &Reader, count: u16) -> PacketContext<MultiBlock<Question>> {
        let packet: PacketContext<MultiBlock<Question>> = Frame::create_packet(Protocol::UNKNOWN);
        let mut p = packet.get().borrow_mut();
        for _ in 0..count {
            let item = packet.read_with_field(reader, DNSVisitor::read_question, None);
            p.push(item);
        }
        drop(p);
        packet
    }
    fn read_rr(reader: &Reader, archor: usize) -> PacketContext<RecordResource> {
        let packet: PacketContext<RecordResource> = Frame::create_packet(Protocol::UNKNOWN);
        let mut p: std::cell::RefMut<RecordResource> = packet.get().borrow_mut();
        let name_ref = reader.read16(true);
        let _read = |reader: &Reader| reader.read_dns_compress_string(archor, "", name_ref);
        p.name = packet.read_with_string(reader, _read, RecordResource::name);
        p._type = packet.read_with_string(reader, Reader::_read16_be, RecordResource::_type);
        p.class = packet.read_with_string(reader, Reader::_read16_be, RecordResource::class);
        p.ttl = packet.read_with_string(reader, Reader::_read32_be, RecordResource::ttl);
        p.len = packet.read_with_string(reader, Reader::_read16_be, RecordResource::len);
        reader.slice(p.len as usize);
        drop(p);
        packet
    }
    fn read_rrs(
        frame: &Frame,
        reader: &Reader,
        count: u16,
        archor: usize,
    ) -> PacketContext<MultiBlock<RecordResource>> {
        let packet: PacketContext<MultiBlock<RecordResource>> = Frame::create_packet(Protocol::UNKNOWN);
        let mut p = packet.get().borrow_mut();
        let _resource = |reader: &Reader| DNSVisitor::read_rr(reader, archor);

        for _ in 0..count {
            let item: Rc<RefCell<RecordResource>> = packet.read_with_field(reader, _resource, None);
            let ctx = frame.ctx.clone();
            ctx.add_dns_record(item.clone());
            p.push(item);
        }
        drop(p);
        packet
    }
}

impl Visitor for DNSVisitor {
    fn visit(&self, frame: &Frame, reader: &Reader) {
        let packet: PacketContext<DNS> = Frame::create_packet(Protocol::DNS);
        let mut p = packet.get().borrow_mut();
        let _cur = reader.cursor();
        p.transaction_id =
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
            let qs = packet.read_with_field(reader, read_question, Some("Questions".into()));
            p.questions_ref = Some(qs);
        }
        if answer_rr > 0 {
            let _read = |reader: &Reader| DNSVisitor::read_rrs(frame,reader, answer_rr, _cur);
            let qs: Rc<RefCell<Vec<Rc<RefCell<RecordResource>>>>> = packet.read_with_field(reader, _read, Some("Answers".into()));
            for r in qs.as_ref().borrow().iter() {
                frame.ctx.add_dns_record(r.clone());
            }
            p.answers_ref = Some(qs);
        }
        if authority_rr > 0 {
            let _read = |reader: &Reader| DNSVisitor::read_rrs(frame,reader, authority_rr, _cur);
            p.authorities_ref = Some(packet.read_with_field(reader, _read, Some("Authorities".into())));
        }
        // if additional_rr > 0 {
        //     let _read = |reader: &Reader| DNSVisitor::read_rrs(frame,reader, additional_rr, _cur);
        //     p.authorities_ref = Some(packet.read_with_field(reader, _read, Some("Additionals".into())));
        // }

        // p.transaction_id = transaction_id;
        p.flag = flag;
        p.questions = questions;
        p.answer_rr = answer_rr;
        p.authority_rr = authority_rr;
        p.additional_rr = additional_rr;
        drop(p);
        frame.add_element(Box::new(packet));
    }
}
