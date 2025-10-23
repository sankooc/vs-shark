// Copyright (c) 2025 sankooc
//
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use serde::Serialize;

use crate::common::{connection::Connection, enum_def::Protocol};

use super::enum_def::PacketStatus;

pub type FrameIndex = u32;
pub type MessageIndex = u64;
pub type HttpConnectIndex = u64;

pub type ConnectionIndex = (usize, usize);
pub type ConversationKey = (u64, u64);

pub type Timestamp = u64;

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

// #[derive(Serialize)]
// pub struct HttpHostRecord {
//     pub host: String,
//     pub count: usize,
// }

// impl HttpHostRecord {
//     pub fn new(host: String, count: usize) -> Self {
//         Self { host, count }
//     }
// }

// pub struct HttpConnectInfo {
//     host: String,
//     method: String,
//     status: String,
//     content_type: String,
// }

// #[derive(Serialize)]
// pub struct FrameStatData {
//     pub time: u64,
//     // pub tcp: [usize; 400],
//     pub list: Vec<CounterItem>,
// }

// impl FrameStatData {
//     pub fn new(time: u64, list: Vec<CounterItem>) -> Self {
//         Self { time, list }
//     }
// }

#[derive(Serialize)]
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

#[derive(Serialize)]
pub struct ProgressStatus {
    pub total: usize,
    pub cursor: usize,
    pub count: usize,
    pub left: usize,
}

impl ProgressStatus {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

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

#[derive(Default, Clone, Serialize)]
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

#[derive(Serialize, Clone)]
pub struct UDPConversation {
    pub index: usize,
    pub sender: String,
    pub receiver: String,
    pub sender_port: u16,
    pub receiver_port: u16,
    pub packets: u32,
    pub bytes: usize,
    pub first_time: u64,
    pub last_time: u64,
}
impl UDPConversation {
    pub fn new(index: usize, sender: String, receiver: String, sender_port: u16, receiver_port: u16, time: u64) -> Self {
        Self {
            index,
            sender,
            receiver,
            sender_port,
            receiver_port,
            packets: 0,
            bytes: 0,
            first_time: time,
            last_time: time,
        }
    }
    pub fn incr(&mut self, mount: usize, time: u64) {
        self.packets += 1;
        self.bytes += mount;
        self.last_time = time;
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

#[derive(Serialize, Default, Clone)]
pub struct VHttpConnection {
    // pub status: String,
    // pub method: String,
    // pub url: String,
    pub request: Option<String>,
    pub response: Option<String>,
    pub rt: String,
    pub hostname: String,
    pub content_type: String,
    pub length: usize,
    pub request_headers: Vec<(usize, usize)>,
    pub request_body: Vec<(usize, usize)>,
    pub response_headers: Vec<(usize, usize)>,
    pub response_body: Vec<(usize, usize)>,
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
