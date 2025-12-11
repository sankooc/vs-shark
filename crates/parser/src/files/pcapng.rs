// Copyright (c) 2025 sankooc
//
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT
use crate::common::{
    core::Context,
    enum_def::{DataError, Protocol},
    file::{CaptureInterface, FileMetadata, InterfaceDescription, OptionParser},
    io::Reader,
    Frame, LinkType,
};
use anyhow::{bail, Result};

pub struct PCAPNG {}

// fn parse_interface(data: &[u8]) -> Result<u16> {
//     // let reader = SliceReader::new(data);
//     let head: &[u8] = &data[..2];
//     let lt = IO::read16(head, false)?;
//     println!("link type: {lt}");
//     // let  plen = IO::read32(HEAD, endian)
//     // let lt = reader.read16(f)?;
//     // let _revert = reader.read16(false)?;
//     // let _snap_len = reader.read32(true)?;
//     Ok(lt)
// }

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
        let comment_str: std::borrow::Cow<'_, str> = String::from_utf8_lossy(comment);
        parser.parse_option(option_code, comment_str);
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

        //
        if reader.left() < packet_size {
            reader.back(8);
            bail!(DataError::EndOfStream)
        }
        match block_type.as_str() {
            "0x0a0d0d0a" => {
                // let _raw = reader.slice(packet_size, true)?;

                let mut reader2 = reader.slice_as_reader(packet_size)?;
                let magic = reader2.read32(false)?;
                println!("magic: {:#010x}", magic);
                let major = reader2.read16(false)?;
                let minor = reader2.read16(false)?;
                let section_len = reader2.read64(true)?;
                println!("version: {}.{}", major, minor);
                println!("section length: {}", section_len);
                {
                    if section_len == 0xFFFFFFFFFFFFFFFF {
                        let mut cap = CaptureInterface::default();
                        option_reader(&mut reader2, &mut cap);
                        println!("capture info: {cap:?}");
                        if let FileMetadata::PcapNg(meta) = &mut ctx.metadata {
                            meta.captrue = Some(cap);
                        }
                    } else {
                        if section_len > reader2.left() as u64 {
                            // return
                        } else {
                            let mut cap = CaptureInterface::default();
                            option_reader(&mut reader2, &mut cap);
                            if let FileMetadata::PcapNg(meta) = &mut ctx.metadata {
                                meta.captrue = Some(cap);
                            }
                        }
                    }
                }
                let _len = reader.read32(false)?;
            }
            "0x00000001" => {
                let mut reader2 = reader.slice_as_reader(packet_size)?;
                {
                    let lt = reader2.read16(false)? as LinkType;
                    let mut id = InterfaceDescription::new(lt);
                    let _snap_len = reader2.read32(false)?;
                    reader2.forward(2);
                    option_reader(&mut reader2, &mut id);
                    if let FileMetadata::PcapNg(meta) = &mut ctx.metadata {
                        meta.add_interface(id);
                    }
                }
                let _len = reader.read32(false)?;
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
                let _raw = reader.slice(packet_size, true)?;
                let _len = reader.read32(false)?;
            }
            _ => {
                println!("Unknown Block Type: {block_type}");
                reader.slice((len - 12) as usize, true)?;
                let _len = reader.read32(false)?;
            }
        }

        Ok((reader.cursor, None, Protocol::None))
    }
}
