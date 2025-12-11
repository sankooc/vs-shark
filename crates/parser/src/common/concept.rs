// Copyright (c) 2025 sankooc
//
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use std::hash::Hash;

use serde::{Deserialize, Serialize};

use crate::{
    common::{connection::Connection, enum_def::Protocol, util::date_str, FastHashMap, Instance, NString},
    protocol::transport::tls::tls_version_map,
};

use super::enum_def::PacketStatus;

pub type FrameIndex = u32;
pub type MessageIndex = u64;
pub type HttpConnectIndex = u64;

pub type ConnectionIndex = (usize, usize);
pub type ConversationKey = (u64, u64);

pub type Timestamp = u64;

#[derive(Deserialize, Serialize)]
pub struct Criteria {
    pub size: usize,
    pub start: usize,
}

pub struct HttpCriteria {
    pub hostname: Option<String>,
}

#[derive(Default)]
pub struct ConversationCriteria {
    pub ip: Option<String>,
}

impl ConversationCriteria {
    pub fn ip(ip: String) -> Self {
        Self { ip: Some(ip) }
    }
}

impl HttpCriteria {
    pub fn hostname(hostname: String) -> Self {
        Self { hostname: Some(hostname) }
    }
}

#[derive(Default, Copy, Clone)]
pub struct InstanceConfig {
    pub batch_size: usize,
}

#[derive(Serialize, Default)]
pub struct LineChartData {
    pub x_axis: Vec<u64>,
    pub y_axis: Vec<String>,
    pub data: Vec<Vec<u32>>,
}

impl LineChartData {
    pub fn new(x_axis: Vec<u64>, y_axis: Vec<String>, data: Vec<Vec<u32>>) -> Self {
        Self { x_axis, y_axis, data }
    }
}

#[derive(Serialize)]
pub struct CounterItem {
    pub key: String,
    pub count: usize,
}

impl CounterItem {
    pub fn new(key: String, count: usize) -> Self {
        Self { key, count }
    }
}

#[derive(Serialize, Debug)]
pub struct ProgressStatus {
    pub total: usize,
    pub cursor: usize,
    pub count: usize,
    pub left: usize,
}

// impl ProgressStatus {
//     pub fn to_json(&self) -> String {
//         serde_json::to_string(self).unwrap()
//     }
// }

#[derive(Serialize)]
pub struct ListResult<T> {
    pub items: Vec<T>,
    pub total: usize,
    pub start: usize,
}

impl<T> ListResult<T> {
    pub fn new(start: usize, total: usize, items: Vec<T>) -> Self {
        Self { start, total, items }
    }
    pub fn empty() -> Self {
        Self {
            start: 0,
            total: 0,
            items: vec![],
        }
    }
}

#[derive(Default)]
pub struct FrameInternInfo {
    pub index: FrameIndex,
    pub time: u64,
    pub len: u32,
    pub irtt: u16,
    pub status: PacketStatus,
}

#[derive(Serialize, Default, Clone)]
pub struct FrameInfo {
    pub index: FrameIndex,
    pub time: u64,
    pub source: String,
    pub dest: String,
    pub protocol: String,
    pub len: u32,
    pub irtt: u16,
    pub info: String,
    pub status: PacketStatus,
}

impl From<&FrameInternInfo> for FrameInfo {
    fn from(value: &FrameInternInfo) -> Self {
        Self {
            index: value.index,
            time: value.time,
            len: value.len,
            irtt: value.irtt,
            status: value.status,
            ..Default::default()
        }
    }
}

#[derive(Clone)]
pub struct HttpHeadContinue {
    pub message_index: usize,
    pub frame_index: FrameIndex,
    pub length: usize,
    pub chunked: bool,
    pub extra: Vec<u8>,
}

