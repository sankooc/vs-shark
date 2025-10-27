// Copyright (c) 2025 sankooc
//
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use std::{
    collections::HashMap,
    net::{Ipv4Addr, Ipv6Addr},
    ops::{AddAssign, Range},
};

use anyhow::{bail, Result};

use crate::common::{
    concept::{
        ConnectionIndex, Conversation, ConversationKey, CounterItem, FrameIndex, HttpConnectIndex, HttpMessageDetail, LineChartData, MessageIndex, Timestamp, VHttpConnection,
    },
    enum_def::{AddressField, Protocol},
    ResourceLoader,
};

use super::{
    connection::{ConnectState, Connection, Endpoint, TCPStat, TmpConnection},
    enum_def::FileType,
    io::DataSource,
    quick_hash, EthernetCache, FastHashMap, Frame, NString,
};

pub struct Segment {
    pub index: FrameIndex,
    pub range: Range<usize>,
}

#[derive(Default)]
pub enum SegmentData {
    #[default]
    None,
    Single(Segment),
    Multiple(Vec<Segment>),
}

impl SegmentData {
    pub fn to_vec(&self) -> Vec<(usize, usize)> {
        match self {
            SegmentData::None => vec![],
            SegmentData::Single(segment) => vec![(segment.range.start, segment.range.end)],
            SegmentData::Multiple(segments) => segments.iter().map(|segment| (segment.range.start, segment.range.end)).collect(),
        }
    }
    pub fn to_range(&self) -> Vec<Range<usize>> {
        let ranges = self.to_vec();
        ranges.iter().map(|(start, end)| *start..*end).collect::<Vec<Range<usize>>>()
    }
}

fn segment_append(data: SegmentData, segment: Segment) -> SegmentData {
    match data {
        SegmentData::None => SegmentData::Single(segment),
        SegmentData::Single(_segment) => SegmentData::Multiple(vec![_segment, segment]),
        SegmentData::Multiple(mut segments) => {
            segments.push(segment);
            SegmentData::Multiple(segments)
        }
    }
}

#[derive(Default)]
pub struct HttpMessage {
    pub frame_index: FrameIndex,
    pub host: String,
    pub hostname: Option<String>,
    pub length: Option<usize>,
    pub chunked: bool,
    pub content_type: Option<String>,
    pub headers: SegmentData,
    pub content: SegmentData,
    pub http_connect_index: Option<HttpConnectIndex>,
}

impl HttpMessage {
    pub fn append_body(&mut self, index: FrameIndex, range: Range<usize>) {
        let segment = Segment { index, range };
        let orgin = std::mem::take(&mut self.content);
        self.content = segment_append(orgin, segment);
    }
    /**
     * request return method
     * response return status code
     */
    pub fn get_method_or_status(&self) -> String {
        let mut parts = self.host.split_whitespace();
        if self.host.starts_with("HTTP/") {
            if let Some(status_code) = parts.nth(1) {
                if let Some(h) = status_code.chars().next() {
                    return format!("{h}XX");
                }
            }
        } else if let Some(method) = parts.next() {
            return method.to_string();
        }
        "None".to_string()
    }
}

#[derive(Default)]
pub struct HttpConntect {
    pub index: ConnectionIndex,
    pub request: Option<MessageIndex>,
    pub response: Option<MessageIndex>,
    pub hostname: Option<String>,
    pub rt: Timestamp,
}

