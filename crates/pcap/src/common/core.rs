use std::{
    net::{Ipv4Addr, Ipv6Addr},
    ops::Range,
};

use anyhow::{bail, Result};

use super::{
    connection::{ConnectState, Connection, Endpoint, TCPStat, TmpConnection},
    enum_def::FileType,
    io::DataSource,
    quick_hash, EthernetCache, FastHashMap, Frame, NString,
};

#[derive(Default)]
pub struct Context {
    pub file_type: FileType,
    pub link_type: u32,
    pub list: Vec<Frame>,
    pub counter: u32,
    pub active_connection: FastHashMap<(u64, u16, u64, u16), usize>,
    pub connections: Vec<Connection>,
    pub ethermap: FastHashMap<u64, EthernetCache>,
    pub ipv6map: FastHashMap<u64, (u8, Ipv6Addr, Ipv6Addr)>,
    pub ipv4map: FastHashMap<u64, (Ipv4Addr, Ipv4Addr)>,
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
}

pub trait Factor {
    fn get(&self) -> (u64, u16);
}

#[derive(PartialEq)]
pub struct IPV4Point<'a> {
    ip: &'a Ipv4Addr,
    pub ip_hash: u64,
    pub port: u16,
}

impl<'a> IPV4Point<'a> {
    fn new(ip: &'a Ipv4Addr, port: u16) -> Self {
        let ip_hash = quick_hash(ip);
        Self { ip, ip_hash, port }
    }
}

impl Into<Endpoint> for IPV4Point<'_> {
    fn into(self) -> Endpoint {
        Endpoint::new(self.ip.to_string(), self.port)
    }
}

impl Factor for IPV4Point<'_> {
    fn get(&self) -> (u64, u16) {
        (self.ip_hash, self.port)
    }
}
impl PartialOrd for IPV4Point<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.ip_hash.partial_cmp(&other.ip_hash) {
            Some(core::cmp::Ordering::Equal) => self.port.partial_cmp(&other.port),
            ord => return ord,
        }
    }
}

#[derive(PartialEq)]
pub struct IPV6Point<'a> {
    ip: &'a Ipv6Addr,
    pub ip_hash: u64,
    pub port: u16,
}

impl<'a> IPV6Point<'a> {
    fn new(ip: &'a Ipv6Addr, port: u16) -> Self {
        let ip_hash = quick_hash(ip);
        Self { ip, ip_hash, port }
    }
}

impl Into<Endpoint> for IPV6Point<'_> {
    fn into(self) -> Endpoint {
        Endpoint::new(self.ip.to_string(), self.port)
    }
}

impl Factor for IPV6Point<'_> {
    fn get(&self) -> (u64, u16) {
        (self.ip_hash, self.port)
    }
}

impl PartialOrd for IPV6Point<'_> {
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

    pub fn _get_connect<T>(&mut self, source: T, target: T, stat: TCPStat, data_source: &DataSource, range: Range<usize>) -> Result<ConnectState>
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
        let rs = tmp_conn.update(&stat, data_source, range)?;

        // remove
        if rs.connect_finished {
            self.active_connection.remove(&key);
        }
        return Ok(rs);
    }
    pub fn get_connect(&mut self, frame: &Frame, port1: u16, port2: u16, stat: TCPStat, data_source: &DataSource, range: Range<usize>) -> Result<ConnectState> {

        if let Some(refer) = &frame.ipv4 {
            if let Some((source, target)) = self.ipv4map.get(refer).cloned() {
                let s = IPV4Point::new(&source, port1);
                let t = IPV4Point::new(&target, port2);
                return self._get_connect(s, t, stat, data_source, range);
            }
        } else if let Some(refer) = &frame.ipv6 {
            if let Some((_, source, target)) = self.ipv6map.get(refer).cloned() {
                let s = IPV6Point::new(&source, port1);
                let t = IPV6Point::new(&target, port2);
                return self._get_connect(s, t, stat, data_source, range);
            }
        }
        bail!("c-1-0")
        
    }
    pub fn get_ip4(&self, frame: &Frame) -> Option<&(Ipv4Addr, Ipv4Addr)>{
        if let Some(refer) = &frame.ipv4 {
            return self.ipv4map.get(refer)
        }
        None
    }
}
