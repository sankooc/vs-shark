pub mod pcap;
pub mod pcapng;

use crate::{
    common::{IPPacket, PortPacket},
    constants::link_type_mapper,
    specs::{
        dns::RecordResource,
        tcp::{TCPOptionKind, ACK, TCP},
        ProtocolData,
    },
};
use chrono::{DateTime, Utc};
use enum_dispatch::enum_dispatch;
use log::{error, info};
use std::{
    borrow::Borrow, cell::{Cell, Ref, RefCell}, collections::HashMap, fmt::Display, ops::{Deref, Range}, rc::Rc, time::{Duration, UNIX_EPOCH}
};

use anyhow::{bail, Result};
// pub mod pcapng;
use crate::common::{FileInfo, FileType, Reader};

pub type Ref2<T> = Rc<RefCell<T>>;

#[derive(Default, Clone)]
pub struct Field {
    pub start: usize,
    pub size: usize,
    pub summary: String,
    pub data: Rc<Vec<u8>>,
    pub children: RefCell<Vec<Field>>,
}
impl Field {
    pub fn new(start: usize, size: usize, data: Rc<Vec<u8>>, summary: String) -> Field {
        Field {
            start,
            size,
            data,
            summary,
            children: RefCell::new(Vec::new()),
        }
    }
    pub fn new2(summary: String, data: Rc<Vec<u8>>, vs: Vec<Field>) -> Field {
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
    fn status(&self) -> String;
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



pub type MultiBlock<T> = Vec<Ref2<T>>;

pub struct COption {
    summary: String,
    limit: usize,
}

pub type PacketOpt = usize;

impl<T> Initer for MultiBlock<T> {
    fn new() -> MultiBlock<T> {
        Vec::new()
    }

    fn summary(&self) -> String {
        String::from("")
    }
}

pub struct PacketContext<T> {
    val: Ref2<T>,
    fields: RefCell<Vec<Box<dyn FieldBuilder<T>>>>,
}

impl<T> PacketContext<T> {
    pub fn _clone_obj(&self) -> Ref2<T> {
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

    fn status(&self) -> String {
        self.get().borrow().status()
    }
}
impl<T> PacketContext<T>
where
    T: Initer + 'static,
{
    pub fn _build(&self, reader: &Reader, start: usize, size: usize, content: String) {
        self.fields.borrow_mut().push(Box::new(TXTPosition {
            start,
            size,
            data: reader.get_raw(),
            content,
        }));
    }

    pub fn _build_lazy(
        &self,
        reader: &Reader,
        start: usize,
        size: usize,
        render: fn(&T) -> String,
    ) {
        self.fields.borrow_mut().push(Box::new(StringPosition {
            start,
            size,
            data: reader.get_raw(),
            render,
        }));
    }

    pub fn build_lazy<K>(
        &self,
        reader: &Reader,
        opt: impl Fn(&Reader) -> Result<K>,
        render: fn(&T) -> String,
    ) -> Result<K> {
        let start = reader.cursor();
        let val: K = opt(reader)?;
        let end = reader.cursor();
        let size = end - start;
        self._build_lazy(reader, start, size, render);
        Ok(val)
    }
    pub fn build_compact(&self, content: String, data: Rc<Vec<u8>>) {
        let size = data.len();
        self.fields.borrow_mut().push(Box::new(TXTPosition {
            start: 0,
            size,
            data,
            content,
        }));
    }
    pub fn append_string(&self, content: String, data: Rc<Vec<u8>>) {
        self.fields.borrow_mut().push(Box::new(TXTPosition {
            start: 0,
            size: 0,
            data,
            content,
        }));
    }
    pub fn build<K>(&self, reader: &Reader, opt: impl Fn(&Reader) -> K, content: String) -> K {
        let start = reader.cursor();
        let val: K = opt(reader);
        let end = reader.cursor();
        let size = end - start;
        self._build(reader, start, size, content);
        val
    }

    pub fn build_format<K>(
        &self,
        reader: &Reader,
        opt: impl Fn(&Reader) -> Result<K>,
        tmp: &str,
    ) -> Result<K>
    where
        K: ToString,
    {
        let start = reader.cursor();
        let val: K = opt(reader)?;
        let end = reader.cursor();
        let size = end - start;
        let content = tmp.replace("{}", val.to_string().as_str());
        self._build(reader, start, size, content);
        Ok(val)
    }

    pub fn build_fn<K>(
        &self,
        reader: &Reader,
        opt: impl Fn(&Reader) -> Result<K>,
        mapper: impl Fn(K) -> String,
    ) -> Result<K>
    where
        K: Clone,
    {
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
    pub fn build_packet<K, M>(
        &self,
        reader: &Reader,
        opt: impl Fn(&Reader, Option<M>) -> Result<PacketContext<K>>,
        packet_opt: Option<M>,
        head: Option<String>,
    ) -> Result<Ref2<K>>
    where
        K: Initer + 'static,
        FieldPosition<K>: FieldBuilder<T>,
    {
        let start = reader.cursor();
        let packet = opt(reader, packet_opt)?;
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
        Field::new(
            self.start,
            self.size,
            self.data.clone(),
            self.content.clone(),
        )
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

pub enum TCPDetail {
    KEEPALIVE,
    NOPREVCAPTURE,
    RETRANSMISSION,
    DUMP,
    // SEGMENT,
    // SEGMENTS(Vec<Segment>),
    NONE,
}

pub struct Segment {
    pub index: u32,
    pub range: Range<usize>,
}
impl Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let len = self.data.len();
        // f.write_fmt(format_args!("[Frame: {}, payload: {} bytes]", self.index, len))
        f.write_fmt(format_args!("[Frame: {}, payload: bytes]", self.index))
    }
}

#[derive(Default)]
pub enum TCPPAYLOAD {
    #[default]
    NONE,
    TLS,
}
#[derive(Default)]
pub struct Endpoint {
    pub host: String,
    pub port: u16,
    seq: u32,
    ack: u32,
    pub next: u32,
    _seq: u32,
    _ack: u32,
    _checksum: u16,
    mss: u16,
    //
    _seg: Option<Vec<u8>>,
    _seg_len: usize,
    _segments: Option<Vec<Segment>>,
    pub _seg_type: TCPPAYLOAD,
}
impl Endpoint {
    fn new(host: String, port: u16) -> Self {
        Self {
            host,
            port,
            ..Default::default()
        }
    }

    // fn add_segment(&mut self, tcp: &TCP, frame: &Frame, data: &[u8]){
    //     let segment = Segment{index: frame.summary.borrow().index, data: Rc::new(data.to_vec())};
    //     match &mut self.segments {
    //         Some(seg) => {
    //             seg.push(segment)
    //         },
    //         None => {
    //             let mut list = Vec::new();
    //             list.push(segment);
    //             self.segments = Some(list);
    //         }

    //     }
    // }
    pub fn segment_count(&mut self) -> usize {
        self._seg_len
    }
    pub fn take_segment(&mut self) -> Vec<u8> {
        let rs = self._seg.take().unwrap();
        self.clear_segment();
        rs
    }
    pub fn get_segment(&self) -> Result<&[u8]> {
        match &self._seg {
            Some(data) => {
                Ok(data)
            },
            None => {
                bail!("nodata")
            }
        }
    }
    pub fn add_segment(&mut self, frame: &Frame, _type: TCPPAYLOAD, data: &[u8]) {
        let range = self._seg_len..(self._seg_len + data.len());
        self._seg_len += data.len();
        self._seg_type = _type;
        match &mut self._seg {
            Some(list) => {
                list.extend_from_slice(data);
            },
            None => {
                self._seg = Some(data.to_vec());
            }
        }
        let segment = Segment {
            index: frame.summary.borrow().index,
            range,
        };
        match &mut self._segments {
            Some(seg) => seg.push(segment),
            None => {
                let mut list = Vec::new();
                list.push(segment);
                self._segments = Some(list);
            }
        }
    }
    fn clear_segment(&mut self) {
        self._segments = None;
        self._seg_len = 0;
        self._seg = None;
        self._seg_type = TCPPAYLOAD::NONE;
    }

    fn update(&mut self, tcp: &TCP, frame: &Frame, data: &[u8]) -> TCPDetail {
        let sequence = tcp.sequence;
        if self._checksum == tcp.crc {
            self.clear_segment();
            return TCPDetail::RETRANSMISSION;
        }

        match tcp.options.borrow() {
            Some(opt) => {
                let _ref = opt.as_ref().borrow();
                for _opt in _ref.iter() {
                    match _opt.as_ref().borrow().data {
                        TCPOptionKind::MSS(mss) => {
                            self.mss = mss;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        if self.seq == 0 {
            self._seq = sequence;
            self.seq = sequence;
            self.next = sequence + tcp.payload_len as u32;
            self._checksum = tcp.crc;
            return TCPDetail::NONE;
        }
        if sequence > self.next {
            self.seq = sequence;
            self.next = sequence + tcp.payload_len as u32;
            self._checksum = tcp.crc;
            self.clear_segment();
            return TCPDetail::NOPREVCAPTURE;
        } else if sequence == self.next {
            self.seq = tcp.sequence;
            self._checksum = tcp.crc;
            let len = tcp.payload_len;
            if len == 0 {
                // if tcp.state.check(ACK) {
                //     return TCPDetail::KEEPALIVE;
                // }
                return TCPDetail::NONE;
            }
            self.next = tcp.sequence + len as u32;
            return TCPDetail::NONE;
            // let _len = len + (tcp.len - 5) * 4;
            // if self.mss > 0 && _len >= self.mss {
            //     self.add_segment(tcp, frame, data);
            //     return TCPDetail::SEGMENT;
            // } else {
            //     return match self.segments.take() {
            //         Some(mut opt) => {
            //             opt.push(Segment{index: frame.summary.borrow().index, data: Rc::new(data.to_vec())});
            //             TCPDetail::SEGMENTS(opt)
            //         },
            //         None => TCPDetail::NONE,
            //     }
            // }
        } else {
            if sequence == self.next - 1 && tcp.payload_len == 1 && tcp.state.check(ACK) {
                self._checksum = tcp.crc;
                return TCPDetail::KEEPALIVE;
            }
            self.clear_segment();
            return TCPDetail::DUMP;
        }
    }
    pub fn stringfy(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
    fn confirm(&mut self, tcp: &TCP) {
        let acknowledge = tcp.acknowledge;
        if self._ack == 0 {
            self._ack = acknowledge;
        }

        if self.ack > acknowledge {
            return;
            // TODO
        }
        if self.seq < acknowledge {
            // TODO
        }
        self.ack = acknowledge;
    }
}
pub struct TCPConnection {
    pub count: Cell<u16>,
    pub throughput: Cell<u32>,
    pub ep1: Ref2<Endpoint>,
    pub ep2: Ref2<Endpoint>,
}

pub struct TCPInfo {
    pub detail: TCPDetail,
    pub _seq: u32,
    pub _ack: u32,
    pub next: u32,
}
impl TCPConnection {
    fn create_ep(src: String, port: u16) -> Ref2<Endpoint> {
        Rc::new(RefCell::new(Endpoint::new(src, port)))
    }
    fn new(ip: &dyn IPPacket, packet: &TCP, arch: bool) -> Self {
        let src = ip.source_ip_address();
        let dst = ip.target_ip_address();
        let srp = packet.source_port();
        let dsp = packet.target_port();
        let ep1 = TCPConnection::create_ep(src, srp);
        let ep2 = TCPConnection::create_ep(dst, dsp);
        if arch {
            return Self {
                count: Cell::new(0),
                throughput: Cell::new(0),
                ep1,
                ep2,
            };
        }
        Self {
            count: Cell::new(0),
            throughput: Cell::new(0),
            ep2: ep1,
            ep1: ep2,
        }
    }
    fn get_endpoint(&self, arch: bool) -> Ref2<Endpoint> {
        match arch {
            true => self.ep1.clone(),
            false => self.ep2.clone(),
        }
    }
    fn update(&self, arch: bool, tcp: &TCP, frame: &Frame, data: &[u8]) -> TCPInfo {
        let (main, rev) = match arch {
            true => (self.ep1.clone(), self.ep2.clone()),
            false => (self.ep2.clone(), self.ep1.clone()),
        };
        let _count = self.count.get();
        self.count.set(_count + 1);
        let mut _main = main.as_ref().borrow_mut();
        let detail = _main.update(tcp, frame, data);
        let _seq = _main._seq;
        let next = _main.next;
        drop(_main);
        let mut _rev = rev.as_ref().borrow_mut();
        _rev.confirm(tcp);
        let _ack = _rev._ack;
        drop(_rev);
        let _size = self.throughput.get();
        self.throughput.set(_size + tcp.payload_len as u32);
        TCPInfo {
            next,
            _ack,
            _seq,
            detail,
        }
    }
}

pub trait Initer {
    fn new() -> Self;
    fn summary(&self) -> String;
}
pub trait InfoPacket {
    fn info(&self) -> String;
    fn status(&self) -> String;
}

#[derive(Default, Clone)]
pub struct FrameSummary {
    pub index: u32,
    pub source: String,
    pub target: String,
    pub protocol: String,
    pub link_type: u16,
    pub ip: Option<Ref2<dyn IPPacket>>,
    pub tcp: Option<Ref2<TCP>>,
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
            None => "N/A".into(),
        }
    }
    pub fn get_ip(&self) -> Ref2<dyn IPPacket> {
        let sum = self.summary.borrow();
        match &sum.ip {
            Some(_ip) => _ip.clone(),
            _ => {
                panic!("nodata")
            }
        }
    }
    pub fn update_host(&self, src: &str, dst: &str) {
        let mut s = self.summary.borrow_mut();
        s.source = src.into();
        s.target = dst.into();
        drop(s);
    }
    pub fn update_ip(&self, packet: Ref2<dyn IPPacket>) {
        let _ip = packet.as_ref().borrow();
        self.update_host(&_ip.source_ip_address(), &_ip.target_ip_address());
        drop(_ip);
        let mut s = self.summary.borrow_mut();
        s.ip = Some(packet);
        drop(s);
    }
    fn add_tcp(&self, packet: Ref2<TCP>) {
        let mut s = self.summary.borrow_mut();
        s.tcp = Some(packet);
        drop(s);
    }
    pub fn update_tcp(&self, packet: &TCP, data: &[u8]) -> TCPInfo {
        let ippacket = self.get_ip();
        let refer = ippacket.deref().borrow();
        self.ctx.update_tcp(self, refer.deref(), packet, data)
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
        rs.push(Field::new2(self.to_string(), Rc::new(Vec::new()), lists));
        for e in self.eles.borrow().iter() {
            let vs = e.get_fields();
            rs.push(Field::new2(e.summary(), self.data.clone(), vs));
        }
        rs
    }
    pub fn data(&self) -> Rc<Vec<u8>> {
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
    pub fn _create<K>(val: K) -> PacketContext<K> {
        PacketContext {
            val: Rc::new(RefCell::new(val)),
            fields: RefCell::new(Vec::new()),
        }
    }
    pub fn get_tcp_info(&self) -> Result<Ref2<Endpoint>> {
        let sum = self.summary.borrow();
        let _ip = sum.ip.clone().expect("no_ip_layer");
        let _tcp = sum.tcp.clone().expect("no_tcp_layer");
        let refer = _ip.deref().borrow();
        let tcp_refer = _tcp.deref().borrow();
        drop(sum);
        self.ctx.get_tcp(refer.deref(), tcp_refer.deref())
    }
    pub fn add_element(&self, ele: ProtocolData) {
        let mut mref = self.summary.borrow_mut();
        mref.protocol = format!("{}", ele);
        drop(mref);
        match &ele {
            ProtocolData::IPV4(packet) => {
                self.update_ip(packet._clone_obj());
            }
            ProtocolData::IPV6(packet) => {
                self.update_ip(packet._clone_obj());
            }
            ProtocolData::ARP(packet) => {
                self.update_ip(packet._clone_obj());
            }
            ProtocolData::TCP(packet) => {
                self.add_tcp(packet._clone_obj());
            }
            _ => {}
        }
        self.eles.borrow_mut().push(ele);
    }
}

pub struct Context {
    count: Cell<u32>,
    info: RefCell<FileInfo>,
    pub dns: RefCell<Vec<Ref2<RecordResource>>>,
    conversation_map: RefCell<HashMap<String, TCPConnection>>,
}

impl Context {
    pub fn add_dns_record(&self, rr: Ref2<RecordResource>) {
        self.dns.borrow_mut().push(rr);
    }
    pub fn get_info(&self) -> FileInfo {
        self.info.borrow().clone()
    }
    pub fn get_dns_count(&self) -> usize {
        self.dns.borrow().len()
    }
    pub fn conversations(&self) -> Ref<HashMap<String, TCPConnection>> {
        let rs = self.conversation_map.borrow();
        rs
    }
    pub fn tcp_key(ip: &dyn IPPacket, packet: &TCP) -> (String, bool) {
        let source = format!("{}:{}", ip.source_ip_address(), packet.source_port());
        let target = format!("{}:{}", ip.target_ip_address(), packet.target_port());
        let arch = source > target;
        if arch {
            return (format!("{}-{}", source, target), arch);
        }
        (format!("{}-{}", target, source), arch)
    }
    fn update_tcp(&self, frame: &Frame, ip: &dyn IPPacket, packet: &TCP, data: &[u8]) -> TCPInfo {
        let (key, arch) = Context::tcp_key(ip, packet);
        let mut _map = self.conversation_map.borrow_mut();
        let v = _map.get(&key);
        let conn = match v {
            Some(conn) => conn,
            None => {
                let con = TCPConnection::new(ip, packet, arch);
                _map.insert(key.clone(), con);
                _map.get(&key).unwrap()
            }
        };
        conn.update(arch, packet, frame, data)
    }
    fn get_tcp(&self,ip: &dyn IPPacket, packet: &TCP) -> Result<Ref2<Endpoint>>{
        let (key, arch) = Context::tcp_key(ip, packet);
        let mut _map = self.conversation_map.borrow_mut();
        let conn = _map.get(&key).expect("no_tcp_connection");
        let ep = conn.get_endpoint(arch);
        drop(_map);
        Ok(ep)
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
            conversation_map: RefCell::new(HashMap::new()),
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
            }
            Err(e) => {
                error!("parse_frame_failed index:[{}]", count);
                error!("msg:[{}]", e.to_string());
                panic!("parse_failed");
            }
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