impl HttpConntect {
    pub fn to_message<'a>(&self, ctx: &'a Context, index: &Option<MessageIndex>) -> Option<&'a HttpMessage> {
        if let Some(_index) = index {
            ctx.http_messages.get(*_index as usize)
        } else {
            None
        }
    }
    pub fn conv(&self, ctx: &Context) -> VHttpConnection {
        let mut rs = VHttpConnection::default();
        if let Some(request_index) = &self.request {
            if let Some(message) = ctx.http_messages.get(*request_index as usize) {
                rs.request_headers = message.headers.to_vec();
                rs.request_body = message.content.to_vec();
                rs.hostname = message.hostname.clone().unwrap_or("".to_string());
                if let Some(ll) = &message.length {
                    rs.length = *ll;
                }
                if let Some(ct) = &message.content_type {
                    rs.content_type = ct.clone();
                }
                rs.request = Some(message.host.clone());
            }
        }
        if let Some(response_index) = &self.response {
            if let Some(message) = ctx.http_messages.get(*response_index as usize) {
                rs.response_headers = message.headers.to_vec();
                rs.response_body = message.content.to_vec();
                if let Some(ll) = message.length {
                    rs.length = ll;
                }
                if let Some(ct) = &message.content_type {
                    rs.content_type = ct.clone();
                }
                rs.response = Some(message.host.clone());
            }
        }
        rs.rt = if self.rt > 0 { format!("{}µs", self.rt) } else { "N/A".to_string() };
        rs
    }

    fn to_detail(&self, ctx: &Context, loader: &dyn ResourceLoader, index: &Option<MessageIndex>, is_request: bool) -> Result<HttpMessageDetail> {
        if let Some(request_index) = index {
            if let Some(message) = ctx.http_messages.get(*request_index as usize) {
                let header_range = message.headers.to_range();
                let header_data = loader.loads(&header_range)?;
                let text = String::from_utf8_lossy(&header_data);
                let headers = text.split("\r\n").map(|f|f.into()).collect();
                let body_range = message.content.to_range();
                let content = loader.loads(&body_range)?;
                return Ok(HttpMessageDetail::new(is_request, headers, content));
            }
        }
        bail!("")
    }
    pub fn convert_to_detail(&self, ctx: &Context, loader: &dyn ResourceLoader) -> Result<Vec<HttpMessageDetail>> {
        let mut list = vec![];
        if let Ok(message) = self.to_detail(ctx, loader, &self.request, true) {
            list.push(message);   
        }
        if let Ok(message) = self.to_detail(ctx, loader, &self.response, false) {
            list.push(message);   
        }
        Ok(list)
    }

    pub fn info(&self, ctx: &Context) -> (String, String, String) {
        let method = if let Some(message) = self.to_message(ctx, &self.request) {
            message.get_method_or_status()
        } else {
            "NONE".to_string()
        };
        let (status, content_type) = if let Some(message) = self.to_message(ctx, &self.response) {
            (message.get_method_or_status(), message.content_type.clone().unwrap_or("NONE".to_string()))
        } else {
            ("NONE".to_string(), "NONE".to_string())
        };
        let ct = if let Some(full) = content_type.split(';').next() {
            full.trim().to_string()
        } else {
            content_type.clone()
        };
        (method, status, ct)
    }
}

impl HttpConntect {
    fn request(index: ConnectionIndex, message_index: MessageIndex) -> Self {
        Self {
            index,
            request: Some(message_index),
            ..Default::default()
        }
    }
    fn response(index: ConnectionIndex, message_index: MessageIndex) -> Self {
        Self {
            index,
            response: Some(message_index),
            ..Default::default()
        }
    }
    fn add_response(&mut self, message_index: MessageIndex, ts: Timestamp) {
        self.response = Some(message_index);
        self.rt = ts;
    }
}

#[derive(Default)]
pub struct Context {
    pub file_type: FileType,
    pub link_type: u32,
    pub list: Vec<Frame>,
    pub counter: FrameIndex,
    // tcp
    pub active_connection: FastHashMap<(u64, u16, u64, u16), usize>,
    pub conversation_map: FastHashMap<ConversationKey, usize>,
    pub conversation_list: Vec<Conversation>,
    // pub connections: Vec<Connection>,
    // http
    pub http_connections_map: FastHashMap<ConnectionIndex, (HttpConnectIndex, Timestamp)>,
    pub http_connections: Vec<HttpConntect>,
    pub http_messages: Vec<HttpMessage>,
    pub http_hostnames: FastHashMap<String, u16>,

    pub tls_sni: FastHashMap<String, u16>,
    // ethernet
    pub ethermap: FastHashMap<u64, EthernetCache>,
    pub ipv6map: FastHashMap<u64, (u8, Ipv6Addr, Ipv6Addr)>,
    pub string_map: FastHashMap<u64, NString>,

    pub stat_ip4: FastHashMap<Ipv4Addr, u16>,
    pub stat_ip6: FastHashMap<Ipv6Addr, u16>,
}

