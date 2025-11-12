// Copyright (c) 2025 sankooc
//
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use core::Context;
use std::{
    borrow::Borrow,
    cmp,
    collections::HashMap,
    hash::{BuildHasherDefault, Hash, Hasher},
    net::{Ipv4Addr, Ipv6Addr},
    ops::Range,
};

use crate::{
    add_field_label_no_range,
    common::{
        concept::{
            ConversationCriteria, CounterItem, DNSRecord, DNSResponse, FrameIndex, HttpCriteria, HttpMessageDetail, IndexHashMap, LineChartData, NameService, TLSConversation,
            TLSItem, UDPConversation, VConnection, VConversation, VHttpConnection,
        },
        connection::{TcpFlagField, TlsData},
        core::HttpConntect,
        util::date_str,
    },
    files::{pcap::PCAP, pcapng::PCAPNG},
    protocol::{application::dns, detail, link_type_map, parse, summary},
};
use anyhow::{bail, Result};
use concept::{Criteria, Field, FrameInfo, FrameInternInfo, ListResult, ProgressStatus};
use connection::ConnectState;
use enum_def::{AddressField, DataError, FileType, Protocol, ProtocolInfoField};
use io::{DataSource, MacAddress, Reader, IO};
use rustc_hash::FxHasher;

pub type FastHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

pub type NString = &'static str;

// pub const EP: String = String::from("");

pub fn range64(range: Range<usize>) -> Range<u64> {
    range.start as u64..range.end as u64
}

// fn field_session_id_str(data: &[u8]) -> String {
//     let len = std::cmp::min(32, data.len());
//     let mut rs = String::with_capacity(len * 2);
//     for (_, item) in data.iter().enumerate().take(len) {
//         rs.push_str(&format!("{:02x}", *item));
//     }
//     rs
// }

pub fn quick_hash<T>(data: T) -> u64
where
    T: Hash,
{
    let mut hasher = FxHasher::default();
    data.hash(&mut hasher);
    hasher.finish()
}

pub fn quick_string(data: &[u8]) -> String {
    unsafe { String::from_utf8_unchecked(data.to_vec()) }
}
pub fn std_string(data: &[u8]) -> Result<&str, std::str::Utf8Error> {
    std::str::from_utf8(data)
}
pub fn trim_data(data: &[u8]) -> &[u8] {
    let size = data.len();
    let mut start = 0;
    let mut end = size;
    for (inx, data) in data.iter().enumerate().take(size) {
        if *data != b' ' {
            start = inx;
            break;
        }
    }
    for (inx, data) in data.iter().enumerate().take(start).skip(size) {
        if *data != b' ' {
            end = inx;
            break;
        }
    }
    &data[start..end]
}

pub fn quick_trim_num(data: &[u8]) -> Result<usize> {
    let v = trim_data(data);
    let num_str = unsafe { std::str::from_utf8_unchecked(v) };
    Ok(num_str.parse()?)
}

pub fn hex_num(data: &[u8]) -> Result<usize> {
    let num_str = unsafe { std::str::from_utf8_unchecked(data) };
    Ok(usize::from_str_radix(num_str, 16)?)
}

pub struct Ethernet {
    pub source: MacAddress,
    pub destination: MacAddress,
    pub protocol_type: u16,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ProtoMask {
    ETHERNET,
    PPPOES,
    IPV4,
    IPV6,
    TCP,
    UDP,
    ICMP,
    HTTP,
    TLS,
    DNS,
    ARP,
}
impl ProtoMask {
    pub const ALL: [ProtoMask; 11] = [
        ProtoMask::ETHERNET,
        ProtoMask::PPPOES,
        ProtoMask::IPV4,
        ProtoMask::IPV6,
        ProtoMask::TCP,
        ProtoMask::UDP,
        ProtoMask::ICMP,
        ProtoMask::HTTP,
        ProtoMask::TLS,
        ProtoMask::DNS,
        ProtoMask::ARP,
    ];
    pub const fn index(self) -> u32 {
        self as u32
    }
    pub const fn bit(self) -> u32 {
        1u32 << self.index()
    }
    pub const fn name(self) -> NString {
        match self {
            ProtoMask::ETHERNET => "ethernet",
            ProtoMask::PPPOES => "pppoes",
            ProtoMask::IPV4 => "ipv4",
            ProtoMask::IPV6 => "ipv6",
            ProtoMask::TCP => "tcp",
            ProtoMask::UDP => "udp",
            ProtoMask::ICMP => "icmp",
            ProtoMask::HTTP => "http",
            ProtoMask::TLS => "tls",
            ProtoMask::DNS => "dns",
            ProtoMask::ARP => "arp",
        }
    }
}

pub trait ResourceLoader {
    fn load(&self, range: &Range<usize>) -> anyhow::Result<Vec<u8>>;
    fn loads(&self, ranges: &[Range<usize>]) -> anyhow::Result<Vec<u8>>;
}

#[derive(Default)]
pub struct Frame {
    pub range: Option<Range<usize>>,
    pub info: FrameInternInfo,
    pub head: Protocol,
    pub tail: Protocol,
    pub iplen: u16,

