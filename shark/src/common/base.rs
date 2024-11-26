use crate::{
    common::{
        concept::{HttpRequestBuilder, LineData, Lines, PCAPInfo, Statistic, TCPConnectInfo},
        io::AReader,
        IPPacket, MultiBlock, PortPacket, Ref2, FIELDSTATUS,
    },
    constants::link_type_mapper,
    specs::{
        dns::{RecordResource, ResourceType, DNS},
        error::ErrorVisitor,
        http::{self, HTTPVisitor, HttpType, HTTP},
        tcp::{ACK, FIN, RESET, SYNC, TCP},
        tls::{
            handshake::{HandshakeClientHello, HandshakeServerHello, HandshakeType},
            TLSRecorMessage, TLSVisitor, TLS,
        },
        ProtocolData,
    },
};
use enum_dispatch::enum_dispatch;
use log::error;
use serde_json::Error;
use std::{
    borrow::Borrow,
    cell::RefCell,
    cmp,
    collections::{HashMap, HashSet, VecDeque},
    fmt::{Binary, Formatter},
    net::{Ipv4Addr, Ipv6Addr},
    ops::{BitAnd, Deref, DerefMut},
    rc::Rc,
};

use anyhow::{bail, Result};
// pub mod pcapng;
use crate::common::io::Reader;
use crate::common::{FileInfo, FileType};

use super::{
    concept::{Connect, Criteria, DNSRecord, Field, FrameInfo, HttpMessage, ListResult, TCPConversation, TLSHS},
    filter::PacketProps,
    io::SliceReader,
    util::date_str,
};

#[enum_dispatch(ProtocolData)]
pub trait Element {
    fn summary(&self) -> String;
    fn get_fields(&self) -> Vec<Field>;
    fn status(&self) -> FIELDSTATUS;
    fn info(&self) -> String;
    fn props(&self) -> &Option<RefCell<PacketProps>>;
}

pub trait Visitor {
    fn visit(&self, frame: &mut Frame, ctx: &mut Context, reader: &Reader) -> Result<(ProtocolData, &'static str)>;
}

pub trait FieldBuilder<T> {
    fn build(&self, t: &T) -> Option<Field>;
    fn get_props(&self) -> Option<(&'static str, &'static str)>;
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
    props: Option<RefCell<PacketProps>>,
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
            if let Some(_t) = pos.build(t) {
                rs.push(_t);
            }
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

