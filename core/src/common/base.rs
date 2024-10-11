use crate::{
    common::{
        concept::{HttpRequestBuilder, LineData, Lines, PCAPInfo, Statistic, TCPConnectInfo},
        io::AReader,
        IPPacket, MultiBlock, PortPacket, Ref2, FIELDSTATUS,
    },
    constants::link_type_mapper,
    specs::{
        dns::{RecordResource, ResourceType, DNS},
        http::{self, HTTPVisitor, HttpType, HTTP},
        tcp::{ACK, RESET, TCP},
        tls::{
            handshake::{HandshakeClientHello, HandshakeServerHello, HandshakeType},
            TLSRecorMessage, TLSVisitor, TLS,
        },
        ProtocolData,
    },
};
use chrono::{DateTime, Utc};
use enum_dispatch::enum_dispatch;
use log::error;
use std::{
    borrow::Borrow,
    cell::{Ref, RefCell},
    collections::{HashMap, HashSet, VecDeque},
    net::{Ipv4Addr, Ipv6Addr},
    ops::{Deref, DerefMut, Range},
    rc::Rc,
    time::{Duration, UNIX_EPOCH},
};

use anyhow::{bail, Result};
// pub mod pcapng;
use crate::common::io::Reader;
use crate::common::{FileInfo, FileType};

use super::{concept::TLSHS, io::SliceReader};

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
    fn status(&self) -> FIELDSTATUS;
    fn info(&self) -> String;
}

pub trait Visitor {
    fn visit(&self, frame: &mut Frame, ctx: &mut Context, reader: &Reader) -> Result<(ProtocolData, &'static str)>;
    // fn visit(&self, frame: &Frame, reader: &Reader) -> Result<(ProtocolData, &'static str)>;
}

pub trait FieldBuilder<T> {
    fn build(&self, t: &T) -> Field;
    fn data(&self) -> Rc<Vec<u8>>;
}

pub type PacketOpt = usize;

impl<T> PacketBuilder for MultiBlock<T> {
    fn new() -> MultiBlock<T> {
        Vec::new()
    }