    pub tcp_info: Option<ConnectState>,
    pub ports: Option<(u16, u16)>,

    pub address_field: AddressField,
    pub protocol_field: ProtocolInfoField,
    pub bitmap: u32,
}

impl Frame {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }
    /**
     * frame range with segments
     */
    pub fn range(&self) -> Option<Range<usize>> {
        if let ProtocolInfoField::TLS(tls_list) = &self.protocol_field {
            if !tls_list.list.is_empty() {
                let mut start = tls_list.list.first().unwrap().segments.first().unwrap().range.start;
                let mut end = tls_list.list.last().unwrap().segments.last().unwrap().range.end;
                if let Some(r) = &self.range {
                    start = cmp::min(start, r.start);
                    end = cmp::max(end, r.end);
                }
                return Some(start..end);
            }
        }
        self.range.clone()
    }
    pub fn compact(&self) -> bool {
        if let ProtocolInfoField::TLS(tls_list) = &self.protocol_field {
            if tls_list.list.len() > 0 {
                return false;
            }
        }
        true
    }
    pub fn frame_range(&self) -> Option<Range<usize>> {
        self.range.clone()
    }
    pub fn tcp_description(&self) -> Option<String> {
        if let Some(stat) = &self.tcp_info {
            let mut source_port = 0;
            let mut target_port = 0;
            if let Some(ports) = &self.ports {
                source_port = ports.0;
                target_port = ports.1;
            }
            let state = TcpFlagField::from(stat.flag_bit);
            return Some(format!("{} -> {} {} Seq={} Len={} ", source_port, target_port, state.list_str(), stat.seq, stat.len));
        }
        None
    }
    pub fn addresses(&self, ctx: &Context) -> Option<(String, String)> {
        match &self.address_field {
            AddressField::IPv4(s, t) => Some((s.to_string(), t.to_string())),
            AddressField::IPv6(key) => {
                if let Some((_, s, t)) = ctx.ipv6map.get(key) {
                    return Some((s.to_string(), t.to_string()));
                }
                None
            }
            _ => None,
        }
    }
}

impl Frame {
    pub fn add_proto(&mut self, proto: ProtoMask) {
        self.bitmap |= proto.bit();
    }
    pub fn rm_proto(&mut self, proto: ProtoMask) {
        self.bitmap &= !proto.bit();
    }
    pub fn has_proto(&self, proto: ProtoMask) -> bool {
        (self.bitmap & proto.bit()) != 0
    }
    pub fn all_protos(&self) -> Vec<NString> {
        let mut list = Vec::new();
        for proto in ProtoMask::ALL {
            if self.has_proto(proto) {
                list.push(proto.name());
            }
        }
        list
    }
}

pub struct EthernetCache {
    pub source: MacAddress,
    pub target: MacAddress,
    // pub info: NString,
    pub ptype: u16,
}

impl EthernetCache {
    pub fn new(source: MacAddress, target: MacAddress, ptype: u16) -> Self {
        Self { source, target, ptype }
    }
}

pub struct Instance<T> {
    loader: T,
    ds: DataSource,
    file_type: FileType,
    ctx: Context,
    last: usize,
}

