use std::{net::{Ipv4Addr, Ipv6Addr}, 
    ops::Range}
;

use anyhow::Result;

use super::{concept::FrameInfo, connection::{ConnectState, Connection, Endpoint, TCPStat, TmpConnection}, enum_def::FileType, io::DataSource, quick_hash_str, EthernetCache, FastHashMap, Frame, NString};

#[derive(Default)]
pub struct Context {
    pub file_type: FileType,
    pub link_type: u32,
    pub list: Vec<Frame>,
    pub counter: u32,
    pub active_connection: FastHashMap<(&'static str, u16, &'static str, u16), usize>,
    pub connections: Vec<Connection>,
    pub ethermap: FastHashMap<u64, EthernetCache>,
    pub ipv6map: FastHashMap<u64, (u8, Ipv6Addr, Ipv6Addr)>,
    pub ipv4map: FastHashMap<u64, (Ipv4Addr, Ipv4Addr)>,
    pub string_map: FastHashMap<u64, NString>,
}

impl Context {
    pub fn cache_str(&mut self, s: String) -> NString { 
        let key = quick_hash_str(&s);
        if let Some(rs) = self.string_map.get(&key){
            return rs;
        }
        let static_ref: NString = Box::leak(s.into_boxed_str());
        self.string_map.insert(key, static_ref);
        static_ref
    }
}

pub fn convert(ctx: &mut Context, frame: &Frame) -> FrameInfo {
    todo!()
}

impl Context {
    pub fn new() -> Self {
        Self::default()
        // Self { ds, file_type: FileType::NONE, link_type:0, list: vec![], counter:0, active_connection: FastHashMap::default(), connections: vec![] }
    }
    pub fn get_connect(&mut self, frame: &Frame, port1: u16, port2: u16, stat: TCPStat, data_source: &DataSource, range: Range<usize>) -> Result<ConnectState> {
        // let host1 = frame.source;
        // let host2 = frame.target;
        let host1 = "";
        let host2 = "";
        let mut key = (host1, port1, host2, port2);
        let mut reverse = true;

        if !self.active_connection.contains_key(&key) {
            key = (host2, port2, host1, port1);
            reverse = false;
        }
        let mut _index: usize = 0;

        if let Some(index) = self.active_connection.get(&key) {
            _index = index.clone();
        } else {
            let connection = Connection::new(Endpoint::new(host1, port1), Endpoint::new(host2, port2));
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

        Ok(rs)
    }
}
