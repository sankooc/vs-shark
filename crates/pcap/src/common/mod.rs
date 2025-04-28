use std::ops::Range;

use anyhow::{bail, Result};
use io::{DataSource, Reader, IO};
use thiserror::Error;

use crate::files::pcap::PCAP;

/// 内部化字符串
pub fn intern(content: &str) -> &'static str {
    content.to_owned().leak()
    // Box::leak(content.to_owned().into_boxed_str())
}

pub trait FileParser {
    fn has_next(&self) -> bool;
    fn next(&mut self) -> Option<(usize, usize)>;
}


#[derive(Default, PartialEq)]
pub struct Frame {
    pub index: u32,
    pub size: u32,
    pub range: Option<Range<usize>>,
    pub time: Option<Vec<u8>>,
    children: Vec<Frame>,
}

impl Frame {
    pub fn new() -> Self {
        Self { ..Default::default()}
    }
}
pub struct Context {
    file_type: FileType,
    list: Vec<Frame>,
    pub counter: u32,
}

impl Context {
    pub fn new() -> Self {
        Self {counter: 0, file_type: FileType::NONE, list: Vec::new()}
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
                    let _linktype = reader.read32(false)?;
                    self.file_type = FileType::PCAP;
                    self.ctx.file_type = FileType::PCAP;
                }
                "a0d0d0a" => {
                    //   return pcapng::parse(reader, conf)
                    bail!(DataError::UnsupportFileType)
                }
                _ => bail!(DataError::UnsupportFileType),
            };
        }
        match self.file_type {
            FileType::PCAP => {
                let ds = &self.ds;
                loop {
                    if let Ok((_next,f)) = PCAP::next(&mut reader){
                        Instance::parse_packet(&mut self.ctx,f, ds);
                        reader.cursor = _next;
                    } else {
                        self.last = reader.cursor;
                        break;
                    }
                }
                // {
                //     let (start, end) = PCAP::next(&mut reader)?;
                //     let mut _reader = Reader::new(&mut self.ds, start, end);
                //     Instance::parse_packet(&mut self.ctx, &mut _reader);
                //     println!("{} {} : {}", start, end, end-start);
                // }
                // pcap::parse(reader, conf);
            }
            FileType::PCAPNG => {}
            _ => {}
        }
        Ok(())
    }
    pub fn parse_packet(ctx: &mut Context, mut frame: Frame, ds: &DataSource) {
        // let f = Frame::new();
        if let Some(range) = &frame.range {
            let mut _reader = Reader::new_sub(&ds, range.clone());
            frame.children = Vec::new();
        }
        frame.index = ctx.counter;
        ctx.counter+=1;
        ctx.list.push(frame);
    }
    pub fn update(&mut self, data: &[u8]) -> Result<()> {
        self.ds.update(data);
        self.parse()?;
        Ok(())
    }
}

pub mod io;