impl<T> Instance<T> {
    pub fn context(&self) -> &Context {
        &self.ctx
    }

    pub fn frame(&self, index: usize) -> Option<&Frame> {
        self.ctx.list.get(index)
    }
}

impl<T> Instance<T>
where
    T: ResourceLoader,
{
    pub fn new(batch_size: usize, loader: T) -> Self {
        let size = cmp::max(batch_size, 1024 * 128);
        let ds = DataSource::new(size, 0);
        Self {
            loader,
            ds,
            file_type: FileType::NONE,
            ctx: Context::new(),
            last: 0,
        }
    }

    pub fn loader(&self) -> &dyn ResourceLoader {
        &self.loader
    }

    pub fn parse(&mut self) -> Result<ProgressStatus> {
        let mut reader = Reader::new(&self.ds);
        reader.cursor = self.last;
        if let FileType::NONE = self.file_type {
            let head: &[u8] = self.ds.slice(0..4)?;
            let head_str = format!("{:x}", IO::read32(head, false)?);
            match head_str.as_str() {
                "a1b2c3d4" => {
                    let _ = reader.read32(true)?;
                    let _major = reader.read16(false)?;
                    let _minor = reader.read16(false)?;
                    reader.forward(8);
                    let _snap_len = reader.read32(false)?;
                    self.ctx.link_type = reader.read32(false)?;
                    self.file_type = FileType::PCAP;
                    self.ctx.file_type = FileType::PCAP;
                }
                "a0d0d0a" => {
                    self.file_type = FileType::PCAPNG;
                    self.ctx.file_type = FileType::PCAPNG;
                }
                _ => bail!(DataError::UnsupportFileType),
            };
        }
        let ds = &self.ds;
        let cxt = &mut self.ctx;
        loop {
            let rs = match self.file_type {
                FileType::PCAP => PCAP::next(&mut reader).map(|(_next, frame)| (_next, Some(frame))),
                FileType::PCAPNG => PCAPNG::next(cxt, &mut reader),
                _ => {
                    bail!(DataError::UnsupportFileType)
                }
            };
            match rs {
                Ok((next, _frame)) => {
                    if let Some(frame) = _frame {
                        Instance::<T>::parse_packet(cxt, frame, ds);
                    }
                    reader.cursor = next;
                }
                Err(e) => {
                    self.last = reader.cursor;
                    if e.is::<DataError>() {
                        let tp = e.downcast::<DataError>().unwrap();
                        match tp {
                            DataError::EndOfStream => {
                                break;
                            }
                            _ => {
                                bail!(DataError::FormatMismatch);
                            }
                        }
                    }
                    bail!(DataError::FormatMismatch);
                }
            }
        }
        let mut rs: ProgressStatus = (&reader).into();
        rs.count = self.ctx.list.len();
        rs.left = reader.left();

        let _cursor = self.last;
        let datasource = &mut self.ds;
        datasource.trim(_cursor)?;
        Ok(rs)
    }
    pub fn parse_packet(ctx: &mut Context, mut frame: Frame, ds: &DataSource) {
        if let Some(range) = &frame.range {
            let mut _reader = Reader::new_sub(ds, range.clone()).unwrap();
            let proto: Protocol = link_type_map(&ctx.file_type, ctx.link_type, &mut _reader);
            frame.range = Some(range.clone());
            frame.head = proto;
            frame.tail = proto;
            frame.info.index = ctx.counter;
            ctx.counter += 1;
            let mut _next = proto;
            loop {
                match &_next {
                    Protocol::None => {
                        break;
                    }
                    _ => {
                        if let Ok(next) = parse(_next, ctx, &mut frame, &mut _reader) {
                            frame.tail = _next;
                            _next = next;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        ctx.list.push(frame);
    }
    pub fn update(&mut self, data: Vec<u8>) -> Result<ProgressStatus> {
        self.ds.update(data);
        self.parse()
    }
    pub fn update_slice(&mut self, data: &[u8]) -> Result<ProgressStatus> {
        self.ds.update_slice(data);
        self.parse()
    }
    pub fn destroy(&mut self) -> bool {
        self.ds.destroy();
        self.ctx = Context::new();
        self.last = 0;
        self.file_type = FileType::NONE;
        true
    }
}

fn conversation_list<V: AsRef<[T]>, T>(start: usize, size: usize, v: V) -> ListResult<VConversation>
where
    T: Borrow<concept::Conversation>,
{
    let slice = v.as_ref();
    let total = slice.len();
    let end = cmp::min(start + size, total);
    if end <= start {
        return ListResult::new(start, 0, vec![]);
    }
    let _data = &slice[start..end];
    let mut list = vec![];
    for item in _data {
        list.push(item.borrow().into());
    }
    ListResult::new(start, total, list)
}

impl<T> Instance<T> {
    pub fn stat_frame(&self) -> LineChartData {
        self.context().stat_frame()
    }
    pub fn stat_ip4(&self) -> Vec<CounterItem> {
        self.context().stat_ip4()
    }
    pub fn stat_ip6(&self) -> Vec<CounterItem> {
        self.context().stat_ip6()
    }
    pub fn stat_http_host(&self) -> Vec<CounterItem> {
        self.context().stat_http_host()
    }
    pub fn stat_http(&self) -> Vec<Vec<CounterItem>> {
        self.context().stat_http_data()
    }
    pub fn stat_ipaddress_distribute(&self) -> Vec<CounterItem> {
        let get_ip4_type = |addr: &Ipv4Addr| {
            if addr.is_loopback() {
                "loopback"
            } else if addr.is_broadcast() {
                "broadcast"
            } else if addr.is_multicast() {
                "multicast"
            } else if addr.is_private() {
                "private"
            } else if addr.is_link_local() {
                "link_local"
            } else if addr.is_documentation() {
                "documentation"
            } else {
                "public"
            }
        };
        let get_ip6_type = |addr: &Ipv6Addr| {
            if addr.is_loopback() {
                "loopback"
            } else if addr.is_multicast() {
                "multicast"
            } else if addr.is_unique_local() {
                "unique_local"
            } else if addr.is_unicast_link_local() {
                "unicast_link_local"
            } else {
                "public"
            }
        };

        let mut map: FastHashMap<String, usize> = FastHashMap::default();
        let ipv4 = String::from("IPv4");
        let ipv6 = String::from("IPv6");
        for frame in &self.ctx.list {
            match &frame.address_field {
                AddressField::IPv4(source, target) => {
                    Context::add_map(&ipv4, &mut map);
                    {
                        let _type = get_ip4_type(source);
                        let str = _type.to_string();
                        Context::add_map(&str, &mut map);
                    }
                    {
                        let _type = get_ip4_type(target);
                        let str = _type.to_string();
                        Context::add_map(&str, &mut map);
                    }
                }
                AddressField::IPv6(key) => {
                    Context::add_map(&ipv6, &mut map);
                    if let Some((_, source, target)) = self.ctx.ipv6map.get(key) {
                        {
                            let _type = get_ip6_type(source);
                            let str = _type.to_string();
                            Context::add_map(&str, &mut map);
                        }
                        {
                            let _type = get_ip6_type(target);
                            let str = _type.to_string();
                            Context::add_map(&str, &mut map);
                        }
                    }
                }
                _ => {}
            }
        }
        Context::_list_map(&map)
    }
}

impl<T> Instance<T>
where
    T: ResourceLoader,
{
    pub fn frames_by(&self, cri: Criteria) -> ListResult<FrameInfo> {
        let start = cri.start;
        let size = cri.size;
        let fs: &[Frame] = &self.ctx.list;
        let total = fs.len();
        let mut items = Vec::new();
        if total <= start {
            return ListResult::new(start, 0, Vec::new());
        }
        let end = cmp::min(start + size, total);
        let _data = &fs[start..end];
        for frame in _data.iter() {
            let mut info = FrameInfo::from(&frame.info);
            match &frame.address_field {
                AddressField::IPv4(s, t) => {
                    info.source = s.to_string();
                    info.dest = t.to_string();
                }
                AddressField::IPv6(key) => {
                    if let Some((_, s, t)) = self.ctx.ipv6map.get(key) {
                        info.source = s.to_string();
                        info.dest = t.to_string();
                    }
                }
                AddressField::Mac(key) => {
                    if let Some(cache) = self.ctx.ethermap.get(key) {
                        info.source = cache.source.to_string();
                        info.dest = cache.target.to_string();
                    }
                }
                _ => {
                    // frame.info_field
                }
            }
            info.protocol = frame.tail.to_string().to_lowercase();

            if let Some(summary) = summary(frame.tail, &self.ctx, frame) {
                info.info = summary;
            }

            items.push(info);
        }
        ListResult::new(start, total, items)
    }
    fn intern_frame_field(&self, frame: &Frame) -> Field {
        let mut f = Field::children();
        if let Some(range) = frame.range.as_ref() {
            f.start = range.start;
            f.size = range.end - range.start;
        }
        let _index = frame.info.index + 1;
        let size = frame.info.len;
        let interface_type = self.ctx.link_type;
        f.summary = format!(
            "Frame {}: {} bytes on wire ({} bits), {} bytes captured ({} bits) on interface {}",
            _index,
            size,
            size * 8,
            size,
            size * 8,
            interface_type
        );
        add_field_label_no_range!(f, format!("Frame number: {}", _index));
        add_field_label_no_range!(f, format!("Epoch Arrival Time: {}", date_str(frame.info.time)));
        add_field_label_no_range!(f, format!("Interface id: {}", interface_type));
        add_field_label_no_range!(f, format!("Frame length: {}", size));
        add_field_label_no_range!(f, format!("Capture length: {}", size));
        f
    }

    pub fn frame_datasource(&self, frame: &Frame) -> Option<DataSource> {
        if let Some(range) = frame.frame_range() {
            if let Ok(data) = self.loader.load(&range) {
                let ds = DataSource::create(data, range);
                return Some(ds);
            }
        }
        None
    }
    pub fn select_frame(&self, index: usize) -> Option<(Vec<Field>, Vec<DataSource>)> {
        if let Some(frame) = self.frame(index) {
            if let Some(range) = frame.frame_range() {
                let data = self.loader.load(&range).unwrap();
                let ds = DataSource::create(data, range);
                let mut datasources = vec![];
                let mut reader = Reader::new(&ds);
                let mut list = vec![];
                let mut _next = frame.head;
                list.push(self.intern_frame_field(frame));
                loop {
                    match &_next {
                        Protocol::None => {
                            break;
                        }
                        _ => {
                            let mut f = Field::children();
                            f.start = reader.cursor;
                            if let Ok((next, _extra_data)) = detail(_next, &mut f, self, frame, &mut reader, &mut datasources) {
                                f.size = reader.cursor - f.start;
                                list.push(f);
                                _next = next;
                            } else {
                                f.summary = format!("Parse [{_next}] failed");
                                break;
                            }
                        }
                    }
                }
                datasources.insert(0, ds);
                return Some((list, datasources));
                // return Some((list, source, extra_data, range));
            }
        }
        None
    }

    pub fn conversations(&self, cri: Criteria, filter: ConversationCriteria) -> ListResult<VConversation> {
        let Criteria { start, size } = cri;
        if let Some(ip) = &filter.ip {
            let c_list: Vec<&concept::Conversation> = self.ctx.conversation_list.iter().filter(|conv| conv.match_ip(ip)).collect();
            conversation_list(start, size, c_list)
        } else {
            conversation_list(start, size, &self.ctx.conversation_list)
        }
    }
    pub fn connections(&self, conversation_index: usize, cri: Criteria) -> ListResult<VConnection> {
        if let Some(connects) = self.ctx.conversation_list.get(conversation_index) {
            paging_into(&connects.connections, cri, |f| f.into())
        } else {
            ListResult::empty()
        }
    }

    fn iter_http<'a>(&'a self, asc: bool) -> Box<dyn Iterator<Item = &'a HttpConntect>+'a> {
        if asc {
            Box::new(self.ctx.http_connections.iter())
        } else {
            Box::new(self.ctx.http_connections.iter().rev())
        }
    }
    pub fn http_connections(&self, cri: Criteria, filter: Option<HttpCriteria>, asc: bool) -> ListResult<VHttpConnection> {
        // let first = self.context().list.first().unwrap().info.time;
        let Criteria { start, size } = cri;
        let mut total = 0;
        let mut list = vec![];
        let itertor = self.iter_http(asc);
        for (index, item) in itertor.enumerate() {
            let count = list.len();
            if item.do_match(&filter) {
                if total >= start && count < size {
                    list.push(item.conv(&self.ctx, index));
                }
                total += 1;
            }
        }
        ListResult::new(start, total, list)
    }

    pub fn http_detail(&self, index: usize) -> Option<Vec<HttpMessageDetail>> {
        let loader = &self.loader;
        if let Some(http_connect) = self.ctx.http_connections.get(index) {
            http_connect.convert_to_detail(&self.ctx, loader).ok()
        } else {
            None
        }
    }

    fn intern_udp_conversations(&self, ip: Option<String>, asc: bool) -> Vec<UDPConversation> {
        let mut map = FastHashMap::<String, UDPConversation>::default();
        let first = self.ctx.list.first().unwrap().info.time;
        for frame in &self.ctx.list {
            if frame.has_proto(ProtoMask::UDP) {
                if let Some((source, target)) = frame.addresses(&self.ctx) {
                    if let Some((source_port, target_port)) = frame.ports {
                        let index = frame.info.index as usize;
                        let len = frame.info.len as usize;
                        let time = frame.info.time;
                        let mut add = || {
                            let key = format!("{source}:{source_port}-{target}:{target_port}");
                            if let Some(item) = map.get_mut(&key) {
                                item.incr(len, time);
                            } else {
                                let mut item = UDPConversation::new(index, time, source.clone(), target.clone(), source_port, target_port);
                                item.incr(len, time);
                                item.init(first);
                                map.insert(key, item);
                            }
                        };
                        if let Some(_ip) = &ip {
                            if *_ip == source || *_ip == target {
                                add();
                            }
                        } else {
                            add();
                        }
                    }
                }
            }
        }
        let mut rs: Vec<UDPConversation> = map.into_iter().map(|(_, v)| v).collect();
        let compare = |a: &UDPConversation, b: &UDPConversation| {
            let rs = a.ts.cmp(&b.ts);
            if asc {
                rs
            } else {
                rs.reverse()
            }
        };
        rs.sort_by(compare);
        rs
    }
    pub fn udp_conversations(&self, cri: Criteria, filter: Option<String>, asc: bool) -> ListResult<UDPConversation> {
        let list = self.intern_udp_conversations(filter, asc);
        paging(&list, cri)
    }

    fn intern_parse_handshake(&self, tls_data: &TlsData, msg_type: u8, item: &mut TLSItem) -> Result<()> {
        let ranges: Vec<Range<usize>> = tls_data.segments.iter().map(|f| f.range.clone()).collect();
        let data = self.loader.loads(&ranges)?;
        let ds = DataSource::create(data, 0..0);
        let mut reader = Reader::new(&ds);
        reader.read8()?;
        reader.forward(2);
        let len1 = reader.read16(true)?;
        let mt = reader.read8()?;
        if mt != msg_type {
            bail!("msg type missmatch")
        }
        let len2 = reader.read24()?;
        let diff = len1.saturating_sub(len2 as u16);
        if diff == 4 {
            match mt {
                1 => {
                    reader.forward(34);
                    let session_id_len = reader.read8()? as usize;
                    reader.forward(session_id_len);
                    let cipher_suite_len = reader.read16(true)?;
                    reader.forward(cipher_suite_len as usize);
                    let compression_len = reader.read8()?;
                    reader.forward(compression_len as usize);
                    let extention_len = reader.read16(true)?;
                    if extention_len > 0 {
                        let mut extention_reader = reader.slice_as_reader(extention_len as usize)?;
                        return resolve_sni(&mut extention_reader, item);
                    }
                }
                2 => {
                    reader.forward(34);
                    let session_id_len = reader.read8()? as usize;
                    reader.forward(session_id_len);
                    let cs = reader.read16(true)?;
                    // let suites = crate::constants::tls_cipher_suites_mapper(cs).to_string();
                    // item.cipher_suite = Some(suites);
                    item.set_cipher_suite(cs);

                    reader.forward(1);
                    let extention_len = reader.read16(true)?;
                    if extention_len > 0 {
                        let mut extention_reader = reader.slice_as_reader(extention_len as usize)?;
                        return resolve_alpn(&mut extention_reader, item);
                    }
                }
                _ => {}
            }
        }

        bail!("")
    }

    fn intern_resolve_tls_handshake(&self, index: FrameIndex, msg_type: u8, item: &mut TLSItem) -> Result<()> {
        if let Some(frame) = self.ctx.list.get(index as usize) {
            if let ProtocolInfoField::TLS(tls_list) = &frame.protocol_field {
                for block in &tls_list.list {
                    if block.content_type == 22 {
                        return self.intern_parse_handshake(block, msg_type, item);
                    }
                }
            }
        }
        bail!("")
    }

    fn intern_tls_connects(&self) -> Vec<usize> {
        let mut list = vec![];
        for conver in &self.ctx.conversation_list {
            for connection in &conver.connections {
                if connection.tls_meta.exists() {
                    list.push(conver.key);
                    break;
                }
            }
        }
        list
    }
    fn intern_tls_conv_from_index(&self, conversaction_index: usize, complete: bool) -> Option<TLSConversation> {
        if let Some(conv) = self.ctx.conversation_list.get(conversaction_index) {
            let mut rs = TLSConversation::new(conversaction_index, conv.primary.clone(), conv.second.clone());
            let primary_host = &conv.primary;
            let second_host = &conv.second;
            let mut map = FastHashMap::default();
            for connection in &conv.connections {
                let meta = &connection.tls_meta;
                if meta.exists() {
                    let p_port = connection.primary.port;
                    let s_port = connection.second.port;

                    let mut item = TLSItem::default();
                    if let Some(index) = meta.client() {
                        self.intern_resolve_tls_handshake(index, 1, &mut item).ok();
                    }
                    if let Some(index) = meta.server() {
                        self.intern_resolve_tls_handshake(index, 2, &mut item).ok();
                    }
                    if complete {
                        item.addr_1 = Some(format!("{primary_host}:{p_port}"));
                        item.addr_2 = Some(format!("{second_host}:{s_port}"));
                    }
                    let key = item.get_trait();
                    let _item = map.entry(key).or_insert(item);
                    _item.update();
                }
            }
            rs.list = map.into_values().collect();
            Some(rs)
        } else {
            None
        }
    }

    pub fn tls_connections(&self, cri: Criteria) -> ListResult<TLSConversation> {
        let Criteria { start, size } = cri;
        let list = self.intern_tls_connects();
        let total = list.len();
        let end = cmp::min(start + size, total);

        if end <= start {
            ListResult::new(start, 0, vec![])
        } else {
            let _list: &[usize] = &list[start..end];
            let mut items = vec![];
            for t_index in _list {
                if let Some(item) = self.intern_tls_conv_from_index(*t_index, false) {
                    items.push(item);
                }
            }
            ListResult::new(start, total, items)
        }
    }

    pub fn tls_conv_list(&self, conv_index: usize, cri: Criteria) -> ListResult<TLSItem> {
        if let Some(conv) = self.intern_tls_conv_from_index(conv_index, true) {
            paging(&conv.list, cri)
        } else {
            ListResult::new(cri.start, 0, vec![])
        }
    }

    fn intern_dns_list(&self, asc: bool) -> Vec<DNSResponse> {
        let mut map: IndexHashMap<u16, DNSResponse> = IndexHashMap::default();
        for frame in &self.context().list{
            let index = frame.info.index;
            match &frame.protocol_field {
                ProtocolInfoField::DNSQUERY(NameService::DNS, transaction_id, _) => {
                    if *transaction_id != 0 {
                        let (_, rs) = map.get_or_add(transaction_id);
                        rs.transaction_id = *transaction_id;
                        rs.request = Some(index as usize);
                        if let Some(frame) = self.context().frame(index) {
                            rs.req_ts = frame.info.time;
                        }
                        
                    }
                }
                ProtocolInfoField::DNSRESPONSE(NameService::DNS, transaction_id, _) => {
                    if *transaction_id != 0 {
                        let (_, rs) = map.get_or_add(transaction_id);
                        rs.transaction_id = *transaction_id;
                        rs.response = Some(index as usize);
                        if let Some(frame) = self.context().frame(index) {
                            rs.res_ts = frame.info.time;
                        }
                    }
                }
                _ => {}
            }
        }

        let mut rs: Vec<DNSResponse> = map.list().into_iter().filter(|f| f.is_complete()).collect();
        for record in rs.iter_mut() {
            record.fix_offset();
        }
        let compare = |a: &DNSResponse, b: &DNSResponse| {
            let rs = a._latency.cmp(&b._latency);
            if asc {
                rs
            } else {
                rs.reverse()
            }
        };
        rs.sort_by(compare);
        rs
    }

    pub fn dns_records(&self, cri: Criteria, asc: bool) -> ListResult<DNSResponse> {
        let first = self.context().list.first().unwrap().info.time;
        let list = self.intern_dns_list(asc);
        let convert = |f: &DNSResponse| DNSResponse::convert(self, f, first);
        paging_into(&list, cri, convert)
    }
    pub fn dns_record(&self, index: usize, cri: Criteria) -> ListResult<DNSRecord> {
        if let Some(frame) = self.ctx.list.get(index) {
            if let ProtocolInfoField::DNSRESPONSE(_, _, start) = &frame.protocol_field {
                if let Some(ds) = self.frame_datasource(frame) {
                    let mut reader = Reader::new(&ds);
                    reader.cursor = *start;
                    if let Ok(list) = dns::Visitor::answers(&mut reader) {
                        return paging(&list, cri);
                    }
                }
            }
        }
        ListResult::empty()
    }
}

fn paging<T>(list: &[T], cri: Criteria) -> ListResult<T>
where
    T: Clone,
{
    let Criteria { start, size } = cri;
    let total = list.len();
    let end = cmp::min(start + size, total);
    if end <= start {
        ListResult::new(start, 0, vec![])
    } else {
        let items = list[start..end].to_vec();
        ListResult::new(start, total, items)
    }
}

fn paging_into<T, R, F>(list: &[T], cri: Criteria, convert: F) -> ListResult<R>
where
    F: Fn(&T) -> R,
{
    let Criteria { start, size } = cri;
    let total = list.len();
    let end = cmp::min(start + size, total);
    if end <= start {
        ListResult::new(start, 0, vec![])
    } else {
        let items = list[start..end].iter().map(convert).collect();
        ListResult::new(start, total, items)
    }
}

fn resolve_sni(reader: &mut Reader, item: &mut TLSItem) -> Result<()> {
    while reader.left() >= 4 {
        let extention_type = reader.read16(true)?;
        let extention_len = reader.read16(true)?;
        if extention_type == 0 {
            let mut ext_reader = reader.slice_as_reader(extention_len as usize)?;
            ext_reader.forward(5);
            let sni = ext_reader.read_string((extention_len - 5) as usize)?;
            item.hostname = Some(sni);
        } else {
            reader.forward(extention_len as usize);
        }
    }
    Ok(())
}

fn resolve_alpn(reader: &mut Reader, item: &mut TLSItem) -> Result<()> {
    while reader.left() >= 6 {
        let extention_type = reader.read16(true)?;
        let _extention_len = reader.read16(true)?;
        match extention_type {
            16 => {
                let mut list = vec![];
                let extention_len = reader.read16(true)?;
                let mut ext_reader = reader.slice_as_reader(extention_len as usize)?;
                loop {
                    if ext_reader.left() == 0 {
                        break;
                    }
                    let item_len = ext_reader.read8()?;
                    let item = ext_reader.read_string(item_len as usize)?;
                    list.push(item);
                }
                item.alpn = Some(list);
            }
            43 => {
                if _extention_len == 2 {
                    let v = reader.read16(true)?;
                    item.set_version(v);
                    // item.version = tls_version_map(v).map(String::from);
                }
            }
            _ => {
                reader.forward(_extention_len as usize);
            }
        }
    }
    Ok(())
}

pub mod concept;
pub mod connection;
pub mod core;
pub mod enum_def;
pub mod io;
pub mod macro_def;
pub mod util;
