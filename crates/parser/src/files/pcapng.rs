// Copyright (c) 2025 sankooc
//
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT
use crate::common::{
    core::Context,
    enum_def::{DataError, Protocol},
    file::{CaptureInterface, FileMetadata, FileStatistics, InterfaceDescription, OptionParser, PcapNg},
    io::Reader,
    Frame, LinkType,
};
use anyhow::{bail, Result};

pub struct PCAPNG;

fn option_reader<T>(reader: &mut Reader, parser: &mut T) -> Result<()>
where
    T: OptionParser,
{
    loop {
        if reader.left() < 4 {
            return Ok(());
        }
        let option_code = reader.read16(false)?;
        let option_len = reader.read16(false)?;
        if option_code == 0 || option_len == 0 {
            return Ok(());
        }
        if option_len as usize > reader.left() {
            return Ok(());
        }
        let comment = reader.slice(option_len as usize, false)?;
        parser.parse_option(option_code, comment);
        let ext = (4 - (option_len % 4)) % 4;
        reader.forward(option_len as usize + ext as usize);
    }
}

fn session_header_block(meta: &mut PcapNg, reader: &mut Reader) -> Result<()> {
    let _magic = reader.read32(false)?;
    let major = reader.read16(false)?;
    let minor = reader.read16(false)?;
    let section_len = reader.read64(true)?;
    meta.major = major;
    meta.minor = minor;
    if section_len == 0xFFFFFFFFFFFFFFFF {
        let mut cap = CaptureInterface::default();
        option_reader(reader, &mut cap)?;
        meta.capture = Some(cap);
    } else if section_len > reader.left() as u64 {
    } else {
        let mut cap = CaptureInterface::default();
        option_reader(reader, &mut cap)?;
        meta.capture = Some(cap);
    }
    Ok(())
}

fn interface_description_block(meta: &mut PcapNg, reader: &mut Reader) -> Result<()> {
    let lt = reader.read16(false)? as LinkType;
    let protocol = PcapNg::_protocol(lt);
    let mut id = InterfaceDescription::new(lt, protocol);
    let _snap_len = reader.read32(false)?;
    reader.forward(2);
    option_reader(reader, &mut id)?;
    meta.add_interface(id);
    Ok(())
}
fn interface_statisic_block(meta: &mut PcapNg, reader: &mut Reader) -> Result<()> {
    let _interface_id = reader.read32(false)?;
    let _ts_high = reader.read32(false)?;
    let _ts_low = reader.read32(false)?;
    let mut data = FileStatistics::default();
    option_reader(reader, &mut data)?;
    meta.statistics = Some(data);
    Ok(())
}
// fn session_header_block(meta: &mut PcapNg, reader: &mut Reader) -> Result<()> {
//     Ok(())
// }

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
                if let FileMetadata::PcapNg(meta) = &mut ctx.metadata {
                    session_header_block(meta, &mut reader2)?;
                }
            }
            "0x00000001" => {
                let mut reader2 = reader.slice_as_reader(packet_size)?;
                if let FileMetadata::PcapNg(meta) = &mut ctx.metadata {
                    interface_description_block(meta, &mut reader2)?;
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
                let mut reader2 = reader.slice_as_reader(packet_size)?;
                if let FileMetadata::PcapNg(meta) = &mut ctx.metadata {
                    interface_statisic_block(meta, &mut reader2)?;
                }
            }
            _ => {
                reader.slice((len - 12) as usize, true)?;
            }
        }
        let _len = reader.read32(false)?;

        Ok((reader.cursor, None, Protocol::None))
    }
}
