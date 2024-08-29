pub mod pcap;
pub mod pcapng;

use crate::{common::IPPacket, constants::link_type_mapper, nshark::DNSRecord, specs::{dns::RecordResource, tcp::TCP, ProtocolData}};
use std::{cell::{Cell, Ref, RefCell, RefMut}, rc::Rc, time::{Duration, UNIX_EPOCH}
};
use chrono::{DateTime, Utc};
use enum_dispatch::enum_dispatch;
use log::error;

use anyhow::Result;
// pub mod pcapng;
use crate::common::{FileInfo, FileType, Reader};

#[derive(Default, Clone)]
pub struct Field {
    pub start: usize,
    pub size: usize,
    pub summary: String,
    pub data: Rc<Vec<u8>>,
    pub children: RefCell<Vec<Field>>,
}
impl Field {
    pub fn new(start: usize, size: usize, data: Rc<Vec<u8>>,summary: String) -> Field {
        Field {
            start,
            size,
            data,
            summary,
            children: RefCell::new(Vec::new()),
        }
    }
    pub fn new2(summary: String, data: Rc<Vec<u8>>,vs: Vec<Field>) -> Field {
        Field {
            start: 0,
            size: 0,
            data,
            summary,
            children: RefCell::new(vs),
        }
    }
    pub fn new3(summary: String) -> Field {
        Field {
            start: 0,
            size: 0,
            data: Rc::new(Vec::new()),
            summary,
            children: RefCell::new(Vec::new()),
        }
    }
}

impl Field {
    pub fn summary(&self) -> String {
        self.summary.clone()
    }
    