    fn props(&self) -> &Option<RefCell<PacketProps>> {
        &self.props
    }
}
impl<T> PacketContext<T>
where
    T: PacketBuilder + 'static,
{
    pub fn set(&self, key: &'static str, val: String) {
        if let Some(props) = &self.props {
            let mut _props = props.borrow_mut();
            _props.add(key, val.leak());
        }
    }
    pub fn _build(&self, reader: &Reader, start: usize, size: usize, props: Option<(&'static str, &'static str)>, content: String) {
        self.fields.borrow_mut().push(Box::new(TXTPosition { start, size, data: reader.get_raw(), content, props }));
    }
    pub fn build_txt(&self, content: String) {
        self.fields.borrow_mut().push(Box::new(TXTPosition {
            start: 0,
            size: 0,
            data: Rc::new(Vec::new()),
            content,
            props: None,
        }));
    }

    pub fn _build_lazy(&self, reader: &Reader, start: usize, size: usize, props: Option<(&'static str, &'static str)>, render: fn(&T) -> String) {
        self.fields.borrow_mut().push(Box::new(StringPosition { start, size, data: reader.get_raw(), render, props }));
    }
    pub fn build_packet_no_position_lazy<K: 'static>(&self, render: fn(&T) -> Option<PacketContext<K>>)
    where
        K: PacketBuilder,
    {
        self.fields.borrow_mut().push(Box::new(PhantomBuilder {
            start: 0,
            size: 0,
            data: Rc::new(Vec::new()),
            render,
            props: None,
        }));
    }
    pub fn build_skip(&self, reader: &Reader, size: usize) {
        let start = reader.cursor();
        let content = format!("resolve later [{}]", size);
        reader.slice(size);
        self._build(reader, start, size, None, content);
    }

    pub fn build_lazy<K>(&self, reader: &Reader, opt: impl FnOnce(&Reader) -> Result<K>, key: Option<&'static str>, render: fn(&T) -> String) -> Result<K>
    where
        K: ToString,
    {
        let start = reader.cursor();
        let val: K = opt(reader)?;
        let end = reader.cursor();
        let size = end - start;
        let mut props: Option<(&str, &str)> = None;
        if let Some(k) = key {
            let v = val.to_string();
            self.set(k, v.clone());
            props = Some((k, v.leak()));
        }
        self._build_lazy(reader, start, size, props, render);
        Ok(val)
    }
    pub fn build_compact(&self, content: String, data: Rc<Vec<u8>>) {
        let size = data.len();
        self.fields.borrow_mut().push(Box::new(TXTPosition { start: 0, size, data, content, props: None }));
    }
    pub fn append_string(&self, content: String, data: Rc<Vec<u8>>) {
        self.fields.borrow_mut().push(Box::new(TXTPosition { start: 0, size: 0, data, content, props: None }));
    }
    pub fn build<K>(&self, reader: &Reader, opt: impl FnOnce(&Reader) -> K, key: Option<&'static str>, content: String) -> K
    where
        K: ToString,
    {
        let start = reader.cursor();
        let val: K = opt(reader);
        let end = reader.cursor();
        let size = end - start;
        let mut props: Option<(&str, &str)> = None;
        if let Some(k) = key {
            let v = val.to_string();
            self.set(k, v.clone());
            props = Some((k, v.leak()));
        }
        self._build(reader, start, size, props, content);
        val
    }

    pub fn build_backward(&self, reader: &Reader, step: usize, content: String) {
        let cur = reader.cursor();
        if cur < step {
            return;
        }
        let from = cur - step;
        self._build(reader, from, step, None, content);
    }

    pub fn build_format<K>(&self, reader: &Reader, opt: impl FnOnce(&Reader) -> Result<K>, key: Option<&'static str>, tmp: &str) -> Result<K>
    where
        K: ToString,
    {
        let start = reader.cursor();
        let val: K = opt(reader)?;

        let end = reader.cursor();
        let size = end - start;
        let content = tmp.replace("{}", val.to_string().as_str());
        let mut props: Option<(&str, &str)> = None;
        if let Some(k) = key {
            let v = val.to_string();
            self.set(k, v.clone());
            props = Some((k, v.leak()));
        }
        self._build(reader, start, size, props, content);
        Ok(val)
    }

    pub fn build_fn<K>(&self, reader: &Reader, opt: impl FnOnce(&Reader) -> Result<K>, key: Option<&'static str>, mapper: impl Fn(K) -> String) -> Result<K>
    where
        K: Clone + ToString,
    {
        let start = reader.cursor();
        let val: K = opt(reader)?;
        let end = reader.cursor();
        let size = end - start;
        let mut props: Option<(&str, &str)> = None;
        if let Some(k) = key {
            let v = val.to_string();
            self.set(k, v.clone());
            props = Some((k, v.leak()));
        }
        let content = mapper(val.clone());
        self.fields.borrow_mut().push(Box::new(TXTPosition { start, size, data: reader.get_raw(), content, props }));
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

        if let Some(props_self) = &self.props {
            let mut props = props_self.borrow_mut();
            if let Some(prop) = &packet.props {
                let mut _props = prop.borrow_mut();
                props.merge(_props.deref_mut());
                drop(_props);
            }
            drop(props);
        }
        self.fields.borrow_mut().push(Box::new(FieldPosition {
            start,
            size,
            data: reader.get_raw(),
            head,
            packet,
            props: None,
        }));
        Ok(rs)
    }
    pub fn build_packet_lazy<M, K: 'static>(&self, reader: &Reader, opt: impl FnOnce(&Reader) -> Result<M>, _props: Option<&'static str>, render: fn(&T) -> Option<PacketContext<K>>) -> Result<M>
    where
        K: PacketBuilder,
    {
        let start = reader.cursor();
        let val: M = opt(reader)?;
        let end = reader.cursor();
        let size = end - start;
        let data = reader.get_raw().clone();
        self.fields.borrow_mut().push(Box::new(PhantomBuilder { start, size, data, render, props: None }));
        Ok(val)
    }
}

pub struct PhantomBuilder<K, T> {
    pub render: fn(&T) -> Option<PacketContext<K>>,
    props: Option<(&'static str, &'static str)>,
    pub start: usize,
    pub size: usize,
    data: Rc<Vec<u8>>,
}
impl<K, T> FieldBuilder<T> for PhantomBuilder<K, T>
where
    K: PacketBuilder,
{
    fn build(&self, t: &T) -> Option<Field> {
        let _packet = (self.render)(t);
        if let Some(packet) = _packet {
            let sum = packet.get().borrow().summary();
            let mut field = Field::new(self.start, self.size, self.data.clone(), sum);
            let fields = packet.get_fields();
            field.children = fields;
            return Some(field);
        }
        None
    }

    fn data(&self) -> Rc<Vec<u8>> {
        self.data.clone()
    }
    fn get_props(&self) -> Option<(&'static str, &'static str)> {
        self.props.clone()
    }
}
pub struct Position<T> {
    pub start: usize,
    pub size: usize,
    data: Rc<Vec<u8>>,
    props: Option<(&'static str, &'static str)>,
    pub render: fn(usize, usize, &T) -> Field,
}
impl<T> FieldBuilder<T> for Position<T> {
    fn build(&self, t: &T) -> Option<Field> {
        Some((self.render)(self.start, self.size, t))
    }

    fn data(&self) -> Rc<Vec<u8>> {
        self.data.clone()
    }
    fn get_props(&self) -> Option<(&'static str, &'static str)> {
        self.props.clone()
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
    props: Option<(&'static str, &'static str)>,
    pub packet: PacketContext<T>,
}
impl<T, K> FieldBuilder<T> for FieldPosition<K>
where
    K: PacketBuilder,
{
    fn build(&self, _: &T) -> Option<Field> {
        let summary = match self.head.clone() {
            Some(t) => t,
            _ => self.packet.get().borrow().summary(),
        };
        let mut field = Field::new(self.start, self.size, self.data.clone(), summary);
        let fields = self.packet.get_fields();
        field.children = fields;
        Some(field)
    }

    fn data(&self) -> Rc<Vec<u8>> {
        self.data.clone()
    }
    fn get_props(&self) -> Option<(&'static str, &'static str)> {
        self.props.clone()
    }
}

pub struct StringPosition<T> {
    pub start: usize,
    pub size: usize,
    data: Rc<Vec<u8>>,
    pub render: fn(&T) -> String,
    props: Option<(&'static str, &'static str)>,
}
impl<T> FieldBuilder<T> for StringPosition<T> {
    fn build(&self, t: &T) -> Option<Field> {
        let summary = (self.render)(t);
        Some(Field::new(self.start, self.size, self.data.clone(), summary))
    }

    fn data(&self) -> Rc<Vec<u8>> {
        self.data.clone()
    }
    fn get_props(&self) -> Option<(&'static str, &'static str)> {
        self.props.clone()
    }
}

pub struct TXTPosition {
    start: usize,
    size: usize,
    data: Rc<Vec<u8>>,
    content: String,
    props: Option<(&'static str, &'static str)>,
}
impl<T> FieldBuilder<T> for TXTPosition {
    fn build(&self, _: &T) -> Option<Field> {
        Some(Field::new(self.start, self.size, self.data.clone(), self.content.clone()))
    }
    fn data(&self) -> Rc<Vec<u8>> {
        self.data.clone()
    }

    fn get_props(&self) -> Option<(&'static str, &'static str)> {
        self.props.clone()
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
    NONE,
}

pub struct Segment {
    pub frame_refer: Ref2<FrameRefer>,
    pub size: usize,
}

#[derive(Default)]
pub enum TCPPAYLOAD {
    #[default]
    NONE,
    TLS(usize),
    HTTPLEN(Ref2<crate::specs::http::HTTP>),
    HTTPCHUNKED(Ref2<crate::specs::http::HTTP>),
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
    pub http_messages: Vec<(u64, Ref2<crate::specs::http::HTTP>)>,

    pub _segments: Option<VecDeque<Segment>>,
    pub _buffer: Vec<u8>,

    pub connec_type: TCPPAYLOAD,
}

impl Endpoint {
    fn new(host: String, port: u16) -> Self {
        Self { host, port, ..Default::default() }
    }
    fn add_packet(&mut self, rs: Result<ProtocolData>, frame_refer: Option<Ref2<FrameRefer>>, segments: Vec<TCPSegment>) -> Option<Ref2<TCPSegments>> {
        if let Some(_ref) = frame_refer {
            let mut last_refer = _ref.as_ref().borrow_mut();
            let ts = last_refer.ts;

            if let Ok(result) = rs {
                let mut _type = "";
                if let ProtocolData::TLS(pcaket) = &result {
                    let tls = pcaket.get().borrow();
                    self.add_tls(tls.deref());
                } else if let ProtocolData::HTTP(packet) = &result {
                    self.add_http(packet._clone_obj(), ts);
                    _type = "HTTP";
                }
                last_refer._app_packet = Some(result);
                let seg = TCPSegments { items: segments, _type };
                return Some(Rc::new(RefCell::new(seg)));
            } else {
                // println!("")
            }
            drop(last_refer);
        }
        None
    }
    fn shift_cache(&mut self, _size: Option<usize>, rs: Result<ProtocolData>) {
        let mut _index = _size.unwrap_or(usize::max_value());
        let mut last_one = None;
        let mut _ts: u64 = 0;
        let mut list_to_list = Vec::new();
        let mut index_list = Vec::new();
        if let Some(segments) = &mut self._segments {
            loop {
                if let Some(seg) = segments.pop_front() {
                    let Segment { size, frame_refer } = seg;
                    let f = frame_refer.as_ref().borrow();
                    let ts = f.ts;
                    let index = f.index;
                    _ts = ts;
                    if size <= _index {
                        _index -= size;
                        index_list.push(TCPSegment { index, size });
                        if segments.len() == 0 {
                            last_one = Some(frame_refer.clone());
                            break;
                        } else {
                            list_to_list.push(frame_refer.clone());
                        }
                    } else {
                        last_one = Some(frame_refer.clone());
                        index_list.push(TCPSegment { index, size: _index });
                        segments.push_front(Segment {
                            size: (size - _index),
                            frame_refer: frame_refer.clone(),
                        });
                        _index = 0;
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        let segments = self.add_packet(rs, last_one.clone(), index_list);
        for pre_refer in list_to_list.iter() {
            let mut reff = pre_refer.as_ref().borrow_mut();
            reff.segments = segments.clone();
            drop(reff);
        }
    }
    pub fn clear(&mut self) {
        self._segments = Some(VecDeque::new());
        self._buffer = Vec::new();
    }
    fn shift(&mut self, size: usize) -> Vec<u8> {
        let mut _buffer = Vec::new();
        _buffer.append(&mut self._buffer);
        if size < self._buffer.len() {
            self._buffer = _buffer[size..].to_vec();
            _buffer[..size].to_vec()
        } else {
            self._buffer = Vec::new();
            _buffer
        }
    }
    pub fn add_segment(&mut self, data: Vec<u8>, frame_refer: Ref2<FrameRefer>) {
        let segment = Segment { frame_refer, size: data.len() };
        let mut _data = data;
        self._buffer.append(&mut _data);
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
                let reader = SliceReader::new(&self._buffer);
                if !HTTPVisitor::check(&reader) {
                    drop(reader);
                    self.clear();
                    return;
                }
                if let Ok(http) = http::parse(&reader) {
                    let len = http.len;
                    let is_chunked = http.chunked;
                    let exist = reader.left();
                    let reff = Rc::new(RefCell::new(http));
                    if len > 0 {
                        self.connec_type = TCPPAYLOAD::HTTPLEN(reff);
                        self._buffer = reader.slice(exist).to_vec();
                        return self.update_segment();
                    } else if is_chunked {
                        self.connec_type = TCPPAYLOAD::HTTPCHUNKED(reff);
                        self._buffer = reader.slice(exist).to_vec();
                        return self.update_segment();
                    } else {
                        let size = reader.cursor();

                        let rs = http::no_content(reff.clone());

                        self._buffer = reader.slice(exist).to_vec();
                        self.shift_cache(Some(size), rs);
                        self.clear();
                        // parse
                        // clearcache
                    }
                }
            }
            TCPPAYLOAD::HTTPLEN(http) => {
                let len = http.as_ref().borrow().len;
                let clen = self._buffer.len();
                if clen > len {
                    let cloned = http.clone();
                    let body = self.shift(len);
                    let rs = http::content_len(cloned, body);
                    self.shift_cache(Some(len), rs);
                } else if clen < len {
                    //pass
                } else {
                    let cloned = http.clone();
                    let body = self.shift(len);
                    let rs = http::content_len(cloned, body);
                    self.shift_cache(None, rs);
                    self.clear();
                    self.connec_type = TCPPAYLOAD::HTTPPRE;
                }
            }
            TCPPAYLOAD::HTTPCHUNKED(_http) => {
                //https://stackoverflow.com/questions/16460012/how-to-get-the-size-of-chunk-in-http-response-using-java-if-transfer-encoding-is
                let _reader = SliceReader::new(&self._buffer);
                // let mut data = Vec::new();
                let mut complete = false;
                loop {
                    if let Ok(line) = _reader.try_read_enter(30) {
                        if let Ok(size) = usize::from_str_radix(&line, 16) {
                            if size == 0 {
                                complete = true;
                                break;
                            }
                            if !_reader._move(size + 2) {
                                break;
                            }
                        } else {
                            // TODO CLEAR
                            break;
                        }
                    } else {
                        break;
                    }
                }
                drop(_reader);
                if complete {
                    let reader2 = SliceReader::new(&self._buffer);
                    let mut data = Vec::new();
                    loop {
                        if let Ok(line) = reader2.try_read_enter(30) {
                            if let Ok(size) = usize::from_str_radix(&line, 16) {
                                if size == 0 {
                                    reader2._move(2);
                                    reader2._move(2);
                                    let _size = reader2.cursor();
                                    self._buffer.drain(0.._size);
                                    let rs = http::content_len(_http.clone(), data);
                                    self.shift_cache(None, rs);
                                    self.clear();
                                    self.connec_type = TCPPAYLOAD::HTTPPRE;
                                    break;
                                }
                                data.append(&mut reader2.slice(size).to_vec());
                                reader2._move(2);
                            } else {
                                // TODO CLEAR
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
            TCPPAYLOAD::TLS(next_size) => {
                let size = *next_size;
                let clen = self._buffer.len();
                if size > clen {
                } else if size == clen {
                    // remove all
                    let data: Vec<u8> = self._buffer.drain(..).collect();
                    let reader = Reader::new_raw(Rc::new(data));
                    let _rs = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| TLSVisitor.visit(&reader)));
                    if let Ok(rs) = _rs {
                        self.shift_cache(None, rs);
                    } else {
                        let _rs = ErrorVisitor.visit2(&reader, "error");
                        self.shift_cache(None, _rs);
                    }
                    self._segments = None;
                    self._buffer = Vec::new();
                    self.connec_type = TCPPAYLOAD::NONE;
                } else {
                    let mount: Vec<u8> = self._buffer.drain(0..size).collect();
                    let data_ref = Rc::new(mount);
                    let reader = Reader::new_raw(data_ref.clone());
                    let _rs = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| TLSVisitor.visit(&reader)));
                    if let Ok(rs) = _rs {
                        self.connec_type = TCPPAYLOAD::NONE;
                        self.shift_cache(Some(size), rs);
                        self.update_segment();
                    } else {
                        self.shift_cache(Some(size), ErrorVisitor.visit2(&reader, "error"));
                    }
                }
            }
            TCPPAYLOAD::NONE => {
                let cache_len = self._buffer.len();
                if cache_len > 5 {
                    let (is_tls, len) = TLS::_check(&self._buffer).unwrap();
                    if is_tls {
                        self.connec_type = TCPPAYLOAD::TLS(len + 5);
                        return self.update_segment();
                    }
                }
                let reader = SliceReader::new(&self._buffer);
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
        self._buffer = Vec::new();
        self.connec_type = TCPPAYLOAD::NONE;
    }
    fn clear_segment(&mut self) {}

    fn update(&mut self, tcp: &TCP, _: &Frame) -> TCPDetail {
        //https://www.wireshark.org/docs/wsug_html_chunked/ChAdvTCPAnalysis.html
        let sequence = tcp.sequence;
        let info = &mut self.info;
        info.count = info.count + 1;
        info.throughput += tcp.payload_len as u32;

        if self.seq == sequence && tcp.payload_len == 0 {
            return TCPDetail::NONE;
        }
        if tcp.state._match(RESET) {
            self.clear_segment();
            return TCPDetail::RESET;
        }
        let mut _tcp_len = 0;
        if tcp.state._match(SYNC) {
            _tcp_len = 1;
        } else if tcp.state._match(FIN) {
            _tcp_len = 1;
        } else {
            _tcp_len = tcp.payload_len as u32;
        }
        if self.seq == 0 {
            self._seq = sequence;
            self.seq = sequence;
            self.next = sequence + _tcp_len;
            self._checksum = tcp.crc;
            return TCPDetail::NONE;
        }
        if sequence > self.next {
            self.seq = sequence;
            self.next = sequence + _tcp_len;
            self._checksum = tcp.crc;
            info.invalid += 1;
            self.clear_segment();
            return TCPDetail::NOPREVCAPTURE;
        } else if sequence == self.next {
            self.seq = tcp.sequence;
            self._checksum = tcp.crc;
            if _tcp_len == 0 {
                // if tcp.state.check(ACK) {
                //     return TCPDetail::KEEPALIVE;
                // }
                return TCPDetail::NONE;
            }
            self.next = tcp.sequence + _tcp_len;
            return TCPDetail::NONE;
        } else {
            if sequence == self.next - 1 && (_tcp_len == 1 || _tcp_len == 0) && tcp.state.check(ACK) {
                self._checksum = tcp.crc;
                return TCPDetail::KEEPALIVE;
            }
            if self.seq == sequence + _tcp_len {
                info.retransmission += 1;
                return TCPDetail::RETRANSMISSION;
            }
            info.invalid += 1;
            return TCPDetail::DUMP;
        }
    }
    pub fn stringfy(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    fn add_tls(&mut self, tls: &TLS) {
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
    fn add_http(&mut self, http: Ref2<crate::specs::http::HTTP>, ts: u64) {
        self.http_messages.push((ts, http));
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
pub struct TCPSegment {
    pub index: u32,
    pub size: usize,
}

pub struct TCPSegments {
    pub items: Vec<TCPSegment>,
    pub _type: &'static str,
}

impl PacketBuilder for TCPSegments {
    fn new() -> Self {
        TCPSegments { items: Vec::new(), _type: "" }
    }

    fn summary(&self) -> String {
        let mut mount = 0;
        for s in self.items.iter() {
            mount += s.size;
        }
        format!("[{} Reassembled TCP Segments({}) ({} bytes)]", self.items.len(), self._type, mount)
    }
}

#[derive(Default)]
pub struct FrameRefer {
    index: u32,
    ts: u64,
    pub _app_packet: Option<ProtocolData>,
    pub segments: Option<Ref2<TCPSegments>>,
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
    pub eles: Vec<ProtocolData>,
    pub refer: Ref2<FrameRefer>,
    pub props: PacketProps,
}
impl Frame {
    pub fn new(data: Vec<u8>, ts: u64, capture_size: u32, origin_size: u32, index: u32, link_type: u32) -> Frame {
        let f = Frame {
            props: PacketProps::new(),
            eles: Vec::new(),
            refer: Rc::new(RefCell::new(FrameRefer { index, ts, ..Default::default() })),
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
    fn set_protocol(&mut self, pro: String) {
        let mref = &mut self.summary;
        mref.protocol = format!("{}", pro);
    }
    pub fn do_match(&self, statement: &[&str]) -> bool {
        // return self.props.match_expr(statement);
        for state in statement {
            if *state == &self.get_protocol() {
                return true;
            }
        }
        false
        // return &self.get_protocol() == statement;
    }
    pub fn info(&self) -> String {
        let the_last = self.eles.last();
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
    fn update_tcp(&mut self, packet: &mut TCP, ctx: &mut Context, reader: &Reader) {
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
        for e in self.eles.iter() {
            let vs = e.get_fields();
            rs.push(Field::new2(e.summary(), self.data.clone(), vs));
        }
        rs
    }
    pub fn get_fields_json(&self) -> core::result::Result<String, Error> {
        let fields = self.get_fields();
        serde_json::to_string(&fields)
    }
    fn data(&self) -> Rc<Vec<u8>> {
        self.data.clone()
    }
    pub fn get_reader(&self) -> Reader {
        return Reader::new_raw(self.data());
    }
    pub fn _create_packet<K>(val: Ref2<K>) -> PacketContext<K> {
        PacketContext { props: None, val, fields: RefCell::new(Vec::new()) }
    }
    pub fn create_packet<K>() -> PacketContext<K>
    where
        K: PacketBuilder,
    {
        let val = K::new();
        Frame::_create_packet(Rc::new(RefCell::new(val)))
    }
    pub fn create_packet_with_props<K>() -> PacketContext<K>
    where
        K: PacketBuilder,
    {
        let val = K::new();
        Frame::_create_with_props(Rc::new(RefCell::new(val)))
    }
    pub fn _create<K>(val: Ref2<K>) -> PacketContext<K> {
        PacketContext { props: None, val, fields: RefCell::new(Vec::new()) }
    }
    pub fn _create_with_props<K>(val: Ref2<K>) -> PacketContext<K> {
        PacketContext {
            props: Some(RefCell::new(PacketProps::new())),
            val,
            fields: RefCell::new(Vec::new()),
        }
    }
    
    fn ipv4_collect(_ip: &Option<Ipv4Addr>, ctx: &mut Context) {
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
    fn ipv6_collect(_ip: &Option<Ipv6Addr>, ctx: &mut Context) {
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
                Frame::ipv4_collect(&ip.source_ip, ctx);
                Frame::ipv4_collect(&ip.target_ip, ctx);
                drop(ip);
            }
            ProtocolData::IPV6(packet) => {
                self.update_ip(ctx, packet._clone_obj());
                let ip = packet.get().borrow();
                Frame::ipv6_collect(&ip.source_ip, ctx);
                Frame::ipv6_collect(&ip.target_ip, ctx);
                drop(ip);
            }
            ProtocolData::ARP(packet) => {
                self.update_ip(ctx, packet._clone_obj());
            }
            ProtocolData::TCP(packet) => {
                let tcp = packet.get();
                self.update_tcp(tcp.borrow_mut().deref_mut(), ctx, reader);
            }
            ProtocolData::DNS(packet) => {
                self.add_dns(packet._clone_obj(), ctx);
            }
            _ => {}
        }
        let reff = &mut self.eles;
        let mut append_prop = |data: ProtocolData| {
            if let Some(_cell) = data.props() {
                let mut _props = _cell.borrow_mut();
                self.props.merge(_props.deref_mut());
                drop(_props);
            }
            reff.push(data);
        };
        append_prop(ele);
        let mut ref_ = self.refer.as_ref().borrow_mut();
        if let Some(app_) = ref_._app_packet.take() {
            append_prop(app_);
        }
        drop(ref_);
        let protocol = format!("{}", reff.last().unwrap());
        self.props.add(protocol.clone().to_lowercase().leak(), "");
        self.set_protocol(protocol);
    }
}

pub struct Context {
    pub count: u32,
    pub cost: usize,
    pub info: FileInfo,
    pub dns: Vec<DNSRecord>,
    pub conversation_map: HashMap<String, RefCell<TCPConnection>>,
    http_list: Vec<Connect<HttpMessage>>,
    pub statistic: Statistic,
    pub dns_map: HashMap<String, String>,
}

fn _append_http_to(list: &mut Vec<HttpMessage>, mut messages: Vec<(u64, Ref2<HTTP>)>, ref_statis: &mut Statistic) {
    loop {
        if let Some((ts, msg)) = messages.pop() {
            let _msg = msg.as_ref().borrow();
            match _msg._type() {
                HttpType::REQUEST(req) => {
                    ref_statis.http_method.inc(&req.method.clone());
                }
                HttpType::RESPONSE(res) => {
                    ref_statis.http_status.inc(&res.code);
                }
                _ => {}
            }
            if let Some(ct) = &_msg.content_type {
                ref_statis.http_type.inc(ct);
            }
            let msg = HttpMessage::new(ts / 1000, _msg.deref());
            list.push(msg);
            drop(_msg);
        } else {
            break;
        }
    }
}
impl Context {
    pub fn cost(&self) -> usize {
        self.cost
    }
    pub fn get_statistc(&self) -> &Statistic {
        &self.statistic
    }
    pub fn http_list_json(&self) -> String {
        serde_json::to_string(&self.http_list).unwrap()
    }
    pub fn http_content(&self, index: usize, ts: u64) -> Option<Rc<Vec<u8>>> {
        if let Some(conn) = self.http_list.get(index) {
            for msg in conn.list.iter() {
                if msg.ts == ts {
                    return msg.body.clone();
                }
            }
        }
        None
    }
    fn resolve_http(&mut self) {
        let list = &mut (self.http_list);
        let ref_statis: &mut Statistic = &mut self.statistic;
        list.clear();
        for con in (&mut self.conversation_map).values().into_iter() {
            let mut reff = con.borrow_mut();
            let (_s, _t) = reff.sort(ref_statis.ip.get_map());
            let source = _s.stringfy();
            let target = _t.stringfy();
            let index = list.len();
            let mut msg_: Connect<HttpMessage> = Connect { source, target, index, list: Vec::new() };
            let mut messages = Vec::new();
            messages.append(&mut reff.ep1.http_messages);
            _append_http_to(&mut msg_.list, messages, ref_statis);
            //
            messages = Vec::new();
            messages.append(&mut reff.ep2.http_messages);
            _append_http_to(&mut msg_.list, messages, ref_statis);

            if msg_.list.len() > 0 {
                list.push(msg_);
            }
            drop(reff);
        }
    }
    pub fn get_http(&self) -> &[Connect<HttpMessage>] {
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

    fn get_dns_record(&self) -> &[DNSRecord] {
        &self.dns
    }
    pub fn get_dns_record_json(&self) -> core::result::Result<String, Error> {
        serde_json::to_string(self.get_dns_record())
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
        let ins = DNSRecord::create(_rr.deref());
        drop(_rr);
        self.dns.push(ins);
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
    pub fn get_conversation_items(&self) -> Vec<TCPConversation> {
        let cons = self.conversations();
        let mut rs = Vec::new();
        for con in cons.values().into_iter() {
            let reff = con.borrow();
            let (source, target) = reff.sort(self.statistic.ip.get_map());
            let tcp = TCPConversation::new(source, target, self);
            rs.push(tcp);
            drop(reff);
        }
        rs
    }
    pub fn get_conversation_json(&self) -> core::result::Result<String, Error> {
        let items = self.get_conversation_items();
        serde_json::to_string(&items)
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

    fn update_tcp(&mut self, frame: &mut Frame, ip: &dyn IPPacket, packet: &mut TCP, reader: &Reader) {
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

        let tcp_len = packet.payload_len;
        let detail = main.update(packet, frame);
        let detail_copy = detail.clone();
        let _seq = main._seq;
        let next = main.next;
        rev.confirm(packet);
        let _ack = rev._ack;
        packet.info = Some(TCPInfo { next, _ack, _seq, detail });
        match detail_copy {
            TCPDetail::KEEPALIVE => {
                main.update_segment();
            }
            TCPDetail::NONE => {
                //APPEND
                if tcp_len > 0 {
                    let lef = reader.slice(tcp_len as usize);
                    let data = lef.to_vec();
                    main.add_segment(data, frame.refer.clone());
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

    pub fn tls_connection_infos(&self) -> Vec<TLSHS> {
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
    pub fn get_tls_connection_json(&self) -> core::result::Result<String, Error> {
        let items = self.tls_connection_infos();
        serde_json::to_string(&items)
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
            cost: 0,
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
    /// parse a frame from data, save the frame to `self.frames`
    ///
    /// The method will parse the frame from the data, and if the frame is incomplete,
    /// it will save the frame to `self.frames` and return the offset of the next frame.
    /// if the frame is complete, it will save the frame to `self.frames` and return 0.
    /// if the frame is corrupted, it will save the frame to `self.frames` and return 0.
    ///
    pub fn create(&mut self, data: Vec<u8>, ts: u64, capture_size: u32, origin_size: u32) {
        let ctx = &mut self.ctx;
        let file_type = &ctx.info.file_type;
        let count = ctx.count;
        ctx.count += 1;
        let link_type = ctx.info.link_type;
        let mut f = Frame::new(data, ts, capture_size, origin_size, count, link_type);
        let reader = f.get_reader();

        let mut next = crate::specs::execute(file_type, link_type, &f, &reader);
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
    pub fn flush(&mut self) {
        let ctx = &mut self.ctx;
        ctx.resolve_http();
    }
    pub fn context(&self) -> &Context {
        &self.ctx
    }
    pub fn get_frames(&self) -> &[Frame] {
        &self.frames
    }
    pub fn get_frames_by(&self, cri: Criteria) -> ListResult<FrameInfo> {
        let Criteria { start, size, criteria } = cri;
        let info = self.context().get_info();
        let start_ts = info.start_time;
        let _fs = self.get_frames();
        let mut total = 0;
        let mut items = Vec::new();
        let _criteria = criteria.trim();
        if _criteria.len() > 0 {
            let mut left = size;

            let clist: Vec<&str> = _criteria.split("&").collect();
            for frame in _fs.iter() {
                if frame.do_match(&clist) {
                    total += 1;
                    if total > start && left > 0 {
                        left -= 1;
                        let item = FrameInfo::new(frame, start_ts);
                        items.push(item);
                    }
                }
            }
            return ListResult::new(start, total, items);
        }
        total = _fs.len();
        if total <= start {
            return ListResult::new(start, 0, Vec::new());
        }
        let end = cmp::min(start + size, total);
        let _data = &_fs[start..end];
        for frame in _data.iter() {
            let item = FrameInfo::new(frame, start_ts);
            items.push(item);
        }
        ListResult::new(start, total, items)
    }

    pub fn get_frames_json(&self, cri: Criteria) -> core::result::Result<String, Error> {
        let item = self.get_frames_by(cri);
        serde_json::to_string(&item)
    }
    pub fn update_ts(&mut self, ts: u64) {
        let info = &mut self.ctx.info;
        if info.start_time > 0 {
            info.end_time = ts;
            return;
        }
        info.start_time = ts;
    }
    /// Returns a PCAPInfo object that contains some information of the capture file.
    ///
    /// The returned object contains the following fields:
    ///
    /// - `file_type`: the type of capture file, such as "PCAP" or "PCAPNG".
    /// - `start_time`: the start time of the capture file in seconds.
    /// - `end_time`: the end time of the capture file in seconds.
    /// - `frame_count`: the total number of frames in the capture file.
    /// - `dns_count`: the number of DNS packets in the capture file.
    /// - `tcp_count`: the number of TCP conversations in the capture file.
    /// - `http_count`: the number of HTTP requests in the capture file.
    /// - `tls_count`: the number of TLS connections in the capture file.
    /// - `cost`: the total time spent on parsing the capture file in milliseconds.
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
        _info.tls_count = ctx.tls_connection_infos().len();
        _info.cost = ctx.cost();
        _info
    }
    /// Generate a line chart data for frames per second
    ///
    /// # Errors
    ///
    /// If the frames count is less than 10, it will return an error
    ///
    /// # Example
    ///
    ///
    pub fn statistic_frames(&self) -> Result<Lines> {
        let list = self.get_frames();
        if list.len() < 10 {
            bail!("no no no ");
        }
        let ctx = self.context();
        let info = &ctx.info;
        if info.start_time > info.end_time {
            return Ok(Lines::empty());
        }
        let duration = info.end_time - info.start_time;
        let start = info.start_time;

        let zone = (duration / 20) + 1;
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

#[derive(Clone)]
pub enum BitType<T> {
    ABSENT(&'static str, &'static str),
    ONEoF(Vec<(T, &'static str)>),
    VAL(&'static str, usize, T),
}

pub trait FlagData<T>
where
    T: Binary + Copy + BitAnd + PartialEq<<T as BitAnd>::Output>,
    <T as BitAnd>::Output: PartialEq<T>,
{
    fn bits(inx: usize) -> Option<(T, BitType<T>)>;
    // fn to_desc(index:usize, buffer: &mut String, word: &str, status: bool);
    fn summary(title: &mut String, value: T);
    fn summary_ext(title: &mut String, desc: &str, status: bool);
}
#[derive(Default)]
pub struct BitFlag<T>
where
    T: Binary + Copy + BitAnd + PartialEq<<T as BitAnd>::Output>,
    <T as BitAnd>::Output: PartialEq<T>,
{
    value: T,
    content: String,
}

impl<T> std::fmt::Display for BitFlag<T>
where
    T: Binary + Copy + BitAnd + PartialEq<<T as BitAnd>::Output>,
    <T as BitAnd>::Output: PartialEq<T>,
{
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        fmt.write_str(self.content.as_str())
    }
}
impl<T> PacketBuilder for BitFlag<T>
where
    T: Default + Binary + Copy + BitAnd + PartialEq<<T as BitAnd>::Output>,
    <T as BitAnd>::Output: PartialEq<T>,
{
    fn new() -> Self {
        Self { ..Default::default() }
    }
    fn summary(&self) -> String {
        self.to_string()
    }
}
impl<T> BitFlag<T>
where
    T: Default + Binary + Copy + std::fmt::Display + BitAnd<Output = T> + PartialEq<<T as BitAnd>::Output> + 'static + std::ops::Shr<usize, Output = T> + std::ops::Shl<usize, Output = T>,
    <T as BitAnd>::Output: PartialEq<T>,
{
    pub fn make<F>(value: T) -> Option<PacketContext<Self>>
    where
        F: FlagData<T>,
    {
        let packet: PacketContext<Self> = Frame::create_packet();
        let mut p = packet.get().borrow_mut();
        p.value = value;
        F::summary(&mut p.content, value);
        let n = size_of_val(&value) * 8;
        for inx in 0..n {
            if let Some(info) = F::bits(inx) {
                let mask = info.0;
                match &info.1 {
                    BitType::ABSENT(succ, failed) => {
                        let (line, status) = BitFlag::print_line(mask, value);
                        // F::summary_ext(&mut p.content, *desc, status);
                        let mut _content = format!("{} = ", line);
                        if status {
                            _content.push_str(&succ);
                        } else {
                            _content.push_str(&failed);
                        }
                        packet.build_txt(_content);
                    }
                    BitType::ONEoF(list) => {
                        let _val = value & mask;
                        for _cur in list.iter() {
                            if _cur.0 == _val {
                                let line = BitFlag::print_line_match(mask, _val);
                                let mut _content = format!("{} = {}", line, _cur.1);
                                packet.build_txt(_content);
                                break;
                            }
                        }
                    }
                    BitType::VAL(prefix, offset, wid) => {
                        let val = (value >> *offset) & *wid;
                        let mask = *wid << *offset;
                        let line = BitFlag::print_line_match(mask, value);
                        let mut _content = format!("{} = {}: {}", line, *prefix, val);
                        packet.build_txt(_content);
                    }
                }
            }
        }
        drop(p);
        Some(packet)
    }
    // fn print_line_off(value: T, mask: T) -> String where T:Binary + Copy + BitAnd, <T as BitAnd>::Output: PartialEq<T> {
    //     let len = size_of_val(&mask) * 8;
    //     let str = format!("{:0len$b}", mask);
    //     let tar = format!("{:0len$b}", value);
    //     let mut str2 = str.replace("0", ".");
    //     // str2 = str2.replace("1", "0");
    //     // let mut chars2 = tar.chars();
    //     // for cur in 0..len {
    //     //     if '1' == chars2.nth(0).unwrap() {
    //     //         str2.replace_range(cur..cur+1, "1");
    //     //     }
    //     // }
    //     // for cur in 1..len/4 {
    //     //     str2.insert_str(len - cur * 4, " ");
    //     // }
    //     // str2
    // }
    fn print_line(mask: T, value: T) -> (String, bool)
    where
        T: Binary + Copy + BitAnd,
        <T as BitAnd>::Output: PartialEq<T>,
    {
        let len = size_of_val(&mask) * 8;
        let str = format!("{:0len$b}", mask);
        let mut str2 = str.replace("0", ".");
        for cur in 1..len / 4 {
            str2.insert_str(len - cur * 4, " ");
        }
        if mask & value != mask {
            return (str2.replace("1", "0"), false);
        }
        (str2, true)
    }
    fn print_line_match(mask: T, val: T) -> String
    where
        T: Binary + Copy + BitAnd + PartialEq<<T as BitAnd>::Output>,
        <T as BitAnd>::Output: PartialEq<T>,
    {
        let len = size_of_val(&mask) * 8;
        let str = format!("{:0len$b}", mask);
        let tar = format!("{:0len$b}", val);
        let mut str2 = str.replace("0", ".");
        str2 = str2.replace("1", "0");
        let mut chars2 = tar.chars();
        for cur in 0..len {
            if '1' == chars2.nth(0).unwrap() {
                str2.replace_range(cur..cur + 1, "1");
            }
        }
        for cur in 1..len / 4 {
            str2.insert_str(len - cur * 4, " ");
        }
        str2
    }
}
