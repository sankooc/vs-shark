use anyhow::{bail, Result};
use io::{DataSource, Reader, IO};
use thiserror::Error;

use crate::files::pcap::{self, PCAP};

/// 内部化字符串
pub fn intern(content: &str) -> &'static str {
    content.to_owned().leak()
    // Box::leak(content.to_owned().into_boxed_str())
}

pub trait FileParser {
    fn has_next(&self) -> bool;
    fn next(&mut self) -> Option<(usize, usize)>;
}

pub struct Instance {
    ds: DataSource,
    file_type: FileType,
    // parser: Option<Box<dyn FileParser>>,
}

#[derive(Default, PartialEq)]
enum ParseState {
    #[default]
    Init,
    Header,
    Packet,
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
        Self { ds, file_type: FileType::NONE }
    }

    pub fn parse(&mut self) -> Result<()> {
        let mut reader = Reader::new(&self.ds, 0);
        if let FileType::NONE = self.file_type {
            let head: &[u8] = &self.ds.data[..4];
            let head_str = format!("{:x}", IO::read32(head, false)?);
            match head_str.as_str() {
                "a1b2c3d4" => {
                    let _magic = reader.read32(true)?;
                    let major = reader.read16(false)?;
                    let minor = reader.read16(false)?;
                    reader._move(8);
                    let _snap_len = reader.read32(false)?;
                    reader._move(2);
                    let linktype = reader.read32(false)?;
                    self.file_type = FileType::PCAP;
                    // self.parser = Some(Box::new(pcap::PCAP::new()));
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
                let (start,end) = PCAP::next(&mut reader)?;
                let mut _reader = Reader::new(&mut self.ds, start);
                // self.parse_packet(&mut _reader);
                // pcap::parse(reader, conf);
            }
            FileType::PCAPNG => {}
            _ => {}
        }
        Ok(())
    }
    pub fn parse_packet(&mut self, reader: &mut Reader){

    }
    pub fn update(&mut self, data: &[u8]) -> Result<()> {
        self.ds.update(data);
        self.parse()?;
        Ok(())
    }
}

pub mod io;
