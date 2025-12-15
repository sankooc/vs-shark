// Copyright (c) 2025 sankooc
//
// This file is part of the pcapview project.
// Licensed under the MIT License - see https://opensource.org/licenses/MIT

use crate::common::{
    Frame, core::Context, enum_def::{DataError, Protocol}, file::FileMetadata, io::Reader
};
use anyhow::{bail, Result};

pub struct PCAP {}

impl PCAP {
    pub fn next(ctx: &mut Context, reader: &mut Reader) -> Result<(usize, Option<Frame>, Protocol)> {
        if reader.left() < 16 {
            bail!(DataError::EndOfStream)
        }

        // let t = reader.slice(8, true)?.to_vec();

        let h_ts: u64 = reader.read32(false)?.into();
        let l_ts: u64 = reader.read32(false)?.into();
        let ts: u64 = h_ts * 1000000 + l_ts;
        let captured = reader.read32(false)?;
        let _origin = reader.read32(false)?;
        if reader.left() < (captured as usize) {
            reader.back(16);
            bail!(DataError::EndOfStream)
        }
        let mut f = Frame::new();
        f.info.len = captured;
        f.info.time = ts;
        f.range = Some(reader.cursor..reader.cursor + captured as usize);

        if let FileMetadata::Pcap(meta) = &ctx.metadata {
            Ok((captured as usize + reader.cursor, Some(f), meta.protocol))
        } else {
            Ok((captured as usize + reader.cursor, Some(f), Protocol::ETHERNET))
        }
    }
}
