use core::Context;
use std::{
    cmp,
    collections::HashMap,
    hash::{BuildHasherDefault, Hash, Hasher},
    ops::Range,
};

use crate::{
    files::{pcap::PCAP, pcapng::PCAPNG},
    protocol::{detail, link_type_map, network, parse, transport::tcp},
};
use anyhow::{bail, Result};
use concept::{Criteria, Field, FrameInfo, FrameInternInfo, ListResult, ProgressStatus};
use connection::ConnectState;
use enum_def::{DataError, FileType, Protocol};
use io::{DataSource, MacAddress, Reader, IO};
use rustc_hash::FxHasher;
use serde_json::Error;

type FastHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

pub type NString = &'static str;

// pub const EP: String = String::from("");

pub fn range64(range: Range<usize>) -> Range<u64> {
    range.start as u64..range.end as u64
}

pub fn quick_hash<T>(data: T) -> u64 where T: Hash {
    let mut hasher = FxHasher::default();
    data.hash(&mut hasher);
    hasher.finish()
}

// fn quick_hash_str(s: &str) -> u64 {
//     let mut hasher = FxHasher::default();
//     s.hash(&mut hasher);
//     hasher.finish()
// }

// pub trait FrameRefer {
//     fn info(&self) -> String;
// }

// pub struct EthernetFrame {
//     data: u64,
// }

// impl FrameRefer for EthernetFrame {
//     fn info(&self) -> String {
//         todo!()
//     }
// }
// pub struct EthernetFrame2 {
//     data: u64,
// }

// impl FrameRefer for EthernetFrame2 {
//     fn info(&self) -> String {
//         todo!()
//     }
// }

pub struct Ethernet {
    pub source: MacAddress,
    pub destination: MacAddress,
    pub protocol_type: u16,
}

#[derive(Default)]
pub struct Frame {
    pub range: Option<Range<usize>>,
    pub info: FrameInternInfo,
    pub head: Protocol,
    pub tail: Protocol,
    pub iplen: u16,

    pub tcp_info: Option<ConnectState>,
    pub ipv6: Option<u64>,
    pub ipv4: Option<u64>,
    pub ports: Option<(u16, u16)>,
    pub ptr: Option<u64>
}

impl Frame {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }
    pub fn range(&self) -> Option<Range<usize>> {
        self.range.clone()
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

pub struct Instance {
    ds: DataSource,
    file_type: FileType,
    ctx: Context,
    last: usize,
}

impl Instance {
    pub fn new() -> Instance {
        let ds = DataSource::new();
        Self {
            ds,
            file_type: FileType::NONE,
            ctx: Context::new(),
            last: 0,
        }
    }

    pub fn get_context(&self) -> &Context {
        &self.ctx
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
        match self.file_type {
            FileType::PCAP => loop {
                if let Ok((_next, f)) = PCAP::next(&mut reader) {
                    Instance::parse_packet(cxt, f, ds);
                    reader.cursor = _next;
                } else {
                    self.last = reader.cursor;
                    break;
                }
            },
            FileType::PCAPNG => loop {
                if let Ok((_next, f)) = PCAPNG::next(cxt, &mut reader) {
                    if let Some(frame) = f {
                        Instance::parse_packet(cxt, frame, ds);
                    }
                    reader.cursor = _next;
                } else {
                    self.last = reader.cursor;
                    break;
                }
            },
            _ => {}
        }
        let mut rs: ProgressStatus = (&reader).into();
        rs.count = self.ctx.list.len();
        Ok(rs)
    }
    pub fn parse_packet(ctx: &mut Context, mut frame: Frame, ds: &DataSource) {
        if let Some(range) = &frame.range {
            let mut _reader = Reader::new_sub(&ds, range.clone()).unwrap();
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
    pub fn destroy(&mut self) -> bool {
        // TODO
        true
    }
}

impl Instance {
    pub fn get_count(&self, catelog: &str) -> usize {
        match catelog {
            "frame" => self.ctx.list.len(),
            _ => 0,
        }
    }
    pub fn frames_by(&self, cri: Criteria) -> ListResult<FrameInfo> {
        // let Criteria { start, size } = cri;
        // let info = self.context().get_info();
        // let start_ts = info.start_time;
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
            if let Some(ip4) = frame.ipv4 {
                if let Some((source, target)) = self.ctx.ipv4map.get(&ip4) {
                    info.source = source.to_string();
                    info.dest = target.to_string();
                }
            } else if let Some(ip6) = frame.ipv6 {
                if let Some((_, source, target)) = self.ctx.ipv6map.get(&ip6) {
                    info.source = source.to_string();
                    info.dest = target.to_string();
                }
            }
            info.protocol = frame.tail.to_string().to_lowercase();
            // let last = frame.tail;
            let frame_info = match frame.tail {
                Protocol::TCP => tcp::Visitor::info(&self.ctx, frame),
                Protocol::IP4 => network::ip4::Visitor::info(&self.ctx, frame),
                Protocol::IP6 => network::ip6::Visitor::info(&self.ctx, frame),
                _ => None
            };
            if let Some(summary) = frame_info {
                info.info = summary;
            }

            items.push(info);
        }
        ListResult::new(start, total, items)
    }
    pub fn frames_list_json(&self, cri: Criteria) -> Result<String, Error> {
        let item = self.frames_by(cri);
        serde_json::to_string(&item)
    }

    pub fn frame(&self, index: usize) -> Option<&Frame> {
        self.ctx.list.get(index)
    }
    pub fn select_frame(&self, index: usize, data: Vec<u8>) -> Option<Vec<Field>> {
        if let Some(frame) = self.frame(index) {
            if let Some(range) = &frame.range {
                let ds: DataSource = DataSource::create(data, range.clone());
                let mut reader = Reader::new(&ds);
                let mut list = vec![];
                let mut _next = frame.head;
                loop {
                    match &_next {
                        Protocol::None => {
                            break;
                        }
                        _ => {
                            let mut f = Field::default();
                            f.start = reader.cursor as u64;
                            if let Ok(next) = detail(_next, &mut f, &self.ctx, &frame, &mut reader) {
                                f.size = (reader.cursor as u64) - f.start;
                                list.push(f);
                                _next = next;
                            } else {
                                break;
                            }
                        }
                    }
                }
                return Some(list);
            }
        }
        None
    }

    pub fn select_frame_json(&self, index: usize, data: Vec<u8>) -> Result<String, Error> {
        if let Some(list) = self.select_frame(index, data) {
            return serde_json::to_string(&list);
        }
        Ok("[]".into())
    }

    pub fn connections_count(&self) -> usize {
        self.ctx.connections.len()
    }
}
pub mod concept;
pub mod connection;
pub mod core;
pub mod enum_def;
pub mod io;
pub mod macro_def;
