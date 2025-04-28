use anyhow::{bail, Result};
use crate::common::{io::Reader, Frame};

pub struct PCAP {}

impl PCAP {

    pub fn next(reader: &mut Reader) -> Result<(usize, Frame)> {
        if reader.left() < 16 {
            bail!("end")
        }
        // let cursor = reader.cursor;
        let t = reader.slice(8, true)?.to_vec();
        let captured = reader.read32(false)?;
        let origin = reader.read32(false)?;
        if captured != origin {
            bail!("nomatch")
        }
        if reader.left() < (origin as usize) {
            reader.back(16);
            bail!("end of stream")
        }
        let mut f = Frame::new();
        f.size = origin;
        f.time = Some(t);
        f.range = Some(reader.cursor..reader.cursor + origin as usize);
        Ok((origin as usize + reader.cursor, f))
    }
}
