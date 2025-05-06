use std::{cmp, ops::Range};

use anyhow::{bail, Result};
use concept::{Criteria, Field, FrameInfo, ListResult, ProgressStatus};
use enum_def::{DataError, FileType, Protocol};
use io::{DataSource, Reader, IO};
use serde_json::Error;

use crate::{
    files::{pcap::PCAP, pcapng::PCAPNG},
    protocol::{link_type_map, parse},
};

pub fn range64(range: Range<usize>) -> Range<u64> {
    range.start as u64..range.end as u64
}

#[derive(Default)]
pub struct Frame {
    // pub index: u32,
    // pub size: u32,
    pub range: Option<Range<usize>>,
    // pub time: Option<Vec<u8>>,
    pub info: FrameInfo,
    pub element: Vec<ProtocolElement>,
}

impl Frame {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }
}

#[derive(Default)]
pub struct Context {
    file_type: FileType,
    pub link_type: u32,
    pub list: Vec<Frame>,
    pub counter: u32,
}

impl Context {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }
}

pub struct Instance {
    ds: DataSource,
    file_type: FileType,
    ctx: Context,
    last: usize,
}

pub trait Element {
    fn title(&self) -> &'static str;
    fn position(&self) -> Option<Range<u64>>;
    fn children(&self) -> Option<&[FieldElement]>;
}

#[derive(Default)]
pub struct FieldElement {
    pub title: &'static str,
    pub position: Option<Range<u64>>,
    pub children: Option<Vec<FieldElement>>,
}

impl FieldElement {
    pub fn create(title: &'static str, position: Option<Range<u64>>) -> Self {
        Self { title, position, children: None }
    }
}

impl Element for FieldElement {
    fn title(&self) -> &'static str {
        self.title
    }

    fn position(&self) -> Option<Range<u64>> {
        self.position.clone()
    }

    fn children(&self) -> Option<&[FieldElement]> {
        self.children.as_deref()
    }
}

pub struct ProtocolElement {
    pub protocol: Protocol,
    pub element: FieldElement,
}
impl ProtocolElement {
    pub fn new(protocol: Protocol) -> Self {
        Self {
            protocol,
            element: FieldElement::default(),
        }
    }
}
impl Element for ProtocolElement {
    fn title(&self) -> &'static str {
        self.element.title
    }

    fn position(&self) -> Option<Range<u64>> {
        self.element.position.clone()
    }

    fn children(&self) -> Option<&[FieldElement]> {
        self.element.children.as_deref()
    }
}

impl Into<Field> for &FieldElement {
    fn into(self) -> Field {
        let mut field = Field {
            start: 0,
            size: 0,
            summary: self.title,
            children: None,
        };
        if let Some(range) = &self.position {
            field.start = range.start;
            field.size = range.end - range.start;
        }
        if let Some(children) = &self.children {
            field.children = Some(children.iter().map(|f| f.into()).collect());
        }
        field
    }
}

impl Into<Field> for &Frame {
    fn into(self) -> Field {
        let mut field = Field {
            start: 0,
            size: 0,
            summary: "",
            children: None,
        };
        if let Some(range) = &self.range {
            field.start = range.start as u64;
            field.size = range.end as u64 - field.start;
        }
        field.children = Some(self.element.iter().map(|f| (&f.element).into()).collect());
        field
    }
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
            let mut _reader = Reader::new_sub(&ds, range.clone());
            let proto = link_type_map(&ctx.file_type, ctx.link_type, &mut _reader);
            frame.range = Some(range.clone());
            let mut _next = proto;
            loop {
                match _next {
                    "none" => {
                        break;
                    }
                    _ => {
                        frame.info.protocol = _next;
                        let _start = _reader.cursor;
                        if let Ok((next, mut pe)) = parse(_next, &mut frame, &mut _reader){
                            let _end = _reader.cursor;
                            pe.element.position = Some(range64(_start.._end));
                            frame.element.push(pe);
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
        frame.info.index = ctx.counter;
        ctx.counter += 1;
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

    pub fn select_frame(&self, index: usize) -> Result<String, Error> {
        if let Some(frame) = self.ctx.list.get(index) {
            let fs: Field = frame.into();
            serde_json::to_string(&fs)
        } else {
            Ok("{}".into())
        }
    }
}
pub mod concept;
pub mod core;
pub mod enum_def;
pub mod io;
pub mod macro_def;
