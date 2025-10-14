// Copyright (c) 2025 sankooc
// 
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use std::{
    net::{Ipv4Addr, Ipv6Addr}, ops::Range
};

use anyhow::{bail, Result};

use crate::common::{
    concept::{ConnectionIndex, Conversation, ConversationKey, FrameIndex, HttpConnectIndex, HttpHostRecord, MessageIndex, Timestamp, VHttpConnection},
    enum_def::AddressField,
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
    pub fn into(&self, ctx: &Context) -> VHttpConnection {
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
}

impl HttpConntect {
    fn request(index: ConnectionIndex, message_index: MessageIndex) -> Self {
        Self{ index, request: Some(message_index), ..Default::default() }
    }
    fn response(index: ConnectionIndex, message_index: MessageIndex) -> Self {
        Self{ index, response: Some(message_index), ..Default::default() }
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
    pub http_hostnames: FastHashMap<String, u8>,
    // ethernet
    pub ethermap: FastHashMap<u64, EthernetCache>,
    pub ipv6map: FastHashMap<u64, (u8, Ipv6Addr, Ipv6Addr)>,
    pub string_map: FastHashMap<u64, NString>,
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
        let mut sg = HttpMessage { frame_index, host, ..Default::default() };
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
    pub fn add_http_hostname(&mut self, message_index: MessageIndex, hostname: &str){
        if let Some(message) = self.http_messages.get(message_index as usize) {
            if let Some(http_connect_index) = &message.http_connect_index {
                if let Some(connect) = self.http_connections.get_mut(*http_connect_index as usize) {
                    let hn = hostname.trim().to_lowercase();
                    *self.http_hostnames.entry(hn.clone()).or_insert(1) += 1;
                    connect.hostname = Some(hn);
                }
            }
        }
    }
    pub fn http_record_stat(&self) -> Vec<HttpHostRecord> {
        self.http_hostnames.iter().map(|(k, v)| HttpHostRecord::new(k.clone(), *v as usize)).collect()
    }
}
