use std::{cmp, collections::HashMap, hash::BuildHasherDefault, ops::Range, rc::Rc};

use anyhow::{bail, Result};
use concept::{Criteria, Field, FrameInfo, ListResult, ProgressStatus};
use connection::{ConnectState, Connection, Endpoint, TCPStat, TmpConnection};
use enum_def::{DataError, FileType, Protocol};
use io::{DataSource, MacAddress, Reader, IO};
use rustc_hash::FxHasher;
use serde_json::Error;

type FastHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;
pub type NString = &'static str;
use crate::{
    files::{pcap::PCAP, pcapng::PCAPNG},
    protocol::{detail, link_type_map, parse},
};

pub fn range64(range: Range<usize>) -> Range<u64> {
    range.start as u64..range.end as u64
}

pub struct Ethernet {
    pub source: MacAddress,
    pub destination: MacAddress,
    pub protocol_type: u16,
}

#[derive(Default)]
pub struct Frame {
    pub range: Option<Range<usize>>,
    pub info: FrameInfo,
    pub head: Protocol,
    pub iplen: u16,
    pub source: NString,
    pub target: NString,
    pub tcp_info: Option<ConnectState>,
    // pub element: Vec<FieldDef>,
}

impl Frame {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }
    pub fn range(&self) -> Option<Range<usize>>{
        self.range.clone()
    }
}

#[derive(Default)]
pub struct Context {
    file_type: FileType,
    pub link_type: u32,
    pub list: Vec<Frame>,
    pub counter: u32,
    pub ethernet: FastHashMap<u64, Rc<Ethernet>>,
    pub connections: FastHashMap<(&'static str, u16, &'static str, u16), Connection>,
}

impl Context {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }
    pub fn get_connect(&mut self, host1:&'static str, port1: u16, host2: &'static str, port2: u16, stat: TCPStat) -> ConnectState {
        let mut key = (host1, port1, host2, port2);
        let mut reverse = true;

        if !self.connections.contains_key(&key) {
            key = (host2, port2, host1, port1);
            reverse = false;
        }

        if self.connections.contains_key(&key) {
            // 
        } else {
            let connection = Connection::new(Endpoint::new(host1, port1), Endpoint::new(host2, port2));
            self.connections.insert(key, connection);
        }
        let conn = self.connections.get_mut(&key).unwrap();
        let mut tmp_conn = TmpConnection::new(conn, reverse);
        tmp_conn.update(&stat)
    }
}

pub struct Instance {
    ds: DataSource,
    file_type: FileType,
    ctx: Context,
    last: usize,
}

// #[enum_dispatch(FieldDef)]
// pub trait Element {
//     fn title(&self) -> NString;
//     fn position(&self) -> Option<Range<u64>>;
//     fn children(&self) -> Option<&[FieldDef]>;
// }

// #[derive(Default)]
// pub struct FieldElement {
//     pub title: NString,
//     pub position: Option<Range<u64>>,
//     pub children: Option<Vec<FieldDef>>,
// }

// impl FieldElement {
//     pub fn create(title: NString, position: Option<Range<u64>>) -> Self {
//         Self { title, position, children: None }
//     }
// }

// impl Element for FieldElement {
//     fn title(&self) -> NString {
//         self.title
//     }

//     fn position(&self) -> Option<Range<u64>> {
//         self.position.clone()
//     }

//     fn children(&self) -> Option<&[FieldDef]> {
//         self.children.as_deref()
//     }
// }

// pub struct ProtocolElement {
//     pub protocol: Protocol,
//     pub element: FieldElement,
// }
// impl ProtocolElement {
//     pub fn new(protocol: Protocol) -> Self {
//         Self {
//             protocol,
//             element: FieldElement::default(),
//         }
//     }
// }
// impl Element for ProtocolElement {
//     fn title(&self) -> &'static str {
//         self.element.title
//     }

//     fn position(&self) -> Option<Range<u64>> {
//         self.element.position.clone()
//     }

//     fn children(&self) -> Option<&[FieldElement]> {
//         self.element.children.as_deref()
//     }
// }

// impl Into<Field> for &FieldDef {
//     fn into(self) -> Field {
//         let mut field = Field {
//             start: 0,
//             size: 0,
//             summary: self.title(),
//             children: None,
//         };
//         if let Some(range) = &self.position() {
//             field.start = range.start;
//             field.size = range.end - range.start;
//         }
//         if let Some(children) = self.children() {
//             field.children = Some(children.iter().map(|f| f.into()).collect());
//         }
//         field
//     }
// }

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
            let mut _reader = Reader::new_sub(&ds, range.clone());
            let proto: Protocol = link_type_map(&ctx.file_type, ctx.link_type, &mut _reader);
            frame.range = Some(range.clone());
            frame.head = proto;
            frame.info.index = ctx.counter;
            ctx.counter += 1;
            let mut _next = proto;
            loop {
                match &_next {
                    Protocol::None => {
                        break;
                    }
                    _ => {
                        frame.info.protocol = format!("{}", _next);
                        // let _start = _reader.cursor;
                        if let Ok(next) = parse(_next, ctx, &mut frame, &mut _reader) {
                            // let _end = _reader.cursor;
                            // pe.element.position = Some(range64(_start.._end));
                            // frame.element.push(pe);
                            _next = next;
                        } else {
                            break;
                        }
                    }
                }
            }
            // let mut _start = _reader.cursor;
            // if let Ok((next, mut pe)) = parse(proto, &mut frame, &mut _reader) {
            //     let mut _end = _reader.cursor;
            //     pe.element.position = Some(range64(_start.._end));
            //     frame.element.push(pe);
            //     let mut _next = next;
            //     loop {
            //         match _next {
            //             "none" => {
            //                 break;
            //             }
            //             _ => {
            //                 _start = _reader.cursor;
            //                 if let Ok((next, mut pe)) = parse(next, &mut frame, &mut _reader){
            //                     _end = _reader.cursor;
            //                     _next = next;
            //                     pe.element.position = Some(range64(_start.._end));
            //                     frame.element.push(pe);
            //                 } else {
            //                     break;
            //                 }
            //             }
            //         }
            //     }
            // }
            // println!("proto: {}", _proto);
            // frame.children = Vec::new();
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
    pub fn frames_by(&self, cri: Criteria) -> ListResult<&FrameInfo> {
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
            items.push(&frame.info);
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
                            let mut f = Field::empty();
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
                return Some(list)
            }
        }
        None
    }

    
    pub fn select_frame_json(&self, index: usize, data: Vec<u8>) -> Result<String, Error> {
        if let Some(list) = self.select_frame(index, data) {
            return serde_json::to_string(&list);
        }
        Ok("{}".into())
    }
}
pub mod concept;
pub mod core;
pub mod enum_def;
pub mod io;
pub mod macro_def;
pub mod connection;