impl Context {
    pub fn cache_str(&mut self, s: String) -> NString {
        let key = quick_hash(&s);
        if let Some(rs) = self.string_map.get(&key) {
            return rs;
        }
        let static_ref: NString = Box::leak(s.into_boxed_str());
        self.string_map.insert(key, static_ref);
        static_ref
    }
    pub fn init_segment_message(&mut self, frame_index: FrameIndex, host: String, is_request: bool, connect_index: ConnectionIndex, timestamp: Timestamp) -> MessageIndex {
        let message_index = self.http_messages.len() as MessageIndex;
        let mut sg = HttpMessage {
            frame_index,
            host,
            ..Default::default()
        };
        // self.http_messages.push(sg);

        if is_request {
            let http_connect_index = self.http_connections.len() as HttpConnectIndex;
            let connect = HttpConntect::request(connect_index, message_index);
            self.http_connections.push(connect);
            self.http_connections_map.insert(connect_index, (http_connect_index, timestamp));
            sg.http_connect_index = Some(http_connect_index);
        } else if let Some((http_connect_index, ts)) = self.http_connections_map.get(&connect_index) {
            if let Some(connect) = self.http_connections.get_mut(*http_connect_index as usize) {
                let fd = timestamp.saturating_sub(*ts);
                connect.add_response(message_index, fd);
                self.http_connections_map.remove(&connect_index);
            }
        } else {
            self.http_connections.push(HttpConntect::response(connect_index, message_index));
        }
        self.http_messages.push(sg);
        message_index
    }
    pub fn get_http_message(&mut self, message_index: MessageIndex) -> Option<&mut HttpMessage> {
        self.http_messages.get_mut(message_index as usize)
    }
}

pub trait Factor {
    fn get(&self) -> (u64, u16);
    fn host(&self) -> String;
}

#[derive(PartialEq)]
pub struct IPV4Point {
    ip: Ipv4Addr,
    pub ip_hash: u64,
    pub port: u16,
}

impl IPV4Point {
    fn new(ip: &Ipv4Addr, port: u16) -> Self {
        let ip_hash = quick_hash(ip);
        Self { ip: *ip, ip_hash, port }
    }
}

impl From<IPV4Point> for Endpoint {
    fn from(val: IPV4Point) -> Self {
        Endpoint::new(val.ip.to_string(), val.port)
    }
}

impl Factor for IPV4Point {
    fn get(&self) -> (u64, u16) {
        (self.ip_hash, self.port)
    }
    fn host(&self) -> String {
        self.ip.to_string()
    }
}
impl PartialOrd for IPV4Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.ip_hash.partial_cmp(&other.ip_hash) {
            Some(core::cmp::Ordering::Equal) => self.port.partial_cmp(&other.port),
            ord => ord,
        }
    }
}

#[derive(PartialEq)]
pub struct IPV6Point {
    ip: Ipv6Addr,
    pub ip_hash: u64,
    pub port: u16,
}

impl IPV6Point {
    fn new(ip: &Ipv6Addr, port: u16) -> Self {
        let ip_hash = quick_hash(ip);
        Self { ip: *ip, ip_hash, port }
    }
}

impl From<IPV6Point> for Endpoint {
    fn from(val: IPV6Point) -> Self {
        Endpoint::new(val.ip.to_string(), val.port)
    }
}

impl Factor for IPV6Point {
    fn get(&self) -> (u64, u16) {
        (self.ip_hash, self.port)
    }
    fn host(&self) -> String {
        self.ip.to_string()
    }
}

