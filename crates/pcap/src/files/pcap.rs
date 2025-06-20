use anyhow::{bail, Result};
use crate::common::{enum_def::DataError, io::Reader, Frame};

pub struct PCAP {}

impl PCAP {

    pub fn next(reader: &mut Reader) -> Result<(usize, Frame)> {
        if reader.left() < 16 {
            bail!(DataError::EndOfStream)
        }

        // let t = reader.slice(8, true)?.to_vec();
        
        let h_ts: u64 = reader.read32(false)?.into();
        let l_ts: u64 = reader.read32(false)?.into();
        let ts: u64 = h_ts * 1000000 + l_ts;
        let captured = reader.read32(false)?;
        let origin = reader.read32(false)?;
        if captured != origin {
            bail!(DataError::FormatMissMatch);
        }
        if reader.left() < (origin as usize) {
            reader.back(16);
            bail!(DataError::EndOfStream)
        }
        let mut f = Frame::new();
        f.info.len = origin;
        f.info.time = ts;
        f.range = Some(reader.cursor..reader.cursor + origin as usize);
        Ok((origin as usize + reader.cursor, f))
    }
}