impl HttpHeadContinue {
    pub fn new(length: usize, chunked: bool, extra: Vec<u8>) -> HttpHeadContinue {
        HttpHeadContinue {
            message_index: 0,
            frame_index: 0,
            length,
            chunked,
            extra,
        }
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Field {
    // #[serde(skip)]
    // pub extra_data: Option<Vec<u8>>,
    pub source: u8,
    pub start: usize,
    pub size: usize,
    pub summary: String,
    pub children: Option<Vec<Field>>,
}

impl Field {
    pub fn new(summary: String, start: usize, end: usize, children: Vec<Field>) -> Field {
        Field {
            start,
            source: 0,
            size: end - start,
            summary,
            children: Some(children),
        }
    }
    pub fn label(summary: String, start: usize, end: usize) -> Field {
        Field {
            start,
            source: 0,
            size: end - start,
            summary,
            children: None,
        }
    }
    pub fn with_children(summary: String, start: usize, size: usize) -> Field {
        Field {
            source: 0,
            start,
            size,
            summary,
            children: Some(Vec::new()),
        }
    }
    pub fn children() -> Self {
        Self {
            children: Some(vec![]),
            ..Default::default()
        }
    }
    pub fn with_children_reader(reader: &super::io::Reader) -> Field {
        Field::with_children(String::from(""), reader.cursor, 0)
    }
}

pub struct Conversation {
    pub key: usize,
    pub primary: String,
    pub second: String,
    pub primary_statistic: TCPStatistic,
    pub second_statistic: TCPStatistic,
    pub connections: Vec<Connection>,
}

impl Conversation {
    pub fn new(key: usize, primary: String, second: String) -> Self {
        Self {
            key,
            primary,
            second,
            primary_statistic: TCPStatistic::default(),
            second_statistic: TCPStatistic::default(),
            connections: Vec::new(),
        }
    }
    pub fn add_connection(&mut self, conn: Connection) -> usize {
        let index = self.connections.len();
        self.connections.push(conn);
        index
    }
    pub fn connection(&mut self, index: usize) -> Option<&mut Connection> {
        self.connections.get_mut(index)
    }
    pub fn statistic(&mut self, reverse: bool) -> &mut TCPStatistic {
        match reverse {
            true => &mut self.primary_statistic,
            false => &mut self.second_statistic,
        }
    }
    pub fn match_ip(&self, ip: &str) -> bool {
        self.primary == ip || self.second == ip
    }
}

impl From<&Conversation> for VConversation {
    fn from(val: &Conversation) -> Self {
        let key = val.key;
        let sender_packets = val.primary_statistic.count;
        let receiver_packets = val.second_statistic.count;
        let sender_bytes = val.primary_statistic.throughput;
        let receiver_bytes = val.second_statistic.throughput;
        let connects = val.connections.len();
        VConversation {
            key,
            sender: val.primary.clone(),
            receiver: val.second.clone(),
            sender_packets,
            receiver_packets,
            sender_bytes,
            receiver_bytes,
            connects,
        }
    }
}

#[derive(Serialize)]
pub struct VConversation {
    pub key: usize,
    pub sender: String,
    pub receiver: String,
    pub sender_packets: u32,
    pub receiver_packets: u32,
    pub sender_bytes: u64,
    pub receiver_bytes: u64,
    pub connects: usize,
}

#[derive(Serialize, Clone, Default)]
pub struct UDPConversation {
    pub index: usize,
    #[serde(skip)]
    pub ts: Timestamp,
    pub sender: String,
    pub receiver: String,
    pub sender_port: u16,
    pub receiver_port: u16,
    pub packets: u32,
    pub bytes: usize,
    pub records: Vec<(u64, usize)>,
    pub ts_str: String,
    pub offset_str: (f64, NString),
}
impl UDPConversation {
    pub fn new(index: usize, ts: Timestamp, sender: String, receiver: String, sender_port: u16, receiver_port: u16) -> Self {
        Self {
            index,
            ts,
            sender,
            receiver,
            sender_port,
            receiver_port,
            ..Default::default()
        }
    }
    pub fn init(&mut self, first: Timestamp) {
        self.ts_str = date_str(self.ts);
        self.offset_str = period(self.ts, self.ts.saturating_sub(first));
    }
    pub fn incr(&mut self, mount: usize, time: u64) {
        self.packets += 1;
        self.bytes += mount;
        self.records.push((time, mount));
    }
}

#[derive(Serialize, Default, Clone)]
pub struct TCPStatistic {
    pub count: u32,
    pub throughput: u64,
    pub clean_throughput: u64,
    pub retransmission: u32,
    pub invalid: u32,
}

impl TCPStatistic {
    pub fn append(&mut self, other: &TCPStatistic) {
        self.count += other.count;
        self.throughput += other.throughput;
        self.clean_throughput += other.clean_throughput;
        self.retransmission += other.retransmission;
        self.invalid += other.invalid;
    }
}

#[derive(Serialize, Default, Clone)]
pub struct VConnection {
    pub primary: VEndpoint,
    pub second: VEndpoint,
    pub protocol: String,
}

impl From<&Connection> for VConnection {
    fn from(value: &Connection) -> Self {
        let protocol = match value.protocol {
            Protocol::HTTP => "http".into(),
            Protocol::TLS => "tls".into(),
            _ => "".into(),
        };
        Self {
            primary: value.primary().into(),
            second: value.second().into(),
            protocol,
        }
    }
}

#[derive(Serialize, Default, Clone)]
pub struct VEndpoint {
    pub host: String,
    pub port: u16,
    pub statistic: TCPStatistic,
}

#[derive(Serialize, Default, Clone, Debug)]
pub struct VHttpConnection {
    pub index: usize,
    pub request: Option<String>,
    pub response: Option<String>,
    pub latency: String,
    pub hostname: String,
    pub content_type: String,
    pub length: usize,
    #[serde(skip)]
    pub ts: Timestamp,
    pub ts_str: String,
}

const NA: &str = "N/A";

impl VHttpConnection {
    pub fn status(&self) -> &str {
        if let Some(response) = &self.response {
            let tokens = response.split_whitespace().collect::<Vec<&str>>();
            if tokens.len() > 1 {
                return tokens[1];
            }
        }
        NA
    }
    pub fn method(&self) -> &str {
        if let Some(request) = &self.request {
            let tokens = request.split_whitespace().collect::<Vec<&str>>();
            if tokens.len() > 2 {
                return tokens[0];
            }
        }
        NA
    }
    pub fn url(&self) -> &str {
        if let Some(request) = &self.request {
            let tokens = request.split_whitespace().collect::<Vec<&str>>();
            if tokens.len() > 2 {
                return tokens[1];
            }
        }
        NA
    }
}

pub fn period(sample: u64, time: u64) -> (f64, NString) {
    let digits = sample.checked_ilog10().unwrap_or(0) + 1;
    let base_per_sec = match digits {
        0..=11 => 1.0,
        12..=14 => 1_000.0,
        15..=17 => 1_000_000.0,
        _ => 1_000_000_000.0,
    };

    let seconds = time as f64 / base_per_sec;

    let (val, unit) = if seconds >= 1.0 {
        (seconds, "s")
    } else if seconds * 1_000.0 >= 1.0 {
        (seconds * 1_000.0, "ms")
    } else if seconds * 1_000_000.0 >= 1.0 {
        (seconds * 1_000_000.0, "µs")
    } else {
        (seconds * 1_000_000_000.0, "ns")
    };
    let val = (val * 10_000.0).round() / 10_000.0;
    (val, unit)
}

#[derive(Serialize, Default, Debug, Clone)]
pub struct DNSResponse {
    pub transaction_id: u16,
    pub source: String,
    pub target: String,
    #[serde(skip)]
    pub request: Option<usize>,
    pub response: Option<usize>,
    #[serde(skip)]
    pub req_ts: Timestamp,
    #[serde(skip)]
    pub res_ts: Timestamp,
    #[serde(skip)]
    pub _latency: Timestamp,
    pub latency: (f64, NString),
    pub ts_str: String,
    pub offset_str: (f64, NString),
}

impl DNSResponse {
    pub fn is_complete(&self) -> bool {
        self.response.is_some()
    }
    pub fn fix_offset(&mut self) -> bool {
        let rs = self.is_complete();
        if self.request.is_some() && rs && self.res_ts > 0 {
            self._latency = self.res_ts.saturating_sub(self.req_ts);
        }
        rs
    }
    pub fn convert<T>(instance: &Instance<T>, item: &DNSResponse, first: Timestamp) -> Self {
        let mut rs = item.clone();

        rs.ts_str = date_str(rs.res_ts);
        rs.offset_str = period(rs.res_ts, rs.res_ts.saturating_sub(first));
        rs.latency = period(rs.res_ts, rs._latency);

        if let Some(index) = item.request {
            if let Some(frame) = instance.frame(index) {
                // start = frame.info.time;
                // rs.ts = start;
                // rs.ts_str = date_str(rs.res_ts);
                // rs.offset_str = period(rs.res_ts, rs.res_ts.saturating_sub(first));
                if let Some((ip, _)) = frame.addresses(instance.context()) {
                    rs.source = ip;
                }
                // rs.source = frame.info.
            }
        }

        if let Some(index) = item.response {
            if let Some(frame) = instance.frame(index) {
                if let Some((ip, _)) = frame.addresses(instance.context()) {
                    rs.target = ip;
                }
            }
        }
        rs
    }
}

#[derive(Default)]
pub struct TLSInfo {
    client_hello: Option<FrameIndex>,
    server_hello: Option<FrameIndex>,
    cert: Option<FrameIndex>,
}

impl TLSInfo {
    pub fn exists(&self) -> bool {
        self.client_hello.is_some() || self.server_hello.is_some() || self.cert.is_some()
    }
    pub fn update_client(&mut self, index: FrameIndex) {
        self.client_hello = Some(index);
    }
    pub fn update_server(&mut self, index: FrameIndex) {
        self.server_hello = Some(index);
    }
    pub fn update_cert(&mut self, index: FrameIndex) {
        self.cert = Some(index);
    }
    pub fn client(&self) -> Option<FrameIndex> {
        self.client_hello
    }
    pub fn server(&self) -> Option<FrameIndex> {
        self.server_hello
    }
    pub fn cert(&self) -> Option<FrameIndex> {
        self.cert
    }
}

#[derive(Clone, Copy)]
pub enum NameService {
    DNS,
    MDNS,
    NBNS,
}

#[derive(Clone, Copy)]
pub enum Language {
    Text,
    Json,
    JavaScript,
    Css,
    Html,
    Xml,
    Csv,
    Yaml,
    Binary,
}
pub enum HttpEncoding {
    None,
    Gzip,
    Deflate,
    Brotli,
    Zstd,
}

#[derive(Serialize, Debug)]
pub struct HttpMessageDetail {
    pub is_request: bool,
    pub headers: Vec<String>,
    pub content: Vec<u8>,
}

impl HttpMessageDetail {
    pub fn new(is_request: bool, headers: Vec<String>, content: Vec<u8>) -> Self {
        Self { is_request, headers, content }
    }
    fn header(&self, head: &str) -> Option<String> {
        let _head = head.to_lowercase();
        for header in &self.headers {
            let _header = header.to_lowercase();
            if _header.starts_with(&_head) {
                let lcount = _head.len() + 1;
                let mut val = &_header[lcount..];
                if let Some(inx) = val.find(";") {
                    val = &val[..inx];
                }
                let value = val.trim().to_string();
                return Some(value);
            }
        }
        None
    }
    pub fn raw_content(&self) -> &[u8] {
        &self.content
    }
    pub fn get_text_content(&self) -> Option<String> {
        let len = self.content.len();
        if len == 0 {
            return None;
        }
        match self.text_type() {
            Language::Binary => None,
            _ => Some(decode_bytes(self.raw_content(), self.text_encoding())),
        }
    }
    pub fn content_type(&self) -> Option<String> {
        self.header("content-type")
    }
    pub fn text_type(&self) -> Language {
        if let Some(main_type) = self.content_type() {
            if main_type.is_empty() {
                return Language::Binary;
            }
            if main_type.contains("/json") {
                return Language::Json;
            }
            if main_type.contains("/javascript") {
                return Language::JavaScript;
            }
            if main_type.contains("/css") {
                return Language::Css;
            }
            if main_type.contains("/html") {
                return Language::Html;
            }
            if main_type.contains("/xml") {
                return Language::Xml;
            }
            if main_type.contains("/csv") {
                return Language::Csv;
            }
            if main_type.contains("/yaml") {
                return Language::Yaml;
            }
            if main_type.contains("text/") {
                return Language::Text;
            }
        }
        Language::Binary
    }
    pub fn text_encoding(&self) -> HttpEncoding {
        if let Some(encoding) = self.header("content-encoding") {
            match encoding.as_str() {
                "gzip" => HttpEncoding::Gzip,
                "deflate" => HttpEncoding::Deflate,
                "br" => HttpEncoding::Brotli,
                "zstd" => HttpEncoding::Zstd,
                _ => HttpEncoding::None,
            }
        } else {
            HttpEncoding::None
        }
    }
}

#[derive(Serialize, Debug)]
pub struct TLSConversation {
    pub index: usize,
    pub primary: String,
    pub second: String,
    pub list: Vec<TLSItem>,
}

impl TLSConversation {
    pub fn new(index: usize, primary: String, second: String) -> Self {
        Self {
            index,
            primary,
            second,
            list: vec![],
        }
    }
}

#[derive(Default, Serialize, Clone, Debug)]
pub struct TLSItem {
    pub hostname: Option<String>,
    pub alpn: Option<Vec<String>>,
    #[serde(skip)]
    pub version_code: u16,
    #[serde(skip)]
    pub cs_code: u16,
    pub version: Option<String>,
    pub cipher_suite: Option<String>,
    pub security: String,
    pub count: usize,
    pub addr_1: Option<String>,
    pub addr_2: Option<String>,
}

// impl Default for TLSItem {
//     fn default() -> Self {
//         Self { security: String::from("unknown"), ..Default::default()  }
//     }
// }

impl TLSItem {
    // pub fn new(hostname: Option<String>) -> Self {
    //     Self { hostname, security: String::from("unknown"), ..Default::default() }
    // }
    pub fn set_cipher_suite(&mut self, code: u16) {
        self.cs_code = code;
    }
    pub fn set_version(&mut self, code: u16) {
        self.version_code = code;
        // self.version = tls_version_map(code).map(String::from);
        // self.security = format!("{:?}", security_level(self.version_code, self.cs_code)).to_lowercase()
    }
    pub fn update(&mut self) {
        self.count += 1;
        if self.version_code != 0 && self.version.is_none() {
            self.version = tls_version_map(self.version_code).map(String::from);
        }
        if self.cs_code != 0 && self.cipher_suite.is_none() {
            let suites = crate::constants::tls_cipher_suites_mapper(self.cs_code).to_string();
            self.cipher_suite = Some(suites);
        }
        if self.security.is_empty() {
            self.security = format!("{:?}", security_level(self.version_code, self.cs_code)).to_lowercase()
        }
    }

    pub fn get_trait(&self) -> (Option<String>, u16, u16) {
        (self.hostname.clone(), self.cs_code, self.version_code)
    }

    // pub fn update(&mut self) {
    //     self.count += 1;
    // }
    // pub fn add_alpn(&mut self, alpn: String) {
    //     self.alpn = Some(alpn);
    // }
}

#[derive(Clone, Copy, Default, Debug)]
pub enum SecurityLevel {
    LOW,
    HIGH,
    #[default]
    UNKNOWN,
}

const TLS12_STRONG_CIPHERS: [u16; 9] = [
    0xC02F, // ECDHE-RSA-AES128-GCM-SHA256
    0xC02B, // ECDHE-ECDSA-AES128-GCM-SHA256
    0xC030, // ECDHE-RSA-AES256-GCM-SHA384
    0xC02C, // ECDHE-ECDSA-AES256-GCM-SHA384
    0xCCA8, // ECDHE-RSA-CHACHA20-POLY1305
    0xCCA9, // ECDHE-ECDSA-CHACHA20-POLY1305
    0x009E, // DHE-RSA-AES128-GCM-SHA256
    0x009F, // DHE-RSA-AES256-GCM-SHA384
    0xCCAA, // DHE-RSA-CHACHA20-POLY1305
];

pub fn security_level(version: u16, ciphersuite: u16) -> SecurityLevel {
    use SecurityLevel::*;
    match version {
        0x0304 => HIGH,
        0x0303 => {
            if TLS12_STRONG_CIPHERS.contains(&ciphersuite) {
                HIGH
            } else {
                LOW
            }
        }
        0x0300..=0x0302 => LOW,
        _ => UNKNOWN,
    }
}
fn decode_bytes(body_raw: &[u8], encoding: HttpEncoding) -> String {
    let decoded_data = match encoding {
        HttpEncoding::None => body_raw.to_vec(),
        HttpEncoding::Gzip => {
            use flate2::read::GzDecoder;
            use std::io::Read;
            let mut decoder = GzDecoder::new(body_raw);
            let mut decoded = Vec::new();
            match decoder.read_to_end(&mut decoded) {
                Ok(_) => decoded,
                Err(_) => body_raw.to_vec(),
            }
        }
        HttpEncoding::Deflate => {
            use flate2::read::DeflateDecoder;
            use std::io::Read;
            let mut decoder = DeflateDecoder::new(body_raw);
            let mut decoded = Vec::new();
            match decoder.read_to_end(&mut decoded) {
                Ok(_) => decoded,
                Err(_) => body_raw.to_vec(),
            }
        }
        HttpEncoding::Brotli => {
            use brotli::Decompressor;
            use std::io::Read;
            let mut decoded = Vec::new();
            match Decompressor::new(body_raw, 4096).read_to_end(&mut decoded) {
                Ok(_) => decoded,
                Err(_) => body_raw.to_vec(),
            }
        }
        HttpEncoding::Zstd => {
            use std::io::Read;
            use zstd::stream::read::Decoder;
            let Ok(mut decoder) = Decoder::new(body_raw) else {
                return String::from_utf8_lossy(body_raw).to_string();
            };
            let mut decoded = Vec::new();
            match decoder.read_to_end(&mut decoded) {
                Ok(_) => decoded,
                Err(_) => body_raw.to_vec(),
            }
        }
    };
    String::from_utf8(decoded_data).unwrap_or_default()
}

#[derive(Default)]
pub struct IndexHashMap<K, V> {
    map: FastHashMap<K, usize>,
    list: Vec<V>,
}

impl<K, V> IndexHashMap<K, V>
where
    K: Hash + std::cmp::Eq + Clone,
    V: Default,
{
    pub fn get_or_add(&mut self, key: &K) -> (usize, &mut V) {
        if let Some(val) = self.map.get(key) {
            (*val, self.list.get_mut(*val).unwrap())
        } else {
            let index = self.list.len();
            self.map.insert(key.clone(), index);
            self.list.push(V::default());
            (index, self.list.get_mut(index).unwrap())
        }
    }

    pub fn get(&mut self, key: &K) -> Option<(usize, &mut V)> {
        if let Some(val) = self.map.get(key) {
            Some((*val, self.list.get_mut(*val).unwrap()))
        } else {
            None
        }
    }
    pub fn list(&mut self) -> Vec<V> {
        let rs = std::mem::take(&mut self.list);
        self.map.clear();
        rs
    }
}

#[derive(Serialize, Debug, Default, Clone)]
pub struct DNSRecord {
    pub host: String,
    pub rtype: String,
    pub class: String,
    pub info: Option<String>,
}
impl DNSRecord {
    pub fn new(host: String, rtype: String, class: String, info: Option<String>) -> Self {
        Self { host, rtype, class, info }
    }
}
