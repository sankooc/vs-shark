use std::ops::Range;

use anyhow::{bail, Result};
use io::{DataSource, Reader, IO};
use thiserror::Error;

use crate::{
    files::{pcap::PCAP, pcapng::PCAPNG},
    protocol::{ethernet::execute, parse},
};

pub fn range64(range: Range<usize>) -> Range<u64> {
    range.start as u64..range.end as u64
}

#[derive(Default)]
pub struct Frame {
    pub index: u32,
    pub size: u32,
    pub range: Option<Range<usize>>,
    pub time: Option<Vec<u8>>,
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

#[derive(Default, Clone, Copy)]
pub enum FileType {
    PCAP,
    PCAPNG,
    #[default]
    NONE,
}

#[derive(Default)]
pub enum Protocol {
    #[default]
    None,
    Ethernet,
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

#[derive(Error, Debug)]
pub enum DataError {
    #[error("unsupport file type")]
    UnsupportFileType,
    #[error("bit error")]
    BitSize,
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

    pub fn parse(&mut self) -> Result<()> {
        let mut reader = Reader::new(&self.ds);
        reader.cursor = self.last;
        if let FileType::NONE = self.file_type {
            let head: &[u8] = &self.ds.data[..4];
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
        Ok(())
    }
    pub fn parse_packet(ctx: &mut Context, mut frame: Frame, ds: &DataSource) {
        if let Some(range) = &frame.range {
            let mut _reader = Reader::new_sub(&ds, range.clone());
            let proto = execute(&ctx.file_type, ctx.link_type, &mut _reader);
            if let Ok((next, pe)) = parse(proto, &mut _reader) {
                frame.element.push(pe);
                let mut _next = next;
                // loop {
                //     match _next {
                //         "none" => {
                //             break;
                //         }
                //         _ => {
                //             if let Ok((next, pe)) = parse(next, &mut _reader){
                //                 _next = next;
                //                 frame.element.push(pe);
                //             } else {
                //                 break;
                //             }
                //         }
                //     }
                // }
            }
            // println!("proto: {}", _proto);
            // frame.children = Vec::new();
        }
        frame.index = ctx.counter;
        ctx.counter += 1;
        ctx.list.push(frame);
    }
    pub fn update(&mut self, data: &[u8]) -> Result<String> {
        self.ds.update(data);
        self.parse()?;
        let msg = format!("size: {} range {} - {}", self.ds.data.len(), self.ds.range.start, self.ds.range.end);
        Ok(msg)
    }
    pub fn destroy(&mut self) -> bool {
        // TODO 
        true
    }
}

pub mod core;
pub mod enum_def;
pub mod io;
pub mod macro_def;