impl PartialOrd for IPV6Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.ip_hash.partial_cmp(&other.ip_hash) {
            Some(core::cmp::Ordering::Equal) => self.port.partial_cmp(&other.port),
            ord => ord,
        }
    }
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn _get_connect<T>(&mut self, _: &mut Frame, source: T, target: T, stat: TCPStat, data_source: &DataSource, range: Range<usize>) -> Result<ConnectState>
    where
        T: Into<Endpoint> + PartialOrd + Factor,
    {
        let reverse = source > target;
        let s = source.get();
        let t = target.get();
        let key = match reverse {
            true => (s.0, s.1, t.0, t.1),
            false => (t.0, t.1, s.0, s.1),
        };
        let conversation_key = match reverse {
            true => (s.0, t.0),
            false => (t.0, s.0),
        };
        let eps = match reverse {
            true => (source, target),
            false => (target, source),
        };
        let conversation_index = self.conversation_map.entry(conversation_key).or_insert_with(|| -> usize {
            let index = self.conversation_list.len();
            self.conversation_list.push(Conversation::new(index, eps.0.host(), eps.1.host()));
            index
        });
        let conversation = self.conversation_list.get_mut(*conversation_index).unwrap();

        let mut _index: usize = 0;

        if let Some(index) = self.active_connection.get(&key) {
            _index = *index;
        } else {
            let connection = Connection::new(eps.0.into(), eps.1.into());
            _index = conversation.add_connection(connection);
            self.active_connection.insert(key, _index);
        }
        let mut tmp_conn = TmpConnection::new(conversation, _index, reverse);
        let mut rs = tmp_conn.update(&stat, data_source, range)?;
        rs.connection = Some(((*conversation_index, _index), reverse));
        // remove
        if rs.connect_finished {
            self.active_connection.remove(&key);
        }
        Ok(rs)
    }
    pub fn get_connect(&mut self, frame: &mut Frame, port1: u16, port2: u16, stat: TCPStat, data_source: &DataSource, range: Range<usize>) -> Result<ConnectState> {
        match &frame.address_field {
            AddressField::IPv4(source, target) => {
                let s = IPV4Point::new(source, port1);
                let t = IPV4Point::new(target, port2);
                self._get_connect(frame, s, t, stat, data_source, range)
            }
            AddressField::IPv6(key) => {
                if let Some((_, source, target)) = self.ipv6map.get(key) {
                    let s = IPV6Point::new(source, port1);
                    let t = IPV6Point::new(target, port2);
                    self._get_connect(frame, s, t, stat, data_source, range)
                } else {
                    bail!("c-1-1")
                }
            }
            _ => bail!("c-1-0"),
        }
    }

    pub fn connection(&mut self, frame: &mut Frame) -> Option<(ConnectionIndex, &mut Endpoint)> {
        if let Some(tcp_info) = &frame.tcp_info {
            if let Some(((conversation_index, connect_index), reverse)) = tcp_info.connection {
                if let Some(conversation) = self.conversation_list.get_mut(conversation_index) {
                    if let Some(conn) = conversation.connection(connect_index) {
                        return match reverse {
                            true => Some(((conversation_index, connect_index), &mut conn.primary)),
                            false => Some(((conversation_index, connect_index), &mut conn.second)),
                        };
                    }
                }
            }
        }
        None
    }
    pub fn add_http_hostname(&mut self, message_index: MessageIndex, hostname: &str) {
        if let Some(message) = self.http_messages.get(message_index as usize) {
            if let Some(http_connect_index) = &message.http_connect_index {
                if let Some(connect) = self.http_connections.get_mut(*http_connect_index as usize) {
                    let hn = hostname.trim().to_lowercase();
                    Context::add_map(&hn, &mut self.http_hostnames);
                    connect.hostname = Some(hn);
                }
            }
        }
    }
}

impl Context {
    fn add_map<K, T>(key: &K, map: &mut FastHashMap<K, T>)
    where
        K: core::hash::Hash + Eq + Clone,
        T: AddAssign + Default + Copy + From<u8>,
    {
        if let Some(v) = map.get_mut(key) {
            *v += T::from(1);
        } else {
            map.insert(key.clone(), T::from(1));
        }
    }
    fn add_map2<K, T>(key: &K, map: &mut HashMap<K, T>)
    where
        K: core::hash::Hash + Eq + Clone,
        T: AddAssign + Default + Copy + From<u8>,
    {
        if let Some(v) = map.get_mut(key) {
            *v += T::from(1);
        } else {
            map.insert(key.clone(), T::from(1));
        }
    }
    fn _list_map<K, T>(map: &FastHashMap<K, T>) -> String
    where
        K: core::hash::Hash + Eq + ToString,
        T: Copy + Into<usize>,
    {
        let rs: Vec<CounterItem> = map.iter().map(|(k, v)| CounterItem::new(k.to_string(), (*v).into())).collect();
        serde_json::to_string(&rs).unwrap_or("[]".into())
    }
}

