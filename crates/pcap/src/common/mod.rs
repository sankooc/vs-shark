// Copyright (c) 2025 sankooc
// 
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use core::Context;
use std::{
    cmp,
    collections::HashMap,
    hash::{BuildHasherDefault, Hash, Hasher},
    ops::Range,
};

use crate::{
    add_field_label_no_range,
    common::{concept::{VConnection, VConversation, VHttpConnection}, connection::TcpFlagField, util::date_str},
    files::{pcap::PCAP, pcapng::PCAPNG},
    protocol::{detail, link_type_map, parse, summary},
};
use anyhow::{bail, Result};
use concept::{Criteria, Field, FrameInfo, FrameInternInfo, ListResult, ProgressStatus};
use connection::ConnectState;
use enum_def::{AddressField, DataError, FileType, Protocol, ProtocolInfoField};
use io::{DataSource, MacAddress, Reader, IO};
use rustc_hash::FxHasher;
use serde_json::Error;

pub type FastHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

pub type NString = &'static str;

// pub const EP: String = String::from("");

pub fn range64(range: Range<usize>) -> Range<u64> {
    range.start as u64..range.end as u64
}

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
}

impl Frame {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }
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
    pub fn frame_range(&self) -> Option<Range<usize>> {
        self.range.clone()
    }
    pub fn tcp_descripion(&self) -> Option<String> {
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
    pub fn new(batch_size: usize) -> Instance {
        let size = cmp::max(batch_size, 1024 * 128);
        let ds = DataSource::new(size, 0);
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
                        Instance::parse_packet(cxt, frame, ds);
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
    pub fn frames_list_json(&self, cri: Criteria) -> Result<String, Error> {
        let item = self.frames_by(cri);
        serde_json::to_string(&item)
    }

    pub fn frame(&self, index: usize) -> Option<&Frame> {
        self.ctx.list.get(index)
    }
    fn frame_field(&self, frame: &Frame) -> Field {
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
    pub fn select_frame(&self, index: usize, data: Vec<u8>) -> Option<(Vec<Field>, Option<Vec<u8>>, Option<Vec<u8>>)> {
        let mut extra_data = None;

        if let Some(frame) = self.frame(index) {
            if let Some(range) = frame.range() {
                let ds: DataSource = DataSource::create(data, range);
                let rg = frame.frame_range().unwrap();
                let mut reader = Reader::new_sub(&ds, rg).unwrap();
                let source = reader.dump_as_vec().ok();
                let mut list = vec![];
                let mut _next = frame.head;
                list.push(self.frame_field(frame));
                loop {
                    match &_next {
                        Protocol::None => {
                            break;
                        }
                        _ => {
                            let mut f = Field::children();
                            f.start = reader.cursor;
                            if let Ok((next, _extra_data)) = detail(_next, &mut f, &self.ctx, frame, &mut reader) {
                                f.size = reader.cursor - f.start;
                                list.push(f);
                                _next = next;
                                if let Some(data) = _extra_data {
                                    extra_data = Some(data);
                                }
                            } else {
                                break;
                            }
                        }
                    }
                }
                return Some((list, source, extra_data));
            }
        }
        None
    }

    pub fn select_frame_json(&self, index: usize, data: Vec<u8>) -> Result<String, Error> {
        if let Some((list, _, _)) = self.select_frame(index, data) {
            return serde_json::to_string(&list);
        }
        Ok("[]".into())
    }

    // pub fn connection_count(&self) -> usize {
    //     self.ctx.connections.len()
    // }
    pub fn conversation_count(&self) -> usize {
        self.ctx.conversation_list.len()
    }
    pub fn conversations(&self, cri: Criteria) -> ListResult<VConversation> {
        let Criteria { start, size } = cri;
        let total = self.ctx.conversation_list.len();
        let end = cmp::min(start + size, total);
        if end <= start {
            return ListResult::new(start, 0, vec![]);
        }
        let _data = &self.ctx.conversation_list[start..end];
        let mut list = vec![];
        for item in _data {
            list.push(item.into());
        }
        ListResult::new(start, total, list)
    }
    pub fn connections(&self, conversation_index: usize, cri: Criteria) -> ListResult<VConnection> {
        if let Some(connects) = self.ctx.conversation_list.get(conversation_index) {
            let Criteria { start, size } = cri;
            let total = connects.connections.len();
            let end = cmp::min(start + size, total);
            if end <= start {
                return ListResult::empty();
            }
            let _data = &connects.connections[start..end];
            let mut list = vec![];
            for item in _data {
                list.push(item.into());
            }
            return ListResult::new(start, total, list);
        }

        ListResult::empty()
    }
    pub fn http_connections(&self, cri: Criteria) -> ListResult<VHttpConnection> {
        let Criteria { start, size } = cri;
        let total = self.ctx.http_connections.len();
        let end = cmp::min(start + size, total);
        if end <= start {
            return ListResult::new(start, 0, vec![]);
        }
        let _data = &self.ctx.http_connections[start..end];
        let mut list = vec![];
        for item in _data {
            list.push(item.into(&self.ctx));
        }
        ListResult::new(start, total, list)
    }
}
pub mod concept;
pub mod connection;
pub mod core;
pub mod enum_def;
pub mod io;
pub mod macro_def;
pub mod util;