    fn summary(&self) -> String {
        String::from("")
    }
}

pub struct PacketContext<T: ?Sized> {
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
    pub fn get_fields(&self) -> Vec<Field> {
        let t: &T = &self.get().borrow();
        let mut rs: Vec<Field> = Vec::new();
        for pos in self.fields.borrow().iter() {
            rs.push(pos.build(t));
        }
        rs
    }
}
fn _convert(f_status: FIELDSTATUS) -> &'static str {
    match f_status {
        FIELDSTATUS::WARN => "deactive",
        FIELDSTATUS::ERROR => "errordata",
        _ => "info",
    }
}
impl<T> Element for PacketContext<T>
where
    T: PacketBuilder + InfoPacket,
{
    fn summary(&self) -> String {
        self.get().borrow().summary()
    }
    fn get_fields(&self) -> Vec<Field> {
        self.get_fields()
    }
    fn info(&self) -> String {
        self.get().borrow().info()
    }

    fn status(&self) -> FIELDSTATUS {
        self.get().borrow().status()
    }
}
impl<T> PacketContext<T>
where
    T: PacketBuilder + 'static,
{
    pub fn _build(&self, reader: &Reader, start: usize, size: usize, content: String) {
        self.fields.borrow_mut().push(Box::new(TXTPosition { start, size, data: reader.get_raw(), content }));
    }

    pub fn _build_lazy(&self, reader: &Reader, start: usize, size: usize, render: fn(&T) -> String) {
        self.fields.borrow_mut().push(Box::new(StringPosition { start, size, data: reader.get_raw(), render }));
    }

    pub fn build_skip(&self, reader: &Reader, size: usize) {
        let start = reader.cursor();
        let content = format!("resolve later [{}]", size);
        reader.slice(size);
        self._build(reader, start, size, content);
    }

    pub fn build_lazy<K>(&self, reader: &Reader, opt: impl Fn(&Reader) -> Result<K>, render: fn(&T) -> String) -> Result<K> {
        let start = reader.cursor();
        let val: K = opt(reader)?;
        let end = reader.cursor();
        let size = end - start;
        self._build_lazy(reader, start, size, render);
        Ok(val)
    }
    pub fn build_compact(&self, content: String, data: Rc<Vec<u8>>) {
        let size = data.len();
        self.fields.borrow_mut().push(Box::new(TXTPosition { start: 0, size, data, content }));
    }
    pub fn append_string(&self, content: String, data: Rc<Vec<u8>>) {
        self.fields.borrow_mut().push(Box::new(TXTPosition { start: 0, size: 0, data, content }));
    }
    pub fn build<K>(&self, reader: &Reader, opt: impl Fn(&Reader) -> K, content: String) -> K {
        let start = reader.cursor();
        let val: K = opt(reader);
        let end = reader.cursor();
        let size = end - start;
        self._build(reader, start, size, content);
        val
    }

    pub fn build_backward(&self, reader: &Reader, step: usize, content: String) {
        let cur = reader.cursor();
        if cur < step {
            return;
        }
        let from = cur - step;
        self._build(reader, from, step, content);
    }

    pub fn build_format<K>(&self, reader: &Reader, opt: impl Fn(&Reader) -> Result<K>, tmp: &str) -> Result<K>
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

    pub fn build_fn<K>(&self, reader: &Reader, opt: impl Fn(&Reader) -> Result<K>, mapper: impl Fn(K) -> String) -> Result<K>
    where
        K: Clone,
    {
        let start = reader.cursor();
        let val: K = opt(reader)?;
        let end = reader.cursor();
        let size = end - start;
        let content = mapper(val.clone());
        self.fields.borrow_mut().push(Box::new(TXTPosition { start, size, data: reader.get_raw(), content }));
        Ok(val)
    }
    pub fn build_packet<K, M>(&self, reader: &Reader, opt: impl Fn(&Reader, Option<M>) -> Result<PacketContext<K>>, packet_opt: Option<M>, head: Option<String>) -> Result<Ref2<K>>
    where
        K: PacketBuilder + 'static,
        FieldPosition<K>: FieldBuilder<T>,
    {
        let start = reader.cursor();
        let packet = opt(reader, packet_opt)?;
        let rs = packet._clone_obj();
        let end = reader.cursor();
        let size = end - start;
        self.fields.borrow_mut().push(Box::new(FieldPosition { start, size, data: reader.get_raw(), head, packet }));
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
    T: PacketBuilder,
{
    pub start: usize,
    pub size: usize,
    data: Rc<Vec<u8>>,
    head: Option<String>,
    pub packet: PacketContext<T>,
}
impl<T, K> FieldBuilder<T> for FieldPosition<K>
where
    K: PacketBuilder,
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

#[derive(Clone)]
pub enum TCPDetail {
    KEEPALIVE,
    NOPREVCAPTURE,
    RETRANSMISSION,
    DUMP,
    RESET,
    // SEGMENT,
    // SEGMENTS(Vec<Segment>),
    NONE,
}

pub struct Segment {
    pub index: u32,
    pub size: usize,
    // pub data: Vec<u8>,
    pub list: Ref2<Vec<ProtocolData>>,
    // pub range: Range<usize>,
}
// impl Display for Segment {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         // let len = self.data.len();
//         // f.write_fmt(format_args!("[Frame: {}, payload: {} bytes]", self.index, len))
//         f.write_fmt(format_args!("[Frame: {}, payload: bytes]", self.index))
//     }
// }

#[derive(Default)]
pub enum TCPPAYLOAD {
    #[default]
    NONE,
    TLS(usize),
    HTTPLEN(Rc<crate::specs::http::HTTP>),
    HTTPCHUNKED(Rc<crate::specs::http::HTTP>),
    HTTPPRE,
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
    // mss: u16,
    //
    pub info: TCPConnectInfo,
    //
    //
    _request: Option<HttpRequestBuilder>,
    pub handshake: Vec<HandshakeType>,

    pub _segments: Option<VecDeque<Segment>>,
    pub _cache: Vec<u8>,

    pub connec_type: TCPPAYLOAD,
}

fn add_to(ep: &mut Endpoint, data: Rc<Vec<u8>>, list: Ref2<Vec<ProtocolData>>) {
    let reader = Reader::new_raw(data);
    if let Ok(rs) = TLSVisitor.visit(&reader) {
        if let ProtocolData::TLS(pcaket) = &rs {
            let tls = pcaket.get().borrow();
            ep.add_tls(tls.deref());
        }
        let mut list_ref = list.as_ref().borrow_mut();
        list_ref.push(rs);
        drop(list_ref);
    }
}
impl Endpoint {
    fn new(host: String, port: u16) -> Self {
        Self { host, port, ..Default::default() }
    }
    pub fn add_segment(&mut self, index: u32, data: Vec<u8>, list: Ref2<Vec<ProtocolData>>) {
        let segment = Segment { index, size: data.len(), list };
        let mut _data = data;
        self._cache.append(&mut _data);
        match self._segments.as_mut() {
            Some(mut _list) => {
                _list.push_back(segment);
            }
            None => {
                let mut _l = VecDeque::new();
                _l.push_back(segment);
                self._segments = Some(_l);
            }
        }
    }
    pub fn take_segment(&mut self) -> Option<VecDeque<Segment>> {
        let rs = self._segments.take();
        self.clear_segment();
        rs
    }
    pub fn update_segment(&mut self) {
        match &self.connec_type {
            TCPPAYLOAD::HTTPPRE => {
                let reader = SliceReader::new(&self._cache);
                if let Ok(http) = http::parse(&reader) {
                    if http.len > 0 {
                        self.connec_type = TCPPAYLOAD::HTTPLEN(Rc::new(http));
                        let exist = reader.left().unwrap();
                        self._cache = reader.slice(exist).to_vec();
                        // resize
                        return self.update_segment();
                    } else if http.chunked {
                        self.connec_type = TCPPAYLOAD::HTTPCHUNKED(Rc::new(http));
                        let exist = reader.left().unwrap();
                        self._cache = reader.slice(exist).to_vec();
                        // resize
                        return self.update_segment();
                    } else {
                        
                        // clearcache
                    }
                }
            }
            TCPPAYLOAD::HTTPLEN(http) => {

            },
            TCPPAYLOAD::HTTPCHUNKED(http) => {

            },
            TCPPAYLOAD::TLS(next_size) => {
                let size = *next_size;
                let clen = self._cache.len();
                if size > clen {
                } else if size == clen {
                    // remove all
                    let data: Vec<u8> = self._cache.drain(..).collect();
                    if let Some(segments) = &mut self._segments {
                        let mut sgs: Vec<(u32, Range<usize>)> = Vec::new();
                        // let reff = Rc::new(RefCell::new(sgs));
                        loop {
                            if let Some(seg) = segments.pop_front() {
                                let Segment { list, size, index } = seg;
                                if segments.len() == 0 {
                                    add_to(self, Rc::new(data), list);
                                    break;
                                } else {
                                    sgs.push((index, 0..size));
                                }
                            } else {
                                break;
                            }
                        }
                    }

                    self._segments = None;
                    self._cache = Vec::new();
                    self.connec_type = TCPPAYLOAD::NONE;
                } else {
                    let mount: Vec<u8> = self._cache.drain(0..size).collect();
                    self.connec_type = TCPPAYLOAD::NONE;
                    let mut _count = size;
                    if let Some(segments) = &mut self._segments {
                        loop {
                            if let Some(seg) = segments.pop_front() {
                                let Segment { list, size, index } = seg;
                                if size <= _count {
                                    _count -= size;
                                } else {
                                    let l2 = list.clone();
                                    segments.push_front(Segment { index, list, size: (size - _count) });
                                    _count = 0;
                                    add_to(self, Rc::new(mount), l2);
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                        println!("]")
                    }
                    self.update_segment();
                }
            }
            TCPPAYLOAD::NONE => {
                let cache_len = self._cache.len();
                if cache_len > 5 {
                    let (is_tls, len) = TLS::_check(&self._cache).unwrap();
                    if is_tls {
                        self.connec_type = TCPPAYLOAD::TLS(len + 5);
                        return self.update_segment();
                    }
                }
                let reader = SliceReader::new(&self._cache);
                if HTTPVisitor::check(&reader) {
                    self.connec_type = TCPPAYLOAD::HTTPPRE;
                    return self.update_segment();
                }
            }
        }
    }
    pub fn flush_segment(&mut self) {
        self.update_segment();
        self._segments = None;
        self._cache = Vec::new();
        self.connec_type = TCPPAYLOAD::NONE;
    }
    fn clear_segment(&mut self) {

    }

    fn update(&mut self, tcp: &TCP, _: &Frame) -> TCPDetail {
        //https://www.wireshark.org/docs/wsug_html_chunked/ChAdvTCPAnalysis.html
        let sequence = tcp.sequence;
        let info = &mut self.info;
        info.count = info.count + 1;
        info.throughput += tcp.payload_len as u32;

        if self._checksum == tcp.crc {
            info.retransmission += 1;
            self.clear_segment();
            return TCPDetail::RETRANSMISSION;
        }
        if tcp.state._match(RESET) {
            self.clear_segment();
            return TCPDetail::RESET;
        }
        let tcp_len = tcp.payload_len as u32;
        if self.seq == 0 {
            self._seq = sequence;
            self.seq = sequence;
            self.next = sequence + tcp_len;
            self._checksum = tcp.crc;
            return TCPDetail::NONE;
        }
        if sequence > self.next {
            self.seq = sequence;
            self.next = sequence + tcp_len;
            self._checksum = tcp.crc;
            info.invalid += 1;
            self.clear_segment();
            return TCPDetail::NOPREVCAPTURE;
        } else if sequence == self.next {
            self.seq = tcp.sequence;
            self._checksum = tcp.crc;
            if tcp_len == 0 {
                if tcp.state.check(ACK) {
                    return TCPDetail::KEEPALIVE;
                }
                return TCPDetail::NONE;
            }
            self.next = tcp.sequence + tcp_len;
            return TCPDetail::NONE;
        } else {
            if sequence == self.next - 1 && tcp_len == 1 && tcp.state.check(ACK) {
                self._checksum = tcp.crc;
                return TCPDetail::KEEPALIVE;
            }
            info.invalid += 1;
            self.clear_segment();
            return TCPDetail::DUMP;
        }
    }
    pub fn stringfy(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    fn add_tls(&mut self, tls: &TLS){
        for record in &tls.records {
            match &record.as_ref().borrow().message {
                TLSRecorMessage::HANDSHAKE(hs) => {
                    for _hs in hs.as_ref().borrow().items.iter() {
                        let _msg = &_hs.as_ref().borrow().msg;
                        match _msg {
                            HandshakeType::Certificate(_) | HandshakeType::ClientHello(_) | HandshakeType::ServerHello(_) => {
                                self.handshake.push(_msg.clone());
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            };
        }
    }
    // (ack_correct, same_with_last_packet)
    fn confirm(&mut self, tcp: &TCP) -> (bool, bool) {
        let acknowledge = tcp.acknowledge;
        if self._ack == 0 {
            self._ack = acknowledge;
        }

        if self.ack > acknowledge {
            return (false, false);
            // TODO false
        }
        if self.seq < acknowledge {
            // TODO
        }
        let same = acknowledge == self.ack;
        self.ack = acknowledge;
        return (true, same);
    }
}
pub struct TCPConnection {
    pub connec_type: TCPPAYLOAD,
    pub ep1: Endpoint,
    pub ep2: Endpoint,
}

pub struct TCPInfo {
    pub detail: TCPDetail,
    pub _seq: u32,
    pub _ack: u32,
    pub next: u32,
}
impl TCPConnection {
    fn create_ep(src: String, port: u16) -> Endpoint {
        Endpoint::new(src, port)
    }
    fn new(ip: &dyn IPPacket, packet: &TCP, arch: bool) -> Self {
        let src = ip.source_ip_address();
        let dst = ip.target_ip_address();
        let srp = packet.source_port();
        let dsp = packet.target_port();
        let ep1 = TCPConnection::create_ep(src, srp);
        let ep2 = TCPConnection::create_ep(dst, dsp);
        if arch {
            return Self { connec_type: TCPPAYLOAD::NONE, ep1, ep2 };
        }
        Self { connec_type: TCPPAYLOAD::NONE, ep2: ep1, ep1: ep2 }
    }
    pub fn get_endpoint(&mut self, arch: bool) -> &mut Endpoint {
        match arch {
            true => &mut self.ep1,
            false => &mut self.ep2,
        }
    }
    pub fn sort(&self, compare: &HashMap<String, usize>) -> (&Endpoint, &Endpoint) {
        let h_1 = *compare.get(&self.ep1.host).unwrap_or(&0);
        let h_2 = *compare.get(&self.ep2.host).unwrap_or(&0);
        if h_1 < h_2 {
            return (&self.ep1, &self.ep2);
        }
        return (&self.ep2, &self.ep1);
    }
}

pub trait PacketBuilder {
    fn new() -> Self;
    fn summary(&self) -> String;
}
pub trait InfoPacket {
    fn info(&self) -> String;
    fn status(&self) -> FIELDSTATUS;
}

#[derive(Default)]
pub struct FrameSummary {
    pub index: u32,
    pub source: String,
    pub target: String,
    pub protocol: String,
    pub link_type: u32,
    pub ip: Option<Ref2<dyn IPPacket>>,
    pub tcp: Option<Ref2<TCP>>,
}

pub struct Frame {
    pub ts: u64,
    pub capture_size: u32,
    pub origin_size: u32,
    pub summary: FrameSummary,
    data: Rc<Vec<u8>>,
    pub eles: Ref2<Vec<ProtocolData>>,
}
impl Frame {
    pub fn new(data: Vec<u8>, ts: u64, capture_size: u32, origin_size: u32, index: u32, link_type: u32) -> Frame {
        let f = Frame {
            eles: Rc::new(RefCell::new(Vec::new())),
            summary: FrameSummary { index, link_type, ..Default::default() },
            data: Rc::new(data),
            ts,
            capture_size,
            origin_size,
        };
        f
    }
    pub fn to_string(&self) -> String {
        format!("Frame {}: {} bytes on wire ({} bits), {} bytes captured ({} bits)", self.summary.index, self.origin_size, self.origin_size * 8, self.capture_size, self.capture_size * 8)
    }
    pub fn get_protocol(&self) -> String {
        self.summary.protocol.to_lowercase()
    }
    pub fn do_match(&self, protos: &HashSet<String>) -> bool {
        let proto = self.get_protocol();
        protos.contains(&proto)
    }
    pub fn info(&self) -> String {
        let reff = self.eles.as_ref().borrow();
        let the_last = reff.last();
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

    pub fn get_ip_address(&self) -> (String, String) {
        let ip = self.get_ip();
        let _ip = ip.deref().borrow();
        (_ip.source_ip_address(), _ip.target_ip_address())
    }
    pub fn get_port(&self) -> (u16, u16) {
        let sum = self.summary.borrow();
        let tcp = sum.tcp.clone().expect("no_tcp_layer");
        let _tcp = tcp.deref().borrow();
        (_tcp.source_port, _tcp.target_port)
    }

    pub fn update_host(&mut self, src: &str, dst: &str) {
        let s = &mut self.summary;
        s.source = src.into();
        s.target = dst.into();
    }
    pub fn update_ip(&mut self, ctx: &mut Context, packet: Ref2<dyn IPPacket>) {
        let _ip = packet.as_ref().borrow();
        self.update_host(&_ip.source_ip_address(), &_ip.target_ip_address());

        // performance issue todo
        let ip_map = &mut ctx.statistic.ip;
        ip_map.inc(_ip.source_ip_address().as_str());
        ip_map.inc(_ip.target_ip_address().as_str());

        drop(_ip);
        let s = &mut self.summary;
        s.ip = Some(packet);
    }
    pub fn add_tcp(&mut self, packet: Ref2<TCP>) {
        let s = &mut self.summary;
        s.tcp = Some(packet);
    }
    fn add_dns(&self, packet: Ref2<DNS>, ctx: &mut Context) {
        let val = packet.as_ref().borrow();
        if val.answer_rr > 0 {
            if let Some(ans) = &val.answers_ref {
                for cel in ans.as_ref().borrow().iter() {
                    ctx.add_dns_record(cel.clone());
                }
            }
        }
        drop(val);
    }
    pub fn update_tcp(&self, packet: &mut TCP, ctx: &mut Context, reader: &Reader) {
        let ippacket = self.get_ip();
        let refer = ippacket.deref().borrow();
        ctx.update_tcp(self, refer.deref(), packet, reader)
    }
    pub fn get_fields(&self) -> Vec<Field> {
        let mut rs = Vec::new();
        let mut lists = Vec::new();
        let ltype = self.summary.link_type;
        lists.push(Field::new3(format!("Encapsulation type: {} ({})", link_type_mapper(ltype as u16), ltype)));
        lists.push(Field::new3(format!("UTC Arrival Time: {} UTC", date_str(self.ts))));
        lists.push(Field::new3(format!("Frame Number: {}", self.summary.index)));
        lists.push(Field::new3(format!("Frame Length: {} bytes ({} bits)", self.origin_size, self.origin_size * 8)));
        lists.push(Field::new3(format!("Capture Length: {} bytes ({} bits)", self.capture_size, self.capture_size * 8)));
        rs.push(Field::new2(self.to_string(), Rc::new(Vec::new()), lists));
        for e in self.eles.as_ref().borrow().iter() {
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
        K: PacketBuilder,
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
    pub fn get_tcp_map_key(&self) -> (String, bool) {
        let sum = &self.summary;
        let _ip = sum.ip.clone().expect("no_ip_layer");
        let _tcp = sum.tcp.clone().expect("no_tcp_layer");
        let refer = _ip.deref().borrow();
        let tcp_refer = _tcp.deref().borrow();
        Context::tcp_key(refer.deref(), tcp_refer.deref())
    }

    fn _create_http_request(&self) -> HttpRequestBuilder {
        let (source, dest) = self.get_ip_address();
        let (srp, dsp) = self.get_port();
        HttpRequestBuilder::new(source, dest, srp, dsp)
    }
    fn ipv4_sta(_ip: &Option<Ipv4Addr>, ctx: &mut Context) {
        let _map = &mut ctx.statistic.ip_type;
        if let Some(ip) = _ip {
            if ip.is_private() {
                _map.inc("private");
            } else if ip.is_documentation() {
                _map.inc("documentation");
            } else if ip.is_link_local() {
                _map.inc("link_local");
            } else if ip.is_loopback() {
                _map.inc("loopback");
            } else if ip.is_multicast() {
                _map.inc("multicast");
            } else {
                _map.inc("public");
            }
        }
    }
    fn ipv6_sta(_ip: &Option<Ipv6Addr>, ctx: &mut Context) {
        let _map = &mut ctx.statistic.ip_type;
        if let Some(ip) = _ip {
            if ip.is_loopback() {
                _map.inc("loopback");
            } else if ip.is_multicast() {
                _map.inc("multicast");
            } else {
                _map.inc("public");
            }
        }
    }
    pub fn add_element(&mut self, ctx: &mut Context, ele: ProtocolData, reader: &Reader) {
        match &ele {
            ProtocolData::IPV4(packet) => {
                self.update_ip(ctx, packet._clone_obj());
                let ip = packet.get().borrow();
                Frame::ipv4_sta(&ip.source_ip, ctx);
                Frame::ipv4_sta(&ip.target_ip, ctx);
                drop(ip);
            }
            ProtocolData::IPV6(packet) => {
                self.update_ip(ctx, packet._clone_obj());
                let ip = packet.get().borrow();
                Frame::ipv6_sta(&ip.source_ip, ctx);
                Frame::ipv6_sta(&ip.target_ip, ctx);
                drop(ip);
            }
            ProtocolData::ARP(packet) => {
                self.update_ip(ctx, packet._clone_obj());
            }
            ProtocolData::TCP(packet) => {
                let tcp = packet.get();
                self.update_tcp(tcp.borrow_mut().deref_mut(), ctx, reader);
            }
            ProtocolData::HTTP(packet) => {
                let http = packet._clone_obj();
                let _http = http.deref().borrow();
                let __type = _http._type();
                match __type {
                    HttpType::REQUEST(request) => {
                        // let ep = self.get_tcp_info(true, ctx);
                        let (key, arch) = self.get_tcp_map_key();
                        let _map = &mut ctx.conversation_map;
                        let mut conn = _map.get(&key).unwrap().borrow_mut();
                        let ep = conn.get_endpoint(arch);
                        // end todo
                        let mut rq = self._create_http_request();
                        rq.set_request(http.clone(), request, self.ts);
                        ep._request = Some(rq);
                    }
                    HttpType::RESPONSE(response) => {
                        // let ep = self.get_tcp_info(false, ctx);

                        let (key, arch) = self.get_tcp_map_key();
                        let _map = &mut ctx.conversation_map;
                        let mut conn = _map.get(&key).unwrap().borrow_mut();
                        let ep = conn.get_endpoint(!arch);
                        // end todo
                        let request = ep._request.take();
                        drop(conn);

                        if let Some(mut req) = request {
                            req.set_response(http.clone(), response, self.ts);
                            ctx.add_http(req);
                        }
                    }
                    _ => {}
                }
                ctx.http_statistic(http.clone());
                drop(_http);
            }

            ProtocolData::DNS(packet) => {
                self.add_dns(packet._clone_obj(), ctx);
            }
            _ => {}
        }
        let mut reff = self.eles.as_ref().borrow_mut();
        if let Some(_lst) = reff.last() {
            match _lst {
                ProtocolData::TLS(_) | ProtocolData::HTTP(_) => {
                    let last = reff.pop().unwrap();
                    // mref.protocol = format!("{}", last);
                    reff.push(ele);
                    reff.push(last);
                }
                _ => {
                    reff.push(ele);
                }
            }
        } else {
            reff.push(ele);
        }
        let mref = &mut self.summary;
        mref.protocol = format!("{}", reff.last().unwrap());
        drop(reff);
    }
}

pub struct Context {
    pub count: u32,
    pub info: FileInfo,
    pub dns: Vec<Ref2<RecordResource>>,
    pub conversation_map: HashMap<String, RefCell<TCPConnection>>,
    http_list: Vec<HttpRequestBuilder>,
    pub statistic: Statistic,
    pub dns_map: HashMap<String, String>,
}

impl Context {
    pub fn get_statistc(&self) -> &Statistic {
        &self.statistic
    }
    pub fn get_http(&self) -> &[HttpRequestBuilder] {
        &self.http_list
    }
    pub fn http_statistic(&mut self, t: Ref2<HTTP>) {
        let reff = t.as_ref().borrow();
        let ref_statis = &mut self.statistic;
        match reff._type() {
            HttpType::REQUEST(req) => {
                ref_statis.http_method.inc(&req.method.clone());
            }
            HttpType::RESPONSE(res) => {
                ref_statis.http_status.inc(&res.code);
            }
            _ => {}
        }
        if let Some(ct) = &reff.content_type {
            ref_statis.http_type.inc(ct);
        }
        drop(reff);
    }
    pub fn add_http(&mut self, req: HttpRequestBuilder) {
        let list = &mut self.http_list;
        list.push(req);
    }
    pub fn add_dns_record(&mut self, rr: Ref2<RecordResource>) {
        let _rr = rr.as_ref().borrow();
        let mut _map = &mut self.dns_map;
        match &_rr.data {
            ResourceType::A(ip) => {
                _map.insert(ip.to_string(), _rr.name());
            }
            ResourceType::AAAA(ip) => {
                _map.insert(ip.to_string(), _rr.name());
            }
            _ => {}
        }
        drop(_rr);
        self.dns.push(rr);
    }
    pub fn get_info(&self) -> FileInfo {
        self.info.clone()
    }
    pub fn get_dns_count(&self) -> usize {
        self.dns.len()
    }
    pub fn conversations(&self) -> &HashMap<String, RefCell<TCPConnection>> {
        &self.conversation_map
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

    fn update_tcp(&mut self, frame: &Frame, ip: &dyn IPPacket, packet: &mut TCP, reader: &Reader) {
        let (key, arch) = Context::tcp_key(ip, packet);
        let mut _map = &mut self.conversation_map;
        let v = _map.get(&key);
        let conn = match v {
            Some(conn) => conn,
            None => {
                let con = TCPConnection::new(ip, packet, arch);
                _map.insert(key.clone(), RefCell::new(con));
                _map.get(&key).unwrap()
            }
        };
        let mut reff = conn.borrow_mut();
        let _conn = reff.deref_mut();
        let (main, rev) = match arch {
            true => (&mut _conn.ep1, &mut _conn.ep2),
            false => (&mut _conn.ep2, &mut _conn.ep1),
        };
        let detail = main.update(packet, frame);
        let detail_copy = detail.clone();
        let _seq = main._seq;
        let next = main.next;
        rev.confirm(packet);
        let _ack = rev._ack;
        packet.info = Some(TCPInfo { next, _ack, _seq, detail });

        let tcp_len = packet.payload_len;
        let index = frame.summary.index;
        match detail_copy {
            TCPDetail::NONE | TCPDetail::KEEPALIVE => {
                //APPEND
                if tcp_len > 0 {
                    let lef = reader.slice(tcp_len as usize);
                    let data = lef.to_vec();
                    let list = frame.eles.clone();
                    main.add_segment(index, data, list);
                }
                main.update_segment();
            }
            TCPDetail::DUMP | TCPDetail::RETRANSMISSION => {
                //SKIP
            }
            TCPDetail::NOPREVCAPTURE | TCPDetail::RESET => {
                // RESET CACHE
                main.flush_segment();
            }
        }
    }

    pub fn tls_connection_info(&self) -> Vec<TLSHS> {
        let _c_list = self.conversations();
        let _clist = _c_list.values();
        let mut list = Vec::new();
        for _c in _clist.into_iter() {
            let tcp = _c.borrow();
            let l1 = tcp.ep1.handshake.len();
            let l2 = tcp.ep2.handshake.len();
            if l1 > 0 || l2 > 0 {
                let mut rs = TLSHS { ..Default::default() };
                let mut incept = |ep1: &Endpoint, ep2: &Endpoint, hs: &HandshakeType| match hs {
                    HandshakeType::ClientHello(_hs) => {
                        let _ch: &HandshakeClientHello = _hs.as_ref();
                        rs.source = format!("{}:{}", ep1.host, ep1.port);
                        rs.target = format!("{}:{}", ep2.host, ep2.port);
                        if let Some(sname) = _ch.server_name() {
                            rs.server_name = sname;
                        }
                        rs.support_cipher = _ch.ciphers();
                        if let Some(versions) = _ch.versions() {
                            rs.support_version = versions;
                        }
                        if let Some(negotiation) = _ch.negotiation() {
                            rs.support_negotiation = negotiation;
                        }
                    }
                    HandshakeType::ServerHello(_hs) => {
                        let _ch: &HandshakeServerHello = _hs.as_ref();
                        rs.source = format!("{}:{}", ep2.host, ep2.port);
                        rs.target = format!("{}:{}", ep1.host, ep1.port);
                        rs.used_cipher = _ch.ciper_suite();

                        if let Some(versions) = _ch.versions() {
                            rs.used_version = versions.into();
                        }
                        if let Some(negotiation) = _ch.negotiation() {
                            rs.used_negotiation = negotiation;
                        }
                    }
                    _ => {}
                };
                if l1 > 0 {
                    for _hs in tcp.ep1.handshake.iter() {
                        incept(&tcp.ep1, &tcp.ep2, _hs);
                    }
                }
                if l2 > 0 {
                    for _hs in tcp.ep2.handshake.iter() {
                        incept(&tcp.ep2, &tcp.ep1, _hs);
                    }
                }
                list.push(rs);
            }
        }
        list
    }
    pub fn _to_hostnames(&self, ep: &Endpoint) -> (String, u16, String) {
        let host = ep.host.clone();
        let port = ep.port;
        let hostname = self.dns_map.get(&host).unwrap_or(&String::from("")).clone();
        (host, port, hostname)
    }
}
pub struct Instance {
    pub ctx: Context,
    pub frames: Vec<Frame>,
}
impl Instance {
    pub fn new(ftype: FileType) -> Instance {
        let ctx = Context {
            count: 1,
            dns: Vec::new(),
            info: FileInfo { file_type: ftype, ..Default::default() },
            http_list: Vec::new(),
            conversation_map: HashMap::new(),
            statistic: Statistic::new(),
            dns_map: HashMap::new(),
        };
        Instance { ctx, frames: Vec::new() }
    }
    pub fn create(&mut self, data: Vec<u8>, ts: u64, capture_size: u32, origin_size: u32) {
        let ctx = &mut self.ctx;
        let count = ctx.count;
        ctx.count += 1;
        let link_type = ctx.info.link_type;
        let mut f = Frame::new(data, ts, capture_size, origin_size, count, link_type);
        let reader = f.get_reader();
        let mut next = crate::specs::execute(link_type, &f, &reader);
        'ins: loop {
            let _result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| crate::specs::parse(&mut f, ctx, &reader, next)));
            match _result {
                Ok(rs) => match rs {
                    Ok(_rs) => match _rs {
                        Some((data, _next)) => {
                            f.add_element(ctx, data, &reader);
                            next = _next;
                        }
                        None => break 'ins,
                    },
                    Err(e) => {
                        error!("parse_frame_failed index:[{}] at {}", count, next);
                        error!("msg:[{}]", e.to_string());
                        let (ep, _) = crate::specs::error::ErrorVisitor.visit(&f, &reader, &next).unwrap();
                        f.add_element(ctx, ep, &reader);
                        // process::exit(0x0100);
                        break 'ins;
                    }
                },
                Err(_) => {
                    error!("parse_err: index[{}] at {}", count, next);
                    let (ep, _) = crate::specs::error::ErrorVisitor.visit(&f, &reader, &next).unwrap();
                    f.add_element(ctx, ep, &reader);
                    // process::exit(0x0100);
                    break 'ins;
                }
            }
        }
        self.frames.push(f);
    }
    pub fn context(&self) -> &Context {
        &self.ctx
    }
    pub fn get_frames(&self) -> &[Frame] {
        &self.frames
    }
    pub fn update_ts(&mut self, ts: u64) {
        let info = &mut self.ctx.info;
        if info.start_time > 0 {
            info.end_time = ts;
            return;
        }
        info.start_time = ts;
    }
    pub fn pcap_info(&self) -> PCAPInfo {
        let mut _info = PCAPInfo::new();
        let ctx = self.context();
        let info = ctx.info.borrow();
        _info.file_type = format!("{:?}", info.file_type);
        _info.end_time = info.end_time;
        _info.start_time = info.start_time;
        _info.frame_count = self.get_frames().len();
        _info.dns_count = ctx.get_dns_count();
        _info.tcp_count = ctx.conversations().len();
        _info.http_count = ctx.get_http().len();
        _info.tls_count = ctx.tls_connection_info().len();
        _info
    }
    pub fn statistic_frames(&self) -> Result<Lines> {
        let list = self.get_frames();
        if list.len() < 30 {
            bail!("no no no ");
        }
        let ctx = self.context();
        let info = &ctx.info;
        // println!("{:#x}", info.end_time);
        // println!("{:#x}", info.start_time);
        if info.start_time > info.end_time {
            bail!("time error");
        }
        let duration = info.end_time - info.start_time;
        let start = info.start_time;

        let zone = (duration / 25) + 1;
        let mut cur: u64 = list.first().unwrap().ts;
        let mut next = cur + zone;
        let mut t_list = Vec::new();
        let mut protos: HashMap<String, u32> = HashMap::new();
        let mut y = HashSet::new();
        let mut x = Vec::new();
        // let mut counter:usize = 1;
        x.push("0".into());
        let total = "total";
        y.insert(total.into());
        for f in list.iter() {
            let _ts = f.ts;
            if _ts > next {
                cur = _ts;
                next = cur + zone;
                t_list.push(protos);
                // counter += 1;
                x.push(ts_to_str(_ts - start));
                protos = HashMap::new();
            }
            let protocol = f.get_protocol();
            let mount = f.capture_size;
            _insert_map(&mut protos, "total".into(), mount);
            y.insert(protocol.clone());
            _insert_map(&mut protos, protocol.clone(), mount);
        }
        t_list.push(protos);

        let mut _data = Vec::new();
        for pro in y.iter() {
            let mut data = Vec::new();
            for it in t_list.iter() {
                let mount = *it.get(pro).unwrap_or(&0);
                data.push(mount);
            }
            _data.push(LineData::new(pro.clone(), data));
        }
        Ok(Lines::new(x, y, _data))
    }
}

fn _insert_map(protos: &mut HashMap<String, u32>, protocol: String, mount: u32) {
    let _mount = *protos.get(protocol.as_str()).unwrap_or(&0);
    protos.insert(protocol, _mount + mount);
}
fn ts_to_str(ts: u64) -> String {
    if ts < 10000 {
        return format!("{} micSec", ts);
    }
    if ts < 1000000 {
        return format!("{}.{} MS", ts / 1000, ts % 1000);
    }
    let _sec = ts / 1000000;
    if _sec < 1000 {
        return format!("{}.{} Sec", _sec, (ts / 1000) % 1000);
    }
    let _min = _sec / 60;
    if _min < 1000 {
        return format!("{}.{} Min", _min, _sec % 60);
    }
    format!("{}.{} H", _min / 60, _min % 60)
}