    pub fn children(&self) -> Ref<Vec<Field>> {
        let ch: Ref<Vec<Field>> = self.children.borrow();
        ch
        // let mut children = Vec::new();
        // for c in ch.iter() {
        //     children.push(c.clone());
        // }
        // children
    }
}
pub fn date_str(ts: u64) -> String {
    let d = UNIX_EPOCH + Duration::from_micros(ts);
    // let dt: DateTime<Utc> = d.clone().into();
    let datetime = DateTime::<Utc>::from(d);
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

#[enum_dispatch(ProtocolData)]
pub trait Element {
    fn summary(&self) -> String;
    fn get_fields(&self) -> Vec<Field>;
    // fn add_next(&mut self, ele: Box<dyn Element>);
    // fn get_protocol(&self) -> Protocol;
    fn info(&self) -> String;
}

pub trait Visitor {
    fn visit(&self, frame: &Frame, reader: &Reader) -> Result<()>;
    // fn from() -> Vec<Protocol>;
}

pub trait FieldBuilder<T> {
    fn build(&self, t: &T) -> Field;
    fn data(&self) -> Rc<Vec<u8>>;
}

pub type MultiBlock<T> = Vec<Rc<RefCell<T>>>;

impl<T> Initer for MultiBlock<T> {
    fn new() -> MultiBlock<T> {
        Vec::new()
    }

    fn summary(&self) -> String {
        String::from("")
    }
}

pub struct PacketContext<T>
{
    val: Rc<RefCell<T>>,
    fields: RefCell<Vec<Box<dyn FieldBuilder<T>>>>,
}

impl<T> PacketContext<T>
{
    pub fn _clone_obj(&self) -> Rc<RefCell<T>> {
        self.val.clone()
    }
    pub fn get(&self) -> &RefCell<T> {
        &self.val
    }
    fn get_fields(&self) -> Vec<Field> {
        let t: &T = &self.get().borrow();
        let mut rs: Vec<Field> = Vec::new();
        for pos in self.fields.borrow().iter() {
            rs.push(pos.build(t));
        }
        rs
    }
}

impl<T> Element for PacketContext<T>
where
    T: Initer + InfoPacket,
{
    fn summary(&self) -> String {
        self.get().borrow().summary()
    }
    fn get_fields(&self) -> Vec<Field> {
        self.get_fields()
    }
    // fn get_protocol(&self) -> Protocol {
    //     self.get().borrow().get_protocol()
    // }
    fn info(&self) -> String {
        self.get().borrow().info()
    }
}
impl<T> PacketContext<T>
where
    T: Initer + 'static,
{
    pub fn read_with_string<K>(
        &self,
        reader: &Reader,
        opt: impl Fn(&Reader) -> Result<K>,
        render: fn(&T) -> String,
    ) -> Result<K> {
        let start = reader.cursor();
        let val: K = opt(reader)?;
        let end = reader.cursor();
        let size = end - start;
        self.fields.borrow_mut().push(Box::new(StringPosition {
            start,
            size,
            data: reader.get_raw(),
            render,
        }));
        Ok(val)
    }
    pub fn append_string(&self, content: String, data: Rc<Vec<u8>>) {
        self.fields.borrow_mut().push(Box::new(TXTPosition {
            start: 0,
            size: 0,
            data,
            content,
        }));
    }
    pub fn _read_with_concrete_string<K>(
        &self,
        reader: &Reader,
        opt: impl Fn(&Reader) -> K,
        content: String,
    ) -> K {
        let start = reader.cursor();
        let val: K = opt(reader);
        let end = reader.cursor();
        let size = end - start;
        self.fields.borrow_mut().push(Box::new(TXTPosition {
            start,
            size,
            data: reader.get_raw(),
            content,
        }));
        val
    }
    pub fn read_txt (&self, reader: &Reader, start: usize, size: usize, content: String){
        self.fields.borrow_mut().push(Box::new(TXTPosition {
            start,
            size,
            data: reader.get_raw(),
            content,
        }));
    }
    pub fn _readoption_with_format_string<K>(
        &self,
        reader: &Reader,
        opt: impl Fn(&Reader) -> Result<K>,
        tmp: &str,
    ) -> Result<K> where K:ToString {
        let start = reader.cursor();
        let val: K = opt(reader)?;
        let end = reader.cursor();
        let size = end - start;
        let content = tmp.replace("{}", val.to_string().as_str());
        self.read_txt(reader, start, size, content);
        // self.fields.borrow_mut().push(Box::new(TXTPosition {
        //     start,
        //     size,
        //     data: reader.get_raw(),
        //     content,
        // }));
        Ok(val)
    }
    pub fn _read_with_format_string_rs<K>(
        &self,
        reader: &Reader,
        opt: impl Fn(&Reader) -> Result<K>,
        tmp: &str,
    ) -> Result<K> where K:ToString {
        let start = reader.cursor();
        let val: K = opt(reader)?;
        let end = reader.cursor();
        let size = end - start;
        let content = tmp.replace("{}", val.to_string().as_str());
        self.fields.borrow_mut().push(Box::new(TXTPosition {
            start,
            size,
            data: reader.get_raw(),
            content,
        }));
        Ok(val)
    }
    pub fn _read_with_format_string<K>(
        &self,
        reader: &Reader,
        opt: impl Fn(&Reader) -> K,
        tmp: &str,
    ) -> K where K:ToString {
        let start = reader.cursor();
        let val: K = opt(reader);
        let end = reader.cursor();
        let size = end - start;
        let content = tmp.replace("{}", val.to_string().as_str());
        self.fields.borrow_mut().push(Box::new(TXTPosition {
            start,
            size,
            data: reader.get_raw(),
            content,
        }));
        val
    }
    pub fn _read_with_mapper<K>(
        &self,
        reader: &Reader,
        opt: impl Fn(&Reader) -> Result<K>,
        mapper: impl Fn(K) -> String,
    ) -> Result<K> where K: Clone {
        let start = reader.cursor();
        let val: K = opt(reader)?;
        let end = reader.cursor();
        let size = end - start;
        let content = mapper(val.clone());
        self.fields.borrow_mut().push(Box::new(TXTPosition {
            start,
            size,
            data: reader.get_raw(),
            content,
        }));
        Ok(val)
    }
    pub fn read_with_field<K>(
        &self,
        reader: &Reader,
        opt: impl Fn(&Reader) -> Result<PacketContext<K>>,
        head: Option<String>,
    ) -> Result<Rc<RefCell<K>>>
    where
        K: Initer + 'static,
        FieldPosition<K>: FieldBuilder<T>,
    {
        let start = reader.cursor();
        let packet = opt(reader)?;
        let rs = packet._clone_obj();
        let end = reader.cursor();
        let size = end - start;
        self.fields.borrow_mut().push(Box::new(FieldPosition {
            start,
            size,
            data: reader.get_raw(),
            head,
            packet,
        }));
        Ok(rs)
    }
}

pub struct Position<T> {
    pub start: usize,
    pub size: usize,
    data: Rc<Vec<u8>>,
    pub render: fn(usize, usize, &T) -> Field,
}
impl<T> FieldBuilder<T> for Position<T> {
    fn build(&self, t: &T) -> Field {
        (self.render)(self.start, self.size, t)
    }
    
    fn data(&self) -> Rc<Vec<u8>> {
        self.data.clone()
    }
}

pub struct FieldPosition<T>
where
    T: Initer,
{
    pub start: usize,
    pub size: usize,
    data: Rc<Vec<u8>>,
    head: Option<String>,
    pub packet: PacketContext<T>,
}
impl<T, K> FieldBuilder<T> for FieldPosition<K>
where
    K: Initer,
{
    fn build(&self, _: &T) -> Field {
        let summary = match self.head.clone() {
            Some(t) => t,
            _ => self.packet.get().borrow().summary(),
        };
        let mut field = Field::new(self.start, self.size, self.data.clone(), summary);
        let fields = self.packet.get_fields();
        field.children = RefCell::new(fields);
        field
    }
    
    fn data(&self) -> Rc<Vec<u8>> {
        self.data.clone()
    }
}

pub struct StringPosition<T> {
    pub start: usize,
    pub size: usize,
    data: Rc<Vec<u8>>,
    pub render: fn(&T) -> String,
}
impl<T> FieldBuilder<T> for StringPosition<T> {
    fn build(&self, t: &T) -> Field {
        let summary = (self.render)(t);
        Field::new(self.start, self.size, self.data.clone(), summary)
    }
    
    fn data(&self) -> Rc<Vec<u8>> {
        self.data.clone()
    }
}

pub struct TXTPosition {
    start: usize,
    size: usize,
    data: Rc<Vec<u8>>,
    content: String,
}
impl<T> FieldBuilder<T> for TXTPosition {
    fn build(&self, _: &T) -> Field {
        Field::new(self.start, self.size, self.data.clone(), self.content.clone())
    }
    fn data(&self) -> Rc<Vec<u8>> {
        self.data.clone()
    }
}

pub trait DomainService {
    fn name(&self) -> String;
    fn _type(&self) -> String;
    fn proto(&self) -> String;
    fn class(&self) -> String;
    fn content(&self) -> String;
    fn ttl(&self) -> u32;
}

pub struct TCPConnection{
    count: usize,
    throughput: usize,    
}

pub trait Initer {
    fn new() -> Self;
    fn summary(&self) -> String;
}
pub trait InfoPacket{
    fn info(&self) -> String;
}

#[derive(Default, Clone)]
pub struct FrameSummary {
    pub index: u32,
    pub source: String,
    pub target: String,
    pub protocol: String,
    pub link_type: u16,
}

pub struct Frame {
    pub ts: u64,
    pub capture_size: u32,
    pub origin_size: u32,
    pub summary: RefCell<FrameSummary>,
    data: Rc<Vec<u8>>,
    pub ctx: Rc<Context>,
    pub eles: RefCell<Vec<ProtocolData>>,
}
impl Frame {
    pub fn new(
        ctx: Rc<Context>,
        data: Vec<u8>,
        ts: u64,
        capture_size: u32,
        origin_size: u32,
        index: u32,
        link_type: u16,
    ) -> Frame {
        let f = Frame {
            ctx,
            eles: RefCell::new(Vec::new()),
            summary: RefCell::new(FrameSummary {
                index,
                link_type,
                ..Default::default()
            }),
            data: Rc::new(data),
            ts,
            capture_size,
            origin_size,
        };
        f
    }
    pub fn to_string(&self) -> String {
        format!(
            "Frame {}: {} bytes on wire ({} bits), {} bytes captured ({} bits)",
            self.summary.borrow().index,
            self.origin_size,
            self.origin_size * 8,
            self.capture_size,
            self.capture_size * 8
        )
    }

    pub fn info(&self) -> String {
        let list = self.eles.borrow();
        let the_last = list.last();
        match the_last {
            Some(data) => data.info(),
            None => "N/A".into()
        }
    }
    pub fn update_host(&self, packet: Ref<impl IPPacket>){
        let mut s = self.summary.borrow_mut();
        s.source = packet.source_ip_address();
        s.target = packet.target_ip_address();
        drop(s);
    }
    pub fn update_tcp(&self,packet: &TCP){
        let s = self.summary.borrow();
        let source = s.source.clone();
        let target = s.target.clone();
        if source > target {}
        
    }
    pub fn get_fields(&self) -> Vec<Field> {
        let mut rs = Vec::new();
        let mut lists = Vec::new();
        let ltype = self.summary.borrow().link_type;
        lists.push(Field::new3(format!(
            "Encapsulation type: {} ({})",
            link_type_mapper(ltype),
            ltype
        )));
        lists.push(Field::new3(format!(
            "UTC Arrival Time: {} UTC",
            date_str(self.ts)
        )));
        lists.push(Field::new3(format!(
            "Frame Number: {}",
            self.summary.borrow().index
        )));
        lists.push(Field::new3(format!(
            "Frame Length: {} bytes ({} bits)",
            self.origin_size,
            self.origin_size * 8
        )));
        lists.push(Field::new3(format!(
            "Capture Length: {} bytes ({} bits)",
            self.capture_size,
            self.capture_size * 8
        )));
        rs.push(Field::new2(self.to_string(),Rc::new(Vec::new()), lists));
        for e in self.eles.borrow().iter() {
            let vs = e.get_fields();
            rs.push(Field::new2(e.summary(), self.data.clone(), vs));
        }
        rs
    }
    pub fn data(&self) -> Rc<Vec<u8>>{
        self.data.clone()
    }
    pub fn get_reader(&self) -> Reader {
        return Reader::new_raw(self.data());
    }
    pub fn create_packet<K>() -> PacketContext<K>
    where
        K: Initer,
    {
        let val = K::new();
        PacketContext {
            val: Rc::new(RefCell::new(val)),
            fields: RefCell::new(Vec::new()),
        }
    }
    pub fn _create<K>(val: K) -> PacketContext<K>{
        PacketContext {
            val: Rc::new(RefCell::new(val)),
            fields: RefCell::new(Vec::new()),
        }
    }
    pub fn add_element(&self, ele: ProtocolData) {
        let mut mref = self.summary.borrow_mut();
        mref.protocol = format!("{}", ele);
        drop(mref);
        match &ele {
            ProtocolData::IPV4(packet) => {
                self.update_host(packet.get().borrow());
            },
            ProtocolData::IPV6(packet) => {
                self.update_host(packet.get().borrow());
            },
            // ProtocolData::TCP(packet) => {
            //     self.update_tcp(packet.get().borrow());
            // },
            _ => {},
        }
        self.eles.borrow_mut().push(ele);
    }
}


pub type Ref2<T> = Rc<RefCell<T>>;
pub struct Context {
    count: Cell<u32>,
    info: RefCell<FileInfo>,
    dns: RefCell<Vec<Ref2<RecordResource>>>,
}

impl Context {
    pub fn add_dns_record(&self, rr: Ref2<RecordResource>){
        self.dns.borrow_mut().push(rr);
    }
    pub fn get_info(&self)-> FileInfo{
        self.info.borrow().clone()
    }
    pub fn get_dns_count(&self) -> usize {
        self.dns.borrow().len()
    }
    pub fn get_dns(&self) -> Vec<DNSRecord> {
        let mut rs = Vec::new();
        for d in self.dns.borrow().iter() {
            let aa = d.as_ref().borrow();
            rs.push(DNSRecord::create(aa));
        }
        rs
    }
}
pub struct Instance {
    ctx: Rc<Context>,
    frames: RefCell<Vec<Frame>>,
}
impl Instance {
    pub fn new(ftype: FileType) -> Instance {
        let ctx = Context {
            count: Cell::new(1),
            dns: RefCell::new(Vec::new()),
            info: RefCell::new(FileInfo {
                file_type: ftype,
                ..Default::default()
            }),
        };
        Instance {
            ctx: Rc::new(ctx),
            frames: RefCell::new(Vec::new()),
        }
    }
    pub fn create(&self, data: &[u8], ts: u64, capture_size: u32, origin_size: u32) {
        let ctx = self.context();
        let count = ctx.count.get();
        let link_type = ctx.info.borrow().link_type;
        let f = Frame::new(
            ctx.clone(),
            data.to_vec(),
            ts,
            capture_size,
            origin_size,
            count,
            link_type,
        );
        let reader = f.get_reader();
        let rs = crate::specs::execute(link_type, &f, &reader);
        match rs {
            Ok(_) => {
                self.frames.borrow_mut().push(f);
            },
            Err(e) => {
                error!("parse_frame_failed index:[{}]", count);
                error!("msg:[{}]", e.to_string());
                panic!("parse_failed");
            },
        }
        ctx.count.set(count + 1);
    }
    pub fn context(&self) -> Rc<Context> {
        self.ctx.clone()
    }
    pub fn get_frames(&self) -> Ref<Vec<Frame>> {
        self.frames.borrow()
    }
    pub fn get_info(&self) -> FileInfo {
        self.context().get_info()
    }
    pub fn update_ts(&self, ts: u64) {
        let ctx = self.context();
        let mut info = ctx.info.borrow_mut();
        if info.start_time > 0 {
            return;
        }
        info.start_time = ts;
    }
}
