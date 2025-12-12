// Copyright (c) 2025 sankooc
//
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT
use crate::common::{
    Frame, LinkType, core::Context, enum_def::{DataError, Protocol}, file::{CaptureInterface, FileMetadata, FileStatistics, InterfaceDescription, OptionParser, PcapNg}, io::Reader
};
use anyhow::{bail, Result};

pub struct PCAPNG;

fn option_reader<T>(reader: &mut Reader, parser: &mut T)
where
    T: OptionParser,
{
    loop {
        let option_code = reader.read16(false).unwrap();
        let option_len = reader.read16(false).unwrap();
        if option_code == 0 || option_len == 0 {
            return;
        }
        if option_len as usize > reader.left() {
            return;
        }
        let comment = reader.slice(option_len as usize, false).unwrap();
        parser.parse_option(option_code, comment);
        let ext = (4 - (option_len % 4)) % 4;
        reader.forward(option_len as usize + ext as usize);
    }
}

impl PCAPNG {
    pub fn next(ctx: &mut Context, reader: &mut Reader) -> Result<(usize, Option<Frame>, Protocol)> {
        if reader.left() < 8 {
            bail!(DataError::EndOfStream)
        }
        let block_type = format!("{:#010x}", reader.read32(false)?);
        let len = reader.read32(false)?;
        let packet_size = len as usize - 12;

        if reader.left() < packet_size {
            reader.back(8);
            bail!(DataError::EndOfStream)
        }
        match block_type.as_str() {
            "0x0a0d0d0a" => {
                let mut reader2 = reader.slice_as_reader(packet_size)?;
                let _magic = reader2.read32(false)?;
                let major = reader2.read16(false)?;
                let minor = reader2.read16(false)?;
                let section_len = reader2.read64(true)?;
                if let FileMetadata::PcapNg(meta) = &mut ctx.metadata {
                    meta.major = major;
                    meta.minor = minor;
                    if section_len == 0xFFFFFFFFFFFFFFFF {
                        let mut cap = CaptureInterface::default();
                        option_reader(&mut reader2, &mut cap);
                        meta.capture = Some(cap);
                    } else if section_len > reader2.left() as u64 {
                    } else {
                        let mut cap = CaptureInterface::default();
                        option_reader(&mut reader2, &mut cap);
                        meta.capture = Some(cap);
                    }

                }
            }
            "0x00000001" => {
                let mut reader2 = reader.slice_as_reader(packet_size)?;
                {
                    let lt = reader2.read16(false)? as LinkType;
                    
                    let protocol = PcapNg::_protocol(lt);
                    let mut id = InterfaceDescription::new(lt, protocol);
                    let _snap_len = reader2.read32(false)?;
                    reader2.forward(2);
                    option_reader(&mut reader2, &mut id);
                    if let FileMetadata::PcapNg(meta) = &mut ctx.metadata {
                        meta.add_interface(id);
                    }
                }
            }
            "0x00000006" => {
                let finish = reader.cursor + packet_size;
                let interface_id = reader.read32(false)? as usize;

                let mut ts = reader.read32(false)? as u64;
                let low_ts = reader.read32(false)? as u64;
                ts = (ts << 32) + low_ts;

                let captured = reader.read32(false)?;
                let _origin = reader.read32(false)?;

                let mut f = Frame::new();
                f.info.len = captured;
                f.info.time = ts;
                let end = reader.cursor + captured as usize;
                f.range = Some(reader.cursor..end);
                if let FileMetadata::PcapNg(meta) = &ctx.metadata {
                    return Ok((finish + 4, Some(f), meta.protocol(interface_id)));
                } else {
                    bail!(DataError::FormatMismatch)
                }
            }
            "0x00000005" => {
                // pcapng statistics block
                let mut reader2 = reader.slice_as_reader(packet_size)?;
                let interface_id = reader2.read32(false)?;
                let ts_high = reader2.read32(false)?;
                let ts_low = reader2.read32(false)?;
                println!("Statistics Block - Interface ID: {}, Timestamp: {}", interface_id, (ts_high as u64) << 32 | (ts_low as u64));
                let mut data = FileStatistics::default();
                option_reader(&mut reader2, &mut data);
                if let FileMetadata::PcapNg(meta) = &mut ctx.metadata {
                    meta.statistics = Some(data);
                }
            }
            _ => {
                reader.slice((len - 12) as usize, true)?;
                // let _len = reader.read32(false)?;
            }
        }
        let _len = reader.read32(false)?;

        Ok((reader.cursor, None, Protocol::None))
    }
}
