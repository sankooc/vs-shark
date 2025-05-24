use std::{
    net::{Ipv4Addr, Ipv6Addr},
    ops::Range,
};

use anyhow::{bail, Result};

use crate::common::enum_def::AddressField;

use super::{
    connection::{ConnectState, Connection, Endpoint, TCPStat, TmpConnection},
    enum_def::{FileType, Protocol},
    io::DataSource,
    quick_hash, EthernetCache, FastHashMap, Frame, NString,
};


pub struct Segment {
    pub index: u32,
    pub range: Range<usize>,
}

pub struct Segments {
    pub message_type: Protocol,
    pub tcp_index: usize,
    pub segments: Vec<Segment>,
}
#[derive(Default)]
pub struct Context {
    pub file_type: FileType,
    pub link_type: u32,
    pub list: Vec<Frame>,
    pub counter: u32,
    pub active_connection: FastHashMap<(u64, u16, u64, u16), usize>,
    pub connections: Vec<Connection>,
    pub segment_messages: Vec<Segments>,
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
    pub fn init_segment_message(&mut self, message_type: Protocol, tcp_index: usize) -> usize {
        let _index = self.segment_messages.len();
        self.segment_messages.push(Segments { message_type, tcp_index, segments: vec![] });
        _index
    }
    pub fn create_segment_message(&mut self, message_type: Protocol, tcp_index: usize, segment: Segment) -> usize {
        let _index = self.segment_messages.len();
        self.segment_messages.push(Segments { message_type, tcp_index, segments: vec![segment] });
        _index
    }
    pub fn add_segment_message(&mut self, message_index: usize, segment: Segment){
        if let Some(msg) = self.segment_messages.get_mut(message_index) {
            msg.segments.push(segment);
        }
    }
}

pub trait Factor {
    fn get(&self) -> (u64, u16);
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
        Self { ip: ip.clone(), ip_hash, port }
    }
}

impl Into<Endpoint> for IPV4Point {
    fn into(self) -> Endpoint {
        Endpoint::new(self.ip.to_string(), self.port)
    }
}

impl Factor for IPV4Point {
    fn get(&self) -> (u64, u16) {
        (self.ip_hash, self.port)
    }
}
impl PartialOrd for IPV4Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.ip_hash.partial_cmp(&other.ip_hash) {
            Some(core::cmp::Ordering::Equal) => self.port.partial_cmp(&other.port),
            ord => return ord,
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
        Self { ip: ip.clone(), ip_hash, port }
    }
}

impl Into<Endpoint> for IPV6Point {
    fn into(self) -> Endpoint {
        Endpoint::new(self.ip.to_string(), self.port)
    }
}

impl Factor for IPV6Point {
    fn get(&self) -> (u64, u16) {
        (self.ip_hash, self.port)
    }
}

impl PartialOrd for IPV6Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.ip_hash.partial_cmp(&other.ip_hash) {
            Some(core::cmp::Ordering::Equal) => self.port.partial_cmp(&other.port),
            ord => return ord,
        }
    }
}

impl Context {
    pub fn new() -> Self {
        Self::default()
        // Self { ds, file_type: FileType::NONE, link_type:0, list: vec![], counter:0, active_connection: FastHashMap::default(), connections: vec![] }
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
        let mut _index: usize = 0;

        if let Some(index) = self.active_connection.get(&key) {
            _index = index.clone();
        } else {
            let connection = match reverse {
                true => Connection::new(source.into(), target.into()),
                false => Connection::new(target.into(), source.into()),
            };
            _index = self.connections.len();
            self.connections.push(connection);
            self.active_connection.insert(key, _index);
        }
        let conn = self.connections.get_mut(_index).unwrap();
        let mut tmp_conn = TmpConnection::new(conn, reverse);
        let mut rs = tmp_conn.update(&stat, data_source, range)?;
        rs.connection = Some((_index, reverse));
        // remove
        if rs.connect_finished {
            self.active_connection.remove(&key);
        }
        return Ok(rs);
    }
    pub fn get_connect(&mut self, frame: &mut Frame, port1: u16, port2: u16, stat: TCPStat, data_source: &DataSource, range: Range<usize>) -> Result<ConnectState> {
        match &frame.ip_field {
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

    pub fn connection(&mut self, frame: &mut Frame) -> Option<(usize, TmpConnection)> {
        if let Some(tcp_info) = &frame.tcp_info {
            if let Some((index, reverse)) = tcp_info.connection {
                if let Some(conn) = self.connections.get_mut(index) {
                    return Some((index, TmpConnection::new(conn, reverse)));
                }
            }
        }
        None
    }
}