impl Context {
    pub fn stat_http_host(&self) -> String {
        Context::_list_map(&self.http_hostnames)
    }
    pub fn add_tls_sni(&mut self, sni: String) {
        Context::add_map(&sni, &mut self.tls_sni);
    }
    pub fn stat_tls_sni(&self) -> String {
        Context::_list_map(&self.tls_sni)
    }
    pub fn add_ip4(&mut self, ip: &Ipv4Addr) {
        Context::add_map(ip, &mut self.stat_ip4);
    }
    pub fn add_ip6(&mut self, ip: &Ipv6Addr) {
        Context::add_map(ip, &mut self.stat_ip6);
    }
    pub fn stat_ip4(&self) -> String {
        Context::_list_map(&self.stat_ip4)
    }
    pub fn stat_ip6(&self) -> String {
        Context::_list_map(&self.stat_ip6)
    }
}

fn flat(map: &HashMap<String, usize>) -> Vec<CounterItem> {
    map.iter().map(|(k, v)| CounterItem::new(k.to_string(), *v)).collect()
}

// fn increase(map: &mut FastHashMap<String, usize>, key: &String, mount: usize) {
//     if let Some(v) = map.get_mut(key) {
//         *v += mount;
//     } else {
//         map.insert(key.clone(), mount);
//     }
// }
// fn flat_fastmap(map: &FastHashMap<String, usize>) -> Vec<CounterItem> {
//     map.iter().map(|(k, v)| CounterItem::new(k.to_string(), *v)).collect()
// }

impl Context {
    pub fn stat_http_data(&self) -> String {
        let mut method_map: HashMap<String, usize> = HashMap::with_capacity(4);
        let mut status_map: HashMap<String, usize> = HashMap::with_capacity(6);
        let mut type_map: HashMap<String, usize> = HashMap::new();
        for connect in &self.http_connections {
            let (method, status, content_type) = connect.info(self);
            Context::add_map2(&method, &mut method_map);
            Context::add_map2(&status, &mut status_map);
            Context::add_map2(&content_type, &mut type_map);
        }
        serde_json::to_string(&vec![flat(&method_map), flat(&status_map), flat(&type_map)]).unwrap()
    }
    pub fn stat_frame(&self) -> String {
        if self.list.len() <= 10 {
            return "{}".to_string();
        }
        let first = self.list.first().unwrap();
        let last = self.list.last().unwrap();
        let period = last.info.time.saturating_sub(first.info.time);
        if period < 100 {
            return "{}".to_string();
        }
        let size: usize = 200;
        let r = period.div_ceil(size as u64);
        let mut limit = first.info.time + r;
        let mut series = vec![];

        let mut tcp: Vec<u32> = vec![0; size];
        let mut udp: Vec<u32> = vec![0; size];
        let mut http: Vec<u32> = vec![0; size];
        let mut tls: Vec<u32> = vec![0; size];
        let mut other: Vec<u32> = vec![0; size];
        let protocols = ["tcp", "udp", "http", "tls", "other"];

        let mut index = 0;
        let incr = |list: &mut Vec<u32>, index: usize, mount: u32| {
            if let Some(v) = list.get_mut(index) {
                *v += mount;
            } else {
                list.insert(index, mount);
            }
        };
        series.push(first.info.time);
        for frame in &self.list {
            let time = frame.info.time;
            if time > limit {
                loop {
                    series.push(limit);
                    limit += r;
                    index += 1;
                    if limit > time {
                        break;
                    }
                }
            }
            let mount = frame.info.len;
            match frame.tail {
                Protocol::TCP => {
                    incr(&mut tcp, index, mount);
                }
                Protocol::UDP => {
                    incr(&mut udp, index, mount);
                }
                Protocol::HTTP => {
                    incr(&mut http, index, mount);
                }
                Protocol::TLS => {
                    incr(&mut tls, index, mount);
                }
                _ => {
                    incr(&mut other, index, mount);
                }
            }
        }

        let data = LineChartData::new(series, protocols.iter().map(|f| (*f).into()).collect(), vec![tcp, udp, http, tls, other]);
        serde_json::to_string(&data).unwrap_or("{}".into())
    }
}
